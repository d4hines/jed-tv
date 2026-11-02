#!/usr/bin/env python3
"""Check StringExtra against upstream Lean and ARM32 Zr using the same oracle."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import struct
import subprocess

ROOT = Path(__file__).resolve().parents[4]
OUT = ROOT / 'assets/build/string-extra'
MARKER = 'PASS: StringExtra'


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
    manifest = ROOT / 'test/lean/zr/string-extra/Cargo.toml'
    print(run(['lake', '-d', 'src/pid1/lean', 'env', 'lean', '--run',
               ROOT / 'test/lean/zr/string-extra/Signatures.lean'], env, 'signatures'), end='')
    host = run(['cargo', '+1.92.0', 'run', '--offline', '--release', '--manifest-path', manifest],
               env, 'host')
    assert MARKER in host, host
    # Build the new module explicitly as well as the public runtime: this also
    # makes the check usable before a consumer adds it to ZrLean's imports.
    run(['lake', '-d', 'src/lean/zr', 'build', 'ZrLean.Low.StringExtra'], env, 'module')
    runtime = OUT / 'runtime'
    run(['lake', '-d', 'tools/lean/zr', 'exe', 'zr-build', 'rt', runtime], env, 'runtime')
    objects = sorted((runtime / 'obj').glob('*.o'))
    assert any(p.name == 'ZrLean_Low_StringExtra.o' for p in objects)
    run(['zig', 'ar', 'rcs', runtime / 'libzr_native.a', *objects], env, 'archive')
    archive = shutil.which('llvm-ar')
    if archive is None and Path('/opt/homebrew/opt/llvm/bin/llvm-ar').is_file():
        archive = '/opt/homebrew/opt/llvm/bin/llvm-ar'
    if archive is None:
        raise RuntimeError('llvm-ar is required by the ARM C build')
    env.update(ZR_RUNTIME_DIR=str(runtime), AR_armv7_unknown_linux_musleabihf=archive,
               CC_armv7_unknown_linux_musleabihf=str(ROOT / 'tools/toolchain/zcc-armv7-unknown-linux-musleabihf'),
               CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=str(ROOT / 'tools/pid1/linker-armv7'))
    run(['cargo', '+1.92.0', 'build', '--offline', '--release', '--target',
         'armv7-unknown-linux-musleabihf', '--manifest-path', manifest], env, 'arm-build')
    binary = OUT / 'target/armv7-unknown-linux-musleabihf/release/zr-string-extra-check'
    elf = binary.read_bytes()
    assert elf[:6] == b'\x7fELF\x01\x01' and struct.unpack_from('<H', elf, 18)[0] == 40
    phoff = struct.unpack_from('<I', elf, 28)[0]
    phsize, phnum = struct.unpack_from('<HH', elf, 42)
    assert all(struct.unpack_from('<I', elf, phoff + n * phsize)[0] not in (2, 3)
               for n in range(phnum)), 'Unexpected dynamic runtime dependency'
    initramfs = OUT / 'check.cpio'
    run(['python3', 'tools/mkinitramfs.py', binary, initramfs], env, 'initramfs')
    console = run(['qemu-system-aarch64', '-M', 'virt,highmem=off,gic-version=2',
                   '-cpu', 'cortex-a55,aarch64=off', '-m', '256M', '-nographic', '-no-reboot',
                   '-kernel', args.kernel, '-initrd', initramfs, '-append',
                   'console=ttyAMA0 rdinit=/init panic=-1'], env, 'arm-console', timeout=40)
    assert MARKER in console and 'panicked at' not in console, console
    evidence = {'result': 'PASS', 'arm_binary_sha256': hashlib.sha256(elf).hexdigest(),
                'unicode_scalars': 1112064, 'integer_range': 'signed128',
                'ownership': 'borrowed inputs reused, owned list shared and consumed'}
    (OUT / 'result.json').write_text(json.dumps(evidence, indent=2) + '\n')
    print(host[host.index(MARKER):].strip())
    print(json.dumps(evidence, indent=2))


if __name__ == '__main__':
    main()
