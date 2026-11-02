#!/usr/bin/env python3
"""Inventory extracted G13 firmware; emit research data, never delete files."""
import argparse
from collections import defaultdict
import hashlib
import json
import os
from pathlib import Path
import stat

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'assets/research/g13-firmware-trim-20261101'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('root', type=Path, help='Directory containing extracted partition subdirectories')
    parser.add_argument('label', choices=['v013', 'v033'])
    args = parser.parse_args()
    records = []
    for partition in sorted(args.root.iterdir()):
        if not partition.is_dir():
            continue
        for current, directories, files in os.walk(partition, followlinks=False):
            directories[:] = sorted(d for d in directories if not (Path(current) / d).is_symlink())
            for name in sorted(files):
                path = Path(current) / name
                info = path.lstat()
                if not stat.S_ISREG(info.st_mode):
                    continue
                relative = path.relative_to(partition).as_posix()
                # system.img contains a system-as-root hierarchy.
                guest = '/' + relative if partition.name == 'system' else '/' + partition.name + '/' + relative
                pieces = guest.strip('/').split('/')
                category, group = 'other', guest
                if len(pieces) >= 3 and pieces[1] in ('app', 'priv-app', 'overlay'):
                    category, group = 'apk-trees', '/' + '/'.join(pieces[:3])
                elif len(pieces) >= 2 and pieces[1] == 'framework':
                    category, group = 'framework', '/' + '/'.join(pieces[:2])
                elif len(pieces) >= 2 and pieces[1] == 'apex':
                    category, group = 'apex', guest
                elif name.endswith('.ko'):
                    category = 'kernel-modules'
                elif len(pieces) >= 2 and pieces[1] == 'lib':
                    category = 'native-libraries'
                records.append({'partition': partition.name, 'path': guest, 'bytes': info.st_size,
                                'category': category, 'group': group})
    partitions, categories, groups = defaultdict(int), defaultdict(int), defaultdict(int)
    for item in records:
        partitions[item['partition']] += item['bytes']
        categories[item['category']] += item['bytes']
        if item['category'] in ('apk-trees', 'framework', 'apex'):
            groups[(item['category'], item['group'])] += item['bytes']
    report = {
        'label': args.label, 'source': str(args.root.resolve()),
        'method': 'Sum regular-file st_size; no symlink traversal; no compression/hardlink deduplication. Not physical flash savings.',
        'file_count': len(records), 'total_bytes': sum(partitions.values()),
        'partition_bytes': dict(partitions), 'category_bytes': dict(categories),
        'groups': [{'category': category, 'path': group, 'bytes': size}
                   for (category, group), size in sorted(groups.items(), key=lambda entry: (-entry[1], entry[0]))],
        'apk_count': sum(item['path'].endswith('.apk') for item in records),
        'files': records,
    }
    OUT.mkdir(parents=True, exist_ok=True)
    destination = OUT / (args.label + '-inventory.json')
    destination.write_text(json.dumps(report, indent=2) + '\n')
    digest = hashlib.sha256(destination.read_bytes()).hexdigest()
    print(json.dumps({key: value for key, value in report.items() if key not in ('files', 'groups')}, indent=2))
    print('Inventory:', destination, 'SHA256:', digest)
    print('Largest APK trees:')
    for item in [g for g in report['groups'] if g['category'] == 'apk-trees'][:20]:
        print(f"{item['bytes'] / 2**20:8.2f} MiB {item['path']}")


if __name__ == '__main__':
    main()
