#!/usr/bin/env python3
"""Boot a prebuilt binary (default: a nolibc build) as PID 1 in QEMU.

Same machine, kernels and headless scripting as test/run.py (imported from
there), but nothing is built here: run nolibc/build.py first.

  python3 nolibc/run.py fast|faithful --bin tvinit [--flavor nolibc|musl] \
      [--add-file HOST:GUEST ...] [--add-app slides] --headless \
      --at 3:shot:home --at 4:key:down ...

--add-app NAME puts out/<flavor>/<target>/NAME at /bin/NAME (for tvinit's
tiles); --tiles adds the home screen art from assets/build/tiles.
Runs go to assets/build/nolibc/qemu-runs/<bin>-<flavor>-<profile>-<time>/.
"""
import argparse
import os
import pathlib
import subprocess
import sys
import tempfile
import threading
import time

ROOT = pathlib.Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / "test"))
import run as qrun  # noqa: E402  (test/run.py: Qmp, PROFILES, tone_report)

BUILD = ROOT / "assets/build"
OUT = BUILD / "nolibc"


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("profile", choices=qrun.PROFILES)
    ap.add_argument("--bin", default="tvinit")
    ap.add_argument("--flavor", default="nolibc", choices=["nolibc", "musl"])
    ap.add_argument("--binary", type=pathlib.Path, help="boot this file instead of out/<flavor>/<target>/<bin>")
    ap.add_argument("--add-app", action="append", default=[], metavar="NAME")
    ap.add_argument("--tiles", action="store_true")
    ap.add_argument("--add-file", action="append", default=[], metavar="HOST:GUEST")
    ap.add_argument("--at", action="append", default=[], metavar="SEC:ACTION")
    ap.add_argument("--headless", action="store_true")
    ap.add_argument("--extra", default="")
    ap.add_argument("--timeout", type=float, default=120)
    args = ap.parse_args()
    p = qrun.PROFILES[args.profile]
    outdir = OUT / "out" / args.flavor / p["target"]
    binary = args.binary or outdir / args.bin
    files = list(args.add_file)
    files += [f"{outdir / a}:/bin/{a}" for a in args.add_app]
    if args.tiles:
        files += [f"{t}:/share/tiles/{t.name}" for t in sorted((BUILD / "tiles").glob("*.rgb"))]
    run_dir = OUT / "qemu-runs" / f"{args.bin}-{args.flavor}-{args.profile}-{time.strftime('%Y%m%dT%H%M%S')}"
    run_dir.mkdir(parents=True)
    initrd = run_dir / "initramfs.cpio"
    subprocess.run([sys.executable, ROOT / "tools/mkinitramfs.py", binary, initrd, *files], check=True)
    append = " ".join(["console=ttyAMA0", "panic=-1", "drm_kms_helper.drm_fbdev_overalloc=200", args.extra])
    qmp_path = tempfile.mktemp(prefix="nolibc-qmp-", suffix=".sock")
    wav = run_dir / "audio.wav"
    audiodev = (f"wav,id=a0,path={wav},out.frequency=48000" if args.headless
                else "coreaudio,id=a0,out.frequency=48000")
    cmd = [p["qemu"], *p["machine"], "-smp", "4", "-m", "1430M",
           "-global", "virtio-mmio.force-legacy=false",
           "-kernel", str(p["kernel"]), "-initrd", str(initrd), "-append", append,
           "-netdev", "user,id=n0", "-device", "virtio-net-device,netdev=n0",
           "-device", "virtio-gpu-device,xres=1280,yres=720",
           "-audiodev", audiodev, "-device", "virtio-sound-device,audiodev=a0,streams=1",
           "-device", "virtio-keyboard-device", "-device", "virtio-rng-device",
           "-serial", "stdio", "-no-reboot",
           "-qmp", f"unix:{qmp_path},server=on,wait=off",
           "-display", "none" if args.headless else "cocoa"]
    (run_dir / "cmd.txt").write_text(" ".join(map(str, cmd)) + "\n")
    print(f"run dir: {run_dir}\nbinary: {binary} ({binary.stat().st_size} bytes)", flush=True)
    t0 = time.time()
    q = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    qmp = qrun.Qmp(qmp_path)
    log = open(run_dir / "serial.log", "w")

    def script():
        steps = sorted((float(a.split(":", 1)[0]), a.split(":", 1)[1]) for a in args.at)
        for at, action in steps:
            time.sleep(max(0.0, at - (time.time() - t0)))
            kind, *rest = action.split(":")
            if kind == "shot":
                png = run_dir / f"{rest[0] if rest else f't{at:g}'}.png"
                r = qmp.cmd("screendump", filename=str(png), format="png")
                msg = f"screendump -> {png.name}: {'ok' if 'return' in r else r}"
            elif kind == "key":
                keys = [{"type": "qcode", "data": k} for k in rest[0].split("+")]
                hold = int(rest[1]) if len(rest) > 1 else 100
                r = qmp.cmd("send-key", keys=keys, **{"hold-time": hold})
                msg = f"key {rest[0]} ({hold} ms): {'ok' if 'return' in r else r}"
            else:
                msg = f"unknown action {action}"
            line = f"[harness] {time.time() - t0:.1f}s {msg}"
            print(line, flush=True)
            log.write(line + "\n")

    if args.at:
        threading.Thread(target=script, daemon=True).start()
    timer = threading.Timer(args.timeout, q.kill)
    timer.start()
    for line in q.stdout:
        log.write(line)
        log.flush()
        print(line, end="", flush=True)
    q.wait()
    timer.cancel()
    print(f"[harness] QEMU exited {q.returncode} after {time.time() - t0:.1f}s", flush=True)
    log.write(f"[harness] QEMU exited {q.returncode} after {time.time() - t0:.1f}s\n")
    try:
        os.unlink(qmp_path)
    except OSError:
        pass


if __name__ == "__main__":
    main()
