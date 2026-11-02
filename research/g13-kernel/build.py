#!/usr/bin/env python3
"""Build MT5867 source with the captured G13 configuration, inside shell.nix."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

sys.dont_write_bytecode = True
from prepare import ROOT, OUT, COMMIT, SHA256, main as prepare

CONFIG = ROOT / "assets/research/g13-kernel-source-20261101/live.config"
CONFIG_SHA = "90fcf9a9d0bc5d51da327392413b13fbd5ef3dcc4509346c67f2e00d361989aa"


def sha(path):
    return hashlib.file_digest(path.open("rb"), "sha256").hexdigest()


def config(path):
    values = {}
    for line in path.read_text().splitlines():
        if line.startswith("CONFIG_"):
            key, value = line.split("=", 1)
            values[key] = value
        elif line.startswith("# CONFIG_") and line.endswith(" is not set"):
            values[line[2:-11]] = "n"
    return values


def main():
    if not (OUT / "source").exists():
        prepare()
    provenance = json.loads((OUT / "source-provenance.json").read_text())
    assert provenance["commit"] == COMMIT and provenance["archive_sha256"] == SHA256
    patches = [Path(__file__).with_name(name) for name in
               ("sync-debug.patch", "fixdep.patch", "makefile-boot.patch")]
    for patch in patches:
        patch_command = ["patch", "--batch", "-p1", "-i", str(patch)]
        if subprocess.run(patch_command + ["--dry-run", "--forward"], cwd=OUT / "source", capture_output=True).returncode == 0:
            subprocess.run(patch_command + ["--forward"], cwd=OUT / "source", check=True)
        else:
            subprocess.run(patch_command + ["--dry-run", "--reverse"], cwd=OUT / "source", check=True)
    # A tarball inside the app checkout otherwise inherits the app's Git ID.
    # Label the compiled release with the actual pinned kernel source ID.
    (OUT / "source/.scmversion").write_text("-g" + COMMIT[:12] + "\n")
    output = OUT / "out"
    # The vendor config rule runs genlink.sh in O=, producing dangling
    # /mstar2 links there. Source links were already repaired by prepare.py.
    for name in ("mstar2", "mstar3party"):
        link = output / "drivers" / name
        if link.is_symlink() and str(link.readlink()) == "/" + name:
            link.unlink()
    make = ["make", "-C", str(OUT / "source"), "O=" + str(output),
            "ARCH=arm", "LLVM=1", "HOSTCC=cc", "HOSTCXX=c++", "NCT5_BUILD=true",
            "KERNEL_VER=linux-5.15_14"]
    assert sha(CONFIG) == CONFIG_SHA, "Unreviewed G13 configuration"
    if not (output / ".config").exists():
        output.mkdir(parents=True, exist_ok=True)
        (output / ".config").write_bytes(CONFIG.read_bytes())
    subprocess.run(make + ["olddefconfig"], check=True)
    subprocess.run(make + ["-j8", "Image", "modules", "dtbs"], check=True)
    paths = [output / "arch/arm/boot/Image", output / "vmlinux", output / ".config", output / "Module.symvers"]
    modules = sorted(output.rglob("*.ko"))
    before, after = config(CONFIG), config(output / ".config")
    symbols = subprocess.check_output(["llvm-nm", "--defined-only", str(output / "vmlinux")], text=True)
    entry = next(int(line.split()[0], 16) for line in symbols.splitlines() if line.split()[-1] == "stext")
    assert entry == int(after["CONFIG_PAGE_OFFSET"], 0) + 0x8000, "Stale or unexpected kernel link address"
    report = {"source": provenance, "patches": {patch.name: sha(patch) for patch in patches}, "seed_config_sha256": CONFIG_SHA,
              "entry_virtual_address": hex(entry),
              "config_changes": {key: [before.get(key), after.get(key)] for key in sorted(before.keys() | after.keys())
                                 if before.get(key) != after.get(key)},
              "toolchain": subprocess.check_output(["clang", "--version"], text=True),
              "release": (output / "include/config/kernel.release").read_text().strip(),
              "command": make + ["-j8", "Image", "modules", "dtbs"],
              "artifacts": {str(p.relative_to(OUT)): {"sha256": sha(p), "bytes": p.stat().st_size} for p in paths},
              "modules": {str(p.relative_to(output)): sha(p) for p in modules},
              "module_count": len(modules)}
    (OUT / "build.json").write_text(json.dumps(report, indent=2) + "\n")
    print("Built", report["release"], "with", len(modules), "modules")


if __name__ == "__main__":
    main()
