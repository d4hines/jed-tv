#!/usr/bin/env python3
"""Verify native PID 1, persistent PTY state, job control and RAM upload in QEMU."""
import gzip
import argparse
import hashlib
import http.client
import json
from pathlib import Path
import socket
import re
import tempfile
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / "tools/qemu"))
import g06


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--init', type=Path, default=ROOT / 'assets/build/pid1-repl/target/armv7-unknown-linux-musleabihf/release/hello-pid1')
    parser.add_argument('--expect-direct-refusal', action='store_true', help='verify direct services reject this non-G06 surrogate while the console survives')
    parser.add_argument("--no-boot-entropy", action="store_true", help="disable QEMU boot entropy and CPU/bootloader trust to check cold initialization")
    parser.add_argument("--apps", type=Path, help="exercise the embedded launcher with these ARM32 applications")
    parser.add_argument("--media", type=Path, help="media derivation with tiles and graphics-enabled kernel")
    args = parser.parse_args()
    if args.apps:
        if not args.media:
            parser.error('--apps requires --media')
        g06.require_frame_kernel(args.media / 'kernel/zImage')
    out = ROOT / "assets/build/pid1-repl/qemu" / str(time.time_ns())
    out.mkdir(parents=True)
    binary = args.init.resolve()
    trampoline = out / "g06-init"
    g06.build_trampoline(trampoline)
    initrd = out / "initramfs.cpio"
    if args.apps:
        assert args.media, "--apps requires --media"
        files = [f"{args.apps}/bin/{name}:/bin/{name}" for name in ("slides", "doomtv", "ytplay")]
        files += [f"{args.media}/doom/freedoom1.wad:/doom/freedoom1.wad"]
        files += [f"{tile}:/share/tiles/{tile.name}" for tile in (args.media / "tiles").glob("*.rgb")]
        subprocess.run([sys.executable, str(ROOT / "tools/mkinitramfs.py"), str(binary), str(initrd), *files], check=True)
        data = initrd.read_bytes()
    else:
        data = gzip.decompress((ROOT / "assets/build/hello-pid1/kit/initramfs.cpio.gz").read_bytes())
    initrd.write_bytes(g06.wrap_ramdisk(data, trampoline, binary))
    reserved = []
    ports = []
    for _ in range(2):
        listener = socket.socket(); listener.bind(("127.0.0.1", 0)); reserved.append(listener)
        ports.append(listener.getsockname()[1])
    http_port, console_port = ports
    kernel = args.media / "kernel/zImage" if args.apps else ROOT / "assets/build/kernel/arm/zImage"
    command = ["qemu-system-arm", "-M", "virt,highmem=off,gic-version=2", "-cpu", "max", "-smp", "4", "-m", "1430M",
               "-global", "virtio-mmio.force-legacy=false", "-kernel", str(kernel), "-initrd", str(initrd),
               "-append", "console=ttyAMA0 rdinit=/init ip=off panic=3 hello.ip=192.168.77.2 hello.peer=192.168.77.1 hello.qemu=1 hello.timeout=90 hello.repl_timeout=120",
               "-netdev", f"user,id=n0,net=192.168.77.0/24,host=192.168.77.1,dhcpstart=192.168.77.15,hostfwd=tcp:127.0.0.1:{http_port}-192.168.77.2:8765,hostfwd=tcp:127.0.0.1:{console_port}-192.168.77.2:8766",
               "-device", "virtio-net-device,netdev=n0", "-rtc", f"base={g06.RTC},clock=vm",
               "-display", "none", "-monitor", "none", "-serial", "stdio", "-no-reboot"]
    if args.no_boot_entropy:
        command[command.index("-M") + 1] += ",dtb-randomness=off"
        command[command.index("-append") + 1] += " random.trust_cpu=off random.trust_bootloader=off"
    qmp = Path(tempfile.mkdtemp(prefix="pid1-ui-")) / "qmp"
    if args.apps:
        command[command.index("-append") + 1] += " drm_kms_helper.drm_fbdev_overalloc=200"
        command[command.index("-monitor") + 1] = "none"
        command += ["-device", "virtio-gpu-device,xres=1280,yres=720", "-device", "virtio-keyboard-device", "-qmp", f"unix:{qmp},server=on,wait=off"]
    for listener in reserved: listener.close()
    (out / "command.json").write_text(json.dumps(command, indent=2) + "\n")
    g06.record(out, kernel, trampoline, binary, bool(args.apps))

    def request(method, path, data=None):
        client = http.client.HTTPConnection("127.0.0.1", http_port, timeout=5)
        try:
            client.request(method, path, body=data)
            reply = client.getresponse(); body = reply.read()
            assert reply.status == 200, (reply.status, body)
            return body
        finally: client.close()

    transcript = bytearray()

    def receive(connection, marker=b"tv-repl# "):
        result = bytearray(); deadline = time.monotonic() + 10
        while marker not in result:
            if time.monotonic() > deadline: raise RuntimeError(f"console timeout: {result!r}")
            try: part = connection.recv(65536)
            except socket.timeout: continue
            if not part:
                time.sleep(.3)
                raise RuntimeError(f"console closed: {result!r}; events={request('GET', '/logs')!r}")
            result += part
        transcript.extend(result)
        return bytes(result)

    with (out / "serial.log").open("w") as log:
        process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT)
        try:
            deadline = time.monotonic() + 30
            status = None
            first_response = None
            while time.monotonic() < deadline:
                try:
                    status = json.loads(request("GET", "/status"))
                    if first_response is None:
                        first_response = {"status": status, "logs": request("GET", "/logs").decode(errors="replace")}
                        (out / "first-http.json").write_text(json.dumps(first_response, indent=2) + "\n")
                    if status["phase"] == "shell-ready": break
                except (OSError, http.client.HTTPException): pass
                if process.poll() is not None: raise RuntimeError((out / "serial.log").read_text())
                time.sleep(.2)
            assert status and status["pid"] == 1 and status["phase"] == "shell-ready", status
            assert b"automatic reboot in 120s" in request("GET", "/logs"), "Inherited Hello timeout overrode REPL timeout"
            first_uptime = status["uptime_ms"]
            for _ in range(30):
                volume=json.loads(request("GET","/volume"))
                if volume["ready"]: break
                time.sleep(.1)
            assert volume["owner_pid"]==1 and volume["ready"], volume
            if args.expect_direct_refusal:
                assert b"direct services refused: direct services require G06 PID1" in request("GET", "/logs")
                assert json.loads(request("GET", "/status"))["services"]["direct"] == "refused"
            with socket.create_connection(("127.0.0.1", console_port), timeout=1) as shell:
                receive(shell)
                shell.sendall(b"mkdir -p /tmp/repl-check; cd /tmp/repl-check; export REPL_VALUE=kept\n")
                receive(shell)
                shell.sendall(b"printf 'STATE=%s:%s\\n' \"$PWD\" \"$REPL_VALUE\"; id; readlink /proc/1/exe\n")
                proof = receive(shell)
                assert b"STATE=/tmp/repl-check:kept" in proof and b"uid=0" in proof and b"/sbin/g06-payload" in proof, proof
                shell.sendall(b"ifconfig lo\n")
                loopback = receive(shell)
                assert b"127.0.0.1" in loopback and b"UP LOOPBACK RUNNING" in loopback, loopback
                shell.sendall(b"printf 'abc\\n' | tr a-z A-Z\n")
                assert b"ABC" in receive(shell)
                shell.sendall(b"echo 0 > /tmp/app-volume; sleep 0.1; test -s /tmp/app-volume-osd.rgba && echo VOLUME_OVERLAY_READY\n")
                assert b"VOLUME_OVERLAY_READY\r\n" in receive(shell)
                assert json.loads(request("GET","/volume"))["gain"]==0
                shell.sendall(b"sleep 30\n"); time.sleep(.3); shell.sendall(b"\x03")
                receive(shell)
                script = b"#!/bin/sh\nprintf 'UPLOAD_OK:%s:%s\\n' \"$PWD\" \"$REPL_VALUE\"\n"
                request("PUT", "/files/upload.sh", script)
                shell.sendall(b"/tmp/upload.sh\n")
                assert b"UPLOAD_OK:/tmp/repl-check:kept" in receive(shell)
                request("PUT", "/files/busybox", (ROOT / "assets/build/pid1-repl/busybox").read_bytes())
                shell.sendall(b"/tmp/busybox echo UPLOADED_BINARY_OK\n")
                assert b"UPLOADED_BINARY_OK" in receive(shell)
                request("PUT", "/files/draw-probe", binary.read_bytes())
                shell.sendall(b"/tmp/draw-probe; echo CHILD_STATUS=$?\n" if args.apps else b"/tmp/draw-probe --draw hello.modules=0; echo CHILD_STATUS=$?\n")
                assert (b"CHILD_STATUS=2" if args.apps else b"CHILD_STATUS=1") in receive(shell)
                assert json.loads(request("GET", "/status"))["phase"] == "shell-ready"
                with socket.create_connection(("127.0.0.1", console_port), timeout=1) as second:
                    receive(second); second.sendall(b"echo SECOND_SESSION; exit\n")
                    receive(second, b"SECOND_SESSION\r\n")
                status = json.loads(request("GET", "/status"))
                assert status["pid"] == 1 and status["uptime_ms"] > first_uptime
                shell.sendall(b"echo STILL_FIRST:$REPL_VALUE\n")
                assert b"STILL_FIRST:kept" in receive(shell)
                shell.sendall(b"exit\n")
            if args.apps:
                def until(marker, count=1, timeout=40):
                    deadline = time.monotonic() + timeout
                    while time.monotonic() < deadline:
                        text = (out / "serial.log").read_text(errors="replace")
                        if text.count(marker) >= count: return text
                        time.sleep(.1)
                    raise AssertionError(f"Missing {marker}: {text[-4000:]}")
                until("home screen:")
                with socket.socket(socket.AF_UNIX) as qsocket:
                    qsocket.settimeout(10); qsocket.connect(str(qmp))
                    wire = qsocket.makefile("rw"); wire.readline()
                    def q(name, **arguments):
                        wire.write(json.dumps({"execute":name,"arguments":arguments})+"\n"); wire.flush()
                        while True:
                            response = json.loads(wire.readline())
                            if "return" in response: return response["return"]
                            if "error" in response: raise AssertionError(response)
                    q("qmp_capabilities")
                    def key(code):
                        for down in [True, False]:
                            q("input-send-event", events=[{"type":"key","data":{"down":down,"key":{"type":"qcode","data":code}}}])
                            time.sleep(.15)
                    # Exercise the actual Lean service, Rust evdev adapter and
                    # atomic gain file on every press, not only policy helpers.
                    def expect_gain(expected):
                        deadline = time.monotonic() + 2
                        while time.monotonic() < deadline:
                            actual = json.loads(request("GET", "/volume"))["gain"]
                            if abs(actual - expected) < 0.000001: return
                            time.sleep(.02)
                        raise AssertionError(f"volume gain {actual}, expected {expected}")
                    expect_gain(0)
                    for step in range(1, 11):
                        key("volumeup"); expect_gain(step * .015)
                    key("audiomute"); expect_gain(0)
                    key("audiomute"); expect_gain(.15)
                    for step in range(9, -1, -1):
                        key("volumedown"); expect_gain(step * .015)
                    assert b"volume: Lean owns global keys" in request("GET", "/logs")
                    q("screendump", filename=str(out / "home.png"), format="png")
                    key("right"); key("ret")
                    text = until("launcher spawned app pid=")
                    pid = int(re.findall(r"launcher spawned app pid=(\d+)", text)[-1])
                    with socket.create_connection(("127.0.0.1", console_port),timeout=1) as shell:
                        receive(shell)
                        shell.sendall(f"awk '/PPid:/ {{print \"APP_PARENT=\" $2}}' /proc/{pid}/status; readlink /proc/{pid}/exe; pidof tvinit; echo LAUNCHER_SHELL_OK\n".encode())
                        proof = receive(shell)
                        assert b"APP_PARENT=1\r\n" in proof and b"/bin/slides" in proof, proof
                        assert b"LAUNCHER_SHELL_OK\r\n" in proof
                        shell.sendall(b"exit\n")
                    time.sleep(2)
                    q("screendump", filename=str(out / "slides.png"), format="png")
                    # A resident launcher must stop an active app, preserve the
                    # debug server, and rebuild the menu without a new process.
                    request("POST", "/launcher/stop")
                    request("POST", "/launcher/reload")
                    until("home screen:", 2)
                    key("right"); key("ret")
                    until("confined pid", 2)
                    key("home")
                    until("Slides stopped", 2)
                    time.sleep(3)
                    q("screendump", filename=str(out / "home-after-app.png"), format="png")
                    assert json.loads(request("GET","/status"))["pid"] == 1
                    wire.close()
            time.sleep(2.2)
            logs = request("GET", "/logs")
            assert b"reaped child" in logs
            before_reload=json.loads(request("GET","/status"))["uptime_ms"]
            request("PUT","/files/pid1-live-handoff",b"verified QEMU reload\n")
            request("POST","/exec",binary.read_bytes())
            time.sleep(.4)
            deadline=time.monotonic()+10
            while time.monotonic()<deadline:
                try:
                    status=json.loads(request("GET","/status"))
                    if status["phase"]=="shell-ready" and status["uptime_ms"]<before_reload: break
                except (OSError,http.client.HTTPException): pass
                time.sleep(.1)
            else: raise RuntimeError("PID1 live reload did not restore console")
            with socket.create_connection(("127.0.0.1",console_port),timeout=1) as shell:
                receive(shell)
                shell.sendall(b"test -f /tmp/upload.sh && test ! -f /tmp/pid1-live-handoff && echo RELOAD_PRESERVED_RAM; exit\n")
                assert b"RELOAD_PRESERVED_RAM\r\n" in receive(shell,b"RELOAD_PRESERVED_RAM\r\n")
            assert json.loads(request("GET","/volume"))["owner_pid"]==1
            assert b"cold hardware startup refused after live PID1 replacement" in request("GET", "/logs")
            (out / "events.log").write_bytes(logs)
            (out / "console.log").write_bytes(transcript)
            (out / "status.json").write_text(json.dumps(status, indent=2) + "\n")
            request("POST", "/reboot")
            process.wait(timeout=10)
            serial = (out / "serial.log").read_text()
            assert "Kernel panic" not in serial and "Restarting system" in serial and g06.NOTICE in serial
            (out / "result.json").write_text(json.dumps({"passed": True, "init_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(),
                "direct_service_identity_refusal": args.expect_direct_refusal,
                "checks": ["native PID 1", "root PTY", "persistent cwd/env", "pipes", "Ctrl-C job control", "RAM script upload", "RAM ELF upload", "failed graphics child leaves shell/PID1 alive", "concurrent sessions", "orphan reaper", "HTTP while shell active", "PID1 volume overlay", "live supervisor reload preserves RAM and console", "normal reboot"],
                "embedded_launcher_tested": bool(args.apps),
                "lean_volume_keys_tested": bool(args.apps),
                "launcher_checks": ["app PPid=1", "shell usable during app", "stop/reload without process replacement", "Home returns to menu"] if args.apps else [], "hardware_watchdog_tested": False}, indent=2) + "\n")
            print(f"PASS: native PID1 REPL, persistent state, Ctrl-C, uploads and reconnect; evidence {out}")
        finally:
            if process.poll() is None:
                process.terminate()
                try: process.wait(timeout=5)
                except subprocess.TimeoutExpired: process.kill(); process.wait()


if __name__ == "__main__":
    main()
