//! Bounded public request headers from the pinned V109 supplicant's SHM list.
//! Never reads RPMB frame payloads, MACs, keys, nonces, object contents or TA payloads.
use crate::wire::{self, Memory, Snapshot};
use std::{collections::BTreeSet, io};
const LOOKUP: [u8; 32] = [
    0xb0, 0xb5, 0x05, 0x46, 0x0a, 0x48, 0x78, 0x44, 0x00, 0xf0, 0x78, 0xfd, 0x09, 0x48, 0x78, 0x44,
    0x04, 0x68, 0x03, 0xe0, 0x20, 0x68, 0xa8, 0x42, 0x03, 0xd0, 0x64, 0x69, 0x00, 0x2c, 0xf9, 0xd1,
];
fn invalid(s: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, s)
}
fn u32at(b: &[u8], n: usize) -> u32 {
    u32::from_le_bytes(b[n..n + 4].try_into().unwrap())
}
fn bytes(m: &mut impl Memory, a: u32, n: usize) -> io::Result<Vec<u8>> {
    wire::range(a as u64, n as u64)?;
    let b = m.read(a, n)?;
    if b.len() != n {
        return Err(invalid("short header read"));
    }
    Ok(b)
}
#[derive(Debug, PartialEq, Eq)]
pub enum Header {
    Rpmb {
        command: u16,
        device: u16,
        blocks: u16,
        frame_type: Option<u16>,
    },
    Object {
        uuid: String,
        offset: u32,
        file_size: u32,
        bytes_read: u32,
    },
}
/// Validate the exact known lookup instructions before trusting its BSS layout.
pub fn profile(m: &mut impl Memory, maps: &str) -> io::Result<u32> {
    for line in maps.lines() {
        let f: Vec<_> = line.split_whitespace().collect();
        if f.len() != 6 || f[2] != "00000000" || !f[5].ends_with("/tee-supplicant") {
            continue;
        }
        let base = u32::from_str_radix(f[0].split('-').next().unwrap_or(""), 16)
            .map_err(|_| invalid("bad executable map"))?;
        let code = base
            .checked_add(0x24f8)
            .ok_or_else(|| invalid("profile overflow"))?;
        if bytes(m, code, LOOKUP.len())? != LOOKUP {
            return Err(invalid("supplicant does not match V109 lookup profile"));
        }
        return base
            .checked_add(0xb030)
            .ok_or_else(|| invalid("list address overflow"));
    }
    Err(invalid("V109 supplicant mapping unavailable"))
}
pub fn capture(m: &mut impl Memory, head: u32, s: &Snapshot) -> io::Result<Option<Header>> {
    let length = match s.word {
        1 => 6,
        100 | 101 => 28,
        _ => return Ok(None),
    };
    let p = s
        .params
        .iter()
        .find(|p| p.attr & 0x100 == 0)
        .ok_or_else(|| invalid("no request header"))?;
    if !(5..=7).contains(&p.attr) || p.b < length as u64 || p.c > i32::MAX as u64 {
        return Err(invalid("invalid header memref"));
    }
    let mut node = u32at(&bytes(m, head, 4)?, 0);
    let mut seen = BTreeSet::new();
    for _ in 0..64 {
        if node == 0 {
            return Err(invalid("header SHM is not owned by stock supplicant"));
        }
        if !seen.insert(node) {
            return Err(invalid("cyclic stock SHM list"));
        }
        let before = bytes(m, node, 24)?;
        if u32at(&before, 0) as u64 == p.c {
            let capacity = u32at(&before, 8) as u64;
            if p.a
                .checked_add(p.b)
                .filter(|end| *end <= capacity)
                .is_none()
            {
                return Err(invalid("header memref exceeds stock SHM"));
            }
            let address = u32at(&before, 4) as u64 + p.a;
            let address = wire::range(address, length as u64)?;
            let header = bytes(m, address, length)?;
            if bytes(m, node, 24)? != before {
                return Err(invalid("stock SHM changed during header read"));
            }
            return if s.word == 1 {
                Ok(Some(Header::Rpmb {
                    command: u16::from_le_bytes(header[0..2].try_into().unwrap()),
                    device: u16::from_le_bytes(header[2..4].try_into().unwrap()),
                    blocks: u16::from_le_bytes(header[4..6].try_into().unwrap()),
                    frame_type: if u16::from_le_bytes(header[0..2].try_into().unwrap()) == 0
                        && p.b >= 518
                    {
                        let operation = bytes(
                            m,
                            address
                                .checked_add(516)
                                .ok_or_else(|| invalid("frame operation overflow"))?,
                            2,
                        )?;
                        Some(u16::from_be_bytes(operation.try_into().unwrap()))
                    } else {
                        None
                    },
                }))
            } else {
                // Vendor object headers contain a native C UUID, unlike the
                // network-order UUID bytes packed in LOAD_TA value parameters.
                let uuid = format!(
                    "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{}",
                    u32at(&header, 0),
                    u16::from_le_bytes(header[4..6].try_into().unwrap()),
                    u16::from_le_bytes(header[6..8].try_into().unwrap()),
                    header[8],
                    header[9],
                    header[10..16]
                        .iter()
                        .map(|b| format!("{b:02x}"))
                        .collect::<String>()
                );
                Ok(Some(Header::Object {
                    uuid,
                    offset: u32at(&header, 16),
                    file_size: u32at(&header, 20),
                    bytes_read: u32at(&header, 24),
                }))
            };
        }
        node = u32at(&before, 20);
    }
    Err(invalid("stock SHM list exceeded trace bound"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wire::Param;
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
    fn setup(function: u32, length: u64) -> (Mem, Snapshot) {
        let mut m = Mem {
            data: vec![0; 65536],
            reads: vec![],
        };
        for (a, v) in [(32, 100u32), (100, 7), (104, 200), (108, 1024)] {
            m.data[a..a + 4].copy_from_slice(&v.to_le_bytes());
        }
        let s = Snapshot {
            word: function,
            count: 1,
            params: vec![Param {
                attr: 7,
                a: 0,
                b: length,
                c: 7,
            }],
        };
        (m, s)
    }
    #[test]
    fn rpmb_reads_only_six_byte_header() {
        let (mut m, s) = setup(1, 518);
        m.data[200..206].copy_from_slice(&[0, 0, 0, 0, 1, 0]);
        m.data[206..718].fill(0xa5);
        m.data[716..718].copy_from_slice(&[0, 2]);
        assert_eq!(
            capture(&mut m, 32, &s).unwrap(),
            Some(Header::Rpmb {
                command: 0,
                device: 0,
                blocks: 1,
                frame_type: Some(2)
            })
        );
        assert_eq!(m.reads, [(32, 4), (100, 24), (200, 6), (100, 24), (716, 2)]);
    }
    #[test]
    fn object_reads_only_public_header() {
        let (mut m, s) = setup(100, 28);
        m.data[200..216].copy_from_slice(&[
            0xad, 0x46, 0xa4, 0xa4, 0x63, 0x09, 0x02, 0x45, 0x9c, 0xb7, 0xdf, 0x5c, 0xec, 0xdd,
            0x7a, 0xe8,
        ]);
        assert!(
            matches!(capture(&mut m,32,&s).unwrap(),Some(Header::Object{uuid,..}) if uuid=="a4a446ad-0963-4502-9cb7-df5cecdd7ae8")
        );
        assert_eq!(m.reads, [(32, 4), (100, 24), (200, 28), (100, 24)]);
    }
    #[test]
    fn unknown_function_never_reads_memory() {
        let (mut m, s) = setup(0, 28);
        assert_eq!(capture(&mut m, 32, &s).unwrap(), None);
        assert!(m.reads.is_empty());
    }
    #[test]
    fn malformed_memrefs_never_read_payloads() {
        for p in [
            Param {
                attr: 1,
                a: 0,
                b: 28,
                c: 7,
            },
            Param {
                attr: 7,
                a: 0,
                b: 27,
                c: 7,
            },
            Param {
                attr: 7,
                a: u64::MAX,
                b: 28,
                c: 7,
            },
            Param {
                attr: 7,
                a: 0,
                b: 28,
                c: u64::MAX,
            },
        ] {
            let (mut m, mut s) = setup(100, 28);
            s.params[0] = p;
            assert!(capture(&mut m, 32, &s).is_err());
            assert!(!m.reads.iter().any(|&(a, _)| a == 200));
        }
    }
    #[test]
    fn cycles_and_missing_ids_are_bounded() {
        let (mut m, mut s) = setup(1, 6);
        s.params[0].c = 8;
        m.data[120..124].copy_from_slice(&100u32.to_le_bytes());
        assert!(capture(&mut m, 32, &s).is_err());
        assert_eq!(m.reads.len(), 2);
    }
    #[test]
    fn profile_requires_exact_code_and_path() {
        let (mut m, _) = setup(1, 6);
        m.data[0x24f8..0x2518].copy_from_slice(&LOOKUP);
        let maps = "00000000-00010000 r-xp 00000000 00:01 2 /vendor/bin/tee-supplicant";
        assert_eq!(profile(&mut m, maps).unwrap(), 0xb030);
        m.data[0x24f8] ^= 1;
        assert!(profile(&mut m, maps).is_err());
    }
}
