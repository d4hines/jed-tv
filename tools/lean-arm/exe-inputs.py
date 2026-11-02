#!/usr/bin/env python3
"""Select one Lake executable's generated C, excluding cached test programs.

Prints absolute paths using each package's configured buildDir, including
path dependencies (lake-manifest.json), e.g. a test package's engine."""
import argparse
import json
import sys
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[1] / "lean"))
from build_paths import build_dir
import re

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('package', type=Path)
parser.add_argument('executable')
parser.add_argument('--root', action='store_true')
args = parser.parse_args()
roots = {}
for match in re.finditer(r'lean_exe\s+(?:«([^»]+)»|(\S+))\s+where\s+root\s*:=\s*`([\w.]+)',
                         (args.package / 'lakefile.lean').read_text()):
    roots[match[1] or match[2]] = match[3]
root = roots.get(args.executable)
if root is None:
    raise SystemExit(f'No lean_exe {args.executable} root in {args.package}/lakefile.lean')
if args.root:
    print(root)
else:
    ir = build_dir(args.package) / 'ir'
    irs = [ir.resolve()]
    manifest = args.package / 'lake-manifest.json'
    if manifest.is_file():
        for dep in json.loads(manifest.read_text()).get('packages', []):
            if dep.get('type') == 'path':
                irs.append((build_dir(args.package / dep['dir']) / 'ir').resolve())
    setup = json.loads((ir / (root.replace('.', '/') + '.setup.json')).read_text())
    # Lake records the transitive import closure. Runtime modules are supplied
    # by libInit; generated package modules have a C artifact beside setup.
    modules = sorted(set(setup['importArts']) | {root})
    for module in modules:
        source = module.replace('.', '/') + '.c'
        for d in irs:
            if (d / source).is_file():
                print(d / source)
                break
