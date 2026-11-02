#!/usr/bin/env python3
"""Stage the stock display runtime under the G06 native PID-1 shell."""
import argparse
import json
from pathlib import Path
import subprocess
import sys

from watch import request

ROOT = Path(__file__).resolve().parents[2]


def upload(path, name=None):
    data = path.read_bytes()
    if len(data) > 8 * 1024 * 1024:
        raise RuntimeError(f"File exceeds upload limit: {path}")
    print(request("PUT", "/files/" + (name or path.name), data, timeout=10).decode(), end="")


def console(command):
    wrapped = command + '; status=$?; echo GRAPHICS_EXIT=$status'
    result = subprocess.run([sys.executable, str(ROOT / "tools/pid1/repl.py"), "-c", wrapped], cwd=ROOT, check=True, stdout=subprocess.PIPE)
    output = result.stdout.decode(errors="replace").replace("\r", "")
    print(output, end="", flush=True)
    if "\nGRAPHICS_EXIT=0\n" not in output:
        raise RuntimeError("TV graphics command failed; see console output")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--start", action="store_true", help="initialize graphics on a fresh native-shell boot")
    parser.add_argument("--seconds", type=int, default=120)
    args = parser.parse_args()
    if not 1 <= args.seconds <= 120:
        parser.error("--seconds must be 1..120")
    status = json.loads(request("GET", "/status"))
    if status.get("pid") != 1 or status.get("phase") != "shell-ready":
        parser.error("Expected the native PID-1 shell")
    jar = subprocess.check_output(["nix", "build", ".#hello-screen", "--no-link", "--print-out-paths"], cwd=ROOT, text=True).strip()
    upload(Path(jar) / "hello.jar")
    upload(ROOT / "research/hello/hello-native.sh")
    if args.start:
        for name in ["tv-binder", "tv-bootlogo"]:
            artifact = ROOT / "assets/build/native-tools/armv7-unknown-linux-musleabihf/release" / name
            if not artifact.is_file():
                raise RuntimeError(f"Build native helpers first: {artifact}")
            upload(artifact)
        upload(ROOT / "assets/research/native-vendor-stack/android-properties.tar.gz")
        subprocess.run([sys.executable, str(ROOT / "tools/pid1/java-linker-config.py")], cwd=ROOT, check=True)
        upload(ROOT / "assets/research/native-vendor-stack/java-ld.config.txt")
        for name in ["src/pid1/scripts/mount-stock-runtime.sh",
                     "research/probes/start-vendor-display.sh",
                     "research/hello/prepare-java-runtime.sh"]:
            upload(ROOT / name)
        console("sh /tmp/start-vendor-display.sh && sh /tmp/prepare-java-runtime.sh")
    console(f"/tmp/tv-bootlogo && sh /tmp/hello-native.sh {args.seconds}")


if __name__ == "__main__":
    main()
