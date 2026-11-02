"""Download the discovered G13 V033 package to the laptop; never touch the TV."""
import hashlib
import json
from pathlib import Path
import time
import urllib.request
import xml.etree.ElementTree as ET
import zipfile

base = Path(__file__).resolve().parents[2] / 'assets/research/new-tv-20261031T183329Z'
reply = ET.fromstring(json.loads((base / 'firmware-query-project20046.json').read_text())['response'])
assert reply.findtext('./upgrade/version') == 'V8-T221T10-LF1V033'
size = int(reply.findtext('./upgrade/size'))
expected_md5 = reply.findtext('./upgrade/md5')
url = reply.findtext('./upgrade/fileurl')
destination = base.parent / 'firmware' / 'V8-T221T10-LF1V033'
destination.mkdir(parents=True, exist_ok=True)
target = destination / 'V8-T221T10-LF1V033.zip'
partial = destination / 'V8-T221T10-LF1V033.zip.partial'
if target.exists():
    raise SystemExit('Package already exists; preserve it')
md5, sha = hashlib.md5(), hashlib.sha256()
downloaded = 0
next_progress = 128 * 1024 * 1024
with partial.open('xb') as output, urllib.request.urlopen(url, timeout=30) as response:
    if response.status != 200:
        raise RuntimeError(f'Unexpected HTTP status {response.status}')
    while chunk := response.read(1024 * 1024):
        downloaded += len(chunk)
        if downloaded > size:
            raise RuntimeError('Download exceeds declared size')
        output.write(chunk)
        md5.update(chunk)
        sha.update(chunk)
        if downloaded >= next_progress:
            print(f'{downloaded}/{size} bytes', flush=True)
            next_progress += 128 * 1024 * 1024
if downloaded != size or md5.hexdigest() != expected_md5:
    raise RuntimeError('Size or server MD5 mismatch; leaving .partial for inspection')
partial.rename(target)
target.chmod(0o400)
record = {'version': reply.findtext('./upgrade/version'), 'bytes': downloaded,
          'md5': md5.hexdigest(), 'server_md5_matches': True,
          'sha256': sha.hexdigest(), 'verified_at_unix': int(time.time()),
          'source_url': url, 'ota_signature_verified': False,
          'installed_on_tv': False, 'installed_tv_version': 'V8-T221T10-LF1V013'}
(destination / 'download-verification.json').write_text(json.dumps(record, indent=2) + '\n')
with zipfile.ZipFile(target) as archive:
    entries = [{'name': i.filename, 'size': i.file_size, 'compressed_size': i.compress_size,
                'crc32': f'{i.CRC:08x}'} for i in archive.infolist()]
    (destination / 'contents.json').write_text(json.dumps(entries, indent=2) + '\n')
    for entry in entries:
        print(entry['size'], entry['name'])
    for name in ('META-INF/com/android/metadata', 'META-INF/com/google/android/updater-script',
                 'payload_properties.txt', 'mboot.bin', 'boot.img', 'init_boot.img', 'vendor_boot.img'):
        if name in archive.namelist():
            if archive.getinfo(name).file_size > 128 * 1024 * 1024:
                raise RuntimeError('Unexpectedly large selected member')
            output = destination / name
            output.parent.mkdir(parents=True, exist_ok=True)
            output.write_bytes(archive.read(name))
print(json.dumps(record, indent=2), flush=True)
