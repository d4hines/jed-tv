use super::*;
use crate::vendor::header_uuid;

fn header() -> [u8; 28] {
    let mut h = [0x5a; 28];
    h[..16].copy_from_slice(&AUDIO_TA.0);
    h[..4].reverse();
    h[4..6].reverse();
    h[6..8].reverse();
    h[16..20].copy_from_slice(&0u32.to_le_bytes());
    h
}
fn setup() -> (
    crate::tests::P,
    std::rc::Rc<std::cell::RefCell<crate::tests::State>>,
    Param,
    Param,
) {
    let (mut p, s) = provider(Limits::default());
    let h = allocate(&mut p, 48);
    let d = allocate(&mut p, 16);
    p.regions
        .get_mut(&(h as i32))
        .unwrap()
        .region
        .write(3, &header())
        .unwrap();
    let hp = Param {
        attr: 7,
        a: 3,
        b: 28,
        c: h,
    };
    let dp = Param {
        attr: 6,
        a: 2,
        b: 12,
        c: d,
    };
    (p, s, hp, dp)
}
fn read_header(p: &crate::tests::P, h: Param) -> [u8; 28] {
    let mut b = [0; 28];
    p.regions
        .get(&(h.c as i32))
        .unwrap()
        .region
        .read(h.a as usize, &mut b)
        .unwrap();
    b
}
#[test]
fn native_header_uuid_is_canonical() {
    assert_eq!(header_uuid(&header()), AUDIO_TA);
    assert_eq!(
        &header()[..8],
        &[0x8e, 0xe6, 0x8e, 0x77, 0xe0, 0x6e, 0xe8, 0x11]
    );
}
#[test]
fn size_changes_only_header_field_and_no_wire_parameters() {
    let (mut p, _, h, _) = setup();
    let mut params = [h];
    assert_eq!(p.handle(OBJECT_SIZE, &mut params), SUCCESS);
    assert_eq!(params, [h]);
    let mut expected = header();
    expected[20..24].copy_from_slice(&4u32.to_le_bytes());
    assert_eq!(read_header(&p, h), expected);
}
#[test]
fn reads_offset_and_short_eof_without_changing_size_metadata() {
    let (mut p, _, h, d) = setup();
    let mut hh = header();
    hh[16..20].copy_from_slice(&2u32.to_le_bytes());
    p.regions
        .get_mut(&(h.c as i32))
        .unwrap()
        .region
        .write(h.a as usize, &hh)
        .unwrap();
    let mut params = [h, d];
    assert_eq!(p.handle(LOAD_OBJECT, &mut params), SUCCESS);
    assert_eq!(params, [h, d]);
    hh[24..28].copy_from_slice(&2u32.to_le_bytes());
    assert_eq!(read_header(&p, h), hh);
    let mut out = [0; 16];
    p.regions
        .get(&(d.c as i32))
        .unwrap()
        .region
        .read(0, &mut out)
        .unwrap();
    assert_eq!(out, [0, 0, 3, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
}
#[test]
fn eof_and_beyond_eof_and_empty_output_are_successful() {
    for (offset, capacity) in [(4, 12), (100, 12), (0, 0)] {
        let (mut p, _, h, mut d) = setup();
        let mut hh = header();
        hh[16..20].copy_from_slice(&(offset as u32).to_le_bytes());
        p.regions
            .get_mut(&(h.c as i32))
            .unwrap()
            .region
            .write(h.a as usize, &hh)
            .unwrap();
        d.b = capacity;
        assert_eq!(p.handle(LOAD_OBJECT, &mut [h, d]), SUCCESS);
        assert_eq!(&read_header(&p, h)[24..28], &[0; 4]);
    }
}
#[test]
fn malformed_directions_counts_and_full_64bit_ranges_never_read_store() {
    let (mut p, s, h, d) = setup();
    for mut bad in [h; 7].into_iter().enumerate().map(|(i, mut h)| {
        match i {
            0 => h.attr = 5,
            1 => h.attr = 6,
            2 => h.b = 27,
            3 => h.c = u64::MAX,
            4 => h.a = u64::MAX,
            5 => h.b = 1 << 32,
            _ => h.attr |= 0x100,
        };
        h
    }) {
        assert_eq!(p.handle(LOAD_OBJECT, &mut [bad, d]), BAD_PARAMETERS);
        assert_eq!(
            p.handle(OBJECT_SIZE, std::slice::from_mut(&mut bad)),
            BAD_PARAMETERS
        );
    }
    for bad in [d; 5].into_iter().enumerate().map(|(i, mut d)| {
        match i {
            0 => d.attr = 5,
            1 => d.c = 999,
            2 => d.a = u64::MAX,
            3 => d.b = u64::MAX,
            _ => d.a = 1 << 32,
        };
        d
    }) {
        assert_eq!(p.handle(LOAD_OBJECT, &mut [h, bad]), BAD_PARAMETERS);
    }
    assert_eq!(p.handle(LOAD_OBJECT, &mut [h]), BAD_PARAMETERS);
    assert_eq!(p.handle(OBJECT_SIZE, &mut [h, d]), BAD_PARAMETERS);
    assert_eq!(s.borrow().loads, 0);
}
#[test]
fn rejects_negative_signed_offset_and_header_destination_overlap() {
    let (mut p, s, h, d) = setup();
    let mut hh = header();
    hh[16..20].copy_from_slice(&0x80000000u32.to_le_bytes());
    p.regions
        .get_mut(&(h.c as i32))
        .unwrap()
        .region
        .write(h.a as usize, &hh)
        .unwrap();
    assert_eq!(p.handle(LOAD_OBJECT, &mut [h, d]), BAD_PARAMETERS);
    let overlap = Param {
        attr: 6,
        a: 20,
        b: 8,
        c: h.c,
    };
    assert_eq!(p.handle(LOAD_OBJECT, &mut [h, overlap]), BAD_PARAMETERS);
    assert_eq!(s.borrow().loads, 0);
}
#[test]
fn store_failures_preserve_header_and_use_stock_command_errors() {
    let (mut p, s, h, d) = setup();
    s.borrow_mut().missing = true;
    assert_eq!(p.handle(OBJECT_SIZE, &mut [h]), GENERIC);
    assert_eq!(p.handle(LOAD_OBJECT, &mut [h, d]), ITEM_NOT_FOUND);
    assert_eq!(read_header(&p, h), header());
    s.borrow_mut().missing = false;
    s.borrow_mut().io_fail = true;
    assert_eq!(p.handle(LOAD_OBJECT, &mut [h, d]), ITEM_NOT_FOUND);
}
#[test]
fn unknown_uuid_and_oversized_store_output_fail_closed() {
    let (mut p, _, h, d) = setup();
    let mut hh = header();
    hh[0] ^= 1;
    p.regions
        .get_mut(&(h.c as i32))
        .unwrap()
        .region
        .write(h.a as usize, &hh)
        .unwrap();
    assert_eq!(p.handle(LOAD_OBJECT, &mut [h, d]), ITEM_NOT_FOUND);
    let (mut p, _, h, d) = setup();
    p.limits.ta_bytes = 3;
    assert_eq!(p.handle(LOAD_OBJECT, &mut [h, d]), OUT_OF_MEMORY);
    assert_eq!(read_header(&p, h), header());
}
#[test]
fn logical_allocation_not_mapping_rounding_controls_range() {
    let (mut p, s) = provider(Limits::default());
    s.borrow_mut().rounded = 4096;
    let id = allocate(&mut p, 16);
    let h = Param {
        attr: 7,
        a: 0,
        b: 28,
        c: id,
    };
    assert_eq!(p.handle(OBJECT_SIZE, &mut [h]), BAD_PARAMETERS);
    assert_eq!(s.borrow().loads, 0);
}
#[test]
fn failed_memory_write_does_not_publish_result_field() {
    let (mut p, s, h, d) = setup();
    s.borrow_mut().write_fail = true;
    assert_eq!(p.handle(LOAD_OBJECT, &mut [h, d]), GENERIC);
    assert_eq!(read_header(&p, h), header());
}
