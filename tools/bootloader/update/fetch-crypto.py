#!/usr/bin/env python3
"""Fetch only hash-pinned HACL* sources; generated/downloaded inputs stay in assets."""
import hashlib
import json
from pathlib import Path
import urllib.request

ROOT = Path(__file__).resolve().parents[3]
LOCK = Path(__file__).with_name('crypto-lock.json')
OUT = ROOT / 'assets/build/leanboot-update/hacl'


def fetch():
    lock = json.loads(LOCK.read_text())
    for name, expected in lock['files'].items():
        path = OUT / name
        if not path.exists() or hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            data = urllib.request.urlopen(f"https://raw.githubusercontent.com/hacl-star/hacl-star/{lock['revision']}/{name}", timeout=30).read()
            if hashlib.sha256(data).hexdigest() != expected:
                raise RuntimeError(f'HACL* source hash mismatch: {name}')
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
    return OUT


if __name__ == '__main__':
    print(fetch())
