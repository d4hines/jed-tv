#!/usr/bin/env python3
"""Read-only Lean eMMC trial at an existing exact-G13 MBOOT prompt."""
import hashlib
import json
import argparse
from pathlib import Path
import struct
import zlib
from elftools.elf.elffile import ELFFile
import tv_session as t


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--advance', action='store_true', help='advance and verify the two-sector journal once')
    args = parser.parse_args()
    t.OUT = t.ROOT/'assets/research/g13-leanboot-default/bounded-io'
    t.OUT.mkdir(parents=True, exist_ok=True)
    code = (t.ROOT/'assets/build/leanboot/g13-bootstrap-probe.bin').read_bytes()
    with t.Session() as s:
        s.guard()
        s.sync()
        print(s.ask('mmc read 26200000 3217fe 200'), flush=True)
        reference = s.read(0x26200000, 512)
        (t.OUT/'reference.bin').write_bytes(reference)
        s.upload(0x26000000, code)
        s.sync()
        reply = s.ask('go 26000000', timeout=20)
        print(reply, flush=True)
        status = struct.unpack('<I', s.read(0x26200100, 4))[0]
        diagnostic = struct.unpack('<3I', s.read(0x26350f00, 12))
        actual = s.read(0x26350000, 512)
        (t.OUT/'actual.bin').write_bytes(actual)
        result = {'status': status, 'diagnostic': diagnostic,
                  'matches': actual == reference,
                  'probe_sha256': hashlib.sha256(code).hexdigest()}
        (t.OUT/'read-test.json').write_text(json.dumps(result, indent=2)+'\n')
        print(json.dumps(result, indent=2))
        assert status == 0 and actual == reference
        if args.advance:
            def records():
                s.ask('mmc read 26200000 3217fe 400')
                raw = s.read(0x26200000, 1024)
                decoded = []
                for offset in (0, 512):
                    block = raw[offset:offset+512]
                    magic, version, seq, state, inverse_seq, inverse_state = struct.unpack_from('<6I', block)
                    assert magic == 0x314a424c and version == 1
                    assert seq ^ inverse_seq == state ^ inverse_state == 0xffffffff
                    assert zlib.crc32(block[:508]) == struct.unpack_from('<I', block, 508)[0]
                    decoded.append((seq, state))
                return raw, decoded
            before, states = records()
            assert states[0] == states[1] == (0, 0), states
            build = t.ROOT/'assets/build/leanboot'
            s.upload(0x26000000, (build/'g13-loader-core.bin').read_bytes())
            with (build/'g13-loader-core.elf').open('rb') as f:
                entry = next(x['st_value'] for x in ELFFile(f).get_section_by_name('.symtab').iter_symbols()
                             if x.name == 'leanboot_journal_advance')
            s.sync()
            reply = s.ask(f'go {0x26000000+entry:x}', timeout=20)
            print(reply, flush=True)
            assert 'rc = 0x1\r' in reply, reply
            after, states = records()
            assert states == [(0, 0), (1, 1)] and before[:512] == after[:512], states
            (t.OUT/'journal-before.bin').write_bytes(before)
            (t.OUT/'journal-after.bin').write_bytes(after)
            (t.OUT/'journal-test.json').write_text(json.dumps({'states': states, 'verified': True})+'\n')
            print('PASS: Lean journal write and independent MBOOT readback')


if __name__ == '__main__': main()
