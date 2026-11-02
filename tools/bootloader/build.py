#!/usr/bin/env python3
"""Build Lean -> C -> ARM probes and G13/QEMU loaders; no device access."""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

sys.dont_write_bytecode = True

ROOT = Path(__file__).resolve().parents[2]
SRC = ROOT / "src/bootloader"
HOST = ROOT / "tools/bootloader"
OUT = ROOT / "assets/build/leanboot"
VENDOR_HASHES = {
    "Isa": "e67c855cb00e1e370c120b0c8418e930c8e57404a12fd9c28ddb82afa188f66c",
    "Print": "808e795e5817a5b380270a58a4e6f7f4231934ca3972ed6b535f52b0a1b7e0ea",
    "Dsl": "733ac456e705ea6c8a3cbcb6b3413bca51251898b0c32609de128c978c55ad4d",
}


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def run(*args, **kwargs):
    try:
        return subprocess.check_output([str(a) for a in args], cwd=kwargs.pop("cwd", ROOT),
                                       text=True, **kwargs)
    except subprocess.CalledProcessError as exc:
        print(exc.output, end="")
        raise


def build():
    OUT.mkdir(parents=True, exist_ok=True)
    oleans = OUT / "lean"
    (oleans / "ZrAsm").mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, LEAN_PATH=str(oleans))
    for name, expected in VENDOR_HASHES.items():
        path = HOST / f"vendor/ZrAsm/{name}.lean"
        assert digest(path) == expected, f"Vendored {name} changed; review provenance"
        run("lean", "-o", oleans / f"ZrAsm/{name}.olean", f"ZrAsm/{name}.lean",
            cwd=HOST / "vendor", env=env)
    assembly = run("lean", "--run", "Unsafe/Arm.lean", cwd=HOST, env=env)
    (OUT / "entry.S").write_text(assembly)
    prefix = run("lean", "--print-prefix").strip()
    flags = ["--target=armv7a-none-eabi", "-marm", "-mfloat-abi=soft"]
    objects = []
    for module in ("Types", "Flow", "Update/Policy", "Update/FlashPlan", "Unsafe/Effects", "Unsafe/Primitives", "MemoryAccess", "Unsafe/Ram", "Probe", "Memory", "Images", "Fdt", "Bootconfig", "Plan", "VerifiedBoot", "Os", "Cpu", "EntryCheck", "OsEntry", "Qemu", "Main", "Ethernet", "Ethernet/Descriptor", "EmacConfig", "Unsafe/Emac", "EthernetEntry", "BootPolicy", "BootJournal/Plan", "BootJournal", "Lz4", "Unsafe/Bootstrap", "BootstrapProbe", "BootBundle", "BootGate", "GateEntry", "Stage0", "KernelProbe", "Harness"):
        name = module.replace("/", "-").lower()
        olean = oleans / (module + ".olean")
        olean.parent.mkdir(parents=True, exist_ok=True)
        run("lean", "-Dcompiler.extract_closed=false", "-o", olean,
            "-c", OUT / (name + ".c"), module + ".lean",
            cwd=HOST if module == "Harness" else SRC, env=env)
        obj = OUT / (name + ".o")
        run("clang", *flags, "-O2", "-DNDEBUG", "-ffreestanding", "-fno-builtin",
            "-fno-stack-protector", "-fno-unwind-tables", "-fno-asynchronous-unwind-tables",
            "-fpie", "-ffunction-sections", "-fdata-sections", "-I" + prefix + "/include",
            "-c", OUT / (name + ".c"), "-o", obj)
        objects.append(obj)
    run("clang", *flags, "-c", OUT / "entry.S", "-o", OUT / "entry.o")
    elf = OUT / "g13-probe.elf"
    run("ld.lld", "--gc-sections", "--no-undefined", "-T", SRC / "ram.ld",
        OUT / "entry.o", *objects, "-o", elf)
    assert not run("llvm-nm", "--undefined-only", elf).strip()
    run("llvm-objcopy", "-O", "binary", elf, OUT / "g13-probe.bin")
    (OUT / "g13-probe.dis").write_text(run("llvm-objdump", "-d", elf))
    # Keep parser exports in a separate offline-test artifact. The UART probe
    # does not invoke them and has no arbitrary memory-read interface.
    core = OUT / "g13-loader-core.elf"
    run("ld.lld", "--gc-sections", "--no-undefined", "-T", SRC / "ram.ld",
        "--undefined=leanboot_read_le32", "--undefined=leanboot_boot_section",
        "--undefined=leanboot_vendor_section", "--undefined=leanboot_dtb_section",
        "--undefined=leanboot_copy", "--undefined=leanboot_fdt_validate", "--undefined=leanboot_fdt_patch",
        "--undefined=leanboot_fdt_reserve",
        "--undefined=leanboot_bootconfig_append", "--undefined=leanboot_os_prepare",
        "--undefined=leanboot_read_sector", "--undefined=leanboot_write_journal",
        "--undefined=leanboot_clean_caches", "--undefined=leanboot_flush_miu", "--undefined=leanboot_enter_linux",
        "--undefined=leanboot_verified_boot_packet", "--undefined=leanboot_send_verified_boot",
        "--undefined=leanboot_policy_step", "--undefined=leanboot_journal_select",
        "--undefined=leanboot_journal_encode", "--undefined=leanboot_journal_decode", "--undefined=leanboot_journal_advance", "--undefined=leanboot_bundle_normal", "--undefined=leanboot_bundle_hash", "--undefined=leanboot_lz4_decode",
        OUT / "entry.o", *objects, "-o", core)
    assert not run("llvm-nm", "--undefined-only", core).strip()
    run("llvm-objcopy", "-O", "binary", core, OUT / "g13-loader-core.bin")
    (OUT / "g13-loader-core.dis").write_text(run("llvm-objdump", "-d", core))
    os_elf = OUT / "g13-os.elf"
    run("ld.lld", "--gc-sections", "--no-undefined", "-T", SRC / "os.ld",
        OUT / "entry.o", *objects, "-o", os_elf)
    run("llvm-objcopy", "-O", "binary", os_elf, OUT / "g13-os.bin")
    (OUT / "g13-os.dis").write_text(run("llvm-objdump", "-d", os_elf))
    qemu_elf = OUT / "qemu.elf"
    run("ld.lld", "--gc-sections", "--no-undefined", "-T", SRC / "qemu.ld",
        OUT / "entry.o", *objects, "-o", qemu_elf)
    run("llvm-objcopy", "-O", "binary", qemu_elf, OUT / "qemu.bin")
    (OUT / "qemu.dis").write_text(run("llvm-objdump", "-d", qemu_elf))
    eth_elf = OUT / "g13-ethernet.elf"
    run("ld.lld", "--gc-sections", "--no-undefined", "-T", SRC / "ethernet.ld",
        "--undefined=leanboot_ethernet_respond", OUT / "entry.o", *objects, "-o", eth_elf)
    run("llvm-objcopy", "-O", "binary", eth_elf, OUT / "g13-ethernet.bin")
    (OUT / "g13-ethernet.dis").write_text(run("llvm-objdump", "-d", eth_elf))
    bootstrap = OUT / "g13-bootstrap-probe.elf"
    run("ld.lld", "--gc-sections", "--no-undefined", "-T", SRC / "bootstrap.ld",
        OUT / "entry.o", *objects, "-o", bootstrap)
    run("llvm-objcopy", "-O", "binary", bootstrap, OUT / "g13-bootstrap-probe.bin")
    (OUT / "g13-bootstrap-probe.dis").write_text(run("llvm-objdump", "-d", bootstrap))
    kernel_probe = OUT / "g13-kernel-probe.elf"
    run("ld.lld", "--gc-sections", "--no-undefined", "-T", SRC / "kernel-probe.ld",
        OUT / "entry.o", *objects, "-o", kernel_probe)
    run("llvm-objcopy", "-O", "binary", kernel_probe, OUT / "g13-kernel-probe.bin")
    (OUT / "g13-kernel-probe.dis").write_text(run("llvm-objdump", "-d", kernel_probe))
    gate = OUT / "g13-gate.elf"
    run("ld.lld", "--gc-sections", "--no-undefined", "-T", SRC / "gate.ld",
        OUT / "entry.o", *objects, "-o", gate)
    run("llvm-objcopy", "-O", "binary", gate, OUT / "g13-gate.bin")
    (OUT / "g13-gate.dis").write_text(run("llvm-objdump", "-d", gate))
    stage0 = OUT / "g13-stage0.elf"
    run("ld.lld", "--gc-sections", "--no-undefined", "-T", SRC / "stage0.ld",
        OUT / "entry.o", *objects, "-o", stage0)
    run("llvm-objcopy", "-O", "binary", stage0, OUT / "g13-stage0.bin")
    report = {
        "target": "Lean ARM loaders: G13 RAM probe/core/OS and QEMU virt",
        "hardware_tested": False, "tv_load_address": None,
        "lean": run("lean", "--version").strip(),
        "clang": run("clang", "--version").splitlines()[0],
        "vendor_commit": "92afd549dd5404eab68b7d9e8b12cc1d1bc9d27e",
        "vendor_sha256": VENDOR_HASHES,
        "source_sha256": {str(p.relative_to(SRC)): digest(p) for p in sorted(SRC.rglob("*"))
                          if p.is_file() and "vendor" not in p.relative_to(SRC).parts},
        "host_source_sha256": {str(p.relative_to(HOST)): digest(p) for p in sorted(HOST.rglob("*"))
                               if p.is_file() and p.suffix in (".lean", ".py", ".txt")},
        "output_sha256": {name: digest(OUT / name) for name in
                          ("g13-probe.bin", "g13-probe.elf", "g13-loader-core.bin", "g13-loader-core.elf", "g13-os.bin", "g13-os.elf", "qemu.bin", "qemu.elf", "g13-ethernet.bin", "g13-ethernet.elf", "g13-gate.bin", "g13-gate.elf", "g13-stage0.bin", "g13-stage0.elf", "g13-bootstrap-probe.bin", "g13-bootstrap-probe.elf", "g13-kernel-probe.bin", "g13-kernel-probe.elf")},
        "binary_bytes": (OUT / "g13-probe.bin").stat().st_size,
        "os_binary_bytes": (OUT / "g13-os.bin").stat().st_size,
        "requirements": ["8-byte aligned valid caller stack", "ARMv7-A ARM mode",
                         "UART mapping and safe RAM placement verified on live TV",
                         "loaded code made instruction-cache coherent before entry"],
    }
    (OUT / "build.json").write_text(json.dumps(report, indent=2) + "\n")
    from audit import audit
    audit()
    return report


if __name__ == "__main__":
    report = build()
    print(f"Built {report['binary_bytes']}-byte Lean/ZrAsm probe in {OUT}")
    print(f"Built {report['os_binary_bytes']}-byte G13 OS loader; current build is not hardware-tested")
    print("Not installed; TV load address and cache state still require validation")
