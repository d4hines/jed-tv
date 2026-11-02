//! ARM syscall-stop direction is supplied in IP/r12 by the kernel, not toggled.
use std::{collections::BTreeMap, io};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Entry,
    Exit,
}
pub fn phase(ip: u32) -> io::Result<Phase> {
    match ip {
        0 => Ok(Phase::Entry),
        1 => Ok(Phase::Exit),
        _ => Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "ARM syscall stop has invalid r12 direction",
        )),
    }
}
#[derive(Default)]
pub struct Births {
    children: BTreeMap<i32, bool>,
    early_stops: BTreeMap<i32, bool>,
}
impl Births {
    pub fn root(&mut self, pid: i32) {
        self.children.insert(pid, false);
    }
    pub fn known(&self, pid: i32) -> bool {
        self.children.contains_key(&pid)
    }
    /// A parent event identifies the child. True means its initial stop is held.
    pub fn announce(&mut self, pid: i32) -> io::Result<bool> {
        if self.children.contains_key(&pid) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "duplicate ptrace child event",
            ));
        }
        let held = self.early_stops.remove(&pid).is_some();
        self.children.insert(pid, !held);
        Ok(held)
    }
    /// True means suppress/resume the synthetic initial SIGSTOP. Unknown tasks
    /// are held until a parent event proves membership in the traced tree.
    pub fn initial_stop(&mut self, pid: i32) -> io::Result<bool> {
        match self.children.get_mut(&pid) {
            Some(waiting) if *waiting => {
                *waiting = false;
                Ok(true)
            }
            Some(_) => Ok(false),
            None => {
                if self.early_stops.insert(pid, true).is_some() {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "duplicate unannounced stop",
                    ));
                }
                Ok(false)
            }
        }
    }
    #[cfg(test)]
    pub fn held(&self, pid: i32) -> bool {
        self.early_stops.contains_key(&pid)
    }
    pub fn exit(&mut self, pid: i32) {
        self.children.remove(&pid);
        self.early_stops.remove(&pid);
    }
    pub fn exec(&mut self, pid: i32, old: i32) {
        if pid != old {
            self.exit(old);
        }
        self.children.insert(pid, false);
    }
    pub fn empty(&self) -> bool {
        self.children.is_empty() && self.early_stops.is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn phase_never_relies_on_alternation() {
        assert_eq!(phase(1).unwrap(), Phase::Exit);
        assert_eq!(phase(0).unwrap(), Phase::Entry);
        assert!(phase(2).is_err());
    }
    #[test]
    fn parent_event_before_stop() {
        let mut b = Births::default();
        b.root(10);
        assert!(!b.announce(11).unwrap());
        assert!(b.initial_stop(11).unwrap());
        assert!(!b.initial_stop(11).unwrap());
    }
    #[test]
    fn stop_before_parent_event_is_held() {
        let mut b = Births::default();
        b.root(10);
        assert!(!b.initial_stop(11).unwrap());
        assert!(b.held(11));
        assert!(!b.known(11));
        assert!(b.announce(11).unwrap());
        assert!(!b.held(11));
        assert!(b.known(11));
    }
    #[test]
    fn exec_recreates_leader_after_leader_exit() {
        let mut b = Births::default();
        b.root(10);
        b.announce(11).unwrap();
        b.exit(10);
        assert!(b.known(11));
        b.exec(10, 11);
        assert!(b.known(10));
        assert!(!b.known(11));
        b.exit(10);
        assert!(b.empty());
    }
    #[test]
    fn exec_thread_tid_change_and_exit() {
        let mut b = Births::default();
        b.root(10);
        b.announce(11).unwrap();
        b.exec(10, 11);
        assert!(!b.known(11));
        assert!(b.known(10));
        b.exit(10);
        assert!(b.empty());
    }
}
