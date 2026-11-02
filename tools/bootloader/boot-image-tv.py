#!/usr/bin/env python3
"""Run the compact boot image from RAM at an existing, verified MBOOT prompt.

The original boot_a supplies the unchanged kernel body; --refresh-prefix
uploads only the modified prefix through compressed UART. No firmware flash, reboot,
environment change or serial ownership change is issued by this tool. The
running selector may advance its two-sector journal if it was initialized.
"""
import argparse
import hashlib
import json
from pathlib import Path
import struct
import subprocess
import time
from elftools.elf.elffile import ELFFile
import tv_session as t


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', type=Path, required=True)
    parser.add_argument('--build', type=Path, default=t.ROOT/'assets/build/leanboot')
    parser.add_argument('--refresh-prefix', action='store_true',
                        help='refresh code/header in RAM through compressed UART transfer before checking the whole image')
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    t.OUT = args.out
    build = args.build
    blob = (build/'g13-boot-ram.img').read_bytes()
    assert len(blob) == 15876096
    assert struct.unpack_from('<I', blob, 8)[0] == 15871916
    with (build/'entry.o').open('rb') as f:
        patch = ELFFile(f).get_section_by_name('.text.leanboot_image_probe_redirect').data()
    assert patch == struct.pack('<I', 0xe3a03101)  # MOV r3,#0x40000000, emitted by the DSL.
    with t.Session() as s:
        s.guard()
        print(s.ask('readKL', timeout=60), flush=True)
        # The source is disjoint from the destination; MBOOT rejects overlap.
        descriptors = struct.unpack('<16I', s.read(0x297f1000, 64))
        assert all(value & 0xfff00003 == (0x40000000 + i*0x100000) | 2
                   for i, value in enumerate(descriptors)), descriptors
        s.sync()
        print(s.ask('mmc read.p 40000000 boot_a f24000', timeout=30), flush=True)
        if args.refresh_prefix:
            for offset, size in ((0, 65536), (65536, 65536), (131072, 4096)):
                s.upload(0x26200000, blob[offset:offset+size])
                s.ask(f'cp.b 26200000 {0x40000000+offset:x} {size:x}')
            s.sync()
        s.crc(0x40000000, blob)
        assert s.read(0x28d56e50, 4) == bytes.fromhex('18301be5')
        s.ask(f'mw.l 28d56e50 {struct.unpack("<I", patch)[0]:08x} 1')
        s.sync()
        start = t.LOG.stat().st_size
        began = time.monotonic()
        subprocess.run([t.SCREEN, '-S', t.SESSION, '-p', '0', '-X', 'stuff', 'bootKL\r'], check=True)
        data = b''
        while time.monotonic()-began < 55:
            with t.LOG.open('rb') as f:
                f.seek(start); data = f.read()
            if b'DANDE_BOOT' in data and b'READY' in data: break
            if t.PROMPT in data: break
            time.sleep(.1)
        (args.out/'boot.bin').write_bytes(data)
        ready = b'DANDE_BOOT' in data and b'READY' in data
        route = 'lean' if b'LEANBOOT OS prep' in data and b'LEANBOOT Linux' in data else 'stock' if b'LEANBOOT stock' in data else 'unknown'
        result = {'image_sha256': hashlib.sha256((build/'g13-boot_a.img').read_bytes()).hexdigest(),
                  'ram_image_sha256': hashlib.sha256(blob).hexdigest(), 'route': route,
                  'ready': ready, 'seconds': round(time.monotonic()-began, 2),
                  'active_boot_partition_changed': False}
        (args.out/'result.json').write_text(json.dumps(result, indent=2)+'\n')
        print(data[-5000:].decode(errors='replace'))
        print(json.dumps(result, indent=2))
        assert ready and route != 'unknown', 'No successful selector boot; inspect UART before any retry'


if __name__ == '__main__': main()
