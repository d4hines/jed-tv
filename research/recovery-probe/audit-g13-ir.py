"""Audit the saved V013 IR database and execute its early IR dispatcher.

Read-only inputs; no device access. Unicorn runs the exact ARM dispatcher,
IR result reader, flash-data reader and CRC routine. Raw reads are served
from a RAM copy of MPOOL. IR pulse decoding/hardware are not simulated.
"""
import argparse
import hashlib
import json
from pathlib import Path
import struct
import zlib

from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE
from unicorn.arm_const import UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2
from unicorn.arm_const import UC_ARM_REG_SP, UC_ARM_REG_LR, UC_ARM_REG_PC

ROOT = Path(__file__).resolve().parents[2]
BACKUP = ROOT / 'assets/backups/g13-v013-stock-20261031'
BASE = 0x273fdc00


def digest(data):
    return hashlib.sha256(data).hexdigest()


def run(mboot, pool, protocol, header, command):
    u = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    u.mem_map(0x27400000, 0x1000000)
    u.mem_write(0x27400000, mboot[0x2400:])
    u.mem_map(0x10000000, 0x100000)
    stop = 0x10000000
    reads = []

    def word(addr):
        return struct.unpack('<I', u.mem_read(addr, 4))[0]

    def ret(value=0):
        u.reg_write(UC_ARM_REG_R0, value)
        u.reg_write(UC_ARM_REG_PC, u.reg_read(UC_ARM_REG_LR))

    def hook(uc, pc, size, data):
        if pc == 0x274c01cc:
            ret()  # printf
        elif pc == 0x2743b84c:
            dest, offset, length = [u.reg_read(r) for r in
                                   (UC_ARM_REG_R0, UC_ARM_REG_R1,
                                    UC_ARM_REG_R2)]
            assert 0 <= offset <= offset + length <= len(pool)
            reads.append([offset, length])
            u.mem_write(dest, bytes(pool[offset:offset + length]))
            ret()
        elif pc in (0x274bc4a0, 0x274c36c4, 0x274ca264):
            raise AssertionError('Unexpected environment mutation or command')

    u.hook_add(UC_HOOK_CODE, hook)
    u.mem_write(word(0x276a3174 + 0xb74), bytes(4))  # logging flags
    u.mem_write(word(0x276a3174 + 0xd24), bytes(pool[:0x160]))
    u.mem_write(0x27dab217, b'\x01')  # fixture: one decoded IR event ready
    u.mem_write(0x276a71c0, bytes([protocol]))
    u.mem_write(0x276a71bc, struct.pack('<I', header))
    u.mem_write(0x276a71c2, struct.pack('<H', command))
    u.mem_write(0x276a71b8, b'\0')
    u.reg_write(UC_ARM_REG_SP, 0x100ff000)
    u.reg_write(UC_ARM_REG_LR, stop)
    u.emu_start(0x27429b98, stop, count=1000000)
    assert u.reg_read(UC_ARM_REG_PC) == stop, 'Instruction bound reached'
    assert reads == [[0xe52c, 200]]
    return u.reg_read(UC_ARM_REG_R0)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    images = {name: (BACKUP / name).read_bytes()
              for name in ('MBOOT.img', 'MPOOL.img')}
    expected = dict(line.split()[::-1] for line in
                    (BACKUP / 'SHA256SUMS').read_text().splitlines())
    for name, data in images.items():
        assert digest(data) == expected[name], name
    mboot, pool = images['MBOOT.img'], images['MPOOL.img']
    table_crc = struct.unpack_from('<I', pool, 0x15c)[0]
    assert zlib.crc32(pool[:0x15c]) == table_crc
    records = {}
    for index, name in ((20, 'bootcheck'), (21, 'legacy_wakeup'),
                        (26, 'ir_ini_info')):
        off, length, crc = struct.unpack_from('<III', pool, index * 12)
        data = pool[off:off + length]
        assert zlib.crc32(data) == crc
        records[name] = dict(index=index, offset=off, size=length,
                             crc32=f'{crc:08x}', all_zero=not any(data),
                             hex=data.hex())
    assert records['bootcheck']['all_zero']
    candidates = [('Mstar recovery', 1, 0x807f, 0x1b),
                  ('Mstar USB OTA', 1, 0x807f, 0x07),
                  ('Mstar force upgrade', 1, 0x807f, 0x16),
                  ('Mstar bootloader upgrade', 1, 0x807f, 0x15),
                  ('Mstar wipe data', 1, 0x807f, 0x1f),
                  ('Mstar wipe cache', 1, 0x807f, 0x19),
                  ('TCL power', 7, 0xf, 0xd5),
                  ('TCL home', 7, 0xf, 0xf7)]
    tested = []
    for name, protocol, header, command in candidates:
        mode = run(mboot, pool, protocol, header, command)
        assert mode == 0
        tested.append(dict(name=name, protocol=protocol, header=header,
                           command=command, result_mode=mode))
    # Positive controls exist only in host RAM, never on the TV or USB.
    # Map 16 distinct synthetic command values into the first record.
    fixture = bytearray(pool)
    off, length, _ = struct.unpack_from('<III', fixture, 20 * 12)
    struct.pack_into('<B3xI16H', fixture, off, 1, 0x807f, *range(16))
    struct.pack_into('<I', fixture, 20 * 12 + 8,
                     zlib.crc32(fixture[off:off + length]))
    modes = [run(mboot, fixture, 1, 0x807f, i) for i in range(16)]
    assert modes == [2, 9, 4, 8, 10, 11] + [0] * 10
    assert modes[:6] == list(struct.unpack_from('<6I', mboot,
                                                0x27641f5c - BASE))
    # The zero record can match an artificial all-zero event. This does
    # not establish an on-air protocol that can produce such an event.
    zero_mode = run(mboot, pool, 0, 0, 0)
    assert zero_mode == 2
    report = dict(
        hashes={name: digest(data) for name, data in images.items()},
        database_crc32=f'{table_crc:08x}', records=records,
        saved_database_tests=tested, synthetic_record_slot_modes=modes,
        artificial_zero_event_mode=zero_mode,
        limits=['Decoded IR events are fixtures; no pulse decoder or TV test.',
                'Synthetic all-zero event has no demonstrated IR encoding.',
                'Original V013 backup; live state after V033 trial unreadable.',
                'This tests early boot selection, not standby wake support.'])
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(dict(output=str(args.output), saved_tests=len(tested),
                          ir_slot_modes=modes, result='passed')))


if __name__ == '__main__':
    main()
