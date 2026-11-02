#!/usr/bin/env python3
"""Inventory saved driver dependencies without loading modules or using a TV."""
import argparse
from collections import defaultdict
import hashlib
import json
from pathlib import Path
import subprocess


def closure(name, graph):
    seen, queue = set(), [name]
    for current in queue:
        if current in seen:
            continue
        seen.add(current)
        queue.extend(sorted(graph.get(current, set()) - seen))
    return sorted(seen)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('audit', type=Path)
    parser.add_argument('firmware', type=Path)
    parser.add_argument('output', type=Path)
    parser.add_argument('--stock-layouts', type=Path)
    parser.add_argument('--candidate-layouts', type=Path)
    args = parser.parse_args()
    audit = json.loads(args.audit.read_text())
    modules = {m['name']: m for m in audit['modules']}
    providers = defaultdict(set)
    by_module_name = {}
    for name, module in modules.items():
        by_module_name[name.removesuffix('.ko').replace('-', '_')] = name
        for symbol in module['metadata']['exports']:
            providers[symbol].add(name)
    graph = {}
    for name, module in modules.items():
        dependencies = {provider for symbol in module['metadata']['imports']
                        for provider in providers[symbol]}
        for dep in module['metadata']['modinfo'].get('depends', '').split(','):
            if dep:
                dependencies.add(by_module_name[dep.replace('-', '_')])
        graph[name] = dependencies - {name}
    entries = []
    for name, module in sorted(modules.items()):
        path = args.firmware / module['path']
        actual = hashlib.sha256(path.read_bytes()).hexdigest()
        if actual != module['sha256']:
            raise ValueError(f'Audit input changed: {path}')
        entries.append({
            'name': name, 'sha256': actual,
            'rebuilt_same_name': module['rebuilt_same_name'],
            'firmware_module_dependencies': sorted(graph[name]),
            'transitive_stock_closure': closure(name, graph),
            'candidate_crc_matches': len(module['matched_crc']),
            'candidate_crc_mismatches': len(module['mismatched_crc']),
            'missing_symbols': module['missing_symbols'],
            'imports_without_crc_count': len(module['imports_without_crc'])})
    roots = ('mstar_fbdev_mi.ko', 'mik.ko', 'mali_kbase_mt5867.ko')
    display_audio = sorted({n for root in roots for n in closure(root, graph)})
    report = {
        'firmware': audit['firmware'], 'kernel_release': audit['kernel_release'],
        'modules': entries, 'display_audio_roots': list(roots),
        'display_audio_stock_closure': display_audio,
        'display_audio_rebuilt_same_name': [n for n in display_audio
                                          if modules[n]['rebuilt_same_name']],
        'display_audio_missing_symbols': sorted({s for n in display_audio
            for s in modules[n]['missing_symbols']}),
        'scope': ('Static import/export and modinfo closure of the saved firmware; '
                  'not a minimal boot module list or a mandate to reuse every stock binary. '
                  'Does not capture runtime module requests or hardware dependencies. '
                  'A rebuilt filename is not proof of equivalent implementation.')}
    if args.stock_layouts and args.candidate_layouts:
        stock = json.loads(args.stock_layouts.read_text())['layouts']
        candidate = json.loads(args.candidate_layouts.read_text())['layouts']
        def member(layouts, name):
            variants = layouts['structure_type:task_struct']
            assert len(variants) == 1
            return next(m for m in variants[0]['members'] if m['name'] == name)
        stock_usage, candidate_usage = member(stock, 'usage'), member(candidate, 'usage')
        mali = modules['mali_kbase_mt5867.ko']
        path = args.firmware / mali['path']
        # Do not use --disassemble-symbols: LLVM stops at local .Ltmp labels
        # inside this function and would omit the atomic instructions.
        text = subprocess.check_output(['llvm-objdump', '-dr', str(path)], text=True)
        # The reference binary gets task_struct from pid_task, then increments
        # its reference count. Retain the code so this interpretation is auditable.
        args.output.parent.mkdir(parents=True, exist_ok=True)
        disassembly = args.output.with_suffix('.mali-disassembly.txt')
        lines = text.splitlines()
        start = next(i for i, line in enumerate(lines) if '<kbase_context_common_init>:' in line)
        index = next(i for i in range(start, len(lines)) if 'R_ARM_THM_CALL\tpid_task' in lines[i])
        excerpt = lines[index:index + 18]
        disassembly.write_text('\n'.join(lines[start:index + 18]) + '\n')
        expected = f'add.w\tr0, r4, #0x{stock_usage["offset"]:x}'
        assert any(expected in line for line in excerpt), 'Unexpected Mali task offset'
        assert any('mov\tr4, r0' in line for line in excerpt), 'Unexpected task pointer register'
        assert any('ldrex\tr1, [r0]' in line for line in excerpt), 'Expected atomic load missing'
        assert any('strex\tr3, r2, [r0]' in line for line in excerpt), 'Expected atomic store missing'
        report['mali_task_reference_evidence'] = {
            'module_sha256': mali['sha256'],
            'function': 'kbase_context_common_init',
            'stock_usage_offset': stock_usage['offset'],
            'candidate_usage_offset': candidate_usage['offset'],
            'offsets_differ': stock_usage['offset'] != candidate_usage['offset'],
            'disassembly': str(disassembly), 'excerpt': excerpt,
            'interpretation': ('The stock code increments the task reference at the stock offset. '
                               'The candidate task has that reference at a different offset. '
                               'This is direct evidence against force-loading this stock driver.')}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({'modules': len(entries), 'display_audio_closure': display_audio,
                      'missing_symbols': report['display_audio_missing_symbols']}))


if __name__ == '__main__':
    main()
