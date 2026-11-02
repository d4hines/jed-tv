//! Linux operations invoked by the Lean IO program, not a second supervisor.
use super::{Arc, Ipv4Addr, State};
use std::sync::OnceLock;
use tvinit::lean_runtime::{Object as LeanObject, discard_io_result};
struct Runtime {
    state: Arc<State>,
    ip: Ipv4Addr,
    peer: Ipv4Addr,
    live_handoff: bool,
}
static RUNTIME: OnceLock<Runtime> = OnceLock::new();

extern "C" {
    fn initialize_TV_Pid1(builtin: u8) -> LeanObject;
    fn lean_io_mark_end_initialization();
    fn tv_pid1_main() -> LeanObject;
    fn tv_pid1_io_ok(value: u32) -> LeanObject;
    fn tv_pid1_io_unit(ignored: u32) -> LeanObject;
}

pub(super) fn run() -> ! {
    // Recovery HTTP is deliberately available before experimental runtime
    // initialization. These Rust threads never allocate Lean objects or run
    // boxed Lean code. State::new only constructs Rust control state.
    let (state, ip, peer, live_handoff) = match super::bootstrap() {
        Ok(ready) => ready,
        Err(()) => loop { std::thread::park(); },
    };
    let runtime = Runtime { state, ip, peer, live_handoff };
    assert!(RUNTIME.set(runtime).is_ok(), "PID1 recovery initialized twice");
    let state = &RUNTIME.get().unwrap().state;
    state.event("Lean runtime: entering module initialization; recovery HTTP remains available");
    let result = unsafe { discard_io_result(initialize_TV_Pid1(1)) };
    if result.is_err() {
        state.event("Lean runtime: module initialization failed; retaining recovery HTTP");
        loop { std::thread::park(); }
    }
    state.event("Lean runtime: module initialization complete");
    unsafe { tvinit::lean_runtime::adopt_initialized_runtime(); }
    unsafe { lean_io_mark_end_initialization(); }
    state.event("Lean runtime: entering IO supervisor");
    let result = unsafe { discard_io_result(tv_pid1_main()) };
    state.event(format!("Lean PID1 unexpectedly returned ({result:?}); retaining recovery HTTP"));
    loop { std::thread::park(); }
}

#[no_mangle]
extern "C" fn tv_pid1_stage(stage: u32) -> LeanObject {
    let status = match stage {
        0 => match RUNTIME.get() {
            Some(rt) => {
                rt.state.event("Lean IO PID1: recovery boundary ready");
                0
            }
            None => 1,
        },
        1 => match RUNTIME.get() {
            Some(rt) => match super::repl::start(rt.state.clone(), rt.ip, rt.peer, rt.live_handoff) {
                Ok(()) => 0,
                Err(error) => {
                    rt.state.event(format!("REPL startup failed: {error}"));
                    rt.state.phase("shell-failed");
                    1
                }
            },
            None => 1,
        },
        2 => match RUNTIME.get() {
            Some(rt) => {
                super::volume_control::start(rt.state.clone());
                if super::arg("hello.qemu", "0") == "1" {
                    let state = rt.state.clone();
                    std::thread::spawn(move || tvinit::run_surrogate_display(&state.launcher));
                }
                rt.state.event("Lean IO PID1: entering single-threaded Core executor");
                super::launcher::run(rt.state.clone());
            }
            None => 1,
        },
        _ => 1,
    };
    unsafe { tv_pid1_io_ok(status) }
}

#[no_mangle]
extern "C" fn tv_pid1_reap() -> LeanObject {
    let result = reap_one();
    unsafe { tv_pid1_io_ok(result) }
}

fn reap_one() -> u32 {
    let Some(rt) = RUNTIME.get() else { return 0; };
    let Ok(_guard) = rt.state.child_wait_lock.try_lock() else { return 0; };
    let mut status = 0;
    let pid = unsafe { libc::waitpid(-1, &mut status, libc::WNOHANG) };
    if pid > 0 {
        let mut foreground = rt.state.foreground_child.lock().unwrap();
        if let Some((child, result)) = foreground.as_mut() {
            if *child == pid { *result = Some(status); }
        }
        drop(foreground);
        rt.state.event(format!("Lean PID1 reaped child {pid} status={status}"));
        pid as u32
    } else if pid == 0 || matches!(std::io::Error::last_os_error().raw_os_error(), Some(libc::ECHILD | libc::EINTR)) {
        0
    } else {
        u32::MAX
    }
}

/// Called on every Core/idle poll. Reap recovery and foreground children even
/// when the launcher has been stopped or a previous Core run failed.
pub(super) fn tick() {
    loop {
        let result = reap_one();
        if result == 0 || result == u32::MAX { break; }
    }
    super::cold_plan::tick();
    super::volume_control::tick();
    super::audio_service::tick();
}

#[no_mangle]
extern "C" fn tv_pid1_sleep(milliseconds: u32) -> LeanObject {
    std::thread::sleep(std::time::Duration::from_millis(milliseconds.into()));
    unsafe { tv_pid1_io_unit(0) }
}

#[no_mangle]
extern "C" fn tv_pid1_notice(event: u32, value: u32) -> LeanObject {
    if let Some(rt) = RUNTIME.get() {
        match event {
            0 => rt.state.event(format!("Lean PID1 startup stopped at stage {value}; retaining recovery loop")),
            1 => rt.state.event("Lean PID1 waitpid failed; retrying on next tick"),
            _ => (),
        }
    } else {
        eprintln!("Lean PID1 event={event} value={value}");
    }
    unsafe { tv_pid1_io_unit(0) }
}
