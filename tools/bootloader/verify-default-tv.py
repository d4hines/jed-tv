#!/usr/bin/env python3
"""Verify four normal restarts of an already installed G13 selector.

Requires a fresh sequence-zero journal from install-boot-image.py. Captures
UART through the existing Screen log, checks ADB identity, persistent records,
boot markers and the installed image hash. Stops immediately on a mismatch.
"""
import hashlib
import json
from pathlib import Path
import struct
import subprocess
import sys
import time
import zlib

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT/'tools/native-app'))
import g13_device as d

LOG = ROOT/'assets/research/g13-ethernet-20261101/serial.log'
OUT = ROOT/'assets/research/g13-leanboot-default/installed/verify'


def journal():
    blob = d.adb('exec-out', 'dd if=/dev/block/by-name/upgrade bs=512 skip=102398 count=2 2>/dev/null',
                 capture_output=True, timeout=15).stdout
    assert len(blob) == 1024
    records = []
    for offset in (0, 512):
        rec = blob[offset:offset+512]
        magic, version, seq, state, inverse_seq, inverse_state = struct.unpack_from('<6I', rec)
        assert magic == 0x314a424c and version == 1
        assert inverse_seq == seq ^ 0xffffffff and inverse_state == state ^ 0xffffffff
        assert struct.unpack_from('<I', rec, 508)[0] == zlib.crc32(rec[:508])
        records.append({'sequence': seq, 'state': state})
    return blob, max(records, key=lambda rec: rec['sequence'])


def reconnect():
    last = None
    for _ in range(15):
        try:
            d.connect(); d.root(); return
        except (subprocess.SubprocessError, RuntimeError) as error:
            last = error
            subprocess.run(['adb', 'disconnect', d.TARGET], capture_output=True, timeout=5)
            time.sleep(1)
    raise RuntimeError(f'ADB did not return: {last}')


def main():
    OUT.mkdir(parents=True, exist_ok=True)
    installed = json.loads((OUT.parent/'install.json').read_text())
    digest = installed['boot_a_sha256']
    d.connect(); d.root()
    assert d.shell('sha256sum /dev/block/by-name/boot_a').split()[0] == digest
    _, initial = journal()
    assert initial == {'sequence': 0, 'state': 0}, initial
    previous = d.shell('cat /proc/sys/kernel/random/boot_id').strip()
    results = []
    for index, state in enumerate((1, 2, 4, 1), 1):
        start = LOG.stat().st_size
        began = time.monotonic()
        d.adb('reboot', capture_output=True, timeout=15)
        data = b''
        while time.monotonic()-began < 75:
            with LOG.open('rb') as f:
                f.seek(start); data = f.read()
            if b'DANDE_BOOT' in data and b'READY:' in data: break
            time.sleep(.1)
        (OUT/f'boot-{index}.bin').write_bytes(data)
        elapsed = time.monotonic()-began
        assert b'READY:' in data, f'Boot {index} did not reach READY'
        assert f'LEANBOOT choice N{state}'.encode() in data, data[-2000:]
        lean = state != 4
        assert (b'LEANBOOT OS prep' in data) == lean
        assert (b'LEANBOOT stock' in data) != lean
        reconnect()
        boot_id = d.shell('cat /proc/sys/kernel/random/boot_id').strip()
        assert boot_id != previous
        previous = boot_id
        blob, record = journal()
        (OUT/f'journal-{index}.bin').write_bytes(blob)
        assert record == {'sequence': index, 'state': state}, record
        marker = d.shell('getprop ro.boot.leanboot').strip()
        assert marker == ('1' if lean else ''), marker
        assert d.shell('sha256sum /dev/block/by-name/boot_a').split()[0] == digest
        network = d.shell('ip -4 addr; ip route')
        assert '192.168.77.3/' in network
        result = {'restart': index, 'path': 'lean' if lean else 'stock',
                  'ready_seconds': round(elapsed, 2), 'journal': record,
                  'boot_id': boot_id, 'lean_marker': marker, 'network': network}
        results.append(result)
        (OUT/'results.json').write_text(json.dumps(results, indent=2)+'\n')
        print(f'Restart {index}: {result["path"]}, READY in {elapsed:.2f}s, journal verified', flush=True)
    print('PASS: Lean, Lean, stock, Lean; Ethernet and installed image verified after every restart')


if __name__ == '__main__': main()
