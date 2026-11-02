# Owned normal-world TEE RPC subset

This library implements three verified V109 tee-supplicant operations:
LOAD_TA(0), SHM_ALLOC(6) and SHM_FREE(7). It does not open a TEE device, receive
requests, change OP-TEE mode or initialize the TV. It is intended to fit the
owning PID1 after the real cold-start trace establishes the remaining RPC set.
All work and tests so far are offline; no TEE allocation was executed.

```
./tools/tee-rpc/check.sh
```

This runs22 host tests and cross-compiles an ARMv7 musl rlib, including its Linux
ioctl/mmap adapter. Output lives under`assets/build/audio-agent/tee-rpc/`.
The independent Cargo manifest has a local`[workspace]` for isolated testing;
remove that stanza if adding this crate to the main workspace.

## Embedding

`Provider<A,S>` owns an allocator, an allowlisted TA store, resource quotas and
all live SHM mappings. It is transport-independent and serializes mutation with
`&mut self`. The transport must strip leading META parameters while preserving
them in its reply, decode the32-byte `Param {attr,a,b,c}` values, call
`handle(function, &mut params)`, and send that returned TEE result unchanged.
Passing META attributes to the semantic handler returns BAD_PARAMETERS.

A future Linux owner can provide an already authorized TEE File to
`linux::LinuxAllocator::new(file, max_bytes)` and select
`DirectoryStore::audio(root)` for the existing audio TA. No service or
standalone CLI is provided. The library does not acquire authority through a
hardcoded device path. It never implements success stubs for unknown RPCs.

`DirectoryStore` derives the filename only from a UUID and looks it up in a
trusted caller-supplied length manifest. Its audio constructor permits only
`778ee68e-6ee0-11e8-adc0fa7ae01bbebc.ta`, length145328. The stock filename omits
the UUID's final hyphen. Linux file opening uses O_NOFOLLOW; files must be
regular, remain exactly the declared length, and fit the read bound. The root
directory and manifest are trusted owner configuration. Length checks do not
replace signature authentication; the opaque signed container still goes to
secure world for its usual verification.

## Exact contracts

The stock dispatcher at0x2930 strips leading META params, then dispatches the
remaining logical params using the table at0x2a56. Value attributes1..3 and
memref attributes5..7 are accepted by its helpers. This implementation accepts
those exact unflagged attributes and rejects unknown flag bits.

### SHM_ALLOC, function6

One value parameter. Requested byte count is param0.b; param0.c receives the
kernel SHM ID. param0.a and param0.b remain unchanged. The stock handler ignores
a's value; this library likewise does not invent a meaning for it.

The allocator uses TEE_IOC_SHM_ALLOC=0xc010a401 with a16-byte structure:
u64 size@0, u32 flags@8 (input0), i32 id@12. The ioctl return is the SHM file
descriptor. It mmaps the returned size with PROT_READ|PROT_WRITE, MAP_SHARED,
offset0. This Rust musl ARM32 mapping prototype correctly uses a64-bit off_t.

The provider clears a newly owned region and executes a Release fence before
publishing its ID. It also executes a Release fence after TA writes and before
returning a reply for the owner to send. The Linux backend uses volatile byte
writes; it never creates ordinary Rust slices over asynchronously shared memory. It maintains
the requested logical length separately from any kernel-rounded mapping length,
matching the stock node's capacity check. Quotas account the full mapped length.
If allocation, ID validation, clearing or quota checks fail, the candidate is
dropped and no ID is published. Duplicate IDs never replace an owned object.
Zero-size allocation is rejected as BAD_PARAMETERS as an explicit local bound.

Defaults: at most64 regions,8MiB per mapped region,32MiB total,8MiB per TA file.
Oversize/exhausted allocations return OUT_OF_MEMORY. A backend failure likewise
returns OUT_OF_MEMORY; invalid/duplicate backend IDs return GENERIC.

### SHM_FREE, function7

One value parameter. param0.b is the SHM ID. Only an ID owned by this provider
can be released; unknown, negative/wide and duplicate frees return BAD_PARAMETERS.
Removal drops the mapping: munmap first, then SHM file close. Provider drop
releases remaining regions before its allocator/device File is dropped.

### LOAD_TA, function0

Two parameters: value UUID and memref destination. The UUID's16 network-order
octets are the little-memory bytes of param0.a followed by param0.b. The stock
code reverses the first word and assembles the next halfwords into its native
UUID structure; the library stores the original network octets directly.

For a known owned SHM ID, param1.a is byte offset and param1.b is input capacity.
Both are checked at full64-bit width, including overflow, against the original
requested allocation length. No arbitrary address is accepted. On success the
TA bytes are copied to that range and param1.b becomes the exact file size.
If the known destination is too small, param1.b still reports the needed size,
SHORT_BUFFER(0xffff0010) is returned, and no partial bytes are written.

The exact stock get_memref helper permits an unknown SHM ID for an OUTPUT/INOUT
memref as a NULL size query. This implementation does the same: it returns the
required size without dereferencing anything. An absent INPUT memref is invalid.
This is specific to LOAD_TA and does not authorize access to any unknown region.
Unknown UUID returns ITEM_NOT_FOUND; manifest/file drift or copy errors return
GENERIC. Files beyond the configured limit return OUT_OF_MEMORY.

Unknown function IDs return NOT_SUPPORTED(0xffff000a), including FS, RPMB,
socket and vendor-specific operations. Nothing is silently acknowledged.

## Validation

Twenty-two host tests cover canonical audio UUID/name encoding; exact changed reply
fields; resource quotas and backend rounding; logical versus mapped capacity;
allocation failure/ID collision cleanup; unowned/double free; NULL size query;
short-buffer no-copy semantics; offset/length overflow; exact destination copy;
read/copy errors; allowlist and file-length checks; unknown commands; malformed
attributes; and final ownership cleanup. ARM cross-compilation covers the Linux
backend but is not hardware execution. Its allocation/mmap and Drop path need a
coordinator-owned smoke test before use in a real provider.

## Evidence and unresolved work

The original stripped Thumb binary is
`assets/research/native-vendor-stack/tee-supplicant`. Exact disassembly is
`assets/research/audio-agent-direct/tee-supplicant-thumb.txt`:

-0x2a56: dispatch table, command0→0x2a6c,6→0x2c46,7→0x2cd4.
-0x2a76..0x2aea: LOAD_TA count/type validation, UUID conversion and file loader.
-0x2454..0x24f4: memref lookup, full-width overflow/capacity checks, NULL handling.
-0x3338..0x3538: bounded filesystem TA lookup/read behavior.
-0x2e92..0x2eac: actual file length output, SHORT_BUFFER versus NULL query.
-0x2c46..0x2cd2 and0x2ec4..0x2ef4: allocation ioctl/mmap/node publication.
-0x2cd4..0x2d4a and0x2e4c..0x2e90: owned-list free, munmap and FD close.

The local Linux UAPI header is under the cached Zig tree at
`lib/zig/libc/include/any-linux-any/linux/tee.h`; request IDs also match the
original binary. The cached U-Boot `drivers/tee/optee/optee_msg_supplicant.h`
confirms RPC IDs but uses the secure-side wire form, not this Linux userspace
Param form; it must not be substituted for it.

Remaining blockers: real cold trace of request sequence, including metadata and
any vendor RPCs; transport receive/send loop and kernel SHM version selection;
FS/RPMB/other requests actually required by the audio TA; maintaining mappings
until secure world requests free; and live validation of this allocator.
The stock supplicant has an alternate registered-memory allocation branch
(TEE_IOC_SHM_REGISTER) selected by capabilities. This library currently exposes
only the proved TEE_IOC_SHM_ALLOC branch; the real trace must establish whether
that branch is suitable. No unsupported registration fallback is attempted.

External/kernel-provided SHM IDs are not imported or guessed. The stock unknown
OUTPUT/INOUT-ID size-query rule is retained, but no bytes can be copied into an
ID absent from our ownership map. If a real cold trace requires such mappings,
an explicit proven import mechanism is a prerequisite; do not silently treat a
missing mapping as a successful completed TA transfer.
