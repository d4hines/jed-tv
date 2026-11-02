//! Run the real Lean FFI and worker/main channel against the pre-port oracle.
#[path = "../../../src/pid1/src/cold_plan.rs"]
mod cold_plan;
extern crate self as tvinit;
#[path = "../../../src/pid1/tvinit/src/lean_runtime.rs"]
pub mod lean_runtime;
// This isolated test initializes ColdPlan directly; production initializes Core first.
#[no_mangle]
unsafe extern "C" fn initialize_tv_TV_Core(builtin: u8) -> lean_runtime::Object {
    extern "C" {
        fn initialize_tv_TV_Hardware_ColdPlan(builtin: u8) -> lean_runtime::Object;
    }
    initialize_tv_TV_Hardware_ColdPlan(builtin)
}
#[allow(dead_code)]
mod reference;
use cold_plan::Operation;
use mi_bootstrap::cold_start::IniPolicy;
use std::{
    io,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
fn normalized(result: io::Result<String>) -> Result<String, (io::ErrorKind, String)> {
    result.map_err(|e| (e.kind(), e.to_string()))
}
fn oracle(operation: Operation, text: &str) -> io::Result<String> {
    match operation {
        Operation::Validate => {
            reference::cold_start::validate_explicit_ini(text).map(|_| String::new())
        }
        Operation::Prepare => reference::explicit_ini(text),
    }
}
fn base() -> String {
    reference::cold_start::FLAGS
        .iter()
        .map(|key| {
            format!(
                "{key}={}\n",
                u8::from(["MI_SYS_INIT", "MI_FLASH_INIT"].contains(key))
            )
        })
        .chain(
            reference::cold_start::PARAMETERS
                .iter()
                .map(|(key, value)| format!("{key}={value}\n")),
        )
        .collect()
}
fn cases(base: &str) -> Vec<(String, String)> {
    let stock = base.replace("MI_TUNER_INIT=0", "MI_TUNER_INIT=1");
    let mut all = vec![
        ("explicit".into(), base.into()),
        ("stock".into(), stock.clone()),
    ];
    for &key in reference::cold_start::FLAGS {
        let line = base
            .lines()
            .find(|s| s.starts_with(&format!("{key}=")))
            .unwrap()
            .to_owned()
            + "\n";
        all.push((format!("missing {key}"), base.replace(&line, "")));
        all.push((
            format!("flip {key}"),
            base.replace(
                &line,
                &format!("{key}={}\n", u8::from(!line.ends_with("1\n"))),
            ),
        ));
        all.push((format!("duplicate {key}"), base.to_owned() + &line));
    }
    for &(key, value) in reference::cold_start::PARAMETERS {
        let line = format!("{key}={value}\n");
        all.push((format!("missing {key}"), base.replace(&line, "")));
        all.push((
            format!("wrong {key}"),
            base.replace(&line, &format!("{key}=9\n")),
        ));
        all.push((format!("duplicate {key}"), base.to_owned() + &line));
    }
    for value in [
        "0",
        "1",
        "00",
        "+0",
        "-0",
        "",
        "false",
        "0=",
        "0 # ignore",
        "0; ignore",
    ] {
        all.push((
            format!("unknown INIT {value}"),
            format!("{base}Z_UNKNOWN_INIT={value}\n"),
        ));
    }
    for (name, suffix) in [
        ("unknown", "OTHER=anything\n"),
        ("empty key", "=a\n"),
        ("duplicate empty", "=a\n =b\n"),
        ("duplicate unknown", "OTHER=1\nOTHER=2\n"),
        ("malformed section", "[oops=1\n"),
        ("ignored line", "garbage\n"),
        ("comment", "# MI_SYS_INIT=0\n; MI_SYS_INIT=0\n"),
        ("nul", "OPAQUE=\0\n"),
        ("unknown order", "Z_NEW_INIT=1\nA_NEW_INIT=1\n"),
    ] {
        all.push((name.into(), format!("{base}{suffix}")));
    }
    for cp in [
        9, 10, 11, 12, 13, 32, 0x85, 0xa0, 0x1680, 0x2000, 0x2001, 0x2002, 0x2003, 0x2004, 0x2005,
        0x2006, 0x2007, 0x2008, 0x2009, 0x200a, 0x2028, 0x2029, 0x202f, 0x205f, 0x3000, 0x1c, 0x1d,
        0x1e, 0x1f, 0x180e, 0x200b, 0xfeff,
    ] {
        let c = char::from_u32(cp).unwrap();
        all.push((
            format!("whitespace {cp}"),
            base.replace("MI_SYS_INIT=1", &format!("{c}MI_SYS_INIT{c}={c}1{c}")),
        ));
        all.push((
            format!("stock whitespace {cp}"),
            stock.replace(
                "MI_TUNER_INIT=1",
                &format!("{c}MI_TUNER_INIT{c}={c}1{c}; comment 111"),
            ),
        ));
    }
    for (name, text) in [
        ("crlf", base.replace('\n', "\r\n")),
        ("bare CR", base.replace('\n', "\r")),
        ("stock CRLF", stock.replace('\n', "\r\n")),
        ("no LF", base.trim_end_matches('\n').into()),
        ("stock no LF", stock.trim_end_matches('\n').into()),
        ("BOM", format!("\u{feff}{base}")),
        (
            "duplicate before schema",
            format!(
                "{}OTHER=1\nOTHER=2\n",
                base.replace("MI_SYS_INIT=1", "MI_SYS_INIT=9")
            ),
        ),
    ] {
        all.push((name.into(), text));
    }
    let padding = 65536 - stock.len() - 2;
    all.push((
        "maximum comment line".into(),
        format!("#{}\n{stock}", "x".repeat(padding)),
    ));
    all.push((
        "many ignored lines".into(),
        "ignored\n".repeat(7000) + &stock,
    ));
    all.push((
        "large Unicode whitespace".into(),
        "\u{2000}".repeat(21000) + &stock,
    ));
    all
}
fn main() {
    tvinit::lean_runtime::initialize_runtime().unwrap();
    let valid = base();
    let stock = valid.replace("MI_TUNER_INIT=0", "MI_TUNER_INIT=1");
    let cases = cases(&valid);
    let mut hash = 0xcbf29ce484222325u64;
    for (name, text) in &cases {
        for (op, tag) in [
            (Operation::Prepare, "prepare"),
            (Operation::Validate, "validate"),
        ] {
            let want = normalized(oracle(op, text));
            let got = normalized(cold_plan::evaluate(op, text));
            assert_eq!(got, want, "{tag}: {name}");
            let bytes = format!("{tag}:{name}:{got:?}\n");
            for b in bytes.bytes() {
                hash = (hash ^ b as u64).wrapping_mul(0x100000001b3);
            }
            if let Ok(prepared) = got {
                if tag == "prepare" {
                    assert_eq!(prepared.len(), text.len());
                    assert_eq!(
                        text.bytes()
                            .zip(prepared.bytes())
                            .filter(|(a, b)| a != b)
                            .collect::<Vec<_>>(),
                        [(b'1', b'0')]
                    );
                }
            }
        }
    }
    // The real adapter rejects a second thread before it can allocate a Lean input.
    let wrong = valid.clone();
    let error = thread::spawn(move || {
        cold_plan::evaluate(Operation::Validate, &wrong)
            .unwrap_err()
            .to_string()
    })
    .join()
    .unwrap();
    assert!(error.contains("second thread"), "{error}");
    let client = cold_plan::start().unwrap();
    assert!(cold_plan::start().is_err());
    let writes = Arc::new(AtomicUsize::new(0));
    let worker_writes = writes.clone();
    let worker = thread::spawn(move || {
        let staged = client.prepare(&stock).unwrap();
        worker_writes.fetch_add(1, Ordering::SeqCst); // would write only after approved bytes
        client.validate(&staged).unwrap();
        client.validate(&staged).unwrap(); // the pre-module-load recheck
        assert!(client.prepare(&valid).is_err()); // non-stock: do not write or retry
        assert_eq!(worker_writes.load(Ordering::SeqCst), 1);
    });
    while !worker.is_finished() {
        cold_plan::tick();
        thread::sleep(Duration::from_millis(1));
    }
    worker.join().unwrap();
    assert_eq!(writes.load(Ordering::SeqCst), 1);
    println!("PASS: {} cold INI cases, exact errors and one-byte edits; real worker/main adapter, wrong-thread rejection, no write after failure",cases.len());
    println!("TRANSCRIPT {hash:016x}");
}
