//! Compare the production byte adapter with the previous ASCII numeric rules.
use crate::storage_records::{device, number};

fn old_number(bytes: &[u8]) -> Option<u64> {
    if !bytes.is_ascii() {
        return None;
    }
    std::str::from_utf8(bytes).ok()?.trim().parse().ok()
}
fn old_device(bytes: &[u8]) -> Option<(u32, u32)> {
    if !bytes.is_ascii() {
        return None;
    }
    let text = std::str::from_utf8(bytes).ok()?.trim();
    let (major, minor) = text.split_once(':')?;
    Some((major.parse().ok()?, minor.parse().ok()?))
}

pub fn check() {
    let words = [
        "",
        "+",
        "-",
        "0",
        "+0",
        "0001",
        "1",
        "9019392",
        "204800",
        "4294967295",
        "4294967296",
        "18446744073709551615",
        "18446744073709551616",
        "184467440737095516150",
        "0x10",
        "0x+10",
        "-1",
        "1 ",
        " 1",
        "1:2",
    ];
    let spaces = ["", " ", "\t", "\n", "\r\n", "\u{b}", "\u{c}"];
    for word in words {
        for left in spaces {
            for right in spaces {
                let text = format!("{left}{word}{right}");
                assert_eq!(
                    number(text.as_bytes()),
                    old_number(text.as_bytes()),
                    "number {text:?}"
                );
            }
        }
    }
    for major in words {
        for minor in words {
            for left in spaces {
                for right in spaces {
                    let text = format!("{left}{major}:{minor}{right}");
                    assert_eq!(
                        device(text.as_bytes()),
                        old_device(text.as_bytes()),
                        "device {text:?}"
                    );
                }
            }
        }
    }
    let mut seed = 0x12345678u32;
    let alphabet = b"0123456789+:-x \t\r\n\x0b\x0c!";
    for _ in 0..20000 {
        seed ^= seed << 13;
        seed ^= seed >> 17;
        seed ^= seed << 5;
        let mut bytes = Vec::new();
        for _ in 0..seed % 72 {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            bytes.push(alphabet[(seed as usize) % alphabet.len()]);
        }
        assert_eq!(number(&bytes), old_number(&bytes), "number {bytes:?}");
        assert_eq!(device(&bytes), old_device(&bytes), "device {bytes:?}");
    }
    for byte in 128..=255 {
        assert_eq!(number(&[b'1', byte]), None);
        assert_eq!(device(&[b'1', b':', b'2', byte]), None);
    }
    for text in ["\u{a0}1\u{a0}", "\u{2003}1", "1\u{3000}", "\u{a0}1:2\u{a0}"] {
        assert_eq!(number(text.as_bytes()), None);
        assert_eq!(device(text.as_bytes()), None);
    }
    let zeros = "0".repeat(65536);
    assert_eq!(
        number(format!("{zeros}18446744073709551615").as_bytes()),
        Some(u64::MAX)
    );
    assert_eq!(
        number(format!("{zeros}18446744073709551616").as_bytes()),
        None
    );
    assert_eq!(
        device(format!("{zeros}179:29\n").as_bytes()),
        Some((179, 29))
    );
    for digit in 0..10 {
        let text = format!("1844674407370955161{digit}");
        assert_eq!(number(text.as_bytes()), old_number(text.as_bytes()));
    }
    let workers: Vec<_> = (0..4)
        .map(|t| {
            std::thread::spawn(move || {
                for n in 0..1000 {
                    let value = t * 1000 + n;
                    assert_eq!(number(format!(" +{value}\n").as_bytes()), Some(value));
                    assert_eq!(
                        device(format!("179:{value}\n").as_bytes()),
                        Some((179, value as u32))
                    );
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    println!("PASS: raw sysfs decimal/device parsers match ASCII oracle, reject overflow/non-ASCII, and run on workers before Lean initialization");
}
