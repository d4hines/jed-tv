#!/usr/bin/env python3
"""Exercise G13 admission and the returnable private-stack entry, offline."""
import json
from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_MEM_WRITE
from unicorn.arm_const import *
from build import OUT


def run(cpsr=0x1d3,sctlr=0xc0181d,direct=False):
    base,old_sp,stop=0x26000000,0x28010000,0x27f00000
    private_low,private_high=0x26330000,0x26340000
    code=(OUT/'g13-os.bin').read_bytes()
    with (OUT/'g13-os.elf').open('rb') as f:
        elf=ELFFile(f);symbols={s.name:base+s['st_value'] for s in elf.get_section_by_name('.symtab').iter_symbols()}
    u=Uc(UC_ARCH_ARM,UC_MODE_ARM)
    for address,size in [(base,(len(code)+4095)&~4095),(old_sp-65536,65536),
                         (private_low,65536),(stop,4096),(0x26200000,65536),(0x1f201000,4096)]:u.mem_map(address,size)
    u.mem_write(base,code)
    u.mem_write(0x1f201328,b'\x60\x00\x00\x00')
    events=[];maximum=[0]
    def writes(uc,access,address,size,value,data):
        if address==0x1f201300:
            assert size==1
            events.append('uart')
            return
        if private_low<=address and address+size<=private_high:
            maximum[0]=max(maximum[0],private_high-address)
        else:assert old_sp-(4096 if direct else 8)<=address and address+size<=old_sp,hex(address)
    u.hook_add(UC_HOOK_MEM_WRITE,writes)
    def handler(uc,address,size,name):
        events.append(name)
        value=sctlr
        uc.reg_write(UC_ARM_REG_R0,value);uc.reg_write(UC_ARM_REG_PC,uc.reg_read(UC_ARM_REG_LR))
    for name in ('leanboot_read_sctlr',):
        u.hook_add(UC_HOOK_CODE,handler,name,begin=symbols[name],end=symbols[name])
    u.reg_write(UC_ARM_REG_CPSR,cpsr)
    u.reg_write(UC_ARM_REG_SP,old_sp);u.reg_write(UC_ARM_REG_LR,stop)
    saved=list(range(UC_ARM_REG_R4,UC_ARM_REG_R11+1))
    for i,r in enumerate(saved):u.reg_write(r,0xa000+i)
    u.emu_start(symbols['leanboot_os_main' if direct else 'leanboot_os_start'],stop,count=1000000)
    valid=(cpsr&0xff)==0xd3 and sctlr==0xc0181d and not direct
    assert u.reg_read(UC_ARM_REG_PC)==stop
    assert u.reg_read(UC_ARM_REG_R0)==(1 if valid else 26)
    assert u.reg_read(UC_ARM_REG_SP)==old_sp
    assert [u.reg_read(r) for r in saved]==[0xa000+i for i in range(len(saved))]
    if valid:assert 'uart' in events
    else:assert not any('uart' in event for event in events)
    if cpsr&0xff!=0xd3:assert 'leanboot_read_sctlr' not in events
    return {'cpsr':hex(cpsr),'sctlr':hex(sctlr),'direct':direct,'stack_bytes':maximum[0],
            'entry_accepted':valid,'return_and_registers_preserved':True}


def main():
    cases=[run(),run(cpsr=0x10),run(cpsr=0x93),run(cpsr=0xd2),run(sctlr=0),run(direct=True)]
    (OUT/'entry-check.json').write_text(json.dumps(cases,indent=2)+'\n')
    print('PASS: six entry/admission cases, private stack and caller register restoration')


if __name__=='__main__':main()
