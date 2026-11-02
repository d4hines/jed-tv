#!/usr/bin/env python3
"""Build the reviewed MT5867/Android-common 5.15.220 merge for QEMU."""
import argparse
import importlib.util
import json
from pathlib import Path
import subprocess
import sys

sys.dont_write_bytecode = True
from minimal import ROOT, WORK, sha
from storage import require_space
sys.path.insert(0, str(ROOT / 'research/g13-kernel'))
spec = importlib.util.spec_from_file_location('old_build', ROOT / 'research/g13-kernel/build.py')
old = importlib.util.module_from_spec(spec)
spec.loader.exec_module(old)


def main():
    require_space(8)
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('-j', type=int, default=4)
    args = parser.parse_args()
    assert args.j > 0
    source, out = WORK / 'kernel-source', WORK / 'kernel-out'
    merge = json.loads((WORK / 'merge-report.json').read_text())
    resolutions = json.loads((WORK / 'resolution-report.json').read_text())
    assert set(merge['conflicts']) == set(resolutions)
    for name, entry in resolutions.items():
        assert sha(source / name) == entry['sha256'], f'Resolved source changed: {name}'
    patches = [ROOT / 'research/g13-kernel' / name for name in
               ('sync-debug.patch', 'fixdep.patch', 'makefile-boot.patch')]
    for patch in patches:
        command = ['patch', '--batch', '-p1', '-i', str(patch)]
        if subprocess.run(command + ['--dry-run', '--forward'], cwd=source, capture_output=True).returncode == 0:
            subprocess.run(command + ['--forward'], cwd=source, check=True)
        else:
            subprocess.run(command + ['--dry-run', '--reverse'], cwd=source, check=True)
    (source / '.scmversion').write_text('-mt5867-ack6ce868e42ba1\n')
    out.mkdir(exist_ok=True)
    assert sha(old.CONFIG) == old.CONFIG_SHA
    if not (out / '.config').exists():
        (out / '.config').write_bytes(old.CONFIG.read_bytes())
    make = ['make', '-C', str(source), 'O=' + str(out), 'ARCH=arm', 'LLVM=1',
            'HOSTCC=cc', 'HOSTCXX=c++', 'NCT5_BUILD=true', 'KERNEL_VER=linux-5.15_14']
    subprocess.run(make + ['olddefconfig'], check=True)
    subprocess.run(make + [f'-j{args.j}', 'Image', 'modules', 'dtbs'], check=True)
    before, after = old.config(old.CONFIG), old.config(out / '.config')
    symbols = subprocess.check_output(['llvm-nm', '--defined-only', str(out / 'vmlinux')], text=True)
    entry = next(int(line.split()[0], 16) for line in symbols.splitlines() if line.split()[-1] == 'stext')
    assert entry == int(after['CONFIG_PAGE_OFFSET'], 0) + 0x8000
    paths = [out / name for name in ('arch/arm/boot/Image', 'vmlinux', '.config', 'Module.symvers')]
    modules = sorted(out.rglob('*.ko'))
    report = {'merge_report_sha256': sha(WORK / 'merge-report.json'),
              'resolution_report_sha256': sha(WORK / 'resolution-report.json'),
              'patches': {p.name: sha(p) for p in patches},
              'seed_config_sha256': old.CONFIG_SHA, 'entry_virtual_address': hex(entry),
              'release': (out / 'include/config/kernel.release').read_text().strip(),
              'command': make + [f'-j{args.j}', 'Image', 'modules', 'dtbs'],
              'toolchain': subprocess.check_output(['clang', '--version'], text=True),
              'config_changes': {key: [before.get(key), after.get(key)] for key in sorted(before.keys() | after.keys())
                                 if before.get(key) != after.get(key)},
              'artifacts': {str(p.relative_to(WORK)): {'sha256': sha(p), 'bytes': p.stat().st_size} for p in paths},
              'modules': {str(p.relative_to(out)): sha(p) for p in modules},
              'module_count': len(modules)}
    (WORK / 'kernel-build.json').write_text(json.dumps(report, indent=2) + '\n')
    print('Built', report['release'], 'with', len(modules), 'modules')


if __name__ == '__main__':
    main()
