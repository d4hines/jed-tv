#!/usr/bin/env python3
"""Explicitly arm one G06 cold boot, upload the MI candidate, and capture logs."""
import argparse
import datetime
import hashlib
import http.client
import json
from pathlib import Path
import subprocess
import time

from watch import request

ROOT = Path(__file__).resolve().parents[2]
SERIAL = "4C0490290FA05A0C0"
LAUNCHER = "3a4dd344956269df0b7fc317683c13ddf8000654468d265019609c3cf713af9a"


def adb(*args):
    return subprocess.check_output(["adb", "-s", SERIAL, *args], timeout=15).decode().strip()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--execute", action="store_true", required=True)
    args = parser.parse_args()
    binary = ROOT / "assets/build/pid1-mi/target/armv7-unknown-linux-musleabihf/release/hello-pid1"
    manifest = json.loads((ROOT / "assets/build/pid1-mi/manifest.json").read_text())
    program = binary.read_bytes()
    assert hashlib.sha256(program).hexdigest() == manifest["init_sha256"]
    assert len(program) <= 8 * 1024 * 1024
    qemu = list((ROOT / "assets/build/hello-pid1/qemu").glob("g06-*/success/fidelity.json"))
    assert any(json.loads(p.read_text())["payload_sha256"] == manifest["init_sha256"]
               and "reboot: Restarting system" in p.with_name("serial.log").read_text()
               and "Kernel panic" not in p.with_name("serial.log").read_text()
               and p.with_name("exec-2.json").exists()
               for p in qemu), "candidate has no completed QEMU success run"
    for key, expected in [("ro.product.device", "G06"), ("ro.software.version_id", "V8-T221T04-LF1V109"),
                          ("sys.boot_completed", "1"), ("sys.tvroot.ready", "1"),
                          ("ro.boot.tv_launcher", LAUNCHER)]:
        assert adb("shell", "getprop " + key) == expected, key
    assert "pending: false" in adb("shell", '/debug_ramdisk/su -c "/data/local/tmp/tv-launcherctl --inspect"')
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S-miZ")
    output = ROOT / "assets/research/hello-pid1" / stamp
    output.mkdir(parents=True)
    (output / "candidate.json").write_text(json.dumps(manifest, indent=2) + "\n")
    armed = adb("shell", '/debug_ramdisk/su -c "/data/local/tmp/tv-launcherctl --arm"')
    (output / "arm.txt").write_text(armed + "\n")
    print(armed, flush=True)
    adb("reboot")
    print(f"Waiting for one-shot; evidence: {output}", flush=True)
    deadline, previous, uploaded, captured, started, visible = time.monotonic() + 240, None, False, False, None, None
    while time.monotonic() < deadline:
        try:
            status = json.loads(request("GET", "/status"))
            assert status["app"] == "hello-pid1" and status["pid"] == 1
            with (output / "status.jsonl").open("a") as stream:
                stream.write(json.dumps(status) + "\n")
            if status["phase"] != previous:
                print(status, flush=True)
                previous = status["phase"]
            if not uploaded:
                if status["phase"] not in ("debug-ready", "display-delay") or status["uptime_ms"] > 2500:
                    print(request("POST", "/reboot").decode(), flush=True)
                    raise RuntimeError("Missed pre-module upload window; requested Android return")
                print(request("POST", "/exec", program, timeout=3).decode(), flush=True)
                uploaded, started = True, time.monotonic()
                time.sleep(.2)
                continue
            if status["build"] != manifest["build"]:
                time.sleep(.1)
                continue
            (output / "logs.txt").write_bytes(request("GET", "/logs"))
            (output / "kernel.log").write_bytes(request("GET", "/kernel-log"))
            captured = True
            if status["phase"] == "hello-visible" and visible is None:
                visible = time.monotonic()
                print("Framebuffer draw completed; observe the TV now", flush=True)
            if status["phase"] == "display-failed" or (visible and time.monotonic() - visible >= 20) or time.monotonic() - started >= 30:
                print(request("POST", "/reboot").decode(), flush=True)
                print(f"Evidence retained: {output}", flush=True)
                return
        except (OSError, http.client.HTTPException, json.JSONDecodeError) as error:
            if captured:
                print(f"Native connection ended: {error}; evidence retained: {output}", flush=True)
                return
        time.sleep(.15 if not uploaded else .6)
    raise RuntimeError("No candidate response before deadline; check Android/standby")


if __name__ == "__main__":
    main()
