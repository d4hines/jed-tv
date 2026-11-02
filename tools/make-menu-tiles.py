#!/usr/bin/env python3
"""Render the vendored app logos to the launcher's 288x162 RGB24 tile format."""
import argparse
from pathlib import Path
import subprocess

WIDTH, HEIGHT = 288, 162
SOURCES = Path(__file__).resolve().parents[1] / 'demo/tiles'


def render(source, output, logo_filter):
    filters = (f'[1:v]{logo_filter}[logo];'
               '[0:v][logo]overlay=x=(W-w)/2:y=8:format=auto:shortest=1,format=rgb24')
    subprocess.run(['ffmpeg', '-v', 'error', '-y', '-f', 'lavfi', '-i',
                    f'color=c=0x161a2c:s={WIDTH}x{HEIGHT}:r=1:d=1',
                    '-i', str(source), '-filter_complex', filters, '-frames:v', '1',
                    '-pix_fmt', 'rgb24', '-f', 'rawvideo', str(output)], check=True)
    if output.stat().st_size != WIDTH * HEIGHT * 3:
        raise RuntimeError(f'Invalid tile byte length: {output}')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    parser.add_argument('--source-dir', type=Path, default=SOURCES)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=True)
    # Leave the bottom 36 pixels free for the title that Lean draws in front.
    render(args.source_dir / 'youtube.png', args.output / 'youtube.rgb',
           'crop=144:108:0:18,scale=152:114:flags=lanczos,format=rgba')
    # The upstream wordmark is black; render its unchanged alpha silhouette in
    # white for contrast on the dark tile. It remains distinct from its caption.
    render(args.source_dir / 'doom.png', args.output / 'doom.rgb',
           'scale=178:114:flags=lanczos,format=rgba,lutrgb=r=255:g=255:b=255')


if __name__ == '__main__':
    main()
