#!/usr/bin/env python3
"""Trial of the Lean direct MI audio driver on the G13, from the installed
native session (stock init stays PID 1).

  python3 tools/audio/g13-direct.py build      # static ARM32 direct-audio binary
  python3 tools/audio/g13-direct.py start      # stage, quiesce Android audio, run
  python3 tools/audio/g13-direct.py log        # the driver's log so far
  python3 tools/audio/g13-direct.py restore    # stop the driver, resume Android audio

Gate before `start`: tools/audio/mi3-audio-requests.py on the G13's libmi3.so
must report every driver request identical to the G06's. The driver refuses
to run unless every media owner is stopped and the TV serial matches.
`start` SIGSTOPs audioserver and the audio HAL (it never kills them) and
terminates only our own AAudio bridge; `restore` resumes them. The global
gain file stays the installed one, so the level is the one already approved.
"""
import json
import os
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/native-app"))
import g13_device  # noqa: E402

TARGET = "armv7-unknown-linux-musleabihf"
BASE = "/data/local/tmp/dande-g13"
OUT = ROOT / "assets/build/g13-direct-audio"
PORT = 8877
STOPPED = OUT / "stopped.json"


def build():
    OUT.mkdir(parents=True, exist_ok=True)
    runtime = Path(os.environ.get("LEAN_ARM") or ROOT / "assets/build/lean-arm" / TARGET)
    if not (runtime / "libleanrt.a").is_file():
        subprocess.run([str(ROOT / "tools/lean-arm/build.sh"), TARGET, str(runtime)], cwd=ROOT, check=True)
    env = dict(os.environ, LEAN_ARM=str(runtime))
    env.setdefault("ZIG_GLOBAL_CACHE_DIR", str(ROOT / "assets/build/zig-global"))
    env.setdefault("ZIG_LOCAL_CACHE_DIR", str(OUT / "zig-local"))
    subprocess.run(["cargo", "build", "--locked", "--offline", "--release", "-p", "mi-audio", "--features", "lean",
                    "--bin", "direct-audio", "--target", TARGET, "--target-dir", str(OUT / "target")],
                   cwd=ROOT / "src", env=env, check=True)
    binary = OUT / "target" / TARGET / "release/direct-audio"
    print(binary, binary.stat().st_size, "bytes")
    return binary


def owners():
    """PIDs of Android's audio owners on the TV: audioserver and the audio HAL."""
    listing = g13_device.shell(
        "for p in /proc/[0-9]*; do c=$(tr '\\0' ' ' < $p/cmdline 2>/dev/null) || continue; "
        "case \"$c\" in *audioserver*|*hardware.audio*) echo \"${p#/proc/} $c\";; esac; done; true")
    return [(int(line.split()[0]), line.split(None, 1)[1]) for line in listing.splitlines() if line.strip()]


def start():
    binary = OUT / "target" / TARGET / "release/direct-audio"
    if not binary.is_file():
        binary = build()
    g13_device.root()
    g13_device.adb("push", binary, f"{BASE}/direct-audio", capture_output=True)
    g13_device.shell(f"chmod 755 {BASE}/direct-audio; restorecon {BASE}/direct-audio || true")
    # Our own AAudio bridge must go: it owns a stream on the same port.
    g13_device.shell(f"for pid in $(pidof native-audio); do kill -TERM $pid || true; done; true")
    time.sleep(1)
    stopped = owners()
    for pid, cmdline in stopped:
        print(f"SIGSTOP {pid}: {cmdline}")
        g13_device.shell(f"kill -STOP {pid}")
    STOPPED.write_text(json.dumps(stopped, indent=2) + "\n")
    g13_device.shell(
        f"export TV_RUNTIME_DIR={BASE}/root/tmp; "
        f"nohup {BASE}/direct-audio --serial {g13_device.SERIAL} --port {PORT} > {BASE}/logs/direct-audio.log 2>&1 &")
    time.sleep(2)
    log()


def log():
    print(g13_device.shell(f"cat {BASE}/logs/direct-audio.log 2>/dev/null || true"))


def restore():
    g13_device.root()
    g13_device.shell("for pid in $(pidof direct-audio); do kill -TERM $pid || true; done; true")
    stopped = json.loads(STOPPED.read_text()) if STOPPED.is_file() else owners()
    for pid, cmdline in stopped:
        print(f"SIGCONT {pid}: {cmdline}")
        g13_device.shell(f"kill -CONT {pid} || true")
    print("Android audio resumed. Restart the native session (tools/native-app/run-g13.py) to bring the AAudio bridge back.")


def main():
    command = sys.argv[1] if len(sys.argv) > 1 else "--help"
    actions = {"build": build, "start": start, "log": log, "restore": restore}
    if command not in actions:
        sys.exit(__doc__)
    actions[command]()


if __name__ == "__main__":
    main()
