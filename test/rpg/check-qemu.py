#!/usr/bin/env python3
"""Compare the complete RPG UI transcript on upstream Lean and ARM32 Zr."""
import argparse
from pathlib import Path
import re
import subprocess
import tempfile

p = argparse.ArgumentParser(__doc__)
p.add_argument('--host-test', type=Path, required=True)
p.add_argument('--arm-test', type=Path, required=True)
p.add_argument('--kernel', type=Path, required=True)
p.add_argument('--qemu', default='qemu-system-aarch64')
a = p.parse_args()
host = subprocess.check_output([a.host_test.resolve(), '--nocapture'], text=True)
expected = re.search(r'TRANSCRIPT ([0-9a-f]+)', host)
assert expected and '1 passed' in host, host

def entry(name, data=b'', mode=0o100755, major=0, minor=0):
    fields = [1, mode, 0, 0, 1, 0, len(data), 0, 0, major, minor, len(name)+1, 0]
    out = b'070701' + b''.join(f'{x:08x}'.encode() for x in fields)
    out += name.encode() + b'\0'
    out += b'\0' * (-len(out) % 4)
    out += data
    return out + b'\0' * (-len(data) % 4)

with tempfile.TemporaryDirectory(prefix='rpg-qemu-') as tmp:
    ramdisk = Path(tmp) / 'initramfs.cpio'
    ramdisk.write_bytes(entry('dev', mode=0o40755) + entry('dev/console', mode=0o20600, major=5, minor=1)
                       + entry('init', a.arm_test.read_bytes()) + entry('TRAILER!!!', mode=0))
    cmd = [a.qemu, '-M', 'virt,highmem=off,gic-version=2', '-cpu', 'cortex-a55,aarch64=off', '-m', '256M', '-nographic', '-monitor', 'none',
           '-kernel', str(a.kernel), '-initrd', str(ramdisk), '-append', 'console=ttyAMA0 panic=-1 -- --nocapture', '-no-reboot']
    run = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    try:
        data, _ = run.communicate(timeout=45)
    except subprocess.TimeoutExpired:
        run.kill()
        data, _ = run.communicate()
    arm = data.decode(errors='replace')
    print(arm)
    actual = re.search(r'TRANSCRIPT ([0-9a-f]+)', arm)
    assert actual and '1 passed' in arm, 'ARM test did not finish'
    assert actual[1] == expected[1], f'ARM={actual[1]} upstream={expected[1]}'
    print(f'PASS: upstream Lean and ARM32 Zr UI transcripts match: {actual[1]}')
