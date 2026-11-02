use super::*;
use std::{cell::RefCell, rc::Rc};
#[derive(Default)]
struct State {
    events: Vec<String>,
    copies: Vec<(i32, usize, Vec<u8>)>,
    next: i32,
    rounded: usize,
    alloc_fail: bool,
    write_fail: bool,
    loads: usize,
    ta: Vec<u8>,
    missing: bool,
    io_fail: bool,
}
struct FakeAllocator(Rc<RefCell<State>>);
impl Drop for FakeAllocator {
    fn drop(&mut self) {
        self.0.borrow_mut().events.push("allocator-drop".into());
    }
}
struct FakeRegion {
    state: Rc<RefCell<State>>,
    id: i32,
    bytes: usize,
    memory: Vec<u8>,
}
impl Drop for FakeRegion {
    fn drop(&mut self) {
        self.state
            .borrow_mut()
            .events
            .push(format!("drop:{}", self.id));
    }
}
impl Region for FakeRegion {
    fn id(&self) -> i32 {
        self.id
    }
    fn len(&self) -> usize {
        self.bytes
    }
    fn read(&self, offset: usize, data: &mut [u8]) -> io::Result<()> {
        let source = self
            .memory
            .get(
                offset
                    ..offset
                        .checked_add(data.len())
                        .ok_or_else(|| io::Error::other("overflow"))?,
            )
            .ok_or_else(|| io::Error::other("bounds"))?;
        data.copy_from_slice(source);
        Ok(())
    }
    fn clear(&mut self) -> io::Result<()> {
        self.memory.fill(0);
        self.state
            .borrow_mut()
            .events
            .push(format!("clear:{}", self.id));
        Ok(())
    }
    fn write(&mut self, offset: usize, data: &[u8]) -> io::Result<()> {
        let mut s = self.state.borrow_mut();
        if s.write_fail {
            return Err(io::Error::other("write failure"));
        }
        self.memory
            .get_mut(
                offset
                    ..offset
                        .checked_add(data.len())
                        .ok_or_else(|| io::Error::other("overflow"))?,
            )
            .ok_or_else(|| io::Error::other("bounds"))?
            .copy_from_slice(data);
        s.copies.push((self.id, offset, data.to_vec()));
        Ok(())
    }
}
impl Allocator for FakeAllocator {
    type Region = FakeRegion;
    fn allocate(&mut self, bytes: usize) -> io::Result<FakeRegion> {
        let mut s = self.0.borrow_mut();
        s.events.push(format!("alloc:{bytes}"));
        if s.alloc_fail {
            return Err(io::Error::other("alloc failure"));
        }
        let id = s.next;
        s.next += 1;
        Ok(FakeRegion {
            state: self.0.clone(),
            id,
            bytes: bytes + s.rounded,
            memory: vec![0; bytes + s.rounded],
        })
    }
}
struct Store(Rc<RefCell<State>>);
impl TaStore for Store {
    fn load_object(&mut self, uuid: Uuid, limit: usize) -> Result<Vec<u8>, LoadError> {
        if uuid != AUDIO_TA {
            return Err(LoadError::NotFound);
        }
        let data = self.load(uuid, limit)?;
        Ok(data)
    }
    fn load(&mut self, uuid: Uuid, _: usize) -> Result<Vec<u8>, LoadError> {
        assert_eq!(uuid, AUDIO_TA);
        let mut s = self.0.borrow_mut();
        s.loads += 1;
        if s.missing {
            return Err(LoadError::NotFound);
        }
        if s.io_fail {
            return Err(LoadError::Io(io::Error::other("read failure")));
        }
        Ok(s.ta.clone())
    }
}
type P = Provider<FakeAllocator, Store>;
fn provider(limits: Limits) -> (P, Rc<RefCell<State>>) {
    let s = Rc::new(RefCell::new(State {
        next: 7,
        ta: vec![1, 2, 3, 4],
        ..State::default()
    }));
    (
        Provider::new(FakeAllocator(s.clone()), Store(s.clone()), limits).unwrap(),
        s,
    )
}
fn value_param() -> Param {
    Param {
        attr: 1,
        a: 9,
        b: 16,
        c: 99,
    }
}
fn uuid_param() -> Param {
    Param {
        attr: 1,
        a: u64::from_le_bytes(AUDIO_TA.0[..8].try_into().unwrap()),
        b: u64::from_le_bytes(AUDIO_TA.0[8..].try_into().unwrap()),
        c: 0,
    }
}
fn request(id: u64, offset: u64, bytes: u64) -> [Param; 2] {
    [
        uuid_param(),
        Param {
            attr: 6,
            a: offset,
            b: bytes,
            c: id,
        },
    ]
}
fn allocate(p: &mut P, bytes: u64) -> u64 {
    let mut q = [Param {
        b: bytes,
        ..value_param()
    }];
    assert_eq!(p.handle(SHM_ALLOC, &mut q), SUCCESS);
    q[0].c
}
#[test]
fn network_uuid_and_vendor_name() {
    let p = uuid_param();
    assert_eq!(Uuid::from_value(&p), AUDIO_TA);
    assert_eq!(AUDIO_TA.to_string(), "778ee68e-6ee0-11e8-adc0-fa7ae01bbebc");
    assert_eq!(
        AUDIO_TA.vendor_filename(),
        "778ee68e-6ee0-11e8-adc0fa7ae01bbebc.ta"
    );
}
#[test]
fn allocation_only_changes_c_and_free_uses_b() {
    let (mut p, s) = provider(Limits::default());
    let mut q = [value_param()];
    assert_eq!(p.handle(SHM_ALLOC, &mut q), SUCCESS);
    assert_eq!(
        q[0],
        Param {
            c: 7,
            ..value_param()
        }
    );
    assert_eq!(p.owned_bytes(), 16);
    let mut free = [Param {
        b: 7,
        ..value_param()
    }];
    assert_eq!(p.handle(SHM_FREE, &mut free), SUCCESS);
    assert_eq!(p.owned_regions(), 0);
    assert_eq!(p.owned_bytes(), 0);
    assert_eq!(s.borrow().events, vec!["alloc:16", "clear:7", "drop:7"]);
}
#[test]
fn unknown_rpc_is_explicit_and_has_no_side_effect() {
    let (mut p, s) = provider(Limits::default());
    for op in [1, 2, 3, 4, 5, 8, 10, u32::MAX] {
        assert_eq!(p.handle(op, &mut []), NOT_SUPPORTED)
    }
    assert!(s.borrow().events.is_empty());
    assert_eq!(s.borrow().loads, 0);
}
#[test]
fn malformed_parameters_never_touch_backend() {
    let (mut p, s) = provider(Limits::default());
    for attr in [0, 4, 5, 7, 0x101, 0x1_0000_0001, u64::MAX] {
        let mut q = [Param {
            attr,
            ..value_param()
        }];
        assert_eq!(p.handle(SHM_ALLOC, &mut q), BAD_PARAMETERS);
        assert_eq!(p.handle(SHM_FREE, &mut q), BAD_PARAMETERS);
    }
    assert_eq!(p.handle(SHM_ALLOC, &mut []), BAD_PARAMETERS);
    assert_eq!(p.handle(LOAD_TA, &mut [uuid_param()]), BAD_PARAMETERS);
    assert!(s.borrow().events.is_empty());
}
#[test]
fn quotas_checked_before_allocation() {
    let limits = Limits {
        max_regions: 1,
        region_bytes: 16,
        total_bytes: 16,
        ta_bytes: 16,
    };
    let (mut p, s) = provider(limits);
    for size in [0, 17, u64::MAX] {
        let mut q = [Param {
            b: size,
            ..value_param()
        }];
        assert_ne!(p.handle(SHM_ALLOC, &mut q), SUCCESS)
    }
    allocate(&mut p, 16);
    let mut q = [Param {
        b: 1,
        ..value_param()
    }];
    assert_eq!(p.handle(SHM_ALLOC, &mut q), OUT_OF_MEMORY);
    assert_eq!(s.borrow().events, vec!["alloc:16", "clear:7"]);
}
#[test]
fn backend_rounding_over_budget_is_released() {
    let limits = Limits {
        region_bytes: 16,
        total_bytes: 16,
        ..Limits::default()
    };
    let (mut p, s) = provider(limits);
    s.borrow_mut().rounded = 1;
    let mut q = [Param {
        b: 16,
        ..value_param()
    }];
    assert_eq!(p.handle(SHM_ALLOC, &mut q), OUT_OF_MEMORY);
    assert_eq!(s.borrow().events, vec!["alloc:16", "drop:7"]);
    assert_eq!(p.owned_bytes(), 0);
}
#[test]
fn collision_never_replaces_owned_region() {
    let (mut p, s) = provider(Limits::default());
    allocate(&mut p, 16);
    s.borrow_mut().next = 7;
    let mut q = [value_param()];
    assert_eq!(p.handle(SHM_ALLOC, &mut q), GENERIC);
    assert_eq!(p.owned_regions(), 1);
    assert_eq!(p.owned_bytes(), 16);
    assert_eq!(s.borrow().events.last().unwrap(), "drop:7");
}
#[test]
fn failed_allocation_does_not_publish_id() {
    let (mut p, s) = provider(Limits::default());
    s.borrow_mut().alloc_fail = true;
    let mut q = [value_param()];
    assert_eq!(p.handle(SHM_ALLOC, &mut q), OUT_OF_MEMORY);
    assert_eq!(q[0].c, 99);
    assert_eq!(p.owned_regions(), 0);
}
#[test]
fn free_rejects_unowned_and_double_free() {
    let (mut p, s) = provider(Limits::default());
    let id = allocate(&mut p, 16);
    let mut q = [Param {
        b: id,
        ..value_param()
    }];
    assert_eq!(p.handle(SHM_FREE, &mut q), SUCCESS);
    assert_eq!(p.handle(SHM_FREE, &mut q), BAD_PARAMETERS);
    q[0].b = u64::MAX;
    assert_eq!(p.handle(SHM_FREE, &mut q), BAD_PARAMETERS);
    assert_eq!(
        s.borrow()
            .events
            .iter()
            .filter(|s| s.starts_with("drop:"))
            .count(),
        1
    );
}
#[test]
fn null_output_size_query_has_no_mapping_write() {
    let (mut p, s) = provider(Limits::default());
    let mut q = request(u64::MAX, 0, 0);
    assert_eq!(p.handle(LOAD_TA, &mut q), SUCCESS);
    assert_eq!(q[1].b, 4);
    assert!(s.borrow().copies.is_empty());
    assert_eq!(s.borrow().loads, 1);
}
#[test]
fn missing_input_mapping_is_rejected() {
    let (mut p, s) = provider(Limits::default());
    let mut q = request(123, 0, 0);
    q[1].attr = 5;
    assert_eq!(p.handle(LOAD_TA, &mut q), BAD_PARAMETERS);
    assert_eq!(s.borrow().loads, 0);
}
#[test]
fn short_buffer_reports_needed_size_without_partial_copy() {
    let (mut p, s) = provider(Limits::default());
    let id = allocate(&mut p, 16);
    let mut q = request(id, 0, 3);
    assert_eq!(p.handle(LOAD_TA, &mut q), SHORT_BUFFER);
    assert_eq!(q[1].b, 4);
    assert!(s.borrow().copies.is_empty());
}
#[test]
fn load_copies_exact_ta_at_requested_offset() {
    let (mut p, s) = provider(Limits::default());
    let id = allocate(&mut p, 16);
    let mut q = request(id, 5, 11);
    assert_eq!(p.handle(LOAD_TA, &mut q), SUCCESS);
    assert_eq!(q[1].b, 4);
    assert_eq!(s.borrow().copies, vec![(7, 5, vec![1, 2, 3, 4])]);
}
#[test]
fn invalid_existing_mapping_ranges_fail_before_store_read() {
    let (mut p, s) = provider(Limits::default());
    let id = allocate(&mut p, 16);
    for (offset, size) in [(17, 0), (15, 2), (u64::MAX, 1), (1, u64::MAX)] {
        let mut q = request(id, offset, size);
        assert_eq!(p.handle(LOAD_TA, &mut q), BAD_PARAMETERS)
    }
    assert_eq!(s.borrow().loads, 0);
}
#[test]
fn store_errors_are_not_success() {
    let (mut p, s) = provider(Limits::default());
    let mut q = request(u64::MAX, 0, 0);
    s.borrow_mut().missing = true;
    assert_eq!(p.handle(LOAD_TA, &mut q), ITEM_NOT_FOUND);
    s.borrow_mut().missing = false;
    s.borrow_mut().io_fail = true;
    assert_eq!(p.handle(LOAD_TA, &mut q), GENERIC);
    assert_eq!(q[1].b, 0);
}
#[test]
fn excessive_store_data_is_rejected() {
    let limits = Limits {
        ta_bytes: 3,
        ..Limits::default()
    };
    let (mut p, _) = provider(limits);
    assert_eq!(
        p.handle(LOAD_TA, &mut request(u64::MAX, 0, 0)),
        OUT_OF_MEMORY
    );
}
#[test]
fn mapping_copy_error_is_reported() {
    let (mut p, s) = provider(Limits::default());
    let id = allocate(&mut p, 16);
    s.borrow_mut().write_fail = true;
    assert_eq!(p.handle(LOAD_TA, &mut request(id, 0, 16)), GENERIC);
}
#[test]
fn dropping_provider_releases_every_owned_region() {
    let (mut p, s) = provider(Limits::default());
    allocate(&mut p, 16);
    allocate(&mut p, 8);
    drop(p);
    assert_eq!(
        s.borrow()
            .events
            .iter()
            .filter(|s| s.starts_with("drop:"))
            .count(),
        2
    );
}
#[test]
fn directory_store_is_allowlisted_and_length_checked() {
    let dir = std::env::temp_dir().join(format!("g06-tee-rpc-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let file = dir.join(AUDIO_TA.vendor_filename());
    std::fs::write(&file, [1, 2, 3, 4]).unwrap();
    let mut store = DirectoryStore::new(dir.clone(), BTreeMap::from([(AUDIO_TA, 4)]));
    assert_eq!(store.load(AUDIO_TA, 4).unwrap(), vec![1, 2, 3, 4]);
    assert!(matches!(
        store.load(Uuid([0; 16]), 4),
        Err(LoadError::NotFound)
    ));
    assert!(matches!(store.load(AUDIO_TA, 3), Err(LoadError::TooLarge)));
    std::fs::write(&file, [1, 2]).unwrap();
    assert!(matches!(store.load(AUDIO_TA, 4), Err(LoadError::Changed)));
    std::fs::remove_file(file).unwrap();
    std::fs::remove_dir(dir).unwrap();
}

#[test]
fn rounded_mapping_does_not_expand_logical_capacity() {
    let (mut p, s) = provider(Limits::default());
    s.borrow_mut().rounded = 4096;
    let id = allocate(&mut p, 16);
    assert_eq!(p.owned_bytes(), 4112);
    assert_eq!(p.handle(LOAD_TA, &mut request(id, 15, 2)), BAD_PARAMETERS);
    assert_eq!(s.borrow().loads, 0);
}

#[test]
fn wire_layout_and_ioctl_match_stock() {
    assert_eq!(std::mem::size_of::<Param>(), 32);
    assert_eq!(std::mem::offset_of!(Param, a), 8);
    assert_eq!(std::mem::offset_of!(Param, c), 24);
    assert_eq!(std::mem::size_of::<ShmAllocRequest>(), 16);
    assert_eq!(std::mem::offset_of!(ShmAllocRequest, flags), 8);
    assert_eq!(std::mem::offset_of!(ShmAllocRequest, id), 12);
    assert_eq!(SHM_ALLOC_IOCTL, (3 << 30) | (16 << 16) | (0xa4 << 8) | 1);
}

#[test]
fn all_mappings_drop_before_allocator_device() {
    let (mut p, s) = provider(Limits::default());
    allocate(&mut p, 16);
    allocate(&mut p, 8);
    drop(p);
    assert_eq!(s.borrow().events.last().unwrap(), "allocator-drop");
}

#[test]
fn allocation_size_high_word_is_never_truncated_to_arm32() {
    let (mut p, state) = provider(Limits::default());
    let mut request = [Param {
        b: (1u64 << 32) + 128,
        ..value_param()
    }];
    assert_eq!(p.handle(SHM_ALLOC, &mut request), OUT_OF_MEMORY);
    assert!(state.borrow().events.is_empty());
    assert_eq!(request[0].b, (1u64 << 32) + 128);
}

#[path = "vendor-tests.rs"]
mod vendor;
