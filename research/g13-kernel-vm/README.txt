G13 Linux 5.15.224 source port and VM build
2026-11-01

This experiment updates the existing Android/MT5867 5.15.220 source port
with the changes between upstream stable 5.15.220 and 5.15.224. The build
runs on the owner's x86_64 NixOS VM; runtime tests use the local partial
MT5867 QEMU machine and the existing small Android 16 native ramdisk.
All work is offline with respect to the TV.

Verified result
---------------

The source-built 5.15.224-mt5867-ack14 kernel boots Android 16's native
subset in the partial G13 QEMU machine. The original twelve checks pass
in qemu-20261101T223607/result.json (19.194 seconds host elapsed). The
extended test passes all eighteen checks in
kernel-test/qemu-20261101T223631/result.json (25.621 seconds), including
eight VMA-remapping cases, timer shutdown/rearm semantics, rebuilt-module
load/unload and rejection of V013/V033 stock devnode modules.
These paths are under assets/build/g13-kernel-vm/.

Image: 15,870,728 bytes, SHA256
  4cd1c33ee7da0a8a956e52a5a29c3f0e7aae2e9e429c0e899b8efab82ae6f0bd
Final source tree: 4cfdd98b42e57e961d76b0d2e486e8be5f996b68
The VM build produces 19 modules, 16 device trees and a separate timer-test
module. Six additional subsystem compile targets pass, with their enabled
configuration recorded in source-check.json. A fresh recipe replay matches
all 79,496 source entries (apart from the recorded build-release suffix);
see replay-verification.json. Forty-three conflicts and two prerequisite
files have explicit reviewed resolutions.

Stock drivers are a confirmed barrier to using this on the TV. Each saved
firmware has 33 ARM32 modules, and all 33 have mismatched symbol CRCs against
this kernel, including module_layout. The two devnode load attempts were
rejected before module initialization. Nine stock filenames also have
rebuilt counterparts, which does not establish equivalent hardware behavior.
Four stock modules additionally reference exports absent from the available
kernel/module set. Details are in v013-module-audit.json and
v033-module-audit.json. No versions were stripped or forced.

The subsequent offline investigation in ../g13-module-compat/README.txt
recovers the actual TCL headers from both firmware kheaders.ko files and
reproduces their module_layout CRC exactly. It identifies real task-layout
differences, including an incompatible reference-count access in the stock
Mali driver. Those findings do not change this tested kernel or establish
stock-driver compatibility.

Inputs
------

The preceding source port, configuration, Android donor and QEMU setup are
documented in ../g13-android16/README.txt and ../g13-kernel/README.txt.
No full AOSP checkout is used. Android binaries remain the pinned donor
subset, including its older security patch level, described in those notes.

Stable repository: https://github.com/gregkh/linux
Comparison base: Linux 5.15.220
  commit 0996e0926f6b4d6123e1b94407d726bc9810248e
  tree   83ec387bfa9c0fac6373f3942803bb203fc1e76f
Update: Linux 5.15.224
  commit 700e62ea909a70b9f72692d8b7a192ac7ebde0aa
  tree   a2a870b5aa81d8ab4c8aaaa203b53a980c39e6cc
  https://github.com/gregkh/linux/tree/700e62ea909a70b9f72692d8b7a192ac7ebde0aa

The live Git references were checked when choosing these pins. Cached web
pages disagreed about the latest versions; they were not used as the pins.
At that check Android common android14-5.15 still pointed to the preceding
6ce868e42ba10faa46e2e9f1faf94af0264eaf35 source snapshot.

prepare.py reconstructs both complete stable Git tree IDs, including modes
and literal symlink text. The preceding MT5867 port is also checked against
its recorded source tree ID. Its earlier local extraction has one equivalent
PowerPC selftest symlink spelling; both known tree IDs are accepted. This
does not affect ARM builds.

The merge compares same-version trees, not a demonstrated Git ancestor of
the Android/vendor branch. Of 1,862 stable-changed paths, 1,286 take upstream
content directly, 165 already have that content, 368 merge textually and
43 require explicit resolution. resolve.py records each resolution and its
output hash. Android interfaces differ from upstream even at the same
numeric version, so resolutions adapt fixes rather than import missing
interfaces. This is an experimental port, not certification that every
upstream stable or Android security fix is present.

Notable adaptations preserve MStar NAND error handling, Android TCPM timer
hooks, per-CPU EROFS workers, anonymous-VMA names, MGLRU and the existing
BPF array-owner representation. F2FS adopts the embedded GC-thread lifetime
while retaining Android's GC control interface. Absent cgroup.kill, SPI
match-data and newer fanotify interfaces are not introduced piecemeal.
The stable timer shutdown implementation is also backported as a required
prerequisite: it prevents concurrent rearming during Wi-Fi/HID teardown.
Android trace hooks, timer layout and old exported delete APIs are retained.
The Quantum650 USB-audio quirk is deferred because this Android tree lacks
the prerequisite fixed-rate mechanism; its other stable audio fixes remain.

Reproduce the build on the VM
----------------------------

Run these commands from a checkout containing this research directory.
Restore the saved configuration before building:

  assets/research/g13-kernel-source-20261101/live.config
  SHA256 90fcf9a9d0bc5d51da327392413b13fbd5ef3dcc4509346c67f2e00d361989aa

The working VM directory used here is /home/eduardo/dande-tv-kernel.
A Git archive is sufficient; shell.nix reads the pinned flake.lock without
requiring a .git directory. Enter the shell there:

  nix develop --impure --file research/g13-kernel-vm/shell.nix
  mkdir -p assets/build/g13-kernel-vm
  python3 research/g13-kernel-vm/prepare.py

prepare.py deliberately exits 1 after reporting the 43 conflicts. Inspect
assets/build/g13-kernel-vm/stable-merge.json, then run separately:

  python3 research/g13-kernel-vm/resolve.py
  python3 research/g13-kernel-vm/build.py -j6
  python3 research/g13-kernel-vm/check-sources.py
  python3 research/g13-kernel-vm/collect.py

Redirect stdout/stderr to prepare.log, build.log and source-check.log under
that assets directory when retaining complete transcripts. Do not chain the
expected-conflict preparation and resolution with an unconditional &&.
Preparation and resolution preserve existing work and refuse to overwrite
it. Build and checks can be repeated. No source tree is automatically deleted.

For a fresh VM, prepare.py downloads the three preceding source archives
and two pinned stable archives. To reuse local downloads, copy these files
to the same repository-relative paths first:

  assets/build/g13-android16/ack-base.tar.gz
  assets/build/g13-android16/ack-new.tar.gz
  assets/research/g13-kernel-source-20261101/oneplus-mt5867-android14-3543dbbd.tar.gz
  assets/build/g13-kernel-vm/stable-base.tar.gz
  assets/build/g13-kernel-vm/stable-new.tar.gz

All large generated files stay under assets/. Preparation requires 18 GiB
headroom above the owner's 15 GiB reserve; building requires another 8 GiB
headroom and the extra compile check 2 GiB. These are preflight checks, not
quotas against unrelated writers. No ADB, serial or TV operation occurs.

The kernel uses Clang/LLD 21.1.8, ARCH=arm, the captured configuration,
NCT5_BUILD=true and KERNEL_VER=linux-5.15_14. Build identity and timestamp
are fixed in build.py. The stext entry must be 0xa0008000. kernel-build.json
records source/config/artifact hashes, configuration changes, toolchain and
host identity. The public source still lacks some captured TCL features;
the complete configuration differences are retained rather than hidden.

check-sources.py uses a separate source-check-out directory and enables
additional features to compile NFSv4, fanotify, Libertas, zoned SCSI and
USB-C files affected by conflict resolutions. The cxgb4 driver is excluded
because the captured MT5867 platform has no PCI support. That configuration is
only for compilation checks and is not used for the QEMU kernel.

collect.py packages the Image, vmlinux, config, symbol versions, modules,
DTBs and available logs/reports in artifacts.tar.gz. Its SHA256 is recorded
in artifacts.tar.gz.sha256. Transfer both, verify the archive's SHA256,
then extract it at the local repository root. Archive members use paths
under assets/build/g13-kernel-vm/.

Reproduce the QEMU checks locally
--------------------------------

Keep the existing QEMU build and Android 16 native assets, or reconstruct
them using ../g13-android16/README.txt. The two required captured inputs
are the configuration above and:

  assets/research/g13-leanboot-os/live.dtb
  SHA256 96c47e15278b28a82b38799506af1517dda77bd34f353b94199d594dc95fdc1b

Then, in the same Nix shell:

  python3 research/g13-kernel-vm/run.py
  python3 research/g13-kernel-vm/audit-modules.py \
    assets/research/g13-firmware-trim-20261101/v013-extracted v013
  python3 research/g13-kernel-vm/audit-modules.py \
    assets/research/g13-v033-offline-20261101/extracted v033
  python3 research/g13-kernel-vm/check-kernel.py

run.py varies only the kernel and reuses the exact previously verified
8,335,081-byte ramdisk, SHA256
e84b6a12448f9fc88eac50d050fe81c5fd22039028dd828f3e44ebe1a0d0f6f5.
Its twelve checks cover Android init as PID 1, property actions, four CPUs,
Bionic threads/sockets, Binder transactions, memfd sharing and concurrent
Binder clients. The ramdisk contains 48 Android ELFs / 13,916,704 bytes.

The static module audit compares ARM32 __versions CRCs and symbol providers
with the built Module.symvers. It also lists matching rebuilt filenames;
neither filename nor CRC agreement is proof of runtime compatibility.
The six ARM64 copies in each firmware extraction are excluded explicitly.
With CONFIG_MODVERSIONS, the first vermagic field (kernel release text) is
not itself the loader's decisive comparison; module_layout and other CRCs
are relevant.

check-kernel.py adds a source-built Bionic probe and module fixtures to a
separate test ramdisk. It exercises eight adjacent-VMA MREMAP_DONTUNMAP
cases (both directions, faulted/unfaulted, whole/partial VMA), source-built
module load/unload, timer shutdown/rearm behavior, and rejection of the
saved V013/V033 devnode modules at module_layout. The timer probe is built
as a separate laboratory module. Tests never strip versions or force-load
a module.

Limits
------

The machine remains a partial MMIO model with six initcalls excluded.
Passing checks cannot establish panel, audio, GPU, Wi-Fi, storage, standby,
proprietary module or physical boot compatibility. The test SELinux policy
is an allow-all laboratory policy running in enforcing mode. The userspace
has no Java framework, ART, media services or vendor HAL stack. This is a
kernel/native-runtime experiment, not an installable Android 16 TV image.
The longer check also logs rejected vendor_init subprocess starts under
that minimal policy; it does not cover vendor services. The compiled kernel
and modules are ARM32. ARM64 source paths have not been build-tested here.
