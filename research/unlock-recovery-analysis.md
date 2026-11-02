# Post-unlock recovery hypothesis

Offline investigation, 2026-10-30. No device commands were issued by this
analysis. The installed V109 bootloader has not been extracted; addresses and
behavior below are from the newer V122 binary identified in unlock-analysis.md.

## Concrete reason to reenter stock recovery

The V122 USB unlock path writes `boot-recovery` and
`recovery\n--wipe_data\n` to `misc`, raw-erases userdata, then changes the
lock state. A recovery boot is therefore part of completing its reset.

Its USB `reboot-bootloader` handler at `0x274a7ffc` writes 16 bytes beginning
`boot-bootloader` to `misc` offset zero, replacing the `boot-recovery` command.
The boot-mode parser compares that command at `0x27420ce4`, zeroes the first
32 bytes of its buffer, invokes `mmc write.p ... misc 32`, then selects the
Fastboot boot mode. The recovery arguments begin at byte 64, beyond the short
write even if the command interprets `32` as hexadecimal.

Normal USB `reboot`, handler `0x274a80b0`, only replies and resets; it does not
restore the previous recovery request. Thus the actual sequence we performed,
unlock -> reboot bootloader -> reboot, can divert the planned recovery reset
in this newer implementation. That is a specific explanation to test on V109,
not a proven diagnosis of its failure.

USB `reboot-recovery`, handler `0x274a810c`, writes 16 bytes beginning
`boot-recovery` to `misc` offset zero and resets. If Fastboot is accessible,
`fastboot reboot recovery` is the least invasive software step to restore the
intended path. It can cause the already-authorized pending data wipe to run
immediately if the arguments survive. Otherwise the recovery menu can expose
logs and its own factory-reset action. Do not substitute a blind Fastboot
erase of metadata, a relock, or a firmware update without new evidence.

## Encryption relevance and limits

The captured V109 baseline has `ro.crypto.metadata.enabled=true` and
`ro.crypto.state=encrypted`. Raw-erasing userdata is not the same operation
as recovery formatting its filesystem and managing encryption metadata.
[AOSP Android 11 recovery WipeData](https://android.googlesource.com/platform/bootable/recovery/+/refs/tags/android-11.0.0_r1/install/wipe_data.cpp)
formats `/data`, `/cache` when present, and `/metadata` when present. This
supports preferring the device's stock recovery reset to ad hoc partition
erases; it does not prove TCL's V109 recovery has identical code.

The user clarified that the TV goes to standby/powers off after the logo,
rather than repeatedly rebooting. This does not establish the cause. USB
enumeration, transient ADB, stock recovery output, and UART boot
logs are the useful next observations. No actual same-family report proving
the same unlock-only boot failure was found in this short primary-source
search.

## Offline reproduction

Use the existing `inspect-mboot.py` with Python and Capstone:

- `--start 0x274a7ffc --length 0x280` shows all four reboot handlers.
- `--start 0x27420ce4 --length 0xb8` shows boot-bootloader consumption.
- Boot-mode comparison text resides at `0x27646fb6` (`boot-bootloader`).

This finding concerns a sequencing problem in a reset path, not evidence that
any system, boot, recovery, or MBOOT image was replaced by our commands.

## Follow-up after physical-button recovery did not open

The owner reports that holding the physical button before connecting mains
leaves the TV off rather than showing recovery. The generic TCL recovery
procedure has therefore not been demonstrated on this unit. Do not describe
another timing or button combination as verified without new evidence.

Further laptop-only extraction from the same official V122 OTA obtained
`tvconfig.img` and `tclconfig.img` (ZIP CRC checks passed; full OTA checksum
was not checked). The latter contains an explicit 32Q3K configuration,
saved as `assets/research/firmware/V8-T221T04-LF1V122/32Q3K-hardware.json`, referencing
MT9221, remote RC813, and the ST3151B07-1 panel. This strengthens model-family
matching but still does not establish the installed V109 implementation.

The V122 boot ramdisk's extracted `fstab.mt5867` mounts userdata at `/data`
with `latemount`, file encryption, and metadata keys under `/metadata`, but
without the `formattable` flag. Its `/metadata` entry does have that flag.
This strengthens the hypothesis that raw-erasing userdata while skipping
recovery leaves a normal boot unable to prepare `/data`. It is not an actual
error log from the failing TV.

V122 `get_boot_mode_from_keypad`, at 0x274211bc, recognizes keycode 0x03 as
mode 12 (enable UART) and a sufficiently long 0xd5 press as mode 4 (USB
upgrade). Mode 4 tries `custar` and an OTA check; this is not proof that a
held physical button directly boots the existing recovery with no media.
These keycodes are logical values, not a verified mapping for the Q3K's
single physical button at power application.

The extracted `ir_config.ini` uses RCA protocol/header 0x000f for normal TV
keys and a distinct NEC 0x807f MStar group for boot actions. The dynamic IR
handler and parser have version-specific differences from some listed keys;
no ordinary RC813 button shortcut is established. Do not prescribe one based
only on the comments in that file.

A 150-second USB kernel-event monitor recorded no events, and ADB/Fastboot
lists were empty. The owner has not yet confirmed completing a normal
startup with USB reconnected during that monitoring window, so this does
not rule out a transient device during a future observed startup.

Prepared `research/capture-uart.py` for 115200 8N1 capture with flow control
disabled and no serial data writes. Its CLI was checked; no serial hardware
was connected or tested. Before use, confirm adapter I/O compatibility and
the candidate HDMI pinout electrically. The immediate first capture should
use only adapter RX and GND; UART was enabled in the actual TV before the
unlock, but continued output has not yet been observed.
