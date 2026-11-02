G13 firmware reduction: measured candidates and dependencies
2026-11-01

Conclusion
----------
For our native launcher, bundled APKs and the Java framework are the largest
removal candidates. In the captured V013 base they occupy 1,426,492,795 bytes
(1.33 GiB), about 61% of the regular-file payload inventoried here. Removing
them would also remove the current full-Android fallback. This is a candidate
budget, NOT a measured reduction in compressed firmware or a tested image.

The existing native startup demonstrates operation without app_process or
system_server. It does not demonstrate that deleting their files, APK assets,
APEX modules, or framework directories is safe. Keep the native Android core,
graphics/audio stack and MediaTek middleware for the first reduced image.

This research used preserved firmware and prior device captures only. No TV
connection, partition write, service change, reboot or new boot test occurred.
No reduced firmware image has been built. The separate Android 16 / Linux
5.15.220 QEMU experiment is still a core-runtime test, not a complete TV OS.

Measured scope
--------------
V013 is the captured G13_2K_US / T221T10 / Android 14 / AU13 stock base for the
previously verified native installation. V033 is the downloaded, verified
G13 / Android 14 / AU21 update; this work has not installed it. See
../G13_V033_OFFLINE.txt for package signature, payload and filesystem checks.

Eight EROFS filesystems were inventoried in each version: system, product,
system_ext, vendor, system_dlkm, vendor_dlkm, linux_rootfs_a and tvservice.
This excludes userdata, other physical partitions and runtime APEX mounts.
Nested files inside APEX archives are counted through their containing APEX
file, not separately. APK counts below exclude APKs inside APEX archives.

Category                         V013 MiB        V033 MiB
APK app/priv-app/overlay trees     1238.32         1361.48
Framework directories              122.09          122.13
APEX containers                    225.72          225.76
Native libraries outside APEX      251.25          251.69
Kernel modules                      88.80           88.81
Other files                        315.54          315.68
Total                             2241.71         2365.54
Standalone APK files                  120             123
Regular files                       4232            4247

Exact totals:
  V013: 2,350,606,537 bytes; APK trees 1,298,475,841; framework 128,016,954.
  V033: 2,480,452,507 bytes; APK trees 1,427,613,654; framework 128,066,849.
  APK trees plus framework: V013 60.69%, V033 62.72% of inventoried bytes.

Method: sum regular-file st_size, without following symlinks, compression or
hardlink deduplication. App-tree sizes include their adjacent compiled code
and libraries; APK internal ZIP compression remains. These are extracted
file payload sizes, not download sizes, RAM use or flash-space savings.

First candidates: bundled applications
-------------------------------------
For an image exclusively running our native launcher, omit stock launchers,
APK streaming clients, Google services/store/Assistant, WebView/browser,
retail demos, screensavers, Android setup, keyboard and accessibility apps.
That deliberately gives up those applications and their features. A native
replacement is needed for any settings, accessibility, pairing or account
workflow we want to retain. Existing native YouTube does not require the
stock YouTube APK, but its own networking/proxy requirements still apply.

Largest V013 candidate directories, including their associated files:
  106.81 MiB  /product/priv-app/AndroidMediaShell
   91.70 MiB  /product/priv-app/PrebuiltGmsCorePano
   88.98 MiB  /product/app/TrichromeLibrary
   88.52 MiB  /product/app/tclnetflix
   70.18 MiB  /system_ext/priv-app/TvSettingsPlus
   64.41 MiB  /product/app/YouTubeLeanback
   56.29 MiB  /product/priv-app/TVDreamXPrebuilt
   55.52 MiB  /product/app/AmazonVideo
   54.96 MiB  /product/priv-app/GoogleTTS
   50.46 MiB  /product/priv-app/Tubesky
   50.34 MiB  /system_ext/priv-app/RetailDisplay
   36.00 MiB  /product/app/TTVSCore
   35.85 MiB  /product/priv-app/Katniss
   34.88 MiB  /product/priv-app/TVLauncherXPrebuilt
   31.74 MiB  /system/app/UserAgreement

These are exact directory labels, not a claim that every package's internal
purpose or native asset consumers have been audited. The machine-readable
inventory contains every candidate tree and file. Whole-directory deletion
requires checking any native references to assets inside it.

Most product/system_ext bytes are apps, but do not delete either partition:
  V013 product:    941.62 MiB app trees, 5.17 MiB other files.
  V013 system_ext: 235.60 MiB app trees, 7.03 MiB other files.
The remainder includes policy, configuration and native components. Partition
names alone do not establish that their contents belong to the Java stack.

Second candidates: Java runtime and unused APEX services
------------------------------------------------------
After removing APK consumers and making native init self-contained, the
framework JAR/compiled-code trees, app_process32, ART services and Java/JNI
libraries are candidates. Remove their init triggers, classpath generation,
dex optimization and service references together, not only executables.

There is a concrete G13 directory-existence trap. Both V013 and V033 use
/system/framework to decide which vendor startup path to take:
  /vendor/etc/rc.local:146,208
  linux_rootfs_a/basic/autorun.sh:118,147,160
The Android branch configures glibc library paths used by TV middleware;
the alternative changes console/Bluetooth startup. Keep the empty directory
when initially removing its contents, until those branches are deliberately
replaced and tested. An empty directory alone does not fix classpath/init
dependencies elsewhere.

Each version has 32 APEX containers, about 226 MiB total. V013 examples:
  com.android.art                  24.76 MiB
  com.android.media.swcodec        20.42 MiB
  com.android.adservices           19.07 MiB
  com.android.btservices           18.72 MiB
  com.android.healthfitness         9.55 MiB
  com.android.mediaprovider         8.53 MiB
ART, adservices, healthfitness, appsearch, ondevicepersonalization and other
Java-only feature consumers are reasonable next audit targets. Media/codecs,
Bluetooth, Wi-Fi, DRM and IPsec depend on the features we retain. Do not infer
that a container is removable from its name or absence of exported libraries.

APEX native-library audit: both versions parsed without ELF failures. It
compares provided native libraries in each APEX manifest with DT_NEEDED in
ARM32 executables/libraries outside APEX. It does not inspect every APEX's
internal dependencies, linker namespaces, dlopen calls, configuration paths
or runtime reachability. In particular runtime/VNDK show no ordinary exports
in this scan but remain fundamental to the current native stack.

Verified traps in both firmware versions:
  * netd imports libnetd_updatable.so, supplied by com.android.tethering.
    Keep that dependency even when we do not provide a hotspot. netd also
    imports libnetd_resolv.so from com.android.resolv.
  * libmedia.so, libsqlite.so and libxml2.so import libandroidicu.so from
    com.android.i18n (33.36 MiB in V013). audioserver loads libmedia.so.
    Native audio therefore still has an ICU dependency without Java.
  * Stopping statsd is not sufficient to delete com.android.os.statsd:
    libstatspull/libstatssocket have native importers, including input/media
    libraries; update_engine directly imports libstatssocket.so.
  * ART's libnativehelper/libnativeloader/libsigchain have native importers,
    largely JNI/Java-associated files. Prune those dependants or rebuild
    their consumers before dropping the provider.

Third candidates: optional native services
------------------------------------------
The native boot script already requests stopping artd, installd, media,
mediaextractor, media.swcodec, mediametrics, the media C2 HAL, statsd, storaged
and mdnsd. This supports testing their omission from native startup, not
deleting every library associated with them. Reconsider media/C2 removal if
we add vendor hardware decoding or Android media APIs later.

A preserved native process capture still contains mdnsd, traced/probes,
incidentd, idmap2d, wificond, update_engine, OEM LogkitInputEvent, vendor
Netflix and Googlecast services. In particular the mdnsd stop request is not
evidence it remains stopped. Inspect property/lazy-service restart triggers.

Suggested conditional targets:
  * OEM logkit, incident reporting and tracing: remove only after keeping
    sufficient serial/logcat/crash diagnostics for development.
  * Netflix/Cast/DRM services: investigate if APK streaming and casting are
    deliberately absent. Audit their native clients and shared middleware.
  * Wi-Fi/Bluetooth: feature-dependent; Ethernet alone does not prove a
    Bluetooth remote, pairing or another required function is independent.
  * HDMI/CEC, tuner/live TV: keep initially. CEC can affect power/remote
    behavior, and TV middleware is shared with panel/audio initialization.
  * OTA services and /vendor/bin/upgrade.img (50 MiB): only after designing
    a replacement update path. AB_OTA_POSTINSTALL_fusion copies this image
    to the shared upgrade partition and invokes trigOAD_fusion. The V033
    OTA makes this postinstall stage mandatory; it is not just dead data.

Native components to retain initially
-------------------------------------
  * Kernel, matching modules/firmware, init/ueventd, Binder service managers,
    Bionic/linkers, runtime/VNDK APEX, linkerconfig/apexd and SELinux policy.
    Keep secilc and matching policy inputs while our debug boot compiles
    policy. Do not drop older-looking HAL support libraries still named in
    DT_NEEDED by SurfaceFlinger or another retained binary.
  * SurfaceFlinger, graphics allocator/mapper/gralloc, MediaTek composer,
    Mali userspace/kernel pieces, display middleware and native-display.
  * audioserver, audio policy/configuration, mstar audio HAL, cpu_audio,
    associated native libraries, native-audio and our standalone app-state
    shim. audioserver still loads package-manager-named native libraries
    even though the Java package manager is absent.
  * netd/resolver/tethering dependencies and authenticated adbd. Our current
    network-g13.sh explicitly waits for netd and its priority-32000 rule;
    removing netd requires a replacement network startup implementation.
  * Storage/encryption services such as vold, TEE/keymint/keystore, required
    power/thermal/health services, device configuration and calibration.
    The current userdata fstab requests file-based encryption and uses
    /metadata/vold/metadata_encryption. Changing that is a separate storage
    migration, not a consequence of removing APKs.

Do not remove linux_rootfs_a/tvservice because they look like a second OS:
the vendor chain waits for mknod_util_end, launches MtkRmInitServer, creates
mtk_rm_end, waits for /dev/fusion0, then runs rc.local/autorun and app_man /
dtv_svc. cpu_audio.sh explicitly uses the glibc loader/libraries in these
filesystems. The composer links libmi3, libfbdev, libvsyncbridge and MediaTek
API/cmpb libraries. Eliminating broadcast apps does not eliminate that
hardware initialization chain. Keep /metadata/dande-tv and boot/recovery
assets; none is an APK-cleanup target.

What prior runtime evidence actually establishes
-----------------------------------------------
../G13_NATIVE_BOOT_NOTES.txt records visible native UI and audible audio
without system_server/app_process, 14 audio-protocol passes, and successful
automatic native startup after a normal reboot. Its current shim, library
loading and media.metrics.enabled=0 setting address reproduced dependencies.
The init service name 'zygote' is reused for a shell script, so its property
being 'running' does not by itself establish a JVM process.

Read-only captures reviewed from /home/eduardo/tv_hack/assets/research/:
  g13-native-pid1-20261101/flash-test/native-attempt-3-result.json
  g13-startup-parallel-20261101/processes.txt
  g13-startup-parallel-20261101/final-state.txt
  g13-ethernet-20261101/final-state.txt
These are historical observations, not a fresh TV-state verification.
Native startup still imports stock native init configuration and stops some
services afterwards. Full mains-off and standby/wake coverage is not implied.

Expected benefit and next implementation step
----------------------------------------------
The main first-stage benefit is a smaller firmware payload and fewer unused
components to maintain. APKs already excluded from native startup consume
no process RAM merely by existing on disk. Removing active optional daemons
may save RAM/CPU, but there is no measured savings figure yet. Removing app
files alone does not establish faster panel/audio startup.

EROFS is read-only and compressed. Real storage measurements require a
repacked image with the intended compression and filesystem metadata. A
smaller logical image inside super does not automatically enlarge userdata;
partition sizes, A/B/update layout and AVB metadata must be handled explicitly.
Do not subtract the payload sizes above from the physical eMMC capacity.

Recommended sequence, entirely offline until hardware access is authorized:
  1. Generate an APK-first candidate image from the pinned base, with a
     retained-file manifest and reproducible filesystem metadata. Preserve
     vendor configuration, APEX and native libraries initially. This report
     is not a deletion allowlist and does not supply a flashable artifact.
  2. Replace the current full-Android fallback with an explicit native rescue
     path before removing framework/ART. Current one-shot/failure fallback
     cannot work once its Android installation has been removed.
  3. Audit init triggers, classpaths, ELF dependencies, dynamic loads and
     filesystem references, then reduce framework/APEX one group at a time.
     Reuse QEMU for native-core and failure-path tests; the current partial
     machine does not validate real G13 panel, audio, CEC or vendor HALs.
  4. Later validate display, sound, input, networking, recovery and power
     transitions on the exact TV before calling a reduced image compatible.

For the Android 16 work, ../g13-android16/README.txt already demonstrates a
48-ELF / 13,916,704-byte native core with init, properties and Binder in QEMU.
It excludes the graphics/audio/vendor stack. That size is not a realistic
complete-TV estimate. Mixing Android 16 userspace with these Android 14 vendor
libraries still requires the ABI/linker/policy compatibility work recorded
there. A full AOSP download is not needed to continue this inventory.

Reproduce this research
-----------------------
Run at the repository root. The private stock backup must be present under
assets/backups/g13-v013-stock-20261031/. V033 uses the previously verified
extraction under assets/research/g13-v033-offline-20261101/extracted/.

Build the legacy EROFS reader:
  nix build --impure --file research/g13-v033/erofs-legacy.nix --no-link --print-out-paths
Select the printed output containing bin/fsck.erofs (not the manual output).
The expression uses a Git-source flake reference so evaluating it does not
copy the ignored firmware/build tree into the Nix store.

Enter the existing research shell:
  nix develop --impure --file research/g13-android16/shell.nix
Then run:
  python3 research/g13-firmware-trim/extract-v013.py /path/to/bin/fsck.erofs
  python3 research/g13-firmware-trim/inventory.py assets/research/g13-firmware-trim-20261101/v013-extracted v013
  python3 research/g13-firmware-trim/inventory.py assets/research/g13-v033-offline-20261101/extracted v033
  python3 research/g13-firmware-trim/apex-audit.py assets/research/g13-firmware-trim-20261101/v013-extracted v013
  python3 research/g13-firmware-trim/apex-audit.py assets/research/g13-v033-offline-20261101/extracted v033

extract-v013.py verifies pinned SHA256 hashes for super.img, linux_rootfs_a_a
and tvservice_a, and validates the LP geometry/header/table checksums. It
supports only this snapshot's slot-zero linear extents on one backing device.
Each temporary logical image is removed after extraction. Existing extraction
directories are preserved, so skip that step on a completed rerun. A failed
partial extraction requires inspection, not blind reuse as complete evidence.
No mounting or root permission is needed. The preflight requires 8 GiB above
the owner's 15 GiB free-space reserve; this is not a quota against unrelated
writers. There were about 160 GiB free after this work.

Generated output: assets/research/g13-firmware-trim-20261101/
  v013-extraction.json, v013-*-extract.log, v013-extracted/
  v013-inventory.json, v033-inventory.json: all files, groups and totals
  v013-apex-audit.json, v033-apex-audit.json: manifest exports and importers
  native-elf-references.json: selected V033 ELF dependency inspection
  saved-native-processes.json: parsed historical native process capture
  apex_manifest.proto: AOSP Android 14 schema used for the manifest fields
The last three files are retained inspection evidence, not regenerated by
the commands above. apex-audit.py uses the shared read-only ELF parser from
tools/g06_firmware.py; no G06 images, settings or device workflow are used.

Verified stock input SHA256 values are pinned in extract-v013.py. Resulting
inventory SHA256 values for this checkout/path:
  V013 3a3a3860e6081ed978a6737a6aab1ca9204884dbb9ba0de9c7c33f730a7b1ede
  V033 d97acbf70b36fe2d65e4ab21d57b844de3fab8b9f81dbef8bbca4f9a32dcb6e2
The JSON includes an absolute source path, so its hash can change if moved.
Both inventory runs and both APEX audits completed successfully. Cross-checks
matched every group, category and partition sum to the per-file records;
both versions contain the netd and libmedia APEX dependencies listed above.
All three scripts passed Python syntax parsing. These are host-side research
checks; no new QEMU or hardware compatibility result is claimed here.

Primary references
------------------
Partition roles, including native/configuration content outside system:
  https://source.android.com/docs/core/architecture/partitions
APEX early activation, native executables/libraries and HALs:
  https://source.android.com/docs/core/ota/apex
APEX manifest field definitions, pinned Android 14:
  https://android.googlesource.com/platform/system/apex/+/refs/tags/android-14.0.0_r1/proto/apex_manifest.proto
Tethering's native library exports (corroborates this image's manifest/ELF):
  https://android.googlesource.com/platform/packages/modules/Connectivity/+/02eee9aa5abc17d9a0a80509effc9654317d5f4a/Tethering/apex/Android.bp
Native graphics and audio architecture:
  https://source.android.com/docs/core/graphics/surfaceflinger-windowmanager
  https://source.android.com/docs/core/audio
Dynamic partition layout and resizing constraints:
  https://source.android.com/docs/core/ota/dynamic_partitions/implement
