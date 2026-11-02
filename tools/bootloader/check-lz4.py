#!/usr/bin/env python3
"""Differential raw-LZ4 tests against the compiled Lean decoder."""
import json
import random
from pathlib import Path
from importlib.machinery import SourceFileLoader
import lz4.block
from unicorn import UC_HOOK_MEM_WRITE, UC_HOOK_MEM_READ
from build import OUT

model = SourceFileLoader('policy_model', str(Path(__file__).with_name('check-boot-policy.py'))).load_module()


def check(packed, size, expected=None):
    u, syms = model.machine()
    src, dst = 0x26600000, 0x26700000
    u.mem_map(src, 0x100000); u.mem_map(dst, 0x100000)
    u.mem_write(src, packed or b'\0')
    def writes(uc, access, a, n, v, data):
        assert model.STACK <= a and a+n <= model.STACK+65536 or dst <= a and a+n <= dst+size, (hex(a), n, size)
    def reads(uc, access, a, n, v, data):
        if src <= a < src+0x100000: assert a+n <= src+len(packed)
        if dst <= a < dst+0x100000: assert a+n <= dst+size
    u.hook_add(UC_HOOK_MEM_WRITE, writes);u.hook_add(UC_HOOK_MEM_READ, reads)
    result = model.call(u, syms, 'leanboot_lz4_decode', src, len(packed), dst, size)
    if expected is None: assert result != 0, (packed, size)
    else: assert result == 0 and bytes(u.mem_read(dst, size)) == expected


if __name__ == '__main__':
    rng = random.Random(771)
    cases = 0
    for size in (0, 1, 3, 4, 15, 16, 31, 255, 256, 4096, 16384):
        for data in (bytes(size), rng.randbytes(size), (b'abc'*((size+2)//3))[:size]):
            packed = lz4.block.compress(data, mode='high_compression', store_size=False)
            check(packed, size, data); cases += 1
            if size:
                check(packed, size-1);cases += 1
            for n in range(min(len(packed), 20)):
                check(packed[:n], size);cases += 1
    for packed, size in [(b'\x00\x00\x00', 4), (b'\x00\x01\x00', 4),
                         (b'\xf0\xff', 1), (b'\x10x\x01\x00', 4)]:
        check(packed, size); cases += 1
    (OUT/'lz4-test.json').write_text(json.dumps({'cases': cases, 'compiled_arm': True})+'\n')
    print(f'PASS: {cases} differential and malformed LZ4 cases')
