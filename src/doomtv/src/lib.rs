//! doomtv: the original Doom engine (id Software's GPL source, via doomgeneric)
//! on the TV platform API. The engine is C (src/third_party/doomgeneric, built by tools/doom/build.sh); this crate is
//! only the glue: the DG_* platform hooks over `platform`, and a `Doom` handle
//! whose `tick()` the caller's (Rust) main loop drives, so a launcher can
//! interleave Doom with other apps and survive Doom quitting.
//!
//! Quitting never ends the process: the engine's exit() is renamed to
//! doomtv_exit (src/doomtv/native/doomtv_shim.c), which longjmps back out of
//! doomgeneric_Tick; `tick()` then returns `Some(Exit)`. The engine's globals
//! are not re-initialisable, so one process runs at most one Doom session; to
//! play again, re-exec the binary (PID 1 may exec itself and stays PID 1).

pub mod keys;
pub mod screen;
pub mod wad;

use platform::alsa::Pcm;
use platform::fb::Framebuffer;
use platform::sys::monotonic;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::ffi::{c_char, c_int, CStr, CString};
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Doom's frame (doomgeneric's DG_ScreenBuffer; tools/doom/build.sh sets these).
pub const RESX: usize = 320;
pub const RESY: usize = 200;

extern "C" {
    fn doomtv_create(argc: c_int, argv: *mut *mut c_char, code: *mut c_int) -> c_int;
    fn doomtv_tick(code: *mut c_int) -> c_int;
    fn doomtv_sound_mix(out: *mut i16, frames: c_int, rate: c_int);
    static DG_ScreenBuffer: *mut u32;
    static gametic: c_int;
    /// Set by -timedemo: run one tic per frame as fast as possible.
    static singletics: c_int;
    /// Engine key bindings (m_controls.c), read at event time. Unbound (0)
    /// or 'y' by default; `Doom::start` gives them keys the remote can send.
    static mut key_prevweapon: c_int;
    static mut key_nextweapon: c_int;
    static mut key_menu_confirm: c_int;
}

/// How the engine ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exit {
    /// exit(0): quit from the menu (I_Quit).
    Quit,
    /// Any other status, normally -1 from I_Error (the message is on stderr).
    Error(i32),
}

impl Exit {
    fn from_code(code: c_int) -> Exit {
        if code == 0 { Exit::Quit } else { Exit::Error(code) }
    }
}

/// Engine-side state the DG_* hooks reach. Everything runs on the thread that
/// calls `Doom::start`/`tick`, so a thread-local is enough.
struct Host {
    fb: Option<(Framebuffer, screen::Scaler)>,
    audio: Option<Audio>,
    keys: VecDeque<(bool, u8)>,
    frames: u64,
    blit: Duration,
    present: Duration,
    slept: Duration,
    /// The engine's time base: its first DG_GetTicksMs value (i_timer.c
    /// subtracts it from every later reading).
    base_ms: Option<u32>,
}

thread_local! {
    static HOST: RefCell<Host> = const { RefCell::new(Host {
        fb: None, audio: None, keys: VecDeque::new(), frames: 0,
        blit: Duration::ZERO, present: Duration::ZERO, slept: Duration::ZERO, base_ms: None,
    }) };
}

fn now_ms() -> u32 {
    monotonic().as_millis() as u32
}

/// The engine's current tic number, as I_GetTime computes it.
fn engine_tic(base: u32, ms: u32) -> u32 {
    (ms.wrapping_sub(base)).wrapping_mul(35) / 1000
}

/// When tic `n` begins, in DG_GetTicksMs milliseconds.
fn tic_start_ms(base: u32, n: u32) -> u32 {
    base.wrapping_add((n as u64 * 1000).div_ceil(35) as u32)
}

static STARTED: AtomicBool = AtomicBool::new(false);

/// Sound effects out: a thread keeps ~`AUDIO_QUEUE_MS` of freshly mixed audio
/// (doomtv_sound.c) queued in ALSA, independent of frame time, wipes and level
/// loads on the engine thread.
struct Audio {
    stop: Arc<AtomicBool>,
    underruns: Arc<AtomicU32>,
    thread: std::thread::JoinHandle<Pcm>,
}

pub const AUDIO_QUEUE_MS: u32 = 60;

fn audio_loop(mut pcm: Pcm, stop: Arc<AtomicBool>, underruns: Arc<AtomicU32>) -> Pcm {
    let rate = pcm.rate as i64;
    let period = pcm.period_frames as i64;
    let hi = pcm.buffer_frames as i64 - period;
    let target = (rate * AUDIO_QUEUE_MS as i64 / 1000).clamp((2 * period).min(hi), hi);
    let mut buf = Vec::new();
    while !stop.load(Ordering::Relaxed) {
        let queued = pcm.delay().unwrap_or(0).max(0); // EPIPE (underrun): empty
        if queued >= target {
            let ms = ((queued - target) * 1000 / rate).max(1);
            std::thread::sleep(Duration::from_millis(ms as u64));
            continue;
        }
        let n = ((target - queued + period - 1) / period * period).min(hi - queued).max(1);
        buf.resize(n as usize * 2, 0);
        unsafe { doomtv_sound_mix(buf.as_mut_ptr(), n as c_int, rate as c_int) };
        if let Err(e) = pcm.write(&buf) {
            eprintln!("doom: audio write failed ({e}); sound off");
            break;
        }
        underruns.store(pcm.underruns, Ordering::Relaxed);
    }
    pcm
}

/// Give Doom an opened S16 stereo PCM (platform::alsa) for its sound effects.
pub fn attach_audio(pcm: Pcm) {
    let stop = Arc::new(AtomicBool::new(false));
    let underruns = Arc::new(AtomicU32::new(0));
    let (s, u) = (stop.clone(), underruns.clone());
    match std::thread::Builder::new().name("doom-audio".into())
        .spawn(move || audio_loop(pcm, s, u))
    {
        Ok(thread) => HOST.with(|h| h.borrow_mut().audio = Some(Audio { stop, underruns, thread })),
        Err(e) => eprintln!("doom: audio thread: {e}"),
    }
}

/// Stop the audio thread and get the PCM back.
pub fn detach_audio() -> Option<Pcm> {
    let a = HOST.with(|h| h.borrow_mut().audio.take())?;
    a.stop.store(true, Ordering::Relaxed);
    a.thread.join().ok()
}

/// Give the display to Doom. Frames drawn while it has none are dropped, so a
/// launcher can take the screen back (`detach_display`) and return it later.
pub fn attach_display(fb: Framebuffer) {
    let sc = screen::Scaler::new(&fb);
    HOST.with(|h| h.borrow_mut().fb = Some((fb, sc)));
}

pub fn detach_display() -> Option<Framebuffer> {
    HOST.with(|h| h.borrow_mut().fb.take().map(|(fb, _)| fb))
}

/// Counters for frame/tic rate reports. Subtract two snapshots for a rate.
#[derive(Debug, Clone, Copy, Default)]
pub struct Stats {
    pub at: Duration,
    /// Frames shown (DG_DrawFrame calls; includes screen-wipe frames).
    pub frames: u64,
    /// Game logic tics run (Doom runs 35 per second of game time).
    pub tics: u64,
    /// Time spent scaling frames into the framebuffer's back page.
    pub blit: Duration,
    /// Time spent in present() (FBIOPAN_DISPLAY).
    pub present: Duration,
    /// Time the engine spent sleeping while waiting for the next tic.
    pub slept: Duration,
    /// ALSA underruns so far (audio queue ran dry).
    pub underruns: u32,
}

pub struct Doom {
    exited: Option<Exit>,
    /// Engine tic number when the last tick returned.
    last_tic: Option<u32>,
}

impl Doom {
    /// Run the engine's start-up (D_DoomMain) with Doom command-line `args`
    /// (without argv[0]), e.g. ["-iwad", "/doom/freedoom1.wad"]. Attach the display
    /// first. Returns Err if the engine exits during start-up (bad WAD...).
    pub fn start(args: &[String]) -> Result<Doom, Exit> {
        if STARTED.swap(true, Ordering::SeqCst) {
            // Doom's globals cannot be reset in-process.
            return Err(Exit::Error(-2));
        }
        // The engine keeps myargv for its whole life: leak it on purpose.
        let argv: Vec<*mut c_char> = std::iter::once("doomtv").chain(args.iter().map(|s| s.as_str()))
            .map(|a| CString::new(a).unwrap_or_default().into_raw())
            .chain(std::iter::once(std::ptr::null_mut()))
            .collect();
        let argv = Box::leak(argv.into_boxed_slice());
        let mut code = 0;
        let quit = unsafe { doomtv_create((argv.len() - 1) as c_int, argv.as_mut_ptr(), &mut code) };
        if quit != 0 {
            return Err(Exit::from_code(code));
        }
        // After M_LoadDefaults (inside D_DoomMain): the remote has no digits
        // for weapons and no 'y' for "are you sure?" prompts. keys.rs sends
        // Page Up/Down for NETFLIX / prime video, and OK is Enter.
        unsafe {
            key_prevweapon = keys::KEY_PGUP as c_int;
            key_nextweapon = keys::KEY_PGDN as c_int;
            key_menu_confirm = keys::KEY_ENTER as c_int;
        }
        Ok(Doom { exited: None, last_tic: None })
    }

    /// Feed one evdev key event (value 1 press, 2 repeat, 0 release).
    pub fn key(&mut self, code: u16, value: i32) {
        let down = value != 0;
        HOST.with(|h| {
            let mut h = h.borrow_mut();
            for &k in keys::doom_keys(code) {
                h.keys.push_back((down, k));
            }
        });
    }

    /// Run the engine once: at least one game tic (sleeping until it is due)
    /// and one frame. Returns Some(exit) once the engine has quit.
    ///
    /// Pacing: if no tic boundary has passed since the last tick returned, sleep
    /// until the next one first. Otherwise TryRunTics notices the boundary only
    /// after building the new tic and returns early without running it, so
    /// every tic would be drawn twice (upstream hides this behind vsync).
    pub fn tick(&mut self) -> Option<Exit> {
        if self.exited.is_some() {
            return self.exited;
        }
        let base = HOST.with(|h| h.borrow().base_ms);
        let timing_demo = unsafe { singletics } != 0;
        if let (Some(base), Some(last), false) = (base, self.last_tic, timing_demo) {
            if engine_tic(base, now_ms()) == last {
                let wait = tic_start_ms(base, last + 1).wrapping_sub(now_ms());
                if wait > 0 && wait <= 1000 / 35 + 1 {
                    DG_SleepMs(wait);
                }
            }
        }
        let mut code = 0;
        if unsafe { doomtv_tick(&mut code) } != 0 {
            self.exited = Some(Exit::from_code(code));
        }
        self.last_tic = base.or(HOST.with(|h| h.borrow().base_ms)).map(|b| engine_tic(b, now_ms()));
        self.exited
    }

    pub fn stats(&self) -> Stats {
        HOST.with(|h| {
            let h = h.borrow();
            Stats { at: monotonic(), frames: h.frames, tics: unsafe { gametic } as u64,
                    blit: h.blit, present: h.present, slept: h.slept,
                    underruns: h.audio.as_ref().map_or(0, |a| a.underruns.load(Ordering::Relaxed)) }
        })
    }
}

// ---- doomgeneric platform hooks (called by the engine, on the tick thread) ----

#[no_mangle]
pub extern "C" fn DG_Init() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tic_clock_matches_i_gettime() {
        let base = 1234;
        for n in 0..1000u32 {
            let start = tic_start_ms(base, n);
            assert_eq!(engine_tic(base, start), n, "tic {n} starts at {start}");
            if n > 0 {
                assert_eq!(engine_tic(base, start - 1), n - 1);
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn DG_DrawFrame() {
    let src = unsafe { DG_ScreenBuffer };
    if src.is_null() {
        return;
    }
    let src = unsafe { std::slice::from_raw_parts(src, RESX * RESY) };
    HOST.with(|h| {
        let mut h = h.borrow_mut();
        let h = &mut *h;
        h.frames += 1;
        if let Some((fb, sc)) = h.fb.as_mut() {
            let t0 = monotonic();
            let mut frame = fb.acquire().expect("acquire Doom frame");
            sc.blit(&mut frame, src, RESX, RESY);
            let t1 = monotonic();
            frame.present().expect("present Doom frame");
            h.blit += t1 - t0;
            h.present += monotonic() - t1;
        }
    });
}

#[no_mangle]
pub extern "C" fn DG_SleepMs(ms: u32) {
    let t0 = monotonic();
    std::thread::sleep(Duration::from_millis(ms as u64));
    HOST.with(|h| h.borrow_mut().slept += monotonic() - t0);
}

#[no_mangle]
pub extern "C" fn DG_GetTicksMs() -> u32 {
    // Uptime, never 0 by the time Doom asks (I_GetTime treats 0 as "unset").
    let ms = now_ms();
    HOST.with(|h| {
        h.borrow_mut().base_ms.get_or_insert(ms);
    });
    ms
}

#[no_mangle]
pub extern "C" fn DG_GetKey(pressed: *mut c_int, key: *mut u8) -> c_int {
    match HOST.with(|h| h.borrow_mut().keys.pop_front()) {
        Some((down, k)) => {
            unsafe {
                *pressed = down as c_int;
                *key = k;
            }
            1
        }
        None => 0,
    }
}

#[no_mangle]
pub extern "C" fn DG_SetWindowTitle(title: *const c_char) {
    if !title.is_null() {
        let t = unsafe { CStr::from_ptr(title) }.to_string_lossy();
        println!("doom: title \"{t}\"");
    }
}
