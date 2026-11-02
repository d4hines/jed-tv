#!/usr/bin/env python3
"""Fake YouTube for offline tests: replays a recorded /player reply and fixtures.

Mimics the real minimal flow (see PROTOCOL.md):
  POST /youtubei/v1/player without a visitor ID -> LOGIN_REQUIRED + visitorData
  POST /youtubei/v1/player with one (header or body) -> recorded reply, with each
       adaptiveFormat URL rewritten to http://<this server>/videoplayback?itag=N
  GET  /videoplayback?itag=N -> fixture bytes, Range supported (206)
Only formats with a fixture file are listed. Plain HTTP for now.

  python3 test/replay.py \
      --player assets/yt-stage0/minimal-.../player.visitor-header.json \
      --fixtures assets/yt-stage0/record-.../fixtures --port 8090
QEMU user-mode guests reach the host at 10.0.2.2.
"""
import argparse
import copy
import http.server
import json
import pathlib
import re
import urllib.parse

VISITOR = "replay-visitor-0001"


def load_fixtures(d):
    out = {}
    for f in pathlib.Path(d).iterdir():
        m = re.fullmatch(r"(\d+(?:-drc)?)\.(mp4|webm|m4a)", f.name)
        if m:
            out[m.group(1)] = f
    return out


def make_handler(player, fixtures, public_base):
    refused = {"responseContext": {"visitorData": VISITOR},
               "playabilityStatus": {"status": "LOGIN_REQUIRED",
                                     "reason": "Sign in to confirm you’re not a bot"}}

    def reply_for(base):
        r = copy.deepcopy(player)
        sd = r.get("streamingData", {})
        kept = []
        for f in sd.get("adaptiveFormats", []):
            itag = str(f["itag"]) + ("-drc" if f.get("isDrc") else "")
            if itag in fixtures:
                f["url"] = f"{base}/videoplayback?itag={itag}"
                kept.append(f)
        sd["adaptiveFormats"] = kept
        for k in ("formats", "serverAbrStreamingUrl", "hlsManifestUrl", "dashManifestUrl"):
            sd.pop(k, None)
        return r

    class H(http.server.BaseHTTPRequestHandler):
        protocol_version = "HTTP/1.1"

        def send_json(self, obj, code=200):
            body = json.dumps(obj).encode()
            self.send_response(code)
            self.send_header("Content-Type", "application/json; charset=UTF-8")
            self.send_header("Content-Length", str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def do_POST(self):
            u = urllib.parse.urlsplit(self.path)
            body = self.rfile.read(int(self.headers.get("Content-Length", 0)) or 0)
            if u.path != "/youtubei/v1/player":
                return self.send_json({"error": "not found"}, 404)
            try:
                req = json.loads(body or b"{}")
            except ValueError:
                return self.send_json({"error": "bad json"}, 400)
            visitor = (self.headers.get("X-Goog-Visitor-Id")
                       or req.get("context", {}).get("client", {}).get("visitorData"))
            if not visitor:
                return self.send_json(refused)
            base = public_base or f"http://{self.headers.get('Host')}"
            self.send_json(reply_for(base))

        def do_GET(self):
            u = urllib.parse.urlsplit(self.path)
            itag = urllib.parse.parse_qs(u.query).get("itag", [""])[0]
            if u.path != "/videoplayback" or itag not in fixtures:
                return self.send_json({"error": "not found"}, 404)
            f = fixtures[itag]
            size = f.stat().st_size
            start, end, code = 0, size - 1, 200
            rng = self.headers.get("Range")
            if rng:
                m = re.fullmatch(r"bytes=(\d+)-(\d*)", rng.strip())
                if not m or int(m.group(1)) >= size:
                    self.send_response(416)
                    self.send_header("Content-Range", f"bytes */{size}")
                    self.send_header("Content-Length", "0")
                    self.end_headers()
                    return
                start = int(m.group(1))
                end = min(int(m.group(2)) if m.group(2) else size - 1, size - 1)
                code = 206
            self.send_response(code)
            self.send_header("Content-Type", "video/webm" if f.suffix == ".webm" else "video/mp4")
            self.send_header("Content-Length", str(end - start + 1))
            if code == 206:
                self.send_header("Content-Range", f"bytes {start}-{end}/{size}")
            self.end_headers()
            with open(f, "rb") as fh:
                fh.seek(start)
                left = end - start + 1
                while left > 0:
                    chunk = fh.read(min(1 << 20, left))
                    if not chunk:
                        break
                    self.wfile.write(chunk)
                    left -= len(chunk)

    return H


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("--player", required=True, type=pathlib.Path)
    ap.add_argument("--fixtures", required=True, type=pathlib.Path)
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--port", type=int, default=8090)
    ap.add_argument("--public-base", help="override URL base written into the reply")
    args = ap.parse_args()

    player = json.loads(args.player.read_text())
    fixtures = load_fixtures(args.fixtures)
    # Check fixtures match the recorded reply byte-for-byte in length.
    for f in player.get("streamingData", {}).get("adaptiveFormats", []):
        itag = str(f["itag"]) + ("-drc" if f.get("isDrc") else "")
        if itag in fixtures and int(f.get("contentLength", -1)) != fixtures[itag].stat().st_size:
            print(f"WARNING itag {itag}: contentLength {f.get('contentLength')} != "
                  f"fixture {fixtures[itag].stat().st_size}")
    print(f"{len(fixtures)} fixtures: {', '.join(sorted(fixtures, key=lambda s: int(s.split('-')[0])))}")
    srv = http.server.ThreadingHTTPServer((args.host, args.port),
                                          make_handler(player, fixtures, args.public_base))
    print(f"replaying on http://{args.host}:{args.port}", flush=True)
    srv.serve_forever()


if __name__ == "__main__":
    main()
