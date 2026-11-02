#!/usr/bin/env python3
"""Offline Lean parser/navigation checks and full-deck/image rendering."""
import pathlib
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[2]
PACKAGE = ROOT / 'test/slides'
OUT = ROOT / 'assets/build/lean-slides/verification'

def run(*args, cwd=PACKAGE):
    subprocess.run([str(a) for a in args], cwd=cwd, check=True)

def ppm(path):
    magic, shape, maxval, pixels = path.read_bytes().split(b'\n', 3)
    assert magic == b'P6' and shape == b'1280 720' and maxval == b'255'
    assert len(pixels) == 1280*720*3
    return pixels

def main():
    OUT.mkdir(parents=True, exist_ok=True)
    run('python3', ROOT / 'tools/slides/prepare.py')
    run('lake', 'exe', 'check')
    run('lake', 'exe', 'preview', OUT / 'deck')
    for i in range(1,13):
        pixels = ppm(OUT / f'deck/slide-{i}.ppm')
        assert pixels.count(bytes([240,240,245])) > 1000, f'no visible text on slide {i}'
    try:
        run('python3', ROOT / 'tools/slides/prepare.py', PACKAGE / 'testdata/features.md')
        run('lake', 'exe', 'preview', OUT / 'features')
        for i in range(1,7):
            ppm(OUT / f'features/slide-{i}.ppm')
        pixels = ppm(OUT / 'features/slide-5.ppm')
        # The fixture's saturated test-card colors must survive conversion/blit.
        saturated = sum(max(pixels[p:p+3])-min(pixels[p:p+3]) > 150 for p in range(0,len(pixels),3))
        assert saturated > 10000, f'image missing: only {saturated} saturated pixels'
    finally:
        run('python3', ROOT / 'tools/slides/prepare.py')
        run('lake', 'exe', 'check')
    print(f'PASS: complete deck and image fixture rendered; evidence: {OUT}')

if __name__ == '__main__':
    main()
