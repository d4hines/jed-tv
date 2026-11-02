#!/usr/bin/env python3
"""Exercise the exact saved G13 V013 go handler with a stub RAM application.

Run with the bootloader Nix dev shell. Only reads the local firmware backup;
never opens UART or contacts the TV. This checks the linked-image call ABI,
not live relocation, cache coherency, a load address or board initialization.
"""
import hashlib
import json
from pathlib import Path
import struct

from capstone import Cs, CS_ARCH_ARM, CS_MODE_ARM
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_MEM_WRITE
from unicorn.arm_const import (
    UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3,
    UC_ARM_REG_R4, UC_ARM_REG_R5, UC_ARM_REG_R6, UC_ARM_REG_R7,
    UC_ARM_REG_R8, UC_ARM_REG_R9, UC_ARM_REG_R10, UC_ARM_REG_R11,
    UC_ARM_REG_SP, UC_ARM_REG_LR, UC_ARM_REG_PC,
)

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "assets/backups/g13-v013-stock-20261031/MBOOT.img"
OUT = ROOT / "assets/research/g13-leanboot"
SHA256 = "643058879b84d044dea58a4572bd9ade1cc3140f3d3357eeaabae53be32ec382"
BASE = 0x273FDC00
GO = 0x274B10FC
GO_EXEC = 0x274B10DC
PRINTF = 0x274C01CC
STRTOUL = 0x274E3408
# Arbitrary emulator memory, not proposed physical addresses for the TV.
APP = 0x01000000
SCRATCH = 0x02000000
SP = SCRATCH + 0x8000
ARGV = SCRATCH + 0x100
RETURN = 0x03000000
SAVED = [UC_ARM_REG_R4, UC_ARM_REG_R5, UC_ARM_REG_R6, UC_ARM_REG_R7,
         UC_ARM_REG_R8, UC_ARM_REG_R9, UC_ARM_REG_R10, UC_ARM_REG_R11]


def main():
    blob = SOURCE.read_bytes()
    assert hashlib.sha256(blob).hexdigest() == SHA256, "Wrong firmware; re-analyze first"
    OUT.mkdir(parents=True, exist_ok=True)
    decoder = Cs(CS_ARCH_ARM, CS_MODE_ARM)
    start, end = GO_EXEC - BASE, 0x274B1260 - BASE
    (OUT / "go.dis").write_text("\n".join(
        f"{i.address:#010x}: {i.mnemonic:8} {i.op_str}"
        for i in decoder.disasm(blob[start:end], BASE + start)) + "\n")
    candidates = []
    for offset in range(0x2B4000, 0x2B7000, 4):
        name, maxargs, repeat, handler, usage, help_, complete = struct.unpack_from("<7I", blob, offset)
        if handler == GO and blob[name - BASE:name - BASE + 3] == b"go\0":
            candidates.append({"table_offset": offset, "maxargs": maxargs,
                               "repeat": repeat, "handler": handler})
    assert len(candidates) == 1, candidates
    table = BASE + candidates[0]["table_offset"]
    cases = []
    for app_result in (0, 0x55):
        uc = Uc(UC_ARCH_ARM, UC_MODE_ARM)
        page = BASE & ~0xFFF
        uc.mem_map(page, (len(blob) + BASE - page + 0xFFF) & ~0xFFF)
        uc.mem_write(BASE, blob)
        uc.mem_map(APP, 0x1000)
        uc.mem_map(SCRATCH, 0x10000)
        uc.mem_map(RETURN, 0x1000)
        strings = [b"go", b"01000000", b"leanboot-probe"]
        pointers = [SCRATCH + 0x200 + i * 0x40 for i in range(len(strings))]
        for ptr, value in zip(pointers, strings):
            uc.mem_write(ptr, value + b"\0")
        uc.mem_write(ARGV, struct.pack("<4I", *pointers, 0))

        def cstring(ptr):
            return bytes(uc.mem_read(ptr, 160)).split(b"\0", 1)[0].decode("ascii")

        seen, printed = [], []

        def return_value(value):
            uc.reg_write(UC_ARM_REG_R0, value)
            uc.reg_write(UC_ARM_REG_PC, uc.reg_read(UC_ARM_REG_LR))

        def hook(emu, address, size, data):
            if address == PRINTF:
                printed.append(cstring(uc.reg_read(UC_ARM_REG_R0)))
                return_value(0)
            elif address == STRTOUL:
                assert cstring(uc.reg_read(UC_ARM_REG_R0)) == "01000000"
                assert uc.reg_read(UC_ARM_REG_R1) == 0
                assert uc.reg_read(UC_ARM_REG_R2) == 16
                return_value(APP)
            elif address == APP:
                argc, argv = uc.reg_read(UC_ARM_REG_R0), uc.reg_read(UC_ARM_REG_R1)
                assert argc == 2 and argv == ARGV + 4
                assert uc.reg_read(UC_ARM_REG_SP) % 8 == 0
                seen.append({"argc": argc, "argv": [cstring(x) for x in
                             struct.unpack("<2I", uc.mem_read(argv, 8))]})
                return_value(app_result)

        def writes(emu, access, address, size, value, data):
            assert SP - 512 <= address < address + size <= SP, hex(address)

        uc.hook_add(UC_HOOK_CODE, hook)
        uc.hook_add(UC_HOOK_MEM_WRITE, writes)
        uc.reg_write(UC_ARM_REG_R0, table)
        uc.reg_write(UC_ARM_REG_R1, 0)
        uc.reg_write(UC_ARM_REG_R2, len(strings))
        uc.reg_write(UC_ARM_REG_R3, ARGV)
        uc.reg_write(UC_ARM_REG_SP, SP)
        uc.reg_write(UC_ARM_REG_LR, RETURN)
        for i, reg in enumerate(SAVED):
            uc.reg_write(reg, 0x12340000 + i)
        uc.emu_start(GO, RETURN, count=1000)
        assert uc.reg_read(UC_ARM_REG_PC) == RETURN
        assert uc.reg_read(UC_ARM_REG_R0) == int(app_result != 0)
        assert uc.reg_read(UC_ARM_REG_SP) == SP
        assert [uc.reg_read(r) for r in SAVED] == [0x12340000 + i for i in range(8)]
        assert len(seen) == 1 and len(printed) == 2
        assert printed[0].startswith("## Starting application")
        assert printed[1].startswith("## Application terminated")
        cases.append({"application_return": app_result, "command_return": int(app_result != 0),
                      "application_call": seen[0], "printf_formats": printed,
                      "stack_and_callee_saved_restored": True})
    report = {"firmware_sha256": SHA256, "command": candidates[0], "cases": cases,
              "stubs": ["printf", "simple_strtoul", "RAM application"],
              "limitations": "Linked layout only; no live RAM/cache/relocation or peripheral model"}
    (OUT / "go-check.json").write_text(json.dumps(report, indent=2) + "\n")
    print("PASS: exact V013 go forwards argc/argv, returns status and restores caller registers")
    print("Linked-image emulation only; not a TV load-address or cache-coherency check")


if __name__ == "__main__":
    main()
