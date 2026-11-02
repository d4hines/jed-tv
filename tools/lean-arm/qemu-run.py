#!/usr/bin/env python3
"""Run a static ARM32 Lean test program as PID 1 under QEMU and capture serial.

  python3 tools/lean-arm/qemu-run.py BINARY [--file HOST:/guest] [--arg WORD]
      [--expect TEXT] [--timeout SEC] [--log PATH]

The binary is /init in a fresh initramfs (tools/mkinitramfs.py) on the virt32
surrogate kernel (Linux 6.18, cortex-a55 in AArch32, TCG). Program arguments go
after `--` on the kernel command line. Exit status 0 when TEXT appears in the
serial log before the timeout. TCG timing is not hardware timing.
"""
import argparse, pathlib, subprocess, sys, tempfile, time

ROOT = pathlib.Path(__file__).resolve().parents[2]
KERNEL = ROOT / "assets/build/kernel/arm/zImage"

ap = argparse.ArgumentParser()
ap.add_argument("binary")
ap.add_argument("--file", action="append", default=[])
ap.add_argument("--arg", action="append", default=[])
ap.add_argument("--expect", default=None)
ap.add_argument("--timeout", type=float, default=300)
ap.add_argument("--memory", default="1024")
ap.add_argument("--smp", default="4")
ap.add_argument("--log", type=pathlib.Path)
ap.add_argument("--arch", default="armv7", choices=["armv7", "aarch64"],
                help="aarch64 runs the arm64 kernel under TCG too, for 32- vs 64-bit comparisons")
a = ap.parse_args()

with tempfile.TemporaryDirectory() as tmp:
    cpio = pathlib.Path(tmp) / "initramfs.cpio"
    subprocess.run([sys.executable, str(ROOT / "tools/mkinitramfs.py"), a.binary, str(cpio), *a.file], check=True)
    log = a.log or pathlib.Path(tmp) / "serial.log"
    log.parent.mkdir(parents=True, exist_ok=True)
    append = "console=ttyAMA0 panic=-1 quiet" + (" -- " + " ".join(a.arg) if a.arg else "")
    machine = (["-M", "virt-11.1,highmem=off,gic-version=2", "-cpu", "cortex-a55,aarch64=off"]
               if a.arch == "armv7" else ["-M", "virt-11.1", "-cpu", "cortex-a55"])
    kernel = KERNEL if a.arch == "armv7" else ROOT / "assets/build/kernel/arm64/Image"
    cmd = ["qemu-system-aarch64", *machine, "-smp", a.smp, "-m", a.memory,
           "-kernel", str(kernel), "-initrd", str(cpio), "-append", append,
           "-nographic", "-no-reboot", "-serial", f"file:{log}", "-monitor", "none"]
    t0 = time.monotonic()
    q = subprocess.Popen(cmd, stdout=subprocess.DEVNULL, stderr=subprocess.PIPE)
    ok = False
    while time.monotonic() - t0 < a.timeout and q.poll() is None:
        time.sleep(0.5)
        text = log.read_text(errors="replace") if log.exists() else ""
        if a.expect and a.expect in text:
            ok = True
            time.sleep(0.5)
            break
    q.kill()
    q.wait()
    text = log.read_text(errors="replace") if log.exists() else ""
    sys.stdout.write(text)
    print(f"\nqemu-run: {'found' if ok else 'did not find'} {a.expect!r} after {time.monotonic() - t0:.1f}s")
    sys.exit(0 if ok or not a.expect else 1)
