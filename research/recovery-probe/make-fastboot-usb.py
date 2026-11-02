#!/usr/bin/env python3
"""Make a local FAT32 recovery-trigger image. Never opens a physical device."""
import hashlib
import io
import json
from pathlib import Path
import struct
import subprocess
import sys
import zipfile


def main():
    image = Path(sys.argv[1])
    if image.suffix != ".img" or image.exists():
        raise ValueError("Choose a new .img regular file; existing paths/devices are refused")
    image.parent.mkdir(parents=True, exist_ok=True)
    size, start = 64 * 1024 * 1024, 2048
    sectors = size // 512 - start
    mbr = bytearray(512)
    mbr[446:462] = struct.pack("<B3sB3sII", 0, b"\xfe\xff\xff", 0x0c, b"\xfe\xff\xff", start, sectors)
    mbr[510:] = b"\x55\xaa"
    with image.open("xb") as stream:
        stream.truncate(size)
        stream.write(mbr)
    spec = f"{image}@@{start * 512}"
    subprocess.run(["mformat", "-i", spec, "-F", "-T", str(sectors), "-c", "1",
                    "-v", "TVRECOVERY", "-N", "6ecb21f4", "::"], check=True)
    empty = io.BytesIO()
    with zipfile.ZipFile(empty, "w"):
        pass
    data = empty.getvalue()
    expected = "8739c76e681f900923b900c9df0ef75cf421d39cabb54650c4b9ad19b6a76d85"
    if len(data) != 22 or hashlib.sha256(data).hexdigest() != expected:
        raise ValueError("Empty ZIP does not match the previously tested recovery trigger")
    for name in ("update_signed.zip", "V8-T221T04-LF1V109.000732.zip"):
        subprocess.run(["mcopy", "-i", spec, "-", "::" + name], input=data, check=True)
    label = b"\n--fast\n   "
    with image.open("r+b") as stream:
        offset = start * 512
        stream.seek(offset)
        boot = stream.read(512)
        if boot[82:90] != b"FAT32   " or boot[71:82] != b"TVRECOVERY ":
            raise ValueError("Unexpected FAT32 boot sector")
        reserved = struct.unpack_from("<H", boot, 14)[0]
        fats, spc = boot[16], boot[13]
        fatsz = struct.unpack_from("<I", boot, 36)[0]
        root = struct.unpack_from("<I", boot, 44)[0]
        backup = struct.unpack_from("<H", boot, 50)[0]
        root_offset = offset + (reserved + fats * fatsz + (root - 2) * spc) * 512
        stream.seek(root_offset)
        entries = stream.read(spc * 512)
        labels = [p for p in range(0, len(entries), 32)
                  if entries[p] not in (0, 0xe5) and entries[p + 11] == 8]
        if len(labels) != 1 or entries[labels[0]:labels[0] + 11] != b"TVRECOVERY ":
            raise ValueError("Unexpected root volume-label entry")
        for where in (offset + 71, offset + backup * 512 + 71, root_offset + labels[0]):
            stream.seek(where)
            if stream.read(11) != b"TVRECOVERY ":
                raise ValueError("Unexpected label before patch")
            stream.seek(where)
            stream.write(label)
    for name in ("update_signed.zip", "V8-T221T04-LF1V109.000732.zip"):
        readback = subprocess.check_output(["mtype", "-i", spec, "::" + name])
        if readback != data:
            raise ValueError("ZIP filesystem readback failed")
    report = {"image": str(image), "bytes": size,
              "sha256": hashlib.sha256(image.read_bytes()).hexdigest(),
              "label_hex": label.hex(), "empty_zip_sha256": expected,
              "purpose": "Original G06 V109 recovery/Fastboot menu trigger; no firmware payload",
              "hardware_validated": False, "physical_devices_written": False}
    image.with_suffix(".json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
