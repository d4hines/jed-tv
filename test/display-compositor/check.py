#!/usr/bin/env python3
"""Compare the production Lean display loop with the old Rust oracle, offline."""
import argparse
import hashlib
import json
import lzma
import os
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets/build/display-compositor"


def run(args, env=None):
    result = subprocess.run(list(map(str, args)), cwd=ROOT, env=env, text=True,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    if result.returncode:
        raise RuntimeError(result.stdout)
    return result.stdout


def no_runtime(binary):
    symbols = run(["nm", binary])
    forbidden = []
    for line in symbols.splitlines():
        symbol = line.split()[-1] if line.split() else ""
        symbol = symbol.removeprefix("_")
        if symbol.startswith(("lean_", "initialize_")) or "___boxed" in symbol:
            forbidden.append(symbol)
    assert not forbidden, f"Unexpected allocator/runtime path: {forbidden}"
    return hashlib.sha256(binary.read_bytes()).hexdigest()


def qemu(binary, name, kernel):
    archive = OUT / f"{name}.cpio"
    run(["python3", ROOT / "tools/mkinitramfs.py", binary, archive])
    command = ["qemu-system-aarch64", "-M", "virt-11.1,highmem=off,gic-version=2", "-cpu", "cortex-a55,aarch64=off",
               "-smp", "4", "-m", "256", "-nographic", "-no-reboot", "-kernel", kernel,
               "-initrd", archive, "-append", "console=ttyAMA0 rdinit=/init panic=1 quiet loglevel=0"]
    try:
        result = subprocess.run(list(map(str, command)), cwd=ROOT, text=True,
                                stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=120)
    except subprocess.TimeoutExpired as error:
        console = error.stdout.decode() if isinstance(error.stdout, bytes) else error.stdout or ""
        raise RuntimeError(f"QEMU timed out:\n{console}") from error
    (OUT / f"{name}.log").write_text(result.stdout)
    assert result.returncode == 0, (result.returncode, result.stdout)
    return result.stdout


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--kernel", type=Path, help="ARM virt kernel with CONFIG_FILE_LOCKING; defaults to demo/kernel/zImage.xz")
    args = ap.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    kernel = args.kernel
    if kernel is None:
        kernel = OUT / "zImage"
        kernel.write_bytes(lzma.decompress((ROOT / "demo/kernel/zImage.xz").read_bytes()))
    env = dict(os.environ, CARGO_TARGET_DIR=str(OUT),
               ZIG_GLOBAL_CACHE_DIR=str(OUT / "zig-global"),
               ZIG_LOCAL_CACHE_DIR=str(OUT / "zig-local"))
    manifest = ROOT / "test/display-compositor/Cargo.toml"
    host = run(["cargo", "+1.92.0", "run", "--offline", "--release", "--manifest-path", manifest], env)
    assert "PASS: 2084 byte-exact display cases" in host, host
    print(host, end="")
    report = {"host_sha256": no_runtime(OUT / "release/display-compositor-check"), "hardware": False}
    target = "armv7-unknown-linux-musleabihf"
    sysroot = Path(run(["rustc", "+1.92.0", "--print", "sysroot"]).strip())
    host_triple = re.search(r"host: (\S+)", run(["rustc", "+1.92.0", "-vV"])).group(1)
    env.update(CC_armv7_unknown_linux_musleabihf=str(ROOT / "tools/toolchain/zcc-armv7-unknown-linux-musleabihf"),
               CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=str(sysroot / "lib/rustlib" / host_triple / "bin/rust-lld"))
    print(run(["cargo", "+1.92.0", "build", "--offline", "--release", "--manifest-path", manifest, "--target", target], env), end="")
    binary = OUT / target / "release/display-compositor-check"
    report["arm32_sha256"] = no_runtime(binary)
    console = qemu(binary, "qemu", kernel)
    assert "PASS: 2084 byte-exact display cases" in console, console
    built = run(["cargo", "+1.92.0", "test", "--offline", "--release", "--no-run", "--message-format=json",
                 "--manifest-path", ROOT / "src/pid1/display/Cargo.toml", "--target", target], env)
    unit = next(Path(item["executable"]) for line in built.splitlines() if line.startswith("{")
                and (item := json.loads(line)).get("reason") == "compiler-artifact"
                and item.get("executable") and item.get("profile", {}).get("test"))
    report["arm32_unit_sha256"] = no_runtime(unit)
    unit_console = qemu(unit, "scanout-units", kernel)
    passed = re.search(r"test result: ok\. (\d+) passed; 0 failed", unit_console)
    assert passed and int(passed.group(1)) >= 27, unit_console
    report["arm32_unit_tests"] = int(passed.group(1))
    report["kernel_sha256"] = hashlib.sha256(kernel.read_bytes()).hexdigest()
    report.update(arm32_qemu=True, forbidden_runtime_symbols=[], valid_cases=2084)
    (OUT / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(console, end="")
    print(unit_console, end="")
    print("PASS: host + ARM32 QEMU; no Lean runtime or boxed ABI in either linked executable")


if __name__ == "__main__":
    main()
