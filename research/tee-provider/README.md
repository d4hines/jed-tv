# Standalone G06 TEE provider candidate

This is an explicit test candidate, not a default runtime change. It wraps the
owned `g06-tee-rpc` library in the exact V109 receive/send transport. No hardware
execution, TEE device open, mount, light or sound test was performed to build it.

Build and tests:

```
./tools/tee-provider/check.sh
./tools/tee-provider/build.sh
```

The output is a static ARM32 binary at:

```
assets/build/audio-agent/tee-provider/armv7-unknown-linux-musleabihf/release/g06-tee-provider-candidate
```

The standalone crate and its lockfile use the existing local `g06-tee-rpc` path
dependency plus RustCrypto sha2 0.10.9 and libc ABI constants. No C runtime service or stock supplicant
is launched. The crate's `[workspace]` stanza is for isolated builds; remove it
if joining a shared parent workspace.

## Authority and singleton checks

The binary takes no arguments and has no alternate device, firmware root, mode
or backend flags. Before reading TA files it requires root, the exact original
G06 serial in `/proc/cmdline`, `tee_mode=optee`, and kernel4.19.116. Duplicate or
conflicting serial/mode arguments fail. It does not create device nodes or
change kernel/TEE state to manufacture these conditions.

ARM32 O_NOFOLLOW is taken from libc (0x8000), with a compile-time check; the
0x20000 generic-x86 value would mean O_LARGEFILE on this target.

It locks `/tmp/g06-tee-provider.lock` with nonblocking exclusive flock, refusing
symlinks and non-root/group-writable lock files. It then opens exactly
`/dev/teepriv0`, with O_NOFOLLOW, once. The open file must be a character device
matching `/sys/class/tee/teepriv0/dev`. TEE_IOC_VERSION must report OP-TEE,
TrustZone capability, GlobalPlatform support and a privileged device.

The lock coordinates our candidates, while the stock kernel supplies actual
exclusion against an unrelated stock supplicant: `optee_open` checks its existing
supplicant context under a mutex and returns EBUSY if present. The allocator
uses a dup of the one successfully opened descriptor, not a second open/context.
No existing process is stopped or replaced by this binary. A busy device simply
fails startup.

## Trusted files

All four files are read and SHA256-verified before opening the TEE device.
Only these exact UUIDs are cached from
`/mnt/vendor/tvservice/glibc/optee_armtz/`:

| Role | Vendor filename | Bytes |
|---|---|---:|
| Observed cold-start TA | 1fc17090-bbdf-4648-bca93af97ad88e63.ta | 149600 |
| Audio | 778ee68e-6ee0-11e8-adc0fa7ae01bbebc.ta | 145328 |
| HDCP2 | 0987be40-05fb-11e6-98930002a5d5c51b.ta | 484192 |
| HDCP1 | 99986025-2c8d-43ca-9598b90e324e9d3b.ta | 190528 |

The exact pins are authored in `pins.rs`, matching the previously extracted
V109 manifest. The directory is not searched, request strings never become
paths, final symlinks are refused, and unexpected size/hash aborts startup.
Verified bytes are cached to eliminate pathname/content races after startup.
Unknown UUIDs receive ITEM_NOT_FOUND. Hash pinning complements secure-world
signature checks; it does not replace them.

Including these candidates does not establish which are needed by the cold
RGB/PCM path. HDCP1 remains conditional on HDMI/EXTIN initialization. No other
TA, filesystem directory, key or certificate is automatically authorized.

## Exact framing

The V109 kernel accepts at most1024 bytes for a TEE request buffer. This is
smaller than the4096-byte bound in newer generic headers. The fixed packet is
1000 bytes: two u32 header words plus31 parameters of32 bytes each. An ioctl's
outer envelope is16 bytes: u64 pointer and u64 length.

RECV request0x8010a406 advertises31 slots and initial param0.attr=0x100 (META),
matching the stock supplicant's request for metadata. On success, all leading
META parameters are excluded from the semantic handler but returned byte-for-byte
in SEND. A META parameter after the first ordinary argument gets BAD_PARAMETERS;
it is not silently stripped. SEND request0x8010a407 uses the actual received
count and length8+32*count. A returned count above capacity is a fatal framing
error; no fabricated reply follows.

The service handles one received request at a time. Supported handlers are
LOAD_TA, SHM_ALLOC, SHM_FREE and the read-only vendor object protocol (100/101).
The standalone pinned store currently provides no object files, so vendor
object requests fail closed until their observed UUIDs and hashes are pinned.
Other commands, including FS/RPMB/socket, receive NOT_SUPPORTED. This is a bounded implementation
subset, not a claim that these errors suffice for cold startup.

The library retains the proved stock NULL-output size-query semantics. The
standalone wrapper additionally refuses an unknown backing ID whenever offset
or requested capacity is nonzero: it never reports a completed copy into a
registered/external region it has not imported. A zero-offset/zero-capacity
probe can still return the required TA size. Actual copies require an ID owned
by this provider. Kernel-provided external mappings remain a trace gate.

Mapped writes and clears use volatile accesses; Release fences precede ID/data
publication. RECV completion uses an Acquire fence; SEND uses a Release fence.
No ambiguous failed SEND is retried. Any ioctl error ends the process, dropping
owned mappings before the remaining device descriptor closes. The kernel then
handles pending-request cancellation through normal supplicant-context release.

Per-request logs contain only command number, counts and status, capped at the
first10000 requests; no TA payloads, keys or shared-memory contents are logged.

## First hardware gate: coordinator only, after the user is awake

1. Keep a responsive independent shell and the recoverable boot selection.
   Keep stock TEE/DSP artifacts available. Do not invoke this during the current
   sleeping/unplugged state.
2. Finish the real cold trace with stock supplicant first, checking which TAs,
   memory registration mode, external mappings and FS/RPMB calls occur. The
   already-passed tracer smoke does not establish those requirements.
3. Verify the fixed TA root exposes the original files and pinned hashes.
   `/vendor/tvcertificate` is a separate stock TEE filesystem root; mounting it
   does not make this candidate implement FS/RPMB. That support remains absent.
4. Ensure no other process owns the privileged TEE context. Launch the candidate
   explicitly from the coordinator; a second-open EBUSY is a refusal, not a
   reason to kill another process automatically.
5. Observe actual receive/send and the first supported allocation/load/free
   sequence. On unsupported RPC, preserve logs and return to the known baseline;
   do not substitute success or expand the allowlist without evidence.
6. Only later attempt complete display/audio initialization. A ready message or
   successful TA size query alone proves neither key access nor working media.

No default launcher or PID1 integration is included.

## Offline validation/evidence

Fourteen service tests cover exact packet layout, leading metadata round trips,
misplaced metadata, zero/all-meta packets, handler errors, transport failures,
oversized response count, root/identity checks, TA digest/length validation,
unknown UUIDs, and refusal of unknown nonempty backing memory. The33 RPC-library tests remain separate, including ten vendor-object tests.

Exact stock evidence in shared ignored assets:

-`audio-agent-direct/kernel-tee-ioctl.txt`: RECV bound at0xc0890234..0xc0890244;
 SEND bound at0xc08904ac..0xc08904bc; maximum is8+1016=1024.
-`audio-agent-direct/kernel-optee-exclusive-open.txt`: singleton check
0xc08937a0..0xc08937d8, EBUSY return at0xc08937c8.
-`audio-agent-direct/tee-supplicant-thumb.txt`: initial META request at0x2952,
 leading-meta scan at0x29e6, misplaced-meta rejection at0x2a00,
 returned original packet/count at0x2d98..0x2dbc.
-`audio-agent-direct/cold-firmware/manifest.json`: exact extracted V109 file
identities; authoring evidence is in the earlier audio/display TA reports.

## Differential tests against the extracted firmware

`./tools/tee-provider/check-firmware.sh` builds an offline Rust packet peer and
runs the exact V109 Thumb dispatcher in Unicorn. The analysis dependencies are
pinned (`unicorn==2.1.4`, `pyelftools==0.32`); `uv` stores its cache under assets.
The test requires the already extracted stock supplicant, kernel ELF and four
TA files in ignored assets. It neither opens a TEE device nor runs secure-world
code. Generated evidence goes under `assets/research/tee-firmware-comparison/`.

The stock executable is SHA256-pinned. The emulator executes its dispatch,
parameter checks, linked-list SHM lookup/free, UUID/path construction, TA size
query and conditional copy instructions. Hooks substitute read-only in-memory
stdio, an inert ioctl peer, zeroed memory allocations and single-threaded locks.
There is an existing pending receiver in the modeled context, so this does not
exercise creation of stock worker threads. Unexpected imported calls fail the
test instead of being silently stubbed. Test replies retain the complete wire
packet; mapped TA data is compared by SHA256, without writing payloads to logs.
The Rust side is the real `serve_one` and `Provider`, using pinned files and an
in-memory allocator, not the Linux mmap/ioctl adapter.

Coverage includes all four TAs, value attributes 1/2/3, memref attributes 5/6/7,
zero/short/exact/extra output capacity, absent size queries, unknown UUIDs,
allocation/free/double-free, malformed arguments, unknown command and leading
metadata preservation. A separate execution of the stock kernel's exact ARM
RECV/SEND length guards checks 18 boundary cases: 8 through 1024 bytes are
accepted; smaller, larger and high-word-overflow lengths are rejected. The
stock userspace advertises five parameter slots, while the kernel ceiling
permits 31 and this Rust transport has room for 31. These are different bounds.

The 2026-11-01 run passed 576 exchanges across 165 scenarios: 156 exact-match
scenarios and nine expected-difference scenarios, plus all 18 kernel guard
cases. Evidence: `assets/research/tee-firmware-comparison/1793551573221245000/comparison.json`.
The RPC/provider unit suites also passed (23 + 14 tests), and the standalone
ARM32 provider cross-build succeeded. No default runtime behavior was changed.

Expected differences are recorded explicitly rather than equated with matches:

- Stock subtracts the memref offset from its advertised capacity after already
  checking offset plus capacity against the mapping. At nonzero offset it can
  report SHORT_BUFFER where the Rust implementation accepts a valid copy.
  Rust retains its existing full-width offset/length bounds and ABI length
  semantics; zero-offset transfers agree.
- With an unknown output backing and nonzero capacity, stock can report a
  successful size query without copying bytes. Rust refuses that request;
  only a zero-offset, zero-capacity query is allowed without owned backing.
- Stock's scalar attribute and allocation-size paths inspect/truncate lower
  bits. Rust rejects high-bit attributes and out-of-quota full-width sizes.
- A misplaced metadata parameter makes the stock worker return without SEND;
  Rust sends BAD_PARAMETERS. Neither result establishes a usable media session.

These comparisons do not establish that this subset suffices for cold startup,
that the three candidate TAs cover every request, or that the Linux allocation
adapter works with real hardware. FS/RPMB, external/registered memory, worker
concurrency, cancellation and secure-world execution still need separate
qualification. Keep the stock receive/send trace gate before replacing its
live service or embedding this candidate in PID1.

## First live cold-start trace, 2026-11-01

The original G06/V109 was booted through the consumed one-shot request into the
QEMU-tested native PID1 image (SHA256
`2a12f31665c64c97e6f5e8855e765a1414bf2539d7cc61faf39a6ff3714c0170`).
The stock supplicant was traced from exec, while Rust initialized the display,
audio modules and amplifier. No Rust TEE provider was started.

The initial capture contains 61 completed requests with 61 successful stock
SEND replies and no tracer decoding errors:

| Function | Meaning / scope | Count |
|---|---|---:|
| 0 | LOAD_TA | 4 |
| 6 | SHM_ALLOC | 23 |
| 7 | SHM_FREE | 23 |
| 1 | RPMB RPC | 9 |
| 101 | UUID-named vendor object size | 1 |
| 100 | UUID-named vendor object read | 1 |

Two TAs were requested (size query followed by load):

- `778ee68e-6ee0-11e8-adc0-fa7ae01bbebc`, already pinned, 145328 bytes.
- `1fc17090-bbdf-4648-bca9-3af97ad88e63`, initially absent and now added to the candidate's pins,
  149600 bytes, SHA256
  `fae08300ca9157395f7fbe98c1666314f188966685c5a4ad437ebf23ab6900f5`.
  A read-only live copy was saved and matched that hash.

All observed memref offsets were zero. The newly observed TA is now pinned.
RPMB function1 and the unconfirmed vendor-object identity remain concrete
blockers to substituting the current Rust provider. Filesystem function2 was not observed in this capture. The initial
label of function1 as filesystem was corrected by following its dispatcher
entry to `rpmb_process_request_unlocked` at0x35c8 and the explicit RPMB device
paths. The metadata-only trace contains no shared buffer payloads or keys;
it cannot identify the actual RPMB subcommands by itself.

The owner confirmed the launcher physically visible after this cold display
initialization. Native PID1, its root shell and the display remained responsive
past90 seconds. A bounded jingle attempt failed before sending samples at
AOUT GetHandle `0xc0081116`, vendor status7: the cold setup had not created the
speaker output object. This is distinct from a TEE failure; the trace did not
add requests during that failed open. Audio audibility is not established by
the successful module/amp initialization.

Evidence: `assets/research/tee-live/20261101T185641Z/`, including initial and
post-audio RPC JSONL, kernel/events, TA inventory/hash and diagnostic output.
The working stock provider and native UI were retained. No firmware partition
was flashed during this test.

A subsequent explicit Rust speaker-owner helper created output0x17000000 and
held it alive. The same native PCM probe then opened and drained all48000
frames of its one-second jingle at30% of the bounded UI volume. Physical
sound confirmation was still pending when recorded. The persistent owner is
now integrated in the next cold-start build and passes48 bootstrap host tests
plus the QEMU REPL test; that integrated build has not yet booted on hardware.


## Follow-up tracing and vendor object implementation

The transport-independent provider now implements vendor commands100/101 with
a fixed28-byte native UUID header, checked owned-memory slices, bounded reads,
EOF handling and command-specific errors. The standalone store deliberately
returns NotFound for object UUIDs until live header evidence identifies them.
These new handlers have host tests; the existing firmware differential suite
currently exercises TA loading, allocation and framing, not vendor objects.

The next tracer build reads only the six-byte RPMB request header and, for a
frame request, its two-byte operation code; vendor requests expose only their
28-byte header. It does not copy RPMB keys, MACs, nonces or data. The stock
service remains responsible for requests. This instrumentation has passed
host tests and an ARM32 build but still needs a cold-start hardware capture.

The four-TA rerun passed762 exchanges across215 scenarios (204 matching,
11 documented differences), plus18 exact-kernel length checks. Evidence:
`assets/research/tee-firmware-comparison/1793560774406596000/comparison.json`.
This extends the same TA/framing coverage to the newly observed TA; it does
not add vendor-object or RPMB execution coverage.
