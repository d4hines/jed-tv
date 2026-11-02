#!/usr/bin/env python3
"""Stage a G06 RAM boot; default is load/unload only, --execute performs the handoff."""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import re
import shlex
import struct
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
SERIAL = "4C0490290FA05A0C0"
SU = "/debug_ramdisk/su"
KIT = ROOT / "assets/build/hello-pid1/result"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--execute", action="store_true", help="Stop Android writers, remount disks read-only, and enter the RAM kernel")
    parser.add_argument("--payload", choices=("hello", "android", "reset-probe"), default="hello")
    parser.add_argument("--single-core", action="store_true", help="Offline secondary CPUs before handoff and boot the new kernel with maxcpus=1")
    options = parser.parse_args()
    if options.execute:
        interface = subprocess.check_output(["ifconfig", "en8"], text=True)
        assert "status: active" in interface and "inet 192.168.77.1 " in interface, "Direct Ethernet must be connected for the diagnostic watcher"
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    output = ROOT / "assets/research/kexec" / ("ram-boot" if options.execute else "load-check") / stamp
    output.mkdir(parents=True)
    remote = "/tmp/tv-kexec-hello-" + stamp
    records = []

    def root(words, data=None, check=True):
        command = words if isinstance(words, str) else shlex.join(words)
        result = subprocess.run(["adb", "-s", SERIAL, "shell", "-T", shlex.join([SU, "-c", command])],
                                input=data, capture_output=True, timeout=60)
        if data is None and not result.stdout.startswith(bytes.fromhex("d00dfeed")):
            records.append({"command": command, "rc": result.returncode,
                            "stdout": result.stdout.decode(errors="replace"), "stderr": result.stderr.decode(errors="replace")})
        (output / "commands.json").write_text(json.dumps(records, indent=2) + "\n")
        if check and result.returncode:
            raise RuntimeError(f"Command failed ({result.returncode}): {command}: {result.stderr.decode(errors='replace')}")
        return result

    for key, expected in (("ro.product.device", "G06"), ("ro.software.version_id", "V8-T221T04-LF1V109"),
                          ("sys.boot_completed", "1"), ("sys.tvroot.ready", "1")):
        assert root(["getprop", key]).stdout.decode().strip() == expected, key
    assert root(["cat", "/sys/kernel/kexec_loaded"]).stdout.strip() == b"0", "Refusing to replace a pre-existing loaded kernel"
    assert root(["cat", "/proc/sys/kernel/kexec_load_disabled"]).stdout.strip() == b"0"
    memory = root(["cat", "/proc/iomem"]).stdout.decode()
    assert "20200000-403fffff : System RAM" in memory
    assert "20208000-212fffff : Kernel code" in memory
    (output / "iomem.txt").write_text(memory)
    manifest = json.loads((KIT / "manifest.json").read_text())
    payloads = {"kernel": (KIT / "kernel").read_bytes(), "initramfs.cpio.gz": (KIT / "initramfs.cpio.gz").read_bytes()}
    assert hashlib.sha256(payloads["kernel"]).hexdigest() == manifest["kernel_sha256"] == "01e36bda0554aed46ad3aa1d7fdf688174f46241117f62b4b95f02e25c47c11e"
    assert hashlib.sha256(payloads["initramfs.cpio.gz"]).hexdigest() == manifest["initrd_sha256"]
    if options.payload == "android":
        android = ROOT / "assets/build/stock-plus-root/result"
        android_manifest = json.loads((android / "manifest.json").read_text())
        image = (android / "boot.img").read_bytes()
        assert hashlib.sha256(image).hexdigest() == android_manifest["boot_sha256"]
        kernel_size, ramdisk_size, page = [struct.unpack_from("<I", image, offset)[0] for offset in (8, 16, 36)]
        assert image[page:page + kernel_size] == payloads["kernel"]
        offset = page + (kernel_size + page - 1) // page * page
        payloads["initramfs.cpio.gz"] = image[offset:offset + ramdisk_size]
        assert payloads["initramfs.cpio.gz"][:2] == b"\x1f\x8b"
    elif options.payload == "reset-probe":
        payloads["kernel"] = (ROOT / "assets/research/kexec/reset-probe/probe.bin").read_bytes()
        assert len(payloads["kernel"]) == 64
        assert hashlib.sha256(payloads["kernel"]).hexdigest() == "4501ad430f70fc62ee6f2e4de74c0db379f69bbbed52b2c6037f9a90b177a9bc"
        del payloads["initramfs.cpio.gz"]
    payloads["dtb"] = root(["cat", "/sys/firmware/fdt"]).stdout
    assert payloads["dtb"][:4] == bytes.fromhex("d00dfeed")
    assert struct.unpack_from(">I", payloads["dtb"], 4)[0] == len(payloads["dtb"])
    words = root(["cat", "/proc/cmdline"]).stdout.decode().split()
    words = [word for word in words if word != "quiet" and not word.startswith(("panic=", "loglevel=", "hello.", "rdinit=", "maxcpus=", "androidboot.tv_ram_trial="))]
    words = ["rdinit=/init", "loglevel=7", "panic=3", "androidboot.tv_ram_trial=" + stamp] + words
    if options.payload == "hello":
        words += ["hello.timeout=90", "hello.return=stock"]
    if options.single_core:
        words += ["maxcpus=1"]
    command_line = " ".join(words)
    (output / "cmdline.txt").write_text(command_line + "\n")
    (output / "original.dtb").write_bytes(payloads["dtb"])
    # TCL's loader appends old /chosen/bootargs to --append; blank only our RAM copy.
    dtb_path = output / "dtb"
    dtb_path.write_bytes(payloads["dtb"])
    subprocess.run(["fdtput", "-t", "s", str(dtb_path), "/chosen", "bootargs", ""], check=True)
    payloads["dtb"] = dtb_path.read_bytes()
    root(["mkdir", "-m", "700", remote])
    handoff_attempted = False
    try:
        for name, data in payloads.items():
            root("cat > " + shlex.quote(remote + "/" + name), data=data)
            actual = root(["sha256sum", remote + "/" + name]).stdout.decode().split()[0]
            assert actual == hashlib.sha256(data).hexdigest(), name
        # Destinations are bounded to real first-bank RAM below the old boot initrd.
        args = ["/vendor/bin/kexec", "-d", "-l", remote + "/kernel", "--type=zImage",
                "--image-size=0x02000000", "--mem-min=0x20200000", "--mem-max=0x23ffffff",
                "--dtb=" + remote + "/dtb",
                "--append=" + command_line]
        if "initramfs.cpio.gz" in payloads:
            args.append("--initrd=" + remote + "/initramfs.cpio.gz")
        result = root(args, check=False)
        (output / "load.txt").write_bytes(result.stdout + result.stderr)
        assert result.returncode == 0, (result.stdout + result.stderr).decode(errors="replace")
        assert root(["cat", "/sys/kernel/kexec_loaded"]).stdout.strip() == b"1"
        debug = (result.stdout + result.stderr).decode()
        assert re.search(r"entry\s*=\s*0x20208000\b", debug), "Unexpected kernel entry; never execute this load"
        line = next(line for line in debug.splitlines() if line.startswith("new kernel cmdline:"))
        assert line.count("androidboot.tv_ram_trial=") == 1 and " quiet " not in line
        if options.execute:
            if options.single_core:
                root("cat > " + shlex.quote(remote + "/single-core"), data=b"1\n")
            root("cat > " + shlex.quote(remote + "/execute.sh"), data=(ROOT / "src/runtime-scripts/root/execute-kexec.sh").read_bytes())
            root(["/vendor/bin/sh", "-n", remote + "/execute.sh"])
            watch_log = (output / "watch.txt").open("wb")
            watcher = subprocess.Popen([sys.executable, str(ROOT / "tools/pid1/watch.py"), "--seconds", "240"], stdout=watch_log, stderr=subprocess.STDOUT)
            (output / "watcher.json").write_text(json.dumps({"pid": watcher.pid}) + "\n")
            handoff_attempted = True
            print("RAM handoff starting:", options.payload, "single_core=", options.single_core,
                  "diagnostics watcher PID", watcher.pid, "evidence:", output, flush=True)
            try:
                try:
                    result = root(["/vendor/bin/sh", remote + "/execute.sh"], check=False)
                    (output / "handoff.txt").write_bytes(result.stdout + result.stderr)
                    print("ADB handoff command returned", result.returncode, "; inspect handoff.txt and watch.txt", flush=True)
                except subprocess.TimeoutExpired as error:
                    (output / "handoff.txt").write_bytes((error.stdout or b"") + (error.stderr or b""))
                    print("ADB handoff command exceeded 60 seconds; watcher remains active", flush=True)
                watcher.wait(timeout=250)
                print("Diagnostic watcher finished with status", watcher.returncode, flush=True)
            finally:
                watch_log.close()
            return
        root(["/vendor/bin/kexec", "-u"])
        assert root(["cat", "/sys/kernel/kexec_loaded"]).stdout.strip() == b"0"
        assert root(["getprop", "sys.boot_completed"]).stdout.strip() == b"1"
        report = {"load": "passed", "unload": "passed", "entry": "0x20208000",
                  "boot_completed_after": 1, "executed": False, "firmware_written": False,
                  "payload": options.payload, "single_core": options.single_core,
                  "kernel_sha256": hashlib.sha256(payloads["kernel"]).hexdigest(),
                  "initrd_sha256": hashlib.sha256(payloads["initramfs.cpio.gz"]).hexdigest() if "initramfs.cpio.gz" in payloads else None}
        (output / "result.json").write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps(report, indent=2))
    finally:
        if not handoff_attempted:
            if root(["cat", "/sys/kernel/kexec_loaded"]).stdout.strip() == b"1":
                root(["/vendor/bin/kexec", "-u"])
            root(["rm", "-rf", remote])
    print("Evidence:", output)


if __name__ == "__main__":
    main()
