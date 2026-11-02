#!/usr/bin/env bash
# Step 4 (runs on the Linux build VM, which has c2rust 0.22.1 + libclang 19):
#   ssh eduardo-vm bash -s -- TARGET [JOBS] [FILTER] < tools/ffh264/transpile-remote.sh
# Input:  ~/Downloads/ffh264/pp/TARGET/ (preprocess.py output, rsynced over)
# Output: ~/Downloads/ffh264/rs/TARGET/<C path minus .c>.rs, one module per C file.
# Translates in parallel, one c2rust process per file (each module is self-contained).
set -euo pipefail
T=$1; JOBS=${2:-10}; FILTER=${3:-}
W=$HOME/Downloads/ffh264
export PP=$W/pp/$T OUT=$W/rs/$T TMP=$W/tmp/$T LOGS=$W/logs/$T C2RUST=$W/c2rust-bin/bin/c2rust
[ -n "$FILTER" ] || rm -rf "$OUT" "$LOGS"
rm -rf "$TMP"
mkdir -p "$OUT" "$LOGS" "$TMP"
python3 - "$FILTER" "$JOBS" <<'EOF'
import json, os, pathlib, shutil, subprocess, sys
from concurrent.futures import ThreadPoolExecutor
flt, jobs = sys.argv[1], int(sys.argv[2])
pp, out, tmp, logs, c2rust = (pathlib.Path(os.environ[k]) for k in ("PP", "OUT", "TMP", "LOGS", "C2RUST"))
cc = json.load(open(pp / "compile_commands.json"))

def one(e):
    name = e["file"].replace("/", "__")
    d = tmp / name
    (d / "db").mkdir(parents=True)
    e = dict(e, directory=str(pp))
    (d / "db/compile_commands.json").write_text(json.dumps([e]))
    with open(logs / f"{name}.log", "w") as log:
        r = subprocess.run([str(c2rust), "transpile", str(d / "db/compile_commands.json"),
                            "-o", str(d / "out"), "--emit-modules", "--overwrite-existing"],
                           stdout=log, stderr=subprocess.STDOUT)
    rs = list((d / "out").rglob("*.rs"))
    if r.returncode or len(rs) != 1:
        return f"FAILED {e['file']} (rc {r.returncode}, {len(rs)} files)"
    dst = out / (e["file"][:-2] + ".rs")
    dst.parent.mkdir(parents=True, exist_ok=True)
    shutil.move(rs[0], dst)
    return None

todo = [e for e in cc if not flt or flt in e["file"]]
with ThreadPoolExecutor(jobs) as ex:
    for msg in ex.map(one, todo):
        if msg:
            print(msg, flush=True)
print(f"{len(todo)} attempted, {len(list(out.rglob('*.rs')))} .rs files in {out}")
EOF
