#!/usr/bin/env python3
"""Audit the updater's distinct crypto/model boundary and freestanding ELF."""
import hashlib
import json
from pathlib import Path
import re
from capstone import Cs, CS_ARCH_ARM, CS_MODE_ARM
from capstone.arm import ARM_OP_IMM
from elftools.elf.elffile import ELFFile

ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'assets/build/leanboot-update'


def audit():
    for name in ['Unsafe/UpdateCrypto.lean']:
        text=(ROOT/'src/bootloader'/name).read_text()
        assert len(re.findall(r'@\[extern',text))==len(re.findall(r'@\[extern "[^"]+"\]\s*private opaque',text))==2
    for source in (ROOT/'src/bootloader/Update').glob('*.lean'):
        if source.name=='Proofs.lean':continue
        assert not re.search(r'\b(?:BaseIO|Unsafe|HardwareM|MemoryM|extern|unsafe|axiom)\b',source.read_text()),source
    lock=json.loads((Path(__file__).parent/'crypto-lock.json').read_text())
    for name,expected in lock['files'].items():
        assert hashlib.sha256((OUT/'hacl'/name).read_bytes()).hexdigest()==expected,name
    with (OUT/'updater-model.elf').open('rb') as f:
        elf=ELFFile(f)
        symbols=list(elf.get_section_by_name('.symtab').iter_symbols())
        assert not [s.name for s in symbols if s.name and s['st_shndx']=='SHN_UNDEF']
        assert not [s.name for s in elf.iter_sections() if s['sh_flags']&1 and s['sh_size']]
        names={s.name for s in symbols if s['st_info']['type']=='STT_FUNC'}
        assert not names.intersection({'malloc','calloc','free','exit','printf','fprintf'})
        assert not any(n.startswith(('lean_alloc','lean_apply','lean_inc','lean_dec','lean_box')) for n in names)
        # No board I/O or CPU transitions are reachable in this separate ELF.
        assert not any(n.startswith(('leanboot_read_sctlr','leanboot_uart','leanboot_write_journal','leanboot_jump','leanboot_model_erase')) for n in names)
        assert {'leanboot_model_read','leanboot_model_write','leanboot_model_flush'}<=names
        assert 'Hacl_Ed25519_verify' in names and 'Hacl_Hash_SHA2_hash_256' in names
        text=elf.get_section_by_name('.text')
        markers=sorted((s['st_value'],s.name[1]) for s in symbols if s.name.startswith(('$a','$d','$t')))
        decoder=Cs(CS_ARCH_ARM,CS_MODE_ARM);decoder.detail=True
        for index,(start,kind) in enumerate(markers):
            if kind=='d':continue
            assert kind=='a','Unexpected instruction set'
            end=markers[index+1][0] if index+1<len(markers) else text['sh_size']
            instructions=list(decoder.disasm(text.data()[start:end],start))
            assert sum(i.size for i in instructions)==end-start
            for instruction in instructions:
                if instruction.mnemonic in ('bl','blx'):
                    assert instruction.operands[0].type==ARM_OP_IMM, instruction.op_str
                    assert 0<=instruction.operands[0].imm<text['sh_size']
                assert instruction.mnemonic not in ('mcr','mrc','msr','cps','cpsid','cpsie','svc','hvc','smc')
    report={'no_runtime_or_allocator':True,'no_writable_globals':True,
            'pinned_crypto_files':len(lock['files']),'crypto_ffi':2,'model_only_storage_ffi':3,
            'direct_calls_only':True,'no_privileged_instructions':True,
            'hardware_storage_backend':False}
    (OUT/'audit.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report))


if __name__=='__main__':audit()
