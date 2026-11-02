#!/usr/bin/env python3
"""Check PID1's Lean volume service independently of device/input effects."""
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
LEAN = ROOT / "src/pid1/lean"
subprocess.run(["lake", "build", "TV.Volume.Native"], cwd=LEAN, check=True)
subprocess.run(["lake", "env", "lean", "--run", str(ROOT / "test/lean/tv/volume/Check.lean")], cwd=LEAN, check=True)
for module in ("Volume", "Volume/Native"):
    code = (ROOT / "assets/build/lake/tv/ir/TV" / (module + ".c")).read_text()
    assert "initialize_Init(" not in code and "initialize_Init_Data_Random(" not in code, module
print("PASS: volume avoids full Init and random initializers")
