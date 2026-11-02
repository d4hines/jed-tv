#!/usr/bin/env python3
"""Build a self-contained RAM-upload candidate from verified V109 evidence."""
import hashlib
import json
import os
from pathlib import Path
import re
import struct
import subprocess
import sys
from source_hash import source_bytes

ROOT = Path(__file__).resolve().parents[2]
EVIDENCE = ROOT / "assets/research/g06-display-offline"
OUT = ROOT / "assets/build/pid1-mi"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    subprocess.run([sys.executable, "research/g06-display/verify.py"], cwd=ROOT,
                   check=True, stdout=subprocess.DEVNULL)
    manifest = json.loads((EVIDENCE / "candidate-manifest.json").read_text())
    files = []
    for entry in manifest["files"]:
        name = entry["target"]
        content = (EVIDENCE / "candidate-root" / name.lstrip("/")).read_bytes()
        if name.endswith("/mik_init.ini"):
            original = (EVIDENCE / "tvconfig-config/config/mik_init.ini").read_bytes()
            assert sha(original) == entry["sha256"]
            expected = original.decode()
            for key, value in manifest["edits"].items():
                expected, count = re.subn(r"(?m)^(" + key + r"\s*=\s*)\d+",
                                         lambda m: m[1] + str(value), expected)
                assert count == 1
            assert content == expected.encode(), "unexpected candidate INI edit"
        else:
            assert sha(content) == entry["sha256"], name
        files.append((name, content))
    for name in ["kdrv_platform", "kdrv_alg", "libdmdbraslia"]:
        path = EVIDENCE / "vendor/lib/modules" / (name + ".ko")
        content = path.read_bytes()
        extracted = json.loads((EVIDENCE / "extracted-manifest.json").read_text())
        expected = next(x for x in extracted if x["path"].endswith("/vendor/lib/modules/" + path.name))
        assert sha(content) == expected["sha256"], path
        files.append(("/modules/" + path.name, content))
    OUT.mkdir(parents=True, exist_ok=True)
    pack = b"".join(struct.pack("<II", len(name.encode()), len(content)) + name.encode() + content
                    for name, content in files)
    pack_path = OUT / "config.pack"
    pack_path.write_bytes(pack)
    source = source_bytes()
    build_id = sha(source + pack)
    env = dict(os.environ, HELLO_MI_PACK=str(pack_path), HELLO_BUILD_ID=build_id)
    if sys.platform == "darwin":
        env.setdefault("CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER", "/usr/bin/cc")
    subprocess.run(["cargo", "build", "--locked", "--offline", "--release", "--features", "mi-display",
                    "--target", "armv7-unknown-linux-musleabihf", "--target-dir", str(OUT / "target")],
                   cwd=ROOT / "src/pid1", env=env, check=True)
    binary = OUT / "target/armv7-unknown-linux-musleabihf/release/hello-pid1"
    assert binary.stat().st_size <= 8 * 1024 * 1024, "candidate exceeds RAM upload limit"
    record = {"build": build_id, "pack_sha256": sha(pack), "init_sha256": sha(binary.read_bytes()),
              "init_bytes": binary.stat().st_size, "hardware_verified": False,
              "files": [{"target": name, "bytes": len(data), "sha256": sha(data)} for name, data in files]}
    (OUT / "manifest.json").write_text(json.dumps(record, indent=2) + "\n")
    print(f"RAM upload candidate: {binary} ({binary.stat().st_size} bytes); build={build_id}")


if __name__ == "__main__":
    main()
