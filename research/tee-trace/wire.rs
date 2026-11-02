//! Linux TEE UAPI, matched to V109's ARM32 ioctl dispatcher.
//! Generic snapshots never dereference memrefs. The separate headers module
//! reads only fixed public fields for the verified V109 RPC profile.
use std::io;
pub const RECV: u32 = 0x8010_a406;
pub const SEND: u32 = 0x8010_a407;
pub const MAX_PARAMS: u32 = 64;
pub const MAX_BUFFER: u64 = 8 + 32 * MAX_PARAMS as u64;

pub trait Memory {
    fn read(&mut self, address: u32, length: usize) -> io::Result<Vec<u8>>;
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Envelope {
    pub address: u32,
    pub length: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Param {
    pub attr: u64,
    pub a: u64,
    pub b: u64,
    pub c: u64,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Snapshot {
    pub word: u32,
    pub count: u32,
    pub params: Vec<Param>,
}
fn invalid(s: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, s)
}
fn u32at(b: &[u8], n: usize) -> u32 {
    u32::from_le_bytes(b[n..n + 4].try_into().unwrap())
}
fn u64at(b: &[u8], n: usize) -> u64 {
    u64::from_le_bytes(b[n..n + 8].try_into().unwrap())
}
pub fn range(address: u64, length: u64) -> io::Result<u32> {
    if address == 0
        || address > u32::MAX as u64
        || length == 0
        || address
            .checked_add(length - 1)
            .filter(|&end| end <= u32::MAX as u64)
            .is_none()
    {
        return Err(invalid("invalid ARM32 memory range"));
    }
    Ok(address as u32)
}
fn read<M: Memory>(m: &mut M, address: u32, size: usize) -> io::Result<Vec<u8>> {
    range(address as u64, size as u64)?;
    let b = m.read(address, size)?;
    if b.len() != size {
        return Err(invalid("short tracee read"));
    }
    Ok(b)
}
pub fn envelope<M: Memory>(m: &mut M, address: u32) -> io::Result<Envelope> {
    let b = read(m, address, 16)?;
    let length = u64at(&b, 8);
    if !(8..=MAX_BUFFER).contains(&length) {
        return Err(invalid("TEE buffer length outside trace bound"));
    }
    Ok(Envelope {
        address: range(u64at(&b, 0), length)?,
        length,
    })
}
/// `values=false` captures RECV capacity without reading uninitialized params.
/// `capacity` is the entry count; receive output may never exceed it.
pub fn snapshot<M: Memory>(
    m: &mut M,
    e: Envelope,
    values: bool,
    capacity: Option<u32>,
) -> io::Result<Snapshot> {
    let header = read(m, e.address, 8)?;
    let count = u32at(&header, 4);
    if count > MAX_PARAMS || 8 + 32 * count as u64 > e.length || capacity.is_some_and(|n| count > n)
    {
        return Err(invalid("TEE parameter count outside entry buffer/capacity"));
    }
    let mut params = Vec::new();
    if values && count > 0 {
        let b = read(
            m,
            e.address
                .checked_add(8)
                .ok_or_else(|| invalid("parameter address overflow"))?,
            count as usize * 32,
        )?;
        for p in b.chunks_exact(32) {
            params.push(Param {
                attr: u64at(p, 0),
                a: u64at(p, 8),
                b: u64at(p, 16),
                c: u64at(p, 24),
            });
        }
    }
    Ok(Snapshot {
        word: u32at(&header, 0),
        count,
        params,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Mem {
        data: Vec<u8>,
        reads: Vec<(u32, usize)>,
    }
    impl Memory for Mem {
        fn read(&mut self, a: u32, n: usize) -> io::Result<Vec<u8>> {
            self.reads.push((a, n));
            self.data
                .get(a as usize..a as usize + n)
                .map(|s| s.to_vec())
                .ok_or_else(|| invalid("unmapped"))
        }
    }
    fn mem(count: u32) -> Mem {
        let mut data = vec![0; 4096];
        data[16..24].copy_from_slice(&128u64.to_le_bytes());
        data[24..32].copy_from_slice(&MAX_BUFFER.to_le_bytes());
        data[128..132].copy_from_slice(&6u32.to_le_bytes());
        data[132..136].copy_from_slice(&count.to_le_bytes());
        for (i, n) in [7u64, 0xffff_ffff_ffff_ffff, 128, 42].iter().enumerate() {
            data[136 + i * 8..144 + i * 8].copy_from_slice(&n.to_le_bytes());
        }
        Mem {
            data,
            reads: vec![],
        }
    }
    #[test]
    fn requests_are_ior_size16() {
        assert_eq!(RECV, (2 << 30) | (16 << 16) | (0xa4 << 8) | 6);
        assert_eq!(SEND, RECV + 1);
    }
    #[test]
    fn recv_entry_never_reads_uninitialized_params() {
        let mut m = mem(4);
        let e = envelope(&mut m, 16).unwrap();
        let s = snapshot(&mut m, e, false, None).unwrap();
        assert_eq!(s.count, 4);
        assert!(s.params.is_empty());
        assert_eq!(m.reads, vec![(16, 16), (128, 8)]);
    }
    #[test]
    fn preserves_64bit_metadata_without_dereferencing_memrefs() {
        let mut m = mem(1);
        let e = envelope(&mut m, 16).unwrap();
        let s = snapshot(&mut m, e, true, Some(1)).unwrap();
        assert_eq!(
            s.params[0],
            Param {
                attr: 7,
                a: u64::MAX,
                b: 128,
                c: 42
            }
        );
        assert_eq!(m.reads.last(), Some(&(136, 32)));
    }
    #[test]
    fn bounds_reject_wide_pointers_and_wrapping_ranges() {
        assert!(range(0x1_0000_0000, 8).is_err());
        assert!(range(0xffff_fffc, 8).is_err());
        assert!(range(0, 8).is_err());
        assert_eq!(range(0xffff_fffc, 4).unwrap(), 0xffff_fffc);
    }
    #[test]
    fn rejects_oversized_buffer_before_header_read() {
        let mut m = mem(0);
        m.data[24..32].copy_from_slice(&(MAX_BUFFER + 1).to_le_bytes());
        assert!(envelope(&mut m, 16).is_err());
        assert_eq!(m.reads.len(), 1);
    }
    #[test]
    fn rejects_expanded_recv_output() {
        let mut m = mem(2);
        let e = envelope(&mut m, 16).unwrap();
        assert!(snapshot(&mut m, e, true, Some(1)).is_err());
        assert_eq!(m.reads.len(), 2);
    }
    #[test]
    fn rejects_count_and_short_buffer() {
        for n in [65, u32::MAX] {
            let mut m = mem(n);
            let e = envelope(&mut m, 16).unwrap();
            assert!(snapshot(&mut m, e, true, None).is_err());
        }
        let mut m = mem(1);
        m.data[24..32].copy_from_slice(&8u64.to_le_bytes());
        let e = envelope(&mut m, 16).unwrap();
        assert!(snapshot(&mut m, e, true, None).is_err());
    }
    #[test]
    fn zero_params_valid() {
        let mut m = mem(0);
        let e = envelope(&mut m, 16).unwrap();
        assert!(snapshot(&mut m, e, true, None).unwrap().params.is_empty());
    }
    #[test]
    fn short_read_is_error() {
        struct Short;
        impl Memory for Short {
            fn read(&mut self, _: u32, _: usize) -> io::Result<Vec<u8>> {
                Ok(vec![])
            }
        }
        assert!(envelope(&mut Short, 16).is_err());
    }
}
