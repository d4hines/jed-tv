#!/usr/bin/env python3
"""Offline Linux permission/descriptor test; does not emulate TEE hardware."""
import argparse
import hashlib
import json
import lzma
from pathlib import Path
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("archive", type=Path, help="ARM32 libtv_control_lean.a from the build being tested")
    args = parser.parse_args()
    archive = args.archive.resolve()
    assert archive.name == "libtv_control_lean.a" and archive.is_file()
    out = ROOT / "assets/build/tee-boundary-qemu" / str(time.time_ns())
    out.mkdir(parents=True)
    common = ["rustc", "+1.92.0", "--edition=2021", "--target", "armv7-unknown-linux-musleabihf",
              "-C", "linker=rust-lld", "-C", "target-cpu=cortex-a55", "-C", "panic=abort", "-C", "opt-level=2"]
    subprocess.run([*common, "test/fixtures/tvinit-child.rs", "-o", str(out / "child")], cwd=ROOT, check=True)
    subprocess.run([*common, "-L", f"native={archive.parent}", "-l", "static=tv_control_lean",
                    "test/fixtures/confinement-init.rs", "-o", str(out / "init")], cwd=ROOT, check=True)
    subprocess.run([sys.executable, str(ROOT / "tools/mkinitramfs.py"), str(out / "init"),
                    str(out / "initramfs.cpio"), f"{out}/child:/bin/ytplay", f"{out}/child:/bin/slides"], check=True)
    kernel = out / "zImage"
    kernel.write_bytes(lzma.decompress((ROOT / "demo/kernel/zImage.xz").read_bytes()))
    command = ["qemu-system-arm", "-M", "virt,highmem=off", "-cpu", "max", "-smp", "4", "-m", "512M",
               "-kernel", str(kernel), "-initrd", str(out / "initramfs.cpio"),
               "-append", "console=ttyAMA0 rdinit=/init panic=-1", "-display", "none",
               "-monitor", "none", "-serial", "stdio", "-no-reboot"]
    passed = False
    with (out / "serial.log").open("w") as log:
        process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT)
        try:
            deadline = time.monotonic() + 45
            while time.monotonic() < deadline:
                text = (out / "serial.log").read_text(errors="replace")
                if "G06-CONFINEMENT PASS" in text:
                    assert "fixture: TEE boundary PASS" in text
                    passed = True
                    break
                if process.poll() is not None:
                    raise RuntimeError(text[-4000:])
                time.sleep(.2)
            if not passed:
                raise RuntimeError(text[-4000:])
        finally:
            process.terminate()
            process.wait(timeout=5)
            (out / "result.json").write_text(json.dumps({
                "passed": passed, "real_tee_hardware": False,
                "archive_sha256": hashlib.sha256(archive.read_bytes()).hexdigest(),
                "kernel_sha256": hashlib.sha256(kernel.read_bytes()).hexdigest(),
                "checks": ["root-only nodes/storage", "inherited descriptor closed",
                           "unprivileged UID", "process/exec denial", "TERM/KILL"],
            }, indent=2) + "\n")
    print(f"PASS: Linux TEE access boundary; evidence {out}")


if __name__ == "__main__":
    main()
