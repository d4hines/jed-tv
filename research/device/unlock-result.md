# Live V109 bootloader unlock

Current outcome: the owner reached Google TV setup after a stock-recovery
factory reset. Read [RECOVERY_NOTES.txt](../../RECOVERY_NOTES.txt) for the
complete recovery procedure and prevention guidance. The chronology below
includes observations made before recovery succeeded.

Tested 2026-10-30 on the owner's TCL 32Q3K, installed firmware
V8-T221T04-LF1V109.000732 and bootloader 01.01.250118. The owner explicitly
accepted a factory reset before the unlock command was issued.

USB Fastboot target: `usb:3-1.3`.

`fastboot flashing unlock` returned `OKAY` in 0.268 seconds. Immediate
readback returned:

```text
unlocked: yes
secure: no
flashing get_unlock_ability: unlock
```

After `fastboot reboot bootloader`, readback again returned:

```text
unlocked: yes
version-bootloader: 01.01.250118
secure: no
```

The laptop's kernel log confirms a USB disconnect and re-enumeration during
that restart. The unlocked state therefore survived a bootloader restart.
This does not establish which hardware secure-boot restrictions remain.

`fastboot reboot` was then issued successfully, with the intention of starting
normal boot and allowing any pending recovery wipe to complete. At that point,
neither Android boot nor the actual wipe had been confirmed. The intervening
bootloader reboot likely diverted the wipe request, as discussed below. No
replacement image had been flashed, and no critical-unlock command was issued.
Android root access had not been established at that point.

Raw command results:

- `fastboot-unlock-attempt.txt`
- `fastboot-unlock-state-before-reboot.txt`
- `fastboot-unlock-reboot-bootloader.txt`
- `fastboot-unlock-state-after-reboot.txt`
- `fastboot-unlock-reboot-system.txt`

At that stage, the original V109 images had not been backed up. Standard USB
Fastboot readback had failed, so UART/MBOOT was the backup avenue under
consideration. A later stock-service route successfully read the original
images; see the final section below and `../ROOT_READBACK_NOTES.txt`.

## Subsequent startup failure

The owner subsequently reported that the Google TV logo appears briefly and
then the TV goes into standby/powers off. They confirmed that only USB and
Ethernet were connected, with no HDMI/UART wiring. A fresh laptop check found
neither ADB nor USB Fastboot enumerated.

Offline examination of the newer V122 bootloader suggests that the
`reboot-bootloader` persistence check may have replaced the pending
`boot-recovery` request, diverting the recovery wipe/format after unlocking.
This is a plausible explanation, not a verified diagnosis of the installed
V109. See `../unlock-recovery-analysis.md` for the binary evidence.

At that stage, the proposed next step was TCL's physical-power-button entry
into stock Android Recovery, followed by its factory reset if necessary. No
additional device write, relock, metadata erase, or firmware flash had been
performed at that point. The subsequent cache write and recovery are below.

## Recovery succeeded

The owner subsequently confirmed normal startup to the Google TV setup
screen after a stock-recovery factory reset. We reached recovery by first
using the crafted FAT volume-label USB probe to enter Fastbootd, selecting
Reboot to bootloader, writing a validated 100 MiB ext4 **cache** image whose
recovery command was only `--show_text`, and issuing `fastboot reboot recovery`.
The resulting recovery menu stayed open and exposed the factory-reset action.

The bootloader reported `unlocked: yes`, `secure: no`, and its original
`01.01.250118` version immediately before that cache write. After reset, ADB
shell access was restored and Android reported `ro.boot.vbmeta.device_state=unlocked`,
`ro.boot.verifiedbootstate=orange`, and `sys.boot_completed=1`. The ADB shell
runs as uid 2000. See `post-reset-adb-state.txt` for the read-only results.
No modified boot image or bootloader was installed. A subsequent stock-service
registration probe established uid0 command execution and verified V109 raw
partition backups; see `../ROOT_READBACK_NOTES.txt`. ADB itself remains uid2000.
See `../recovery-probe/NOTES.txt` and `cache-menu-flash-result.txt` for the
recovery sequence and its validation limits.
