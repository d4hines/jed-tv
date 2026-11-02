G06 QEMU test approximations

Adapted from the app agent's test/fixtures/g06-init.rs and test/run.py described in
research/CODEX_HANDOFF.txt. These helpers are test-only and never included
by the firmware image builders. No TV access is needed for these tests.

tools/pid1/check-qemu.py now exercises current Hello source with:
  ARM32, four emulated CPUs, approximately the observed RAM, 1280x720 virtio GPU
  userspace static IPv4 on 192.168.77.0/24 (host .1 / guest .2), ip=off
  stale RTC at 2025-01-17, matching the observed unset TV clock
  inherited seccomp ENOSYS responses for ARM EABI calls 403..0xeffff
  retained ARM-private calls, including TLS/cacheflush
  self-checks for time64/new calls plus legacy thread/clock/sleep fallbacks
  actual PID 1 retained across trampoline exec and native hot uploads

Run:
  python3 tools/pid1/check-qemu.py

The harness builds current Hello into a separate assets/build/pid1-qemu
directory. It wraps a COPY of the kit ramdisk and never repins or modifies
the installed launcher, boot image, or kit. To use an existing Hello ELF:
  python3 tools/pid1/check-qemu.py --init PATH --case success

Cases cover visible Hello and repeated hot uploads, incompatible module
failure, no framebuffer device, debug-before-display delay, and the Linux
bootloader restart request. Failure cases must retain HTTP and reboot at
their userspace deadline. Kernel logs are fetched twice to verify reads do
not consume the ring. The RTC setting and syscall self-check are asserted.
Every run gets a unique directory under assets/build/hello-pid1/qemu/;
each case records command.json and fidelity.json with kernel, payload,
trampoline and initramfs hashes, modeled constraints and remaining gaps.

Limits:
The syscall filter is an absence model, not Linux 4.19 emulation. It inherits
no_new_privs and cannot model all old-kernel behavior or vendor backports.
The kernel is still newer, and virtio is not MediaTek. We do not model MI
initialization, configuration file reads inside vendor modules, IOMMU,
reserved memory, panel timing, target watchdogs, or audio routing. In
particular, a virtio framebuffer opening successfully cannot validate the
MI selector-1 readiness gate, DISP/OSD ioctls, or onebin memory map selection.
No-framebuffer is a generic negative case, not emulation of the MI driver.
Frame rate and boot duration under TCG do not predict the TV's speed.

The separate Doom and launcher harnesses have not yet adopted this helper;
their existing results must not be described as using the new profile.
