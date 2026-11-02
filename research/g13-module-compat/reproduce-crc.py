#!/usr/bin/env python3
"""Reproduce module symbol CRCs using extracted TCL headers and genksyms."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shlex
import struct
import subprocess

from elftools.elf.elffile import ELFFile

ROOT = Path(__file__).resolve().parents[2]


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def graph(path):
    records = dict(line.split(' ', 1) for line in path.read_text().splitlines())
    result, queue = {}, ['module_layout']
    for key in queue:
        if key in result:
            continue
        text = records.get(key, 'MISSING')
        result[key] = ' '.join(text.split())
        queue.extend(x for x in re.findall(r'\b[stueE]#[A-Za-z_][A-Za-z_0-9]*', text)
                     if x not in result)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('headers', type=Path)
    parser.add_argument('output', type=Path)
    parser.add_argument('--source', type=Path, default=ROOT / 'assets/build/g13-kernel-vm/kernel-source')
    parser.add_argument('--genksyms', type=Path, required=True)
    parser.add_argument('--candidate-symtypes', type=Path)
    parser.add_argument('--expected-crc', default='a03a05bf')
    args = parser.parse_args()
    headers, source, out = args.headers.resolve(), args.source.resolve(), args.output.resolve()
    out.mkdir(parents=True, exist_ok=True)
    common = ['clang', '--target=arm-linux-gnueabi', '-nostdinc', '-D__KERNEL__',
              '-D__LINUX_ARM_ARCH__=7', '-DKBUILD_BASENAME="module"', '-DKBUILD_MODNAME="module"',
              '-mthumb', '-march=armv7-a', '-mfloat-abi=soft']
    for include in ('arch/arm/include', 'arch/arm/include/generated', 'include',
                    'arch/arm/include/uapi', 'arch/arm/include/generated/uapi',
                    'include/uapi', 'include/generated/uapi'):
        common.append('-I' + str(headers / include))
    # kheaders omits the vendor tree. These two existing MT5867 include roots
    # supply its machine headers; the dependency manifest records every use.
    for include in ('mstar2/hal/mt5867/cpu/arm/include', 'mstar2/drv/cpu/arm/include'):
        common.append('-I' + str(source / include))
    for include in ('linux/compiler-version.h', 'linux/kconfig.h', 'linux/compiler_types.h'):
        common += ['-include', str(headers / 'include' / include)]
    deps = out / 'module.d'
    cpp = common + ['-E', '-D__GENKSYMS__', '-MD', '-MF', str(deps), '-MT', 'module',
                    str(source / 'kernel/module.c')]
    result = subprocess.run(cpp, capture_output=True)
    (out / 'preprocess.stderr').write_bytes(result.stderr)
    result.check_returncode()
    (out / 'module.i').write_bytes(result.stdout)
    symtypes = out / 'module.symtypes'
    command = [str(args.genksyms.resolve()), '-T', str(symtypes), '-r', '/dev/null']
    symbols = subprocess.run(command, input=result.stdout, capture_output=True)
    (out / 'genksyms.stderr').write_bytes(symbols.stderr)
    symbols.check_returncode()
    (out / 'module.crcs').write_bytes(symbols.stdout)
    crcs = {name: crc.lower().zfill(8) for name, crc in re.findall(
        r'__crc_(\w+) = 0x([0-9a-fA-F]+);', symbols.stdout.decode())}
    observed = crcs.get('module_layout')
    # No CRC is overridden: this asserts what the unmodified tools produced.
    if observed != args.expected_crc.lower():
        raise ValueError(f'Expected {args.expected_crc}, generated {observed}')
    includes = sorted(set(shlex.split(deps.read_text().replace('\\\n', ' '))[1:]))
    dependencies = {str(Path(p).relative_to(ROOT)) if Path(p).is_relative_to(ROOT) else p:
                    sha(Path(p)) for p in includes}
    report = {'module_layout_crc': observed, 'expected_crc': args.expected_crc,
              'matches': True, 'preprocessor_command': cpp, 'genksyms_command': command,
              'genksyms_sha256': sha(args.genksyms), 'symtypes_sha256': sha(symtypes),
              'compiler': subprocess.check_output(['clang', '--version'], text=True),
              'dependencies': dependencies, 'export_crcs': crcs,
              'scope': 'Exact CRC reproduction with stock headers; no ABI checks bypassed, no module loaded.'}
    probe = Path(__file__).with_name('layout-probe.c').resolve()
    obj = out / 'layout-probe.o'
    compile_command = common + ['-O2', '-c', str(probe), '-o', str(obj)]
    compilation = subprocess.run(compile_command, capture_output=True)
    (out / 'layout-probe.stderr').write_bytes(compilation.stderr)
    compilation.check_returncode()
    layout = {}
    with obj.open('rb') as stream:
        elf = ELFFile(stream)
        assert elf.elfclass == 32 and elf.little_endian
        for symbol in elf.get_section_by_name('.symtab').iter_symbols():
            if symbol.name.startswith('g13_'):
                data = elf.get_section(symbol['st_shndx']).data()
                assert symbol['st_size'] == 4
                layout[symbol.name] = struct.unpack_from('<I', data, symbol['st_value'])[0]
    report.update(layout_constants=layout, layout_probe_command=compile_command,
                  layout_probe_source_sha256=sha(probe), layout_probe_object_sha256=sha(obj))
    if args.candidate_symtypes:
        stock, candidate = graph(symtypes), graph(args.candidate_symtypes)
        report['type_graph_comparison'] = {
            'stock_reachable': len(stock) - 1, 'candidate_reachable': len(candidate) - 1,
            'changed': {key: {'stock': stock[key], 'candidate': candidate[key]}
                        for key in sorted(stock.keys() & candidate.keys()) if stock[key] != candidate[key]},
            'stock_only': sorted(stock.keys() - candidate.keys()),
            'candidate_only': sorted(candidate.keys() - stock.keys())}
    (out / 'result.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({'module_layout_crc': observed, 'dependencies': len(dependencies),
                      'export_crcs': len(crcs), 'matches': True}))


if __name__ == '__main__':
    main()
