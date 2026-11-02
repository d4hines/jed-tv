#!/usr/bin/env python3
"""Compare saved DWARF layouts; trace differences through a genksyms graph."""
import argparse
from collections import Counter, deque
import hashlib
import json
from pathlib import Path
import re


def canonical(record, little_endian, types=True):
    members = []
    for field in record['members']:
        entry = {'name': field['name']}
        if 'bit_size' in field:
            start = field.get('data_bit_offset', field.get('bit_start'))
            if start is None:
                within = field['bit_offset']
                if little_endian:
                    within = 8 * field['bytes'] - within - field['bit_size']
                start = 8 * field['offset'] + within
            entry.update(bit_start=start, bit_size=field['bit_size'])
        else:
            entry.update(offset=field['offset'], bytes=field['bytes'])
        if types:
            entry['type'] = field['type']
        if 'anonymous' in field:
            entry['anonymous'] = canonical(field['anonymous'], little_endian, types)
        members.append(entry)
    return {'bytes': record['bytes'], 'members': members}


def flat(record, prefix=''):
    result = {}
    for i, field in enumerate(record['members']):
        path = prefix + (field['name'] or f'<anonymous:{i}>')
        result[path] = {k: v for k, v in field.items() if k not in ('name', 'anonymous')}
        if 'anonymous' in field:
            result.update(flat(field['anonymous'], path + '.'))
    return result


def routes(path, symbol):
    graph = {}
    for line in path.read_text().splitlines():
        key, text = line.split(' ', 1)
        graph[key] = re.findall(r'\b[stueE]#[A-Za-z_][A-Za-z_0-9]*', text)
    if symbol not in graph:
        raise ValueError(f'{symbol} absent from {path}')
    found = {symbol: [symbol]}
    queue = deque([symbol])
    while queue:
        key = queue.popleft()
        for ref in graph.get(key, []):
            if ref not in found:
                found[ref] = found[key] + [ref]
                queue.append(ref)
    return found


def compare(stock, candidate, graph):
    result = []
    for key in sorted(stock['layouts'].keys() & candidate['layouts'].keys()):
        a = [canonical(r, stock.get('little_endian', True)) for r in stock['layouts'][key]]
        b = [canonical(r, candidate.get('little_endian', True)) for r in candidate['layouts'][key]]
        ashape = [canonical(r, stock.get('little_endian', True), False) for r in stock['layouts'][key]]
        bshape = [canonical(r, candidate.get('little_endian', True), False) for r in candidate['layouts'][key]]
        if all(x in b for x in a) and all(y in a for y in b):
            status = 'matching_named_layout'
        elif all(x in bshape for x in ashape) and all(y in ashape for y in bshape):
            status = 'matching_storage_different_type'
        else:
            status = 'different_layout'
        tag, name = key.split(':', 1)
        symbol = ('s#' if tag == 'structure_type' else 'u#') + name
        entry = {'type': key, 'status': status,
                 'stock_sizes': sorted({x['bytes'] for x in a}),
                 'candidate_sizes': sorted({x['bytes'] for x in b}),
                 'stock_variants': len(a), 'candidate_variants': len(b),
                 'crc_route': graph.get(symbol)}
        if status != 'matching_named_layout':
            # Keep each definition variant in the raw input. This detail view
            # compares the first pair, not an arbitrary claim of completeness.
            aa, bb = flat(a[0]), flat(b[0])
            entry['first_variant_differences'] = [
                {'member': n, 'stock': aa.get(n), 'candidate': bb.get(n)}
                for n in sorted(aa.keys() | bb.keys()) if aa.get(n) != bb.get(n)]
        result.append(entry)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('stock', type=Path)
    parser.add_argument('candidate', type=Path)
    parser.add_argument('symtypes', type=Path)
    parser.add_argument('output', type=Path)
    parser.add_argument('--symbol', default='module_layout')
    args = parser.parse_args()
    stock = json.loads(args.stock.read_text())
    candidate = json.loads(args.candidate.read_text())
    graph = routes(args.symtypes, args.symbol)
    records = compare(stock, candidate, graph)
    report = {
        'stock': {k: stock[k] for k in ('input', 'sha256')},
        'candidate': {k: candidate[k] for k in ('input', 'sha256')},
        'symtypes': str(args.symtypes),
        'symtypes_sha256': hashlib.sha256(args.symtypes.read_bytes()).hexdigest(),
        'root_symbol': args.symbol, 'reachable_type_records': len(graph) - 1,
        'summary': dict(Counter(x['status'] for x in records)),
        'types': records,
        'stock_types_not_observed_in_candidate': sorted(stock['layouts'].keys() - candidate['layouts'].keys()),
        'scope': ('Observed DWARF members only. Matching layout does not prove ABI or semantic compatibility; '
                  'function signatures, enum values, macros and unobserved types are not compared. '
                  'CRC routes describe the candidate genksyms graph, not the unavailable TCL graph.')}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps(report['summary'], sort_keys=True))


if __name__ == '__main__':
    main()
