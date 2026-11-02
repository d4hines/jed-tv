"""Offline audit of the exact V013 recovery handoff; requires Unicorn.

Executes MBOOT boot_mode_recovery, do_ReadKL through its AVB call, and the
bootconfig converter through environment output. Hardware, allocations and
environment storage are fixtures. Commands are recorded, never executed.
No device interfaces are opened. This is not a simulation of a complete boot.
"""
import argparse
import hashlib
import json
import lzma
from pathlib import Path
import struct
import zlib

from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE
from unicorn.arm_const import *

ROOT = Path(__file__).resolve().parents[2]
BACKUP = ROOT / 'assets/backups/g13-v013-stock-20261031'
PATCH = ROOT / 'assets/research/g13-magisk-v31.1-20261031'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def cpio(path):
    blob = path.read_bytes()
    offset, entries = 0, {}
    while blob[offset:offset + 6] in (b'070701', b'070702'):
        h = [int(blob[offset + 6 + i * 8:offset + 14 + i * 8], 16)
             for i in range(13)]
        name = blob[offset + 110:offset + 110 + h[11] - 1].decode()
        start = (offset + 110 + h[11] + 3) & ~3
        if name == 'TRAILER!!!':
            return entries
        entries[name] = {'mode': h[1], 'data': blob[start:start + h[6]]}
        offset = (start + h[6] + 3) & ~3
    raise ValueError('Incomplete newc archive')


def simulate(mboot, environment, recovery):
    u = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    u.mem_map(0x27400000, 0x1000000)
    u.mem_write(0x27400000, mboot[0x2400:])
    u.mem_map(0x10000000, 0x800000)
    stop, stack, heap = 0x10000000, 0x107ff000, 0x10010000
    env = dict(environment)
    state = {'commands': [], 'usb_stop_ports': [], 'env_changes': [],
             'requested_partition': None}

    def alloc(data):
        nonlocal heap
        ptr = heap
        heap += (len(data) + 31) & ~15
        assert heap < stack - 0x10000
        u.mem_write(ptr, data)
        return ptr

    def word(ptr, value=None):
        if value is not None:
            u.mem_write(ptr, struct.pack('<I', value))
        return struct.unpack('<I', u.mem_read(ptr, 4))[0]

    def cstr(ptr):
        for i in range(16384):
            if not u.mem_read(ptr + i, 1)[0]:
                return bytes(u.mem_read(ptr, i)).decode()
        raise ValueError('Unterminated string')

    def ret(value=0):
        u.reg_write(UC_ARM_REG_R0, value & 0xffffffff)
        u.reg_write(UC_ARM_REG_PC, u.reg_read(UC_ARM_REG_LR))

    def hook(uc, pc, size, data):
        r0, r1, r2 = [u.reg_read(r) for r in
                      (UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2)]
        if pc in (0x274c01cc, 0x274c016c, 0x274c08c4):
            ret()  # logging/free
        elif pc == 0x274c0b20:
            ret(alloc(bytes(r0)))
        elif pc == 0x274bc900:
            val = env.get(cstr(r0))
            ret(alloc(val.encode() + b'\0') if val is not None else 0)
        elif pc == 0x274bc4a0:
            key, val = cstr(r0), cstr(r1) if r1 else None
            state['env_changes'].append([key, val])
            if val is None:
                env.pop(key, None)
            else:
                env[key] = val
            ret()
        elif pc == 0x274ca264:
            state['commands'].append(cstr(r0))
            ret()  # never execute commands on the host or a device
        elif pc == 0x274cb940:
            state['usb_stop_ports'].append(r0)
            ret()
        elif pc in (0x2742d610, 0x2744132c):
            ret()  # hardware memory layout / CMA setup
        elif pc == 0x274ddb28:
            state['requested_partition'] = cstr(r0)
            u.emu_stop()  # do not pretend to verify or load partitions
        elif pc == 0x2742bf3c:
            ret(4)  # actual backup uses Android boot header version 4
        elif pc == 0x2742bfbc:
            ret(0x10008000)  # header fixture, unused before converter stop
        elif pc == 0x274afdd0:
            u.emu_stop()  # bootargs/bootconfig now emitted; no trailer emulation

    u.hook_add(UC_HOOK_CODE, hook)

    def call(entry, args):
        for reg, value in zip((UC_ARM_REG_R0, UC_ARM_REG_R1,
                               UC_ARM_REG_R2, UC_ARM_REG_R3), args):
            u.reg_write(reg, value)
        u.reg_write(UC_ARM_REG_SP, stack)
        u.reg_write(UC_ARM_REG_LR, stop)
        u.emu_start(entry, stop, count=1000000)
        return u.reg_read(UC_ARM_REG_PC)

    # Clear debug flags and a loader-state fixture through actual GOT pointers.
    word(word(0x276a3174 + 0xb74), 0)
    word(word(0x276a3174 + 0x668), 0)
    if recovery:
        assert call(0x27428220, []) == stop
        assert 'readKL recovery; bootKL' in state['commands']
        assert word(0x276b43d8) == 2

    argv = alloc(bytes(16))
    word(argv, alloc(b'readKL\0'))
    word(argv + 4, alloc(b'recovery\0'))
    assert call(0x27456ab4, [0, 0, 2 if recovery else 1, argv]) == 0x274ddb28
    state['bootargs_before_conversion'] = env['bootargs']
    assert call(0x274afc24, [0x10007000, 0, 0x10007004]) == 0x274afdd0
    state['kernel_cmdline_after_conversion'] = env['bootargs']
    state['android_bootconfig_after_conversion'] = env['bootconfig']
    expected = f'androidboot.force_normal_boot={0 if recovery else 1}'
    assert state['bootargs_before_conversion'].split().count(expected) == 1
    assert env['bootconfig'].splitlines().count(expected) == 1
    assert 'androidboot.force_normal_boot' not in env['bootargs']
    assert state['requested_partition'] == 'boot'
    return state


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    mboot = (BACKUP / 'MBOOT.img').read_bytes()
    assert sha(mboot) == '643058879b84d044dea58a4572bd9ade1cc3140f3d3357eeaabae53be32ec382'
    mpool = (BACKUP / 'MPOOL.img').read_bytes()
    assert zlib.crc32(mpool[0x2e0004:0x2f0000]) == struct.unpack_from('<I', mpool, 0x2e0000)[0]
    env = dict(x.decode().split('=', 1) for x in
               mpool[0x2e0004:0x2f0000].split(b'\0\0')[0].split(b'\0'))
    stock, patched, vendor = [cpio(PATCH / p) for p in (
        'stock-ramdisk.cpio', 'patched-ramdisk.cpio',
        'stock-vendor-boot/vendor_ramdisk/ramdisk.cpio')]
    restored = lzma.decompress(patched['.backup/init.xz']['data'])
    assert restored == stock['init']['data']
    vb = (BACKUP / 'vendor_boot_a.img').read_bytes()
    assert vb[:8] == b'VNDRBOOT'
    page = struct.unpack_from('<I', vb, 12)[0]
    align = lambda n: (n + page - 1) // page * page
    table_size, _, _, config_size = struct.unpack_from('<4I', vb, 2112)
    config_offset = (align(struct.unpack_from('<I', vb, 2096)[0])
                     + align(struct.unpack_from('<I', vb, 24)[0])
                     + align(struct.unpack_from('<I', vb, 2100)[0])
                     + align(table_size))
    report = {
        'scope': __doc__, 'mboot_sha256': sha(mboot),
        'input_sha256': {str(path.relative_to(ROOT)): sha(path.read_bytes())
                         for path in (BACKUP / 'MPOOL.img', BACKUP / 'vendor_boot_a.img',
                                      PATCH / 'stock-ramdisk.cpio', PATCH / 'patched-ramdisk.cpio',
                                      PATCH / 'stock-vendor-boot/vendor_ramdisk/ramdisk.cpio')},
        'stock_init_sha256': sha(restored),
        'magisk_backup_init_is_exact_stock': True,
        'vendor_root_init_symlink': vendor['init']['data'].decode(),
        'vendor_has_recovery_binary': 'system/bin/recovery' in vendor,
        'magisk_config': patched['.backup/.magisk']['data'].decode(),
        'vendor_bootconfig': vb[config_offset:config_offset + config_size].decode(),
        'normal': simulate(mboot, env, False),
        'recovery': simulate(mboot, env, True),
    }
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({k: report[k] for k in (
        'magisk_backup_init_is_exact_stock', 'vendor_root_init_symlink',
        'vendor_bootconfig')}))
    for case in ('normal', 'recovery'):
        r = report[case]
        print(case, json.dumps({'commands': r['commands'],
                               'usb_stop_ports': r['usb_stop_ports'],
                               'bootconfig': r['android_bootconfig_after_conversion']}))


if __name__ == '__main__':
    main()
