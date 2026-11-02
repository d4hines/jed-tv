# Offline peer review: Rust TEE provider candidate

Reviewed the audio worktree candidate, then the integrated main files after
the ARM O_NOFOLLOW correction. No remaining blocking source defect was found
in the reviewed framing, ownership, metadata or failure paths. This is not
hardware validation or proof of the full cold TA request sequence.

Checks completed:

- RECV/SEND framing matches the exact V109 kernel dispatch. The receive
  header is8 bytes, params are32 bytes, and31 params occupy1000 bytes within
  the kernel's1024-byte bound. Kernel count is checked before slicing or
  calculating reply length. The wire buffer and16-byte envelope live through
  each synchronous ioctl.
- Leading META params remain outside the semantic handler and are returned
  unchanged with the original total count. Misplaced META returns an explicit
  error; malformed oversized counts stop the transport. Replies use a private
  mutable packet copy, avoiding mutable aliases of an immutable Rust packet.
- LOAD_TA accepts only owned writable backing for a nonempty transfer. Unknown
  backing can serve only the explicit zero-capacity/zero-offset size query.
  Unknown operations return NOT_SUPPORTED. Nothing reports successful copying
  to memory absent from the provider's ownership map.
- The exclusive teepriv open is checked against sysfs identity and OP-TEE /
  privileged GlobalPlatform version flags. The advisory lock is additional;
  exact kernel `optee_open` itself excludes a second supplicant context.
  Allocator descriptors are duplicates of that same open file description,
  not new privileged contexts.
- Region descriptors/mappings survive until explicit FREE or provider drop.
  Mappings drop before the allocator/device. Bounds use checked full-width
  offsets, lengths and IDs; mapped writes are volatile, with ordering fences.
  Failed allocation, duplicate IDs and failed TA writes do not publish a
  successful new result. Receive/send errors stop instead of sending a fake
  success or blindly retrying a possibly consumed operation.
- Three allowlisted TAs are loaded and hash-checked before entering the RPC
  loop. Path construction comes only from fixed UUIDs. The corrected main
  implementation uses ARM O_NOFOLLOW=0x8000, including a target compile-time
  assertion; the earlier generic0x20000 literal is no longer relied upon.

Concrete remaining hardware gates:

1. Confirm TEE_IOC_SHM_ALLOC and mmap work on this boot's privileged context,
   including actual rounded size/ID behavior and cleanup.
2. Observe the complete cold request sequence. FS/RPMB/vendor RPCs and external
   registered-memory imports are intentionally unsupported; they must remain
   visible errors until a justified implementation exists.
3. Establish that the serialized provider suffices for the actual TA sequence;
   nested or concurrency-dependent requests have not been demonstrated.
4. Verify successful initialization produces the intended display/audio result
   without stock tee-supplicant. A software reply alone is not that evidence.

Evidence checked: `assets/research/audio-agent-direct/kernel-tee-ioctl.txt`,
`kernel-optee-exclusive-open.txt`, the exact stock supplicant disassembly, and
current `src/tee-provider` / `src/tee-rpc` source. Root reports tee-rpc22 and
provider14 tests passing plus the ARM build. My separate locked offline test
attempt initially lacked a cached block-buffer dependency; I did not fetch
dependencies or repeat root's successful run. No source under review or main
was modified, and no device access occurred.
