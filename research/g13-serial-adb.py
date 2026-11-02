#!/usr/bin/env python3
"""Use an existing Screen UART shell to reach authenticated loopback ADB.

The private ADB key stays on the host. No serial device is opened here.
Temporary shell-owned transfer files are removed after each command. This is a
bootstrap path when Ethernet has no address and USB data is unavailable.
"""
import argparse
import base64
import fcntl
from pathlib import Path
import re
import shlex
import struct
import subprocess
import time
import uuid


class SerialAdb:
    def __init__(self, screen, session, log, key):
        self.screen, self.session = screen, session
        self.log, self.key = Path(log), Path(key)
        self.base = '/data/local/tmp/dande-uart-adb-' + uuid.uuid4().hex[:12]
        self.offset = 0
        self.pending = b''
        self.pid = None

    def shell(self, command, timeout=15):
        tag = 'UART_' + uuid.uuid4().hex
        start = self.log.stat().st_size
        encoded = base64.b64encode(command.encode()).decode()
        transfer = '/data/local/tmp/' + tag
        line = (f'echo {tag}_BEGIN; eval "$(echo {encoded} | base64 -d)" >{transfer} 2>&1; '
                f'base64 {transfer} | sed "s/.*/{tag}_DATA:\u0026/"; rm {transfer}; echo {tag}_END\r')
        # Screen silently ignores an oversized single `stuff` argument.
        for offset in range(0, len(line), 192):
            subprocess.run([self.screen, '-S', self.session, '-p', '0', '-X',
                            'stuff', line[offset:offset + 192]], check=True)
            time.sleep(.03)
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            output = self.log.read_bytes()[start:].replace(b'\r', b'')
            match = re.search(rb'\n' + tag.encode() + rb'_BEGIN\n(.*?)' + tag.encode() + rb'_END', output, re.S)
            if match:
                chunks = re.findall(rb'^' + tag.encode() + rb'_DATA:([A-Za-z0-9+/=]+)$', match[1], re.M)
                return base64.b64decode(b''.join(chunks), validate=True)
            time.sleep(.1)
        raise TimeoutError(f'UART command did not finish: {command}')

    def open(self):
        result = self.shell(f'umask 077; mkdir {self.base}; touch {self.base}/in; '
                            f'( (toybox tail -c +1 -f {self.base}/in & echo $! >{self.base}/tail.pid; wait) '
                            f'| toybox nc 127.0.0.1 5555 >{self.base}/out 2>{self.base}/err) '
                            f'</dev/null >{self.base}/launch.out 2>{self.base}/launch.err & echo $!')
        self.pid = int(result.strip().splitlines()[-1])
        self.send(b'CNXN', 0x01000000, 4096, b'host::\0')
        while True:
            kind, arg0, arg1, data = self.receive()
            if kind == b'CNXN':
                return
            if kind != b'AUTH' or arg0 != 1:
                raise RuntimeError(f'Unexpected ADB authentication reply: {kind!r} {arg0}')
            signature = subprocess.run(['openssl', 'pkeyutl', '-sign', '-inkey', str(self.key),
                                        '-pkeyopt', 'digest:sha1'], input=data,
                                       capture_output=True, check=True).stdout
            self.send(b'AUTH', 2, 0, signature)
            kind, arg0, arg1, data = self.receive()
            if kind != b'CNXN':
                raise RuntimeError('The host ADB key is not authorized on this TV')
            return

    def send(self, kind, arg0, arg1, data=b''):
        command = struct.unpack('<I', kind)[0]
        packet = struct.pack('<6I', command, arg0, arg1, len(data), sum(data), command ^ 0xffffffff) + data
        encoded = base64.b64encode(packet).decode()
        result = self.shell(f'echo {encoded} | base64 -d >> {self.base}/in')
        if result:
            raise RuntimeError(result.decode(errors='replace'))

    def receive(self, timeout=15):
        deadline = time.monotonic() + timeout
        while time.monotonic() < deadline:
            if len(self.pending) >= 24:
                command, arg0, arg1, length, checksum, magic = struct.unpack('<6I', self.pending[:24])
                if magic != command ^ 0xffffffff or length > 1024 * 1024:
                    raise RuntimeError('Invalid ADB frame')
                if len(self.pending) >= 24 + length:
                    data = self.pending[24:24+length]
                    self.pending = self.pending[24+length:]
                    if sum(data) != checksum:
                        raise RuntimeError('ADB checksum mismatch')
                    return struct.pack('<I', command), arg0, arg1, data
            data = self.shell(f'dd if={self.base}/out bs=1 skip={self.offset} 2>/dev/null')
            self.offset += len(data)
            self.pending += data
            if not data:
                time.sleep(.2)
        raise TimeoutError('No ADB packet: ' + self.shell(f'cat {self.base}/err').decode(errors='replace'))

    def service(self, service):
        self.send(b'OPEN', 1, 0, service.encode() + b'\0')
        output = bytearray()
        while True:
            kind, remote, local, data = self.receive()
            if kind == b'WRTE':
                output.extend(data)
                self.send(b'OKAY', 1, remote)
                if service in ('root:', 'unroot:'):
                    return bytes(output)  # The daemon exits during this request.
            elif kind == b'CLSE':
                self.send(b'CLSE', 1, remote)
                return bytes(output)
            elif kind != b'OKAY':
                raise RuntimeError(f'Unexpected ADB stream packet: {kind!r}')

    def close(self):
        if self.pid is not None:
            self.shell(f'kill {self.pid} $(cat {self.base}/tail.pid) 2>/dev/null; rm -rf {shlex.quote(self.base)}')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--screen', default='screen')
    parser.add_argument('--session', required=True)
    parser.add_argument('--log', type=Path, required=True)
    parser.add_argument('--key', type=Path, default=Path.home() / '.android/adbkey')
    parser.add_argument('service', help='ADB service, e.g. shell:id or root:')
    args = parser.parse_args()
    # Concurrent Screen writers can interleave partial command lines.
    with args.log.with_suffix('.lock').open('w') as lock:
        fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        bridge = SerialAdb(args.screen, args.session, args.log, args.key)
        try:
            bridge.open()
            print(bridge.service(args.service).decode(errors='replace'), end='', flush=True)
        finally:
            bridge.close()


if __name__ == '__main__':
    main()
