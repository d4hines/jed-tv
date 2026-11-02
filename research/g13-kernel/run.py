#!/usr/bin/env python3
"""Boot the source-built MT5867 kernel on the partial QEMU model, offline."""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import struct
import subprocess
import socket
import sys
import tempfile
import time

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
BUILD = ROOT / "assets/build/g13-kernel"
sys.path.insert(0, str(ROOT / "tools/bootloader"))
from qemu import cpio, read_cpio, PINS

LIVE = ROOT / "assets/research/g13-leanboot-os/live.dtb"
LIVE_SHA = "96c47e15278b28a82b38799506af1517dda77bd34f353b94199d594dc95fdc1b"
SKIP = ["mstar_mci_init", "mmc_sdio_init", "mtktv_regulator_init",
        "mtktv_thermal_init", "ir_dynamic_config_init", "mtk_wifi_ctl_init"]


def digest(path):
    return hashlib.file_digest(path.open("rb"), "sha256").hexdigest()


def registers(path):
    with socket.socket(socket.AF_UNIX) as sock:
        sock.settimeout(2)
        sock.connect(str(path))
        with sock.makefile("rw") as stream:
            json.loads(stream.readline())
            def command(name, arguments=None):
                stream.write(json.dumps({"execute": name, "arguments": arguments or {}}) + "\n")
                stream.flush()
                while True:
                    reply = json.loads(stream.readline())
                    if "return" in reply:
                        return reply["return"]
                    if "error" in reply:
                        raise RuntimeError(reply["error"])
            command("qmp_capabilities")
            return command("human-monitor-command", {"command-line": "info registers -a"})


def ramdisk(cpus):
    path = ROOT / "assets/build/leanboot/qemu-input/initrd.gz"
    assert digest(path) == PINS["initrd.gz"], "Unreviewed BusyBox input"
    entries = read_cpio(gzip.decompress(path.read_bytes()))
    script = r'''#!/bin/busybox sh
export PATH=/bin
set -eu
mount -t proc proc /proc
mount -t sysfs sysfs /sys
# Android's captured configuration has no devtmpfs; supply minimal nodes.
mknod /dev/null c 1 3
mknod /dev/zero c 1 5
mount -t tmpfs tmpfs /tmp
mount -t debugfs debugfs /sys/kernel/debug
cat /sys/kernel/debug/sync/info
echo SOURCE_KERNEL_PID1=$$
test "$$" = 1
uname -a
uname -r | grep '^5\.15\.148'
echo ONLINE_CPUS=$(cat /sys/devices/system/cpu/online)
test "$(grep -c '^processor' /proc/cpuinfo)" = CPUS_EXPECTED
echo TIMER_BEFORE=$(cat /proc/uptime)
sleep 1 &
wait $!
echo TIMER_AFTER=$(cat /proc/uptime)
/bin/busybox sh -c 'echo child-exec-ok > /tmp/child'
grep '^child-exec-ok$' /tmp/child
dd if=/dev/zero of=/tmp/test bs=1024 count=64
test "$(wc -c < /tmp/test)" = 65536
cat /proc/meminfo
echo SOURCE_KERNEL_QEMU_PASS
while :; do sleep 10; done
'''.replace("CPUS_EXPECTED", str(cpus)).encode()
    selected = [(name, 0o40755, b"") for name in
                ("bin", "lib", "lib/arm-linux-gnueabihf", "dev", "proc", "sys", "tmp")]
    selected += [("dev/console", 0o20600, b""), ("init", 0o100755, script),
                 ("bin/busybox", *entries["usr/bin/busybox"]),
                 ("lib/ld-linux-armhf.so.3", *entries["usr/lib/arm-linux-gnueabihf/ld-linux-armhf.so.3"]),
                 ("lib/arm-linux-gnueabihf/libc.so.6", *entries["usr/lib/arm-linux-gnueabihf/libc.so.6"])]
    selected += [("bin/" + name, 0o120777, b"busybox") for name in
                 ("sh", "mount", "mknod", "cat", "grep", "uname", "sleep", "dd", "wc")]
    return gzip.compress(cpio(selected), mtime=0)


def main():
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--timeout", type=float, default=90)
    ap.add_argument("--cpus", type=int, choices=[1, 4], default=4)
    ap.add_argument("--extra", default="")
    args = ap.parse_args()
    qemu = ROOT / "assets/build/qemu-g06/build/qemu-system-aarch64"
    kernel = BUILD / "out/arch/arm/boot/Image"
    assert qemu.is_file() and kernel.is_file()
    assert digest(LIVE) == LIVE_SHA, "Unreviewed board reference"
    out = BUILD / ("qemu-" + time.strftime("%Y%m%dT%H%M%S", time.gmtime()))
    out.mkdir()
    (out / "Image").write_bytes(kernel.read_bytes())
    kernel = out / "Image"
    (out / "kernel.config").write_bytes((BUILD / "out/.config").read_bytes())
    dtb = out / "board.dtb"
    dtb.write_bytes(LIVE.read_bytes())
    original = subprocess.check_output(["fdtget", "-t", "s", str(dtb), "/chosen", "bootargs"], text=True).split()
    keep = {"LX_MEM", "LX_MEM2", "EMAC_MEM", "CMA0", "CMA1", "CMA2", "DMA_SIZE",
            "SD_CONFIG", "platform", "Pmode", "SAR5", "PM51_ADDR", "PM51_LEN", "cma", "vmalloc"}
    cmdline = " ".join(s for s in original if s.split("=")[0] in keep)
    cmdline += " console=ttyS0,115200 loglevel=7 rdinit=/init root=/dev/ram rw panic=-1 initcall_debug"
    cmdline += " initcall_blacklist=" + ",".join(SKIP) + " " + args.extra
    ram = out / "initramfs.cpio.gz"
    ram.write_bytes(ramdisk(args.cpus))
    for prop, value in [("linux,initrd-start", 0x24000000),
                        ("linux,initrd-end", 0x24000000 + ram.stat().st_size)]:
        subprocess.run(["fdtput", "-t", "x", str(dtb), "/chosen", prop, f"{value:x}"], check=True)
    subprocess.run(["fdtput", "-t", "s", str(dtb), "/chosen", "bootargs", cmdline], check=True)
    stub = out / "entry.bin"
    stub.write_bytes(struct.pack("<6I", 0xe3a00000, 0xe3e01000, 0xe59f2000,
                                 0xe59ff000, 0x23000000, 0x20208000))
    serial = out / "serial.log"
    control = tempfile.TemporaryDirectory(prefix="g13-kernel-qmp-")
    qmp = Path(control.name) / "socket"
    cmd = [str(qemu), "-M", "g06", "-smp", str(args.cpus), "-display", "none", "-monitor", "none",
           "-nic", "none", "-no-reboot", "-serial", "file:" + str(serial),
           "-qmp", f"unix:{qmp},server=on,wait=off"]
    for path, addr in [(kernel, 0x20208000), (dtb, 0x23000000), (ram, 0x24000000), (stub, 0x20200000)]:
        cmd += ["-device", f"loader,file={path},addr={addr:#x},force-raw=on" + (",cpu-num=0" if path == stub else "")]
    (out / "cmd.json").write_text(json.dumps(cmd, indent=2) + "\n")
    report = {"source": json.loads((BUILD / "source-provenance.json").read_text()),
              "kernel_sha256": digest(kernel), "config_sha256": digest(out / "kernel.config"),
              "qemu_sha256": digest(qemu), "dtb_sha256": digest(dtb),
              "initramfs_sha256": digest(ram), "cpus": args.cpus,
              "disabled_initcalls": SKIP, "extra_cmdline": args.extra,
              "model": "Partial MT5867 G06 machine; G13 captured memory layout and device tree",
              "limitations": "No physical display/audio/storage/network validation; RIU largely backing cells",
              "tv_accessed": False}
    print(out, flush=True)
    start = time.monotonic()
    with (out / "qemu.log").open("w") as err:
        proc = subprocess.Popen(cmd, stdout=err, stderr=err)
        try:
            while proc.poll() is None and time.monotonic() - start < args.timeout:
                text = serial.read_text(errors="replace") if serial.exists() else ""
                if any(marker in text for marker in ["SOURCE_KERNEL_QEMU_PASS", "Kernel panic", "Internal error:"]):
                    break
                time.sleep(0.1)
        finally:
            try:
                (out / "registers.txt").write_text(registers(qmp))
            except (OSError, RuntimeError, ValueError) as error:
                report["register_capture_error"] = str(error)
            proc.terminate()
            try:
                proc.wait(timeout=3)
            except subprocess.TimeoutExpired:
                proc.kill()
                proc.wait()
            control.cleanup()
    text = serial.read_text(errors="replace") if serial.exists() else ""
    report["elapsed_seconds"] = round(time.monotonic() - start, 3)
    report["pass"] = "SOURCE_KERNEL_QEMU_PASS" in text and not any(x in text for x in ["Kernel panic", "Internal error:"])
    (out / "result.json").write_text(json.dumps(report, indent=2) + "\n")
    print("\n".join(text.splitlines()[-35:]))
    print("PASS" if report["pass"] else "FAIL")
    return 0 if report["pass"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
