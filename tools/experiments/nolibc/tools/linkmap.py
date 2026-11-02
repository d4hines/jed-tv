#!/usr/bin/env python3
"""What is actually linked into a nolibc binary, from lld's link map.

  python3 nolibc/tools/linkmap.py [--target T ...] [--bin B ...]

Relinks each bin with `-Map` (same flags as nolibc/build.py otherwise) and
sums the sizes of the input sections that survived, per input file. Flags
anything from musl (rustlib/<target>/lib/self-contained) and lists the
compiler-builtins members that come from compiler-rt's C/assembly sources
rather than Rust. Writes assets/build/nolibc/linkmap/<target>-<bin>.{map,txt}.
"""
import argparse
import collections
import pathlib
import re
import shutil
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[1]))
import build as B  # noqa: E402

OUT = B.BUILD / "linkmap"


def classify(f):
    name = f.split("/")[-1]
    if "self-contained" in f or (name.startswith("libc.a(") and "nolibc_rt" not in name):
        return "MUSL (must not appear)"
    if "nolibc_rt" in name:
        return "nolibc-rt (our libc.a)"
    if name.startswith("libcompiler_builtins"):
        member = name[name.index("(") + 1:-1]
        return ("compiler-builtins (Rust)" if member.startswith("compiler_builtins-")
                else f"compiler-rt C/asm member: {member}")
    if name.startswith("<"):
        return "linker-synthesized"
    # Thin-LTO objects: <bin>-<hash>.<crate>-<hash>.<crate>.<cgu>...rcgu.o
    m = re.match(r"[^.]+\.([A-Za-z0-9_]+)-[0-9a-f]{16}\.", name)
    if m:
        crate = m.group(1)
        std = {"std", "core", "alloc", "addr2line", "gimli", "object", "miniz_oxide", "adler2", "hashbrown",
               "rustc_demangle", "memchr", "std_detect", "panic_abort", "unwind", "cfg_if", "libc"}
        return f"Rust std ({crate})" if crate in std else f"Rust crate {crate}"
    return f"other: {name}"


LINE = re.compile(r"^\s*[0-9a-f]+\s+[0-9a-f]+\s+([0-9a-f]+)\s+\d+\s+(\S.*?):\(")


def summarize(mapfile):
    sizes = collections.Counter()
    for line in mapfile.read_text().splitlines():
        m = LINE.match(line)
        if m:
            sizes[classify(m.group(2))] += int(m.group(1), 16)
    return sizes


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--target", action="append", choices=B.TARGETS)
    ap.add_argument("--bin", action="append", choices=list(B.BINS))
    args = ap.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    tdir = B.BUILD / "target-map"
    for t in args.target or B.TARGETS:
        sysroot = B.BUILD / "sysroot" / t
        last = tdir / f"last-{t}.map"
        flags = B.nolibc_flags(t, sysroot) + ["-C", f"link-arg=-Map={last}"]
        for b in args.bin or ["tvinit", "slides"]:
            B.force_relink(tdir, t, [b])
            B.cargo_build([b], t, tdir, flags)
            m = OUT / f"{t}-{b}.map"
            shutil.copy2(last, m)
            sizes = summarize(m)
            total = sum(sizes.values())
            lines = [f"{t} {b}: {total} bytes of input sections"]
            for k, v in sorted(sizes.items(), key=lambda kv: -kv[1]):
                lines.append(f"  {v:>9}  {k}")
            (OUT / f"{t}-{b}.txt").write_text("\n".join(lines) + "\n")
            print("\n".join(lines))


if __name__ == "__main__":
    main()
