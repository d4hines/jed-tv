//! Fallback overlay publisher for supervisors without built-in volume control.
#[path="../../shared/volume.rs"]
mod volume;
use std::{fs,thread,time::Duration};
fn main() {
    let mut previous=None;
    loop {
        if std::path::Path::new("/tmp/pid1-volume.ready").exists() { return; }
        let modified=fs::metadata("/tmp/app-volume").and_then(|m|m.modified()).ok();
        if modified.is_some() && modified!=previous {
            if let Some(gain)=fs::read_to_string("/tmp/app-volume").ok()
                .and_then(|s|s.trim().parse::<f32>().ok()).filter(|v|v.is_finite()) {
                if let Ok(revision)=volume::publish(gain) {
                    previous=modified;
                    println!("VOLUME_OSD revision={revision} level={}",volume::level::percent(gain));
                }
            }
        }
        thread::sleep(Duration::from_millis(40));
    }
}
