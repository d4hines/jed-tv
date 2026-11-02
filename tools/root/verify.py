#!/usr/bin/env python3
"""Read-only post-flash root/kexec-access check; does not load a kernel or reboot."""
import datetime
import json
from pathlib import Path
import shlex
import subprocess

ROOT = Path(__file__).resolve().parents[2]
ADB = ["adb", "-s", "4C0490290FA05A0C0", "shell"]


def shell(*args):
    result = subprocess.run(ADB + [shlex.join(args)], capture_output=True, timeout=20, check=True)
    return result.stdout.decode().strip()


def main():
    identity = {key: shell("getprop", key) for key in (
        "ro.product.device", "ro.software.version_id", "sys.boot_completed", "sys.tvroot.ready")}
    assert identity["ro.product.device"] == "G06", identity
    assert identity["ro.software.version_id"] == "V8-T221T04-LF1V109", identity
    assert identity["sys.boot_completed"] == "1", identity
    assert identity["sys.tvroot.ready"] == "1", "Root setup not ready; inspect /dev/tv-adb-root.log"
    path = next((candidate for candidate in ("/debug_ramdisk/su", "/sbin/su")
                 if subprocess.run(ADB + [shlex.join(["test", "-x", candidate])], capture_output=True).returncode == 0), None)
    assert path, "Magisk su was not found"
    checks = {
        "id": "id", "domain": "cat /proc/self/attr/current", "selinux": "getenforce",
        "iomem": "cat /proc/iomem", "kexec_version": "/vendor/bin/kexec --version",
        "kexec_loaded": "cat /sys/kernel/kexec_loaded",
        "kexec_disabled": "cat /proc/sys/kernel/kexec_load_disabled",
    }
    results = {name: shell(path, "-c", command) for name, command in checks.items()}
    assert results["id"].startswith("uid=0("), results["id"]
    assert results["domain"].rstrip("\0") == "u:r:magisk:s0", results["domain"]
    assert results["selinux"] == "Enforcing", results["selinux"]
    assert "System RAM" in results["iomem"], "Real memory map not readable"
    assert results["kexec_disabled"] == "0", results["kexec_disabled"]
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    output = ROOT / "assets/research/stock-plus-root/live" / stamp
    output.mkdir(parents=True)
    report = {"identity": identity, "su": path, "checks": results,
              "root_and_memory_map_access": "passed", "kernel_load_or_handoff_tested": False}
    (output / "result.json").write_text(json.dumps(report, indent=2) + "\n")
    print("PASS: Android boot, ADB su uid0, Magisk domain, global Enforcing, readable memory map, kexec enabled")
    print("No kernel was loaded/executed. Evidence:", output)


if __name__ == "__main__":
    main()
