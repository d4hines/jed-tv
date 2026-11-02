#!/usr/bin/env python3
"""Offline YouTube for the Lean app's QEMU tests: test/replay.py's player API
and ranged media, plus the two things it lacks:

  POST /youtubei/v1/search      -> a recorded live search reply
  GET  /vi/<id>/mqdefault.jpg   -> assets/build/tiles/src/<id>.jpg if present,
                                   else a recorded live thumbnail

The app takes thumbnails from its server when that server is plain http://.
"""
import json, pathlib, re, sys

ROOT = pathlib.Path(__file__).resolve().parents[3]
LIVE = ROOT / 'assets/build/lean-formats/live-20261101T082442Z'
TILES = ROOT / 'assets/build/tiles/src'


def make_handler(media):
    sys.path.insert(0, str(ROOT / 'test'))
    import replay
    base = replay.make_handler(json.loads((media / 'player.json').read_text()),
                               replay.load_fixtures(media / 'fixtures'), None)
    search = (LIVE / 'search.json').read_bytes()
    default_thumb = (LIVE / 'mqdefault.jpg').read_bytes()

    class Handler(base):
        def reply(self, status, ctype, body):
            self.send_response(status)
            self.send_header('Content-Type', ctype)
            self.send_header('Content-Length', str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def do_POST(self):
            if self.path.split('?')[0] == '/youtubei/v1/search':
                self.rfile.read(int(self.headers.get('Content-Length', 0)) or 0)
                return self.reply(200, 'application/json', search)
            return super().do_POST()

        def do_GET(self):
            m = re.fullmatch(r'/vi/([A-Za-z0-9_-]{11})/mqdefault\.jpg', self.path)
            if m:
                tile = TILES / f'{m.group(1)}.jpg'
                return self.reply(200, 'image/jpeg', tile.read_bytes() if tile.exists() else default_thumb)
            return super().do_GET()

        def log_message(self, *args):
            pass

    return Handler
