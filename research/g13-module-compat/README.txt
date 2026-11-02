G13 stock-driver ABI investigation
2026-11-01

Result
------

The saved V013 and V033 firmware contain their actual kernel headers inside
kheaders.ko. These have now been extracted. Using either set of headers,
the unmodified kernel genksyms tool reproduces the stock module_layout CRC
a03a05bf exactly. This supplies a verified reference for the compatibility
port; it does not make the new kernel compatible by itself.

The source-built 5.15.224 baseline is unchanged. Its Android 16 native/QEMU
results remain documented in ../g13-kernel-vm/README.txt. This investigation
does not load any stock module, override any CRC, connect to the TV, alter
its boot configuration or flash anything. No additional QEMU run is claimed.

Recovered headers
-----------------

extract-kheaders.py reads kernel_headers_data and kernel_headers_data_end
from each saved ELF. It validates paths and link targets before writing
the embedded archive and extracted files under assets/.

V013: 5,978 regular files, two symlinks, 24,272,735 uncompressed file bytes.
  kheaders.ko SHA256:
  243c8fb86d6456c6a7892b3fe277b029d4fa66f7bc3af86ec28913f3573333a2
  embedded archive, 3,710,152 bytes, SHA256:
  12d7ed630c6fa951d25e972b3a550d78eb3996bdf23edfffa0c0402fe6446e51
V033: 5,979 regular files, two symlinks, 24,291,269 uncompressed file bytes.
  kheaders.ko SHA256:
  5b3cf069420e58bd9d0ebeb5fed674e9e8feb4d49e200abf409fc3b8d86beb9e
  embedded archive, 3,710,936 bytes, SHA256:
  aeef5fdc981e5a02a25abaf6a4261a957ccf7611ef3f297498177ffd9c3971aa

The archives contain arch/arm/include, include/linux, generated autoconf,
asm offsets and release headers. They do not contain a complete kernel
implementation or the out-of-tree MediaTek driver sources. In particular,
the ARM entry/switch/fault implementation needed for TCL's stack features
cannot be obtained merely by extracting these header archives.

reproduce-crc.py preprocesses the existing 5.15.224 kernel/module.c with
the stock headers and runs real genksyms. Its 597-file dependency manifest
records all input hashes. Five inputs come from the existing source tree:
kernel/module.c, kernel/module-internal.h, and the MT5867 mach/io.h,
mach/irqs.h and mach/memory.h headers omitted from kheaders. Both firmware
sets produce identical symbol-type files and the exact expected CRC.
The code does not replace or patch a generated checksum.

A separate compile-only ARM object, layout-probe.c, independently measures
the stock layouts with Clang 21.1.8. Its constants agree with the stock
driver DWARF, including the task reference-count offset. This needs neither
execution of the object nor a complete AOSP tree.

What actually differs
---------------------

The stock rtk_btusb.ko and 8733bu.ko retain DWARF. dwarf-layouts.py recovers
438 and 1,294 named aggregate layouts respectively. Across V013 and V033,
all 438 Bluetooth and all 1,294 Wi-Fi observed layouts match. That statement
does not extend to every kernel type or every semantic change in the OTA.

                         stock V013/V033     source-built 5.15.224
  sizeof(struct module)          576                      576
  module.init offset             208                      208
  module.exit offset             544                      544
  sizeof(struct task_struct)    2,432                    1,920
  task.usage offset              528                        8
  task.mm offset               1,220                      708
  task.comm offset             1,640                    1,128
  sizeof(struct mm_struct)       464                      480

The identical module container does not imply compatibility. The real
module_layout genksyms graph reaches task_struct through module ->
trace_event_call -> perf_event. Its stock graph has 565 reachable type
records; the candidate has 557. Nine common definitions differ:

  dev_pm_info, key, key_tag, keyring_index_key, mm_struct, signal_struct,
  task_struct, vm_area_struct and the mm_context_t typedef.

The stock graph additionally reaches thread_info, its FP/VFP state types
and vm_struct. The candidate instead reaches anon_vma_name. The complete
definitions and paths are in v013-symbols/result.json and module.symtypes.

The DWARF comparison against the candidate's kernel/module.o observes
282 common named layouts: 273 match, eight have different member layouts,
and vm_area_struct retains its storage shape with a changed anon_name type.
The broader Wi-Fi/cfg80211 comparison observes 779 common layouts: 765
match, thirteen differ, and vm_area_struct has the same type distinction.
Additional differences include cfg80211_update_owe_info, IPv6 configuration
and XFRM state. Equal storage is not proof of equal pointer semantics.

The original public OnePlus 5.15.148 build already has the 1,920-byte task
and the wrong task field offsets. The captured TCL configuration enables
THREAD_INFO_IN_TASK, VMAP_STACK and DYNAMIC_STACK; the public ARM port does
not implement them. The recovered current.h also shows TCL using the
TPIDRURO current-task path. These are implementation gaps predating the
5.15.224 update, not merely a changed release string.

Other differences come from later common/stable changes. Examples include
futex_unique_id in mm_struct, key indexing fields, async PM state, signal
flags and anonymous-VMA names. Preserving the fixes while preserving any
required binary layout needs deliberate implementation work. Replacing
headers wholesale, adding padding alone or forcing matching CRCs is not
a verified repair.

Direct machine-code evidence
----------------------------

In both saved Mali modules, kbase_context_common_init calls pid_task,
copies the returned pointer to r4, adds 0x210, then performs ldrex/strex
to increment the task reference count. 0x210 is stock task.usage. In the
candidate kernel task.usage is at 0x8. This is a concrete incompatible
access in a display-related driver, beyond the loader's CRC warning.

driver-stack.py verifies this instruction sequence against both binaries
and saves the relevant disassembly. It never executes the module.

Display/audio dependencies
--------------------------

Static ELF imports and modinfo dependency declarations give this 13-module
closure for mstar_fbdev_mi, mik and mali_kbase_mt5867:

  firmware_class, hdcp1x, hdcp2x, iniparser, kdrv_xc, mali_kbase_mt5867,
  mik, misck, mstar_fbdev_mi, mtk_lmkd, mwgifker, utpa2k, xcker.

Only firmware_class and mtk_lmkd currently have rebuilt same-name outputs
within that closure. This is a static dependency set, not a minimal boot
list, proof of equivalent rebuilt implementations or a requirement to
keep using every stock binary. The existing native TV installation also
uses Android/MediaTek userspace and configuration.

Seven LED exports required by the closure remain absent from the available
kernel/module providers: BreathLed_ini_data_get/set,
BreathLed_state_str_type_get/set, BreathLed_ui_state_get/set,
and led_ui_state_get. The wider DTV set additionally needs led_ui_state_set.
The stock Wi-Fi driver requires the old out-of-line netif_napi_add export;
the candidate has an inline wrapper around netif_napi_add_weight instead.
These are separate from the structure-layout barrier. No stubs were added.

Current boundary
----------------

There is now enough saved evidence to continue ABI work without TV access.
The recovered headers are a better reference than guessing the missing
TCL types. Remaining work includes implementing the missing ARM stack/
current-task behavior, resolving the actual layout and function-interface
differences, and supplying/rebuilding the vendor driver interfaces.

Hardware access becomes useful for a bounded RAM-only boot and peripheral
initialization test after an appropriate candidate exists. At that point
the saved recovery path, current identity and sole UART owner need a fresh
check. There is no new boot candidate or request to flash at this checkpoint.

Reproduction
------------

From the repository root:

  nix develop --impure --file research/g13-module-compat/shell.nix

In that shell, extract a saved firmware's headers to a NEW directory:

  python3 research/g13-module-compat/extract-kheaders.py \
    assets/research/g13-firmware-trim-20261101/v013-extracted/vendor_dlkm/lib/modules/kheaders.ko \
    assets/research/g13-module-compat/v013-kheaders

Use the corresponding V033 extracted path and v033-kheaders destination
for the other set. The extractor refuses to overwrite an existing result.

Reproduce the CRC and compile the layout probe:

  python3 research/g13-module-compat/reproduce-crc.py \
    assets/research/g13-module-compat/v013-kheaders/headers \
    assets/research/g13-module-compat/v013-symbols \
    --genksyms assets/build/g13-kernel/out/scripts/genksyms/genksyms \
    --candidate-symtypes assets/research/g13-module-compat/module-5.15.224.symtypes

The genksyms path above is the native executable retained from the original
source build on the local ARM64 host. On the x86_64 build VM, use its native
assets/build/g13-kernel-vm/kernel-out/scripts/genksyms/genksyms instead.
Do not copy a host executable across architectures and assume it runs.

The candidate symtypes input can be regenerated on the existing build VM:

  make -C assets/build/g13-kernel-vm/kernel-source \
    O=/home/eduardo/dande-tv-kernel/assets/build/g13-kernel-vm/kernel-out \
    ARCH=arm LLVM=1 HOSTCC=cc HOSTCXX=c++ NCT5_BUILD=true \
    KERNEL_VER=linux-5.15_14 kernel/module.symtypes

Copy kernel/module.symtypes and kernel/module.o from that output into
assets/research/g13-module-compat/ as module-5.15.224.symtypes and
module-5.15.224.o. DWARF extraction and comparison then use:

  python3 research/g13-module-compat/dwarf-layouts.py \
    assets/research/g13-firmware-trim-20261101/v013-extracted/vendor_dlkm/lib/modules/rtk_btusb.ko \
    assets/research/g13-module-compat/v013-rtk-layouts.json
  python3 research/g13-module-compat/dwarf-layouts.py \
    assets/research/g13-module-compat/module-5.15.224.o \
    assets/research/g13-module-compat/candidate-module-layouts.json
  python3 research/g13-module-compat/compare-layouts.py \
    assets/research/g13-module-compat/v013-rtk-layouts.json \
    assets/research/g13-module-compat/candidate-module-layouts.json \
    assets/research/g13-module-compat/module-5.15.224.symtypes \
    assets/research/g13-module-compat/v013-vs-candidate.json
  python3 research/g13-module-compat/driver-stack.py \
    assets/build/g13-kernel-vm/v013-module-audit.json \
    assets/research/g13-firmware-trim-20261101/v013-extracted \
    assets/research/g13-module-compat/v013-driver-stack.json \
    --stock-layouts assets/research/g13-module-compat/v013-rtk-layouts.json \
    --candidate-layouts assets/research/g13-module-compat/candidate-module-layouts.json

For the broader comparison, extract 8733bu.ko and the candidate's
net/wireless/cfg80211.ko instead. The comparison normalizes the two DWARF
bit-offset encodings used by the stock and new compilers. It does not
compare function signatures, enum values or types absent from the DWARF.

Evidence lives under assets/research/g13-module-compat/. Extraction and
CRC reports include hashes, commands and inputs. verification.json records
36 passing checks, including compiled constants against independent stock
DWARF, recovered file/link integrity and CRC agreement with every saved
ARM32 module in each firmware. The read-only VM step
only generated symtypes and retrieved module.o; it did not rebuild or
replace the tested kernel. The retained evidence uses about 155 MiB, preserving the
owner's 15 GiB free-space reserve. Firmware and recovered headers remain
untracked; the tools, compile-only probe and this note are tracked.
