#!/usr/bin/env python3
"""Stage 0 census: which YouTube player clients yield playable streams today.

For each (video, client) pair this runs a pinned yt-dlp release with only that
client enabled, dumps the extracted info JSON plus the verbose log, then does a
small ranged GET (first 64 KiB) on the best https video and audio URLs. A 206
response means the URL works with no extra token. Nothing is downloaded beyond
those probes.

Run from the residential connection, NOT from eduardo-vm or QEMU loops:
  python3 research/yt-stage0/census.py --deno /path/to/deno VIDEO_ID [...]
Output: assets/yt-stage0/census-<UTC>/ (summary.json, summary.md, raw logs).
"""
import argparse
import datetime
import json
import os
import pathlib
import re
import shutil
import subprocess
import sys
import time
import urllib.error
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parents[2]
DEFAULT_YTDLP = "2026.10.26"
DEFAULT_CLIENTS = [
    "tv", "tv_simply", "tv_samsung", "tv_downgraded", "web", "web_safari",
    "web_embedded", "mweb", "android", "android_vr", "ios", "visionos",
]
INTERESTING = re.compile(
    r"po.?token|sabr|challenge|sign in|bot|forbidden|http error|unavailable|"
    r"skipp|missing|requires|js runtime|player (?:ver|js)", re.I)


def codec_family(vcodec):
    v = (vcodec or "").lower()
    for prefix, name in (("avc1", "h264"), ("vp09", "vp9"), ("vp9", "vp9"),
                         ("av01", "av1"), ("hev1", "hevc"), ("hvc1", "hevc")):
        if v.startswith(prefix):
            return name
    return v or "?"


def probe(fmt, timeout=20):
    """Ranged GET of the first 64 KiB. Returns a small result dict."""
    headers = dict(fmt.get("http_headers") or {})
    headers["Range"] = "bytes=0-65535"
    req = urllib.request.Request(fmt["url"], headers=headers)
    try:
        with urllib.request.urlopen(req, timeout=timeout) as r:
            body = r.read(65536)
            return {"status": r.status, "type": r.headers.get("Content-Type"),
                    "bytes": len(body), "magic": body[:12].hex()}
    except urllib.error.HTTPError as e:
        return {"status": e.code, "error": str(e)}
    except Exception as e:  # network errors, timeouts
        return {"status": None, "error": repr(e)}


def best(formats, kind):
    """Highest-resolution video-only or highest-bitrate audio-only https format."""
    cand = [f for f in formats if f.get("url") and f.get("protocol") == "https"]
    if kind == "video":
        cand = [f for f in cand if f.get("vcodec") not in (None, "none")
                and f.get("acodec") in (None, "none")]
        key = lambda f: ((f.get("height") or 0), (f.get("tbr") or 0))
    else:
        cand = [f for f in cand if f.get("acodec") not in (None, "none")
                and f.get("vcodec") in (None, "none")]
        key = lambda f: (f.get("abr") or f.get("tbr") or 0)
    return max(cand, key=key) if cand else None


def summarize(info, log_lines):
    fmts = info.get("formats") or []
    by_codec = {}
    for f in fmts:
        if f.get("vcodec") in (None, "none") or not f.get("height"):
            continue
        fam = codec_family(f.get("vcodec"))
        key = f"{fam}/{f.get('protocol')}"
        by_codec[key] = max(by_codec.get(key, 0), f["height"])
    audio = sorted({f"{(f.get('acodec') or '?').split('.')[0]}/{f.get('protocol')}"
                    for f in fmts if f.get("acodec") not in (None, "none")
                    and f.get("vcodec") in (None, "none")})
    return {
        "formats": len(fmts),
        "formats_without_url": sum(1 for f in fmts if not f.get("url")),
        "protocols": sorted({f.get("protocol") or "?" for f in fmts}),
        "max_height_by_codec": dict(sorted(by_codec.items())),
        "audio": audio,
        "muxed_max_height": max((f.get("height") or 0 for f in fmts
                                 if f.get("vcodec") not in (None, "none")
                                 and f.get("acodec") not in (None, "none")
                                 and f.get("protocol") == "https"), default=0),
        "notes": sorted({l.strip() for l in log_lines if INTERESTING.search(l)})[:40],
    }


def main():
    ap = argparse.ArgumentParser(description=__doc__,
                                 formatter_class=argparse.RawDescriptionHelpFormatter)
    ap.add_argument("videos", nargs="+", help="YouTube video IDs")
    ap.add_argument("--clients", default=",".join(DEFAULT_CLIENTS))
    ap.add_argument("--ytdlp-version", default=DEFAULT_YTDLP)
    ap.add_argument("--deno", default=shutil.which("deno"),
                    help="path to deno binary (JS challenge solver runtime)")
    ap.add_argument("--sleep", type=float, default=3.0, help="seconds between runs")
    ap.add_argument("--out", type=pathlib.Path)
    args = ap.parse_args()

    if not args.deno or not os.access(args.deno, os.X_OK):
        sys.exit("need a deno binary: pass --deno PATH")
    env = dict(os.environ, PATH=f"{os.path.dirname(args.deno)}:{os.environ['PATH']}")
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    out = args.out or ROOT / "assets" / "yt-stage0" / f"census-{stamp}"
    (out / "raw").mkdir(parents=True, exist_ok=True)
    ytdlp = ["uvx", "--from", f"yt-dlp[default]=={args.ytdlp_version}", "yt-dlp"]

    results = []
    for vid in args.videos:
        for client in args.clients.split(","):
            tag = f"{vid}.{client}"
            cmd = ytdlp + ["-v", "-J", "--no-playlist", "--no-cache-dir",
                           "--extractor-args", f"youtube:player_client={client}",
                           f"https://www.youtube.com/watch?v={vid}"]
            t0 = time.time()
            p = subprocess.run(cmd, capture_output=True, text=True, env=env)
            (out / "raw" / f"{tag}.log").write_text(p.stderr)
            row = {"video": vid, "client": client, "exit": p.returncode,
                   "seconds": round(time.time() - t0, 1)}
            log_lines = p.stderr.splitlines()
            if p.returncode == 0 and p.stdout.strip():
                info = json.loads(p.stdout)
                (out / "raw" / f"{tag}.json").write_text(json.dumps(info, indent=1))
                row["title"] = info.get("title")
                row["duration"] = info.get("duration")
                row.update(summarize(info, log_lines))
                for kind in ("video", "audio"):
                    f = best(info.get("formats") or [], kind)
                    if f:
                        row[f"probe_{kind}"] = {"itag": f.get("format_id"),
                                                "codec": f.get("vcodec") if kind == "video" else f.get("acodec"),
                                                "height": f.get("height"),
                                                **probe(f)}
            else:
                row["error"] = next((l for l in reversed(log_lines) if "ERROR" in l),
                                    (log_lines or ["(no output)"])[-1])
                row["notes"] = sorted({l.strip() for l in log_lines if INTERESTING.search(l)})[:40]
            results.append(row)
            print(f"{tag:28} exit={row['exit']} "
                  f"v={row.get('probe_video', {}).get('status')} "
                  f"a={row.get('probe_audio', {}).get('status')} "
                  f"{row.get('max_height_by_codec', row.get('error', ''))}", flush=True)
            time.sleep(args.sleep)

    meta = {"utc": stamp, "ytdlp": args.ytdlp_version, "deno": args.deno,
            "clients": args.clients.split(","), "videos": args.videos}
    (out / "summary.json").write_text(json.dumps({"meta": meta, "results": results}, indent=1))

    lines = [f"# Stage 0 census {stamp}", "",
             f"yt-dlp {args.ytdlp_version}; probes are 64 KiB ranged GETs (206 = playable).", "",
             "| video | client | ok | video probe | audio probe | max height by codec/protocol | muxed |",
             "|---|---|---|---|---|---|---|"]
    for r in results:
        pv, pa = r.get("probe_video", {}), r.get("probe_audio", {})
        lines.append(
            f"| {r['video']} | {r['client']} | {'yes' if r['exit'] == 0 else 'NO'} | "
            f"{pv.get('status', '-')} {pv.get('codec', '')} {pv.get('height', '') or ''} | "
            f"{pa.get('status', '-')} {pa.get('codec', '')} | "
            f"{', '.join(f'{k}:{v}' for k, v in r.get('max_height_by_codec', {}).items()) or r.get('error', '')[:80]} | "
            f"{r.get('muxed_max_height', '')} |")
    (out / "summary.md").write_text("\n".join(lines) + "\n")
    print(f"\nwrote {out}")


if __name__ == "__main__":
    main()
