#!/usr/bin/env python3
"""Package one static Doom PID 1 and data with the unchanged original G06 kernel."""
import gzip
import hashlib
import json
from pathlib import Path
import stat
import struct
import subprocess
import sys
import tarfile
import tempfile


def cpio(files):
    result = bytearray()
    for index, (name, mode, data, device) in enumerate(files, 1):
        values = (index, mode, 0, 0, 1, 0, len(data), 0, 0, *device, len(name) + 1, 0)
        result.extend(("070701" + "".join(f"{v:08x}" for v in values)).encode() + name.encode() + b"\0")
        result.extend(b"\0" * (-len(result) % 4))
        result.extend(data)
        result.extend(b"\0" * (-len(result) % 4))
    result.extend(b"\0" * (-len(result) % 512))
    return bytes(result)


def main():
    stock_path, runtime, output = map(Path, sys.argv[1:])
    stock = stock_path.read_bytes()
    if hashlib.sha256(stock).hexdigest() != "d116dfa370d8e5dd5809f6f65266c86671b5e9bec9d62f95c0977bca0ff30085":
        raise ValueError("Wrong stock image")
    output.mkdir(parents=True, exist_ok=True)
    with tarfile.open(runtime) as archive:
        if sorted(archive.getnames()) != ["doom-data.xz", "init", "manifest.json"]:
            raise ValueError("Unexpected runtime archive")
        init, packed, manifest_bytes = [archive.extractfile(name).read() for name in ("init", "doom-data.xz", "manifest.json")]
    manifest = json.loads(manifest_bytes)
    for name, data in (("init", init), ("data", packed)):
        if hashlib.sha256(data).hexdigest() != manifest[name + "_sha256"]:
            raise ValueError("Runtime hash mismatch: " + name)
    files = [(name, stat.S_IFDIR | 0o755, b"", (0, 0)) for name in ("dev", "proc", "sys", "tmp")]
    files += [("dev/console", stat.S_IFCHR | 0o600, b"", (5, 1)),
              ("init", stat.S_IFREG | 0o755, init, (0, 0)),
              ("doom-data.xz", stat.S_IFREG | 0o644, packed, (0, 0)),
              ("TRAILER!!!", 0, b"", (0, 0))]
    ramdisk = cpio(files)
    (output / "initramfs.cpio.gz").write_bytes(gzip.compress(ramdisk, mtime=0))
    with tempfile.TemporaryDirectory() as temporary:
        work = Path(temporary)
        args = subprocess.check_output(["unpack_bootimg", "--boot_img", str(stock_path), "--out", str(work), "--format", "mkbootimg", "-0"]).decode().rstrip("\0").split("\0")
        # A no-change round trip must reproduce every original payload byte.
        reference = work / "reference.img"
        subprocess.run(["mkbootimg", *args, "--output", str(reference)], check=True)
        payload_size = struct.unpack_from(">Q", stock, len(stock) - 64 + 12)[0]
        if reference.read_bytes() != stock[:payload_size]:
            raise ValueError("Stock mkbootimg round trip changed the payload")
        original_kernel = (work / "kernel").read_bytes()
        original_dtb = (work / "dtb").read_bytes()
        args[args.index("--ramdisk") + 1] = str(output / "initramfs.cpio.gz")
        command_index = args.index("--cmdline") + 1
        words = [word for word in args[command_index].split() if word != "quiet" and not word.startswith("loglevel=")]
        words += ["rdinit=/init", "loglevel=7", "panic=0", "doom.iwad=/doom/freedoom1.wad", "doom.args=-warp,1,1"]
        args[command_index] = " ".join(words)
        image = output / "boot.img"
        subprocess.run(["mkbootimg", *args, "--output", str(image)], check=True)
        if image.stat().st_size > len(stock) - 69632:
            raise ValueError("Doom payload does not fit boot with AVB metadata")
        unpadded_bytes = image.stat().st_size
        subprocess.run(["avbtool", "add_hash_footer", "--image", str(image), "--partition_name", "boot",
                        "--partition_size", str(len(stock)), "--algorithm", "NONE", "--hash_algorithm", "sha256",
                        "--salt", "2e821884d9f0ff3adfe64ff0c90fbaa9ac772594db46a3e2e8f3755cb644c7d3"], check=True)
        subprocess.run(["avbtool", "verify_image", "--image", str(image)], check=True)
        check = work / "check"
        subprocess.run(["unpack_bootimg", "--boot_img", str(image), "--out", str(check)], check=True, stdout=subprocess.DEVNULL)
        if (check / "kernel").read_bytes() != original_kernel or (check / "dtb").read_bytes() != original_dtb:
            raise ValueError("Kernel or DTB changed")
    manifest.update(boot_bytes=image.stat().st_size, unpadded_boot_bytes=unpadded_bytes,
                    boot_sha256=hashlib.sha256(image.read_bytes()).hexdigest(),
                    kernel_unchanged=True, dtb_unchanged=True, ramdisk_bytes=len(ramdisk),
                    ramdisk_gzip_bytes=(output / "initramfs.cpio.gz").stat().st_size,
                    kernel_cmdline=" ".join(words), hardware_verified=False)
    (output / "manifest.json").write_text(json.dumps(manifest, indent=2) + "\n")
    print(json.dumps({k: manifest[k] for k in ("boot_bytes", "init_bytes", "compressed_data_bytes", "ramdisk_gzip_bytes", "hardware_verified")}, indent=2))


if __name__ == "__main__":
    main()
