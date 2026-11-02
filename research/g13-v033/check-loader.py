#!/usr/bin/env python3
"""Exercise V033 bytes through the compiled Lean readers and virtual loader.

No TV access. The virtual loader's hardware handoff is intercepted in Unicorn:
this proves image preparation, not execution of the MediaTek kernel in QEMU.
Run after tools/bootloader/qemu.py --check, inside nix develop .#bootloader.
"""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import struct
import sys

from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_MEM_WRITE
from unicorn.arm_const import (
    UC_ARM_REG_CPSR, UC_ARM_REG_LR, UC_ARM_REG_PC, UC_ARM_REG_R0,
    UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_SP,
)

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/bootloader'))
from os_profile import decode_fdt, encode_recipe, OUT

PINS = {
    'boot': '0032a04d2359180b23600fb4ca28e55daf7d8eb9163998134ff234decf7ad077',
    'init_boot': '6fbbbd85998466c8eb84446f5e237fb7f231ed50f0dc3aa8d895e2c7da325ffb',
    'vendor_boot': 'c2885063121945508f7c4c6e2518bf2d6dc0e33ad99b7adceb3d3d053f0de702',
}


def sha(b):
    return hashlib.sha256(b).hexdigest()


def up(n, alignment):
    return (n + alignment - 1) // alignment * alignment


def symbols(name, base):
    with (OUT / (name + '.elf')).open('rb') as f:
        return {s.name: base + s['st_value'] for s in
                ELFFile(f).get_section_by_name('.symtab').iter_symbols()}


def parsers(images):
    spec = importlib.util.spec_from_file_location('image_checks', ROOT / 'tools/bootloader/check-images.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    m, report = module.Machine(), {}
    for name, kind, role, count in [('boot', 'boot', 0, 3), ('init_boot', 'boot', 1, 3),
                                   ('vendor_boot', 'vendor', 0, 4)]:
        data = images[name]
        m.load(data, 3)
        found = []
        for selector in range(count):
            args = (role, selector) if kind == 'boot' else (selector,)
            actual = m.call('leanboot_' + kind + '_section', len(data), *args)
            expected = module.reference(data, len(data), kind, selector, role)
            assert actual == expected and actual >> 32 != 0xffffffff
            found.append({'offset': actual >> 32, 'length': actual & 0xffffffff})
        report[name] = found
    dt = report['vendor_boot'][1]
    wrapper = images['vendor_boot'][dt['offset']:dt['offset'] + dt['length']]
    m.load(wrapper, 3)
    actual = m.call('leanboot_dtb_section', len(wrapper))
    assert actual == module.reference(wrapper, len(wrapper), 'dtb')
    report['dtb'] = {'offset': actual >> 32, 'length': actual & 0xffffffff}
    return report, wrapper[actual >> 32:(actual >> 32) + (actual & 0xffffffff)]


def prepare(images, sections, dtb, out):
    bsz = up(max(x['offset'] + x['length'] for x in sections['boot']), 4096)
    vsz = up(max(x['offset'] + x['length'] for x in sections['vendor_boot']), 2048)
    boot, vendor, generic = images['boot'][:bsz], images['vendor_boot'][:vsz], images['init_boot']
    config = b'source = "v033-offline-format-check"\n'
    vlen, ilen = sections['vendor_boot'][0]['length'], sections['init_boot'][1]['length']
    final = up(vlen + ilen + len(config) + 1, 4) + 20
    changes = [('/chosen/bootargs', b'v033.offline=1\0'),
               ('/chosen/linux,initrd-start', struct.pack('>I', 0x44000000)),
               ('/chosen/linux,initrd-end', struct.pack('>I', 0x44000000 + final))]
    recipe = encode_recipe(changes)
    control = struct.pack('<16I', 0x534f424c, 1, 0, 64, bsz, vsz, len(generic),
                          64, len(recipe), 64 + len(recipe), len(config), final, 0, 0, 0, 0)
    control += recipe + config
    code = (OUT / 'qemu.bin').read_bytes()
    syms = symbols('qemu', 0x46000000)
    u = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    for addr, size, data in [(0x40207000, up(len(boot), 4096), boot),
                             (0x43000000, 65536, b''), (0x44000000, 0x1400000, vendor),
                             (0x45400000, len(generic), generic),
                             (0x46000000, up(len(code), 4096), code),
                             (0x46200000, 65536, control), (0x46330000, 65536, b''),
                             (0x09000000, 4096, b'')]:
        u.mem_map(addr, size)
        if data:
            u.mem_write(addr, data)
    uart, commits = bytearray(), []

    def writes(uc, access, addr, size, value, opaque):
        assert any(lo <= addr and addr + size <= hi for lo, hi in
                   [(0x43000000, 0x43010000), (0x44000000, 0x45400000),
                    (0x46330000, 0x46340000), (0x09000000, 0x09000004)]), hex(addr)
        if addr == 0x09000000:
            uart.append(value & 255)

    def hardware(uc, addr, size, opaque):
        if addr == syms['leanboot_qemu_transition']:
            commits.append((u.reg_read(UC_ARM_REG_R1), u.reg_read(UC_ARM_REG_R2)))
            u.emu_stop()
            return
        u.reg_write(UC_ARM_REG_R0, 0)
        u.reg_write(UC_ARM_REG_PC, u.reg_read(UC_ARM_REG_LR))

    u.hook_add(UC_HOOK_MEM_WRITE, writes)
    for name in ['leanboot_read_sctlr', 'l_LeanBoot_Cpu_cleanCaches___redArg', 'leanboot_qemu_transition']:
        u.hook_add(UC_HOOK_CODE, hardware, begin=syms[name], end=syms[name])
    u.reg_write(UC_ARM_REG_CPSR, 0x1d3)
    u.reg_write(UC_ARM_REG_SP, 0x46340000)
    u.emu_start(syms['leanboot_qemu_main'], 0, timeout=120_000_000, count=500_000_000)
    assert commits == [(0x40208000, 0x43000000)], (commits, uart)
    assert bytes(u.mem_read(0x40207000, len(boot))) == boot
    assert bytes(u.mem_read(0x45400000, len(generic))) == generic
    assert bytes(u.mem_read(0x46200000, len(control))) == control
    assert bytes(u.mem_read(0x46000000, len(code))) == code
    raw_dtb = bytes(u.mem_read(0x43000000, 65536))
    expected, _, _ = decode_fdt(dtb)
    expected.update(changes)
    props, _, reserved = decode_fdt(raw_dtb)
    assert props == expected and reserved == [(0x44000000, final)]
    raw = vendor[4096:4096 + vlen] + generic[4096:4096 + ilen]
    pad = config + bytes(final - len(raw) - len(config) - 20)
    expected_ramdisk = raw + pad + struct.pack('<II', len(pad), sum(pad)) + b'#BOOTCONFIG\n'
    actual = bytes(u.mem_read(0x44000000, final))
    assert actual == expected_ramdisk
    (out / 'v033-virt-prepared.dtb').write_bytes(raw_dtb[:struct.unpack_from('>I', raw_dtb, 4)[0]])
    (out / 'v033-virt-prepared-initramfs.bin').write_bytes(actual)
    (out / 'v033-virt-control.bin').write_bytes(control)

    # The production entry must still reject V033 before touching any output.
    g = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    guard_code = (OUT / 'g13-loader-core.bin').read_bytes()
    for addr, n in [(0x26000000, up(len(guard_code), 4096)), (0x26200000, 65536),
                    (0x28000000, 65536), (0x27f00000, 4096)]:
        g.mem_map(addr, n)
    g.mem_write(0x26000000, guard_code)
    g.mem_write(0x26200000, control)
    g.reg_write(UC_ARM_REG_CPSR, 0x1d3)
    g.reg_write(UC_ARM_REG_SP, 0x28010000)
    g.reg_write(UC_ARM_REG_LR, 0x27f00000)

    def stack_only(uc, access, addr, size, value, opaque):
        assert 0x28000000 <= addr and addr + size <= 0x28010000

    g.hook_add(UC_HOOK_MEM_WRITE, stack_only)
    g.emu_start(symbols('g13-loader-core', 0x26000000)['leanboot_os_prepare'], 0x27f00000, count=100000)
    assert g.reg_read(UC_ARM_REG_PC) == 0x27f00000 and g.reg_read(UC_ARM_REG_R0) == 2
    return dict(kernel_unchanged=True, init_unchanged=True, fdt_properties=len(props),
                initramfs_bytes=final, initramfs_sha256=sha(actual),
                boot_input_bytes=bsz, vendor_input_bytes=vsz, uart=uart.decode(),
                production_guard_rejection=2, kernel_executed=False,
                hardware_primitives_intercepted=['SCTLR read', 'cache clean', 'terminal handoff'])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('images', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    images = {n: (args.images / (n + '.img')).read_bytes() for n in PINS}
    assert all(sha(images[n]) == digest for n, digest in PINS.items()), 'Not the pinned G13 V033 images'
    args.output.mkdir(parents=True, exist_ok=True)
    sections, dtb = parsers(images)
    report = {'image_sha256': PINS, 'sections': sections,
              'preparation': prepare(images, sections, dtb, args.output),
              'scope': 'ARM emulation of unchanged Lean virtual loader; no MediaTek kernel boot or hardware validation'}
    (args.output / 'loader-check.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
