#!/usr/bin/env python3
"""Compile extra conflict-affected ARM32 subsystems in a separate output tree.

These options are for source validation, not the TV or QEMU kernel config.
"""
import json
import os
from pathlib import Path
import subprocess
import sys

sys.dont_write_bytecode = True
from prepare import WORK, previous_prepare
from storage import require_space

OPTIONS = ('NFSD', 'NFSD_V4', 'NFS_FS', 'NFS_V4', 'NFS_V4_1', 'NFS_V4_2',
           'NFSD_V4_2_INTER_SSC', 'FANOTIFY', 'FANOTIFY_ACCESS_PERMISSIONS',
           'BLK_DEV_ZONED', 'WLAN', 'WLAN_VENDOR_MARVELL', 'LIBERTAS_THINFIRM', 'TYPEC', 'TYPEC_TCPM',
           'TYPEC_UCSI', 'TYPEC_DP_ALTMODE')
TARGETS = ('fs/nfsd/', 'fs/notify/fanotify/',
           'drivers/net/wireless/marvell/libertas_tf/',
           'drivers/scsi/sd_zbc.o', 'drivers/usb/typec/tcpm/tcpm.o',
           'drivers/usb/typec/ucsi/displayport.o')


def main():
    require_space(2)
    source = WORK / 'kernel-source'
    out = WORK / 'source-check-out'
    out.mkdir(exist_ok=True)
    (out / '.config').write_bytes((WORK / 'kernel-out/.config').read_bytes())
    command = [str(source / 'scripts/config'), '--file', str(out / '.config')]
    for option in OPTIONS:
        command += ['--enable', option]
    subprocess.run(command, check=True)
    env = dict(os.environ, KBUILD_BUILD_USER='dande', KBUILD_BUILD_HOST='kernel-builder',
               KBUILD_BUILD_TIMESTAMP='Fri Oct 30 11:57:45 UTC 2026', KBUILD_BUILD_VERSION='1')
    make = ['make', '-C', str(source), 'O=' + str(out), 'ARCH=arm', 'LLVM=1',
            'HOSTCC=cc', 'HOSTCXX=c++', 'NCT5_BUILD=true', 'KERNEL_VER=linux-5.15_14']
    subprocess.run(make + ['olddefconfig'], env=env, check=True)
    config = dict(line.split('=', 1) for line in (out / '.config').read_text().splitlines()
                  if line.startswith('CONFIG_'))
    assert config.get('CONFIG_NFSD_V4') == 'y' and config.get('CONFIG_FANOTIFY') == 'y'
    disabled = [key for key in OPTIONS if config.get('CONFIG_' + key, 'n') == 'n']
    assert not disabled, ('Requested validation features were not enabled', disabled)
    subprocess.run(make + ['-j6', 'prepare', 'scripts'], env=env, check=True)
    completed = []
    for target in TARGETS:
        subprocess.run(make + ['-j6', target], env=env, check=True)
        completed.append(target)
    objects = sorted(out.rglob('*.o'))
    report = {'requested_options': {key: config.get('CONFIG_' + key, 'n') for key in OPTIONS},
              'targets': completed,
              'not_checked': {'drivers/infiniband/hw/cxgb4/': 'The captured MT5867 platform has no PCI support.'},
              'config_sha256': previous_prepare.sha(out / '.config'),
              'objects': {str(p.relative_to(out)): previous_prepare.sha(p) for p in objects},
              'scope': 'ARM32 compile checks only; separate configuration, no linked kernel or runtime claim.'}
    (WORK / 'source-check.json').write_text(json.dumps(report, indent=2) + '\n')
    print('Extra source checks passed:', len(completed), 'targets,', len(objects), 'objects')


if __name__ == '__main__':
    main()
