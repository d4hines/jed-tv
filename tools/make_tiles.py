#!/usr/bin/env python3
"""Build the home screen's tile art as raw RGB24 (288x162) with ffmpeg.
The TV then needs no image decoder: tvinit reads /share/tiles/<name>.rgb.
  python3 tools/make_tiles.py   -> assets/build/tiles/*.rgb
Sources: YouTube's hqdefault thumbnails (4:3 with the 16:9 picture inside,
fetched into assets/build/tiles/src/), a frame of the Coffee Run fixture, and
the vendored YouTube and Doom logos.
"""
import glob
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUT = ROOT / "assets/build/tiles"
W, H = 288, 162


def ffmpeg(src, vf, out, pre=()):
    subprocess.run(["ffmpeg", "-v", "error", "-y", *pre, "-i", str(src), "-frames:v", "1",
                    "-vf", f"{vf},scale={W}:{H}:flags=lanczos", "-pix_fmt", "rgb24",
                    "-f", "rawvideo", str(out)], check=True)
    assert out.stat().st_size == W * H * 3, out


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    subprocess.run([sys.executable, str(ROOT / "tools/make-menu-tiles.py"), str(OUT)], check=True)
    # 480x360 thumbnails: the 16:9 picture is the middle 480x270.
    for jpg in sorted((OUT / "src").glob("*.jpg")):
        ffmpeg(jpg, "crop=480:270:0:45", OUT / f"yt-{jpg.stem}.rgb")
    fixture = sorted(glob.glob(str(ROOT / "assets/yt-stage0/record-*/fixtures/136.mp4")))[-1]
    ffmpeg(fixture, "crop=ih*16/9:ih", OUT / "yt-PVGeM40dABA.rgb", pre=("-ss", "7"))
    # Slides: the deck's title slide from a run like
    #   test/run.py fast --headless --bin slides --at 2:shot:1-title
    shots = sorted(glob.glob(str(ROOT / "assets/build/qemu-runs/slides-*/1-title.png")))
    if shots:
        ffmpeg(shots[-1], "null", OUT / "slides.rgb")
    for f in sorted(OUT.glob("*.rgb")):
        print(f"{f.name:24} {f.stat().st_size} bytes")


if __name__ == "__main__":
    main()
