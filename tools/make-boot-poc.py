#!/usr/bin/env python3
"""Add one inert Android boot property to the exact G06 V109 boot image."""
import hashlib
import json
from pathlib import Path
import struct
import sys


def make_poc(original):
    expected = "d116dfa370d8e5dd5809f6f65266c86671b5e9bec9d62f95c0977bca0ff30085"
    if hashlib.sha256(original).hexdigest() != expected:
        raise ValueError("Input must be the exact original G06 V109 boot image")
    image = bytearray(original)
    if image[:8] != b"ANDROID!" or struct.unpack_from("<I", image, 40)[0] != 2:
        raise ValueError("Expected the original Android v2 boot header")
    cmdline = original[64:576].split(b"\0", 1)[0]
    marker = b"androidboot.codex_poc=1"
    new_cmdline = cmdline + b" " + marker
    if len(new_cmdline) >= 512 or original[608:1632].strip(b"\0"):
        raise ValueError("Unexpected boot command line layout")
    image[64:576] = new_cmdline.ljust(512, b"\0")

    magic, major, minor, payload_size, vbmeta_offset, vbmeta_size = struct.unpack_from(
        ">4sIIQQQ", original, len(original) - 64)
    if (magic, major, minor) != (b"AVBf", 1, 0):
        raise ValueError("Unexpected AVB footer")
    vbmeta = original[vbmeta_offset:vbmeta_offset + vbmeta_size]
    if vbmeta[:4] != b"AVB0" or struct.unpack_from(">I", vbmeta, 28)[0] != 0:
        raise ValueError("Embedded AVB metadata must use algorithm NONE")
    salt = bytes.fromhex("2e821884d9f0ff3adfe64ff0c90fbaa9ac772594db46a3e2e8f3755cb644c7d3")
    old_digest = hashlib.sha256(salt + original[:payload_size]).digest()
    if vbmeta.count(old_digest) != 1:
        raise ValueError("Cannot uniquely locate the original embedded boot hash")
    digest_offset = vbmeta_offset + vbmeta.index(old_digest)
    image[digest_offset:digest_offset + 32] = hashlib.sha256(salt + image[:payload_size]).digest()

    changed = [offset for offset, (before, after) in enumerate(zip(original, image)) if before != after]
    if len(original) != len(image) or not changed or any(
            not (64 <= offset < 576 or digest_offset <= offset < digest_offset + 32)
            for offset in changed):
        raise ValueError("Unexpected change outside command line and embedded hash")
    report = {
        "original_sha256": expected,
        "modified_sha256": hashlib.sha256(image).hexdigest(),
        "image_bytes": len(image),
        "changed_bytes": len(changed),
        "original_cmdline": cmdline.decode(),
        "modified_cmdline": new_cmdline.decode(),
        "embedded_hash_offset": digest_offset,
        "changes_limited_to": ["boot header command line", "unsigned embedded boot hash"],
        "kernel_and_ramdisk_byte_identical": True,
        "expected_property_if_bootloader_passes_cmdline": "ro.boot.codex_poc=1",
        "other_partitions_modified": False,
    }
    return bytes(image), report


if __name__ == "__main__":
    if len(sys.argv) != 3:
        sys.exit("Usage: make-boot-poc.py ORIGINAL.img OUTPUT.img")
    image, report = make_poc(Path(sys.argv[1]).read_bytes())
    with Path(sys.argv[2]).open("xb") as output:
        output.write(image)
    print(json.dumps(report, indent=2))
