#!/usr/bin/env python3
"""Recover embedded kernel headers from a saved kheaders.ko; no device access."""
import argparse
import hashlib
import io
import json
from pathlib import Path, PurePosixPath
import posixpath
import tarfile

from elftools.elf.elffile import ELFFile


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('module', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    if args.output.exists():
        raise FileExistsError(f'Refusing to overwrite {args.output}')
    with args.module.open('rb') as stream:
        elf = ELFFile(stream)
        symbols = {s.name: s for s in elf.get_section_by_name('.symtab').iter_symbols()}
        start, end = (symbols[n] for n in ('kernel_headers_data', 'kernel_headers_data_end'))
        assert start['st_shndx'] == end['st_shndx']
        section = elf.get_section(start['st_shndx'])
        lo, hi = start['st_value'], end['st_value']
        assert 0 <= lo < hi <= section['sh_size']
        archive = section.data()[lo:hi]
    assert archive.startswith(b'\xfd7zXZ\x00')
    # Validate the entire archive before creating any destination files.
    files, links = {}, {}
    with tarfile.open(fileobj=io.BytesIO(archive), mode='r:xz') as tar:
        for member in tar:
            path = PurePosixPath(member.name)
            if path.is_absolute() or '..' in path.parts:
                raise ValueError(f'Unsafe archive path: {member.name}')
            if member.isdir():
                continue
            name = str(path)
            if name in files or name in links:
                raise ValueError(f'Duplicate path: {name}')
            if member.issym():
                links[name] = member.linkname
                continue
            if not member.isfile():
                raise ValueError(f'Unexpected archive type: {member.name}')
            files[name] = tar.extractfile(member).read()
    for name, target in links.items():
        resolved = posixpath.normpath(posixpath.join(posixpath.dirname(name), target))
        if target.startswith('/') or resolved not in files:
            raise ValueError(f'Link must resolve to an archived regular file: {name}')
    for name in files.keys() | links.keys():
        if any(str(parent) in links for parent in PurePosixPath(name).parents):
            raise ValueError(f'Symlink parent: {name}')
    args.output.mkdir(parents=True)
    for name, data in files.items():
        destination = args.output / 'headers' / name
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_bytes(data)
    for name, target in links.items():
        (args.output / 'headers' / name).symlink_to(target)
    (args.output / 'kheaders.tar.xz').write_bytes(archive)
    report = {'input': str(args.module), 'module_sha256': sha(args.module.read_bytes()),
              'archive_sha256': sha(archive), 'archive_bytes': len(archive),
              'file_count': len(files), 'uncompressed_bytes': sum(map(len, files.values())),
              'symlinks': links,
              'files': {name: {'sha256': sha(data), 'bytes': len(data)}
                        for name, data in sorted(files.items())},
              'scope': 'Embedded stock headers and generated includes, not a complete kernel source tree.'}
    (args.output / 'extraction.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({k: v for k, v in report.items() if k != 'files'}))


if __name__ == '__main__':
    main()
