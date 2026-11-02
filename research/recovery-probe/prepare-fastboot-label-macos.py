#!/usr/bin/env python3
"""Verify/repair only the original PNY recovery stick's three FAT labels."""
import argparse
import base64
import datetime
import hashlib
import json
import os
from pathlib import Path
import plistlib
import re
import stat
import struct
import subprocess

SIZE = 31042043904
SERIAL = "07210C69972A5005"
NEW = b"\n--fast\n   "
ALLOWED = {b"\n--show\n   ", b"TVRECOVERY ", NEW}
ROOT = Path(__file__).resolve().parents[2]


def require(ok, message):
    if not ok:
        raise RuntimeError(message)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("device", help="current whole disk, e.g. /dev/disk4")
    parser.add_argument("--apply", action="store_true")
    args = parser.parse_args()
    require(re.fullmatch(r"/dev/disk[0-9]+", args.device), "Select a whole /dev/diskN")
    info = plistlib.loads(subprocess.check_output(["/usr/sbin/diskutil", "info", "-plist", args.device]))
    is_whole = info.get("WholeDisk", info.get("Whole")) is True
    require(is_whole and info.get("Internal") is False and info.get("BusProtocol") == "USB"
            and info.get("ParentWholeDisk") == info.get("DeviceIdentifier"),
            "Expected a whole external USB disk")
    require(info.get("TotalSize") == SIZE and info.get("MediaName") == "USB 3.1 FD",
            "Drive identity/size mismatch")
    devices = plistlib.loads(subprocess.check_output(["/usr/sbin/diskutil", "list", "-plist", "external", "physical"]))
    whole = [d["DeviceIdentifier"] for d in devices["AllDisksAndPartitions"]]
    require(whole == [info["DeviceIdentifier"]], "Disconnect other external drives before repair")
    usb = subprocess.check_output(["/usr/sbin/ioreg", "-p", "IOUSB", "-l", "-w", "0"]).decode()
    require(len(re.findall(r'"USB Serial Number" = "' + SERIAL + r'"', usb)) == 1,
            "Original PNY USB serial is not uniquely present")
    raw = args.device.replace("/dev/disk", "/dev/rdisk")
    # Unmounting preserves files and avoids racing macOS filesystem writes.
    subprocess.run(["/usr/sbin/diskutil", "unmountDisk", args.device], check=True)
    flags = os.O_RDWR | os.O_SYNC if args.apply else os.O_RDONLY
    fd = os.open(raw, flags)
    try:
        require(stat.S_ISCHR(os.fstat(fd).st_mode), "Expected a raw character device")

        def read(offset, count=512):
            data = os.pread(fd, count, offset)
            require(len(data) == count, "Short device read")
            return data

        mbr = read(0)
        require(mbr[510:] == b"\x55\xaa" and mbr[450] in (0x0b, 0x0c), "Unexpected MBR")
        start, sectors = struct.unpack_from("<II", mbr, 454)
        require(start == 2048 and sectors == 60626944 and not any(mbr[462:510]),
                "Partition layout differs from original PNY stick")
        offset = start * 512
        boot = read(offset)
        require(boot[510:] == b"\x55\xaa" and boot[82:90] == b"FAT32   ", "Not FAT32")
        require(boot[67:71] == bytes.fromhex("f421cb6e"), "Unexpected FAT UUID; refusing another filesystem")
        bps = struct.unpack_from("<H", boot, 11)[0]
        spc = boot[13]
        reserved = struct.unpack_from("<H", boot, 14)[0]
        fats = boot[16]
        fatsz = struct.unpack_from("<I", boot, 36)[0]
        root = struct.unpack_from("<I", boot, 44)[0]
        backup = struct.unpack_from("<H", boot, 50)[0]
        require((bps, spc, reserved, fats, fatsz, root, backup) == (512, 32, 32, 2, 14816, 2, 6),
                "FAT geometry differs from captured original")
        backoff = offset + backup * bps
        back = read(backoff)
        require(back == boot, "Primary and backup FAT boot sectors differ")
        rootoff = offset + (reserved + fats * fatsz + (root - 2) * spc) * bps
        directory = read(rootoff, spc * bps)
        positions = []
        for pos in range(0, len(directory), 32):
            entry = directory[pos:pos + 32]
            if entry[0] == 0:
                break
            if entry[0] != 0xe5 and entry[11] == 8:
                positions.append(pos)
        require(len(positions) == 1, "Expected exactly one root volume-label entry")
        pos = positions[0]
        old = boot[71:82]
        require(old in ALLOWED and directory[pos:pos + 11] == old,
                "Root/boot labels differ or are not recognized; no write performed")
        require(directory[pos + 20:pos + 22] == bytes(2)
                and directory[pos + 26:pos + 32] == bytes(6), "Volume label unexpectedly refers to file data")
        plans = []
        for sector, within in ((offset, 71), (backoff, 71), (rootoff + pos // bps * bps, pos % bps)):
            before = read(sector)
            require(before[within:within + 11] == old, "Label changed during inspection")
            after = before[:within] + NEW + before[within + 11:]
            plans.append({"offset": sector, "within": within,
                          "before_b64": base64.b64encode(before).decode(),
                          "after_b64": base64.b64encode(after).decode()})
        report = {"device": raw, "serial": SERIAL, "bytes": SIZE,
                  "uuid_hex": boot[67:71].hex(), "old_label_hex": old.hex(),
                  "new_label_hex": NEW.hex(), "sectors": plans, "applied": False}
        print(json.dumps({k: v for k, v in report.items() if k != "sectors"}, indent=2), flush=True)
        if not args.apply:
            print("DRY RUN: all three labels verified; no bytes written")
            return
        if old == NEW:
            print("All three labels already match yesterday's Fastbootd trigger; no write needed")
            return
        directory = ROOT / "assets/research/recovery-probe/usb-backup"
        directory.mkdir(parents=True, exist_ok=True)
        stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
        record = directory / ("macos-fastboot-label-" + stamp + ".json")

        def save(exclusive=False):
            with record.open("x" if exclusive else "w") as stream:
                json.dump(report, stream, indent=2)
                stream.write("\n")
                stream.flush()
                os.fsync(stream.fileno())
            record.chmod(0o600)
            if "SUDO_UID" in os.environ:
                os.chown(record, int(os.environ["SUDO_UID"]), int(os.environ["SUDO_GID"]))

        save(exclusive=True)
        for plan in plans:
            require(read(plan["offset"]) == base64.b64decode(plan["before_b64"]), "Device changed before write")
            after = base64.b64decode(plan["after_b64"])
            require(os.pwrite(fd, after, plan["offset"]) == 512, "Short device write")
        os.fsync(fd)
        os.close(fd)
        fd = os.open(raw, os.O_RDONLY)
        for plan in plans:
            require(read(plan["offset"]) == base64.b64decode(plan["after_b64"]), "Sector readback mismatch")
        report["applied"] = True
        save()
        print("APPLIED: all three sectors read back correctly. Files and partition layout preserved.")
        print("Sector backup:", record)
    finally:
        os.close(fd)


if __name__ == "__main__":
    main()
