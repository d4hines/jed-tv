#!/usr/bin/env python3
"""Run V033 trigOAD's real ARM main with in-memory libc/environment mocks.

No real filesystem, environment or TV writes. This observes requested changes;
it does not emulate the storage implementation or the encrypted OAD package.
"""
import argparse
import hashlib
import io
import json
import lzma
from pathlib import Path
import struct

from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE
from unicorn.arm_const import UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3, UC_ARM_REG_SP, UC_ARM_REG_PC, UC_ARM_REG_LR


def exercise(binary, config, slot):
    elf = ELFFile(io.BytesIO(binary))
    u = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    base, stack, shims, stop = 0x1000000, 0x2000000, 0x3000000, 0x4000000
    for p in (base, stack, shims, stop):
        u.mem_map(p, 0x10000)
    for seg in elf.iter_segments():
        if seg['p_type'] == 'PT_LOAD':
            u.mem_write(base + seg['p_vaddr'], seg.data())
    debug = ELFFile(io.BytesIO(lzma.decompress(elf.get_section_by_name('.gnu_debugdata').data())))
    symbols = {s.name: base + s['st_value'] for s in debug.get_section_by_name('.symtab').iter_symbols()}
    handlers, imports = {}, {}

    def word(addr, value=None):
        if value is not None:
            u.mem_write(addr, struct.pack('<I', value & 0xffffffff))
        return struct.unpack('<I', u.mem_read(addr, 4))[0]

    for name in ('.rel.dyn', '.rel.plt'):
        rel = elf.get_section_by_name(name)
        syms = elf.get_section(rel['sh_link'])
        for r in rel.iter_relocations():
            addr, kind = base + r['r_offset'], r['r_info_type']
            if kind == 23:  # R_ARM_RELATIVE
                word(addr, base + word(addr))
            else:
                assert kind in (21, 22), kind
                sym = syms.get_symbol(r['r_info_sym']).name
                if sym not in imports:
                    imports[sym] = shims + 0x100 * len(imports)
                word(addr, imports[sym])
                handlers[imports[sym]] = sym
    for name in ('mbootenv_init', 'mbootenv_save', 'mbootenv_set', 'mbootenv_set2bootargs'):
        handlers[symbols[name] & ~1] = name
    lines = iter(config.splitlines(keepends=True))
    changes, bootargs, events = [], [], []

    def cstr(addr):
        if not addr:
            return ''
        buf = bytearray()
        while len(buf) < 8192:
            b = u.mem_read(addr + len(buf), 1)[0]
            if not b:
                return buf.decode()
            buf.append(b)
        raise ValueError('Unterminated string')

    def call(uc, address, size, opaque):
        name = handlers[address]
        a, b, c, d = [u.reg_read(r) for r in (UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3)]
        result = 0
        if name == 'fopen':
            assert cstr(a) == '/postinstall/etc/set_env' and cstr(b) == 'r'
            result = 0x1234
        elif name == 'fgets':
            assert c == 0x1234
            line = next(lines, b'')
            assert len(line) < b
            if line:
                u.mem_write(a, line + b'\0')
                result = a
        elif name in ('__strchr_chk', 'strchr'):
            s = cstr(a).encode() + b'\0'
            pos = s.find(bytes([b & 255]))
            result = a + pos if pos >= 0 else 0
        elif name in ('memcmp', 'strncmp'):
            x, y = bytes(u.mem_read(a, c)), bytes(u.mem_read(b, c))
            if name == 'strncmp':
                x, y = x.split(b'\0')[0], y.split(b'\0')[0]
            result = (x > y) - (x < y)
        elif name == 'property_get':
            assert cstr(a) == 'ro.boot.slot_suffix'
            u.mem_write(b, slot.encode() + b'\0')
            result = len(slot)
        elif name == 'memset':
            u.mem_write(a, bytes([b & 255]) * c)
            result = a
        elif name == 'mbootenv_set':
            changes.append([cstr(a), cstr(b) if b else None])
        elif name == 'mbootenv_set2bootargs':
            bootargs.append([cstr(a), cstr(b) if b else None])
        elif name in ('mbootenv_init', 'mbootenv_save'):
            events.append(name)
        elif name not in ('fprintf', 'fwrite', 'fclose', 'puts', 'printf'):
            raise ValueError('Unexpected import: ' + name)
        u.reg_write(UC_ARM_REG_R0, result & 0xffffffff)
        u.reg_write(UC_ARM_REG_PC, u.reg_read(UC_ARM_REG_LR))

    for addr in handlers:
        u.hook_add(UC_HOOK_CODE, call, begin=addr, end=addr)
    u.reg_write(UC_ARM_REG_SP, stack + 0xf000)
    u.reg_write(UC_ARM_REG_LR, stop)
    u.emu_start(symbols['main'], stop, count=1000000)
    assert u.reg_read(UC_ARM_REG_PC) == stop
    rc = u.reg_read(UC_ARM_REG_R0)
    assert rc == 0
    assert not {'devicestate', 'adb_enable', 'first_setup'} & {k for k, v in changes}
    return dict(slot=slot, returncode=rc, environment_changes=changes, bootarg_changes=bootargs,
                environment_events=events, ignored_protected_keys=['devicestate', 'adb_enable', 'first_setup'])


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('vendor', type=Path)
    p.add_argument('output', type=Path)
    a = p.parse_args()
    binary = (a.vendor / 'bin/trigOAD_fusion').read_bytes()
    config = (a.vendor / 'etc/set_env').read_bytes()
    report = dict(binary_sha256=hashlib.sha256(binary).hexdigest(),
                  config_sha256=hashlib.sha256(config).hexdigest(),
                  scope=__doc__, cases=[exercise(binary, config, slot) for slot in ('_a', '_b')])
    a.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
