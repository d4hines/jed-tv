"""Offline execution of exact V013 A/B selection with controlled AVB results.

Only reads backup images and writes the requested JSON report. Storage is an
in-memory copy; verification results are fixtures, NOT verification of images.
Requires unicorn. No serial, USB, network or block-device access.
"""
import argparse
import hashlib
import json
from pathlib import Path
import struct
import zlib

from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE
from unicorn.arm_const import *

ROOT = Path(__file__).resolve().parents[2]
BACKUP = ROOT / 'assets/backups/g13-v013-stock-20261031'
MBOOT = (BACKUP / 'MBOOT.img').read_bytes()
HASH = hashlib.sha256(MBOOT).hexdigest()
assert HASH == '643058879b84d044dea58a4572bd9ade1cc3140f3d3357eeaabae53be32ec382'
with (BACKUP / 'misc.img').open('rb') as source:
    source.seek(2048)
    ORIGINAL = source.read(32)
assert ORIGINAL[4:8] == b'BCAB'
assert zlib.crc32(ORIGINAL[:28]) == struct.unpack_from('<I', ORIGINAL, 28)[0]


def slots(raw):
    return [{'priority': raw[i] & 15, 'tries': (raw[i] >> 4) & 7,
             'successful': bool(raw[i] & 128)} for i in (12, 14)]


def simulate(raw, verify_result=0, allow_error=True):
    u = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    u.mem_map(0x27400000, 0x1000000)
    u.mem_write(0x27400000, MBOOT[0x2400:])
    u.mem_map(0x10000000, 0x200000)
    stop, read, write = 0x10000000, 0x10000100, 0x10000200
    abops, ops, request, out, data = (0x10001000, 0x10001100,
                                    0x10002000, 0x10003000, 0x10004000)
    stack = 0x101ff000
    state = {'raw': bytes(raw), 'reads': 0, 'writes': [], 'verified': [],
             'commands': [], 'environment': []}

    def word(address, value=None):
        if value is not None:
            u.mem_write(address, struct.pack('<I', value & 0xffffffff))
        return struct.unpack('<I', u.mem_read(address, 4))[0]

    def cstr(address):
        for length in range(4096):
            if u.mem_read(address + length, 1)[0] == 0:
                return bytes(u.mem_read(address, length)).decode('ascii', 'replace')
        raise RuntimeError('Unterminated string')

    def ret(value=0):
        u.reg_write(UC_ARM_REG_R0, value & 0xffffffff)
        u.reg_write(UC_ARM_REG_PC, u.reg_read(UC_ARM_REG_LR))

    def hook(uc, pc, size, context):
        r0, r1, r2, r3 = [u.reg_read(r) for r in
                           (UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3)]
        sp = u.reg_read(UC_ARM_REG_SP)
        if pc in (read, write):
            assert cstr(r1) == 'misc' and r2 == 2048 and r3 == 0
            assert word(sp) == 32
            pointer = word(sp + 4)
            if pc == read:
                u.mem_write(pointer, state['raw'])
                word(word(sp + 8), 32)
                state['reads'] += 1
            else:
                state['raw'] = bytes(u.mem_read(pointer, 32))
                state['writes'].append(state['raw'].hex())
            ret()
        elif pc == 0x274d8aa8:  # avb_slot_verify: controlled fixture
            suffix = cstr(r2)
            state['verified'].append(suffix)
            u.mem_write(data, bytes(512))  # zero rollback indexes
            word(data, r2)
            word(word(sp + 4), data)
            ret(verify_result)
        elif pc in (0x274c01cc, 0x274c016c, 0x274d972c, 0x274d9718):
            ret()  # logging only
        elif pc == 0x274d99fc:
            ret(r0)  # logging basename
        elif pc == 0x274bc4a0:
            state['environment'].append([cstr(r0), cstr(r1) if r1 else None])
            ret()
        elif pc == 0x274c36c4:
            ret()  # saveenv: never access storage
        elif pc == 0x274ca264:
            state['commands'].append(cstr(r0))
            ret()  # record command only; never execute
        elif pc == 0x274d89b4:
            ret()  # free fixture result

    word(abops, ops)
    word(abops + 4, 0x274dbab8)  # exact read/validate/convert
    word(abops + 8, 0x274db9b0)  # exact convert/write
    word(ops + 12, read)
    word(ops + 16, write)
    word(request, request + 0x100)
    u.mem_write(request + 0x100, b'boot\0')
    word(stack, out)
    for reg, value in ((UC_ARM_REG_R0, abops), (UC_ARM_REG_R1, request),
                       (UC_ARM_REG_R2, int(allow_error)), (UC_ARM_REG_R3, 0),
                       (UC_ARM_REG_SP, stack), (UC_ARM_REG_LR, stop)):
        u.reg_write(reg, value)
    u.hook_add(UC_HOOK_CODE, hook)
    u.emu_start(0x274dbc0c, stop, count=1000000)
    assert u.reg_read(UC_ARM_REG_PC) == stop, 'Instruction budget exhausted'
    result = u.reg_read(UC_ARM_REG_R0)
    selected = cstr(word(word(out))) if word(out) else None
    return {'before': slots(raw), 'after': slots(state['raw']),
            'selected': selected, 'flow_result': result,
            'raw_after': state.pop('raw').hex(), **state}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    cases = {}
    raw = ORIGINAL
    cases['saved_state_repeated_successful_verification'] = []
    for _ in range(10):
        result = simulate(raw)
        cases['saved_state_repeated_successful_verification'].append(result)
        raw = bytes.fromhex(result['raw_after'])
    cases['saved_state_allowed_verification_error'] = simulate(ORIGINAL, 3, True)
    unconfirmed = bytearray(ORIGINAL)
    unconfirmed[12] &= 127
    struct.pack_into('<I', unconfirmed, 28, zlib.crc32(unconfirmed[:28]))
    cases['synthetic_a_unconfirmed'] = simulate(unconfirmed)
    disabled = bytearray(ORIGINAL)
    disabled[12] = 0
    struct.pack_into('<I', disabled, 28, zlib.crc32(disabled[:28]))
    cases['control_a_unbootable_b_selected'] = simulate(disabled)
    assert cases['control_a_unbootable_b_selected']['selected'] == '_b'
    cases['control_disallowed_verification_error'] = simulate(ORIGINAL, 3, False)
    assert cases['control_disallowed_verification_error']['selected'] is None
    for result in cases['saved_state_repeated_successful_verification']:
        assert result['selected'] == '_a' and not result['writes']
    assert cases['saved_state_allowed_verification_error']['selected'] == '_a'
    report = {'mboot_sha256': HASH, 'backup_misc_bcAB_hex': ORIGINAL.hex(),
              'scope': 'Original ARM A/B selection and BCAB conversion; in-memory storage; '
                       'AVB verification results and rollback indexes are controlled fixtures. '
                       'No actual image verification, Linux execution, or live state read.',
              'cases': cases}
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({k: ({'iterations': len(v), 'selected': v[-1]['selected'],
                          'after': v[-1]['after']} if isinstance(v, list)
                         else {'selected': v['selected'], 'after': v['after'],
                               'flow_result': v['flow_result'],
                               'metadata_writes': len(v['writes'])})
                      for k, v in cases.items()}, indent=2))


if __name__ == '__main__':
    main()
