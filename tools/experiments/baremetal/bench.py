#!/usr/bin/env python3
"""Repeatable measurements for the bare-metal spike, via run.py:
boot-to-first-pixel (N boots per accelerator), uncapped frame rate on 1 and
4 cores, and host CPU while idling in WFI vs busy polling.

  python3 baremetal/bench.py [--boots 5] [--accel tcg hvf]

Writes assets/build/baremetal/bench-<time>.json and prints a table.
"""
import argparse
import json
import pathlib
import re
import statistics
import subprocess
import sys
import time

ROOT = pathlib.Path(__file__).resolve().parents[3]
OUT = ROOT / "assets" / "build" / "baremetal"


def run(accel, name, append):
    p = subprocess.run([sys.executable, str(ROOT / "tools/experiments/baremetal/run.py"), accel, "--no-build",
                        "--name", name, "--append", append, "--timeout", "40"],
                       capture_output=True, text=True)
    m = re.search(r"run dir: (\S+)", p.stdout)
    if not m:
        sys.exit(f"run.py failed:\n{p.stdout[-2000:]}\n{p.stderr[-2000:]}")
    run_dir = pathlib.Path(m.group(1))
    s = json.loads((run_dir / "summary.json").read_text())
    s["run_dir"] = str(run_dir.relative_to(ROOT))
    m = re.search(r"([\d.]+) fps \(target", s.get("fps_line") or "")
    s["fps"] = float(m.group(1)) if m else None
    m = re.search(r"draw avg ([\d.]+) .* flush avg ([\d.]+)", s.get("fps_line") or "")
    s["draw_ms"], s["flush_ms"] = (float(m.group(1)), float(m.group(2))) if m else (None, None)
    return s


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--boots", type=int, default=5)
    ap.add_argument("--accel", nargs="+", default=["tcg", "hvf"])
    args = ap.parse_args()
    subprocess.run(["cargo", "build", "--release"], cwd=ROOT / "research/experiments/baremetal", check=True)
    res = {"time": time.strftime("%Y-%m-%dT%H:%M:%S"), "loadavg_before": None, "runs": {}}
    res["loadavg_before"] = subprocess.run(["sysctl", "-n", "vm.loadavg"], capture_output=True,
                                           text=True).stdout.strip()
    rows = []
    for accel in args.accel:
        for quiet in ["0", "1"]:
            boots = [run(accel, f"bench-boot-q{quiet}", f"seconds=0.5 net=0 sound=0 quiet={quiet}")
                     for _ in range(args.boots)]
            res["runs"][f"{accel}-boot-quiet{quiet}"] = boots
            for key in ["guest_ms_to_rust", "guest_ms_to_first_pixel", "host_s_to_first_pixel"]:
                v = [float(b[key]) for b in boots if b.get(key)]
                rows.append(f"{accel} quiet={quiet} {key}: median {statistics.median(v):.3f} "
                            f"min {min(v):.3f} max {max(v):.3f} (n={len(v)})")
        for cores in [1, 4]:
            s = run(accel, f"bench-uncapped-smp{cores}", f"seconds=3 fps=100000 smp={cores} net=0 sound=0")
            res["runs"][f"{accel}-uncapped-smp{cores}"] = s
            rows.append(f"{accel} uncapped smp={cores}: {s['fps']} fps, draw {s['draw_ms']} ms, "
                        f"flush {s['flush_ms']} ms  [{s['run_dir']}]")
        for idle in ["wfi", "spin"]:
            s = run(accel, f"bench-idle-{idle}", f"seconds=4 idle={idle} net=0 sound=0")
            res["runs"][f"{accel}-idle-{idle}"] = s
            rows.append(f"{accel} 60 fps idle={idle}: {s['fps']} fps, host CPU {s['host_cpu_s']} s "
                        f"in {s['wall_s']} s ({100 * s['host_cpu_s'] / s['wall_s']:.0f}% of a core)"
                        f"  [{s['run_dir']}]")
    res["loadavg_after"] = subprocess.run(["sysctl", "-n", "vm.loadavg"], capture_output=True,
                                          text=True).stdout.strip()
    res["table"] = rows
    path = OUT / f"bench-{time.strftime('%Y%m%dT%H%M%S')}.json"
    path.write_text(json.dumps(res, indent=2) + "\n")
    print("\n".join(rows))
    print(f"load average before {res['loadavg_before']} after {res['loadavg_after']}")
    print(f"wrote {path.relative_to(ROOT)}")


if __name__ == "__main__":
    main()
