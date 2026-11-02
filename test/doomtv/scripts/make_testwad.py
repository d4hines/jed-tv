#!/usr/bin/env python3
"""Write a copy of an IWAD with a map's THINGS edited, for flow tests that
random input cannot reach (e.g. the Doom II boss brain).

usage: make_testwad.py IN.wad OUT.wad MAP [move:TYPE:X:Y:ANGLE] [add:TYPE:X:Y:ANGLE:FLAGS] ...
  move: moves the first thing of TYPE; add: appends a thing.
The edited THINGS lump is appended to the file and its directory entry
repointed; every other lump is byte-identical.
"""
import struct, sys

src, dst, mapname, *edits = sys.argv[1:]
d = bytearray(open(src, "rb").read())
ident, n, off = struct.unpack_from("<4sii", d, 0)
names = [d[off + 16 * i + 8: off + 16 * i + 16].rstrip(b"\0").decode() for i in range(n)]
m = names.index(mapname)
ti = next(i for i in range(m + 1, m + 11) if names[i] == "THINGS")
o, s = struct.unpack_from("<ii", d, off + 16 * ti)
things = [list(struct.unpack_from("<hhhhh", d, o + 10 * k)) for k in range(s // 10)]
for e in edits:
    kind, *v = e.split(":")
    v = list(map(int, v))
    if kind == "move":
        t = next(t for t in things if t[3] == v[0])
        t[0], t[1], t[2] = v[1], v[2], v[3]
    elif kind == "add":
        things.append([v[1], v[2], v[3], v[0], v[4]])
    else:
        raise SystemExit(f"bad edit {e!r}")
lump = b"".join(struct.pack("<hhhhh", *t) for t in things)
struct.pack_into("<ii", d, off + 16 * ti, len(d), len(lump))
d += lump
open(dst, "wb").write(d)
print(f"{dst}: {mapname} THINGS {len(things)} entries")
