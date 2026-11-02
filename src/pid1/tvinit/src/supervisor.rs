//! Formatting the raw status classified by the shared Lean policy.
extern "C" { fn tv_supervisor_status(status: u32) -> u64; }

pub fn status(raw: i32) -> String {
    let result = unsafe { tv_supervisor_status(raw as u32) };
    let detail = result as u32;
    match result >> 32 {
        0 => format!("exited ({detail})"), 1 => "stopped".into(),
        2 => format!("killed by signal {detail}"), 3 => format!("status {detail:#x}"),
        _ => panic!("invalid Lean exit status"),
    }
}
