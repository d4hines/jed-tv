"""Inspect the discovered stock V122 ZIP via bounded HTTP Range reads.

This accesses the laptop's internet connection only. No TV writes occur.
"""
import io
import json
from pathlib import Path
import urllib.request
import xml.etree.ElementTree as ET
import zipfile

base = Path(__file__).resolve().parents[1] / 'assets/research'
reply = ET.fromstring(json.loads((base / "firmware-query-result.json").read_text())["response"])
url = reply.findtext("./upgrade/fileurl")
total_size = int(reply.findtext("./upgrade/size"))
version = reply.findtext("./upgrade/version")
if version != "V8-T221T04-LF1V122":
    raise SystemExit("Unexpected firmware version; inspect metadata first")
destination = base / "firmware" / version
destination.mkdir(parents=True, exist_ok=True)

class RemoteZip(io.RawIOBase):
    def __init__(self):
        self.position = 0
        self.requests = []

    def seekable(self):
        return True

    def tell(self):
        return self.position

    def seek(self, offset, whence=0):
        self.position = offset if whence == 0 else self.position + offset if whence == 1 else total_size + offset
        return self.position

    def read(self, length=-1):
        if length < 0:
            length = total_size - self.position
        if length == 0:
            return b""
        if length > 80 * 1024 * 1024:
            raise ValueError(f"Refusing oversized range read: {length}")
        start, end = self.position, min(self.position + length, total_size) - 1
        request = urllib.request.Request(url, headers={"Range": f"bytes={start}-{end}"})
        with urllib.request.urlopen(request, timeout=30) as response:
            if response.status != 206:
                raise ValueError(f"Server ignored Range: HTTP {response.status}")
            data = response.read(length + 1)
            if len(data) != end - start + 1:
                raise ValueError("Unexpected response size")
        self.position += len(data)
        self.requests.append({"start": start, "end": end, "bytes": len(data)})
        return data

remote = RemoteZip()
with zipfile.ZipFile(remote) as archive:
    entries = [{"name": info.filename, "size": info.file_size, "compressed": info.compress_size, "crc32": f"{info.CRC:08x}"} for info in archive.infolist()]
    (destination / "contents.json").write_text(json.dumps(entries, indent=2) + "\n")
    for entry in entries:
        print(f"{entry['size']:12} {entry['compressed']:12} {entry['name']}")
    selected = [info for info in archive.infolist() if info.filename in {
        "META-INF/com/android/metadata", "META-INF/com/google/android/updater-script",
        "boot.img", "vbmeta.img", "vbmeta_system.img", "recovery.img", "dtbo.img",
        "mboot.bin", "upgrade.img",
    }]
    for info in selected:
        if info.file_size > 256 * 1024 * 1024:
            raise ValueError("Selected image unexpectedly large")
        target = destination / info.filename
        if target.exists() and target.stat().st_size == info.file_size:
            continue
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(archive.read(info))
        print("EXTRACTED", info.filename, info.file_size)
(destination / "range-reads.json").write_text(json.dumps(remote.requests, indent=2) + "\n")
print("Total downloaded bytes:", sum(item["bytes"] for item in remote.requests))
