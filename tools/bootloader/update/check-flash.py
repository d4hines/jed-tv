#!/usr/bin/env python3
"""Real ARM installer, verifier and FCIE driver; only registers/card are modeled."""
import argparse
import hashlib
import json
import struct
import sys
import importlib.util
sys.dont_write_bytecode = True
from pathlib import Path
from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_MEM_WRITE
from unicorn.arm_const import *
from package import package

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'assets/build/leanboot-network'
BASE, SOURCE, SCRATCH, STACK, STOP = 0x26000000, 0x21200000, 0x26500000, 0x28000000, 0x29000000
FIRST, SECTORS, SIZE, REG = 0x309000, 0x8000, 0x1000000, 0x1f222600


class PowerCut(Exception): pass


def bank_image(blob):
    data = blob[:160] + bytes(352) + blob[160:]
    assert len(data) <= SIZE
    return bytearray(data + bytes(SIZE - len(data)))


class Card:
    def __init__(self, banks=None, cache=0, partition=0, sector_size=0):
        self.banks = banks or [bytearray(SIZE), bytearray(SIZE)]
        self.u = Uc(UC_ARCH_ARM, UC_MODE_ARM)
        for address, size in [(BASE, 0x40000), (SOURCE, SIZE), (SCRATCH, SIZE),
                              (STACK, 0x10000), (STOP, 0x1000), (0x26350000, 0x1000),
                              (0x28d00000, 0x100000), (0x28fa3000, 0x1000), (0x294a3000, 0x1000),
                              (0x26400000, 0x10000), (0x40300000, 0x12000), (0x23000000, 0x10000)]:
            self.u.mem_map(address, size)
        self.u.mem_write(BASE, (OUT / 'flash-model.bin').read_bytes())
        with (OUT / 'flash-model.elf').open('rb') as f:
            self.syms = {s.name: BASE+s['st_value'] for s in ELFFile(f).get_section_by_name('.symtab').iter_symbols()}
        for address, value in [(0x28d8030c, 0xe1a0c00d), (0x28d80310, 0xe92ddef0),
                               (0x28d8031c, 0xe59f93e8), (0x28d7dfc4, 0xe1a0c00d),
                               (0x28d7dfcc, 0xe92ddef0), (0x28fa384c, 0x294a34c0)]:
            self.u.mem_write(address, struct.pack('<I', value))
        self.u.mem_write(0x294a3735, b'\1')
        self.regs = {0: 3, 7: 1, 0x0b: 4, 0x0c: 0, 0x0d: 0xff00, 0x0f: 0x1100, 0x3f: 1}
        self.ext = bytearray(512)
        self.ext[33], self.ext[179], self.ext[61] = cache, partition, sector_size
        self.writes, self.commands, self.reads = [], [], []
        self.cut_write = self.corrupt_read = self.fail_read = None
        self.torn = 0
        self.cut = False
        self.sctlr = 0
        for name, fn in [('leanboot_load16', self.load), ('leanboot_store16', self.store),
                         ('leanboot_read_sctlr', lambda *_: self.ret(self.sctlr))]:
            self.u.hook_add(UC_HOOK_CODE, fn, begin=self.syms[name], end=self.syms[name])
        self.u.hook_add(UC_HOOK_MEM_WRITE, self.memory_write)

    def memory_write(self, u, access, address, size, value, data):
        assert any(a <= address and address+size <= b for a, b in
                   [(SCRATCH, SCRATCH+SIZE), (STACK, STACK+0x10000), (0x26350000, 0x26351000),
                    (SOURCE, SOURCE+SIZE), (0x26404000, 0x26404800),
                    (0x40300000, 0x40300080), (0x23000000, 0x23010000)]), hex(address)

    def ret(self, value):
        self.u.reg_write(UC_ARM_REG_R0, value)
        self.u.reg_write(UC_ARM_REG_PC, self.u.reg_read(UC_ARM_REG_LR))

    def load(self, u, *_):
        address = u.reg_read(UC_ARM_REG_R0)
        if address == 0x1f203140: return self.ret(0x1000)
        if address == 0x1f203114: return self.ret(0)
        assert REG <= address < REG+256 and address % 4 == 0, hex(address)
        self.ret(self.regs.get((address-REG)//4, 0))

    def store(self, u, *_):
        address, value = u.reg_read(UC_ARM_REG_R0), u.reg_read(UC_ARM_REG_R1)
        if address == 0x1f203114: return self.ret(0)
        assert REG <= address < REG+256 and address % 4 == 0, hex(address)
        index, r = (address-REG)//4, self.regs
        if index in (0, 0x0d): r[index] = r.get(index, 0) & ~value
        elif index == 0x3f and value == 0: r[index] = 14
        elif index == 0x0c and value & 0x40:
            assert r[8] == 1 and r[5] == r[9] == 512 and r[6] == 0 and r[0x0b] == 5
            dma = (r[3] | r[4] << 16) + 0x20000000
            assert r[2] & 12 == 0
            if value == 0x58:
                assert dma == 0x26350800 and self.commands[-1][0] == 24
                bank, sector = divmod(self.commands[-1][1]-FIRST, SECTORS)
                self.writes.append((bank, sector))
                count = self.torn if len(self.writes) == self.cut_write else 512
                self.banks[bank][sector*512:sector*512+count] = u.mem_read(dma, count)
                r[0] |= 1
                if len(self.writes) == self.cut_write:
                    self.cut = True
                    u.emu_stop()
                    return
            else:
                cmd = r[0x20] & 63
                lba = ((r[0x20] >> 8) << 24) | ((r[0x21] & 255) << 16) | (r[0x21] & 0xff00) | r[0x22]
                assert cmd in (8, 17, 24)
                assert value == (0x46 if cmd == 24 else 0x4e)
                assert dma == (0x26350c00 if cmd == 8 else 0x26350800)
                if cmd != 8: assert FIRST <= lba < FIRST+2*SECTORS, hex(lba)
                self.commands.append((cmd, lba))
                r[0x20], r[0x21], r[0x22] = cmd, 0x900, 0
                r[0] |= 2
                if cmd == 8: u.mem_write(dma, bytes(self.ext)); r[0] |= 1
                if cmd == 17:
                    bank, sector = divmod(lba-FIRST, SECTORS)
                    self.reads.append((bank, sector))
                    if len(self.reads) == self.fail_read: r[0x0d] |= 0x10
                    data = bytearray(self.banks[bank][sector*512:(sector+1)*512])
                    if len(self.reads) == self.corrupt_read: data[0] ^= 1
                    u.mem_write(dma, bytes(data)); r[0] |= 1
            r[index] = value & ~0x40
        else: r[index] = value
        self.ret(0)

    def call(self, name, *args):
        self.u.reg_write(UC_ARM_REG_CPSR, 0x1d3)
        self.u.reg_write(UC_ARM_REG_SP, STACK+0x10000)
        self.u.reg_write(UC_ARM_REG_LR, STOP)
        for reg, value in zip([UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3], args):
            self.u.reg_write(reg, value)
        self.u.emu_start(self.syms[name], STOP, count=100_000_000)
        if self.cut: raise PowerCut
        assert self.u.reg_read(UC_ARM_REG_PC) == STOP, name+' instruction budget'
        return self.u.reg_read(UC_ARM_REG_R0)

    def install(self, blob):
        self.u.mem_write(SOURCE, blob)
        return self.call('leanboot_flash_install', SOURCE, len(blob))

    def version(self, bank): return self.call('leanboot_flash_version', bank)

    def frame(self, packet):
        self.u.mem_write(0x26401000, packet)
        size = self.call('leanboot_flash_frame', 0x26401000, len(packet))
        return bytes(self.u.mem_read(0x26404000, size))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--key', type=Path, required=True)
    args = parser.parse_args()
    build = json.loads((OUT/'build.json').read_text())
    for name, expected in build['source_sha256'].items():
        assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest() == expected, 'Stale build: '+name
    assert hashlib.sha256((OUT/'flash-model.bin').read_bytes()).hexdigest() == build['outputs']['flash-model']['sha256']
    old = package(bytes(range(256))*4, args.key, 1, component=2)
    new = package(bytes((i*31+17) % 256 for i in range(1537)), args.key, 2, component=2)
    original = bank_image(old)
    def fresh(**kw): return Card([bytearray(original), bytearray(SIZE)], **kw)
    count = 0
    m = Card(); assert m.install(old) == 0 and m.version(0) == 1 and m.version(1) == 0
    assert all(b == 0 for b, _ in m.writes); count += 1
    m = fresh(); assert m.install(new) == 0 and m.version(1) == 2
    assert m.banks[0] == original and m.writes == [(1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 0)]
    count += 1
    print('PASS: real FCIE driver provisions and alternates authenticated app banks', flush=True)
    for offset in [0, 8, 12, 16, 20, 24, 28, 36, 40, 72, 96, 159, 160, len(new)-1]:
        bad = bytearray(new); bad[offset] ^= 1
        m = fresh(); assert m.install(bytes(bad)) != 0 and not m.writes, offset; count += 1
    for bad in [new[:-1], new+b'\0', old, package(new[160:], args.key, 2, component=1),
                package(new[160:], args.key, 2, component=2, entry=4)]:
        m = fresh(); assert m.install(bad) != 0 and not m.writes; count += 1
    for kw in [dict(cache=1), dict(partition=1), dict(sector_size=1)]:
        m = fresh(**kw); assert m.install(new) != 0 and not m.writes; count += 1
    for bank, sector, pointer in [(2, 0, SCRATCH), (0, SECTORS, SCRATCH), (0, 0, SCRATCH-4),
                                  (0, 0, SCRATCH+SIZE-508), (0, 0, SCRATCH+1)]:
        m = fresh(); assert m.call('leanboot_flash_transfer', 1, bank, sector, pointer) == 1
        assert not m.commands; count += 1
    for control in [1, 4, 5]:
        m = fresh(); m.sctlr = control; assert m.install(new) != 0 and not m.commands; count += 1
    print('PASS: signatures, payloads, targets, versions, card state and DMA bounds reject', flush=True)
    for cut in range(1, 7):
        for torn in [0, 1, 159, 160, 511, 512]:
            m = fresh(); m.cut_write, m.torn = cut, torn
            try: m.install(new); raise AssertionError('cut not reached')
            except PowerCut: pass
            after = Card(m.banks)
            assert m.banks[0] == original and after.version(0) == 1
            assert after.version(1) in (0, 2), (cut, torn)
            count += 1
    m = fresh(); m.corrupt_read = 5
    assert m.install(new) == 5 and m.writes[-1] != (1, 0) and m.banks[0] == original
    count += 1
    for failed in [1, 2, 3, 4]:
        m = fresh(); m.fail_read = failed
        assert m.install(new) == 8 and not m.writes
        count += 1
    # Ethernet/IP/UDP parsing -> session receiver -> real signed FCIE commit.
    sys.path.append(str(ROOT/'tools/bootloader'))
    spec = importlib.util.spec_from_file_location('ethernet_fixture', ROOT/'tools/bootloader/check-ethernet.py')
    eth = importlib.util.module_from_spec(spec); spec.loader.exec_module(eth)
    from fixtures_tv import transfer
    m = Card(); bad = bytearray(old); bad[96] ^= 1
    for request, expected in transfer(bytes(bad), 51, 18) + transfer(old, 52):
        reply = m.frame(eth.udp(request))
        _, op, session, offset, length = struct.unpack('<5I', request[:20])
        acknowledged = 0 if op == 1 else offset+length if op == 2 else offset
        assert len(reply) == 62 and reply[42:] == struct.pack('<5I', 0x3152554c, session, op, expected, acknowledged)
        if session == 51: assert not m.writes
    writes = len(m.writes)
    assert m.frame(eth.udp(request)) == reply and len(m.writes) == writes
    assert m.version(0) == 1
    assert m.frame(eth.udp(struct.pack('<5I', 0x3152554c, 1, 53, 0, len(new)))) == b''
    count += 1
    # The actual FDT editor publishes the selected bank without changing any
    # other properties; invalid choices perform no edit.
    from os_profile import EVIDENCE, decode_fdt
    original_fdt = (EVIDENCE/'live.dtb').read_bytes()
    original_props = decode_fdt(original_fdt)[0]
    for choice, expected in [(0, b'-\0'), (1, b'0\0'), (2, b'1\0')]:
        m.u.mem_write(0x23000000, original_fdt)
        assert m.call('leanboot_flash_handoff', choice, 0x23000000) == 0
        props = decode_fdt(bytes(m.u.mem_read(0x23000000, 65536)))[0]
        assert props.pop('/chosen/dande,apps-bank') == expected and props == original_props
        count += 1
    before = bytes(m.u.mem_read(0x23000000, 65536))
    assert m.call('leanboot_flash_handoff', 3, 0x23000000) != 0
    assert bytes(m.u.mem_read(0x23000000, 65536)) == before
    count += 1
    report = dict(cases=count, power_cuts=36, real_arm_crypto=True, real_lean_fcie=True, hardware_tested=False,
                  sha256=build['outputs']['flash-model']['sha256'])
    (OUT/'flash-checks.json').write_text(json.dumps(report, indent=2)+'\n')
    print('PASS:', json.dumps(report), flush=True)


if __name__ == '__main__': main()
