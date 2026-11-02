#!/usr/bin/env python3
"""Emulate exact V109 ioctl_wrapper instructions; no vendor code runs natively."""
import hashlib
import json
from pathlib import Path
import struct

from unicorn import Uc, UC_ARCH_ARM, UC_MODE_THUMB, UC_HOOK_CODE
from unicorn.arm_const import UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_SP

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "assets/research/firmware-implications/lib/libmi3.so"
EXPECTED = "a10f65b026358c7acbbb5e8767c6b7a6901695f8c620b7e02abed6ea31b890a9"


def main():
    data = SOURCE.read_bytes()
    assert hashlib.sha256(data).hexdigest() == EXPECTED
    header = struct.unpack_from("<16sHHIIIIIHHHHHH", data)
    results = []
    for request in [0xc0101805, 0xc0101806, 0xc00c1200, 0xc00c1400]:
        cpu = Uc(UC_ARCH_ARM, UC_MODE_THUMB)
        cpu.mem_map(0, 0x1000000)
        for index in range(header[10]):
            entry = struct.unpack_from("<IIIIIIII", data, header[5] + index * header[9])
            if entry[0] == 1:
                cpu.mem_write(entry[2], data[entry[1]:entry[1] + entry[4]])
        # The imported stack-guard GOT slot is zero without relocation. Map
        # page zero for this isolated call; stop at ioctl before returning.
        cpu.mem_map(0x2000000, 0x10000)
        cpu.reg_write(UC_ARM_REG_SP, 0x200f000)
        cpu.reg_write(UC_ARM_REG_R0, 7)
        cpu.reg_write(UC_ARM_REG_R1, request)
        cpu.reg_write(UC_ARM_REG_R2, 0x2008000)
        captured = []

        def instruction(emulator, address, size, user):
            if address == 0xa2910:
                pointer = emulator.reg_read(UC_ARM_REG_R2)
                wire = bytes(emulator.mem_read(pointer, 16))
                expected = struct.pack("<H6xQ", (request >> 16) & 0x3fff, 0x2008000)
                assert wire == expected, (wire.hex(), expected.hex())
                assert emulator.reg_read(UC_ARM_REG_R0) == 7
                assert emulator.reg_read(UC_ARM_REG_R1) == request
                captured.append(wire.hex())
                emulator.emu_stop()

        cpu.hook_add(UC_HOOK_CODE, instruction)
        cpu.emu_start(0x5090d, 0, count=100)
        assert len(captured) == 1
        results.append({"request": hex(request), "actual_outer_wire_hex": captured[0],
                        "argument_bytes": (request >> 16) & 0x3fff})
    output = ROOT / "assets/research/g06-display-offline/ioctl-wrapper-check.json"
    output.write_text(json.dumps({"source_sha256": EXPECTED, "passed": True, "cases": results}, indent=2) + "\n")
    print("PASS: exact V109 wrapper constructs size/reserved/u64-pointer envelope for SYS GET/SET and DISP/OSD")


if __name__ == "__main__":
    main()
