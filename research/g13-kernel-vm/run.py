#!/usr/bin/env python3
"""Run the existing Android 16 native checks against the new kernel output."""
import importlib.util
from pathlib import Path
import shutil
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
WORK = ROOT / 'assets/build/g13-kernel-vm'
OLD = ROOT / 'assets/build/g13-android16'
sys.path.insert(0, str(ROOT / 'research/g13-android16'))
spec = importlib.util.spec_from_file_location('android16_run', ROOT / 'research/g13-android16/run.py')
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)


def main():
    # Reuse the exact previously tested ramdisk; only the kernel is varied.
    assert runner.sha(OLD / 'native-initramfs.cpio.gz') == 'e84b6a12448f9fc88eac50d050fe81c5fd22039028dd828f3e44ebe1a0d0f6f5'
    WORK.mkdir(parents=True, exist_ok=True)
    for name in ('native-initramfs.cpio.gz', 'native-manifest.json'):
        shutil.copy2(OLD / name, WORK / name)
    runner.WORK = WORK
    return runner.main()


if __name__ == '__main__':
    sys.exit(main())
