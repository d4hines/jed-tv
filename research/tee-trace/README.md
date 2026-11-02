# ARM32 TEE supplicant RPC tracer

This diagnostic launches an ARM32 ELF under ptrace before it executes. It does
not attach to an already running supplicant and does not implement or answer
TEE requests. Its purpose is to record the normal-world RPC contract needed by
our eventual Rust implementation.

Build and offline tests:

```
./tools/tee-trace/build.sh
./tools/tee-trace/check.sh
```

Output: `assets/build/audio-agent/tee-trace/tee-trace` (static ARMv7 musl).
The hardware coordinator must ensure no other supplicant is consuming the
same RPC queue, choose a unique log filename and launch it in the correct cold
phase, before the waiting audio initialization:

```
/tmp/tee-trace --log /tmp/tee-rpc.jsonl -- /system/bin/tee-supplicant
```

Any required supplicant arguments follow the executable unchanged. The path must
be absolute, point to a little-endian ARM32 ELF, and not be a shell wrapper.
The log is created exclusively with mode0600; existing logs are never overwritten. The child
inherits environment, working directory, credentials, stdin/stdout/stderr and
resource limits. The dynamic linker and first supplicant instruction execute
under tracing. The tracer returns the root child's status after all observed
descendants exit; signaled exits use128+signal.

## Captured data

Only ioctl syscall54 with requests0x8010a406 (RECV) and0x8010a407 (SEND) is
captured. Every matched syscall has a per-invocation sequence number, TID, FD,
best-effort `/proc/TID/fd/FD` path, entry/exit records, raw kernel return value,
header word, parameter count and attr/a/b/c values. The exit's return is the raw
syscall register, including negative Linux errno/restart values; it is not
rewritten to libc's -1/errno representation. A negative return does not cause
invalid output memory to be decoded. Successful RECV output contains the RPC
function number in header_word; SEND's header_word is the RPC reply status.

RECV entry records capacity only, not uninitialized parameter slots. SEND entry
and successful exits record valid bounded metadata. The original entry envelope
is retained for exit decoding, and receive output cannot exceed entry capacity.
A read/decode error becomes an error field; the ioctl proceeds unchanged.
Generic snapshots record memref offsets/lengths/IDs without dereferencing them.
For the verified V109 supplicant, separate `rpc_header` records may contain a
fixed public request header: function1 reads six RPMB command/device/count bytes
and, for a data request, only the two-byte first-frame operation code (never its
MAC, key, nonce or data). Functions100/101 read exactly28 bytes: UUID, offset,
file size and result length. Reads require the known lookup-code fingerprint,
owned SHM ID, valid mapping bounds and an unchanged SHM node. Failure to obtain
a header is logged without disrupting the service. No object or TA binary
data, TLS data or application payloads are dumped by this tool. Fixed metadata
can still be sensitive: keep logs in ignored assets when collected.

Observer bounds: at most64 parameters,2056-byte inner buffer,16-byte outer envelope.
The stock kernel accepts at most1024 inner bytes (31 complete parameters);
the wider observer cap also permits inspecting rejected caller requests. Reads
use aligned4-byte PTRACE_PEEKDATA words. No target-provided count determines an
unbounded allocation. Log size is limited to64MiB.

## Process and signal contract

The tracer sets TRACESYSGOOD, TRACEFORK, TRACEVFORK, TRACECLONE, TRACEEXEC and
EXITKILL. Before that option is installed, the child sets
PR_SET_PDEATHSIG(SIGKILL) and checks its parent PID to close the early parent-death
race. Clone/fork/vfork children are followed automatically. Initial child
stops are held until the parent's event identifies them, including when wait
returns those events out of order. A non-leader exec's former TID is removed.
Every syscall stop reads the real ARM r12 direction marker, so an initial child
syscall-exit stop does not desynchronize an entry/exit toggle. Real signal-delivery
stops, including SIGTRAP, are passed through unchanged. No registers, target
memory, syscall arguments or return values are written.

Constraints are explicit:

- ptrace changes timing and signal scheduling; this is not a timing measurement.
- Do not trace a setuid/credential-changing target; ptrace changes exec security
  semantics. Intended target is the already-root stock supplicant.
- Job-control group stops are unsupported with TRACEME. An actual group stop
  fails loudly instead of swallowing it or repeatedly resuming it; no terminal
  Ctrl-Z or external SIGSTOP during the test.
- CLONE_UNTRACED is refused before execution because it would escape observation.
- A fatal tracer/ptrace/logging error or death kills the traced process tree
  (EXITKILL plus explicit cleanup), including a64MiB log overflow. It does not
  leave an unobserved half-started supplicant. Therefore a separate responsive
  coordinator shell is required for the cold-init experiment.
- No privilege escalation, TEE device open, firmware change, reboot or target
  execution occurs merely by building/testing this tool.

## ABI evidence

Local kernel UAPI header:
`/nix/store/hna5xx0f2igssp4xaix69avnf6azfdk7-zig-0.17.0/lib/zig/libc/include/any-linux-any/linux/tee.h`.
The envelope is two u64s; receive/send headers two u32s; each parameter four u64s.
The header's requests are `_IOR(0xa4,6/7,16)`, not `_IOWR`.

Exact V109 confirmation is saved under`assets/research/audio-agent-direct/`:

- `tee-supplicant-thumb.txt`: original supplicant constructs0x8010a406 at0x297e,
  0x298e and passes the16-byte envelope at0x2994.
- `kernel-tee-ioctl.txt`: stock tee_ioctl switch covers the same request range.
- `kernel-arm-ptrace-phase.txt`: stock tracehook_report_syscall saves r12 at
  0xc010ad00, writes direction at0xc010ad08, restores at0xc010adc4. Entry caller
  supplies0; exit caller supplies1 at0xc010aef8. GETREGS is18u32s; r7 is EABI
  syscall number, r0..r2 carry ioctl arguments, r0 is the raw exit result.

Offline parser/state tests and a static build pass. ptrace execution itself has
not yet been tested on the TV or in a Linux VM; coordinator review and a
non-TEE dummy-child smoke test should precede the real cold-start trace.

## First execution gate

The builder also produces `tee-trace-smoke-v2`, a static child that sends the two
ioctl numbers to `/dev/null`, across three threads and a separately executed
child whose nonleader thread execs again. It also verifies caught SIGUSR1 delivery. It never opens a TEE or hardware node. Both ioctls must fail with ENOTTY.
The coordinator can run this before tracing any actual supplicant:

```
/tmp/tee-trace --log /tmp/tee-smoke.jsonl -- /tmp/tee-trace-smoke-v2
```

After collecting the log on the host:

```
python3 tools/tee-trace/check-smoke.py path/to/tee-smoke.jsonl
```

This verifies ten entry/exit pairs, exact metadata, unchanged raw errno,
thread creation events, child creation/exec, nonleader TID replacement, caught
signal forwarding and a successful root exit.
It does not establish TEE functionality or cold initialization.

## Target smoke evidence

On the original G06, the harmless V2 smoke completed successfully: ten paired
ioctls retained ENOTTY, three threads and child/nonleader exec were observed,
and SIGUSR1 reached its handler. The host validator passed. No TEE device was
opened by that test. Evidence: assets/build/audio-agent/tee-trace/live-smoke-v2.jsonl.
The actual cold supplicant RPC trace is still pending.
