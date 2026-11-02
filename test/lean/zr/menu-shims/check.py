#!/usr/bin/env python3
"""Exercise the menu's ABI shims in ARM32, with normal Lean values and sharing."""
import importlib.util
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[4]
spec = importlib.util.spec_from_file_location("core_zr_tools", ROOT / "test/lean/core-zr/check.py")
tools = importlib.util.module_from_spec(spec)
spec.loader.exec_module(tools)
OUT = ROOT / "assets/build/menu-shims"
OUT.mkdir(parents=True, exist_ok=True)

print(tools.run(["lake", "-d", ROOT / "src/lean/zr", "build", "ZrLean.Low.InitShim"]), end="")
print(tools.run(["lake", "-d", ROOT / "test/lean/zr/menu-shims", "build"]), end="")
# Build the normal runtime first with `lake -d tools/lean/zr exe zr-build rt`.
# The local object below is explicit so this test does not rely on cached shims.
assert (tools.RT / "obj/runtime.o").is_file(), "Build the Zr runtime first"
source = ROOT / "assets/build/lake/menu-shims/ir/MenuShimCheck.c"
shim = OUT / "shim.o"
case = OUT / "test.o"
tools.compile_c(ROOT / "assets/build/lake/zr/ir/ZrLean/Low/InitShim.c", shim)
tools.compile_c(source, case)
init = re.search(r"LEAN_EXPORT lean_object\* (initialize_\w+)\(", source.read_text()).group(1)
asm = OUT / "entry.S"
generator = ROOT / "tools/lean/zr/.lake/build/bin/zr-asm-gen"
assert generator.is_file(), "Build the Zr host tools first"
tools.run([generator, "app", asm, init, "zr_menu_shim_test_main"])
tools.run(["zig", "cc", *tools.TARGET, "-c", asm, "-o", OUT / "entry.o"])
objects = sorted((tools.RT / "obj").glob("*.o")) + sorted((tools.RT / "obj/tv").glob("*.o"))
objects = [p for p in objects if p.name != "ZrLean_Low_InitShim.o"]
binary = OUT / "menu-shims-arm32"
tools.run(["zig", "cc", *tools.TARGET, "-static", "-nostdlib", "-Wl,--gc-sections",
           shim, case, OUT / "entry.o", *objects, "-o", binary])
console = tools.run([ROOT / "tools/lean/zr/.lake/build/bin/zr-qemu32", binary, "30"])
(OUT / "console.txt").write_text(console)
assert "PASS: menu shims, 5000 assertions" in console and "FAIL" not in console, console
assert "timeout" not in console, console
print("PASS: 5000 assertions on ARM32, including shared lists, big indices and UTF-8 slices")
