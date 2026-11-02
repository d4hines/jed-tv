"""Shared offline G06 approximations; never used to package a TV image."""
import hashlib
import importlib.util
import json
import lzma
from pathlib import Path
import stat
import subprocess

ROOT = Path(__file__).resolve().parents[2]
NOTICE = "g06-compat: ARM32 post-4.19 syscalls return ENOSYS; legacy clock/thread fallback passed; PID 1 retained"
RTC = "2025-01-17T00:00:00"


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def require_frame_kernel(kernel):
    """Frame-lease tests use the versioned kernel with POSIX file locking."""
    expected = hashlib.sha256(lzma.decompress((ROOT / 'demo/kernel/zImage.xz').read_bytes())).hexdigest()
    actual = sha(kernel)
    if actual != expected:
        raise RuntimeError(
            f'Frame-lease tests require the current demo/kernel/zImage.xz (CONFIG_FILE_LOCKING=y). '
            f'{kernel} has SHA256 {actual}; expected {expected}. '
            'Rebuild the media with: nix build .#pid1.passthru.media --no-link --print-out-paths')


def build_trampoline(output):
    output.parent.mkdir(parents=True, exist_ok=True)
    subprocess.run(["rustc", "--edition=2021", "--target", "armv7-unknown-linux-musleabihf",
                    "-C", "target-cpu=cortex-a55", "-C", "linker=rust-lld",
                    "-C", "opt-level=2", "-C", "panic=abort",
                    str(ROOT / "test/fixtures/qemu/g06-init.rs"), "-o", str(output)], check=True)


def wrap_ramdisk(data, trampoline, payload=None):
    spec = importlib.util.spec_from_file_location("root_image", ROOT / "tools/make-stock-plus-root.py")
    writer = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(writer)
    entries = writer.read_cpio(data)
    original = entries.pop("init")
    next_inode = max(fields[0] for fields, _ in entries.values()) + 1

    def add(name, mode, content=b""):
        nonlocal next_inode
        entries[name] = ([next_inode, mode, 0, 0, 1, 0, len(content), 0, 0, 0, 0, len(name) + 1, 0], content)
        next_inode += 1

    if "sbin" not in entries:
        add("sbin", stat.S_IFDIR | 0o755)
    add("sbin/g06-payload", stat.S_IFREG | 0o755,
        Path(payload).read_bytes() if payload else original[1])
    add("init", stat.S_IFREG | 0o755, Path(trampoline).read_bytes())
    return writer.write_cpio(entries)


def record(output, kernel, trampoline, payload, framebuffer):
    (output / "fidelity.json").write_text(json.dumps({
        "target": "original G06/V109 ARM32", "hardware_verified": False,
        "kernel_sha256": sha(kernel), "trampoline_sha256": sha(trampoline),
        "initramfs_sha256": sha(output / "initramfs.cpio"),
        "payload_sha256": sha(payload), "rtc_base": RTC,
        "network": "userspace static .2, host .1 on 192.168.77.0/24; ip=off",
        "syscall_absence": "ARM EABI 403..0xeffff return ENOSYS; ARM-private calls retained",
        "framebuffer": "virtio-gpu" if framebuffer else "deliberately absent",
        "gaps": ["newer Linux kernel, not stock 4.19", "no MediaTek board/MMIO or reserved-memory model",
                 "no MI module/configuration/IOMMU/panel emulation", "no target watchdog emulation",
                 "TCG timings do not predict hardware speed", "no_new_privs inherited from test trampoline"],
    }, indent=2) + "\n")
