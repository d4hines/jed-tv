#!/usr/bin/env python3
"""Check the RAM trial's actual ELF and unchanged assembly trust boundary."""
import sys
sys.dont_write_bytecode=True
import hashlib
import importlib.util
import json
from pathlib import Path
from capstone import Cs, CS_ARCH_ARM, CS_MODE_ARM
from capstone.arm import ARM_OP_IMM
from elftools.elf.elffile import ELFFile
from fixtures_tv import ROOT, OUT

def main():
    spec=importlib.util.spec_from_file_location('boundary',ROOT/'tools/bootloader/audit.py')
    boundary=importlib.util.module_from_spec(spec);spec.loader.exec_module(boundary)
    build=json.loads((OUT/'build.json').read_text())
    for name,expected in build['source_sha256'].items():
        assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==expected,name
    assert hashlib.sha256((OUT/'g13-update-ram.bin').read_bytes()).hexdigest()==build['binary_sha256']
    decoder=Cs(CS_ARCH_ARM,CS_MODE_ARM);decoder.detail=True
    with (ROOT/'assets/build/leanboot-update/entry.o').open('rb') as f:
        elf=ELFFile(f)
        for section in elf.iter_sections():
            if section['sh_flags']&4 and section['sh_size']:
                name=section.name.removeprefix('.text.')
                assert [(i.mnemonic,i.op_str) for i in decoder.disasm(section.data(),0)]==boundary.PRIMITIVES[name]
    with (OUT/'g13-update-ram.elf').open('rb') as f:
        elf=ELFFile(f);symbols=list(elf.get_section_by_name('.symtab').iter_symbols())
        assert elf['e_entry']==0
        assert not [s.name for s in symbols if s.name and s['st_shndx']=='SHN_UNDEF']
        assert not [s.name for s in elf.iter_sections() if s['sh_flags']&1 and s['sh_size']]
        names={s.name for s in symbols if s['st_info']['type']=='STT_FUNC'}
        assert not names.intersection({'malloc','calloc','free','exit','printf','fprintf'})
        forbidden=('lean_alloc','lean_apply','lean_inc','lean_dec','lean_box','leanboot_model',
                   'leanboot_write_journal','leanboot_read_sector','leanboot_jump','leanboot_begin_transition')
        assert not any(n.startswith(forbidden) or 'Bootstrap' in n for n in names)
        assert {'Hacl_Ed25519_verify','Hacl_Hash_SHA2_hash_256','leanboot_ethernet_start'}<=names
        text=elf.get_section_by_name('.text')
        trusted=[(s['st_value'],s['st_value']+4*len(boundary.PRIMITIVES[s.name])) for s in symbols if s.name in boundary.PRIMITIVES]
        markers=sorted((s['st_value'],s.name[1]) for s in symbols if s.name.startswith(('$a','$d','$t')))
        for index,(start,kind) in enumerate(markers):
            if kind=='d':continue
            assert kind=='a'
            end=markers[index+1][0] if index+1<len(markers) else text['sh_size']
            instructions=list(decoder.disasm(text.data()[start:end],start))
            assert sum(i.size for i in instructions)==end-start
            for ins in instructions:
                if ins.mnemonic in ('bl','blx'):
                    assert ins.operands[0].type==ARM_OP_IMM and 0<=ins.operands[0].imm<text['sh_size']
                if ins.mnemonic in ('mcr','mrc','msr','cps','cpsid','cpsie','svc','hvc','smc'):
                    assert any(a<=ins.address<b for a,b in trusted)
    report={'binary_sha256':build['binary_sha256'],'no_storage_driver':True,'no_model_hooks':True,
            'no_allocator':True,'no_writable_globals':True,'new_assembly':0,'direct_calls_only':True}
    (OUT/'audit.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report))

if __name__=='__main__':main()
