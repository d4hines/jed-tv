#!/usr/bin/env python3
"""Read-only TFTP server for one harmless 4096-byte marker, never firmware."""
import argparse
import hashlib
import json
import socket
import struct
import sys
import threading
import time
import zlib

NAME = "tv-probe.bin"
PAYLOAD = (b"DANDE TV TFTP RAM-TRANSFER PROBE - NOT EXECUTABLE\n" * 100)[:4096]


def event(kind, **fields):
    print(json.dumps({"event": kind, **fields}), flush=True)


def rrq(packet):
    if len(packet) < 4 or packet[:2] != b"\x00\x01":
        raise ValueError("Only read requests are supported")
    fields = packet[2:].split(b"\0")
    if len(fields) < 3 or fields[0] != NAME.encode() or fields[1].lower() != b"octet":
        raise ValueError("Only tv-probe.bin in octet mode is served")


def error(sock, address, message):
    sock.sendto(struct.pack("!HH", 5, 2) + message.encode() + b"\0", address)


def transfer(sock, address):
    sock.settimeout(1)
    for index, offset in enumerate(range(0, len(PAYLOAD) + 1, 512), 1):
        data = PAYLOAD[offset:offset + 512]
        packet = struct.pack("!HH", 3, index) + data
        for attempt in range(5):
            sock.sendto(packet, address)
            deadline = time.monotonic() + 1
            while time.monotonic() < deadline:
                sock.settimeout(max(0.01, deadline - time.monotonic()))
                try:
                    ack, source = sock.recvfrom(2048)
                except socket.timeout:
                    break
                if source == address and ack == struct.pack("!HH", 4, index):
                    break
            else:
                continue
            if 'ack' in locals() and source == address and ack == struct.pack("!HH", 4, index):
                break
        else:
            raise TimeoutError("No ACK for block " + str(index))
        if len(data) < 512:
            return


def serve_once(listener, peer, bind_ip, interface=None):
    packet, address = listener.recvfrom(4096)
    if address[0] != peer:
        error(listener, address, "Peer is not the directly connected TV")
        return False
    try:
        rrq(packet)
    except ValueError as failure:
        error(listener, address, str(failure))
        return False
    event("request", peer=list(address), filename=NAME)
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as session:
        if interface:
            bind_interface(session, interface)
        session.bind((bind_ip, 0))
        transfer(session, address)
    event("complete", peer=list(address), bytes=len(PAYLOAD), crc32=f"{zlib.crc32(PAYLOAD):08x}",
          sha256=hashlib.sha256(PAYLOAD).hexdigest())
    return True


def bind_interface(sock, interface):
    if sys.platform == "darwin":
        sock.setsockopt(socket.IPPROTO_IP, getattr(socket, "IP_BOUND_IF", 25), socket.if_nametoindex(interface))
    elif hasattr(socket, "SO_BINDTODEVICE"):
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_BINDTODEVICE, interface.encode() + b"\0")
    else:
        raise RuntimeError("Cannot bind the TFTP server to the requested interface")


def selftest():
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as listener, socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as client:
        listener.bind(("127.0.0.1", 0))
        listener.settimeout(5)
        client.settimeout(3)
        result = []
        def worker():
            result.append(serve_once(listener, "127.0.0.1", "127.0.0.1"))
        thread = threading.Thread(target=worker)
        thread.start()
        client.sendto(b"\0\1" + NAME.encode() + b"\0octet\0blksize\0" + b"1468\0", listener.getsockname())
        received = bytearray()
        expected = 1
        while True:
            packet, server = client.recvfrom(2048)
            opcode, block = struct.unpack_from("!HH", packet)
            assert opcode == 3 and block == expected
            received.extend(packet[4:])
            client.sendto(struct.pack("!HH", 4, block), server)
            expected += 1
            if len(packet) < 516:
                break
        thread.join(timeout=5)
        assert not thread.is_alive() and result == [True] and bytes(received) == PAYLOAD
        for invalid in (b"\0\2file\0octet\0", b"\0\1../secret\0octet\0", b"\0\1tv-probe.bin\0netascii\0"):
            try: rrq(invalid)
            except ValueError: pass
            else: raise AssertionError("Unsafe request accepted")
    event("selftest-passed", bytes=len(PAYLOAD), rejects="write requests, other filenames and modes")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--selftest", action="store_true")
    parser.add_argument("--bind", default="192.168.77.1")
    parser.add_argument("--peer", default="192.168.77.2")
    parser.add_argument("--interface", default="en8")
    parser.add_argument("--port", type=int, default=69)
    parser.add_argument("--seconds", type=int, default=300)
    args = parser.parse_args()
    if args.selftest:
        selftest()
        return
    if args.bind != "192.168.77.1" or args.peer != "192.168.77.2" or not 1 <= args.seconds <= 1800:
        parser.error("This probe is restricted to the isolated .1/.2 link and a bounded duration")
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as listener:
        bind_interface(listener, args.interface)
        listener.bind((args.bind, args.port))
        listener.settimeout(1)
        event("ready", bind=args.bind, port=args.port, peer=args.peer, interface=args.interface,
              filename=NAME, bytes=len(PAYLOAD), bytes_hex=hex(len(PAYLOAD)),
              crc32=f"{zlib.crc32(PAYLOAD):08x}", sha256=hashlib.sha256(PAYLOAD).hexdigest())
        deadline = time.monotonic() + args.seconds
        while time.monotonic() < deadline:
            try:
                if serve_once(listener, args.peer, args.bind, args.interface):
                    return
            except socket.timeout:
                pass
            except TimeoutError as failure:
                event("transfer-failed", reason=str(failure))


if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        print("Stopped.")
