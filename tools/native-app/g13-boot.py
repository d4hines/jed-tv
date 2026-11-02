#!/usr/bin/env python3
"""Stage G13 startup files or select the next boot. Does not flash or reboot."""
import argparse
import hashlib
import json
from pathlib import Path
import shlex
import subprocess
import datetime
import g13_device as device

SERIAL = '18ACC22D1C47'
CONFIG = '/metadata/dande-tv'
BASE = '/data/local/tmp/dande-g13'
ROOT_SHA = 'afc34332ba1b0f4b737eecdc512016e94bb3b9c058c920f411ef53c0393a70a4'


def adb(*args):
    return device.adb(*args, capture_output=True, text=True, timeout=120).stdout


def shell(command):
    return adb('shell', 'set -eu; ' + command)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def checks():
    device.root()
    state = shell('id -u; getprop ro.serialno; getprop ro.product.device; '
                  'getprop ro.build.version.incremental; getprop ro.boot.slot_suffix')
    if state.splitlines() != ['0', SERIAL, 'G13', 'AU13', '_a']:
        raise RuntimeError(f'Expected rooted G13/AU13 in slot a: {state!r}')


def read_hash(path):
    return shell('sha256sum ' + shlex.quote(path)).split()[0]


def verify(manifest, entry=None):
    expected = {f'{CONFIG}/{name}': value for name, value in manifest['config_sha256'].items()
                if name not in ['mode', 'entry.rc']}
    expected.update({f'{BASE}/{name}': value for name, value in manifest['runtime_sha256'].items()})
    for path, value in expected.items():
        if read_hash(path) != value:
            raise RuntimeError(f'Device hash mismatch: {path}')
    if entry:
        shell(f'cmp {CONFIG}/entry.rc {CONFIG}/{entry}')


def stage(candidate):
    manifest = json.loads((candidate / 'manifest.json').read_text())
    if read_hash('/dev/block/by-name/init_boot_a') != ROOT_SHA:
        raise RuntimeError('Stage before flashing, with the verified working root image installed')
    if digest(candidate / 'init_boot-native-startup.img') != manifest['image_sha256']:
        raise RuntimeError('Candidate image hash mismatch')
    for name, value in manifest['config_sha256'].items():
        if digest(candidate / 'config' / name) != value:
            raise RuntimeError(f'Local config hash mismatch: {name}')
    for name, value in manifest['runtime_sha256'].items():
        if digest(candidate / name) != value:
            raise RuntimeError(f'Local runtime hash mismatch: {name}')
    shell(f'test ! -e {CONFIG}; test ! -e {CONFIG}.next; '
          f'test -x {BASE}/root/bin/tvinit; test -x {BASE}/android-client; '
          f'test -x {BASE}/native-display; test -x {BASE}/adb-relay')
    before = candidate / 'device-before'
    before.mkdir(exist_ok=False)
    for name in manifest['runtime_sha256']:
        adb('pull', f'{BASE}/{name}', before / name)
    # Build the entire inactive directory, then publish it in one rename.
    shell(f'mkdir -m 700 {CONFIG}.next')
    for name in manifest['config_sha256']:
        adb('push', candidate / 'config' / name, f'{CONFIG}.next/{name}')
    adb('push', candidate / 'manifest.json', f'{CONFIG}.next/manifest.json')
    shell(f'chown -R 0:0 {CONFIG}.next; chmod 600 {CONFIG}.next/*; '
          f'restorecon -RF {CONFIG}.next; sync; mv {CONFIG}.next {CONFIG}; sync')
    for name in manifest['runtime_sha256']:
        adb('push', candidate / name, f'{BASE}/{name}.next')
        shell(f'chown 0:0 {BASE}/{name}.next; chmod 755 {BASE}/{name}.next; '
              f'restorecon {BASE}/{name}.next; mv {BASE}/{name}.next {BASE}/{name}')
    shell('sync')
    verify(manifest, 'android.rc')
    state = shell(f'ls -ldZ {CONFIG}; ls -lZ {CONFIG}; getprop init.svc.zygote; '
                  'pidof system_server; getenforce; sha256sum /dev/block/by-name/init_boot_a')
    (candidate / 'staging.json').write_text(json.dumps(dict(
        verified=True, entry='android.rc', mode='disabled', flashed=False,
        rebooted=False, state=state), indent=2) + '\n')
    print('Staged and readback verified. Android remains selected; no flash or reboot.')


def select(mode):
    manifest = json.loads(shell(f'cat {CONFIG}/manifest.json'))
    if read_hash('/dev/block/by-name/init_boot_a') != manifest['image_sha256']:
        raise RuntimeError('The matching startup image must be installed first')
    verify(manifest)
    if mode == 'persistent':
        state = shell('getprop sys.dande.native_ready; getprop sys.dande.launcher_ready; '
                      'if pidof system_server; then exit 1; fi; pidof tvinit >/dev/null')
        if state.splitlines() != ['1', '1']:
            raise RuntimeError('Confirm the one-shot native boot before enabling persistence')
    entry = 'android.rc' if mode == 'android' else 'native-entry.rc'
    value = 'disabled' if mode == 'android' else mode
    shell(f'umask 077; printf "%s\\n" {value} > {CONFIG}/mode.next; '
          f'chown 0:0 {CONFIG}/mode.next; chmod 600 {CONFIG}/mode.next; '
          f'restorecon {CONFIG}/mode.next; mv {CONFIG}/mode.next {CONFIG}/mode; '
          f'cp {CONFIG}/{entry} {CONFIG}/entry.next; chmod 600 {CONFIG}/entry.next; '
          f'restorecon {CONFIG}/entry.next; sync; mv {CONFIG}/entry.next {CONFIG}/entry.rc; sync')
    verify(manifest, entry)
    print(f'Selected {mode} for the next boot. No reboot performed.')


def install_config(sources, label, runtime_sources=()):
    """Back up, validate and replace scripts, preserving the boot selection."""
    manifest = json.loads(shell(f'cat {CONFIG}/manifest.json'))
    if read_hash('/dev/block/by-name/init_boot_a') != manifest['image_sha256']:
        raise RuntimeError('Expected the installed native-startup image')
    verify(manifest)
    root = Path(__file__).resolve().parents[2]
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    backup = root / 'assets/research' / f'g13-{label}-config-{stamp}'
    backup.mkdir(parents=True, exist_ok=False)
    adb('pull', CONFIG, backup / 'before')
    for _, name in runtime_sources:
        (backup / 'runtime-before').mkdir(exist_ok=True)
        adb('pull', f'{BASE}/{name}', backup / 'runtime-before' / name)
    (backup / 'before-state.txt').write_text(shell(
        f'ls -laZ {CONFIG}; ip address; ip rule; ip route show table all'))
    selection = {name: read_hash(f'{CONFIG}/{name}') for name in ['mode', 'entry.rc']}
    # Stage and check every file before publishing any; put shared helpers first.
    files = [(source, name, CONFIG, 'config_sha256', '600') for source, name in sources]
    files += [(source, name, BASE, 'runtime_sha256', '755') for source, name in runtime_sources]
    for source, name, directory, hashes, mode in files:
        local = root / 'src/pid1/scripts/native-app' / source
        destination = f'{directory}/{name}'
        adb('push', local, destination + '.next')
        shell(f'chown 0:0 {destination}.next; chmod {mode} {destination}.next; '
              f'restorecon {destination}.next')
        if name.endswith('.sh'):
            shell(f'/system/bin/sh -n {destination}.next')
        if read_hash(destination + '.next') != digest(local):
            raise RuntimeError(f'Staged configuration readback failed: {name}')
        manifest[hashes][name] = digest(local)
    shell('sync')
    for _, name, directory, _, _ in files:
        shell(f'mv {directory}/{name}.next {directory}/{name}')
    manifest['network'] = dict(interface='eth0', address='192.168.77.3/24', adb='192.168.77.3:5555')
    updated = backup / 'manifest.json'
    updated.write_text(json.dumps(manifest, indent=2) + '\n')
    adb('push', updated, f'{CONFIG}/manifest.json.next')
    shell(f'chown 0:0 {CONFIG}/manifest.json.next; chmod 600 {CONFIG}/manifest.json.next; '
          f'restorecon {CONFIG}/manifest.json.next; sync; '
          f'mv {CONFIG}/manifest.json.next {CONFIG}/manifest.json; sync')
    verify(manifest)
    if selection != {name: read_hash(f'{CONFIG}/{name}') for name in selection}:
        raise RuntimeError('Boot selection changed during configuration installation')
    return backup


def network():
    """Install the native-boot Ethernet service without replacing the boot image."""
    backup = install_config([('boot-log-g13.sh', 'boot-log.sh'),
                             ('network-g13.sh', 'network.sh'),
                             ('native-g13.rc', 'native.rc')], 'network')
    state = shell(f'/system/bin/sh {CONFIG}/network.sh; ip -4 address show dev eth0; '
                  'ip route get 192.168.77.1; getprop service.adb.tcp.port')
    (backup / 'after-state.txt').write_text(state)
    print(state, end='')
    print(f'Installed Ethernet startup service. Backup: {backup}. No flash or reboot.')


def logging():
    """Install startup milestones for the next boot; do not restart services."""
    backup = install_config([('boot-log-g13.sh', 'boot-log.sh'),
                             ('early-g13.sh', 'early.sh'),
                             ('boot-g13.sh', 'boot.sh'),
                             ('network-g13.sh', 'network.sh')], 'logging')
    print(f'Installed UART startup milestones for the next boot. Backup: {backup}. No flash or reboot.')


def parallel():
    """Overlap audio initialization with the launcher's first-frame setup."""
    backup = install_config([], 'parallel', [('supervise-android.sh', 'supervise.sh')])
    print(f'Installed concurrent audio startup for the next boot. Backup: {backup}. No flash or reboot.')


def appimages():
    """Consume bootloader-authenticated application banks on the next boot."""
    backup = install_config([('app-storage-g13.rc', 'app-storage.rc'),
                             ('app-storage-g13.sh', 'app-storage.sh'),
                             ('apps-g13.sh', 'apps.sh'), ('boot-g13.sh', 'boot.sh')], 'app-images',
                            [('start-android.sh', 'start.sh')])
    manifest = json.loads(shell(f'cat {CONFIG}/manifest.json'))
    # Keep the selected OS and existing imports; protect the reserved partition
    # even if a later native-startup failure selects the Android template.
    names = ['android.rc', 'native-entry.rc', 'entry.rc']
    for name in names:
        content = shell(f'cat {CONFIG}/{name}')
        line = f'import {CONFIG}/app-storage.rc'
        if line not in content.splitlines(): content = content.rstrip() + '\n' + line + '\n'
        local = backup / ('reserved-' + name)
        local.write_text(content)
        adb('push', local, f'{CONFIG}/{name}.next')
        shell(f'chown 0:0 {CONFIG}/{name}.next; chmod 600 {CONFIG}/{name}.next; restorecon {CONFIG}/{name}.next')
        if read_hash(f'{CONFIG}/{name}.next') != digest(local):
            raise RuntimeError(f'Reservation import readback failed: {name}')
        manifest['config_sha256'][name] = digest(local)
    shell(f'/system/bin/sh {CONFIG}/app-storage.sh')
    for name in names: shell(f'sync; mv {CONFIG}/{name}.next {CONFIG}/{name}')
    updated = backup / 'reserved-manifest.json'
    updated.write_text(json.dumps(manifest, indent=2) + '\n')
    adb('push', updated, f'{CONFIG}/manifest.json.next')
    shell(f'chown 0:0 {CONFIG}/manifest.json.next; chmod 600 {CONFIG}/manifest.json.next; '
          f'restorecon {CONFIG}/manifest.json.next; sync; '
          f'mv {CONFIG}/manifest.json.next {CONFIG}/manifest.json; sync')
    verify(manifest)
    print(f'Installed signed app-image startup and reserved upgrade read-only in Linux. Backup: {backup}. No flash or reboot.')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('action', choices=['stage', 'android', 'once', 'persistent', 'network', 'logging', 'parallel', 'appimages'])
    parser.add_argument('candidate', nargs='?', type=Path)
    args = parser.parse_args()
    if (args.action == 'stage') != bool(args.candidate):
        parser.error('Only stage takes the candidate directory')
    checks()
    if args.action == 'stage':
        stage(args.candidate.resolve())
    elif args.action == 'network':
        network()
    elif args.action == 'logging':
        logging()
    elif args.action == 'parallel':
        parallel()
    elif args.action == 'appimages':
        appimages()
    else:
        select(args.action)


if __name__ == '__main__':
    main()
