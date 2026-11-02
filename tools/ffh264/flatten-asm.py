#!/usr/bin/env python3
"""Stretch step: flattened, macro-free copies of the assembly ffh264 links, for
the Lean assembly DSL.
  flatten-asm.py TARGET [LLVM_MC]
For each .S in src/ffh264/asm/<arch>/SOURCES (+ our varargs-<arch>.S):
  1. cpp with the crate's flags (zig cc -E)         -> <name>.i.s
  2. llvm-mc re-emits it with every gas macro, .irp/.rept and conditional
     expanded into plain instructions + directives  -> <name>.flat.s
  3. check: assembling the flat file gives byte-identical code/data sections
     to zig cc's object from the original .S
Output: assets/build/ffh264/flat-asm/TARGET/ (mirrors the source paths)."""
import hashlib, pathlib, re, subprocess, sys

T = sys.argv[1]
LLVM_MC = sys.argv[2] if len(sys.argv) > 2 else "/opt/homebrew/opt/llvm/bin/llvm-mc"
OBJDUMP = str(pathlib.Path(LLVM_MC).with_name("llvm-objcopy"))
ARCH, ZT, TRIPLE = {
    "aarch64-unknown-linux-musl": ("aarch64", "aarch64-linux-musl", "aarch64-unknown-linux-musl"),
    "armv7-unknown-linux-musleabihf": ("arm", "arm-linux-musleabihf", "armv7-unknown-linux-musleabihf"),
}[T]
ROOT = pathlib.Path(__file__).resolve().parents[2]
ASM = ROOT / "src/ffh264/asm"
OUT = ROOT / "assets/build/ffh264/flat-asm" / T
CPP = ["zig", "cc", "-target", ZT, "-mcpu=cortex_a55", f"-I{ASM / ARCH}", "-D_ISOC11_SOURCE",
       "-D_FILE_OFFSET_BITS=64", "-D_LARGEFILE_SOURCE", "-D_POSIX_C_SOURCE=200112",
       "-D_XOPEN_SOURCE=600", "-Qunused-arguments"]
MC = [LLVM_MC, f"-triple={TRIPLE}", "-mcpu=cortex-a55"]


def sections(obj):
    """{section name: sha1} of the allocated sections' bytes."""
    out = {}
    names = subprocess.run([str(pathlib.Path(LLVM_MC).with_name("llvm-readelf")), "-S", "-W", str(obj)],
                           capture_output=True, text=True, check=True).stdout
    for name in re.findall(r"\]\s+(\.(?:text|rodata|data)[\w.]*)\s", names):
        dump = obj.with_suffix(f".{name.strip('.').replace('.', '_')}.bin")
        subprocess.run([OBJDUMP, "-O", "binary", f"--only-section={name}", str(obj), str(dump)], check=True)
        out[name] = hashlib.sha1(dump.read_bytes()).hexdigest()
        dump.unlink()
    return out


srcs = [ASM / ARCH / s for s in (ASM / ARCH / "SOURCES").read_text().split()] + [ASM / f"varargs-{ARCH}.S"]
ok, lines = 0, 0
for s in srcs:
    rel = s.relative_to(ASM / ARCH) if s.is_relative_to(ASM / ARCH) else pathlib.Path(s.name)
    base = OUT / rel.with_suffix("")
    base.parent.mkdir(parents=True, exist_ok=True)
    pre, flat = base.with_suffix(".i.s"), base.with_suffix(".flat.s")
    subprocess.run(CPP + ["-E", "-P", "-x", "assembler-with-cpp", "-o", str(pre), str(s)], check=True)
    subprocess.run(MC + ["-o", str(flat), str(pre)], check=True)
    flat.write_text(f"// Flattened from FFmpeg 9.0.1 {rel} for {T} by tools/ffh264/flatten-asm.py\n"
                    f"// (cpp + llvm-mc); see that file's original LGPL notice.\n"
                    + re.sub(r"\n(\s*\n)+", "\n", flat.read_text()))
    ref, chk = base.with_suffix(".ref.o"), base.with_suffix(".flat.o")
    subprocess.run(CPP + ["-c", "-o", str(ref), str(s)], check=True)
    subprocess.run(MC + ["-filetype=obj", "-o", str(chk), str(flat)], check=True)
    a, b = sections(ref), sections(chk)
    same = a == b
    ok += same
    n = sum(1 for l in flat.read_text().splitlines() if l.startswith("\t") and not l.lstrip().startswith("."))
    lines += n
    print(f"  {rel}: {n} instructions, sections {'identical' if same else 'DIFFER'}"
          + ("" if same else f"\n    ref  {a}\n    flat {b}"))
    ref.unlink(); chk.unlink()
print(f"{T}: {ok}/{len(srcs)} flattened files reassemble byte-identically; "
      f"{lines} instruction lines -> {OUT}")
