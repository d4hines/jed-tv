#!/usr/bin/env python3
"""Test, arm and collect a one-shot pre-graphics Android boot trace on G06."""
import argparse
import datetime
import hashlib
from pathlib import Path
import shlex
import subprocess

ROOT = Path(__file__).resolve().parents[1]
SERIAL = "4C0490290FA05A0C0"
BASE = "/data/local/tmp/g06-early-trace"
HOOK = "/data/adb/post-fs-data.d/g06-early-trace.sh"


def adb(*args, **kwargs):
    return subprocess.run(["adb", "-s", SERIAL, *args], check=True, **kwargs)


def shell(command):
    return adb("shell", command, capture_output=True, text=True, timeout=20).stdout.strip()


def root(command):
    return shell("/debug_ramdisk/su -c " + shlex.quote(command))


def identity():
    for key, expected in [("ro.serialno", SERIAL), ("ro.product.device", "G06"),
                          ("ro.software.version_id", "V8-T221T04-LF1V109"),
                          ("sys.boot_completed", "1"), ("sys.tvroot.ready", "1")]:
        assert shell("getprop " + key) == expected, key
    assert "uid=0" in root("id")


def test_probe():
    identity()
    binary = ROOT / "assets/build/graphics-early-trace/probe"
    assert binary.exists(), "Build research/probes/graphics-trace-probe.S first"
    adb("push", str(binary), "/data/local/tmp/g06-trace-probe", stdout=subprocess.DEVNULL)
    command = r'''
set -e
chmod 755 /data/local/tmp/g06-trace-probe
mkdir -p /sys/kernel/tracing
mount -t tracefs tracefs /sys/kernel/tracing 2>/dev/null || true
T=/sys/kernel/tracing
I=$T/instances/g06_probe
mkdir "$I"
trap 'echo 0 > "$I/tracing_on"; echo 0 > "$I/events/enable"; echo "-:g06probe/args" >> "$T/uprobe_events"; rmdir "$I"' EXIT
echo 'p:g06probe/args /data/local/tmp/g06-trace-probe:0xd5 r0=%r0 r1=%r1 data=%r2 a0=+0(%r2):x32 a1=+4(%r2):x32' >> "$T/uprobe_events"
echo 1 > "$I/events/g06probe/args/enable"
echo 1 > "$I/tracing_on"
/data/local/tmp/g06-trace-probe
echo 0 > "$I/tracing_on"
cat "$I/trace"
'''
    result = root(command)
    output = ROOT / "assets/research/graphics-early-trace"
    output.mkdir(parents=True, exist_ok=True)
    (output / "probe-test.txt").write_text(result + "\n")
    assert "11223344" in result and "55667788" in result, result
    print("PASS: ARM Thumb uprobe executed and captured register/pointer arguments")


def install():
    test_probe()
    source = ROOT / "research/probes/graphics-early-trace.sh"
    assert root(f"if [ -e {HOOK} ]; then echo exists; fi") != "exists", "Trace hook already exists; inspect it first"
    assert "pending: false" in root("/data/local/tmp/tv-launcherctl --inspect")
    adb("push", str(source), "/data/local/tmp/g06-trace-stage.sh", stdout=subprocess.DEVNULL)
    print(root(f'''
set -e
mkdir -p {BASE} /data/adb/post-fs-data.d
chmod 700 {BASE}
cp /data/local/tmp/g06-trace-stage.sh {HOOK}
chmod 700 {HOOK}
echo one-shot-pre-graphics > {BASE}/armed
chmod 600 {BASE}/armed
sync
sha256sum {HOOK}
'''))
    print("Hook armed; it starts at Magisk post-fs-data and consumes its request before capture.")
    print("Next: adb -s " + SERIAL + " reboot")


def collect():
    identity()
    assert root(f"cat {BASE}/complete") == "complete", "Capture has not finished its 120-second window"
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    output = ROOT / "assets/research/graphics-early-trace" / stamp
    output.mkdir(parents=True)
    archive = output / "capture.tar"
    with archive.open("wb") as stream:
        adb("exec-out", "/debug_ramdisk/su", "-c", f"tar -C {BASE} -cf - .", stdout=stream, timeout=90)
    import tarfile
    with tarfile.open(archive) as tar:
        tar.extractall(output, filter="data")
    print(root(f"rm -f {HOOK}"))
    print("Collected", output)
    print("SHA256", hashlib.sha256(archive.read_bytes()).hexdigest())
    modules = (output / "modules-at-arm.txt").read_text()
    if any(line.split()[0] in {"mik", "utpa2k", "mstar_fbdev_mi"} for line in modules.splitlines() if line.split()):
        print("WARNING: vendor modules were already loaded when tracing began; this is not a complete early-startup capture")
    else:
        print("PASS: capture started before the vendor display modules were loaded")
    print("Inspect startup.log, probe markers and buffer-stats.txt before claiming complete initialization coverage.")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("action", choices=["test", "install", "collect"])
    args = parser.parse_args()
    {"test": test_probe, "install": install, "collect": collect}[args.action]()


if __name__ == "__main__":
    main()
