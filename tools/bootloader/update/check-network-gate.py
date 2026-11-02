#!/usr/bin/env python3
"""Exercise compressed gate/OS/fallback integration; network window checked separately."""
import sys
sys.dont_write_bytecode=True
import importlib.util
import json
from fixtures_tv import ROOT
sys.path.append(str(ROOT/'tools/bootloader'))
spec=importlib.util.spec_from_file_location('gatecheck',ROOT/'tools/bootloader/check-boot-gate.py')
gate=importlib.util.module_from_spec(spec);spec.loader.exec_module(gate)
gate.OUT=ROOT/'assets/build/leanboot-network'
cases=[gate.run(network=True,app_choice=1),gate.run(2,network=True,app_choice=2),gate.run(normal=False,network=True),
       gate.run(corrupt=True,network=True),gate.run(bad_plan=True,network=True),gate.run(io_error=True,network=True)]
(gate.OUT/'gate-checks.json').write_text(json.dumps(cases,indent=2)+'\n')
print('PASS: compressed network gate, ordinary OS, third-start fallback, recovery, corrupt metadata, preparation and journal failure')
