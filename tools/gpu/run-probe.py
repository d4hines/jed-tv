#!/usr/bin/env python3
"""Build the GPU probe, upload it to the native PID 1 session on the G06 and run it.

Needs the TV booted into the native session with its console reachable
(tools/pid1/graphics.py), and the exact Bionic libraries for linking
(tools/gpu/build-probe.sh). Saves the probe log and the rendered frame under
assets/research/gpu-probe/<timestamp>/.
"""
import datetime
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/pid1'))
from graphics import console, upload


def main():
    binary = subprocess.run(['sh', 'tools/gpu/build-probe.sh'], cwd=ROOT, check=True, capture_output=True, text=True).stdout.strip()
    upload(Path(binary))
    upload(ROOT / 'src/pid1/scripts/mount-stock-runtime.sh')
    upload(ROOT / 'research/gpu/gpu-probe.sh')
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    print('GPU probe:', stamp, flush=True)
    console('sh /tmp/gpu-probe.sh ' + ' '.join(sys.argv[1:]))
    out = ROOT / 'assets/research/gpu-probe' / stamp
    out.mkdir(parents=True, exist_ok=True)
    console(f'cat /tmp/gpu/probe.log')
    print('Frame: /tmp/gpu-frame.rgba on the TV (1280x720 RGBA, bottom row first); log under', out, flush=True)


if __name__ == '__main__':
    main()
