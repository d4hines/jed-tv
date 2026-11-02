#!/usr/bin/env python3
"""Three-way merge pinned Android 5.15 updates into the MT5867 release."""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile

sys.dont_write_bytecode = True
from storage import require_space

ROOT = Path(__file__).resolve().parents[2]
WORK = ROOT / 'assets/build/g13-android16'
VENDOR_COMMIT = '3543dbbd43a7e69112b1169b34171d7bd745cdee'
BASE_COMMIT = '34148b6a5103338e78d6b545099d5200b75c7e58'
NEW_COMMIT = '6ce868e42ba10faa46e2e9f1faf94af0264eaf35'
TREE_IDS = {'base': '2f06cbf74d9475396c270d623a6844f25793fc88',
            'new': 'ae700f4a97747191ab0c1063ad9d37508ddca5ac'}


def git_object(kind, data):
    return hashlib.sha1(kind.encode() + b' ' + str(len(data)).encode() + b'\0' + data).digest()


def git_tree(root):
    """Reconstruct the Git tree ID, excluding mutable archive timestamps."""
    entries = []
    for path in root.iterdir():
        name = os.fsencode(path.name)
        if path.is_symlink():
            mode, oid, key = b'120000', git_object('blob', os.fsencode(os.readlink(path))), name
        elif path.is_dir():
            mode, oid, key = b'40000', git_tree(path), name + b'/'
        else:
            mode = b'100755' if path.stat().st_mode & 0o111 else b'100644'
            oid, key = git_object('blob', path.read_bytes()), name
        entries.append((key, mode + b' ' + name + b'\0' + oid))
    return git_object('tree', b''.join(value for key, value in sorted(entries)))


def digest(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def extract(archive, destination, vendor=False):
    if destination.exists():
        return
    temp = destination.with_name(destination.name + '.unpack')
    temp.mkdir()
    prefix = 'android_kernel_oneplus_mt5867-' + VENDOR_COMMIT

    def filter_member(member, target):
        if vendor and member.issym() and member.linkname.startswith('/home/ubuntu/ack/'):
            relative = prefix + '/' + member.linkname.removeprefix('/home/ubuntu/ack/')
            member = member.replace(linkname=os.path.relpath(relative, os.path.dirname(member.name)))
        # Python 3.14 normalizes symlink text. Validate the destination but keep
        # Git's exact link bytes, which are part of the tree object hash.
        filtered = tarfile.data_filter(member, target)
        if filtered is not None and member.issym():
            filtered = filtered.replace(linkname=member.linkname)
        return filtered

    with tarfile.open(archive) as stream:
        stream.extractall(temp, filter=filter_member)
    if vendor:
        (temp / prefix).rename(destination)
        temp.rmdir()
    else:
        temp.rename(destination)


def inventory(root):
    result = {}
    for current, dirs, files in os.walk(root):
        for name in dirs[:]:
            path = Path(current) / name
            if path.is_symlink():
                files.append(name)
                dirs.remove(name)
        for name in files:
            path = Path(current) / name
            rel = str(path.relative_to(root))
            if path.is_symlink():
                result[rel] = ('link', os.readlink(path), 0)
            elif path.is_file():
                result[rel] = ('file', digest(path), path.stat().st_mode & 0o111)
    return result


def main():
    require_space(12)
    archives = {
        'base': WORK / 'ack-base.tar.gz',
        'new': WORK / 'ack-new.tar.gz',
        'vendor': ROOT / 'assets/research/g13-kernel-source-20261101/oneplus-mt5867-android14-3543dbbd.tar.gz',
    }
    source = WORK / 'kernel-source'
    if source.exists():
        raise SystemExit('Preserving existing merge; inspect merge-report.json and kernel-source')
    assert digest(archives['vendor']) == '048a1cb80336c823b3a705dbd10a413091d635f30e5c83645f4b91f562e318a3'
    for name, path in archives.items():
        extract(path, WORK / ('source-' + name), vendor=name == 'vendor')
    # Gitiles archives carry request-time timestamps, so their SHA256 changes.
    # Verify every file, executable bit, and symlink against the pinned Git tree.
    for name, expected in TREE_IDS.items():
        actual = git_tree(WORK / ('source-' + name)).hex()
        assert actual == expected, (name, actual, expected)
    trees = {name: inventory(WORK / ('source-' + name)) for name in archives}
    shutil.copytree(WORK / 'source-vendor', source, symlinks=True)
    counts, conflicts, changes = {}, [], []
    for rel in sorted(trees['base'].keys() | trees['new'].keys()):
        base, new, vendor = [trees[name].get(rel) for name in ('base', 'new', 'vendor')]
        if base == new:
            continue
        path = source / rel
        if vendor == new:
            status = 'already-present'
        elif vendor == base:
            if path.is_symlink() or path.is_file():
                path.unlink()
            if new is not None:
                path.parent.mkdir(parents=True, exist_ok=True)
                incoming = WORK / 'source-new' / rel
                if incoming.is_symlink():
                    path.symlink_to(os.readlink(incoming))
                else:
                    shutil.copy2(incoming, path)
            status = 'upstream'
        elif all(item and item[0] == 'file' for item in (base, new, vendor)):
            result = subprocess.run(['git', 'merge-file', '-p', '--diff3',
                '-L', 'MT5867-vendor', '-L', 'Android-5.15.148-base', '-L', 'Android-5.15.220',
                str(path), str(WORK / 'source-base' / rel), str(WORK / 'source-new' / rel)], capture_output=True)
            if result.returncode < 0 or result.returncode > 127:
                raise RuntimeError((rel, result.stderr.decode(errors='replace')))
            if result.returncode == 0:
                path.write_bytes(result.stdout)
                status = 'merged'
            else:
                # Keep exact inputs and conflict markers for review; never choose a side silently.
                if result.stdout:
                    path.write_bytes(result.stdout)
                status = 'conflict'
        else:
            status = 'structural-conflict'
        counts[status] = counts.get(status, 0) + 1
        changes.append({'path': rel, 'status': status})
        if 'conflict' in status:
            conflicts.append(rel)
    report = {
        'vendor_commit': VENDOR_COMMIT, 'base_commit': BASE_COMMIT, 'new_commit': NEW_COMMIT,
        'verified_git_trees': TREE_IDS,
        'base_note': 'Same-version Android common comparison base; not a proven vendor Git ancestor.',
        'archives': {name: {'path': str(path.relative_to(ROOT)), 'sha256': digest(path)} for name, path in archives.items()},
        'inventory_files': {name: len(tree) for name, tree in trees.items()},
        'vendor_equal_base': sum(trees['vendor'].get(p) == value for p, value in trees['base'].items()),
        'counts': counts, 'conflicts': conflicts, 'changes': changes,
    }
    (WORK / 'merge-report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({k:v for k,v in report.items() if k != 'changes'}, indent=2))
    return bool(conflicts)


if __name__ == '__main__':
    sys.exit(main())
