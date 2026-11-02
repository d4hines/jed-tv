#!/usr/bin/env python3
"""Verify PID-1 diagnostics, drawing, display failure, and normal reboot in QEMU."""
import argparse
import gzip
import http.client
import json
import os
import re
from pathlib import Path
import socket
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/qemu"))
import g06
KIT = ROOT / "assets/build/hello-pid1/kit"
OUT = ROOT / "assets/build/hello-pid1/qemu" / f"g06-{time.time_ns()}"
TRAMPOLINE = OUT / "g06-init"
PAYLOAD = KIT / "init"


def run_case(name, modules, return_mode="stock", display_delay=0, framebuffer=True):
    output = OUT / name
    output.mkdir(parents=True, exist_ok=True)
    initrd = output / "initramfs.cpio"
    initrd.write_bytes(g06.wrap_ramdisk(gzip.decompress((KIT / "initramfs.cpio.gz").read_bytes()),
                                      TRAMPOLINE, PAYLOAD))
    kernel = ROOT / "assets/build/kernel/arm/zImage"
    g06.record(output, kernel, TRAMPOLINE, PAYLOAD, framebuffer)
    with socket.socket() as free:
        free.bind(("127.0.0.1", 0))
        port = free.getsockname()[1]
    with tempfile.TemporaryDirectory(prefix="hello-qmp-") as temporary:
        qmp_path = str(Path(temporary) / "qmp")
        command = ["qemu-system-arm", "-M", "virt,highmem=off,gic-version=2", "-cpu", "max", "-smp", "4", "-m", "1430M",
                   "-global", "virtio-mmio.force-legacy=false", "-kernel", str(ROOT / "assets/build/kernel/arm/zImage"),
                   "-initrd", str(initrd), "-append", f"console=ttyAMA0 rdinit=/init ip=off panic=3 drm_kms_helper.drm_fbdev_overalloc=200 hello.ip=192.168.77.2 hello.peer=192.168.77.1 hello.modules={modules} hello.timeout=20 hello.return={return_mode} hello.display_delay={display_delay}",
                   "-netdev", f"user,id=net,net=192.168.77.0/24,host=192.168.77.1,dhcpstart=192.168.77.15,hostfwd=tcp:127.0.0.1:{port}-192.168.77.2:8765", "-device", "virtio-net-device,netdev=net",
                   "-rtc", f"base={g06.RTC},clock=vm", "-display", "none", "-monitor", "none", "-serial", "stdio",
                   "-no-reboot", "-qmp", f"unix:{qmp_path},server=on,wait=off"]
        if framebuffer:
            command.extend(["-device", "virtio-gpu-device,xres=1280,yres=720"])
        (output / "command.json").write_text(json.dumps(command, indent=2) + "\n")
        with (output / "serial.log").open("w") as log:
            process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT)
            try:
                def request(method, path, payload=None, expected_status=200):
                    connection = http.client.HTTPConnection("127.0.0.1", port, timeout=1)
                    try:
                        connection.request(method, path, body=payload)
                        response = connection.getresponse()
                        body = response.read()
                        assert response.status == expected_status, body
                        return body
                    finally: connection.close()

                expected = "hello-visible" if modules == 0 and framebuffer else "display-failed"
                if display_delay:
                    deadline = time.monotonic() + 10
                    early = None
                    while time.monotonic() < deadline:
                        try:
                            early = json.loads(request("GET", "/status"))
                            if early["phase"] == "display-delay": break
                        except (OSError, http.client.HTTPException): pass
                        time.sleep(0.1)
                    assert early and early["pid"] == 1 and early["phase"] == "display-delay", early
                    (output / "before-display.json").write_text(json.dumps(early, indent=2) + "\n")
                    assert b"opening /dev/" not in request("GET", "/logs")
                deadline = time.monotonic() + 30
                status = None
                while time.monotonic() < deadline:
                    try:
                        status = json.loads(request("GET", "/status"))
                        if status["phase"] == expected: break
                    except (OSError, http.client.HTTPException, json.JSONDecodeError): pass
                    if process.poll() is not None: raise RuntimeError("Guest exited before diagnostics were ready")
                    time.sleep(0.25)
                assert status and status["phase"] == expected and status["pid"] == 1, status
                (output / "status.json").write_text(json.dumps(status, indent=2) + "\n")
                (output / "debug.log").write_bytes(request("GET", "/logs"))
                kernel_log = request("GET", "/kernel-log")
                assert b"Linux version" in kernel_log
                assert b"Linux version" in request("GET", "/kernel-log")
                (output / "kernel.log").write_bytes(kernel_log)
                if name == "success":
                    request("POST", "/exec", b"not an executable", expected_status=400)
                    program = PAYLOAD.read_bytes()
                    # Replace PID 1 twice, including while executing the previous upload path.
                    for generation in range(2):
                        time.sleep(1.5)
                        before = json.loads(request("GET", "/status"))
                        request("POST", "/exec", program)
                        deadline = time.monotonic() + 10
                        after = None
                        while time.monotonic() < deadline:
                            try:
                                after = json.loads(request("GET", "/status"))
                                if after["phase"] == expected and after["uptime_ms"] < before["uptime_ms"]: break
                            except (OSError, http.client.HTTPException): pass
                            time.sleep(0.1)
                        assert after and after["pid"] == 1 and after["phase"] == expected and after["uptime_ms"] < before["uptime_ms"], after
                        (output / f"exec-{generation + 1}.json").write_text(json.dumps({"before": before, "after": after}, indent=2) + "\n")
                    print("PASS: rejected invalid upload; two native uploads replaced PID 1 without reboot and retained HTTP/display", flush=True)
                if expected == "hello-visible":
                    q = socket.socket(socket.AF_UNIX)
                    q.connect(qmp_path)
                    channel = q.makefile("rw")
                    channel.readline()
                    for cmd in ({"execute": "qmp_capabilities"},
                                {"execute": "screendump", "arguments": {"filename": str(output / "screen.png"), "format": "png"}}):
                        channel.write(json.dumps(cmd) + "\n"); channel.flush()
                        while True:
                            response = json.loads(channel.readline())
                            if "error" in response: raise RuntimeError(response)
                            if "return" in response: break
                    q.close()
                    (output / "reboot.txt").write_bytes(request("POST", "/reboot"))
                    process.wait(timeout=10)
                else:
                    # The display path failed, but HTTP status/logs worked; now test the independent deadline.
                    process.wait(timeout=30)
                serial = (output / "serial.log").read_text()
                assert "Kernel panic" not in serial and "Restarting system" in serial
                assert g06.NOTICE in serial, "ARM32 syscall-absence self-check did not pass"
                clock = re.search(r"setting system clock to 2025-01-17T\S+ UTC \((\d+)\)", serial)
                assert clock and 1737072000 <= int(clock[1]) < 1737072060
                if not framebuffer:
                    assert "opening /dev/fb0" in serial and "No such file or directory" in serial
                assert f"return={return_mode}" in serial
                if expected == "display-failed": assert "bounded test deadline" in serial
                print("PASS:", name, "PID 1", expected, "ARM32 legacy syscalls, HTTP/kernel logs,", "remote reboot" if expected == "hello-visible" else "automatic reboot after display failure", flush=True)
            finally:
                if process.poll() is None:
                    process.terminate()
                    try: process.wait(timeout=5)
                    except subprocess.TimeoutExpired: process.kill(); process.wait()


parser = argparse.ArgumentParser()
cases = {"success": (0, "stock", 0), "display-failure": (1, "stock", 0),
         "bootloader-return": (0, "bootloader", 0), "display-delay": (0, "stock", 3),
         "display-unavailable": (0, "stock", 0, False)}
parser.add_argument("--case", choices=["all", *cases], default="all")
parser.add_argument("--init", type=Path,
                    help="existing Hello ELF; otherwise compile current source separately from the pinned kit")
args = parser.parse_args()
chosen = args.case
if args.init:
    PAYLOAD = args.init.resolve()
else:
    target = ROOT / "assets/build/pid1-qemu"
    env = dict(os.environ, HELLO_BUILD_ID="qemu-current-source")
    if sys.platform == "darwin":
        env.setdefault("CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER", "/usr/bin/cc")
    subprocess.run(["cargo", "build", "--locked", "--offline", "--release",
                    "--target", "armv7-unknown-linux-musleabihf", "--target-dir", str(target)],
                   cwd=ROOT / "src/pid1", env=env, check=True)
    PAYLOAD = target / "armv7-unknown-linux-musleabihf/release/hello-pid1"
g06.build_trampoline(TRAMPOLINE)
print(f"QEMU captures: {OUT}", flush=True)
for name, values in cases.items():
    if chosen in ("all", name):
        run_case(name, *values)
