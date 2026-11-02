#!/usr/bin/env python3
"""Build the city flight GPU renderer, upload it to the native PID 1 session on
the G06 and start it. Then start City flight from the home screen: it finds the
renderer and the Mali draws the picture.

Needs the TV booted into the native session with its console reachable
(tools/pid1/graphics.py), and the exact Bionic libraries for linking
(tools/gpu/build-flightgpu.sh).
"""
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/pid1'))
from graphics import console, upload


def main():
    binary = subprocess.run(['sh', 'tools/gpu/build-flightgpu.sh'], cwd=ROOT, check=True, capture_output=True, text=True).stdout.strip().splitlines()[-1]
    upload(Path(binary))
    upload(ROOT / 'src/pid1/scripts/mount-stock-runtime.sh')
    upload(ROOT / 'src/pid1/scripts/flight-gpu.sh')
    console('sh /tmp/flight-gpu.sh')
    print('Start City flight from the home screen; the log is /tmp/gpu/flightgpu.log on the TV', flush=True)


if __name__ == '__main__':
    main()
