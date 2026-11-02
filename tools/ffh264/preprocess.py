#!/usr/bin/env python3
"""Step 3: preprocess the used C translation units with the exact zig cc
command lines from the FFmpeg build (make V=1), so c2rust sees precisely what
the C build compiled (same config.h, musl headers, predefined macros).
  preprocess.py TARGET
Reads  assets/build/ffh264/tus-TARGET.json and work-TARGET/make.log.
Writes assets/build/ffh264/pp/TARGET/<src path>.c plus compile_commands.json
(relative paths; c2rust runs elsewhere and only needs --target)."""
import json, pathlib, shlex, subprocess, sys

T = sys.argv[1]
ROOT = pathlib.Path(__file__).resolve().parents[2]
B = ROOT / "assets/build/ffh264"
work = B / f"work-{T}"
out = B / "pp" / T
CLANG_TARGET = {"aarch64-unknown-linux-musl": "aarch64-unknown-linux-musl",
                "armv7-unknown-linux-musleabihf": "armv7-unknown-linux-musleabihf"}[T]

cmds = {}
for line in (work / "make.log").read_text().splitlines():
    w = shlex.split(line)
    if len(w) > 3 and w[0].endswith("/bin/cc") and "-c" in w and w[-1].endswith(".c"):
        cmds[w[-1].removeprefix("src/")] = w

entries = []
for tu in json.load(open(B / f"tus-{T}.json")):
    if tu["kind"] != "c":
        continue
    s = tu["src"]
    dst = out / s
    dst.parent.mkdir(parents=True, exist_ok=True)
    if s == "src/ffh264/ytdec.c":
        argv = [str(work / "bin/cc"), "-O2", f"-I{B / 'c' / T / 'include'}", "-E",
                "-o", str(dst), str(ROOT / s)]
    else:
        w = cmds[s]
        argv, skip = [], 0
        for a in w:
            if skip:
                skip -= 1
                continue
            if a in ("-MF", "-MT", "-o"):
                skip = 1
                continue
            if a in ("-MMD", "-c"):
                continue
            argv.append(a)
        argv = argv[:-1] + ["-E", "-o", str(dst), argv[-1]]
    subprocess.run(argv, cwd=work, check=True)
    entries.append({"directory": str(out), "file": s,
                    "arguments": ["clang", f"--target={CLANG_TARGET}", "-std=c17",
                                  "-w", "-c", s]})
(out / "compile_commands.json").write_text(json.dumps(entries, indent=1))
print(f"{T}: {len(entries)} C files preprocessed into {out}")
