#!/usr/bin/env python3
"""Exercise the AudioBridge O/W/D/R/F protocol using silent PCM only.

No endpoint is contacted without --run. Examples:
  python3 tools/audio/check-bridge.py --self-test
  python3 tools/audio/check-bridge.py --run --host 127.0.0.1 --port 8878 \
      --output assets/build/audio-contract.json

This checks protocol and software counters, not speaker audibility or volume.
"""
from __future__ import annotations

import argparse
import json
from pathlib import Path
import socket
import socketserver
import struct
import threading
import time


ROOT = Path(__file__).resolve().parents[2]
LIMIT = 1_048_576


class Client:
    def __init__(self, host: str, port: int, timeout: float):
        self.socket = socket.create_connection((host, port), timeout=timeout)
        self.socket.settimeout(timeout)
        self.socket.setsockopt(socket.IPPROTO_TCP, socket.TCP_NODELAY, 1)

    def close(self):
        self.socket.close()

    def send(self, data: bytes, fragmented: bool = False):
        if not fragmented:
            self.socket.sendall(data)
            return
        sizes = (1, 2, 7, 31, 997)
        offset = index = 0
        while offset < len(data):
            size = sizes[index % len(sizes)]
            self.socket.sendall(data[offset:offset + size])
            offset += size
            index += 1

    def read(self, size: int) -> bytes:
        chunks = bytearray()
        while len(chunks) < size:
            part = self.socket.recv(size - len(chunks))
            if not part:
                raise EOFError(f"bridge closed after {len(chunks)}/{size} response bytes")
            chunks.extend(part)
        return bytes(chunks)

    def status(self):
        status, = struct.unpack("<i", self.read(4))
        if status:
            raise AssertionError(f"operation status {status}")

    def open(self, rate: int, channels: int, boost: int = 1, fragmented: bool = False):
        self.send(b"O" + struct.pack("<III", rate, channels, boost), fragmented)
        self.status()

    def silence(self, frames: int, channels: int, fragmented: bool = False):
        payload = bytes(frames * channels * 2)
        self.send(b"W" + struct.pack("<I", len(payload)) + payload, fragmented)
        self.status()

    def counters(self) -> tuple[int, int]:
        self.send(b"D")
        return struct.unpack("<QQ", self.read(16))

    def reset(self):
        self.send(b"R")
        self.status()

    def drain(self):
        self.send(b"F")
        self.status()


def format_case(host: str, port: int, timeout: float, rate: int, channels: int) -> dict:
    client = Client(host, port, timeout)
    try:
        client.open(rate, channels, fragmented=True)
        initial = client.counters()
        assert initial == (0, 0), ("nonzero initial counters", initial)
        # NativePcm primes after OPEN and RESET with rate/4 silent frames.
        # The server must not add its own priming to the exposed write counter.
        prime = rate // 4
        client.silence(prime, channels, fragmented=True)
        first = client.counters()
        assert first[0] == prime and 0 <= first[1] <= first[0], first
        extra = max(1, rate // 50)
        client.silence(extra, channels, fragmented=True)
        second = client.counters()
        assert second[0] == prime + extra, second
        assert first[1] <= second[1] <= second[0], (first, second)
        time.sleep(0.05)
        third = client.counters()
        assert third[0] == second[0] and second[1] <= third[1] <= third[0], third
        # F has a five-second server deadline in the existing Java protocol.
        client.socket.settimeout(max(timeout, 6.0))
        client.drain()
        drained = client.counters()
        assert drained == (prime + extra, prime + extra), drained
        client.reset()
        reset = client.counters()
        assert reset == (0, 0), ("reset did not clear counters", reset)
        client.silence(prime, channels)
        client.drain()
        after_reset = client.counters()
        assert after_reset == (prime, prime), after_reset
        return {"rate": rate, "channels": channels, "initial": initial,
                "snapshots": [first, second, third], "drained": drained,
                "reset": reset, "after_reset": after_reset}
    finally:
        client.close()


def rejected_case(host: str, port: int, timeout: float, payload: bytes,
                  opened: bool = False, half_close: bool = False) -> dict:
    client = Client(host, port, timeout)
    try:
        if opened:
            client.open(48000, 2)
        started = time.monotonic()
        client.send(payload, fragmented=True)
        if half_close:
            client.socket.shutdown(socket.SHUT_WR)
        try:
            response = client.socket.recv(4)
            if response:
                while len(response) < 4:
                    more = client.socket.recv(4 - len(response))
                    if not more:
                        raise AssertionError("partial error status")
                    response += more
                value, = struct.unpack("<i", response)
                assert value != 0, "invalid request was acknowledged as successful"
                disposition = f"status {value}"
            else:
                disposition = "closed"
        except (ConnectionResetError, BrokenPipeError):
            disposition = "reset"
        return {"disposition": disposition,
                "elapsed_ms": round((time.monotonic() - started) * 1000, 2)}
    finally:
        client.close()


def run_suite(host: str, port: int, timeout: float) -> dict:
    report = {"endpoint": {"host": host, "port": port}, "silent_pcm_only": True,
              "scope": "wire protocol and software frame counters; not audibility",
              "cases": []}

    def case(name, work):
        started = time.monotonic()
        try:
            details = work()
            entry = {"name": name, "passed": True, "details": details}
        except Exception as error:
            entry = {"name": name, "passed": False,
                     "error": f"{type(error).__name__}: {error}"}
        entry["elapsed_ms"] = round((time.monotonic() - started) * 1000, 2)
        report["cases"].append(entry)
        print(f"{'PASS' if entry['passed'] else 'FAIL'}: {name}", flush=True)

    for rate in (44100, 48000):
        for channels in (1, 2):
            case(f"{rate}Hz/{channels}ch fragmented PCM, counters, reset, drain",
                 lambda r=rate, c=channels: format_case(host, port, timeout, r, c))
    malformed = [
        ("invalid first command", b"X", False, False),
        ("invalid sample rate", b"O" + struct.pack("<III", 7999, 2, 1), False, False),
        ("invalid channel count", b"O" + struct.pack("<III", 48000, 3, 1), False, False),
        ("invalid boost", b"O" + struct.pack("<III", 48000, 2, 0), False, False),
        ("truncated open", b"O\x80\xbb", False, True),
        ("unknown stream command", b"X", True, False),
        ("oversized PCM", b"W" + struct.pack("<I", LIMIT + 4), True, False),
        ("misaligned PCM", b"W" + struct.pack("<I", 3), True, False),
        ("truncated PCM", b"W" + struct.pack("<I", 16) + bytes(4), True, True),
    ]
    for name, payload, opened, half_close in malformed:
        case(name, lambda p=payload, o=opened, h=half_close:
             rejected_case(host, port, timeout, p, o, h))
    case("valid client after malformed requests",
         lambda: format_case(host, port, timeout, 48000, 2))
    report["passed"] = all(item["passed"] for item in report["cases"])
    return report


class MockPeer(socketserver.BaseRequestHandler):
    """Strict in-process software peer for checking this test harness itself."""
    def handle(self):
        self.request.settimeout(2)

        def read(n):
            result = bytearray()
            while len(result) < n:
                block = self.request.recv(n - len(result))
                if not block:
                    raise EOFError
                result.extend(block)
            return bytes(result)

        def status(value=0):
            self.request.sendall(struct.pack("<i", value))

        try:
            if read(1) != b"O":
                return
            rate, channels, boost = struct.unpack("<III", read(12))
            if not 8000 <= rate <= 96000 or channels not in (1, 2) or not 1 <= boost <= 4:
                return
            status()
            sent = 0
            start = None
            while True:
                command = read(1)
                if command == b"W":
                    count, = struct.unpack("<I", read(4))
                    if count > LIMIT or count % (channels * 2):
                        return
                    payload = read(count)
                    assert not any(payload), "harness generated nonzero audio"
                    sent += count // (channels * 2)
                    if start is None:
                        start = time.monotonic()
                    status()
                elif command == b"D":
                    played = min(sent, int((time.monotonic() - start) * rate)) if start else 0
                    self.request.sendall(struct.pack("<QQ", sent, played))
                elif command == b"R":
                    sent, start = 0, None
                    status()
                elif command == b"F":
                    if start:
                        time.sleep(max(0, sent / rate - (time.monotonic() - start)))
                    status()
                else:
                    return
        except (EOFError, OSError):
            pass


class MockServer(socketserver.ThreadingTCPServer):
    daemon_threads = True
    allow_reuse_address = True


def output_path(value: str) -> Path:
    path = Path(value)
    if not path.is_absolute():
        path = ROOT / path
    path = path.resolve()
    if not path.is_relative_to(ROOT / "assets"):
        raise argparse.ArgumentTypeError("report must be under this repository's assets/")
    return path


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--self-test", action="store_true", help="contact only a local mock created by this process")
    mode.add_argument("--run", action="store_true", help="explicitly enable endpoint testing")
    parser.add_argument("--host", help="required with --run; no default TV address")
    parser.add_argument("--port", type=int, help="required with --run")
    parser.add_argument("--timeout", type=float, default=2.0, help="per-operation timeout, except the bounded six-second drain")
    parser.add_argument("--output", type=output_path, default=ROOT / "assets/build/audio-contract.json")
    args = parser.parse_args()
    if args.timeout <= 0 or args.timeout > 10:
        parser.error("timeout must be in (0, 10] seconds")
    if args.run and (not args.host or args.port is None or not 1 <= args.port <= 65535):
        parser.error("--run requires --host and --port (1..65535)")
    if args.self_test:
        with MockServer(("127.0.0.1", 0), MockPeer) as server:
            thread = threading.Thread(target=server.serve_forever, daemon=True)
            thread.start()
            try:
                report = run_suite(*server.server_address, args.timeout)
            finally:
                server.shutdown()
                thread.join(timeout=2)
        report["self_test"] = True
    else:
        report = run_suite(args.host, args.port, args.timeout)
        report["self_test"] = False
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + "\n")
    print(f"Report: {args.output}")
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
