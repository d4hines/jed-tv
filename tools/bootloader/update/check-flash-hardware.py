#!/usr/bin/env python3
"""Offline adversarial FCIE scheduling. Runs actual ARM, never opens the TV.

The saved TCL header/HAL is an independent source for register semantics.
This extends the immediate register fixture with delayed command, DMA and
card-program completion. Accepted writes are volatile until card-ready;
reset may lose them. This is a contract model, not an electrical simulator.
"""
import hashlib
import argparse
import importlib.util
import json
import struct
import sys
from pathlib import Path
sys.dont_write_bytecode = True
spec = importlib.util.spec_from_file_location('flash_fixture', Path(__file__).with_name('check-flash.py'))
fixture = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fixture)
from unicorn.arm_const import UC_ARM_REG_R0, UC_ARM_REG_R1


class ScheduledCard(fixture.Card):
    def __init__(self, fault=None, fault_cmd=24, cut_busy_write=None, **kw):
        super().__init__(**kw)
        self.fault = fault
        self.fault_cmd = fault_cmd
        self.cut_busy_write = cut_busy_write
        self.command_ticks = self.dma_ticks = self.busy_ticks = 0
        self.pending = None
        self.pending_dma = None
        self.polls = 0
        self.resets = 0
        self.active_cmd = 0

    def load(self, u, *_):
        address = u.reg_read(UC_ARM_REG_R0)
        self.polls += 1
        if address == fixture.REG:
            fault = self.fault if self.active_cmd == self.fault_cmd else None
            if self.command_ticks and fault != 'command_timeout':
                self.command_ticks -= 1
                if not self.command_ticks: self.regs[0] |= 2
            elif self.dma_ticks and fault != 'dma_timeout':
                self.dma_ticks -= 1
                if not self.dma_ticks:
                    if self.pending_dma:
                        pointer, data = self.pending_dma
                        u.mem_write(pointer, data)
                        self.pending_dma = None
                    self.regs[0] |= 1
        if address == fixture.REG+0x0d*4 and not self.dma_ticks and self.busy_ticks:
            if self.fault != 'busy_timeout': self.busy_ticks -= 1
            if self.pending and len(self.writes) == self.cut_busy_write and self.busy_ticks == 4:
                bank, sector, data = self.pending
                start = sector*512
                # Non-prefix, bit-level tearing; untouched bytes retain the
                # previous durable data. This intentionally lacks atomicity.
                before = self.banks[bank][start:start+512]
                self.banks[bank][start:start+512] = bytes(
                    (a & 0xaa) | (b & 0x55) if i % 3 else a
                    for i, (a, b) in enumerate(zip(before, data)))
                self.cut = True
                u.emu_stop()
                return
            if not self.busy_ticks:
                self.regs[0x0d] &= ~0x40
                if self.pending:
                    bank, sector, data = self.pending
                    self.banks[bank][sector*512:(sector+1)*512] = data
                    self.pending = None
                if self.fault == 'late_error': self.regs[0x0d] |= 2
        super().load(u)

    def store(self, u, *_):
        address, value = u.reg_read(UC_ARM_REG_R0), u.reg_read(UC_ARM_REG_R1)
        starting = address == fixture.REG+0x0c*4 and value & 0x40
        before = None
        if starting and value == 0x58:
            bank, sector = divmod(self.commands[-1][1]-fixture.FIRST, fixture.SECTORS)
            before = bytes(self.banks[bank][sector*512:(sector+1)*512])
        super().store(u)
        if address == fixture.REG+0x3f*4 and value == 0:
            self.resets += 1
            self.pending = self.pending_dma = None
            self.command_ticks = self.dma_ticks = self.busy_ticks = 0
        if not starting: return
        if value != 0x58:
            self.active_cmd = self.commands[-1][0]
            self.command_ticks = 3
            self.regs[0] &= ~2
            if isinstance(self.fault, int) and self.active_cmd == self.fault_cmd:
                # Construct R1 as wire bytes, then pack adjacent FIFO bytes.
                response = bytes([self.active_cmd]) + (0x900 | self.fault).to_bytes(4, 'big') + b'\0'
                for index, offset in enumerate(range(0, 6, 2)):
                    self.regs[0x20+index] = int.from_bytes(response[offset:offset+2], 'little')
            if self.fault == 'response_index' and self.active_cmd == self.fault_cmd:
                self.regs[0x20] ^= 1
            if self.active_cmd != 24:
                dma = (self.regs[3] | self.regs[4] << 16) + 0x20000000
                self.pending_dma = (dma, bytes(u.mem_read(dma, 512)))
                u.mem_write(dma, b'\xcc'*512)
                self.dma_ticks = 5
                self.regs[0] &= ~1
        else:
            self.pending = (bank, sector, bytes(self.banks[bank][sector*512:(sector+1)*512]))
            self.banks[bank][sector*512:(sector+1)*512] = before
            self.dma_ticks, self.busy_ticks = 5, 7
            self.regs[0] &= ~1
            self.regs[0x0d] |= 0x40


def evidence():
    directory = fixture.ROOT/'assets/research/g13-leanboot-default/sources'
    records = json.loads((directory/'manifest.json').read_text())
    for record in records:
        path = fixture.ROOT/record['file']
        assert hashlib.sha256(path.read_bytes()).hexdigest() == record['sha256'], path
    header = (directory/'eMMC_reg_v5.h').read_text()
    import re
    for name, bit in [('BIT_DMA_END', 0), ('BIT_CMD_END', 1), ('BIT_SD_CARD_BUSY', 6),
                      ('BIT_JOB_START', 6), ('BIT_CHK_CMD', 7), ('BIT_DAT_WR_CERR', 1),
                      ('BIT_DAT_RD_TOUT', 5)]:
        assert re.search(r'#define\s+'+name+r'\s+BIT'+str(bit)+r'\b', header), name
    return records


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--key', type=Path, required=True)
    args = parser.parse_args()
    records = evidence()
    build = json.loads((fixture.OUT/'build.json').read_text())
    for name, expected in build['source_sha256'].items():
        assert hashlib.sha256((fixture.ROOT/name).read_bytes()).hexdigest() == expected, 'Stale build: '+name
    binary_hash = hashlib.sha256((fixture.OUT/'flash-model.bin').read_bytes()).hexdigest()
    assert binary_hash == build['outputs']['flash-model']['sha256']
    cases = []
    geometry_path = fixture.ROOT/'assets/backups/g13-v013-stock-20261031/manifest.json'
    geometry = json.loads(geometry_path.read_text())
    assert geometry['target'] == '18ACC22D1C47' and 'G13' in geometry['fingerprint']
    upgrade = geometry['geometry']['upgrade']
    start, end = fixture.FIRST*512, (fixture.FIRST+2*fixture.SECTORS)*512
    assert upgrade['start_bytes']+1048576 == start
    assert end < upgrade['start_bytes']+upgrade['size_bytes']-1024
    for name, entry in geometry['geometry'].items():
        if name != 'upgrade':
            assert end <= entry['start_bytes'] or entry['start_bytes']+entry['size_bytes'] <= start, name
    cases.append(dict(case='captured_partition_geometry', sha256=hashlib.sha256(geometry_path.read_bytes()).hexdigest()))
    data = bytes((i*17+3) % 256 for i in range(512))
    for bank, sector in [(0, 0), (0, 0x7fff), (1, 0), (1, 0x7fff)]:
        m = ScheduledCard()
        m.u.mem_write(fixture.SCRATCH, data)
        assert m.call('leanboot_flash_transfer', 1, bank, sector, fixture.SCRATCH) == 0
        assert m.pending is None and m.banks[bank][sector*512:(sector+1)*512] == data
        m.u.mem_write(fixture.SCRATCH, bytes(512))
        assert m.call('leanboot_flash_transfer', 0, bank, sector, fixture.SCRATCH) == 0
        assert bytes(m.u.mem_read(fixture.SCRATCH, 512)) == data
        # Golden eMMC wire arguments for the four boundary LBAs.
        assert [c for c in m.commands if c[0] != 8] == [(24, fixture.FIRST+bank*0x8000+sector), (17, fixture.FIRST+bank*0x8000+sector)]
        cases.append(dict(case='delayed_boundary', bank=bank, sector=sector, polls=m.polls))
    for kw in [dict(partition=n) for n in range(1, 8)] + [dict(cache=1), dict(sector_size=1)]:
        for writing in [0, 1]:
            m = ScheduledCard(**kw)
            assert m.call('leanboot_flash_transfer', writing, 0, 0, fixture.SCRATCH) != 0
            assert all(cmd == 8 for cmd, _ in m.commands) and not m.writes
            cases.append(dict(case='card_state', writing=writing, **kw))
    errors = [1 << bit for bit in range(32) if 0xfdffa080 & (1 << bit)]
    for fault in errors + ['response_index', 'command_timeout', 'dma_timeout', 'busy_timeout', 'late_error']:
        m = ScheduledCard(fault=fault)
        m.u.mem_write(fixture.SCRATCH, data)
        assert m.call('leanboot_flash_transfer', 1, 0, 0, fixture.SCRATCH) != 0, fault
        assert m.resets == 1 and m.regs[7] == 0 and m.regs[0x3f] == 14, fault
        before = len(m.commands)
        assert m.call('leanboot_flash_transfer', 1, 0, 0, fixture.SCRATCH) != 0
        assert len(m.commands) == before, 'A reset controller must not be reused'
        cases.append(dict(case='fault', fault=fault, polls=m.polls))
    # Directly exercise the actual compiled barrier, including the added
    # reset/job checks. This symbol is already retained by the installer.
    for reg, value, expected in [(7, 1, 1), (7, 0, 0), (0x3f, 14, 0),
                                 (0x0c, 0x40, 0), (0x0d, 2, 0), (0x0d, 0x40, 0)]:
        m = ScheduledCard()
        m.regs[reg] = value
        assert m.call('l_LeanBoot_Unsafe_Bootstrap_flashFlush___redArg') == expected
        assert not m.commands and not m.writes
        cases.append(dict(case='flush_guard', register=reg, value=value, polls=m.polls))
    old = fixture.package(bytes(range(256))*4, args.key, 1, component=2)
    new = fixture.package(bytes((i*31+17) % 256 for i in range(1537)), args.key, 2, component=2)
    original = fixture.bank_image(old)
    m = ScheduledCard(banks=[bytearray(original), bytearray(fixture.SIZE)])
    assert m.install(new) == 0 and m.version(1) == 2 and m.banks[0] == original
    assert m.pending is None
    cases.append(dict(case='delayed_install', polls=m.polls))
    for cut in range(1, 7):
        m = ScheduledCard(banks=[bytearray(original), bytearray(fixture.SIZE)], cut_busy_write=cut)
        try: m.install(new); raise AssertionError('cut not reached')
        except fixture.PowerCut: pass
        after = ScheduledCard(banks=m.banks)
        assert m.banks[0] == original and after.version(0) == 1
        assert after.version(1) in (0, 2)
        cases.append(dict(case='busy_phase_bit_tear', write=cut))
    report = dict(cases=len(cases), results=cases, binary_sha256=binary_hash,
                  evidence=records, hardware_tested=False, busy_phase_power_cuts=6,
                  test_sha256=hashlib.sha256(Path(__file__).read_bytes()).hexdigest(),
                  fixture_sha256=hashlib.sha256(Path(fixture.__file__).read_bytes()).hexdigest(),
                  model='delayed CMD_END/DMA_END/DAT0; volatile writes until ready; bounded faults and reset')
    (fixture.OUT/'flash-hardware-checks.json').write_text(json.dumps(report, indent=2)+'\n')
    print('PASS:', len(cases), 'scheduled FCIE/card checks', flush=True)


if __name__ == '__main__': main()
