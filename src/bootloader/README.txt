Bootloader source map
=====================

Signed second-stage updates and network execution: Update/README.txt. The
G13 gate uses the shared signed manifest and UDP receiver to run a temporary
bootloader from Ethernet or write signed application images to two fixed eMMC
banks, without recovery Linux. Current physical validation and deployment
state are in research/G13_APP_UPDATES.txt. Policy and effect-order proofs are included in
prove.py; deployment notes and ARM checks live in tools/bootloader/update/.

The code follows the same separation as TV.Runtime / TV.Supervisor: pure
decisions, an interpreter parameterized by effects, then a physical adapter.
TV.Settings and TV.Proofs demonstrate proving the executable definitions
directly. The bootloader proofs use those definitions too, not a second model.

Start here:

  BootGate.lean          installed G13 gate and fallback orchestration
  Stage0.lean            checks and expands the compressed resident gate
  NetworkGate.lean       bounded Ethernet window before Lean OS preparation
  NetworkBoot.lean       scalar entry into the typed network controller
  Types.lean             nominal units/roles without implicit coercions
  Flow.lean              small ordered workflows and total effect runner
  NetworkBoot/Plan.lean  layout and proof-carrying executable plan
  NetworkBoot/Runtime.lean named effects and failure ordering
  NetworkBoot/Receiver.lean generic signed transport and receiver state
  Update/FlashPlan.lean  fixed G13 application banks and checked sector mapping
  Update/Flash.lean      authenticated selection and direct-install policy
  Update/StorageModel.lean durable/pending sector specification and power-loss proofs
  Update/Handoff.lean    chosen application bank in the final device tree
  Unsafe/Flash.lean      bounded FCIE storage adapter
  Unsafe/Network.lean    physical network/context/cache/call adapter
  NetworkExample.lean    returnable Lean-to-C payload for network experiments
  BootPolicy.lean        named persistent states and full-start transition
  BootJournal/Plan.lean  pure slot selection and next Commit callback
  BootJournal.lean       record codec and monadic storage interpreter
  Plan.lean             pure image validation and Linux configuration
  Os.lean               inspect and prepare images through Reader/Writer
  Memory.lean           checked ranges, loads, stores and copies
  Ethernet/Descriptor.lean pure RX descriptor validation
  Update/Receiver/Plan.lean pure transfer states and decisions
  Unsafe/               physical effects, adapters and FFI contracts

The installed journal word format remains 0 (initial), 1 (first Lean start),
2 (second Lean start), 4 (stock escape), with 0x80000004 denoting rejection.
The normal cycle is Lean -> Lean -> stock -> Lean, including adb reboot.
Stock here means the original MBOOT-prepared kernel inputs for the same OS.
It does not select the Android slot. Home readiness does not reset the count.

BootJournal.withNext accepts decoded records and their validity flags. It
either rejects or passes a Commit containing source slot, next sequence and
next state to its tail callback. The interpreter writes Commit.target, then
reads it back and checks it before returning the state. Keeping this callback
form supports the existing freestanding specialization strategy; a boxed
result is not required by the source interface.

Formal proofs
-------------
VERIFICATION.txt maps the properties to their source theorems and records
the remaining assumptions. State machines cover ordering-sensitive workflows;
parsing and arithmetic use ordinary safe definitions with direct proofs.

Run from the repository root:

  nix develop .#bootloader -c python3 tools/bootloader/prove.py
  nix develop .#bootloader -c python3 tools/bootloader/check-safety.py

Proofs.lean proves properties of the executable source, including:

  - Policy equivalence to the previous bit encoding for all UInt32 inputs;
    invalid inputs reject, inspection preserves state, and the cycle repeats.
  - Journal packing preserves both 32-bit fields; selection never chooses a
    record marked invalid and recognizes an adjacent newer sequence at wrap.
  - Every accepted Commit has a valid state and targets the other slot (0/1).
    Two missing records reject without invoking the acceptance callback.
  - Range checks equal widened arithmetic. Accepted byte addresses do not
    wrap and stay inside the span. Out-of-bounds read8/write8 return their
    rejection value without calling the memory capability, for any monad.

ProofAudit.lean checks every theorem's transitive assumptions, including
generated helper lemmas. Only Lean's standard propext, Classical.choice and
Quot.sound are permitted: no sorry or native-evaluation assumptions. Standalone
proof modules are excluded from the firmware import graph; runtime termination
and derived equality proofs are erased. Generated proof artifacts stay
under assets/build/leanboot-proofs/.

These statements do not prove flash atomicity/durability, CRC collision
resistance, DMA/cache coherence, memory ownership, termination of partial
loops, stack bounds, or the Lean/C/ARM compiler chain. Record validity flags
are inputs to the pure selection proof; full codec/storage correctness is
still separate work. Read Unsafe/CONTRACTS.txt for the hardware assumptions.

NetworkBoot/Proofs.lean additionally proves entry alignment and a complete
first instruction inside the image, nonwrapping executable-address arithmetic,
busy-controller noninterference and no
crypto call for rejected input spans. check-safety.py exercises the actual
controller with deterministic effects and checks nominal type confusion is
rejected. Flow/Proofs.lean proves authentication, execution, installation and
journal ordering, including preparation before invocation. Every effect-failure
prefix is checked against the actual runner. Update/Receiver/Proofs.lean proves
sealed/duplicate protection and exact nonwrapping append bounds. ParserProofs
checks rejection before reads and RX payload bounds. Crypto observations and
the physical adapter are still trusted.

The safety/type cleanup and direct writer have passed physical RAM trials;
see research/G13_APP_UPDATES.txt for the current deployment state and hashes.
Earlier physical evidence is recorded separately in
research/G13_SIGNED_NETWORK_BOOT.txt. Rebuilding assets/build/ does not update
the TV. The previous installed-build snapshot remains in
assets/research/g13-network-default/build-snapshot.zip.
