#!/usr/bin/env python3
"""Run every gate headless and print a PASS/FAIL table.
  python3 test/check.py            # all local gates (~3-4 min)
  python3 test/check.py --quick    # skip the slow 32-bit (virt32) runs
  python3 test/check.py --live     # also play 3 s from real YouTube (residential IP only!)
  python3 test/check.py -k doom    # only checks whose name contains "doom"
Starts the replay server if it isn't running. Exit status 1 if anything fails.
"""
import argparse
import glob
import pathlib
import re
import socket
import subprocess
import sys
import time
import run as qrun

ROOT = pathlib.Path(__file__).resolve().parents[1]
TEST = ROOT / "test"
TOOLS = ROOT / "tools"
FIXTURE = sorted(glob.glob(str(ROOT / "assets/yt-stage0/record-*/fixtures/136.mp4")))[-1]
WAD = ROOT / "assets/third_party/freedoom/freedoom1.wad"
PLAYER = sorted(glob.glob(str(ROOT / "assets/yt-stage0/minimal-PVGeM40dABA-*/player.visitor-header.json")))[-1]


def sh(cmd, timeout=600, cwd=ROOT):
    p = subprocess.run([str(c) for c in cmd], capture_output=True, text=True, timeout=timeout, cwd=cwd)
    return p.returncode, p.stdout + p.stderr


def boot(profile, *args, timeout=120):
    """test/run.py headless without rebuilding; returns (serial log text, run dir)."""
    _, out = sh([sys.executable, TEST / "run.py", profile, "--headless", "--no-build",
                 "--timeout", str(timeout), *args], timeout=timeout + 60)
    m = re.search(r"run dir: (\S+)", out)
    d = pathlib.Path(m.group(1)) if m else None
    log = (d / "serial.log").read_text(errors="replace") if d and (d / "serial.log").exists() else out
    return log + "\n" + out, d


def tv(profile, *args, timeout=40):
    _, out = sh([TEST / "tv.sh", profile, "--headless", "--timeout", str(timeout), *args],
                timeout=timeout + 300)
    m = re.search(r"run dir: (\S+)", out)
    return out, (pathlib.Path(m.group(1)) if m else None)


def need(text, *patterns):
    missing = [p for p in patterns if not re.search(p, text)]
    return (not missing, "missing: " + "; ".join(missing) if missing else "")


# ---------------- checks: each returns (ok, detail, evidence) ----------------

def c_stock_kernel():
    code, out = sh([sys.executable, TEST / "run.py", "faithful", "--stock-smoke", "--no-build", "--timeout", "60"], timeout=90)
    ok = code == 0 and "G06-SMOKE ABI PASS" in out and "cpus=4" in out and "tvinit: hold:" in out
    return ok, "exact dumped 4.19 kernel; four CPUs, threads/timers/fork/wait and real tvinit" if ok else out[-700:], None


def c_unit_tests():
    code, out = sh(["cargo", "test", "-p", "mp4"], cwd=ROOT / "src")
    passed = sum(int(n) for n in re.findall(r"test result: \w+\. (\d+) passed", out))
    lean_code, lean_out = sh(["python3", ROOT / "test/slides/check.py"])
    return code == 0 and lean_code == 0 and passed > 0, f"{passed} Rust tests; Lean slide parser/layout/render checks", None


def c_build(target):
    def run():
        code, out = sh(["cargo", "build", "--release", "--target", target,
                        "-p", "platform", "-p", "tvinit", "-p", "ytplay"], cwd=ROOT / "src")
        if code == 0:
            release = ROOT / "assets/build/app-target" / target / "release"
            code, out = sh(["env", f"PLATFORM_LIB={release / 'libplatform.a'}", "bash",
                            ROOT / "tools/doomtv/build.sh", target, release / "doomtv"])
        if code == 0:
            code, out = sh(["env", f"PLATFORM_LIB={release / 'libplatform.a'}", "bash",
                            ROOT / "tools/slides/build.sh", target, release / "slides"])
        return code == 0, "" if code == 0 else out[-400:], None
    return run


def c_framemd5(profile, frames=None):
    def run():
        args = ["--mode", "framemd5"] + (["--frames", str(frames)] if frames else [])
        log, d = boot(profile, *args, timeout=300)
        if not d:
            return False, "no run", None
        code, out = sh([sys.executable, TEST / "check_framemd5.py", d / "serial.log", FIXTURE,
                        *([str(frames)] if frames else [])])
        return code == 0, out.strip(), d
    return run


def c_play(profile):
    def run():
        log, d = boot(profile, "--mode", "play", "--frames", "240", "--seconds", "5")
        m = re.search(r"video: (\d+) shown, (\d+) dropped", log)
        s = re.search(r"max \|([\d.]+)\| ms", log)
        u = re.search(r"(\d+) underruns", log)
        if not (m and s and u):
            return False, "no stats", d
        shown, dropped, maxoff, under = int(m[1]), int(m[2]), float(s[1]), int(u[1])
        ok = dropped <= 3 and maxoff < 50 and under == 0 if profile == "fast" else under == 0
        return ok, f"{shown} shown, {dropped} dropped, max A/V {maxoff:.0f} ms, {under} underruns", d
    return run


def c_controls(profile):
    """Pause, seek while paused, resume, jump to 50%, back 10 s, quit."""
    def run():
        log, d = boot(profile, "--mode", "play", "--at", "3:key:ret", "--at", "5.5:key:right",
                      "--at", "7:key:ret", "--at", "10:key:5", "--at", "13:key:left",
                      "--at", "15:shot:after", "--at", "16:key:esc", timeout=40)
        ok, why = need(log, r"\] paused", r"seek to 0:1\d", r"\] resumed", r"seek to 1:32",
                       r"seek to 1:2\d", r"seek: 3 seeks", r" 0 underruns", r"video: stopped by remote")
        if "write failed" in log or "play: error" in log:
            ok, why = False, "audio device error"
        return ok, why or "pause, seek while paused, resume, 50% jump, -10 s, quit", d
    return run


def c_selftest(profile):
    def run():
        log, d = boot(profile, "--mode", "selftest")
        ok, why = need(log, r"display: \S+ 1280x720", r"audio: played 144000 frames",
                       r"input: key 28 pressed", r"net:   header\+index \d+ bytes")
        return ok, why or "display, audio, input, network ok", d
    return run


def c_seccomp_trap(profile):
    def run():
        log, d = boot(profile, "--mode", "seccomp-test", timeout=60)
        ok, why = need(log, r"seccomp: allowlist of \d+ syscalls", r"seccomp: blocked syscall \d+")
        return ok, why or "stray getuid() trapped", d
    return run


def c_doom():
    if not WAD.exists():
        return False, f"missing {WAD}", None
    log, d = boot("fast", "--bin", "doomtv", "--add-file", f"{WAD}:/doom/freedoom1.wad",
                  "--at", "3:shot:title", timeout=8)
    ok, why = need(log, r"doom: engine up")
    ok = ok and "PANIC" not in log
    return ok, why or "engine up, no panic", d


def c_slides():
    code, out = sh([sys.executable, ROOT / "test/slides/check-qemu.py"], timeout=600)
    return code == 0, out[-800:], None


def c_tv_tour():
    # Use the maintained app tour: it includes the nested YouTube screen,
    # Lean launch gate and the current menu shape rather than old tile offsets.
    code, out = sh(["nix", "run", ".#pid1", "--", "--check"], timeout=900)
    match = re.search(r"Evidence: (\S+)", out)
    evidence = pathlib.Path(match.group(1)) if match else None
    return code == 0 and "PASS: graphical menu" in out, out[-700:], evidence


def c_tv_boot_m1a():
    code, out = sh([sys.executable, TOOLS / "make_tv_boot.py", "m1a"])
    if code != 0:
        return False, out[-300:], None
    rd = ROOT / "assets/build/tv-boot/m1a/ramdisk.cpio.gz"
    p = qrun.PROFILES["virt32"]
    cmd = [p["qemu"], *p["machine"], "-smp", "4",
           "-m", "1430M", "-global", "virtio-mmio.force-legacy=false",
           "-kernel", p["kernel"], "-initrd", rd,
           "-append", "console=ttyAMA0 panic=-1", "-netdev", "user,id=n0",
           "-device", "virtio-net-device,netdev=n0", "-serial", "stdio", "-display", "none", "-no-reboot"]
    try:
        out = subprocess.run([str(c) for c in cmd], capture_output=True, text=True, timeout=12).stdout
    except subprocess.TimeoutExpired as e:
        out = (e.stdout or b"").decode(errors="replace")
    ok, why = need(out, r"tvinit: network: eth0 = 192\.168\.77\.2/24", r"tvinit: hold")
    return ok, why or "TV ramdisk userspace boots on virt32 surrogate; this is not a stock-kernel gate", rd.parent


def c_g06_bringup():
    """Exercise PID 1 without kernel DHCP or a valid host wall clock."""
    log, d = boot("virt32", "--bin", "tvinit", "--extra", "tvinit.hold=1", timeout=8)
    ok, why = need(log, r"tvinit: network: eth0 = 192\.168\.77\.2/24",
                   r"setting system clock to 2025-01-17T", r"tvinit: hold",
                   r"g06-compat: ARM32 post-4.19 syscalls return ENOSYS; legacy clock/thread fallback passed")
    if d:
        import json
        fidelity = json.loads((d / "fidelity.json").read_text())
        ok = ok and fidelity["userspace_static_ipv4"] and "ip=off" in (d / "cmd.txt").read_text()
    return ok, why or "PID 1 configures static IPv4 without kernel autoconfig; stale RTC", d


def c_g06_home():
    target = ROOT / "assets/build/app-target/armv7-unknown-linux-musleabihf/release"
    log, d = boot("virt32", "--bin", "tvinit", "--add-file", f"{target / 'ytplay'}:/bin/ytplay",
                  "--at", "3:key:right", "--at", "4:key:ret", "--at", "9:key:home",
                  "--at", "11:shot:home", timeout=13)
    ok, why = need(log, r"tvinit: network: eth0 = 192\.168\.77\.2/24",
                   r"starting Coffee Run", r"video: \d+ segments", r"Coffee Run stopped")
    ok = ok and d is not None and (d / "home.png").exists()
    return ok, why or "static-IP PID 1 launches offline video and Home returns to menu", d


def c_tv_child_stop():
    """Home must escalate without a watcher thread, even if TERM is ignored."""
    target = "armv7-unknown-linux-musleabihf"
    fixture = ROOT / "assets/build/tvinit-shrink/uncooperative-app"
    fixture.parent.mkdir(parents=True, exist_ok=True)
    code, out = sh(["rustc", "--edition=2021", "--target", target,
                    "-C", "linker=rust-lld", "-C", "panic=abort", "-C", "opt-level=2",
                    TEST / "tvinit-child.rs", "-o", fixture], cwd=ROOT / "src")
    if code:
        return False, out[-400:], None
    log, d = boot("virt32", "--bin", "tvinit", "--add-file", f"{fixture}:/bin/ytplay",
                  "--at", "3:key:right", "--at", "4:key:ret", "--at", "6:key:home",
                  "--at", "10:shot:home", timeout=12)
    ok, why = need(log, r"fixture: uncooperative app running", r"fixture: ignored SIGTERM",
                   r"home button: stopping the app", r"Coffee Run stopped after [3-5]s")
    return ok, why or "Home escalates TERM to KILL, reaps the app and returns to the menu", d


def c_audio_missing():
    log, d = boot("virt32", "--mode", "play", "--without", "sound", timeout=12)
    ok, why = need(log, r"play: error:", r"QEMU exited 0")
    return ok, why or "missing PCM device reports an error and exits without waiting for an audio clock", d


def c_short_bench():
    fixture = ROOT / "assets/build/tvinit-shrink/four-frames.mp4"
    fixture.parent.mkdir(parents=True, exist_ok=True)
    code, out = sh(["ffmpeg", "-v", "error", "-f", "lavfi", "-i", "testsrc2=size=128x72:rate=24",
                    "-frames:v", "4", "-c:v", "libx264", "-bf", "3", "-pix_fmt", "yuv420p",
                    "-movflags", "dash+frag_keyframe+empty_moov+default_base_moof", "-y", fixture])
    if code:
        return False, out[-400:], None
    for profile in ["fast", "virt32"]:
        log, d = boot(profile, "--mode", "bench", "--frames", "4", "--add-file",
                      f"{fixture}:/fixture.mp4", "--extra", "ytplay.file=/fixture.mp4", timeout=20)
        if (set(re.findall(r"bench: decode only, threads=(\d+).*: 4 frames", log)) != {"1", "2", "4"}
                or "bench: convert 128x72" not in log
                or any(error in log for error in ["PANIC", "bench: error:", "nan ms", "inf ms"])):
            return False, f"{profile}: short-clip decode/convert did not complete", d
    return True, "four B-frames drain correctly and conversion completes on both architectures", d


def c_display_missing():
    log, d = boot("virt32", "--bin", "tvinit", "--without", "display", timeout=5)
    ok, why = need(log, r"no display .*retrying without launching an app",
                   r"tvinit: network: eth0 = 192\.168\.77\.2/24")
    ok = ok and "tvinit: starting " not in log and "PANIC" not in log
    return ok, why or "missing framebuffer retries with diagnostics, without an app respawn loop", d


def c_live():
    log, d = boot("fast", "--mode", "play", "--server", "https://www.youtube.com", "--frames", "72")
    m = re.search(r"video: (\d+) shown, (\d+) dropped", log)
    ok = bool(m) and int(m[1]) > 60 and "seccomp: allowlist" in log
    return ok, (f"{m[1]} shown, {m[2]} dropped over TLS" if m else "no playback"), d


def replay_up():
    try:
        socket.create_connection(("127.0.0.1", 8090), timeout=0.5).close()
        return True
    except OSError:
        return False


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--quick", action="store_true")
    ap.add_argument("--live", action="store_true")
    ap.add_argument("-k", default="")
    a = ap.parse_args()

    checks = [
        ("stock: exact kernel and PID1 ABI smoke", c_stock_kernel),
        ("host: unit tests (mp4, slides)", c_unit_tests),
        ("build: all apps, arm64", c_build("aarch64-unknown-linux-musl")),
        ("build: all apps, armv7", c_build("armv7-unknown-linux-musleabihf")),
        ("fast: framemd5 all 4431 frames bit-exact", c_framemd5("fast")),
        ("fast: play 10 s, A/V sync", c_play("fast")),
        ("fast: remote controls (pause/seek/jump)", c_controls("fast")),
        ("fast: platform self-test", c_selftest("fast")),
        ("fast: seccomp traps a stray syscall", c_seccomp_trap("fast")),
        ("fast: doom boots", c_doom),
        ("fast: slides render and navigate", c_slides),
        ("fast: home screen tour (YouTube, Home, Slides, Doom)", c_tv_tour),
        ("artifact: m1a package plus virt32 userspace smoke", c_tv_boot_m1a),
        ("virt32 G06-constraints: userspace network and stale clock (32-bit)", c_g06_bringup),
        ("virt32 G06-constraints: launcher stops uncooperative app", c_tv_child_stop),
        ("virt32 G06-constraints: missing audio fails promptly", c_audio_missing),
        ("virt32 G06-constraints: missing display does not launch apps", c_display_missing),
        ("virt32 G06-constraints: short reordered-video benchmark", c_short_bench),
    ]
    if not a.quick:
        checks += [
            ("virt32: framemd5 first 240 frames bit-exact", c_framemd5("virt32", 240)),
            ("virt32: platform self-test", c_selftest("virt32")),
            ("virt32 G06-constraints: home and offline video (32-bit)", c_g06_home),
            ("virt32: seccomp traps a stray syscall", c_seccomp_trap("virt32")),
            ("virt32: play (no underruns)", c_play("virt32")),
            ("virt32: remote controls (pause/seek/jump)", c_controls("virt32")),
        ]
    if a.live:
        checks.append(("live: 3 s from real YouTube over TLS", c_live))
    checks = [c for c in checks if a.k.lower() in c[0].lower()]

    server = None
    if not replay_up():
        fixtures = pathlib.Path(FIXTURE).parent
        server = subprocess.Popen([sys.executable, TEST / "replay.py", "--player", PLAYER,
                                   "--fixtures", fixtures, "--host", "127.0.0.1", "--port", "8090"],
                                  stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        time.sleep(1)
    results = []
    try:
        for name, fn in checks:
            t0 = time.time()
            try:
                ok, detail, evidence = fn()
            except Exception as e:  # a crashed check is a failed check
                ok, detail, evidence = False, f"{type(e).__name__}: {e}", None
            dt = time.time() - t0
            results.append((name, ok, detail, evidence, dt))
            print(f"{'PASS' if ok else 'FAIL'}  {dt:5.1f}s  {name}  -- {detail}", flush=True)
    finally:
        if server:
            server.terminate()
    failed = [r for r in results if not r[1]]
    print(f"\n{len(results) - len(failed)}/{len(results)} passed")
    for name, ok, detail, ev, _ in failed:
        print(f"  FAIL {name}: {detail} ({ev})")
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
