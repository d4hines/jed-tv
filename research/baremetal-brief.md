# Brief: bare-metal feasibility spike (no Linux) on QEMU virt

Question to answer with data: can our platform API run with no OS at all?
Today our apps run as PID 1 on Linux and use only fbdev, ALSA PCM, evdev,
sockets, a clock and getrandom (33 syscalls, 11 ioctls; see
`research/syscall-census-20261031.md` and main's `research/platform-api.md`).
Bare metal would replace Linux with our own Rust code that drives the
hardware directly. Build a minimal prototype on QEMU's `virt` machine and
report what it takes.

## Where (worktree /Users/d4hines/repos/dande-tv/agent1, branch d4hines/agent1)
- Work in `baremetal/` at the worktree root, as its own Cargo workspace
  (NOT under `app/`, so the main workspace is unaffected). Build outputs go
  to `assets/build/baremetal/` (`assets` is a symlink to main/assets).
- Never create files under /Users/d4hines/repos/dande-tv/main. The shell
  guard refuses complex compound commands (loops with variables, cd chains):
  keep Bash calls simple, use absolute paths, put multi-step work in scripts.
- Rust: `app/rust-toolchain.toml` pins 1.92.0. Add a `rust-toolchain.toml` in
  `baremetal/` (target `aarch64-unknown-none`; nightly only if you must, and
  justify it).
- QEMU 11.1.3 is installed. Use `qemu-system-aarch64 -M virt -cpu cortex-a55`
  (TCG) and also try `-accel hvf -cpu host`, with
  `-global virtio-mmio.force-legacy=false` and the same virtio-mmio devices
  as `qemu/run.py`: virtio-gpu-device (1280x720), virtio-keyboard-device,
  virtio-net-device, virtio-sound-device. Reuse the QMP helper idea from
  `qemu/run.py` (screendump, send-key) in your own `baremetal/run.py`.
  Only additive changes are allowed to `qemu/*.py`.

## Gates (prove each with evidence under assets/build/baremetal/runs/)
1. Boot to Rust: exception vectors, stack, BSS; log over the PL011 UART.
   MMU and caches on with identity mapping (normal memory for RAM, device for
   MMIO), or explain why not.
2. Pixels: virtio-gpu framebuffer showing the same colour-bar test pattern as
   ytplay's self-test (`app/crates/ytplay/src/selftest.rs`). Screendump it and
   look at it yourself with the Read tool.
3. Input: virtio-keyboard events (QMP send-key) logged as Linux keycodes,
   mapped with `app/crates/platform/src/remote.rs` (copy or path-depend on
   just that file's logic).
4. Time: generic timer; animate the moving square at a steady rate; report
   the frame rate.
5. Stretch: virtio-net + smoltcp (DHCP, then an HTTP GET of
   `http://10.0.2.2:8090/videoplayback?itag=140` with a Range header from the
   replay server, if it's running; check with `curl -s -X POST
   localhost:8090/youtubei/v1/player -d '{}'`). Stretch: virtio-sound 440 Hz tone.
Suggested crates: `virtio-drivers` (rcore-os), `smoltcp`, an allocator
(e.g. `linked_list_allocator`), `aarch64-cpu`. Record versions. Avoid C.

## Report (concise)
- What works, with evidence paths (checked by you).
- Lines of code, binary size, boot-to-first-pixel time, and frame rate,
  under TCG and HVF.
- What was hard: MMU/caches/DMA coherency, interrupts vs polling, SMP.
- An effort estimate for running the real player on bare metal: threads
  (SMP scheduler or async?), what FFmpeg's C and the Rust std/libc layer would
  need, TLS/DNS/TCP via smoltcp.
- What changes for the real TV: MStar display/audio/IR/EMAC instead of virtio;
  the TV currently boots in AArch32 (open question: GitHub issue #1).

## Hard constraints
No TV access (no adb/fastboot/192.168.77.x). No git commit/push. Don't
contact YouTube. No system-wide installs on the Mac (nix shells are fine).
