#!/usr/bin/env python3
"""One-shot boot our native PID 1 plus a persistent shell, then leave it running."""
import argparse
import datetime
import hashlib
import http.client
import json
from pathlib import Path
import socket
import subprocess
import time

from watch import request
from direct_link import connect

ROOT = Path(__file__).resolve().parents[2]
SERIAL = "4C0490290FA05A0C0"
ADB_SERIAL = SERIAL


def adb(command):
    return subprocess.check_output(["adb", "-s", ADB_SERIAL, "shell", command], timeout=15).decode().strip()


def console():
    return connect(8766, 2)


def receive(connection):
    text = bytearray(); deadline = time.monotonic() + 10
    while b"tv-repl# " not in text:
        if time.monotonic() > deadline: raise RuntimeError(f"console timeout: {text!r}")
        try: data = connection.recv(65536)
        except socket.timeout: continue
        if not data: raise RuntimeError(f"console closed: {text!r}")
        text.extend(data)
    return text.decode(errors="replace")


def main():
    global ADB_SERIAL
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--execute", action="store_true", required=True)
    parser.add_argument("--serial", default=SERIAL, help="ADB transport; device identity is still checked separately")
    args = parser.parse_args()
    ADB_SERIAL = args.serial
    manifest = json.loads((ROOT / "assets/build/pid1-repl/manifest.json").read_text())
    binary = (ROOT / "assets/build/pid1-repl/target/armv7-unknown-linux-musleabihf/release/hello-pid1").read_bytes()
    assert hashlib.sha256(binary).hexdigest() == manifest["init_sha256"]
    results = list((ROOT / "assets/build/pid1-repl/qemu").glob("*/result.json"))
    assert any(json.loads(p.read_text()).get("passed") and json.loads(p.read_text()).get("init_sha256") == manifest["init_sha256"] for p in results), "Run the REPL QEMU test first"
    watchdog = json.loads((ROOT / "assets/build/pid1-repl/watchdog-check.json").read_text())
    assert watchdog["passed"] and watchdog["writes"] == [[0x1f006000, 1, 1]]
    for key, expected in [("ro.serialno", SERIAL), ("ro.product.device", "G06"), ("ro.software.version_id", "V8-T221T04-LF1V109"),
                          ("sys.boot_completed", "1"), ("sys.tvroot.ready", "1"),
                          ("ro.boot.tv_launcher", "3a4dd344956269df0b7fc317683c13ddf8000654468d265019609c3cf713af9a")]:
        assert adb("getprop " + key) == expected, key
    assert "pending: false" in adb('/debug_ramdisk/su -c "/data/local/tmp/tv-launcherctl --inspect"')
    output = ROOT / "assets/research/pid1-repl" / datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    output.mkdir(parents=True)
    (output / "candidate.json").write_text(json.dumps(manifest, indent=2) + "\n")
    arm = adb('/debug_ramdisk/su -c "/data/local/tmp/tv-launcherctl --arm"')
    (output / "arm.txt").write_text(arm + "\n"); print(arm, flush=True)
    subprocess.run(["adb", "-s", ADB_SERIAL, "reboot"], check=True, timeout=15)
    print(f"Waiting for native PID1; evidence {output}", flush=True)
    deadline = time.monotonic() + 240; uploaded = False; previous = None
    while time.monotonic() < deadline:
        try:
            status = json.loads(request("GET", "/status"))
            assert status["app"] == "hello-pid1" and status["pid"] == 1
            if not uploaded:
                if status["phase"] not in ("debug-ready", "display-delay") or status["uptime_ms"] > 2500:
                    request("POST", "/reboot")
                    raise RuntimeError("Missed pre-module upload window; returning to Android")
                print(request("POST", "/exec", binary, timeout=3).decode(), flush=True)
                uploaded = True
            elif status["build"] == manifest["build"]:
                with (output / "startup-status.jsonl").open("a") as stream: stream.write(json.dumps(status) + "\n")
                (output / "events.log").write_bytes(request("GET", "/logs"))
                (output / "kernel.log").write_bytes(request("GET", "/kernel-log"))
                if status["phase"] != previous:
                    print(status, flush=True); previous = status["phase"]
                if status["phase"] == "shell-failed":
                    raise RuntimeError(f"REPL startup failed; logs retained at {output}; automatic Android return requested")
                if status["phase"] == "shell-ready": break
        except (OSError, http.client.HTTPException, json.JSONDecodeError): pass
        time.sleep(.15)
    else: raise RuntimeError("Native shell did not appear; inspect Android/standby")
    assert b"no automatic deadline" in request("GET", "/logs"), "Native shell unexpectedly has a reboot deadline"
    print("Native PID1 console is ready: python3 tools/pid1/repl.py", flush=True)
    with console() as shell, (output / "console.log").open("w") as transcript:
        transcript.write(receive(shell))
        def command(text):
            shell.sendall(text.encode() + b"\n")
            result = receive(shell); transcript.write(result); transcript.flush(); return result
        proof = command("id; readlink /proc/1/exe; sha256sum /proc/1/exe; cat /proc/sys/kernel/random/boot_id; ps")
        assert "uid=0" in proof and manifest["init_sha256"] in proof and "/tmp/.hello-next" in proof
        command("cd /tmp; export REPL_PROOF=kept")
        assert "STATE:/tmp:kept" in command('printf "STATE:%s:%s\\n" "$PWD" "$REPL_PROOF"')
        command("echo ANDROID_PIDS=$(pidof adbd zygote surfaceflinger)")
        print("Root shell and persistent cwd/environment verified; checking 90 seconds of uptime", flush=True)
        start = time.monotonic()
        while time.monotonic() - start < 90:
            status = json.loads(request("GET", "/status"))
            assert status["build"] == manifest["build"] and status["pid"] == 1 and status["phase"] == "shell-ready"
            with (output / "status.jsonl").open("a") as stream: stream.write(json.dumps(status) + "\n")
            (output / "events.log").write_bytes(request("GET", "/logs"))
            (output / "kernel.log").write_bytes(request("GET", "/kernel-log"))
            time.sleep(5)
        assert "STILL:/tmp:kept" in command('printf "STILL:%s:%s\\n" "$PWD" "$REPL_PROOF"')
        command("cat /proc/sys/kernel/random/boot_id; cat /proc/uptime")
        shell.sendall(b"exit\n")
    (output / "result.json").write_text(json.dumps({"passed": True, "native_pid1": True, "root_shell": True,
        "persistent_shell_state": True, "uptime_test_seconds": 90, "left_running": True,
        "init_sha256": manifest["init_sha256"]}, indent=2) + "\n")
    print("PASS: native PID1 + root REPL remained alive past 90 seconds; left running", flush=True)
    print("Connect: python3 tools/pid1/repl.py ; return to Android: python3 tools/pid1/watch.py --reboot", flush=True)


if __name__ == "__main__":
    main()
