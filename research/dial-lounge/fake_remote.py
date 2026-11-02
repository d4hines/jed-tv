#!/usr/bin/env python3
"""Stand-in for the phone, to check screen.py end to end through YouTube.

  python3 fake_remote.py --code 123456789012        # "Link with TV code" path
  python3 fake_remote.py --dial 127.0.0.1:56790     # DIAL launch path (loopback)

It pairs the way remote clients do (pyytlounge 3.6.0 / iSponsorBlockTV):
POST /api/lounge/pairing/get_screen with the code, then joins the screen's
lounge as a REMOTE_CONTROL and sends a short script: setPlaylist, pause,
play, seekTo, setVolume, next, getNowPlaying, then disconnects. It checks that
the screen's replies (relayed by YouTube) match.

In --dial mode it first acts like the YouTube app's DIAL client: unicast
M-SEARCH to <ip>:1900, GET the LOCATION, read Application-URL, GET and POST
/apps/YouTube with a fresh pairingCode, then resolves that code with
get_screen once the screen has registered it.

This is not the YouTube app. It shows that our screen side works with
YouTube's servers, not that the official app lists or accepts it.
"""

import argparse
import json
import pathlib
import queue
import socket
import sys
import threading
import time
import urllib.request
import uuid
import xml.etree.ElementTree as ET

import lounge

VIDEOS = ["PVGeM40dABA", "aqz-KE-bpKQ"]  # Blender open movies (Coffee Run, Big Buck Bunny)


def say(*a):
    print(time.strftime("%H:%M:%S"), *a, flush=True)


class RemoteSession(lounge.BindSession):
    def open(self):
        # Same request shape as pyytlounge's connect(): parameters in the body.
        form = {"app": "web", "mdx-version": "3", "name": self.name, "id": self.device_id,
                "device": "REMOTE_CONTROL", "capabilities": "que,dsdtr,atp,vsp",
                "magnaKey": "cloudPairedDevice", "ui": "false", "theme": "cl",
                "loungeIdToken": self.lounge_token}
        st, reason, body = lounge.request(
            "POST", lounge.API + "/bc/bind",
            query={"RID": "1", "VER": "8", "CVER": "1", "auth_failure_option": "send_error"}, form=form)
        if st != 200:
            raise lounge.HttpError(st, reason, body)
        self.rid = 2
        msgs = lounge.parse_body(body)
        for m in msgs:
            self.aid = max(self.aid, m.aid)
            if m.name == "c":
                self.sid = m.args[0]
            elif m.name == "S":
                self.gsessionid = m.args[0]
        return [m for m in msgs if m.name not in ("c", "S")]


def dial_launch(target):
    host, _, port = target.partition(":")
    port = int(port or 56790)
    # 1. SSDP: unicast M-SEARCH (the app multicasts; unicast keeps this on-host)
    msearch = ("M-SEARCH * HTTP/1.1\r\nHOST: 239.255.255.250:1900\r\nMAN: \"ssdp:discover\"\r\n"
               "MX: 1\r\nST: urn:dial-multiscreen-org:service:dial:1\r\n"
               "USER-AGENT: fake_remote/0 (dande-tv test)\r\n\r\n").encode()
    s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
    s.settimeout(3)
    s.bind((host, 0))
    s.sendto(msearch, (host, 1900))
    reply, src = s.recvfrom(4096)
    s.close()
    text = reply.decode()
    say("SSDP reply from", src, "\n" + text.strip())
    location = next(line.split(":", 1)[1].strip() for line in text.split("\r\n")
                    if line.upper().startswith("LOCATION:"))
    # 2. device description -> Application-URL
    with urllib.request.urlopen(location, timeout=5) as r:
        app_url = r.headers["Application-URL"]
        dd = r.read().decode()
    say("GET", location, "-> Application-URL:", app_url)
    assert "urn:dial-multiscreen-org:device:dial:1" in dd
    # 3. app state
    with urllib.request.urlopen(app_url + "YouTube", timeout=5) as r:
        app = r.read().decode()
        say("GET", app_url + "YouTube", "->", r.status, "\n" + app.strip())
    ns = {"d": "urn:dial-multiscreen-org:schemas:dial"}
    extra = ET.fromstring(app).find("d:additionalData", ns)
    token = extra.find("d:loungeToken", ns) if extra is not None else None
    if token is not None and token.text:
        # Running screen that advertises its lounge token: join directly, no POST
        # (what Playlet's maintainer and kodi-yt-caster describe the app doing).
        sid = extra.find("d:screenId", ns).text
        say("additionalData carries a loungeToken; joining without a launch POST")
        return None, {"screenId": sid, "loungeToken": token.text, "name": "(from DIAL)"}
    # 4. launch with a sender-generated pairing code
    code = str(uuid.uuid4())
    body = f"pairingCode={code}&theme=cl&dialLaunch=watch".encode()
    req = urllib.request.Request(app_url + "YouTube", data=body, method="POST", headers={
        "Content-Type": "text/plain; charset=\"utf-8\"",
        "Origin": "package:com.google.android.youtube"})
    with urllib.request.urlopen(req, timeout=5) as r:
        say("POST", app_url + "YouTube", "->", r.status, "Location:", r.headers.get("Location"))
    return code, None


def resolve(code, wait):
    deadline = time.monotonic() + wait
    while True:
        try:
            return lounge.get_screen(code)
        except lounge.HttpError as e:
            if time.monotonic() > deadline:
                raise
            say(f"get_screen not ready ({e.status}); retrying")
            time.sleep(1.5)


def main():
    ap = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--code", help="12-digit TV code shown by screen.py")
    g.add_argument("--code-file", help="file screen.py --code-file writes")
    g.add_argument("--dial", metavar="IP[:PORT]", help="emulate a DIAL launch against screen.py --dial")
    ap.add_argument("--name", default="fake_remote (dande-tv test)")
    ap.add_argument("--log", help="JSONL transcript path")
    args = ap.parse_args()

    logf = open(args.log, "a", buffering=1) if args.log else None

    def log(kind, **f):
        if logf:
            logf.write(json.dumps({"t": time.time(), "kind": kind, **f}, default=str) + "\n")

    if args.dial:
        code, screen = dial_launch(args.dial)
        screen = screen or resolve(code, 20)
    else:
        code = args.code or pathlib.Path(args.code_file).read_text().strip()
        screen = resolve(code.replace(" ", ""), 0)
    say(f"paired: screen name {screen.get('name')!r}, screenId {screen['screenId']}")
    log("paired", screen={k: v for k, v in screen.items() if k != "loungeToken"})

    sess = RemoteSession("remote", lounge.new_device_id(), args.name, screen["loungeToken"],
                         app="youtube-desktop", log=log)
    initial = sess.open()
    say(f"joined lounge: SID {sess.sid}")
    events = queue.Queue()
    for m in initial:
        events.put(m)
    stop = threading.Event()

    def reader():
        while not stop.is_set():
            try:
                sess.poll(lambda ms: [events.put(m) for m in ms], timeout=60, should_stop=stop.is_set)
            except Exception as e:
                if not stop.is_set():
                    say("remote poll error:", e)
                    time.sleep(1)
    threading.Thread(target=reader, daemon=True).start()

    seen = []

    def drain(secs):
        end = time.monotonic() + secs
        while time.monotonic() < end:
            try:
                m = events.get(timeout=0.2)
            except queue.Empty:
                continue
            if m.name == "noop":
                continue
            seen.append(m)
            p = m.payload if isinstance(m.payload, dict) else {}
            brief = {k: p[k] for k in ("videoId", "state", "currentTime", "volume", "muted",
                                         "hasNext", "autoplayMode") if k in p}
            say(f"  screen says: {m.name} {json.dumps(brief) if brief else ''}")

    def last(name):
        for m in reversed(seen):
            if m.name == name:
                return m.payload if isinstance(m.payload, dict) else {}
        return None

    results = []

    def check(label, ok):
        results.append((label, bool(ok)))
        say(("PASS " if ok else "FAIL ") + label)

    drain(3)
    steps = [
        ("setPlaylist", {"videoId": VIDEOS[0], "currentTime": "0", "currentIndex": "0",
                         "videoIds": ",".join(VIDEOS)}, 4),
        ("pause", {}, 3), ("play", {}, 3), ("seekTo", {"newTime": "60"}, 3),
        ("setVolume", {"volume": "30", "muted": "false"}, 3),
        ("next", {}, 4), ("getNowPlaying", {}, 3)]
    for name, fields, wait in steps:
        say(f"remote -> {name} {json.dumps(fields) if fields else ''}")
        seen.clear()
        sess.send([(name, fields)])
        drain(wait)
        np, sc, vol = last("nowPlaying"), last("onStateChange"), last("onVolumeChanged")
        if name == "setPlaylist":
            check("setPlaylist -> nowPlaying with our video", np and np.get("videoId") == VIDEOS[0])
            check("setPlaylist -> state reaches playing (1)", sc and str(sc.get("state")) == "1")
        elif name == "pause":
            check("pause -> onStateChange state 2", sc and str(sc.get("state")) == "2")
        elif name == "play":
            check("play -> onStateChange state 1", sc and str(sc.get("state")) == "1")
        elif name == "seekTo":
            check("seekTo 60 -> currentTime ~60", sc and 59 <= float(sc.get("currentTime", 0)) < 63)
        elif name == "setVolume":
            check("setVolume 30 -> onVolumeChanged 30", vol and str(vol.get("volume")) == "30")
        elif name == "next":
            check("next -> nowPlaying second video", np and np.get("videoId") == VIDEOS[1])
        elif name == "getNowPlaying":
            check("getNowPlaying -> nowPlaying", np is not None)

    say("remote -> disconnect (TYPE=terminate)")
    stop.set()
    sess.terminate()
    failed = [l for l, ok in results if not ok]
    say(f"{len(results) - len(failed)}/{len(results)} checks passed")
    log("result", results=results)
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
