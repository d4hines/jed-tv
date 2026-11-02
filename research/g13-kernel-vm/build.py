#!/usr/bin/env python3
"""Build the reviewed 5.15.224 MT5867 tree with a recorded host/toolchain."""
import argparse
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import sys

sys.dont_write_bytecode = True
from prepare import ROOT, WORK, merge
from storage import require_space
from prepare import previous_prepare

CONFIG = ROOT / 'assets/research/g13-kernel-source-20261101/live.config'
CONFIG_SHA = '90fcf9a9d0bc5d51da327392413b13fbd5ef3dcc4509346c67f2e00d361989aa'


def config(path):
    values = {}
    for line in path.read_text().splitlines():
        if line.startswith('CONFIG_'):
            key, value = line.split('=', 1)
            values[key] = value
        elif line.startswith('# CONFIG_') and line.endswith(' is not set'):
            values[line[2:-11]] = 'n'
    return values


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('-j', type=int, default=6)
    args = parser.parse_args()
    assert args.j > 0
    require_space(8)
    source, out = WORK / 'kernel-source', WORK / 'kernel-out'
    report = json.loads((WORK / 'stable-merge.json').read_text())
    expected = {name: tuple(value) for name, value in report['source_hashes'].items()}
    resolutions = {}
    if report['conflicts']:
        resolutions = json.loads((WORK / 'stable-resolutions.json').read_text())
        assert set(report['conflicts']) <= set(resolutions)
        for name, value in resolutions.items():
            expected[name] = ('file', value['sha256'], expected[name][2])
    current = merge.inventory(source)
    differences = [name for name in expected.keys() | current.keys()
                   if name != '.scmversion' and expected.get(name) != current.get(name)]
    assert not differences, ('Unreviewed source changes', differences)
    (source / '.scmversion').write_text('-mt5867-ack14\n')
    out.mkdir(exist_ok=True)
    assert previous_prepare.sha(CONFIG) == CONFIG_SHA
    if not (out / '.config').exists():
        (out / '.config').write_bytes(CONFIG.read_bytes())
    environment = dict(os.environ)
    identity = {'KBUILD_BUILD_USER': 'dande', 'KBUILD_BUILD_HOST': 'kernel-builder',
                'KBUILD_BUILD_TIMESTAMP': 'Fri Oct 30 11:57:45 UTC 2026', 'KBUILD_BUILD_VERSION': '1'}
    environment.update(identity)
    make = ['make', '-C', str(source), 'O=' + str(out), 'ARCH=arm', 'LLVM=1',
            'HOSTCC=cc', 'HOSTCXX=c++', 'NCT5_BUILD=true', 'KERNEL_VER=linux-5.15_14']
    subprocess.run(make + ['olddefconfig'], env=environment, check=True)
    subprocess.run(make + [f'-j{args.j}', 'Image', 'modules', 'dtbs'], env=environment, check=True)
    probe = WORK / 'timer-probe'
    probe.mkdir(exist_ok=True)
    shutil.copy2(Path(__file__).with_name('timer-probe.c'), probe / 'timer_probe.c')
    (probe / 'Makefile').write_text('obj-m := timer_probe.o\n')
    subprocess.run(make + ['M=' + str(probe), 'modules'], env=environment, check=True)
    before, after = config(CONFIG), config(out / '.config')
    symbols = subprocess.check_output(['llvm-nm', '--defined-only', str(out / 'vmlinux')], text=True)
    entry = next(int(line.split()[0], 16) for line in symbols.splitlines() if line.split()[-1] == 'stext')
    assert entry == int(after['CONFIG_PAGE_OFFSET'], 0) + 0x8000
    release = (out / 'include/config/kernel.release').read_text().strip()
    assert release == '5.15.224-mt5867-ack14', release
    files = [out / name for name in ('arch/arm/boot/Image', 'vmlinux', '.config', 'Module.symvers')]
    modules = sorted(out.rglob('*.ko'))
    dtbs = sorted((out / 'arch/arm/boot/dts').rglob('*.dtb'))
    record = {
        'release': release, 'host': platform.uname()._asdict(), 'build_identity': identity,
        'command': make + [f'-j{args.j}', 'Image', 'modules', 'dtbs'],
        'toolchain': subprocess.check_output(['clang', '--version'], text=True),
        'source_tree': merge.git_tree(source).hex(),
        'recipe_sha256': {str(p.relative_to(ROOT)): previous_prepare.sha(p)
                         for p in [Path(__file__).with_name(name) for name in
                                   ('build.py', 'prepare.py', 'resolve.py', 'shell.nix', 'timer-probe.c')]},
        'merge_sha256': previous_prepare.sha(WORK / 'stable-merge.json'),
        'resolutions': resolutions, 'seed_config_sha256': CONFIG_SHA,
        'entry_virtual_address': hex(entry),
        'config_changes': {key: [before.get(key), after.get(key)] for key in sorted(before.keys() | after.keys())
                           if before.get(key) != after.get(key)},
        'artifacts': {str(p.relative_to(out)): {'sha256': previous_prepare.sha(p), 'bytes': p.stat().st_size}
                      for p in files + modules + dtbs},
        'module_count': len(modules), 'dtb_count': len(dtbs), 'tv_accessed': False,
        'test_module': {'path': 'timer-probe/timer_probe.ko',
                        'sha256': previous_prepare.sha(probe / 'timer_probe.ko')},
    }
    (WORK / 'kernel-build.json').write_text(json.dumps(record, indent=2) + '\n')
    print('Built', release, 'modules:', len(modules), 'DTBs:', len(dtbs), flush=True)


if __name__ == '__main__':
    main()
