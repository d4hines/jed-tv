#!/usr/bin/env python3
"""Compare the Lean verified-boot packet with exact V013 MBOOT serialization.

The secure call is intercepted. No TV or secure firmware is contacted.
"""
import hashlib
import json
from pathlib import Path
import struct

from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_MEM_READ, UC_HOOK_MEM_WRITE
from unicorn.arm_const import *
from elftools.elf.elffile import ELFFile

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/build/leanboot'
BASE=0x273fdc00
SHA='643058879b84d044dea58a4572bd9ade1cc3140f3d3357eeaabae53be32ec382'


def stock_packet(version,digest):
    blob=(ROOT/'assets/backups/g13-v013-stock-20261031/MBOOT.img').read_bytes()
    assert hashlib.sha256(blob).hexdigest()==SHA
    u=Uc(UC_ARCH_ARM,UC_MODE_ARM)
    page=BASE&~4095
    u.mem_map(page,(len(blob)+BASE-page+4095)&~4095);u.mem_write(BASE,blob)
    heap,stack,stop=0x1000000,0x2000000,0x3000000
    for a in (heap,stack,stop):u.mem_map(a,0x10000)
    sp=stack+0x8000;source=stack+0x100
    u.mem_write(source,digest)
    u.mem_write(sp,struct.pack('<3I',0,len(digest),source))
    allocations=[];packets=[];flushed=[]
    def ret(value=0):
        u.reg_write(UC_ARM_REG_R0,value);u.reg_write(UC_ARM_REG_PC,u.reg_read(UC_ARM_REG_LR))
    def hook(uc,addr,size,data):
        if addr==0x274c0b20:
            n=u.reg_read(UC_ARM_REG_R0);p=heap+len(allocations)*0x1000
            allocations.append((p,n));ret(p)
        elif addr==0x274c08c4:ret()
        elif addr==0x274b09dc:
            flushed.append((u.reg_read(UC_ARM_REG_R0),u.reg_read(UC_ARM_REG_R1)));ret()
        elif addr==0x2742ebb4:
            args=u.reg_read(UC_ARM_REG_R0)
            fn,p=struct.unpack('<2I',u.mem_read(args,8))
            assert fn==0xb200585e and p==allocations[-1][0]
            assert flushed==[allocations[-1]]
            packets.append(bytes(u.mem_read(p,allocations[-1][1])));ret()
    for addr in (0x274c0b20,0x274c08c4,0x274b09dc,0x2742ebb4):
        u.hook_add(UC_HOOK_CODE,hook,begin=addr,end=addr)
    for reg,value in ((UC_ARM_REG_R0,version),(UC_ARM_REG_R1,0),(UC_ARM_REG_R2,0),
                      (UC_ARM_REG_R3,2),(UC_ARM_REG_SP,sp),(UC_ARM_REG_LR,stop)):
        u.reg_write(reg,value)
    u.emu_start(0x274c784c,stop,count=100000)
    assert u.reg_read(UC_ARM_REG_PC)==stop and len(packets)==1
    return packets[0]


def expected_packet(version,digest):
    out=bytearray(68+len(digest))
    struct.pack_into('<I',out,0,len(out)-4)
    out[4:10]=b'mboot\0';out[12:26]=b'verified-boot\0'
    struct.pack_into('<6I',out,40,version,0,0,2,0,len(digest))
    out[64:64+len(digest)]=digest
    return bytes(out)


def lean_packet(version,digest,capacity=132,output_delta=0,input_delta=0):
    code=(OUT/'g13-loader-core.bin').read_bytes()
    u=Uc(UC_ARCH_ARM,UC_MODE_ARM)
    base,source,destination,stack,stop=0x26000000,0x21000000,0x22000000,0x23000000,0x24000000
    u.mem_map(base,(len(code)+4095)&~4095);u.mem_write(base,code)
    for a in (source,destination,stack,stop):u.mem_map(a,0x10000)
    source+=input_delta;destination+=output_delta;sp=stack+0x8000
    u.mem_write(source,digest);u.mem_write(destination,bytes([0xa5])*256)
    u.mem_write(sp,struct.pack('<I',len(digest)))
    with (OUT/'g13-loader-core.elf').open('rb') as f:
        elf=ELFFile(f);symbols={s.name:base+s['st_value'] for s in elf.get_section_by_name('.symtab').iter_symbols()}
    def access(uc,kind,addr,size,value,writing):
        spans=[(sp-1024,sp+4),(destination,destination+min(capacity,132))]
        if not writing:spans += [(source,source+len(digest)),(base,base+len(code))]
        assert any(lo<=addr and addr+size<=hi for lo,hi in spans),(hex(addr),size,writing)
    u.hook_add(UC_HOOK_MEM_READ,access,False);u.hook_add(UC_HOOK_MEM_WRITE,access,True)
    for reg,value in ((UC_ARM_REG_R0,destination),(UC_ARM_REG_R1,capacity),(UC_ARM_REG_R2,version),
                      (UC_ARM_REG_R3,source),(UC_ARM_REG_SP,sp),(UC_ARM_REG_LR,stop)):
        u.reg_write(reg,value)
    saved=list(range(UC_ARM_REG_R4,UC_ARM_REG_R11+1))
    for i,reg in enumerate(saved):u.reg_write(reg,0x12340000+i)
    u.emu_start(symbols['leanboot_verified_boot_packet'],stop,count=100000)
    assert u.reg_read(UC_ARM_REG_PC)==stop and u.reg_read(UC_ARM_REG_SP)==sp
    assert [u.reg_read(reg) for reg in saved]==[0x12340000+i for i in range(8)]
    result=u.reg_read(UC_ARM_REG_R0)
    data=bytes(u.mem_read(destination,256))
    assert data[132:]==bytes([0xa5])*124
    if result:assert data==bytes([0xa5])*256
    return result,data[:132]


def call_boundary():
    code=(OUT/'g13-loader-core.bin').read_bytes();base=0x26000000;stack=0x23000000;stop=0x24000000
    with (OUT/'g13-loader-core.elf').open('rb') as f:
        elf=ELFFile(f);start=base+next(s['st_value'] for s in elf.get_section_by_name('.symtab').iter_symbols() if s.name=='leanboot_send_verified_boot')
    for status in (0,0xffffffff):
        u=Uc(UC_ARCH_ARM,UC_MODE_ARM);u.mem_map(base,(len(code)+4095)&~4095);u.mem_write(base,code)
        u.mem_map(stack,65536);u.mem_map(stop,4096);sp=stack+65536
        saved=list(range(UC_ARM_REG_R4,UC_ARM_REG_R12+1))
        for i,reg in enumerate(saved):u.reg_write(reg,0x12340000+i)
        u.reg_write(UC_ARM_REG_SP,sp);u.reg_write(UC_ARM_REG_LR,stop);u.reg_write(UC_ARM_REG_R0,0x26320000)
        seen=[]
        def smc(uc,addr,size,context):
            assert u.reg_read(UC_ARM_REG_R0)==0xb200585e and u.reg_read(UC_ARM_REG_R1)==0x26320000
            assert u.reg_read(UC_ARM_REG_SP)%8==0
            for reg in range(UC_ARM_REG_R0,UC_ARM_REG_R12+1):u.reg_write(reg,0xdeadbeef)
            u.reg_write(UC_ARM_REG_R0,status);u.reg_write(UC_ARM_REG_PC,addr+4);seen.append(True)
        u.hook_add(UC_HOOK_CODE,smc,begin=start+16,end=start+16)
        u.emu_start(start,stop,count=50)
        assert seen==[True] and u.reg_read(UC_ARM_REG_R0)==status
        assert u.reg_read(UC_ARM_REG_PC)==stop and u.reg_read(UC_ARM_REG_SP)==sp
        assert [u.reg_read(reg) for reg in saved]==[0x12340000+i for i in range(9)]


def main():
    digest=b'd8581697e658b05546004121ac9704c361c98e26ecad988e295f8f2e0a7df0c5'
    packet=stock_packet(0x1c000196,digest)
    assert packet==expected_packet(0x1c000196,digest)
    cases=0
    for version in (0,0x1c000196,0xffffffff):
        for text in (digest,b'0'*64,b'f'*64):
            expected=stock_packet(version,text)
            for shift in (0,1):
                result,actual=lean_packet(version,text,output_delta=shift,input_delta=shift)
                assert result==0 and actual==expected
                cases+=1
    for text,capacity in ((digest,131),(digest[:-1],132),(digest+b'0',132),(b'g'+digest[1:],132),(digest[:-1]+b'\0',132)):
        assert lean_packet(0x1c000196,text,capacity)[0]!=0;cases+=1
    call_boundary()
    (OUT/'verified-boot-packet.bin').write_bytes(packet)
    report={'firmware_sha256':SHA,'packet_bytes':len(packet),'packet_sha256':hashlib.sha256(packet).hexdigest(),
            'lean_packet_cases':cases,'ffi_cases':2,
            'scope':'Exact stock serialization vs compiled Lean; intercepted allocator, cache flush and secure call'}
    (OUT/'verified-boot-check.json').write_text(json.dumps(report,indent=2)+'\n')
    print(f'PASS: {cases} Lean packet cases vs exact V013 serialization; 2 FFI ABI cases; no secure call executed')


if __name__=='__main__':main()
