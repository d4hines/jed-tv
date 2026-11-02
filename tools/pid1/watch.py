#!/usr/bin/env python3
"""Capture the standalone PID-1 diagnostic channel over the direct Mac link."""
import argparse
import datetime
import http.client
import json
from pathlib import Path
import socket
import sys
import time
from direct_link import connect


class DirectTV(http.client.HTTPConnection):
    def connect(self):
        self.sock = connect(8765, self.timeout)


def request(method, path, body=None, timeout=1):
    connection = DirectTV("192.168.77.2", 8765, timeout=timeout)
    try:
        connection.request(method, path, body=body)
        reply = connection.getresponse()
        body = reply.read(512 * 1024)
        if reply.status != 200:
            raise RuntimeError(f"HTTP {reply.status}: {body!r}")
        return body
    finally:
        connection.close()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--seconds", type=int, default=240)
    action = parser.add_mutually_exclusive_group()
    action.add_argument("--reboot", action="store_true")
    action.add_argument("--exec", type=Path, dest="executable")
    action.add_argument("--kernel-log", action="store_true",
                        help="save the kernel ring from a binary supporting GET /kernel-log")
    args = parser.parse_args()
    if sys.platform != "darwin":
        parser.error("This helper binds the routed macOS TV debug link")
    if not 1 <= args.seconds <= 600:
        parser.error("--seconds must be 1..600")
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    output = Path(__file__).resolve().parents[2] / "assets/research/hello-pid1" / stamp
    output.mkdir(parents=True)
    print(f"Waiting on the TV debug link for PID-1 diagnostics; captures: {output}", flush=True)
    deadline, previous, seen = time.monotonic() + args.seconds, None, False
    while time.monotonic() < deadline:
        try:
            status = json.loads(request("GET", "/status"))
            if status.get("app") != "hello-pid1" or status.get("pid") != 1:
                raise RuntimeError(f"Unexpected target: {status}")
            logs = request("GET", "/logs")
            with (output / "status.jsonl").open("a") as stream:
                stream.write(json.dumps(status) + "\n")
            (output / "logs.txt").write_bytes(logs)
            seen = True
            if status["phase"] != previous:
                print(json.dumps(status), flush=True)
                previous = status["phase"]
            if args.reboot:
                print(request("POST", "/reboot").decode(), end="", flush=True)
                return
            if args.kernel_log:
                (output / "kernel.log").write_bytes(request("GET", "/kernel-log"))
                print(f"Kernel log saved: {output / 'kernel.log'}", flush=True)
                return
            if args.executable:
                program = args.executable.read_bytes()
                if not 52 <= len(program) <= 8 * 1024 * 1024 or program[:7] != b"\x7fELF\x01\x01\x01":
                    parser.error("--exec needs the static ARM32 init executable, not boot.img or an archive")
                print(request("POST", "/exec", program).decode(), end="", flush=True)
                return
        except (OSError, http.client.HTTPException, json.JSONDecodeError) as error:
            if seen:
                print(f"Diagnostic connection ended: {error}; captures retained.", flush=True)
                return
        time.sleep(0.5)
    if not seen:
        raise SystemExit("No PID-1 response before deadline. This does not distinguish kernel failure from Ethernet failure.")


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        pass
