#[path = "../../../src/pid1/src/ownership.rs"]
mod ownership;
const SERIAL: &str = "androidboot.serialno=4C0490290FA05A0C0";
const NAMES: [&str; 7] = [
    "surfaceflinger",
    "composer@2.4-se",
    "audioserver",
    "mstar.hardware.",
    "midaemon",
    "app-rust-displa",
    "app-rust-audio-",
];
fn identity_ref(pid: u32, s: &str) -> bool {
    pid == 1 && s.is_ascii() && s.split_whitespace().any(|s| s == SERIAL)
}
fn owner_ref(s: &str) -> Option<u32> {
    if !s.is_ascii() {
        None
    } else {
        Some(
            NAMES
                .iter()
                .position(|n| *n == s.trim())
                .map_or(0, |i| i as u32 + 1),
        )
    }
}
fn stopped_ref(s: &str) -> bool {
    s.is_ascii()
        && s.lines()
            .find_map(|s| s.strip_prefix("State:"))
            .is_some_and(|s| matches!(s.trim().chars().next(), Some('T' | 't')))
}
fn hash(h: &mut u64, s: &str) {
    for b in s.bytes() {
        *h = (*h ^ b as u64).wrapping_mul(0x100000001b3)
    }
}
fn main() {
    for (index, name) in NAMES.iter().enumerate() {
        assert_eq!(ownership::label(index as u32 + 1), *name);
    }
    let mut h = 0xcbf29ce484222325;
    let mut count = 0;
    let mut names = vec![
        "".into(),
        "+".into(),
        "0".into(),
        "+0".into(),
        "0001".into(),
        "4294967295".into(),
        "4294967296".into(),
        "+4294967296".into(),
        "18446744073709551616".into(),
        "0x1".into(),
        " 1".into(),
        "1 ".into(),
        "-1".into(),
        "1\0".into(),
        "١".into(),
    ];
    for n in [
        0u64,
        1,
        2147483647,
        4294967294,
        4294967295,
        4294967296,
        u64::MAX,
    ] {
        for prefix in ["", "+", "00", "+00"] {
            names.push(format!("{prefix}{n}"));
        }
    }
    for s in names {
        let got = ownership::numeric_name(s.as_bytes());
        let want = s.parse::<u32>().is_ok();
        assert_eq!(got, want, "PID {s:?}");
        hash(&mut h, &format!("p{s:?}{got}"));
        count += 1;
    }
    let mut cmds = vec![
        String::new(),
        SERIAL.into(),
        format!("a {SERIAL} b"),
        format!("x{SERIAL}"),
        format!("{SERIAL}x"),
        format!("{SERIAL}\0"),
        format!("bad={SERIAL}"),
        format!("{SERIAL}\u{a0}"),
        format!("{SERIAL} \u{a0}late"),
    ];
    for ws in [" ", "\t", "\n", "\r", "\x0b", "\x0c"] {
        cmds.push(format!("{ws}first{ws}{SERIAL}{ws}last{ws}"));
    }
    for s in cmds {
        for pid in [0, 1, 2, u32::MAX] {
            let got = ownership::identity(pid, s.as_bytes());
            assert_eq!(got, identity_ref(pid, &s), "identity {pid} {s:?}");
            hash(&mut h, &format!("i{pid}{s:?}{got}"));
            count += 1;
        }
    }
    for n in NAMES {
        for (prefix, suffix) in [
            ("", ""),
            (" ", "\n"),
            ("\x0b", "\x0c"),
            ("x", ""),
            ("", "x"),
            ("", " extra"),
            ("", "\u{a0}"),
            ("\u{2000}", ""),
            ("", "\0"),
        ] {
            let s = format!("{prefix}{n}{suffix}");
            assert_eq!(ownership::owner(s.as_bytes()), owner_ref(&s), "comm{s:?}");
            hash(
                &mut h,
                &format!("c{s:?}{:?}", ownership::owner(s.as_bytes())),
            );
            count += 1;
        }
    }
    let mut statuses = vec![
        String::new(),
        "State:".into(),
        "State:\nState:T".into(),
        "State:R\nState:T".into(),
        " State:T".into(),
        "state:T".into(),
        "State:T\u{a0}".into(),
        "State:\r\nState:T".into(),
        "State:T\nX:\u{a0}".into(),
    ];
    for state in [
        "T",
        "t",
        "R",
        "S",
        "",
        "TT",
        "T (stopped)",
        "t (tracing stop)",
    ] {
        for ws in ["", " ", "\t", "\r", "\x0b", "\x0c"] {
            statuses.push(format!("Name: test\nState:{ws}{state}\nPid: 1\n"));
        }
    }
    for s in statuses {
        let got = ownership::stopped(s.as_bytes());
        assert_eq!(got, stopped_ref(&s), "status{s:?}");
        hash(&mut h, &format!("s{s:?}{got}"));
        count += 1;
    }
    let alphabet = b"State:TtR012349 +:=\t\r\n\x0b\x0c\0midaemon";
    let mut rng = 0x12345678u32;
    for _ in 0..10000 {
        rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
        let n = (rng % 60) as usize;
        let mut b = Vec::new();
        for _ in 0..n {
            rng = rng.wrapping_mul(1664525).wrapping_add(1013904223);
            b.push(alphabet[(rng as usize) % alphabet.len()]);
        }
        let s = String::from_utf8(b).unwrap();
        assert_eq!(ownership::owner(s.as_bytes()), owner_ref(&s));
        assert_eq!(ownership::stopped(s.as_bytes()), stopped_ref(&s));
        assert_eq!(
            ownership::numeric_name(s.as_bytes()),
            s.parse::<u32>().is_ok()
        );
        assert_eq!(ownership::identity(1, s.as_bytes()), identity_ref(1, &s));
        hash(&mut h, &format!("r{s:?}"));
        count += 4;
    }
    for owner in [0, 1, 2, 3, 4, 5, 6, 7, 8, 255, u32::MAX] {
        for stopped in [false, true] {
            assert_eq!(
                ownership::permitted(owner, stopped),
                owner == 0 || ((1..=7).contains(&owner) && stopped)
            );
            count += 1;
        }
    }
    // No Lean initialization: these scalar APIs are safe to call on the hardware worker.
    std::thread::spawn(|| {
        assert!(ownership::identity(1, SERIAL.as_bytes()));
        assert_eq!(ownership::owner(b"midaemon\n"), Some(5));
        assert!(ownership::stopped(b"State:\tT (stopped)\n"));
    })
    .join()
    .unwrap();
    println!("PASS: {count} ownership adapter cases; no Lean initialization; worker call passed");
    println!("TRANSCRIPT {h:016x}");
}
