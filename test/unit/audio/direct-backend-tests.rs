use super::*;
use crate::{mi_pcm::BufferInfo, protocol::Stream};
use std::sync::Mutex;
#[derive(Default)]
struct State {
    output: Vec<u8>,
    queue: u32,
    closes: usize,
    stops: usize,
    fail_write: bool,
    busy: Option<Arc<AtomicU8>>,
    events: Vec<&'static str>,
    inputs: Vec<u32>,
    fail_open: bool,
    fail_output_lookup: bool,
    fail_start: bool,
    fail_attach: bool,
    partial_attach: bool,
    fail_detach: bool,
    fail_close: bool,
    gain: VolumeDb,
    fail_gain_restore: bool,
    fail_gain_set: bool,
}
#[derive(Clone)]
struct Mock(Arc<Mutex<State>>);
fn u32at(w: &[u8], i: usize) -> u32 {
    u32::from_ne_bytes(w[i..i + 4].try_into().unwrap())
}
impl Transport for Mock {
    fn transact(&self, r: u32, w: &mut [u8]) -> io::Result<()> {
        let mut s = self.0.lock().unwrap();
        match r {
            0xc0803402 => {
                s.events.push("pcm-open");
                if s.fail_open {
                    return Err(io::Error::other("open failure"));
                };
                w[..4].copy_from_slice(&0x81000000u32.to_ne_bytes())
            }
            0xc0083404 => {
                s.events.push("pcm-start");
                if s.fail_start {
                    return Err(io::Error::other("start failure"));
                }
            }
            0xc010110a => {
                w[8..12].copy_from_slice(&s.gain.integer.to_ne_bytes());
                w[12..16].copy_from_slice(&s.gain.thousandths.to_ne_bytes())
            }
            0x40101109 => {
                let value = VolumeDb {
                    integer: u32at(w, 8) as i32,
                    thousandths: u32at(w, 12) as i32,
                };
                if value.integer == 0 {
                    s.events.push("gain-set");
                    if s.fail_gain_set {
                        return Err(io::Error::other("gain set failed"));
                    }
                } else {
                    s.events.push("gain-restore");
                    if s.fail_gain_restore {
                        return Err(io::Error::other("gain restore failed"));
                    }
                }
                s.gain = value;
            }
            0xc0081116 => {
                if s.fail_output_lookup {
                    return Err(io::Error::other("lookup failure"));
                };
                w[4..8].copy_from_slice(&0x17000001u32.to_ne_bytes())
            }
            0xc010111e => {
                let count = s.inputs.len() as u32;
                w[12..16].copy_from_slice(&count.to_ne_bytes())
            }
            0xc018111f => {
                let count = u32at(w, 12) as usize;
                let ptr = u64::from_ne_bytes(w[16..24].try_into().unwrap()) as usize as *mut u32;
                for (i, h) in s.inputs.iter().take(count).enumerate() {
                    unsafe { ptr.add(i).write(*h) }
                }
            }
            0xc00c111c => {
                s.events.push("route-connect");
                if !s.fail_attach || s.partial_attach {
                    let h = u32at(w, 4);
                    s.inputs.push(h)
                };
                if s.fail_attach {
                    return Err(io::Error::other("attach failure"));
                }
            }
            0xc008111d => {
                s.events.push("route-disconnect");
                if s.fail_detach {
                    return Err(io::Error::other("detach failure"));
                };
                s.inputs.retain(|h| *h != u32at(w, 4))
            }
            0xc0383408 => {
                s.events.push("pcm-write");
                if s.fail_write {
                    return Err(io::Error::other("mock write failed"));
                }
                let n = (u32at(w, 16) as usize).min(16);
                let ptr = u64::from_ne_bytes(w[8..16].try_into().unwrap()) as usize as *const u8;
                s.output
                    .extend_from_slice(unsafe { std::slice::from_raw_parts(ptr, n) });
                w[48..52].copy_from_slice(&(n as u32).to_ne_bytes());
            }
            0xc018340b => {
                let ptr =
                    u64::from_ne_bytes(w[16..24].try_into().unwrap()) as usize as *mut BufferInfo;
                unsafe {
                    *ptr = BufferInfo {
                        capacity_bytes: 19200,
                        queued_bytes: s.queue,
                        free_bytes: 19200 - s.queue,
                    }
                }
            }
            0xc0043405 => {
                s.events.push("pcm-stop");
                s.stops += 1
            }
            0xc0043403 => {
                if let Some(b) = &s.busy {
                    assert_ne!(b.load(Ordering::Acquire), 0, "lease released before Close")
                };
                s.events.push("pcm-close");
                s.closes += 1;
                if s.fail_close {
                    return Err(io::Error::other("close failure"));
                }
            }
            _ => panic!("unexpected mock ioctl {r:x}"),
        }
        Ok(())
    }
}
fn track(
    backend: &DirectBackend,
    rate: u32,
    channels: u32,
) -> (DirectTrack<Mock>, Arc<Mutex<State>>) {
    let state = Arc::new(Mutex::new(State {
        busy: Some(backend.owner.clone()),
        gain: VolumeDb {
            integer: -47,
            thousandths: 0,
        },
        ..Default::default()
    }));
    let lease = backend.acquire().unwrap();
    let mut pcm = Pcm::open_with(Mock(state.clone())).unwrap();
    pcm.start().unwrap();
    (
        DirectTrack {
            pcm: Some(pcm),
            route: None,
            volume: None,
            unresolved_route: None,
            _lease: lease,
            converter: Converter::new(rate, channels).unwrap(),
            rate,
            channels,
            input_written: 0,
            output_written: 0,
        },
        state,
    )
}
#[test]
fn single_owner_and_lease_release() {
    let backend = DirectBackend::default();
    let lease = backend.acquire().unwrap();
    assert!(backend.acquire().is_err());
    drop(lease);
    assert!(backend.acquire().is_ok());
}
#[test]
fn partial_hardware_writes_preserve_converted_samples_and_input_counters() {
    let backend = DirectBackend::default();
    let (mut stream, state) = track(&backend, 44100, 1);
    let input: Vec<i16> = (0..4410).map(|n| (n * 3 - 6000) as i16).collect();
    for block in input.chunks(137) {
        stream.write(block).unwrap()
    }
    assert_eq!(stream.written(), 4410);
    assert_eq!(stream.played().unwrap(), 4410);
    let expected = Converter::new(44100, 1).unwrap().convert(&input).unwrap();
    assert_eq!(state.lock().unwrap().output, expected);
    assert_eq!(expected.len() / 4, 4800);
    state.lock().unwrap().queue = 400;
    assert_eq!(
        stream.played().unwrap(),
        consumed_input_frames(4700, 44100, 4410)
    );
    drop(stream);
    {
        let state = state.lock().unwrap();
        assert_eq!((state.stops, state.closes), (1, 1));
    }
    assert_eq!(backend.owner.load(Ordering::Acquire), 0);
}
#[test]
fn write_failure_keeps_reported_input_unaccepted_and_releases_owner() {
    let backend = DirectBackend::default();
    let (mut stream, state) = track(&backend, 48000, 2);
    state.lock().unwrap().fail_write = true;
    assert!(stream.write(&[0; 8]).is_err());
    assert_eq!(stream.written(), 0);
    drop(stream);
    assert_eq!(state.lock().unwrap().closes, 1);
    assert!(backend.acquire().is_ok());
}
#[test]
fn impossible_queue_is_not_a_fake_playback_counter() {
    let backend = DirectBackend::default();
    let (stream, state) = track(&backend, 48000, 2);
    state.lock().unwrap().queue = 4;
    assert!(stream.played().is_err());
    drop(stream);
}

fn state(backend: &DirectBackend) -> Arc<Mutex<State>> {
    Arc::new(Mutex::new(State {
        busy: Some(backend.owner.clone()),
        gain: VolumeDb {
            integer: -47,
            thousandths: 0,
        },
        ..Default::default()
    }))
}
fn routed_open(
    backend: &DirectBackend,
    s: &Arc<Mutex<State>>,
) -> io::Result<DirectTrack<Mock, Mock>> {
    backend.open_using(
        48000,
        2,
        || Pcm::open_with(Mock(s.clone())),
        || Aout::existing_speaker_with(Mock(s.clone())),
    )
}
#[test]
fn route_restore_precedes_pcm_stop_and_close() {
    let backend = DirectBackend::with_speaker_route(RoutePolicy::RequireEmpty);
    let s = state(&backend);
    let track = routed_open(&backend, &s).unwrap();
    track.finish().unwrap();
    assert_eq!(
        s.lock().unwrap().events,
        vec![
            "pcm-open",
            "route-connect",
            "gain-set",
            "pcm-start",
            "gain-restore",
            "route-disconnect",
            "pcm-stop",
            "pcm-close"
        ]
    );
    assert!(s.lock().unwrap().inputs.is_empty());
    assert_eq!(backend.owner.load(Ordering::Acquire), 0);
}
#[test]
fn pcm_open_failure_releases_lease_without_closing_unowned_stream() {
    let backend = DirectBackend::with_speaker_route(RoutePolicy::RequireEmpty);
    let s = state(&backend);
    s.lock().unwrap().fail_open = true;
    assert!(routed_open(&backend, &s).is_err());
    assert_eq!(s.lock().unwrap().events, vec!["pcm-open"]);
    assert_eq!(backend.owner.load(Ordering::Acquire), 0);
}
#[test]
fn output_lookup_failure_closes_only_owned_pcm() {
    let backend = DirectBackend::with_speaker_route(RoutePolicy::RequireEmpty);
    let s = state(&backend);
    s.lock().unwrap().fail_output_lookup = true;
    assert!(routed_open(&backend, &s).is_err());
    assert_eq!(s.lock().unwrap().events, vec!["pcm-open", "pcm-close"]);
    assert_eq!(backend.owner.load(Ordering::Acquire), 0);
}
#[test]
fn attach_failure_rolls_back_before_closing_unstarted_pcm() {
    let backend = DirectBackend::with_speaker_route(RoutePolicy::RequireEmpty);
    let s = state(&backend);
    {
        let mut state = s.lock().unwrap();
        state.fail_attach = true;
        state.partial_attach = true;
    }
    assert!(routed_open(&backend, &s).is_err());
    assert_eq!(
        s.lock().unwrap().events,
        vec!["pcm-open", "route-connect", "route-disconnect", "pcm-close"]
    );
    assert!(s.lock().unwrap().inputs.is_empty());
    assert_eq!(backend.owner.load(Ordering::Acquire), 0);
}
#[test]
fn start_failure_restores_route_before_close() {
    let backend = DirectBackend::with_speaker_route(RoutePolicy::RequireEmpty);
    let s = state(&backend);
    s.lock().unwrap().fail_start = true;
    assert!(routed_open(&backend, &s).is_err());
    assert_eq!(
        s.lock().unwrap().events,
        vec![
            "pcm-open",
            "route-connect",
            "gain-set",
            "pcm-start",
            "gain-restore",
            "route-disconnect",
            "pcm-close"
        ]
    );
    assert_eq!(backend.owner.load(Ordering::Acquire), 0);
}
#[test]
fn write_failure_drop_restores_route() {
    let backend = DirectBackend::with_speaker_route(RoutePolicy::RequireEmpty);
    let s = state(&backend);
    let mut track = routed_open(&backend, &s).unwrap();
    s.lock().unwrap().fail_write = true;
    assert!(track.write(&[0; 4]).is_err());
    drop(track);
    assert_eq!(
        s.lock().unwrap().events,
        vec![
            "pcm-open",
            "route-connect",
            "gain-set",
            "pcm-start",
            "pcm-write",
            "gain-restore",
            "route-disconnect",
            "pcm-stop",
            "pcm-close"
        ]
    );
    assert_eq!(backend.owner.load(Ordering::Acquire), 0);
}
#[test]
fn restore_failure_retains_pcm_lifetime_and_blocks_blind_reopen() {
    let backend = DirectBackend::with_speaker_route(RoutePolicy::RequireEmpty);
    let s = state(&backend);
    let track = routed_open(&backend, &s).unwrap();
    s.lock().unwrap().fail_detach = true;
    let error = track.finish().unwrap_err();
    let cleanup = error
        .get_ref()
        .unwrap()
        .downcast_ref::<DirectCleanupError>()
        .unwrap();
    assert_eq!(cleanup.retained_pcm_handle, Some(0x81000000));
    assert!(cleanup.original_route.as_ref().unwrap().inputs.is_empty());
    assert_eq!(s.lock().unwrap().stops, 0);
    assert_eq!(s.lock().unwrap().closes, 0);
    assert_eq!(backend.owner.load(Ordering::Acquire), 2);
    assert!(routed_open(&backend, &s).is_err());
    assert_eq!(s.lock().unwrap().closes, 0);
}
#[test]
fn failed_attach_rollback_poison_is_preserved_after_pcm_cleanup() {
    let backend = DirectBackend::with_speaker_route(RoutePolicy::RequireEmpty);
    let s = state(&backend);
    {
        let mut state = s.lock().unwrap();
        state.fail_attach = true;
        state.partial_attach = true;
        state.fail_detach = true;
    }
    assert!(routed_open(&backend, &s).is_err());
    assert_eq!(s.lock().unwrap().closes, 0);
    assert_eq!(s.lock().unwrap().stops, 0);
    assert_eq!(backend.owner.load(Ordering::Acquire), 2);
}
#[test]
fn close_failure_blocks_reset_retry() {
    let backend = DirectBackend::with_speaker_route(RoutePolicy::RequireEmpty);
    let s = state(&backend);
    let track = routed_open(&backend, &s).unwrap();
    s.lock().unwrap().fail_close = true;
    assert!(track.finish().is_err());
    assert!(s.lock().unwrap().inputs.is_empty());
    assert_eq!(backend.owner.load(Ordering::Acquire), 2);
}
#[test]
fn reset_releases_route_before_reopening_with_zero_counters() {
    let backend = DirectBackend::with_speaker_route(RoutePolicy::RequireEmpty);
    let s = state(&backend);
    let mut track = routed_open(&backend, &s).unwrap();
    track.write(&[0; 4]).unwrap();
    assert_eq!(track.written(), 2);
    drop(track);
    let track = routed_open(&backend, &s).unwrap();
    assert_eq!(track.written(), 0);
    track.finish().unwrap();
    let events = s.lock().unwrap().events.clone();
    let close = events.iter().position(|e| *e == "pcm-close").unwrap();
    assert_eq!(events[close + 1], "pcm-open");
    assert_eq!(s.lock().unwrap().closes, 2);
    assert!(s.lock().unwrap().inputs.is_empty());
}

struct ProtocolMockBackend {
    backend: DirectBackend,
    state: Arc<Mutex<State>>,
}
impl protocol::Backend for ProtocolMockBackend {
    type Track = DirectTrack<Mock, Mock>;
    fn supports(&self, rate: u32, channels: u32) -> bool {
        protocol::Backend::supports(&self.backend, rate, channels)
    }
    fn open(&self, rate: u32, channels: u32) -> io::Result<Self::Track> {
        self.backend.open_using(
            rate,
            channels,
            || Pcm::open_with(Mock(self.state.clone())),
            || Aout::existing_speaker_with(Mock(self.state.clone())),
        )
    }
    fn name(&self) -> &'static str {
        "routed-test"
    }
}
struct MemorySession<'a> {
    input: std::io::Cursor<Vec<u8>>,
    output: &'a mut Vec<u8>,
}
impl std::io::Read for MemorySession<'_> {
    fn read(&mut self, b: &mut [u8]) -> io::Result<usize> {
        std::io::Read::read(&mut self.input, b)
    }
}
impl std::io::Write for MemorySession<'_> {
    fn write(&mut self, b: &[u8]) -> io::Result<usize> {
        self.output.extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
impl protocol::Session for MemorySession<'_> {
    fn set_read_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
    fn set_write_timeout(&self, _: Option<Duration>) -> io::Result<()> {
        Ok(())
    }
}
#[test]
fn real_shared_protocol_reset_restores_route_before_next_open() {
    let backend = DirectBackend::with_speaker_route(RoutePolicy::RequireEmpty);
    let s = state(&backend);
    let backend = ProtocolMockBackend {
        backend,
        state: s.clone(),
    };
    let mut input = vec![b'O'];
    for n in [44100u32, 1, 1] {
        input.extend(n.to_le_bytes())
    }
    input.push(b'W');
    input.extend(4u32.to_le_bytes());
    input.extend([0u8; 4]);
    input.extend(b"DRDF");
    let mut output = vec![];
    protocol::client(
        MemorySession {
            input: std::io::Cursor::new(input),
            output: &mut output,
        },
        &backend,
        &std::sync::atomic::AtomicU32::new(0f32.to_bits()),
    )
    .unwrap();
    let mut expected = vec![];
    expected.extend(0i32.to_le_bytes());
    expected.extend(0i32.to_le_bytes());
    expected.extend(2u64.to_le_bytes());
    expected.extend(2u64.to_le_bytes());
    expected.extend(0i32.to_le_bytes());
    expected.extend(0u64.to_le_bytes());
    expected.extend(0u64.to_le_bytes());
    expected.extend(0i32.to_le_bytes());
    assert_eq!(output, expected);
    let state = s.lock().unwrap();
    assert_eq!(state.closes, 2);
    assert!(state.inputs.is_empty());
    let first_close = state.events.iter().position(|e| *e == "pcm-close").unwrap();
    assert_eq!(state.events[first_close + 1], "pcm-open");
    assert_eq!(
        state
            .events
            .iter()
            .filter(|e| **e == "route-disconnect")
            .count(),
        2
    );
}

#[test]
fn default_backend_sets_scoped_gain_and_restores_it() {
    let backend = DirectBackend::default();
    let s = state(&backend);
    let track = routed_open(&backend, &s).unwrap();
    assert_eq!(s.lock().unwrap().gain.integer, 0);
    track.finish().unwrap();
    assert_eq!(s.lock().unwrap().gain.integer, -47);
}
#[test]
fn failed_gain_restore_poison_does_not_skip_route_cleanup() {
    let backend = DirectBackend::default();
    let s = state(&backend);
    let track = routed_open(&backend, &s).unwrap();
    s.lock().unwrap().fail_gain_restore = true;
    assert!(track.finish().is_err());
    assert!(s.lock().unwrap().inputs.is_empty());
    assert_eq!(s.lock().unwrap().closes, 1);
    assert_eq!(backend.owner.load(Ordering::Acquire), 2);
}

#[test]
fn gain_set_failure_restores_then_detaches_and_closes_before_start() {
    let backend = DirectBackend::default();
    let s = state(&backend);
    s.lock().unwrap().fail_gain_set = true;
    assert!(routed_open(&backend, &s).is_err());
    assert_eq!(
        s.lock().unwrap().events,
        vec![
            "pcm-open",
            "route-connect",
            "gain-set",
            "gain-restore",
            "route-disconnect",
            "pcm-close"
        ]
    );
    assert_eq!(s.lock().unwrap().gain.integer, -47);
    assert_eq!(backend.owner.load(Ordering::Acquire), 0);
}
