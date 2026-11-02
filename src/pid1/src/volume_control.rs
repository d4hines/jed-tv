//! Linux effects for the Lean-owned global volume service.
use crate::{volume, State};
use std::{cell::RefCell, collections::VecDeque, fs, path::Path, sync::Arc,
    time::{Duration, Instant}};
use tvinit::lean_runtime::{self, Object};

pub fn read_gain() -> f32 {
    fs::read_to_string("/tmp/app-volume").ok().and_then(|s| s.trim().parse::<f32>().ok())
        .filter(|n| n.is_finite()).unwrap_or(volume::level::DEFAULT_GAIN)
        .clamp(0.0, volume::level::MAX_GAIN)
}

struct Context {
    state: Arc<State>,
    devices: Vec<platform::input::Device>,
    pending: VecDeque<platform::input::Key>,
    observed_gain: Option<u32>,
    lean_state: Option<u64>,
    last_poll: Option<Instant>,
}
thread_local! { static CONTEXT: RefCell<Option<Context>> = const { RefCell::new(None) }; }
extern "C" {
    fn tv_volume_start() -> Object;
    fn tv_volume_tick(state: u64) -> Object;
    fn tv_core_io_word(value: u64) -> Object;
    fn tv_core_io_unit(unused: u32) -> Object;
}
fn context<R>(f: impl FnOnce(&mut Context) -> R) -> R {
    CONTEXT.with(|slot| f(slot.borrow_mut().as_mut().expect("volume effect outside service")))
}
fn gain_units() -> u32 { (read_gain() * 1_000_000.0).round() as u32 }
fn gain_value(units: u32) -> f32 { units as f32 / 1_000_000.0 }

pub fn start(state: Arc<State>) {
    let devices = platform::input::open_all();
    CONTEXT.with(|slot| *slot.borrow_mut() = Some(Context {
        state: state.clone(), devices, pending: VecDeque::new(), observed_gain: None,
        lean_state: None, last_poll: None,
    }));
    match unsafe { lean_runtime::consume_io_u64(tv_volume_start()) } {
        Ok(word) if word != u64::MAX => context(|ctx| ctx.lean_state = Some(word)),
        result => {
            state.event(format!("Lean volume initialization failed: {result:?}"));
            let _ = fs::remove_file("/tmp/pid1-volume.ready");
            CONTEXT.with(|slot| { slot.borrow_mut().take(); });
        }
    }
}

/// Service one event on PID 1's Lean thread. Taking the scalar state releases
/// the RefCell borrow before Lean calls back into the native effect handlers.
pub fn tick() {
    let word = CONTEXT.with(|slot| {
        let mut slot = slot.borrow_mut();
        let ctx = slot.as_mut()?;
        if ctx.last_poll.is_some_and(|last| last.elapsed() < Duration::from_millis(20)) {
            return None;
        }
        ctx.last_poll = Some(Instant::now());
        ctx.lean_state.take()
    });
    let Some(word) = word else { return; };
    match unsafe { lean_runtime::consume_io_u64(tv_volume_tick(word)) } {
        Ok(next) => context(|ctx| ctx.lean_state = Some(next)),
        Err(error) => {
            context(|ctx| ctx.state.event(format!("Lean volume service failed: {error}")));
            let _ = fs::remove_file("/tmp/pid1-volume.ready");
            CONTEXT.with(|slot| { slot.borrow_mut().take(); });
        }
    }
}

#[no_mangle]
extern "C" fn tv_volume_initial() -> Object {
    unsafe { tv_core_io_word(gain_units() as u64) }
}
#[no_mangle]
extern "C" fn tv_volume_ready() -> Object {
    let ready = context(|ctx| {
        if !Path::new("/tmp/app-volume").exists() {
            if let Err(error) = fs::write("/tmp/app-volume", format!("{}\n", read_gain())) {
                ctx.state.event(format!("volume: {error}")); return 0;
            }
        }
        if let Err(error) = fs::write("/tmp/pid1-volume.ready", "1\n") {
            ctx.state.event(format!("volume: {error}")); return 0;
        }
        ctx.state.event(format!("volume: Lean owns global keys and overlay ({} inputs)", ctx.devices.len()));
        1
    });
    unsafe { tv_core_io_word(ready) }
}
#[no_mangle]
extern "C" fn tv_volume_poll() -> Object {
    let word = context(|ctx| {
        // Compare the value, not mtime: a fast external write can share the
        // previous timestamp, especially on the TV's tmpfs or under QEMU.
        let current = gain_units();
        if Some(current) != ctx.observed_gain {
            ctx.observed_gain = Some(current);
            return (current as u64) << 32 | 1;
        }
        if ctx.pending.is_empty() {
            for device in &mut ctx.devices {
                ctx.pending.extend(device.poll_keys().unwrap_or_default());
            }
        }
        if let Some(key) = ctx.pending.pop_front() {
            return (key.code as u64) << 32 | ((key.value as u32 as u64) & 0xffffff) << 8 | 2;
        }
        if ctx.devices.is_empty() { ctx.devices = platform::input::open_all(); }
        0
    });
    unsafe { tv_core_io_word(word) }
}
#[no_mangle]
extern "C" fn tv_volume_commit(units: u32) -> Object {
    let committed = context(|ctx| {
        let gain = gain_value(units);
        match fs::write("/tmp/.pid1-volume", format!("{gain}\n"))
            .and_then(|_| fs::rename("/tmp/.pid1-volume", "/tmp/app-volume")) {
            Ok(()) => {
                ctx.observed_gain = Some(units);
                ctx.state.event(format!("volume: Lean committed gain={gain:.4}"));
                1
            }
            Err(error) => { ctx.state.event(format!("volume write: {error}")); 0 }
        }
    });
    unsafe { tv_core_io_word(committed) }
}
#[no_mangle]
extern "C" fn tv_volume_publish(units: u32) -> Object {
    context(|ctx| {
        if let Err(error) = volume::publish(gain_value(units)) {
            ctx.state.event(format!("volume overlay: {error}"));
        }
    });
    unsafe { tv_core_io_unit(0) }
}
