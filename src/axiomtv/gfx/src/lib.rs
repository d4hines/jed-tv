//! Axiom's platform layer on the TV, in place of Raylib. The Lean game calls
//! these `agfx_*` functions (through src/axiomtv/native/lean_glue.c, which
//! unpacks Lean strings and byte arrays) to draw a frame, show it, read the
//! remote and play sound. All state lives in one process-wide instance; the
//! Lean game loop is single-threaded.

pub mod raster;

use raster::Renderer;
use std::sync::Mutex;

struct State {
    render: Renderer,
    #[cfg(target_os = "linux")]
    tv: Option<tv::Tv>,
}

// The display holds a raw pointer to the mapped framebuffer. Only the Lean game
// loop (one thread) calls into this library, and the mutex serializes the calls.
unsafe impl Send for State {}

static STATE: Mutex<Option<State>> = Mutex::new(None);

fn with<R>(f: impl FnOnce(&mut State) -> R, fallback: R) -> R {
    let mut guard = STATE.lock().unwrap_or_else(|e| e.into_inner());
    match guard.as_mut() {
        Some(s) => f(s),
        None => fallback,
    }
}

/// Open the display, the remote socket PID 1 passed as `remote_fd` (or -1 to read
/// /dev/input directly) and audio at `rate` Hz mono. Returns 0 or a negative errno
/// for the display; audio and input failures are logged and tolerated.
#[no_mangle]
pub extern "C" fn agfx_open(remote_fd: i32, rate: u32) -> i32 {
    let mut guard = STATE.lock().unwrap_or_else(|e| e.into_inner());
    if guard.is_some() {
        return 0;
    }
    #[cfg(target_os = "linux")]
    {
        match tv::Tv::open(remote_fd, rate) {
            Ok(t) => {
                *guard = Some(State { render: Renderer::new(), tv: Some(t) });
                0
            }
            Err(e) => -e.raw_os_error().unwrap_or(libc::EIO),
        }
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (remote_fd, rate);
        *guard = Some(State { render: Renderer::new() });
        0
    }
}

/// Next key event as `0x80000000 | value << 16 | keycode` (value 0 release, 1 press,
/// 2 repeat), or 0 when none is pending.
#[no_mangle]
pub extern "C" fn agfx_poll_key() -> u32 {
    #[cfg(target_os = "linux")]
    {
        with(|s| s.tv.as_mut().and_then(|t| t.poll_key()).unwrap_or(0), 0)
    }
    #[cfg(not(target_os = "linux"))]
    {
        0
    }
}

#[no_mangle]
pub extern "C" fn agfx_begin_frame() {
    with(|s| s.render.begin_frame(), ())
}

#[no_mangle]
pub extern "C" fn agfx_rect(x: i32, y: i32, w: i32, h: i32, rgba: u32) {
    with(|s| s.render.rect(x, y, w, h, rgba), ())
}

#[no_mangle]
pub extern "C" fn agfx_gradient_v(x: i32, y: i32, w: i32, h: i32, top: u32, bottom: u32) {
    with(|s| s.render.gradient_v(x, y, w, h, top, bottom), ())
}

#[no_mangle]
pub extern "C" fn agfx_gradient_h(x: i32, y: i32, w: i32, h: i32, left: u32, right: u32) {
    with(|s| s.render.gradient_h(x, y, w, h, left, right), ())
}

#[no_mangle]
pub extern "C" fn agfx_circle(x: i32, y: i32, r: f32, rgba: u32) {
    with(|s| s.render.circle(x, y, r, rgba), ())
}

#[no_mangle]
pub extern "C" fn agfx_line(x0: i32, y0: i32, x1: i32, y1: i32, rgba: u32) {
    with(|s| s.render.line(x0, y0, x1, y1, rgba), ())
}

/// UTF-8 text of `len` bytes; characters outside ASCII draw as blanks.
///
/// # Safety
/// `s` must point to `len` readable bytes.
#[no_mangle]
pub unsafe extern "C" fn agfx_text(x: i32, y: i32, scale: u32, rgba: u32, s: *const u8, len: usize) {
    let text = String::from_utf8_lossy(std::slice::from_raw_parts(s, len)).into_owned();
    with(|st| st.render.text(x, y, scale, rgba, &text), ())
}

#[no_mangle]
pub extern "C" fn agfx_begin_3d(px: f32, py: f32, pz: f32, tx: f32, ty: f32, tz: f32, fovy: f32) {
    with(|s| s.render.begin_3d([px, py, pz], [tx, ty, tz], fovy), ())
}

/// Faces of one world section, 8 bytes each (x y z face r g b a), see `raster::Renderer::section`.
///
/// # Safety
/// `faces` must point to `len` readable bytes.
#[no_mangle]
pub unsafe extern "C" fn agfx_section(ox: u32, oy: u32, oz: u32, size: u32, faces: *const u8, len: usize) {
    let data = std::slice::from_raw_parts(faces, len);
    with(|s| s.render.section([ox, oy, oz], size, data), ())
}

#[no_mangle]
pub extern "C" fn agfx_cube(cx: f32, cy: f32, cz: f32, sx: f32, sy: f32, sz: f32, rgba: u32) {
    with(|s| s.render.cube([cx, cy, cz], [sx, sy, sz], rgba), ())
}

#[no_mangle]
pub extern "C" fn agfx_cube_wires(cx: f32, cy: f32, cz: f32, size: f32, rgba: u32) {
    with(|s| s.render.cube_wires([cx, cy, cz], size, rgba), ())
}

#[no_mangle]
pub extern "C" fn agfx_plane(cx: f32, cy: f32, cz: f32, w: f32, d: f32, rgba: u32) {
    with(|s| s.render.plane([cx, cy, cz], w, d, rgba), ())
}

/// Show the frame: scaled to the TV display. Returns 0 or a negative errno.
#[no_mangle]
pub extern "C" fn agfx_present() -> i32 {
    #[cfg(target_os = "linux")]
    {
        with(|s| match s.tv.as_mut() {
            Some(t) => t.present(&s.render.color),
            None => -libc::ENODEV,
        }, -libc::ENODEV)
    }
    #[cfg(not(target_os = "linux"))]
    {
        0
    }
}

/// Queue signed 16-bit mono samples. Returns 0, or a negative errno (including when
/// there is no audio device).
///
/// # Safety
/// `samples` must point to `count` readable `i16`s.
#[no_mangle]
pub unsafe extern "C" fn agfx_audio_write(samples: *const i16, count: usize) -> i32 {
    #[cfg(target_os = "linux")]
    {
        let data = std::slice::from_raw_parts(samples, count);
        with(|s| s.tv.as_mut().map_or(-libc::ENODEV, |t| t.audio_write(data)), -libc::ENODEV)
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (samples, count);
        -38
    }
}

/// Milliseconds on the monotonic clock.
#[no_mangle]
pub extern "C" fn agfx_now_ms() -> u32 {
    use std::sync::OnceLock;
    static START: OnceLock<std::time::Instant> = OnceLock::new();
    START.get_or_init(std::time::Instant::now).elapsed().as_millis() as u32
}

#[cfg(target_os = "linux")]
mod tv {
    use crate::raster::{H, W};
    use platform::{alsa, fb};
    use std::io::{self, Read};
    use std::os::fd::FromRawFd;

    pub struct Tv {
        display: fb::Framebuffer,
        input: Option<std::fs::File>,
        pending: std::collections::VecDeque<u32>,
        audio: Option<alsa::Pcm>,
    }

    impl Tv {
        pub fn open(remote_fd: i32, rate: u32) -> io::Result<Tv> {
            let display = fb::Framebuffer::open("/dev/fb0")?;
            let input = if remote_fd >= 0 {
                let copy = unsafe { libc::fcntl(remote_fd, libc::F_DUPFD_CLOEXEC, 4) };
                if copy < 0 {
                    eprintln!("axiomgfx: remote socket: {}", io::Error::last_os_error());
                    None
                } else {
                    Some(unsafe { std::fs::File::from_raw_fd(copy) })
                }
            } else {
                None
            };
            let audio = match alsa::Pcm::open("/dev/snd/pcmC0D0p", rate, 1) {
                Ok(p) => Some(p),
                Err(e) => {
                    eprintln!("axiomgfx: no audio: {e}");
                    None
                }
            };
            Ok(Tv { display, input, pending: Default::default(), audio })
        }

        /// Drain the launcher's datagram socket of `input_event`s; keys only.
        pub fn poll_key(&mut self) -> Option<u32> {
            if let Some(ev) = self.pending.pop_front() {
                return Some(ev);
            }
            let file = self.input.as_mut()?;
            let sz = std::mem::size_of::<libc::input_event>();
            let mut buf = vec![0u8; sz * 32];
            loop {
                match file.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        for chunk in buf[..n].chunks_exact(sz) {
                            let ev: libc::input_event = unsafe { std::ptr::read_unaligned(chunk.as_ptr() as *const _) };
                            if ev.type_ == 1 && (0..=2).contains(&ev.value) {
                                self.pending.push_back(0x8000_0000 | (ev.value as u32) << 16 | ev.code as u32);
                            }
                        }
                    }
                    Err(e) if e.kind() == io::ErrorKind::WouldBlock => break,
                    Err(_) => break,
                }
            }
            self.pending.pop_front()
        }

        /// Nearest-neighbour scale of the 320x180 frame to the display, centred, in the
        /// display's pixel format.
        pub fn present(&mut self, color: &[u32]) -> i32 {
            let var = self.display.var;
            let native = var.red.offset == 16 && var.green.offset == 8 && var.blue.offset == 0 && var.transp.length == 0;
            let bb = self.display.back_buffer();
            let scale = (bb.width / W).min(bb.height / H).max(1);
            let (fw, fh) = (W * scale, H * scale);
            let (ox, oy) = ((bb.width.saturating_sub(fw)) / 2, (bb.height.saturating_sub(fh)) / 2);
            let black = var.pixel(0, 0, 0);
            let mut xmap = vec![0usize; bb.width];
            for (x, m) in xmap.iter_mut().enumerate() {
                *m = x.saturating_sub(ox) / scale;
            }
            for y in 0..bb.height {
                let row = &mut bb.px[y * bb.stride..y * bb.stride + bb.width];
                if y < oy || y >= oy + fh {
                    row.fill(black);
                    continue;
                }
                let sy = (y - oy) / scale;
                if (y - oy) % scale != 0 {
                    // Same source row as the line above: copy it.
                    let (prev, cur) = bb.px.split_at_mut(y * bb.stride);
                    cur[..bb.width].copy_from_slice(&prev[(y - 1) * bb.stride..(y - 1) * bb.stride + bb.width]);
                    continue;
                }
                let src = &color[sy * W..(sy + 1) * W];
                for (x, p) in row.iter_mut().enumerate() {
                    *p = if x < ox || x >= ox + fw {
                        black
                    } else {
                        let c = src[xmap[x]];
                        if native { c } else { var.pixel((c >> 16) as u8, (c >> 8) as u8, c as u8) }
                    };
                }
            }
            match self.display.present() {
                Ok(()) => 0,
                Err(e) => -e.raw_os_error().unwrap_or(libc::EIO),
            }
        }

        pub fn audio_write(&mut self, samples: &[i16]) -> i32 {
            match self.audio.as_mut() {
                None => -libc::ENODEV,
                Some(p) => match p.write(samples) {
                    Ok(()) => 0,
                    Err(e) => -e.raw_os_error().unwrap_or(libc::EIO),
                },
            }
        }
    }
}
