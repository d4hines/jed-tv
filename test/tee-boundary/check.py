#!/usr/bin/env python3
"""Compile the Lean proofs and run the production TEE parsers, without hardware."""
import argparse
import hashlib
import json
import lzma
import os
from pathlib import Path
import platform
import subprocess
import shutil
import time

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--arm", action="store_true", help="also run the actual adapter in ARM32 QEMU")
args = parser.parse_args()

ROOT = Path(__file__).resolve().parents[2]
PACKAGE = ROOT / "src/pid1/lean"
OUT = ROOT / "assets/build/tee-boundary-scalar"
OUT.mkdir(parents=True, exist_ok=True)

def run(args, **kwargs):
    return subprocess.run(args, check=True, **kwargs)

source = OUT / "TeeBoundary.c"
run(["lake", "env", "lean", "-c", str(source), "TV/Hardware/TeeBoundary.lean"], cwd=PACKAGE)
prefix = subprocess.check_output(["lean", "--print-prefix"], cwd=PACKAGE, text=True).strip()
env = os.environ.copy()
env.setdefault("ZIG_GLOBAL_CACHE_DIR", str(ROOT / "assets/build/zig-global"))
env.setdefault("ZIG_LOCAL_CACHE_DIR", str(OUT / "zig-local"))
cc = env.get("CC", "cc")
if platform.system() == "Darwin" and "CC" not in env:
    cc = "/Library/Developer/CommandLineTools/usr/bin/clang"
    env["SDKROOT"] = "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk"
obj = OUT / "TeeBoundary.o"
run([cc, "-O2", "-DNDEBUG", "-ffunction-sections", "-fdata-sections", "-I" + prefix + "/include",
     "-c", str(source), "-o", str(obj)], env=env)
exe = OUT / "tee-boundary-tests"
run(["rustc", "+1.92.0", "--edition=2021", "--test", "-O", str(ROOT / "test/tee-boundary/driver.rs"),
     "-C", "link-arg=" + str(obj), "-o", str(exe)])
run([str(exe), "--nocapture"])
imports = subprocess.check_output(["nm", "-u", str(exe)], text=True)
assert "lean_" not in imports, imports
print("PASS: scalar Lean TEE proofs and actual Rust adapter; no Lean runtime linked, no hardware access")

if args.arm:
    arm = OUT / "arm"
    arm.mkdir(exist_ok=True)
    obj = arm / "TeeBoundary.o"
    run(["zig", "cc", "-target", "arm-linux-musleabihf", "-mcpu=cortex_a55", "-O2", "-DNDEBUG",
         "-ffunction-sections", "-fdata-sections", "-I" + prefix + "/include",
         "-c", str(source), "-o", str(obj)], env=env)
    exe = arm / "tests"
    run(["rustc", "+1.92.0", "--edition=2021", "--test", "-O", "--target", "armv7-unknown-linux-musleabihf",
         "-C", "linker=rust-lld", "-C", "target-cpu=cortex-a55",
         str(ROOT / "test/tee-boundary/driver.rs"), "-C", "link-arg=" + str(obj), "-o", str(exe)])
    imports = subprocess.check_output(["nm", "-u", str(exe)], text=True)
    assert "lean_" not in imports, imports
    initramfs = arm / "initramfs.cpio"
    run(["python3", str(ROOT / "tools/mkinitramfs.py"), str(exe), str(initramfs)])
    kernel = arm / "zImage"
    kernel.write_bytes(lzma.decompress((ROOT / "demo/kernel/zImage.xz").read_bytes()))
    qemu = shutil.which("qemu-system-arm")
    if not qemu:
        candidates = sorted(Path("/nix/store").glob("*-qemu-*/bin/qemu-system-arm"))
        qemu = str(candidates[-1]) if candidates else None
    assert qemu, "qemu-system-arm required"
    log = arm / "serial.log"
    with log.open("w") as output:
        process = subprocess.Popen([qemu, "-M", "virt,highmem=off", "-cpu", "max", "-smp", "4", "-m", "512M",
            "-kernel", str(kernel), "-initrd", str(initramfs), "-append", "console=ttyAMA0 rdinit=/init panic=-1",
            "-display", "none", "-monitor", "none", "-serial", "stdio", "-no-reboot"], stdout=output, stderr=subprocess.STDOUT)
        try:
            deadline = time.monotonic() + 45
            while time.monotonic() < deadline:
                text = log.read_text(errors="replace")
                if "test result: ok. 6 passed; 0 failed" in text:
                    break
                if "test result: FAILED" in text or process.poll() is not None:
                    raise RuntimeError(text[-5000:])
                time.sleep(.1)
            else:
                raise RuntimeError(log.read_text(errors="replace")[-5000:])
        finally:
            if process.poll() is None:
                process.terminate()
            process.wait(timeout=5)
    (arm / "result.json").write_text(json.dumps({
        "passed": True, "tests": 6, "hardware_tested": False,
        "kernel_sha256": hashlib.sha256(kernel.read_bytes()).hexdigest(),
        "binary_sha256": hashlib.sha256(exe.read_bytes()).hexdigest(),
        "lean_source_sha256": hashlib.sha256((PACKAGE / "TV/Hardware/TeeBoundary.lean").read_bytes()).hexdigest()
    }, indent=2) + "\n")
    print("PASS: ARM32 actual TEE parser adapter, six tests; " + str(arm / "result.json"))
