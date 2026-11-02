//! Just enough flattened-device-tree reading for /chosen/bootargs and the
//! RAM range. QEMU places the DTB at the base of RAM for ELF kernels.

pub struct Info {
    pub bootargs: &'static str,
    pub mem: Option<(u64, u64)>,
}

fn be32(base: usize, off: usize) -> u32 {
    u32::from_be(unsafe { core::ptr::read_unaligned((base + off) as *const u32) })
}

fn cstr(base: usize) -> &'static [u8] {
    let mut n = 0;
    while unsafe { *((base + n) as *const u8) } != 0 {
        n += 1;
    }
    unsafe { core::slice::from_raw_parts(base as *const u8, n) }
}

pub fn parse(base: usize) -> Option<Info> {
    if be32(base, 0) != 0xd00d_feed {
        return None;
    }
    let (structs, strings) = (base + be32(base, 8) as usize, base + be32(base, 12) as usize);
    let mut info = Info { bootargs: "", mem: None };
    let (mut off, mut depth, mut node): (usize, usize, &[u8]) = (0, 0, b"");
    loop {
        let tok = be32(structs, off);
        off += 4;
        match tok {
            1 => {
                let name = cstr(structs + off);
                off += (name.len() + 4) & !3;
                depth += 1;
                if depth == 2 {
                    node = name;
                }
            }
            2 => depth -= 1,
            3 => {
                let (len, nameoff) = (be32(structs, off) as usize, be32(structs, off + 4) as usize);
                let val = structs + off + 8;
                off += 8 + ((len + 3) & !3);
                let prop = cstr(strings + nameoff);
                if depth == 2 && node == b"chosen" && prop == b"bootargs" && len > 0 {
                    let s = unsafe { core::slice::from_raw_parts(val as *const u8, len - 1) };
                    info.bootargs = core::str::from_utf8(s).unwrap_or("");
                }
                if depth == 2 && node.starts_with(b"memory@") && prop == b"reg" && len >= 16 {
                    let a = ((be32(val, 0) as u64) << 32) | be32(val, 4) as u64;
                    let s = ((be32(val, 8) as u64) << 32) | be32(val, 12) as u64;
                    info.mem = Some((a, s));
                }
            }
            4 => {}
            _ => break,
        }
    }
    Some(info)
}

/// `key=value` lookup in a kernel-style command line.
pub fn arg<'a>(cmdline: &'a str, key: &str) -> Option<&'a str> {
    cmdline.split_ascii_whitespace().find_map(|w| {
        let (k, v) = w.split_once('=')?;
        (k == key).then_some(v)
    })
}
