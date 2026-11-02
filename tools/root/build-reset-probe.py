#!/usr/bin/env python3
"""Build and instruction-test a G06-only RAM watchdog reset probe. No TV access."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import struct
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_MEM_WRITE

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets/research/kexec/reset-probe"


def tool(name):
    found = shutil.which(name)
    if found:
        return found
    path = Path("/opt/homebrew/opt/llvm/bin") / name
    if not path.exists():
        raise RuntimeError("Missing LLVM tool " + name)
    return str(path)


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    kernel = (ROOT / "assets/build/hello-pid1/result/kernel").read_bytes()
    mboot = (ROOT / "assets/backups/v109-stock-20261031/MBOOT.img").read_bytes()
    assert hashlib.sha256(kernel).hexdigest() == "01e36bda0554aed46ad3aa1d7fdf688174f46241117f62b4b95f02e25c47c11e"
    assert hashlib.sha256(mboot).hexdigest() == "444f290af82a5da16e22ca310aedca598e82611db0358d15aed8db323fc67b9f"
    base = 0xc0008000
    def words(address, count):
        return struct.unpack_from("<" + "I" * count, kernel, address - base)
    assert words(0xc0943fa4, 7) == (0xe3060000, 0xe3a01001, 0xe34f0d00, 0xe3a02000,
                                     0xe1c011b0, 0xe1c021b4, 0xe5c01000)
    assert words(0xc1180170, 4) == (0xfd000000, 0x1f000, 0x400000, 0)
    assert words(0xc05aa23c, 3) == (0xe3000c00, 0xe3a01a01, 0xe3410f00)
    delta = 0x27400000 - 0x2400
    def bootword(address):
        return struct.unpack_from("<I", mboot, address - delta)[0]
    assert bootword(0x27420f38) == 0xe3000602 and bootword(0x27420f40) == 0xe30d3ead
    assert bootword(bootword(0x27680338 + 0x17c)) == 0x1f000000
    subprocess.run([tool("llvm-mc"), "-triple=armv7-none-eabi", "-filetype=obj",
                    str(ROOT / "research/probes/reset-probe.S"), "-o", str(OUT / "probe.o")], check=True)
    subprocess.run([tool("llvm-objcopy"), "--only-section=.text", "-O", "binary",
                    str(OUT / "probe.o"), str(OUT / "probe.bin")], check=True)
    code = (OUT / "probe.bin").read_bytes()
    assert len(code) == 64
    emulator = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    entry = 0x20208000
    emulator.mem_map(entry, 4096)
    emulator.mem_write(entry, code)
    emulator.mem_map(0x1f000000, 0x10000)
    writes = []

    def on_write(uc, access, address, size, value, user):
        writes.append([address, size, value])
        if len(writes) == 4:
            uc.emu_stop()

    emulator.hook_add(UC_HOOK_MEM_WRITE, on_write)
    emulator.emu_start(entry, entry + len(code), count=100)
    assert writes == [[0x1f000c04, 4, 0], [0x1f006010, 2, 1], [0x1f006014, 2, 0], [0x1f006000, 1, 1]], writes
    report = {
        "type": "G06-only RAM watchdog-reset payload, not Linux or a flash image",
        "sha256": hashlib.sha256(code).hexdigest(), "bytes": len(code),
        "source_sha256": hashlib.sha256((ROOT / "research/probes/reset-probe.S").read_bytes()).hexdigest(),
        "emulated_writes": writes,
        "evidence": {
            "stock_kernel_MHAL_PM_WdtRst": "0xc0943f94: count low/high then byte kick at virtual 0xfd006000",
            "stock_kernel_chip_io_desc": "0xc1180170: virtual 0xfd000000 -> PFN0x1f000, length0x400000",
            "stock_kernel_panic_flag_clear": "0xc05aa22c: maps physical0x1f000c00 and clears32bits at+4",
            "stock_MBOOT_panic_check": "0x27420f38: RIU reg0x602 ==0xdead =>kernel_panic; else watchdog test",
        },
        "hardware_verified": False,
        "expected_if_reached": "Immediate watchdog reset, followed by installed Android; no Linux PID1",
        "if_reset_fails": "Requires manual power cycle; no firmware or filesystem writes in payload",
    }
    (OUT / "manifest.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
