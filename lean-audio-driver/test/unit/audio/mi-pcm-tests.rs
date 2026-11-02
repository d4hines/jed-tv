//! Wire layout of the MI PCM requests: the bytes each function sends and the
//! bytes it reads back. Lifecycle and ownership are Lean's (test/lean/tv/audio).
use super::*;
use std::cell::RefCell;
const HANDLE: u32 = 0x81000100;
struct Mock(RefCell<Vec<(u32, Vec<u8>)>>, fn(u32, &mut [u8]) -> io::Result<()>);
impl Transport for Mock {
    fn transact(&self, request: u32, wire: &mut [u8]) -> io::Result<()> {
        self.0.borrow_mut().push((request, wire.to_vec()));
        (self.1)(request, wire)
    }
}
fn u32at(wire: &[u8], at: usize) -> u32 {
    u32::from_ne_bytes(wire[at..at + 4].try_into().unwrap())
}
fn put(wire: &mut [u8], at: usize, value: u32) {
    wire[at..at + 4].copy_from_slice(&value.to_ne_bytes())
}
#[test]
fn open_sends_the_stock_48k_stereo_request_and_returns_the_raw_handle() {
    let mock = Mock(RefCell::new(vec![]), |_, w| {
        put(w, 0, HANDLE);
        Ok(())
    });
    assert_eq!(open(&mock).unwrap(), HANDLE);
    let calls = mock.0.borrow();
    let (request, wire) = &calls[0];
    assert_eq!(*request, OPEN);
    assert_eq!(wire.len(), 128);
    assert_eq!(&wire[4..18], b"HW DMA Reader1");
    assert_eq!((wire[68], wire[69]), (0, 0), "nonblocking, no mixing");
    assert_eq!(u32at(wire, 72), 48000);
    assert_eq!((wire[76], wire[77], wire[78]), (2, 16, 0));
    assert_eq!((wire[120], u32at(wire, 124)), (1, 100), "duration control 100 ms");
}
#[test]
fn zero_handle_is_returned_raw_for_lean_to_reject() {
    let mock = Mock(RefCell::new(vec![]), |_, _| Ok(()));
    assert_eq!(open(&mock).unwrap(), 0);
}
#[test]
fn write_sends_buffer_pointer_and_length_and_returns_the_raw_count() {
    let mock = Mock(RefCell::new(vec![]), |_, w| {
        put(w, 48, 12);
        Ok(())
    });
    let pcm = [7u8; 16];
    assert_eq!(write(&mock, HANDLE, &pcm).unwrap(), 12);
    let calls = mock.0.borrow();
    let (request, wire) = &calls[0];
    assert_eq!(*request, WRITE);
    assert_eq!(wire.len(), 56);
    assert_eq!(u32at(wire, 0), HANDLE);
    assert_eq!(u64::from_ne_bytes(wire[8..16].try_into().unwrap()), pcm.as_ptr() as usize as u64);
    assert_eq!(u32at(wire, 16), 16);
    assert!(wire[24..48].iter().all(|b| *b == 0), "pts, flags and metadata are zero");
}
#[test]
fn start_stop_close_carry_only_the_handle() {
    let mock = Mock(RefCell::new(vec![]), |_, w| {
        w[0] = 0xff; // a driver scribbling on its argument must not matter
        Ok(())
    });
    start(&mock, HANDLE).unwrap();
    stop(&mock, HANDLE).unwrap();
    close(&mock, HANDLE).unwrap();
    let calls = mock.0.borrow();
    assert_eq!(calls[0].0, START);
    assert_eq!((calls[0].1.len(), u32at(&calls[0].1, 0), u32at(&calls[0].1, 4)), (8, HANDLE, 0));
    assert_eq!((calls[1].0, calls[1].1.len(), u32at(&calls[1].1, 0)), (STOP, 4, HANDLE));
    assert_eq!((calls[2].0, calls[2].1.len(), u32at(&calls[2].1, 0)), (CLOSE, 4, HANDLE));
}
#[test]
fn buffer_query_reads_exact_output_shape() {
    let mock = Mock(RefCell::new(vec![]), |_, w| {
        assert_eq!(w.len(), 24);
        assert_eq!(u32at(w, 0), HANDLE);
        assert_eq!(u32at(w, 4), 0);
        let pointer = u64::from_ne_bytes(w[16..24].try_into().unwrap()) as usize as *mut BufferInfo;
        unsafe {
            *pointer = BufferInfo {
                capacity_bytes: 19200,
                queued_bytes: 800,
                free_bytes: 18400,
            }
        };
        Ok(())
    });
    let info = buffer_info(&mock, HANDLE).unwrap();
    assert_eq!(
        info,
        BufferInfo {
            capacity_bytes: 19200,
            queued_bytes: 800,
            free_bytes: 18400
        }
    );
    assert_eq!(mock.0.borrow()[0].0, GET_ATTR);
}
#[test]
fn vendor_status_and_errno_are_distinguishable() {
    let vendor = io::Error::other(VendorError {
        request: WRITE,
        status: 7,
    });
    assert_eq!(
        vendor.get_ref().unwrap().downcast_ref::<VendorError>().map(|v| (v.request, v.status)),
        Some((WRITE, 7))
    );
    assert_eq!(format!("{vendor}"), "MI request 0xc0383408: status 7");
    assert!(io::Error::from_raw_os_error(5).get_ref().is_none());
}
