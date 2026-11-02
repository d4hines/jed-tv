#!/usr/bin/env python3
"""Exercise ARM32 launcher filesystem/fallback semantics, with no TV access."""
import argparse
import gzip
import importlib.util
import json
import os
from pathlib import Path
import stat
import subprocess
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "assets/build/launcher/qemu"
TARGET = ROOT / "assets/build/launcher/target/armv7-unknown-linux-musleabihf/release"
KERNEL = Path("/nix/store/3qam90lha5ppgm1k2a5ckj3qd5civzpb-linux-6.1.143/Image")
E2FS = Path("/nix/store/gly23cvy7pbx4ngy3ac2x2n51sxg57i9-e2fsprogs-1.47.4-bin/bin")
REQUEST = b"TV-LAUNCHER/1\nG06\nV109\n4C0490290FA05A0C0\nhello\ntest-build\n"


def tool(command):
    result = subprocess.run(command, capture_output=True, check=True)
    return result.stdout + result.stderr


def debug(disk, command):
    return tool([str(E2FS / "debugfs"), "-w", "-R", command, str(disk)])


def disk_image(name, request=None, mode="0100600", size=104857600):
    disk = OUT / (name + ".ext4")
    with disk.open("wb") as file:
        file.truncate(size)
    tool([str(E2FS / "mke2fs"), "-q", "-F", "-t", "ext4", "-b", "4096",
          "-O", "^metadata_csum_seed,^orphan_file", "-E", "lazy_itable_init=0,lazy_journal_init=0", str(disk)])
    sentinel = OUT / "sentinel.txt"
    sentinel.write_bytes(b"KEEP-UNRELATED-CACHE-FILE\n")
    debug(disk, f"write {sentinel} /sentinel")
    if request is not None:
        request_path = OUT / (name + ".request")
        request_path.write_bytes(request)
        debug(disk, "mkdir /tv-launcher")
        debug(disk, "set_inode_field /tv-launcher mode 040700")
        debug(disk, f"write {request_path} /tv-launcher/request")
        debug(disk, f"set_inode_field /tv-launcher/request mode {mode}")
    return disk


def ramdisk(name, missing_hello=False, missing_magisk=False, production=False, restore_check=False):
    spec = importlib.util.spec_from_file_location("root_image", ROOT / "tools/make-stock-plus-root.py")
    writer = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(writer)
    entries = {}

    def add(path, mode, data=b"", device=(0, 0)):
        entries[path] = ([len(entries) + 1, mode, 0, 0, 1, 0, len(data), 0, 0, *device, len(path) + 1, 0], data)

    for path in ("dev", "proc", "sys", "cache", "launcher-cache"):
        add(path, stat.S_IFDIR | 0o755)
    add("dev/console", stat.S_IFCHR | 0o600, device=(5, 1))
    add("init", stat.S_IFREG | 0o750, (ROOT / "assets/build/launcher/production-launcher" if production else TARGET / "tv-launcher").read_bytes())
    probe = (TARGET / "launcher-probe").read_bytes()
    if not missing_magisk:
        add("launcher-magiskinit", stat.S_IFREG | 0o750, probe)
    if not missing_hello:
        add("launcher-hello", stat.S_IFREG | 0o750, probe)
    add("launcher-stock-init", stat.S_IFREG | 0o750, probe)
    if restore_check:
        candidate = ROOT / "assets/build/launcher/result"
        if not candidate.exists():
            candidate = ROOT / "assets/build/launcher/image"
        validation = candidate / "validation"
        add("check-magiskboot", stat.S_IFREG | 0o700, (validation / "magiskboot").read_bytes())
        add("check-candidate.cpio", stat.S_IFREG | 0o600, (validation / "patched.cpio").read_bytes())
        add("check-stock.cpio", stat.S_IFREG | 0o600, (validation / "stock.cpio").read_bytes())
    path = OUT / (name + ".cpio.gz")
    path.write_bytes(gzip.compress(writer.write_cpio(entries), mtime=0))
    return path


def run(name, disk, expected="android", readonly=False, extra="", missing_hello=False, missing_magisk=False,
        serial="4C0490290FA05A0C0", production=False, restore_check=False):
    initrd = ramdisk(name, missing_hello, missing_magisk, production, restore_check)
    cmdline = f"console=ttyAMA0 rdinit=/init panic=1 androidboot.serialno={serial} androidboot.tv_launcher=test-build " + extra
    command = ["qemu-system-aarch64", "-M", "virt,gic-version=3", "-cpu", "max", "-smp", "2", "-m", "1024M",
               "-kernel", str(KERNEL), "-initrd", str(initrd), "-append", cmdline,
               "-drive", f"if=none,id=cache,format=raw,file={disk}" + (",readonly=on" if readonly else ""),
               "-device", "virtio-blk-device,drive=cache", "-display", "none", "-monitor", "none", "-serial", "stdio", "-no-reboot"]
    if production:
        command[0] = "qemu-system-arm"
        command[2] = "virt,highmem=off,gic-version=2"
        command[command.index("-kernel") + 1] = str(ROOT / "assets/build/kernel/arm/zImage")
        index = command.index("-drive")
        del command[index:index + 4]
    (OUT / (name + ".command.json")).write_text(json.dumps(command, indent=2) + "\n")
    result = subprocess.run(command, capture_output=True, timeout=75)
    log = result.stdout + result.stderr
    (OUT / (name + ".log")).write_bytes(log)
    text = log.decode(errors="replace")
    assert result.returncode == 0 and f"PROBE role={expected} pid=1" in text, text[-5000:]
    assert "Kernel panic" not in text and "no_launcher_mounts=true" in text, text[-5000:]
    assert "Restarting system" in text, text[-5000:]
    if expected == "hello":
        assert "request durably consumed and verified absent" in text and "consumed_absent=true sentinel_preserved=true" in text
    if restore_check:
        assert "upstream_magisk_restore_matches_stock=true" in text
    print("PASS", name, "->", expected, flush=True)
    return {"case": name, "role": expected, "pid": 1, "mounts_clean": True}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--first-only", action="store_true")
    args = parser.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    results = [run("default", disk_image("default"))]
    if args.first_only:
        return
    results.append(run("malformed", disk_image("malformed", b"invalid\n")))
    results.append(run("wrong-build", disk_image("wrong-build", REQUEST.replace(b"test-build", b"another-build"))))
    results.append(run("wrong-permissions", disk_image("wrong-permissions", REQUEST, "0100644")))
    results.append(run("wrong-serial", disk_image("wrong-serial", REQUEST), serial="OTHER-TV"))
    results.append(run("wrong-geometry", disk_image("wrong-geometry", REQUEST, size=103809024)))
    disk = disk_image("one-shot", REQUEST)
    results.append(run("one-shot", disk, expected="hello"))
    results.append(run("one-shot-next-boot", disk))
    disk = disk_image("readonly", REQUEST)
    results.append(run("readonly", disk, readonly=True))
    results.append(run("readonly-next-boot", disk, readonly=True))
    disk = disk_image("busy-unmount", REQUEST)
    results.append(run("busy-unmount", disk, extra="launcher.test=busy-unmount"))
    results.append(run("busy-unmount-next-boot", disk))
    disk = disk_image("missing-hello", REQUEST)
    results.append(run("missing-hello", disk, missing_hello=True))
    results.append(run("missing-hello-next-boot", disk))
    results.append(run("missing-magisk", disk_image("missing-magisk"), expected="stock-fallback", missing_magisk=True))
    disk = disk_image("control-arm")
    results.append(run("control-arm", disk, extra="launcher.test=control-arm"))
    results.append(run("control-armed-next-boot", disk, expected="hello"))
    disk = disk_image("control-cancel")
    results.append(run("control-cancel", disk, extra="launcher.test=control-cancel"))
    results.append(run("control-cancel-next-boot", disk))
    results.append(run("production-default-native-arm32", disk_image("production-default-native-arm32"), production=True))
    results.append(run("upstream-restore", disk_image("upstream-restore"), extra="launcher.test=restore-image", restore_check=True))
    (OUT / "results.json").write_text(json.dumps({"ARM32_executable": True, "kernel": str(KERNEL), "compat_kernel": True,
                                                 "results": results, "TV_hardware_verified": False}, indent=2) + "\n")


if __name__ == "__main__":
    main()
