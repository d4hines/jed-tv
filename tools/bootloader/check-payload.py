#!/usr/bin/env python3
"""Run final Lean/ZrAsm bytes at several synthetic addresses with a UART model.

This models the UART's status/data registers, not the TV's MMU/cache/hardware.
No TV connection is made. Run after build.py or check.py.
"""
import hashlib
import json
from pathlib import Path

from unicorn import (Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_MEM_READ,
                     UC_HOOK_MEM_WRITE)
from unicorn.arm_const import (
    UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R4, UC_ARM_REG_R5,
    UC_ARM_REG_R6, UC_ARM_REG_R7, UC_ARM_REG_R8, UC_ARM_REG_R9,
    UC_ARM_REG_R10, UC_ARM_REG_R11, UC_ARM_REG_SP, UC_ARM_REG_LR,
    UC_ARM_REG_PC, UC_ARM_REG_CPSR,
)

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets/build/leanboot"
UART = 0x1F201300
STACK = 0x70000000
SP = STACK + 0x8000
RETURN = 0x7FFF0000
SAVED = [UC_ARM_REG_R4, UC_ARM_REG_R5, UC_ARM_REG_R6, UC_ARM_REG_R7,
         UC_ARM_REG_R8, UC_ARM_REG_R9, UC_ARM_REG_R10, UC_ARM_REG_R11]
MARKER = b"LEANBOOT RAM\r\n"
LIMIT = 1048576


def exercise(blob, base, mode, cpsr):
    uc = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    uc.mem_map(base, 0x10000)
    uc.mem_write(base, blob)
    uc.mem_map(STACK, 0x10000)
    uc.mem_map(RETURN, 0x1000)
    uc.mem_map(UART & ~0xFFF, 0x1000)
    output = bytearray()
    total_reads, consecutive_reads, stack_low = 0, 0, SP

    def read(emu, access, address, size, value, data):
        nonlocal total_reads, consecutive_reads
        if address == UART + 0x28:
            assert size == 4
            total_reads += 1
            consecutive_reads += 1
            if mode == "stuck":
                status = 0
            elif len(output) == len(MARKER):
                status = 0x20 if mode == "no-drain" or consecutive_reads <= 5 else 0x60
            else:
                status = 0 if mode == "delayed" and consecutive_reads <= 7 else 0x20
            uc.mem_write(address, status.to_bytes(4, "little"))
        else:
            assert (base <= address < address + size <= base + len(blob) or
                    SP - 512 <= address < address + size <= SP), (hex(address), size)

    def write(emu, access, address, size, value, data):
        nonlocal consecutive_reads, stack_low
        if address == UART:
            assert size == 1 and len(output) < len(MARKER)
            output.append(value & 255)
            assert output == MARKER[:len(output)]
            consecutive_reads = 0
        else:
            assert SP - 512 <= address < address + size <= SP, (hex(address), size)
            stack_low = min(stack_low, address)

    uc.hook_add(UC_HOOK_MEM_READ, read)
    uc.hook_add(UC_HOOK_MEM_WRITE, write)
    uc.reg_write(UC_ARM_REG_CPSR, cpsr)
    uc.reg_write(UC_ARM_REG_SP, SP)
    uc.reg_write(UC_ARM_REG_LR, RETURN)
    uc.reg_write(UC_ARM_REG_R0, 1)  # Standard go argc/argv, unused by the probe.
    uc.reg_write(UC_ARM_REG_R1, 0)
    for i, reg in enumerate(SAVED):
        uc.reg_write(reg, 0xA5B60000 + i)
    # Bound every run, including a broken timeout implementation.
    uc.emu_start(base, RETURN, timeout=30_000_000, count=60_000_000)
    assert uc.reg_read(UC_ARM_REG_PC) == RETURN, "Did not return within emulator budget"
    assert uc.reg_read(UC_ARM_REG_SP) == SP
    assert [uc.reg_read(r) for r in SAVED] == [0xA5B60000 + i for i in range(8)]
    # Arithmetic flags may change under the ABI. Mode, interrupt masks, endian
    # and instruction set must not change across this ARM-to-ARM call.
    assert uc.reg_read(UC_ARM_REG_CPSR) & 0x3FF == cpsr & 0x3FF
    if mode in ("stuck", "no-drain"):
        assert uc.reg_read(UC_ARM_REG_R0) == 1
        assert consecutive_reads == LIMIT + 1
    else:
        assert uc.reg_read(UC_ARM_REG_R0) == 0
    assert output == (b"" if mode == "stuck" else MARKER)
    assert bytes(uc.mem_read(base, len(blob))) == blob
    return {"base": hex(base), "mode": mode, "cpsr": hex(cpsr),
            "uart_reads": total_reads, "uart_bytes": len(output),
            "return": uc.reg_read(UC_ARM_REG_R0), "stack_bytes": SP - stack_low}


def main():
    blob = (OUT / "g13-probe.bin").read_bytes()
    report = json.loads((OUT / "build.json").read_text())
    digest = hashlib.sha256(blob).hexdigest()
    assert digest == report["output_sha256"]["g13-probe.bin"]
    cases = []
    # These addresses are deliberately synthetic, not recommendations for TV RAM.
    for base in (0x01000000, 0x34567000, 0x56789000):
        for mode, cpsr in (("ready", 0x600001D3), ("delayed", 0x20000013)):
            cases.append(exercise(blob, base, mode, cpsr))
    for mode in ("stuck", "no-drain"):
        cases.append(exercise(blob, 0x01000000, mode, 0x600001D3))
    (OUT / "payload-check.json").write_text(json.dumps({
        "binary_sha256": digest, "binary_bytes": len(blob), "cases": cases,
        "scope": "ARM instruction execution with UART register model; no TV/cache validation",
    }, indent=2) + "\n")
    print(f"PASS: {len(cases)} complete payload executions; relocation, UART, timeout and return")
    print("PASS: writes only to caller stack and UART data byte; saved registers/control preserved")


if __name__ == "__main__":
    main()
