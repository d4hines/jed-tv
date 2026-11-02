#!/usr/bin/env python3
"""Run the Lean Lounge session (LiveMain.lean) against live YouTube.

  (cd test/lean/yt/lounge && lake build live)
  python3 live.py --seconds 120 --code-file /tmp/lounge-code.txt [--dial-check]
  # in another shell, the phone stand-in from the host prototype:
  python3 research/dial-lounge/fake_remote.py --code-file /tmp/lounge-code.txt

The Lean driver decides every request; this script only performs them
(stdlib http.client to www.youtube.com, one connection per request, the long
poll streamed) and feeds the results back with timestamps and a tick per
second. --dial-check also registers a fresh DIAL-style pairing code through
the session and resolves it with get_screen, as a phone would.

Host-only and modest: one screen, one long poll, a few POSTs. Logs every
line exchanged to --log (default under assets/research/dial-lounge/lean-live/).
"""
import argparse
import base64
import http.client
import os
import pathlib
import queue
import ssl
import subprocess
import threading
import time
import urllib.parse
import uuid
import json

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parents[3]
CTX = ssl.create_default_context()
T0 = time.monotonic()


def now_ms():
    return int((time.monotonic() - T0) * 1000)


def b64(b):
    return base64.b64encode(b).decode() if b else "-"


class Live:
    def __init__(self, args, log):
        self.args, self.log = args, log
        self.events = queue.Queue()
        self.polls = {}  # id -> connection
        self.screen_id = None
        self.code = None
        self.registered = {}  # request id -> status
        exe = HERE / ".lake" / "build" / "bin" / "live"
        self.proc = subprocess.Popen([str(exe)], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True,
                                     bufsize=1)

    def say(self, *a):
        print(time.strftime("%H:%M:%S"), *a, flush=True)

    # -- HTTP ------------------------------------------------------------------
    def do_request(self, rid, method, url, headers, body):
        u = urllib.parse.urlsplit(url)
        path = u.path + ("?" + u.query if u.query else "")
        try:
            c = http.client.HTTPSConnection(u.hostname, context=CTX, timeout=20)
            c.request(method, path, body=body or None, headers=headers)
            r = c.getresponse()
            data = r.read()
            c.close()
            self.events.put(f"R {rid} {r.status} {b64(data)}")
        except Exception as e:
            self.say(f"request {rid} failed: {e!r}")
            self.events.put(f"R {rid} 0 -")

    def do_poll(self, rid, url, headers):
        u = urllib.parse.urlsplit(url)
        try:
            c = http.client.HTTPSConnection(u.hostname, context=CTX, timeout=90)
            self.polls[rid] = c
            c.request("GET", u.path + "?" + u.query, headers=headers)
            r = c.getresponse()
            if r.status != 200:
                r.read()
                self.events.put(f"E {rid} {r.status}")
                return
            while True:
                data = r.read1(65536)
                if not data:
                    break
                self.events.put(f"C {rid} {b64(data)}")
            self.events.put(f"E {rid} 200")
        except Exception as e:
            if rid in self.polls:
                self.events.put(f"E {rid} -")
        finally:
            self.polls.pop(rid, None)

    def supersede(self, keep):
        for rid, c in list(self.polls.items()):
            if rid != keep:
                self.polls.pop(rid, None)
                try:
                    c.sock.shutdown(2)
                except Exception:
                    pass

    # -- driver ----------------------------------------------------------------
    def send(self, line):
        line = f"{now_ms()} {line}"
        self.log.write(">> " + line[:300] + "\n")
        self.proc.stdin.write(line + "\n")
        self.proc.stdin.flush()
        while True:
            out = self.proc.stdout.readline()
            if not out:
                raise SystemExit("driver exited")
            out = out.rstrip("\n")
            self.log.write("<< " + out[:2000] + "\n")
            if out == "OK":
                return
            self.handle(out)

    def handle(self, out):
        kind, _, rest = out.partition(" ")
        if kind == "REQ":
            rid, tag, method, stream, url, hh, hb = rest.split(" ")
            headers = dict(l.split(": ", 1) for l in bytes.fromhex(hh).decode().splitlines() if l)
            body = b"" if hb == "-" else bytes.fromhex(hb)
            self.say(f"REQ #{rid} {tag} {method}" + (f" body {body[:120]!r}" if body else ""))
            if tag == "poll":
                self.supersede(int(rid))
                threading.Thread(target=self.do_poll, args=(int(rid), url, headers), daemon=True).start()
            else:
                if tag == "registerCode":
                    self.registered[int(rid)] = None
                threading.Thread(target=self.do_request, args=(rid, method, url, headers, body),
                                 daemon=True).start()
        elif kind == "CODE":
            self.code = rest
            self.say(f"CODE {rest[0:3]} {rest[3:6]} {rest[6:9]} {rest[9:12]}")
            if self.args.code_file:
                pathlib.Path(self.args.code_file).write_text(rest + "\n")
        elif kind == "ID":
            self.screen_id = rest
            self.say("screen ID", rest)
        else:
            self.say(kind, rest)

    def run(self):
        a = self.args
        entropy = os.urandom(16)
        name = base64.b64encode(a.name.encode()).decode()
        self.send(f"S {b64(entropy)} - {uuid.uuid4()} {name}")
        deadline = time.monotonic() + a.seconds
        dial_done = not a.dial_check
        dial_code = None
        while time.monotonic() < deadline:
            try:
                ev = self.events.get(timeout=1.0)
            except queue.Empty:
                ev = "T"
            if ev.startswith("R "):
                rid = int(ev.split(" ")[1])
                if rid in self.registered:
                    self.say(f"registerCode #{rid} -> HTTP {ev.split(' ')[2]}")
                    self.check_dial(dial_code)
            self.send(ev)
            if not dial_done and self.code:
                dial_code = str(uuid.uuid4())
                self.say("DIAL check: registering", dial_code)
                self.send(f"D {dial_code}")
                dial_done = True
        self.send("Q")
        time.sleep(1.5)
        self.proc.stdin.close()
        self.proc.wait(timeout=5)

    def check_dial(self, code):
        for attempt in range(8):  # the code resolves a moment after registration
            c = http.client.HTTPSConnection("www.youtube.com", context=CTX, timeout=20)
            c.request("POST", "/api/lounge/pairing/get_screen",
                      body=urllib.parse.urlencode({"pairing_code": code}),
                      headers={"Content-Type": "application/x-www-form-urlencoded"})
            r = c.getresponse()
            data = r.read()
            c.close()
            if r.status == 200:
                break
            self.say(f"DIAL check: get_screen -> HTTP {r.status}, retrying")
            time.sleep(1.5)
        ok = False
        if r.status == 200:
            scr = json.loads(data)["screen"]
            ok = scr.get("screenId") == self.screen_id
            self.say(f"DIAL check: get_screen -> 200, screen {scr.get('name')!r}, matches our screen ID: {ok}")
        else:
            self.say(f"DIAL check: get_screen -> HTTP {r.status}")
        self.log.write(f"## dial-check get_screen status={r.status} matches={ok}\n")


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--seconds", type=float, default=120)
    ap.add_argument("--name", default="Dande TV (Lean session)")
    ap.add_argument("--code-file")
    ap.add_argument("--dial-check", action="store_true")
    ap.add_argument("--log", default=str(ROOT / "assets" / "research" / "dial-lounge" / "lean-live" /
                                         time.strftime("%Y%m%dT%H%M%SZ.log", time.gmtime())))
    args = ap.parse_args()
    pathlib.Path(args.log).parent.mkdir(parents=True, exist_ok=True)
    with open(args.log, "w", buffering=1) as log:
        print("log:", args.log, flush=True)
        Live(args, log).run()


if __name__ == "__main__":
    main()
