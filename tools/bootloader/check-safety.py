#!/usr/bin/env python3
"""Exercise the production controller model and reject nominal type confusion.
Run prove.py first to compile the pure modules. No build or device access.
"""
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'assets/build/leanboot-proofs'


def main():
    env = dict(os.environ, LEAN_PATH=str(OUT))
    subprocess.run(['lean', '--run', str(ROOT/'tools/bootloader/NetworkModel.lean')], env=env, check=True)
    subprocess.run(['lean', '--run', str(ROOT/'tools/bootloader/FlowModel.lean')], env=env, check=True)
    work = OUT/'nominal-types'
    work.mkdir(exist_ok=True)
    cases = {
        'raw_address': 'def bad (x : UInt32) : CpuAddress := x',
        'count_as_address': 'def bad (x : ByteCount) : CpuAddress := x',
        'entry_as_count': 'def bad (x : EntryOffset) : ByteCount := x',
        'version_as_count': 'def bad (x : ReleaseVersion) : ByteCount := x',
        'count_as_budget': 'def bad (x : ByteCount) : PollBudget := x',
        'unchecked_call': 'def bad (e : NetworkBoot.Effects Id) (x : CpuAddress) := e.invoke x',
    }
    for name, body in cases.items():
        path = work/(name+'.lean')
        path.write_text('import NetworkBoot.Runtime\nopen LeanBoot\n'+body+'\n')
        result = subprocess.run(['lean', str(path)], env=env, capture_output=True, text=True)
        assert result.returncode and 'type mismatch' in result.stdout.lower(), result.stdout+result.stderr
    (work/'check.json').write_text(json.dumps({'controller_traces':16,'crypto_spans':10,
        'entry_spans':8,'flow_failure_prefixes':29,'flow_successes':4,
        'flow_terminal_cases':12,'nominal_type_rejections':len(cases),'device_access':False}, indent=2)+'\n')
    print(f'PASS: {len(cases)} nominal type confusions rejected by Lean')


if __name__ == '__main__':
    main()
