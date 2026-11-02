#!/usr/bin/env python3
"""Exercise compiled Lean parsers on exact backups and hostile inputs in ARM.

No TV access. The byte reader is real ARM code, not an emulator service stub.
Every machine memory access is checked against code, input and bounded stack.
"""
import hashlib
import json
from pathlib import Path
import random
import struct

from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_MEM_READ, UC_HOOK_MEM_WRITE
from unicorn.arm_const import (UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2,
    UC_ARM_REG_R3, UC_ARM_REG_R4, UC_ARM_REG_R5, UC_ARM_REG_R6, UC_ARM_REG_R7,
    UC_ARM_REG_R8, UC_ARM_REG_R9, UC_ARM_REG_R10, UC_ARM_REG_R11,
    UC_ARM_REG_PC, UC_ARM_REG_SP, UC_ARM_REG_LR, UC_ARM_REG_CPSR)

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets/build/leanboot"
BACKUP = ROOT / "assets/backups/g13-v013-stock-20261031"
FIXTURES = {
    "boot_a": "dc3154fd0767fcb6a40080798b154ada3cd38c3887f444b20eaf80d8cf27ab43",
    "init_boot_a": "ba21e944050398f71b9d137f4c569e930dd912f94aa50d44392aa94f61dfa0b9",
    "vendor_boot_a": "8bea2a7c9a392ae73cf9269f2c530aa9f50bf36a521437b69a5e89430a07d458",
}
INPUT, STACK, RETURN = 0x10000000, 0x70000000, 0x7FFF0000
SP = STACK + 0x8000
ARGS = (UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3)
SAVED = (UC_ARM_REG_R4, UC_ARM_REG_R5, UC_ARM_REG_R6, UC_ARM_REG_R7,
         UC_ARM_REG_R8, UC_ARM_REG_R9, UC_ARM_REG_R10, UC_ARM_REG_R11)
ERROR = 0xFFFFFFFF00000000


def up(n, page):
    return ((n + page - 1) // page) * page


def reference(data, size, kind, selector=0, role=0):
    """Independent host format decoder; Python integers do not overflow."""
    def word(at, endian="<"):
        assert at + 4 <= size
        return struct.unpack_from(endian + "I", data, at)[0]
    if size < {"boot": 1584, "vendor": 2128, "dtb": 64}[kind]:
        return ERROR | 1
    if kind == "boot":
        if data[:8] != b"ANDROID!": return ERROR | 2
        if word(40) != 4: return ERROR | 3
        if word(20) != 1584 or any(word(p) for p in (24, 28, 32, 36)): return ERROR | 4
        k, r, sig = word(8), word(12), word(1580)
        if (role == 0 and (not k or r)) or (role == 1 and (k or not r)) or role not in (0, 1):
            return ERROR | 6
        offsets = (4096, 4096 + up(k, 4096), 4096 + up(k, 4096) + up(r, 4096))
        lengths = (k, r, sig)
        if offsets[2] + sig > size: return ERROR | 5
    elif kind == "vendor":
        if data[:8] != b"VNDRBOOT": return ERROR | 2
        if word(8) != 4 or word(12) != 2048: return ERROR | 3
        if word(2096) != 2128: return ERROR | 4
        r, d, t, c, e, b = [word(p) for p in (24, 2100, 2112, 2116, 2120, 2124)]
        if not r or not d: return ERROR | 4
        if (t, c, e) != (108, 1, 108): return ERROR | 8
        dtb = 4096 + up(r, 2048)
        table = dtb + up(d, 2048)
        config = table + up(t, 2048)
        if config + b > size: return ERROR | 5
        if tuple(word(table + i) for i in (0, 4, 8)) != (r, 0, 1): return ERROR | 8
        if any(word(table + i) for i in range(44, 108, 4)): return ERROR | 8
        offsets, lengths = (4096, dtb, config, table), (r, d, b, t)
    else:
        h = struct.unpack_from(">8I", data)
        if h[0] != 0xD7B7AB1E: return ERROR | 2
        if h[2:] != (32, 32, 1, 32, 2048, 0): return ERROR | 3
        length, offset, *ids = struct.unpack_from(">8I", data, 32)
        if h[1] > size or h[1] < 104 or offset != 64 or length < 40 or offset + length != h[1]:
            return ERROR | 5
        if any(ids): return ERROR | 3
        if tuple(word(p, ">") for p in (64, 68)) != (0xD00DFEED, length): return ERROR | 9
        return offset << 32 | length
    if selector >= len(offsets): return ERROR | 7
    return offsets[selector] << 32 | lengths[selector]


class Machine:
    def __init__(self, codebase=0x01000000):
        self.base = codebase
        self.code = (OUT / "g13-loader-core.bin").read_bytes()
        self.uc = Uc(UC_ARCH_ARM, UC_MODE_ARM)
        self.uc.mem_map(codebase, up(len(self.code), 4096))
        self.uc.mem_write(codebase, self.code)
        self.uc.mem_map(INPUT, 0x4000000)
        self.uc.mem_map(STACK, 0x10000)
        self.uc.mem_map(RETURN, 4096)
        with (OUT / "g13-loader-core.elf").open("rb") as stream:
            elf = ELFFile(stream)
            self.symbols = {s.name: codebase + s["st_value"]
                            for s in elf.get_section_by_name(".symtab").iter_symbols()}
        self.calls, self.max_stack = 0, 0
        self.uc.hook_add(UC_HOOK_MEM_READ, self.read)
        self.uc.hook_add(UC_HOOK_MEM_WRITE, self.write)

    def read(self, emu, access, address, size, value, opaque):
        if SP - 512 <= address < address + size <= SP:
            return
        if self.base <= address < address + size <= self.base + len(self.code):
            return
        assert self.input_base <= address < address + size <= self.input_base + self.input_size, (
            "out-of-bounds load", hex(address), size, self.input_size)
        self.input_reads += 1

    def write(self, emu, access, address, size, value, opaque):
        assert SP - 512 <= address < address + size <= SP, ("unexpected write", hex(address), size)
        self.max_stack = max(self.max_stack, SP - address)

    def load(self, data, misalign=0):
        self.input_base = INPUT + misalign
        self.uc.mem_write(self.input_base, bytes(data))

    def call(self, symbol, size, *extra, base=None):
        self.input_size, self.input_reads = size, 0
        base = self.input_base if base is None else base
        for reg, value in zip(ARGS, (base, size, *extra)):
            self.uc.reg_write(reg, value)
        self.uc.reg_write(UC_ARM_REG_CPSR, 0x600001D3)
        self.uc.reg_write(UC_ARM_REG_SP, SP)
        self.uc.reg_write(UC_ARM_REG_LR, RETURN)
        for i, reg in enumerate(SAVED): self.uc.reg_write(reg, 0x12340000 + i)
        self.uc.emu_start(self.symbols[symbol], RETURN, count=40000)
        assert self.uc.reg_read(UC_ARM_REG_PC) == RETURN, "Parser did not return"
        assert self.uc.reg_read(UC_ARM_REG_SP) == SP
        assert self.uc.reg_read(UC_ARM_REG_CPSR) & 0x3FF == 0x1D3
        assert [self.uc.reg_read(r) for r in SAVED] == [0x12340000 + i for i in range(8)]
        self.calls += 1
        return self.uc.reg_read(UC_ARM_REG_R1) << 32 | self.uc.reg_read(UC_ARM_REG_R0)


def main():
    fixtures = {}
    for name, expected in FIXTURES.items():
        data = (BACKUP / (name + ".img")).read_bytes()
        assert hashlib.sha256(data).hexdigest() == expected, name
        fixtures[name] = data
    vendor = fixtures["vendor_boot_a"]
    # Independently known byte offsets and sizes from the exact image layout.
    dtb = vendor[18132992:18132992 + 42446]
    cases = [("boot_a", "boot", 0), ("init_boot_a", "boot", 1),
             ("vendor_boot_a", "vendor", 0), ("dtb", "dtb", 0)]
    fixtures["dtb"] = dtb
    machine = Machine()
    rng = random.Random(0x134)
    valid = {}
    for name, kind, role in cases:
        data = bytearray(fixtures[name])
        machine.load(data, 3)  # All firmware field loads must tolerate unaligned input.
        symbol = "leanboot_" + kind + "_section"
        fields = 3 if kind == "boot" else 4 if kind == "vendor" else 1

        def compare(size, selector=0):
            extra = (role, selector) if kind == "boot" else (selector,) if kind == "vendor" else ()
            result = machine.call(symbol, size, *extra)
            expected = reference(data, size, kind, selector, role)
            assert result == expected, (name, size, selector, hex(result), hex(expected))
            return result

        valid[name] = [compare(len(data), s) for s in range(fields)]
        if kind != "dtb": compare(len(data), fields)
        # Every byte of fixed-header truncation, then page and end-of-section edges.
        for size in range({"boot": 1584, "vendor": 2128, "dtb": 104}[kind]):
            compare(size)
        edges = {len(data) - 1, len(data), 4095, 4096}
        for packed in valid[name]:
            offset, length = packed >> 32, packed & 0xFFFFFFFF
            edges.update(max(0, p) for p in (offset - 1, offset, offset + length - 1, offset + length))
        for size in sorted(edges):
            if size <= len(data): compare(size)
        positions = ({"boot": [0, 4, 8, 12, 20, 24, 28, 32, 36, 40, 1580],
                      "vendor": [0, 4, 8, 12, 24, 2096, 2100, 2112, 2116, 2120, 2124,
                                 18176000, 18176004, 18176008, *range(18176044, 18176108, 4)],
                      "dtb": list(range(0, 72, 4))})[kind]
        for _ in range(300):
            at = rng.choice(positions)
            old = bytes(data[at:at + 4])
            value = rng.choice([0, 1, 0x7FFFFFFF, 0xFFFFFFFF, rng.getrandbits(32)])
            struct.pack_into(">I" if kind == "dtb" else "<I", data, at, value)
            machine.uc.mem_write(machine.input_base + at, bytes(data[at:at + 4]))
            compare(len(data), rng.randrange(fields + (kind != "dtb")))
            data[at:at + 4] = old
            machine.uc.mem_write(machine.input_base + at, old)
        extra = (role, 0) if kind == "boot" else (0,) if kind == "vendor" else ()
        assert machine.call(symbol, 0x1000, *extra, base=0xFFFFF800) == ERROR | 1
        assert machine.input_reads == 0

    # Checked byte reader: wrap, exact-end access, truncation, and invalid offsets.
    machine.load(b"\x78\x56\x34\x12\xff", 1)
    for size, offset, result in [(5, 0, 0x12345678), (5, 1, 0xFF123456),
                                 (3, 0, ERROR | 1), (5, 2, ERROR | 1),
                                 (5, 0xFFFFFFFE, ERROR | 1), (0, 0, ERROR | 1)]:
        assert machine.call("leanboot_read_le32", size, offset) == result
    # Independent code relocation, with exact fixture sections checked again.
    relocated = Machine(0x34567000)
    for name, kind, role in cases:
        relocated.load(fixtures[name])
        extra = (role, 0) if kind == "boot" else (0,) if kind == "vendor" else ()
        assert relocated.call("leanboot_" + kind + "_section", len(fixtures[name]), *extra) == valid[name][0]
    report = {"calls": machine.calls + relocated.calls,
              "max_stack_bytes": max(machine.max_stack, relocated.max_stack),
              "binary_sha256": hashlib.sha256(machine.code).hexdigest(),
              "fixture_sha256": FIXTURES,
              "sections": {k: [{"offset": p >> 32, "size": p & 0xFFFFFFFF} for p in v]
                           for k, v in valid.items()},
              "scope": "ARM execution; structural parsing, bounds, ABI, relocation; not authentication or Linux boot"}
    (OUT / "image-check.json").write_text(json.dumps(report, indent=2) + "\n")
    print(f"PASS: {report['calls']} parser/read executions; exact G13 images and malformed inputs")
    print(f"PASS: reads within input, writes only to stack (max {report['max_stack_bytes']} bytes), ABI and relocation")


if __name__ == "__main__":
    main()
