#!/usr/bin/env python3
"""Report APEX native-library exports and ELF importers in extracted firmware."""
import argparse
from collections import defaultdict
import json
from pathlib import Path
import sys
import zipfile

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools'))
from g06_firmware import elf


def manifest(data):
    # Field numbers: AOSP system/apex/proto/apex_manifest.proto, Android 14 r1.
    offset = 0
    result = defaultdict(list)

    def varint():
        nonlocal offset
        value = 0
        for shift in range(0, 70, 7):
            byte = data[offset]
            offset += 1
            value |= (byte & 127) << shift
            if byte < 128:
                return value
        raise ValueError('Invalid protobuf varint')

    while offset < len(data):
        key = varint()
        field, wire = key >> 3, key & 7
        if wire == 0:
            value = varint()
        elif wire == 2:
            size = varint()
            value = data[offset:offset+size]
            offset += size
            if len(value) != size:
                raise ValueError('Truncated manifest')
            if field in (1, 5, 7, 8, 9, 14):
                value = value.decode('utf-8')
            else:
                continue
        else:
            raise ValueError('Unexpected manifest wire type')
        result[field].append(value)
    return {'name': result[1][0], 'version': result[2][0],
            'provide_native_libs': result[7], 'require_native_libs': result[8],
            'jni_libs': result[9]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('root', type=Path)
    parser.add_argument('label', choices=['v013', 'v033'])
    args = parser.parse_args()
    output = ROOT / 'assets/research/g13-firmware-trim-20261101'
    inventory = json.loads((output / (args.label + '-inventory.json')).read_text())
    importers = defaultdict(list)
    apexs, failures = [], []
    for item in inventory['files']:
        name = item['path']
        path = args.root / (('system' + name) if item['partition'] == 'system' else name.lstrip('/'))
        if item['category'] == 'apex' and name.endswith('.apex'):
            with zipfile.ZipFile(path) as archive:
                info = manifest(archive.read('apex_manifest.pb'))
            info.update({'path': name, 'bytes': item['bytes']})
            apexs.append(info)
        elif '/bin/' in name or '/lib/' in name:
            with path.open('rb') as stream:
                signature = stream.read(6)
            if signature != b'\x7fELF\x01\x01':
                continue
            try:
                metadata = elf(path.read_bytes())
            except (ValueError, UnicodeError) as error:
                failures.append({'path': name, 'error': str(error)})
                continue
            for needed in metadata['needed']:
                importers[needed].append(name)
    for item in apexs:
        item['outside_elf_importers'] = {name: importers[name] for name in item['provide_native_libs'] if importers[name]}
    report = {'label': args.label, 'apex': apexs, 'elf_parse_failures': failures,
              'scope': 'Static ELF32 DT_NEEDED imports outside APEX only; not a runtime reachability or deletion proof.'}
    (output / (args.label + '-apex-audit.json')).write_text(json.dumps(report, indent=2) + '\n')
    for item in apexs:
        print(item['name'], 'exports=', ','.join(item['provide_native_libs']),
              'outside_importers=', len(set(p for paths in item['outside_elf_importers'].values() for p in paths)))
    assert not failures, failures


if __name__ == '__main__':
    main()
