#!/usr/bin/env python3
"""Stage 0 recorder: capture one client's exact YouTube traffic plus media fixtures.

1. Starts mitmdump (ephemeral, via uvx) and runs a pinned yt-dlp through it for
   one client, downloading one small audio format. The HAR keeps every request
   and response body except media bodies over 512 KiB (headers are kept).
2. Downloads the requested formats directly (no proxy) as replay fixtures,
   one file per itag, plus the info JSON with the resolved URLs.

Run from the residential connection, NOT from eduardo-vm or QEMU loops:
  python3 research/yt-stage0/record.py --deno PATH --client tv \
      --formats 140,251,134,137,248 VIDEO_ID
Output: assets/yt-stage0/record-<video>-<client>-<UTC>/
"""
import argparse
import datetime
import os
import pathlib
import shutil
import signal
import socket
import subprocess
import sys
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]


def free_port():
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("video")
    ap.add_argument("--client", default="tv")
    ap.add_argument("--formats", default="",
                    help="comma-separated itags to save as fixtures (empty = skip)")
    ap.add_argument("--proxy-format", default="139/249/140",
                    help="small format to download through the proxy")
    ap.add_argument("--ytdlp-version", default="2026.10.26")
    ap.add_argument("--deno", default=shutil.which("deno"))
    ap.add_argument("--out", type=pathlib.Path)
    args = ap.parse_args()

    if not args.deno or not os.access(args.deno, os.X_OK):
        sys.exit("need a deno binary: pass --deno PATH")
    env = dict(os.environ, PATH=f"{os.path.dirname(args.deno)}:{os.environ['PATH']}")
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    out = args.out or ROOT / "assets" / "yt-stage0" / f"record-{args.video}-{args.client}-{stamp}"
    (out / "proxied").mkdir(parents=True, exist_ok=True)
    url = f"https://www.youtube.com/watch?v={args.video}"
    ytdlp = ["uvx", "--from", f"yt-dlp[default]=={args.ytdlp_version}", "yt-dlp",
             "-v", "--no-playlist", "--no-cache-dir",
             "--extractor-args", f"youtube:player_client={args.client}"]

    # 1. Proxied capture.
    port = free_port()
    mitm = subprocess.Popen(
        ["uvx", "--from", "mitmproxy==12.3.1", "mitmdump", "-q",
         "--listen-host", "127.0.0.1", "-p", str(port),
         "--set", f"hardump={out / 'traffic.har'}",
         "--set", "stream_large_bodies=512k"],
        stdout=subprocess.DEVNULL, stderr=open(out / "mitmdump.log", "w"),
        start_new_session=True)  # uvx does not forward signals; signal the group
    try:
        for _ in range(100):
            try:
                socket.create_connection(("127.0.0.1", port), timeout=0.2).close()
                break
            except OSError:
                time.sleep(0.2)
        else:
            sys.exit("mitmdump did not start; see mitmdump.log")
        p = subprocess.run(
            ytdlp + ["--proxy", f"http://127.0.0.1:{port}", "--no-check-certificates",
                     "--write-info-json", "-f", args.proxy_format,
                     "-o", str(out / "proxied" / "%(format_id)s.%(ext)s"), url],
            capture_output=True, text=True, env=env)
        (out / "proxied" / "yt-dlp.log").write_text(p.stderr)
        print(f"proxied run exit={p.returncode}")
    finally:
        os.killpg(mitm.pid, signal.SIGINT)  # mitmdump writes the HAR on shutdown
        try:
            mitm.wait(timeout=30)
        except subprocess.TimeoutExpired:
            os.killpg(mitm.pid, signal.SIGKILL)
    print(f"HAR: {out / 'traffic.har'} exists={(out / 'traffic.har').exists()}")

    # 2. Direct fixture downloads.
    if args.formats:
        (out / "fixtures").mkdir(exist_ok=True)
        p = subprocess.run(
            ytdlp + ["--write-info-json", "--fixup", "never", "-f", args.formats,
                     "-o", str(out / "fixtures" / "%(format_id)s.%(ext)s"), url],
            capture_output=True, text=True, env=env)
        (out / "fixtures" / "yt-dlp.log").write_text(p.stderr)
        print(f"fixtures exit={p.returncode}")
        for f in sorted((out / "fixtures").iterdir()):
            print(f"  {f.name:24} {f.stat().st_size:>12,}")
    print(f"wrote {out}")


if __name__ == "__main__":
    main()
