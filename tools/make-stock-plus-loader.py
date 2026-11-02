#!/usr/bin/env python3
"""Add a one-shot PID-1 selector to the prepared G06 stock-plus-root image."""
import gzip
import hashlib
import importlib.util
import json
from pathlib import Path
import stat
import struct
import subprocess
import sys
import tarfile
import tempfile

sys.dont_write_bytecode = True
STOCK_SHA = "d116dfa370d8e5dd5809f6f65266c86671b5e9bec9d62f95c0977bca0ff30085"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def check_static_arm(binary):
    if len(binary) < 52 or binary[:7] != b"\x7fELF\x01\x01\x01" or struct.unpack_from("<H", binary, 18)[0] != 40:
        raise ValueError("Launcher must be an ARM32 ELF")
    offset = struct.unpack_from("<I", binary, 28)[0]
    size, count = struct.unpack_from("<HH", binary, 42)
    if size != 32 or count == 0 or offset + size * count > len(binary):
        raise ValueError("Invalid launcher ELF program headers")
    if any(struct.unpack_from("<I", binary, offset + size * index)[0] == 3 for index in range(count)):
        raise ValueError("Launcher requires a dynamic interpreter before Android is mounted")


def unpack(image, directory):
    return subprocess.check_output([
        "unpack_bootimg", "--boot_img", str(image), "--out", str(directory),
        "--format", "mkbootimg", "-0",
    ]).decode().rstrip("\0").split("\0")


def main():
    stock_path, root_image, hello_kit, runtime, root_tool, output = map(Path, sys.argv[1:])
    stock = stock_path.read_bytes()
    if sha(stock) != STOCK_SHA:
        raise ValueError("Wrong original G06 V109 boot image")
    spec = importlib.util.spec_from_file_location("root_image", root_tool)
    writer = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(writer)
    root_manifest = json.loads((root_image / "manifest.json").read_text())
    root_boot = (root_image / "boot.img").read_bytes()
    if sha(root_boot) != root_manifest["boot_sha256"] or root_manifest["original_boot_sha256"] != STOCK_SHA:
        raise ValueError("Wrong stock-plus-root input")
    hello_manifest = json.loads((hello_kit / "manifest.json").read_text())
    hello_cpio = gzip.decompress((hello_kit / "initramfs.cpio.gz").read_bytes())
    if sha((hello_kit / "initramfs.cpio.gz").read_bytes()) != hello_manifest["initrd_sha256"]:
        raise ValueError("Hello initramfs checksum mismatch")
    hello = writer.read_cpio(hello_cpio)
    with tarfile.open(runtime) as archive:
        if sorted(archive.getnames()) != ["launcher", "manifest.json"]:
            raise ValueError("Unexpected launcher runtime entries")
        launcher = archive.extractfile("launcher").read()
        launcher_manifest = json.loads(archive.extractfile("manifest.json").read())
    if sha(launcher) != launcher_manifest["binary_sha256"] or launcher_manifest["profile"] != "G06-V109-production":
        raise ValueError("Wrong production launcher runtime")
    if b"TV-LAUNCHER-PROFILE:G06-V109" not in launcher or b"TV-LAUNCHER-PROFILE:QEMU-TEST-ONLY" in launcher:
        raise ValueError("Refusing test-profile launcher in a TV image")
    check_static_arm(launcher)
    output.mkdir(parents=True, exist_ok=True)
    validation = output / "validation"
    validation.mkdir()
    with tempfile.TemporaryDirectory() as temporary:
        work = Path(temporary)
        stock_dir, base, check = work / "stock", work / "base", work / "check"
        unpack(stock_path, stock_dir)
        args = unpack(root_image / "boot.img", base)
        subprocess.run(["mkbootimg", *args, "--output", str(work / "reference.img")], check=True)
        payload_size = struct.unpack_from(">Q", root_boot, len(root_boot) - 64 + 12)[0]
        if (work / "reference.img").read_bytes() != root_boot[:payload_size]:
            raise ValueError("No-change root-image round trip failed")
        original = writer.read_cpio(gzip.decompress((stock_dir / "ramdisk").read_bytes()))
        entries = writer.read_cpio(gzip.decompress((base / "ramdisk").read_bytes()))
        if any(name.startswith("launcher-") for name in entries):
            raise ValueError("Input already contains a launcher")
        next_inode = max(fields[0] for fields, _ in entries.values()) + 1

        def add(name, mode, data=b"", device=(0, 0)):
            nonlocal next_inode
            entries[name] = ([next_inode, mode, 0, 0, 1, 0, len(data), 0, 0, *device, len(name) + 1, 0], data)
            next_inode += 1

        magiskinit = entries["init"][1]
        add("launcher-magiskinit", stat.S_IFREG | 0o750, magiskinit)
        add("launcher-stock-init", stat.S_IFREG | 0o750, original["init"][1])
        add("launcher-hello", stat.S_IFREG | 0o750, hello["init"][1])
        add("init", stat.S_IFREG | 0o750, launcher)
        add("launcher-cache", stat.S_IFDIR | 0o700)
        for name, (fields, content) in hello.items():
            if name == "init":
                continue
            if name in entries:
                if stat.S_IFMT(fields[1]) != stat.S_IFDIR or stat.S_IFMT(entries[name][0][1]) != stat.S_IFDIR:
                    raise ValueError("Unexpected Hello/root ramdisk collision: " + name)
                continue
            add(name, fields[1], content, (fields[9], fields[10]))
        # Preserve the upstream restore contract for every additional file.
        removed = sorted(name for name in entries if name not in original and not name.startswith(".backup"))
        add(".backup/.rmlist", stat.S_IFREG, b"".join(name.encode() + b"\0" for name in removed))
        for name, entry in original.items():
            if name != "init" and entries[name] != entry:
                raise ValueError("Original stock ramdisk entry changed: " + name)
        patched = writer.write_cpio(entries)
        if writer.read_cpio(patched) != entries:
            raise ValueError("Launcher cpio round trip failed")
        ramdisk = work / "launcher-ramdisk.gz"
        ramdisk.write_bytes(gzip.compress(patched, mtime=0))
        ramdisk_bytes = ramdisk.stat().st_size
        args[args.index("--ramdisk") + 1] = str(ramdisk)
        index = args.index("--cmdline") + 1
        marker = "androidboot.tv_launcher=" + launcher_manifest["boot_marker"]
        if "androidboot.tv_launcher=" in args[index]:
            raise ValueError("Launcher boot marker already present")
        args[index] += " " + marker
        image = output / "boot.img"
        subprocess.run(["mkbootimg", *args, "--output", str(image)], check=True)
        unpadded = image.stat().st_size
        if unpadded > len(stock) - 69632:
            raise ValueError("Stock-plus-loader does not fit the 31 MiB boot partition")
        subprocess.run([
            "avbtool", "add_hash_footer", "--image", str(image), "--partition_name", "boot",
            "--partition_size", str(len(stock)), "--algorithm", "NONE", "--hash_algorithm", "sha256",
            "--salt", "2e821884d9f0ff3adfe64ff0c90fbaa9ac772594db46a3e2e8f3755cb644c7d3",
        ], check=True)
        subprocess.run(["avbtool", "verify_image", "--image", str(image)], check=True)
        unpack(image, check)
        for name in ("kernel", "dtb"):
            if (check / name).read_bytes() != (stock_dir / name).read_bytes():
                raise ValueError("Stock kernel/packed DTB changed: " + name)
        if gzip.decompress((check / "ramdisk").read_bytes()) != patched:
            raise ValueError("Repacked launcher ramdisk changed")
        (validation / "patched.cpio").write_bytes(patched)
        (validation / "stock.cpio").write_bytes((root_image / "validation/stock.cpio").read_bytes())
        for name in ("magiskboot", "magiskpolicy"):
            (validation / name).write_bytes((root_image / "validation" / name).read_bytes())
    (output / "launcherctl").write_bytes(launcher)
    (output / "request.txt").write_text(launcher_manifest["request_body"])
    report = {
        "type": "Experimental one-shot cold-boot launcher with stock-plus-root Android default",
        "original_boot_sha256": STOCK_SHA, "root_boot_sha256": root_manifest["boot_sha256"],
        "boot_sha256": sha(image.read_bytes()), "boot_bytes": image.stat().st_size,
        "unpadded_boot_bytes": unpadded, "ramdisk_gzip_bytes": ramdisk_bytes,
        "kernel_unchanged": True, "dtb_unchanged": True, "fstab_and_avb_keys_unchanged": True,
        "default": "restore Magiskinit to /init and exec as PID1; original stock init is last-resort fallback",
        "experiment": "Hello PID1 only after a matching root-only cache request is durably deleted and verified absent",
        "experiment_args": ["hello.timeout=90", "hello.return=stock", "hello.display_delay=5"],
        "cache_path": "/cache/tv-launcher/request", "request_writes": "own file/directory only; ext4 journal recovery may write metadata",
        "raw_partition_or_bcb_writes": False, "hardware_boot_verified": False,
        "launcher": launcher_manifest, "hello_init_sha256": sha(hello["init"][1]),
    }
    (output / "manifest.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: report[key] for key in ("boot_sha256", "boot_bytes", "unpadded_boot_bytes", "hardware_boot_verified")}, indent=2))


if __name__ == "__main__":
    main()
