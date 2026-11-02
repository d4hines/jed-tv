# The actual Core loop on the Lean-written runtime

Run from the repository root:

```
python3 test/lean/core-zr/check.py
```

Requires the normal Lean/Zig/QEMU tools and the project's ARM32 surrogate
kernel at `assets/build/kernel/arm/zImage`. Nothing connects to a TV.

The check builds `TV.Core` and its proofs normally, then links the **unchanged
compiler output** of `TV.Core`, `TV.Apps` and the finite `TV.Volume.Native`
service with ZrLean. It builds that runtime
from this checkout, rather than linking upstream Lean's C++ runtime. The
result is a static ARM32 ELF that runs as PID 1 under the surrogate kernel.
No handwritten C, libc, C++ runtime, `libInit`, or vendor services are linked.
Generated C, objects, transcripts, symbol lists, link map and hashes go to
`assets/build/core-zr/`; package outputs go to `assets/build/lake/core-zr/`.

`CoreZr.lean` implements all thirteen native Core effects for this test. A
Linux file offset supplies scripted input; stdout records every output. These
are mocks for the hardware effects, **not** implementations of app isolation,
physical display, storage durability or power control. The only platform
operations are the existing Lean/Zr scalar Linux syscall functions. The test
powers down its QEMU guest after four runs of the real `TV.Core.main` and nine finite volume ticks:

- Launch/exit, press/repeat/release ABI, reserved Home-code rejection, Home
  taking the screen before TERM, exact and expired kill deadlines, repeated
  Home not extending the deadline, and Power reaching its dedicated effect.
- Enabled-app mask, disabled launches, bad PINs, successful enable and PIN
  change, and rejecting the previous PIN after it changes.
- A failed `save` returning failed IO: the loop stops before another input or
  a changed `menuApps` publication. The native production supervisor still
  must stop children and display its safe screen after this error.
- Unreadable storage and invalid saved PIN: zero enabled apps and no launch.
- Finite volume ticks preserve mute memory, ignore repeated mute, accept external
  gain, and retain state without publication when a write fails.
- Mutable references preserve shared values and swap/mutate correctly, including
  persistent references initialized at module startup.
- Binary32 conversion handles signed zero, NaN, infinities, integer truncation
  and saturation, along with PCM multiplication and unsigned rounding.

The console must match `expected.txt` byte for byte. The ELF is also checked
for ARM32 identity, no dynamic loader/dependencies and no unresolved symbols.

## Initializers and runtime boundary

`Core` imports IO **types**, arithmetic/list **lemmas**, and proof tactics.
Its actual compiled functions require only Nat, strings, closures, allocation
and reference counting, all already implemented by ZrLean. The compiler still
emits calls to a small, explicitly enumerated set of module initializers. For this isolated executable,
`ZrAsm.Runtime` exports narrow assembly aliases for exactly those names:

```
initialize_Init_System_IO
initialize_Init_Data_Int_OfNat
initialize_Init_Data_List_Lemmas
initialize_Init_Omega
initialize_Init_Data_UInt
initialize_Init_Control_Basic
initialize_Init_Data_SInt_Float32
initialize_tv_TV_CoreProofs
```

They use Zr's bootstrap to return a real `IO.ok ()` object. This does **not**
run the upstream initializers, and is only valid while Core/Apps reference no
runtime values from those modules. The check records every unresolved object
symbol and rejects additional initializer dependencies or executable
upstream IO/Init/Lean references; final linking rejects unsupported standard
library functions. A production extraction should preserve this explicit
audited boundary, or move proof-only imports behind a source/build boundary.
Do not turn this list into a catch-all initializer stub.

The proof tactics themselves run on the host when `lake build` checks Core.
They are not needed to execute the checked state machine on the guest.

## What this establishes, and what remains

The Core decisions and real effect-dispatch loop can execute using our Lean
runtime now. The Core loop needs no extra library operations; mutable menu state uses the
reference operations validated by the additional checks.
This is ARM32 QEMU evidence, not a physical-TV run or a full Lean PID 1 image.

The thirteen real effects still need their implementations: input/clock,
policy load/save, menu/drawing, child start/signal/key forwarding and power.
The existing Zr scalar Linux syscall layer is the next boundary for those
ports. Port small effects first (clock, signal, atomic file save), then child
setup and input multiplexing, then display/menu/power integration. Keep
hardware ownership and cleanup behavior from the current native adapter.

ZrLean is single-threaded. The native PID 1 integration runs Core on the
supervisor thread and invokes one finite volume tick from each Core/idle
poll. Display/audio/recovery workers remain native and must not enter Lean.
Preserve this single-executor rule when replacing the executable's runtime;
do not simply swap the allocator beneath concurrent Lean callers.
