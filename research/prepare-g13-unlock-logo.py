#!/usr/bin/env python3
"""Prepare and audit a G13 V013 boot-logo asset replacement, entirely offline.

Never writes to the TV. Requires Pillow and Unicorn. Inputs must be the exact
captured stock MBOOT and a baseline JPEG that fits the original asset slot.
"""
import argparse
import hashlib
import io
import json
from pathlib import Path
import struct

from PIL import Image
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE
from unicorn.arm_const import (UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2,
                               UC_ARM_REG_SP, UC_ARM_REG_LR, UC_ARM_REG_PC)

EXPECTED = '643058879b84d044dea58a4572bd9ade1cc3140f3d3357eeaabae53be32ec382'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def jpeg(data):
    image = Image.open(io.BytesIO(data))
    image.load()
    assert image.format == 'JPEG'
    assert not image.info.get('progressive'), 'Boot decoder requires baseline JPEG'
    return dict(size=list(image.size), mode=image.mode)


def emulate(image):
    """Run exact do_avbShowStatus + header getter; stub hardware/allocator I/O."""
    u = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    u.mem_map(0x27400000, 0x1000000)
    u.mem_write(0x27400000, image[0x2400:])
    u.mem_map(0x10000000, 0x200000)
    stop, argv, heap = 0x10000000, 0x10000100, 0x10100000
    reads, decoded, draws = [], [], []

    def word(addr):
        return struct.unpack('<I', u.mem_read(addr, 4))[0]

    def ret(value=0):
        u.reg_write(UC_ARM_REG_R0, value)
        u.reg_write(UC_ARM_REG_PC, u.reg_read(UC_ARM_REG_LR))

    def hook(uc, pc, size, data):
        if pc in (0x274c01cc, 0x2745e74c, 0x2743aa64, 0x2743b50c,
                  0x2743acd4, 0x274c08c4):
            ret()  # logging, panel ready, raw I/O setup/restore, free
        elif pc == 0x274c0b20:
            assert u.reg_read(UC_ARM_REG_R0) <= 0x10000
            ret(heap)
        elif pc == 0x2743b84c:
            dest, offset, length = [u.reg_read(r) for r in
                                   (UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2)]
            assert dest == heap and 0 <= offset < offset + length <= len(image)
            reads.append(dict(offset=offset, length=length))
            u.mem_write(dest, image[offset:offset + length])
            ret()
        elif pc == 0x27461728:
            ptr, length = u.reg_read(UC_ARM_REG_R0), u.reg_read(UC_ARM_REG_R1)
            data = bytes(u.mem_read(ptr, length))
            decoded.append(dict(sha256=sha(data), **jpeg(data)))
            ret(1)  # opaque decoded image; hardware decoder is not emulated
        elif pc == 0x274024a0:
            draws.append(u.reg_read(UC_ARM_REG_R2))
            ret()

    u.hook_add(UC_HOOK_CODE, hook)
    u.mem_write(word(0x276a3174 + 0xb74), bytes(4))
    u.mem_write(0x27ba2da4, b'\x01')  # header has been loaded from MBOOT
    u.mem_write(0x27ba2cec, image[:0xb4])
    for n, value in enumerate([b'avbShowStatus', b'0', b'0', b'1', b'1', b'2']):
        address = argv + 0x100 + n * 32
        u.mem_write(address, value + b'\0')
        u.mem_write(argv + n * 4, struct.pack('<I', address))
    u.reg_write(UC_ARM_REG_R0, 0)
    u.reg_write(UC_ARM_REG_R1, 0)
    u.reg_write(UC_ARM_REG_R2, 6)
    # r3 is the command's argv pointer.
    from unicorn.arm_const import UC_ARM_REG_R3
    u.reg_write(UC_ARM_REG_R3, argv)
    u.reg_write(UC_ARM_REG_SP, 0x100ff000)
    u.reg_write(UC_ARM_REG_LR, stop)
    u.emu_start(0x27403474, stop, count=1000000)
    assert u.reg_read(UC_ARM_REG_PC) == stop
    assert u.reg_read(UC_ARM_REG_R0) == 0
    assert len(reads) == len(decoded) == len(draws) == 1 and draws == [1]
    return dict(reads=reads, decoded=decoded, draws=draws,
                limit='Panel, raw-device setup and JPEG hardware are stubbed; Pillow validates JPEG.')


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('mboot', type=Path)
    p.add_argument('logo', type=Path)
    p.add_argument('output', type=Path)
    a = p.parse_args()
    original, logo = a.mboot.read_bytes(), a.logo.read_bytes()
    assert sha(original) == EXPECTED and len(original) == 5 * 1024 * 1024
    start, ram_start, ram_end, code_end = struct.unpack_from('<4I', original)
    code_size = ram_end - ram_start
    assert start == 0x2400 and start + code_size == code_end == 0x2bf860
    assert original[code_end-4:code_end] == b'UBOT'
    assert original[code_end:code_end+8] == b'SECURITY'
    assert struct.unpack_from('<II', original, code_end + 8) == (1, code_size)
    signature_end = code_end + 16 + 256
    offset, length = struct.unpack_from('<II', original, 0x27 * 4)
    assert (offset, length) == (0x2bfa00, 47903)
    assert signature_end <= offset and offset % 512 == 0
    assert logo[:3] == b'\xff\xd8\xff' and logo[-2:] == b'\xff\xd9'
    assert len(logo) <= length
    before_jpeg, after_jpeg = jpeg(original[offset:offset+length]), jpeg(logo)
    candidate = bytearray(original)
    candidate[offset:offset+length] = logo + bytes(length-len(logo))
    candidate = bytes(candidate)
    assert candidate[:offset] == original[:offset]
    assert candidate[offset+length:] == original[offset+length:]
    span = (length + 511) // 512 * 512
    a.output.mkdir(parents=True, exist_ok=False)
    artifacts = {'MBOOT-candidate.img': candidate,
                 'logo-sectors.before.bin': original[offset:offset+span],
                 'logo-sectors.after.bin': candidate[offset:offset+span]}
    for name, data in artifacts.items():
        (a.output / name).write_bytes(data)
    report = dict(original_sha256=sha(original), candidate_sha256=sha(candidate),
                  source_logo=str(a.logo), source_logo_sha256=sha(logo),
                  offset=offset, jpeg_slot_length=length, sector_count=span//512,
                  sector_start=offset//512, sector_size=512,
                  changed_bytes=sum(x != y for x, y in zip(original, candidate)),
                  code_start=start, code_end=code_end, signature_end=signature_end,
                  header_code_signature_unchanged=True,
                  other_assets_and_partition_tail_unchanged=True,
                  before_jpeg=before_jpeg, after_jpeg=after_jpeg,
                  original_emulation=emulate(original), candidate_emulation=emulate(candidate),
                  artifacts={name:sha(data) for name,data in artifacts.items()},
                  applied=False, rebooted=False,
                  limitation='No exact-device early-loader authentication or hardware JPEG execution test.')
    (a.output/'manifest.json').write_text(json.dumps(report, indent=2)+'\n')
    print(json.dumps(report, indent=2))


if __name__ == '__main__':
    main()
