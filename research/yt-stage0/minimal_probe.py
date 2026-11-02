#!/usr/bin/env python3
"""Find the smallest request sequence that yields playable YouTube streams.

Standard library only: no yt-dlp, no watch page, no API key, no cookies, no JS.
For each header variant it POSTs youtubei/v1/player once, then does a 64 KiB
ranged GET on the best video-only and audio-only URL with no extra headers.
Stores responses under assets/yt-stage0/minimal-<UTC>/ for later replay.

Run from the residential connection:
  python3 research/yt-stage0/minimal_probe.py VIDEO_ID [--client visionos]
"""
import argparse
import datetime
import json
import pathlib
import time
import urllib.error
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parents[2]

# Client identities copied from yt-dlp 2026.10.26 (yt_dlp/extractor/youtube/_base.py).
CLIENTS = {
    "visionos": {
        "id": 101,
        "client": {
            "clientName": "VISIONOS", "clientVersion": "1.02",
            "deviceMake": "Apple", "deviceModel": "RealityDevice17,1",
            "osName": "visionOS", "osVersion": "26.5.23O471", "hl": "en",
            "userAgent": ("Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 "
                          "(KHTML, like Gecko) Version/26.0 Safari/605.1.15"),
        },
    },
}


def player(video, client, headers, visitor=None, sts=None):
    ctx = dict(client["client"])
    if visitor:
        ctx["visitorData"] = visitor
    body = {"context": {"client": ctx}, "videoId": video,
            "contentCheckOk": True, "racyCheckOk": True}
    if sts:
        body["playbackContext"] = {"contentPlaybackContext": {"signatureTimestamp": sts}}
    req = urllib.request.Request(
        "https://www.youtube.com/youtubei/v1/player?prettyPrint=false",
        data=json.dumps(body).encode(),
        headers={"Content-Type": "application/json", **headers})
    try:
        with urllib.request.urlopen(req, timeout=20) as r:
            return r.status, json.loads(r.read())
    except urllib.error.HTTPError as e:
        return e.code, {"error": e.read()[:500].decode(errors="replace")}


def ranged_get(url, headers):
    req = urllib.request.Request(url, headers={"Range": "bytes=0-65535", **headers})
    try:
        with urllib.request.urlopen(req, timeout=20) as r:
            return r.status, len(r.read(65536))
    except urllib.error.HTTPError as e:
        return e.code, 0
    except Exception as e:
        return repr(e), 0


def pick(fmts, kind):
    if kind == "video":
        c = [f for f in fmts if f.get("mimeType", "").startswith("video/")]
        return max(c, key=lambda f: (f.get("height", 0), f.get("bitrate", 0)), default=None)
    c = [f for f in fmts if f.get("mimeType", "").startswith("audio/")]
    return max(c, key=lambda f: f.get("bitrate", 0), default=None)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("video")
    ap.add_argument("--client", default="visionos", choices=sorted(CLIENTS))
    args = ap.parse_args()
    c = CLIENTS[args.client]
    ua = c["client"]["userAgent"]
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%SZ")
    out = ROOT / "assets" / "yt-stage0" / f"minimal-{args.video}-{args.client}-{stamp}"
    out.mkdir(parents=True)

    base = {"User-Agent": ua, "X-YouTube-Client-Name": str(c["id"]),
            "X-YouTube-Client-Version": c["client"]["clientVersion"]}
    report = {"utc": stamp, "video": args.video, "client": args.client, "variants": {}}

    def run(name, hdrs, visitor=None):
        status, data = player(args.video, c, hdrs, visitor=visitor)
        (out / f"player.{name}.json").write_text(json.dumps(data, indent=1))
        ps = data.get("playabilityStatus", {})
        sd = data.get("streamingData", {})
        fmts = sd.get("adaptiveFormats", [])
        row = {"http": status, "playability": ps.get("status"), "reason": ps.get("reason"),
               "adaptive_formats": len(fmts),
               "with_url": sum(1 for f in fmts if "url" in f),
               "with_signatureCipher": sum(1 for f in fmts if "signatureCipher" in f),
               "server_abr_url": "serverAbrStreamingUrl" in sd}
        for kind in ("video", "audio"):
            f = pick([f for f in fmts if "url" in f], kind)
            if f:
                row[f"{kind}_itag"] = f["itag"]
                row[f"{kind}_mime"] = f["mimeType"]
                row[f"{kind}_height"] = f.get("height")
                row[f"{kind}_get_no_headers"] = ranged_get(f["url"], {})
        report["variants"][name] = row
        print(name, json.dumps(row), flush=True)
        time.sleep(2)
        return data

    # Step 1: bare call. Expected to be refused, but it hands out visitorData.
    first = run("bare", base)
    visitor = first.get("responseContext", {}).get("visitorData")
    report["visitor_from_bare_call"] = bool(visitor)
    if visitor:
        # Step 2: send that visitor ID back, as a header and/or in the body.
        run("visitor-header", {**base, "X-Goog-Visitor-Id": visitor})
        run("visitor-body", base, visitor=visitor)
        run("visitor-both-no-client-headers", {"User-Agent": ua, "X-Goog-Visitor-Id": visitor},
            visitor=visitor)
    (out / "report.json").write_text(json.dumps(report, indent=1))
    print(f"wrote {out}")


if __name__ == "__main__":
    main()
