#!/usr/bin/env python3
"""Canonical authored inputs shared by the native PID1 build variants."""
import hashlib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]


def source_bytes():
    paths = [ROOT / name for name in (
        "src/pid1/tvinit/Cargo.toml", "tools/tvinit/build.rs", "src/pid1/lean/native/linux.c",
        "src/pid1/lean/lean-toolchain", "src/pid1/lean/lakefile.lean", "tools/pid1/build.rs",
        "src/pid1/.cargo/config.toml", "tools/pid1/linker-armv7", "src/pid1/Cargo.toml", "src/pid1/Cargo.lock", "src/platform/Cargo.toml",
        "src/pid1/audio/Cargo.toml", "src/pid1/display/Cargo.toml", "tools/display/build.rs",
        "src/pid1/mi-bootstrap/Cargo.toml", "src/pid1/scripts/mount-stock-runtime.sh",
    )]
    for directory in ("src/pid1/src", "src/platform/src", "src/shared", "src/pid1/audio", "src/pid1/display", "src/pid1/mi-bootstrap/src"):
        paths.extend((ROOT / directory).rglob("*.rs"))
    paths.extend((ROOT / "src/pid1/tvinit/src").rglob("*.rs"))
    for directory in ("src/pid1/lean", "src/lean/zr", "tools/lean/zr"):
        paths.extend((ROOT / directory).rglob("*.lean"))
    paths = [p for p in paths if ".lake" not in p.parts]
    chunks = []
    for path in sorted(set(paths)):
        name = path.relative_to(ROOT).as_posix().encode()
        content = path.read_bytes()
        chunks.append(len(name).to_bytes(4, "little") + name
                      + len(content).to_bytes(8, "little") + content)
    return b"".join(chunks)


if __name__ == "__main__":
    print(hashlib.sha256(source_bytes()).hexdigest())
