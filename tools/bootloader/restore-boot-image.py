#!/usr/bin/env python3
"""G13 UART rollback: copy verified stock boot_b bytes to boot_a, without switching slots.

Requires the existing Screen session stopped at MBOOT. Default is read-only:
load/CRC-check boot_b in RAM. --apply restores only boot_a and verifies its
readback. Does not reboot, erase, change slots or touch either MBOOT copy.
"""
import argparse
import hashlib
from pathlib import Path
import struct
import tv_session as t


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--apply', action='store_true')
    args = parser.parse_args()
    stock = t.ROOT/'assets/research/g13-leanboot-default/before/boot_a.img'
    blob = stock.read_bytes()
    assert len(blob) == 0x3500000
    assert hashlib.sha256(blob).hexdigest() == 'dc3154fd0767fcb6a40080798b154ada3cd38c3887f444b20eaf80d8cf27ab43'
    t.OUT = t.ROOT/'assets/research/g13-leanboot-default/uart-rollback'
    t.OUT.mkdir(parents=True, exist_ok=True)
    with t.Session() as s:
        s.guard()
        # Validate every 1 MiB translation section covering the scratch copy.
        entries = struct.unpack('<53I', s.read(0x297f1000, 53*4))
        assert all(v & 0xfff00003 == (0x40000000+i*0x100000) | 2 for i,v in enumerate(entries))
        print(s.ask('mmc read.p 40000000 boot_b 3500000', timeout=30))
        s.crc(0x40000000, blob)
        print('PASS: boot_b contains the exact original boot image; slot is unchanged')
        if not args.apply: return
        print(s.ask('mmc write.p 40000000 boot_a 3500000 0', timeout=30))
        print(s.ask('mmc read.p 40000000 boot_a 3500000', timeout=30))
        s.crc(0x40000000, blob)
        print('Restored and readback-verified boot_a. Use normal reset when ready.')


if __name__ == '__main__': main()
