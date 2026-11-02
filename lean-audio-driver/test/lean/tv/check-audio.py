#!/usr/bin/env python3
"""Check PID1's Lean audio driver against a mock of its mechanisms."""
from pathlib import Path
import os
import platform
import subprocess

ROOT = Path(__file__).resolve().parents[3]
LEAN = ROOT / "src/lean"
subprocess.run(["lake", "build", "TV.Audio.Native"], cwd=LEAN, check=True)
subprocess.run(["lake", "env", "lean", "--run", str(ROOT / "test/lean/tv/audio/Check.lean")], cwd=LEAN, check=True)
# The three driver properties, proved about the shipping definitions, then an
# audit of what each main theorem depends on.
subprocess.run(["lake", "build", "TV.Audio.Proofs"], cwd=LEAN, check=True)
audit = subprocess.run(["lake", "env", "lean", str(ROOT / "test/lean/tv/audio/Axioms.lean")], cwd=LEAN,
                       check=True, capture_output=True, text=True).stdout
reports = [block for block in audit.replace("\n ", " ").splitlines() if "depends on axioms" in block]
assert len(reports) == 9, audit
for report in reports:
    used = report.split("[", 1)[1].rstrip("]").split(",")
    for axiom in (a.strip() for a in used):
        assert axiom in ("propext", "Classical.choice", "Quot.sound") or "._native." in axiom, report
print("PASS: audio proofs (ceiling, resampler, cleanup order) use only the allowed axioms")
for module in ("Audio/Fault", "Audio/Gain", "Audio/Resample", "Audio/Protocol", "Audio/Session", "Audio/Native"):
    code = (ROOT / "assets/build/lake/tv/ir/TV" / (module + ".c")).read_text()
    assert "initialize_Init(" not in code and "initialize_Init_Data_Random(" not in code, module
print("PASS: audio avoids full Init and random initializers")

prefix = subprocess.check_output(["lean", "--print-prefix"], cwd=LEAN, text=True).strip()
env = os.environ.copy()
cc = env.get("CC", "cc")
if platform.system() == "Darwin" and "CC" not in env:
    cc = "/Library/Developer/CommandLineTools/usr/bin/clang"
    env["SDKROOT"] = "/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk"
subprocess.run([cc, "-fsyntax-only", "-Wall", "-Werror", "-I" + prefix + "/include", str(LEAN / "native/audio.c")], check=True, env=env)
print("PASS: audio marshalling C compiles against lean.h")
