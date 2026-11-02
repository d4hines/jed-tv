#!/usr/bin/env python3
"""Test JPEGs and reference decodes for the Lean JPEG test.

Run inside a shell with libjpeg-turbo's tools (tools/lean/yt/Fixtures.lean
jpeg does this).
Inputs: the YouTube thumbnails (480x360 hqdefault, 320x180 mqdefault) plus
variants made with cjpeg/jpegtran from one of them: every chroma
subsampling djpeg's fancy upsampling distinguishes (4:2:0, 4:2:2, 4:4:0,
4:4:4, 4:1:1), grayscale, RGB, restart intervals, progressive (with
successive approximation), optimized tables, 16-bit quantization tables,
multi-scan sequential, odd and tiny sizes. Reference: `djpeg -rgb` with
JSIMD_FORCENONE=1 (libjpeg-turbo's C code); also checked equal to the SIMD
build's output. Writes NAME.jpg.b64, NAME.rgb.b64 (raw RGB24) and
manifest.txt ("NAME WIDTH HEIGHT").

  python3 -I ref.py OUT_DIR THUMB.jpg...
"""
import base64
import os
import pathlib
import subprocess
import sys


def run(args, data=None, env=None, warnings_ok=False):
    r = subprocess.run(args, input=data, capture_output=True, env=env)
    # djpeg exits 2 after warnings (e.g. "Premature end of JPEG file").
    if r.returncode != 0 and not (warnings_ok and r.returncode == 2):
        raise RuntimeError("%s: exit %d: %s" % (args, r.returncode, r.stderr.decode()))
    return r.stdout


def ppm(b):
    """(width, height, maxval, pixels) of a binary PPM/PGM."""
    fields, i = [], 0
    while len(fields) < 4:
        while b[i:i + 1].isspace():
            i += 1
        if b[i:i + 1] == b"#":
            while b[i:i + 1] != b"\n":
                i += 1
            continue
        j = i
        while not b[j:j + 1].isspace():
            j += 1
        fields.append(b[i:j])
        i = j
    assert fields[0] == b"P6", fields[0]
    return int(fields[1]), int(fields[2]), b[i + 1:]


def crop(src, w, h):
    sw, sh, px = ppm(src)
    rows = [px[(y * sw) * 3:(y * sw + w) * 3] for y in range(h)]
    return b"P6\n%d %d\n255\n" % (w, h) + b"".join(rows)


def main(out, *thumbs):
    out = pathlib.Path(out)
    out.mkdir(parents=True, exist_ok=True)
    cases = []
    for t in thumbs:
        cases.append(("yt-" + pathlib.Path(t).stem.replace("_", "-"), pathlib.Path(t).read_bytes()))
    src = run(["djpeg", "-ppm", thumbs[0]])
    sw, sh, _ = ppm(src)
    odd = crop(src, 333, 197)
    tiny = {k: crop(src, *k) for k in [(1, 1), (3, 2), (17, 9), (2, 9)]}
    scans = out / "seq.scans"
    scans.write_text("0;\n1;\n2;\n")
    cj = lambda args, img=src: run(["cjpeg"] + args, img)
    cases += [
        ("s420", cj(["-quality", "85", "-sample", "2x2"])),
        ("s422", cj(["-quality", "85", "-sample", "2x1"])),
        ("s440", cj(["-quality", "85", "-sample", "1x2"])),
        ("s444", cj(["-quality", "85", "-sample", "1x1"])),
        ("s411", cj(["-quality", "85", "-sample", "4x1"])),
        ("gray", cj(["-grayscale"])),
        ("rgb", cj(["-rgb"])),
        ("rst3blocks", cj(["-sample", "2x2", "-restart", "3B"])),
        ("rst1row-422", cj(["-sample", "2x1", "-restart", "1"])),
        ("prog420", cj(["-progressive"])),
        ("prog444", cj(["-progressive", "-sample", "1x1"])),
        ("prog-rst", cj(["-progressive", "-restart", "2B"])),
        ("prog-gray", cj(["-progressive", "-grayscale"])),
        ("optimized", cj(["-optimize", "-quality", "95"])),
        ("q100", cj(["-quality", "100", "-sample", "1x1"])),
        ("q5", cj(["-quality", "5"])),
        ("q1-16bit", cj(["-quality", "1"])),
        ("seq-scans", cj(["-scans", str(scans)])),
        ("odd-420", cj(["-sample", "2x2"], odd)),
        ("odd-prog-422", cj(["-progressive", "-sample", "2x1"], odd)),
        ("odd-440", cj(["-sample", "1x2"], odd)),
        ("jpegtran-prog", run(["jpegtran", "-progressive", thumbs[0]])),
        ("jpegtran-rst", run(["jpegtran", "-restart", "1", thumbs[-1]])),
    ]
    yt = cases[1][1]
    prog = dict(cases)["prog420"]
    rst = dict(cases)["rst3blocks"]
    cases += [("trunc50", yt[:len(yt) // 2]), ("trunc90", yt[:len(yt) * 9 // 10]),
              ("trunc-prog60", prog[:len(prog) * 6 // 10]), ("trunc-rst70", rst[:len(rst) * 7 // 10])]
    for (w, h), img in tiny.items():
        cases.append(("tiny-%dx%d" % (w, h), cj(["-sample", "2x2"], img)))
        cases.append(("tiny-%dx%d-422" % (w, h), cj(["-sample", "2x1"], img)))
    manifest = []
    plain = dict(os.environ, JSIMD_FORCENONE="1")
    simd_same = 0
    for name, jpg in cases:
        ref = run(["djpeg", "-rgb", "-ppm"], jpg, env=plain, warnings_ok=True)
        if run(["djpeg", "-rgb", "-ppm"], jpg, warnings_ok=True) == ref:
            simd_same += 1
        w, h, px = ppm(ref)
        assert len(px) == w * h * 3
        (out / (name + ".jpg.b64")).write_text(base64.b64encode(jpg).decode())
        (out / (name + ".rgb.b64")).write_text(base64.b64encode(px).decode())
        manifest.append("%s %d %d %d" % (name, w, h, len(jpg)))
        if name.startswith("yt-"):
            # browse.rs's nearest-neighbour scale to the 288x162 tile.
            aw, ah = 288, 162
            sc = bytearray()
            for y in range(ah):
                for x in range(aw):
                    o = ((y * h // ah) * w + x * w // aw) * 3
                    sc += px[o:o + 3]
            (out / (name + ".scaled.b64")).write_text(base64.b64encode(bytes(sc)).decode())
    (out / "manifest.txt").write_text("\n".join(manifest) + "\n")
    print(len(cases), "cases;", simd_same, "with identical SIMD and C reference output")


if __name__ == "__main__":
    main(*sys.argv[1:])
