#!/usr/bin/env python3
"""Probe YouTube's InnerTube search for the minimal working request.
Standard library only. Run from a residential connection:
  python3 research/yt-stage0/search_probe.py "blender open movie"
Prints, per client identity, whether results come back and the first few
videos (id, title, channel, length, thumbnail URL).
"""
import json
import sys
import urllib.request

CLIENTS = {
    "WEB": {"clientName": "WEB", "clientVersion": "2.20260708.00.00", "hl": "en", "gl": "US"},
    "MWEB": {"clientName": "MWEB", "clientVersion": "2.20260708.05.00", "hl": "en", "gl": "US"},
    "VISIONOS": {"clientName": "VISIONOS", "clientVersion": "1.02", "deviceMake": "Apple",
                 "deviceModel": "RealityDevice17,1", "osName": "visionOS",
                 "osVersion": "26.5.23O471", "hl": "en"},
}


def post(client, query, visitor=None):
    ctx = dict(client)
    if visitor:
        ctx["visitorData"] = visitor
    body = json.dumps({"context": {"client": ctx}, "query": query}).encode()
    req = urllib.request.Request("https://www.youtube.com/youtubei/v1/search?prettyPrint=false",
                                 data=body, headers={"Content-Type": "application/json",
                                                     "User-Agent": "Mozilla/5.0"})
    with urllib.request.urlopen(req, timeout=20) as r:
        return json.loads(r.read())


def videos(obj, out):
    """Collect every videoRenderer / compactVideoRenderer anywhere in the reply."""
    if isinstance(obj, dict):
        for key in ("videoRenderer", "compactVideoRenderer", "tileRenderer"):
            if key in obj and isinstance(obj[key], dict) and obj[key].get("videoId"):
                v = obj[key]
                title = v.get("title", {})
                title = title.get("runs", [{}])[0].get("text") or title.get("simpleText")
                owner = (v.get("ownerText") or v.get("longBylineText") or {}).get("runs", [{}])[0].get("text")
                length = (v.get("lengthText") or {}).get("simpleText")
                thumbs = (v.get("thumbnail") or {}).get("thumbnails", [])
                out.append((v["videoId"], title, owner, length, thumbs[-1]["url"] if thumbs else None))
        for val in obj.values():
            videos(val, out)
    elif isinstance(obj, list):
        for val in obj:
            videos(val, out)
    return out


def main():
    query = sys.argv[1] if len(sys.argv) > 1 else "blender open movie"
    for name, client in CLIENTS.items():
        try:
            r = post(client, query)
            found = videos(r, [])
            print(f"{name}: {len(found)} videos, top keys {list(r.keys())[:5]}")
            for v in found[:4]:
                print("   ", v)
        except Exception as e:
            print(f"{name}: error {e}")


if __name__ == "__main__":
    main()
