"""Test peer for the TV PCM wire protocol; models buffering, not speaker hardware."""
import socketserver
import struct
import threading
import time


class PcmStub(socketserver.ThreadingTCPServer):
    daemon_threads = True
    allow_reuse_address = True

    def __init__(self):
        self.opens = 0
        self.frames = 0
        self.resets = 0
        self.errors = []
        super().__init__(('127.0.0.1', 0), Peer)
        threading.Thread(target=self.serve_forever, daemon=True).start()


class Peer(socketserver.BaseRequestHandler):
    def handle(self):
        def read(n):
            result = b''
            while len(result) < n:
                block = self.request.recv(n - len(result))
                if not block:
                    raise EOFError
                result += block
            return result

        rate = channels = sent = 0
        start = None
        try:
            while True:
                command = read(1)
                if command == b'O':
                    rate, channels, boost = struct.unpack('<III', read(12))
                    assert 8000 <= rate <= 96000 and channels in (1, 2) and 1 <= boost <= 4
                    self.server.opens += 1
                elif command == b'W':
                    length, = struct.unpack('<I', read(4))
                    assert channels and length <= 1024 * 1024 and length % (channels * 2) == 0
                    read(length)
                    frames = length // (channels * 2)
                    sent += frames
                    self.server.frames += frames
                    if start is None:
                        start = time.monotonic()
                elif command == b'D':
                    played = min(sent, int((time.monotonic() - start) * rate)) if start else 0
                    self.request.sendall(struct.pack('<QQ', sent, played))
                    continue
                elif command == b'R':
                    sent, start = 0, None
                    self.server.resets += 1
                elif command != b'F':
                    raise ValueError(f'unknown PCM command {command!r}')
                self.request.sendall(struct.pack('<i', 0))
        except (EOFError, ConnectionResetError, BrokenPipeError):
            pass
        except Exception as error:
            self.server.errors.append(str(error))
