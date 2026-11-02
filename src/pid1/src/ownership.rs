//! Allocation-free scalar Lean recognition of kernel ownership records.
//! No module initializer, boxed Lean value or runtime entry is used here.
extern "C" {
    fn tv_owner_serial_step(state: u32, byte: u32) -> u32;
    fn tv_owner_identity(pid: u32, state: u32) -> u8;
    fn tv_owner_pid_step(state: u64, byte: u32) -> u64;
    fn tv_owner_pid_finish(state: u64) -> u8;
    fn tv_owner_comm_step(state: u32, byte: u32) -> u32;
    fn tv_owner_comm_finish(state: u32) -> u32;
    fn tv_owner_status_step(state: u32, byte: u32) -> u32;
    fn tv_owner_status_finish(state: u32) -> u8;
    fn tv_owner_permitted(owner: u32, stopped: u8) -> u8;
}
pub fn identity(pid: u32, bytes: &[u8]) -> bool {
    let state = bytes.iter().fold(0, |state, &b| unsafe {
        tv_owner_serial_step(state, b.into())
    });
    unsafe { tv_owner_identity(pid, state) != 0 }
}
pub fn numeric_name(bytes: &[u8]) -> bool {
    let state = bytes
        .iter()
        .fold(0, |state, &b| unsafe { tv_owner_pid_step(state, b.into()) });
    unsafe { tv_owner_pid_finish(state) != 0 }
}
/// None means malformed/non-ASCII, Some(0) an unrelated comm name.
pub fn owner(bytes: &[u8]) -> Option<u32> {
    let state = bytes.iter().fold(0, |state, &b| unsafe {
        tv_owner_comm_step(state, b.into())
    });
    let result = unsafe { tv_owner_comm_finish(state) };
    (result != u32::MAX).then_some(result)
}
pub fn stopped(bytes: &[u8]) -> bool {
    let state = bytes.iter().fold(0, |state, &b| unsafe {
        tv_owner_status_step(state, b.into())
    });
    unsafe { tv_owner_status_finish(state) != 0 }
}
pub fn permitted(owner: u32, stopped: bool) -> bool {
    unsafe { tv_owner_permitted(owner, stopped.into()) != 0 }
}
/// Display text only; recognition is defined by the Lean automaton.
pub fn label(owner: u32) -> &'static str {
    match owner {
        1 => "surfaceflinger",
        2 => "composer@2.4-se",
        3 => "audioserver",
        4 => "mstar.hardware.",
        5 => "midaemon",
        6 => "app-rust-displa",
        7 => "app-rust-audio-",
        _ => "invalid owner",
    }
}
