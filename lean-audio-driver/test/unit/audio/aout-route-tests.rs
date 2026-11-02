//! Wire layout of the MI AOUT requests. Routing policy, snapshot consistency
//! and restoration are Lean's (test/lean/tv/audio).
use super::*;
use crate::mi_pcm::Transport;
use std::cell::RefCell;
const OUT: u32 = 0x17000001;
const OURS: u32 = 0x81000100;
struct Mock(RefCell<Vec<(u32, Vec<u8>)>>, fn(u32, &mut [u8]) -> io::Result<()>);
impl Transport for Mock {
    fn transact(&self, request: u32, wire: &mut [u8]) -> io::Result<()> {
        self.0.borrow_mut().push((request, wire.to_vec()));
        (self.1)(request, wire)
    }
}
fn get(w: &[u8], i: usize) -> u32 {
    u32::from_ne_bytes(w[i..i + 4].try_into().unwrap())
}
fn put(w: &mut [u8], i: usize, n: u32) {
    w[i..i + 4].copy_from_slice(&n.to_ne_bytes())
}
#[test]
fn speaker_lookup_asks_for_device_4_and_returns_the_raw_handle() {
    let mock = Mock(RefCell::new(vec![]), |_, w| {
        put(w, 4, OUT);
        Ok(())
    });
    assert_eq!(speaker(&mock).unwrap(), OUT);
    let calls = mock.0.borrow();
    assert_eq!((calls[0].0, calls[0].1.len(), get(&calls[0].1, 0)), (GET_HANDLE, 8, 4));
}
#[test]
fn count_and_connected_use_the_input_direction_and_fill_the_caller_array() {
    let mock = Mock(RefCell::new(vec![]), |r, w| {
        if r == GET_COUNT {
            put(w, 12, 2);
        } else {
            let pointer = u64::from_ne_bytes(w[16..24].try_into().unwrap()) as usize as *mut u32;
            unsafe {
                pointer.write(OURS);
                pointer.add(1).write(OURS + 1);
            }
        }
        Ok(())
    });
    assert_eq!(count(&mock, OUT).unwrap(), 2);
    assert_eq!(connected(&mock, OUT, 2).unwrap(), vec![OURS, OURS + 1]);
    assert_eq!(connected(&mock, OUT, 0).unwrap(), vec![], "a zero count makes no request");
    let calls = mock.0.borrow();
    assert_eq!(calls.len(), 2);
    assert_eq!((calls[0].0, calls[0].1.len()), (GET_COUNT, 16));
    assert_eq!((get(&calls[0].1, 0), get(&calls[0].1, 4)), (OUT, 1));
    assert_eq!((calls[1].0, calls[1].1.len()), (GET_CONNECTED, 24));
    assert_eq!((get(&calls[1].1, 0), get(&calls[1].1, 4), get(&calls[1].1, 8), get(&calls[1].1, 12)), (OUT, 1, 0, 2));
}
#[test]
fn connect_and_disconnect_carry_output_then_input() {
    let mock = Mock(RefCell::new(vec![]), |_, _| Ok(()));
    connect(&mock, OUT, OURS).unwrap();
    disconnect(&mock, OUT, OURS).unwrap();
    let calls = mock.0.borrow();
    assert_eq!((calls[0].0, calls[0].1.len()), (CONNECT, 12));
    assert_eq!((get(&calls[0].1, 0), get(&calls[0].1, 4), get(&calls[0].1, 8)), (OUT, OURS, 0));
    assert_eq!((calls[1].0, calls[1].1.len()), (DISCONNECT, 8));
    assert_eq!((get(&calls[1].1, 0), get(&calls[1].1, 4)), (OUT, OURS));
}
#[test]
fn volume_uses_path_4_and_the_exact_pair_layout() {
    let mock = Mock(RefCell::new(vec![]), |r, w| {
        if r == GET_VOLUME {
            put(w, 8, (-12i32) as u32);
            put(w, 12, (-500i32) as u32);
        }
        Ok(())
    });
    assert_eq!(
        volume_db(&mock, OUT).unwrap(),
        VolumeDb {
            integer: -12,
            thousandths: -500
        }
    );
    set_volume_db(&mock, OUT, VolumeDb { integer: 0, thousandths: 0 }).unwrap();
    let calls = mock.0.borrow();
    assert_eq!((calls[0].0, calls[0].1.len(), get(&calls[0].1, 0), get(&calls[0].1, 4)), (GET_VOLUME, 16, OUT, 4));
    assert_eq!((calls[1].0, calls[1].1.len(), get(&calls[1].1, 0), get(&calls[1].1, 4)), (SET_VOLUME, 16, OUT, 4));
    assert_eq!((get(&calls[1].1, 8), get(&calls[1].1, 12)), (0, 0));
}
#[test]
fn errors_pass_through_untouched() {
    let mock = Mock(RefCell::new(vec![]), |r, _| {
        Err(io::Error::other(crate::mi_pcm::VendorError { request: r, status: 9 }))
    });
    let error = speaker(&mock).unwrap_err();
    let vendor = error.get_ref().unwrap().downcast_ref::<crate::mi_pcm::VendorError>().unwrap();
    assert_eq!((vendor.request, vendor.status), (GET_HANDLE, 9));
}
