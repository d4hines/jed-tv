#!/usr/bin/env python3
"""Bounded hello-world display test with stock Android/ADB and saved diagnostics."""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys

SERIAL = "4C0490290FA05A0C0"
REMOTE = "/data/local/tmp/tv-hello/hello.jar"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("jar", type=Path)
    parser.add_argument("--seconds", type=int, default=20)
    parser.add_argument("--inspect", action="store_true", help="list display APIs; do not create a surface")
    parser.add_argument("--stop", action="store_true", help="stop only the shell-owned tv.hello helper")
    args = parser.parse_args()
    if not 1 <= args.seconds <= 120:
        parser.error("--seconds must be 1..120")
    adb = ["adb", "-s", SERIAL]

    def run(*words, check=True):
        result = subprocess.run(adb + list(words), stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=30)
        if check and result.returncode:
            raise RuntimeError(result.stderr.decode(errors="replace") + result.stdout.decode(errors="replace"))
        return result

    identity = run("shell", "getprop ro.serialno; getprop ro.product.device; getprop ro.software.version_id; getprop sys.boot_completed").stdout.decode().strip().splitlines()
    if identity != [SERIAL, "G06", "V8-T221T04-LF1V109", "1"]:
        raise RuntimeError("Unexpected device or Android not ready: " + repr(identity))
    existing = run("shell", "pidof tv.hello", check=False).stdout.decode().split()

    def stop(pids):
        for pid in pids:
            if not re.fullmatch(r"[0-9]+", pid):
                raise RuntimeError("Invalid helper PID")
            # Verify the name again; do not signal a recycled or unrelated PID.
            current = run("shell", "pidof tv.hello", check=False).stdout.decode().split()
            if pid in current:
                run("shell", "kill -TERM " + pid, check=False)

    if args.stop:
        stop(existing)
        print("Hello helper stopped; stock Android and ADB remain running.")
        return
    if existing:
        raise RuntimeError("Hello already running; use ./hello.sh --stop first")
    os.umask(0o077)
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    output = Path("assets/research/hello") / stamp
    output.mkdir(parents=True)
    (output / "session.json").write_text(json.dumps({"serial": SERIAL, "identity": identity,
        "jar_sha256": hashlib.sha256(args.jar.read_bytes()).hexdigest(), "seconds": args.seconds,
        "inspect": args.inspect, "firmware_writes": False}, indent=2) + "\n")
    run("shell", "mkdir -p /data/local/tmp/tv-hello && chmod 0700 /data/local/tmp/tv-hello")
    run("push", str(args.jar), REMOTE)
    run("shell", "chmod 0600 " + REMOTE)
    print("Evidence:", output, flush=True)
    command = f"CLASSPATH={REMOTE} app_process32 /system/bin --nice-name=tv.hello HelloScreen "
    command += "--inspect" if args.inspect else str(args.seconds)
    pid = None
    with (output / "logcat.txt").open("wb") as log, (output / "stdout.txt").open("w") as transcript:
        logger = subprocess.Popen(adb + ["logcat", "-v", "threadtime", "-T", "1", "-s", "TVHello:I", "*:S"],
                                  stdout=log, stderr=subprocess.STDOUT)
        process = subprocess.Popen(adb + ["shell", command], stdout=subprocess.PIPE,
                                   stderr=subprocess.STDOUT, text=True)
        try:
            for line in process.stdout:
                transcript.write(line)
                transcript.flush()
                print(line, end="", flush=True)
                match = re.search(r"TVHELLO pid=([0-9]+)", line)
                if match:
                    pid = match[1]
                if "TVHELLO phase=visible" in line:
                    health = run("shell", "getprop sys.boot_completed; getprop ro.software.version_id; id").stdout
                    (output / "adb-while-visible.txt").write_bytes(health)
                    if not health.decode().startswith("1\nV8-T221T04-LF1V109\n"):
                        raise RuntimeError("ADB health check failed while overlay visible")
                    print("ADB verified while overlay is visible.", flush=True)
                    screenshot = run("exec-out", "screencap", "-p").stdout
                    offset = screenshot.find(b"\x89PNG\r\n\x1a\n")
                    if offset < 0:
                        raise RuntimeError("Screenshot did not contain a PNG")
                    (output / "screen.png").write_bytes(screenshot[offset:])
            if process.wait(timeout=10):
                raise RuntimeError("Display helper failed; see " + str(output / "stdout.txt"))
        finally:
            if pid:
                stop([pid])
            if process.poll() is None:
                process.terminate()
                process.wait(timeout=5)
            logger.terminate()
            try:
                logger.wait(timeout=5)
            except subprocess.TimeoutExpired:
                logger.kill()
                logger.wait()


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("\nHello stopped.")
    except (OSError, RuntimeError, subprocess.TimeoutExpired) as error:
        sys.exit("STOP: " + str(error))
