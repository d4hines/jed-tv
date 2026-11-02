# Lean audio processor

Runtime modules are in `src/pid1/lean/TV/Audio/`:

- `Protocol.lean` owns the O/W/D/R/F stream, validation, PCM boost/clamp/gain,
  counters, reset, drain, deadlines and reporting. `Effects` contains the
  transport/device operations. Byte writes are exact; PCM writes finish or fail.
- `Converter.lean` owns the streaming rational 44.1/48 kHz mono/stereo to
  48 kHz stereo conversion and the inverse consumed-input-frame calculation.
- `Direct.lean` joins the complete session as `Direct.run transport device`;
  each session owns its converter and counters, and the native device enforces
  the exclusive lease.
- `Queue.lean` owns the 1920-byte ioctl bound, positive partial writes, 2 ms
  backpressure sleeps, common 5 s deadline and queue-count validation.

These are executable implementations. They are not yet connected to the
TV's Rust listener/device adapter. Native code still owns sockets, gain-file
reading, the exclusive route/volume lease, hardware initialization and cleanup.
The adapter calls `Direct.run` with transport/device effects. Each device write
is a single bounded ioctl, and all conversion/queue/accounting runs in Lean.

Counters and clocks cross effect boundaries as `Word64` (two UInt32 fields).
Keep those halves at native boundaries: boxed UInt64/unaligned scalar fields
in the pinned upstream ARM32 runtime reproduced a corrupted fake clock. The
processor uses UInt64 arithmetic only after decoding, and stores halves.

Host tests and comparison against the actual existing Rust source:

```sh
python3 test/lean/tv/audio/check.py
```

This checks every signed-16 sample with all four boosts and seven gain values
(1,835,008 conversions), 44 wire fixtures, eight converter comparisons, every
consumed-frame counter in those converter cases, fragmented input, error
propagation, reset cleanup, queue partial writes/backpressure and timeouts.
It includes the maximum permitted 1 MiB wire block.

ARM32 Linux execution, using the repo's cross runtime and kernel:

```sh
tools/lean-arm/build-exe.sh armv7-unknown-linux-musleabihf test/lean/tv/audio audioTests assets/build/lake/audio-tests/audio-tests-arm
python3 tools/mkinitramfs.py assets/build/lake/audio-tests/audio-tests-arm assets/build/lake/audio-tests/audio-tests.cpio
qemu-system-arm -M virt -cpu max -m 256 -nographic -no-reboot -device virtio-rng-device -kernel assets/build/kernel/arm/zImage -initrd assets/build/lake/audio-tests/audio-tests.cpio -append 'console=ttyAMA0 rdinit=/init panic=1'
```

All three PASS lines must appear, followed by init exitcode 0. The final kernel
panic is expected because this test executable is PID 1 and returns success.
This is ARM execution with fake hardware effects, not a TV/audio qualification.
Generated C, binaries, fixtures and logs stay under `assets/build/lake/`.
