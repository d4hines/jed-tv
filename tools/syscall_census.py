#!/usr/bin/env python3
"""Name the syscalls and ioctls in a ytplay census (ytplay.trace=1).
  python3 tools/syscall_census.py SERIAL_LOG [--markdown]
Syscall tables come from the kernel we built (assets/build/kernel/tables/,
copied from the 6.18.57 source): scripts/syscall.tbl for arm64,
arch/arm/tools/syscall.tbl for 32-bit ARM.
"""
import pathlib
import re
import sys

TABLES = pathlib.Path(__file__).resolve().parents[1] / "assets/build/kernel/tables"

# ioctls the platform uses, by (type char, nr). Size/direction are decoded generically.
IOCTL_NAMES = {
    ("F", 0x00): "FBIOGET_VSCREENINFO", ("F", 0x02): "FBIOGET_FSCREENINFO",
    ("F", 0x06): "FBIOPAN_DISPLAY", ("F", 0x20): "FBIO_WAITFORVSYNC",
    ("A", 0x00): "SNDRV_PCM_IOCTL_PVERSION", ("A", 0x01): "SNDRV_PCM_IOCTL_INFO",
    ("A", 0x11): "SNDRV_PCM_IOCTL_HW_PARAMS", ("A", 0x13): "SNDRV_PCM_IOCTL_SW_PARAMS",
    ("A", 0x21): "SNDRV_PCM_IOCTL_DELAY", ("A", 0x40): "SNDRV_PCM_IOCTL_PREPARE",
    ("A", 0x44): "SNDRV_PCM_IOCTL_DRAIN", ("A", 0x50): "SNDRV_PCM_IOCTL_WRITEI_FRAMES",
    ("E", 0x06): "EVIOCGNAME", ("T", 0x01): "TCGETS", ("T", 0x13): "TIOCGWINSZ",
    ("T", 0x21): "FIONBIO",
}


def load_table(arch):
    names = {}
    if arch == "aarch64":
        for line in (TABLES / "generic-syscall.tbl").read_text().splitlines():
            f = line.split()
            if len(f) >= 3 and f[0].isdigit() and f[1] in ("common", "64"):
                names.setdefault(int(f[0]), f[2])
    else:
        for line in (TABLES / "arm-syscall.tbl").read_text().splitlines():
            f = line.split()
            if len(f) >= 3 and f[0].isdigit() and f[1] in ("common", "eabi"):
                names.setdefault(int(f[0]), f[2])
        names.update({0x0f0005: "ARM_set_tls", 0x0f0002: "ARM_cacheflush"})
    return names


def ioctl_name(cmd):
    d, size, ty, nr = cmd >> 30, (cmd >> 16) & 0x3fff, (cmd >> 8) & 0xff, cmd & 0xff
    t = chr(ty) if 32 <= ty < 127 else f"0x{ty:02x}"
    name = IOCTL_NAMES.get((t, nr), "?")
    if cmd == 0x5401:
        name = "TCGETS"
    elif cmd == 0x5413:
        name = "TIOCGWINSZ"
    elif cmd == 0x5421:
        name = "FIONBIO"
    return f"{name} (dir {['none', 'W', 'R', 'RW'][d]}, '{t}', 0x{nr:02x}, {size} B)"


def main():
    text = pathlib.Path(sys.argv[1]).read_text(errors="replace")
    md = "--markdown" in sys.argv
    arch = re.search(r"census: arch (\S+)", text)
    if not arch:
        sys.exit("no census in log")
    arch = arch.group(1)
    names = load_table(arch)
    sc = [(int(a), int(b)) for a, b in re.findall(r"census: syscall (\d+) (\d+)", text)]
    io = [(int(a, 16), int(b)) for a, b in re.findall(r"census: ioctl 0x([0-9a-f]+) (\d+)", text)]
    head = re.search(r"census: arch .*", text).group(0)
    print(f"{head}: {len(sc)} distinct syscalls, {len(io)} distinct ioctls")
    rows = sorted(((names.get(nr, f"#{nr}"), nr, n) for nr, n in sc), key=lambda r: -r[2])
    if md:
        print("\n| syscall | nr | calls |\n|---|---|---|")
        for name, nr, n in rows:
            print(f"| {name} | {nr} | {n} |")
        print("\n| ioctl | code | calls |\n|---|---|---|")
        for cmd, n in sorted(io, key=lambda r: -r[1]):
            print(f"| {ioctl_name(cmd)} | 0x{cmd:08x} | {n} |")
    else:
        for name, nr, n in rows:
            print(f"  {name:24} {nr:>4} {n:>8}")
        for cmd, n in sorted(io, key=lambda r: -r[1]):
            print(f"  ioctl 0x{cmd:08x} {n:>8}  {ioctl_name(cmd)}")


if __name__ == "__main__":
    main()
