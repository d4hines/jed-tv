#!/usr/bin/env python3
"""Boot the bare-metal tvbare ELF on QEMU virt and collect evidence.

  python3 baremetal/run.py tcg|hvf [--append "seconds=10 smp=4"] \\
      [--at SEC:shot[:NAME]] [--at SEC:key:QCODE[+QCODE][:HOLD_MS]] [--name NAME]

Same virtio-mmio devices as test/run.py (gpu 1280x720, keyboard, net,
sound, rng), modern virtio-mmio. Reuses test/run.py's QMP client and WAV
tone report. Writes assets/build/baremetal/runs/<name>-<accel>-<time>/ with
cmd.txt, serial.log (host-timestamped), screenshots, audio.wav, summary.json.
--at times are seconds after QEMU starts.
"""
import argparse
import hashlib
import importlib.util
import json
import pathlib
import re
import resource
import shutil
import subprocess
import sys
import tempfile
import threading
import time
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parents[3]
OUT = ROOT / "assets" / "build" / "baremetal"
ELF = OUT / "target" / "aarch64-unknown-none" / "release" / "tvbare"
ACCEL = {
    "tcg": ["-M", "virt", "-cpu", "cortex-a55"],
    "hvf": ["-M", "virt", "-accel", "hvf", "-cpu", "host"],
}

_spec = importlib.util.spec_from_file_location("qemu_run", ROOT / "test" / "run.py")
qemu_run = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(qemu_run)


def fnv1a64(data):
    h = 0xcbf29ce484222325
    for b in data:
        h = ((h ^ b) * 0x100000001b3) & 0xFFFFFFFFFFFFFFFF
    return h


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("accel", choices=ACCEL)
    ap.add_argument("--append", default="", help="boot arguments (see src/main.rs)")
    ap.add_argument("--at", action="append", default=[], metavar="SEC:ACTION")
    ap.add_argument("--name", default="run")
    ap.add_argument("--machine-opts", default="", help="extra -M options, e.g. gic-version=3")
    ap.add_argument("--no-build", action="store_true")
    ap.add_argument("--timeout", type=float, default=90)
    args = ap.parse_args()

    if not args.no_build:
        subprocess.run(["cargo", "build", "--release"], cwd=ROOT / "research/experiments/baremetal", check=True)
    stamp = time.strftime('%Y%m%dT%H%M%S') + f".{int(time.time() * 1000) % 1000:03d}"
    run_dir = OUT / "runs" / f"{args.name}-{args.accel}-{stamp}"
    run_dir.mkdir(parents=True)
    elf = run_dir / "tvbare.elf"
    shutil.copy(ELF, elf)
    qmp_path = tempfile.mktemp(prefix="tvbare-qmp-", suffix=".sock")
    wav = run_dir / "audio.wav"
    machine = list(ACCEL[args.accel])
    if args.machine_opts:
        machine[1] += "," + args.machine_opts
    cmd = ["qemu-system-aarch64", *machine, "-smp", "4", "-m", "1430M",
           "-global", "virtio-mmio.force-legacy=false",
           "-kernel", str(elf), "-append", args.append,
           "-netdev", "user,id=n0", "-device", "virtio-net-device,netdev=n0",
           "-device", "virtio-gpu-device,xres=1280,yres=720",
           "-audiodev", f"wav,id=a0,path={wav},out.frequency=48000",
           "-device", "virtio-sound-device,audiodev=a0,streams=1",
           "-device", "virtio-keyboard-device", "-device", "virtio-rng-device",
           "-serial", "stdio", "-no-reboot", "-display", "none",
           "-qmp", f"unix:{qmp_path},server=on,wait=off"]
    (run_dir / "cmd.txt").write_text(" ".join(map(str, cmd)) + "\n")
    print(f"run dir: {run_dir}", flush=True)
    ru0 = resource.getrusage(resource.RUSAGE_CHILDREN)
    t0 = time.time()
    q = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    log = open(run_dir / "serial.log", "w")
    lines, first_pixel, exited = [], [], []

    def reader():  # timestamp serial lines from the moment QEMU is spawned
        for line in q.stdout:
            t = time.time() - t0
            line = line.rstrip("\r\n")
            lines.append(line)
            log.write(f"+{t:7.3f} {line}\n")
            print(f"+{t:7.3f} {line}", flush=True)
            if not first_pixel and "first frame flushed" in line:
                first_pixel.append(t)
        q.wait()
        exited.append(time.time() - t0)

    serial = threading.Thread(target=reader)
    serial.start()
    events = []
    try:
        qmp = qemu_run.Qmp(qmp_path)
    except (OSError, ValueError, json.JSONDecodeError) as e:  # QEMU already gone (short runs)
        qmp, args.at = None, []
        events.append(f"[harness] no QMP: {e!r}")

    def script():
        steps = sorted((float(a.split(":", 1)[0]), a.split(":", 1)[1]) for a in args.at)
        for at, action in steps:
            time.sleep(max(0.0, at - (time.time() - t0)))
            kind, *rest = action.split(":")
            if kind == "shot":
                png = run_dir / f"{rest[0] if rest else f't{at:g}'}.png"
                r = qmp.cmd("screendump", filename=str(png), format="png")
            elif kind == "key":
                keys = [{"type": "qcode", "data": k} for k in rest[0].split("+")]
                hold = int(rest[1]) if len(rest) > 1 else 100
                r = qmp.cmd("send-key", keys=keys, **{"hold-time": hold})
            line = f"[harness +{time.time() - t0:7.3f}] {action}: {'ok' if 'return' in r else r}"
            events.append(line)
            print(line, flush=True)

    threading.Thread(target=script, daemon=True).start()
    timer = threading.Timer(args.timeout, q.kill)
    timer.start()
    serial.join()
    q.wait()
    timer.cancel()
    wall = exited[0] if exited else time.time() - t0
    ru1 = resource.getrusage(resource.RUSAGE_CHILDREN)
    cpu = (ru1.ru_utime - ru0.ru_utime) + (ru1.ru_stime - ru0.ru_stime)
    log.write("\n".join(events) + "\n")
    log.close()
    print(f"[harness] QEMU exited {q.returncode} after {wall:.2f}s; host CPU {cpu:.2f}s "
          f"({100 * cpu / wall:.0f}% of one core)", flush=True)

    text = "\n".join(lines)
    summary = {"accel": args.accel, "append": args.append, "exit": q.returncode,
               "wall_s": round(wall, 3), "host_cpu_s": round(cpu, 3),
               "host_s_to_first_pixel": first_pixel and round(first_pixel[0], 3),
               "elf_bytes": elf.stat().st_size,
               "elf_sha256": hashlib.sha256(elf.read_bytes()).hexdigest()}
    for key, pat in [("guest_ms_to_rust", r"Rust entry at ([\d.]+) ms"),
                     ("guest_ms_to_first_pixel", r"first frame flushed ([\d.]+) ms after reset"),
                     ("fps_line", r"frame: (\d+ frames in .*)"),
                     ("net_line", r"net: (body .*)"), ("sound_line", r"sound: (played .*)"),
                     ("idle_line", r"idle: (.*)")]:
        m = re.search(pat, text)
        summary[key] = m and m.group(1)
    m = re.search(r"net: body (\d+) bytes .* FNV-1a64 ([0-9a-f]{16})", text)
    if m:
        rng = re.search(r"range=(\S+)", args.append)
        rng = rng.group(1) if rng else "0-"
        req = urllib.request.Request("http://127.0.0.1:8090/videoplayback?itag=140",
                                     headers={"Range": f"bytes={rng}"})
        body = urllib.request.urlopen(req, timeout=10).read()
        ok = len(body) == int(m.group(1)) and f"{fnv1a64(body):016x}" == m.group(2)
        summary["net_host_check"] = f"host fetched {len(body)} bytes, FNV match: {ok}"
        print(f"[harness] {summary['net_host_check']}")
    if wav.exists():
        try:
            summary["audio"] = qemu_run.tone_report(wav)
        except Exception as e:  # noqa: BLE001 - report, don't fail the run
            summary["audio"] = f"no usable WAV ({e})"
        print(f"[harness] {summary['audio']}")
    (run_dir / "summary.json").write_text(json.dumps(summary, indent=2) + "\n")
    try:
        pathlib.Path(qmp_path).unlink()
    except OSError:
        pass


if __name__ == "__main__":
    main()
