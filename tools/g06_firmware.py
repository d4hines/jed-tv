#!/usr/bin/env python3
"""Stage and audit original G06/V109 dependencies offline; never contact the TV.

Usage: python3 tools/g06_firmware.py
Output: assets/build/g06-platform/{root,manifest.json}. Vendor userspace is
staged for analysis, never automatically started or included in boot images.
"""
import collections
import hashlib
import json
import os
import pathlib
import re
import shutil
import struct
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[1]
BACKUP = ROOT / 'assets/backups/v109-stock-20261031'
OUT = ROOT / 'assets/build/g06-platform'
VENDOR = ROOT / 'assets/research/tv-pid1/v109/super/vendor.img'
VENDOR_SHA = '8de133daa7fe13a4910136cde938852dc182c309aef7fdc84c6656b21546dcd3'
MODULES = ['iniparser', 'kdrv_platform', 'kdrv_xc', 'kdrv_alg', 'libdmdbraslia',
           'utpa2k', 'hdcp1x', 'hdcp2x', 'xcker', 'mwgifker', 'misck', 'mik', 'mstar_fbdev_mi']
SEARCH = ['/mnt/vendor/tvservice/glibc', '/vendor/tvconfig/config', '/vendor/lib']

def sha(path):
    h = hashlib.sha256()
    with open(path, 'rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()


def elf(data):
    """Read ARM little-endian ELF32 metadata without running vendor code."""
    if len(data) < 52 or data[:7] != b'\x7fELF\x01\x01\x01':
        raise ValueError('expected little-endian ELF32')
    h = struct.unpack_from('<16sHHIIIIIHHHHHH', data)
    if h[2] != 40:
        raise ValueError('expected ARM ELF')
    def block(off, size):
        if off < 0 or size < 0 or off + size > len(data):
            raise ValueError('ELF range outside file')
        return data[off:off + size]
    def string(b, off):
        if off >= len(b):
            raise ValueError('ELF string outside table')
        return b[off:].split(b'\0', 1)[0].decode('utf-8')
    if h[11] < 40 or h[12] == 0 or h[13] >= h[12]:
        raise ValueError('invalid ELF section table')
    sections = [struct.unpack('<IIIIIIIIII', block(h[6] + i * h[11], 40)) for i in range(h[12])]
    ns = sections[h[13]]
    names = block(ns[4], ns[5])
    byname = {string(names, s[0]): s for s in sections}
    result = {'needed': [], 'interpreter': None, 'modinfo': {}, 'imports': [], 'exports': []}
    for name, section in byname.items():
        if name == '__ksymtab_strings':
            result['exports'] = [x.decode() for x in block(section[4], section[5]).split(b'\0') if x]
        if section[1] == 2:  # SHT_SYMTAB: only strong undefined symbols need providers
            strings = sections[section[6]]
            st = block(strings[4], strings[5])
            symbols = block(section[4], section[5])
            for i in range(0, len(symbols), 16):
                symbol = struct.unpack_from('<IIIBBH', symbols, i)
                if symbol[5] == 0 and symbol[3] >> 4 == 1:
                    result['imports'].append(string(st, symbol[0]))
        if name == '.modinfo':
            for entry in block(section[4], section[5]).split(b'\0'):
                if b'=' in entry:
                    k, v = entry.decode().split('=', 1)
                    result['modinfo'][k] = v
        if name == '.gnu.linkonce.this_module':
            # Exact ARM32 4.19 struct module: state + list_head, then name[56].
            namebytes = block(section[4] + 12, 56).split(b'\0', 1)[0]
            result['module_name'] = namebytes.decode('ascii')
        if section[1] == 6:  # SHT_DYNAMIC
            strings = sections[section[6]]
            st = block(strings[4], strings[5])
            dyn = block(section[4], section[5])
            for i in range(0, len(dyn), 8):
                tag, val = struct.unpack_from('<II', dyn, i)
                if tag == 1:
                    result['needed'].append(string(st, val))
    if h[10] and h[9] < 32:
        raise ValueError('invalid ELF program table')
    for i in range(h[10]):
        p = struct.unpack('<IIIIIIII', block(h[5] + i * h[9], 32))
        if p[0] == 3:
            result['interpreter'] = block(p[1], p[4]).rstrip(b'\0').decode()
    return result


class Files:
    def __init__(self):
        self.debugfs = shutil.which('debugfs')
        if not self.debugfs:
            candidates = sorted(pathlib.Path('/nix/store').glob('*e2fsprogs*-bin/bin/debugfs'))
            if not candidates:
                raise RuntimeError('run in nix shell nixpkgs#e2fsprogs')
            self.debugfs = str(candidates[-1])
        self.records = {}
        self.absent = set()

    def map(self, guest):
        p = pathlib.PurePosixPath(guest)
        if not p.is_absolute() or '..' in p.parts or any(c in guest for c in '\n\r" '):
            raise ValueError('unsafe guest path: ' + guest)
        for prefix, image in [('/vendor/tvconfig', BACKUP / 'tvconfig.img'),
                              ('/mnt/vendor/tvservice', BACKUP / 'tvservice.img'),
                              ('/lib', BACKUP / 'tvservice.img'),
                              ('/tclconfig', BACKUP / 'tclconfig.img'), ('/vendor', VENDOR)]:
            if guest.startswith(prefix + '/'):
                return image, ('/glibc/' if prefix == '/lib' else '/') + guest[len(prefix) + 1:]
        raise ValueError('unmapped guest path: ' + guest)

    def call(self, image, cmd):
        r = subprocess.run([self.debugfs, '-R', cmd, str(image)], capture_output=True, text=True, check=True)
        return r.stdout

    def get(self, guest):
        if guest in self.records:
            return OUT / 'root' / guest.lstrip('/')
        if guest in self.absent:
            return None
        image, source = self.map(guest)
        source = os.path.normpath(source)
        original_source = source
        for _ in range(16):
            info = self.call(image, f'stat {source}')
            if 'Type: regular' in info:
                break
            m = re.search(r'Fast link dest: "([^"]+)"', info)
            if not m:
                self.absent.add(guest)
                return None
            target = m[1]
            source = os.path.normpath(target if target.startswith('/') else str(pathlib.PurePosixPath(source).parent / target))
        else:
            raise ValueError('symlink loop: ' + guest)
        dst = OUT / 'root' / guest.lstrip('/')
        dst.parent.mkdir(parents=True, exist_ok=True)
        self.call(image, f'dump {source} {dst}')
        if not dst.is_file():
            raise RuntimeError('debugfs failed to extract ' + guest)
        dst.chmod(0o755 if '/bin/' in guest else 0o644)
        self.records[guest] = {'source_image': image.name, 'source_path': original_source,
                               'resolved_path': source, 'sha256': sha(dst), 'bytes': dst.stat().st_size}
        return dst


def module_order(modules, kernel_exports):
    # Stock modinfo still names utpa2k-{system,avdec,frontend}, although the
    # shipped/live module is utpa2k. Resolve actual imported/exported symbols.
    providers = {name: m['module_name'] for m in modules for name in m['exports']}
    loaded = set()
    errors = []
    for m in modules:
        for symbol in m['imports']:
            provider = providers.get(symbol)
            if provider and provider not in loaded and provider != m['module_name']:
                errors.append({'file': m['file'], 'symbol': symbol, 'provider_not_loaded': provider})
            elif not provider and symbol not in kernel_exports:
                errors.append({'file': m['file'], 'unresolved_symbol': symbol})
        loaded.add(m.get('module_name', ''))
    return errors


def main():
    expected = {line.split()[1].lstrip('*'): line.split()[0]
                for line in (BACKUP / 'SHA256SUMS').read_text().splitlines() if len(line.split()) == 2}
    sources = {}
    for name in ['boot.img', 'super.img', 'tvconfig.img', 'tvservice.img', 'tclconfig.img']:
        digest = sha(BACKUP / name)
        if digest != expected[name]:
            raise RuntimeError('backup hash mismatch: ' + name)
        sources[name] = digest
    if sha(VENDOR) != VENDOR_SHA:
        raise RuntimeError('derived V109 vendor image hash mismatch')
    # Rebuild only this dedicated generated directory, never originals.
    if OUT.exists():
        shutil.rmtree(OUT)
    OUT.mkdir(parents=True)
    files = Files()
    modules = []
    for name in MODULES:
        guest = f'/vendor/lib/modules/{name}.ko'
        p = files.get(guest)
        if p is None:
            raise RuntimeError('missing module: ' + guest)
        record = elf(p.read_bytes())
        record['file'] = guest
        if not record['modinfo'].get('vermagic', '').startswith('4.19.116+ '):
            raise RuntimeError('wrong module ABI: ' + guest)
        modules.append(record)
    symbols = ROOT / "assets/research/kexec/kallsyms.txt"
    kernel_exports = set()
    for line in symbols.read_text().splitlines():
        words = line.split()
        if len(words) == 3 and words[2].startswith('__ksymtab_'):
            kernel_exports.add(words[2][len('__ksymtab_'):])
    order_errors = module_order(modules, kernel_exports)
    if order_errors:
        raise RuntimeError('module dependency/order failure: ' + json.dumps(order_errors))
    # Preserve exact config bytes. Follow literal board/config references;
    # absent/dynamic references are recorded, never silently claimed complete.
    seeds = ['/vendor/tvconfig/config/' + x for x in
        ['sys.ini', 'mik_init.ini', 'MMAP_MI.h', 'OSD/OsdDefine.ini', 'OSD/OsdConfig.ini']]
    for guest in seeds:
        if files.get(guest) is None:
            raise RuntimeError('required platform config absent: ' + guest)
    queue = collections.deque(seeds)
    seen = set()
    config_edges = []
    pattern = re.compile(r'/(?:vendor/tvconfig|tclconfig)/[A-Za-z0-9_./+%{}-]+')
    while queue:
        guest = queue.popleft()
        if guest in seen:
            continue
        seen.add(guest)
        p = files.get(guest)
        if not p:
            continue
        if p.stat().st_size > 1024 * 1024:
            continue
        try:
            text = p.read_text()
        except (UnicodeError, ValueError):
            continue
        # Do not turn commented examples into presumed runtime dependencies.
        active = '\n'.join(line.split('#', 1)[0] for line in text.splitlines()
                           if not line.lstrip().startswith(('#', ';')))
        for dest in sorted(set(pattern.findall(active))):
            dest = os.path.normpath(dest)
            if dest.endswith('/') or '%' in dest or '{' in dest:
                continue
            config_edges.append({'from': guest, 'to': dest})
            queue.append(dest)
    # Analyze the native daemon closure separately: it is not a required app
    # dependency and must never be started automatically by the kit.
    queue = collections.deque(['/mnt/vendor/tvservice/bin/midaemon'])
    seen = set()
    elf_edges = []
    unresolved = []
    while queue:
        guest = queue.popleft()
        if guest in seen:
            continue
        seen.add(guest)
        p = files.get(guest)
        if p is None:
            unresolved.append({'from': guest, 'needed': 'file absent'})
            continue
        meta = elf(p.read_bytes())
        interpreter = meta['interpreter']
        if interpreter:
            queue.append(interpreter)
        for needed in meta['needed']:
            found = None
            for directory in SEARCH:
                candidate = directory + '/' + needed
                if files.get(candidate) is not None:
                    found = candidate
                    break
            elf_edges.append({'from': guest, 'needed': needed, 'resolved': found})
            if found:
                queue.append(found)
            else:
                unresolved.append({'from': guest, 'needed': needed})
    manifest = {'target': 'G06/V109 only', 'source_sha256': sources, 'vendor_sha256': VENDOR_SHA,
                'files': files.records, 'modules': modules, 'module_order_errors': order_errors,
                'kernel_exports_evidence_sha256': sha(symbols),
                'module_metadata_note': 'utpa2k split dependency names are stale; resolved by actual symbols',
                'configuration_edges': config_edges, 'configuration_unresolved': sorted(files.absent & seen_config(config_edges)),
                'configuration_note': 'Literal references may name optional/disabled alternatives or directories; not all are required',
                'native_runtime_edges': elf_edges, 'native_runtime_unresolved': unresolved,
                'native_runtime_files': sorted(seen), 'hardware_ready': False,
                'unverified': ['MI initialization structs/call sequencing and side effects',
                               'runtime dlopen and config/firmware dependencies beyond literal references',
                               'kernel devices/ioctls, shared memory, Android-service dependencies',
                               'panel/speaker behavior, boot transport and RAM placement']}
    (OUT / 'manifest.json').write_text(json.dumps(manifest, indent=2, sort_keys=True) + '\n')
    print(json.dumps({'output': str(OUT), 'files': len(files.records),
                      'bytes': sum(r['bytes'] for r in files.records.values()),
                      'modules': len(modules), 'configuration_unresolved': manifest['configuration_unresolved'],
                      'native_runtime_unresolved': unresolved, 'hardware_ready': False}, indent=2))


def seen_config(edges):
    return {e['to'] for e in edges}


if __name__ == '__main__':
    main()
