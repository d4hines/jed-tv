//! Direct services owned by PID1. Does not initialize vendor hardware.
use super::State;
use std::{fs, io, os::unix::ffi::OsStrExt, sync::Arc, thread, time::Duration};
#[path = "ownership.rs"]
mod ownership;

pub(super) fn verify_ownership() -> io::Result<()> {
    if !ownership::identity(std::process::id(), &fs::read("/proc/cmdline")?) {
        return Err(io::Error::other("direct services require G06 PID1"));
    }
    for entry in fs::read_dir("/proc")?.flatten() {
        if !ownership::numeric_name(entry.file_name().as_bytes()) { continue; }
        let Ok(name) = fs::read(entry.path().join("comm")) else { continue; };
        let owner = ownership::owner(&name)
            .ok_or_else(|| io::Error::other("invalid ASCII media owner comm record"))?;
        if owner == 0 { continue; }
        let status = fs::read(entry.path().join("status"))?;
        if !ownership::permitted(owner, ownership::stopped(&status)) {
            return Err(io::Error::other(format!(
                "media owner must be stopped first: {}", ownership::label(owner)
            )));
        }
    }
    Ok(())
}

pub(super) fn start_audio(state: Arc<State>) -> io::Result<()> {
    verify_ownership()?;
    super::audio_service::prepare(state)
}

pub(super) fn start_display(state: Arc<State>) -> io::Result<()> {
    verify_ownership()?;
    state.service("display", "waiting");
    thread::spawn(move || {
        let path = "/run/tv-core/menu.frames";
        state.event("PID1 direct display: waiting for /run/tv-core/menu.frames");
        while g06_display::frames::Frames::open(path).is_err() {
            thread::sleep(Duration::from_millis(50));
        }
        let mut display = match g06_display::service::Service::open(path) {
            Ok(display) => display,
            Err(error) => {
                state.service("display", "failed");
                state.event(format!("PID1 direct display setup failed: {error}"));
                return;
            }
        };
        state.event("PID1 direct display: persistent scanout allocated");
        let mut presented = false;
        let mut source_name = path.to_owned();
        let mut generation = u64::MAX;
        loop {
            let result = state.launcher.with_presentation(|path, epoch| {
                if source_name != path || generation != epoch {
                    let Ok(frames) = g06_display::frames::Frames::open(path) else { return Ok(None); };
                    display.set_source(frames);
                    source_name = path.to_owned(); generation = epoch;
                }
                display.tick(false)
            });
            match result {
                Ok(Some(sequence)) => {
                    if !presented {
                        state.service("display", "presenting");
                        state.event(format!(
                            "PID1 direct display: first frame presented, sequence={sequence}"
                        ));
                        presented = true;
                    }
                }
                Ok(None) => thread::sleep(Duration::from_millis(3)),
                Err(error) if matches!(error.kind(), io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut | io::ErrorKind::InvalidData | io::ErrorKind::UnexpectedEof) => {
                    thread::sleep(Duration::from_millis(2));
                }
                Err(error) => {
                    state.service("display", "failed");
                    state.event(format!("PID1 direct display stopped: {error}"));
                    return;
                }
            }
        }
    });
    Ok(())
}
