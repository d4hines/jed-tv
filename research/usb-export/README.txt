TCL 32Q3K V109 firmware backup USB

This FAT32 stick stores a copy of the verified V109 dump and the supporting
recovery/root notes. Its ordinary volume label is TVBACKUP. It is a storage
backup; inserting it into the TV does not install or restore these images.
The earlier special Fastbootd-trigger volume label has been removed by format.

The list below describes the original verified USB export. The laptop's later
assets/ reorganization did not modify that stick. New copies made with the
updated copy-backup.py preserve the new layout: backup directories become
TCL32Q3K/assets/backups/ and generated research artifacts move beneath
TCL32Q3K/assets/research/. Authored notes/scripts remain in TCL32Q3K/research/.
COPY_MANIFEST.json and SHA256SUMS give exact paths for each export.

Original export contents:
  TCL32Q3K/backups/v109-stock-20261031/
    33 raw images, 4,741,660,672 bytes total, plus manifests and checksums.
    Includes boot, recovery, MBOOT/MBOOTBAK, vbmeta, super and calibration.
    Userdata, RPMB and hardware fuses are excluded. Restore is untested.
  TCL32Q3K/RECOVERY_NOTES.txt
    Earlier unlock failure, working recovery procedure and precautions.
  TCL32Q3K/research/ROOT_READBACK_NOTES.txt
    Stock-service root access, exact helper commands and backup method.
  TCL32Q3K/research/recovery-probe/
    Working cache-menu image and recovery-trigger artifacts. These are
    stored in this subdirectory as references, not as automatic USB updates.
  TCL32Q3K/research/
    Root/readback helper sources, compiled helpers and supporting evidence.
  PREVIOUS_USB_IMAGE.txt
    Location and reassembly instructions for the original recovery-stick image.
  TCL32Q3K/backups/usb-fastboot-trigger-20261031T054144Z/
    Complete old USB image compressed and split into parts of at most 2 GiB.
    This preserves the earlier Fastbootd-trigger stick for future use.
  SHA256SUMS
    Checksums for all copied payload files, relative to this USB root.

From this USB root on a host with sha256sum:
  sha256sum -c SHA256SUMS
The V109 backup also has its own SHA256SUMS in its directory.

All individual files fit FAT32's maximum file size, including super.img.
Only the old USB archive is split; all 33 TV partition images remain intact.
The laptop retains the original mode0400 image files. FAT32 does not preserve
their Unix permissions; make separate working copies before patching images.
Calibration and certificate images belong to this particular TV.

This is a capture after bootloader unlock and factory reset, not an untouched
factory-shipped whole-device clone. Firmware was not replaced during readback;
mutable partitions are online captures. See the backup README and manifest.
