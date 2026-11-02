#!/usr/bin/env python3
"""Boot applications with the exact G06 kernel or explicit QEMU surrogates.

Profiles:
  fast      qemu-system-aarch64, HVF, -cpu host       (Mac dev loop, near native)
  faithful  exact dumped G06 kernel, partial G06 hardware model, ARM32
  virt32    Linux 6.18 + virtio surrogate for media regression tests

Interactive (window + speakers):   python3 test/run.py fast
Headless self-test (for agents/CI): python3 test/run.py fast --headless
  Screenshots the guest when it reports its display, presses Enter in the
  guest, records audio to WAV and reports the dominant frequency.
Start the replay server first so the guest (10.0.2.2) can reach it:
  python3 test/replay.py --player ... --fixtures ...
Other apps: --bin doomtv boots src/doomtv instead (add its WAD with
  --add-file assets/third_party/freedoom/freedoom1.wad:/doom/freedoom1.wad).
Headless scripts: --at SEC:shot[:NAME] and --at SEC:key:QCODE[+QCODE][:HOLD_MS]
  take screenshots / press keys at SEC seconds after QEMU starts, e.g.
  --at 8:shot:title --at 9:key:ret --at 12:key:up:2000 --at 15:shot:walk
"""
import argparse
import hashlib
import json
import math
import os
import pathlib
import socket
import struct
import subprocess
import sys
import tempfile
import threading
import time
import wave

ROOT = pathlib.Path(__file__).resolve().parents[1]
BUILD = ROOT / "assets" / "build"
sys.path.insert(0, str(ROOT / "tools/kernel"))
import identity as kernel_identity
PROFILES = {
    "faithful": dict(target="armv7-unknown-linux-musleabihf",
        kernel=BUILD / "kernel/g06/Image",
        qemu=str(BUILD / "qemu-g06/build/qemu-system-aarch64"), machine=["-M", "g06"]),
    "fast": dict(qemu="qemu-system-aarch64", target="aarch64-unknown-linux-musl",
                 kernel=BUILD / "kernel/arm64/Image",
                 machine=["-M", "virt-11.1", "-accel", "hvf", "-cpu", "host"]),
    "virt32": dict(qemu="qemu-system-aarch64", target="armv7-unknown-linux-musleabihf",
                     kernel=BUILD / "kernel/arm/zImage",
                     machine=["-M", "virt-11.1,highmem=off,gic-version=2",
                              "-cpu", "cortex-a55,aarch64=off"]),
}


def build_app(name, target):
    if name in ("doomtv", "slides"):
        output = BUILD / "app-target" / target / "release" / name
        subprocess.run(["bash", str(ROOT / f"tools/{name}/build.sh"), target, str(output)], cwd=ROOT, check=True)
    else:
        subprocess.run(["cargo", "build", "--release", "--target", target, "-p", name], cwd=ROOT / "src", check=True)


class Qmp:
    def __init__(self, path):
        for _ in range(100):
            try:
                self.s = socket.socket(socket.AF_UNIX)
                self.s.connect(path)
                break
            except OSError:
                time.sleep(0.1)
        self.f = self.s.makefile("rw")
        self.f.readline()  # greeting
        self.cmd("qmp_capabilities")

    def cmd(self, name, **args):
        self.f.write(json.dumps({"execute": name, "arguments": args}) + "\n")
        self.f.flush()
        while True:
            r = json.loads(self.f.readline())
            if "return" in r or "error" in r:
                return r


def tone_report(path):
    # QEMU leaves the RIFF/data sizes at 0 when the guest powers off, so read
    # the canonical 44-byte header fields directly and take the rest as samples.
    raw = path.read_bytes()
    ch, rate = struct.unpack_from("<HI", raw, 22)
    data = raw[44:]
    data = data[:len(data) - len(data) % (2 * ch)]
    n = len(data) // (2 * ch)
    left = struct.unpack(f"<{len(data) // 2}h", data)[::ch]
    loud = [s for s in left if abs(s) > 200]
    if not loud:
        return f"{path.name}: {n} frames @ {rate} Hz, silent"
    first = next(i for i, s in enumerate(left) if abs(s) > 200)
    last = len(left) - next(i for i, s in enumerate(reversed(left)) if abs(s) > 200)
    seg = left[first:last]
    crossings = sum(1 for a, b in zip(seg, seg[1:]) if a < 0 <= b)
    rms = math.sqrt(sum(s * s for s in seg) / len(seg))
    return (f"{path.name}: {n} frames @ {rate} Hz; sound for {len(seg) / rate:.2f}s, "
            f"~{crossings / (len(seg) / rate):.1f} Hz, RMS {rms:.0f}")


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("profile", choices=PROFILES)
    ap.add_argument("--stock-smoke", action="store_true", help="stock-kernel ABI check followed by real tvinit")
    ap.add_argument("--bin", default="ytplay", help="binary crate to boot as /init (ytplay, doomtv)")
    ap.add_argument("--at", action="append", default=[], metavar="SEC:ACTION",
                    help="headless script step: SEC:shot[:NAME] or SEC:key:QCODE[+QCODE][:HOLD_MS]")
    ap.add_argument("--headless", action="store_true")
    ap.add_argument("--without", action="append", default=[], choices=["sound", "display"],
                    help="omit a surrogate device to test hardware bring-up failures")
    ap.add_argument("--no-build", action="store_true")
    ap.add_argument("--server", help="replay URL (defaults to the profile host address)")
    ap.add_argument("--video", default="PVGeM40dABA")
    ap.add_argument("--seconds", default="4")
    ap.add_argument("--mode", default="play", choices=["play", "browse", "framemd5", "selftest", "bench", "seccomp-test"])
    ap.add_argument("--frames", help="stop after N video frames")
    ap.add_argument("--add-file", action="append", default=[], metavar="HOST:GUEST",
                    help="extra file in the initramfs, e.g. for --mode bench")
    ap.add_argument("--extra", default="", help="extra kernel command line words")
    ap.add_argument("--hostfwd", action="append", default=[], metavar="HOSTPORT:GUESTPORT",
                    help="forward a host TCP port (127.0.0.1) to the guest, e.g. 17777:7777")
    ap.add_argument("--timeout", type=float, default=120)
    ap.add_argument("--out", type=pathlib.Path, default=BUILD / "qemu-runs")
    args = ap.parse_args()
    if args.profile == "faithful":
        kernel_identity.prepare(ROOT)
        if args.at or args.hostfwd or args.without:
            raise SystemExit("Stock G06 peripheral emulation is incomplete; use virt32 for explicit surrogate media/device tests")
        name = "tvinit" if args.stock_smoke else args.bin
        target = PROFILES["faithful"]["target"]
        if not args.no_build:
            build_app(name, target)
        runner = (ROOT / "assets").resolve().parent / "research/qemu-g06/run.py"
        extra = f"ytplay.mode={args.mode} ytplay.video={args.video} ytplay.server={args.server or 'http://192.168.77.1:8090'} " + args.extra
        if args.frames: extra += f" ytplay.frames={args.frames}"
        cmd = [sys.executable, str(runner), "--init", str(BUILD / "app-target" / target / "release" / name),
               "--timeout", str(args.timeout), "--out", str(args.out), "--extra", extra]
        if args.stock_smoke: cmd.append("--smoke")
        for f in args.add_file: cmd += ["--add-file", f]
        raise SystemExit(subprocess.run(cmd).returncode)
    if args.stock_smoke:
        raise SystemExit("--stock-smoke requires faithful (exact kernel)")
    p = PROFILES[args.profile]
    identities = kernel_identity.prepare(ROOT)
    active_kernel = identities["profiles"][args.profile]
    print(f"kernel: {active_kernel['version']} SURROGATE; NOT dumped G06 4.19.116+ "
          f"({active_kernel['sha256'][:12]} != {identities['stock_kernel_sha256'][:12]})", flush=True)
    faithful = args.profile == "virt32"
    host = "192.168.77.1" if faithful else "10.0.2.2"
    server = args.server or f"http://{host}:8090"
    # Only tvinit implements userspace network bring-up. Direct app tests still
    # need the QEMU kernel's IP_PNP helper; record that gap in fidelity.json.
    userspace_ip = faithful and args.bin == "tvinit"
    ip = ("ip=off" if userspace_ip else
          "ip=192.168.77.2::192.168.77.1:255.255.255.0::eth0:off" if faithful else "ip=dhcp")

    if not args.no_build:
        build_app(args.bin, p["target"])
    binary = BUILD / "app-target" / p["target"] / "release" / args.bin
    prefix = "" if args.bin == "ytplay" else f"{args.bin}-"
    run_dir = args.out / f"{prefix}{args.profile}-{time.strftime('%Y%m%dT%H%M%S')}-{time.time_ns()}"
    run_dir.mkdir(parents=True)
    initrd = run_dir / "initramfs.cpio"
    init_binary = binary
    extra_files = list(args.add_file)
    if faithful:
        # A modern QEMU kernel otherwise hides musl fallback bugs on 4.19.
        # This test-only trampoline execs the payload, preserving PID 1.
        init_binary = run_dir / "g06-init"
        subprocess.run(["rustc", "--edition=2021", "--target", p["target"],
                        "-C", "target-cpu=cortex-a55", "-C", "linker=rust-lld",
                        "-C", "opt-level=2", "-C", "panic=abort",
                        ROOT / "test/fixtures/g06-init.rs", "-o", init_binary],
                       cwd=ROOT / "src", check=True)
        extra_files.append(f"{binary}:/sbin/g06-payload")
    subprocess.run([sys.executable, ROOT / "tools/mkinitramfs.py", init_binary, initrd, *extra_files],
                   check=True)

    append = " ".join([
        "console=ttyAMA0", ip, "panic=-1",
        "drm_kms_helper.drm_fbdev_overalloc=200",  # room for double-buffered fbdev
        f"ytplay.server={server}", f"ytplay.video={args.video}",
        "tvinit.iface=eth0 tvinit.ip=192.168.77.2/24" if userspace_ip else "",
        f"ytplay.mode={args.mode}",
        f"ytplay.frames={args.frames}" if args.frames else "", args.extra])
    qmp_path = tempfile.mktemp(prefix="ytplay-qmp-", suffix=".sock")
    wav = run_dir / "audio.wav"
    audiodev = (f"wav,id=a0,path={wav},out.frequency=48000" if args.headless
                else "coreaudio,id=a0,out.frequency=48000")
    # Linux virtio-snd/gpu require modern (v1.x) virtio-mmio; QEMU defaults to legacy.
    cmd = [p["qemu"], *p["machine"], "-smp", "4", "-m", "1430M",
           "-global", "virtio-mmio.force-legacy=false",
           "-kernel", str(p["kernel"]), "-initrd", str(initrd), "-append", append,
           "-netdev", "user,id=n0" + (",net=192.168.77.0/24,host=192.168.77.1" if faithful else "") + "".join(f",hostfwd=tcp:127.0.0.1:{h}-:{g}" for h, g in
                                             (f.split(":") for f in args.hostfwd)),
           "-device", "virtio-net-device,netdev=n0",
           "-device", "virtio-gpu-device,xres=1280,yres=720",
           "-audiodev", audiodev, "-device", "virtio-sound-device,audiodev=a0,streams=1",
           "-device", "virtio-keyboard-device", "-device", "virtio-rng-device",
           "-serial", "stdio", "-no-reboot",
           "-qmp", f"unix:{qmp_path},server=on,wait=off",
           "-display", "none" if args.headless else "cocoa"]
    for kind, prefix in [("sound", "virtio-sound-device"), ("display", "virtio-gpu-device")]:
        if kind in args.without:
            at = next(i for i, word in enumerate(cmd) if word.startswith(prefix))
            del cmd[at - 1:at + 1]  # -device + its argument
    if faithful:
        # The observed target clock was wrong. Offline tests must not inherit
        # the Mac's valid TLS time; supplying trusted time is a separate gate.
        cmd += ["-rtc", "base=2025-01-17T00:00:00,clock=vm"]
    contract = json.loads((ROOT / "test/g06-firmware.json").read_text())
    contract.update(profile=args.profile, kernel_identity=active_kernel,
                    stock_kernel_sha256=identities["stock_kernel_sha256"], omitted_devices=args.without, userspace_static_ipv4=userspace_ip,
                    kernel_sha256=hashlib.sha256(p["kernel"].read_bytes()).hexdigest(),
                    init_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                    wall_clock="2025-01-17T00:00:00" if faithful else "host RTC",
                    post_419_syscalls="ENOSYS via test-only inherited seccomp filter" if faithful else "native",
                    framebuffer_pages="virtio double-buffer surrogate; target unmeasured")
    (run_dir / "fidelity.json").write_text(json.dumps(contract, indent=2) + "\n")
    print(f"profile: {args.profile}; MediaTek display/audio initialization is NOT emulated", flush=True)
    (run_dir / "cmd.txt").write_text(" ".join(map(str, cmd)) + "\n")
    print(f"run dir: {run_dir}", flush=True)
    t0 = time.time()
    q = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    qmp = Qmp(qmp_path)
    log = open(run_dir / "serial.log", "w")

    def act_on_display():
        time.sleep(1.0)
        png = run_dir / "screen.png"
        r = qmp.cmd("screendump", filename=str(png), format="png")
        print(f"[harness] screendump -> {png.name}: {'ok' if 'return' in r else r}", flush=True)
        r = qmp.cmd("send-key", keys=[{"type": "qcode", "data": "ret"}])
        print(f"[harness] sent Enter: {'ok' if 'return' in r else r}", flush=True)

    def script():
        # --at steps, in time order, relative to QEMU start (t0).
        steps = sorted((float(a.split(":", 1)[0]), a.split(":", 1)[1]) for a in args.at)
        for at, action in steps:
            time.sleep(max(0.0, at - (time.time() - t0)))
            kind, *rest = action.split(":")
            if kind == "shot":
                png = run_dir / f"{rest[0] if rest else f't{at:g}'}.png"
                r = qmp.cmd("screendump", filename=str(png), format="png")
                print(f"[harness] {time.time() - t0:.1f}s screendump -> {png.name}: "
                      f"{'ok' if 'return' in r else r}", flush=True)
            elif kind == "key":
                keys = [{"type": "qcode", "data": k} for k in rest[0].split("+")]
                hold = int(rest[1]) if len(rest) > 1 else 100
                r = qmp.cmd("send-key", keys=keys, **{"hold-time": hold})
                print(f"[harness] {time.time() - t0:.1f}s key {rest[0]} ({hold} ms): "
                      f"{'ok' if 'return' in r else r}", flush=True)

    if args.headless and args.at:
        threading.Thread(target=script, daemon=True).start()
    elif args.headless and args.mode == "play" and args.bin == "ytplay":
        def snap():
            png = run_dir / "screen.png"
            r = qmp.cmd("screendump", filename=str(png), format="png")
            print(f"[harness] screendump -> {png.name}: {'ok' if 'return' in r else r}", flush=True)
        threading.Timer(float(args.seconds), snap).start()
    timer = threading.Timer(args.timeout, q.kill)
    timer.start()
    for line in q.stdout:
        log.write(line)
        print(line, end="", flush=True)
        if args.headless and args.mode == "selftest" and "display: " in line and "x" in line \
                and "missing" not in line:
            threading.Thread(target=act_on_display, daemon=True).start()
    q.wait()
    timer.cancel()
    print(f"[harness] QEMU exited {q.returncode} after {time.time() - t0:.1f}s", flush=True)
    if args.headless and wav.exists():
        try:
            print("[harness] " + tone_report(wav))
        except (wave.Error, EOFError, StopIteration, struct.error) as e:
            print(f"[harness] audio: no usable WAV ({e})")
    try:
        os.unlink(qmp_path)
    except OSError:
        pass


if __name__ == "__main__":
    main()
