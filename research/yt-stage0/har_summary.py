#!/usr/bin/env python3
"""Print a readable transcript of a recorder HAR: one entry per request.

Shows method, host, path, query parameter names (values elided when long),
JSON request bodies, response status/type/size, and the top-level keys of JSON
responses. Long opaque strings are shortened so the transcript stays readable.
  python3 research/yt-stage0/har_summary.py assets/yt-stage0/record-.../traffic.har
"""
import json
import sys
import urllib.parse


def shorten(v, n=48):
    s = v if isinstance(v, str) else json.dumps(v)
    return s if len(s) <= n else f"{s[:n // 2]}…{s[-8:]} ({len(s)} chars)"


def squash(obj, depth=0):
    """Copy of a JSON value with long strings shortened and deep nesting cut."""
    if isinstance(obj, dict):
        if depth > 4:
            return f"{{…{len(obj)} keys}}"
        return {k: squash(v, depth + 1) for k, v in obj.items()}
    if isinstance(obj, list):
        if depth > 4:
            return f"[…{len(obj)}]"
        return [squash(v, depth + 1) for v in obj[:3]] + ([f"…+{len(obj) - 3}"] if len(obj) > 3 else [])
    if isinstance(obj, str):
        return shorten(obj)
    return obj


def main(path):
    har = json.load(open(path))
    for i, e in enumerate(har["log"]["entries"], 1):
        req, resp = e["request"], e["response"]
        u = urllib.parse.urlsplit(req["url"])
        q = urllib.parse.parse_qsl(u.query, keep_blank_values=True)
        print(f"\n## {i}. {req['method']} {u.netloc}{u.path}")
        if q:
            print("query: " + ", ".join(f"{k}={shorten(v, 24)}" for k, v in q))
        hdrs = {h["name"].lower(): h["value"] for h in req["headers"]}
        for h in ("user-agent", "range", "content-type", "x-youtube-client-name",
                  "x-youtube-client-version", "origin", "referer"):
            if h in hdrs:
                print(f"{h}: {shorten(hdrs[h], 120)}")
        body = (req.get("postData") or {}).get("text")
        if body:
            try:
                print("request body:\n" + json.dumps(squash(json.loads(body)), indent=1))
            except ValueError:
                print(f"request body ({len(body)} bytes, not JSON)")
        c = resp.get("content", {})
        print(f"-> {resp['status']} {c.get('mimeType', '')} {c.get('size', '?')} bytes")
        text = c.get("text")
        if text and "json" in (c.get("mimeType") or ""):
            try:
                print("response keys: " + ", ".join(json.loads(text).keys()))
            except (ValueError, AttributeError):
                pass


if __name__ == "__main__":
    main(sys.argv[1])
