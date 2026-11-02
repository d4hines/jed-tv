#!/usr/bin/env python3
"""List the C library symbols a program still needs.

  python3 nolibc/tools/probe_undefined.py BIN TARGET [--against empty|nolibc] [--out FILE]

BIN is one of nolibc/build.py's bins (tvinit, slides, selftest, ytplay, doomtv).
--against empty  (default) link with EMPTY libc.a/libunwind.a: everything the
                 program imports from a libc (std + crates + C code).
--against nolibc link with our nolibc-rt libc.a: what is still MISSING.

Unresolved symbols are ignored at link time and read back from the
unstripped binary with llvm-nm -u (weak ones are marked). Builds go to
assets/build/nolibc/target-probe-<against>.
"""
import argparse
import pathlib
import subprocess
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
import build as B  # noqa: E402  (nolibc/build.py)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("bin", choices=list(B.BINS))
    ap.add_argument("target", choices=B.TARGETS)
    ap.add_argument("--against", choices=["empty", "nolibc"], default="empty")
    ap.add_argument("--out", type=pathlib.Path)
    args = ap.parse_args()
    if args.against == "empty":
        lib = B.BUILD / "empty-sysroot"
        lib.mkdir(parents=True, exist_ok=True)
        for name in ("libc.a", "libunwind.a"):
            (lib / name).write_bytes(b"!<arch>\n")
        extra = ["-C", "link-arg=--entry=main"]  # no _start: root the GC at main
    else:
        lib = B.BUILD / "sysroot" / args.target
        if not (lib / "libc.a").exists():
            sys.exit(f"{lib}/libc.a missing: run nolibc/build.py first")
        extra = []
    flags = B.nolibc_flags(args.target, lib) + ["-C", "link-arg=--unresolved-symbols=ignore-all", *extra]
    tdir = B.BUILD / f"target-probe-{args.against}"
    B.force_relink(tdir, args.target, [args.bin])
    B.cargo_build([args.bin], args.target, tdir, flags, env={"CARGO_PROFILE_RELEASE_STRIP": "false"})
    exe = tdir / args.target / "release" / args.bin
    out = subprocess.run([B.LLVM / "llvm-nm", "-u", str(exe)], capture_output=True, text=True, check=True).stdout
    syms = {}
    for line in out.splitlines():
        kind, name = line.split()[-2:]
        syms[name] = "  (weak)" if kind in ("w", "v") else ""
    text = "".join(f"{s}{syms[s]}\n" for s in sorted(syms))
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text)
        print(f"{args.out}: {len(syms)} undefined symbols ({args.bin}, {args.target}, against {args.against})")
    else:
        sys.stdout.write(text)


if __name__ == "__main__":
    main()
