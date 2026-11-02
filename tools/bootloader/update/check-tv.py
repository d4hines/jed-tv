#!/usr/bin/env python3
"""Execute the exact RAM-trial ARM image, including real crypto and RAM banks."""
import sys
sys.dont_write_bytecode=True
import hashlib
import importlib.util
import json
import struct
from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_MEM_READ, UC_HOOK_MEM_WRITE
from unicorn.arm_const import *
from fixtures_tv import ROOT, OUT, BANK_SIZE, bank_image, prepare, transfer

spec=importlib.util.spec_from_file_location('eth',ROOT/'tools/bootloader/check-ethernet.py')
eth=importlib.util.module_from_spec(spec)
sys.path.append(str(ROOT/'tools/bootloader'));spec.loader.exec_module(eth)
BASE=0x26000000;STOP=0x27000000

def trial(fixtures,bad_mapping=False,bad_seed=False,busy=False):
    packets=[(eth.arp(),None)]
    for name,session,status,dup in [('v2',101,0,True),('forged',102,18,False),
            ('changed',103,18,False),('seed',104,18,False),('v3',105,0,True)]:
        for request,expected in transfer(fixtures[name],session,status,dup):
            packets.append((eth.udp(request),(request,expected)))
    code=(OUT/'g13-update-ram.bin').read_bytes()
    with (OUT/'g13-update-ram.elf').open('rb') as f:
        symbols={s.name:BASE+s['st_value'] for s in ELFFile(f).get_section_by_name('.symtab').iter_symbols()}
    u=Uc(UC_ARCH_ARM,UC_MODE_ARM)
    for a,n in [(BASE,0x20000),(STOP,4096),(0x26200000,0x50000),(0x26330000,65536),
                (0x28000000,65536),(0x1f000000,0x210000),(0x297f0000,65536),(0x66400000,65536)]:u.mem_map(a,n)
    u.mem_write(BASE,code)
    def word(a,v):u.mem_write(a,struct.pack('<I',v))
    for a,v in [(0x297f0980,0x26000c0e),(0x297f0988,0x26200c0e),(0x297f098c,0x26300c0e),
                (0x297f0990,0x26400c0e),(0x297f1990,0x26400c06)]:word(a,v)
    if bad_mapping:word(0x297f0988,0)
    seed=bytearray(fixtures['seed'])
    if bad_seed:seed[96]^=1
    word(0x26200000,0x55504431);word(0x26200004,len(seed));u.mem_write(0x26201000,bytes(seed))
    regs={0:12 if busy else 0,8:4,0x14:0x1f98,0x34:0};phy={1:0x786d,2:0,3:0,4:0x1e1,5:0x1e1}
    injected=0;completed=0;maximum=0;serial=bytearray();before=None;writes_before=0
    allowed=[(0x2800fff8,0x28010000),(0x26200100,0x26200160),(0x26200200,0x26200228),
             (0x26210000,0x26210000+BANK_SIZE),(0x26220000,0x26220000+BANK_SIZE),
             (0x26230000,0x26230000+32768+160),(0x26240000,0x26240000+BANK_SIZE),
             (0x26330000,0x26340000),(0x66400000,0x66404800)]
    def banks():return [bytes(u.mem_read(a,BANK_SIZE)) for a in (0x26210000,0x26220000)]
    def read(uc,access,a,n,v,d):
        nonlocal injected,before,writes_before
        if 0x1f204000<=a<0x1f204300:
            off=((a-0x1f204000)//8)*4;word(a,(regs.get(off,0)>>(16 if a&4 else 0))&65535)
        elif a==0x1f201328:word(a,0x61 if completed==len(packets) else 0x60)
        elif 0x66400000<=a<0x66400020 and a%8==0 and regs.get(0,0)&4 and injected==completed and injected<len(packets):
            frame,expect=packets[injected];index=(a-0x66400000)//8
            u.mem_write(0x66401000+index*2048,frame+bytes(4))
            word(a,0x06401001+index*2048+(2 if index==3 else 0));word(a+4,len(frame)+4)
            if expect and struct.unpack_from('<I',expect[0],4)[0]==3:
                before=banks();writes_before=struct.unpack('<I',u.mem_read(0x26200140,4))[0]
            injected+=1
    def write(uc,access,a,n,v,d):
        nonlocal completed,maximum
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
                    assert regs[0]&8 and regs[0xc]==0x06404000
                    reply=bytes(u.mem_read(0x66404000,v));frame,expect=packets[completed]
                    assert reply[:12]==eth.PEER+eth.OWN
                    if expect:
                        request,status=expect;_,op,session,offset,count=struct.unpack('<5I',request[:20])
                        acknowledged=0 if op==1 else offset+count if op==2 else offset
                        assert v==62 and eth.checksum(reply[14:34])==0
                        assert eth.checksum(eth.IP+eth.REMOTE+struct.pack('!BBH',0,17,28)+reply[34:])==0
                        assert reply[42:]==struct.pack('<5I',0x3152554c,session,op,status,acknowledged)
                        if op==3:
                            after=banks();writes=struct.unpack('<I',u.mem_read(0x26200140,4))[0]
                            if status:assert before==after and writes==writes_before
                            if session==101:assert after==[bank_image(fixtures['seed']),bank_image(fixtures['v2'])]
                            if session==105:assert after==[bank_image(fixtures['v3']),bank_image(fixtures['v2'])]
                    completed+=1;regs[0x14]=0x1fb8
        else:
            assert any(lo<=a and a+n<=hi for lo,hi in allowed),(hex(a),n)
            if 0x26330000<=a<0x26340000:maximum=max(maximum,0x26340000-a)
    u.hook_add(UC_HOOK_MEM_READ,read);u.hook_add(UC_HOOK_MEM_WRITE,write)
    def primitive(uc,a,n,name):
        arg=uc.reg_read(UC_ARM_REG_R0)
        value=0xc0181d if name=='leanboot_read_sctlr' else 0x1000 if name=='leanboot_load16' and arg==0x1f203140 else 0
        uc.reg_write(UC_ARM_REG_R0,value);uc.reg_write(UC_ARM_REG_PC,uc.reg_read(UC_ARM_REG_LR))
    for name in ['leanboot_read_sctlr','leanboot_read_clidr','leanboot_load16']:
        u.hook_add(UC_HOOK_CODE,primitive,name,begin=symbols[name],end=symbols[name])
    u.reg_write(UC_ARM_REG_CPSR,0x13);u.reg_write(UC_ARM_REG_SP,0x28010000);u.reg_write(UC_ARM_REG_LR,STOP)
    u.emu_start(BASE,STOP,count=300_000_000)
    assert u.reg_read(UC_ARM_REG_PC)==STOP,hex(u.reg_read(UC_ARM_REG_PC))
    result=u.reg_read(UC_ARM_REG_R0);expected=27 if bad_mapping else 29 if busy else 41 if bad_seed else 0
    assert result==expected,(result,expected)
    assert u.reg_read(UC_ARM_REG_SP)==0x28010000 and u.reg_read(UC_ARM_REG_CPSR)&255==0x13
    if not (bad_mapping or bad_seed or busy):
        assert completed==len(packets) and regs[0]&12==0
        assert struct.unpack('<4I',u.mem_read(0x26200140,16))==(72,0,0,3)
    return {'result':result,'frames':completed,'stack_bytes':maximum,'uart':serial.decode()}

def main():
    fixtures=prepare()
    result={'valid':trial(fixtures),'bad_mapping':trial(fixtures,bad_mapping=True),
            'busy_mac':trial(fixtures,busy=True),'bad_signature_seed':trial(fixtures,bad_seed=True),
            'binary_sha256':hashlib.sha256((OUT/'g13-update-ram.bin').read_bytes()).hexdigest()}
    (OUT/'checks.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result))

if __name__=='__main__':main()
