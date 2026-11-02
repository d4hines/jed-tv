#!/usr/bin/env python3
"""Write a newc cpio initramfs holding only the player as /init.

Device nodes go straight into the archive, so no root is needed:
  python3 tools/mkinitramfs.py path/to/ytplay out.cpio [HOST_FILE:/guest/path ...]
Contents: /dev, /dev/console (c 5 1), /proc, /sys, /tmp, /init, plus extra files.
"""
import os
import stat
import sys


def entry(name, mode, data=b"", rdev=(0, 0), ino=[1]):
    ino[0] += 1
    hdr = "070701" + "".join(f"{v:08X}" for v in (
        ino[0], mode, 0, 0, 1, 0, len(data), 0, 0, rdev[0], rdev[1], len(name) + 1, 0))
    out = hdr.encode() + name.encode() + b"\0"
    out += b"\0" * (-len(out) % 4)
    out += data + b"\0" * (-len(data) % 4)
    return out


def main(binary, out, *extra):
    init = open(binary, "rb").read()
    files = []
    dirs = {"dev", "proc", "sys", "tmp"}
    for spec in extra:
        src, dst = spec.split(":", 1)
        mode = 0o755 if os.access(src, os.X_OK) else 0o644  # keep executables runnable
        parts = dst.strip("/").split("/")
        for i in range(1, len(parts)):  # parent directories, e.g. bin/
            d = "/".join(parts[:i])
            if d not in dirs:
                dirs.add(d)
                files.append(entry(d, stat.S_IFDIR | 0o755))
        files.append(entry("/".join(parts), stat.S_IFREG | mode, open(src, "rb").read()))
    blob = b"".join([
        entry("dev", stat.S_IFDIR | 0o755),
        entry("dev/console", stat.S_IFCHR | 0o600, rdev=(5, 1)),
        entry("dev/null", stat.S_IFCHR | 0o666, rdev=(1, 3)),
        entry("proc", stat.S_IFDIR | 0o755),
        entry("sys", stat.S_IFDIR | 0o755),
        entry("tmp", stat.S_IFDIR | 0o1777),
        entry("init", stat.S_IFREG | 0o755, init),
        *files,
        entry("TRAILER!!!", 0),
    ])
    blob += b"\0" * (-len(blob) % 512)
    open(out, "wb").write(blob)
    print(f"{out}: {len(blob)} bytes (init {len(init)} bytes)")


if __name__ == "__main__":
    main(*sys.argv[1:])
