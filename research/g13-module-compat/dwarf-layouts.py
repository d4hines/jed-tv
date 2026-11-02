#!/usr/bin/env python3
"""Extract named C aggregate layouts from saved kernel/module DWARF."""
import argparse
import hashlib
import json
from pathlib import Path

from elftools.elf.elffile import ELFFile


def value(die, name, default=None):
    attr = die.attributes.get('DW_AT_' + name)
    return attr.value if attr else default


def name(die):
    text = value(die, 'name', b'')
    return text.decode(errors='replace') if isinstance(text, bytes) else str(text)


def target(die):
    return die.get_DIE_from_attribute('DW_AT_type') if 'DW_AT_type' in die.attributes else None


def typename(die, seen=()):
    if die is None:
        return 'void'
    if die.offset in seen:
        return '<recursive>'
    seen = (*seen, die.offset)
    n = name(die)
    if die.tag == 'DW_TAG_pointer_type':
        return typename(target(die), seen) + ' *'
    if die.tag == 'DW_TAG_const_type':
        return 'const ' + typename(target(die), seen)
    if die.tag == 'DW_TAG_volatile_type':
        return 'volatile ' + typename(target(die), seen)
    if die.tag == 'DW_TAG_array_type':
        dims = [value(d, 'count', value(d, 'upper_bound', -1) + 1)
                for d in die.iter_children() if d.tag == 'DW_TAG_subrange_type']
        return typename(target(die), seen) + ''.join(f'[{d}]' for d in dims)
    if n:
        return n
    if die.tag == 'DW_TAG_subroutine_type':
        return '<function>'
    if target(die):
        return typename(target(die), seen)
    return '<anonymous ' + die.tag.removeprefix('DW_TAG_') + '>'


def size(die, seen=()):
    if die is None or die.offset in seen:
        return None
    direct = value(die, 'byte_size')
    if direct is not None:
        return direct
    if die.tag == 'DW_TAG_pointer_type':
        return die.cu['address_size']
    seen = (*seen, die.offset)
    child_size = size(target(die), seen)
    if die.tag == 'DW_TAG_array_type' and child_size is not None:
        for child in die.iter_children():
            if child.tag == 'DW_TAG_subrange_type':
                child_size *= value(child, 'count', value(child, 'upper_bound', -1) + 1)
    return child_size


def layout(die, seen=()):
    members = []
    for member in die.iter_children():
        if member.tag != 'DW_TAG_member':
            continue
        typ = target(member)
        while typ and typ.tag in ('DW_TAG_typedef', 'DW_TAG_const_type', 'DW_TAG_volatile_type'):
            typ = target(typ)
        offset = value(member, 'data_member_location', 0 if die.tag == 'DW_TAG_union_type' else None)
        if isinstance(offset, list):
            # Keep an unevaluated DWARF expression explicit.
            offset = {'expression': offset}
        entry = {'name': name(member), 'offset': offset,
                 'bytes': size(typ), 'type': typename(target(member))}
        for field in ('bit_size', 'bit_offset', 'data_bit_offset'):
            if 'DW_AT_' + field in member.attributes:
                entry[field] = value(member, field)
        if 'bit_size' in entry:
            # DWARF 4 producers may use either encoding. Canonicalize to an
            # offset from the aggregate's first bit, preserving the raw data.
            if 'data_bit_offset' in entry:
                entry['bit_start'] = entry['data_bit_offset']
            elif isinstance(offset, int) and entry['bytes'] is not None:
                within = entry['bit_offset']
                if die.dwarfinfo.config.little_endian:
                    within = entry['bytes'] * 8 - within - entry['bit_size']
                entry['bit_start'] = offset * 8 + within
        if typ and not name(typ) and typ.tag in ('DW_TAG_structure_type', 'DW_TAG_union_type'):
            if typ.offset not in seen:
                entry['anonymous'] = layout(typ, (*seen, die.offset))
        members.append(entry)
    return {'bytes': size(die), 'alignment': value(die, 'alignment'), 'members': members}


def extract(path):
    aggregates = {}
    with path.open('rb') as stream:
        elf = ELFFile(stream)
        if not elf.has_dwarf_info():
            raise ValueError(f'No DWARF in {path}')
        dwarf = elf.get_dwarf_info()
        count = 0
        producers, versions = set(), set()
        for cu in dwarf.iter_CUs():
            producers.add(value(cu.get_top_DIE(), 'producer', b'').decode(errors='replace'))
            versions.add(cu['version'])
            for die in cu.iter_DIEs():
                if die.tag not in ('DW_TAG_structure_type', 'DW_TAG_union_type'):
                    continue
                if not name(die) or value(die, 'declaration', False):
                    continue
                count += 1
                key = die.tag.removeprefix('DW_TAG_') + ':' + name(die)
                record = layout(die)
                variants = aggregates.setdefault(key, [])
                if record not in variants:
                    variants.append(record)
    with path.open('rb') as stream:
        digest = hashlib.file_digest(stream, 'sha256').hexdigest()
    return {'input': str(path), 'sha256': digest, 'aggregate_definitions': count,
            'elf_class': elf.elfclass, 'little_endian': elf.little_endian,
            'dwarf_versions': sorted(versions), 'producers': sorted(producers),
            'layouts': aggregates,
            'scope': 'DWARF type-layout evidence; equal layout does not establish semantic or runtime compatibility.'}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('elf', type=Path)
    parser.add_argument('output', type=Path)
    args = parser.parse_args()
    result = extract(args.elf)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + '\n')
    print(args.elf.name, len(result['layouts']), 'named layouts')


if __name__ == '__main__':
    main()
