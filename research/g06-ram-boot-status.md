# G06 RAM boot: measured status

## Cold-boot alternative verified: 2026-11-01T01:20Z

The installed stock-plus-loader image boots Android by default and selects
native Hello once when explicitly armed. Hardware demonstrated real PID 1,
Ethernet HTTP, durable flag consumption and return to Android after the
hardware watchdog reset. A subsequent test uploaded a diagnostics-only
executable over HTTP and replaced PID 1 without rebooting the kernel.
Kernel logs identify absent mik_init.ini and sys.ini as the immediate
mik.ko initialization failure. No framebuffer Hello was displayed.
See launcher/README.txt for evidence paths. This working cold-boot and native
upload path does not resolve the separate kexec panic described below.

## Live update: 2026-10-31 23:45Z

The owner installed .#stock-plus-root and root/verify.py passed on hardware.
Root memory-map access and execution restrictions are resolved in the Magisk
domain. Android is booted and accessible over USB ADB after both tests below.

Successful load/unload:
- Exact stock raw ARM Image, Hello initramfs, fresh live FDT.
- Stock loader at /vendor/bin/kexec, type zImage (its actual probe accepts
  raw images), image-size 32 MiB, destinations constrained to real RAM
  0x20200000..0x23ffffff. Entry 0x20208000 matches the stock boot header.
- Kernel at 0x20208000, initrd at 0x22208000, DTB after the initrd. Loaded
  flag changed 0 -> 1 -> 0; Android stayed boot_completed=1. Host/TV hashes
  matched. No firmware partition was written.
- TCL's loader appends old /chosen/bootargs to the supplied command line.
  A later load test blanks only that property in our RAM FDT copy to avoid
  duplicate/overriding arguments. Its load/unload also passed.
- Evidence: assets/research/kexec/load-check/20261031T233347.206988Z/ and
  assets/research/kexec/load-check/20261031T233605.957346Z/.

Actual RAM handoffs (both failed):
1. Hello PID-1 payload, normal SMP. Android writers stopped, disk filesystems
   synced and forced read-only, then kexec -e requested. No Hello HTTP was
   observed. The TV automatically reset through the unlocked/TCL/Google TV
   screens and returned to normal Android. Boot ID changed; reason kernel_panic.
   Evidence: assets/research/kexec/ram-boot/20261031T233701.910148Z/.
2. Known stock-plus-root Android payload, old CPUs explicitly reduced to CPU0
   and new command line maxcpus=1. A unique androidboot.tv_ram_trial marker
   was placed in the new command line. Again storage was read-only before
   execution. Android returned via the installed image, marker absent,
   boot ID changed, reason kernel_panic. This rules out a Hello-specific
   cause, but does not identify where the panic happened.
   Evidence: assets/research/kexec/ram-boot/20261031T234516.216659Z/.

No boot-image flashing or recovery USB was needed for these tests. Our
scripts do not write BCB directly. Later exact-kernel disassembly found a
stock reboot notifier, recovery_sys_notify_reboot, that can write boot-poweron
to misc when the restart command is NULL/empty and check_boot_mode is enabled.
kexec calls kernel_restart_prepare(NULL). Thus these tests cannot be described
as guaranteed BCB-unchanged: stock shutdown code may perform its normal
boot-control write. No pre/post BCB readback was captured.
The loaded test disappears on a normal reset. The installed image remains
the original working stock-plus-root image; a newer build that suppresses
Magisk UI logging has been prepared but not flashed.

Debugging limits and next step:
- CONFIG_PSTORE and CONFIG_NETCONSOLE are unset; no last_kmsg interface was
  found. Current boot's dmesg did not contain the failed boot's panic trace.
- The exact kernel's machine_kexec contains a BUG when more than one CPU is
  online. A controlled hotplug check successfully offlined CPUs 1..3 and
  restored 0..3. The second boot test had CPU0 only, so that simplification
  did not resolve the failure. Do not claim it was the cause.
- Privileged kallsyms readback was captured temporarily with kptr_restrict=1;
  the original value 2 was restored immediately. A host ELF wrapper around
  the exact Image (not a rebuilt kernel) enabled offline disassembly. The
  linux_banner pointer/string validates the virtual-address mapping.
- We cannot yet distinguish a panic in the outgoing kernel's shutdown/jump
  from a panic during the new kernel's early startup. A UART panic trace or
  another deliberate way to preserve that trace is needed before attributing
  the failure or choosing another handoff variation.

Tools:
  python3 root/kexec.py
  python3 root/kexec.py --payload android --single-core

Both default to load/unload only. --execute explicitly requests the handoff;
that path stops Android framework services, forces disks read-only and
requests a normal reboot if preparation fails. Hello has a userspace 90-second
timer once it starts; the Android payload has no such extra timer. Neither
can guarantee recovery from an early kernel hang. Keep the recovery fallback.
The loader refuses a pre-existing loaded kernel and verifies device identity,
input hashes and entry address. Ethernet must be connected for --execute.

## Prepared reset-only discriminator (not executed)

root/reset-probe.S is a 64-byte ARMv7 payload that clears the transient panic
flag, writes the same watchdog count/kick registers as stock MHAL_PM_WdtRst,
and otherwise loops. It has no Linux startup, filesystem access, or flash
operation. Its physical addresses come from the exact kernel's chip_io_desc
and the MBOOT RIU base; root/build-reset-probe.py checks the original binary
hashes and relevant opcode/data sequences before building it. Unicorn then
executes its instructions and verifies precisely four MMIO writes:
0x1f000c04/32-bit=0, 0x1f006010/16-bit=1, 0x1f006014/16-bit=0,
0x1f006000/8-bit=1. This validates the instruction sequence, not board reset.

The exact MBOOT checks RIU register0x602 (physical0x1f000c04) for 0xdead and
reports kernel_panic if set; otherwise it checks watchdog reset status. A
prompt watchdog reset from this payload would support successful transfer
to the payload, helping separate Linux startup from outgoing-kernel failure.
A boot-reason change alone is not an instruction trace or absolute proof of
execution: an unhandled relocation failure could also end in watchdog reset.
Capture timing and reason together, and keep the conclusion appropriately
qualified. If the watchdog reset does not work, a manual power cycle is needed.

Artifact: assets/research/kexec/reset-probe/probe.bin, SHA256
4501ad430f70fc62ee6f2e4de74c0db379f69bbbed52b2c6037f9a90b177a9bc.
Loader support is prepared via --payload reset-probe; default remains
load/unload only. No hardware load or execution has been performed for it.
The owner is away for about30 minutes; further TV boot tests/flashes are
deferred until the owner returns. The background one-shot launcher track
is local build/QEMU only.

The following sections preserve earlier findings from before Magisk root.

The native PID-1 Hello kit is ready and passes QEMU, but hardware execution
is not established. Stock Android remains installed and running. This work
has not loaded a replacement kernel, rebooted the TV, or written a partition.

## Measured on the original G06/V109

- USB ADB serial `4C0490290FA05A0C0`; `boot_completed=1`, SELinux Enforcing.
- `CONFIG_KEXEC=y`; `kexec_loaded=0`, `kexec_load_disabled=0`.
- Stock `/vendor/bin/kexec` is static ARM32, kexec-tools 2.0.22.git. Its guarded
  empty unload returned 0. This proves the unload path, not image loading.
- A loader check using a non-kernel binary and an invalid image type failed
  before format selection: `Cannot open /proc/iomem: Permission denied`,
  `Could not get memory layout`, exit 255. Loaded state remained 0.
- A root-created RAM file can be populated through ADB while mode 0666. Its
  native execution is denied under `tcl_sitatvservice` with type `tmpfs`.
- Relabeling that same private RAM file to `vendor_file` was denied; execution
  returned 126. Readback confirmed its label stayed `tmpfs` throughout. A
  restore attempt was also denied, but no label change had occurred.
- Live FDT readback succeeded: 38,787 bytes, SHA256
  `0a1ebf816890411ad9ab3652b9fc187a3ab09aa978df6d5e6c66c4f5e742649b`.

Raw evidence: `assets/research/kexec/probe/`, `assets/research/kexec/live.dtb`.
The fixed-operation helper is `hello/FactoryBootProbe.java` (`.#boot-probe`).
It uses the existing on-device factory authorization token, never prints it,
and limits each service command to 220 bytes after a longer command caused a
stack-protector abort in the caller. Android and the service survived that
earlier abort. A successful factory API boolean alone is not execution proof;
use the nonce-tagged output and actual return codes.

## Stock crash-loader discovery

Exact V109 vendor init.fusion.rc defines disabled, oneshot `tcl_kdump_loader`:
`/vendor/bin/sh /vendor/bin/set_kdump.sh`, explicit SELinux domain
`u:r:kdump_set_bootargs:s0`. Stock policy grants that domain `proc_iomem` read
and `sys_boot`. Its script invokes kexec -p on fixed `/vendor/etc/Image` and
`/vendor/etc/dtb` paths. It is not a general upload/execute interface.

Setting persist.tcl.kdump.enable also invokes kdump_set_bootargs, which is
allowed to write MBOOT-related block devices. We have NOT changed that
property or started the loader. Do not enable it merely to obtain a memory
map: it has persistent boot-configuration effects and no proven general RAM
boot handoff. Keep crash-kernel loading separate from normal kexec loading.

## TFTP

Exact V109 MBOOT registered command records confirm tftpboot, bootp, dhcp and
ping. Table addresses and handlers are captured in
`assets/research/kexec/v109-network-commands.json`. These commands require a
bootloader command prompt; ADB and USB Fastboot do not supply one. Fastboot's
inspected command table has no temporary `boot` handler.

`tools/tftp_probe.py --selftest` verifies a bounded, read-only server for one
fixed non-executable 4,096-byte marker. CRC32 is `1a578772`, SHA256 is
`2c0bd296343cb69572428434259302735ce4580d0b40e3714c85fd89b8fa6b20`.
Only loopback has been tested. No TV-side TFTP transfer has occurred.

With a future MBOOT prompt: inspect help/bdinfo/environment, select scratch
RAM from verified live board data, temporarily disable autostart, transfer
only the marker, and compare size/CRC. Do not use auto_tftp/update helpers,
saveenv, execute the marker, or assume a generic compiled loadaddr is safe.

## Next options

The direct stock-service loader route has two independent restrictions:
memory-map access and execution of our loader. Supplying guessed memory
ranges would not fix native execution and would risk corrupting reserved RAM.

A serial MBOOT console would allow the marker test before any RAM kernel boot.
A stock-startup debug image is another possible approach. Upstream Android 11
first-stage init was inspected: it mounts /dev and /proc itself, switches root
to system, then frees the old ramdisk. A naive wrapper that leaves mounts or
files behind is insufficient. No such wrapper has been implemented or flashed.

The owner then explicitly accepted the recovery-USB fallback and requested
the direct PID-1 route. A separate .#hello-pid1-boot image now packages the
kit with the unchanged stock kernel/packed DTB. Its deadline requests Linux
RESTART2 bootloader, with no direct firmware/BCB writes. TV handling remains
unverified. If it restarts the experiment instead, use the proven USB trigger
and restore .#stock-boot. No TV flash has yet occurred during this work.
See pid1/README.txt for the capture and installation commands.

A smaller iteration path is now implemented in the standalone Hello program:
POST /exec accepts a static ARM32 ELF up to 8 MiB, stages it on RAM-backed /tmp,
and execs it as PID 1. QEMU verified two successive uploads, retained PID 1,
display and HTTP, reset userspace uptime, no kernel reboot, and rejection of
a non-ELF upload. This bypasses the need for kexec when only native userspace
changes. The new executable replaces the entire process including diagnostics
and timer threads, so replacements need their own debugging/return behavior.
Kernel/DTB/initramfs changes still require Fastboot or a future kernel loader.
