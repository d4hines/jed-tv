#!/usr/bin/env python3
"""Check ColdPlan and its real worker/main adapter on host and owned ARM runtime."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'assets/build/cold-plan-check'


def run(command, env, name):
    result = subprocess.run(list(map(str, command)), cwd=ROOT, env=env,
                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True)
    (OUT / f'{name}.log').write_text(result.stdout)
    if result.returncode:
        raise RuntimeError(f'{name} failed:\n{result.stdout[-10000:]}')
    return result.stdout


def entry(name, data=b'', mode=0o100755, major=0, minor=0):
    fields = [1, mode, 0, 0, 1, 0, len(data), 0, 0, major, minor, len(name) + 1, 0]
    header = b'070701' + b''.join(f'{v:08x}'.encode() for v in fields) + name.encode() + b'\0'
    header += b'\0' * (-len(header) % 4)
    return header + data + b'\0' * (-len(data) % 4)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--kernel', type=Path, required=True)
    parser.add_argument('--runtime', type=Path, help='Previously built Zr runtime cache (gen/ and libzr_native.a)')
    args = parser.parse_args()
    OUT.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, CARGO_TARGET_DIR=str(OUT),
               ZIG_GLOBAL_CACHE_DIR=str(OUT / 'zig-global'), ZIG_LOCAL_CACHE_DIR=str(OUT / 'zig-local'))
    for name in ('LEAN_PATH', 'LEAN_SRC_PATH', 'LD_LIBRARY_PATH', 'DYLD_LIBRARY_PATH', 'RUSTFLAGS'):
        env.pop(name, None)
    host_env = dict(env)
    if os.uname().sysname == 'Darwin':
        host_env['RUSTFLAGS'] = '-C linker=/usr/bin/cc'
    manifest = ROOT / 'test/cold-plan/Cargo.toml'
    host = run(['cargo', '+1.92.0', 'run', '--offline', '--release', '--manifest-path', manifest], host_env, 'host')
    expected = re.search(r'TRANSCRIPT ([0-9a-f]+)', host)
    assert expected and 'PASS:' in host, host
    runtime = args.runtime.resolve() if args.runtime else OUT / 'runtime'
    if not args.runtime:
        run(['lake', '-d', 'tools/lean/zr', 'exe', 'zr-build', 'rt', runtime], env, 'runtime')
        run(['zig', 'ar', 'rcs', runtime / 'libzr_native.a', *sorted((runtime / 'obj').glob('*.o'))], env, 'archive')
    env.update(COLD_ZR_RUNTIME_DIR=str(runtime),
               CC_armv7_unknown_linux_musleabihf=str(ROOT / 'tools/toolchain/zcc-armv7-unknown-linux-musleabihf'),
               AR_armv7_unknown_linux_musleabihf=shutil.which('llvm-ar') or '/opt/homebrew/opt/llvm/bin/llvm-ar',
               CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER=str(ROOT / 'tools/pid1/linker-armv7'))
    run(['cargo', '+1.92.0', 'build', '--offline', '--release', '--target', 'armv7-unknown-linux-musleabihf', '--manifest-path', manifest], env, 'arm-build')
    binary = OUT / 'armv7-unknown-linux-musleabihf/release/cold-plan-check'
    image = OUT / 'init.cpio'
    image.write_bytes(entry('dev', mode=0o40755) + entry('dev/console', mode=0o20600, major=5, minor=1)
                      + entry('init', binary.read_bytes()) + entry('TRAILER!!!', mode=0))
    command = ['qemu-system-aarch64', '-M', 'virt,highmem=off,gic-version=2', '-cpu', 'cortex-a55,aarch64=off',
               '-m', '256M', '-nographic', '-monitor', 'none', '-net', 'none', '-no-reboot',
               '-kernel', str(args.kernel.resolve()), '-initrd', str(image), '-append', 'console=ttyAMA0 panic=-1']
    process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
    try:
        output, _ = process.communicate(timeout=60)
    except subprocess.TimeoutExpired:
        process.kill()
        output, _ = process.communicate()
    arm = output.decode(errors='replace')
    (OUT / 'arm-console.log').write_text(arm)
    actual = re.search(r'TRANSCRIPT ([0-9a-f]+)', arm)
    assert actual and 'PASS:' in arm, arm[-10000:]
    assert actual[1] == expected[1], f'host {expected[1]}, ARM {actual[1]}'
    record = {'result': 'PASS', 'transcript': actual[1], 'arm_sha256': hashlib.sha256(binary.read_bytes()).hexdigest(),
              'hardware_tested': False, 'adapter': 'real worker/main channel and shared Lean owner guard'}
    (OUT / 'result.json').write_text(json.dumps(record, indent=2) + '\n')
    print(json.dumps(record, indent=2))


if __name__ == '__main__':
    main()
