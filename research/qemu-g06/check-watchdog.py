#!/usr/bin/env python3
"""Exercise real QEMU MMIO writes; no TV access or invented WDT timeout."""
import hashlib
import json
import pathlib
import socket
import subprocess
import tempfile
import time

from contract import ROOT, verified_snapshot


def connect(path):
    for _ in range(100):
        sock = socket.socket(socket.AF_UNIX)
        try:
            sock.connect(str(path))
            sock.settimeout(3)
            return sock.makefile("rw")
        except (FileNotFoundError, ConnectionRefusedError):
            sock.close()
            time.sleep(.05)
    raise RuntimeError("QEMU control socket did not appear")


def main():
    observed = verified_snapshot()
    watchdog = observed["watchdog"]
    assert not watchdog["expiration_period_measured"]
    addr = watchdog["riu_physical_base"] + watchdog["clear_offset"]
    qemu = ROOT / "assets/build/qemu-g06/build/qemu-system-aarch64"
    with tempfile.TemporaryDirectory(prefix="g06-wdt-") as tmp:
        tmp = pathlib.Path(tmp)
        with (tmp / "stderr").open("w") as err:
            process = subprocess.Popen([
                str(qemu), "-M", "g06", "-smp", "4", "-S",
                "-display", "none", "-serial", "none", "-monitor", "none",
                "-qtest", f"unix:{tmp / 'qtest'},server=on,wait=off",
                "-qmp", f"unix:{tmp / 'qmp'},server=on,wait=off",
            ], stdout=err, stderr=err)
            try:
                qtest, qmp = connect(tmp / "qtest"), connect(tmp / "qmp")
                assert "QMP" in json.loads(qmp.readline())

                def command(name, **arguments):
                    qmp.write(json.dumps({"execute": name, "arguments": arguments}) + "\n")
                    qmp.flush()
                    while True:
                        reply = json.loads(qmp.readline())
                        if "return" in reply or "error" in reply:
                            return reply

                def count(name):
                    return command("qom-get", path="/machine", property=name)["return"]

                def write(kind, value):
                    qtest.write(f"{kind} {addr:#x} {value:#x}\n")
                    qtest.flush()
                    assert qtest.readline().strip() == "OK"

                command("qmp_capabilities")
                assert count("watchdog-clear-count") == 0
                for _ in range(3):
                    write("writeb", watchdog["clear_value"])
                assert count("watchdog-clear-count") == 3
                # Width and value matter; neither is evidence of a HAL clear.
                write("writew", 1)
                write("writeb", 0)
                assert count("watchdog-clear-count") == 3
                assert count("watchdog-other-write-count") == 2
                assert "error" in command("qom-set", path="/machine",
                                           property="watchdog-clear-count", value=99)
                assert process.poll() is None
                report = {"passed": True, "clear_address": hex(addr),
                          "clear_count": 3, "other_write_count": 2,
                          "expiration_model": "none; hardware interval unmeasured",
                          "qemu_sha256": hashlib.sha256(qemu.read_bytes()).hexdigest(),
                          "hardware_contract_sha256": observed["sha256"],
                          "hardware_evidence_files_verified": observed["evidence_files_verified"]}
                out = ROOT / "assets/build/qemu-g06/watchdog-check.json"
                out.write_text(json.dumps(report, indent=2) + "\n")
                print(json.dumps(report, indent=2))
            finally:
                process.terminate()
                try:
                    process.wait(timeout=3)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait()


if __name__ == "__main__":
    main()
