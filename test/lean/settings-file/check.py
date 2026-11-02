#!/usr/bin/env python3
"""Check the production settings adapter and Lean codec on host and ARM32/Zr."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import struct
import subprocess

ROOT = Path(__file__).resolve().parents[3]
OUT = ROOT / 'assets/build/settings-file-check'


def audit_storage_scalar():
    """Worker-facing exports must not reach boxed objects or runtime once cells."""
    code = '\n'.join((ROOT / f'assets/build/lake/tv/ir/TV/{module}.c').read_text()
                     for module in ['Storage', 'StorageText', 'Hardware/TeeBoundary'])
    bodies = {}
    for match in re.finditer(r'^(?:LEAN_EXPORT |static )[^\n]+? (\w+)\([^\n]*\)\{', code, re.M):
        depth, end = 1, match.end()
        while depth:
            depth += (code[end] == '{') - (code[end] == '}')
            end += 1
        bodies[match[1]] = code[match.start():end]
    exports = [name for name in bodies if name.startswith('tv_storage_')]
    assert len(exports) == 12, exports
    pending = exports + ['tv_tee_device_step', 'tv_tee_device_major', 'tv_tee_device_minor']
    visited = set()
    while pending:
        name = pending.pop()
        if name in visited:
            continue
        visited.add(name)
        body = bodies[name]
        assert 'lean_object*' not in body and '_closed__' not in body, name
        calls = set(re.findall(r'\b(\w+)\(', body))
        assert not any(call.startswith(('lean_alloc', 'lean_box', 'lean_inc', 'lean_dec',
                                        'lean_apply')) or call.endswith('_once') for call in calls), name
        pending.extend(calls & bodies.keys() - visited)
    print(f'PASS: twelve storage exports and three shared TEE parser exports reach {len(visited)} allocation-free functions')


def run(args, env, name, timeout=300):
    result = subprocess.run(list(map(str, args)), cwd=ROOT, env=env,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                            text=True, timeout=timeout)
    (OUT / f'{name}.log').write_text(result.stdout)
    if result.returncode:
        raise RuntimeError(f'{name} failed ({result.returncode}):\n{result.stdout[-10000:]}')
    return result.stdout


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--kernel', type=Path, default=ROOT / 'assets/build/kernel/arm/zImage')
    args = parser.parse_args()
    if not args.kernel.is_file():
        parser.error('Build the ARM32 QEMU kernel or supply --kernel')
    OUT.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, CARGO_TARGET_DIR=str(OUT / 'target'),
               ZIG_GLOBAL_CACHE_DIR=str(OUT / 'zig-global'),
               ZIG_LOCAL_CACHE_DIR=str(OUT / 'zig-local'))
    for key in ('LEAN_PATH', 'LEAN_SRC_PATH', 'LD_LIBRARY_PATH', 'DYLD_LIBRARY_PATH'):
        env.pop(key, None)
    if os.uname().sysname == 'Darwin':
        env['CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER'] = '/usr/bin/cc'
    manifest = ROOT / 'test/lean/settings-file/Cargo.toml'
    signatures = run(['lake', '-d', 'src/pid1/lean', 'env', 'lean', '--run',
                      ROOT / 'test/lean/settings-file/Signatures.lean'], env, 'signatures')
    print(signatures, end='')
    host = run(['cargo', '+1.92.0', 'run', '--offline', '--release', '--manifest-path', manifest],
               env, 'host')
    assert 'PASS: Lean settings' in host and 'PASS: String/List' in host, host
    audit_storage_scalar()
    runtime = OUT / 'runtime'
    print(run(['lake', '-d', 'tools/lean/zr', 'exe', 'zr-build', 'rt', runtime / 'zr'],
              env, 'runtime'), end='')
    objects = sorted((runtime / 'zr/obj').glob('*.o'))
    assert objects, 'Runtime produced no objects'
    run(['zig', 'ar', 'rcs', runtime / 'libzr_native.a', *objects], env, 'archive')
    env.update(ZR_RUNTIME_DIR=str(runtime),
               CC_armv7_unknown_linux_musleabihf=str(ROOT / 'tools/toolchain/zcc-armv7-unknown-linux-musleabihf'),
               CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=str(ROOT / 'tools/pid1/linker-armv7'))
    # Use LLVM ar, or the same Zig wrapper as production when available.
    import shutil
    archive = shutil.which('llvm-ar')
    if archive is None and Path('/opt/homebrew/opt/llvm/bin/llvm-ar').is_file():
        archive = '/opt/homebrew/opt/llvm/bin/llvm-ar'
    if archive is None:
        raise RuntimeError('llvm-ar is required by the ARM C build')
    env['AR_armv7_unknown_linux_musleabihf'] = archive
    run(['cargo', '+1.92.0', 'build', '--offline', '--release', '--target',
         'armv7-unknown-linux-musleabihf', '--manifest-path', manifest], env, 'arm-build')
    binary = OUT / 'target/armv7-unknown-linux-musleabihf/release/settings-file-test'
    elf = binary.read_bytes()
    assert elf[:6] == b'\x7fELF\x01\x01' and struct.unpack_from('<H', elf, 18)[0] == 40
    phoff = struct.unpack_from('<I', elf, 28)[0]
    phsize, phnum = struct.unpack_from('<HH', elf, 42)
    assert all(struct.unpack_from('<I', elf, phoff + n * phsize)[0] not in (2, 3)
               for n in range(phnum)), 'Unexpected dynamic runtime dependency'
    initramfs = OUT / 'settings.cpio'
    run(['python3', 'tools/mkinitramfs.py', binary, initramfs], env, 'initramfs')
    console = run(['qemu-system-aarch64', '-M', 'virt,highmem=off,gic-version=2',
                   '-cpu', 'cortex-a55,aarch64=off', '-m', '256M', '-nographic', '-no-reboot',
                   '-kernel', args.kernel, '-initrd', initramfs, '-append',
                   'console=ttyAMA0 rdinit=/init panic=-1'], env, 'arm-console', timeout=40)
    markers = ['PASS: String/List', 'PASS: Lean settings', 'PASS: real CoreStore', 'PASS: Lean Wi-Fi',
               'PASS: scalar storage acceptance', 'PASS: production commit effect sequence',
               'PASS: raw sysfs decimal/device parsers']
    assert all(marker in console for marker in markers), console
    assert 'panicked at' not in console, console
    assert 'Attempted to kill init! exitcode=0x00000000' in console, console
    unit_output = run(['cargo', '+1.92.0', 'test', '--offline', '--release', '--target',
                      'armv7-unknown-linux-musleabihf', '--manifest-path', manifest,
                      '--no-run', '--message-format=json'], env, 'unit-build')
    binaries = [row['executable'] for line in unit_output.splitlines() if line.startswith('{')
                for row in [json.loads(line)] if row.get('reason') == 'compiler-artifact'
                and row.get('profile', {}).get('test') and row.get('executable')]
    assert len(binaries) == 1, binaries
    run(['python3', 'tools/mkinitramfs.py', binaries[0], initramfs], env, 'unit-initramfs')
    units = run(['qemu-system-aarch64', '-M', 'virt,highmem=off,gic-version=2',
                 '-cpu', 'cortex-a55,aarch64=off', '-m', '256M', '-nographic', '-no-reboot',
                 '-kernel', args.kernel, '-initrd', initramfs, '-append',
                 'console=ttyAMA0 rdinit=/init panic=-1'], env, 'unit-console', timeout=40)
    assert 'raw_scan_worker_result_runs_policy_on_the_supervisor_thread ... ok' in units, units
    assert '0 failed' in units and 'Attempted to kill init! exitcode=0x00000000' in units, units
    evidence = dict(executable_sha256=hashlib.sha256(elf).hexdigest(),
                    kernel_sha256=hashlib.sha256(args.kernel.read_bytes()).hexdigest(),
                    unicode_scalars=1_112_064, all_pins=10_000, all_masks=256,
                    fuzz_strings=10_000, wifi_hostile_scans=2000, wifi_worker='passed',
                    storage_metadata='previous predicates matched',
                    storage_commit='all five failures, cleanup failures and concurrent scalar workers passed',
                    sysfs_records='raw ASCII decimal and device pair oracle, bounds and worker checks passed',
                    real_adapter='passed', runtime='Zr',
                    scope='ARM32 QEMU and host; no TV access')
    (OUT / 'result.json').write_text(json.dumps(evidence, indent=2) + '\n')
    print('PASS: host oracle, ARM32/Zr codecs, Unicode ownership, CoreStore and Wi-Fi worker; ' + str(OUT))


if __name__ == '__main__':
    main()
