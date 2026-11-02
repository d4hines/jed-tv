#!/usr/bin/env python3
"""Build a G06-only Hello test boot image with unchanged stock kernel and DTB."""
import hashlib
import json
from pathlib import Path
import struct
import subprocess
import sys
import tempfile


def main():
    stock_path, kit, output = map(Path, sys.argv[1:])
    stock = stock_path.read_bytes()
    if hashlib.sha256(stock).hexdigest() != "d116dfa370d8e5dd5809f6f65266c86671b5e9bec9d62f95c0977bca0ff30085":
        raise ValueError("Wrong original G06 V109 boot image")
    manifest = json.loads((kit / "manifest.json").read_text())
    initrd = (kit / "initramfs.cpio.gz").read_bytes()
    if hashlib.sha256(initrd).hexdigest() != manifest["initrd_sha256"]:
        raise ValueError("Initramfs checksum mismatch")
    output.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory() as temporary:
        work = Path(temporary)
        args = subprocess.check_output([
            "unpack_bootimg", "--boot_img", str(stock_path), "--out", str(work),
            "--format", "mkbootimg", "-0",
        ]).decode().rstrip("\0").split("\0")
        reference = work / "reference.img"
        subprocess.run(["mkbootimg", *args, "--output", str(reference)], check=True)
        payload_size = struct.unpack_from(">Q", stock, len(stock) - 64 + 12)[0]
        if reference.read_bytes() != stock[:payload_size]:
            raise ValueError("No-change stock round trip changed the payload")
        kernel, dtb = [(work / name).read_bytes() for name in ("kernel", "dtb")]
        if hashlib.sha256(kernel).hexdigest() != manifest["kernel_sha256"]:
            raise ValueError("Kit does not use the exact stock kernel")
        args[args.index("--ramdisk") + 1] = str(kit / "initramfs.cpio.gz")
        index = args.index("--cmdline") + 1
        words = [word for word in args[index].split() if word != "quiet"
                 and not word.startswith(("panic=", "loglevel=", "rdinit=", "hello."))]
        words += ["rdinit=/init", "loglevel=7", "panic=3", "hello.timeout=90", "hello.return=bootloader"]
        args[index] = " ".join(words)
        image = output / "boot.img"
        subprocess.run(["mkbootimg", *args, "--output", str(image)], check=True)
        unpadded = image.stat().st_size
        if unpadded > len(stock) - 69632:
            raise ValueError("Hello image does not fit boot with AVB metadata")
        subprocess.run([
            "avbtool", "add_hash_footer", "--image", str(image), "--partition_name", "boot",
            "--partition_size", str(len(stock)), "--algorithm", "NONE", "--hash_algorithm", "sha256",
            "--salt", "2e821884d9f0ff3adfe64ff0c90fbaa9ac772594db46a3e2e8f3755cb644c7d3",
        ], check=True)
        subprocess.run(["avbtool", "verify_image", "--image", str(image)], check=True)
        check = work / "check"
        subprocess.run(["unpack_bootimg", "--boot_img", str(image), "--out", str(check)],
                       check=True, stdout=subprocess.DEVNULL)
        if (check / "kernel").read_bytes() != kernel or (check / "dtb").read_bytes() != dtb:
            raise ValueError("Stock kernel or DTB changed")
        if (check / "ramdisk").read_bytes() != initrd:
            raise ValueError("Packaged initramfs changed")
    manifest.update(
        type="Experimental flashable Hello PID-1 image", boot_bytes=image.stat().st_size,
        boot_sha256=hashlib.sha256(image.read_bytes()).hexdigest(), unpadded_boot_bytes=unpadded,
        kernel_unchanged=True, dtb_unchanged=True, kernel_cmdline=" ".join(words),
        return_request="Linux RESTART2 bootloader; hardware handling unverified",
        automatic_return_to_stock=False, hardware_boot_verified=False,
        recovery="Use prepared USB Fastbootd trigger if bootloader return fails",
    )
    (output / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(json.dumps({key: manifest[key] for key in ("boot_bytes", "boot_sha256", "return_request")}, indent=2))


if __name__ == "__main__":
    main()
