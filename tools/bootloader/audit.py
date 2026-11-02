#!/usr/bin/env python3
"""Check the source trust boundary and linked ARM artifacts; no TV access."""
import hashlib
import json
from pathlib import Path
import re

from capstone import Cs, CS_ARCH_ARM, CS_MODE_ARM
from capstone.arm import ARM_OP_IMM
from elftools.elf.elffile import ELFFile

ROOT = Path(__file__).resolve().parents[2]
SRC = ROOT / "src/bootloader"
HOST = ROOT / "tools/bootloader"
OUT = ROOT / "assets/build/leanboot"

# Intentional review gate: expanding this list requires updating the contracts
# and behavioral tests. Includes all returns and address materialization.
PRIMITIVES = {
    "leanboot_call_network": [("push", "{r4, lr}"), ("mov", "r4, sp"), ("mov", "r3, r0"),
        ("mov", "r0, #0"), ("mcr", "p15, #0, r0, c7, c5, #0"),
        ("mcr", "p15, #0, r0, c7, c5, #6"), ("dsb", "sy"), ("isb", "sy"),
        ("mov", "r0, r1"), ("movw", "sp, #0"), ("movt", "sp, #0x2636"),
        ("blx", "r3"), ("mov", "sp, r4"), ("pop", "{r4, pc}")],
    "leanboot_write_sctlr": [("mcr", "p15, #0, r0, c1, c0, #0"), ("isb", "sy"), ("mov", "r0, #0"), ("bx", "lr")],
    "leanboot_image_probe_redirect": [("mov", "r3, #0x40000000")],
    "leanboot_bundle_start": [("movw", "sp, #0"), ("movt", "sp, #0x2634"), ("b", "#8")],
    "leanboot_gate_run": [("b", "#0")],
    "leanboot_kernel_probe_redirect": [("mov", "r3, #0x26000000")],
    "leanboot_kernel_probe_start": [("movw", "sp, #0"), ("movt", "sp, #0x2634"), ("b", "#8")],
    "leanboot_load8": [("ldrb", "r0, [r0]"), ("bx", "lr")],
    "leanboot_store8": [("strb", "r1, [r0]"), ("mov", "r0, #0"), ("bx", "lr")],
    "leanboot_load32": [("ldr", "r0, [r0]"), ("bx", "lr")],
    "leanboot_store32": [("str", "r1, [r0]"), ("mov", "r0, #0"), ("bx", "lr")],
    "leanboot_load16": [("ldrh", "r0, [r0]"), ("bx", "lr")],
    "leanboot_store16": [("strh", "r1, [r0]"), ("mov", "r0, #0"), ("bx", "lr")],
    "leanboot_read_sctlr": [("mrc", "p15, #0, r0, c1, c0, #0"), ("bx", "lr")],
    "leanboot_read_cpsr": [("mrs", "r0, apsr"), ("bx", "lr")],
    "leanboot_mask_interrupts": [("mrs", "r0, apsr"), ("cpsid", "if"), ("bx", "lr")],
    "leanboot_restore_interrupts": [("msr", "cpsr_c, r0"), ("mov", "r0, #0"), ("bx", "lr")],
    "leanboot_read_sp": [("mov", "r0, sp"), ("bx", "lr")],
    "leanboot_os_start": [("push", "{r4, lr}"), ("mov", "r4, sp"),
        ("movw", "sp, #0"), ("movt", "sp, #0x2634"), ("bl", "#0x10"),
        ("mov", "sp, r4"), ("pop", "{r4, pc}")],
    "leanboot_read_clidr": [("mrc", "p15, #1, r0, c0, c0, #1"), ("bx", "lr")],
    "leanboot_ethernet_start": [("push", "{r4, lr}"), ("mov", "r4, sp"),
        ("movw", "sp, #0"), ("movt", "sp, #0x2634"), ("bl", "#0x10"),
        ("mov", "sp, r4"), ("pop", "{r4, pc}")],
    "leanboot_read_ccsidr": [("mrc", "p15, #1, r0, c0, c0, #0"), ("bx", "lr")],
    "leanboot_select_cache": [("mcr", "p15, #2, r0, c0, c0, #0"), ("isb", "sy"), ("mov", "r0, #0"), ("bx", "lr")],
    "leanboot_clean_setway": [("mcr", "p15, #0, r0, c7, c14, #2"), ("mov", "r0, #0"), ("bx", "lr")],
    "leanboot_barrier": [("dsb", "sy"), ("isb", "sy"), ("mov", "r0, #0"), ("bx", "lr")],
    "leanboot_send_verified_boot": [("push", "{r4, r5, r6, r7, r8, sb, sl, fp, ip, lr}"),
        ("mov", "r1, r0"), ("movw", "r0, #0x585e"), ("movt", "r0, #0xb200"),
        ("smc", "#0"), ("pop", "{r4, r5, r6, r7, r8, sb, sl, fp, ip, pc}")],
    "leanboot_begin_transition": [("cpsid", "if"), ("dsb", "sy"), ("mcr", "p15, #0, r0, c1, c0, #0"),
        ("isb", "sy"), ("movw", "sp, #0"), ("movt", "sp, #0x2631"),
        ("mov", "r0, r1"), ("mov", "r1, r2"), ("b", "#0x20")],
    "leanboot_jump_linux": [("push", "{r0, r1}"), ("mov", "r0, #0"), ("mcr", "p15, #0, r0, c7, c5, #0"),
        ("mcr", "p15, #0, r0, c7, c5, #6"), ("dsb", "sy"), ("isb", "sy"),
        ("movw", "r0, #0xf"), ("movt", "r0, #0x8200"), ("smc", "#0"), ("cpsid", "if"),
        ("pop", "{r3, r4}"), ("mov", "r0, #0"), ("movw", "r1, #0xffff"), ("movt", "r1, #0xffff"),
        ("mov", "r2, r4"), ("bx", "r3")],
    "leanboot_qemu_start": [("movw", "sp, #0"), ("movt", "sp, #0x4641"), ("b", "#8")],
    "leanboot_qemu_transition": [("cpsid", "if"), ("dsb", "sy"), ("mcr", "p15, #0, r0, c1, c0, #0"),
        ("isb", "sy"), ("movw", "sp, #0"), ("movt", "sp, #0x4631"),
        ("mov", "r0, r1"), ("mov", "r1, r2"), ("b", "#0x20")],
    "leanboot_qemu_jump": [("mov", "r3, r0"), ("mov", "r2, r1"), ("mov", "r0, #0"), ("mcr", "p15, #0, r0, c7, c5, #0"),
        ("mcr", "p15, #0, r0, c7, c5, #6"), ("dsb", "sy"), ("isb", "sy"),
        ("movw", "r1, #0xffff"), ("movt", "r1, #0xffff"), ("bx", "r3")],

}


def audit():
    foreign = []
    core_modules = {'Types.lean', 'Flow.lean', 'MemoryAccess.lean', 'Memory.lean', 'Images.lean', 'Fdt.lean',
                    'Bootconfig.lean', 'VerifiedBoot.lean', 'Plan.lean', 'Os.lean', 'EntryCheck.lean',
                    'Ethernet.lean', 'Ethernet/Descriptor.lean', 'EmacConfig.lean', 'BootPolicy.lean', 'BootJournal.lean',
                    'BootJournal/Plan.lean', 'BootBundle.lean', 'Lz4.lean',
                    'Update/Policy.lean', 'Update/FlashPlan.lean', 'Update/Flash.lean', 'Update/Handoff.lean', 'Update/Access.lean', 'Update/Manifest.lean',
                    'Update/Install.lean', 'Update/Receiver.lean', 'Update/Server.lean',
                    'Update/Receiver/Plan.lean', 'Update/CryptoBounds.lean', 'NetworkBoot/Plan.lean', 'NetworkBoot/Runtime.lean', 'NetworkBoot/Receiver.lean'}
    # A parser must not acquire physical capabilities through another local
    # import. Audit the transitive import graph, not only tokens in its file.
    checked = set()
    def safe_imports(relative):
        if relative in checked:
            return
        checked.add(relative)
        path = SRC / relative
        content = path.read_text()
        assert not re.search(r'\b(?:Unsafe|BaseIO|IO|HardwareM|MemoryM)\b', content), relative
        for dependency in re.findall(r'^import\s+(\S+)', content, re.M):
            child = dependency.replace('.', '/') + '.lean'
            if (SRC / child).is_file():
                safe_imports(child)
    for relative in core_modules:
        safe_imports(relative)
    for path in [*SRC.rglob("*"), HOST / "Unsafe/Arm.lean", HOST / "Harness.lean"]:
        if not path.is_file():
            continue
        relative = path.relative_to(SRC) if path.is_relative_to(SRC) else path.relative_to(HOST)
        # The separate updater links pinned HACL* through this small C boundary.
        # Its own build audit checks linked dependencies and foreign imports.
        crypto_c = {'Unsafe/update/crypto.c', 'Unsafe/update/include/assert.h',
                    'Unsafe/update/include/inttypes.h', 'Unsafe/update/include/stdio.h',
                    'Unsafe/update/include/stdlib.h', 'Unsafe/update/include/string.h',
                    'Unsafe/update/include/sys/types.h'}
        assert path.suffix.lower() not in (".c", ".h", ".s", ".asm") or str(relative) in crypto_c, relative
        if path.suffix != ".lean":
            continue
        content = path.read_text()
        if str(relative) in core_modules:
            assert not re.search(r'\b(?:Unsafe|BaseIO|IO|HardwareM|MemoryM)\b', content), relative
        if str(relative) in ('Unsafe/Primitives.lean', 'Unsafe/Bootstrap.lean', 'Unsafe/UpdateCrypto.lean', 'Unsafe/Network.lean'):
            assert len(re.findall(r'@\[extern "[^"]+"\]\s*private opaque', content)) == len(re.findall(r'@\[extern', content))
        # Conservative token guard, including comments. Explicit unsafe Lean
        # implementations may only be introduced in the reviewed boundary.
        if relative.parts[0] != "Unsafe":
            assert not re.search(r"\b(?:extern|unsafe|implemented_by|axiom)\b|\bZrAsm\b", content), relative
        foreign += re.findall(r'@\[extern "([^"]+)"\]', content)
        if "ZrAsm" in content:
            assert path == HOST / "Unsafe/Arm.lean", relative
    assert sorted(foreign) == sorted((set(PRIMITIVES)-{'leanboot_qemu_start','leanboot_os_start','leanboot_ethernet_start','leanboot_kernel_probe_start','leanboot_kernel_probe_redirect','leanboot_image_probe_redirect','leanboot_bundle_start','leanboot_gate_run'}) | {'leanboot_update_signature', 'leanboot_update_digest'}), foreign
    decoder = Cs(CS_ARCH_ARM, CS_MODE_ARM)
    decoder.detail = True
    with (OUT / "entry.o").open("rb") as stream:
        elf = ELFFile(stream)
        actual = {}
        for section in elf.iter_sections():
            if section["sh_flags"] & 4 and section["sh_size"]:
                name = section.name.removeprefix(".text.")
                instructions = list(decoder.disasm(section.data(), 0))
                assert sum(i.size for i in instructions) == section["sh_size"]
                actual[name] = [(i.mnemonic, i.op_str) for i in instructions]
        assert actual == PRIMITIVES, actual

    artifacts = {}
    for name in ("g13-probe", "g13-loader-core", "g13-os", "qemu", "g13-ethernet", "g13-bootstrap-probe", "g13-kernel-probe", "g13-gate", "g13-stage0"):
        with (OUT / (name + ".elf")).open("rb") as stream:
            elf = ELFFile(stream)
            symbols = list(elf.get_section_by_name(".symtab").iter_symbols())
            assert elf["e_entry"] == 0
            entry = {"qemu": "leanboot_qemu_start", "g13-os": "leanboot_os_start",
                     "g13-ethernet": "leanboot_ethernet_start",
                     "g13-bootstrap-probe": "leanboot_bootstrap_probe"}.get(name, "leanboot_main")
            if name == 'g13-kernel-probe': entry = 'leanboot_kernel_probe_start'
            if name == 'g13-gate': entry = 'leanboot_gate_run'
            if name == 'g13-stage0': entry = 'leanboot_bundle_start'
            assert next(s for s in symbols if s.name == entry)["st_value"] == 0
            assert not [s.name for s in symbols if s.name and s["st_shndx"] == "SHN_UNDEF"]
            assert not [s.name for s in elf.iter_sections() if s["sh_flags"] & 1 and s["sh_size"]]
            # ARM ELF mapping symbols distinguish instructions from literals.
            text = elf.get_section_by_name(".text")
            trusted = [(s['st_value'], s['st_value'] + 4*len(PRIMITIVES[s.name]))
                       for s in symbols if s.name in PRIMITIVES]
            markers = sorted((s["st_value"], s.name[1]) for s in symbols
                             if s.name.startswith(("$a", "$d", "$t")))
            for index, (start, kind) in enumerate(markers):
                if kind == "d":
                    continue
                assert kind == "a", "Unexpected instruction set"
                end = markers[index + 1][0] if index + 1 < len(markers) else text["sh_size"]
                decoded = list(decoder.disasm(text.data()[start:end], start))
                assert sum(i.size for i in decoded) == end-start, (name, start, end)
                for ins in decoded:
                    if ins.mnemonic in ("bl", "blx"):
                        assert ins.operands[0].type == ARM_OP_IMM, (name, ins.op_str)
                        assert 0 <= ins.operands[0].imm < text["sh_size"], (name, ins.op_str)
                    if ins.mnemonic in ("mcr", "mrc", "msr", "cps", "cpsid", "cpsie", "svc", "hvc", "smc"):
                        assert any(a <= ins.address < b for a,b in trusted), (name, ins.address)
            blob = (OUT / (name + ".bin")).read_bytes()
            artifacts[name] = {"bytes": len(blob), "sha256": hashlib.sha256(blob).hexdigest()}
    result = {"authored_arm_instructions": sum(map(len, PRIMITIVES.values())),
              "ffi": PRIMITIVES, "artifacts": artifacts,
              "scope": "regression audit, not formal verification"}
    (OUT / "boundary-audit.json").write_text(json.dumps(result, indent=2) + "\n")
    print(f"PASS: {len(foreign)} private FFI declarations / {result['authored_arm_instructions']} authored ARM instructions; pure core boundary and no runtime imports")
    return result


if __name__ == "__main__":
    audit()
