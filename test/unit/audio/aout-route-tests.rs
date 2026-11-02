use super::*;
use std::{cell::RefCell, rc::Rc};
const OUT: u32 = 0x17000001;
const OURS: u32 = 0x81000100;
const OLD: u32 = 0x81000101;
const OTHER: u32 = 0x81000102;
#[derive(Default)]
struct State {
    inputs: Vec<u32>,
    calls: Vec<u32>,
    connect_error: bool,
    connect_partial: bool,
    disconnect_error: bool,
    replace: bool,
    bad_handle: bool,
    oversize: bool,
    changing: bool,
    fail_count: bool,
    invalid_boolean: bool,
    fail_status: bool,
    volume_db: Option<VolumeDb>,
    volume_sets: Vec<VolumeDb>,
    volume_set_fail: bool,
    volume_restore_fail: bool,
    volume_ignore_first: bool,
}
#[derive(Clone)]
struct Mock(Rc<RefCell<State>>);
fn get(w: &[u8], i: usize) -> u32 {
    u32::from_ne_bytes(w[i..i + 4].try_into().unwrap())
}
fn put(w: &mut [u8], i: usize, n: u32) {
    w[i..i + 4].copy_from_slice(&n.to_ne_bytes())
}
fn error(r: u32) -> io::Error {
    io::Error::other(crate::mi_pcm::VendorError {
        request: r,
        status: 9,
    })
}
impl Transport for Mock {
    fn transact(&self, r: u32, w: &mut [u8]) -> io::Result<()> {
        let mut s = self.0.borrow_mut();
        s.calls.push(r);
        match r {
            0xc00c110e | 0xc00c1108 => {
                assert_eq!(w.len(), 12);
                assert_eq!(get(w, 0), OUT);
                assert_eq!(get(w, 4), if r == 0xc00c1108 { 4 } else { 0 });
                if s.fail_status {
                    return Err(error(r));
                }
                w[8] = if r == 0xc00c1108 {
                    37
                } else if s.invalid_boolean {
                    2
                } else {
                    1
                };
            }
            0xc010110a => {
                assert_eq!(w.len(), 16);
                assert_eq!(get(w, 0), OUT);
                assert_eq!(get(w, 4), 4);
                let db = s.volume_db.unwrap_or(VolumeDb {
                    integer: -12,
                    thousandths: -500,
                });
                put(w, 8, db.integer as u32);
                put(w, 12, db.thousandths as u32);
            }
            0x40101109 => {
                assert_eq!(w.len(), 16);
                assert_eq!(get(w, 0), OUT);
                assert_eq!(get(w, 4), 4);
                let value = VolumeDb {
                    integer: get(w, 8) as i32,
                    thousandths: get(w, 12) as i32,
                };
                s.volume_sets.push(value);
                if !(s.volume_ignore_first && s.volume_sets.len() == 1) {
                    s.volume_db = Some(value)
                }
                if (s.volume_set_fail && s.volume_sets.len() == 1)
                    || (s.volume_restore_fail && s.volume_sets.len() > 1)
                {
                    return Err(error(r));
                }
            }
            0xc0481115 => {
                assert_eq!(w.len(), 72);
                assert_eq!(get(w, 0), OUT);
                let attr = get(w, 4);
                assert!([0, 6].contains(&attr));
                assert!(w[8..16].iter().all(|n| *n == 0));
                assert!(w[24..72].iter().all(|n| *n == 0));
                if s.fail_status {
                    return Err(error(r));
                }
                let ptr = u64::from_ne_bytes(w[16..24].try_into().unwrap()) as usize as *mut u8;
                unsafe {
                    ptr.write(if s.invalid_boolean {
                        3
                    } else if attr == 6 {
                        1
                    } else {
                        0
                    })
                };
            }
            GET_HANDLE => {
                assert_eq!(w.len(), 8);
                assert_eq!(get(w, 0), 4);
                let h = if s.bad_handle { 0 } else { OUT };
                put(w, 4, h)
            }
            GET_COUNT => {
                assert_eq!(w.len(), 16);
                assert_eq!(get(w, 0), OUT);
                assert_eq!((get(w, 4), get(w, 8)), (1, 0));
                if s.fail_count {
                    return Err(error(r));
                }
                let count = if s.oversize {
                    33
                } else if s.changing {
                    (s.calls.len() % 2) as u32
                } else {
                    s.inputs.len() as u32
                };
                put(w, 12, count)
            }
            GET_CONNECTED => {
                assert_eq!(w.len(), 24);
                assert_eq!(get(w, 0), OUT);
                assert_eq!((get(w, 4), get(w, 8)), (1, 0));
                let count = get(w, 12) as usize;
                let ptr = u64::from_ne_bytes(w[16..24].try_into().unwrap()) as usize as *mut u32;
                for (i, n) in s.inputs.iter().take(count).enumerate() {
                    unsafe { ptr.add(i).write(*n) }
                }
            }
            CONNECT => {
                assert_eq!(w.len(), 12);
                assert_eq!(get(w, 0), OUT);
                assert_eq!(&w[8..12], &[0; 4]);
                if !s.connect_error || s.connect_partial {
                    if s.replace {
                        s.inputs.clear()
                    }
                    let n = get(w, 4);
                    if !s.inputs.contains(&n) {
                        s.inputs.push(n)
                    }
                }
                if s.connect_error {
                    return Err(error(r));
                }
            }
            DISCONNECT => {
                assert_eq!(w.len(), 8);
                assert_eq!(get(w, 0), OUT);
                if s.disconnect_error {
                    return Err(error(r));
                }
                s.inputs.retain(|n| *n != get(w, 4))
            }
            _ => panic!("unowned/forbidden AOUT operation {r:x}"),
        }
        Ok(())
    }
}
fn setup(inputs: &[u32]) -> (Aout<Mock>, Rc<RefCell<State>>) {
    let state = Rc::new(RefCell::new(State {
        inputs: inputs.to_vec(),
        ..Default::default()
    }));
    let aout = Aout::existing_speaker_with(Mock(state.clone())).unwrap();
    (aout, state)
}
fn input(n: u32) -> InputHandle {
    InputHandle::from_raw(n).unwrap()
}
fn calls(s: &Rc<RefCell<State>>, r: u32) -> usize {
    s.borrow().calls.iter().filter(|n| **n == r).count()
}
#[test]
fn snapshot_is_typed_and_read_only() {
    let (a, s) = setup(&[OLD, OTHER]);
    let snap = a.snapshot().unwrap();
    assert_eq!(snap.output.raw(), OUT);
    assert_eq!(
        snap.inputs.iter().map(|n| n.raw()).collect::<Vec<_>>(),
        vec![OLD, OTHER]
    );
    assert_eq!(calls(&s, CONNECT) + calls(&s, DISCONNECT), 0)
}
#[test]
fn empty_route_attaches_and_restores() {
    let (mut a, s) = setup(&[]);
    let route = a.route(input(OURS), RoutePolicy::RequireEmpty).unwrap();
    assert!(route.before().inputs.is_empty());
    assert_eq!(route.snapshot().unwrap().inputs, vec![input(OURS)]);
    route.restore().unwrap();
    assert!(s.borrow().inputs.is_empty());
    assert_eq!((calls(&s, CONNECT), calls(&s, DISCONNECT)), (1, 1))
}
#[test]
fn borrowed_route_never_disconnects() {
    let (mut a, s) = setup(&[OLD, OURS]);
    a.route(input(OURS), RoutePolicy::RequireEmpty)
        .unwrap()
        .restore()
        .unwrap();
    assert_eq!(s.borrow().inputs, vec![OLD, OURS]);
    assert_eq!(calls(&s, CONNECT) + calls(&s, DISCONNECT), 0)
}
#[test]
fn nonempty_default_is_refused_without_write() {
    let (mut a, s) = setup(&[OLD]);
    assert!(a.route(input(OURS), RoutePolicy::RequireEmpty).is_err());
    assert_eq!(calls(&s, CONNECT) + calls(&s, DISCONNECT), 0)
}
#[test]
fn explicit_policy_preserves_old_connections() {
    let (mut a, s) = setup(&[OLD]);
    a.route(input(OURS), RoutePolicy::PreserveExisting)
        .unwrap()
        .restore()
        .unwrap();
    assert_eq!(s.borrow().inputs, vec![OLD]);
    assert_eq!((calls(&s, CONNECT), calls(&s, DISCONNECT)), (1, 1))
}
#[test]
fn cleanup_does_not_remove_foreign_new_connection() {
    let (mut a, s) = setup(&[OLD]);
    let route = a.route(input(OURS), RoutePolicy::PreserveExisting).unwrap();
    s.borrow_mut().inputs.push(OTHER);
    drop(route);
    assert_eq!(s.borrow().inputs, vec![OLD, OTHER]);
}
#[test]
fn cleanup_reconnects_missing_original() {
    let (mut a, s) = setup(&[OLD]);
    let route = a.route(input(OURS), RoutePolicy::PreserveExisting).unwrap();
    s.borrow_mut().inputs.retain(|n| *n != OLD);
    route.restore().unwrap();
    assert_eq!(s.borrow().inputs, vec![OLD]);
    assert_eq!(calls(&s, CONNECT), 2);
}
#[test]
fn failed_connect_preserves_vendor_error_and_does_not_disconnect_old() {
    let (mut a, s) = setup(&[OLD]);
    s.borrow_mut().connect_error = true;
    let error = a
        .route(input(OURS), RoutePolicy::PreserveExisting)
        .err()
        .unwrap();
    let attach = error
        .get_ref()
        .unwrap()
        .downcast_ref::<AttachError>()
        .unwrap();
    assert!(attach.rollback.is_none());
    let vendor = attach
        .cause
        .get_ref()
        .unwrap()
        .downcast_ref::<crate::mi_pcm::VendorError>()
        .unwrap();
    assert_eq!(vendor.status, 9);
    assert_eq!(s.borrow().inputs, vec![OLD]);
    assert_eq!(calls(&s, DISCONNECT), 0);
}
#[test]
fn failed_connect_partial_side_effect_is_rolled_back() {
    let (mut a, s) = setup(&[]);
    {
        let mut b = s.borrow_mut();
        b.connect_error = true;
        b.connect_partial = true;
    }
    assert!(a.route(input(OURS), RoutePolicy::RequireEmpty).is_err());
    assert!(s.borrow().inputs.is_empty());
    assert_eq!(calls(&s, DISCONNECT), 1);
}
#[test]
fn destructive_connect_is_detected_and_original_restored() {
    let (mut a, s) = setup(&[OLD]);
    s.borrow_mut().replace = true;
    assert!(a.route(input(OURS), RoutePolicy::PreserveExisting).is_err());
    assert_eq!(s.borrow().inputs, vec![OLD]);
    assert_eq!(calls(&s, DISCONNECT), 1);
}
#[test]
fn disconnect_failure_is_reported_without_drop_retry() {
    let (mut a, s) = setup(&[]);
    let route = a.route(input(OURS), RoutePolicy::RequireEmpty).unwrap();
    s.borrow_mut().disconnect_error = true;
    let error = route.restore().unwrap_err();
    let restore = error
        .get_ref()
        .unwrap()
        .downcast_ref::<RestoreError>()
        .unwrap();
    assert!(restore.failures.len() >= 2);
    assert_eq!(calls(&s, DISCONNECT), 1);
    assert_eq!(s.borrow().inputs, vec![OURS]);
}
#[test]
fn cleanup_snapshot_failure_performs_no_blind_write() {
    let (mut a, s) = setup(&[]);
    let route = a.route(input(OURS), RoutePolicy::RequireEmpty).unwrap();
    s.borrow_mut().fail_count = true;
    assert!(route.restore().is_err());
    assert_eq!(calls(&s, DISCONNECT), 0);
}
#[test]
fn rejects_oversize_zero_and_duplicate_snapshots() {
    for entries in [vec![0], vec![OLD, OLD]] {
        let (a, _) = setup(&entries);
        assert!(a.snapshot().is_err())
    }
    let (a, s) = setup(&[]);
    s.borrow_mut().oversize = true;
    assert!(a.snapshot().is_err());
    assert_eq!(calls(&s, GET_CONNECTED), 0);
}
#[test]
fn changing_count_is_bounded() {
    let (a, s) = setup(&[]);
    s.borrow_mut().changing = true;
    assert_eq!(a.snapshot().unwrap_err().kind(), io::ErrorKind::WouldBlock);
    assert_eq!(calls(&s, GET_COUNT), 6);
}
#[test]
fn zero_input_and_missing_output_are_rejected() {
    assert!(InputHandle::from_raw(0).is_err());
    let state = Rc::new(RefCell::new(State {
        bad_handle: true,
        ..Default::default()
    }));
    assert!(Aout::existing_speaker_with(Mock(state.clone())).is_err());
    assert_eq!(state.borrow().calls, vec![GET_HANDLE]);
}

#[test]
fn typed_status_queries_use_exact_getter_layouts_only() {
    let (a, s) = setup(&[]);
    assert!(a.muted().unwrap());
    assert!(!a.amplifier_muted().unwrap());
    assert!(a.output_enabled().unwrap());
    assert_eq!(a.volume_index().unwrap(), 37);
    assert_eq!(
        a.volume_db().unwrap(),
        VolumeDb {
            integer: -12,
            thousandths: -500
        }
    );
    assert_eq!(calls(&s, CONNECT) + calls(&s, DISCONNECT), 0);
}
#[test]
fn getter_errors_and_invalid_booleans_are_not_silently_false() {
    let (a, s) = setup(&[]);
    s.borrow_mut().invalid_boolean = true;
    assert_eq!(a.muted().unwrap_err().kind(), io::ErrorKind::InvalidData);
    assert!(a.output_enabled().is_err());
    s.borrow_mut().fail_status = true;
    assert!(a.volume_index().is_err());
    assert!(a.amplifier_muted().is_err());
}

fn db(integer: i32) -> VolumeDb {
    VolumeDb {
        integer,
        thousandths: 0,
    }
}
#[test]
fn volume_lease_verifies_target_and_restores_original() {
    let (a, s) = setup(&[]);
    s.borrow_mut().volume_db = Some(db(-47));
    let lease = a.volume_owned(db(-27)).unwrap();
    assert_eq!(lease.original(), db(-47));
    assert_eq!(lease.current().unwrap(), db(-27));
    lease.restore().unwrap();
    assert_eq!(s.borrow().volume_db, Some(db(-47)));
    assert_eq!(s.borrow().volume_sets, vec![db(-27), db(-47)]);
}
#[test]
fn volume_noop_does_not_set_hardware() {
    let (a, s) = setup(&[]);
    s.borrow_mut().volume_db = Some(db(-27));
    drop(a.volume_owned(db(-27)).unwrap());
    assert!(s.borrow().volume_sets.is_empty());
}
#[test]
fn volume_set_failure_rolls_back_partial_change() {
    let (a, s) = setup(&[]);
    {
        let mut state = s.borrow_mut();
        state.volume_db = Some(db(-47));
        state.volume_set_fail = true;
    }
    let error = a.volume_owned(db(-27)).err().unwrap();
    let takeover = error
        .get_ref()
        .unwrap()
        .downcast_ref::<VolumeTakeoverError>()
        .unwrap();
    assert!(takeover.rollback.is_none());
    assert_eq!(s.borrow().volume_db, Some(db(-47)));
}
#[test]
fn volume_readback_mismatch_rolls_back() {
    let (a, s) = setup(&[]);
    {
        let mut state = s.borrow_mut();
        state.volume_db = Some(db(-47));
        state.volume_ignore_first = true;
    }
    assert!(a.volume_owned(db(-27)).is_err());
    assert_eq!(s.borrow().volume_sets, vec![db(-27), db(-47)]);
}
#[test]
fn volume_restore_error_is_reported_without_retry() {
    let (a, s) = setup(&[]);
    s.borrow_mut().volume_db = Some(db(-47));
    let lease = a.volume_owned(db(-27)).unwrap();
    s.borrow_mut().volume_restore_fail = true;
    assert!(lease.restore().is_err());
    assert_eq!(s.borrow().volume_sets.len(), 2);
}
#[test]
fn volume_invalid_target_never_writes() {
    for target in [
        db(1),
        db(-97),
        VolumeDb {
            integer: 0,
            thousandths: i32::MIN,
        },
    ] {
        let (a, s) = setup(&[]);
        assert!(a.volume_owned(target).is_err());
        assert!(s.borrow().volume_sets.is_empty());
    }
}
