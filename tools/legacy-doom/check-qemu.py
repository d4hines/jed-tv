#!/usr/bin/env python3
"""Exercise the actual Doom ramdisk as PID 1 on ARM32 QEMU, without sound/network."""
import gzip
import json
from pathlib import Path
import socket
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[2]
IMAGE = Path(sys.argv[1] if len(sys.argv) > 1 else "assets/build/doom-minimal/result").resolve()
OUT = ROOT / "assets/build/doom-minimal/qemu"
OUT.mkdir(parents=True, exist_ok=True)
initrd = OUT / "initramfs.cpio"
initrd.write_bytes(gzip.decompress((IMAGE / "initramfs.cpio.gz").read_bytes()))
with tempfile.TemporaryDirectory(prefix="doom-qmp-") as temporary:
    sock = str(Path(temporary) / "qmp")
    command = ["qemu-system-arm", "-M", "virt,highmem=off,gic-version=2", "-cpu", "max",
               "-smp", "4", "-m", "1430M", "-global", "virtio-mmio.force-legacy=false",
               "-kernel", str(ROOT / "assets/build/kernel/arm/zImage"), "-initrd", str(initrd),
               "-append", "console=ttyAMA0 rdinit=/init panic=0 drm_kms_helper.drm_fbdev_overalloc=200 doom.modules=0 doom.args=-warp,1,1",
               "-nic", "none", "-device", "virtio-gpu-device,xres=1280,yres=720",
               "-device", "virtio-keyboard-device", "-display", "none", "-monitor", "none",
               "-serial", "stdio", "-no-reboot", "-qmp", f"unix:{sock},server=on,wait=off"]
    (OUT / "command.json").write_text(json.dumps(command, indent=2) + "\n")
    with (OUT / "serial.log").open("w") as log:
        process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT)
        try:
            connection = socket.socket(socket.AF_UNIX)
            deadline = time.monotonic() + 15
            while True:
                try:
                    connection.connect(sock)
                    break
                except (FileNotFoundError, ConnectionRefusedError):
                    if time.monotonic() >= deadline: raise
                    time.sleep(0.1)
            channel = connection.makefile("rw")
            channel.readline()

            def qmp(name, **arguments):
                channel.write(json.dumps({"execute": name, "arguments": arguments}) + "\n")
                channel.flush()
                while True:
                    response = json.loads(channel.readline())
                    if "error" in response: raise RuntimeError(response)
                    if "return" in response: return response["return"]

            def wait_for(text, seconds=120):
                deadline = time.monotonic() + seconds
                while text not in (OUT / "serial.log").read_text():
                    if "engine exited: Error" in (OUT / "serial.log").read_text():
                        raise RuntimeError("Doom startup failed; inspect serial.log")
                    if process.poll() is not None or time.monotonic() >= deadline:
                        raise RuntimeError("Guest did not reach: " + text)
                    time.sleep(0.5)

            def key(name, hold=100):
                qmp("send-key", keys=[{"type": "qcode", "data": name}], **{"hold-time": hold})
                time.sleep(0.6 + hold / 1000)

            qmp("qmp_capabilities")
            wait_for("doom: engine up")
            time.sleep(4)
            qmp("screendump", filename=str(OUT / "game.png"), format="png")
            key("up", 1500)
            key("ctrl", 300)
            qmp("screendump", filename=str(OUT / "after-input.png"), format="png")
            key("esc")
            key("q")
            key("ret")
            key("y")
            wait_for("PID 1 still running", 20)
            qmp("screendump", filename=str(OUT / "after-quit.png"), format="png")
            serial = (OUT / "serial.log").read_text()
            if "pid 1" not in serial or "audio:" in serial or "Kernel panic" in serial:
                raise RuntimeError("PID-1/silent boot check failed")
            assert (OUT / "game.png").read_bytes() != (OUT / "after-input.png").read_bytes()
            print("PASS: ARM32 Doom PID 1, game, input, quit without panic; no sound or network devices")
            print(OUT)
        finally:
            process.terminate()
            try: process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                process.kill()
                process.wait()
