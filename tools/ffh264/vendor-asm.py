#!/usr/bin/env python3
"""Step 5: copy the FFmpeg assembly that survives into ytplay (per prune.py),
unmodified, plus the headers it includes (from the build's .d files), into
src/ffh264/asm/<arch>/ so the crate's build.rs can assemble it.
  vendor-asm.py TARGET"""
import json, pathlib, shutil, sys

T = sys.argv[1]
ARCH = {"aarch64-unknown-linux-musl": "aarch64", "armv7-unknown-linux-musleabihf": "arm"}[T]
ROOT = pathlib.Path(__file__).resolve().parents[2]
B = ROOT / "assets/build/ffh264"
work = B / f"work-{T}"
dst = ROOT / "src/ffh264/asm" / ARCH
if dst.exists():
    shutil.rmtree(dst)
srcs, deps = [], set()
for tu in json.load(open(B / f"tus-{T}.json")):
    if tu["kind"] != "asm":
        continue
    srcs.append(tu["src"])
    d = (work / tu["obj"]).with_suffix(".d").read_text().replace("\\\n", " ").split()
    deps.update(x for x in d[1:])
for dep in sorted(deps):
    rel = dep.removeprefix("src/")
    s = work / dep                      # src/ is a symlink to the FFmpeg tree
    out = dst / rel
    out.parent.mkdir(parents=True, exist_ok=True)
    shutil.copyfile(s, out)
(dst / "SOURCES").write_text("".join(f"{s}\n" for s in srcs))
print(f"{T}: {len(srcs)} .S files, {len(deps)} files copied to {dst}")
