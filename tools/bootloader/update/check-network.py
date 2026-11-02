#!/usr/bin/env python3
"""Check real ARM network admission, isolated-stack call, and return, with a MAC model."""
import sys
sys.dont_write_bytecode=True
import hashlib
import argparse
import importlib.util
import json
import struct
from elftools.elf.elffile import ELFFile
from unicorn import Uc,UC_ARCH_ARM,UC_MODE_ARM,UC_HOOK_CODE,UC_HOOK_MEM_READ,UC_HOOK_MEM_WRITE
from unicorn.arm_const import *
from fixtures_tv import ROOT,transfer
from package import package

OUT=ROOT/'assets/build/leanboot-network'
spec=importlib.util.spec_from_file_location('eth',ROOT/'tools/bootloader/check-ethernet.py')
eth=importlib.util.module_from_spec(spec);sys.path.append(str(ROOT/'tools/bootloader'));spec.loader.exec_module(eth)

def run(key,mmu=True,upload=True):
    payload=(OUT/'network-example.bin').read_bytes()
    blob=package(payload,key,2)
    (OUT/'example.lbu').write_bytes(blob)
    bad=bytearray(blob);bad[96]^=1
    requests=transfer(bytes(bad),1,18)+transfer(blob,2) if upload else []
    packets=[(eth.udp(p),p,status) for p,status in requests]
    dma=0x66400000 if mmu else 0x26400000
    u=Uc(UC_ARCH_ARM,UC_MODE_ARM);code=(OUT/'network-ram.bin').read_bytes()
    for a,n in [(0x26000000,0x20000),(0x26200000,65536),(0x26330000,0x30000),
                (dma,65536),(0x28000000,65536),(0x27000000,4096),(0x40300000,0x500000),(0x21200000,0x1000000),
                (0x1f000000,0x210000)]:u.mem_map(a,n)
    u.mem_write(0x26000000,code)
    with (OUT/'network-ram.elf').open('rb') as f:
        syms={s.name:0x26000000+s['st_value'] for s in ELFFile(f).get_section_by_name('.symtab').iter_symbols()}
    def word(a,v):u.mem_write(a,struct.pack('<I',v))
    regs={0:0,8:4,0x14:0x1f98,0x34:0};phy={1:0x786d,4:0x1e1,5:0x1e1}
    injected=0;completed=0;entered=0;serial=bytearray();stack_low=0x26360000
    def read(uc,access,a,n,v,d):
        nonlocal injected
        if 0x1f204000<=a<0x1f204300:
            off=((a-0x1f204000)//8)*4;word(a,(regs.get(off,0)>>(16 if a&4 else 0))&65535)
        elif a==0x1f201328:word(a,0x60)
        elif dma<=a<dma+32 and a%8==0 and regs.get(0,0)&4 and injected==completed and injected<len(packets):
            frame,_,_=packets[injected];index=(a-dma)//8
            u.mem_write(dma+0x1000+index*2048,frame+bytes(4))
            word(a,0x06401001+index*2048+(2 if index==3 else 0));word(a+4,len(frame)+4);injected+=1
    def write(uc,access,a,n,v,d):
        nonlocal completed,stack_low
        if 0x1f000000<=a<0x1f210000:
            assert uc.reg_read(UC_ARM_REG_CPSR)&255==0xd3
            if a==0x1f201300:serial.append(v&255)
            if 0x1f204000<=a<0x1f204300:
                off=((a-0x1f204000)//8)*4;old=regs.get(off,0)
                regs[off]=((old&65535)|(v<<16)) if a&4 else ((old&0xffff0000)|v)
                if off==0x34 and a&4:
                    command=regs[off];reg=(command>>18)&31
                    if command&0x30000000==0x20000000:regs[off]=(command&0xffff0000)|phy.get(reg,0)
                    else:phy[reg]=command&65535
                if off==0x14:regs[off]=0x1f98
                if off==0x10 and not a&4 and v:
                    assert regs[0]&8 and regs[0xc]==0x06404000 and entered==0
                    reply=bytes(u.mem_read(dma+0x4000,v));_,request,status=packets[completed]
                    _,op,session,offset,count=struct.unpack('<5I',request[:20])
                    ack=0 if op==1 else offset+count if op==2 else offset
                    assert v==62 and eth.checksum(reply[14:34])==0
                    assert eth.checksum(eth.IP+eth.REMOTE+struct.pack('!BBH',0,17,28)+reply[34:])==0
                    assert reply[42:]==struct.pack('<5I',0x3152554c,session,op,status,ack)
                    completed+=1;regs[0x14]=0x1fb8
        else:
            allowed=[(0x2800fff8,0x28010000),(0x26200100,0x26200128),
                     (0x26330000,0x26340000),(0x26350000,0x26360000),(dma,dma+0x4800),
                     (0x40300000,0x40300080),(0x40301000,0x40301024),
                     (0x21200000,0x21200000+len(blob)),(0x40600000,0x40600000+len(payload))]
            assert any(lo<=a and a+n<=hi for lo,hi in allowed),(hex(a),n)
            if 0x26350000<=a<0x26360000:stack_low=min(stack_low,a)
    u.hook_add(UC_HOOK_MEM_READ,read);u.hook_add(UC_HOOK_MEM_WRITE,write)
    def primitive(uc,a,n,name):
        arg=uc.reg_read(UC_ARM_REG_R0)
        value=(0xc0181d if mmu else 0xc01818) if name=='leanboot_read_sctlr' else 0x1000 if name=='leanboot_load16' and arg==0x1f203140 else 0
        uc.reg_write(UC_ARM_REG_R0,value);uc.reg_write(UC_ARM_REG_PC,uc.reg_read(UC_ARM_REG_LR))
    for name in ['leanboot_read_sctlr','leanboot_read_clidr','leanboot_load16']:
        u.hook_add(UC_HOOK_CODE,primitive,name,begin=syms[name],end=syms[name])
    def entry(uc,a,n,d):
        nonlocal entered
        assert upload and completed==len(packets) and regs[0]&12==0
        assert bytes(uc.mem_read(a,len(payload)))==payload
        assert uc.reg_read(UC_ARM_REG_SP)==0x26360000
        entered+=1
    u.hook_add(UC_HOOK_CODE,entry,begin=0x40600000,end=0x40600000)
    # Test the callable window in both MBOOT and final-kernel CPU contexts.
    u.reg_write(UC_ARM_REG_CPSR,0xd3);u.reg_write(UC_ARM_REG_SP,0x26340000)
    u.reg_write(UC_ARM_REG_LR,0x27000000);u.reg_write(UC_ARM_REG_R0,0x284ddc00)
    u.reg_write(UC_ARM_REG_R1,400000 if upload else 1000)
    u.emu_start(syms['leanboot_network_window'],0x27000000,count=100_000_000)
    assert u.reg_read(UC_ARM_REG_PC)==0x27000000 and u.reg_read(UC_ARM_REG_R0)==0
    assert u.reg_read(UC_ARM_REG_SP)==0x26340000 and regs[0]&12==0
    assert entered==int(upload)
    if upload:assert struct.unpack('<I',u.mem_read(0x40301020,4))[0]==0x1ea4
    return {'mmu':mmu,'upload':upload,'frames':completed,'executions':entered,
            'payload_stack_bytes':0x26360000-stack_low,'uart':serial.decode()}

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--key',type=__import__('pathlib').Path,required=True)
    args=parser.parse_args()
    result={'cases':[run(args.key),run(args.key,False),run(args.key,upload=False),run(args.key,False,False)],
            'sha256':hashlib.sha256((OUT/'network-ram.bin').read_bytes()).hexdigest()}
    (OUT/'checks.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))

if __name__=='__main__':main()
