#!/usr/bin/env python3
"""Build the TV apps twice, with musl (baseline) and with nolibc-rt, and
compare sizes.

  python3 nolibc/build.py [--target T ...] [--bin tvinit --bin slides ...] [--only nolibc|musl]

The apps are built straight from src/Cargo.toml (`--manifest-path`,
`--locked`: same sources and lockfile as the shipping musl builds, nothing
written under src/), but from this directory, so nolibc/.cargo/config.toml and
nolibc/rust-toolchain.toml apply and outputs stay under assets/build/nolibc.
`selftest` lives in this workspace.

Outputs (git-ignored):
  assets/build/nolibc/sysroot/<target>/{libc.a,libunwind.a}  our C library
  assets/build/nolibc/out/{musl,nolibc}/<target>/<bin>      stripped binaries
  assets/build/nolibc/sizes.json                             size table

How nolibc linking works: std (prebuilt by rustup for *-linux-musl) asks
for `-lc` and `-lunwind`. With `-C link-self-contained=no`, rustc stops
adding musl's crt objects and its self-contained library directory, and our
`-L` directory supplies `libc.a` = the nolibc-rt object (Rust + asm, which
also defines `_start`) and an empty `libunwind.a` (the unwinder stubs live in
libc.a). Stable Rust 1.92, no -Z flags.
"""
import argparse
import json
import os
import pathlib
import shutil
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[3]
NOLIBC = ROOT / "research/experiments/nolibc"
BUILD = ROOT / "assets/build/nolibc"
LLVM = pathlib.Path("/opt/homebrew/opt/llvm/bin")
TARGETS = ["aarch64-unknown-linux-musl", "armv7-unknown-linux-musleabihf"]
# bin name -> (workspace, package)
BINS = {"tvinit": ("src", "tvinit"), "slides": ("src", "slides"), "selftest": ("nolibc", "nolibc-selftest"),
        "ytplay": ("src", "ytplay"), "doomtv": ("src", "doomtv")}
DEFAULT_BINS = ["tvinit", "slides", "selftest"]


def cpu_flags(target):
    return ["-C", "target-cpu=cortex-a55"] if target.startswith("armv7") else []


def nolibc_flags(target, sysroot):
    return cpu_flags(target) + ["-C", "link-self-contained=no", "-L", f"native={sysroot}"]


def run(cmd, **kw):
    print("+", " ".join(map(str, cmd)), flush=True)
    subprocess.run(cmd, check=True, **kw)


def force_relink(tdir, target, bins):
    """Cargo does not track native libraries named via RUSTFLAGS; drop the
    fingerprints of the binary units so they relink against a new libc.a."""
    fps = tdir / target / "release" / ".fingerprint"
    for b in bins:
        for fp in fps.glob(f"{BINS[b][1]}-*"):
            if (fp / f"bin-{b}").exists():
                shutil.rmtree(fp)


def cargo_build(bins, target, tdir, flags, env=None):
    """Build `bins` (same workspace) for `target` with RUSTFLAGS=flags."""
    where = {BINS[b][0] for b in bins}
    assert len(where) == 1, "build src and nolibc bins separately"
    cmd = ["cargo", "build", "--release", "--target", target, "--target-dir", str(tdir)]
    if where == {"src"}:
        cmd += ["--manifest-path", str(ROOT / "src/Cargo.toml"), "--locked"]
    for b in bins:
        cmd += ["-p", BINS[b][1]]
    # Assembly in crates (ffh264's NEON) goes through src/'s zig cc wrappers,
    # which src/.cargo/config.toml sets but this directory's config does not.
    t_ = target.replace("-", "_")
    cc = {f"CC_{t_}": str(ROOT / "tools/toolchain" / f"zcc-{target}"), f"AR_{t_}": str(ROOT / "tools/toolchain/zar")}
    run(cmd, cwd=NOLIBC, env=dict(os.environ, RUSTFLAGS=" ".join(flags), **cc, **(env or {})))


def build_sysroot(target):
    """libc.a = just the nolibc-rt object out of its staticlib (the staticlib
    also bundles compiler_builtins, which every Rust program links anyway)."""
    run(["cargo", "build", "-p", "nolibc-rt", "--profile", "rt", "--target", target,
         "--target-dir", str(BUILD / "target")], cwd=NOLIBC,
        env=dict(os.environ, RUSTFLAGS=" ".join(cpu_flags(target))))
    staticlib = BUILD / "target" / target / "rt" / "libnolibc_rt.a"
    sysroot = BUILD / "sysroot" / target
    work = BUILD / "sysroot-work" / target
    shutil.rmtree(work, ignore_errors=True)
    work.mkdir(parents=True)
    members = subprocess.run([LLVM / "llvm-ar", "t", staticlib], capture_output=True, text=True,
                             check=True).stdout.split()
    ours = [m for m in members if m.startswith("nolibc_rt-")]
    if len(ours) != 1:
        sys.exit(f"expected one nolibc_rt object in {staticlib}, got {ours}")
    run([LLVM / "llvm-ar", "x", staticlib, ours[0]], cwd=work)
    sysroot.mkdir(parents=True, exist_ok=True)
    for lib in ("libc.a", "libunwind.a"):
        (sysroot / lib).unlink(missing_ok=True)
    run([LLVM / "llvm-ar", "rcs", sysroot / "libc.a", work / ours[0]])
    (sysroot / "libunwind.a").write_bytes(b"!<arch>\n")
    undef = subprocess.run([LLVM / "llvm-nm", "-u", work / ours[0]], capture_output=True, text=True,
                           check=True).stdout.split()
    print(f"{sysroot / 'libc.a'}: {(sysroot / 'libc.a').stat().st_size} bytes; imports: "
          f"{' '.join(s for s in undef if s != 'U')}")
    return sysroot


def build_apps(flavor, target, bins, sysroot=None):
    flags = nolibc_flags(target, sysroot) if flavor == "nolibc" else cpu_flags(target)
    tdir = BUILD / f"target-{flavor}"
    if flavor == "nolibc":
        force_relink(tdir, target, bins)
    for where in ("src", "nolibc"):
        group = [b for b in bins if BINS[b][0] == where]
        if group:
            cargo_build(group, target, tdir, flags)
    out = BUILD / "out" / flavor / target
    out.mkdir(parents=True, exist_ok=True)
    sizes = {}
    for b in bins:
        shutil.copy2(tdir / target / "release" / b, out / b)
        sizes[b] = (out / b).stat().st_size
    return sizes


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--target", action="append", choices=TARGETS)
    ap.add_argument("--bin", action="append", choices=list(BINS))
    ap.add_argument("--only", choices=["musl", "nolibc"])
    args = ap.parse_args()
    targets = args.target or TARGETS
    bins = args.bin or DEFAULT_BINS
    path = BUILD / "sizes.json"
    table = json.loads(path.read_text()) if path.exists() else {}
    for t in targets:
        for flavor in (["musl", "nolibc"] if not args.only else [args.only]):
            sysroot = build_sysroot(t) if flavor == "nolibc" else None
            for b, size in build_apps(flavor, t, bins, sysroot).items():
                table.setdefault(t, {}).setdefault(b, {})[flavor] = size
    path.write_text(json.dumps(table, indent=1) + "\n")
    print(f"\n{'target':34} {'bin':8} {'musl':>10} {'nolibc':>10} {'delta':>9}")
    for t, bs in table.items():
        for b, s in bs.items():
            m, n = s.get("musl"), s.get("nolibc")
            d = f"{(n - m) / m * 100:+.1f}%" if m and n else ""
            print(f"{t:34} {b:8} {m or '-':>10} {n or '-':>10} {d:>9}")


if __name__ == "__main__":
    main()
