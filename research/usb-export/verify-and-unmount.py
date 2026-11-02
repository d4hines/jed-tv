"""Read the USB copy after cache invalidation, record verification and unmount."""
from pathlib import Path
import datetime as dt
import hashlib
import json
import os
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
EXPORT = ROOT / 'assets/research/usb-export'
USB = Path('/mnt/tcl-v109-usb')
DEVICE = '/dev/disk/by-id/usb-PNY_USB_3.1_FD_07210C69972A5005-0:0'
PARTITION = DEVICE + '-part1'


def run(*args):
    return subprocess.check_output(list(args), text=True)


def sudo(*args):
    return run('sudo', '-n', *args)


def verify_mount():
    state = json.loads(run('findmnt', '-J', '--mountpoint', str(USB)))['filesystems'][0]
    assert state['fstype'] == 'vfat'
    assert Path(state['source']).resolve() == Path(PARTITION).resolve()
    return state


def mount_readonly():
    options = f'ro,nosuid,nodev,noexec,uid={os.getuid()},gid={os.getgid()},umask=0077'
    sudo('mount', '-t', 'vfat', '-o', options, PARTITION, str(USB))
    assert 'ro' in verify_mount()['options'].split(',')


disk = json.loads(run('lsblk', '-b', '-J', '--tree', '-o',
                     'PATH,SIZE,TRAN,SERIAL,LABEL', DEVICE))['blockdevices'][0]
assert disk['serial'] == '07210C69972A5005' and disk['tran'] == 'usb'
assert disk['size'] == 31042043904 and disk['children'][0]['label'] == 'TVBACKUP'
verify_mount()
copy = json.loads((EXPORT / 'copy-manifest.json').read_text())
assert copy['payload_files'] == 95 and copy['tv_images'] == 33
assert copy['split_archive_matches_original']
assert (USB / 'COPY_MANIFEST.json').read_bytes() == (EXPORT / 'copy-manifest.json').read_bytes()
assert (USB / 'SHA256SUMS').read_bytes() == (EXPORT / 'USB-SHA256SUMS').read_bytes()

run('sync', '-f', str(USB))
sudo('umount', str(USB))
assert not USB.is_mount()
sudo('blockdev', '--flushbufs', PARTITION)
sudo('blockdev', '--flushbufs', DEVICE)
before_fsck = sudo('fsck.fat', '-n', PARTITION)
(EXPORT / 'fsck-before-verification.txt').write_text(before_fsck)
mount_readonly()
print('READBACK: unmounted, block cache flushed, filesystem checked, remounted read-only', flush=True)
total = copy['payload_bytes']
checked = 0
last = time.monotonic()
with (EXPORT / 'usb-readback.log').open('w') as log:
    for record in copy['files']:
        file = USB / record['destination']
        assert file.stat().st_size == record['bytes'], file
        digest = hashlib.sha256()
        with file.open('rb') as stream:
            while data := stream.read(8 * 1024**2):
                digest.update(data)
                checked += len(data)
                if time.monotonic() - last >= 10:
                    print(f'VERIFIED READ {checked}/{total} bytes ({100*checked/total:.1f}%)', flush=True)
                    last = time.monotonic()
        assert digest.hexdigest() == record['sha256'], file
        log.write('OK ' + record['destination'] + '\n')
    assert checked == total
    assert (USB / 'COPY_MANIFEST.json').read_bytes() == (EXPORT / 'copy-manifest.json').read_bytes()
    assert (USB / 'SHA256SUMS').read_bytes() == (EXPORT / 'USB-SHA256SUMS').read_bytes()
    log.write('OK COPY_MANIFEST.json and SHA256SUMS\n')
print('All 95 payload files matched the source hashes on USB readback.', flush=True)

# Record the completed readback on both copies. Only these small metadata
# files change; all payload files have already passed independent readback.
copy['usb_readback_verified'] = True
copy['usb_verified_at_utc'] = dt.datetime.now(dt.timezone.utc).isoformat()
copy['verification_method'] = 'Unmount; flush device cache; fsck; read-only remount; all sizes and SHA-256 hashes'
encoded = (json.dumps(copy, indent=2) + '\n').encode()
lines = [r['sha256'] + '  ' + r['destination'] + '\n' for r in copy['files']]
lines.append(hashlib.sha256(encoded).hexdigest() + '  COPY_MANIFEST.json\n')
checksums = ''.join(lines).encode()
sudo('mount', '-o', 'remount,rw', str(USB))
(USB / 'COPY_MANIFEST.json').write_bytes(encoded)
(USB / 'SHA256SUMS').write_bytes(checksums)
(EXPORT / 'copy-manifest.json').write_bytes(encoded)
(EXPORT / 'USB-SHA256SUMS').write_bytes(checksums)
run('sync', '-f', str(USB))
sudo('umount', str(USB))
sudo('blockdev', '--flushbufs', PARTITION)
sudo('blockdev', '--flushbufs', DEVICE)
mount_readonly()
assert (USB / 'COPY_MANIFEST.json').read_bytes() == encoded
assert (USB / 'SHA256SUMS').read_bytes() == checksums
space = os.statvfs(USB)
free_bytes = space.f_bavail * space.f_frsize
sudo('umount', str(USB))
after_fsck = sudo('fsck.fat', '-n', PARTITION)
(EXPORT / 'fsck-final.txt').write_text(after_fsck)
assert not USB.is_mount()
result = dict(complete=True, verified_at_utc=copy['usb_verified_at_utc'],
              device_serial=disk['serial'], filesystem='FAT32', label='TVBACKUP',
              payload_files=copy['payload_files'], payload_bytes=total,
              tv_images=33, original_usb_archive_parts=copy['original_usb_archive_parts'],
              all_payload_hashes_match=True, final_metadata_readback_matches=True,
              filesystem_check_clean=True, unmounted=True, free_bytes=free_bytes,
              original_usb_archive=str(ROOT / 'assets/backups/usb-fastboot-trigger-20261031T054144Z'))
(EXPORT / 'verification.json').write_text(json.dumps(result, indent=2) + '\n')
print(json.dumps(result, indent=2), flush=True)
