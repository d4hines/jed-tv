use super::*;
#[test]
fn requires_receive_ioctl_and_a_real_descriptor() {
    assert_eq!(receive_fd(b"54 0x6 0x8010a406 0x1234 0 0"), Some(6));
    assert_eq!(receive_fd(b"54 6 2148574214 0"), Some(6));
    assert_eq!(receive_fd(b"0 0x3 0x8010a406 0xbe93db28"), Some(3));
    assert_eq!(receive_fd(b"54 6 0x8010a407 0"), None);
    assert_eq!(receive_fd(b"3 6 0x8010a406 0"), None);
    assert_eq!(receive_fd(b"running"), None);
}

fn old_receive(bytes: &[u8]) -> Option<u32> {
    let text = std::str::from_utf8(bytes).ok()?;
    let mut fields = text.split_whitespace();
    if !matches!(fields.next()?, "0" | "54") { return None; }
    let fd = fields.next()?;
    let request = fields.next()?;
    let number = |s: &str| s.strip_prefix("0x").map_or_else(
        || s.parse::<u64>().ok(), |n| u64::from_str_radix(n, 16).ok());
    if number(request)? != 0x8010a406 { return None; }
    number(fd)?.try_into().ok()
}

#[test]
fn readiness_matches_original_ascii_records() {
    let first = ["0", "54", "054", "+54", "0x36", "running", "", "-1"];
    let fds = ["0", "6", "+6", "0006", "0x6", "0x+6", "+0x6", "0X6", "4294967295",
        "4294967296", "18446744073709551615", "18446744073709551616", "0xffffffffffffffff",
        "0x10000000000000000", "-1", "+", "0x", "", "6x"];
    let requests = ["0x8010a406", "0x+8010a406", "2148574214", "+2148574214", "02148574214",
        "0x8010a407", "18446744073709551616", "0x1000000008010a406", "", "running"];
    for a in first { for b in fds { for c in requests { for ws in [" ", "\t", "\r\n", "\x0b", "\x0c"] {
        for tail in ["", " 0xbe93db28 0 0 0 0 0\n"] {
            let input = format!("{ws}{a}{ws}{b}{ws}{c}{tail}");
            assert_eq!(receive_fd(input.as_bytes()), old_receive(input.as_bytes()), "{input:?}");
        }
    }}}}
}

fn old_device(bytes: &[u8]) -> Option<(u64, u64)> {
    let text = std::str::from_utf8(bytes).ok()?;
    let (major, minor) = text.trim().split_once(':')?;
    Some((major.parse().ok()?, minor.parse().ok()?))
}

#[test]
fn device_identity_matches_original_ascii_records() {
    let values = ["0", "1", "10", "+10", "00010", "4294967296", "18446744073709551615",
        "18446744073709551616", "184467440737095516150", "-1", "0x10", "+", "", "1:2", " 1"];
    for a in values { for b in values { for ws in ["", " ", "\t", "\r\n", "\x0b", "\x0c"] {
        let input = format!("{ws}{a}:{b}{ws}");
        assert_eq!(device_identity(input.as_bytes()), old_device(input.as_bytes()), "{input:?}");
    }}}
}

#[test]
fn malformed_and_non_ascii_records_fail_closed() {
    for input in [&b""[..], b"54 6", b"54 6 0x", b"54 6 +", b"54\0 6 0x8010a406",
        b"54 6 0x8010a406\xff", b"54\xc2\xa0 6 0x8010a406"] {
        assert_eq!(receive_fd(input), None, "{input:?}");
    }
    for input in [&b""[..], b"1:", b":1", b"1:2:3", b"1:2\xff", b"\xc2\xa0 1:2"] {
        assert_eq!(device_identity(input), None, "{input:?}");
    }
    // Even very long zero prefixes remain bounded scalar work, with no parser recursion.
    let zeros = "0".repeat(65536);
    assert_eq!(receive_fd(format!("54 {zeros}6 0x8010a406").as_bytes()), Some(6));
    assert_eq!(device_identity(format!("{zeros}10:2").as_bytes()), Some((10, 2)));
}

#[test]
fn linux_device_identity_requires_character_and_exact_numbers() {
    for major in [0u64, 1, 10, 0xfff, 0x12345, 0xffffffff] {
        for minor in [0u64, 1, 255, 256, 0x12345, 0xffffffff] {
            let raw = ((major & 0xfff) << 8) | ((major & 0xfffff000) << 32)
                | (minor & 0xff) | ((minor & 0xffffff00) << 12);
            assert_eq!(unsafe { tv_tee_device_matches(major, minor, raw, 1) }, 1);
            assert_eq!(unsafe { tv_tee_device_matches(major, minor, raw, 0) }, 0);
            assert_eq!(unsafe { tv_tee_device_matches(major ^ 1, minor, raw, 1) }, 0);
        }
    }
}

#[test]
fn scalar_parsers_need_no_runtime_and_share_no_mutable_state() {
    let threads: Vec<_> = (0..8).map(|thread| std::thread::spawn(move || {
        for i in 0..1000 {
            let fd = thread * 1000 + i;
            assert_eq!(receive_fd(format!("54 {fd} 0x8010a406 0 0\n").as_bytes()), Some(fd));
            assert_eq!(device_identity(format!("10:{fd}\n").as_bytes()), Some((10, fd.into())));
        }
    })).collect();
    for thread in threads { thread.join().unwrap(); }
}
