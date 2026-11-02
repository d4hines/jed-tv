#!/usr/bin/env python3
"""Loopback-only transport checks: delayed commit and lost final reply."""
import sys
sys.dont_write_bytecode = True
import socket
import struct
import threading
import time
from send import MAGIC, send


def run(answer):
    blob = bytes(range(256))*5
    stop = threading.Event()
    errors, seals = [], []
    with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as server:
        server.bind(('127.0.0.1', 0)); server.settimeout(.05)
        port = server.getsockname()[1]
        def receive():
            data = bytearray(); started = None; dropped = False
            try:
                while not stop.is_set():
                    try: request, peer = server.recvfrom(2048)
                    except TimeoutError: continue
                    magic, op, session, offset, count = struct.unpack('<5I', request[:20])
                    assert magic == MAGIC
                    if op == 1: ack = 0
                    elif op == 2:
                        assert offset == len(data)
                        data.extend(request[20:]); ack = len(data)
                    else:
                        assert op == 3 and data == blob
                        seals.append(time.monotonic())
                        if started is None: started = seals[-1]
                        if not answer or seals[-1]-started < .15: continue
                        if not dropped: dropped = True; continue
                        ack = len(data)
                    server.sendto(struct.pack('<5I', MAGIC, session, op, 0, ack), peer)
            except BaseException as error: errors.append(error)
        thread = threading.Thread(target=receive); thread.start()
        start = time.monotonic()
        try:
            try:
                send(blob, '127.0.0.1', port, timeout=.05, retries=3,
                     commit_timeout=2 if answer else .3)
                assert answer, 'A silent commit was accepted'
            except TimeoutError:
                assert not answer, 'Did not retry the lost commit reply'
        finally:
            stop.set(); thread.join(2)
        assert not thread.is_alive() and not errors, errors
        assert len(seals) >= 3, seals
        assert time.monotonic()-start < 3


if __name__ == '__main__':
    run(True); run(False)
    print('PASS: delayed commit, lost reply, repeated Seal and bounded silent-commit deadline')
