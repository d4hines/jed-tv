"""Exercise flash identity gates and command ordering without TV access."""
import contextlib
import io
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

import flash


class FlashTests(unittest.TestCase):
    def setUp(self):
        self.commands = []
        self.booted = False
        self.vars = {"product": "C06_eu", "is-userspace": "no", "unlocked": "yes",
                     "version-bootloader": flash.BOOTLOADER,
                     "partition-size:boot": hex(flash.BOOT_SIZE)}

    def fake_run(self, *command, **kwargs):
        self.commands.append(command)
        out, err = "", ""
        if command[:3] == ("adb", "devices", "-l"):
            out = "List of devices attached\nUSBSERIAL device usb:1-1 product:G06_2K_US\n"
        elif command[0] == "adb" and command[3] == "shell":
            out = flash.SERIAL + "\n" if command[4] == "getprop ro.serialno" else "\n".join([
                flash.SERIAL, "G06", flash.FINGERPRINT, flash.BOOTLOADER, "unlocked", "1"])
        elif command[0] == "adb" and command[3:] == ("reboot", "bootloader"):
            self.booted = True
        elif command == ("fastboot", "devices", "-l"):
            out = "garbled-serial fastboot usb:1-1.3\n" if self.booted else ""
        elif command[0] == "fastboot" and command[3] == "getvar":
            name = command[4]
            err = f"(bootloader) {name}: {self.vars[name]}\nFinished. Total time: 0.001s\n"
        return subprocess.CompletedProcess(command, 0, out, err)

    def invoke(self):
        with tempfile.TemporaryDirectory() as directory, contextlib.chdir(directory), \
             patch("sys.argv", ["flash"]), \
             patch.object(flash, "verify_backup", return_value=Path("boot.img")), \
             patch.object(flash, "sha256", return_value=flash.BOOT_HASH), \
             patch.object(flash.Device, "run", side_effect=self.fake_run), \
             contextlib.redirect_stdout(io.StringIO()):
            flash.main()

    def test_only_boot_written_after_identity_checks(self):
        self.invoke()
        writes = [c for c in self.commands if c[0] == "fastboot" and len(c) > 3 and c[3] == "flash"]
        self.assertEqual(writes, [("fastboot", "-s", "usb:1-1.3", "flash", "boot", "boot.img")])
        self.assertEqual(self.commands[-1][0], "adb")
        index = self.commands.index(writes[0])
        self.assertTrue(any(c[-1] == "partition-size:boot" for c in self.commands[:index]))

    def test_each_fastboot_identity_mismatch_prevents_write(self):
        for name in self.vars:
            with self.subTest(name=name):
                saved = self.vars[name]
                self.vars[name] = "0" if name == "partition-size:boot" else "WRONG"
                self.commands.clear()
                self.booted = False
                with self.assertRaises(RuntimeError):
                    self.invoke()
                self.assertFalse(any("flash" in c for c in self.commands))
                self.vars[name] = saved

    def test_ethernet_only_prevents_reboot(self):
        device = flash.Device(io.StringIO())
        with patch.object(device, "run", return_value=subprocess.CompletedProcess([], 0,
                          "List of devices attached\n192.168.77.2:5555 device product:G06_2K_US\n", "")):
            with self.assertRaisesRegex(RuntimeError, "USB ADB"):
                device.usb_adb()

    def test_g13_identity_rejected(self):
        device = flash.Device(io.StringIO())
        with patch.object(device, "shell", return_value="18ACC22D1C47\nG13_2K_US\n"):
            with self.assertRaisesRegex(RuntimeError, "mismatch"):
                device.identity("USB")

    def test_missing_fastboot_variable_rejected(self):
        device = flash.Device(io.StringIO())
        with patch.object(device, "run", return_value=subprocess.CompletedProcess([], 0, "", "")):
            with self.assertRaisesRegex(RuntimeError, "Missing Fastboot variable"):
                device.getvar("usb:1-1.3", "unlocked")


if __name__ == "__main__":
    unittest.main()
