#!/usr/bin/env python3
"""Step 2: list the FFmpeg translation units that survive into the final ytplay
binary (rust-lld -Map with --gc-sections), mapped back to source files.
  prune.py MAP WORKDIR SRC OUT.json
Archive members are basenames, so a member name shared by several objects
(e.g. libavutil/cpu.o and libavutil/arm/cpu.o) keeps all of them; each such
case was checked by hand to have live symbols in both."""
import json, pathlib, re, sys
mapf, work, src, out = sys.argv[1:5]
work, src = pathlib.Path(work), pathlib.Path(src)
used = sorted(set(re.findall(r"lib(avcodec|avutil|ytdec)\.a\(([^)]*)\)", open(mapf).read())))
objs = {}
for o in work.rglob("*.o"):
    rel = o.relative_to(work)
    if rel.name == "ytdec.o":
        objs.setdefault(("ytdec", rel.name), []).append(rel)
    elif rel.parts[0] in ("libavcodec", "libavutil"):
        objs.setdefault((rel.parts[0][3:], rel.name), []).append(rel)
res = []
for lib, member in used:
    for rel in sorted(objs.get((lib, member), [])):
        if rel.name == "ytdec.o":
            res.append({"lib": lib, "obj": str(rel), "src": "src/ffh264/ytdec.c", "kind": "c"})
            continue
        base = rel.with_suffix("")
        for ext, kind in ((".c", "c"), (".S", "asm")):
            if (src / (str(base) + ext)).exists():
                res.append({"lib": lib, "obj": str(rel), "src": str(base) + ext, "kind": kind})
                break
        else:
            sys.exit(f"no source for {rel}")
json.dump(res, open(out, "w"), indent=1)
c = sum(r["kind"] == "c" for r in res)
print(f"{len(used)} members -> {len(res)} objects: {c} C, {len(res) - c} asm")
