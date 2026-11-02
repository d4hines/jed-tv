#!/usr/bin/env python3
"""Read the pinned stock backup and extract its eight EROFS filesystems offline."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import struct
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
BACKUP = ROOT / 'assets/backups/g13-v013-stock-20261031'
OUT = ROOT / 'assets/research/g13-firmware-trim-20261101'
PINS = {
    'super.img': 'efd11e24aa7c164c364da2e6cc86b7b76726772e5adf6458dc878efb4f02e55d',
    'linux_rootfs_a_a.img': 'bc81a2ab87e007d8e4d1b4e3ddedb6fdf964890a4c587c8ea14c9bfada8cff25',
    'tvservice_a.img': '72de36e49da9be1031d971a5fafcac348d16c452c19bba08ad6793131a59d00e',
}


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('fsck_erofs', help='Path to the pinned erofs-utils 1.8.10 fsck.erofs')
    args = parser.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    if shutil.disk_usage(OUT).free < 23 * 1024**3:
        raise SystemExit('Need 8 GiB working headroom above the 15 GiB free-space reserve')
    destination = OUT / 'v013-extracted'
    if destination.exists():
        raise SystemExit('Preserving existing extraction; inspect v013-extraction.json')
    for name, expected in PINS.items():
        assert sha(BACKUP / name) == expected, name
    destination.mkdir()
    report = {'source_sha256': PINS, 'fsck_erofs': args.fsck_erofs, 'filesystems': {}}

    def extract(name, image):
        target = destination / name
        target.mkdir()
        with (OUT / ('v013-' + name + '-extract.log')).open('w') as log:
            subprocess.run([args.fsck_erofs, '--extract=' + str(target), str(image)],
                           stdout=log, stderr=log, check=True)
        report['filesystems'][name] = {'image_bytes': image.stat().st_size, 'image_sha256': sha(image)}
        print('Extracted', name, flush=True)

    # AOSP system/core/fs_mgr/liblp/include/liblp/metadata_format.h,
    # android-14.0.0_r1. This intentionally handles only this pinned snapshot.
    with (BACKUP / 'super.img').open('rb') as source:
        source.seek(4096)
        geometry = source.read(4096)
        magic, size = struct.unpack_from('<II', geometry)
        assert magic == 0x616c4467 and size == 52
        assert hashlib.sha256(geometry[:8] + bytes(32) + geometry[40:size]).digest() == geometry[8:40]
        source.seek(12288)  # primary metadata, slot zero
        header = source.read(256)
        magic, major, minor, size = struct.unpack_from('<IHHI', header)
        assert magic == 0x414c5030 and major == 10 and size in (128, 256)
        assert hashlib.sha256(header[:12] + bytes(32) + header[44:size]).digest() == header[12:44]
        table_size = struct.unpack_from('<I', header, 44)[0]
        source.seek(12288 + size)
        tables = source.read(table_size)
        assert hashlib.sha256(tables).digest() == header[48:80]
        part_offset, part_count, part_size = struct.unpack_from('<III', header, 80)
        extent_offset, extent_count, extent_size = struct.unpack_from('<III', header, 92)
        assert part_size == 52 and extent_size == 24
        assert part_offset + part_count * part_size <= len(tables)
        assert extent_offset + extent_count * extent_size <= len(tables)
        wanted = {'system_a', 'system_ext_a', 'product_a', 'vendor_a', 'vendor_dlkm_a', 'system_dlkm_a'}
        found = set()
        for index in range(part_count):
            offset = part_offset + index * part_size
            name = tables[offset:offset+36].split(b'\0')[0].decode('ascii')
            if name not in wanted:
                continue
            first, count = struct.unpack_from('<II', tables, offset + 40)
            assert count and first + count <= extent_count
            with tempfile.NamedTemporaryFile(dir=OUT, suffix='.img') as temp:
                for i in range(first, first + count):
                    sectors, kind, start, device = struct.unpack_from('<QIQI', tables, extent_offset + i * extent_size)
                    assert kind == 0 and device == 0
                    assert (start + sectors) * 512 <= (BACKUP / 'super.img').stat().st_size
                    source.seek(start * 512)
                    remaining = sectors * 512
                    while remaining:
                        chunk = source.read(min(remaining, 1024**2))
                        assert chunk
                        temp.write(chunk)
                        remaining -= len(chunk)
                temp.flush()
                extract(name.removesuffix('_a'), Path(temp.name))
            found.add(name)
        assert found == wanted, wanted - found
    extract('linux_rootfs_a', BACKUP / 'linux_rootfs_a_a.img')
    extract('tvservice', BACKUP / 'tvservice_a.img')
    (OUT / 'v013-extraction.json').write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
