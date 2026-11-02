use super::*;
use std::sync::Mutex;

#[derive(Default)]
struct MockState {
    calls: Vec<u32>,
    inputs: Vec<u32>,
    volume: VolumeDb,
    fail_attach: bool,
    fail_detach: bool,
}
#[derive(Clone)]
struct Mock(Arc<Mutex<MockState>>);
fn word(bytes: &[u8], at: usize) -> u32 {
    u32::from_ne_bytes(bytes[at..at + 4].try_into().unwrap())
}
impl Transport for Mock {
    fn transact(&self, request: u32, bytes: &mut [u8]) -> io::Result<()> {
        let mut s = self.0.lock().unwrap();
        s.calls.push(request);
        match request {
            0xc0803402 => bytes[..4].copy_from_slice(&0x81000000u32.to_ne_bytes()),
            0xc0081116 => bytes[4..8].copy_from_slice(&0x17000001u32.to_ne_bytes()),
            0xc010111e => bytes[12..16].copy_from_slice(&(s.inputs.len() as u32).to_ne_bytes()),
            0xc018111f => {
                let dest =
                    u64::from_ne_bytes(bytes[16..24].try_into().unwrap()) as usize as *mut u32;
                for (i, input) in s.inputs.iter().take(word(bytes, 12) as usize).enumerate() {
                    unsafe {
                        dest.add(i).write(*input);
                    }
                }
            }
            0xc00c111c => {
                s.inputs.push(word(bytes, 4));
                if s.fail_attach {
                    return Err(io::Error::other("partial attach failure"));
                }
            }
            0xc008111d => {
                if s.fail_detach {
                    return Err(io::Error::other("detach failure"));
                }
                s.inputs.retain(|n| *n != word(bytes, 4));
            }
            0xc010110a => {
                bytes[8..12].copy_from_slice(&s.volume.integer.to_ne_bytes());
                bytes[12..16].copy_from_slice(&s.volume.thousandths.to_ne_bytes());
            }
            0x40101109 => {
                s.volume = VolumeDb {
                    integer: word(bytes, 8) as i32,
                    thousandths: word(bytes, 12) as i32,
                }
            }
            0xc0083404 | 0xc0043405 | 0xc0043403 => (),
            _ => panic!("unexpected request {request:x}"),
        }
        Ok(())
    }
}
fn resources() -> (Resources<Mock, Mock>, Arc<Mutex<MockState>>) {
    let state = Arc::new(Mutex::new(MockState {
        volume: VolumeDb {
            integer: -47,
            thousandths: 0,
        },
        ..Default::default()
    }));
    let pcm = state.clone();
    let output = state.clone();
    (
        Resources::with_factories(
            move || Pcm::open_with(Mock(pcm.clone())),
            move || Aout::existing_speaker_with(Mock(output.clone())),
        ),
        state,
    )
}
fn run(resources: &mut Resources<Mock, Mock>, operations: &[u32]) {
    for operation in operations {
        resources
            .step(*operation)
            .unwrap_or_else(|e| panic!("operation {operation}: {}", e.error));
    }
}
#[test]
fn explicit_steps_restore_route_before_destroying_pcm() {
    let (mut r, s) = resources();
    run(&mut r, &[0, 1, 2, 3, 4, 5, 6, 7, 9]);
    let s = s.lock().unwrap();
    assert!(s.inputs.is_empty());
    assert_eq!(s.volume.integer, -47);
    let detach = s.calls.iter().position(|n| *n == 0xc008111d).unwrap();
    let stop = s.calls.iter().position(|n| *n == 0xc0043405).unwrap();
    let close = s.calls.iter().position(|n| *n == 0xc0043403).unwrap();
    assert!(detach < stop && stop < close);
    assert_eq!(s.calls.iter().filter(|n| **n == 0xc0043403).count(), 1);
}
#[test]
fn partial_attach_rollback_failure_is_reported_and_pcm_can_be_retained() {
    let (mut r, s) = resources();
    run(&mut r, &[0, 1]);
    {
        let mut s = s.lock().unwrap();
        s.fail_attach = true;
        s.fail_detach = true;
    }
    assert!(r.step(2).unwrap_err().uncertain);
    run(&mut r, &[8, 9]);
    drop(r);
    let s = s.lock().unwrap();
    assert_eq!(s.inputs, vec![0x81000000]);
    assert!(!s.calls.contains(&0xc0043403));
}
#[test]
fn close_is_rejected_while_an_owned_output_can_reference_pcm() {
    let (mut r, s) = resources();
    run(&mut r, &[0, 1, 2, 3, 4]);
    assert!(r.step(7).unwrap_err().uncertain);
    assert!(!s.lock().unwrap().calls.contains(&0xc0043403));
    run(&mut r, &[5, 6, 7, 9]);
}
#[test]
fn drop_of_unresolved_resources_does_not_guess_cleanup_order() {
    let (mut r, s) = resources();
    run(&mut r, &[0, 1, 2, 3, 4]);
    let count = s.lock().unwrap().calls.len();
    drop(r);
    assert_eq!(s.lock().unwrap().calls.len(), count);
}
