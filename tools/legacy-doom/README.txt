Legacy Doom image research
==========================
The Java-backed standalone runners have been retired. Launch Doom through
  python3 tools/native-app/run.py
which uses the current Rust display/audio backends and App API v1.
The legacy engine/image builders below remain for historical reproduction;
PID1 now uses the shared src/platform crate; this legacy workspace remains
for historical image reproduction.

Historical direct-PID1 image experiment (do not use for this workflow)
------------------------------------------------------------------

Silent Doom as PID 1 on the original G06

Build the prepared image from the repository root:

  nix build .#doom-boot

Output: result/boot.img, result/initramfs.cpio.gz and result/manifest.json.
Building never contacts or flashes the TV. The original kernel, DTB, boot
addresses and header version are preserved. Only the boot ramdisk, command
line and unsigned embedded AVB metadata are replaced. Other TV partitions
are not written. The separate stock vbmeta remains unchanged.

The sole userspace executable is the static ARM32 Doom binary at /init.
It mounts devtmpfs/proc/sysfs, unpacks its read-only assets into RAM, loads
the exact stock framebuffer-module dependency chain, then runs Doom while
remaining PID 1. There is no Android init, launcher, shell, ADB daemon,
network setup or audio backend. Sound calls are no-ops; no sound assets are
included. Freedoom 0.13.0 supplies E1M1 and its graphics. Other levels and
attract demos are removed. All runtime files and save/config writes are in
the RAM filesystem; no eMMC filesystem is mounted by this program.

The stock IR configuration is included at the kernel's expected vendor path.
Display uses /dev/graphics/fb0 if present, otherwise /dev/fb0; currently only
32-bit framebuffer formats are supported. A framebuffer/setup failure keeps
PID 1 alive and logs to the console rather than exiting/panicking.

Important hardware boundary

QEMU validation uses the actual packaged ramdisk with the ARM32 QEMU kernel
and doom.modules=0 (MStar modules cannot run on QEMU). This validates asset
unpacking, game startup, evdev, drawing and quitting. It does not validate
the stock kernel on the real TV, module initialization, panel visibility,
the remote map or the watchdog. Vendor userspace may still be needed to
initialize the panel. Treat this as a prepared hardware experiment.

This image intentionally has no ADB. Before trying it on hardware, establish
a way back into bootloader Fastboot without Android, using the documented
recovery route. Read RECOVERY_NOTES.txt. Restoring original boot is:

  ./flash.sh .#stock-boot

That command requires the TV to already be reachable over USB Fastboot.
Successful flashing alone does not establish that the image boots.

Rebuild the local runtime from source

Sources here are a separate snapshot of the other agent's Doom/platform
implementation, with sound and unused platform modules removed. Changes in
the agent1 worktree do not silently alter this image. The original C engine
is doomgeneric commit e257a09527062bfb986b6e5e0780839b6cbd5b71, under
assets/third_party/doomgeneric. Rust 1.92.0 is pinned; the local build uses
zig cc for ARM32 musl and rust-lld for the final static link.

  nix develop
  bash tools/legacy-doom/build.sh
  nix build .#doom-boot

build.sh builds the silent engine and Rust binary, verifies the stock super
and tvconfig backups, extracts the display modules and IR configuration,
makes the reduced Freedoom WAD, and imports a frozen runtime tar into the
local Nix store. tools/legacy-doom/runtime.json pins that tar's hash. Nix image builds
therefore consume exact frozen inputs; they do not rebuild Rust/C by default.
The output manifest records the source-file and asset hashes.

The existing local assets are prerequisites. Source firmware and game data
are not committed to Git. Preserve assets/ when moving this workspace.

ARM32 QEMU check (requires qemu-system-arm and assets/build/kernel/arm/zImage):

  python3 tools/legacy-doom/check-qemu.py ./result

Logs and screenshots: assets/build/doom-minimal/qemu/.
The QEMU command has no network or audio device. Quitting Doom must leave
PID 1 alive. This check does not flash the TV.

Validation on 2026-10-31: the ARM32 QEMU check passed with the packaged
ramdisk. Gameplay, movement/fire input, and the post-quit "PID 1 IS STILL
RUNNING" screen were visually inspected. Representative screenshot:
assets/build/doom-minimal/qemu/after-input.png. The stock kernel and DTB
were independently re-extracted from the new boot image and compared to
the originals; the embedded AVB hash verified.

First hardware attempt, 2026-10-31: the owner flashed ./flash.sh .#doom-boot;
all identity/size checks passed, and sending/writing/reboot returned OKAY.
Boot image SHA256:
0b49b8166b0ecc53a6cd05d0729b6fc76cd1286d97181a5d7524e070a2dda6e6.
The owner saw the unlocked warning for about a minute, followed by a black
screen. Pressing Power returned to the unlocked warning. No Fastboot USB,
USB ADB, or Ethernet link was visible from the Mac. Missing ADB/network is
expected for this image and does not locate the failure. No serial console
log or visible Doom output was captured. Module loading, framebuffer access,
panel initialization, and even reaching PID 1 remain unverified on hardware.
Recovery to the original boot image is being arranged through a spare USB
stick; do not treat this image as a successful hardware boot.

Subsequent recovery succeeded on 2026-10-31. The original PNY recovery stick
had --show in its labels; the proven --fast trigger was missing. The guarded
macOS repair restored all three labels, with sector backup/readback. The
owner then reached Fastbootd, selected Reboot to bootloader, reconnected USB,
and flashed .#stock-boot. Live USB ADB confirmed G06/V109, boot_completed=1,
no codex_poc marker, and SELinux Enforcing. There are no retained crash logs:
the stock kernel has CONFIG_PSTORE disabled and exposes no last_kmsg.
The precise Doom boot failure remains unknown.

Before another hardware boot experiment, establish reachable diagnostics
before module/display initialization, bound failure waits, and test the
return-to-bootloader mechanism. The current halt() deliberately waits forever
and its console output was inaccessible; that is unsuitable for another
unobserved TV test. Bring up framebuffer access under stock startup first.
