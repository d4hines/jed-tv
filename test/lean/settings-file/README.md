# Private settings codecs and native adapters

Run `python3 test/lean/settings-file/check.py` from the repository root after
building the normal ARM32 QEMU kernel. `--kernel PATH` accepts an existing copy
of that kernel. The check never connects to a TV.

`TV.SettingsFile` owns the parental file format, default policy, numeric bounds
and serialization. `TV.Storage` decides metadata acceptance and the atomic commit
sequence, with proofs for private-file acceptance and success only after file
and directory synchronization. Rust's `CoreStore` performs metadata reads,
bounded UTF-8 reads, write/fsync/rename operations, and error propagation. The menu
never receives this store or its packed PIN. The file's existing Unicode
whitespace and optional leading `+` on masks remain accepted; all eight stored
mask bits are preserved, including retired apps.

The host oracle compares the codec with the previous Rust parsing rule over
all 10,000 PINs, all 256 masks, malformed values, Unicode separators and 10,000
random strings. ARM32 links the Lean-written Zr runtime and repeats these checks
using the real production CoreStore adapter against a temporary directory. It
also checks permissions, symlinks, hardlinks, corrupt files, default policy,
invalid saves, failed saves, and atomic replacement without following links.
Storage's allocation-free scalar exports run before runtime initialization;
metadata acceptance is compared with the previous predicates, every commit
failure is injected through the production effect interpreter, and worker
threads exercise the same scalar API without entering the Lean runtime.

`TV.StorageText` reads sysfs decimal and major:minor records through scalar
byte transitions. The device-pair parser reuses `TV.Hardware.TeeBoundary` and
adds the UInt32 bounds required by Linux `makedev`. The syscall adapter only
reads bytes. Kernel records now require ASCII; non-ASCII whitespace previously
accepted by Rust's Unicode trim is deliberately rejected. The parental-file
format and command-line/bootconfig parsing are unchanged. Tests compare against
the previous ASCII grammar, exercise UInt64/UInt32 overflows, invalid bytes and
long records, and call the real adapter concurrently before Lean initialization.

The Wi-Fi codec applies scan filtering, SSID deduplication, the 24-network cap,
byte-length checks, passphrase validation and WPA configuration escaping in
Lean. An oracle includes 2,000 hostile scan listings, control characters,
Unicode names, quoted synthetic passphrases and password-length boundaries.
The ARM test also sends a raw scan result from a native worker and checks that
the production controller applies Lean policy on the supervisor thread. No
test invokes WPA, DHCP or a network interface; credentials are synthetic.

String/List conversion is a runtime dependency of the codec. The same binary
round-trips every Unicode scalar, embedded NUL and shared inputs through the
actual generated String.toList/String.ofList calls. Their owned argument ABI is
checked against the pinned upstream Lean compiler. The runtime implementation
is scalar Lean in `ZrLean.Low.Str`; no upstream runtime is linked on ARM32.

Evidence is written below `assets/build/settings-file-check/`, including host
and QEMU logs, runtime build log, executable/kernel hashes and `result.json`.
The QEMU kernel reports `Attempted to kill init` with exit code zero after all
PASS lines because the test executable itself is PID 1.
