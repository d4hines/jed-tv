#!/usr/bin/env python3
"""Unpack the pinned Android 14 MT5867 source, relocating publisher links."""
import hashlib
import json
import os
from pathlib import Path
import tarfile
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets/build/g13-kernel"
COMMIT = "3543dbbd43a7e69112b1169b34171d7bd745cdee"
SHA256 = "048a1cb80336c823b3a705dbd10a413091d635f30e5c83645f4b91f562e318a3"
ARCHIVE = ROOT / "assets/research/g13-kernel-source-20261101/oneplus-mt5867-android14-3543dbbd.tar.gz"
URL = f"https://codeload.github.com/techyminati/android_kernel_oneplus_mt5867/tar.gz/{COMMIT}"


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    if not ARCHIVE.exists():
        ARCHIVE.parent.mkdir(parents=True, exist_ok=True)
        urllib.request.urlretrieve(URL, ARCHIVE)
    assert hashlib.file_digest(ARCHIVE.open("rb"), "sha256").hexdigest() == SHA256
    src = OUT / "source"
    if src.exists():
        raise SystemExit(f"Preserving existing source: {src}")
    temp = OUT / "unpack"
    temp.mkdir()  # An interrupted extraction must be inspected, never reused silently.
    links = {}
    prefix = f"android_kernel_oneplus_mt5867-{COMMIT}"

    def relocate(member, destination):
        if member.issym() and member.linkname.startswith("/home/ubuntu/ack/"):
            target = prefix + "/" + member.linkname.removeprefix("/home/ubuntu/ack/")
            new_target = os.path.relpath(target, os.path.dirname(member.name))
            links[member.name.removeprefix(prefix + "/")] = [member.linkname, new_target]
            member = member.replace(linkname=new_target)
        return tarfile.data_filter(member, destination)

    with tarfile.open(ARCHIVE) as archive:
        archive.extractall(temp, filter=relocate)
    (temp / prefix).rename(src)
    temp.rmdir()
    (OUT / "source-provenance.json").write_text(json.dumps({
        "url": URL, "commit": COMMIT, "archive_sha256": SHA256,
        "relocated_links": links,
    }, indent=2) + "\n")
    print(src)


if __name__ == "__main__":
    main()
