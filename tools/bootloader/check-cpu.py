#!/usr/bin/env python3
"""Exercise compiled Lean cache geometry and MIU sequencing with register models.

QEMU separately executes the real CP15 opcodes. This test controls register
values to exercise geometries/timeouts the virtual CPU does not expose.
"""
import json
from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE
from unicorn.arm_const import *
from os_profile import OUT


def run(clidr, sizes, expected, timeout=False, transition=False):
    u = Uc(UC_ARCH_ARM,UC_MODE_ARM)
    code = (OUT/'g13-loader-core.bin').read_bytes()
    base,stack,stop = 0x26000000,0x28000000,0x27000000
    for a,n in ((base,(len(code)+4095)&~4095),(stack,65536),(stop,4096)):
        u.mem_map(a,n)
    u.mem_write(base,code)
    with (OUT/'g13-loader-core.elf').open('rb') as f:
        e=ELFFile(f)
        symbols={s.name:base+s['st_value'] for s in e.get_section_by_name('.symtab').iter_symbols()}
    selection,operations,writes,control,polls = [0],[],[],[0xab00],[0]
    handlers={symbols[n]:n for n in ('leanboot_read_clidr','leanboot_read_ccsidr','leanboot_select_cache',
        'leanboot_clean_setway','leanboot_barrier','leanboot_load16','leanboot_store16',
        'leanboot_store32','leanboot_read_sctlr','leanboot_begin_transition')}
    def hook(uc,address,size,data):
        name=handlers.get(address)
        if name is None:return
        r0,r1=uc.reg_read(UC_ARM_REG_R0),uc.reg_read(UC_ARM_REG_R1)
        result=0
        if name=='leanboot_read_clidr':result=clidr
        elif name=='leanboot_read_ccsidr':result=sizes[selection[0]//2]
        elif name=='leanboot_select_cache':selection[0]=r0
        elif name=='leanboot_clean_setway':operations.append(r0)
        elif name=='leanboot_read_sctlr':result=0xc0181d
        elif name=='leanboot_load16':
            assert r0 in (0x1f203114,0x1f203140)
            if r0==0x1f203114:result=control[0]
            else:
                polls[0]+=1
                result=0 if timeout or polls[0]<3 else 0x1000
        elif name=='leanboot_store16':
            assert r0==0x1f203114
            control[0]=r1
            writes.append((r0,r1))
        elif name=='leanboot_store32':writes.append((r0,r1))
        elif name=='leanboot_begin_transition':
            assert r0==0xc00818 and transition
            assert r1==0x20208000 and uc.reg_read(UC_ARM_REG_R2)==0x23000000
            uc.emu_stop();return
        uc.reg_write(UC_ARM_REG_R0,result)
        uc.reg_write(UC_ARM_REG_PC,uc.reg_read(UC_ARM_REG_LR))
    # Hook only primitive entry addresses; avoid a Python callback per instruction.
    for address in handlers:u.hook_add(UC_HOOK_CODE,hook,begin=address,end=address)
    u.reg_write(UC_ARM_REG_CPSR,0x1d3)
    u.reg_write(UC_ARM_REG_SP,stack+65536)
    u.reg_write(UC_ARM_REG_LR,stop)
    if transition:
        u.reg_write(UC_ARM_REG_R0,0x20208000)
        u.reg_write(UC_ARM_REG_R1,0x23000000)
    entry='leanboot_enter_linux' if transition else 'leanboot_flush_miu' if timeout else 'leanboot_clean_caches'
    u.emu_start(symbols[entry],stop,count=30000000)
    assert operations==expected,(operations,expected)
    assert selection[0]==0
    if timeout:
        assert polls[0]==1048576 and u.reg_read(UC_ARM_REG_R0)==1
        assert writes==[(0x1f203114,0xab00),(0x1f203114,0xab01),(0x1f203114,0xab00)]
    elif transition:
        assert u.reg_read(UC_ARM_REG_PC)==symbols['leanboot_begin_transition']
        assert writes[3:]==[(a,0xffff) for a in (0x1f203290,0x1f203294,0x1f2032d0,0x1f2032d4,
                                                0x1f203298,0x1f20329c,0x1f2032d8,0x1f2032dc)]
    else:assert u.reg_read(UC_ARM_REG_PC)==stop


def main():
    run(0,{},[])
    run(0x01000001,{},[])
    cases=[[(3,4,8,6),(4,8,16,6)],[(4,1,4,5)],[(4,1024,1,4)]]
    for cache in cases:
        clidr=len(cache)<<24;sizes={};expected=[]
        for level,(kind,ways,sets,shift) in enumerate(cache):
            clidr|=kind<<(level*3)
            sizes[level]=((sets-1)<<13)|((ways-1)<<3)|(shift-4)
            way_shift=32-(ways-1).bit_length()
            expected += [(level<<1)|(w<<way_shift)|(s<<shift) for w in reversed(range(ways)) for s in reversed(range(sets))]
        run(clidr,sizes,expected)
        run(clidr,sizes,expected,transition=True)
    run(0,{},[],timeout=True)
    (OUT/'cpu-check.json').write_text(json.dumps({'cases':9,'passed':True,'model':'register-controlled ARM execution'})+'\n')
    print('PASS: 9 cache geometry, terminal sequencing and MIU timeout cases')


if __name__=='__main__':main()
