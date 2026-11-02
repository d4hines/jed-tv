//! Shared state between the simulation (flighttv, confined, static musl) and a
//! GPU renderer in another process (flightgpu, Android ABI, drawing with the
//! Mali through the stock GL stack). One fixed-size record in a memory-mapped
//! file, published with a sequence lock: odd while being written, even when
//! complete. The renderer polls it; nothing else is shared.
//!
//! The renderer announces itself by writing the record's path into
//! `/tmp/flight-gpu.ready`; flighttv reads that at start-up.
//!
//! The finished picture travels the other way through a second mapped file,
//! `<record>.frame` (`Frame`): the GPU process cannot reach the display in
//! the native session (the display service owns it), while flighttv already
//! presents through the platform's display path. So flighttv publishes the
//! state, the renderer publishes the frame, and flighttv shows it.

use crate::render::{OUT_H, OUT_W};
use crate::{Overlay, Snap};
use std::sync::atomic::{AtomicU32, Ordering};

pub const READY_FILE: &str = "/tmp/flight-gpu.ready";
pub const MAX_PATH: usize = 256;

/// A finished 1280x720 frame (0x00RRGGBB), published with the same sequence lock as `Shared`.
#[repr(C)]
pub struct Frame {
    seq: AtomicU32,
    _pad: [u32; 15],
    pub px: [u32; OUT_W * OUT_H],
}

impl Frame {
    pub const SIZE: usize = std::mem::size_of::<Frame>();

    pub fn publish(&mut self, px: &[u32]) {
        self.seq.fetch_add(1, Ordering::AcqRel);
        let dst = self.px.as_mut_ptr();
        for (i, &p) in px[..OUT_W * OUT_H].iter().enumerate() {
            unsafe { std::ptr::write_volatile(dst.add(i), p) };
        }
        self.seq.fetch_add(1, Ordering::AcqRel);
    }

    /// Copy the latest complete frame into `out` if it is newer than `last`; returns its sequence number.
    pub fn read(&self, last: u32, out: &mut [u32]) -> Option<u32> {
        let before = self.seq.load(Ordering::Acquire);
        if before % 2 == 1 || before == 0 || before == last {
            return None;
        }
        let src = self.px.as_ptr();
        for (i, o) in out[..OUT_W * OUT_H].iter_mut().enumerate() {
            *o = unsafe { std::ptr::read_volatile(src.add(i)) };
        }
        (self.seq.load(Ordering::Acquire) == before).then_some(before)
    }
}

/// The frame file's path for a record path.
pub fn frame_path(record: &str) -> String {
    format!("{record}.frame")
}

/// The state record's payload, copied whole with volatile reads and writes.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Record {
    pub s: [f64; 10],
    pub time: f64,
    pub agl: f64,
    pub tick_ms: f64,
    pub fps: f32,
    pub speed: f32,
    pub scenario: u32,
    /// bit 0 filter on, 1 crashed, 2 kamikaze, 3 pilot accepted, 4 show help, 5 cockpit camera.
    pub flags: u32,
    pub escape: [u8; 64],
    pub status: [u8; 64],
    pub path_len: u32,
    pub path: [[f32; 3]; MAX_PATH],
}

#[repr(C)]
pub struct Shared {
    seq: AtomicU32,
    _pad: [u32; 15],
    rec: Record,
}

/// What the renderer gets: the snapshot plus the frame loop's own values.
#[derive(Clone, Debug)]
pub struct Published {
    pub snap: Snap,
    pub fps: f64,
    pub speed: f64,
    pub show_help: bool,
    pub cam_mode: u8,
    pub status: String,
}

fn put(dst: &mut [u8], s: &str) {
    let n = s.len().min(dst.len() - 1);
    dst[..n].copy_from_slice(&s.as_bytes()[..n]);
    dst[n..].fill(0);
}

fn get(src: &[u8]) -> String {
    let n = src.iter().position(|&b| b == 0).unwrap_or(src.len());
    String::from_utf8_lossy(&src[..n]).into_owned()
}

impl Shared {
    pub const SIZE: usize = std::mem::size_of::<Shared>();

    pub fn publish(&mut self, snap: &Snap, o: &Overlay, cam_mode: u8) {
        let mut r: Record = unsafe { std::mem::zeroed() };
        r.s = snap.s;
        r.time = snap.time;
        r.agl = snap.agl;
        r.tick_ms = snap.tick_ms;
        r.fps = o.fps as f32;
        r.speed = o.speed as f32;
        r.scenario = snap.scenario as u32;
        r.flags = (snap.filter_on as u32) | (snap.crashed as u32) << 1 | (snap.kamikaze as u32) << 2
            | (snap.accepted as u32) << 3 | (o.show_help as u32) << 4 | ((cam_mode == 1) as u32) << 5;
        put(&mut r.escape, &snap.escape);
        put(&mut r.status, o.status);
        let n = snap.path.len().min(MAX_PATH);
        r.path_len = n as u32;
        for (i, p) in snap.path.iter().take(n).enumerate() {
            r.path[i] = [p[0] as f32, p[1] as f32, p[2] as f32];
        }
        self.seq.fetch_add(1, Ordering::AcqRel); // odd: writing
        unsafe { std::ptr::write_volatile(&mut self.rec, r) };
        self.seq.fetch_add(1, Ordering::AcqRel); // even: complete
    }

    /// The latest complete record and its sequence number, or `None` while a write is in progress.
    pub fn read(&self) -> Option<(u32, Published)> {
        let before = self.seq.load(Ordering::Acquire);
        if before % 2 == 1 || before == 0 {
            return None;
        }
        let r: Record = unsafe { std::ptr::read_volatile(&self.rec) };
        if self.seq.load(Ordering::Acquire) != before {
            return None;
        }
        let flags = r.flags;
        let n = (r.path_len as usize).min(MAX_PATH);
        let p = Published {
            snap: Snap {
                s: r.s, time: r.time, scenario: r.scenario as usize, filter_on: flags & 1 != 0, crashed: flags & 2 != 0,
                kamikaze: flags & 4 != 0, agl: r.agl, path: r.path[..n].iter().map(|p| [p[0] as f64, p[1] as f64, p[2] as f64]).collect(),
                accepted: flags & 8 != 0, escape: get(&r.escape), tick_ms: r.tick_ms,
            },
            fps: r.fps as f64, speed: r.speed as f64, show_help: flags & 16 != 0, cam_mode: (flags >> 5 & 1) as u8,
            status: get(&r.status),
        };
        Some((before, p))
    }
}

#[cfg(any(target_os = "linux", target_os = "android"))]
mod map {
    use super::Shared;
    use std::os::fd::AsRawFd;
    extern "C" {
        // Bionic ARM32 mmap takes a 32-bit offset; mmap64 matches this ABI.
        #[cfg_attr(target_os = "android", link_name = "mmap64")]
        fn mmap(addr: *mut u8, len: usize, prot: i32, flags: i32, fd: i32, off: i64) -> *mut u8;
    }

    fn map_file<T>(path: &str, size: usize, create: bool) -> std::io::Result<&'static mut T> {
        let file = std::fs::OpenOptions::new().read(true).write(true).create(create).open(path)?;
        if file.metadata()?.len() != size as u64 {
            if !create {
                return Err(std::io::Error::other("shared file has the wrong size"));
            }
            file.set_len(size as u64)?;
        }
        let p = unsafe { mmap(std::ptr::null_mut(), size, 3, 1, file.as_raw_fd(), 0) };
        if p as isize == -1 {
            return Err(std::io::Error::last_os_error());
        }
        std::mem::forget(file);
        Ok(unsafe { &mut *(p as *mut T) })
    }

    /// Map the record at `path`, creating or resizing the file (the renderer) or requiring it (flighttv).
    pub fn open(path: &str, create: bool) -> std::io::Result<&'static mut Shared> {
        map_file(path, Shared::SIZE, create)
    }

    /// Map the frame next to the record at `path` (see `frame_path`).
    pub fn open_frame(path: &str, create: bool) -> std::io::Result<&'static mut super::Frame> {
        map_file(&super::frame_path(path), super::Frame::SIZE, create)
    }
}
#[cfg(any(target_os = "linux", target_os = "android"))]
pub use map::{open, open_frame};

/// The record path a running GPU renderer announced, if any.
pub fn announced() -> Option<String> {
    std::fs::read_to_string(READY_FILE).ok().map(|s| s.trim().to_owned()).filter(|s| !s.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_round_trips() {
        let mut boxed: Box<Shared> = unsafe { Box::new(std::mem::zeroed()) };
        assert!(boxed.read().is_none(), "nothing published yet");
        let snap = Snap {
            s: [1.0, 2.0, 3.0, 45.0, 0.1, 1.5, -0.2, 0.0, 1.0, 0.7], time: 12.5, scenario: 4, filter_on: true, crashed: false, kamikaze: true,
            agl: 88.0, path: vec![[1.0, 2.0, 3.0], [4.0, 5.0, 6.0]], accepted: false, escape: "verified escape: street (879 ms)".into(), tick_ms: 584.0,
        };
        boxed.publish(&snap, &Overlay { fps: 2.1, speed: 0.02, show_help: true, status: "hello" }, 1);
        let (seq, p) = boxed.read().unwrap();
        assert_eq!(seq, 2);
        assert_eq!(p.snap.s, snap.s);
        assert_eq!(p.snap.path, snap.path);
        assert_eq!(p.snap.escape, snap.escape);
        assert!(p.snap.filter_on && !p.snap.crashed && p.snap.kamikaze && !p.snap.accepted && p.show_help);
        assert_eq!(p.cam_mode, 1);
        assert_eq!(p.status, "hello");
        assert!((p.fps - 2.1).abs() < 1e-6);
    }

    #[test]
    fn frame_round_trips_and_is_read_once() {
        let mut f: Box<Frame> = unsafe { Box::new(std::mem::zeroed()) };
        let mut out = vec![0u32; OUT_W * OUT_H];
        assert!(f.read(0, &mut out).is_none(), "nothing published yet");
        let px: Vec<u32> = (0..OUT_W * OUT_H as usize).map(|i| i as u32).collect();
        f.publish(&px);
        let seq = f.read(0, &mut out).unwrap();
        assert_eq!(seq, 2);
        assert_eq!(out, px);
        assert!(f.read(seq, &mut out).is_none(), "the same frame is not read twice");
        assert_eq!(frame_path("/run/flight-gpu/state"), "/run/flight-gpu/state.frame");
    }
}
