//! Playback: fetch fragments by byte range, demux, decode, and present video
//! against the audio clock. Also a `framemd5` mode for bit-exactness checks.

use crate::decode::{Aac, H264};
use crate::{alsa, fb, sys, yt, yuv};
use std::io;
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

fn log(msg: impl AsRef<str>) {
    crate::log(msg)
}

/// One track of a YouTube format: init + index, and a way to fetch fragments.
pub struct Track {
    pub fmt: yt::Format,
    pub init: mp4::Init,
    pub index: mp4::Index,
}

impl Track {
    pub fn open(fmt: &yt::Format) -> io::Result<Self> {
        let head = yt::fetch_range(&fmt.url, 0, fmt.index_range.1)?;
        let init = mp4::parse_init(&head).map_err(io::Error::other)?;
        let idx_start = fmt.index_range.0 as usize;
        let index = mp4::parse_sidx(head.get(idx_start..).unwrap_or(&[]), fmt.index_range.0)
            .map_err(io::Error::other)?;
        Ok(Track { fmt: fmt.clone(), init, index })
    }

    pub fn fetch(&self, i: usize) -> io::Result<Vec<mp4::Sample>> {
        let s = &self.index.segments[i];
        let bytes = yt::fetch_range(&self.fmt.url, s.offset, s.offset + s.size as u64 - 1)?;
        mp4::parse_fragments(&bytes, &self.init).map_err(io::Error::other)
    }

    /// Presentation time in seconds for a sample pts.
    pub fn seconds(&self, pts: i64) -> f64 {
        (pts - self.init.edit_offset) as f64 / self.init.timescale as f64
    }
}

fn avc_config(t: &Track) -> io::Result<Vec<u8>> {
    match &t.init.codec {
        mp4::Codec::Avc { avcc, .. } => Ok(avcc.clone()),
        c => Err(io::Error::other(format!("not H.264: {c:?}"))),
    }
}

/// Decode up to `limit` frames and print one framemd5 line per frame.
pub fn framemd5(video: &Track, limit: usize, threads: u32) -> io::Result<()> {
    let mut dec = H264::new(&avc_config(video)?, threads)?;
    let mut n = 0usize;
    let t0 = sys::monotonic();
    let emit = |dec: &mut H264, n: &mut usize| -> io::Result<()> {
        while *n < limit {
            let Some(f) = dec.receive()? else { break };
            println!("framemd5 {} {} {}x{} {}", *n, f.pts, f.width, f.height, f.md5());
            *n += 1;
        }
        Ok(())
    };
    'outer: for i in 0..video.index.segments.len() {
        for s in video.fetch(i)? {
            while !dec.send(&s.data, s.pts, s.dts as i64)? {
                emit(&mut dec, &mut n)?;
                if n >= limit { break 'outer; }
            }
            emit(&mut dec, &mut n)?;
            if n >= limit {
                break 'outer;
            }
        }
    }
    if n < limit {
        dec.flush()?;
        emit(&mut dec, &mut n)?;
    }
    let el = (sys::monotonic() - t0).as_secs_f64();
    log(format!("framemd5: {n} frames in {el:.2}s ({:.1} fps incl. fetch)", n as f64 / el));
    Ok(())
}

/// Shared playback control, driven by the remote.
struct Control {
    quit: AtomicBool,
    paused: AtomicBool,
    /// Bumped on every seek; video and audio restart when it changes.
    epoch: AtomicU64,
    /// Media time (µs) the current epoch starts from.
    seek_us: AtomicI64,
    /// Show the overlay until this monotonic time (ns); 0 = hidden.
    overlay_until_ns: AtomicU64,
    /// Short note for the overlay, e.g. "+10 s".
    note: std::sync::Mutex<String>,
}

impl Control {
    fn epoch(&self) -> u64 {
        self.epoch.load(Ordering::Acquire)
    }
    fn seek_target(&self) -> f64 {
        self.seek_us.load(Ordering::Acquire) as f64 / 1e6
    }
    fn seek(&self, to: f64) {
        self.seek_us.store((to.max(0.0) * 1e6) as i64, Ordering::Release);
        self.epoch.fetch_add(1, Ordering::AcqRel);
    }
    fn show_overlay(&self, secs: f64, note: &str) {
        *self.note.lock().unwrap() = note.to_owned();
        let until = sys::monotonic().as_nanos() as u64 + (secs * 1e9) as u64;
        self.overlay_until_ns.store(until, Ordering::Release);
    }
    fn overlay_visible(&self) -> bool {
        self.paused.load(Ordering::Acquire)
            || sys::monotonic().as_nanos() as u64 <= self.overlay_until_ns.load(Ordering::Acquire)
    }
}

/// Audio clock in media time: where playback is, in seconds from the start.
/// `base` is where the current run of audio began (after a seek or resume);
/// `played` is frames actually heard since then.
struct Clock {
    base_us: AtomicI64,
    played: AtomicI64,
    at_ns: AtomicU64,
    rate: AtomicU64,
    started: AtomicBool,
    frozen: AtomicBool,
    done: AtomicBool,
}

impl Clock {
    fn new() -> Self {
        Clock { base_us: AtomicI64::new(0), played: AtomicI64::new(0), at_ns: AtomicU64::new(0),
                rate: AtomicU64::new(0), started: AtomicBool::new(false),
                frozen: AtomicBool::new(false), done: AtomicBool::new(false) }
    }
    fn seconds(&self) -> f64 {
        let base = self.base_us.load(Ordering::Acquire) as f64 / 1e6;
        let rate = self.rate.load(Ordering::Acquire) as f64;
        if !self.started.load(Ordering::Acquire) || rate == 0.0 {
            return base;
        }
        let played = self.played.load(Ordering::Acquire) as f64;
        let still = self.frozen.load(Ordering::Acquire) || self.done.load(Ordering::Acquire);
        let since = sys::monotonic().as_nanos() as f64 - self.at_ns.load(Ordering::Acquire) as f64;
        base + played / rate + if still { 0.0 } else { since.max(0.0) / 1e9 }
    }
    fn running(&self) -> bool {
        self.started.load(Ordering::Acquire) && !self.frozen.load(Ordering::Acquire)
    }
    /// Start a new run of audio at media time `base` (not started until audio is written).
    fn restart(&self, base: f64) {
        self.started.store(false, Ordering::Release);
        self.frozen.store(false, Ordering::Release);
        self.done.store(false, Ordering::Release);
        self.played.store(0, Ordering::Release);
        self.base_us.store((base * 1e6) as i64, Ordering::Release);
    }
    /// Hold still at media time `at` (paused).
    fn freeze(&self, at: f64) {
        self.base_us.store((at * 1e6) as i64, Ordering::Release);
        self.played.store(0, Ordering::Release);
        self.frozen.store(true, Ordering::Release);
        self.started.store(true, Ordering::Release);
    }
}

/// Linear-interpolation resampler for interleaved i16 (stand-in until we need better).
struct Resampler {
    step: f64,
    pos: f64,
    prev: Vec<i16>,
}

impl Resampler {
    fn process(&mut self, input: &[i16], ch: usize, out: &mut Vec<i16>) {
        let n = input.len() / ch;
        let at = |i: usize, c: usize| -> f64 {
            if i == 0 { self.prev[c] as f64 } else { input[(i - 1) * ch + c] as f64 }
        };
        while self.pos < n as f64 {
            let i = self.pos as usize;
            let frac = self.pos - i as f64;
            for c in 0..ch {
                let a = at(i, c);
                let b = at(i + 1, c);
                out.push((a + (b - a) * frac) as i16);
            }
            self.pos += self.step;
        }
        self.pos -= n as f64;
        if n > 0 {
            self.prev.copy_from_slice(&input[(n - 1) * ch..n * ch]);
        }
    }
}

/// Index of the segment that contains media time `t` (segments start on keyframes).
fn segment_at(track: &Track, t: f64) -> usize {
    let ts = track.index.timescale.max(1) as f64;
    let off = track.init.edit_offset as f64 / track.init.timescale.max(1) as f64;
    track.index.segments.iter().rposition(|s| s.start as f64 / ts - off <= t + 1e-6).unwrap_or(0)
}

fn audio_thread(track: Track, clock: Arc<Clock>, ctl: Arc<Control>) -> io::Result<String> {
    let (asc, rate, ch) = match &track.init.codec {
        mp4::Codec::Aac { asc, sample_rate, channels, .. } => (asc.clone(), *sample_rate, *channels),
        c => return Err(io::Error::other(format!("not AAC: {c:?}"))),
    };
    let mut pcm = match alsa::Pcm::open("/dev/snd/pcmC0D0p", rate, ch as u32) {
        Ok(p) => p,
        Err(_) => alsa::Pcm::open("/dev/snd/pcmC0D0p", 48000, ch as u32)?,
    };
    log(format!("audio: AAC {rate} Hz x{ch} -> ALSA {} Hz, period {} buffer {}",
                pcm.rate, pcm.period_frames, pcm.buffer_frames));
    clock.rate.store(pcm.rate as u64, Ordering::Release);
    let chn = ch as usize;
    let frame_s = 1024.0 / rate as f64;
    let mut total: i64 = 0;
    let mut out = Vec::new();
    // Each pass plays from `start` until the end, a seek, a pause, or quit.
    let mut start = 0.0f64;
    let mut epoch = ctl.epoch();
    'passes: loop {
        let mut dec = Aac::new(&asc, rate, ch)?;
        let mut rs = (pcm.rate != rate).then(|| Resampler {
            step: rate as f64 / pcm.rate as f64, pos: 0.0, prev: vec![0; chn] });
        clock.restart(start);
        let mut written: i64 = 0;
        let mut interrupted = false;
        'segments: for i in segment_at(&track, start)..track.index.segments.len() {
            for s in track.fetch(i)? {
                if ctl.quit.load(Ordering::Relaxed) {
                    break 'passes;
                }
                if ctl.epoch() != epoch || ctl.paused.load(Ordering::Relaxed) {
                    interrupted = true;
                    break 'segments;
                }
                let t = track.seconds(s.pts);
                let pcm_in = dec.decode(&s.data, s.dts)?;
                if t + frame_s <= start {
                    continue; // before the seek/resume point
                }
                let block: &[i16] = match rs.as_mut() {
                    Some(r) => {
                        out.clear();
                        r.process(pcm_in, chn, &mut out);
                        &out
                    }
                    None => pcm_in,
                };
                if let Err(e) = pcm.write(block) {
                    // A stuck stream (EIO after the driver's timeout): start the
                    // device afresh and carry on rather than ending playback.
                    log(format!("audio: write failed ({e}); reopening the device"));
                    pcm.reset()?;
                    clock.restart(t);
                    written = 0;
                    continue;
                }
                written += (block.len() / chn) as i64;
                total += (block.len() / chn) as i64;
                let delay = pcm.delay().unwrap_or(0);
                clock.played.store((written - delay).max(0), Ordering::Release);
                clock.at_ns.store(sys::monotonic().as_nanos() as u64, Ordering::Release);
                clock.started.store(true, Ordering::Release);
            }
        }
        if !interrupted {
            let _ = pcm.drain();
            clock.played.store(written, Ordering::Release);
            clock.done.store(true, Ordering::Release);
        }
        if !interrupted {
            break 'passes; // played to the end; video stops when it sees `done`
        }
        // Interrupted: by a seek, a pause, or quit.
        loop {
            if ctl.quit.load(Ordering::Relaxed) {
                break 'passes;
            }
            if ctl.epoch() != epoch {
                pcm.reset()?;
                epoch = ctl.epoch();
                start = ctl.seek_target();
                continue 'passes;
            }
            if !ctl.paused.load(Ordering::Relaxed) {
                // Resume exactly where we stopped (video was waiting on the clock).
                start = clock.seconds();
                continue 'passes;
            }
            if !clock.frozen.load(Ordering::Relaxed) {
                let at = clock.seconds();
                pcm.reset()?; // silence now; queued audio is replayed from `at` on resume
                clock.freeze(at);
            }
            thread::sleep(Duration::from_millis(20));
        }
    }
    clock.done.store(true, Ordering::Release);
    Ok(format!("audio: {total} frames ({:.1}s), {} underruns", total as f64 / pcm.rate as f64,
               pcm.underruns))
}

#[derive(Default)]
struct Stats {
    shown: u64,
    dropped: u64,
    skipped: u64,
    decode_ns: u64,
    draw_ns: u64,
    offset_sum: f64,
    offset_max: f64,
    seeks: u64,
}

enum Shown {
    Yes,
    No,
    Abort,
}

/// Whether the frame at `pts` may be shown: the audio clock has reached it, or
/// audio has ended. The ended clock stops at the last sample and the video
/// track can run a few ms longer, so waiting for it would freeze the picture.
fn frame_due(pts: f64, audio_clock: f64, running: bool, audio_done: bool) -> bool {
    audio_done || (running && audio_clock + 0.002 >= pts)
}

fn mmss(t: f64) -> String {
    let t = t.max(0.0) as u64;
    format!("{}:{:02}", t / 60, t % 60)
}

/// 8x8 bitmap font at integer scale.
fn text(bb: &mut fb::BackBuffer, x: usize, y: usize, scale: usize, color: u32, s: &str) {
    use font8x8::legacy::BASIC_LEGACY;
    for (i, ch) in s.chars().enumerate() {
        let glyph = BASIC_LEGACY.get(ch as usize).copied().unwrap_or([0; 8]);
        for (row, bits) in glyph.iter().enumerate() {
            for col in 0..8 {
                if bits & (1 << col) != 0 {
                    bb.fill_rect(x + (i * 8 + col) * scale, y + row * scale, scale, scale, color);
                }
            }
        }
    }
}

struct Palette {
    bar: u32,
    white: u32,
    grey: u32,
    red: u32,
    track: u32,
}

/// The playback bar: title, paused marker, progress, elapsed/total, note.
fn draw_overlay(bb: &mut fb::BackBuffer, p: &Palette, title: &str, pos: f64, dur: f64,
                paused: bool, note: &str) {
    let (w, h) = (bb.width, bb.height);
    let (x0, y0, bw, bh) = (40usize, h.saturating_sub(130), w - 80, 100usize);
    bb.fill_rect(x0, y0, bw, bh, p.bar);
    bb.fill_rect(x0, y0, 6, bh, p.red);
    let max_chars = (bw - 340) / 16;
    let t: String = title.chars().take(max_chars).collect();
    text(bb, x0 + 26, y0 + 16, 2, p.white, &t);
    if paused {
        text(bb, x0 + bw - 150, y0 + 16, 2, p.red, "PAUSED");
    } else if !note.is_empty() {
        text(bb, x0 + bw - 150, y0 + 16, 2, p.white, note);
    }
    let (px, py, pw) = (x0 + 26, y0 + 60, bw - 260);
    bb.fill_rect(px, py, pw, 8, p.track);
    let frac = if dur > 0.0 { (pos / dur).clamp(0.0, 1.0) } else { 0.0 };
    bb.fill_rect(px, py, (pw as f64 * frac) as usize, 8, p.red);
    let knob = px + (pw as f64 * frac) as usize;
    bb.fill_rect(knob.saturating_sub(6), py - 6, 12, 20, p.white);
    text(bb, px + pw + 20, py - 4, 2, p.grey, &format!("{} / {}", mmss(pos), mmss(dur)));
}

/// Play `video` + `audio` until the end, `limit` frames, or Back. The remote
/// controls playback: OK / Play-Pause pause, Left/Right and Rewind/FF seek
/// 10 s, digits jump to 0-90%, Info/Up/Down show the bar.
pub fn play(video: Track, audio: Track, title: &str, duration: f64, limit: usize, threads: u32)
            -> io::Result<()> {
    if std::env::var_os("TV_FB_PATH").is_none() && !std::path::Path::new("/dev/fb0").exists() {
        thread::sleep(Duration::from_millis(500));
    }
    let mut screen = platform::display::Display::open_default()?;
    let v = screen.var;
    let alpha = if v.transp.length > 0 { ((1u32 << v.transp.length) - 1) << v.transp.offset } else { 0 };
    let mut conv = yuv::Converter::new((v.red.offset, v.green.offset, v.blue.offset), alpha);
    let pal = Palette { bar: screen.pixel(14, 16, 28), white: screen.pixel(240, 240, 245),
                        grey: screen.pixel(160, 164, 180), red: screen.pixel(230, 40, 50),
                        track: screen.pixel(70, 72, 86) };
    let avcc = avc_config(&video)?;
    let fps = video.fmt.fps.max(1) as f64;

    let clock = Arc::new(Clock::new());
    let ctl = Arc::new(Control { quit: AtomicBool::new(false), paused: AtomicBool::new(false),
                                 epoch: AtomicU64::new(0), seek_us: AtomicI64::new(0),
                                 overlay_until_ns: AtomicU64::new(0),
                                 note: std::sync::Mutex::new(String::new()) });
    let aud = { let (c, k) = (clock.clone(), ctl.clone()); thread::spawn(move || {
        let result = audio_thread(audio, c.clone(), k.clone());
        if result.is_err() {
            // Missing/failed audio is expected during hardware bring-up. Wake
            // video immediately instead of waiting forever for its clock.
            c.done.store(true, Ordering::Release);
            k.quit.store(true, Ordering::Release);
        }
        result
    }) };
    let finished = Arc::new(AtomicBool::new(false));
    let keys = {
        let (c, k, f) = (clock.clone(), ctl.clone(), finished.clone());
        thread::spawn(move || remote_control(&c, &k, &f, duration))
    };

    let mut st = Stats::default();
    let mut n = 0usize;
    let t_start = sys::monotonic();
    let mut epoch = ctl.epoch();
    let mut target = 0.0f64;
    let mut overlay_drawn = false;

    // Wait for the clock to reach `pts`, keeping the pause overlay fresh.
    let present = |f: &crate::decode::Frame, epoch: u64, st: &mut Stats,
                   screen: &mut fb::Framebuffer, conv: &mut yuv::Converter,
                   overlay_drawn: &mut bool, fresh: &mut bool| -> io::Result<Shown> {
        let pts = video.seconds(f.pts);
        let t0 = sys::monotonic();
        let mut last_paint = Duration::ZERO;
        loop {
            if ctl.quit.load(Ordering::Relaxed) || ctl.epoch() != epoch {
                return Ok(Shown::Abort);
            }
            let paused = ctl.paused.load(Ordering::Relaxed);
            if paused && *fresh {
                // First frame after a seek while paused: show where we landed.
                let note = ctl.note.lock().unwrap().clone();
                let mut frame = screen.acquire()?;
                let mut bb = frame.pixels();
                conv.draw(f, &mut bb);
                draw_overlay(&mut bb, &pal, title, pts, duration, true, &note);
                drop(bb);
                frame.present()?;
                *fresh = false;
                *overlay_drawn = true;
                last_paint = sys::monotonic();
            }
            if paused {
                // Repaint the bar on the visible frame now and then (time, PAUSED).
                if sys::monotonic() - last_paint > Duration::from_millis(250) {
                    let note = ctl.note.lock().unwrap().clone();
                    let mut frame = screen.acquire()?;
                    let mut pixels = frame.pixels();
                    conv.draw(f, &mut pixels);
                    draw_overlay(&mut pixels, &pal, title, clock.seconds(), duration, true, &note);
                    drop(pixels);
                    frame.present()?;
                    last_paint = sys::monotonic();
                    *overlay_drawn = true;
                }
                thread::sleep(Duration::from_millis(20));
                continue;
            }
            let a = clock.seconds();
            if frame_due(pts, a, clock.running(), clock.done.load(Ordering::Acquire)) {
                break;
            }
            // Startup / after a seek: don't wait forever if audio never starts.
            if !clock.running() && sys::monotonic() - t0 > Duration::from_secs(3) {
                break;
            }
            let wait = if clock.running() { (pts - a).clamp(0.001, 0.010) } else { 0.005 };
            thread::sleep(Duration::from_secs_f64(wait));
        }
        let off = clock.seconds() - pts;
        if off > 1.0 / fps && !ctl.overlay_visible() {
            st.dropped += 1; // late by more than a frame: skip drawing
            return Ok(Shown::No);
        }
        let t1 = sys::monotonic();
        let mut frame = screen.acquire()?;
                let mut bb = frame.pixels();
        conv.draw(f, &mut bb);
        if ctl.overlay_visible() {
            let note = ctl.note.lock().unwrap().clone();
            draw_overlay(&mut bb, &pal, title, pts, duration, false, &note);
            *overlay_drawn = true;
        } else if *overlay_drawn {
            *overlay_drawn = false; // the next full frame paints over the old bar
        }
        drop(bb);
        frame.present()?;
        *fresh = false;
        st.draw_ns += (sys::monotonic() - t1).as_nanos() as u64;
        st.shown += 1;
        st.offset_sum += off;
        st.offset_max = st.offset_max.max(off.abs());
        Ok(Shown::Yes)
    };

    'epochs: loop {
        let mut dec = H264::new(&avcc, threads)?;
        let mut aborted = false;
        let mut fresh = true;
        'segments: for i in segment_at(&video, target)..video.index.segments.len() {
            for s in video.fetch(i)? {
                if ctl.quit.load(Ordering::Relaxed) || ctl.epoch() != epoch {
                    aborted = true;
                    break 'segments;
                }
                let t0 = sys::monotonic();
                while !dec.send(&s.data, s.pts, s.dts as i64)? {
                    while let Some(f) = dec.receive()? {
                        if video.seconds(f.pts) + 1e-3 < target {
                            st.skipped += 1;
                            continue;
                        }
                        if let Shown::Abort = present(&f, epoch, &mut st, &mut screen, &mut conv, &mut overlay_drawn, &mut fresh)? {
                            aborted = true;
                            break 'segments;
                        }
                        n += 1;
                    }
                }
                st.decode_ns += (sys::monotonic() - t0).as_nanos() as u64;
                while let Some(f) = dec.receive()? {
                    if video.seconds(f.pts) + 1e-3 < target {
                        st.skipped += 1;
                        continue;
                    }
                    if let Shown::Abort = present(&f, epoch, &mut st, &mut screen, &mut conv, &mut overlay_drawn, &mut fresh)? {
                        aborted = true;
                        break 'segments;
                    }
                    n += 1;
                }
                if n >= limit {
                    ctl.quit.store(true, Ordering::Relaxed);
                    break 'epochs;
                }
            }
        }
        if !aborted {
            dec.flush()?;
            while let Some(f) = dec.receive()? {
                if let Shown::Abort = present(&f, epoch, &mut st, &mut screen, &mut conv, &mut overlay_drawn, &mut fresh)? {
                    break;
                }
            }
        }
        // End of video, a seek, or quit.
        loop {
            if ctl.quit.load(Ordering::Relaxed) {
                break 'epochs;
            }
            if ctl.epoch() != epoch {
                epoch = ctl.epoch();
                target = ctl.seek_target();
                st.seeks += 1;
                continue 'epochs;
            }
            if clock.done.load(Ordering::Relaxed) {
                break 'epochs; // audio finished too
            }
            thread::sleep(Duration::from_millis(20));
        }
    }
    let quitting = ctl.quit.load(Ordering::Relaxed) && n < limit;
    ctl.quit.store(true, Ordering::Relaxed);
    let audio_result = aud.join().map_err(|_| io::Error::other("audio thread panicked"));
    finished.store(true, Ordering::Relaxed);
    let _ = keys.join();
    let audio_report = audio_result??;
    if quitting {
        log("video: stopped by remote (Back/Exit/Stop/Home)");
    }
    let el = (sys::monotonic() - t_start).as_secs_f64();
    let frames = (st.shown + st.dropped).max(1);
    log(format!("video: {} shown, {} dropped in {el:.1}s; send+decode {:.2} ms/frame, draw {:.2} ms/frame",
                st.shown, st.dropped, st.decode_ns as f64 / 1e6 / frames as f64,
                st.draw_ns as f64 / 1e6 / st.shown.max(1) as f64));
    log(format!("sync: frame time minus audio clock: mean {:+.1} ms, max |{:.1}| ms",
                st.offset_sum * 1e3 / st.shown.max(1) as f64, st.offset_max * 1e3));
    if st.seeks > 0 {
        log(format!("seek: {} seeks, {} frames decoded but skipped to reach targets", st.seeks, st.skipped));
    }
    log(audio_report);
    Ok(())
}

/// Remote control during playback. Back or Exit ends playback (Home is
/// handled by the launcher, which stops the app; on our own, Home quits too).
/// The TCL remote has no transport keys: OK pauses and resumes, Left/Right
/// seek 10 s, Channel -/+ seek a minute.
fn remote_control(clock: &Clock, ctl: &Control, finished: &AtomicBool, duration: f64) {
    use platform::remote::{poll, Action};
    let mut devs = crate::input::open_all();
    while !finished.load(Ordering::Relaxed) && !ctl.quit.load(Ordering::Relaxed) {
        for (a, repeat) in poll(&mut devs) {
            let pos = clock.seconds();
            let seek = |to: f64, note: &str| {
                let to = to.clamp(0.0, (duration - 1.0).max(0.0));
                ctl.seek(to);
                ctl.show_overlay(3.0, note);
                log(format!("seek to {}", mmss(to)));
            };
            match a {
                Action::Back | Action::Exit | Action::Home if !repeat => {
                    ctl.quit.store(true, Ordering::Relaxed);
                }
                Action::Ok | Action::PlayPause if !repeat => {
                    let p = !ctl.paused.load(Ordering::Relaxed);
                    ctl.paused.store(p, Ordering::Relaxed);
                    ctl.show_overlay(3.0, "");
                    log(if p { "paused" } else { "resumed" });
                }
                Action::Right => seek(pos + 10.0, "+10 s"),
                Action::Left => seek(pos - 10.0, "-10 s"),
                Action::ChannelUp if !repeat => seek(pos + 60.0, "+60 s"),
                Action::ChannelDown if !repeat => seek(pos - 60.0, "-60 s"),
                Action::Digit(d) if !repeat => seek(duration * d as f64 / 10.0, &format!("{}0%", d)),
                Action::Info | Action::Up | Action::Down => ctl.show_overlay(4.0, ""),
                _ => {}
            }
        }
        thread::sleep(Duration::from_millis(20));
    }
}

/// Decode-speed benchmark for the real TV (runs from `adb shell`, no fb/ALSA
/// needed): loads a local MP4 into memory, then times decoding at each thread
/// count and YUV->RGB conversion into an off-screen 1280x720 buffer.
pub fn bench(file: &str, limit: usize, threads_list: &str) -> io::Result<()> {
    let bytes = std::fs::read(file).map_err(|e| io::Error::other(format!("{file}: {e}")))?;
    let init = mp4::parse_init(&bytes).map_err(io::Error::other)?;
    let index = mp4::parse_sidx(&bytes, 0).map_err(io::Error::other)?;
    let mut samples = Vec::new();
    for s in &index.segments {
        let frag = bytes.get(s.offset as usize..(s.offset + s.size as u64) as usize)
            .ok_or_else(|| io::Error::other("segment outside file"))?;
        samples.extend(mp4::parse_fragments(frag, &init).map_err(io::Error::other)?);
        if samples.len() >= limit {
            break;
        }
    }
    samples.truncate(limit);
    let avcc = match &init.codec {
        mp4::Codec::Avc { avcc, width, height, profile, level, .. } => {
            log(format!("bench: {file}: H.264 {width}x{height} profile {profile} level {level}, {} frames",
                        samples.len()));
            avcc.clone()
        }
        c => return Err(io::Error::other(format!("not H.264: {c:?}"))),
    };
    let cpus = thread::available_parallelism().map_or(1, |n| n.get());
    // One pass per configuration; `convert` adds YUV->RGB into an off-screen buffer.
    let run = |threads: u32, convert: bool| -> io::Result<(u64, f64, f64)> {
        let mut dec = H264::new(&avcc, threads)?;
        let mut conv = yuv::Converter::new((16, 8, 0), 0);
        let mut screen = vec![0u32; 1280 * 720];
        let (mut n, mut conv_ns) = (0u64, 0u64);
        let t0 = sys::monotonic();
        let mut drain = |dec: &mut H264, n: &mut u64, conv_ns: &mut u64| -> io::Result<()> {
            while let Some(f) = dec.receive()? {
                if convert {
                    let c0 = sys::monotonic();
                    let mut bb = fb::BackBuffer { px: &mut screen, stride: 1280, width: 1280, height: 720 };
                    conv.draw(&f, &mut bb);
                    *conv_ns += (sys::monotonic() - c0).as_nanos() as u64;
                }
                *n += 1;
            }
            Ok(())
        };
        for s in &samples {
            while !dec.send(&s.data, s.pts, s.dts as i64)? {
                drain(&mut dec, &mut n, &mut conv_ns)?;
            }
            drain(&mut dec, &mut n, &mut conv_ns)?;
        }
        dec.flush()?;
        drain(&mut dec, &mut n, &mut conv_ns)?;
        Ok((n, (sys::monotonic() - t0).as_secs_f64(), conv_ns as f64 / 1e9))
    };
    let (mut best, mut best_fps) = (0u32, 0.0);
    for threads in threads_list.split(',').filter_map(|t| t.trim().parse::<u32>().ok()) {
        let (n, total, _) = run(threads, false)?;
        log(format!("bench: decode only, threads={threads} (of {cpus} CPUs): {n} frames, {:.1} fps ({:.2} ms/frame)",
                    n as f64 / total, total * 1e3 / n.max(1) as f64));
        let fps = n as f64 / total;
        if fps > best_fps { (best, best_fps) = (threads, fps); }
    }
    let (n, total, _) = run(best, true)?;
    log(format!("bench: decode+convert, decoder threads={best}: {:.1} fps end to end", n as f64 / total));

    // Conversion alone: copy 48 decoded frames, then time YUV->RGB on 1..N threads.
    let mut dec = H264::new(&avcc, best)?;
    type Picture = (Vec<u8>, Vec<u8>, Vec<u8>, usize, usize, usize, usize);
    let mut frames: Vec<Picture> = Vec::new();
    let collect = |dec: &mut H264, frames: &mut Vec<Picture>| -> io::Result<()> {
        while let Some(f) = dec.receive()? {
            frames.push((f.y.to_vec(), f.u.to_vec(), f.v.to_vec(), f.y_stride, f.c_stride, f.width, f.height));
        }
        Ok(())
    };
    for s in &samples {
        while !dec.send(&s.data, s.pts, s.dts as i64)? {
            collect(&mut dec, &mut frames)?;
        }
        collect(&mut dec, &mut frames)?;
        if frames.len() >= 48 { break; }
    }
    // Frame-threading and B-frame reordering can buffer the entire short clip.
    dec.flush()?;
    collect(&mut dec, &mut frames)?;
    if frames.is_empty() {
        return Err(io::Error::other("no decoded frames for conversion benchmark"));
    }
    let mut screen = vec![0u32; 1280 * 720];
    let mut threads_set: Vec<usize> = vec![1, 2, cpus];
    threads_set.dedup();
    for t in threads_set {
        let mut conv = yuv::Converter::with_threads((16, 8, 0), 0, t);
        let t0 = sys::monotonic();
        let rounds = 3;
        for _ in 0..rounds {
            for (y, u, v, ys, cs, w, h) in &frames {
                let f = crate::decode::Frame { y, u, v, y_stride: *ys, c_stride: *cs, width: *w, height: *h,
                                               pts: 0, keyframe: false };
                let mut bb = fb::BackBuffer { px: &mut screen, stride: 1280, width: 1280, height: 720 };
                conv.draw(&f, &mut bb);
            }
        }
        let per = (sys::monotonic() - t0).as_secs_f64() * 1e3 / (rounds * frames.len()) as f64;
        log(format!("bench: convert {}x{} -> 1280x720 on {t} thread(s): {per:.2} ms/frame",
                    frames[0].5, frames[0].6));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::frame_due;

    #[test]
    fn last_frame_after_audio_end_is_shown() {
        // The demo's audio ends at 16.024 s and its last frame is at 16.042 s.
        assert!(frame_due(16.042, 16.024, true, true));
        assert!(!frame_due(16.042, 16.024, true, false));
        assert!(frame_due(16.042, 16.041, true, false));
        assert!(!frame_due(1.0, 5.0, false, false));
    }
}
