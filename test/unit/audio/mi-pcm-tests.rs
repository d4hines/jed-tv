use super::*;
use std::{cell::RefCell, collections::VecDeque, rc::Rc};
const OPEN: u32 = 0xc0803402;
const START: u32 = 0xc0083404;
const STOP: u32 = 0xc0043405;
const CLOSE: u32 = 0xc0043403;
const WRITE: u32 = 0xc0383408;
const ATTR: u32 = 0xc018340b;
const HANDLE: u32 = 0x81000100;
type Action = Box<dyn FnOnce(&mut [u8]) -> io::Result<()>>;
struct Step {
    request: u32,
    action: Action,
}
#[derive(Clone)]
struct Mock(Rc<RefCell<(VecDeque<Step>, Vec<u32>)>>);
impl Mock {
    fn new(steps: Vec<Step>) -> Self {
        Self(Rc::new(RefCell::new((steps.into(), vec![]))))
    }
    fn finished(&self, expected: &[u32]) {
        let state = self.0.borrow();
        assert!(state.0.is_empty(), "unused script actions");
        assert_eq!(state.1, expected);
    }
}
impl Transport for Mock {
    fn transact(&self, request: u32, wire: &mut [u8]) -> io::Result<()> {
        let step = {
            let mut state = self.0.borrow_mut();
            state.1.push(request);
            state.0.pop_front().expect("unexpected ioctl")
        };
        assert_eq!(request, step.request, "wrong lifecycle order");
        (step.action)(wire)
    }
}
fn step(request: u32, action: impl FnOnce(&mut [u8]) -> io::Result<()> + 'static) -> Step {
    Step {
        request,
        action: Box::new(action),
    }
}
fn u32at(wire: &[u8], at: usize) -> u32 {
    u32::from_ne_bytes(wire[at..at + 4].try_into().unwrap())
}
fn put(wire: &mut [u8], at: usize, value: u32) {
    wire[at..at + 4].copy_from_slice(&value.to_ne_bytes())
}
fn open() -> Step {
    step(OPEN, |w| {
        assert_eq!(w.len(), 128);
        assert_eq!(&w[4..18], b"HW DMA Reader1");
        assert_eq!(w[68], 0);
        assert_eq!(w[69], 0);
        assert_eq!(u32at(w, 72), 48000);
        assert_eq!(&w[76..80], &[2, 16, 0, 0]);
        assert!(w[80..120].iter().all(|b| *b == 0));
        assert_eq!(w[120], 1);
        assert_eq!(u32at(w, 124), 100);
        put(w, 0, HANDLE);
        Ok(())
    })
}
fn handle(request: u32) -> Step {
    step(request, move |w| {
        assert_eq!(u32at(w, 0), HANDLE);
        Ok(())
    })
}
fn error(request: u32, status: i32) -> Step {
    step(request, move |_| vendor_status(request, status))
}
fn check_vendor(error: &io::Error, request: u32, status: i32) {
    let value = error
        .get_ref()
        .unwrap()
        .downcast_ref::<VendorError>()
        .unwrap();
    assert_eq!((value.request, value.status), (request, status));
}
#[test]
fn failed_open_does_not_close_unowned_handle() {
    let mock = Mock::new(vec![step(OPEN, |w| {
        put(w, 0, HANDLE);
        vendor_status(OPEN, 7)
    })]);
    let error = Pcm::open_with(mock.clone()).err().unwrap();
    check_vendor(&error, OPEN, 7);
    mock.finished(&[OPEN]);
}
#[test]
fn zero_handle_success_is_rejected_without_close() {
    let mock = Mock::new(vec![step(OPEN, |_| Ok(()))]);
    assert!(Pcm::open_with(mock.clone()).is_err());
    mock.finished(&[OPEN]);
}
#[test]
fn unused_stream_is_closed_without_stop() {
    let mock = Mock::new(vec![open(), handle(CLOSE)]);
    drop(Pcm::open_with(mock.clone()).unwrap());
    mock.finished(&[OPEN, CLOSE]);
}
#[test]
fn start_is_idempotent_and_finish_closes_original_handle_once() {
    let mock = Mock::new(vec![
        open(),
        handle(START),
        step(STOP, |w| {
            assert_eq!(u32at(w, 0), HANDLE);
            put(w, 0, 42);
            Ok(())
        }),
        handle(CLOSE),
    ]);
    let mut pcm = Pcm::open_with(mock.clone()).unwrap();
    pcm.start().unwrap();
    pcm.start().unwrap();
    pcm.finish().unwrap();
    mock.finished(&[OPEN, START, STOP, CLOSE]);
}
#[test]
fn failed_start_still_closes_but_does_not_stop() {
    let mock = Mock::new(vec![open(), error(START, 2), handle(CLOSE)]);
    let mut pcm = Pcm::open_with(mock.clone()).unwrap();
    check_vendor(&pcm.start().unwrap_err(), START, 2);
    drop(pcm);
    mock.finished(&[OPEN, START, CLOSE]);
}
#[test]
fn stop_failure_does_not_skip_close_or_allow_driver_handle_mutation() {
    let mock = Mock::new(vec![
        open(),
        handle(START),
        step(STOP, |w| {
            put(w, 0, 0);
            vendor_status(STOP, 9)
        }),
        handle(CLOSE),
    ]);
    let mut pcm = Pcm::open_with(mock.clone()).unwrap();
    pcm.start().unwrap();
    check_vendor(&pcm.finish().unwrap_err(), STOP, 9);
    mock.finished(&[OPEN, START, STOP, CLOSE]);
}
#[test]
fn both_cleanup_errors_are_retained_without_retry() {
    let mock = Mock::new(vec![
        open(),
        handle(START),
        error(STOP, 9),
        error(CLOSE, 10),
    ]);
    let mut pcm = Pcm::open_with(mock.clone()).unwrap();
    pcm.start().unwrap();
    let error = pcm.finish().unwrap_err();
    let both = error
        .get_ref()
        .unwrap()
        .downcast_ref::<CleanupError>()
        .unwrap();
    check_vendor(&both.stop, STOP, 9);
    check_vendor(&both.close, CLOSE, 10);
    mock.finished(&[OPEN, START, STOP, CLOSE]);
}
#[test]
fn close_error_is_returned_without_retry() {
    let mock = Mock::new(vec![open(), error(CLOSE, 7)]);
    let pcm = Pcm::open_with(mock.clone()).unwrap();
    check_vendor(&pcm.finish().unwrap_err(), CLOSE, 7);
    mock.finished(&[OPEN, CLOSE]);
}
#[test]
fn write_before_start_does_not_touch_hardware() {
    let mock = Mock::new(vec![open(), handle(CLOSE)]);
    let mut pcm = Pcm::open_with(mock.clone()).unwrap();
    assert_eq!(
        pcm.write(&[0; 4]).unwrap_err().kind(),
        io::ErrorKind::InvalidInput
    );
    pcm.finish().unwrap();
    mock.finished(&[OPEN, CLOSE]);
}
fn write(expected: Vec<u8>, accepted: u32) -> Step {
    step(WRITE, move |w| {
        assert_eq!(w.len(), 56);
        assert_eq!(u32at(w, 0), HANDLE);
        assert_eq!(u32at(w, 16), expected.len() as u32);
        let pointer = u64::from_ne_bytes(w[8..16].try_into().unwrap()) as usize as *const u8;
        let data = unsafe { std::slice::from_raw_parts(pointer, expected.len()) };
        assert_eq!(data, expected);
        put(w, 48, accepted);
        Ok(())
    })
}
#[test]
fn short_and_zero_writes_preserve_every_byte_and_back_off() {
    let data: Vec<u8> = (0..16).collect();
    let mock = Mock::new(vec![
        open(),
        handle(START),
        write(data.clone(), 4),
        write(data[4..].to_vec(), 0),
        write(data[4..].to_vec(), 8),
        write(data[12..].to_vec(), 4),
        handle(STOP),
        handle(CLOSE),
    ]);
    let mut pcm = Pcm::open_with(mock.clone()).unwrap();
    pcm.start().unwrap();
    let mut stalls = 0;
    pcm.write_all_with(&data, || Ok(()), || stalls += 1)
        .unwrap();
    assert_eq!(stalls, 1);
    pcm.finish().unwrap();
    mock.finished(&[OPEN, START, WRITE, WRITE, WRITE, WRITE, STOP, CLOSE]);
}
#[test]
fn impossible_driver_write_counts_are_rejected() {
    for accepted in [3, 8] {
        let mock = Mock::new(vec![
            open(),
            handle(START),
            write(vec![0; 4], accepted),
            handle(STOP),
            handle(CLOSE),
        ]);
        let mut pcm = Pcm::open_with(mock.clone()).unwrap();
        pcm.start().unwrap();
        assert!(pcm.write(&[0; 4]).is_err());
        pcm.finish().unwrap();
        mock.finished(&[OPEN, START, WRITE, STOP, CLOSE]);
    }
}
#[test]
fn progress_does_not_extend_the_write_deadline() {
    let mock = Mock::new(vec![
        open(),
        handle(START),
        write(vec![0; 16], 4),
        write(vec![0; 12], 4),
        handle(STOP),
        handle(CLOSE),
    ]);
    let mut pcm = Pcm::open_with(mock.clone()).unwrap();
    pcm.start().unwrap();
    let mut attempts = 0;
    let result = pcm.write_all_with(
        &[0; 16],
        || {
            attempts += 1;
            if attempts > 2 {
                Err(io::Error::new(io::ErrorKind::TimedOut, "test deadline"))
            } else {
                Ok(())
            }
        },
        || panic!("no zero writes"),
    );
    assert_eq!(result.unwrap_err().kind(), io::ErrorKind::TimedOut);
    pcm.finish().unwrap();
    mock.finished(&[OPEN, START, WRITE, WRITE, STOP, CLOSE]);
}
#[test]
fn expired_real_deadline_makes_no_write() {
    let mock = Mock::new(vec![open(), handle(START), handle(STOP), handle(CLOSE)]);
    let mut pcm = Pcm::open_with(mock.clone()).unwrap();
    pcm.start().unwrap();
    assert_eq!(
        pcm.write_all_until(&[0; 4], Instant::now())
            .unwrap_err()
            .kind(),
        io::ErrorKind::TimedOut
    );
    pcm.finish().unwrap();
    mock.finished(&[OPEN, START, STOP, CLOSE]);
}
#[test]
fn vendor_write_errors_preserve_status_and_cleanup() {
    let mock = Mock::new(vec![
        open(),
        handle(START),
        error(WRITE, 11),
        handle(STOP),
        handle(CLOSE),
    ]);
    let mut pcm = Pcm::open_with(mock.clone()).unwrap();
    pcm.start().unwrap();
    check_vendor(&pcm.write(&[0; 4]).unwrap_err(), WRITE, 11);
    pcm.finish().unwrap();
    mock.finished(&[OPEN, START, WRITE, STOP, CLOSE]);
}
#[test]
fn syscall_errno_is_preserved() {
    let mock = Mock::new(vec![
        open(),
        step(START, |_| Err(io::Error::from_raw_os_error(5))),
        handle(CLOSE),
    ]);
    let mut pcm = Pcm::open_with(mock.clone()).unwrap();
    assert_eq!(pcm.start().unwrap_err().raw_os_error(), Some(5));
    pcm.finish().unwrap();
    mock.finished(&[OPEN, START, CLOSE]);
}
#[test]
fn empty_and_unaligned_buffers_make_no_write() {
    let mock = Mock::new(vec![open(), handle(START), handle(STOP), handle(CLOSE)]);
    let mut pcm = Pcm::open_with(mock.clone()).unwrap();
    pcm.start().unwrap();
    assert_eq!(pcm.write(&[]).unwrap(), 0);
    assert!(pcm.write(&[0; 3]).is_err());
    pcm.finish().unwrap();
    mock.finished(&[OPEN, START, STOP, CLOSE]);
}
#[test]
fn buffer_query_reads_exact_output_shape() {
    let mock = Mock::new(vec![
        open(),
        step(ATTR, |w| {
            assert_eq!(w.len(), 24);
            assert_eq!(u32at(w, 0), HANDLE);
            assert_eq!(u32at(w, 4), 0);
            let pointer =
                u64::from_ne_bytes(w[16..24].try_into().unwrap()) as usize as *mut BufferInfo;
            unsafe {
                *pointer = BufferInfo {
                    capacity_bytes: 19200,
                    queued_bytes: 800,
                    free_bytes: 18400,
                }
            };
            Ok(())
        }),
        handle(CLOSE),
    ]);
    let pcm = Pcm::open_with(mock.clone()).unwrap();
    let info = pcm.buffer_info().unwrap();
    assert_eq!(
        (info.capacity_bytes, info.queued_bytes, info.free_bytes),
        (19200, 800, 18400)
    );
    pcm.finish().unwrap();
    mock.finished(&[OPEN, ATTR, CLOSE]);
}
