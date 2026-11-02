#!/usr/bin/env python3
"""Add a pinned Magisk bootstrap to this G06's original boot ramdisk only."""
import gzip
import hashlib
import json
import lzma
from pathlib import Path
import stat
import struct
import subprocess
import sys
import tempfile
from zipfile import ZipFile

STOCK_SHA = "d116dfa370d8e5dd5809f6f65266c86671b5e9bec9d62f95c0977bca0ff30085"
APK_SHA = "8bfd3346b3da5814f82eff6f1b1b5fedd0ad585f39a25709b23eb54aac45691d"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def read_cpio(data):
    entries, offset = {}, 0
    while True:
        if data[offset:offset + 6] != b"070701":
            raise ValueError("Expected uncompressed newc ramdisk")
        fields = [int(data[offset + 6 + i * 8:offset + 14 + i * 8], 16) for i in range(13)]
        name = data[offset + 110:offset + 110 + fields[11] - 1].decode()
        offset = (offset + 110 + fields[11] + 3) & ~3
        content = data[offset:offset + fields[6]]
        if len(content) != fields[6]:
            raise ValueError("Truncated cpio entry")
        offset = (offset + fields[6] + 3) & ~3
        if name == "TRAILER!!!":
            return entries
        if name in entries or name.startswith("/") or ".." in name.split("/"):
            raise ValueError("Unexpected cpio name")
        entries[name] = (fields, content)


def write_cpio(entries):
    output = bytearray()
    for name, (original, content) in [*sorted(entries.items()), ("TRAILER!!!", ([0] * 13, b""))]:
        fields = original.copy()
        fields[6], fields[11] = len(content), len(name.encode()) + 1
        output.extend(("070701" + "".join(f"{v:08x}" for v in fields)).encode())
        output.extend(name.encode() + b"\0")
        output.extend(b"\0" * (-len(output) % 4))
        output.extend(content)
        output.extend(b"\0" * (-len(output) % 4))
    output.extend(b"\0" * (-len(output) % 512))
    return bytes(output)


def xz(content):
    return lzma.compress(content, format=lzma.FORMAT_XZ, check=lzma.CHECK_CRC32)


def main():
    stock_path, apk_path, rc_path, setup_path, output = map(Path, sys.argv[1:])
    stock, apk = stock_path.read_bytes(), apk_path.read_bytes()
    if sha(stock) != STOCK_SHA or sha(apk) != APK_SHA:
        raise ValueError("Wrong original boot image or pinned official Magisk APK")
    output.mkdir(parents=True, exist_ok=True)
    validation = output / "validation"
    validation.mkdir()
    with tempfile.TemporaryDirectory() as temporary:
        work = Path(temporary)
        args = subprocess.check_output([
            "unpack_bootimg", "--boot_img", str(stock_path), "--out", str(work),
            "--format", "mkbootimg", "-0",
        ]).decode().rstrip("\0").split("\0")
        subprocess.run(["mkbootimg", *args, "--output", str(work / "reference.img")], check=True)
        payload_size = struct.unpack_from(">Q", stock, len(stock) - 64 + 12)[0]
        if (work / "reference.img").read_bytes() != stock[:payload_size]:
            raise ValueError("No-change stock boot round trip failed")
        original_ramdisk = gzip.decompress((work / "ramdisk").read_bytes())
        original = read_cpio(original_ramdisk)
        if any(name.startswith((".backup", "overlay.d")) for name in original):
            raise ValueError("Expected unmodified stock ramdisk")
        entries = dict(original)
        next_inode = max(entry[0][0] for entry in entries.values()) + 1

        def add(name, mode, content=b""):
            nonlocal next_inode
            entries[name] = ([next_inode, mode, 0, 0, 1, 0, len(content), 0, 0, 0, 0, len(name) + 1, 0], content)
            next_inode += 1

        component_hashes = {}
        with ZipFile(apk_path) as archive:
            def component(name):
                content = archive.read(name)
                component_hashes[name] = sha(content)
                return content

            add("init", stat.S_IFREG | 0o750, component("lib/armeabi-v7a/libmagiskinit.so"))
            add("overlay.d", stat.S_IFDIR | 0o750)
            add("overlay.d/sbin", stat.S_IFDIR | 0o750)
            for target, source in (
                ("magisk", "lib/armeabi-v7a/libmagisk.so"),
                ("init-ld", "lib/armeabi-v7a/libinit-ld.so"),
                ("stub", "assets/stub.apk"),
            ):
                add(f"overlay.d/sbin/{target}.xz", stat.S_IFREG | 0o644, xz(component(source)))
            add("overlay.d/tv-adb-root.rc", stat.S_IFREG | 0o644, rc_path.read_bytes())
            add("overlay.d/sbin/tv-adb-root.sh", stat.S_IFREG | 0o700, setup_path.read_bytes())
            (validation / "magiskboot").write_bytes(component("lib/armeabi-v7a/libmagiskboot.so"))
            (validation / "magiskpolicy").write_bytes(component("lib/armeabi-v7a/libmagiskpolicy.so"))

        # Same backup layout consumed by upstream Magisk init and cpio restore.
        added = sorted(set(entries) - set(original))
        add(".backup", stat.S_IFDIR)
        backup_fields = original["init"][0].copy()
        backup = xz(original["init"][1])
        backup_fields[6], backup_fields[11] = len(backup), len(".backup/init.xz") + 1
        entries[".backup/init.xz"] = (backup_fields, backup)
        add(".backup/.rmlist", stat.S_IFREG, b"".join(name.encode() + b"\0" for name in added))
        config = ("KEEPVERITY=true\nKEEPFORCEENCRYPT=true\nRECOVERYMODE=false\n"
                  + "SHA1=" + hashlib.sha1(stock).hexdigest() + "\n").encode()
        add(".backup/.magisk", stat.S_IFREG, config)
        if lzma.decompress(entries[".backup/init.xz"][1]) != original["init"][1]:
            raise ValueError("Original init backup mismatch")
        for name in original:
            if name != "init" and entries[name] != original[name]:
                raise ValueError("Unexpected change to original ramdisk entry: " + name)
        patched = write_cpio(entries)
        if read_cpio(patched) != entries:
            raise ValueError("Patched cpio round trip failed")
        (validation / "stock.cpio").write_bytes(original_ramdisk)
        (validation / "patched.cpio").write_bytes(patched)
        ramdisk_path = work / "root-ramdisk.gz"
        ramdisk_path.write_bytes(gzip.compress(patched, mtime=0))
        args[args.index("--ramdisk") + 1] = str(ramdisk_path)
        image = output / "boot.img"
        subprocess.run(["mkbootimg", *args, "--output", str(image)], check=True)
        unpadded = image.stat().st_size
        if unpadded > len(stock) - 69632:
            raise ValueError("Root image does not fit boot with AVB metadata")
        subprocess.run([
            "avbtool", "add_hash_footer", "--image", str(image), "--partition_name", "boot",
            "--partition_size", str(len(stock)), "--algorithm", "NONE", "--hash_algorithm", "sha256",
            "--salt", "2e821884d9f0ff3adfe64ff0c90fbaa9ac772594db46a3e2e8f3755cb644c7d3",
        ], check=True)
        subprocess.run(["avbtool", "verify_image", "--image", str(image)], check=True)
        check = work / "check"
        subprocess.run(["unpack_bootimg", "--boot_img", str(image), "--out", str(check)], check=True, stdout=subprocess.DEVNULL)
        for name in ("kernel", "dtb"):
            if (check / name).read_bytes() != (work / name).read_bytes():
                raise ValueError("Original payload changed: " + name)
        if gzip.decompress((check / "ramdisk").read_bytes()) != patched:
            raise ValueError("Repacked ramdisk mismatch")
        rebuilt = image.read_bytes()
        if rebuilt[64:576] != stock[64:576] or rebuilt[608:1632] != stock[608:1632]:
            raise ValueError("Original command line or header identity changed")
    report = {
        "device": "G06", "firmware": "V109", "magisk_version": "28.1",
        "original_boot_sha256": STOCK_SHA, "magisk_apk_sha256": APK_SHA,
        "magisk_url": "https://github.com/topjohnwu/Magisk/releases/download/v28.1/Magisk-v28.1.apk",
        "boot_sha256": sha(image.read_bytes()), "boot_bytes": image.stat().st_size,
        "unpadded_boot_bytes": unpadded, "kernel_unchanged": True, "dtb_unchanged": True,
        "boot_cmdline_unchanged": True, "fstab_and_avb_keys_unchanged": True,
        "keepverity": True, "keepforceencrypt": True,
        "changed_original_ramdisk_entries": ["init"], "added_ramdisk_entries": sorted(set(entries) - set(original)),
        "automatic_su_uid": 2000, "su_access_setting": "ADB only",
        "global_selinux": "Enforcing; upstream Magisk root domain is permissive",
        "persistent_runtime_state": "/data/adb (Magisk runtime, root policy database; created on first boot)",
        "stock_reflash_disables_root_but_does_not_remove_data_adb": True,
        "hardware_boot_verified": False, "kexec_load_or_execution_verified": False,
        "upstream_components": component_hashes,
    }
    (output / "manifest.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))


if __name__ == "__main__":
    main()
