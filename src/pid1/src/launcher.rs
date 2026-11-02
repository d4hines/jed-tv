//! Resident home screen. PID1 owns both the UI and all child exit statuses.
use super::State;
use std::{io, sync::Arc};

struct Children(Arc<State>);
impl tvinit::Children for Children {
    fn tick(&mut self) {
        super::lean_entry::tick();
    }
    fn signal(&mut self, pid: i32, signal: i32) -> io::Result<bool> {
        let _wait = self.0.child_wait_lock.lock().unwrap();
        let foreground = self.0.foreground_child.lock().unwrap();
        if !matches!(*foreground, Some((child, None)) if child == pid) { return Ok(false); }
        if unsafe { libc::kill(pid, signal) } == 0 { Ok(true) }
        else if io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH) { Ok(false) }
        else { Err(io::Error::last_os_error()) }
    }

    fn spawn(&mut self, start: &mut dyn FnMut() -> io::Result<i32>) -> io::Result<i32> {
        let _wait = self.0.child_wait_lock.lock().unwrap();
        let mut foreground = self.0.foreground_child.lock().unwrap();
        if foreground.is_some() { return Err(io::Error::other("foreground app already registered")); }
        let pid = start()?;
        *foreground = Some((pid, None));
        self.0.event(format!("launcher spawned app pid={pid} parent=1"));
        Ok(pid)
    }
    fn poll(&mut self, pid: i32) -> io::Result<Option<i32>> {
        let mut foreground = self.0.foreground_child.lock().unwrap();
        match *foreground {
            Some((child, result)) if child == pid => {
                if result.is_some() { *foreground = None; }
                Ok(result)
            }
            _ => Err(io::Error::from_raw_os_error(libc::ECHILD)),
        }
    }
}

pub(super) fn run(state: Arc<State>) -> ! {
    state.service("launcher", "resident");
    // Launch automatically when a boot image already contains applications.
    if ["/bin/slides", "/bin/doomtv", "/bin/ytplay"].iter().any(|p| std::path::Path::new(p).exists()) {
        state.launcher.reload();
    }
    tvinit::serve(&state.launcher, &mut Children(state.clone()));
}
