#!/usr/bin/env python3
"""Freeze the locally built silent Doom binary and exact V109 display modules."""
import gzip
import hashlib
import io
import json
import lzma
from pathlib import Path
import re
import struct
import subprocess
import sys
import tarfile

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets/build/doom-minimal"
MODULES = "kdrv_xc utpa2k hdcp1x hdcp2x xcker iniparser mwgifker misck mik mstar_fbdev_mi".split()


def digest(data):
    return hashlib.sha256(data).hexdigest()


def single_map(wad, sound=False):
    if digest(wad) != "7323bcc168c5a45ff10749b339960e98314740a734c30d4b9f3337001f9e703d":
        raise ValueError("Unexpected Freedoom 0.13.0 input")
    magic, count, directory = struct.unpack_from("<4sII", wad)
    lumps, skip = [], False
    map_lumps = "THINGS LINEDEFS SIDEDEFS VERTEXES SEGS SSECTORS NODES SECTORS REJECT BLOCKMAP BEHAVIOR".split()
    for i in range(count):
        offset, size, name = struct.unpack_from("<II8s", wad, directory + i * 16)
        name = name.rstrip(b"\0").decode()
        if re.fullmatch(r"E\dM\d", name):
            skip = name != "E1M1"
        elif name not in map_lumps:
            skip = False
        if not skip and not name.startswith(("DEMO",) if sound else ("DS", "D_", "DEMO")):
            lumps.append((name, wad[offset:offset + size]))
    payload = b"".join(data for _, data in lumps)
    result = bytearray(struct.pack("<4sII", magic, len(lumps), 12 + len(payload)) + payload)
    offset = 12
    for name, data in lumps:
        result.extend(struct.pack("<II8s", offset, len(data), name.encode()))
        offset += len(data)
    return bytes(result)


def main():
    debugfs = sys.argv[1]
    OUT.mkdir(parents=True, exist_ok=True)
    stock = ROOT / "assets/backups/v109-stock-20261031"
    manifest = json.loads((stock / "manifest.json").read_text())
    super_meta = next(f for f in manifest["files"] if f["name"] == "super")
    if super_meta["status"] != "verified" or not super_meta["source_matches"]:
        raise ValueError("Unverified super backup")
    with (stock / "super.img").open("rb") as source:
        if hashlib.file_digest(source, "sha256").hexdigest() != super_meta["sha256"]:
            raise ValueError("Super backup checksum failed")
        source.seek(1433600 * 512)
        vendor = source.read(682632 * 512)
    vendor_path = OUT / "vendor.img"
    vendor_path.write_bytes(vendor)
    files = {"licenses/FREEDOOM.txt": (ROOT / "assets/third_party/freedoom/extract/freedoom-0.13.0/COPYING.txt").read_bytes()}
    for module in MODULES:
        destination = OUT / (module + ".ko")
        subprocess.run([debugfs, "-R", f"dump /lib/modules/{module}.ko {destination}", str(vendor_path)], check=True)
        content = destination.read_bytes()
        if b"vermagic=4.19.116+ " not in content:
            raise ValueError("Module does not match the original kernel: " + module)
        files["modules/" + module + ".ko"] = content
    files["modules/order"] = ("\n".join(m + ".ko" for m in MODULES) + "\n").encode()
    files["doom/freedoom1.wad"] = single_map((ROOT / "assets/third_party/freedoom/freedoom1.wad").read_bytes())
    ir_path = OUT / "ir_config.ini"
    config_meta = next(f for f in manifest["files"] if f["name"] == "tvconfig")
    with (stock / "tvconfig.img").open("rb") as source:
        if hashlib.file_digest(source, "sha256").hexdigest() != config_meta["sha256"]:
            raise ValueError("TV configuration backup checksum failed")
    subprocess.run([debugfs, "-R", f"dump /config/ir_config.ini {ir_path}", str(stock / "tvconfig.img")], check=True)
    if not ir_path.is_file() or not ir_path.stat().st_size:
        raise ValueError("Missing stock IR configuration")
    files["vendor/tvconfig/config/ir_config.ini"] = ir_path.read_bytes()
    data = bytearray(b"DOOMDATA" + struct.pack("<I", len(files)))
    for name, content in sorted(files.items()):
        data.extend(struct.pack("<II", len(name.encode()), len(content)))
        data.extend(name.encode())
        data.extend(content)
    packed = lzma.compress(bytes(data), check=lzma.CHECK_CRC32, preset=6)
    binary = (OUT / "target/armv7-unknown-linux-musleabihf/release/doomtv").read_bytes()
    if binary[:4] != b"\x7fELF" or binary[4] != 1 or struct.unpack_from("<H", binary, 18)[0] != 40:
        raise ValueError("Expected a 32-bit ARM ELF")
    phoff = struct.unpack_from("<I", binary, 28)[0]
    phsize, phnum = struct.unpack_from("<HH", binary, 42)
    if any(struct.unpack_from("<I", binary, phoff + i * phsize)[0] == 3 for i in range(phnum)):
        raise ValueError("Doom binary must be static (no ELF interpreter)")
    report = {
        "doomgeneric_commit": "e257a09527062bfb986b6e5e0780839b6cbd5b71",
        "freedoom_version": "0.13.0", "map": "E1M1", "sound": False,
        "userspace_executables": ["/init"], "pid1": "doomtv",
        "init_sha256": digest(binary), "init_bytes": len(binary),
        "compressed_data_bytes": len(packed), "data_sha256": digest(packed),
        "source_super_sha256": super_meta["sha256"], "vendor_sha256": digest(vendor),
        "source_files": {str(path.relative_to(ROOT)): digest(path.read_bytes())
                         for path in sorted((ROOT / "src/legacy-doom").rglob("*"))
                         if path.is_file() and path.suffix in (".rs", ".c", ".h", ".toml", ".lock", ".sh")},
        "files": {name: {"bytes": len(content), "sha256": digest(content)} for name, content in sorted(files.items())},
        "hardware_verified": False,
    }
    runtime = OUT / "g06-doom-runtime.tar"
    with tarfile.open(runtime, "w", format=tarfile.USTAR_FORMAT) as archive:
        for name, content in [("init", binary), ("doom-data.xz", packed),
                              ("manifest.json", (json.dumps(report, indent=2) + "\n").encode())]:
            info = tarfile.TarInfo(name)
            info.size, info.mode = len(content), 0o755 if name == "init" else 0o644
            archive.addfile(info, io.BytesIO(content))
    pin = {"name": runtime.name, "sha256": digest(runtime.read_bytes())}
    (ROOT / "tools/legacy-doom/runtime.json").write_text(json.dumps(pin, indent=2) + "\n")
    print(json.dumps({**pin, "path": str(runtime), "init_bytes": len(binary), "compressed_data_bytes": len(packed)}, indent=2))


if __name__ == "__main__":
    main()
