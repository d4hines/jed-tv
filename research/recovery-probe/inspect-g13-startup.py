"""Read-only disassembly of exact G13/V013 backup regions (requires capstone).

Addresses are linked addresses, not a claim about the live relocation address.
The boot0 mapping applies to the readable loader body at file offset 0xe000.
Other boot0 regions include headers, padding, tables, and opaque payloads.
Linear decoding includes data; callers must establish function boundaries.
"""
import argparse
import hashlib
from pathlib import Path
import struct

from capstone import Cs, CS_ARCH_ARM, CS_MODE_ARM, CS_MODE_THUMB
from capstone.arm import ARM_OP_IMM, ARM_OP_MEM, ARM_OP_REG, ARM_REG_PC


ROOT = Path(__file__).resolve().parents[2]
BACKUP = ROOT / "assets/backups/g13-v013-stock-20261031"
IMAGES = {
    "boot0": ("mmcblk0boot0.img", 0x1FBF2000,
              "5ad07b30f93e8d36867d3c247907122672316634eba1e209b031419a3dfaac76"),
    "mboot": ("MBOOT.img", 0x273FDC00,
              "643058879b84d044dea58a4572bd9ade1cc3140f3d3357eeaabae53be32ec382"),
}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("image", choices=IMAGES)
    parser.add_argument("start", type=lambda v: int(v, 0), help="file offset")
    parser.add_argument("length", type=lambda v: int(v, 0))
    parser.add_argument("--thumb", action="store_true")
    args = parser.parse_args()
    name, delta, expected = IMAGES[args.image]
    blob = (BACKUP / name).read_bytes()
    digest = hashlib.sha256(blob).hexdigest()
    if digest != expected:
        raise SystemExit("Backup hash differs; these address mappings need review")
    if args.start < 0 or args.length <= 0 or args.start + args.length > len(blob):
        raise SystemExit("Requested region is outside the backup")

    def word(address):
        offset = address - delta
        if 0 <= offset <= len(blob) - 4:
            return struct.unpack_from("<I", blob, offset)[0]

    def string(address):
        offset = address - delta
        if 0 <= offset < len(blob):
            end = blob.find(b"\0", offset, offset + 256)
            if end >= 0:
                raw = blob[offset:end]
                if raw and all(32 <= c < 127 or c in (9, 10, 13) for c in raw):
                    return repr(raw.decode("ascii"))
        return ""

    engine = Cs(CS_ARCH_ARM, CS_MODE_THUMB if args.thumb else CS_MODE_ARM)
    engine.detail = True
    engine.skipdata = True
    known = {}
    print(f"# {name}; sha256={digest}; VA=file_offset+{delta:#x}")
    print("# Literal/string comments are decoding aids, not control-flow proof.")
    for ins in engine.disasm(blob[args.start:args.start + args.length],
                             args.start + delta):
        notes = []
        if ins.id:
            ops = ins.operands
            pc = ((ins.address + 4) & ~3) if args.thumb else ins.address + 8
            if (ins.mnemonic.startswith("ldr") and len(ops) == 2
                    and ops[0].type == ARM_OP_REG and ops[1].type == ARM_OP_MEM
                    and ops[1].mem.base == ARM_REG_PC and not ops[1].mem.index):
                value = word(pc + ops[1].mem.disp)
                if value is not None:
                    known[ops[0].reg] = (value, ins.address)
                    notes.append(f"literal {value:#x} {string(value)}")
            elif (ins.mnemonic.startswith("add") and len(ops) == 3
                  and ops[0].type == ARM_OP_REG and ops[1].type == ARM_OP_REG
                  and ops[1].reg == ARM_REG_PC and ops[2].type == ARM_OP_REG):
                value, at = known.get(ops[2].reg, (None, 0))
                if value is not None and ins.address - at < 48:
                    value = (pc + value) & 0xFFFFFFFF
                    notes.append(f"address {value:#x} {string(value)}")
                    known[ops[0].reg] = (value, ins.address)
            if ins.mnemonic in ("b", "bl", "blx") and ops[0].type == ARM_OP_IMM:
                target = ops[0].imm - delta
                if 0 <= target < len(blob):
                    notes.append(f"target file {target:#x}")
        suffix = " ; " + "; ".join(notes) if notes else ""
        print(f"{ins.address:#010x} [file {ins.address-delta:#08x}]: "
              f"{ins.mnemonic:8s} {ins.op_str:38s}{suffix}")


if __name__ == "__main__":
    main()
