#!/usr/bin/env python3
"""Build and sign the G13 application filesystem; no device access."""
import argparse
import ast
import hashlib
import json
from pathlib import Path
import shutil
import struct
import subprocess
import tarfile
import tempfile
import sys
sys.dont_write_bytecode = True
from package import package

ROOT = Path(__file__).resolve().parents[3]
CAPACITY = 16 * 1024 * 1024 - 512


def build(record, key, version, output):
    inputs = json.loads(record.read_text())
    apps, media = Path(inputs['apps']), Path(inputs['media'])
    # Reuse the deployment list without importing any device-control code.
    definitions = ast.parse((ROOT/'tools/native-app/run-g13.py').read_text())
    constants = {node.targets[0].id: ast.literal_eval(node.value)
                 for node in definitions.body if isinstance(node, ast.Assign)
                 and isinstance(node.targets[0], ast.Name)
                 and node.targets[0].id in ('APP_BINARIES', 'APP_UIDS')}
    for name in constants['APP_BINARIES']:
        if not (apps/'bin'/name).is_file(): raise ValueError('Missing app: '+name)
    # The image retains the existing G13 build's confinement identities.
    identities = json.loads((apps/'app-uids.json').read_text())
    if identities != constants['APP_UIDS']:
        raise ValueError('Expected G13 application identities')
    output.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='apps-', dir=output) as temporary:
        tree = Path(temporary)/'root'; tree.mkdir()
        (tree/'bin').mkdir()
        for name in constants['APP_BINARIES']: shutil.copyfile(apps/'bin'/name, tree/'bin'/name)
        shutil.copytree(media/'tiles', tree/'share/tiles', symlinks=False)
        for path in (tree/'share').rglob('*'):
            path.chmod(0o755 if path.is_dir() else 0o644)
        shutil.copytree(apps/'share', tree/'share', symlinks=False, dirs_exist_ok=True)
        files = {}
        for path in sorted(tree.rglob('*')):
            if path.is_file():
                path.chmod(0o755 if path.parent == tree/'bin' else 0o644)
                files[str(path.relative_to(tree))] = hashlib.sha256(path.read_bytes()).hexdigest()
            elif path.is_dir(): path.chmod(0o755)
        image = output/'apps.erofs'
        # Put labels in the image. G13 rejects context= overrides on EROFS;
        # unlabeled app files cannot be executed by the confined launcher.
        # PAX xattrs avoid requiring host root privileges to set security.*.
        archive = Path(temporary)/'root.tar'
        with tarfile.open(archive, 'w', format=tarfile.PAX_FORMAT) as tar:
            for path in [tree, *sorted(tree.rglob('*'))]:
                info = tar.gettarinfo(str(path), arcname='.' if path == tree else str(path.relative_to(tree)))
                info.uid = info.gid = info.mtime = 0
                info.uname = info.gname = ''
                info.pax_headers = {'SCHILY.xattr.security.selinux': 'u:object_r:shell_data_file:s0\0'}
                if path.is_file():
                    with path.open('rb') as source: tar.addfile(info, source)
                else: tar.addfile(info)
        # 4 KiB LZ4 layout supported by the recorded G13 5.15.167 kernel.
        subprocess.run(['mkfs.erofs', '-zlz4hc', '-E', 'legacy-compress', '-C4096',
                        '-T0', '-U00000000-0000-0000-0000-000000000013', '--all-root',
                        '--tar=f', str(image), str(archive)], check=True)
        raw = image.read_bytes()
        if len(raw) > CAPACITY: raise ValueError(f'Image {len(raw)} exceeds bank payload {CAPACITY}')
        if struct.unpack_from('<I', raw, 1024)[0] != 0xe0f5e1e2: raise ValueError('EROFS magic')
        if struct.unpack_from('<I', raw, 1024+80)[0] & ~1:
            raise ValueError('EROFS feature newer than the G13 compression profile')
        subprocess.run(['fsck.erofs', '--extract', str(image)], check=True)
        blob = package(raw, key, version, component=2)
        (output/'apps.lbu').write_bytes(blob)
        result = dict(version=version, component=2, payload_bytes=len(raw), capacity=CAPACITY,
                      payload_sha256=hashlib.sha256(raw).hexdigest(),
                      package_sha256=hashlib.sha256(blob).hexdigest(), files=files,
                      app_uids=identities, source_apps=str(apps), hardware_tested=False)
        (output/'manifest.json').write_text(json.dumps(result, indent=2)+'\n')
        print(json.dumps({k:v for k,v in result.items() if k != 'files'}, indent=2))


if __name__ == '__main__':
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument('--record', type=Path, default=ROOT/'assets/build/native-app-g13/build.json')
    p.add_argument('--key', type=Path, required=True)
    p.add_argument('--version', type=int, required=True)
    p.add_argument('--output', type=Path, default=ROOT/'assets/build/leanboot-apps')
    a = p.parse_args(); build(a.record, a.key, a.version, a.output)
