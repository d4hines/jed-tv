Android 16 native subset + MT5867 Linux 5.15.220
2026-11-01

The subsequent VM-built 5.15.224 experiment is documented in
../g13-kernel-vm/README.txt. It retains this exact Android 16 test ramdisk
and adds kernel regression and stock-module ABI checks. Results below
remain the 5.15.220 baseline.

Status
------

The source-built ARM32 MT5867 kernel boots the Android 16 native subset in
the existing partial G13/G06 QEMU machine. Android init is PID 1, its real
property service handles a property-triggered action, and servicemanager
publishes a native Binder service. The test passes 512 transactions from
four concurrent client threads and a 4 KiB memfd transfer through Binder;
the server modifies the shared mapping and the client verifies every byte.
It also checks four online CPUs, Bionic threads/socketpair, and a 64 KiB
tmpfs write. This is an offline QEMU result. No TV was accessed or changed.

The kernel is compiled from source. Android native binaries are extracted
from a pinned TrebleDroid Android 16 donor, not rebuilt from AOSP. Our Binder
probe and the host SELinux policy compiler are built from source. The donor
release is a prerelease, with a June 2025 security patch level; this experiment
does not establish a current Android security patch level.

The native subset contains 48 Android ELF files, about 13.9 MB uncompressed.
Its complete compressed test initramfs is about 8.3 MB, including the Debian
BusyBox/glibc bootstrap. These are core-runtime sizes, not a complete TV OS.
No ART, Java framework, zygote, system_server, launcher, media services,
graphics stack or vendor HALs are included yet. A full AOSP checkout is not
needed for this milestone. It would be needed, or a separately validated
source subset/build setup, to rebuild the Android binaries themselves.

Final verified run: assets/build/g13-android16/qemu-20261101T215916/.
All 12 required markers pass in result.json (18.979 seconds host elapsed).
The exact Image and ramdisk, command, serial output and final CPU registers
are retained there. The final build output matches that run's kernel hash:
  Image SHA256 74aba6d8c709df822f71209f2991616c22a583da2235d11c6ecf939a8b7289a2
  ramdisk SHA256 e84b6a12448f9fc88eac50d050fe81c5fd22039028dd828f3e44ebe1a0d0f6f5
  Android ELF bytes: 13,916,704; compressed full ramdisk: 8,335,081 bytes.
The independent merge replay under assets/build/g13-android16/reproduce/
matches all 79,492 source files, including modes and symlink targets.
See its verification.json, plus kernel-final.log and kernel-build.json.
Kernel build timestamps are not normalized; future rebuild hashes may differ.

Reproduce
---------

All commands run at the repository root on the ARM64 Nix development host.
Keep the two already captured G13 inputs at these locations:

  assets/research/g13-kernel-source-20261101/live.config
    SHA256 90fcf9a9d0bc5d51da327392413b13fbd5ef3dcc4509346c67f2e00d361989aa
  assets/research/g13-leanboot-os/live.dtb
    SHA256 96c47e15278b28a82b38799506af1517dda77bd34f353b94199d594dc95fdc1b

They are not fetched from a connected TV. All new generated files go under
assets/build/g13-android16/. The shared QEMU build and pinned Debian input
remain at assets/build/qemu-g06/ and assets/build/leanboot/qemu-input/.

Enter the shell, then run each step:

  nix develop --impure --file research/g13-android16/shell.nix
  python3 -B -c 'import sys; sys.path.insert(0, "tools/bootloader"); from qemu import inputs; inputs()'
  python3 research/qemu-g06/build.py
  python3 research/g13-android16/prepare.py
  python3 research/g13-android16/merge-kernel.py
  python3 research/g13-android16/resolve-kernel.py
  python3 research/g13-android16/build-kernel.py -j4
  python3 research/g13-android16/minimal.py
  python3 research/g13-android16/run.py

merge-kernel.py deliberately exits 1 after recording its 25 expected
conflicted files. Inspect merge-report.json, then run resolve-kernel.py.
Do not put those two steps in an unqualified && chain. Both preserve an
existing merge/resolution and refuse to overwrite it on a repeated run.
prepare.py, build-kernel.py, minimal.py and run.py may be repeated.
For an already prepared checkout, only the latter two are needed to rerun
the native test. run.py --baseline selects the earlier source-built 5.15.148
kernel, if present, to distinguish userspace problems from the kernel merge.

The larger new preparation/build steps require 8-12 GiB of working headroom
above a 15 GiB free-space reserve. The small ramdisk/run steps check smaller
budgets above the same reserve. These are conservative preflight checks,
not filesystem quotas; unrelated writers can still consume host space.
The shared, older QEMU/BusyBox preparation commands have their own behavior.
No source checkout, build or download is automatically deleted.

To regenerate the static dependency survey for the next native services:

  python3 research/g13-android16/inventory.py

The survey finds 84 files / 16,659,472 bytes for SurfaceFlinger, 149 files /
20,432,100 bytes for audioserver plus unresolved libandroidicu.so, and 22
files / 4,933,328 bytes for hwservicemanager. The sets overlap: do not add
their sizes together. These are DT_NEEDED lower bounds, excluding dynamic
loads, data, HALs and linker-namespace restrictions, not runnable TV images.

Inputs and provenance
---------------------

MT5867 vendor source, Linux 5.15.148:
  https://github.com/techyminati/android_kernel_oneplus_mt5867
  commit 3543dbbd43a7e69112b1169b34171d7bd745cdee
  archive SHA256 048a1cb80336c823b3a705dbd10a413091d635f30e5c83645f4b91f562e318a3

Android common comparison base, Linux 5.15.148:
  https://android.googlesource.com/kernel/common/+/34148b6a5103338e78d6b545099d5200b75c7e58/
  tree 2f06cbf74d9475396c270d623a6844f25793fc88
  tag android14-5.15-2024-04_r1

Android common update, Linux 5.15.220:
  https://android.googlesource.com/kernel/common/+/6ce868e42ba10faa46e2e9f1faf94af0264eaf35/
  tree ae700f4a97747191ab0c1063ad9d37508ddca5ac
  branch android14-5.15 at the research snapshot

The comparison base is the same kernel version, not a demonstrated Git
ancestor of the published vendor tree. 73,579 of its 74,004 files match the
vendor snapshot exactly. The merge takes 8,129 unchanged-vendor paths from
the newer common tree, merges 155 files textually, recognizes 17 already
present changes, and resolves 23 textual plus two structural conflicts.
It retains the MT5867 source and captured configuration rather than
substituting a generic GKI kernel. The result is labeled
5.15.220-mt5867-ack6ce868e42ba1.

Gitiles archive timestamps vary with each request. merge-kernel.py therefore
reconstructs the actual Git tree IDs, including file modes and literal
symlink contents, to verify both Android common archives. The vendor
archive has a fixed SHA256. Publisher /home/ubuntu/ack symlinks are relocated
to relative links as in the earlier 5.15.148 reproduction.

Android donor:
  https://github.com/TrebleDroid/treble_experimentations/releases/tag/ci-20250617
  system-td-arm32_binder64-vanilla.img.xz
  SHA256 3f4fc02daf6f7d0d1333dab3fb2e70f3b30c268dfc419eb42c1c2b3f6d9cc666
  Manifest default: android-16.0.0_r2, despite the release title mentioning r1
  Fingerprint: google/treble_a64_bvS/tdgsi_a64_ab:16/BP2A.250605.031.A3/250617:userdebug/test-keys

prepare.py also verifies the release's pinned project manifest, developer
patch archive, expanded system image and runtime APEX. Its inputs.json
records URLs and hashes. debugfs extracts into ordinary host directories;
there are no privileged host mounts. Only the native dependency closure
is copied into the final initramfs. Native-manifest hashes identify each
selected ELF file. Android version properties are checked against the donor
before constructing the small test property set.

Kernel merge details
--------------------

resolve-kernel.py expresses each resolution, retains pre-resolution files
in unresolved/, and records output hashes/reasons in resolution-report.json.
The build checks those hashes. The same three earlier source-build patches
fix sync_debug guards, Kconfig/fixdep dependency paths and portable shell
arithmetic. See ../g13-kernel/README.txt for those reproduced failures.

Newer common implementations supersede earlier non-MStar backports in the
conflicted networking, F2FS, CoreSight, RCU and page-size migration files.
MStar-specific pagemap restrictions, Utopia exit callback, semaphore/CMA
hooks and DMA-buffer declarations are retained. USB resolutions retain
MStar reset/disconnect handling and polling while applying the newer
descriptor/maxpacket checks, control-pipe API, timeout cap and killable
wait semantics. The optional MStar LZO checksum wrapper is adapted to the
new compression API; CONFIG_MP_ZSM is disabled and that optional path is
not validated. ARM64 ABI lists are reference metadata, not a proven ABI.

The build uses Clang/LLD 21.1.8, ARCH=arm, NCT5_BUILD=true and
KERNEL_VER=linux-5.15_14. It produces Image, vmlinux, device trees and 19
modules. The stext entry is checked against the 2.5 GiB split (0xa0008000).
kernel-build.json includes every configuration change and artifact hash.
Several captured TCL configuration features are absent from the public
source, including its dynamic/VMAP-stack backports and MTK module-loading
extensions. This remains a port, not an exact TCL source reconstruction.
Existing TCL binary modules are not established as compatible.

Native boot details and limits
------------------------------

The small BusyBox first stage mounts proc/sysfs/tmpfs and BinderFS, creates
device nodes, loads a QEMU-only SELinux policy, then execs Android 16 init
second_stage as PID 1. This is not a full Android first-stage/AVB/APEX boot.
Android init starts servicemanager and the test using init.rc. Tests verify
/proc/1/exe, real property round-trips and a property-triggered init action.

The policy is single-domain and allow-all, with enforcing enabled to exercise
the interfaces expected by servicemanager. It provides no production
isolation. Bionic is flattened from its runtime APEX into /system; the test
uses one shared linker namespace. Neither policy nor layout is a proposed
production TV configuration. The native Binder probe uses the stable C
Binder ABI; it does not depend on private C++ object layouts.

The model runs four Cortex-A55 cores in AArch32. It uses the captured G13
memory layout/FDT and the same six QEMU-only initcall exclusions documented
in ../g13-kernel/README.txt. It has no real display, GPU, audio DMA, eMMC,
Wi-Fi, OP-TEE or vendor middleware. MMC timeout and OP-TEE/IOMMU errors are
expected here; registration messages do not prove working peripherals.
The inherited six modpost section-lifetime warnings also remain unreviewed
for hardware use. Nothing here establishes physical TV boot or recovery.

Both G06 and G13 have ARM64-capable Cortex-A55 CPUs, but the captured stock
kernels/userspace and this experiment are ARM32. The kernel source includes
an ARM64 MT5867 path; it has not been shown to boot either physical board.
Retaining ARM32 avoids assuming an unproved bootloader/driver conversion.

Next useful boundary
--------------------

Add Android 16 native media/graphics services and test their software-facing
interfaces separately before integrating vendor services. The saved V033
vendor manifest targets FCM 8, audio HIDL 7.1/effects 7.0, composer3 AIDL v2,
mapper HIDL 4.0 and Codec2 HIDL 1.1. The donor includes VNDK 34 compatibility
components, but that alone is not a VINTF or binary-ABI pass. Its MediaTek
and MStar services, vendor linker namespaces, SELinux rules and exact
module ABI must be reconciled before any physical TV experiment. A complete
TV userspace will therefore be larger than this 13.9 MB core subset.
