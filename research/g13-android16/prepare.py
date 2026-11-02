#!/usr/bin/env python3
"""Fetch pinned sources and a donor image; extract only on the host, never a TV."""
import hashlib
import json
import lzma
from pathlib import Path
import shutil
import subprocess
import zipfile
import sys

sys.dont_write_bytecode = True
from storage import require_space

ROOT = Path(__file__).resolve().parents[2]
WORK = ROOT / 'assets/build/g13-android16'
RELEASE = 'https://github.com/TrebleDroid/treble_experimentations/releases/download/ci-20250617/'
INPUTS = {
    'ack-base.tar.gz': ('https://android.googlesource.com/kernel/common/+archive/34148b6a5103338e78d6b545099d5200b75c7e58.tar.gz', None),
    'ack-new.tar.gz': ('https://android.googlesource.com/kernel/common/+archive/6ce868e42ba10faa46e2e9f1faf94af0264eaf35.tar.gz', None),
    'system-td-arm32_binder64-vanilla.img.xz': (RELEASE + 'system-td-arm32_binder64-vanilla.img.xz',
        '3f4fc02daf6f7d0d1333dab3fb2e70f3b30c268dfc419eb42c1c2b3f6d9cc666'),
    'manifest.xml': (RELEASE + 'manifest.xml', 'af0aab7afc0daab6e3e8cc9a27d99b60742539202f9077aeedb7f4ae75f60d89'),
    'patches-for-developers.zip': (RELEASE + 'patches-for-developers.zip', '291b24cefa0430e88e08a7a9d5ca413468a723d96a4a60954607ca6b71789ce3'),
    'secilc.c': ('https://raw.githubusercontent.com/SELinuxProject/selinux/3.11/secilc/secilc.c',
        'c90dc07871e3cf52e0043cf43fd8b871af2eea699a73374e564a972d9d1a2729'),
}


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def fetch(path, url, expected):
    if not path.exists():
        path.parent.mkdir(parents=True, exist_ok=True)
        temp = path.with_suffix(path.suffix + '.download')
        subprocess.run(['curl', '-fL', '--retry', '3', '--output', str(temp), url], check=True)
        if expected:
            assert sha(temp) == expected, path
        temp.rename(path)
    actual = sha(path)
    if expected:
        assert actual == expected, path
    return {'url': url, 'sha256': actual,
            'verification': 'SHA256' if expected else 'Git tree ID checked by merge-kernel.py'}


def extract(image, destination):
    marker = destination / '.source-sha256'
    expected = sha(image)
    if destination.exists():
        assert marker.read_text().strip() == expected, f'Unverified extraction: {destination}'
        return
    temp = destination.with_suffix('.unpack')
    temp.mkdir()
    with (WORK / (destination.name + '.log')).open('w') as log:
        # debugfs does not mount the filesystem. Non-root chown warnings are
        # expected; the ramdisk builder explicitly assigns its own metadata.
        subprocess.run(['debugfs', '-R', f'rdump / "{temp}"', str(image)],
                       stdout=log, stderr=log, check=True)
    (temp / '.source-sha256').write_text(expected + '\n')
    temp.rename(destination)


def main():
    require_space(8)
    WORK.mkdir(parents=True, exist_ok=True)
    provenance = {}
    for name, (url, expected) in INPUTS.items():
        provenance[name] = fetch(WORK / name, url, expected)
    provenance['vendor_source'] = fetch(
        ROOT / 'assets/research/g13-kernel-source-20261101/oneplus-mt5867-android14-3543dbbd.tar.gz',
        'https://codeload.github.com/techyminati/android_kernel_oneplus_mt5867/tar.gz/3543dbbd43a7e69112b1169b34171d7bd745cdee',
        '048a1cb80336c823b3a705dbd10a413091d635f30e5c83645f4b91f562e318a3')
    image = WORK / 'system-td-arm32_binder64-vanilla.img'
    if not image.exists():
        temp = image.with_suffix('.unpack')
        with lzma.open(WORK / (image.name + '.xz'), 'rb') as source, temp.open('wb') as dest:
            shutil.copyfileobj(source, dest)
        temp.rename(image)
    assert sha(image) == '031dee89b2c5b3332c829f97f2c001855311487aa505343f2458d535b5fac4d7'
    extract(image, WORK / 'android16-extracted')
    runtime = WORK / 'runtime-apex.img'
    if not runtime.exists():
        with zipfile.ZipFile(WORK / 'android16-extracted/system/apex/com.android.runtime.apex') as archive:
            runtime.write_bytes(archive.read('apex_payload.img'))
    assert sha(runtime) == '8a5e7e1fd425a7d95e45a1223635887af197a18a0daad1a3a461ebfb4459e602'
    extract(runtime, WORK / 'runtime-apex')
    flags = subprocess.check_output(['pkg-config', '--cflags', '--libs', 'libsepol'], text=True).split()
    subprocess.run(['cc', str(WORK / 'secilc.c'), '-o', str(WORK / 'secilc'), *flags], check=True)
    provenance['system_image_sha256'] = sha(image)
    provenance['runtime_apex_sha256'] = sha(runtime)
    (WORK / 'inputs.json').write_text(json.dumps(provenance, indent=2) + '\n')
    print('Prepared native donor and source archives in', WORK)


if __name__ == '__main__':
    main()
