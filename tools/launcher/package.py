#!/usr/bin/env python3
"""Pin the source-built production launcher without any firmware binaries."""
import hashlib
import io
import json
from pathlib import Path
import struct
import sys
import tarfile

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets/build/launcher"


def main():
    build_id = sys.argv[1]
    assert len(build_id) == 64 and all(c in "0123456789abcdef" for c in build_id)
    binary = (OUT / "target-prod/armv7-unknown-linux-musleabihf/release/tv-launcher").read_bytes()
    assert binary[:7] == b"\x7fELF\x01\x01\x01" and struct.unpack_from("<H", binary, 18)[0] == 40
    assert b"TV-LAUNCHER-PROFILE:G06-V109" in binary and b"TV-LAUNCHER-PROFILE:QEMU-TEST-ONLY" not in binary
    assert b"mmcblk0p29" in binary and build_id.encode() in binary
    request = f"TV-LAUNCHER/1\nG06\nV109\n4C0490290FA05A0C0\nhello\n{build_id}\n"
    report = {"build_id": build_id, "binary_sha256": hashlib.sha256(binary).hexdigest(), "binary_bytes": len(binary),
              "request_body": request, "boot_marker": build_id, "profile": "G06-V109-production",
              "cache_partition": "mmcblk0p29", "cache_start_sector": 9019392, "cache_sectors": 204800,
              "hardware_boot_verified": False}
    archive = OUT / "g06-launcher-runtime.tar"
    with tarfile.open(archive, "w", format=tarfile.USTAR_FORMAT) as tar:
        for name, content, mode in (("launcher", binary, 0o750), ("manifest.json", (json.dumps(report, indent=2) + "\n").encode(), 0o644)):
            entry = tarfile.TarInfo(name)
            entry.size, entry.mode = len(content), mode
            tar.addfile(entry, io.BytesIO(content))
    (OUT / "production-launcher").write_bytes(binary)
    pin = {"name": archive.name, "sha256": hashlib.sha256(archive.read_bytes()).hexdigest()}
    (ROOT / "tools/launcher/runtime.json").write_text(json.dumps(pin, indent=2) + "\n")
    print(json.dumps({**pin, **report}, indent=2))


if __name__ == "__main__":
    main()
