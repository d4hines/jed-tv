#!/usr/bin/env python3
"""Apply or restore the prepared G13 warning JPEG over rooted ADB. No reboot.

Only the 94 sectors containing the logo are written. All other MBOOT bytes
must match the captured stock image. The on-TV transaction rolls back a
failed write while Linux is still running, even if the host disconnects.
"""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import shlex
import time

import g13_device as device

ROOT = Path(__file__).resolve().parents[2]
ASSETS = ROOT / 'assets/research/g13-unlock-warning-20261101'
STOCK = '643058879b84d044dea58a4572bd9ade1cc3140f3d3357eeaabae53be32ec382'
LOGOS = {
    'tcl': (ASSETS / 'candidate/MBOOT-candidate.img',
            '6f3fec853370af6f4759102d656f0bfd80abaa39d9de694882fd228034f69e8f'),
    'lean': (ROOT / 'assets/research/g13-lean-logo-20261101/candidate-rgb/MBOOT-candidate.img',
             'a342fcdd62b2ce7121d977b9410d2c05aa51757bf7d26d6b445423e97122439b'),
}
INIT_BOOT = '0ebb78f7eb550f8b2f524cf661342baeeaa55a70e3c3e5708d71f26381fc5e8b'
OFFSET, LENGTH, SECTORS, SIZE = 0x2bfa00, 47903, 94, 5242880
PARTITION = '/dev/block/by-name/MBOOT'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def capture(path):
    with path.open('wb') as f:
        device.adb('exec-out', 'cat', PARTITION, stdout=f, timeout=30)
    return path.read_bytes()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['apply', 'restore'])
    parser.add_argument('--logo', choices=LOGOS, default='lean',
                        help='replacement artwork for apply (restore reinstates the warning)')
    args = parser.parse_args()
    original = (ASSETS / 'MBOOT.img').read_bytes()
    candidate_path, candidate_hash = LOGOS[args.logo]
    candidate = candidate_path.read_bytes() if args.action == 'apply' else original
    assert len(original) == len(candidate) == SIZE
    assert sha(original) == STOCK
    assert sha(candidate) == (candidate_hash if args.action == 'apply' else STOCK)
    assert original[:OFFSET] == candidate[:OFFSET]
    assert original[OFFSET+LENGTH:] == candidate[OFFSET+LENGTH:]
    after = candidate
    expected_after = sha(after)

    device.root()
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
    out = ASSETS / (args.action + '-' + stamp)
    out.mkdir()
    before = capture(out / 'MBOOT.before.img')
    expected_before = sha(before)
    if expected_before not in {STOCK, *(value[1] for value in LOGOS.values())}:
        raise RuntimeError('Live MBOOT is not a known, verified version; no write')
    assert before[:OFFSET] == original[:OFFSET]
    assert before[OFFSET+LENGTH:] == original[OFFSET+LENGTH:]
    if before == after:
        print('Selected artwork is already installed; no write or reboot.')
        return
    before_span = before[OFFSET:OFFSET+SECTORS*512]
    after_span = after[OFFSET:OFFSET+SECTORS*512]
    checks = device.shell(
        'test "$(getprop ro.boot.slot_suffix)" = _a; '
        f'test "$(readlink -f {PARTITION})" = /dev/block/mmcblk0p1; '
        f'test "$(blockdev --getro {PARTITION})" = 0; '
        f'test "$(blockdev --getsize64 {PARTITION})" = {SIZE}; '
        'test "$(getprop sys.dande.network_ready)" = 1; '
        'test "$(getprop sys.dande.launcher_ready)" = 1; '
        'sha256sum /dev/block/by-name/MBOOT_b /dev/block/by-name/MBOOTBAK '
        '/dev/block/by-name/init_boot_a /dev/block/by-name/MPOOL '
        '/metadata/dande-tv/entry.rc /metadata/dande-tv/mode')
    hashes = dict(line.split()[::-1] for line in checks.splitlines())
    assert hashes['/dev/block/by-name/init_boot_a'] == INIT_BOOT
    assert hashes['/dev/block/by-name/MBOOT_b'] == STOCK
    assert hashes['/dev/block/by-name/MBOOTBAK'] == STOCK
    (out / 'before-state.txt').write_text(checks)
    for name, data in [('before.bin', before_span), ('after.bin', after_span)]:
        (out / name).write_bytes(data)
    work = '/data/local/tmp/dande-unlock-logo-' + stamp
    script = f'''#!/system/bin/sh
set -eu
cd {shlex.quote(work)}
started=0
committed=0
finish() {{
    status=$?
    trap - EXIT
    if [ "$started" = 1 ] && [ "$committed" = 0 ]; then
        echo 'Write failed verification; restoring the original sector span'
        if dd if=before.bin of={PARTITION} bs=512 seek={OFFSET//512} count={SECTORS} conv=notrunc,fsync &&
           sync && [ "$(sha256sum {PARTITION} | cut -d' ' -f1)" = {expected_before} ]; then
            echo ROLLED_BACK > result
        else
            echo ROLLBACK_FAILED > result
        fi
    elif [ "$committed" = 0 ]; then
        echo PREFLIGHT_FAILED > result
    fi
    exit "$status"
}}
trap finish EXIT
test "$(id -u)" = 0
test "$(getprop ro.serialno)" = 18ACC22D1C47
test "$(getprop ro.product.device)" = G13
test "$(getprop ro.build.version.incremental)" = AU13
test "$(readlink -f {PARTITION})" = /dev/block/mmcblk0p1
test "$(blockdev --getsize64 {PARTITION})" = {SIZE}
test "$(sha256sum {PARTITION} | cut -d' ' -f1)" = {expected_before}
test "$(stat -c %s before.bin)" = {SECTORS*512}
test "$(stat -c %s after.bin)" = {SECTORS*512}
test "$(sha256sum before.bin | cut -d' ' -f1)" = {sha(before_span)}
test "$(sha256sum after.bin | cut -d' ' -f1)" = {sha(after_span)}
started=1
dd if=after.bin of={PARTITION} bs=512 seek={OFFSET//512} count={SECTORS} conv=notrunc,fsync
sync
test "$(sha256sum {PARTITION} | cut -d' ' -f1)" = {expected_after}
committed=1
echo VERIFIED > result
sync
'''
    (out / 'transaction.sh').write_text(script)
    device.shell(f'test ! -e {work}; mkdir -m 700 {work}')
    for name in ['before.bin', 'after.bin', 'transaction.sh']:
        device.adb('push', out/name, work+'/'+name, capture_output=True, timeout=30)
    device.shell(f'chown -R 0:0 {work}; chmod 600 {work}/*; sync')
    (out/'operation.json').write_text(json.dumps(dict(
        action=args.action, logo=args.logo if args.action == 'apply' else 'warning',
        device_work=work, expected_before=expected_before,
        expected_after=expected_after, offset=OFFSET, sectors=SECTORS,
        bytes_written=SECTORS*512, rebooted=False), indent=2)+'\n')
    device.shell(f'nohup /system/bin/sh {work}/transaction.sh >{work}/transaction.log 2>&1 </dev/null &')
    result = ''
    for _ in range(60):
        result = device.shell(f'cat {work}/result 2>/dev/null || true').strip()
        if result:
            break
        time.sleep(.5)
    log = device.shell(f'cat {work}/transaction.log')
    (out/'transaction.log').write_text(log)
    (out/'result.txt').write_text(result+'\n')
    if result != 'VERIFIED':
        raise RuntimeError(f'No reboot: transaction result {result!r}; see {out}')
    readback = capture(out/'MBOOT.after.img')
    assert readback == after, 'Do not reboot: full host readback mismatch'
    after_checks = device.shell('sha256sum ' + ' '.join(map(shlex.quote, hashes)))
    assert after_checks == checks, 'Do not reboot: unrelated state changed'
    (out/'verification.json').write_text(json.dumps(dict(
        verified=True, full_readback_sha256=sha(readback),
        unrelated_partitions_and_boot_selection_unchanged=True,
        code_and_signatures_unchanged=True, rebooted=False), indent=2)+'\n')
    print(log, end='')
    print(f'{args.action}: full readback verified at {expected_after}')
    print(f'Evidence and rollback span: {out}')
    print('No reboot performed.')


if __name__ == '__main__':
    main()
