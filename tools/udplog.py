#!/usr/bin/env python3
"""Receive tvinit's UDP log (tvinit.log=IP:PORT) and print/append it.
  python3 tools/udplog.py [--port 5140] [--out FILE]
On the TV link: run on the laptop (192.168.77.1) and boot with
tvinit.log=192.168.77.1:5140. In QEMU user networking the guest reaches the
host as 10.0.2.2.
"""
import argparse
import socket
import sys

ap = argparse.ArgumentParser()
ap.add_argument("--host", default="0.0.0.0")
ap.add_argument("--port", type=int, default=5140)
ap.add_argument("--out")
a = ap.parse_args()
s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)
s.bind((a.host, a.port))
out = open(a.out, "a", buffering=1) if a.out else None
print(f"listening on udp {a.host}:{a.port}", file=sys.stderr, flush=True)
while True:
    data, peer = s.recvfrom(65535)
    text = data.decode(errors="replace")
    sys.stdout.write(text)
    sys.stdout.flush()
    if out:
        out.write(text)
