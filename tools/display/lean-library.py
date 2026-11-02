#!/usr/bin/env python3
"""Build the scalar Lean display archive for a standalone Rust caller."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", choices=["armv7-unknown-linux-musleabihf", "armv7-linux-androideabi"],
                        default="armv7-unknown-linux-musleabihf")
    args = parser.parse_args()
    build = ROOT / "assets/build/display-library"
    env = dict(os.environ, CARGO_TARGET_DIR=str(build))
    env.setdefault("ZIG_GLOBAL_CACHE_DIR", str(build / "zig-global"))
    env.setdefault("ZIG_LOCAL_CACHE_DIR", str(build / "zig-local"))
    if env.get("TV_RUSTC"):
        env["RUSTC"] = env["TV_RUSTC"]
        cargo = [str(Path(env["TV_RUSTC"]).with_name("cargo"))]
    else:
        cargo = ["cargo", "+1.92.0"]
    result = subprocess.run([*cargo, "build", "--offline", "--locked", "--release", "--lib",
                             "--target", args.target, "--manifest-path", str(ROOT / "src/pid1/display/Cargo.toml"),
                             "--message-format=json-render-diagnostics"],
                            cwd=ROOT, env=env, text=True, stdout=subprocess.PIPE, stderr=sys.stderr)
    if result.returncode:
        sys.stderr.write(result.stdout)
        raise SystemExit(result.returncode)
    for line in result.stdout.splitlines():
        try:
            item = json.loads(line)
        except json.JSONDecodeError:
            continue
        if item.get("reason") == "build-script-executed" and "g06-display" in item.get("package_id", ""):
            archive = Path(item["out_dir"]) / "libtv_display_lean.a"
            if archive.is_file():
                print(archive)
                return
    raise SystemExit("Cargo did not report the Lean display archive")


if __name__ == "__main__":
    main()
