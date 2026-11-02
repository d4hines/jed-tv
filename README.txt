Current workspace layout
========================
G13 development standard: Ethernet ADB at 192.168.77.3:5555 plus UART.
See research/G13_DEVELOPMENT.txt for setup, boot persistence, commands,
local changes and compatibility with the original G06 at 192.168.77.2.

G13 research status and available capabilities: research/G13_RESEARCH_HANDOFF.txt.
Tools, saved results and provenance: research/G13_RESEARCH_ARTIFACTS.txt.

src/ contains authored code and payloads that execute on the TV or in a guest:
  tvinit, ytplay, doomtv, slides, platform, ffh264: current apps and interfaces
  pid1, launcher: native boot supervisor and one-shot boot selector
  hello, audio, lean, shared: Android bridges, audio work, Lean policy, shared code
  legacy-doom, probes: retained older runtimes and probes
  lean/zr, lean/yt: standalone Lean runtime and experimental YouTube app
  runtime-scripts/: shell/config payloads delivered to the TV
  bootloader/: Lean -> C RAM loader prototype; tiny audited ARM/FFI boundary
test/fixtures and test/unit: host and QEMU checks, outside the TV source tree

tools/ contains host-side build, upload and orchestration commands. Domain tools
are under tools/{pid1,launcher,native-app,audio,root,legacy-doom,experiments}.
test/ holds host test harnesses; research/ holds investigations and evidence notes;
assets/ remains ignored storage for firmware, captures and build outputs.
Source-package Lean builds place generated C, objects and binaries under
assets/build/lake/, never in src/.

The root entry points remain ./flash.sh, ./repl.sh, ./hello.sh and ./shell.sh.
Example host commands now use python3 tools/pid1/build-repl.py and
./tools/native-app/build.sh. Nix attribute names are unchanged.
This reorganization changes source paths only; it does not flash or restart the TV.

Graphical demo (no TV or private assets required)
-----------------------------------------------
  nix run .#pid1

Arrows/Enter select; Home returns to the menu. Includes offline generated
video/audio, Slides and Doom. This is the graphical virtio surrogate, not
the stock-kernel hardware model. Its menu and child-supervisor policy are now Lean-generated;
run nix run .#lean-check for executable ABI/state tests. See demo/README.md.

The default apps retain Rust YouTube and Rust/C Doom. Build the experimental
standalone Lean YouTube separately with nix build .#lean-youtube; it does not
replace the default app bundle. Its Zr runtime currently targets ARM32 and is
single-threaded, so it does not replace the multithreaded PID1 runtime.

Lean bootloader virtual machine (no TV required)
-----------------------------------------------
  nix develop .#bootloader -c python3 tools/bootloader/qemu.py

Runs our Lean -> C -> ARM loader, Linux and a BusyBox shell on QEMU virt.
Use --check for automated boot/rejection checks. Type poweroff -f to exit.
See tools/bootloader/README.txt for coverage and the separate G13 hardware status.

TCL TV bootloader, recovery and firmware-readback research

Read AGENTS.md and RECOVERY_NOTES.txt before changing a TV's boot state or
writing partitions. The original G06/T221T04 V109 TV and newer G13/T221T10
V013 TV use different firmware and partition layouts.

Repository layout

  src/            Target/runtime code, shared app interfaces, retained probes
                  (see src/README.md).
  tools/          Host build, upload and analysis commands, grouped by domain.
  test/           Host QEMU harnesses and gates (test/check.py, test/tv.sh).
  demo/           Self-contained Nix demo packaging and QEMU runner.
  research/       Investigations, archived experiments and specialist host probes.
  assets/         Firmware, dependencies, generated files and evidence; ignored.

Assets retain their original hierarchy beneath assets/. For example:
  assets/backups/v109-stock-20261031/
  assets/backups/g13-v013-stock-20261031/
  assets/backups/usb-fastboot-trigger-20261031T054144Z/
  assets/research/firmware/
  assets/research/tools/
  assets/research/manuals/
  assets/research/device/
  assets/research/g13-readback/build/

Firmware images, downloaded tools/manuals, extracted vendor code, device logs,
generated reports, helper build products and signing keys belong in assets/.
The tracked scripts and instructions use the new locations. Archived reports
and backup manifests inside assets/ are preserved verbatim; their historical
paths may refer to the original layout. Prefix those old workspace-relative
asset paths with assets/. A local relocation inventory and original copies
of the edited source/notes are in assets/layout-migration/.

Git contains neither the firmware backups nor the tools needed to analyze
them. Preserve assets/ separately when copying or archiving this workspace.
Keep original firmware images read-only and patch separate copies.

Reproduction guides

  RECOVERY_NOTES.txt               Original G06 startup failure and recovery.
  research/ROOT_READBACK_NOTES.txt  G06 stock-service root/readback route.
  research/G13_READBACK_NOTES.txt   G13 stock factory HAL readback route.
  research/G13_ADB_ROOT_NOTES.txt   Verified G13/V013 adb root and stock rollback.
  research/G13_MAGISK_NOTES.txt     G13 resistor/UART recovery investigation.
  research/G13_LEAN_BOOTLOADER.txt  Lean/ZrAsm RAM entry, tests and recovery limits.

The G13 helper builds on the laptop with:
  bash research/g13-readback/build-helper.sh
Its output is assets/research/g13-readback/build/readback.apk. The optional
TV_READBACK_BUILD_DIR environment variable selects an isolated build directory.
Building does not install the app or communicate with the TV.

Original G06 boot restore

Manual build/flash workflow (original boot partition only):

After building, ./flash.sh ./result/boot.img consolidates the identity/size checks,
boot flash and reboot below. The TV must already be in USB bootloader
Fastboot mode. It accepts compatible modified boot images too; it does not
enforce the stock image hash. nix build .#stock-boot produces the original.
Alternatively, ./flash.sh .#stock-boot builds that Nix target without creating
a result symlink, then flashes its boot.img with the same checks. Flake
references and .drv paths must produce exactly one output containing boot.img.

Minimal modified-image experiment:

  ./flash.sh .#poc-boot

This target appends androidboot.codex_poc=1 to the boot-header command line
and updates only the unsigned embedded boot hash. It preserves the kernel,
ramdisk, DTB, image size and all other bytes. The build verifies the embedded
hash with avbtool; change.json in the output records the exact change count.
The separate signed vbmeta still expects the stock boot hash. Acceptance
of a modified boot on this unlocked TV is not established by offline checks;
no vbmeta patch or verification-disable flag is part of this experiment.
After Android starts, check:

  adb -s 4C0490290FA05A0C0 shell getprop ro.boot.codex_poc

Expected value: 1, if the bootloader passes the header command line through.
A blank value does not alone establish a failed flash; the bootloader may
ignore that field. The property is absent on the confirmed stock startup.
The owner subsequently flashed this marker image successfully. After pressing
Power from apparent standby, Android booted and live USB ADB confirmed both
sys.boot_completed=1 and ro.boot.codex_poc=1, with SELinux Enforcing. Evidence:
assets/research/boot-poc/live-verification/android-state.txt. This establishes
a modified-boot-image round trip; an independent post-flash partition hash
was not captured. To restore the original once USB bootloader Fastboot is available:

  ./flash.sh .#stock-boot

  nix-store --add-fixed sha256 assets/backups/v109-stock-20261031/boot.img
  nix build .#stock-boot
  fastboot devices -l
  fastboot -s usb:CURRENT-PATH getvar product
  fastboot -s usb:CURRENT-PATH getvar version-bootloader
  fastboot -s usb:CURRENT-PATH getvar unlocked
  fastboot -s usb:CURRENT-PATH getvar partition-size:boot
  fastboot -s usb:CURRENT-PATH flash boot ./result/boot.img
  fastboot -s usb:CURRENT-PATH reboot

The first line is a one-time import of boot.img into the local Nix store.
The build accepts only the exact SHA256 pinned below; result/boot.img is
byte-identical to the backup, with no repacking or patching. nix build does
not access the TV. Only this OS boot image is imported, not calibration or
certificate backups. result is a generated symlink, excluded from Git.
If fastboot is not on PATH, run nix develop to enter a shell providing it.
Use the current USB path from fastboot devices -l. Before flashing, require
C06_eu, 01.01.250118, unlocked:yes and boot size 0x01f00000 (31 MiB).
Stop if identity/size differs; the manual commands do not enforce these gates.

Run these from this repository's root, with assets/ present:

  nix run .#flash -- --check       Verify all 33 original V109 backup images.
  nix run .#flash -- --preflight   Check original boot and USB TV identity only.
  nix run .#flash                  Restore original boot, reboot, check Android.

The default writes ONLY boot.img (31 MiB) from the verified G06 V109 backup.
It pins that image's SHA256 and requires serial 4C0490290FA05A0C0, G06/AR33,
bootloader 01.01.250118, and an unlocked device. A working USB ADB connection
is required before reboot; Ethernet ADB alone does not prove bootloader USB
connectivity. Fastboot must then report C06_eu, the expected bootloader and
boot size, is-userspace:no and unlocked:yes. A mismatch stops the operation.

For an existing bootloader session, first read fastboot devices -l, then:

  nix run .#flash -- --fastboot usb:CURRENT-PATH --preflight
  nix run .#flash -- --fastboot usb:CURRENT-PATH

That recovery entry cannot check Android's serial/fingerprint; the owner must
select the original G06's USB path. It still checks product, bootloader, boot
size and unlocked state. Never select the G13 or assume yesterday's USB path.

The command does not restore the whole device, reset user data, unlock,
relock or alter verification settings. The backup's mutable partitions were
captured after reset/recovery; they are not an untouched factory clone.
The automated restore reads assets/ directly; the manual build imports only
boot.img into the local Nix store. The Doom target additionally imports its
frozen runtime. Firmware stays outside Git. Commands and boot-health
results are saved under assets/research/flash/. A successful Fastboot write
and Android boot do not replace an independent partition readback hash check.

As of 2026-10-31 20:33Z: all 33 local images (4,741,660,672 bytes) passed
fresh size/hash checks. The live G06 boot range also matched the original
SHA256 d116dfa370d8e5dd5809f6f65266c86671b5e9bec9d62f95c0977bca0ff30085.
See assets/research/flash/preflash-20261031/live-boot-hash.txt. The Nix command
built and its local-image check passed; live preflight stopped because USB
ADB was absent at that time. Later, the owner completed the original-boot
round trip and the modified marker boot described above. The subsequent
Doom image displayed black on hardware; recovery is pending (tools/legacy-doom/README.txt).

Flash safety checks can be tested without TV access:

  python3 -m unittest discover -s tools -p test_flash.py -v

Silent Doom PID-1 image preparation

  nix build .#doom-boot

See tools/legacy-doom/README.txt for contents, source rebuilds, ARM32 QEMU validation and
the hardware/recovery limits. This image has no Android, ADB or audio backend.

Hello-world display and debug loop on stock Android

  ./hello.sh --seconds 30
  ./hello.sh --stop

See tools/hello/README.txt. This draws a temporary shell-owned layer while keeping
stock startup and ADB. It requires no firmware flash or APK installation.
The runner captures logs, a screenshot and an ADB check while the layer is
visible under assets/research/hello/. The display helper has a timeout and
an independent process deadline.
