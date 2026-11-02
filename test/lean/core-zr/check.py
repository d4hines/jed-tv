#!/usr/bin/env python3
"""Run the unmodified TV.Core loop on the Lean-written runtime in ARM32 QEMU."""
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import hashlib
import json
import os
import re
import subprocess
import struct

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / "assets/build/core-zr"
TOOLS = ROOT / "tools/lean/zr"
RT = TOOLS / ".lake/rt"
TARGET = ["-target", "arm-linux-musleabihf", "-mcpu=cortex_a55"]
ENV = dict(os.environ, ZIG_GLOBAL_CACHE_DIR=str(OUT / "zig-global"),
           ZIG_LOCAL_CACHE_DIR=str(OUT / "zig-local"))
for key in ("LEAN_PATH", "LEAN_SRC_PATH", "LD_LIBRARY_PATH", "DYLD_LIBRARY_PATH"):
    ENV.pop(key, None)


def run(args):
    proc = subprocess.run(list(map(str, args)), cwd=ROOT, env=ENV,
                          text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    if proc.returncode:
        raise RuntimeError(f"{' '.join(map(str, args))}\n{proc.stdout[:8000]}")
    return proc.stdout


def compile_c(source, target):
    run(["zig", "cc", *TARGET, "-O2", "-ffreestanding", "-fno-builtin", "-nostdinc",
         "-fno-exceptions", "-ffunction-sections", "-fdata-sections", "-fno-unwind-tables",
         "-fno-asynchronous-unwind-tables", "-std=gnu11", "-Wno-everything", "-DNDEBUG",
         f"-I{RT}/gen/include", f"-I{RT}/gen/rt", "-include", "lean/zr_fastpath.h",
         "-c", source, "-o", target])


def undefined(obj):
    return sorted(match.group(1) for line in run(["nm", "-u", obj]).splitlines()
                  if (match := re.search(r"(?:^|\s)U\s+(\S+)$", line)))


def main():
    assert (ROOT / "assets/build/kernel/arm/zImage").is_file(), "Build the ARM32 surrogate kernel first"
    OUT.mkdir(parents=True, exist_ok=True)
    print(run(["lake", "-d", TOOLS, "build", "zr-build", "zr-asm-gen", "zr-qemu32"]), end="")
    print(run(["lake", "-d", ROOT / "test/lean/core-zr", "build", "CoreZr"]), end="")
    print("Building Lean runtime from this checkout...", flush=True)
    print(run([TOOLS / ".lake/build/bin/zr-build", "rt"]), end="")
    sources = [ROOT / "assets/build/lake/tv/ir/TV/Core.c",
               ROOT / "assets/build/lake/tv/ir/TV/Apps.c",
               ROOT / "assets/build/lake/tv/ir/TV/Volume.c",
               ROOT / "assets/build/lake/tv/ir/TV/Volume/Native.c",
               ROOT / "assets/build/lake/core-zr/ir/CoreZr.c"]
    objs = [OUT / (name + ".o") for name in ("core", "apps", "volume", "volume-native", "harness")]
    with ThreadPoolExecutor(max_workers=3) as pool:
        list(pool.map(lambda pair: compile_c(*pair), zip(sources, objs)))
    symbols = {obj.stem: undefined(obj) for obj in objs}
    # Fail if Core starts requiring any executable upstream Init/Lean function.
    # Initializers alone are adapted, because all values actually referenced by
    # Core are either in Core/Apps or supplied by the freestanding runtime.
    allowed_init = {"initialize_Init_System_IO", "initialize_Init_Data_Int_OfNat",
                    "initialize_Init_Data_List_Lemmas", "initialize_Init_Omega",
                    "initialize_tv_TV_CoreProofs", "initialize_Init_Data_UInt",
                    "initialize_Init_Control_Basic", "initialize_Init_Data_SInt_Float32"}
    for obj in objs[:-1]:
        for symbol in symbols[obj.stem]:
            if symbol.startswith("initialize_"):
                assert symbol in allowed_init | {"initialize_tv_TV_Apps", "initialize_tv_TV_Volume"}, symbol
            if symbol.startswith(("l_Init_", "l_Lean_", "l_IO_", "lp_Init_", "lp_Lean_", "lp_IO_")):
                raise AssertionError(f"Unreviewed standard-library function: {symbol}")
    asm = OUT / "entry.S"
    print(run([TOOLS / ".lake/build/bin/zr-asm-gen", "app", asm,
               re.search(r"LEAN_EXPORT lean_object\* (initialize_\w+)\(", sources[-1].read_text()).group(1), "zr_core_test_main"]), end="")
    entry_obj = OUT / "entry.o"
    run(["zig", "cc", *TARGET, "-c", asm, "-o", entry_obj])
    runtime_objs = sorted((RT / "obj").glob("*.o")) + sorted((RT / "obj/tv").glob("*.o"))
    binary = OUT / "core-zr-arm32"
    link = run(["zig", "cc", *TARGET, "-static", "-nostdlib", "-Wl,--gc-sections",
                "-Wl,--strip-debug", "-Wl,--print-map", *objs, entry_obj, *runtime_objs, "-o", binary])
    (OUT / "link.map").write_text(link)
    assert not undefined(binary), "Unresolved symbols remain in final ELF"
    elf = binary.read_bytes()
    assert elf[:6] == b"\x7fELF\x01\x01", "Expected little-endian ELF32"
    assert struct.unpack_from("<H", elf, 18)[0] == 40, "Expected ARM machine"
    phoff = struct.unpack_from("<I", elf, 28)[0]
    phentsize, phnum = struct.unpack_from("<HH", elf, 42)
    assert all(struct.unpack_from("<I", elf, phoff + n * phentsize)[0] not in (2, 3)
               for n in range(phnum)), "Dynamic loader/library dependency in binary"
    # The map must contain no upstream runtime/library objects or vendor code.
    assert not any(x in link for x in ("libleanrt", "libInit", "libstdc++", "libc.a"))
    (OUT / "symbols.json").write_text(json.dumps(symbols, indent=2) + "\n")
    console = run([TOOLS / ".lake/build/bin/zr-qemu32", binary, "30"])
    (OUT / "console.txt").write_text(console)
    begin, end = "==core-zr-begin==\n", "==core-zr-end=="
    assert "[qemu32: timeout]" not in console, console
    assert begin in console and end in console, console
    transcript = console.split(begin, 1)[1].split(end, 1)[0]
    expected = (ROOT / "test/lean/core-zr/expected.txt").read_text()
    assert transcript == expected, f"Expected:\n{expected}\nActual:\n{transcript}"
    assert "FAIL" not in console, console
    report = {"binary": str(binary.relative_to(ROOT)), "bytes": binary.stat().st_size,
              "sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
              "core_c_sha256": hashlib.sha256(sources[0].read_bytes()).hexdigest(),
              "runtime": "ZrLean", "architecture": "ARM32", "qemu_pid1": True,
              "hardware": False, "unresolved_symbols": [],
              "scenarios": ["lifecycle-input-policy", "save-failure", "unreadable-policy", "invalid-pin", "finite-volume", "references", "float32"],
              "initializer_adapters": sorted(allowed_init)}
    (OUT / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(transcript, end="")
    print(f"PASS: real TV.Core loop + Lean Zr runtime, ARM32 QEMU, {binary.stat().st_size} bytes")


if __name__ == "__main__":
    main()
