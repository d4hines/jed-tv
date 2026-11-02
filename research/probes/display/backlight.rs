//! Keep the G06 panel dark without stopping the debugging environment.
use std::{fs, io::{self, Write}, path::Path, sync::atomic::{AtomicBool, Ordering}, time::Duration};
static STOP: AtomicBool = AtomicBool::new(false);
static RESTORE: AtomicBool = AtomicBool::new(false);
extern "C" { fn signal(number: i32, handler: extern "C" fn(i32)) -> usize; }
extern "C" fn stop(signal: i32) {
    RESTORE.store(signal == 10, Ordering::Relaxed);
    STOP.store(true, Ordering::Relaxed);
}
const BASE: &str = "/sys/class/backlight/backlight";
fn number(path: impl AsRef<Path>) -> io::Result<u32> {
    fs::read_to_string(path)?.trim().parse().map_err(|_| io::Error::other("invalid backlight value"))
}
fn off() -> io::Result<()> {
    if number(format!("{BASE}/brightness"))? != 0 || number(format!("{BASE}/actual_brightness"))? != 0 {
        fs::write(format!("{BASE}/brightness"), b"0")?;
    }
    if number(format!("{BASE}/bl_power"))? != 4 {
        fs::write(format!("{BASE}/bl_power"), b"4")?;
    }
    Ok(())
}
fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    if args.len() != 2 || !["--hold-off", "--restore"].contains(&args[0].as_str()) {
        return Err(io::Error::other("usage: backlight --hold-off|--restore STATE_DIRECTORY; SIGUSR1 restores and exits"));
    }
    if !fs::read_to_string("/proc/cmdline")?.split_whitespace().any(|s| s == "androidboot.serialno=4C0490290FA05A0C0") {
        return Err(io::Error::other("requires original G06"));
    }
    let state = Path::new(&args[1]);
    fs::create_dir_all(state)?;
    for (saved, node) in [("brightness", "brightness"), ("power", "bl_power")] {
        if !state.join(saved).exists() {
            let mut file = fs::OpenOptions::new().write(true).create_new(true).open(state.join(saved))?;
            writeln!(file, "{}", number(format!("{BASE}/{node}"))?)?;
        }
    }
    let brightness = number(state.join("brightness"))?;
    let power = number(state.join("power"))?;
    if brightness > number(format!("{BASE}/max_brightness"))? || power > 4 {
        return Err(io::Error::other("invalid saved backlight settings"));
    }
    if args[0] == "--restore" {
        fs::write(format!("{BASE}/brightness"), brightness.to_string())?;
        return fs::write(format!("{BASE}/bl_power"), power.to_string());
    }
    unsafe { signal(10, stop); signal(15, stop); signal(2, stop); }
    fs::write(state.join("pid"), std::process::id().to_string())?;
    println!("BACKLIGHT holding off; SIGUSR1 restores saved brightness={brightness} power={power}");
    io::stdout().flush()?;
    while !STOP.load(Ordering::Relaxed) {
        off()?;
        std::thread::sleep(Duration::from_millis(250));
    }
    if RESTORE.load(Ordering::Relaxed) {
        fs::write(format!("{BASE}/brightness"), brightness.to_string())?;
        fs::write(format!("{BASE}/bl_power"), power.to_string())?;
    }
    fs::remove_file(state.join("pid"))?;
    Ok(())
}
