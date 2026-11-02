#!/usr/bin/env python3
"""Create a signed temporary bootloader or persistent app image; no TV access."""
import argparse
import hashlib
from pathlib import Path
import struct
import subprocess
import tempfile

MAGIC = b'LBOOTUP1'
BOARD = 0x00130001
MAX_PAYLOAD = 16 * 1024 * 1024


def package(image, key, version, entry=0, board=BOARD, component=1):
    if not 0 < len(image) <= MAX_PAYLOAD or not 0 <= entry < len(image) or entry % 4:
        raise ValueError('Image length or ARM entry offset is invalid')
    if not 0 < version <= 0xffffffff:
        raise ValueError('Version must be 1..4294967295; wrapping is not permitted')
    public = subprocess.check_output(['openssl','pkey','-in',str(key),'-pubout','-outform','DER'])
    if len(public) != 44 or public[:12] != bytes.fromhex('302a300506032b6570032100'):
        raise ValueError('The signing key must be Ed25519')
    header = MAGIC + struct.pack('<8I', 1, board, component, version, len(image), entry, 0, 1)
    header += hashlib.sha256(image).digest() + bytes(24)
    assert len(header) == 96
    with tempfile.TemporaryDirectory() as directory:
        message=Path(directory)/'manifest';message.write_bytes(header)
        signature=subprocess.check_output(['openssl','pkeyutl','-sign','-rawin','-inkey',str(key),'-in',str(message)])
    if len(signature) != 64:
        raise ValueError('Expected a 64-byte Ed25519 signature; check the key type')
    return header + signature + image


if __name__ == '__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('image',type=Path);p.add_argument('output',type=Path)
    p.add_argument('--key',type=Path,required=True,help='private Ed25519 PEM key (host only)')
    p.add_argument('--version',type=int,required=True)
    p.add_argument('--entry',type=lambda s:int(s,0),default=0,help='ARM entry offset within image')
    p.add_argument('--component',choices=['bootloader','apps'],default='bootloader')
    a=p.parse_args()
    if a.component=='apps' and a.entry:
        p.error('App images must have entry offset zero')
    a.output.write_bytes(package(a.image.read_bytes(),a.key,a.version,a.entry,
                                component=2 if a.component=='apps' else 1))
    print(f'Wrote {a.output} ({a.output.stat().st_size} bytes)')
