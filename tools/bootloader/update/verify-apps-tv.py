#!/usr/bin/env python3
"""Read back a completed app update and its live mounts; no flash or reboot.

The baseline must be a full upgrade snapshot taken after Linux storage
reservation. Older snapshots may include unrelated vendor ext4 changes.
"""
import argparse
import hashlib
import json
from pathlib import Path
import shlex
import sys
sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'native-app'))
import g13_device as d


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', type=Path, required=True)
    parser.add_argument('--apps', type=Path, required=True, help='apps.lbu and manifest.json directory')
    parser.add_argument('--trial', type=Path, required=True, help='completed boot trial with result.json')
    args = parser.parse_args()
    trial = json.loads((args.trial / 'result.json').read_text())
    assert trial['ready'], 'A completed native boot is required'
    manifest = json.loads((args.apps / 'manifest.json').read_text())
    package = (args.apps / 'apps.lbu').read_bytes()
    assert sha(package) == manifest['package_sha256']
    assert sha(package[160:]) == manifest['payload_sha256']
    before = args.baseline.read_bytes()
    d.root()
    chosen = d.shell('cat /sys/firmware/devicetree/base/chosen/dande,apps-bank').strip('\0\r\n')
    assert chosen in ('0', '1'), 'No authenticated app bank selected'
    bank = int(chosen)
    after = d.adb('exec-out', 'cat /dev/block/by-name/upgrade', capture_output=True, timeout=120).stdout
    (args.trial / 'upgrade-after.img').write_bytes(after)
    assert len(before) == len(after) == 52428800
    assert sha(after) == d.shell('sha256sum /dev/block/by-name/upgrade').split()[0]
    offset = 1048576 + bank * 16777216
    assert after[offset:offset+512] == package[:160] + bytes(352)
    pid = d.shell('pidof tvinit').strip()
    assert pid.isdecimal(), 'Expected one native launcher'
    files = {name: d.shell('sha256sum ' + shlex.quote('/proc/' + pid + '/root/' + name)).split()[0]
             for name in manifest['files']}
    mounts = d.shell('cat /proc/' + pid + '/mountinfo')
    (args.trial / 'mountinfo.txt').write_text(mounts)
    app_mounts = {}
    for line in mounts.splitlines():
        left, right = line.split(' - ', 1)
        fields, filesystem = left.split(), right.split()
        if fields[4] in ('/bin', '/share'):
            app_mounts[fields[4]] = (fields[2], filesystem[0], filesystem[1])
    mapped = d.shell('dmctl getpath dande-apps').strip()
    assert set(app_mounts) == {'/bin', '/share'}, 'App submounts missing from launcher namespace'
    assert app_mounts['/bin'] == app_mounts['/share'], 'App trees use different devices'
    assert app_mounts['/bin'][1:] == ('erofs', mapped), 'Launcher is not using the authenticated mapping'
    mapping = d.shell('dmctl table dande-apps')
    (args.trial / 'mapping.txt').write_text(mapping)
    assert f'0-32767: linear, 179:45 {2049 + bank*32768}' in mapping
    other = 1048576 + (1-bank) * 16777216
    length = manifest['payload_bytes']
    tail = offset + 512 + (length+511)//512*512
    result = dict(image_sha256=trial['image_sha256'],
                  boot_a_sha256=d.shell('sha256sum /dev/block/by-name/boot_a').split()[0],
                  bank=bank, version=manifest['version'], package_sha256=sha(package),
                  payload_sha256=sha(after[offset+512:offset+512+length]),
                  payload_matches=after[offset+512:offset+512+length] == package[160:],
                  files_match=files == manifest['files'],
                  active_tail_unchanged=before[tail:offset+16777216] == after[tail:offset+16777216],
                  other_bank_unchanged=before[other:other+16777216] == after[other:other+16777216],
                  reserved_regions_unchanged=before[:1048576] == after[:1048576] and
                      before[34603008:-1024] == after[34603008:-1024],
                  upgrade_read_only=d.shell('blockdev --getro /dev/block/by-name/upgrade').strip() == '1',
                  updater_stopped=d.shell('getprop init.svc.update_engine').strip() == 'stopped',
                  image_mapping_read_only=d.shell('blockdev --getro "$(dmctl getpath dande-apps)"').strip() == '1',
                  ready=d.shell('getprop sys.dande.launcher_ready').strip() == '1', upgrade_sha256=sha(after))
    (args.trial / 'readback.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps(result, indent=2))
    for name, value in result.items():
        if isinstance(value, bool):
            assert value, name


if __name__ == '__main__':
    main()
