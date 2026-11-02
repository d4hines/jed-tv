#!/usr/bin/env python3
"""Check bootloader proofs and their assumptions; no ARM build or device access."""
import os
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'src/bootloader'
OUTPUT = ROOT / 'assets/build/leanboot-proofs'


def main():
    OUTPUT.mkdir(parents=True, exist_ok=True)
    environment = dict(os.environ, LEAN_PATH=str(OUTPUT))
    done = set()

    def compile_module(name):
        if name in done:
            return
        source = SOURCE / (name.replace('.', '/') + '.lean')
        if not source.is_file():
            return  # Init/Std belong to the pinned Lean toolchain.
        for dependency in re.findall(r'^import\s+(\S+)', source.read_text(), re.M):
            compile_module(dependency)
        target = OUTPUT / source.relative_to(SOURCE).with_suffix('.olean')
        target.parent.mkdir(parents=True, exist_ok=True)
        subprocess.run(['lean', '-o', str(target), str(source.relative_to(SOURCE))],
                       cwd=SOURCE, env=environment, check=True)
        done.add(name)

    compile_module('Proofs')
    compile_module('Update.Proofs')
    compile_module('NetworkBoot.Proofs')
    compile_module('Flow.Proofs')
    compile_module('Update.Receiver.Proofs')
    compile_module('Update.ParserProofs')
    compile_module('Update.FlashProofs')
    compile_module('Update.StorageModel')
    subprocess.run(['lean', 'ProofAudit.lean'], cwd=ROOT / 'tools/bootloader',
                   env=environment, check=True)
    print(f'Bootloader proofs elaborated from {len(done)} source modules.')


if __name__ == '__main__':
    main()
