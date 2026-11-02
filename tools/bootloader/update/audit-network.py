#!/usr/bin/env python3
"""Audit network-boot images, allowing only the named authenticated-call bridge."""
import sys
sys.dont_write_bytecode=True
import hashlib
import importlib.util
import json
from capstone import Cs,CS_ARCH_ARM,CS_MODE_ARM
from capstone.arm import ARM_OP_IMM
from elftools.elf.elffile import ELFFile
from fixtures_tv import ROOT

OUT=ROOT/'assets/build/leanboot-network'
def main():
    spec=importlib.util.spec_from_file_location('boundary',ROOT/'tools/bootloader/audit.py')
    boundary=importlib.util.module_from_spec(spec);spec.loader.exec_module(boundary)
    build=json.loads((OUT/'build.json').read_text())
    for name,digest in build['source_sha256'].items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==digest,name
    decoder=Cs(CS_ARCH_ARM,CS_MODE_ARM);decoder.detail=True
    with (ROOT/'assets/build/leanboot-update/entry.o').open('rb') as f:
        for section in ELFFile(f).iter_sections():
            if section['sh_flags']&4 and section['sh_size']:
                name=section.name.removeprefix('.text.')
                assert [(i.mnemonic,i.op_str) for i in decoder.disasm(section.data(),0)]==boundary.PRIMITIVES[name],name
    for name,info in build['outputs'].items():
        assert hashlib.sha256((OUT/(name+'.bin')).read_bytes()).hexdigest()==info['sha256']
        with (OUT/(name+'.elf')).open('rb') as f:
            elf=ELFFile(f);symbols=list(elf.get_section_by_name('.symtab').iter_symbols())
            assert elf['e_entry']==0
            assert not [s.name for s in symbols if s.name and s['st_shndx']=='SHN_UNDEF']
            assert not [s.name for s in elf.iter_sections() if s['sh_flags']&1 and s['sh_size']]
            names={s.name for s in symbols if s['st_info']['type']=='STT_FUNC'}
            assert not names.intersection({'malloc','calloc','free','exit','printf','fprintf'})
            assert not any(n.startswith(('lean_alloc','lean_apply','lean_inc','lean_dec','lean_box','leanboot_model')) for n in names)
            if name!='network-gate':
                assert not names.intersection({'leanboot_write_journal','leanboot_read_sector'})
            trusted=[(s['st_value'],s['st_value']+4*len(boundary.PRIMITIVES[s.name])) for s in symbols if s.name in boundary.PRIMITIVES]
            call=next((s['st_value'] for s in symbols if s.name=='leanboot_call_network'),-100)
            text=elf.get_section_by_name('.text')
            markers=sorted((s['st_value'],s.name[1]) for s in symbols if s.name.startswith(('$a','$d','$t')))
            for index,(start,kind) in enumerate(markers):
                if kind=='d':continue
                assert kind=='a'
                end=markers[index+1][0] if index+1<len(markers) else text['sh_size']
                instructions=list(decoder.disasm(text.data()[start:end],start))
                assert sum(i.size for i in instructions)==end-start
                for ins in instructions:
                    if ins.mnemonic in ('bl','blx'):
                        if ins.operands[0].type!=ARM_OP_IMM:assert ins.address==call+44 and ins.op_str=='r3'
                        else:assert 0<=ins.operands[0].imm<text['sh_size']
                    if ins.mnemonic in ('mcr','mrc','msr','cps','cpsid','cpsie','svc','hvc','smc'):
                        assert any(a<=ins.address<b for a,b in trusted)
    result={'outputs':build['outputs'],'only_indirect_call':'leanboot_call_network + 44',
            'new_assembly_instructions':14,'no_allocator':True,'no_writable_globals':True,
            'network_storage':'signed app images in fixed upgrade banks; temporary bootloaders in RAM',
            'gate_storage':'fixed application banks plus existing two-sector boot journal'}
    (OUT/'audit.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))

if __name__=='__main__':main()
