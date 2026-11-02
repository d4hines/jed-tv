#!/usr/bin/env python3
"""Read stock eMMC ranges through the authorized TCL factory service.

Requires readback.jar and readback.sh prepared as documented locally.
Never writes a TV block device. Stages at most 64 MiB in RAM, transfers via
ADB shell, verifies each chunk and rereads each source range for SHA-256.
The live userdata filesystem is deliberately excluded.
"""
import argparse
import datetime as dt
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import uuid

MIB = 1024 * 1024
BASE = Path(__file__).resolve().parent
parser = argparse.ArgumentParser()
parser.add_argument('--target', default='192.168.77.2:5555')
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--only', nargs='+')
args = parser.parse_args()
os.umask(0o077)
output = args.output.resolve()
output.mkdir(mode=0o700, parents=True, exist_ok=False)
adb = ['adb', '-s', args.target]


def run(command, timeout=300):
    result = subprocess.run(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=timeout)
    if result.returncode:
        raise RuntimeError(f'{command}: {result.stderr.decode(errors="replace")} {result.stdout.decode(errors="replace")}')
    return result.stdout


def shell(command):
    return run(adb + ['shell', command]).decode()


def get_remote(name):
    if not re.fullmatch(r'[a-z.-]+', name):
        raise ValueError(name)
    return run(adb + ['exec-out', 'cat', '/tmp/tcl-readback/' + name])


def call(*words):
    if not all(re.fullmatch('[A-Za-z0-9_]+', word) for word in words):
        raise ValueError(words)
    command = ('CLASSPATH=/data/local/tmp/tcl-auth-probe/readback.jar '
               'app_process32 /system/bin --nice-name=com.tcl.factory.view '
               'FactoryReadback ' + ' '.join(words))
    result = shell(command)
    with (output / 'dispatcher.log').open('a') as log:
        log.write(' '.join(words) + '\n' + result + '\n')
    if 'systemAuthorized=0' not in result:
        raise RuntimeError('Factory registration failed')


def stage(operation, device, start, count):
    nonce = uuid.uuid4().hex
    call(operation, device, str(start), str(count), nonce)
    if get_remote('status.txt').decode().strip() != nonce + ' OK':
        raise RuntimeError('Missing/stale/error status from readback operation')
    digest = get_remote('digest.txt').decode().split()[0]
    if not re.fullmatch('[0-9a-f]{64}', digest):
        raise RuntimeError('Invalid device digest')
    return digest


capabilities = get_remote('capabilities.txt').decode()
mapping = shell('ls -l /dev/block/by-name')
fingerprint = shell('getprop ro.build.fingerprint').strip()
if fingerprint != 'TCL/G06_2K_US/G06:11/AR2101/AR33:user/release-keys':
    raise RuntimeError('Unexpected running firmware: ' + fingerprint)
(output / 'capabilities.txt').write_text(capabilities)
(output / 'partition-map.txt').write_text(mapping)
(output / 'mounts.txt').write_text(shell('cat /proc/mounts'))
blocks = {}
for match in re.finditer(r'^BLOCK (\w+) SIZE (\d+)\n(?:START (\d+)\n)?', capabilities, re.M):
    blocks[match[1]] = {'sectors': int(match[2]), 'start_sector': int(match[3] or 0)}
names = dict(re.findall(r' (\w+) -> /dev/block/(\w+)', mapping))
priority = ['boot', 'recovery', 'vbmeta', 'vbmeta_system', 'dtbo', 'MBOOT', 'MBOOTBAK',
            'MPOOL', 'mmcblk0boot0', 'mmcblk0boot1', 'emmc_head', 'emmc_tail',
            'tvcertificate', 'eeprom_a', 'demura', 'persist', 'impdata', 'reserve',
            'misc', 'optee', 'armfw', 'RTPM', 'oem', 'tclconfig', 'tvconfig',
            'upgrade', 'linux_rootfs_a', '3rd_a', 'tvservice', 'super',
            'metadata', 'tcluserdata', 'cache']
plan = []
for name in priority:
    if args.only and name not in args.only:
        continue
    if name in ('emmc_head', 'emmc_tail'):
        source, count = 'mmcblk0', 2
        start = 0 if name == 'emmc_head' else blocks['mmcblk0']['sectors'] // 2048 - count
    else:
        node = names[name]
        info = blocks[node]
        if info['start_sector'] % 2048 or info['sectors'] % 2048:
            raise RuntimeError('Range is not MiB aligned: ' + name)
        source = node if node in ('mmcblk0boot0', 'mmcblk0boot1') else 'mmcblk0'
        start = info['start_sector'] // 2048
        count = info['sectors'] // 2048
    plan.append(dict(name=name, source=source, start_mib=start, size_mib=count,
                     size_bytes=count*MIB, status='pending'))
if args.only and set(args.only) != {item['name'] for item in plan}:
    raise ValueError('Unknown requested partition')
manifest = dict(schema=1, target=args.target, fingerprint=fingerprint,
                started_utc=dt.datetime.now(dt.timezone.utc).isoformat(),
                method='Factory registration; raw eMMC reads; RAM staging; ADB exec-out',
                consistency='Online read; mutable partitions are not an atomic filesystem snapshot',
                excluded={'userdata': 'Mounted live user filesystem; this is a firmware backup'},
                blocks=blocks, names=names, files=plan, complete=False)


def save():
    path = output / 'manifest.json'
    temporary = path.with_suffix('.json.tmp')
    temporary.write_text(json.dumps(manifest, indent=2) + '\n')
    temporary.replace(path)


save()
try:
    for item in plan:
        name, count, start, source = item['name'], item['size_mib'], item['start_mib'], item['source']
        print(f'START {name} {count} MiB', flush=True)
        item['status'] = 'copying'
        item['chunks'] = []
        save()
        partial = output / (name + '.img.partial')
        whole = hashlib.sha256()
        with partial.open('xb') as destination:
            for offset in range(0, count, 64):
                length = min(64, count-offset)
                source_digest = stage('read', source, start+offset, length)
                chunk = hashlib.sha256()
                received = 0
                process = subprocess.Popen(adb + ['exec-out', 'cat', '/tmp/tcl-readback/chunk.bin'],
                                           stdout=subprocess.PIPE, stderr=subprocess.PIPE)
                try:
                    while data := process.stdout.read(MIB):
                        received += len(data)
                        if received > length*MIB:
                            raise RuntimeError('Oversized staging file')
                        destination.write(data)
                        chunk.update(data)
                        whole.update(data)
                    error = process.stderr.read()
                    code = process.wait(timeout=15)
                finally:
                    if process.poll() is None:
                        process.kill()
                        process.wait()
                if code or received != length*MIB or chunk.hexdigest() != source_digest:
                    raise RuntimeError(f'Chunk failed {name}@{offset}: bytes={received} code={code} error={error!r}')
                item['chunks'].append(dict(offset_mib=offset, size_mib=length, sha256=source_digest))
                save()
                if count > 64:
                    print(f'COPIED {name} {offset+length}/{count} MiB', flush=True)
            destination.flush()
            os.fsync(destination.fileno())
        item['sha256'] = whole.hexdigest()
        print(f'CHECK source {name}', flush=True)
        item['source_sha256_after_copy'] = stage('hash', source, start, count)
        item['source_matches'] = item['source_sha256_after_copy'] == item['sha256']
        filename = name + '.img'
        partial.rename(output / filename)
        item['filename'] = filename
        item['status'] = 'verified' if item['source_matches'] else 'source_changed'
        save()
        print(f'{item["status"].upper()} {name} {item["sha256"]}', flush=True)
        if not item['source_matches'] and name not in ('metadata', 'cache', 'misc', 'persist', 'tcluserdata', 'impdata', 'reserve'):
            raise RuntimeError('Unexpected source change in firmware partition ' + name)
    call('cleanup')
    manifest['complete'] = True
    manifest['finished_utc'] = dt.datetime.now(dt.timezone.utc).isoformat()
    (output / 'SHA256SUMS').write_text(''.join(f'{item["sha256"]}  {item["filename"]}\n' for item in plan))
    save()
    print('COMPLETE ' + str(output), flush=True)
except BaseException as error:
    manifest['error'] = repr(error)
    save()
    raise
