"""Run the stock V013 MBR/FAT reader against a read-only captured disk prefix.

Requires unicorn. No USB/TV access. The block transport, heap and console are
host stubs; the partition/FAT/filename parsing executes original ARM code.
Reads outside the captured prefix fail, rather than returning fabricated data.
"""
import argparse
from collections import deque
import hashlib
import json
from pathlib import Path
import struct

from unicorn import Uc, UcError, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE
from unicorn.arm_const import *

p = argparse.ArgumentParser(description=__doc__)
p.add_argument('snapshot', type=Path)
p.add_argument('output', type=Path)
p.add_argument('--filename', default='/update_signed.zip')
a = p.parse_args()
root = Path(__file__).resolve().parents[2]
binary = (root / 'assets/backups/g13-v013-stock-20261031/MBOOT.img').read_bytes()
assert hashlib.sha256(binary).hexdigest() == '643058879b84d044dea58a4572bd9ade1cc3140f3d3357eeaabae53be32ec382'
disk = a.snapshot.read_bytes()
u = Uc(UC_ARCH_ARM, UC_MODE_ARM)
u.mem_map(0x27400000, 0x1000000)
u.mem_write(0x27400000, binary[0x2400:])
u.mem_map(0x10000000, 0x3000000)
stop, block_read, descriptor = 0x10000000, 0x10000100, 0x10001000
filename, destination, stack = 0x10002000, 0x10010000, 0x10200000
heap = 0x11000000
reads, messages, trace, stub_calls = [], [], deque(maxlen=32), {}

def word(address, value):
    u.mem_write(address, struct.pack('<I', value & 0xffffffff))

def cstr(address):
    out = bytearray()
    for offset in range(8192):
        b = u.mem_read(address + offset, 1)[0]
        if not b:
            return bytes(out)
        out.append(b)
    raise RuntimeError('Unterminated string')

def ret(value):
    u.reg_write(UC_ARM_REG_R0, value & 0xffffffff)
    u.reg_write(UC_ARM_REG_PC, u.reg_read(UC_ARM_REG_LR))

def hook(uc, pc, size, context):
    global heap
    trace.append(hex(pc))
    r0, r1, r2, r3 = [u.reg_read(r) for r in
                       (UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3)]
    if pc == block_read:
        begin, end = r1 * 512, (r1 + r2) * 512
        if end > len(disk):
            raise RuntimeError(f'Uncaptured sector read {r1}+{r2}')
        reads.append({'lba': r1, 'sectors': r2})
        u.mem_write(r3, disk[begin:end])
        ret(r2)
    elif pc in (0x274c01cc, 0x274c016c):
        messages.append({'format': cstr(r0).decode('ascii', 'replace'),
                         'args': [r1, r2, r3]})
        ret(0)
    elif pc == 0x274c0b20:  # malloc
        address = heap
        heap += (r0 + 63) & ~63
        assert heap < 0x12f00000
        stub_calls['malloc'] = stub_calls.get('malloc', 0) + 1
        ret(address)
    elif pc == 0x274c08c4:  # free
        stub_calls['free'] = stub_calls.get('free', 0) + 1
        ret(0)

u.hook_add(UC_HOOK_CODE, hook)
word(descriptor, 4)              # IF_TYPE_USB
word(descriptor + 4, 0)          # device index
word(descriptor + 8, 2)          # PART_TYPE_DOS
word(descriptor + 0x10, 123009761280 // 512)  # source disk LBA count
word(descriptor + 0x14, 512)      # block size consumed by FAT directory walker
word(descriptor + 0x60, block_read)
u.mem_write(filename, a.filename.encode() + b'\0')

def call(address, args):
    for r, value in zip((UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3), args):
        u.reg_write(r, value)
    u.reg_write(UC_ARM_REG_SP, stack)
    u.reg_write(UC_ARM_REG_LR, stop)
    u.emu_start(address, stop, count=3000000)
    assert u.reg_read(UC_ARM_REG_PC) == stop, 'Instruction budget exhausted'
    value = u.reg_read(UC_ARM_REG_R0)
    return value if value < 0x80000000 else value - 0x100000000

report = {'snapshot': str(a.snapshot), 'snapshot_sha256': hashlib.sha256(disk).hexdigest(),
          'filename': a.filename, 'source_mboot_sha256': hashlib.sha256(binary).hexdigest(),
          'scope': 'Exact ARM partition and FAT read code; USB block transport, heap and console stubbed. No hardware enumeration or recovery boot is emulated.'}
try:
    report['register_result'] = call(0x27444994, [descriptor, 1])
    assert report['register_result'] == 0, 'FAT device registration failed'
    report['read_result'] = call(0x27445a3c, [filename, destination, 22])
    if report['read_result'] > 0:
        data = bytes(u.mem_read(destination, report['read_result']))
        report.update(data_hex=data.hex(), data_sha256=hashlib.sha256(data).hexdigest())
except (UcError, AssertionError, RuntimeError) as error:
    report.update(error=str(error), pc=hex(u.reg_read(UC_ARM_REG_PC)), recent_pcs=list(trace))
report.update(block_reads=reads, console=messages, stubs=stub_calls)
a.output.write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps(report, indent=2))
raise SystemExit(1 if 'error' in report else 0)
