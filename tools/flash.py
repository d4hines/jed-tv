#!/usr/bin/env python3
"""Restore only this G06's original boot partition, with identity/size gates.

Firmware stays outside the Nix store. Run from the repository root.
No unlock, erase, relock, vbmeta changes, or factory reset commands exist here.
"""
import argparse
import datetime as dt
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time

SERIAL = "4C0490290FA05A0C0"
FINGERPRINT = "TCL/G06_2K_US/G06:11/AR2101/AR33:user/release-keys"
BOOTLOADER = "01.01.250118"
BOOT_HASH = "d116dfa370d8e5dd5809f6f65266c86671b5e9bec9d62f95c0977bca0ff30085"
BOOT_SIZE = 32505856
BACKUP = Path("assets/backups/v109-stock-20261031")


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def sha256(path):
    with path.open("rb") as stream:
        return hashlib.file_digest(stream, "sha256").hexdigest()


def verify_backup(directory, all_images=False):
    manifest = json.loads((directory / "manifest.json").read_text())
    require(manifest["complete"] is True and manifest["fingerprint"] == FINGERPRINT,
            "Backup is incomplete or belongs to different firmware")
    items = manifest["files"]
    require(len(items) == 33 and len({f["name"] for f in items}) == 33,
            "Expected the complete 33-image G06 backup")
    boot = next(f for f in items if f["name"] == "boot")
    require(boot["sha256"] == BOOT_HASH and boot["size_bytes"] == BOOT_SIZE
            and boot["filename"] == "boot.img", "Unexpected original boot metadata")
    for item in items if all_images else [boot]:
        require(Path(item["filename"]).name == item["filename"], "Invalid image filename")
        image = directory / item["filename"]
        require(item["status"] == "verified" and item["source_matches"] is True
                and item["sha256"] == item["source_sha256_after_copy"],
                "Backup source verification failed: " + item["name"])
        require(image.stat().st_size == item["size_bytes"]
                and sha256(image) == item["sha256"], "Image checksum/size failed: " + str(image))
        require(image.stat().st_mode & 0o222 == 0, "Original must remain read-only: " + str(image))
        print("Verified original:", item["name"], item["size_bytes"], item["sha256"], flush=True)
    return directory / "boot.img"


class Device:
    def __init__(self, log):
        self.log = log

    def run(self, *command, timeout=30, check=True):
        result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                timeout=timeout, text=True, errors="replace")
        self.log.write(json.dumps(dict(command=command, returncode=result.returncode,
                                      stdout=result.stdout, stderr=result.stderr)) + "\n")
        self.log.flush()
        if check:
            require(result.returncode == 0, f"Command failed: {command}: {result.stdout}{result.stderr}")
        return result

    def shell(self, target, command):
        return self.run("adb", "-s", target, "shell", command).stdout.strip()

    def identity(self, target):
        expected = {
            "ro.serialno": SERIAL,
            "ro.product.device": "G06",
            "ro.build.fingerprint": FINGERPRINT,
            "ro.bootloader": BOOTLOADER,
            "ro.boot.vbmeta.device_state": "unlocked",
            "sys.boot_completed": "1",
        }
        output = self.shell(target, "; ".join("getprop " + p for p in expected))
        require(output.splitlines() == list(expected.values()),
                "TV identity/boot state mismatch: " + repr(output))
        return expected

    def usb_adb(self):
        candidates = []
        for line in self.run("adb", "devices", "-l").stdout.splitlines()[1:]:
            fields = line.split()
            if len(fields) >= 2 and fields[1] == "device" and ":" not in fields[0]:
                if self.shell(fields[0], "getprop ro.serialno") == SERIAL:
                    candidates.append(fields[0])
        require(len(candidates) == 1,
                "Need exactly one authorized USB ADB connection to this G06 before reboot. "
                "Ethernet ADB alone is insufficient; check the USB data cable.")
        self.identity(candidates[0])
        return candidates[0]

    def fastboot_devices(self):
        devices = []
        for line in self.run("fastboot", "devices", "-l").stdout.splitlines():
            if "fastboot" in line.split():
                path = re.search(r"\busb:\S+", line)
                require(path is not None, "Cannot identify Fastboot USB path: " + line)
                devices.append(path[0])
        return devices

    def getvar(self, target, name):
        result = self.run("fastboot", "-s", target, "getvar", name)
        match = re.search(r"^(?:\(bootloader\)\s*)?" + re.escape(name) + r":\s*(\S+)",
                          result.stdout + result.stderr, re.M)
        require(match is not None, "Missing Fastboot variable " + name)
        return match[1]

    def fastboot_identity(self, target):
        for name, value in {"product": "C06_eu", "is-userspace": "no", "unlocked": "yes",
                            "version-bootloader": BOOTLOADER}.items():
            require(self.getvar(target, name) == value, "Fastboot identity mismatch: " + name)
        require(int(self.getvar(target, "partition-size:boot"), 16) == BOOT_SIZE,
                "Fastboot boot partition size differs from verified backup")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--backup", type=Path, default=BACKUP)
    parser.add_argument("--check", action="store_true", help="verify all 33 local images; no TV access")
    parser.add_argument("--preflight", action="store_true", help="check images and USB TV identity; no reboot/write")
    parser.add_argument("--fastboot", metavar="USB_PATH",
                        help="restore from an existing bootloader session (e.g. usb:1-1.3); "
                             "requires matching product, bootloader, size and unlocked state")
    args = parser.parse_args()
    os.umask(0o077)
    boot = verify_backup(args.backup.resolve(), args.check)
    if args.check:
        return
    stamp = dt.datetime.now(dt.timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    output = Path("assets/research/flash") / stamp
    output.mkdir(parents=True)
    print("Evidence:", output, flush=True)
    with (output / "commands.jsonl").open("x") as log:
        dev = Device(log)
        if args.fastboot:
            require(args.fastboot.startswith("usb:"), "Select a USB path from fastboot devices -l")
            require(dev.fastboot_devices() == [args.fastboot], "Ambiguous or missing Fastboot device")
            target = args.fastboot
        else:
            usb = dev.usb_adb()
            require(not dev.fastboot_devices(), "An unrelated Fastboot device is already connected")
            print("Confirmed original G06 over USB:", usb, flush=True)
            if args.preflight:
                print("Preflight passed; no reboot or write performed.")
                return
            print("Rebooting to bootloader; the TV screen may go black.", flush=True)
            dev.run("adb", "-s", usb, "reboot", "bootloader")
            deadline = time.monotonic() + 45
            devices = []
            while time.monotonic() < deadline:
                devices = dev.fastboot_devices()
                if devices:
                    break
                time.sleep(2)
            require(len(devices) == 1, "No unique USB Fastboot transport appeared; no image written")
            target = devices[0]
        dev.fastboot_identity(target)
        if args.preflight:
            print("Bootloader preflight passed; no write performed.")
            return
        # Recheck immediately before use. Only the pinned original is accepted.
        require(sha256(boot) == BOOT_HASH, "Original boot image changed during preflight")
        print("Flashing ONLY original boot.img:", BOOT_HASH, flush=True)
        dev.run("fastboot", "-s", target, "flash", "boot", str(boot), timeout=120)
        print("Flash accepted. Rebooting to Android.", flush=True)
        dev.run("fastboot", "-s", target, "reboot")
        print("Waiting for Android USB ADB and boot_completed=1…", flush=True)
        deadline = time.monotonic() + 180
        while time.monotonic() < deadline:
            try:
                usb = dev.usb_adb()
                state = dev.identity(usb)
                (output / "boot-success.json").write_text(json.dumps(state, indent=2) + "\n")
                print("Original boot restored; correct G06 Android boot completed.", flush=True)
                print("Block readback is a separate verification; Fastboot OKAY alone is not a hash check.")
                return
            except RuntimeError:
                time.sleep(3)
        raise RuntimeError("Flash accepted, but normal Android startup was not confirmed within 180 seconds; "
                           "inspect the TV and evidence before further writes")


if __name__ == "__main__":
    try:
        main()
    except (RuntimeError, OSError, ValueError, KeyError, StopIteration, subprocess.TimeoutExpired) as error:
        sys.exit("STOP: " + str(error))
