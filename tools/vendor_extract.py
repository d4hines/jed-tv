#!/usr/bin/env python3
"""Copy files out of the V109 vendor partition image (ext4) with debugfs,
without mounting. Read-only on the image.
  python3 tools/vendor_extract.py OUTDIR /usr/keylayout/X.kl [/path/in/vendor ...]
  python3 tools/vendor_extract.py --ls /usr/keylayout
"""
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
VENDOR = ROOT / "assets/research/tv-pid1/v109/super/vendor.img"


def debugfs(req):
    return subprocess.run(["nix", "shell", "nixpkgs#e2fsprogs", "-c", "debugfs", "-R", req,
                           str(VENDOR)], capture_output=True, text=True).stdout


def main():
    if sys.argv[1] == "--ls":
        print(debugfs(f"ls -l {sys.argv[2]}"))
        return
    out = pathlib.Path(sys.argv[1])
    out.mkdir(parents=True, exist_ok=True)
    for src in sys.argv[2:]:
        dst = out / pathlib.PurePosixPath(src).name
        debugfs(f"dump {src} {dst}")
        print(f"{src} -> {dst} ({dst.stat().st_size if dst.exists() else 'missing'} bytes)")


if __name__ == "__main__":
    main()
