#!/usr/bin/env python3
"""Verify prepared G06 ramdisk content against its audited source manifest.
Usage: python3 tools/check_tv_boot.py m1a|m1b|m1c. No device access.
"""
import gzip
import hashlib
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]


def digest(data):
    return hashlib.sha256(data).hexdigest()


def cpio(data):
    files = {}
    i = 0
    while i + 110 <= len(data):
        if data[i:i + 6] != b'070701':
            raise ValueError('invalid newc archive')
        h = [int(data[i + 6 + j * 8:i + 14 + j * 8], 16) for j in range(13)]
        start = i + 110
        if not h[11] or start + h[11] > len(data) or data[start + h[11] - 1] != 0:
            raise ValueError('invalid cpio name')
        name = data[start:start + h[11] - 1].decode()
        i = (start + h[11] + 3) & ~3
        end = i + h[6]
        if end > len(data):
            raise ValueError('truncated cpio content')
        if name == 'TRAILER!!!':
            return files
        if pathlib.PurePosixPath(name).is_absolute() or '..' in pathlib.PurePosixPath(name).parts:
            raise ValueError('unsafe cpio path')
        if h[1] & 0o170000 == 0o100000:
            key = '/' + name
            if key in files:
                raise ValueError('duplicate cpio file: ' + key)
            files[key] = data[i:end]
        i = (end + 3) & ~3
    raise ValueError('missing cpio trailer')


def verify(stage):
    if stage not in ('m1a', 'm1b', 'm1c'):
        raise ValueError('expected m1a, m1b or m1c')
    out = ROOT / 'assets/build/tv-boot' / stage
    m = json.loads((out / 'manifest.json').read_text())
    if digest((out / 'boot.img').read_bytes()) != m['boot_img_sha256']:
        raise ValueError('boot image hash mismatch')
    compressed = (out / 'ramdisk.cpio.gz').read_bytes()
    if len(compressed) != m['ramdisk_gz_bytes']:
        raise ValueError('ramdisk size mismatch')
    files = cpio(gzip.decompress(compressed))
    if digest(files['/init']) != m['tvinit_sha256']:
        raise ValueError('init hash mismatch')
    if b'g06-compat: ARM32' in files['/init']:
        raise ValueError('QEMU-only compatibility bootstrap leaked into TV image')
    conf = dict(line.split('=', 1) for line in files['/etc/tvinit.conf'].decode().splitlines()
                if line and not line.startswith('#'))
    if conf != m['tvinit_conf']:
        raise ValueError('boot configuration mismatch')
    count = 0
    if m['modules']:
        kit = ROOT / 'assets/build/g06-platform/manifest.json'
        if digest(kit.read_bytes()) != m['platform_kit_manifest_sha256']:
            raise ValueError('source platform manifest changed; rebuild the image')
        fw = json.loads(kit.read_text())
        for guest, record in fw['files'].items():
            if guest.startswith(('/vendor/tvconfig/', '/tclconfig/')):
                if digest(files[guest]) != record['sha256']:
                    raise ValueError('configuration content mismatch: ' + guest)
                count += 1
        expected = []
        for name in m['modules']:
            path = '/lib/modules/' + name + '.ko'
            expected.append(path)
            source = '/vendor/lib/modules/' + name + '.ko'
            if digest(files[path]) != fw['files'][source]['sha256']:
                raise ValueError('module content mismatch: ' + name)
        loaded = [line for line in files['/etc/modules.list'].decode().splitlines()
                  if line and not line.startswith('#')]
        if loaded != expected:
            raise ValueError('module load order mismatch')
    if any('/tvservice/' in name or name.endswith('/midaemon') for name in files):
        raise ValueError('analysis-only vendor userspace leaked into minimal TV image')
    if m['hardware_ready']:
        raise ValueError('offline packaging cannot establish hardware readiness')
    print(f'{stage}: boot/init/config/module hashes and load order verified; '
          f'{count} exact configuration files; no vendor daemon or QEMU bootstrap; hardware unverified')


if __name__ == '__main__':
    verify(sys.argv[1])
