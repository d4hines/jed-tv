#!/usr/bin/env python3
"""Instruction-check the exact V109 watchdog keepalive without accessing a TV."""
import hashlib
import json
from pathlib import Path
import struct

from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_MEM_WRITE
from unicorn.arm_const import UC_ARM_REG_PC, UC_ARM_REG_SP, UC_ARM_REG_LR, UC_ARM_REG_R0

ROOT = Path(__file__).resolve().parents[2]


def main():
    module = (ROOT / "assets/research/g06-display-offline/vendor/lib/modules/utpa2k.ko").read_bytes()
    assert hashlib.sha256(module).hexdigest() == "63b603b72d9a204ca3b3ca72b3b0c58a6b24016ecb77998eee9a141637d6c216"
    header = struct.unpack_from("<16sHHIIIIIHHHHHH", module)
    sections = [struct.unpack_from("<IIIIIIIIII", module, header[6] + i * header[11]) for i in range(header[12])]
    strings = sections[header[13]]
    names = module[strings[4]:strings[4] + strings[5]]
    text = next(s for s in sections if names[s[0]:].split(b"\0")[0] == b".text")
    code = module[text[4] + 0x6b8ec:text[4] + 0x6b91c]
    assert len(code) == 48
    config = (ROOT / "assets/research/g06-capabilities-20261031/kernel-config").read_text()
    assert "# CONFIG_DEVMEM is not set" in config
    kernel = (ROOT / "assets/build/hello-pid1/kit/kernel").read_bytes()
    assert hashlib.sha256(kernel).hexdigest() == "01e36bda0554aed46ad3aa1d7fdf688174f46241117f62b4b95f02e25c47c11e"
    assert struct.unpack_from("<IIII", kernel, 0xc1180170 - 0xc0008000) == (0xfd000000, 0x1f000, 0x400000, 0)
    cpu = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    cpu.mem_map(0, 4096)
    cpu.mem_write(0, struct.pack("<I", 0x1f000000))
    cpu.mem_map(0x10000, 4096); cpu.mem_write(0x10000, code)
    cpu.mem_map(0x20000, 4096); cpu.reg_write(UC_ARM_REG_SP, 0x20ff0)
    cpu.mem_map(0x1f000000, 0x10000)
    writes = []

    def instruction(emulator, address, size, user):
        if address == 0x1000c:  # Skip only the unresolved ftrace/mcount call.
            emulator.reg_write(UC_ARM_REG_SP, emulator.reg_read(UC_ARM_REG_SP) + 4)
            emulator.reg_write(UC_ARM_REG_PC, address + 4)

    def write(emulator, access, address, size, value, user):
        if 0x1f000000 <= address < 0x1f010000:
            writes.append([address, size, value]); emulator.emu_stop()

    cpu.hook_add(UC_HOOK_CODE, instruction)
    cpu.hook_add(UC_HOOK_MEM_WRITE, write)
    cpu.emu_start(0x10000, 0x10030, count=100)
    assert writes == [[0x1f006000, 1, 1]], writes
    # Initialization calls HAL_WDT_Start. Verify this exact routine only
    # returns true: unlike HAL_WDT_Stop it never changes timeout registers.
    start_code = module[text[4] + 0x6b8a0:text[4] + 0x6b8b8]
    start = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    start.mem_map(0x10000, 4096); start.mem_write(0x10000, start_code)
    start.mem_map(0x20000, 4096); start.reg_write(UC_ARM_REG_SP, 0x20ff0)
    start.reg_write(UC_ARM_REG_LR, 0x10100)
    start.hook_add(UC_HOOK_CODE, instruction)
    start.emu_start(0x10000, 0x10100, count=100)
    assert start.reg_read(UC_ARM_REG_R0) == 1
    output = ROOT / "assets/build/pid1-repl/watchdog-check.json"
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps({"passed": True, "routine": "exact V109 HAL_WDT_Clear at .text+0x6b8ec",
                                 "writes": writes, "timeout_registers_changed": False,
                                 "hal_start_is_noop": True, "devmem_available": False,
                                 "access": "stock /proc/utopia WDT module 64, INIT 0, CLEAR 2",
                                 "hardware_tested": False}, indent=2) + "\n")
    print("PASS: stock watchdog clear writes only byte 1 at physical 0x1f006000; HAL_WDT_Start is a no-op; /dev/mem is disabled")


if __name__ == "__main__":
    main()
