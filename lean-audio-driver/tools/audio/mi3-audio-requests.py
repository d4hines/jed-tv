#!/usr/bin/env python3
"""Recover the MI PCM/AOUT ioctl request numbers a libmi3.so uses, offline.

Each exported MI_PCM_* / MI_AOUT_* function materializes its request number
with a Thumb movw/movt pair before calling libmi3's ioctl_wrapper. This
disassembles every such export with llvm-objdump and reports the requests
found, decoded as direction/type/number/size. Compare two TVs' libraries:

  python3 tools/audio/mi3-audio-requests.py G06/libmi3.so G13/libmi3.so

The G06 (V109) numbers are the ones src/audio/{mi-pcm,aout-route}.rs send.
A differing size means a differing wire structure; the Lean driver must not
be pointed at that TV until the structure is mapped. Never accesses a device.
"""
import hashlib
import json
import re
import shutil
import subprocess
import sys
from pathlib import Path

LLVM = Path("/opt/homebrew/opt/llvm/bin")
EXPORT = re.compile(r"^(MI_PCM_|MI_AOUT_)")
# What src/audio sends (V109 libmi3.so a10f65b0...). Names are libmi3 exports.
DRIVER = {
    "MI_PCM_Open": 0xC0803402, "MI_PCM_Close": 0xC0043403, "MI_PCM_Start": 0xC0083404,
    "MI_PCM_Stop": 0xC0043405, "MI_PCM_Write": 0xC0383408, "MI_PCM_GetAttr": 0xC018340B,
    "MI_AOUT_GetHandle": 0xC0081116, "MI_AOUT_GetConnectedNum": 0xC010111E,
    "MI_AOUT_GetConnected": 0xC018111F, "MI_AOUT_ConnectInput": 0xC00C111C,
    "MI_AOUT_DisconnectInput": 0xC008111D, "MI_AOUT_GetVolumeDb": 0xC010110A,
    "MI_AOUT_SetVolumeDb": 0x40101109,
}


def tool(name):
    path = LLVM / name
    return str(path) if path.exists() else shutil.which(name) or sys.exit(f"need {name}")


def decode(request):
    direction = {0: "none", 1: "write", 2: "read", 3: "read/write"}[request >> 30]
    return {"request": f"{request:#010x}", "direction": direction, "type": (request >> 8) & 0xFF,
            "number": request & 0xFF, "size": (request >> 16) & 0x3FFF}


def exports(path):
    out = subprocess.check_output([tool("llvm-nm"), "-D", "--defined-only", str(path)], text=True)
    found = {}
    for line in out.splitlines():
        parts = line.split()
        if len(parts) == 3 and parts[1] == "T" and EXPORT.match(parts[2]):
            found[parts[2]] = int(parts[0], 16)
    return found


def requests(path, names):
    """Per export: every movw/movt-built word that looks like an MI ioctl request."""
    text = subprocess.check_output([tool("llvm-objdump"), "-d", "--no-show-raw-insn", str(path)], text=True)
    starts = sorted((addr, name) for name, addr in names.items())
    bounds = {}
    for i, (addr, name) in enumerate(starts):
        bounds[name] = (addr, starts[i + 1][0] if i + 1 < len(starts) else addr + 0x400)
    result = {name: [] for name in names}
    current = None
    low = {}
    for line in text.splitlines():
        m = re.match(r"\s*([0-9a-f]+):\s+(\S+)\s*(.*)", line)
        if not m:
            continue
        pc = int(m[1], 16)
        current = next((n for n, (a, b) in bounds.items() if a <= pc < b), None)
        if current is None:
            continue
        op, args = m[2], m[3]
        mw = re.match(r"(r\d+|lr|sl|fp|ip),\s*#(0x[0-9a-f]+|\d+)", args)
        if op == "movw" and mw:
            low[(current, mw[1])] = int(mw[2], 0)
        elif op == "movt" and mw and (current, mw[1]) in low:
            value = (int(mw[2], 0) << 16) | low.pop((current, mw[1]))
            if value >> 30 and (value >> 8) & 0xFF in (0x11, 0x34) and value not in result[current]:
                result[current].append(value)
    return result


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    reports = []
    for arg in sys.argv[1:]:
        path = Path(arg)
        names = exports(path)
        found = requests(path, names)
        reports.append({"path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest(),
                        "exports": {n: [decode(r) for r in rs] for n, rs in sorted(found.items()) if rs}})
    for report in reports:
        print(f"{report['path']} sha256 {report['sha256']}")
        for name, decoded in report["exports"].items():
            for d in decoded:
                flag = ""
                if name in DRIVER:
                    flag = "  == driver" if int(d["request"], 16) == DRIVER[name] else "  != driver (G06 sends %#010x)" % DRIVER[name]
                print(f"  {name:28} {d['request']} {d['direction']:10} type {d['type']:#04x} nr {d['number']:3} size {d['size']:4}{flag}")
        missing = [n for n in DRIVER if n not in report["exports"]]
        if missing:
            print("  no request recovered for:", ", ".join(missing))
    if len(reports) == 2:
        a, b = reports
        print("\nDifferences in the requests the driver uses:")
        same = True
        for name in DRIVER:
            ra = [d["request"] for d in a["exports"].get(name, [])]
            rb = [d["request"] for d in b["exports"].get(name, [])]
            if ra != rb:
                same = False
                print(f"  {name:28} {ra} vs {rb}")
        if same:
            print("  none: every driver request has the same number, so the same direction, type, number and size")
    out = Path("mi3-audio-requests.json")
    out.write_text(json.dumps(reports, indent=2) + "\n")
    print(f"\nwrote {out}")


if __name__ == "__main__":
    main()
