#!/usr/bin/env python3
"""Build this owner's G13 V013 debug ramdisk without replacing stock init.

Does not communicate with or flash a TV. Inputs must be the verified V013
stock image and the live platform policy captured during this investigation.
"""

import argparse
import hashlib
import json
from pathlib import Path
import struct
import subprocess


STOCK_SHA = "ba21e944050398f71b9d137f4c569e930dd912f94aa50d44392aa94f61dfa0b9"
POLICY_SHA = "22a4ead533008aeed1bd698c05b6a07259b4540bdd0df44935c4079eb8cc81f5"
POLICY_ADDITION = b"""
; G13 owner-authorized adb root domain; other domains remain enforcing.
(allow adbd self (process (setcurrent)))
(allow adbd su (process (dyntransition)))
(typepermissive su)
"""


def sha(data):
    return hashlib.sha256(data).hexdigest()


def entries(data):
    pos = 0
    result = []
    while True:
        start = pos
        if data[pos:pos + 6] != b"070701":
            raise ValueError(f"Expected newc header at {pos:#x}")
        fields = tuple(int(data[pos + 6 + i * 8:pos + 14 + i * 8], 16)
                       for i in range(13))
        size, namesize = fields[6], fields[11]
        name = data[pos + 110:pos + 110 + namesize - 1].decode()
        if data[pos + 110 + namesize - 1] != 0:
            raise ValueError("Missing name terminator")
        body_start = (pos + 110 + namesize + 3) & ~3
        body = data[body_start:body_start + size]
        if len(body) != size:
            raise ValueError("Truncated member")
        pos = (body_start + size + 3) & ~3
        result.append((name, fields, body, start))
        if name == "TRAILER!!!":
            return result


def member(name, body, ino):
    encoded = name.encode() + b"\0"
    fields = (ino, 0o100644, 0, 0, 1, 0, len(body), 0, 0, 0, 0,
              len(encoded), 0)
    data = b"070701" + b"".join(f"{v:08x}".encode() for v in fields) + encoded
    data += b"\0" * (-len(data) % 4)
    data += body
    return data + b"\0" * (-len(data) % 4)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--stock", type=Path, required=True)
    parser.add_argument("--policy", type=Path, required=True)
    parser.add_argument("--magiskboot", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    stock, policy = args.stock.read_bytes(), args.policy.read_bytes()
    if len(stock) != 8388608 or sha(stock) != STOCK_SHA:
        raise ValueError("Not the verified stock G13 V013 init_boot image")
    if sha(policy) != POLICY_SHA:
        raise ValueError("Not the verified live G13 V013 platform policy")
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    work = output / "work"
    check = output / "verification"
    work.mkdir()
    check.mkdir()
    tool = str(args.magiskboot.resolve())
    records = []

    def run(cwd, *parts):
        command = [tool, *map(str, parts)]
        result = subprocess.run(command, cwd=cwd, capture_output=True, text=True)
        records.append(dict(command=command, cwd=str(cwd),
                            returncode=result.returncode,
                            stdout=result.stdout, stderr=result.stderr))
        (output / "commands.json").write_text(json.dumps(records, indent=2) + "\n")
        if result.returncode:
            raise RuntimeError(result.stdout + result.stderr)

    run(work, "unpack", args.stock.resolve())
    original = (work / "ramdisk.cpio").read_bytes()
    before = entries(original)
    additions = {
        "force_debuggable": b"",
        "adb_debug.prop": b"ro.debuggable=1\nro.force.debuggable=1\n",
        "userdebug_plat_sepolicy.cil": policy + POLICY_ADDITION,
    }
    if set(additions) & {e[0] for e in before}:
        raise ValueError("Debug files already exist in the stock ramdisk")
    trailer = before[-1][3]
    ino = max(e[1][0] for e in before) + 1
    changed = original[:trailer]
    for index, (name, body) in enumerate(additions.items()):
        changed += member(name, body, ino + index)
    changed += original[trailer:]
    changed += b"\0" * (-len(changed) % 512)
    (work / "ramdisk.cpio").write_bytes(changed)
    image = output / "init_boot-adb-root.img"
    run(work, "repack", args.stock.resolve(), image)
    built = image.read_bytes()
    if len(built) != len(stock):
        raise ValueError("Repacked image has unexpected partition size")
    if stock[:12] != built[:12] or stock[16:4096] != built[16:4096]:
        raise ValueError("Unexpected boot-header changes")
    run(check, "unpack", image)
    after = entries((check / "ramdisk.cpio").read_bytes())
    original_members = [(name, fields, body) for name, fields, body, _ in before[:-1]]
    preserved = [(name, fields, body) for name, fields, body, _ in after[:-1]
                 if name not in additions]
    if original_members != preserved:
        raise ValueError("A stock ramdisk entry or its metadata changed")
    actual_additions = {name: body for name, _, body, _ in after if name in additions}
    if actual_additions != additions or len(after) != len(before) + len(additions):
        raise ValueError("Unexpected debug ramdisk entries")
    init = next(body for name, _, body, _ in before if name == "init")
    manifest = {
        "stock_image": str(args.stock.resolve()), "stock_sha256": sha(stock),
        "image": str(image), "image_sha256": sha(built), "image_size": len(built),
        "stock_init_sha256": sha(init), "stock_entries_preserved": True,
        "stock_entry_count": len(before) - 1,
        "original_ramdisk_size": struct.unpack_from("<I", stock, 12)[0],
        "modified_ramdisk_size": struct.unpack_from("<I", built, 12)[0],
        "added_files": {name: {"size": len(body), "sha256": sha(body)}
                        for name, body in additions.items()},
        "authentication": "Stock ro.adb.secure is not overridden",
        "selinux": "Only su is permissive; normal domains remain enforcing",
        "flash_performed": False,
    }
    (output / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(json.dumps(manifest, indent=2))


if __name__ == "__main__":
    main()
