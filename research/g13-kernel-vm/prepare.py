#!/usr/bin/env python3
"""Reproduce the MT5867 port and merge pinned Linux stable updates offline."""
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
WORK = ROOT / 'assets/build/g13-kernel-vm'
PREVIOUS = ROOT / 'assets/build/g13-android16'
sys.path.insert(0, str(ROOT / 'research/g13-android16'))
from storage import require_space


def module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


previous_prepare = module('android16_prepare', ROOT / 'research/g13-android16/prepare.py')
merge = module('previous_merge', ROOT / 'research/g13-android16/merge-kernel.py')
STABLE = {
    'base': ('0996e0926f6b4d6123e1b94407d726bc9810248e', '83ec387bfa9c0fac6373f3942803bb203fc1e76f'),
    'new': ('700e62ea909a70b9f72692d8b7a192ac7ebde0aa', 'a2a870b5aa81d8ab4c8aaaa203b53a980c39e6cc'),
}


def baseline():
    PREVIOUS.mkdir(parents=True, exist_ok=True)
    for name in ('ack-base.tar.gz', 'ack-new.tar.gz'):
        url, digest = previous_prepare.INPUTS[name]
        previous_prepare.fetch(PREVIOUS / name, url, digest)
    previous_prepare.fetch(
        ROOT / 'assets/research/g13-kernel-source-20261101/oneplus-mt5867-android14-3543dbbd.tar.gz',
        'https://codeload.github.com/techyminati/android_kernel_oneplus_mt5867/tar.gz/3543dbbd43a7e69112b1169b34171d7bd745cdee',
        '048a1cb80336c823b3a705dbd10a413091d635f30e5c83645f4b91f562e318a3')
    if not (PREVIOUS / 'merge-report.json').exists():
        result = subprocess.run([sys.executable, str(ROOT / 'research/g13-android16/merge-kernel.py')])
        assert result.returncode == 1, 'Expected the documented 25-conflict baseline merge'
        assert len(json.loads((PREVIOUS / 'merge-report.json').read_text())['conflicts']) == 25
    if not (PREVIOUS / 'resolution-report.json').exists():
        subprocess.run([sys.executable, str(ROOT / 'research/g13-android16/resolve-kernel.py')], check=True)
    source = PREVIOUS / 'kernel-source'
    for patch in ('sync-debug.patch', 'fixdep.patch', 'makefile-boot.patch'):
        command = ['patch', '--batch', '-p1', '-i', str(ROOT / 'research/g13-kernel' / patch)]
        if subprocess.run(command + ['--dry-run', '--forward'], cwd=source, capture_output=True).returncode == 0:
            subprocess.run(command + ['--forward'], cwd=source, check=True)
        else:
            subprocess.run(command + ['--dry-run', '--reverse'], cwd=source, check=True)
    (source / '.scmversion').write_text('-mt5867-ack6ce868e42ba1\n')
    # The old host extraction normalized one PowerPC selftest symlink. Both
    # spellings resolve to the same file; all other bytes/modes must match.
    tree = merge.git_tree(source).hex()
    assert tree in {
        '858fe360987b2c5edc4d8a78113080b4b3d7e729',
        '9f2032a3aaefa1aa8df16a30d27c5b6a0d7afa22',
    }, ('Unexpected Android/MT5867 baseline', tree)
    return source


def stable_tree(label, commit, tree_id):
    archive = WORK / ('stable-' + label + '.tar.gz')
    provenance = previous_prepare.fetch(archive,
        'https://codeload.github.com/gregkh/linux/tar.gz/' + commit, None)
    destination = WORK / ('stable-' + label)
    if not destination.exists():
        temporary = destination.with_suffix('.unpack')
        temporary.mkdir()

        def safe_member(member, target):
            result = tarfile.data_filter(member, target)
            if result is not None and member.issym():
                result = result.replace(linkname=member.linkname)
            return result

        with tarfile.open(archive) as stream:
            stream.extractall(temporary, filter=safe_member)
        (temporary / ('linux-' + commit)).rename(destination)
        temporary.rmdir()
    actual = merge.git_tree(destination).hex()
    assert actual == tree_id, (label, actual, tree_id)
    provenance.update(commit=commit, tree=actual, verification='Git tree ID and recorded archive SHA256')
    return destination, provenance


def copy_entry(incoming, target):
    if target.is_file() or target.is_symlink():
        target.unlink()
    if incoming is not None:
        target.parent.mkdir(parents=True, exist_ok=True)
        if incoming.is_symlink():
            target.symlink_to(os.readlink(incoming))
        else:
            shutil.copy2(incoming, target)


def main():
    require_space(18)
    WORK.mkdir(parents=True, exist_ok=True)
    target = WORK / 'kernel-source'
    if target.exists():
        raise SystemExit('Preserving existing update; inspect stable-merge.json and kernel-source')
    original = baseline()
    trees, provenance = {}, {}
    for label, pins in STABLE.items():
        trees[label], provenance[label] = stable_tree(label, *pins)
    index = {key: merge.inventory(value) for key, value in trees.items()}
    current = merge.inventory(original)
    shutil.copytree(original, target, symlinks=True)
    report = {'inputs': provenance, 'changes': {}, 'conflicts': [],
              'baseline_source_tree': merge.git_tree(original).hex(),
              'base_note': 'Same-version stable comparison base, not a proven ancestor of the Android/vendor tree.'}
    for name in sorted(index['base'].keys() | index['new'].keys()):
        base, new, ours = index['base'].get(name), index['new'].get(name), current.get(name)
        if base == new:
            continue
        dest = target / name
        if ours == new:
            state = 'already-present'
        elif ours == base:
            copy_entry(trees['new'] / name if new is not None else None, dest)
            state = 'upstream'
        elif all(item and item[0] == 'file' for item in (base, new, ours)):
            result = subprocess.run(['git', 'merge-file', '-p', '--diff3',
                '-L', 'MT5867-Android', '-L', 'stable-5.15.220', '-L', 'stable-5.15.224',
                str(dest), str(trees['base'] / name), str(trees['new'] / name)], capture_output=True)
            assert 0 <= result.returncode <= 127, (name, result.stderr)
            dest.write_bytes(result.stdout)
            if ours[2] == base[2]:
                dest.chmod((dest.stat().st_mode & ~0o111) | new[2])
            state = 'merged' if result.returncode == 0 else 'conflict'
            if result.returncode:
                report['conflicts'].append(name)
        else:
            state = 'structural-conflict'
            report['conflicts'].append(name)
        report['changes'][name] = state
    report['source_hashes'] = {name: entry for name, entry in merge.inventory(target).items()}
    report['counts'] = {state: list(report['changes'].values()).count(state)
                        for state in sorted(set(report['changes'].values()))}
    (WORK / 'stable-merge.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({'counts': report['counts'], 'conflicts': report['conflicts']}, indent=2))
    return int(bool(report['conflicts']))


if __name__ == '__main__':
    sys.exit(main())
