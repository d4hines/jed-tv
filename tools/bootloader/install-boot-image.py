#!/usr/bin/env python3
"""Install the hardware-validated default selector on this exact G13 only.

Without --apply, check the device, local candidate and RAM-trial evidence.
Restores the temporary upgrade staging area, initializes the two journal
sectors, then writes only boot_a. Does not reboot or change the active slot.
"""
import argparse
import hashlib
import json
from pathlib import Path
import struct
import sys
import zlib

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT/'tools/native-app'))
import g13_device as d

STOCK = 'dc3154fd0767fcb6a40080798b154ada3cd38c3887f444b20eaf80d8cf27ab43'
PINNED = {
    'boot_a': STOCK, 'boot_b': STOCK,
    'init_boot_a': '0ebb78f7eb550f8b2f524cf661342baeeaa55a70e3c3e5708d71f26381fc5e8b',
    'vendor_boot_a': '8bea2a7c9a392ae73cf9269f2c530aa9f50bf36a521437b69a5e89430a07d458',
    'MBOOT': 'a342fcdd62b2ce7121d977b9410d2c05aa51757bf7d26d6b445423e97122439b',
    'MBOOT_b': '643058879b84d044dea58a4572bd9ade1cc3140f3d3357eeaabae53be32ec382',
}


def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()


def upload(path, remote):
    d.adb('push', path, remote, capture_output=True, timeout=90)
    assert d.shell('sha256sum '+remote).split()[0] == sha(path)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args()
    evidence = ROOT/'assets/research/g13-leanboot-default'
    build = ROOT/'assets/build/leanboot'
    report = json.loads((build/'boot-image.json').read_text())
    trial = json.loads((evidence/'compact-lean-dma/result.json').read_text())
    candidate = build/'g13-boot_a.img'
    digest = sha(candidate)
    assert digest == report['image_sha256'] == trial['image_sha256']
    assert trial['ready'] and trial['route'] == 'lean', 'Successful matching RAM trial required'
    before = json.loads((evidence/'before/manifest.json').read_text())
    stock = evidence/'before/boot_a.img'
    upgrade = evidence/'before/upgrade.img'
    assert sha(stock) == STOCK and sha(upgrade) == before['upgrade']['sha256']
    a, b = stock.read_bytes(), candidate.read_bytes()
    assert len(a) == len(b) == 55574528
    assert a[:4096] == b[:4096] and a[15876096:] == b[15876096:]
    assert a[0x21000:] == b[0x21000:], 'Only the kernel prefix may change'
    d.connect(); d.root()
    assert d.shell('getprop ro.boot.slot_suffix').strip() == '_a'
    for name, expected in PINNED.items():
        actual = d.shell('sha256sum /dev/block/by-name/'+name).split()[0]
        assert actual == expected, (name, actual, expected)
    assert d.shell('blockdev --getsize64 /dev/block/by-name/boot_a').strip() == '55574528'
    assert d.shell('blockdev --getsize64 /dev/block/by-name/upgrade').strip() == '52428800'
    print('PASS: exact target, preserved firmware, candidate and successful RAM trial')
    if not args.apply: return
    out = evidence/'installed'; out.mkdir(parents=True, exist_ok=True)
    # Keep an on-device stock copy until post-install verification is finished.
    upload(stock, '/data/local/tmp/leanboot-stock-boot.img')
    upload(upgrade, '/data/local/tmp/leanboot-stock-upgrade.img')
    upload(candidate, '/data/local/tmp/leanboot-candidate.img')
    d.shell('dd if=/data/local/tmp/leanboot-stock-upgrade.img of=/dev/block/by-name/upgrade bs=1048576 conv=fsync 2>/dev/null; sync; blockdev --flushbufs /dev/block/by-name/upgrade')
    assert d.shell('sha256sum /dev/block/by-name/upgrade').split()[0] == sha(upgrade)
    record = bytearray(512)
    struct.pack_into('<6I', record, 0, 0x314a424c, 1, 0, 0, 0xffffffff, 0xffffffff)
    struct.pack_into('<I', record, 508, zlib.crc32(record[:508]))
    journal = out/'journal-initial.bin'; journal.write_bytes(bytes(record)*2)
    upload(journal, '/data/local/tmp/leanboot-journal.bin')
    d.shell('dd if=/data/local/tmp/leanboot-journal.bin of=/dev/block/by-name/upgrade bs=512 seek=102398 count=2 conv=fsync 2>/dev/null; sync; blockdev --flushbufs /dev/block/by-name/upgrade')
    readback = d.adb('exec-out', 'dd if=/dev/block/by-name/upgrade bs=512 skip=102398 count=2 2>/dev/null', capture_output=True).stdout
    assert readback == journal.read_bytes()
    d.shell('dd if=/data/local/tmp/leanboot-candidate.img of=/dev/block/by-name/boot_a bs=4096 skip=1 seek=1 count=32 conv=fsync 2>/dev/null; sync; blockdev --flushbufs /dev/block/by-name/boot_a')
    actual = d.shell('sha256sum /dev/block/by-name/boot_a').split()[0]
    if actual != digest:
        d.shell('dd if=/data/local/tmp/leanboot-stock-boot.img of=/dev/block/by-name/boot_a bs=1048576 conv=fsync 2>/dev/null; sync; blockdev --flushbufs /dev/block/by-name/boot_a')
        restored = d.shell('sha256sum /dev/block/by-name/boot_a').split()[0]
        raise RuntimeError(f'Candidate readback mismatch; stock restore hash={restored}')
    result = {'boot_a_sha256': actual, 'journal_initialized': True,
              'upgrade_staging_restored': True, 'rebooted': False, 'slot': '_a',
              'boot_a_written_offset': 4096, 'boot_a_written_bytes': 131072}
    (out/'install.json').write_text(json.dumps(result, indent=2)+'\n')
    print(json.dumps(result, indent=2))


if __name__ == '__main__': main()
