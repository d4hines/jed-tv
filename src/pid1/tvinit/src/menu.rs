//! Scalar ABI to the Lean menu reducer. Rendering and OS effects stay in Rust.
extern "C" { fn tv_menu_step(state: u64, shape: u64, key: u32, repeated: u32) -> u64; }
pub fn step(row: usize, cols: &mut [usize], widths: &[usize;3], key: u32, repeated: bool) -> (usize,u8) {
    assert!(!cols.is_empty() && cols.len() <= 3 && row < cols.len());
    let mut state=row as u64;
    let mut shape=cols.len() as u64;
    for i in 0..3 {
        assert!(widths[i] <= u16::MAX as usize);
        shape |= (widths[i] as u64) << (8+16*i);
        if let Some(&c)=cols.get(i) {
            assert!(c < widths[i]);
            state |= (c as u64) << (8+16*i);
        }
    }
    let next=unsafe { tv_menu_step(state,shape,key,u32::from(repeated)) };
    let row=(next & 255) as usize;
    assert!(row < cols.len());
    for (i,c) in cols.iter_mut().enumerate() {
        *c=((next >> (8+16*i)) & 65535) as usize;
        assert!(*c < widths[i]);
    }
    (row,(next >> 56) as u8)
}
