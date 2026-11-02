"""Independently verify completed host files and recorded source-read coverage."""
import argparse
import datetime
import hashlib
import json
from pathlib import Path
import stat

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('backup', type=Path)
args = parser.parse_args()
root = args.backup
manifest = json.loads((root / 'manifest.json').read_text())
if not manifest.get('complete'):
    raise SystemExit('Capture is incomplete; refusing to certify it')
plan = {p['name']: p for p in manifest['plan']}
records = {p['name']: p for p in manifest['images']}
if len(plan) != 53 or len(records) != 53 or set(plan) != set(records):
    raise SystemExit('Missing or duplicate image records')
if len(manifest['images']) != 53 or len(manifest['plan']) != 53:
    raise SystemExit('Duplicate plan or image entries')
userdata_start = manifest['geometry']['userdata']['start_bytes']
results = []
for name, item in plan.items():
    record = records[name]
    path = root / (name + '.img')
    if name == 'userdata' or name != path.stem:
        raise SystemExit('Unexpected image name')
    if any(record[k] != item[k] for k in ('source', 'offset', 'size')):
        raise SystemExit('Source geometry mismatch: ' + name)
    if item['source'] == '/dev/block/mmcblk0':
        if item['offset'] < 0 or item['offset'] + item['size'] > userdata_start:
            raise SystemExit('Source range crosses excluded userdata: ' + name)
    elif item['source'] not in ('/dev/block/mmcblk0boot0', '/dev/block/mmcblk0boot1'):
        raise SystemExit('Unexpected source: ' + name)
    mode = stat.S_IMODE(path.stat().st_mode)
    size = path.stat().st_size
    if size != item['size'] or mode != 0o400:
        raise SystemExit('Wrong size or protection mode: ' + name)
    chunk_results = []
    covered = 0
    full = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in record['chunks']:
            if chunk['relative_offset'] != covered or chunk['length'] <= 0:
                raise SystemExit('Non-contiguous source verification: ' + name)
            data = f.read(chunk['length'])
            if len(data) != chunk['length']:
                raise SystemExit('Short host file: ' + name)
            digest = hashlib.sha256(data).hexdigest()
            full.update(data)
            matched = digest == chunk['sha256'] == chunk['second_source_sha256']
            if not matched or not chunk['source_matches']:
                raise SystemExit('Source or host chunk mismatch: ' + name)
            covered += len(data)
            chunk_results.append({'relative_offset': chunk['relative_offset'],
                                  'length': len(data), 'sha256': digest,
                                  'matches_both_recorded_source_reads': matched})
        if f.read(1) or covered != item['size']:
            raise SystemExit('Incomplete chunk coverage: ' + name)
    digest = full.hexdigest()
    if digest != record['sha256'] or digest != record['host_reread_sha256']:
        raise SystemExit('Full-image host hash mismatch: ' + name)
    if record['status'] != 'verified' or not record['source_matches']:
        raise SystemExit('Capture was not stable across source reads: ' + name)
    results.append({'name': name, 'size_bytes': size, 'mode': oct(mode),
                    'sha256': digest, 'chunks': chunk_results, 'verified': True})
total = sum(r['size_bytes'] for r in results)
if total != manifest['total_bytes']:
    raise SystemExit('Total-byte mismatch')
expected_sums = {r['name'] + '.img': r['sha256'] for r in results}
actual_sums = {}
for line in (root / 'SHA256SUMS').read_text().splitlines():
    digest, name = line.split('  ', 1)
    if name in actual_sums:
        raise SystemExit('Duplicate SHA256SUMS entry')
    actual_sums[name] = digest
if actual_sums != expected_sums:
    raise SystemExit('SHA256SUMS differs from verified files')
report = {'verified_at': datetime.datetime.now(datetime.timezone.utc).isoformat(),
          'all_verified': True, 'image_count': len(results), 'total_bytes': total,
          'fingerprint': manifest['fingerprint'],
          'method': 'Fresh host reads compared with both recorded source reads per chunk',
          'new_device_reads_performed_by_this_verifier': False,
          'restore_tested': False, 'images': results}
(root / 'host-verification.json').write_text(json.dumps(report, indent=2) + '\n')
print(f'Verified {len(results)} images, {total} bytes ({total / 2**20:.0f} MiB).')
