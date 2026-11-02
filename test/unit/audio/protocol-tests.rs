use super::*;
use std::{cell::RefCell, io::Cursor, rc::Rc, sync::Mutex};
#[derive(Default)]
struct State {
    opens: usize,
    drops: usize,
    samples: Vec<Vec<i16>>,
    fail_open: bool,
    fail_write: bool,
    fail_played: bool,
}
struct MockBackend(Arc<Mutex<State>>);
struct MockTrack {
    state: Arc<Mutex<State>>,
    written: u64,
    channels: u32,
}
impl Backend for MockBackend {
    type Track = MockTrack;
    fn name(&self) -> &'static str {
        "mock"
    }
    fn supports(&self, r: u32, c: u32) -> bool {
        [44100, 48000].contains(&r) && [1, 2].contains(&c)
    }
    fn open(&self, _: u32, channels: u32) -> io::Result<MockTrack> {
        let mut s = self.0.lock().unwrap();
        if s.fail_open {
            return Err(fail("open failed"));
        }
        s.opens += 1;
        Ok(MockTrack {
            state: self.0.clone(),
            written: 0,
            channels,
        })
    }
}
impl Stream for MockTrack {
    fn write(&mut self, samples: &[i16]) -> io::Result<()> {
        let mut s = self.state.lock().unwrap();
        if s.fail_write {
            return Err(fail("write failed"));
        }
        s.samples.push(samples.to_vec());
        self.written += (samples.len() / self.channels as usize) as u64;
        Ok(())
    }
    fn written(&self) -> u64 {
        self.written
    }
    fn played(&self) -> io::Result<u64> {
        if self.state.lock().unwrap().fail_played {
            Err(fail("counter failed"))
        } else {
            Ok(self.written)
        }
    }
}
impl Drop for MockTrack {
    fn drop(&mut self) {
        self.state.lock().unwrap().drops += 1
    }
}
struct Wire {
    input: Cursor<Vec<u8>>,
    output: Rc<RefCell<Vec<u8>>>,
}
impl Read for Wire {
    fn read(&mut self, b: &mut [u8]) -> io::Result<usize> {
        let len = b.len().min(1);
        self.input.read(&mut b[..len])
    }
}
impl Write for Wire {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        let n = b.len().min(1);
        self.output.borrow_mut().extend_from_slice(&b[..n]);
        Ok(n)
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl Session for Wire {
    fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
}
fn open(rate: u32, channels: u32, boost: u32) -> Vec<u8> {
    let mut x = vec![b'O'];
    for n in [rate, channels, boost] {
        x.extend(n.to_le_bytes())
    }
    x
}
fn write(x: &mut Vec<u8>, samples: &[i16]) {
    x.push(b'W');
    x.extend(((samples.len() * 2) as u32).to_le_bytes());
    for s in samples {
        x.extend(s.to_le_bytes())
    }
}
fn execute(input: Vec<u8>, level: f32, state: Arc<Mutex<State>>) -> (io::Result<()>, Vec<u8>) {
    let out = Rc::new(RefCell::new(vec![]));
    let result = client(
        Wire {
            input: Cursor::new(input),
            output: out.clone(),
        },
        &MockBackend(state),
        &AtomicU32::new(level.to_bits()),
    );
    let bytes = out.borrow().clone();
    (result, bytes)
}
fn status(x: &mut Vec<u8>, s: i32) {
    x.extend(s.to_le_bytes())
}
fn counters(x: &mut Vec<u8>, a: u64, b: u64) {
    x.extend(a.to_le_bytes());
    x.extend(b.to_le_bytes())
}
#[test]
fn full_protocol_reset_counters_gain_and_fragmentation() {
    let state = Arc::new(Mutex::new(State::default()));
    let mut input = open(44100, 1, 2);
    write(&mut input, &[10000, -10000]);
    input.extend(b"DRD");
    write(&mut input, &[32767]);
    input.extend(b"FD");
    let (result, out) = execute(input, 0.0025, state.clone());
    result.unwrap();
    let mut expected = vec![];
    status(&mut expected, 0);
    status(&mut expected, 0);
    counters(&mut expected, 2, 2);
    status(&mut expected, 0);
    counters(&mut expected, 0, 0);
    status(&mut expected, 0);
    status(&mut expected, 0);
    counters(&mut expected, 1, 1);
    assert_eq!(out, expected);
    let s = state.lock().unwrap();
    assert_eq!((s.opens, s.drops), (2, 2));
    assert_eq!(s.samples, vec![vec![50, -50], vec![82]]);
}
#[test]
fn unsupported_formats_are_einval_before_open() {
    for (r, c, b) in [(32000, 2, 1), (48000, 3, 1), (48000, 1, 0), (48000, 1, 5)] {
        let state = Arc::new(Mutex::new(State::default()));
        let (result, out) = execute(open(r, c, b), 0.0, state.clone());
        assert!(result.is_err());
        assert_eq!(out, (-22i32).to_le_bytes());
        assert_eq!(state.lock().unwrap().opens, 0);
    }
}
#[test]
fn bad_first_command_and_unknown_command_are_einval() {
    let state = Arc::new(Mutex::new(State::default()));
    let (r, out) = execute(vec![b'D'], 0.0, state);
    assert!(r.is_err());
    assert_eq!(out, (-22i32).to_le_bytes());
    let state = Arc::new(Mutex::new(State::default()));
    let mut x = open(48000, 2, 1);
    x.push(b'?');
    let (r, out) = execute(x, 0.0, state.clone());
    assert!(r.is_err());
    assert_eq!(out, [0i32.to_le_bytes(), (-22i32).to_le_bytes()].concat());
    assert_eq!(state.lock().unwrap().drops, 1);
}
#[test]
fn invalid_block_sizes_are_einval_and_close_owner() {
    for n in [3u32, 1048577] {
        let state = Arc::new(Mutex::new(State::default()));
        let mut x = open(48000, 2, 1);
        x.push(b'W');
        x.extend(n.to_le_bytes());
        let (r, out) = execute(x, 0.0, state.clone());
        assert!(r.is_err());
        assert_eq!(out, [0i32.to_le_bytes(), (-22i32).to_le_bytes()].concat());
        assert_eq!(state.lock().unwrap().drops, 1);
    }
}
#[test]
fn open_and_write_failures_return_abi_error() {
    let state = Arc::new(Mutex::new(State {
        fail_open: true,
        ..Default::default()
    }));
    let (r, out) = execute(open(48000, 2, 1), 0.0, state);
    assert!(r.is_err());
    assert_eq!(out, (-1i32).to_le_bytes());
    let state = Arc::new(Mutex::new(State {
        fail_write: true,
        ..Default::default()
    }));
    let mut x = open(48000, 2, 1);
    write(&mut x, &[0, 0]);
    let (r, out) = execute(x, 0.0, state.clone());
    assert!(r.is_err());
    assert_eq!(out, [0i32.to_le_bytes(), (-1i32).to_le_bytes()].concat());
    assert_eq!(state.lock().unwrap().drops, 1);
}
#[test]
fn truncated_payload_closes_owner_without_write() {
    let state = Arc::new(Mutex::new(State::default()));
    let mut x = open(48000, 2, 1);
    x.push(b'W');
    x.extend(4u32.to_le_bytes());
    x.push(0);
    let (r, out) = execute(x, 0.0, state.clone());
    assert!(r.is_err());
    assert_eq!(out, 0i32.to_le_bytes());
    let s = state.lock().unwrap();
    assert_eq!(s.drops, 1);
    assert!(s.samples.is_empty())
}
#[test]
fn zero_global_gain_is_silent() {
    let state = Arc::new(Mutex::new(State::default()));
    let mut x = open(48000, 2, 4);
    write(&mut x, &[32767, -32768]);
    execute(x, 0.0, state.clone()).0.unwrap();
    assert_eq!(state.lock().unwrap().samples, vec![vec![0, 0]]);
}

#[test]
fn drain_counter_failure_returns_status_and_closes_owner() {
    let state = Arc::new(Mutex::new(State {
        fail_played: true,
        ..Default::default()
    }));
    let mut x = open(48000, 2, 1);
    x.push(b'F');
    let (result, out) = execute(x, 0.0, state.clone());
    assert!(result.is_err());
    assert_eq!(out, [0i32.to_le_bytes(), (-1i32).to_le_bytes()].concat());
    assert_eq!(state.lock().unwrap().drops, 1);
}
