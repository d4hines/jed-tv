#!/usr/bin/env python3
"""Round-trip binary YMODEM through a separate Screen PTY, never the TV port."""
import hashlib
import json
import os
from pathlib import Path
import pty
import select
import shutil
import subprocess
import sys
import termios
import threading
import time
import tty

from tv_session import ROOT, SCREEN, pack


def main():
    out = ROOT / 'assets/build/leanboot/serial-test'
    out.mkdir(parents=True, exist_ok=True)
    recv = out / 'received'
    recv.mkdir(exist_ok=True)
    # Covers control bytes, NUL and high-bit bytes, plus the actual LZ4 payload.
    data = bytes(range(256)) * 256 + pack((ROOT / 'assets/build/leanboot/g13-os.bin').read_bytes())
    src = out / 'binary.bin'
    src.write_bytes(data)
    result = out / 'sender.json'
    result.unlink(missing_ok=True)
    result.with_suffix('.ready').unlink(missing_ok=True)
    master, slave = pty.openpty()
    remote_master, remote_slave = pty.openpty()
    # rb drains and then flushes its terminal on exit. PTY tcdrain returns
    # before the master has read pending bytes, so TCIOFLUSH can discard the
    # final ACK. Give receiver TX a pipe (a drained UART FIFO); RX stays a TTY.
    remote_tx_read, remote_tx_write = os.pipe()
    tty.setraw(master)
    tty.setraw(slave)
    tty.setraw(remote_master)
    tty.setraw(remote_slave)
    # Separate endpoints: sender and receiver must not change the same termios.
    stop = threading.Event()
    def wire():
        while not stop.is_set():
            for source in select.select([master, remote_tx_read], [], [], .05)[0]:
                block = os.read(source, 4096)
                target = remote_master if source == master else master
                # Model 115200-baud 8N1 rather than an instantaneous PTY.
                time.sleep(len(block) * 10 / 115200)
                while block:
                    block = block[os.write(target, block):]
    relay = threading.Thread(target=wire)
    relay.start()
    session = f'leanboot-transfer-test-{os.getpid()}'
    # Match the physical UART contract: no automatic flow control.
    subprocess.run([SCREEN, '-dmS', session, '-fn', os.ttyname(slave), '115200'], check=True)
    try:
        time.sleep(.1)
        before = termios.tcgetattr(slave)
        with (out / 'receiver.stderr').open('wb') as errors:
            receiver = None
            try:
                subprocess.run([SCREEN, '-S', session, '-p', '0', '-X', 'exec', '!!',
                                sys.executable, str(ROOT / 'tools/bootloader/serial_send.py'),
                                '--sender', shutil.which('sb'), '--file', str(src),
                                '--result', str(result)], check=True)
                deadline = time.monotonic() + 5
                while not result.with_suffix('.ready').exists() and time.monotonic() < deadline:
                    time.sleep(.01)
                assert result.with_suffix('.ready').exists(), 'Sender not ready'
                receiver = subprocess.Popen([shutil.which('rb'), '--binary', '--overwrite'],
                                            stdin=remote_slave, stdout=remote_tx_write, stderr=errors, cwd=recv)
                assert receiver.wait(timeout=55) == 0
                deadline = time.monotonic() + 50
                while not result.exists() and time.monotonic() < deadline:
                    time.sleep(.05)
                record = json.loads(result.read_text())
                assert record.get('returncode') == 0, record
                assert (recv / src.name).read_bytes() == data
                assert termios.tcgetattr(slave) == before, 'Serial settings changed'
                record['sha256'] = hashlib.sha256(data).hexdigest()
                record['serial_settings_preserved'] = True
                (out / 'check.json').write_text(json.dumps(record, indent=2) + '\n')
                print(f'PASS: Screen exec YMODEM binary round trip, {len(data)} bytes; serial settings preserved')
            finally:
                if receiver is not None and receiver.poll() is None:
                    receiver.kill()
                    receiver.wait()
    finally:
        subprocess.run([SCREEN, '-S', session, '-X', 'quit'], check=False)
        stop.set()
        relay.join(timeout=2)
        os.close(master)
        os.close(slave)
        os.close(remote_master)
        os.close(remote_slave)
        os.close(remote_tx_read)
        os.close(remote_tx_write)


if __name__ == '__main__':
    main()
