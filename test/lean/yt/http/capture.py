#!/usr/bin/env python3
"""Record raw HTTP/1.1 exchanges with YouTube for the Lean HTTP/JSON tests.

Sends byte-for-byte the requests ytplay's Rust client sends (src/ytplay/src/
http.rs + yt.rs: Host, Connection: close, the given headers, Content-Length;
serde_json bodies with sorted keys) over TLS and stores the request, the raw
response bytes (status line, headers, framing intact) and the decoded body.
Standard library only. Two live requests: one search, one thumbnail.

  python3 test/lean/yt/http/capture.py OUT_DIR ["search query"]
"""
import json
import pathlib
import socket
import ssl
import sys

UA = ("Mozilla/5.0 (Macintosh; Intel Mac OS X 15_7_3) AppleWebKit/605.1.15 "
      "(KHTML, like Gecko) Version/26.0 Safari/605.1.15")


def request_bytes(method, host, path, headers, body):
    req = f"{method} {path} HTTP/1.1\r\nHost: {host}\r\nConnection: close\r\n"
    for k, v in headers:
        req += f"{k}: {v}\r\n"
    if body or method == "POST":
        req += f"Content-Length: {len(body)}\r\n"
    return (req + "\r\n").encode() + body


def exchange(host, req):
    ctx = ssl.create_default_context()
    with socket.create_connection((host, 443), timeout=20) as raw:
        with ctx.wrap_socket(raw, server_hostname=host) as s:
            s.sendall(req)
            out = bytearray()
            while True:
                b = s.recv(65536)
                if not b:
                    break
                out += b
    return bytes(out)


def decode(resp):
    head, _, rest = resp.partition(b"\r\n\r\n")
    lines = head.split(b"\r\n")
    status = int(lines[0].split()[1])
    hdrs = {}
    for l in lines[1:]:
        k, _, v = l.partition(b":")
        hdrs[k.strip().lower().decode()] = v.strip().decode()
    if hdrs.get("transfer-encoding", "").lower() == "chunked":
        body = bytearray()
        while True:
            line, _, rest = rest.partition(b"\r\n")
            n = int(line.split(b";")[0], 16)
            if n == 0:
                break
            body += rest[:n]
            rest = rest[n + 2:]
        return status, hdrs, bytes(body)
    if "content-length" in hdrs:
        return status, hdrs, rest[:int(hdrs["content-length"])]
    return status, hdrs, rest


def main(out, query="big buck bunny"):
    out = pathlib.Path(out)
    out.mkdir(parents=True, exist_ok=True)
    body = json.dumps({"context": {"client": {"clientName": "WEB", "clientVersion": "2.20260708.00.00",
                                              "hl": "en", "gl": "US"}}, "query": query},
                      sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()
    jobs = [
        ("search", "www.youtube.com",
         request_bytes("POST", "www.youtube.com", "/youtubei/v1/search?prettyPrint=false",
                       [("Content-Type", "application/json"), ("User-Agent", UA)], body), "search.json"),
        ("thumb", "i.ytimg.com",
         request_bytes("GET", "i.ytimg.com", "/vi/PVGeM40dABA/mqdefault.jpg", [], b""), "mqdefault.jpg"),
    ]
    for name, host, req, body_name in jobs:
        resp = exchange(host, req)
        (out / f"{name}.request").write_bytes(req)
        (out / f"{name}.response").write_bytes(resp)
        status, hdrs, body = decode(resp)
        (out / body_name).write_bytes(body)
        print(name, status, len(resp), "bytes raw,", len(body), "body,",
              "chunked" if "transfer-encoding" in hdrs else f"content-length {hdrs.get('content-length')}")


if __name__ == "__main__":
    main(*sys.argv[1:])
