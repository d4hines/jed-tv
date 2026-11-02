#!/usr/bin/env python3
"""UI regression check on the embedded PID1 launcher in headless ARM32 QEMU.

  python3 tools/ui-audit/check.py --apps DIR

DIR holds ARM32 slides, doomtv and ytplay. Boots tools/ui-audit/session.py,
drives the remote with QMP key events and checks serial-log markers and exact
screenshot pixels (all text is the 8x8 bitmap font, so regions are
deterministic). YouTube data comes from the local fixture server, never from
live YouTube. Every case runs even after a failure; exit status 1 if any failed.

Cases, each a defect found in the 2026-11-01 UI audit:
  demo-seek-end      seeking into the last second ends playback (no frozen frame)
  yt-help-wrapped    search status column wraps instead of cutting text
  yt-back-steps      Back from a search video returns to results, then keys, hub, home
  yt-play-error      a result that cannot play reports it; the summary returns after a good play
  yt-failed-search   a failed search shows the full error and clears old results
  yt-long-query      a long query stays inside the query box
  doom-lobby-back    Back leaves the Doom LAN lobby
"""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import struct
import subprocess
import sys
import time
import zlib

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location("ui_session", HERE / "session.py")
session = importlib.util.module_from_spec(spec)
spec.loader.exec_module(session)

WHITE = (240, 240, 245)      # browse.rs text colour
KEY_BG = (34, 36, 50)        # browse.rs key/tile fill
STATUS = (802, 270, 1260, 400)       # status column right of the keyboard
STATUS_LINE2 = (802, 294, 1260, 312)
RESULTS = (40, 455, 1240, 640)
QUERY_OVERFLOW = (1209, 100, 1280, 150)  # right of the query box


def png_rgb(path):
    """Decode QEMU's 8-bit RGB, non-interlaced screendump PNG."""
    data = Path(path).read_bytes()
    pos, idat, width, height = 8, b"", 0, 0
    while pos < len(data):
        length, kind = struct.unpack(">I4s", data[pos:pos + 8])
        chunk = data[pos + 8:pos + 8 + length]
        if kind == b"IHDR":
            width, height, depth, colour = struct.unpack(">IIBB", chunk[:10])
            assert depth == 8 and colour == 2, (depth, colour)
        elif kind == b"IDAT":
            idat += chunk
        pos += 12 + length
    raw = zlib.decompress(idat)
    stride = width * 3
    rows, prev = [], bytearray(stride)
    for y in range(height):
        f = raw[y * (stride + 1)]
        line = bytearray(raw[y * (stride + 1) + 1:(y + 1) * (stride + 1)])
        for i in range(stride):
            a = line[i - 3] if i >= 3 else 0
            b = prev[i]
            c = prev[i - 3] if i >= 3 else 0
            if f == 1:
                line[i] = (line[i] + a) & 255
            elif f == 2:
                line[i] = (line[i] + b) & 255
            elif f == 3:
                line[i] = (line[i] + (a + b) // 2) & 255
            elif f == 4:
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                line[i] = (line[i] + (a if pa <= pb and pa <= pc else b if pb <= pc else c)) & 255
        rows.append(bytes(line))
        prev = line
    return width, height, rows


def region(path, box):
    _, _, rows = png_rgb(path)
    x0, y0, x1, y1 = box
    return [rows[y][x0 * 3:x1 * 3] for y in range(y0, y1)]


def count(path, box, colour):
    target = bytes(colour)
    return sum(sum(1 for i in range(0, len(r), 3) if r[i:i + 3] == target) for r in region(path, box))


class Check:
    def __init__(self, name):
        self.s = session.Session(name)
        self.results = {}

    def shot(self, label):
        path = self.s.out / "shots" / f"check-{label}.png"
        self.s.qmp("screendump", filename=str(path), format="png")
        time.sleep(0.2)
        return path

    def markers(self, marker):
        return self.s.serial().count(marker)

    def wait_new(self, marker, before, timeout=30):
        deadline = time.monotonic() + timeout
        while self.markers(marker) <= before:
            if time.monotonic() > deadline:
                return False
            time.sleep(0.1)
        return True

    def keys(self, script):
        self.s.run(script)

    def case(self, name, ok, detail=""):
        self.results[name] = {"passed": bool(ok), "detail": detail}
        print(f"{'PASS' if ok else 'FAIL'} {name} {detail}", flush=True)

    def type_ab_and_search(self):
        # From a fresh search screen with focus on 'a': a, b, then GO (row 3, column 9).
        before = self.markers("results for")
        self.keys("ret right ret down*3 right*8 ret")
        return self.wait_new("results for", before, 60)

    def run(self):
        assert self.wait_new("home screen:", 0, 120), "launcher never drew the home screen"
        time.sleep(1)

        # demo-seek-end: the home screen opens on the Demo row.
        n = self.markers("audio: AAC")
        self.keys("ret")
        self.wait_new("audio: AAC", n)
        self.keys("sleep:1.5 right sleep:0.3 right sleep:0.3 right")
        n = self.markers("Demo video exited")
        ended = self.wait_new("Demo video exited", n, 15)
        self.case("demo-seek-end", ended, "player returned after seeking to the end" if ended else "player still on its last frame 15 s after the seek")
        if not ended:
            self.keys("esc")
            self.wait_new("Demo video exited", n, 10)
        time.sleep(1)

        # YouTube hub -> Search.
        n = self.markers("YouTube app: search")
        self.keys("down ret")
        self.wait_new("YouTube app: search", n, 30)
        self.keys("sleep:0.5 ret sleep:1")
        help_shot = self.shot("search-help")
        wrapped = count(help_shot, STATUS_LINE2, WHITE) > 0
        self.case("yt-help-wrapped", wrapped, "status continues on a second line" if wrapped else "second status line empty: text cut at the column edge")

        searched = self.type_ab_and_search()
        time.sleep(0.5)
        summary = self.shot("summary")

        # yt-play-error: the seventh fixture result cannot be resolved.
        n = self.markers("failed:")
        self.keys("right*6 ret")
        failed = self.wait_new("failed:", n, 20)
        time.sleep(0.8)
        error_shot = self.shot("play-error")
        reported = failed and region(error_shot, STATUS) != region(summary, STATUS)
        # A good play afterwards restores the search summary.
        n = self.markers("stopped by remote")
        a = self.markers("audio: AAC")
        self.keys("left*6 ret")
        self.wait_new("audio: AAC", a, 30)
        self.keys("sleep:1 esc")
        self.wait_new("stopped by remote", n, 10)
        time.sleep(1)
        restored_shot = self.shot("summary-restored")
        restored = region(restored_shot, STATUS) == region(summary, STATUS)
        self.case("yt-play-error", searched and reported and restored,
                  f"error shown={reported}, summary restored={restored}")

        # yt-back-steps: player -> results -> keys -> hub -> home, one step per Back.
        r = self.markers("YouTube app: returned")
        x = self.markers("YouTube exited")
        a = self.markers("audio: AAC")
        self.keys("ret")
        self.wait_new("audio: AAC", a, 30)
        self.keys("sleep:1 esc sleep:1.5")
        step1 = self.markers("YouTube app: returned") == r and count(self.shot("back1"), RESULTS, WHITE) > 0
        self.keys("esc sleep:1.5")
        step2 = self.markers("YouTube app: returned") == r
        self.keys("esc")
        step3 = self.wait_new("YouTube app: returned", r, 10) and self.markers("YouTube exited") == x
        self.keys("sleep:0.8 ret sleep:1.5")   # back into Search for the next cases
        self.case("yt-back-steps", step1 and step2 and step3, f"results={step1} keys={step2} hub={step3}")

        # yt-failed-search: edit "ab" to "abfail" and search; old results must go.
        # Focus is on the keys (GO). f=(0,5) a=(0,0) i=(0,8) l=(1,1).
        n = self.markers("Search failed")
        self.keys("up*3 left*4 ret left*5 ret right*8 ret down left*7 ret down*2 right*8 ret")
        failed = self.wait_new("Search failed", n, 30)
        time.sleep(0.8)
        fail_shot = self.shot("search-failed")
        cleared = count(fail_shot, RESULTS, KEY_BG) == 0
        full = count(fail_shot, STATUS_LINE2, WHITE) > 0
        self.case("yt-failed-search", failed and cleared and full, f"results cleared={cleared}, error wraps to line 2={full}")

        # yt-long-query: 50 more characters; nothing may spill right of the box.
        self.keys(" ".join(str(i % 10) for i in range(1, 51)) + " sleep:0.5")
        long_shot = self.shot("long-query")
        spill = count(long_shot, QUERY_OVERFLOW, WHITE)
        self.case("yt-long-query", spill == 0, f"{spill} text pixels right of the query box")

        # doom-lobby-back: Home, then Doom > New Game > Host co-op > episode > skill.
        self.keys("home")
        self.wait_new("YouTube stopped", 0, 15)
        n = self.markers("doom: engine up")
        self.keys("sleep:1 right right ret")
        self.wait_new("doom: engine up", n, 180)
        h = self.markers("hosting co-op")
        self.keys("sleep:2 ret sleep:1.2 ret sleep:1.2 down sleep:0.4 ret sleep:1.2 ret sleep:1.2 ret")
        hosting = self.wait_new("hosting co-op", h, 30)
        self.keys("sleep:2")
        left = self.markers("net: left the lobby")
        self.keys("esc")
        gone = self.wait_new("net: left the lobby", left, 10)
        self.shot("after-lobby-back")
        self.case("doom-lobby-back", hosting and gone, "Back returned to the title" if gone else "Back did not leave the lobby")
        self.keys("home")


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--apps", type=Path, required=True, help="directory with ARM32 slides, doomtv and ytplay")
    args = parser.parse_args()
    name = f"check-{time.strftime('%Y%m%dT%H%M%S')}"
    process = subprocess.Popen([sys.executable, str(HERE / "session.py"), "start", name, "--apps", str(args.apps)],
                               stdout=subprocess.DEVNULL, stderr=subprocess.STDOUT)
    try:
        out = session.BASE / name
        for _ in range(600):
            if (out / "session.json").exists() and Path(json.loads((out / "session.json").read_text())["qmp"]).exists():
                break
            if process.poll() is not None:
                raise SystemExit("session failed to start")
            time.sleep(0.1)
        check = Check(name)
        try:
            check.run()
        finally:
            apps = {n: hashlib.sha256((args.apps / n).read_bytes()).hexdigest() for n in ("ytplay", "slides", "doomtv")}
            (out / "check.json").write_text(json.dumps({"apps": apps, "results": check.results,
                                                        "runtime": "embedded PID1 launcher, QEMU virt surrogate",
                                                        "youtube_data": "local fixtures"}, indent=2) + "\n")
        failed = [k for k, v in check.results.items() if not v["passed"]]
        print(f"{'FAIL' if failed else 'PASS'}: {len(check.results) - len(failed)}/{len(check.results)} UI cases; evidence {out}")
        return 1 if failed else 0
    finally:
        if process.poll() is None:
            pid = int((session.BASE / name / "qemu.pid").read_text())
            subprocess.run(["kill", str(pid)])
            process.wait(timeout=10)


if __name__ == "__main__":
    raise SystemExit(main())
