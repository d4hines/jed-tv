#!/usr/bin/env python3
"""Compare a tvbare screendump with a ytplay self-test screendump.

  python3 baremetal/pngcmp.py A.png B.png

Both show the self-test frame: colour bars over the top 2/3 and a moving
80x80 square in the black band. Reports where the square is in each image
and how many pixels differ once the square band is masked out.
(Plain zlib PNG decoding: QEMU writes 8-bit RGB, non-interlaced.)
"""
import struct
import sys
import zlib


def read_png(path):
    data = open(path, "rb").read()
    assert data[:8] == b"\x89PNG\r\n\x1a\n", path
    pos, idat = 8, b""
    while pos < len(data):
        n, kind = struct.unpack(">I4s", data[pos:pos + 8])
        body = data[pos + 8:pos + 8 + n]
        if kind == b"IHDR":
            w, h, depth, ctype, _, _, interlace = struct.unpack(">IIBBBBB", body)
            assert depth == 8 and ctype in (2, 6) and interlace == 0, (depth, ctype, interlace)
        elif kind == b"IDAT":
            idat += body
        pos += 12 + n
    bpp = 3 if ctype == 2 else 4
    raw, stride = zlib.decompress(idat), w * bpp
    rows, prev = [], bytearray(stride)
    for y in range(h):
        f, line = raw[y * (stride + 1)], bytearray(raw[y * (stride + 1) + 1:(y + 1) * (stride + 1)])
        for i in range(stride):
            a = line[i - bpp] if i >= bpp else 0
            b, c = prev[i], prev[i - bpp] if i >= bpp else 0
            if f == 1:
                line[i] = (line[i] + a) & 255
            elif f == 2:
                line[i] = (line[i] + b) & 255
            elif f == 3:
                line[i] = (line[i] + (a + b) // 2) & 255
            elif f == 4:
                p = a + b - c
                pa, pb, pc = abs(p - a), abs(p - b), abs(p - c)
                line[i] = (line[i] + (a if pa <= pb and pa <= pc else b if pb <= pc else c)) & 255
        rows.append([tuple(line[x * bpp:x * bpp + 3]) for x in range(w)])
        prev = line
    return w, h, rows


def square(rows, w, h):
    """Bounding box of white pixels in the black band (the square)."""
    pts = [(x, y) for y in range(h * 2 // 3, h) for x in range(w) if rows[y][x] != (0, 0, 0)]
    if not pts:
        return None
    xs, ys = [p[0] for p in pts], [p[1] for p in pts]
    return min(xs), min(ys), max(xs) - min(xs) + 1, max(ys) - min(ys) + 1, rows[ys[0]][xs[0]]


def main():
    (wa, ha, a), (wb, hb, b) = read_png(sys.argv[1]), read_png(sys.argv[2])
    assert (wa, ha) == (wb, hb), ((wa, ha), (wb, hb))
    sa, sb = square(a, wa, ha), square(b, wb, hb)
    print(f"size {wa}x{ha}; square A {sa}; square B {sb}")
    diff = sum(1 for y in range(ha * 2 // 3) for x in range(wa) if a[y][x] != b[y][x])
    print(f"bars region (rows 0..{ha * 2 // 3}): {diff} differing pixels of {wa * ha * 2 // 3}")
    bars = [a[0][i * wa // 8 + 10] for i in range(8)]
    print(f"bar colours A: {bars}")


if __name__ == "__main__":
    main()
