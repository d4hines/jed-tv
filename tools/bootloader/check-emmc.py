#!/usr/bin/env python3
"""Run compiled Lean FCIE I/O against a register model with injected failures.

The model checks command framing, DMA bounds and bounded error returns.
It does not model electrical timing or establish hardware compatibility.
"""
import json
import struct
from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE
from unicorn.arm_const import *
from build import OUT

BASE, STACK, STOP = 0x26000000, 0x28000000, 0x27000000
REG = 0x1f222600


def run(fault=None, sector=0x3217fe, buffer=0x26350000, write=False, cache=0, partition=0, sector_size=0):
    u = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    for a, n in ((BASE, 65536), (STACK, 65536), (STOP, 4096), (0x26350000, 4096)):
        u.mem_map(a, n)
    u.mem_map(0x28fa3000, 4096); u.mem_map(0x294a3000, 4096)
    u.mem_write(0x28fa384c, struct.pack('<I', 0x294a34c0))
    u.mem_write(0x294a3735, b'\x01')
    u.mem_write(BASE, (OUT/'g13-loader-core.bin').read_bytes())
    with (OUT/'g13-loader-core.elf').open('rb') as f:
        syms = {s.name: BASE+s['st_value'] for s in ELFFile(f).get_section_by_name('.symtab').iter_symbols()}
    data = bytes((i*13+7) % 256 for i in range(512))
    regs = {0: 3, 7: 1, 0x0b: 4, 0x0c: 0, 0x0d: 0xff00, 0x0f: 0x1100, 0x3f: 1}
    state = {'reads': 0, 'writes': 0, 'commands': 0, 'dma_bytes': 0}
    flash = [bytes(512)]
    if write: u.mem_write(buffer, data)
    def ret(value):
        u.reg_write(UC_ARM_REG_R0, value)
        u.reg_write(UC_ARM_REG_PC, u.reg_read(UC_ARM_REG_LR))
    def read(uc, a, n, _):
        addr = uc.reg_read(UC_ARM_REG_R0)
        state['reads'] += 1
        if addr == 0x1f203140: ret(0x1000)
        elif addr == 0x1f203114: ret(0)
        else:
            assert REG <= addr < REG+256 and addr % 4 == 0
            index = (addr-REG)//4
            ret(regs.get(index, 0))
    def store(uc, a, n, _):
        addr, value = uc.reg_read(UC_ARM_REG_R0), uc.reg_read(UC_ARM_REG_R1)
        if addr == 0x1f203114:
            ret(0)
            return
        assert REG <= addr < REG+256 and addr % 4 == 0
        index = (addr-REG)//4
        state['writes'] += 1
        if index in (0, 0x0d): regs[index] = regs.get(index, 0) & ~value
        elif index == 0x3f and value == 0: regs[index] = 14
        elif index == 0x0c and value & 0x40:
            assert regs[8] == 1 and regs[5] == 512 and regs[6] == 0
            assert regs[9] == 512 and regs[0x0b] == 5
            dma = (regs[3] | regs[4] << 16) + 0x20000000
            assert regs[2] & 12 == 0
            if value == 0x58:
                assert write and dma == buffer and state['commands'] == 2
                flash[0] = bytes(uc.mem_read(dma, 512))
                regs[0] |= 1
                state['dma_bytes'] += 512
            else:
                cmd = regs[0x20] & 63
                assert cmd in (8, 17, 24)
                assert value == (0x46 if cmd == 24 else 0x4e)
                assert regs[0x20] == 0x40 | cmd
                assert regs[0x21] == (0 if cmd == 8 else 0x1732)
                assert regs[0x22] == (0 if cmd == 8 else 0xfe)
                assert dma == (0x26350c00 if cmd == 8 else buffer)
                state['commands'] += 1
                regs[0x20], regs[0x21], regs[0x22] = cmd, 0x900, 0
                if fault != 'command_timeout': regs[0] |= 2
                if fault == 'crc': regs[0x0d] |= 0x10
                if cmd != 24 and fault != 'dma_timeout':
                    ext = bytearray(512)
                    ext[33], ext[179], ext[61] = cache, partition, sector_size
                    uc.mem_write(dma, bytes(ext) if cmd == 8 else data)
                    state['dma_bytes'] += 512
                    regs[0] |= 1
            regs[index] = value & ~0x40
        else: regs[index] = value
        ret(0)
    for name, fn in [('leanboot_load16', read), ('leanboot_store16', store)]:
        u.hook_add(UC_HOOK_CODE, fn, begin=syms[name], end=syms[name])
    u.hook_add(UC_HOOK_CODE, lambda *_: ret(0), begin=syms['leanboot_read_sctlr'], end=syms['leanboot_read_sctlr'])
    u.reg_write(UC_ARM_REG_CPSR, 0x1d3)
    u.reg_write(UC_ARM_REG_SP, STACK+65536)
    u.reg_write(UC_ARM_REG_LR, STOP)
    u.reg_write(UC_ARM_REG_R0, sector); u.reg_write(UC_ARM_REG_R1, buffer)
    u.emu_start(syms['leanboot_write_journal' if write else 'leanboot_read_sector'], STOP, count=40000000)
    assert u.reg_read(UC_ARM_REG_PC) == STOP, 'I/O failed to return within instruction budget'
    status = u.reg_read(UC_ARM_REG_R0)
    expected = {'command_timeout': 44, 'crc': 45, 'dma_timeout': 52}.get(fault, 0)
    if sector != 0x3217fe or buffer != 0x26350000: expected = 40
    if write and (cache or partition or sector_size): expected = 55
    assert status == expected, (fault, status, expected)
    if status == 0: assert bytes(u.mem_read(buffer, 512)) == data
    if write: assert flash[0] == (data if status == 0 else bytes(512))
    assert regs[0x0b] == 4 and regs[0x0f] == 0x1100 and regs[0x0c] == 0
    assert state['reads'] < 1001000
    if status == 40: assert state['writes'] == state['reads'] == 0
    return dict(fault=fault, status=status, **state)


if __name__ == '__main__':
    results = [run(), run('crc'), run('command_timeout'), run('dma_timeout'),
               run(sector=0), run(buffer=0x26350001), run(write=True),
               run(write=True, cache=1), run(write=True, partition=1),
               run(write=True, sector_size=1), run(write=True, buffer=0x26350c00)]
    (OUT/'emmc-test.json').write_text(json.dumps(results, indent=2)+'\n')
    print('PASS: FCIE read/write, CRC/timeout returns, DMA bounds, cache/partition and address guards')
