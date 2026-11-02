#!/usr/bin/env python3
"""Run the compiled Lean memory/FDT editor against the live G13 oracle in ARM."""
import json
import random
import struct
from pathlib import Path

from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_MEM_READ, UC_HOOK_MEM_WRITE
from unicorn.arm_const import *
from os_profile import profile, decode_fdt, encode_recipe, EVIDENCE, OUT

CODE, DATA, RECIPE, STACK, STOP = 0x26000000, 0x23000000, 0x26200000, 0x28000000, 0x27F00000
SP = STACK + 0x10000
REGS = (UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3)
SAVED = tuple(range(UC_ARM_REG_R4, UC_ARM_REG_R11 + 1))


class Machine:
    def __init__(self):
        self.uc = Uc(UC_ARCH_ARM, UC_MODE_ARM)
        self.code = (OUT / 'g13-loader-core.bin').read_bytes()
        self.uc.mem_map(CODE, (len(self.code) + 4095) & ~4095)
        self.uc.mem_write(CODE, self.code)
        for address in (DATA, RECIPE, STACK, STOP):
            self.uc.mem_map(address, 0x20000)
        with (OUT / 'g13-loader-core.elf').open('rb') as f:
            elf = ELFFile(f)
            self.symbols = {s.name: CODE + s['st_value'] for s in elf.get_section_by_name('.symtab').iter_symbols()}
        self.ranges = [(DATA, 0x10000, True), (RECIPE, 0x10000, False)]
        self.uc.hook_add(UC_HOOK_MEM_READ, self.access, False)
        self.uc.hook_add(UC_HOOK_MEM_WRITE, self.access, True)
        self.calls, self.max_stack = 0, 0

    def access(self, uc, access, address, size, value, writing):
        if SP - 8192 <= address and address + size <= SP + 64:
            self.max_stack = max(self.max_stack, SP - address)
            return
        if not writing and CODE <= address and address + size <= CODE + len(self.code):
            return
        assert any(start <= address and address + size <= start + length and (writable or not writing)
                   for start, length, writable in self.ranges), (hex(address), size, writing)

    def call(self, symbol, *args, count=20000000):
        u = self.uc
        u.reg_write(UC_ARM_REG_CPSR, 0x1d3)
        u.reg_write(UC_ARM_REG_SP, SP)
        u.reg_write(UC_ARM_REG_LR, STOP)
        for i, r in enumerate(SAVED): u.reg_write(r, 0x12340000 + i)
        for r, v in zip(REGS, args): u.reg_write(r, v)
        if len(args) > 4: u.mem_write(SP, struct.pack('<' + 'I' * (len(args)-4), *args[4:]))
        u.emu_start(self.symbols[symbol], STOP, count=count)
        assert u.reg_read(UC_ARM_REG_PC) == STOP, (symbol, 'did not return', hex(u.reg_read(UC_ARM_REG_PC)))
        assert u.reg_read(UC_ARM_REG_SP) == SP
        assert [u.reg_read(r) for r in SAVED] == [0x12340000+i for i in range(8)]
        self.calls += 1
        return u.reg_read(UC_ARM_REG_R0)


def main():
    p, m = profile(), Machine()
    for blob in (p['original_dtb'], (EVIDENCE/'live.dtb').read_bytes()):
        m.uc.mem_write(DATA, blob)
        assert m.call('leanboot_fdt_validate', DATA, len(blob)) == 0
    m.uc.mem_write(DATA, p['original_dtb'])
    m.uc.mem_write(RECIPE, p['recipe'])
    assert m.call('leanboot_fdt_patch', DATA, 65536, RECIPE, len(p['recipe'])) == 0
    assert m.call('leanboot_fdt_reserve', DATA, 65536, 0x24000000, p['initrd_length']) == 0
    result = bytes(m.uc.mem_read(DATA, 65536))
    props, nodes, reservations = decode_fdt(result)
    assert props == p['expected_properties'], [(k, props.get(k), v) for k,v in p['expected_properties'].items() if props.get(k) != v]
    assert set(nodes) == set(decode_fdt((EVIDENCE/'live.dtb').read_bytes())[1])
    assert reservations == [(0x24000000, p['initrd_length'])]
    (OUT/'lean-prepared.dtb').write_bytes(result[:struct.unpack_from('>I', result, 4)[0]])
    # Capacity failure, malformed FDT header/tokens, unterminated strings.
    for offset, value in ((0, 0), (4, 65537), (8, 41), (12, 4), (16, 43),
                          (20, 18), (24, 17), (32, 65536), (36, 0xffffffff), (56, 0xff)):
        blob = bytearray(p['original_dtb'])
        struct.pack_into('>I', blob, offset, value)
        m.uc.mem_write(DATA, bytes(blob))
        assert m.call('leanboot_fdt_validate', DATA, len(blob)) != 0, offset
    m.uc.mem_write(DATA, p['original_dtb'])
    assert m.call('leanboot_fdt_patch', DATA, len(p['original_dtb']), RECIPE, len(p['recipe'])) != 0
    for recipe in (b'\0'*3, struct.pack('>I', 65), encode_recipe([('/a//b/c', b'v')]),
                   encode_recipe([('/a/b', b'v')])[:-1], encode_recipe([('/a/b\0c', b'v')])):
        m.uc.mem_write(DATA, p['original_dtb'])
        m.uc.mem_write(RECIPE, recipe)
        assert m.call('leanboot_fdt_patch', DATA, 65536, RECIPE, len(recipe)) != 0
    # Random overlap, equal pointers, unaligned ranges and exact bounds.
    rng = random.Random(0x47523133)
    m.ranges = [(DATA, 1024, True)]
    for _ in range(300):
        blob = rng.randbytes(1024)
        src, dst = rng.randrange(1025), rng.randrange(1025)
        count = rng.randrange(1025-max(src,dst))
        expected = bytearray(blob)
        expected[dst:dst+count] = blob[src:src+count]
        m.uc.mem_write(DATA, blob)
        assert m.call('leanboot_copy', DATA, 1024, dst, DATA, 1024, src, count) == 0
        assert bytes(m.uc.mem_read(DATA, 1024)) == expected
    for dst, src, count in ((1024, 0, 1), (0, 1024, 1), (0, 0, 1025), (0xffffffff, 0, 2)):
        assert m.call('leanboot_copy', DATA, 1024, dst, DATA, 1024, src, count) != 0
    report = {'calls': m.calls, 'max_stack_bytes': m.max_stack,
              'properties_matched': len(props), 'reservations': reservations}
    (OUT/'fdt-check.json').write_text(json.dumps(report, indent=2)+'\n')
    print('PASS:', report)


if __name__ == '__main__': main()
