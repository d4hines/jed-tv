#!/usr/bin/env python3
"""Interactive offline UI session: embedded PID1 launcher in headless QEMU.

  session.py start NAME --apps DIR [--init PID1] [--production]   # blocks; run in background
  session.py run NAME 'right ret sleep:2 shot:slides wait:marker log:20'
  session.py stop NAME

The guest runs the real ARM32 PID1 (embedded launcher, global volume thread)
behind the G06 syscall trampoline, with virtio-gpu, virtio-keyboard and a
WAV-file sound backend. QEMU is headless; screenshots come from QMP. YouTube
traffic goes to a local fixture server (test/replay.py plus a generated
search fixture); nothing in this session is live YouTube data.

Script words for `run`:
  KEY            press and release a QEMU qcode (right, ret, esc, ac_back, home...)
  KEY*N          press N times
  hold:KEY:SECS  hold a key down (guest autorepeat) then release
  down:KEY / up:KEY  raw transitions
  sleep:SECS     wait
  shot:NAME      screendump to <session>/shots/NAME.png
  wait:TEXT[@N[@TIMEOUT]]  wait until serial log contains TEXT N times (_ = space)
  next:TEXT[@TIMEOUT]      wait for one more TEXT than seen when the script started
                           (each next: of the same TEXT waits for another one); fails on timeout
  log:N          print the last N serial lines
  sh:COMMAND     run a command on the guest root shell, print output
  get:/path      GET from the PID1 diagnostics server
"""
import argparse
import http.client
import http.server
import io
import json
import os
from pathlib import Path
import re
import socket
import subprocess
import sys
import threading
import time

ROOT = Path(__file__).resolve().parents[2]
PARENT = ROOT.parents[2] if ROOT.parts[-3:-1] == ("assets", "build") else ROOT
MEDIA = Path("/nix/store/8574yn23zqyq4838183q47bmxdfz6i0l-pid1-demo-media")
INIT = PARENT / "assets/build/pid1-repl/target/armv7-unknown-linux-musleabihf/release/hello-pid1"
BASE = ROOT / "assets/build/ui-audit"
sys.path.insert(0, str(ROOT / "tools/qemu"))
sys.path.insert(0, str(ROOT / "test"))

# Search fixture: clearly labelled synthetic results, not YouTube data.
FIXTURE_VIDEOS = [
    ("fixAAAAAAA1", "[fixture] Result one - plays generated demo media", "Fixture channel A", "3:04"),
    ("fixAAAAAAA2", "[fixture] Result two", "Fixture channel B", "10:34"),
    ("fixAAAAAAA3", "[fixture] Result three with a deliberately long title that should be clipped at the screen edge", "Fixture channel C with a long name", "1:02:03"),
    ("fixAAAAAAA4", "[fixture] Result four", "Fixture channel D", "0:42"),
    ("fixAAAAAAA5", "[fixture] Result five", "Fixture channel E", "8:00"),
    ("fixAAAAAAA6", "[fixture] Result six", "Fixture channel F", "12:12"),
    ("fixUNPLAYBL", "[fixture] Result seven - unplayable", "Fixture channel G", "4:04"),
]


def search_reply(query):
    """Fixture search: 'none' -> empty, 'fail' -> HTTP 500, 'one' -> 1 hit."""
    if "none" in query:
        videos = []
    elif "one" in query.split():
        videos = FIXTURE_VIDEOS[:1]
    else:
        videos = FIXTURE_VIDEOS
    items = [{"videoRenderer": {"videoId": v, "title": {"runs": [{"text": t}]},
                                "ownerText": {"runs": [{"text": c}]}, "lengthText": {"simpleText": d}}}
             for v, t, c, d in videos]
    # One live entry without a length must be filtered out by the client.
    items.append({"videoRenderer": {"videoId": "fixLIVE0001", "title": {"runs": [{"text": "[fixture] live, no length"}]}}})
    return {"contents": {"sectionListRenderer": {"contents": [{"itemSectionRenderer": {"contents": items}}]}}}


def serve_fixtures(out):
    import replay
    player = json.loads((MEDIA / "player.json").read_text())
    base = replay.make_handler(player, replay.load_fixtures(MEDIA / "fixtures"), None)
    requests = (out / "fixture-requests.log").open("a", buffering=1)

    class Handler(base):
        def log_message(self, fmt, *args):
            requests.write(f"{time.time():.3f} {self.address_string()} {fmt % args}\n")

        def do_POST(self):
            if self.path.startswith("/youtubei/v1/search"):
                body = json.loads(self.rfile.read(int(self.headers.get("Content-Length", 0)) or 0) or b"{}")
                query = body.get("query", "")
                requests.write(f"{time.time():.3f} search query={query!r}\n")
                if "slow" in query:
                    time.sleep(4)
                if "fail" in query:
                    return self.send_json({"error": "fixture failure"}, 500)
                return self.send_json(search_reply(query))
            body = self.rfile.read(int(self.headers.get("Content-Length", 0)) or 0)
            if b"fixUNPLAYBL" in body:
                requests.write(f"{time.time():.3f} player unplayable fixture\n")
                return self.send_json({"playabilityStatus": {"status": "ERROR", "reason": "Fixture: video unavailable"}})
            self.rfile = io.BytesIO(body)
            return super().do_POST()

    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Handler)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    return server


def free_ports(n):
    sockets = [socket.socket() for _ in range(n)]
    for s in sockets:
        s.bind(("127.0.0.1", 0))
    ports = [s.getsockname()[1] for s in sockets]
    for s in sockets:
        s.close()
    return ports


def start(args):
    import g06
    out = BASE / args.name
    (out / "shots").mkdir(parents=True, exist_ok=True)
    trampoline = BASE / "g06-init"
    if not trampoline.exists():
        g06.build_trampoline(trampoline)
    server = serve_fixtures(out)
    fixture_port = server.server_port
    apps = args.apps.resolve()
    init = args.init.resolve()
    files = [f"{apps}/{name}:/bin/{name}" for name in ("slides", "doomtv", "ytplay")]
    files += [f"{MEDIA}/doom/freedoom1.wad:/doom/freedoom1.wad", f"{MEDIA}/tiles/demo-video.rgb:/share/tiles/demo-video.rgb"]
    files += [f"{PARENT}/assets/build/tiles/{n}.rgb:/share/tiles/{n}.rgb" for n in ("slides", "doom")
              if (PARENT / f"assets/build/tiles/{n}.rgb").exists()]
    cpio = out / "initramfs.cpio"
    subprocess.run([sys.executable, str(ROOT / "tools/mkinitramfs.py"), str(init), str(cpio), *files], check=True,
                   stdout=subprocess.DEVNULL)
    cpio.write_bytes(g06.wrap_ramdisk(cpio.read_bytes(), trampoline, init))
    http_port, console_port = free_ports(2)
    qmp = Path("/tmp") / f"ui-audit-{args.name}.qmp"
    qmp.unlink(missing_ok=True)
    append = ("console=ttyAMA0 rdinit=/init ip=off panic=3 drm_kms_helper.drm_fbdev_overalloc=200 "
              "hello.ip=192.168.77.2 hello.peer=192.168.77.1 hello.qemu=1 hello.repl_timeout=0")
    if not args.production:
        append += f" tvinit.demo=1 ytplay.server=http://192.168.77.1:{fixture_port}"
    # restrict=on: no guest traffic leaves the host; only the fixture port is forwarded.
    netdev = (f"user,id=n0,restrict=on,net=192.168.77.0/24,host=192.168.77.1,dhcpstart=192.168.77.15,"
              f"hostfwd=tcp:127.0.0.1:{http_port}-192.168.77.2:8765,hostfwd=tcp:127.0.0.1:{console_port}-192.168.77.2:8766,"
              f"guestfwd=tcp:192.168.77.1:{fixture_port}-cmd:nc 127.0.0.1 {fixture_port}")
    command = ["qemu-system-arm", "-M", "virt,highmem=off,gic-version=2", "-cpu", "max", "-smp", "4", "-m", "1430M",
               "-global", "virtio-mmio.force-legacy=false", "-kernel", str(MEDIA / "kernel/zImage"), "-initrd", str(cpio),
               "-append", append, "-netdev", netdev, "-device", "virtio-net-device,netdev=n0",
               "-rtc", f"base={g06.RTC},clock=vm", "-device", "virtio-gpu-device,xres=1280,yres=720",
               "-device", "virtio-keyboard-device", "-device", "virtio-rng-device",
               "-audiodev", f"wav,id=a0,path={out}/audio.wav", "-device", "virtio-sound-device,audiodev=a0,streams=1",
               "-display", "none", "-monitor", "none", "-serial", f"file:{out}/serial.log",
               "-qmp", f"unix:{qmp},server=on,wait=off", "-no-reboot"]
    (out / "command.json").write_text(json.dumps(command, indent=2) + "\n")
    session = {"qmp": str(qmp), "http": http_port, "console": console_port, "fixture": fixture_port,
               "init": str(init), "apps": str(apps), "runtime": "embedded PID1 launcher (hello-pid1 repl), QEMU virt surrogate"}
    (out / "session.json").write_text(json.dumps(session, indent=2) + "\n")
    with (out / "qemu.log").open("w") as log:
        process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT)
        (out / "qemu.pid").write_text(str(process.pid))
        print(f"session {args.name}: pid {process.pid}, http {http_port}, console {console_port}, fixture {fixture_port}", flush=True)
        code = process.wait()
    server.shutdown()
    print(f"qemu exited {code}", flush=True)


class Session:
    def __init__(self, name):
        self.out = BASE / name
        self.info = json.loads((self.out / "session.json").read_text())
        self.wire = None

    def qmp(self, name, **arguments):
        if self.wire is None:
            s = socket.socket(socket.AF_UNIX)
            s.settimeout(30)
            s.connect(self.info["qmp"])
            self.sock = s
            self.wire = s.makefile("rw")
            self.wire.readline()
            self.qmp("qmp_capabilities")
        self.wire.write(json.dumps({"execute": name, "arguments": arguments}) + "\n")
        self.wire.flush()
        while True:
            response = json.loads(self.wire.readline())
            if "return" in response:
                return response["return"]
            if "error" in response:
                raise RuntimeError(response)

    def event(self, code, down):
        self.qmp("input-send-event", events=[{"type": "key", "data": {"down": down, "key": {"type": "qcode", "data": code}}}])

    def press(self, code):
        self.event(code, True)
        time.sleep(0.08)
        self.event(code, False)
        time.sleep(0.35)

    def serial(self):
        return (self.out / "serial.log").read_text(errors="replace")

    def http(self, method, path, body=None):
        c = http.client.HTTPConnection("127.0.0.1", self.info["http"], timeout=30)
        try:
            c.request(method, path, body=body)
            r = c.getresponse()
            return r.status, r.read().decode(errors="replace")
        finally:
            c.close()

    def shell(self, command):
        with socket.create_connection(("127.0.0.1", self.info["console"]), timeout=5) as s:
            def until(marker):
                data = b""
                deadline = time.monotonic() + 15
                while marker not in data and time.monotonic() < deadline:
                    try:
                        data += s.recv(65536)
                    except socket.timeout:
                        pass
                return data
            until(b"tv-repl# ")
            s.sendall(b"printf '__BE%sGIN__\\n' ''; " + command.encode() + b"; printf '__DO%sNE__\\n' ''\n")
            data = until(b"__DONE__\r\n").decode(errors="replace")
            s.sendall(b"exit\n")
            return data.split("__BEGIN__\r\n", 1)[-1].rsplit("__DONE__", 1)[0]

    def run(self, script):
        start = self.serial()
        seen = {}
        for word in script.split():
            if word.startswith("next:"):
                marker, _, timeout = word[5:].partition("@")
                marker = marker.replace("_", " ")
                seen[marker] = seen.get(marker, start.count(marker)) + 1
                deadline = time.monotonic() + float(timeout or 30)
                while self.serial().count(marker) < seen[marker]:
                    if time.monotonic() > deadline:
                        raise SystemExit(f"TIMEOUT waiting for new {marker!r}")
                    time.sleep(0.1)
                print(f"saw new {marker!r}")
            elif word.startswith("sleep:"):
                time.sleep(float(word[6:]))
            elif word.startswith("shot:"):
                path = self.out / "shots" / f"{word[5:]}.png"
                self.qmp("screendump", filename=str(path), format="png")
                print(f"shot {path}")
            elif word.startswith("wait:"):
                parts = word[5:].split("@")
                marker = parts[0].replace("_", " ")
                count = int(parts[1]) if len(parts) > 1 else 1
                timeout = float(parts[2]) if len(parts) > 2 else 30
                deadline = time.monotonic() + timeout
                while self.serial().count(marker) < count:
                    if time.monotonic() > deadline:
                        print(f"TIMEOUT waiting for {marker!r} x{count}")
                        break
                    time.sleep(0.1)
                else:
                    print(f"saw {marker!r} x{count}")
            elif word.startswith("log:"):
                lines = self.serial().splitlines()
                print("\n".join(lines[-int(word[4:]):]))
            elif word.startswith("hold:"):
                _, code, secs = word.split(":")
                self.event(code, True)
                time.sleep(float(secs))
                self.event(code, False)
                time.sleep(0.35)
            elif word.startswith("down:"):
                self.event(word[5:], True)
            elif word.startswith("up:"):
                self.event(word[3:], False)
            elif word.startswith("sh:"):
                print(self.shell(word[3:].replace("~", " ")))
            elif word.startswith("get:"):
                print(*self.http("GET", word[4:]))
            elif word.startswith("post:"):
                print(*self.http("POST", word[5:]))
            else:
                code, _, n = word.partition("*")
                for _ in range(int(n or 1)):
                    self.press(code)


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    sub = parser.add_subparsers(dest="cmd", required=True)
    p = sub.add_parser("start")
    p.add_argument("name")
    p.add_argument("--apps", type=Path, required=True, help="directory with ARM32 slides, doomtv and ytplay")
    p.add_argument("--init", type=Path, default=INIT)
    p.add_argument("--production", action="store_true",
                   help="production catalog: no Demo row; YouTube uses its live URL, unreachable offline")
    p = sub.add_parser("run")
    p.add_argument("name")
    p.add_argument("script")
    p = sub.add_parser("sh")
    p.add_argument("name")
    p.add_argument("command")
    p = sub.add_parser("upload", help="copy a host file to the guest's /tmp/NAME")
    p.add_argument("name")
    p.add_argument("file", type=Path)
    p.add_argument("guest_name")
    p = sub.add_parser("stop")
    p.add_argument("name")
    args = parser.parse_args()
    if args.cmd == "start":
        start(args)
    elif args.cmd == "sh":
        print(Session(args.name).shell(args.command), end="")
    elif args.cmd == "upload":
        print(*Session(args.name).http("PUT", f"/files/{args.guest_name}", args.file.read_bytes()))
    elif args.cmd == "run":
        Session(args.name).run(args.script)
    else:
        pid = int((BASE / args.name / "qemu.pid").read_text())
        os.kill(pid, 15)


if __name__ == "__main__":
    main()
