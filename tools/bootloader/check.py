#!/usr/bin/env python3
"""Build and exercise the Lean probe core as freestanding ARM32 machine code.

Run: nix develop .#bootloader -c python3 tools/bootloader/check.py
This separately exercises the policy function in an emulator-only ELF.
No TV, serial port, network or firmware partition is accessed.
"""
import hashlib
import json
from pathlib import Path
import random
import subprocess
import sys

sys.dont_write_bytecode = True
from build import build

from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_MEM_WRITE
from unicorn.arm_const import (
    UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R4, UC_ARM_REG_R5,
    UC_ARM_REG_R6, UC_ARM_REG_R7, UC_ARM_REG_R8, UC_ARM_REG_R9,
    UC_ARM_REG_R10, UC_ARM_REG_R11, UC_ARM_REG_SP, UC_ARM_REG_LR,
    UC_ARM_REG_PC,
)

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets/build/leanboot"
MARKER = b"LEANBOOT RAM\r\n"
LIMIT = 1048576
# Synthetic emulator addresses; never use these as a TV load-address recipe.
TEXT = 0x01000000
STACK = 0x02000000
SP = STACK + 0x8000
RETURN = 0x03000000
SAVED_REGS = [UC_ARM_REG_R4, UC_ARM_REG_R5, UC_ARM_REG_R6, UC_ARM_REG_R7,
              UC_ARM_REG_R8, UC_ARM_REG_R9, UC_ARM_REG_R10, UC_ARM_REG_R11]


def run(*args):
    return subprocess.check_output([str(a) for a in args], cwd=ROOT, text=True)


def reference(state, status):
    index, polls = state & 255, state >> 8
    if index > len(MARKER) or polls > LIMIT:
        return state, 0x202
    if index == len(MARKER) and status & 2:
        return state, 0x200
    if index < len(MARKER) and status & 1:
        return index + 1, 0x100 | MARKER[index]
    if polls == LIMIT:
        return state, 0x201
    return state + 256, 0


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    build()
    source = ROOT / "src/bootloader/Probe.lean"
    obj, binary = OUT / "probe.o", OUT / "probe-core.elf"
    # No libc, Lean runtime, startup code, compiler runtime or ignored imports.
    run("ld.lld", "--gc-sections", "--entry=leanboot_step",
        f"--section-start=.text={TEXT:#x}", obj, "-o", binary)
    assert not run("llvm-nm", "--undefined-only", binary).strip()
    (OUT / "probe-core.dis").write_text(run("llvm-objdump", "-d", binary))

    emu = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    emu.mem_map(TEXT - 0x10000, 0x30000)
    emu.mem_map(STACK, 0x10000)
    emu.mem_map(RETURN, 0x1000)
    with binary.open("rb") as handle:
        elf = ELFFile(handle)
        assert elf["e_machine"] == "EM_ARM" and elf.elfclass == 32
        entry = elf["e_entry"]
        allocated = []
        for section in elf.iter_sections():
            if not section["sh_flags"] & 2 or not section["sh_size"]:
                continue
            assert not section["sh_flags"] & 1, "Writable data/runtime initialization"
            address, size = section["sh_addr"], section["sh_size"]
            assert TEXT - 0x10000 <= address < address + size <= TEXT + 0x20000
            emu.mem_write(address, section.data())
            allocated.append({"name": section.name, "address": address, "size": size})

    def check_write(uc, access, address, size, value, user_data):
        assert SP - 512 <= address < address + size <= SP, (hex(address), size)

    emu.hook_add(UC_HOOK_MEM_WRITE, check_write)
    cases = 0

    def call(state, status):
        nonlocal cases
        emu.reg_write(UC_ARM_REG_R0, state)
        emu.reg_write(UC_ARM_REG_R1, status)
        for i, reg in enumerate(SAVED_REGS):
            emu.reg_write(reg, 0x12340000 + i)
        emu.reg_write(UC_ARM_REG_SP, SP)
        emu.reg_write(UC_ARM_REG_LR, RETURN)
        emu.emu_start(entry, RETURN, count=1000)
        assert emu.reg_read(UC_ARM_REG_PC) == RETURN, "Did not return"
        assert emu.reg_read(UC_ARM_REG_SP) == SP
        assert [emu.reg_read(r) for r in SAVED_REGS] == [0x12340000 + i for i in range(8)]
        result = (emu.reg_read(UC_ARM_REG_R1), emu.reg_read(UC_ARM_REG_R0))
        assert result == reference(state, status), (state, status, result)
        cases += 1
        return result

    # Every byte index, both timeout edges, corrupt states, ignored status bits.
    for index in range(256):
        for polls in (0, 1, LIMIT - 1, LIMIT, LIMIT + 1, 0xFFFFFF):
            for status in (0, 1, 2, 3, 0xFFFFFFFC, 0xFFFFFFFF):
                call((polls << 8) | index, status)

    # Whole-message traces: delays before each byte and before final drain.
    rng = random.Random(0)
    for _ in range(32):
        state, output = 0, bytearray()
        for _ in MARKER:
            for _ in range(rng.randrange(16)):
                state, action = call(state, 0)
                assert action == 0
            state, action = call(state, 1)
            output.append(action & 255)
        assert bytes(output) == MARKER
        state, action = call(state, 1)
        assert action == 0, "TX space alone does not mean the last byte is drained"
        state, action = call(state, 3)
        assert action == 0x200

    report = {
        "scope": "Isolated Lean policy function; whole-payload checks are separate",
        "lean": run("lean", "--version").strip(),
        "clang": run("clang", "--version").splitlines()[0],
        "source_sha256": hashlib.sha256(source.read_bytes()).hexdigest(),
        "elf_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
        "cases": cases, "marker": MARKER.decode(), "sections": allocated,
        "undefined_symbols": [], "writable_sections": [],
        "callee_saved_registers_preserved": True,
        "writes_confined_to_stack": True,
    }
    (OUT / "check.json").write_text(json.dumps(report, indent=2) + "\n")
    print(f"PASS: {cases} ARM32 executions; marker, timeout, drain and ABI checks")
    print("PASS: no runtime imports, writable globals or writes outside stack")
    print("Core function only; run check-payload.py for the complete RAM payload")


if __name__ == "__main__":
    main()
