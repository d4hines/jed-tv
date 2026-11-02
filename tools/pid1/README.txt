Current owned cold-start candidate
==================================
The REPL image now stages the verified stock configuration and modules in RAM,
then initializes display and audio directly in Rust. It starts no Binder
manager, midaemon, SurfaceFlinger, AudioFlinger or Java runtime. The remaining
vendor process is tee-supplicant, launched under our Rust RPC tracer. The new
cold sequence is build-verified but still requires its first hardware test.

Build the canonical image (includes its pinned console and TEE tracer):
  python3 tools/pid1/build-repl.py
Verify console, loopback, reload and non-G06 hardware refusal in QEMU:
  python3 tools/pid1/check-repl-qemu.py --expect-direct-refusal

The console and watchdog stay independent of hardware startup failures.
GET /status reports hardware and media states. An audio failure retains the
display owner. Once PID1 claims hardware, POST /exec is refused to preserve
DMA lifetimes; applications can still be uploaded and restarted through REPL.
Use a normal reboot before replacing the supervisor.

Do not boot during a no-light/no-sound session: even a muted image can show the
bootloader logo before PID1 runs. Keep this work local until physically tested.

Historical investigation follows; the Java/vendor startup commands below
are evidence for the old bring-up path, not the current boot recipe.

G06 V109 native PID-1 Hello world and diagnostics

Latest checkpoint: physical Hello confirmed under native PID 1 (2026-11-01)
----------------------------------------------------------------------------
The owner confirmed "Hello world is visible" after the stock boot-logo
layer was explicitly disabled. Native Rust init remained PID 1, with its
persistent root shell and watchdog service. Android's system_server and
launcher were not running. Stock SurfaceFlinger, hardware composer,
allocator, Binder managers, midaemon and tee-supplicant ran as child
processes. The Hello client used app_process32 and the stock Java runtime.

Interactive shell:
  ./repl.sh
Repeat Hello on the CURRENT initialized native session:
  python3 tools/pid1/graphics.py --seconds 120
Or directly from the TV shell:
  /tmp/tv-bootlogo
  sh /tmp/hello-native.sh 120

For a fresh native shell boot, the consolidated startup is prepared:
  python3 tools/pid1/graphics.py --start --seconds 120
This consolidation is syntax-checked; the equivalent steps were measured
interactively, but the full fresh-boot script has not yet been rerun.
It needs the captured android-properties.tar.gz under assets/research/
native-vendor-stack/ and the existing native helper binaries. Build them:
  cd pid1
  CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER=/usr/bin/cc cargo build --locked --offline --release --target armv7-unknown-linux-musleabihf --bin tv-binder --bin tv-bootlogo --target-dir ../assets/build/native-tools
  cd ..
Do not run --start twice in the same session. It refuses an already-running
midaemon or already-loaded mik. No flash is required for repeat Hello runs.

Key missing startup pieces, now identified:
- Restore stock properties in RAM, especially ro.hardware=mt5867.
- Provide named eMMC aliases under /dev/block/platform/mstar_mci.0/by-name;
  libmutils reads MBOOT and MPOOL to select the exact model configuration.
- Alias /vendor/tclconfig to the read-only /tclconfig mount.
- Start allocator alongside the correct hardware composer and SurfaceFlinger.
- Provide Java linker namespaces, the stock boot classpath, and TCL's
  /system_ext/framework/tcl-framework.jar.
- Hand off the boot logo with MI_OSD_DisableBootLogo followed by
  MI_DISP_DisableBootLogo using the existing display controller handle.
  Until that handoff, screenshots showed Hello but the panel showed unlock.

All runtime changes were in RAM. eMMC nodes remained read-only; ext4
mounts used ro,noload. Userdata was not mounted. The loaded stock SELinux
policy definitions ran with the native session's enforce=0. The simple
Java linker map is deliberately permissive for this research environment,
not an Android security configuration. Next: instrument these child
processes and reduce the required startup set without losing visible output.

Evidence: assets/research/native-vendor-stack/native-hello.png,
hello-result-console.log, bootlogo-disabled-console.log,
after-disable-bootlogo-kernel.log. Physical visibility is the owner's
confirmation; a compositor screenshot alone did not establish it.

Earlier checkpoint (superseded): native vendor-stack experiment

--------------------------------------------------------------------
Early Android tracing is ON HOLD at the owner's request. Its prepared hook
was never installed. Do not arm it or return to that investigation unless
requested. Current goal: start a reusable display stack under our native
PID 1, with clearing the unlocked warning accepted as an initial milestone.

The owner powered Android on, G06/V109/root were verified, and another
one-shot native REPL boot succeeded. Evidence:
  assets/research/pid1-repl/20261101T035502Z/
Stock tvconfig (p7), tvservice (p19), and tclconfig (p27) were mounted
read-only, with journal replay disabled. The original super partition p28
was exposed through read-only loop devices: system offset 1048576, vendor
offset 734003200, matching the original lpdump single-extent layout.
src/pid1/scripts/mount-stock-runtime.sh exposes system/vendor binaries and flattened
APEX libraries. Userdata was never mounted. Native PID 1 remained running.

Stock midaemon initially blocked on a TEE RPC. Its original tee-supplicant
and small Bionic runtime were extracted from the exact firmware into
assets/research/native-vendor-stack/. Running tee-supplicant (PID 359 in
this boot) resolved the wait; its working directories are in RAM. Creating
Binder devices via the small tv-binder binary then revealed that the daemon
also needs /vendor/bin/vndservicemanager /dev/vndbinder (the argument is
essential). With that process (PID 547), midaemon stayed running (PID 548).
Its log reports MI daemon startup but display initialization is incomplete;
visible output is NOT proven. Logs are in assets/research/native-vendor-stack/.

Additional stock processes started: logd (468), servicemanager (497), and
hwservicemanager (601). Their initial SELinux-status/unknown-class errors
were addressed by mounting selinuxfs and loading the exact stock policy
definitions; the native session's pre-existing enforce=0 was retained.
No persistent policy or boot image changed. Loading the stock mali_kbase
module succeeded and created /dev/mali0. The hardware composer binary then
selected the fallback framebuffer adapter and exited: missing property
infrastructure means ro.hardware is unset, so it cannot choose the correct
mt5867 HAL. Magisk resetprop could not initialize absent property areas.
That was the current lead when the owner said they were unplugging the TV.
All device commands were stopped; no flash was in progress. A normal power
cycle should select installed Android, since the one-shot was consumed.

The initial dtv_driver/devnode captures through su were corrupted by CRLF
translation. Both were re-extracted using read-only debugfs and placed into
tee-runtime.tar, but the corrected modules have NOT been reloaded yet.
The bad copies were rejected as invalid format and never loaded. Do not
use raw su stdout for binary transfer; base64 or verified file pull is needed.
tv-binder is built at assets/build/native-tools/armv7-unknown-linux-musleabihf/
release/tv-binder. Other new source: research/probes/start-vendor-display.sh (still needs
to incorporate the later TEE/Binder dependencies and corrected modules).

Graphics and early Android trace checkpoint (2026-11-01)
------------------------------------------------------
From the working native shell, stock DISP/OSD initialization, framebuffer
allocation, exported MMA DMA-buffer mmap, pixel writes and page presentation
all succeeded. The owner did NOT see Hello. The software phase named
hello-visible is therefore only a presentation return-code check, not proof
of physical visibility. Evidence: assets/research/pid1-graphics/first-helper/.

The legacy fbdev mmap truncates the 64-bit IOVA; the first allocation at
0x200000000 consequently appears to have smem_start=0. The native helper now
gets its full IOVA with 0x80104672, queries the MMA global name (0xc0284d12),
imports the DMA-buffer fd (0xc0284d0a, positive fd return is success), sets
uncached mapping (0x40284d06), and mmaps that fd. It retains the same fbdev fd
for page flips. Closing a failed drawing child can leave OSD layer records
behind, so --reset-osd uses the stock MI_OSD_DeInit/Init pair between attempts.
These changes have worked on hardware and the generic framebuffer path
still passed its QEMU drawing/HTTP/hot-exec/reboot test. Visible output remains
unresolved. A direct boot-logo fb0 drawing attempt also returned success.

The user requested instrumentation of a complete successful Android startup,
not attachment after Android has booted. Prepared files:
  research/probes/graphics-early-trace.sh       one-shot Magisk post-fs-data hook
  research/graphics-early-trace.py       test/install/collect host runner
  research/probes/graphics-trace-probe.S       inert ARM/Thumb uprobe execution test
The hook is NOT INSTALLED yet. The native session requested normal Android
reboot after saving its logs, and USB ADB has not returned; the TV may need
one remote Power press. No boot image was flashed during this graphics work.

Exact vendor init.fusion.rc loads utpa2k/mik/fbdev in early-boot, after
post-fs-data. The proposed hook records module presence at arming so this
ordering can be verified live. It captures syscall entry/exit plus the
Android/native MI library ioctl wrappers and selected initialization calls,
with argument words. Syscall trace events were verified in the native
session. Function tracing reports CORRUPTED/ENODEV and is not relied on.
Thumb uprobe registration worked; execution with known arguments still needs
testing before probing Android libraries. The host install command performs
that test first. Native libmi.so probes are installed when its mount appears;
their ready marker must precede its first init call for complete coverage.

Native PID-1 shell (current work, 2026-11-01)
------------------------------------------
The repl feature runs our Rust supervisor as PID 1 and static BusyBox shells
as children. Android userspace is absent. The shell preserves cwd, variables
and jobs across commands; disconnecting closes that shell, while uploaded
files remain in RAM for a new session. HTTP diagnostics stay on port 8765;
the PTY console is on 8766. Both accept only the directly connected Mac .1.
This is for the isolated Ethernet link, without authentication or encryption.

  python3 tools/pid1/build-repl.py
  python3 tools/pid1/check-repl-qemu.py
  python3 tools/pid1/boot-repl.py --execute

The boot runner checks G06/V109 identity, the installed one-shot launcher,
matching QEMU evidence, and the watchdog instruction check before arming.
It performs one normal reboot and uploads the new executable into RAM in
the launcher's initial delay. It does not flash partitions. The one-shot
request is consumed before entry, so the next normal boot runs Android.
After a reboot the TV may remain in standby: press Power once if it has no
logo; do not keep toggling Power while boot logos are showing.

Once the runner reports shell-ready:

  ./repl.sh
  ./repl.sh -c 'id; readlink /proc/1/exe; ps'
  ./repl.sh --put path/to/helper --name helper

In the interactive console, run /tmp/helper. Ctrl-C interrupts a job;
Ctrl-] disconnects. Files and kernel modules last until the next reboot.
Use python3 tools/pid1/watch.py --reboot to return to Android. Do not use POST
/exec for ordinary helper tests: that replaces PID 1 and its watchdog
thread. Upload and execute helpers as child processes instead.

The default native shell has no automatic reboot deadline. Its optional
hello.repl_timeout is separate from the launcher's inherited hello.timeout.
It sets visible
eMMC block devices read-only for this kernel session and feeds the existing
watchdog through the stock /proc/utopia driver. It loads iniparser, kdrv_xc
and utpa2k from the existing RAM kit, opens WDT module 64, issues INIT 0,
then CLEAR 2 every two seconds. Exact V109 disassembly shows INIT obtains
the kernel RIU mapping; HAL_WDT_Start returns true without hardware writes.
CLEAR writes byte 1 at physical 0x1f006000 and leaves timeout registers alone.
The kernel has CONFIG_DEVMEM disabled, so /dev/mem cannot support this path.

Hardware success (2026-11-01T03:41Z): the corrected driver-based build ran
as PID 1 on the original G06/V109. A live root shell retained its working
directory and environment across a 90-second observation with no reboot.
A second console, Ctrl-C job interruption, and uploading/executing a script
in RAM also passed. Android adbd/zygote/surfaceflinger were absent. All 33
eMMC block devices were verified read-only, and the stock watchdog INIT and
CLEAR calls succeeded. The native environment was left running.
Evidence: assets/research/pid1-repl/20261101T034148Z/ (result.json,
console.log, live-job-control.txt, status.jsonl, events.log, kernel.log).
Build 56431dc839fae66939b79c8bb49aee3a3a2479dba4be3968ecc406864d20288d;
ELF SHA256 3ba05b1aa5464b4143a21aa3cdc52e82af442cc4704ff437c143cae4ac8aee50.

QEMU independently verifies the PTY, root shell, persistent state, Ctrl-C,
simultaneous shells, child reaping, uploads and execution, and normal reboot.
Unicorn verifies the exact stock HAL start/clear instructions. QEMU has no
real G06 watchdog. The first hardware attempt lost contact after hot exec;
the earlier /dev/mem startup requirement was invalid. The successful build
uses the stock kernel's /proc/utopia interface instead.

For graphics iteration, build the separate MI helper with build-mi.py,
upload its hello-pid1 executable as /tmp/hello-mi, and run
/tmp/hello-mi --draw from the native shell. This keeps our supervisor,
network console and watchdog alive while the child tests the display.
The child exits after a failed attempt or 60 seconds of successful drawing.
Vendor kernel faults can still reboot or hang the whole TV.

MI display candidate (2026-11-01T01:35Z)
--------------------------------------
Update 2026-11-01T03:13Z: corrected SYS ioctl envelope worked on the TV.
Selectors 40/6/9 were all ready=1; selector 1 was published and read back.
The MI module registered fb1 alongside the kernel's existing fb0. Opening
fb1 succeeded, but the generic framebuffer geometry/backing check rejected
it before mmap/drawing. Evidence: assets/research/hello-pid1/20261101T031318-miZ/.

The next candidate logs both fb var/fix structures, and for an unallocated
MI framebuffer uses exact driver's private ioctls: 0x40044696 to request
IOMMU memory, 0x40104675 for width/height/pitch/format, 0x4681 to initialize
the OSD layer, and 0x40014663 to show it. These are direct fbdev ioctl
arguments, unlike MI's extra wrapper. The dispatch/argument layout was
traced offline at mstar_fb_ioctl offsets 0x2950, 0x1d30, 0x212c and 0x1f84.
This allocation path is built and QEMU-plumbing tested but not yet tried on
the real framebuffer. Raw structures and the last completed ioctl will be
captured on the next test; visible Hello remains unproven.

  python3 tools/pid1/build-mi.py
  python3 tools/pid1/check-qemu.py --init assets/build/pid1-mi/target/armv7-unknown-linux-musleabihf/release/hello-pid1 --case success
  python3 tools/pid1/test-mi-live.py --execute

The live runner deliberately reboots the verified original G06. It requires
Android/root ready, the installed launcher marker, no pending request, and
matching candidate/QEMU evidence. It arms one boot and uploads the candidate
during the installed Hello's five-second pre-module delay. A missed window
requests normal reboot instead of retrying initialization after old modules.
It saves status/logs/kernel logs and requests Android return on display
failure, after 20 seconds of successful drawing, or after 30 seconds. The
candidate has its own 40-second deadline; the hardware watchdog may act too.
Normal reboot may leave standby requiring one remote Power press.

This separate static ELF embeds 331 board-specific configuration files and
three extra stock modules, staging 334 files into RAM. It enables DISP/OSD,
disables tuner auto-init, retains stock FLASH/SYS initialization, preserves
the model-specific onebin maps, and uses stock module ordering. Before MI
initialization it makes every visible mmcblk0 block device read-only with
BLKROSET and verifies BLKROGET; it refuses hardware initialization if that
cannot be done. These flags are kernel state, not partition writes. They
reset on the next kernel boot. This protects ordinary block writes; it is
not a proof against arbitrary vendor-driver hardware accesses.

After module load, the helper queries selectors 40/6/9. Only if all report
ready does it publish selector 1, read it back, and load/open the framebuffer.
It records framebuffer nodes before/after loading mstar_fbdev_mi and selects
the newly registered node. It does not assume fb0, because the stock kernel
already registers a boot-logo framebuffer. Hot uploads can reuse the MI
node recorded in RAM, and verify their existing block aliases before reuse.
It uses native /dev/mik/sys or Android-style /dev/mik!sys. MI ioctls require
an additional outer size/reserved/u64-pointer envelope; see the corrected
research/g06-display/mi-display-abi.h and check-ioctl-wrapper.py. Native
kernel logs exposed that missing wrapper in the first direct query.

Measured hardware progress:
  20261101T013118-miZ: 33 eMMC devices verified read-only; 334 files staged;
  mik loaded and the selected MMAP was the model_1 1.5G map. Query failed
  because the initial helper used Android's node path. After Power, Android
  returned with boot_completed=1 and tvroot.ready=1.
  20261101T013259-miZ: native /dev/mik/sys opened; unwrapped GET returned
  vendor code 8. No readiness was published and no framebuffer was opened.
  Logs captured and normal reboot requested. Owner left; Android return
  has not been verified after this test. One Wake-on-LAN attempt has not
  produced an observed wake.
Evidence directories are under assets/research/hello-pid1/. MI's attempted
environment writes failed in kernel logs with the read-only protection in
place; its generic damaged-environment wording is not proof of disk damage.

The corrected three-level ioctl candidate is built and QEMU-tested, and
exact libmi3 wrapper instructions were tested offline with Unicorn. Actual
MI status/readiness, framebuffer opening, and visible Hello remain unproven.
The installed boot image and pinned launcher payload have not been changed.

Current hardware result (2026-11-01T01:20Z): the one-shot launcher runs
this program as real PID 1 with working Ethernet diagnostics. Display fails
loading mik.ko because the kit lacks mik_init.ini and sys.ini. The first
test returned to Android via a roughly one-minute hardware watchdog reset.
A second test successfully replaced PID 1 over HTTP with a diagnostics-only
build and captured the kernel log. See tools/launcher/README.txt for evidence.

GET /kernel-log in newly built binaries copies up to 256 KiB of the kernel
ring without clearing it. The separately built diagnostics-only Cargo
feature skips display initialization, preserving existing kernel/module
state after a hot upload. It retains HTTP and the bounded reboot deadline.
These source changes have not been repinned into the installed launcher.

Build a separate diagnostic executable without replacing the pinned kit:
  cd pid1
  CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER=/usr/bin/cc HELLO_BUILD_ID=kernel-diagnostics-v1 cargo build --locked --offline --release --features diagnostics-only --target armv7-unknown-linux-musleabihf --target-dir ../assets/build/pid1-diagnostics
  cd ..

Once the one-shot Hello reports display-failed, upload and capture:
  python3 tools/pid1/watch.py --exec assets/build/pid1-diagnostics/armv7-unknown-linux-musleabihf/release/hello-pid1
  python3 tools/pid1/watch.py --kernel-log
  python3 tools/pid1/watch.py --reboot

The upload replaces all userspace threads, including its previous deadline;
the diagnostic program starts a fresh deadline. The hardware watchdog is
not serviced yet and may reset earlier. Retrieve the log promptly.

Build the preserved RAM-boot kit:
  nix build .#hello-pid1-kit

result contains init, kernel, dtb, initramfs.cpio.gz, cmdline.txt and manifest.json.
The kit is not a flashable boot.img. Its default normal reboot returns to stock
only while the original stock boot partition remains installed.

The owner subsequently authorized a direct-flash experiment using the proven
USB recovery stick as fallback. Build the separate flashable version:
  nix build .#hello-pid1-boot --out-link assets/build/hello-pid1/boot-result

It retains the stock packed DTB and kernel, and adds hello.return=bootloader.
The remote reboot endpoint and 90-second deadline use Linux RESTART2 with
the command bootloader. This request is QEMU-tested, but the TV's handling
is unverified. It might restart the Hello image instead of entering Fastboot.
It does not restore stock, write misc/BCB, or write any firmware itself.
If automatic bootloader return fails, use the RECOVERY_NOTES.txt USB route.

First test, with the working recovery stick available, in two terminals:
  python3 tools/pid1/watch.py
  adb -s 4C0490290FA05A0C0 reboot bootloader
  ./flash.sh .#hello-pid1-boot

The watcher binds en8 and 192.168.77.1, waits up to four minutes, and saves
status/logs under assets/research/hello-pid1/. Press the remote's Power once
if the TV remains in standby after flashing, as with the stock/PoC trials.
There is no ADB in this image. Full boot images are installed through USB
Fastboot again. To request an early bootloader return:
  python3 tools/pid1/watch.py --reboot

When Fastboot is available, either iterate or restore Android:
  ./flash.sh .#hello-pid1-boot
  ./flash.sh .#stock-boot

Do not run both alternatives in sequence. Restoring stock boot is required
to return to Android; a normal reboot alone cannot restore it.

Fast native-program iteration after the first successful hardware boot:
  ./pid1/build.sh
  nix build .#hello-pid1-kit --out-link assets/build/hello-pid1/result
  python3 tools/pid1/watch.py --exec assets/build/hello-pid1/result/init

POST /exec stages a bounded static ARM32 ELF in RAM, acknowledges the upload,
and execs it as PID 1. It preserves the kernel, loaded modules and RAM files;
the new program replaces all current userspace threads. Upload the init ELF,
not boot.img or the initramfs archive. ELF structure/static-linking checks
reject obvious wrong inputs, but cannot prove a program will work. The new
program must retain its own diagnostics and return timer if those are wanted.
Our Hello binary does so and tolerates already-loaded modules. Assets, kernel,
device tree and initramfs changes are not supplied by this endpoint.

QEMU verified two successive uploads, continued PID 1, renewed Hello display
and HTTP, uptime reset without a kernel reboot, and invalid-upload rejection.
The endpoint is restricted to the direct host IP, with no shell or ADB daemon.

Rebuild the native program and repin the local artifact:
  ./pid1/build.sh

Uses the exact backed-up V109 kernel and framebuffer modules, plus the live
device tree captured from this G06. The private firmware artifact is imported
into the local Nix store; firmware binaries are not committed to Git.

Runtime order:
  RAM/pseudo mounts -> independent reboot deadline -> Ethernet -> HTTP debug
  thread -> stock display modules -> framebuffer Hello world.

hello.display_delay=0..10 optionally keeps HTTP diagnostics up before display
modules start. The one-shot launcher uses 5 seconds. A dedicated QEMU case
verified HTTP status/logs during that delay and Hello/reboot afterward:
  python3 tools/pid1/check-qemu.py --case display-delay

The program requires PID 1, never mounts eMMC, and has no Android, ADB, sound,
shell or game. Default runtime is 90 seconds, adjustable with hello.timeout
(15..300). Default interface is eth0, IP 192.168.77.2/24; only 192.168.77.1 is
accepted by the diagnostic server. No default gateway or DNS is configured.

During a RAM boot, with the Mac's direct Ethernet route verified:
  curl --max-time 3 http://192.168.77.2:8765/status
  curl --max-time 3 http://192.168.77.2:8765/logs
  curl --max-time 3 -X POST http://192.168.77.2:8765/reboot

QEMU verification:
  python3 tools/pid1/check-qemu.py

Cases cover Hello visible as PID 1 with HTTP status/logs, repeated program
replacement and remote reboot;
deliberately incompatible display module with HTTP still alive and automatic
reboot at 20 seconds; an absent framebuffer; and the bootloader restart-command branch.
The harness compiles current Hello separately from the pinned kit. It adds a
test-only ARM32 syscall-absence trampoline, static .77 subnet, stale RTC and
per-case fidelity manifests. See tools/qemu/README.txt. Captures are under
assets/build/hello-pid1/qemu/. QEMU uses its own ARM kernel and virtio devices;
this does not establish G06 cold display initialization, Ethernet, watchdog,
or kexec compatibility. The independent userspace timer cannot recover a
kernel hang or a failure before PID 1 starts.

Native launcher tests that use shared frames (check-repl-qemu.py --apps and
check-display-ownership.py) require the current versioned demo kernel. The
older minimal surrogate has CONFIG_FILE_LOCKING disabled: fcntl returns
EACCES even without another owner, so a normal frame acquisition times out.
Those harnesses reject mismatched media before booting. Rebuild current media
with nix build .#pid1.passthru.media --no-link --print-out-paths and pass that
output as --media. This check does not change TV firmware or lock deadlines.

Current hardware state and boot evidence are recorded at the top of this
file and in research/g06-ram-boot-status.md. Earlier factory-service access
restrictions were superseded by stock-plus-root and the one-shot launcher.
