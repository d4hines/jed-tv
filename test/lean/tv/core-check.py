#!/usr/bin/env python3
"""Executable checks for the PR14 pure core and native scalar event contract."""
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
LEAN = ROOT / "src/pid1/lean"
subprocess.run(["lake", "build", "TV.Core"], cwd=LEAN, check=True)
subprocess.run(["lake", "env", "lean", "--run", str(ROOT / "test/lean/tv/core-check.lean")], cwd=LEAN, check=True)
code = (ROOT / "assets/build/lake/tv/ir/TV/Core.c").read_text()
assert "initialize_Init(" not in code and "initialize_Init_Data_Random(" not in code
print("PASS: Core avoids full Init and random initializers")
