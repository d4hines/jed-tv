#!/usr/bin/env python3
"""Package verified build outputs and reports for transfer from the build VM."""
import hashlib
import json
from pathlib import Path
import tarfile

ROOT = Path(__file__).resolve().parents[2]
WORK = ROOT / 'assets/build/g13-kernel-vm'


def main():
    build = json.loads((WORK / 'kernel-build.json').read_text())
    files = []
    for name, expected in build['artifacts'].items():
        relative = Path(name)
        assert not relative.is_absolute() and '..' not in relative.parts
        source = WORK / 'kernel-out' / relative
        with source.open('rb') as stream:
            assert hashlib.file_digest(stream, 'sha256').hexdigest() == expected['sha256'], name
        files.append(source)
    files.append(WORK / 'kernel-out/include/config/kernel.release')
    probe = WORK / build['test_module']['path']
    with probe.open('rb') as stream:
        assert hashlib.file_digest(stream, 'sha256').hexdigest() == build['test_module']['sha256']
    files.append(probe)
    for name in ('kernel-build.json', 'stable-merge.json', 'stable-resolutions.json',
                 'source-check.json', 'source-check.log',
                 'replay-verification.json', 'replay.log',
                 'build.log', 'build-final-validation.log', 'prepare.log', 'baseline.log', 'environment.log'):
        source = WORK / name
        if source.exists():
            files.append(source)
    for pattern in ('build-attempt*.log', 'build-owner-stop.log', 'source-check-attempt*.log'):
        files.extend(sorted(WORK.glob(pattern)))
    temporary = WORK / 'artifacts.tar.gz.partial'
    with tarfile.open(temporary, 'w:gz') as archive:
        for source in files:
            assert source.is_file() and not source.is_symlink(), source
            archive.add(source, arcname=str(source.relative_to(ROOT)), recursive=False)
    destination = WORK / 'artifacts.tar.gz'
    temporary.replace(destination)
    with destination.open('rb') as stream:
        digest = hashlib.file_digest(stream, 'sha256').hexdigest()
    (WORK / 'artifacts.tar.gz.sha256').write_text(digest + '\n')
    print('Packaged', len(files), 'files:', destination, digest)


if __name__ == '__main__':
    main()
