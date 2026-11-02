#!/usr/bin/env python3
"""Execute the full compiled boot-image gate; only hardware I/O is modeled."""
import json
import struct
import zlib
import lz4.block
from pathlib import Path
from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_MEM_WRITE
from unicorn.arm_const import *
from os_profile import OUT, EVIDENCE, profile, decode_fdt
from importlib.machinery import SourceFileLoader

policy = SourceFileLoader('boot_policy_test', str(Path(__file__).with_name('check-boot-policy.py'))).load_module()
builder = SourceFileLoader('boot_image_builder', str(Path(__file__).with_name('build-boot-image.py'))).load_module()


def run(state=0, normal=True, corrupt=False, bad_plan=False, io_error=False, network=False, app_choice=0):
    p = profile()
    bundle = bytearray((OUT/'g13-boot-bundle.bin').read_bytes())
    header = 0x2000
    if corrupt: bundle[struct.unpack_from('<I', bundle, header+36)[0]+100] ^= 1
    if bad_plan:
        metadata = struct.unpack_from('<I', bundle, header+36)[0]
        size = struct.unpack_from('<I', bundle, header+40)[0]
        decoded = bytearray(lz4.block.decompress(bytes(bundle[metadata:metadata+size]),uncompressed_size=56896))
        decoded[53760+64:53760+68] = b'\xff'*4
        packed = lz4.block.compress(bytes(decoded),mode='high_compression',store_size=False)
        assert metadata+len(packed)<0x20000
        bundle[metadata:metadata+len(packed)] = packed
        struct.pack_into('<I', bundle, header+40, len(packed))
        struct.pack_into('<I', bundle, header+44, builder.word_hash(packed))
        struct.pack_into('<I', bundle, header+76, zlib.crc32(bundle[header:header+76]))
    u = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    u.mem_map(0x20000000, 0x9800000)
    u.mem_map(0x40000000, 0x1000000)
    u.mem_map(0x1f200000, 0x4000)
    u.mem_write(0x1f201328, struct.pack('<I', 0x60))
    u.mem_write(0x1f203140, struct.pack('<H', 0x1000))
    u.mem_write(0x20208000, bytes(bundle))
    fdt = (EVIDENCE/'live.dtb').read_bytes()
    props, _, _ = decode_fdt(fdt)
    old_end = props['/chosen/linux,initrd-end']
    config = (EVIDENCE/'bootconfig.txt').read_bytes()
    if not normal: config = config.replace(b'force_normal_boot = "1"', b'force_normal_boot = "0"')
    used = 18128651 + 2262498
    padded = ((used + len(config) + 4) & ~3) - used
    config += bytes(padded-len(config))
    end = 0x24000000 + used + len(config) + 20
    # The captured live FDT predates the larger native init_boot ramdisk.
    # Model the current MBOOT end, not that old fixture's shorter end.
    assert fdt.count(old_end) == 1
    u.mem_write(0x23000000, fdt.replace(old_end, struct.pack('>I', end)))
    original_fdt = bytes(u.mem_read(0x23000000, 65536))
    u.mem_write(end-20-len(config), config)
    u.mem_write(end-20, struct.pack('<II', len(config), sum(config))+b'#BOOTCONFIG\n')
    u.mem_write(0x24000000, p['vendor'][4096:4096+18128651]+p['init'][4096:4096+2262498])
    # Restore the bootconfig after populating the original compressed ramdisks.
    u.mem_write(end-20-len(config), config)
    u.mem_write(end-20, struct.pack('<II', len(config), sum(config))+b'#BOOTCONFIG\n')
    original_ramdisk = bytes(u.mem_read(0x24000000, 0x1400000))
    with (OUT/('network-gate.elf' if network else 'g13-gate.elf')).open('rb') as f:
        symbols = {s.name: s['st_value'] for s in ELFFile(f).get_section_by_name('.symtab').iter_symbols()}
    with (OUT/'g13-stage0.elf').open('rb') as f:
        stage0 = {s.name:s['st_value'] for s in ELFFile(f).get_section_by_name('.symtab').iter_symbols()}
    records = [policy.record(0, state)]*2
    writes = []
    messages = bytearray()
    result = {}
    network_calls = []
    def ret(uc, value):
        uc.reg_write(UC_ARM_REG_R0, value)
        uc.reg_write(UC_ARM_REG_PC, uc.reg_read(UC_ARM_REG_LR))
    def sctlr(uc, a, n, data): ret(uc, 0xc00818)
    def io(uc, a, n, write):
        sector, buffer = uc.reg_read(UC_ARM_REG_R0), uc.reg_read(UC_ARM_REG_R1)
        assert sector in (0x3217fe, 0x3217ff)
        if io_error:
            assert not write
            ret(uc, 53)
            return
        if write:
            records[sector-0x3217fe] = bytes(uc.mem_read(buffer, 512)); writes.append(sector)
        else: uc.mem_write(buffer, records[sector-0x3217fe])
        ret(uc, 0)
    def jump(uc, a, n, data):
        target, fdt = uc.reg_read(UC_ARM_REG_R0), uc.reg_read(UC_ARM_REG_R1)
        if target == 0x26000000:
            uc.reg_write(UC_ARM_REG_R0, 0); uc.reg_write(UC_ARM_REG_R1, 0xffffffff)
            uc.reg_write(UC_ARM_REG_R2, fdt); uc.reg_write(UC_ARM_REG_PC, target)
        else:
            assert target == 0x20208000
            result.update(target=target, fdt=fdt)
            uc.emu_stop()
    for base, table in ((0x20208000, stage0), (0x26000000, symbols)):
        for name, fn in [('leanboot_read_sctlr', sctlr), ('leanboot_qemu_jump', jump)]:
            a = base+table[name];u.hook_add(UC_HOOK_CODE, fn, begin=a, end=a)
    if network:
        def window(uc,a,n,data):
            network_calls.append(uc.reg_read(UC_ARM_REG_R0))
            ret(uc,0)
        a=0x26000000+symbols['l_LeanBoot_NetworkBoot_window___redArg']
        u.hook_add(UC_HOOK_CODE,window,begin=a,end=a)
        a=0x26000000+symbols['l_LeanBoot_Unsafe_Flash_choice___redArg']
        u.hook_add(UC_HOOK_CODE,lambda uc,*_:ret(uc,app_choice),begin=a,end=a)
    for name, write in [('l_LeanBoot_Unsafe_Bootstrap_readSector___redArg', False),
                        ('l_LeanBoot_Unsafe_Bootstrap_writeJournal___redArg', True)]:
        a = 0x26000000 + symbols[name]
        u.hook_add(UC_HOOK_CODE, io, write, begin=a, end=a)
    for a, value in [(0x28d8030c, 0xe1a0c00d), (0x28d80310, 0xe92ddef0),
                     (0x28d8031c, 0xe59f93e8), (0x28d7dfc4, 0xe1a0c00d), (0x28d7dfcc, 0xe92ddef0)]:
        u.mem_write(a, struct.pack('<I', value))
    def uart(uc, access, a, n, value, data):
        if a == 0x1f201300: messages.append(value & 255)
    u.hook_add(UC_HOOK_MEM_WRITE, uart, begin=0x1f201300, end=0x1f201300)
    u.reg_write(UC_ARM_REG_CPSR, 0x1d3)
    # Match the TV: r2 is an opaque legacy-tags pointer, not the DTB address.
    u.reg_write(UC_ARM_REG_R2, 0x284ddc00)
    u.emu_start(0x20208000, 0, count=1600000000)
    assert result, messages.decode(errors='replace')
    assert bytes(u.mem_read(0x20208000, len(p['boot'])-4096)) == p['boot'][4096:]
    lean = state != 2 and normal and not corrupt and not bad_plan and not io_error
    assert result['fdt'] == (0x23000000 if lean else 0x284ddc00), messages
    if lean:
        props, _, _ = decode_fdt(bytes(u.mem_read(0x23000000, 65536)))
        if network: assert props.pop('/chosen/dande,apps-bank') == [b'-\0', b'0\0', b'1\0'][app_choice]
        assert props == p['expected_properties']
        expected_ramdisks = p['vendor'][4096:4096+18128651]+p['init'][4096:4096+2262498]
        assert bytes(u.mem_read(0x24000000, len(expected_ramdisks))) == expected_ramdisks
    else:
        assert bytes(u.mem_read(0x24000000, 0x1400000)) == original_ramdisk
        if network:
            props = decode_fdt(bytes(u.mem_read(0x23000000, 65536)))[0]
            assert props.pop('/chosen/dande,apps-bank') == [b'-\0', b'0\0', b'1\0'][app_choice]
            assert props == decode_fdt(original_fdt)[0]
        else:
            assert bytes(u.mem_read(0x23000000, 65536)) == original_fdt
    assert len(writes) == int(normal and not io_error)
    assert len(network_calls)==int(network)
    return {'initial_state': state, 'normal': normal, 'corrupt_payload': corrupt, 'bad_plan': bad_plan,
            'io_error': io_error, 'lean': lean, 'network_window_calls':len(network_calls), 'messages': messages.decode()}


if __name__ == '__main__':
    cases = [run(), run(2), run(normal=False), run(corrupt=True), run(bad_plan=True), run(io_error=True)]
    (OUT/'boot-gate-test.json').write_text(json.dumps(cases, indent=2)+'\n')
    print('PASS: Lean entry, third-start fallback, recovery bypass, corrupt payload, preparation rollback and I/O failure')
