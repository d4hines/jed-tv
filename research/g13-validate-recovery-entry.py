"""Offline validation of installed G13 V013 label-to-recovery argument path.

Requires Python with unicorn. Reads exact backup and previously extracted
recovery; writes only a report under assets. No USB or TV access.
"""
from pathlib import Path
import hashlib
import json
import struct
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE
from unicorn.arm_const import (
    UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3,
    UC_ARM_REG_R5, UC_ARM_REG_R6, UC_ARM_REG_R10, UC_ARM_REG_FP,
    UC_ARM_REG_SP, UC_ARM_REG_PC, UC_ARM_REG_LR,
)

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / 'assets/research/g13-magisk-v31.1-20261031'
binary = (ROOT / 'assets/backups/g13-v013-stock-20261031/MBOOT.img').read_bytes()
u = Uc(UC_ARCH_ARM, UC_MODE_ARM)
u.mem_map(0x27400000, 0x1000000)
u.mem_write(0x27400000, binary[0x2400:])
u.mem_map(0x10000000, 0x20000)
label = b'\n--fast\n   '
entry, output, serial, fp = 0x10001000, 0x10002000, 0x10003000, 0x10010000
u.mem_write(entry, label + b'\x08' + bytes(20))
u.mem_write(serial + 0x100, bytes.fromhex('f421cb6e'))
u.mem_write(fp - 0x2c, struct.pack('<I', serial + 0x100 - 3))


def cstr(address):
    result = bytearray()
    for i in range(4096):
        byte = u.mem_read(address + i, 1)[0]
        if not byte:
            return bytes(result)
        result.append(byte)
    raise RuntimeError('Unterminated string')


def ret(value):
    u.reg_write(UC_ARM_REG_R0, value & 0xffffffff)
    u.reg_write(UC_ARM_REG_PC, u.reg_read(UC_ARM_REG_LR))


HOOKS = {0x274e2a24: 'strncmp', 0x274e2968: 'strncpy',
         0x274e2aa4: 'strlen', 0x274e29b4: 'strncat'}


def hook(uc, pc, size, context):
    if pc not in HOOKS:
        return
    a = u.reg_read(UC_ARM_REG_R0)
    b = u.reg_read(UC_ARM_REG_R1)
    n = u.reg_read(UC_ARM_REG_R2)
    kind = HOOKS[pc]
    if kind == 'strncmp':
        x, y = cstr(a)[:n], cstr(b)[:n]
        ret((x > y) - (x < y))
    elif kind == 'strncpy':
        u.mem_write(a, cstr(b)[:n].ljust(n, b'\0'))
        ret(a)
    elif kind == 'strlen':
        ret(len(cstr(a)))
    elif kind == 'strncat':
        u.mem_write(a + len(cstr(a)), cstr(b)[:n] + b'\0')
        ret(a)


u.hook_add(UC_HOOK_CODE, hook)
for reg, value in [(UC_ARM_REG_R3, entry + 32), (UC_ARM_REG_R6, entry),
                   (UC_ARM_REG_R5, output), (UC_ARM_REG_R10, serial),
                   (UC_ARM_REG_FP, fp), (UC_ARM_REG_SP, fp - 0x2000)]:
    u.reg_write(reg, value)
u.emu_start(0x274478ac, 0x27447a18, count=10000)
actual = cstr(output)
assert actual == label[:8]
u.mem_write(fp - 0x114, actual + b'\0')
u.mem_write(fp - 0x214, b'6ecb-21f4\0')
u.reg_write(UC_ARM_REG_R5, 0x2768325b)
u.emu_start(0x274a1814, 0x274a1934, count=10000)
args = [a.decode() for a in cstr(fp - 0x9d4).split(b'\n') if a]
assert args == ['recovery', '--update_package=/mnt/usb/sda1/update_signed.zip',
                '--label=', '--fast', '--uuid=6ecb-21f4'], args

recovery = (BASE / 'stock-recovery/system/bin/recovery').read_bytes()
assert recovery[:5] == b'\x7fELF\x01'
phoff = struct.unpack_from('<I', recovery, 28)[0]
entry_size, count = struct.unpack_from('<HH', recovery, 42)
segments = []
for i in range(count):
    ph = struct.unpack_from('<8I', recovery, phoff + i * entry_size)
    if ph[0] == 1:
        segments.append((ph[1], ph[2], ph[4]))


def string(va):
    for offset, start, length in segments:
        if start <= va < start + length:
            offset += va - start
            data = recovery[offset:offset + 80].split(b'\0')[0]
            if data and all(c in (95, 45) or 97 <= c <= 122 for c in data):
                return data.decode()
    return None


def option(offset):
    if not 0 <= offset <= len(recovery) - 16:
        return None
    ptr, has_arg, flag, value = struct.unpack_from('<4I', recovery, offset)
    name = string(ptr)
    return name if name and has_arg <= 2 and flag == 0 and value < 256 else None


candidates = []
start = 0
while (at := recovery.find(b'fastboot\0', start)) >= 0:
    start = at + 1
    for file_offset, va, length in segments:
        if not file_offset <= at < file_offset + length:
            continue
        pointer = struct.pack('<I', va + at - file_offset)
        pos = 0
        while (pos := recovery.find(pointer, pos)) >= 0:
            if option(pos) == 'fastboot':
                first = pos
                while option(first - 16):
                    first -= 16
                names = []
                while (name := option(first)):
                    names.append(name)
                    first += 16
                if 'update_package' in names:
                    candidates.append(names)
            pos += 1
assert candidates, 'Recovery option table not found'
options = candidates[0]
assert [name for name in options if name.startswith('fast')] == ['fastboot']
result = {
    'firmware': 'installed G13 V013',
    'mboot_sha256': hashlib.sha256(binary).hexdigest(),
    'label_hex': label.hex(), 'copied_label_hex': actual.hex(),
    'bcb_arguments': args, 'recovery_long_options': options,
    'fast_is_unambiguous_fastboot_prefix': True,
    'emulated': ['actual MBOOT root-label copying',
                 'actual MBOOT BCB argument construction'],
    'limitations': ['FAT discovery and libc string calls stubbed',
                    'getopt_long dispatch not emulated',
                    'physical G13 recovery entry still untested'],
}
(BASE / 'recovery-usb/label-validation.json').write_text(
    json.dumps(result, indent=2) + '\n')
print(json.dumps(result, indent=2))
