//! One cold hardware owner. No Binder service or multimedia daemon is started.
use super::{direct_services, State};
use mi_bootstrap::{
    amplifier::Plan, owned::DisplayHardware, staging::ColdPlatform, ColdBootProvenance, Device,
};
use std::{
    fs::{self, File},
    io,
    os::unix::fs::{OpenOptionsExt, PermissionsExt},
    process::{Child, Command, Stdio},
    sync::Arc,
    thread,
    time::{Duration, Instant},
};

#[path = "tee_boundary.rs"]
mod tee_boundary;
const AMP_PLAN: &[u8] = include_bytes!(env!("HELLO_AMP_PLAN"));
const PROPERTIES: &[u8] = include_bytes!(env!("HELLO_ANDROID_PROPERTIES"));
const MOUNT_RUNTIME: &str = include_str!("../scripts/mount-stock-runtime.sh");

pub(super) fn start(state: Arc<State>) -> io::Result<()> {
    direct_services::verify_ownership()?;
    let mut ownership = state.media_owned.lock().unwrap();
    if *ownership {
        return Err(io::Error::other("hardware startup already claimed"));
    }
    // SAFETY: called once by native PID1 before exposing its interactive
    // console. Only the audited watchdog setup has used Utopia so far; no
    // userspace MI initializer or client has been started in this kernel boot.
    let staging = unsafe { ColdPlatform::begin()? };
    let policy = super::cold_plan::start()?;
    *ownership = true;
    fs::write("/tmp/app-volume", b"0\n")?;
    state.service("hardware", "staging");
    let worker = state.clone();
    thread::spawn(move || {
        if let Err(error) = run(&worker, staging, policy) {
            worker.service("hardware", "failed");
            worker.event(format!("cold hardware startup stopped: {error}"));
        }
    });
    Ok(())
}

fn command(state: &State, command: &mut Command) -> io::Result<()> {
    // The independent watchdog keeps feeding while its orphan reaper skips
    // this interval; it must not steal Command's child exit status.
    let _guard = state.child_wait_lock.lock().unwrap();
    let status = command.status()?;
    if status.success() {
        Ok(())
    } else {
        Err(io::Error::other(format!(
            "hardware staging command failed: {status}"
        )))
    }
}

struct TeeProvider {
    child: Child,
    state: Arc<State>,
    exited: Option<String>,
}
impl TeeProvider {
    fn exit_status(&mut self) -> Option<String> {
        if self.exited.is_none() {
            let _guard = self.state.child_wait_lock.lock().unwrap();
            self.exited = match self.child.try_wait() {
                Ok(None) => None,
                Ok(Some(status)) => Some(status.to_string()),
                Err(error) => Some(format!("child status unavailable: {error}")),
            };
        }
        self.exited.clone()
    }
}
impl Drop for TeeProvider {
    fn drop(&mut self) {
        let _guard = self.state.child_wait_lock.lock().unwrap();
        // If the orphan reaper already collected it, ECHILD means we must not
        // signal a possibly reused PID. Holding the lock closes that race.
        if matches!(self.child.try_wait(), Ok(None)) {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

fn monitor_tee(state: &State, tee: &mut TeeProvider, reported: &mut bool) {
    if !*reported {
        if let Some(status) = tee.exit_status() {
            *reported = true;
            state.service("tee", "failed");
            state.service("hardware", "degraded");
            state.event(format!(
                "TEE provider stopped; media owners retained: {status}"
            ));
        }
    }
}

fn start_tee(state: &Arc<State>) -> io::Result<TeeProvider> {
    // The single retained vendor process. Its device and storage interfaces
    // are root-only; apps never inherit its descriptors.
    fs::create_dir_all("/mnt/vendor/tvservice")?;
    command(
        state,
        Command::new("/bin/mount").args([
            "-t",
            "ext4",
            "-o",
            "ro,noload",
            "/dev/mmcblk0p19",
            "/mnt/vendor/tvservice",
        ]),
    )?;
    std::os::unix::fs::symlink("/mnt/vendor/tvservice/glibc", "/lib")?;
    fs::write("/tmp/android-properties.tar.gz", PROPERTIES)?;
    command(
        state,
        Command::new("/bin/tar").args(["-xzf", "/tmp/android-properties.tar.gz", "-C", "/dev"]),
    )?;
    tee_boundary::restrict()?;
    let log = fs::OpenOptions::new().write(true).create(true).truncate(true)
        .mode(0o600).custom_flags(libc::O_NOFOLLOW).open("/tmp/tee-supplicant.log")?;
    log.set_permissions(fs::Permissions::from_mode(0o600))?;
    let child = {
        let _guard = state.child_wait_lock.lock().unwrap();
        Command::new("/vendor/bin/tee-supplicant")
            .env("ANDROID_ROOT", "/system")
            .env("ANDROID_DATA", "/data")
            .env("ANDROID_STORAGE", "/storage")
            .env("ANDROID_TZDATA_ROOT", "/apex/com.android.tzdata")
            .stdin(Stdio::null())
            .stdout(Stdio::from(log.try_clone()?))
            .stderr(Stdio::from(log))
            .spawn()?
    };
    let mut provider = TeeProvider {
        child,
        state: state.clone(),
        exited: None,
    };
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if let Some(status) = provider.exit_status() {
            return Err(io::Error::other(format!(
                "TEE helper exited before RPC readiness: {status}"
            )));
        }
        if tee_boundary::waiting(provider.child.id()) {
            state.event(format!(
                "TEE provider pid={} waiting; devices root:root0600, storage0700; no tracer",
                provider.child.id()
            ));
            state.service("tee", "waiting");
            return Ok(provider);
        }
        thread::sleep(Duration::from_millis(20));
    }
    Err(io::Error::other(
        "TEE provider did not enter its receive loop",
    ))
}

fn run(state: &Arc<State>, mut staging: ColdPlatform, policy: super::cold_plan::Client) -> io::Result<()> {
    let plan = Plan::parse(AMP_PLAN)?;
    command(state, Command::new("/bin/sh").arg("-c").arg(MOUNT_RUNTIME))?;
    let mut observer = |event: &mi_bootstrap::staging::Event| {
        state.event(format!("cold {}: {}", event.action, event.detail))
    };
    staging.mount_stock(&mut observer)?;
    staging.stage_configuration(&policy, &mut observer)?;
    staging.load_modules(&policy, &mut observer)?;
    let staged = staging.finish()?;
    state.event(format!(
        "cold platform staged: {} observations",
        staged.events.len()
    ));
    let mut tee = start_tee(state)?;
    let mut tee_reported = false;
    // SAFETY: the fixed startup sequence above has loaded only the verified
    // kernel profile. No userspace MI initializer/client was spawned. The TEE
    // provider services secure-world RPC and does not initialize MI modules.
    let cold = unsafe { ColdBootProvenance::claim_exclusive_boot_phase()? };
    state.service("hardware", "display-init");
    let mut display = DisplayHardware::initialize(Device::open()?, cold)?;
    state.event(format!(
        "cold display controller={:#x}, observations={:?}",
        display.controller(),
        display.observations()
    ));
    direct_services::start_display(state.clone())?;
    state.service("hardware", "audio-init");
    let mut audio = match display.initialize_audio(&plan) {
        Ok(audio) => audio,
        Err(error) => {
            state.service("audio", "failed");
            state.service("hardware", "display-only");
            state.event(format!(
                "cold audio stopped; display owner retained: {error}"
            ));
            loop {
                monitor_tee(state, &mut tee, &mut tee_reported);
                thread::sleep(Duration::from_secs(1));
            }
        }
    };
    state.event(format!(
        "cold audio observations={:?}, amplifier={:?}, speaker={:#x}",
        audio.audio_observations(),
        audio.amplifier_handles(),
        audio.speaker_handle()
    ));
    // No PCM client exists yet and global gain is still zero. Leave the amplifier
    // ready for the bounded app mixer; this operation does not emit samples.
    if let Err(error) = audio
        .unmute()
        .and_then(|_| direct_services::start_audio(state.clone()))
    {
        state.service("audio", "failed");
        state.service("hardware", "display-only");
        state.event(format!(
            "cold audio service stopped; owners retained: {error}"
        ));
        loop {
            let _ = &audio;
            monitor_tee(state, &mut tee, &mut tee_reported);
            thread::sleep(Duration::from_secs(1));
        }
    }
    state.service("hardware", "ready");
    state.event("cold Rust hardware startup complete; no Binder or multimedia daemon");
    loop {
        let _ = &audio;
        monitor_tee(state, &mut tee, &mut tee_reported);
        thread::sleep(Duration::from_secs(1));
    }
}
