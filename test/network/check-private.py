#!/usr/bin/env python3
"""Run the actual root-only network installer under ARM32 QEMU; no TV access."""
import argparse
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets/build/network-private-check"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--kernel", type=Path, default=ROOT / "assets/build/app-api-kernel/zImage")
    parser.add_argument("--busybox", type=Path, default=ROOT / "assets/build/pid1-repl/busybox")
    parser.add_argument("--qemu", default=shutil.which("qemu-system-arm"))
    args = parser.parse_args()
    if not args.qemu or not args.kernel.is_file() or not args.busybox.is_file():
        parser.error("supply ARM QEMU, kernel and static BusyBox paths")
    OUT.mkdir(parents=True, exist_ok=True)
    binary = OUT / "check"
    subprocess.run(["rustc", "+1.92.0", "--edition=2021", "--target", "armv7-unknown-linux-musleabihf",
                    "-C", "linker=rust-lld", "-C", "panic=abort", "-O", "test/network/private-boundary.rs",
                    "-o", str(binary)], cwd=ROOT, check=True)
    image = OUT / "init.cpio"
    subprocess.run(["python3", "tools/mkinitramfs.py", str(binary), str(image),
                    f"{args.busybox}:/bin/busybox",
                    f"{ROOT / 'src/pid1/scripts/network/install.sh'}:/tmp/network-install.sh"], cwd=ROOT, check=True)
    result = subprocess.run([args.qemu, "-M", "virt", "-cpu", "max", "-m", "128", "-nographic", "-no-reboot",
                             "-kernel", str(args.kernel), "-initrd", str(image), "-append",
                             "console=ttyAMA0 panic=-1 androidboot.serialno=4C0490290FA05A0C0"],
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=30)
    text = result.stdout.decode(errors="replace")
    (OUT / "serial.log").write_text(text)
    if "PASS: production network installer" not in text or "exitcode=0x00000000" not in text:
        raise SystemExit(text[-10000:])
    print(text[text.index("PASS: production network installer"):].splitlines()[0])
    print(f"Evidence: {OUT / 'serial.log'}")


if __name__ == "__main__":
    main()
