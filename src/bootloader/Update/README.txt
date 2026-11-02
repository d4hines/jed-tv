Signed bootloader updates, version 1
===================================

Current source: direct G13 eMMC application-image updates, using the same
signed Ethernet protocol as temporary network boot. No recovery Linux is
involved. This new writer and its application mounts are NOT installed or
hardware-tested. The installed older gate still supports only temporary
network execution. See tools/bootloader/update/FLASHING.txt for provisioning.

Design
------
The resident gate holds the Ed25519 public key and the fixed storage map.
Component 1 remains a temporary ARM bootloader (1 MiB maximum, no persistent
write). Component 2 is a signed application filesystem, entry=0, maximum
16 MiB minus one 512-byte header sector. Its EROFS/LZ4 bytes are opaque to the
bootloader. Filesystem parsing happens later in the normal Linux kernel.
The current gate is replaceable on this unlocked TV, not a hardware root of
trust. Signing an image authorizes its code; it does not prove that code safe.

FlashPlan.lean defines two 16 MiB banks inside the G13 upgrade partition:
  bank 0: absolute sectors 0x309000..0x310fff
  bank 1: absolute sectors 0x311000..0x318fff
The first MiB of upgrade and the boot journal at 0x3217fe/0x3217ff are excluded.
Boot firmware, partition tables, calibration and userdata are not writable
through this interface. These extents must be reserved before deployment;
they must not also be used by the vendor OTA system.

Flash.installImage derives the active bank from full local authentication.
Read errors abort the update; they are not treated as empty storage. A newer
signed version is required. If both banks are invalid, a signed first image
may initialize bank 0. Equal valid versions reject. No packet can choose a
bank, sector, partition name or physical pointer.

Installation checks the canonical header, policy, Ed25519 signature and image
SHA-256 before writing. It durably invalidates the other bank's header, writes
the payload, verifies its storage readback and flushes, then writes the signed
header last, flushes and checks header readback. The authenticated active bank
is never the write target. Failure during final commit may still leave a valid
new bank: callers must rescan, not assume failure means nothing changed.

The network gate offers Ethernet before checking the normal kernel, including
on the third-start MBOOT-input fallback. Stage0 and the gate must still load.
Before handoff it authenticates both app banks and publishes only its chosen
bank through /chosen/dande,apps-bank in the final FDT (also on stock-input
fallback). Normal startup mounts that bank read-only over root/bin and
root/share. Settings, writable app data, Doom's existing WAD and native
platform helpers remain in their current locations. With no authenticated
bank, the existing pre-provisioning app tree remains available. This is not a
claim that the entire legacy OS is verified.

The generic installNext API and RAM-only model still require a pre-existing
authenticated bank. Flash.lean adds the explicit first-provisioning policy.
Neither physical kernel/bootloader replacement nor key rotation is supported
by the application writer. Recovery of a broken app image works without a
running OS; recovery of a broken kernel or gate needs a separate path.

Manifest and package
--------------------
All integer fields are little-endian UInt32. The package is exactly 160 bytes
of header followed by the raw image, with no trailing bytes or outer compression. An application image is
already an internally compressed EROFS filesystem.
The 96 signed bytes are a fixed encoding; no JSON normalization, scripts,
optional fields or embedded public key are accepted.

  offset  size  field
       0     8  ASCII LBOOTUP1 (signature domain)
       8     4  format = 1
      12     4  target board/ABI = 0x00130001 for G13 V013
      16     4  component = 1 (temporary ARM stage) or 2 (application image)
      20     4  release version, 1..0xffffffff, never wraps
      24     4  image length, policy-bounded; global cap 16777216
      28     4  entry offset (ARM component); exactly zero for applications
      32     4  codec = 0 (raw)
      36     4  signature algorithm = 1 (Ed25519)
      40    32  SHA-256 of exactly the image bytes
      72    24  zero, reserved
      96    64  Ed25519 signature over bytes 0..95
     160     N  image

The private signing key stays on the development/signing machine. A build
requires an explicit raw 32-byte public-key file; there is no built-in default
key, accept-unsigned mode or network key replacement. Tools/tests use an
obviously public RFC 8032 test key and must never be provisioned as releases.
Key rotation is outside version 1. A future rotation needs authorization by
an already trusted key and a defined revocation/recovery policy.

Version checks reject downgrades relative to the currently valid stored banks
and the verifier's compiled minimum. They are not hardware anti-rollback:
restoring the entire storage or invalidating a newer bank can reveal an older
valid release. RPMB/fuses/protected monotonic state have not been established.

Raw transfer avoids adding decompression to this first authentication boundary.
If compression is added, sign both compressed and expanded sizes/hashes and
bound expansion before decoding. CRCs remain transport diagnostics, never
signature substitutes.

Transport
---------
Receiver.lean processes bounded UDP payloads, independently of storage. The
existing Ethernet parser validates IP/UDP and hands over a CPU-owned buffer;
NetworkBoot and UpdateRamEntry connect it to the shared server. Transport is
untrusted and need not provide TLS for image authenticity. It can still be
jammed or interrupted; availability is not established by signatures.

Request is five little-endian words plus at most 1024 data bytes:
  magic 0x3152554c, opcode, nonzero session ID, offset, count.
  Begin=1: offset 0, count total package size, no data.
  Data=2: exact next offset, count data bytes. Exact duplicate chunks ACK;
          modified duplicates, gaps and overlapping extensions reject.
  Seal=3: offset total, count 0, no data. Requires all bytes received.
  Abort=4: offset/count 0, no data. Requires current session ID.

Server.lean connects the receiver to authenticated installation. Its 40-byte
state includes the receiver's 16 bytes and a cached final Seal response. It
resets receive state after each completed installation attempt, allowing the
next transfer without losing idempotence for a repeated final packet. The
host model performs no authorization, version or commit decisions.

Reply: magic, session ID, request opcode, status, acknowledged byte count.
Receiver status: 0 ACK, 1 malformed, 2 wrong/busy session, 3 offset/bounds,
4 incomplete, 5 changed retransmission. The local server adds 16 to nonzero
installer results on Seal. A successful Seal response means the selected
operation accepted the package: bank installation completed, or temporary
execution was authenticated and scheduled. It does not attest that uploaded
code ran successfully. Cache the final response across
retransmission; a new transfer uses a fresh session. The session is a transfer
identifier, not authentication. No bytes are executed or persisted by Receiver.

Installer errors are 1 input/workspace, 2 authentication/policy, 3 invalidation,
4 payload write, 5 payload readback/hash, 6 header commit/readback, 7 missing or
ambiguous active selection, and 8 physical-bank read failure. The application
wrapper permits signed initial provisioning when both banks are invalid;
the generic installNext wrapper rejects that missing-active case with 7.

Functional and trusted boundaries
---------------------------------
Policy.lean: ordinary values and decisions, no effects.
Manifest.lean: Reader + Crypto, canonical parsing and authentication.
../Flow.lean: total runner and explicit ordered stages, shared by authentication,
network execution, installation and journal persistence. Failed stages stop
immediately; a signed header is published only after image readback and flush.
Receiver/Plan.lean: pure typed transfer state and decisions, with append,
retransmission, session and sealing proofs in Receiver/Proofs.lean.
Receiver.lean: Reader + Writer for owned RAM, no storage capability.
Server.lean: request dispatch, installation and response caching in Lean.
Install.lean: Reader + Writer + Crypto + Storage; callbacks return selected
metadata and keep the freestanding specialization scalar.
Crypto takes nominal CpuAddress and ByteCount values; raw wire fields are
converted explicitly. CryptoBounds.lean rejects null/wrapping spans and
oversized hash input before any foreign call.
Unsafe/UpdateCrypto.lean: MemoryM-colored signature/hash calls, explicitly
lifted into HardwareM. Private raw foreign functions are never exposed to
policy modules. Unsafe/update/crypto.c is the small C boundary and memory
support for pinned HACL*. No new authored ARM instructions.

Storage.read/write take bank 0/1 and a relative sector, not an absolute LBA.
Every concrete backend must enforce those bounds, own stable DMA buffers,
serialize operations, and honor flush/durability. UpdateModel under tools/
supplies three trapping model functions. Unicorn intercepts them; accidental
hardware execution traps rather than writing arbitrary eMMC. Unsafe/UpdateRam
supplies the two 32 KiB volatile banks used in the physical TV trial. Neither
is a persistent eMMC backend. The physical writeJournal restriction has not
been widened.

Unsafe/Flash is the separate physical application-bank backend in the local
candidate. FlashPlan supplies its address allowlist; the RAM-only experiments
remain useful independent test adapters. No physical provisioning occurred.

Proofs and limits
-----------------
The 12 named lemmas in Update/Proofs.lean prove the executable policy requires
signature success, digest success, compatibility and a strictly newer version; false signature
or digest cannot invoke withInstall's continuation; its target is the other
bank; commit requires readback/durability success; selection only returns a
bank marked authenticated and rejects missing/ambiguous candidates.

Flow/Proofs.lean additionally proves prerequisite ordering over arbitrary
completion traces, rejection of failed/out-of-order steps, successful runner
ordering, and a sufficient finite execution budget. Production interpreters use
that runner. Receiver/Proofs.lean proves sealed and duplicate chunks never
append, complete-length sealing, session isolation and nonwrapping appends.
ParserProofs.lean covers early rejection and Ethernet descriptor payload bounds.
FlashProofs.lean adds fixed physical bounds and journal exclusion. Its
rejected_signature_only_rejects theorem equates the actual parsed-manifest
installer to a pure error, including insufficient workspace, when the verifier
action is pure false. It is independent of the writer/storage implementations.
The payload-rejection theorem covers valid signatures with rejected payload
digests. Exact sector arithmetic and disjoint-bank proofs support StorageModel,
which separates pending writes from durable media and permits arbitrary torn
bytes. It proves preservation of the other bank and journal over arbitrary
finite traces, conditional on physical sector isolation. A completed abstract
flush survives power loss; whether the card implements that barrier remains
a hardware obligation. No driver-to-model refinement theorem is claimed.
See ../VERIFICATION.txt for exact theorem names and scope.

These are not an end-to-end theorem that all machine executions require a
cryptographic signature. Crypto observations, their link to actual bytes, the
full refinement of physical effects to traces and the physical backend remain
obligations. HACL* supplies separately verified crypto
source; its F* proofs are not replayed here or imported as Lean axioms. The
Lean proof audit permits only standard logical axioms, with no sorry.

The executable ARM checks use actual Ed25519/SHA-256, independent OpenSSL
signatures, real packet/manifest parsing and a bounded two-bank store. Fault
injection covers interrupted write prefixes and flush failure. These finite
checks are not a proof of arbitrary eMMC torn writes, write reordering, wear,
DMA/coherency, CPU faults, secret-key security, or compiler correctness.

Initial local validation (2026-11-01; public test-key build)
---------------------------------------------------------
The final model build is 101368 bytes, with no heap allocation, writable ELF
globals or new authored ARM instructions. Its 23 recorded source hashes
matched the checkout at validation. It uses the public RFC test key only.

All 71 ARM checks passed, including 36 interrupted-write cases and readback /
flush failures. The largest observed stack use in those checks was 7892 bytes;
this is a measurement, not a worst-case bound. A separate loopback UDP check
passed two consecutive updates, duplicate final acknowledgements, and rejection
of a forged signature without storage changes. All protocol and installation
decisions in that check ran in the compiled Lean ARM implementation.

The proof command checked 103 theorem declarations (including existing proofs
and generated helpers), with only standard Lean logical axioms. Both the
existing bootloader boundary audit and the new updater audit passed.
No TV access or flashing was performed for this validation.

Evidence logs: assets/research/g13-update-ram/. Build outputs under
assets/build/leanboot-update/ are regenerated when selecting a different
public key; they are not immutable snapshots. Reproduction commands are in
tools/bootloader/update/README.txt. These are model results. The later hardware
RAM-bank and network-entry results are in research/G13_SIGNED_NETWORK_BOOT.txt.

Temporary network execution
---------------------------
NetworkBoot uses Server.dispatch with an authentication continuation instead
of installNext. It shares the package, key checks and wire protocol above.
The installed policy allows component 1 for board 0x00130001, up to 1 MiB,
minimum version 1. Repeating a signed version is allowed for development
because no persistent bank version changes. This does not relax installNext.

Once authenticated, the package is frozen against new chunks. After a bounded
final-ACK interval, Ethernet DMA stops, the package is authenticated again,
and the copied executable is hashed before entry. A returnable ARM/AAPCS
payload gets its own 64 KiB stack and a versioned context; see
Unsafe/CONTRACTS.txt. No upload or a rejected upload falls through to the
ordinary OS boot. The installed older third-start escape skips Ethernet;
the new local candidate offers the update window on that path too.

Signatures establish authorization and image integrity, not that development
code will return or preserve the resident loader. The current root of trust
is still replaceable by privileged access to this unlocked TV.

Next hardware integration
-------------------------
1. Establish dedicated bank extents and backup/provisioning/recovery, without
   overlapping live partitions, upgrade scratch usage or the boot journal.
2. Add a bank-bounded FCIE backend and validate durability assumptions.
3. Reuse the established Ethernet receiver for persistent-bank installation
   and integrate boot selection and an authenticated previous-bank fallback.
4. Exercise power interruption and actual storage durability. The successful
   volatile-bank trial does not establish those properties. Do not overwrite
   a running first stage as an ordinary bank update.

References (design guidance and exact crypto provenance):
https://www.rfc-editor.org/rfc/rfc9124.html
https://www.rfc-editor.org/rfc/rfc8032.html
https://github.com/hacl-star/hacl-star
The exact HACL* revision and every downloaded file hash are in
 tools/bootloader/update/crypto-lock.json.
