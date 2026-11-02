G06 exact-kernel QEMU
====================

Build: python3 research/qemu-g06/build.py
Check: python3 research/qemu-g06/run.py --smoke --timeout 60
App checkout: python3 test/run.py faithful --stock-smoke --no-build

faithful now means the exact dumped G06/V109 4.19.116+ kernel. The previous
ARM32 6.18/virtio profile is named virt32; fast remains an ARM64 surrogate.
The stock profile has serial diagnostics, not a working media display.

Kernel SHA256:
01e36bda0554aed46ad3aa1d7fdf688174f46241117f62b4b95f02e25c47c11e

The runner checks the stock boot backup and captured live FDT hashes,
extracts the kernel unchanged, constructs a RAM-only initramfs and loads
kernel / FDT / initramfs at 20208000 / 23000000 / 24000000. A separate
24-byte loader stub initializes the normal ARM boot registers. The live
FDT's MBOOT memory data is retained; chosen bootargs and initrd bounds are
replaced for this offline test. No kernel code or data is patched.

The custom QEMU 11.1.3 machine supplies four Cortex-A55 cores in AArch32,
the G06 Linux RAM banks plus reserved ION/PM51 DRAM, GICv2 addresses, 12 MHz timers, UART register stride,
PSCI SMC startup, and partial RIU register behavior. AArch32 DBGDIDR is
supplied from the A55 debug-resource layout; upstream's model omits it.
MIU drain completion is immediate, appropriate to QEMU's coherent memory
but not evidence of physical DMA/cache behavior. Most RIU addresses are
only read/write backing cells: device initialization messages do NOT
establish working hardware emulation.

Explicit initcall exclusions:
  mstar_mci_init        eMMC not modeled
  mmc_sdio_init         SDIO not modeled
  mtktv_regulator_init  external PMIC/I2C not modeled
  mtktv_thermal_init    sensor/calibration not modeled
  ir_dynamic_config_init no captured runtime key-configuration service

The stock kernel still waits for bounded MMC completions before running
/init (roughly 12 guest seconds). These warnings are not a successful MMC
probe. Keep the exclusions visible in fidelity.json and the audit issue.

Smoke checks PID 1, uname, pthread creation/join, advancing monotonic time,
nanosleep, fork/wait, and exec of the actual tvinit binary. The wrapper also
requires four online CPUs and a live tvinit hold marker. Randomness is
reported separately: currently not ready in the isolated emulator.

Files per run under assets/build/qemu-runs/g06-stock-*/:
  Image, board.dtb, entry.bin, ramdisk.cpio.gz, cmd.json, serial.log,
  qemu.log, fidelity.json. Generated inputs/logs remain ignored by Git.

Remaining hardware gaps: https://github.com/d4hines/dande-tv/issues/11
No TV connection, flash, reboot, or other hardware state change is needed.

Verified final model:
  ABI/PID1: assets/build/qemu-runs/g06-stock-1793505465856833000/
  Decoder:  assets/build/qemu-runs/g06-stock-1793505466082731000/
The decoder completed four H.264 frames with 1/2/4 worker configurations
and RGB conversion. Its subsequent firmware poweroff handshake is not
modeled; do not call this a hardware throughput or clean-shutdown test.

Hardware-evidence update
------------------------
research/ISSUE_11_HARDWARE_DELTA.txt and research/g06-hardware-contract.json
narrow the physical gaps. The runner verifies all ten source hashes and
attaches their observations to fidelity.json, without the device serial.
Physical Ethernet/root PTY/upload/child execution and watchdog servicing are
proven separately from this emulator. Physical display initialization and
MMA mapping worked but did not produce visible Hello output.

QEMU exposes three read-only /machine QOM properties:
  watchdog-clear-count, watchdog-other-write-count, watchdog-last-clear-ns
Only byte value 1 at 1f006000 counts as the observed HAL clear. Expiration
remains unmodeled; no timeout is derived from the 90-second survival checks.
The runner saves these counters at shutdown. Test actual MMIO handling with:
  python3 research/qemu-g06/check-watchdog.py

Graphics regression requirements retained in the contract (not yet a device
model): fb0 1080p/one-page differs from MI fb1 720p/two-page; 64-bit IOVA
truncation causes legacy mmap ENOMEM; positive import fd is success; keep
the fbdev fd for pan while mapping the uncached exported MMA DMA buffer.
Do not treat successful drawing or presentation calls as visible output.

Entropy: one physical boot's CRNG message arrived at uptime 120.412019 s.
This is an observation, not a prescribed delay. Preserve early not-ready
coverage; neither this log nor our smoke test proves blocking getrandom
will be ready by a particular time.
