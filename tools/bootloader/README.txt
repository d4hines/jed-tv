Lean bootloader development
===========================

Signed-update packaging, network boot, compiled ARM model, UDP loopback server
and fault checks: tools/bootloader/update/README.txt. The installed network
gate requires an explicit trusted public key. The persistent bank writer is
a direct G13 eMMC application updater. Deployment state, physical readbacks
and remaining limits are recorded in research/G13_APP_UPDATES.txt.

Source architecture and formal proof coverage: src/bootloader/README.txt.
Check the proofs without building firmware or running the test suites:

  nix develop .#bootloader -c python3 tools/bootloader/prove.py

This elaborates the executable dependencies and Proofs.lean, then audits all
theorem assumptions. It allows only Lean's standard logical axioms and rejects
sorry and native-evaluation assumptions. Proof artifacts are separate from
firmware outputs, in assets/build/leanboot-proofs/. Hardware/FFI contracts and
compiler correctness are outside these proofs.

From the repository root, boot the virtual development machine:

  nix develop .#bootloader -c python3 tools/bootloader/qemu.py

This builds the ordinary Lean -> C -> ARM loader, starts QEMU virt, enters
Linux through our loader, and opens a BusyBox shell. Type poweroff -f to exit.
It downloads two hash-pinned Debian ARM inputs on first use (about 31 MiB).
All files and logs stay in assets/build/leanboot/. No TV, sudo, firmware dump,
guest network or host disk access is required.

For the automatic boot and invalid-image checks:

  nix develop .#bootloader -c python3 tools/bootloader/qemu.py --check

--no-build reuses the last compiled binaries. The default always rebuilds.
Logs: assets/build/leanboot/qemu-input/{boot.log,reject.log,checks.json}.
The manifest records input URLs/hashes and the loader hash. QEMU's device tree
includes a fresh RNG seed so Linux userspace does not stall waiting for entropy.

The virtual CPU runs the real compiled Lean code, not a Python replacement:
  Android v4 readers -> checked memory copies -> FDT validation/fixups
  -> vendor + generic initramfs assembly -> bootconfig trailer
  -> shared cache geometry/maintenance -> CPU handoff -> Linux -> /init.

Os.withPrepared is shared by G13 and QEMU, with explicit board addresses and an
exact-image guard for G13. QEMU's UART and terminal entry differ. It uses a
generic Debian ARM kernel, not the TV's MediaTek-specific kernel or launcher.
That kernel has CONFIG_BOOT_CONFIG disabled: the appended trailer is checked
by the ARM tests below, but QEMU Linux does not parse it. Successful virtual
boot is not proof of physical cache coherency, G13 panel/audio behavior, secure
firmware, MIU, DRAM initialization or eMMC transport. There is no MBOOT in QEMU.

Additional tests (first run build.py or the QEMU command):
  nix develop .#bootloader -c python3 tools/bootloader/check-model.py
  nix develop .#bootloader -c python3 tools/bootloader/check-entry.py
  nix develop .#bootloader -c python3 tools/bootloader/check-cpu.py
  nix develop .#bootloader -c python3 tools/bootloader/check-fdt.py
  nix develop .#bootloader -c python3 tools/bootloader/check-os.py
  nix develop .#bootloader -c python3 tools/bootloader/check-images.py
  nix develop .#bootloader -c python3 tools/bootloader/check-payload.py
  nix develop .#bootloader -c python3 tools/bootloader/check-transfer.py
  nix develop .#bootloader -c python3 tools/bootloader/check-verifiedboot.py
  nix develop .#bootloader -c python3 tools/bootloader/check-serial.py
  nix develop .#bootloader -c python3 tools/bootloader/check-ethernet.py
  nix develop .#bootloader -c python3 tools/bootloader/check.py

check-model.py executes the actual shared core with a pure memory interpreter,
checks deterministic output against an independent byte oracle, and compiles
negative examples to ensure the memory/hardware effect colors cannot mix.
check-entry.py tests CPU admission, private-stack use and return-state recovery.
Neither requires the TV's private reference images.
The FDT/OS/image tests require this TV's ignored, hash-checked reference files.
They execute ARM under Unicorn with memory/ABI guards; the CPU test supplies
register models to cover additional cache geometries and MIU timeout. QEMU's
boot and rejection tests do not need those private TV references.

Runtime machine operations are centralized in src/bootloader/Unsafe/.
The host-only ARM generator is tools/bootloader/Unsafe/Arm.lean, with its
vendored DSL under tools/bootloader/vendor/ and test exports in Harness.lean.
The source audit counts 146 authored ARM instructions across the target
entries and primitives, including returns/address setup and the QEMU entry.
All loops and decisions remain ordinary Lean. audit.py checks the exact
instruction allowlist, permitted
privileged code, linked call targets, and absence of runtime/allocator imports
or writable globals. See Unsafe/CONTRACTS.txt for assumptions and limitations.

Core and effect types
---------------------
  Plan.withPlan       pure values -> validated BootPlan or error callback
  Os.inspect          [Monad m] [Reader m] -> plan callback
  Os.withPrepared     [Monad m] [Reader m] [Writer m] -> KernelConfig callback
  Unsafe.MemoryM      physical RAM reads/writes
  Unsafe.HardwareM    CPU, MMIO, UART, secure calls and terminal transition

The core has no hardware imports. Inspection can run with only a Reader;
preparation can run with a pure StateM interpreter. On the board, Unsafe/Ram
supplies the physical instances. The outer OsEntry driver receives a value
configuration, prints progress, and performs the secure/CPU handoff. The
callback is called once after successful preparation and never on error.
It specializes to direct scalar code; no heap-allocated continuation is kept.

Example of the explicit effect boundary (LeanBoot namespaces):
  def readRam : Unsafe.MemoryM UInt8 := Unsafe.load8 0x26200000
  def driver : Unsafe.HardwareM UInt8 := Unsafe.liftMemory readRam
  -- Unsafe.readSctlr cannot be assigned MemoryM or BaseIO directly.

Only private foreign declarations use raw BaseIO. Explicit runMemory and
runHardware adapters discharge the color at a reviewed outer boundary; the
offline Harness exports use runMemory. The erased Permission type is a color,
not a proof of physical safety. The source audit keeps these adapters out of
the core. A deliberately malicious implementation can bypass the convention.

G13's supported entry is leanboot_os_start at image offset zero. It saves the
caller stack and establishes a private 64 KiB stack at 0x26330000..0x26340000.
The first Lean code checks ARM SVC mode, IRQ/FIQ masking, SP alignment/range and
the expected SCTLR, returning 26 if admission fails. This assumes coherent
executable code, working mappings and a valid caller/private stack already
exist. It cannot recover from invoking an instruction in inaccessible RAM.
The exact-image tests measured 584 bytes of private-stack use; this is not a
formal bound. See Unsafe/CONTRACTS.txt before any hardware entry.

The Ethernet entry accepts MBOOT's observed ARM/SVC state with interrupts
enabled, saves and masks them through two small primitives, then applies the
strict session admission checks. It restores the original interrupt state
before returning. The unchanged OS entry still requires IRQ/FIQ already masked.

The shared functional preparation core now also runs on the TV through the
installed default gate described below. The standalone g13-os.bin entry from
the refactor has only been checked offline; the earlier standalone hardware
results belong to its preserved predecessor build.

Default Lean boot on G13
------------------------
The installed boot_a gate enters compiled Lean at MBOOT's normal kernel
handoff. It restores and validates the original kernel, commits a two-sector
journal, then prepares the kernel inputs through Os.withPrepared. Every third
full start uses the original MBOOT-prepared inputs; the following start uses
Lean again. This includes ordinary adb reboots, and successful Home startup
does not reset the count. Both paths run the same native OS, without changing
slots. MBOOT still supplies early board initialization and image loading.

The current network build inserts a bounded signed-upload window before
Os.withPrepared on Lean starts. The third-start escape skips this window.
Stage0 expands the compressed gate within the same 128 KiB prefix. For the
installed build, signing command and hardware evidence, read
research/G13_SIGNED_NETWORK_BOOT.txt and tools/bootloader/update/README.txt.
The commands below build/check the core gate without the network entry;
build-network.py produces the network gate using those core artifacts.

The gate, LZ4 decoder, journal and bounded FCIE/DMA driver are Lean. Hardware
effects stay in Unsafe/; the runtime calls no MBOOT functions. Only 128 KiB
of boot_a's kernel field changes. The journal occupies upgrade's final two
sectors. Read research/G13_LEANBOOT_DEFAULT.txt for the exact installed hash,
hardware evidence, storage limits and UART rollback procedure before making
any boot-state change.

Build and offline checks:
  nix develop .#bootloader -c python3 tools/bootloader/build.py
  nix develop .#bootloader -c python3 tools/bootloader/build-boot-image.py
  nix develop .#bootloader -c python3 tools/bootloader/check-boot-policy.py
  nix develop .#bootloader -c python3 tools/bootloader/check-lz4.py
  nix develop .#bootloader -c python3 tools/bootloader/check-emmc.py
  nix develop .#bootloader -c python3 tools/bootloader/check-boot-gate.py

install-boot-image.py requires exact target hashes and a matching successful
RAM trial before its explicit --apply write. verify-default-tv.py starts from
a freshly initialized journal and performs four reboots; it is not a read-only
status command. Neither is a general installer for other firmware or boards.

G13 hardware experiments
------------------------
Read RECOVERY_NOTES.txt and research/G13_LEAN_BOOTLOADER.txt first. These RAM
experiments retain the installed images. OS-capable payloads must not use the
original probe's
0x25000000 address: it overlaps the assembled initramfs.

tv_session.py contains the current experiment's Screen transport and RAM-only
helpers. It is tied to a verified session/memory map; it is not an automatic
installer or a general recovery command. Do not invoke it from an Android
shell or use stale session IDs/addresses. It never opens the serial port.
The G13 experiment still relies on stock early DRAM/secure initialization and
MBOOT's initial raw reads/upload. Payload execution calls no MBOOT functions.

Large serial uploads use raw LZ4 blocks when compression saves space, staged at
0x26100000 and expanded by the verified MBOOT lz4 command. This is a bootstrap
transport dependency, not a runtime call from Lean. The host checks a local
round trip, the compressed CRC before decompression, and the final RAM CRC.
The stock decoder trusts the source length, so never use untrusted compressed
input or omit the first CRC. check-transfer.py executes that exact decoder on
14 inputs with memory guards. The hardware-tested 18992-byte loader compresses to
12651 bytes; the 3136-byte control block compresses to 1927 bytes.

Bulk transport uses MBOOT loady and lrzsz sb through Screen's exec !! filter;
the existing Screen session owns UART throughout. Start the sender before
loady so it receives the first handshake; wait for MBOOT's prompt and receiver
drain afterward. The bounded sender restores terminal settings, records its
exit status, and the host checks the exact RAM CRC. Failed transfers stop;
they do not silently fall back to writes into an unknown receiver state.
Transfers below 1024 bytes use word commands. Local check-serial.py tests the
compressed current loader plus every byte value over a 115200-baud model,
and checks terminal settings. Receiver TX uses a pipe because a PTY's drain
does not wait for its master to read data; a subsequent flush could otherwise
discard the final ACK. This test never accesses the TV port.
Hardware loader + control upload took 12.22 s including decompression/CRCs,
versus 79.87 s for the same compressed data sent as mw.l commands. Prefer
Ethernet ADB for bulk data once Linux is up.

Bare-metal Ethernet
-------------------
The separate g13-ethernet.bin payload now exchanges ARP, ICMP echo and UDP echo
(port 7533) at 192.168.77.3 through our compiled Lean driver. It uses the exact
V013 PHY register recipe, four RX buffers and a polled TX path. The deterministic
Ethernet.respond core has only Reader/Writer capabilities; Unsafe/Emac.lean
owns MMIO, DMA ownership and bounded polling. No MBOOT network function is called.

At an already verified MBOOT prompt, using the existing Screen session:

  nix develop .#bootloader -c python3 tools/bootloader/ethernet-tv.py --mode phy
  nix develop .#bootloader -c python3 tools/bootloader/ethernet-tv.py --mode echo

These are session-specific RAM experiments, not commands to run while Linux
owns Ethernet. The echo check uses sudo -n for a raw socket on host enu1u3;
it sends 11 bounded packet tests, then a UART byte stops the payload. Code is
uploaded with YMODEM/raw LZ4 and CRC checks. The payload disables DMA before
returning to MBOOT; a normal reset restores the existing native TV startup.
This separate Ethernet probe has no update protocol, eMMC writer or automatic
boot entry, and its runner does not flash firmware.

check-ethernet.py executes compiled ARM against independent packet/checksum
oracles, guarded buffers and an MMIO model. When the private backup is present,
it also compares the PHY writes with actual emulation of exact V013 MBOOT.
QEMU virt has no model of this MStar MAC; its OS tests do not validate Ethernet.
Hardware evidence, current limitations and the original entry-guard rejection:
research/G13_LEANBOOT_ETHERNET.txt.

The second G13 OS trial reached native READY at 19.91 s. The missing step in
the first trial was secure verified-boot parameter initialization: Lean now
builds the exact unlocked/orange message, and a six-instruction FFI sends it
to the existing secure firmware. Nonzero status stops before Linux. See
research/G13_LEAN_BOOTLOADER.txt for evidence, the first trial's init_user0
recovery request and remaining early-board dependencies. No automatic boot
selection or persistent loader installation is part of this RAM experiment.
