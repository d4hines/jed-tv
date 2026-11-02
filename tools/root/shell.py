#!/usr/bin/env python3
"""Open a verified G06/V109 root shell while stock Android keeps running."""
import argparse
import os
import shlex
import subprocess
import sys
import time

SERIAL = "4C0490290FA05A0C0"
ADB = ["adb", "-s", SERIAL]


def query(command):
    result = subprocess.run(ADB + ["shell", command], capture_output=True, text=True, timeout=5)
    return result.stdout.strip() if result.returncode == 0 else None


def ready(seconds):
    deadline = time.monotonic() + seconds
    waiting = False
    while True:
        identity = query("getprop ro.serialno; getprop ro.product.device; getprop ro.software.version_id; getprop sys.boot_completed; getprop sys.tvroot.ready")
        fields = identity.splitlines() if identity else []
        if len(fields) >= 3 and fields[:3] != [SERIAL, "G06", "V8-T221T04-LF1V109"]:
            raise RuntimeError(f"Refusing unexpected device: {fields[:3]!r}")
        if fields == [SERIAL, "G06", "V8-T221T04-LF1V109", "1", "1"]:
            break
        if time.monotonic() >= deadline:
            raise RuntimeError("G06 Android/root is not ready. Press Power once if the TV is in standby, then retry.")
        if not waiting:
            print("Waiting for G06 Android/root; press Power once if it is in standby.", file=sys.stderr, flush=True)
            waiting = True
        time.sleep(1)
    su = query("for p in /debug_ramdisk/su /sbin/su; do if test -x \"$p\"; then echo \"$p\"; break; fi; done")
    if su not in ("/debug_ramdisk/su", "/sbin/su"):
        raise RuntimeError("Expected Magisk su was not found")
    proof = query(f"{shlex.quote(su)} -c {shlex.quote('id -u; cat /proc/self/attr/current')}")
    if not proof or proof.splitlines()[0] != "0":
        raise RuntimeError(f"Root verification failed: {proof!r}")
    return su, proof.replace("\x00", "").splitlines()[-1]


def main():
    parser = argparse.ArgumentParser(description=__doc__, epilog="Examples: ./shell.sh ; ./shell.sh id ; ./shell.sh sh -c 'id; getprop sys.boot_completed'")
    parser.add_argument("--check", action="store_true", help="verify readiness/root without opening a shell")
    parser.add_argument("--wait", type=int, default=60, metavar="SECONDS", help="wait for Android/root (default 60; use 0 for one check)")
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    if not 0 <= args.wait <= 300:
        parser.error("--wait must be 0..300")
    su, domain = ready(args.wait)
    if args.check:
        print(f"G06/V109: Android ready; root uid=0; domain={domain}")
        return
    command = shlex.join(args.command) if args.command else "cd /data/local/tmp && exec /system/bin/sh"
    remote = f"exec {shlex.quote(su)} -c {shlex.quote(command)}"
    tty = ["-t"] if not args.command and sys.stdin.isatty() else []
    os.execvp("adb", ADB + ["shell", *tty, remote])


if __name__ == "__main__":
    try:
        main()
    except (OSError, RuntimeError, subprocess.TimeoutExpired) as error:
        sys.exit(f"STOP: {error}")
