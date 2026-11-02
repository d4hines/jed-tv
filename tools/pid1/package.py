#!/usr/bin/env python3
"""Prepare a RAM-boot kit; deliberately emits no flashable boot.img."""
import gzip
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import stat
import struct
import tarfile

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets/build/hello-pid1"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    source = ROOT / "assets/backups/v109-stock-20261031/boot.img"
    stock = source.read_bytes()
    if sha(stock) != "d116dfa370d8e5dd5809f6f65266c86671b5e9bec9d62f95c0977bca0ff30085":
        raise ValueError("Unexpected stock boot image")
    kernel_size = struct.unpack_from("<I", stock, 8)[0]
    page = struct.unpack_from("<I", stock, 36)[0]
    kernel = stock[page:page + kernel_size]
    dtb = (ROOT / "assets/research/kexec/live.dtb").read_bytes()
    if dtb[:4] != bytes.fromhex("d00dfeed") or struct.unpack_from(">I", dtb, 4)[0] != len(dtb):
        raise ValueError("Expected exact live FDT, without the TCL packed-DTB header")
    init = (OUT / "target/armv7-unknown-linux-musleabihf/release/hello-pid1").read_bytes()
    if init[:5] != b"\x7fELF\x01" or struct.unpack_from("<H", init, 18)[0] != 40:
        raise ValueError("Expected ARM32 ELF")
    spec = importlib.util.spec_from_file_location("cpio", ROOT / "tools/make-doom-boot.py")
    writer = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(writer)
    doom = json.loads((ROOT / "assets/build/doom-minimal/result/manifest.json").read_text())
    files = [(name, stat.S_IFDIR | 0o755, b"", (0, 0)) for name in
             ("dev", "proc", "sys", "tmp", "modules", "vendor", "vendor/tvconfig", "vendor/tvconfig/config")]
    files += [("dev/console", stat.S_IFCHR | 0o600, b"", (5, 1)),
              ("init", stat.S_IFREG | 0o755, init, (0, 0))]
    components = {}
    for name, metadata in doom["files"].items():
        if not name.startswith("modules/") and name != "vendor/tvconfig/config/ir_config.ini":
            continue
        if name == "modules/order":
            data = b"kdrv_xc.ko\nutpa2k.ko\nhdcp1x.ko\nhdcp2x.ko\nxcker.ko\niniparser.ko\nmwgifker.ko\nmisck.ko\nmik.ko\nmstar_fbdev_mi.ko\n"
        else:
            local = name.split("/")[-1]
            data = (ROOT / "assets/build/doom-minimal" / local).read_bytes()
        if sha(data) != metadata["sha256"]:
            raise ValueError("Stock component checksum mismatch: " + name)
        files.append((name, stat.S_IFREG | 0o644, data, (0, 0)))
        components[name] = metadata
    files.append(("TRAILER!!!", 0, b"", (0, 0)))
    initrd = gzip.compress(writer.cpio(files), mtime=0)
    inspect = (ROOT / "assets/research/kexec/probe/inspect.txt").read_text()
    cmdline = next(line[len("CMDLINE "):] for line in inspect.splitlines() if line.startswith("CMDLINE "))
    words = [word for word in cmdline.split() if word != "quiet" and not word.startswith(("panic=", "loglevel="))]
    words += ["rdinit=/init", "loglevel=7", "panic=3", "hello.timeout=90"]
    cmdline = " ".join(words) + "\n"
    manifest = {
        "type": "RAM-boot kit; not a flashable firmware image",
        "pid1": "hello-pid1", "kernel_entry": "0x20208000",
        "init_bytes": len(init), "init_sha256": sha(init),
        "kernel_bytes": len(kernel), "kernel_sha256": sha(kernel),
        "dtb_sha256": sha(dtb), "initrd_bytes": len(initrd), "initrd_sha256": sha(initrd),
        "stock_components": components,
        "debug": {"ip": "192.168.77.2", "port": 8765, "peer": "192.168.77.1",
                  "routes": ["GET /status", "GET /logs", "POST /reboot", "POST /exec (static ARM32 ELF <=8 MiB)"]},
        "timeout_seconds": 90, "writes_firmware": False, "hardware_boot_verified": False,
        "normal_reboot_returns_to_stock_only_if_stock_is_still_on_disk": True,
    }
    kit = OUT / "kit"
    kit.mkdir(parents=True, exist_ok=True)
    contents = {"init": init, "kernel": kernel, "dtb": dtb, "initramfs.cpio.gz": initrd,
                "cmdline.txt": cmdline.encode(), "manifest.json": (json.dumps(manifest, indent=2) + "\n").encode()}
    archive = OUT / "g06-hello-pid1-kit.tar"
    with tarfile.open(archive, "w", format=tarfile.USTAR_FORMAT) as tar:
        for name, data in contents.items():
            (kit / name).write_bytes(data)
            entry = tarfile.TarInfo(name)
            entry.size, entry.mode = len(data), 0o644
            tar.addfile(entry, io.BytesIO(data))
    pin = {"name": archive.name, "sha256": sha(archive.read_bytes())}
    (ROOT / "tools/pid1/runtime.json").write_text(json.dumps(pin, indent=2) + "\n")
    print(json.dumps({**pin, "init_bytes": len(init), "initrd_bytes": len(initrd)}, indent=2))


if __name__ == "__main__":
    main()
