#!/usr/bin/env python3
"""Run the compiled ARM journal/policy with torn-write and I/O fault models."""
import json
import re
import struct
import zlib
from pathlib import Path
from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_MEM_WRITE
from unicorn.arm_const import *
from build import OUT
from os_profile import profile

BASE, STACK, STOP = 0x26000000, 0x28000000, 0x27000000


def record(seq, state):
    data = bytearray(512)
    struct.pack_into('<6I', data, 0, 0x314a424c, 1, seq, state, seq ^ 0xffffffff, state ^ 0xffffffff)
    struct.pack_into('<I', data, 508, zlib.crc32(data[:508]))
    return bytes(data)


def machine():
    u = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    for a, n in ((BASE, 65536), (STACK, 65536), (STOP, 4096), (0x26350000, 4096)):
        u.mem_map(a, n)
    u.mem_write(BASE, (OUT/'g13-loader-core.bin').read_bytes())
    with (OUT/'g13-loader-core.elf').open('rb') as f:
        symbols = {s.name: BASE+s['st_value'] for s in ELFFile(f).get_section_by_name('.symtab').iter_symbols()}
    return u, symbols


def call(u, symbols, name, *args):
    u.reg_write(UC_ARM_REG_CPSR, 0x1d3)
    u.reg_write(UC_ARM_REG_SP, STACK+65536)
    u.reg_write(UC_ARM_REG_LR, STOP)
    for reg, value in zip((UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3), args):
        u.reg_write(reg, value)
    u.emu_start(symbols[name], STOP, count=3000000)
    assert u.reg_read(UC_ARM_REG_PC) == STOP, name
    return u.reg_read(UC_ARM_REG_R0)


def advance(records, torn=512, read_error=False, write_error=False):
    records = list(records)
    u, symbols = machine()
    writes = []
    def io(uc, address, size, kind):
        sector, buffer = uc.reg_read(UC_ARM_REG_R0), uc.reg_read(UC_ARM_REG_R1)
        assert sector in (0x3217fe, 0x3217ff)
        assert 0x26350000 <= buffer <= 0x26350e00 and buffer % 64 == 0
        slot = sector - 0x3217fe
        result = 0
        if kind == 'read':
            uc.mem_write(buffer, records[slot])
            result = int(read_error)
        else:
            new = bytes(uc.mem_read(buffer, 512))
            records[slot] = new[:torn] + records[slot][torn:]
            writes.append((slot, new))
            result = int(write_error)
        uc.reg_write(UC_ARM_REG_R0, result)
        uc.reg_write(UC_ARM_REG_PC, uc.reg_read(UC_ARM_REG_LR))
    for name, kind in [('l_LeanBoot_Unsafe_Bootstrap_readSector___redArg', 'read'),
                       ('l_LeanBoot_Unsafe_Bootstrap_writeJournal___redArg', 'write')]:
        u.hook_add(UC_HOOK_CODE, io, kind, begin=symbols[name], end=symbols[name])
    def ram(uc, access, a, size, value, data):
        assert STACK <= a < STACK+65536 or 0x26350000 <= a and a+size <= 0x26351000, hex(a)
    u.hook_add(UC_HOOK_MEM_WRITE, ram)
    result = call(u, symbols, 'leanboot_journal_advance')
    return result, records, writes


def test():
    u, symbols = machine()
    cases = 0
    # Codec is checked against an independent Python CRC32 implementation.
    for seq in (0, 1, 0xfffffffe, 0xffffffff):
        for state in (0, 1, 2, 4):
            assert call(u, symbols, 'leanboot_journal_encode', 0x26350000, seq, state) == 0
            assert bytes(u.mem_read(0x26350000, 512)) == record(seq, state)
            assert call(u, symbols, 'leanboot_journal_decode', 0x26350000) == state
            assert u.reg_read(UC_ARM_REG_R1) == seq
            cases += 1
    records = [record(0, 0)]*2
    for i in range(60):
        result, records, writes = advance(records)
        assert result == (1, 2, 4)[i % 3] and len(writes) == 1
        cases += 1
    for prefix in range(513):
        initial = [record(41, 2), record(40, 1)]
        result, after, writes = advance(initial, torn=prefix)
        committed = after[1] == record(42, 4)
        assert result == (4 if committed else 0x80000004), (prefix, result)
        assert after[0] == initial[0] and len(writes) == 1
        # Whichever record survives the interruption, a subsequent stable
        # start can progress; it never requires the partially written record.
        nxt, _, _ = advance(after)
        assert nxt in (1, 4), (prefix, nxt)
        cases += 1
    for initial in ([bytes(512)]*2, [record(1, 1), record(7, 2)],
                    [record(2, 1), record(2, 2)]):
        result, _, writes = advance(initial)
        assert result == 0x80000004 and not writes
        cases += 1
    for error in ('read_error', 'write_error'):
        result, _, writes = advance([record(0, 0)]*2, **{error: True})
        assert result == 0x80000004
        if error == 'read_error': assert not writes
        cases += 1
    result, after, _ = advance([record(0xfffffffe, 1)]*2)
    assert result == 2
    result, after, _ = advance(after)
    assert result == 4
    cases += 2
    # The real captured FDT + bootconfig must select normal boot, and the
    # recovery value / missing flag / malformed trailer must bypass it.
    p = profile()
    u.mem_map(0x284c0000, 0x20000)
    u.mem_map(0x24000000, 0x1400000)
    from os_profile import EVIDENCE
    fdt = (EVIDENCE/'live.dtb').read_bytes()
    end = int.from_bytes(p['expected_properties']['/chosen/linux,initrd-end'], 'big')
    # This FDT fixture is the original live one, before the Lean-marker recipe.
    from os_profile import decode_fdt
    original, _, _ = decode_fdt(fdt)
    end = int.from_bytes(original['/chosen/linux,initrd-end'], 'big')
    u.mem_write(0x284cd000, fdt)
    for normal, raw in ((True, False), (False, False), (None, False), (True, True), (False, True)):
        config = (EVIDENCE/'bootconfig.txt').read_bytes()
        if normal is False: config = config.replace(b'force_normal_boot = "1"', b'force_normal_boot = "0"')
        if normal is None: config = config.replace(b'force_normal_boot', b'force_norMal_boot')
        if raw: config = re.sub(rb' = "([^"]*)"', rb'=\1', config)
        config += bytes(-len(config) % 4)
        u.mem_write(end - 20 - len(config), config)
        u.mem_write(end - 20, struct.pack('<II', len(config), sum(config)) + b'#BOOTCONFIG\n')
        assert call(u, symbols, 'leanboot_bundle_normal', 0x284cd000) == int(normal is True), normal
        cases += 1
    report = {'cases': cases, 'scope': 'compiled ARM policy/codec/recovery gate; modeled block I/O',
              'torn_sector_prefixes': 513, 'full_start_sequence': ['Lean', 'Lean', 'stock', 'Lean']}
    (OUT/'boot-policy-test.json').write_text(json.dumps(report, indent=2)+'\n')
    print(f'PASS: {cases} boot-policy, torn-write and recovery cases')


if __name__ == '__main__': test()
