#!/usr/bin/env python3
"""Compare saved G13 modules with the candidate's exported symbol CRCs."""
import argparse
from collections import defaultdict
import hashlib
import json
from pathlib import Path
import struct
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools'))
from g06_firmware import elf


def versions(data):
    # elf() first validates the ARM ELF32 section/string tables.
    header = struct.unpack_from('<16sHHIIIIIHHHHHH', data)
    sections = [struct.unpack_from('<IIIIIIIIII', data, header[6] + i * header[11])
                for i in range(header[12])]
    names = sections[header[13]]
    names = data[names[4]:names[4] + names[5]]
    for section in sections:
        name = names[section[0]:].split(b'\0', 1)[0]
        if name == b'__versions':
            content = data[section[4]:section[4] + section[5]]
            assert len(content) == section[5] and len(content) % 64 == 0
            result = {}
            for offset in range(0, len(content), 64):
                symbol = content[offset + 4:offset + 64].split(b'\0', 1)[0].decode('ascii')
                assert symbol and symbol not in result
                result[symbol] = struct.unpack_from('<I', content, offset)[0]
            return result
    return {}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('firmware', type=Path)
    parser.add_argument('label', choices=['v013', 'v033'])
    args = parser.parse_args()
    work = ROOT / 'assets/build/g13-kernel-vm'
    candidate = work / 'kernel-out'
    symbols = {}
    for line in (candidate / 'Module.symvers').read_text().splitlines():
        fields = line.split()
        crc, name, provider = fields[:3]
        symbols[name] = (int(crc, 16), provider)
    build = json.loads((work / 'kernel-build.json').read_text())
    assert hashlib.sha256((candidate / 'Module.symvers').read_bytes()).hexdigest() == build['artifacts']['Module.symvers']['sha256']
    rebuilt = {Path(name).name: name for name in build['artifacts'] if name.endswith('.ko')}
    modules, omitted = [], []
    providers = defaultdict(list)
    for path in sorted(args.firmware.rglob('*.ko')):
        data = path.read_bytes()
        if data[:6] != b'\x7fELF\x01\x01':
            omitted.append(str(path.relative_to(args.firmware)))
            continue
        info = elf(data)
        entry = {'path': str(path.relative_to(args.firmware)), 'name': path.name,
                 'sha256': hashlib.sha256(data).hexdigest(), 'metadata': info,
                 'versions': versions(data), 'rebuilt_same_name': rebuilt.get(path.name)}
        modules.append(entry)
        for name in info['exports']:
            providers[name].append(entry['path'])
    for entry in modules:
        matched, mismatched, stock_only, missing = [], {}, {}, []
        for symbol in sorted(set(entry['metadata']['imports']) | set(entry['versions'])):
            crc = entry['versions'].get(symbol)
            if symbol in symbols:
                new_crc, provider = symbols[symbol]
                if crc is None:
                    continue
                if new_crc == crc:
                    matched.append(symbol)
                else:
                    mismatched[symbol] = {'stock_crc': f'{crc:08x}', 'candidate_crc': f'{new_crc:08x}', 'candidate_provider': provider}
            elif symbol in providers:
                stock_only[symbol] = providers[symbol]
            else:
                missing.append(symbol)
        entry.update(matched_crc=matched, mismatched_crc=mismatched,
                     stock_only_providers=stock_only, missing_symbols=missing,
                     imports_without_crc=sorted(set(entry['metadata']['imports']) - set(entry['versions'])))
    report = {'firmware': args.label, 'kernel_release': build['release'],
              'module_symvers_sha256': build['artifacts']['Module.symvers']['sha256'],
              'modules': modules, 'omitted_non_arm32': omitted,
              'summary': {'arm32_modules': len(modules),
                          'rebuilt_same_name': sum(x['rebuilt_same_name'] is not None for x in modules),
                          'with_crc_mismatch': sum(bool(x['mismatched_crc']) for x in modules),
                          'with_missing_symbols': sum(bool(x['missing_symbols']) for x in modules),
                          'with_stock_only_providers': sum(bool(x['stock_only_providers']) for x in modules)},
              'scope': 'Static provider/CRC comparison only. No module is loaded. Matching CRCs or filenames do not prove runtime ABI compatibility. Release text alone is not a rejection when CONFIG_MODVERSIONS ignores its first field.'}
    (work / (args.label + '-module-audit.json')).write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report['summary'], indent=2))


if __name__ == '__main__':
    main()
