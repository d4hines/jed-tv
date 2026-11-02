"""Exact G13 reference data for RAM-only OS experiments; no device access.

The captured running DTB is an oracle for board fixups, not the input DTB
that the loader consumes. The loader patches the original DTB in Lean.
"""
import hashlib
import json
from pathlib import Path
import struct

ROOT = Path(__file__).resolve().parents[2]
BACKUP = ROOT / "assets/backups/g13-v013-stock-20261031"
EVIDENCE = ROOT / "assets/research/g13-leanboot-os"
OUT = ROOT / "assets/build/leanboot"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def decode_fdt(data):
    header = struct.unpack_from(">10I", data)
    assert header[0] == 0xD00DFEED and header[1] <= len(data)
    strings = data[header[3]:header[3] + header[8]]
    pos, end, stack, nodes, properties = header[2], header[2] + header[9], [], [], {}
    while pos < end:
        token, = struct.unpack_from(">I", data, pos)
        pos += 4
        if token == 1:
            stop = data.index(0, pos, end)
            stack.append(data[pos:stop].decode())
            nodes.append("/".join(stack) or "/")
            pos = (stop + 4) & ~3
        elif token == 2:
            stack.pop()
        elif token == 3:
            length, nameoff = struct.unpack_from(">II", data, pos)
            pos += 8
            name = strings[nameoff:strings.index(0, nameoff)].decode()
            path = "/".join(stack) + "/" + name
            assert path not in properties
            properties[path] = data[pos:pos + length]
            pos = (pos + length + 3) & ~3
        elif token == 4:
            pass
        elif token == 9:
            assert pos == end and not stack
            break
        else:
            raise ValueError((pos, token))
    reservations, pos = [], header[4]
    while True:
        address, size = struct.unpack_from(">QQ", data, pos)
        pos += 16
        if not address and not size:
            break
        reservations.append((address, size))
    return properties, nodes, reservations


def encode_recipe(changes):
    data = bytearray(struct.pack(">I", len(changes)))
    for key, value in changes:
        node, name = key.rsplit("/", 1)
        path, name = (node or "/").encode(), name.encode()
        data += struct.pack(">III", len(path), len(name), len(value)) + path + name + value
        data += bytes(-len(data) % 4)
    return bytes(data)


def profile():
    boot = (BACKUP / "boot_a.img").read_bytes()
    vendor = (BACKUP / "vendor_boot_a.img").read_bytes()
    init = (EVIDENCE / "init_boot_a.img").read_bytes()
    live = (EVIDENCE / "live.dtb").read_bytes()
    for data, expected in [(boot, "dc3154fd0767fcb6a40080798b154ada3cd38c3887f444b20eaf80d8cf27ab43"),
                           (vendor, "8bea2a7c9a392ae73cf9269f2c530aa9f50bf36a521437b69a5e89430a07d458"),
                           (init, "0ebb78f7eb550f8b2f524cf661342baeeaa55a70e3c3e5708d71f26381fc5e8b"),
                           (live, "96c47e15278b28a82b38799506af1517dda77bd34f353b94199d594dc95fdc1b")]:
        assert digest(data) == expected, "Exact board reference changed; review required"
    original = vendor[18133056:18133056 + 42382]
    before, _, _ = decode_fdt(original)
    expected, _, _ = decode_fdt(live)
    assert before.keys() <= expected.keys(), "Property deletion not implemented"
    config = (EVIDENCE / "bootconfig.txt").read_bytes() + b'androidboot.leanboot = "1"\n'
    generic_size, = struct.unpack_from("<I", init, 12)
    initrd_length = ((18128651 + generic_size + len(config) + 1 + 3) & ~3) + 20
    expected["/chosen/bootargs"] = expected["/chosen/bootargs"].rstrip(b"\0 ") + b" leanboot=1\0"
    expected["/chosen/linux,initrd-end"] = struct.pack(">I", 0x24000000 + initrd_length)
    changes = [(k, v) for k, v in sorted(expected.items()) if before.get(k) != v]
    recipe = encode_recipe(changes)
    config_offset = 64 + len(recipe)
    digest_offset = (config_offset + len(config) + 3) & ~3
    # Exact captured AVB digest; sent as ASCII, matching MBOOT's secure message.
    digest_text = b'd8581697e658b05546004121ac9704c361c98e26ecad988e295f8f2e0a7df0c5'
    assert b'androidboot.vbmeta.digest = "'+digest_text+b'"' in config
    # Control block is data, not executable code. Mode 0 prepares and returns;
    # mode 1 will be the separately validated terminal Linux handoff.
    control = struct.pack("<16I", 0x534F424C, 1, 0, 64, 15876096, 18178560,
                          len(init), 64, len(recipe), config_offset, len(config),
                          initrd_length, digest_offset, len(digest_text), 0, 0) + recipe + config
    control += bytes(digest_offset-len(control)) + digest_text
    return {"boot": boot[:15876096], "vendor": vendor[:18178560], "init": init,
            "original_dtb": original, "expected_properties": expected,
            "recipe": recipe, "bootconfig": config, "control": control,
            "initrd_length": initrd_length, "changes": changes}


def main():
    p = profile()
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / "os-control.bin").write_bytes(p["control"])
    manifest = {"profile": "G13 V013 / 18ACC22D1C47 / slot a / native init_boot",
                "input_sha256": {k: digest(p[k]) for k in ("boot", "vendor", "init", "original_dtb", "control")},
                "input_bytes": {k: len(p[k]) for k in ("boot", "vendor", "init", "control")},
                "initrd_length": p["initrd_length"],
                "fixups": {k: v.hex() for k, v in p["changes"]},
                "reference": "Captured live DTB oracle; board fixups are applied by Lean, no MBOOT FDT service"}
    (OUT / "os-profile.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"Prepared exact G13 control data: {len(p['changes'])} fixups, {len(p['control'])} bytes")


if __name__ == "__main__":
    main()
