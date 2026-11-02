G06 V109 stock plus root development image (hardware root verified)

Root-shell iteration with Android retained
-----------------------------------------
The installed stock-plus-loader image also takes this Android/root path by
default. No experiment should be armed for this workflow. From the repo:

  ./shell.sh
  ./shell.sh id
  ./shell.sh --check

shell.sh waits for the original G06/V109 to finish Android/root startup,
verifies uid 0, then opens a terminal in /data/local/tmp. Commands with
arguments are supported; for shell syntax use ./shell.sh sh -c '...'. It
uses the existing authorized USB ADB connection and Magisk su. It does not
flash, reboot, stop services, change policy, or start another root server.
Exiting the shell leaves Android and ADB running.

Android's compositor is SurfaceFlinger, backed by the vendor Hardware
Composer. Its WindowManager/framework normally organizes application
windows. The existing Hello helper draws a SurfaceControl layer directly
through those running services; no standalone framebuffer setup is needed:

  ./hello.sh --seconds 60

To prepare the same helper for manual use from the root shell:

  ./hello.sh --inspect
  ./shell.sh

Then type on the TV shell:

  CLASSPATH=/data/local/tmp/tv-hello/hello.jar app_process32 /system/bin --nice-name=tv.hello HelloScreen 60

The automatic hello.sh runner uses the ADB shell UID. The direct root-shell
form uses uid 0. Both retain the stock display initialization.

Live verification, 2026-11-01T03:21Z: ./shell.sh opened the interactive
G06:/data/local/tmp # prompt. Typed id returned uid=0(root), Magisk domain;
getprop sys.boot_completed returned 1. A root-launched HelloScreen then
created the overlay. A second root command succeeded while it was visible,
with SurfaceFlinger and Android still running. The captured screen shows
centered white Hello world on dark blue. Evidence:
  assets/research/android-root-shell/20261101T032149Z/
After the 60-second overlay ended, root and boot_completed=1 remained
available, the helper process was gone, the boot ID was unchanged, and no
one-shot launcher request was pending. See completion.json in that folder.

The owner flashed this image, and post-flash verification passed at
2026-10-31T23:28:36Z. A later agent-run verification also passed at 23:29:57Z:
Android boot complete, ADB su uid0 in magisk domain, global Enforcing,
readable /proc/iomem, kexec enabled. No kernel load or handoff has been tested.
Evidence: assets/research/stock-plus-root/live/20261031T232957.074635Z/.

The automatically provisioned Magisk manager stub displayed an upgrade
prompt that reopened after Cancel. Force-stopping only its UI resolved it:
  adb -s 4C0490290FA05A0C0 shell am force-stop com.topjohnwu.magisk

The TV launcher became foreground and root verification passed again without
reopening the prompt. No full-manager download/install was needed; the root
daemon was not stopped and the manager package was not disabled/uninstalled.

Later root probes reopened the prompt. Exact upstream su code showed why:
root-access logging attempts a manager provider call and falls back to an
activity launch. The live UID-2000 policy was changed to logging=0 and
notification=0 while retaining policy=2 (allow). The recipe now provisions
those quiet values too. The installed first image still provisions logging=1
at boot, so its popup can return after restarting until a future reflash.
No reflash is required just to use root or dismiss the manager UI.

Build:
  nix build .#stock-plus-root

Produces result/boot.img and result/manifest.json. It starts with the exact
verified original V109 boot image and pins the official Magisk v28.1 APK by
SHA256. Only the original ramdisk init is replaced; its exact original is
backed up inside the new ramdisk. Other original ramdisk files, fstab/AVB
keys, kernel, packed DTB and boot command line remain unchanged. The build
verifies a no-change stock repack, final payload identities and unsigned AVB
hash metadata. The image is the original 31 MiB partition size.

This is a Magisk-based development image with verified G06 Android startup
and ADB root. Keep the recovery USB ready for future boot experiments.
No TV flash, reboot, live policy change or app installation was performed
during its preparation; the owner subsequently flashed it. No change to the installed bootloader, recovery, vbmeta,
vendor or system partition is required by this recipe.

On a successful boot, Magisk continues Android startup and provides its root
daemon. A custom nonblocking init service waits for sys.boot_completed=1,
sets Magisk's root-access setting to ADB only, and grants UID 2000 (shell)
su access without a manager approval dialog. It writes a RAM log at
/dev/tv-adb-root.log and sets sys.tvroot.ready=1 after the root/memory-map
checks succeed. The stock ADB daemon remains UID 2000; use su for root.
There is no new unauthenticated root TCP listener, and this recipe does not
enable network ADB or modify the existing authorized keys.

Android remains globally SELinux Enforcing. Upstream Magisk adds broad root
rules and a permissive magisk domain, plus other standard Magisk policy
changes. This is broader than a custom rule permitting only kexec. Zygisk
and custom modules are not enabled by this recipe. Magisk may provision its
manager stub as part of its standard startup; no manager UI is needed for
the ADB grant. Magisk creates persistent runtime/database state in /data/adb.
Existing Magisk state/modules, if any, are not erased by this image.

Post-flash checks (passed on this image):
  python3 tools/root/verify.py

That checks exact G06/V109 identity, Android boot completion, unattended
ADB su, uid0 in the magisk domain, global Enforcing, /proc/iomem readability,
the installed kexec version and its enabled/loaded state. It saves evidence
under assets/research/stock-plus-root/live/. It never loads or executes a
kernel, unloads an image, reboots, or changes policy.

For a manual check after Android finishes booting:
  adb -s 4C0490290FA05A0C0 shell getprop sys.tvroot.ready
  adb -s 4C0490290FA05A0C0 shell /debug_ramdisk/su -c id

Magisk uses /sbin instead if that directory exists; verify.py checks both.
Use the absolute su path rather than relying on Android shell PATH.

Kernel load and unload are now proven on hardware; actual RAM handoff has
failed twice and returned automatically to the installed Android image with
reported kernel_panic. Root plus readable memory-map access does not prove
that the TV's raw ARM kernel, DTB, reserved memory, watchdog and initialized
drivers survive a kexec boot. Do not automatically execute a test at startup.
The desired iteration loop is ADB upload -> explicit RAM boot -> reboot or
power cycle back to this installed Android image.

RAM loader tooling (hardware load/unload passed, execution still failing):
  nix build .#hello-pid1-kit --out-link assets/build/hello-pid1/result
  python3 tools/root/kexec.py
  python3 tools/root/kexec.py --payload android --single-core

Those commands only load and unload. Adding --execute explicitly starts a
handoff and stops Android framework services. Inputs are uploaded directly
to tmpfs, hashed, and staged within verified first-bank RAM. The execution
helper syncs and forces disk filesystems read-only before requesting kexec.
Both Hello/SMP and Android/single-core attempts returned automatically to
installed Android with kernel_panic; their failure location is unresolved.
See research/g06-ram-boot-status.md for exact evidence and the prepared,
not-yet-executed reset-only discriminator.

Eventual installation, when ready to test (not performed by the build):
  adb -s 4C0490290FA05A0C0 reboot bootloader
  ./flash.sh .#stock-plus-root

The working recovery USB and exact stock boot remain the fallback. Restore:
  ./flash.sh .#stock-boot

Restoring stock boot disables this Magisk startup but does not remove its
already-created /data/adb runtime/database state or any provisioned manager
app. No cleanup, reset, relock or additional unlock is part of this recipe.

Validation completed:
- Nix build succeeded; exact stock kernel/DTB/cmdline/fstab/keys preserved.
- Upstream ARM magiskboot recognized the modified ramdisk and restored all
  original ramdisk files, permissions/owners/device metadata and contents.
- Upstream ARM magiskpolicy applied its rules to a copied V109 precompiled
  policy file and saved it. --live was never used; installed policy untouched.
- ADB-root service shell syntax passed; its SQL was exercised against the
  upstream schema and granted only UID 2000 with ADB-only access selected.
- Android remained boot_completed=1 and SELinux Enforcing after validation;
  temporary on-TV validation files were removed.

File-only validation can be reproduced with:
  python3 tools/root/check-image.py result

It uses ordinary USB ADB and copied files in a unique /data/local/tmp folder,
then removes that folder. It does not run the root-setup script. Captures:
  assets/research/stock-plus-root/validation/20261031T232310.941072Z/

Pinned upstream references:
  https://github.com/topjohnwu/Magisk/releases/tag/v28.1
  https://github.com/topjohnwu/Magisk/blob/v28.1/scripts/boot_patch.sh
  https://github.com/topjohnwu/Magisk/blob/v28.1/native/src/boot/cpio.rs
  https://github.com/topjohnwu/Magisk/blob/v28.1/native/src/init/rootdir.cpp
  https://github.com/topjohnwu/Magisk/blob/v28.1/native/src/sepolicy/rules.rs
