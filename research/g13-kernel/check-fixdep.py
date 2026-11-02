#!/usr/bin/env python3
"""Check that changing a Kconfig stamp actually rebuilds a dependent target."""
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
BUILD = ROOT / "assets/build/g13-kernel"
work = BUILD / "fixdep-check"
work.mkdir(exist_ok=True)
(work / "include/config").mkdir(parents=True, exist_ok=True)
(work / "probe.c").write_text("int value = CONFIG_PAGE_OFFSET;\n")
(work / "include/config/PAGE_OFFSET").write_text("")
(work / "probe.d").write_text("probe.o: probe.c\n")
(work / "probe.o").write_text("original\n")
base = time.time() - 10
for name in ("probe.c", "include/config/PAGE_OFFSET"):
    os.utime(work / name, (base, base))
os.utime(work / "probe.o", (base + 1, base + 1))
deps = subprocess.check_output([str(BUILD / "out/scripts/basic/fixdep"),
                                "probe.d", "probe.o", "cc"], cwd=work, text=True)
(work / "deps.mk").write_text(deps)
assert "$(wildcard include/config/PAGE_OFFSET)" in deps
(work / "Makefile").write_text("include deps.mk\nprobe.o: $(deps_probe.o)\n\t@echo REBUILT\n\t@touch probe.o\n")
before = subprocess.check_output(["make", "--no-print-directory", "probe.o"], cwd=work, text=True)
assert "REBUILT" not in before
os.utime(work / "include/config/PAGE_OFFSET", (base + 2, base + 2))
after = subprocess.check_output(["make", "--no-print-directory", "probe.o"], cwd=work, text=True)
assert "REBUILT" in after
result = {"pass": True, "before_config_change": before.strip(), "after_config_change": after.strip()}
(work / "result.json").write_text(json.dumps(result, indent=2) + "\n")
print(json.dumps(result))
