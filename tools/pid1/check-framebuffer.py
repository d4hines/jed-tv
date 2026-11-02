#!/usr/bin/env python3
"""Run the real framebuffer module's host tests; no device access."""
import os
from pathlib import Path
import subprocess
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'assets/build/framebuffer-host-tests'
OUT.mkdir(parents=True, exist_ok=True)
lock = tomllib.loads((ROOT / 'src/Cargo.lock').read_text())
version = next(p['version'] for p in lock['package'] if p['name'] == 'libc')
source = ROOT / 'test/fixtures/framebuffer_host.rs'
(OUT / 'Cargo.toml').write_text(f'''[package]
name = "framebuffer-host-tests"
version = "0.0.0"
edition = "2021"
[workspace]
[lib]
path = "{source.as_posix()}"
[dependencies]
libc = "={version}"
''')
env = os.environ.copy()
if sys.platform == 'darwin':
    env['CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER'] = '/usr/bin/cc'
subprocess.run(['cargo', '+1.92.0', 'test', '--offline', '--manifest-path',
                str(OUT / 'Cargo.toml'), '--target-dir', str(OUT / 'target')],
               cwd=ROOT, env=env, check=True)
