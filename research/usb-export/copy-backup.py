"""Copy this session's backups to the already formatted, identified PNY USB.

Does not format or mount devices. Verify the copy after unmount/remount.
"""
from pathlib import Path
import datetime as dt
import hashlib
import json
import os
import subprocess
import time

ROOT = Path(__file__).resolve().parents[2]
USB = Path('/mnt/tcl-v109-usb')
DEVICE = '/dev/disk/by-id/usb-PNY_USB_3.1_FD_07210C69972A5005-0:0'
ARCHIVE = ROOT / 'assets/backups/usb-fastboot-trigger-20261031T054144Z'
BACKUP = ROOT / 'assets/backups/v109-stock-20261031'
EXPORT = ROOT / 'assets/research/usb-export'
EXPORT_SOURCE = ROOT / 'research/usb-export'
os.umask(0o077)

mount = json.loads(subprocess.check_output(
    ['findmnt', '-J', '--mountpoint', str(USB)]))['filesystems'][0]
assert mount['fstype'] == 'vfat' and 'rw' in mount['options'].split(',')
assert Path(mount['source']).resolve() == Path(DEVICE + '-part1').resolve()
disk = json.loads(subprocess.check_output([
    'lsblk', '-b', '-J', '--tree', '-o', 'PATH,TYPE,SIZE,TRAN,SERIAL,LABEL', DEVICE]))['blockdevices'][0]
assert disk['serial'] == '07210C69972A5005' and disk['tran'] == 'usb'
assert disk['size'] == 31042043904 and disk['children'][0]['label'] == 'TVBACKUP'
assert not list(USB.iterdir()), 'Expected an empty, newly formatted USB'
backup = json.loads((BACKUP / 'manifest.json').read_text())
archive = json.loads((ARCHIVE / 'manifest.json').read_text())
assert backup['complete'] and archive['complete']
assert archive['decompressed_matches_source_stream']
assert len(backup['files']) == 33
assert all(f['status'] == 'verified' and f['source_matches'] for f in backup['files'])

plan = []


def add(source, destination=None, offset=0, length=None, expected=None):
    source = Path(source)
    assert source.is_file() and not source.is_symlink(), source
    destination = destination or ('TCL32Q3K/' + str(source.relative_to(ROOT)))
    assert not Path(destination).is_absolute() and '..' not in Path(destination).parts
    length = source.stat().st_size if length is None else length
    assert 0 <= length < 2**32 and offset + length <= source.stat().st_size
    plan.append(dict(source=str(source), destination=destination,
                     source_offset=offset, bytes=length, expected=expected))


image_hashes = {f['filename']: f['sha256'] for f in backup['files']}
for source in sorted(BACKUP.iterdir()):
    add(source, expected=image_hashes.get(source.name))
for name in ['README.txt', 'RECOVERY_NOTES.txt', 'research/ROOT_READBACK_NOTES.txt',
             'research/probes/FactoryAuthorizedIdentityProbe.java', 'research/probes/FactoryReadback.java',
             'research/probes/readback.sh', 'research/dump-stock.py', 'research/root-research.md',
             'assets/research/native-setinfo-disassembly.txt',
             'assets/research/native-factory-set-system-info-disassembly.txt',
             'assets/research/tools/r8-8.3.37.jar',
             'assets/research/factory-decompiled/sources/com/tcl/factory/view/FactoryApplication.java',
             'assets/research/tvmanager-decompiled/sources/com/tcl/tosapi/atv/TvFunctionApi.java',
             'assets/research/device/post-backup-adb-state.txt', 'research/device/unlock-result.md',
             'assets/research/device/cache-menu-preflash-state.txt',
             'assets/research/device/cache-menu-flash-result.txt',
             'assets/research/device/cache-menu-reboot-recovery.txt']:
    add(ROOT / name)
for source in sorted((ROOT / 'assets/research/factory-authorized-probe').iterdir()):
    if source.is_file():
        add(source)
recovery = ROOT / 'research/recovery-probe'
recovery_assets = ROOT / 'assets/research/recovery-probe'
for directory in (recovery, recovery_assets):
    for source in sorted(directory.iterdir()):
        if source.is_file():
            expected = '0f2f01c087e4fa0df4ce97b5f70b5c966edee994787c3577a3ba0fc1013bedad' if source.name == 'cache-menu.img' else None
            add(source, expected=expected)
for source in sorted((recovery / 'cache-menu-root').rglob('*')):
    if source.is_file():
        add(source)
for source in sorted((recovery_assets / 'usb-backup').glob('*.json')):
    add(source)
for name in ['README.txt', 'PREVIOUS_USB_IMAGE.txt']:
    add(EXPORT_SOURCE / name, destination=name)
archive_prefix = 'TCL32Q3K/' + str(ARCHIVE.relative_to(ROOT)) + '/'
for source in sorted(ARCHIVE.iterdir()):
    if source.is_file() and source.suffix != '.zst':
        dest = archive_prefix + ('original-SHA256SUMS.txt' if source.name == 'SHA256SUMS' else source.name)
        add(source, destination=dest)
archive_image = ARCHIVE / archive['archive']
assert archive_image.stat().st_size == archive['archive_bytes']
part_size = 2 * 1024**3
for index, offset in enumerate(range(0, archive['archive_bytes'], part_size)):
    add(archive_image, archive_prefix + archive_image.name + f'.part{index:03d}',
        offset=offset, length=min(part_size, archive['archive_bytes'] - offset))

assert len({p['destination'].casefold() for p in plan}) == len(plan)
total = sum(p['bytes'] for p in plan)
fs = os.statvfs(USB)
assert total + 100 * 1024**2 < fs.f_bavail * fs.f_frsize
print(f'COPY {len(plan)} files, {total} bytes to {USB}', flush=True)
copied = 0
last = time.monotonic()
archive_digest = hashlib.sha256()
records = []
for item in plan:
    source = Path(item['source'])
    destination = USB / item['destination']
    destination.parent.mkdir(parents=True, exist_ok=True)
    remaining = item['bytes']
    digest = hashlib.sha256()
    with source.open('rb') as reader, destination.open('xb') as writer:
        reader.seek(item['source_offset'])
        while remaining:
            data = reader.read(min(8 * 1024**2, remaining))
            if not data:
                raise RuntimeError('Unexpected end of source: ' + str(source))
            writer.write(data)
            digest.update(data)
            if source == archive_image:
                archive_digest.update(data)
            remaining -= len(data)
            copied += len(data)
            if time.monotonic() - last >= 10:
                print(f'COPIED {copied}/{total} bytes ({100*copied/total:.1f}%) {destination.name}', flush=True)
                last = time.monotonic()
        writer.flush()
        os.fsync(writer.fileno())
    if item['expected']:
        assert digest.hexdigest() == item['expected'], destination
    record = {k: v for k, v in item.items() if k != 'expected'}
    record['sha256'] = digest.hexdigest()
    records.append(record)
assert archive_digest.hexdigest() == archive['archive_sha256']
report = dict(copied_at_utc=dt.datetime.now(dt.timezone.utc).isoformat(),
              source_usb_serial=disk['serial'], filesystem='FAT32', label='TVBACKUP',
              payload_files=len(records), payload_bytes=total,
              tv_images=33, original_usb_archive_parts=(archive['archive_bytes'] + part_size - 1)//part_size,
              split_archive_matches_original=True,
              usb_readback_verified=False, files=records)
encoded = (json.dumps(report, indent=2) + '\n').encode()
(USB / 'COPY_MANIFEST.json').write_bytes(encoded)
(EXPORT / 'copy-manifest.json').write_bytes(encoded)
lines = [r['sha256'] + '  ' + r['destination'] + '\n' for r in records]
lines.append(hashlib.sha256(encoded).hexdigest() + '  COPY_MANIFEST.json\n')
(USB / 'SHA256SUMS').write_text(''.join(lines))
(EXPORT / 'USB-SHA256SUMS').write_text(''.join(lines))
subprocess.run(['sync', '-f', str(USB)], check=True)
print('COPY COMPLETE; unmount and remount read-only before verification', flush=True)
