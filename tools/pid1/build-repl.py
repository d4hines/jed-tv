#!/usr/bin/env python3
"""Build our native PID 1 with a pinned static BusyBox console; no TV changes."""
import hashlib
import json
import os
from pathlib import Path
import struct
import subprocess
import sys
from source_hash import source_bytes
import zipfile

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets/build/pid1-repl"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    apk = (ROOT / "assets/research/stock-plus-root/upstream/Magisk-v28.1.apk").read_bytes()
    assert sha(apk) == "8bfd3346b3da5814f82eff6f1b1b5fedd0ad585f39a25709b23eb54aac45691d"
    import io
    with zipfile.ZipFile(io.BytesIO(apk)) as archive:
        busybox = archive.read("lib/armeabi-v7a/libbusybox.so")
    assert sha(busybox) == "7edf6024ed2305dc7bc5d91bad6a8f0ebffbecdd41b8a861d0ced297a5ea1b6d"
    assert busybox[:7] == b"\x7fELF\x01\x01\x01"
    assert struct.unpack_from("<HH", busybox, 16) == (2, 40)
    offset = struct.unpack_from("<I", busybox, 28)[0]
    size, count = struct.unpack_from("<HH", busybox, 42)
    assert all(struct.unpack_from("<I", busybox, offset + size * i)[0] != 3 for i in range(count)), "BusyBox requires a dynamic interpreter"
    OUT.mkdir(parents=True, exist_ok=True)
    path = OUT / "busybox"
    path.write_bytes(busybox)
    amp = ROOT / 'assets/research/midaemon-replacement/amplifier/ad82111.plan'
    amp_manifest = json.loads(amp.with_name('ad82111-manifest.json').read_text())
    assert sha(amp.read_bytes()) == amp_manifest['plan_sha256']
    properties = ROOT / 'assets/research/native-vendor-stack/android-properties.tar.gz'
    source = source_bytes()
    build = sha(source + busybox + amp.read_bytes() + properties.read_bytes())
    env = dict(os.environ, HELLO_BUSYBOX=str(path), HELLO_BUILD_ID=build,
               HELLO_AMP_PLAN=str(amp),
               HELLO_ANDROID_PROPERTIES=str(properties))
    env.setdefault("ZIG_GLOBAL_CACHE_DIR", str(ROOT / "assets/build/zig-global"))
    env.setdefault("ZIG_LOCAL_CACHE_DIR", str(OUT / "zig-local"))
    if sys.platform == "darwin":
        env.setdefault("CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER", "/usr/bin/cc")
    subprocess.run(["cargo", "build", "--locked", "--offline", "--release", "--features", "repl",
                    "--target", "armv7-unknown-linux-musleabihf", "--target-dir", str(OUT / "target")],
                   cwd=ROOT / "src/pid1", env=env, check=True)
    binary = OUT / "target/armv7-unknown-linux-musleabihf/release/hello-pid1"
    assert binary.stat().st_size <= 8 * 1024 * 1024
    record = {"build": build, "init_sha256": sha(binary.read_bytes()), "init_bytes": binary.stat().st_size,
              "busybox_sha256": sha(busybox), "console_port": 8766, "http_port": 8765,
              "android_framework": False, "remaining_vendor_processes": ["tee-supplicant"],
              "pid1": "Lean IO supervisor with Rust hardware operations", "hardware_verified": False}
    (OUT / "manifest.json").write_text(json.dumps(record, indent=2) + "\n")
    print(f"Native REPL init: {binary} ({binary.stat().st_size} bytes); build={build}")


if __name__ == "__main__":
    main()
