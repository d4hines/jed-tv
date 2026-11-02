G06 one-shot cold-boot launcher (Android and native PID 1 verified)

NATIVE PID-1 HARDWARE RESULTS: 2026-11-01T01:16Z and 01:20Z.
The one-shot request selected Hello as PID 1, with working Ethernet HTTP
status/logs. Display initialization failed at mik.ko; no Hello was shown.
The first test reset after about a minute with bootreason=watchdog and
returned to working Android/root. launcherctl --inspect then confirmed
the request absent, demonstrating the consumed-request fallback.

The second test uploaded a diagnostics-only static executable over HTTP.
It replaced PID 1 without a kernel reboot, served the kernel log, and
accepted a normal reboot request. The log identifies missing files:
  /vendor/tvconfig/config/mik_init.ini
  /vendor/tvconfig/config/sys.ini
MI_DEV_SYS_Init consequently failed, aborting mik module initialization.
Further MI configuration/display initialization remains necessary.
After the owner pressed Power once from black/standby, Android returned
with bootreason=reboot. tools/root/verify.py passed again (boot complete, root
Magisk domain, global Enforcing). Return verification:
  assets/research/stock-plus-root/live/20261101T012214.810452Z/
Evidence:
  assets/research/hello-pid1/20261101T011619.426813Z/
  assets/research/hello-pid1/20261101T012047-diagnosticsZ/

DEFAULT HARDWARE BOOT VERIFIED: 2026-11-01T01:15Z. After the owner's flash
and one remote Power press from standby, Android booted normally. Live boot
partition SHA256548bdde9954c4b97de301a956e24f0976d2a82c2f69659c3d656bb64d5e0ce6b
and ro.boot.tv_launcher both matched the prepared image. tools/root/verify.py
passed, and the matching native launcherctl --inspect reported no pending
experiment. The helper is installed at /data/local/tmp/tv-launcherctl.
This initial check preceded the experiments above. Evidence:
  assets/research/launcher-live/20261101T011508.242200Z/default-check.json
  assets/research/stock-plus-root/live/20261101T011508.173546Z/

The launcher keeps the original G06 V109 kernel and packed DTB. A native
ARM32 /init selects either the existing stock-plus-root Android startup or
the Hello program as real PID 1. It never uses kexec or writes raw boot,
recovery, misc/BCB, MBOOT, or vbmeta partitions.

Normal behavior is Android. A matching root-only request on the existing
cache filesystem selects Hello once. Before Hello starts, the launcher
deletes that request, fsyncs its parent directory, syncfs()es the filesystem,
cleanly unmounts, mounts it read-only again and verifies the request absent.
Only then does it remove its temporary mounts and exec Hello as PID 1.
Hello receives a 5-second display delay (HTTP first), a 90-second return
timer, and a normal reboot target. The next boot has no matching request and
selects Android. A power cycle after a stuck experiment also starts Android
provided the flag was durably consumed and the launcher/default path works.

Default Android path restores Magiskinit itself to /init before exec. This
is necessary because upstream Magisk first_stage() renames /init to /sdcard;
simply running renamed Magiskinit while /init is still our wrapper loops.
The exact original stock init is also included as last-resort fallback if
Magiskinit cannot be restored/executed. All files from the original stock
ramdisk except /init remain identical. Magisk's cpio restore list covers all
launcher/Hello additions, and the original stock-init backup is preserved.

Partition guard:
  original G06 serial 4C0490290FA05A0C0 from /proc/cmdline, with no duplicates
  cache = mmcblk0p29, start sector 9019392, size 204800 sectors (100 MiB)
  device node major/minor must match its sysfs identity
  ext4 magic and filesystem geometry must match the 100 MiB partition

The request is /cache/tv-launcher/request. Its directory must be root:root
0700; the file must be a root:root 0600 regular file, one link, <=256 bytes,
not a symlink. Exact schema/device/firmware/serial/experiment/build-ID content
is required. No cache file is created automatically by the boot image.
Unrelated cache files are preserved. A dirty ext4 journal can be replayed
even on the initial read-only mount, so journal recovery may write ordinary
cache metadata. The cache is never formatted or replaced.

Build:
  ./launcher/build.sh
  nix build .#stock-plus-loader

build.sh compiles the source-pinned production ARM32 launcher and imports
assets/build/launcher/g06-launcher-runtime.tar into the local Nix store.
The Nix package recipe is tools/launcher/package.nix. Parent-flake integration:
  stock-plus-loader = import ./launcher/package.nix {
    inherit pkgs originalBoot;
    stockPlusRoot = self.packages.${system}.stock-plus-root;
    helloKit = self.packages.${system}.hello-pid1-kit;
  };
Add g06-launcher-runtime.tar to the existing requireFile allowlist.

The image builder is tools/make-stock-plus-loader.py. Output includes
boot.img, manifest.json, launcherctl, request.txt and validation files.
The prepared image fits the original 31 MiB partition; stock kernel/DTB
identities and unsigned AVB hash metadata are checked by the builder.
Only the boot command line adds androidboot.tv_launcher=<build ID>.

Use the Nix output as the canonical image. Parent integration built it at
assets/build/launcher/result, boot SHA256:
  548bdde9954c4b97de301a956e24f0976d2a82c2f69659c3d656bb64d5e0ce6b
Payload before padding/AVB is 30,486,528 bytes. A direct macOS builder run
produced different gzip compression bytes; its uncompressed patched cpio
was verified byte-identical to the Nix output. Do not mix boot hashes from
those two compressed images. The manifest beside the actual image is the
authoritative identity for that output.

First hardware milestone: install and prove DEFAULT ANDROID before arming
anything. Keep the working recovery USB and .#stock-boot available. Before
installing, inspect /cache/tv-launcher/request through existing root; if a
matching request from an earlier installation is pending, explicitly cancel
it first. Reflashing an identical launcher build does not invalidate its old
pending request. No installation or arming was performed during preparation.
Read-only TV preflight at2026-11-01T00:08Z found neither /cache/tv-launcher
nor its request file; Android and root were ready. Evidence:
  assets/research/launcher-preflight/20261101T000847.261431Z/
Recheck at actual installation time because this is a captured observation.

After default Android is verified, install the native flag-control tool:
  adb -s 4C0490290FA05A0C0 push result/launcherctl /data/local/tmp/tv-launcherctl
  adb -s 4C0490290FA05A0C0 shell chmod 0700 /data/local/tmp/tv-launcherctl
adb -s 4C0490290FA05A0C0 shell '/debug_ramdisk/su -c "/data/local/tmp/tv-launcherctl --inspect"'

launcherctl requires root, exact G06 serial and a running launcher boot
marker matching its own build. It verifies the physical cache identity and
that Android's /cache mount uses that device. It does not reboot itself.
Explicitly arm a single Hello attempt:
adb -s 4C0490290FA05A0C0 shell '/debug_ramdisk/su -c "/data/local/tmp/tv-launcherctl --arm"'
  python3 tools/pid1/watch.py

Then, in another terminal, with Ethernet connected and the watcher running:
  adb -s 4C0490290FA05A0C0 reboot

Cancel a matching request before reboot instead:
adb -s 4C0490290FA05A0C0 shell '/debug_ramdisk/su -c "/data/local/tmp/tv-launcherctl --cancel"'

--arm refuses to overwrite any existing file. --cancel removes only a valid
request for the current build. A wrong-build/partial/invalid file remains
untouched and always selects Android; inspect that exact file through root
and remove it explicitly if confirmed stale before rearming. Do not remove
unrelated files or the recovery command. If an arm/consume operation reports
an I/O error, inspect/cancel before rebooting or retrying: storage that rejects
writes cannot provide a guarantee that a pending request was cancelled.
In particular, a read-only cache can leave a valid request pending. The
launcher refuses the experiment until every durable-consume check succeeds.

Expected experiment behavior:
- During Hello there is no Android or ADB; diagnostics use .2:8765 over the
  directly connected Ethernet link. tools/pid1/watch.py captures status and logs.
- The userspace timer only protects failures after Hello starts. It cannot
  recover an early kernel hang; reset/power cycle is then needed. A consumed
  flag means that the next cold boot selects the default Android branch.
- Any replacement uploaded through Hello's /exec replaces all its threads;
  replacement programs must supply their own debug/return behavior.

Local validation:
  cd launcher
  TV_LAUNCHER_ID=test-build cargo build --locked --offline --release --features qemu-test --target armv7-unknown-linux-musleabihf
  python3 check-qemu.py

The QEMU profile is compiled separately and cannot be packaged for the TV.
It checks the same ARM32 logic using an ext4 virtual disk and a local ARM64
compat kernel with builtin ext4/virtio-blk. The exact production ARM32
default path is separately checked on the native ARM32 tiny QEMU kernel.
QEMU fallback programs stand in for Magisk/Android; this does NOT prove actual
G06 Android startup, cache-driver timing, display, watchdog or power behavior.

21 cases passed, including default Android PID-1 chain-exec and restoration
of /init; malformed, wrong-build, wrong-mode, wrong-serial and wrong-geometry
refusal; durable consume followed by a next-boot Android default; read-only
storage refusal; actual busy-unmount error after deletion followed by default
Android on both boots; missing Hello exec fallback; original stock-init
fallback; native control arm/inspect/cancel and duplicate-arm refusal; and
upstream ARM magiskboot restoring the built candidate cpio exactly to the
original stock metadata/content map. An unrelated sentinel file survived.
Evidence: assets/build/launcher/qemu/results.json and individual logs.

Known limits: filesystem syscalls can stall on hardware I/O failure; a
userspace device-enumeration timeout cannot bound a stuck kernel. If both
validated fallback init binaries cannot execute, the launcher logs a fatal
error and parks rather than exiting PID 1; USB recovery would be required.
The first physical flash/default-boot gate has passed for the exact image
hash above. The cache-write/consume path and Hello PID1 still require a TV test.
