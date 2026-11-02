//! Platform self-test: display (fbdev), audio (ALSA), input (evdev), network.

use crate::{alsa, fb, input, sys, yt};
use crate::{log, wait_for};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

pub fn display_test(seconds: f64, stop: Arc<AtomicBool>) {
    if !wait_for("/dev/fb0", 3000) {
        log("display: /dev/fb0 missing");
        return;
    }
    let mut fb = match fb::Framebuffer::open("/dev/fb0") {
        Ok(fb) => fb,
        Err(e) => return log(format!("display: open failed: {e}")),
    };
    let v = fb.var;
    log(format!(
        "display: {} {}x{} virt {}x{} {}bpp R{}/{} G{}/{} B{}/{} A{}/{} stride {} pages {} vsync {}",
        fb.id(), v.xres, v.yres, v.xres_virtual, v.yres_virtual, v.bits_per_pixel,
        v.red.offset, v.red.length, v.green.offset, v.green.length, v.blue.offset,
        v.blue.length, v.transp.offset, v.transp.length, fb.fix.line_length, fb.pages, fb.vsync_ok));
    let bars: Vec<u32> = [(255, 255, 255), (255, 255, 0), (0, 255, 255), (0, 255, 0),
                          (255, 0, 255), (255, 0, 0), (0, 0, 255), (0, 0, 0)]
        .iter().map(|&(r, g, b)| fb.pixel(r, g, b)).collect();
    let white = fb.pixel(255, 255, 255);
    let start = sys::monotonic();
    let mut frames = 0u32;
    while (sys::monotonic() - start).as_secs_f64() < seconds && !stop.load(Ordering::Relaxed) {
        let t = (sys::monotonic() - start).as_secs_f64();
        let mut bb = fb.back_buffer();
        let (w, h) = (bb.width, bb.height);
        for (i, &c) in bars.iter().enumerate() {
            bb.fill_rect(i * w / 8, 0, w / 8 + 1, h * 2 / 3, c);
        }
        bb.fill_rect(0, h * 2 / 3, w, h - h * 2 / 3, bars[7]);
        let x = ((t * 400.0) as usize) % (w - 80);
        bb.fill_rect(x, h * 2 / 3 + 40, 80, 80, white);
        if let Err(e) = fb.present() {
            return log(format!("display: present failed: {e}"));
        }
        frames += 1;
        if !fb.vsync_ok {
            thread::sleep(Duration::from_millis(33));
        }
    }
    let el = (sys::monotonic() - start).as_secs_f64();
    log(format!("display: {frames} frames in {el:.2}s ({:.1} fps)", frames as f64 / el));
}

pub fn audio_test(seconds: f64) {
    let path = "/dev/snd/pcmC0D0p";
    if !wait_for(path, 3000) {
        return log(format!("audio: {path} missing"));
    }
    let mut pcm = match alsa::Pcm::open(path, 48000, 2) {
        Ok(p) => p,
        Err(e) => return log(format!("audio: open/hw_params failed: {e}")),
    };
    log(format!("audio: S16_LE {} Hz x{} period {} buffer {} frames",
                pcm.rate, pcm.channels, pcm.period_frames, pcm.buffer_frames));
    let total = (pcm.rate as f64 * seconds) as usize;
    let chunk = pcm.period_frames as usize;
    let mut phase = 0f64;
    let step = 2.0 * std::f64::consts::PI * 440.0 / pcm.rate as f64;
    let mut buf = vec![0i16; chunk * 2];
    let mut written = 0;
    let mut max_delay = 0;
    while written < total {
        for f in buf.chunks_exact_mut(2) {
            let s = (phase.sin() * 0.3 * i16::MAX as f64) as i16;
            f[0] = s;
            f[1] = s;
            phase += step;
        }
        if let Err(e) = pcm.write(&buf) {
            return log(format!("audio: write failed: {e}"));
        }
        written += chunk;
        max_delay = max_delay.max(pcm.delay().unwrap_or(-1));
    }
    let _ = pcm.drain();
    log(format!("audio: played {written} frames of 440 Hz; max delay {max_delay} frames; {} underruns",
                pcm.underruns));
}

pub fn input_watch(stop: Arc<AtomicBool>) {
    let mut devs = input::open_all();
    for d in &devs {
        log(format!("input: {} \"{}\"", d.path, d.name));
    }
    while !stop.load(Ordering::Relaxed) {
        for d in &mut devs {
            if let Ok(keys) = d.poll_keys() {
                for k in keys.iter().filter(|k| k.value == 1) {
                    log(format!("input: key {} pressed on {}", k.code, d.path));
                }
            }
        }
        thread::sleep(Duration::from_millis(20));
    }
}

/// Top-level MP4 boxes in a byte buffer: (type, size).
fn mp4_boxes(b: &[u8]) -> Vec<(String, u64)> {
    let mut out = Vec::new();
    let mut i = 0usize;
    while i + 8 <= b.len() {
        let size = u32::from_be_bytes(b[i..i + 4].try_into().unwrap()) as u64;
        let ty = String::from_utf8_lossy(&b[i + 4..i + 8]).into_owned();
        out.push((ty, size));
        if size < 8 {
            break;
        }
        i += size as usize;
    }
    out
}

pub fn network_test(server: &str, video: &str) {
    log(format!("net: resolving {video} via {server}"));
    let v = match yt::resolve(server, video) {
        Ok(v) => v,
        Err(e) => return log(format!("net: resolve failed: {e}")),
    };
    log(format!("net: \"{}\" ({} s), {} formats", v.title, v.length_s, v.formats.len()));
    let (Some(vf), Some(af)) = (yt::pick_video(&v.formats, 720), yt::pick_audio(&v.formats)) else {
        return log("net: no H.264<=720p + AAC pair");
    };
    for f in [vf, af] {
        log(format!("net: itag {} {} {}x{} {} bytes", f.itag, f.mime, f.width, f.height,
                    f.content_length));
        let t0 = sys::monotonic();
        match yt::fetch_range(&f.url, 0, f.index_range.1) {
            Ok(b) => log(format!("net:   header+index {} bytes in {} ms: {:?}", b.len(),
                                 (sys::monotonic() - t0).as_millis(), mp4_boxes(&b))),
            Err(e) => log(format!("net:   fetch failed: {e}")),
        }
    }
}

