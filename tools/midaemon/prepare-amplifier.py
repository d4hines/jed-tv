#!/usr/bin/env python3
"""Extract an exact-V109 AD82111 plan offline; never opens a device."""
import hashlib
import json
from pathlib import Path
import re
import struct
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "research/g06-display"))
from analyze import ELF

FIRMWARE = ROOT / "assets/research/audio-native/sitatvservice"
CONFIG = ROOT / "assets/research/g06-display-offline/tclconfig-hardware/hardware"
EXPECTED_FIRMWARE_SHA256 = "2a561d62ec29eff19ad9c47921831b5437faf581f4f16b6877a96850dcca930b"
EXPECTED_HID_SHA256 = "6138a85e647928e1fb5c97ad0c1e454bff10c444ad04201c2f99fba8c3c15f26"
EXPECTED_EQ_SHA256 = "ec358d7d25fd0722643668035d712dcf8fb871f695d0a81cebf916d0751f493f"


def at(elf, address, count):
    for i, section in enumerate(elf.ss):
        if section[1] != 8 and section[3] <= address < section[3] + section[5]:
            start = address - section[3]
            result = elf.data(i)[start:start + count]
            if len(result) == count:
                return result
    raise ValueError(f"unmapped address {address:x}")


def section_pairs(text, name, key):
    section = re.search(r"(?ms)^\[" + re.escape(name) + r"\]\s*(.*?)(?=^\[|\Z)", text)[1]
    body = re.search(r"(?s)\b" + re.escape(key) + r"\s*=\s*\"(.*?)\"", section)[1]
    values = [int(token, 0) for token in re.findall(r"0x[0-9a-fA-F]+|\b[0-9]+\b", body)]
    if len(values) % 2 or not all(0 <= value <= 255 for value in values):
        raise ValueError(f"invalid {name}.{key} register table")
    return bytes(values)


def main():
    digest = hashlib.sha256(FIRMWARE.read_bytes()).hexdigest()
    if not EXPECTED_FIRMWARE_SHA256 or digest != EXPECTED_FIRMWARE_SHA256:
        raise ValueError(f"unrecognized sitatvservice SHA256: {digest}")
    elf = ELF(FIRMWARE)
    table = at(elf, 0x11967d1, 156)
    assert at(elf, 0x1721774, 4) == bytes([2, 0x10, 2, 0x1e])
    hid = CONFIG / "HID_32_185_2K_NA_S5400A.ini"
    assert hashlib.sha256(hid.read_bytes()).hexdigest() == EXPECTED_HID_SHA256
    model = json.loads(hid.read_text())
    amp = model["Drives"]["AMP_CHIPS"][0]["AMP_1"]
    assert amp["TYPE"] == "AD82111" and amp["IIC"] == 1 and amp["ADDR"] == 96
    eq = CONFIG / amp["EQ"].removeprefix("/tclconfig/hardware/")
    assert hashlib.sha256(eq.read_bytes()).hexdigest() == EXPECTED_EQ_SHA256
    text = eq.read_text()
    drc = section_pairs(text, "DRC_TABLE", "data")
    gain = section_pairs(text, "VOL_GAIN_DATA", "data1")
    # Route fields: GPIO pad, inversion, MI IIC port, vendor-format slave address.
    header = b"AD81" + bytes([1, 30, 0, 9, 0x60, 0]) + struct.pack("<HHH", 78, len(drc) // 2, len(gain) // 2)
    output = ROOT / "assets/research/midaemon-replacement/amplifier"
    output.mkdir(parents=True, exist_ok=True)
    plan = header + table + drc + gain
    (output / "ad82111.plan").write_bytes(plan)
    manifest = {
        "firmware_sha256": digest,
        "hid_sha256": hashlib.sha256(hid.read_bytes()).hexdigest(),
        "eq_sha256": hashlib.sha256(eq.read_bytes()).hexdigest(),
        "plan_sha256": hashlib.sha256(plan).hexdigest(),
        "counts": {"factory": 78, "drc": len(drc) // 2, "gain": len(gain) // 2},
        "source_table_address": "0x11967d1",
        "eq_path": amp["EQ"],
    }
    (output / "ad82111-manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(json.dumps(manifest, indent=2))


if __name__ == "__main__":
    main()
