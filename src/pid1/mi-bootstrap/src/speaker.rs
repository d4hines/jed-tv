//! Persistent ALL and MAIN_SPEAKER outputs in the stock cold-start order.
//! This never initializes modules or opens PCM.
use std::{fs::File, io};
const GET: u32 = 0xc0081116;
const OPEN: u32 = 0xc0581103;
const CLOSE: u32 = 0x40041104;
const COUNT: u32 = 0xc010111e;
const MUTE: u32 = 0xc00c110d;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum OutputPath {
    All = 0,
    MainSpeaker = 4,
}
pub trait Io {
    fn authorize(&mut self) -> io::Result<()>;
    fn call(&mut self, request: u32, payload: &mut [u8]) -> io::Result<i32>;
}
fn error(s: &str) -> io::Error {
    io::Error::other(s)
}
fn word(data: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(data[offset..offset + 4].try_into().unwrap())
}
fn put(data: &mut [u8], offset: usize, n: u32) {
    data[offset..offset + 4].copy_from_slice(&n.to_le_bytes())
}
fn checked(io: &mut impl Io, request: u32, data: &mut [u8]) -> io::Result<()> {
    let code = io.call(request, data)?;
    if code != 0 {
        return Err(error(&format!(
            "output MI request {request:#010x}: status {code}"
        )));
    }
    Ok(())
}
pub struct Output<I: Io> {
    io: Option<I>,
    handle: u32,
    close_uncertain: bool,
}
impl<I: Io> Output<I> {
    pub fn create(mut io: I, path: OutputPath) -> io::Result<Self> {
        io.authorize()?;
        let mut query = [0u8; 8];
        put(&mut query, 0, path as u32);
        let status = io.call(GET, &mut query)?;
        if status == 0 {
            return Err(error("output already exists; refusing ownership transfer"));
        }
        if status != 7 {
            return Err(error(&format!(
                "output lookup status {status}; no Open issued"
            )));
        }
        let mut open = [0u8; 88];
        put(&mut open, 68, path as u32);
        // Exact stock HAL: empty name, default volume table, null table pointer.
        if let Err(error) = checked(&mut io, OPEN, &mut open) {
            // A failed Open is not proof that no kernel object was created.
            // Keep its file context; the startup coordinator must not retry.
            std::mem::forget(io);
            return Err(error);
        }
        let handle = word(&open, 0);
        if handle >> 24 != 0x17 {
            std::mem::forget(io);
            return Err(error("output Open returned invalid handle"));
        }
        let mut owner = Self {
            io: Some(io),
            handle,
            close_uncertain: false,
        };
        let mut query = [0u8; 8];
        put(&mut query, 0, path as u32);
        checked(owner.io.as_mut().unwrap(), GET, &mut query)?;
        if word(&query, 4) != handle {
            return Err(error("output handle readback mismatch"));
        }
        Ok(owner)
    }
    pub fn handle(&self) -> u32 {
        self.handle
    }
    /// Explicit close is permitted only after every input/output route is gone.
    /// On failure the owner remains live so callers can resolve the dependency.
    pub fn close(&mut self) -> io::Result<()> {
        if self.io.is_none() {
            return Ok(());
        }
        if self.close_uncertain {
            return Err(error("output Close outcome uncertain; no retry"));
        }
        let Some(io) = self.io.as_mut() else {
            return Ok(());
        };
        for direction in [1u32, 0] {
            let mut count = [0u8; 16];
            put(&mut count, 0, self.handle);
            put(&mut count, 4, direction);
            checked(io, COUNT, &mut count)?;
            if word(&count, 12) != 0 {
                return Err(error("output still has routes; retained output"));
            }
        }
        let mut mute = [0u8; 12];
        put(&mut mute, 0, self.handle); // USER mute type0.
        mute[8] = 1;
        checked(io, MUTE, &mut mute)?;
        let mut close = self.handle.to_le_bytes();
        self.close_uncertain = true;
        checked(io, CLOSE, &mut close)?;
        self.io = None;
        Ok(())
    }
}
impl<I: Io> Drop for Output<I> {
    fn drop(&mut self) {
        if let Err(e) = self.close() {
            eprintln!("output owner retained after cleanup failure: {e}");
            // Keep the descriptor and logical output alive when a route may reference
            // it. Never let device-file release implicitly invalidate a live output.
            if let Some(io) = self.io.take() {
                std::mem::forget(io)
            }
        }
    }
}

/// The stock HAL first opens ALL, then MAIN_SPEAKER. The ALL owner remains
/// alive until the speaker and every route have safely gone away.
pub struct Outputs<I: Io> {
    speaker: Option<Output<I>>,
    all: Option<Output<I>>,
}
impl<I: Io> Outputs<I> {
    pub fn create(all_io: I, speaker_io: I) -> io::Result<Self> {
        let all = Output::create(all_io, OutputPath::All)?;
        let speaker = match Output::create(speaker_io, OutputPath::MainSpeaker) {
            Ok(speaker) => speaker,
            Err(error) => {
                // Retain global setup on partial startup failure. In particular,
                // an ambiguous speaker Open must not outlive its ALL owner.
                std::mem::forget(all);
                return Err(error);
            }
        };
        Ok(Self {
            speaker: Some(speaker),
            all: Some(all),
        })
    }
    pub fn speaker_handle(&self) -> u32 {
        self.speaker.as_ref().unwrap().handle()
    }
    pub fn all_handle(&self) -> u32 {
        self.all.as_ref().unwrap().handle()
    }
    pub fn close(&mut self) -> io::Result<()> {
        if let Some(speaker) = self.speaker.as_mut() {
            speaker.close()?;
        }
        self.speaker = None;
        if let Some(all) = self.all.as_mut() {
            all.close()?;
        }
        self.all = None;
        Ok(())
    }
}
impl<I: Io> Drop for Outputs<I> {
    fn drop(&mut self) {
        if let Err(error) = self.close() {
            eprintln!("output owners retained after cleanup failure: {error}");
            if let Some(speaker) = self.speaker.take() {
                std::mem::forget(speaker);
            }
            if let Some(all) = self.all.take() {
                std::mem::forget(all);
            }
        }
    }
}

/// A device FD detached from the temporary borrow; the owning AudioHardware
/// still retains Device/cold provenance for the entire speaker lifetime.
pub struct NativeIo {
    file: File,
    pid: i32,
}
impl NativeIo {
    pub fn from_cold_device(device: &mut super::Device) -> io::Result<Self> {
        use super::Transport;
        device.ensure_exclusive_cold_boot()?;
        for module in [
            super::Module::Audio,
            super::Module::AudioOutput,
            super::Module::Pcm,
        ] {
            if !device.completed.contains(&module) {
                return Err(error(
                    "speaker creation requires completed native audio initialization",
                ));
            }
        }
        if device.status(1)? {
            return Err(error("SYS_USER ready before speaker creation"));
        }
        Ok(Self {
            file: super::open_node("aout", true)?,
            pid: unsafe { super::getpid() },
        })
    }
}
impl Io for NativeIo {
    fn authorize(&mut self) -> io::Result<()> {
        if unsafe { super::geteuid() } != 0 || unsafe { super::getpid() } != self.pid {
            return Err(error("speaker authority changed"));
        }
        Ok(())
    }
    fn call(&mut self, request: u32, data: &mut [u8]) -> io::Result<i32> {
        self.authorize()?;
        super::call(
            &self.file,
            request,
            data.as_mut_ptr() as u64,
            data.len() as u16,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        collections::VecDeque,
        sync::{Arc, Mutex},
    };
    struct Mock {
        steps: VecDeque<(u32, i32, u32)>,
        calls: Arc<Mutex<Vec<u32>>>,
        allowed: bool,
    }
    impl Io for Mock {
        fn authorize(&mut self) -> io::Result<()> {
            if self.allowed {
                Ok(())
            } else {
                Err(error("denied"))
            }
        }
        fn call(&mut self, r: u32, p: &mut [u8]) -> io::Result<i32> {
            self.calls.lock().unwrap().push(r);
            let (er, s, v) = self.steps.pop_front().expect("unexpected call");
            assert_eq!(r, er);
            match r {
                OPEN => {
                    assert_eq!(p.len(), 88);
                    assert_eq!(word(p, 68), 4);
                    assert!(p[4..68].iter().all(|&b| b == 0));
                    assert!(p[72..].iter().all(|&b| b == 0));
                    put(p, 0, v)
                }
                GET => {
                    assert_eq!(word(p, 0), 4);
                    put(p, 4, v)
                }
                COUNT => {
                    assert_eq!(p.len(), 16);
                    assert_eq!(word(p, 8), 0);
                    put(p, 12, v)
                }
                MUTE => {
                    assert_eq!(word(p, 4), 0);
                    assert_eq!(p[8], 1)
                }
                CLOSE => assert_eq!(word(p, 0), 0x17000001),
                _ => panic!(),
            };
            Ok(s)
        }
    }
    fn setup(steps: Vec<(u32, i32, u32)>) -> (Mock, Arc<Mutex<Vec<u32>>>) {
        let calls = Arc::new(Mutex::new(vec![]));
        (
            Mock {
                steps: steps.into(),
                calls: calls.clone(),
                allowed: true,
            },
            calls,
        )
    }
    fn opened() -> Vec<(u32, i32, u32)> {
        vec![(GET, 7, 0), (OPEN, 0, 0x17000001), (GET, 0, 0x17000001)]
    }
    fn closing() -> Vec<(u32, i32, u32)> {
        vec![(COUNT, 0, 0), (COUNT, 0, 0), (MUTE, 0, 0), (CLOSE, 0, 0)]
    }
    #[test]
    fn creates_and_closes_only_after_routes_clear() {
        let mut steps = opened();
        steps.extend(closing());
        let (m, c) = setup(steps);
        let mut s = Output::create(m, OutputPath::MainSpeaker).unwrap();
        assert_eq!(s.handle(), 0x17000001);
        s.close().unwrap();
        drop(s);
        assert_eq!(
            *c.lock().unwrap(),
            vec![GET, OPEN, GET, COUNT, COUNT, MUTE, CLOSE]
        );
    }
    #[test]
    fn ambiguous_close_is_never_retried() {
        let mut steps = opened();
        steps.extend(closing());
        steps.last_mut().unwrap().1 = 3;
        let (m, c) = setup(steps);
        let mut owner = Output::create(m, OutputPath::MainSpeaker).unwrap();
        assert!(owner.close().is_err());
        assert!(owner.close().is_err());
        drop(owner);
        assert_eq!(c.lock().unwrap().iter().filter(|&&r| r == CLOSE).count(), 1);
    }
    #[test]
    fn close_direction_matches_verified_wrapper() {
        assert_eq!(CLOSE, 0x40041104);
    }
    #[test]
    fn existing_output_never_opened_or_closed() {
        let (m, c) = setup(vec![(GET, 0, 0x17000001)]);
        assert!(Output::create(m, OutputPath::MainSpeaker).is_err());
        assert_eq!(*c.lock().unwrap(), vec![GET]);
    }
    #[test]
    fn unexpected_lookup_stops() {
        for code in [3, 4, 8, 9] {
            let (m, c) = setup(vec![(GET, code, 0)]);
            assert!(Output::create(m, OutputPath::MainSpeaker).is_err());
            assert_eq!(*c.lock().unwrap(), vec![GET]);
        }
    }
    #[test]
    fn failed_open_does_not_close_output() {
        let (m, c) = setup(vec![(GET, 7, 0), (OPEN, 3, 0)]);
        assert!(Output::create(m, OutputPath::MainSpeaker).is_err());
        assert_eq!(*c.lock().unwrap(), vec![GET, OPEN]);
    }
    #[test]
    fn readback_failure_closes_owned_handle() {
        let mut steps = opened();
        steps[2].2 = 0x17000002;
        steps.extend(closing());
        let (m, c) = setup(steps);
        assert!(Output::create(m, OutputPath::MainSpeaker).is_err());
        assert_eq!(c.lock().unwrap().last(), Some(&CLOSE));
    }
    #[test]
    fn live_route_retains_output_on_drop() {
        let mut steps = opened();
        steps.push((COUNT, 0, 1));
        let (m, c) = setup(steps);
        drop(Output::create(m, OutputPath::MainSpeaker).unwrap());
        assert!(!c.lock().unwrap().contains(&CLOSE));
    }
    #[test]
    fn count_error_retains_output_on_drop() {
        let mut steps = opened();
        steps.push((COUNT, 3, 0));
        let (m, c) = setup(steps);
        drop(Output::create(m, OutputPath::MainSpeaker).unwrap());
        assert!(!c.lock().unwrap().contains(&CLOSE));
    }
    #[test]
    fn permission_failure_performs_no_io() {
        let (mut m, c) = setup(vec![]);
        m.allowed = false;
        assert!(Output::create(m, OutputPath::MainSpeaker).is_err());
        assert!(c.lock().unwrap().is_empty());
    }
}

#[cfg(test)]
mod order_tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    #[derive(Default)]
    struct State {
        events: Vec<(&'static str, u32)>,
        speaker_routes: u32,
        failed_close: Option<u32>,
        failed_open: Option<u32>,
        existing_all: bool,
    }
    struct Device {
        path: u32,
        opened: bool,
        state: Arc<Mutex<State>>,
    }
    impl Io for Device {
        fn authorize(&mut self) -> io::Result<()> {
            Ok(())
        }
        fn call(&mut self, request: u32, payload: &mut [u8]) -> io::Result<i32> {
            let mut state = self.state.lock().unwrap();
            let handle = 0x17000001 + self.path;
            match request {
                GET => {
                    assert_eq!(word(payload, 0), self.path);
                    if self.opened || (self.path == 0 && state.existing_all) {
                        put(payload, 4, handle);
                        Ok(0)
                    } else {
                        Ok(7)
                    }
                }
                OPEN => {
                    let mut expected = [0; 88];
                    put(&mut expected, 68, self.path);
                    assert_eq!(payload, expected);
                    state.events.push(("open", self.path));
                    if state.failed_open == Some(self.path) {
                        return Ok(3);
                    }
                    self.opened = true;
                    put(payload, 0, handle);
                    Ok(0)
                }
                COUNT => {
                    assert_eq!(word(payload, 0), handle);
                    let count = if self.path == 4 {
                        state.speaker_routes
                    } else {
                        0
                    };
                    put(payload, 12, count);
                    Ok(0)
                }
                MUTE => {
                    assert_eq!(word(payload, 0), handle);
                    assert_eq!(payload[8], 1);
                    Ok(0)
                }
                CLOSE => {
                    assert_eq!(word(payload, 0), handle);
                    state.events.push(("close", self.path));
                    if state.failed_close == Some(self.path) {
                        return Ok(3);
                    }
                    self.opened = false;
                    Ok(0)
                }
                _ => panic!("unexpected request"),
            }
        }
    }
    impl Drop for Device {
        fn drop(&mut self) {
            self.state.lock().unwrap().events.push(("drop", self.path));
        }
    }
    fn devices(state: Arc<Mutex<State>>) -> (Device, Device) {
        (
            Device {
                path: 0,
                opened: false,
                state: state.clone(),
            },
            Device {
                path: 4,
                opened: false,
                state,
            },
        )
    }
    #[test]
    fn all_opens_before_speaker_and_closes_after_it() {
        let state = Arc::new(Mutex::new(State::default()));
        let (all, speaker) = devices(state.clone());
        let outputs = Outputs::create(all, speaker).unwrap();
        assert_eq!(outputs.all_handle(), 0x17000001);
        assert_eq!(outputs.speaker_handle(), 0x17000005);
        drop(outputs);
        assert_eq!(
            state.lock().unwrap().events,
            vec![
                ("open", 0),
                ("open", 4),
                ("close", 4),
                ("drop", 4),
                ("close", 0),
                ("drop", 0)
            ]
        );
    }
    #[test]
    fn routed_speaker_retains_both_owners() {
        let state = Arc::new(Mutex::new(State::default()));
        let (all, speaker) = devices(state.clone());
        let outputs = Outputs::create(all, speaker).unwrap();
        state.lock().unwrap().speaker_routes = 1;
        drop(outputs);
        assert_eq!(state.lock().unwrap().events, vec![("open", 0), ("open", 4)]);
    }
    #[test]
    fn ambiguous_speaker_close_is_not_retried_and_keeps_all_alive() {
        let state = Arc::new(Mutex::new(State::default()));
        let (all, speaker) = devices(state.clone());
        let mut outputs = Outputs::create(all, speaker).unwrap();
        state.lock().unwrap().failed_close = Some(4);
        assert!(outputs.close().is_err());
        assert!(outputs.close().is_err());
        drop(outputs);
        assert_eq!(
            state.lock().unwrap().events,
            vec![("open", 0), ("open", 4), ("close", 4)]
        );
    }
    #[test]
    fn failed_speaker_open_retains_global_setup() {
        let state = Arc::new(Mutex::new(State {
            failed_open: Some(4),
            ..State::default()
        }));
        let (all, speaker) = devices(state.clone());
        assert!(Outputs::create(all, speaker).is_err());
        assert_eq!(state.lock().unwrap().events, vec![("open", 0), ("open", 4)]);
    }
    #[test]
    fn existing_all_never_opens_speaker_or_takes_global_ownership() {
        let state = Arc::new(Mutex::new(State {
            existing_all: true,
            ..State::default()
        }));
        let (all, speaker) = devices(state.clone());
        assert!(Outputs::create(all, speaker).is_err());
        assert!(state
            .lock()
            .unwrap()
            .events
            .iter()
            .all(|(event, _)| *event == "drop"));
    }
}
