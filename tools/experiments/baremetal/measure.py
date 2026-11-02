#!/usr/bin/env python3
"""Size report for the bare-metal spike: lines of code, ELF sections, raw
image size, dependency versions. Writes assets/build/baremetal/measure.txt.

  python3 baremetal/measure.py
"""
import pathlib
import re
import struct
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[3]
HERE = ROOT / "research/experiments/baremetal"
OUT = ROOT / "assets" / "build" / "baremetal"
ELF = OUT / "target" / "aarch64-unknown-none" / "release" / "tvbare"
OBJCOPY = (pathlib.Path.home() / ".rustup/toolchains/1.92.0-aarch64-apple-darwin/lib/rustlib"
           / "aarch64-apple-darwin/bin/rust-objcopy")


def loc(path):
    total = code = 0
    for line in path.read_text().splitlines():
        total += 1
        s = line.strip()
        if s and not s.startswith(("//", "#", "/*", "*")):
            code += 1
    return total, code


def sections(path):
    d = path.read_bytes()
    shoff, = struct.unpack_from("<Q", d, 0x28)
    shentsize, shnum, shstrndx = struct.unpack_from("<HHH", d, 0x3A)
    hdrs = [struct.unpack_from("<IIQQQQIIQQ", d, shoff + i * shentsize) for i in range(shnum)]
    strtab = hdrs[shstrndx]
    name = lambda off: d[strtab[4] + off:d.index(b"\0", strtab[4] + off)].decode()
    return [(name(h[0]), h[3], h[5]) for h in hdrs if h[3] and h[5]]  # (name, addr, size)


def main():
    out = []
    files = sorted(HERE.glob("src/*.rs")) + [HERE / "build.rs", HERE / "link.ld"]
    t = c = 0
    out.append("Lines of code (total / non-comment non-blank):")
    for f in files:
        a, b = loc(f)
        t, c = t + a, c + b
        out.append(f"  {f.relative_to(HERE)}: {a} / {b}")
    out.append(f"  Rust+asm+linker script: {t} / {c}")
    for f in [ROOT / "tools/experiments/baremetal/run.py", ROOT / "tools/experiments/baremetal/bench.py", ROOT / "tools/experiments/baremetal/pngcmp.py", ROOT / "tools/experiments/baremetal/measure.py"]:
        a, b = loc(f)
        out.append(f"  (host tooling) {f.relative_to(HERE)}: {a} / {b}")
    asm = sum(1 for line in (HERE / "src/boot.rs").read_text().splitlines()
              if re.match(r"\s+[a-z]+\s", line) and not line.strip().startswith("//"))
    out.append(f"  assembly instructions in boot.rs (approx.): {asm}")

    out.append(f"\nELF {ELF.relative_to(ROOT)}: {ELF.stat().st_size} bytes")
    for n, addr, size in sections(ELF):
        if not n.startswith((".debug", ".symtab", ".strtab", ".shstrtab", ".comment")):
            out.append(f"  {n:10} {addr:#x} {size:>9} bytes")
    raw = OUT / "tvbare.bin"
    subprocess.run([str(OBJCOPY), "-O", "binary", str(ELF), str(raw)], check=True)
    out.append(f"raw image (objcopy -O binary): {raw.stat().st_size} bytes")

    # What FFmpeg's C H.264 decoder (as linked into ytplay) needs from libc:
    # symbols undefined in the archives and defined in none of them.
    libs = sorted((ROOT / "assets/build/decoder/aarch64-unknown-linux-musl/lib").glob("*.a"))
    if libs:
        nm = subprocess.run(["nm", "-g", *map(str, libs)], capture_output=True, text=True).stdout
        defined, undef = set(), set()
        for line in nm.splitlines():
            parts = line.split()
            if len(parts) == 2 and parts[0] == "U":
                undef.add(parts[1])
            elif len(parts) == 3 and parts[1] not in "Uw":
                defined.add(parts[2])
        need = sorted(undef - defined)
        out.append(f"\nFFmpeg C decoder ({', '.join(p.name for p in libs)}) imports {len(need)} "
                   f"external symbols:\n  " + " ".join(need))

    lock = (HERE / "Cargo.lock").read_text()
    pkgs = re.findall(r'name = "([^"]+)"\nversion = "([^"]+)"', lock)
    out.append(f"\nCrates in Cargo.lock ({len(pkgs)}): " + ", ".join(f"{n} {v}" for n, v in pkgs))
    text = "\n".join(out) + "\n"
    (OUT / "measure.txt").write_text(text)
    print(text, end="")


if __name__ == "__main__":
    main()
