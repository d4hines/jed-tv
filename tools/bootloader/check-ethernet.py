#!/usr/bin/env python3
"""Execute the compiled Lean packet core and EMAC driver under guarded ARM emulation.

The MMIO model tests sequencing and failure paths, not the electrical PHY/DMA.
No TV access. The optional exact-MBOOT recipe comparison uses the saved backup.
"""
import hashlib
import json
from pathlib import Path
import random
import struct

from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_MEM_READ, UC_HOOK_MEM_WRITE, UC_HOOK_CODE
from unicorn.arm_const import *
from build import ROOT, OUT

BASE, STOP, RX, TX = 0x26000000, 0x27000000, 0x26500000, 0x26501000
OWN = bytes.fromhex('18acc22d1c47')
PEER = bytes.fromhex('020000000001')
IP, REMOTE = bytes([192,168,77,3]), bytes([192,168,77,1])


def checksum(data):
    data += bytes(len(data) % 2)
    total = sum(struct.unpack('!'+'H'*(len(data)//2), data))
    while total >> 16: total = (total & 65535) + (total >> 16)
    return total ^ 65535


def arp():
    return b'\xff'*6 + PEER + bytes.fromhex('08060001080006040001') + PEER + REMOTE + bytes(6) + IP


def ip_packet(protocol, data, fragment=0):
    header = bytearray(struct.pack('!BBHHHBBH4s4s', 0x45,0,20+len(data),7,fragment,64,protocol,0,REMOTE,IP))
    struct.pack_into('!H',header,10,checksum(bytes(header)))
    return OWN + PEER + bytes.fromhex('0800') + header + data


def icmp(payload):
    data = bytearray(bytes.fromhex('0800000012340001') + payload)
    struct.pack_into('!H',data,2,checksum(bytes(data)))
    return ip_packet(1,data)


def udp(payload, checked=True):
    data = bytearray(struct.pack('!HHHH',43210,7533,len(payload)+8,0)+payload)
    if checked:
        pseudo = REMOTE + IP + struct.pack('!BBH',0,17,len(data))
        struct.pack_into('!H',data,6,checksum(pseudo+data) or 65535)
    return ip_packet(17,data)


def machine():
    code = (OUT/'g13-ethernet.bin').read_bytes()
    with (OUT/'g13-ethernet.elf').open('rb') as f:
        symbols = {s.name:BASE+s['st_value'] for s in ELFFile(f).get_section_by_name('.symtab').iter_symbols()}
    u = Uc(UC_ARCH_ARM,UC_MODE_ARM)
    for a,n in [(BASE,65536),(STOP,4096),(0x26330000,65536),(0x28000000,65536),(RX,8192)]:u.mem_map(a,n)
    u.mem_write(BASE,code)
    u.reg_write(UC_ARM_REG_CPSR,0x1d3)
    u.reg_write(UC_ARM_REG_SP,0x28010000)
    u.reg_write(UC_ARM_REG_LR,STOP)
    return u,symbols


def core(frame, accepted=True):
    frame=bytes(frame)
    u,s = machine(); writes=[]
    u.mem_write(RX,frame);u.mem_write(TX,b'\xa5'*2048)
    def guard(uc,access,a,n,v,d):
        assert 0x28000000<=a and a+n<=0x28010000 or TX<=a and a+n<=TX+1514,hex(a)
        if TX<=a<TX+2048:writes.append(a)
    u.hook_add(UC_HOOK_MEM_WRITE,guard)
    def readguard(uc,access,a,n,v,d):
        if RX<=a<RX+4096:assert a+n<=RX+len(frame),(a,n,len(frame))
    u.hook_add(UC_HOOK_MEM_READ,readguard)
    for reg,value in [(UC_ARM_REG_R0,RX),(UC_ARM_REG_R1,len(frame)),(UC_ARM_REG_R2,TX),(UC_ARM_REG_R3,2048)]:u.reg_write(reg,value)
    u.emu_start(s['leanboot_ethernet_respond'],STOP,count=2000000)
    assert u.reg_read(UC_ARM_REG_PC)==STOP
    n=u.reg_read(UC_ARM_REG_R0)
    if not accepted:
        assert n==0 and not writes
        return
    assert 60<=n<=1514
    reply=bytes(u.mem_read(TX,n))
    assert reply[:12]==PEER+OWN
    if frame[12:14]==b'\x08\x06':
        assert reply[14:22]==bytes.fromhex('0001080006040002')
        assert reply[22:42]==OWN+IP+PEER+REMOTE
        assert reply[42:]==bytes(18)
    else:
        total=struct.unpack_from('!H',reply,16)[0]
        assert checksum(reply[14:34])==0 and reply[26:34]==IP+REMOTE
        assert reply[34+total-20:]==bytes(n-total-14)
        data=reply[34:14+total]
        assert data[8:]==frame[42:14+total]
        if reply[23]==1: assert data[:2]==b'\0\0' and checksum(data)==0 and data[4:8]==frame[38:42]
        else:
            assert struct.unpack_from('!HH',data)==(7533,43210)
            assert checksum(IP+REMOTE+struct.pack('!BBH',0,17,len(data))+data)==0
    return reply


def driver(mode=1, bad_id=False, busy=False, corrupt=False, mdio_timeout=False):
    u,s=machine()
    u.reg_write(UC_ARM_REG_CPSR,0x13)
    for a,n in [(0x1f000000,0x210000),(0x297f0000,65536),(0x26200000,65536),(0x66400000,65536)]:u.mem_map(a,n)
    def word(a,v):u.mem_write(a,struct.pack('<I',v))
    word(0x297f0990,0x26400c0e);word(0x297f1990,0x26400c06)
    word(0x26200000,0x45544831);word(0x26200004,mode)
    regs={0:12 if busy else 0,8:0 if mdio_timeout else 4,0x14:0x1f98,0x34:0}
    mmio=[];packets=[];injected=[False];maximum=[0]
    phy={1:0 if bad_id else 0x786d,2:0,3:0,4:0x1e1,5:0x1e1}
    def read(uc,access,a,n,v,d):
        if 0x1f204000<=a<0x1f204300:
            off=((a-0x1f204000)//8)*4
            word(a,(regs.get(off,0)>> (16 if (a&4) else 0)) & 65535)
        elif a==0x1f201328:word(a,0x61 if packets else 0x60)
        elif a==0x66400000 and regs.get(0,0)&4 and not injected[0]:
            data=arp()+bytes(18)+bytes(4)
            u.mem_write(0x66401000,data)
            word(a,0x06401005 if corrupt else 0x06401001);word(a+4,len(data));injected[0]=True
    def write(uc,access,a,n,v,d):
        if 0x1f000000<=a<0x1f210000:
            assert uc.reg_read(UC_ARM_REG_CPSR)&255==0xd3
            mmio.append((a,n,v))
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
                    packets.append(bytes(u.mem_read(0x66404000,v)));regs[0x14]=0x1fb8
        elif 0x26330000<=a and a+n<=0x26340000:maximum[0]=max(maximum[0],0x26340000-a)
        else:
            assert (0x2800fff8<=a and a+n<=0x28010000 or
                    0x26200100<=a and a+n<=0x26200128 or
                    0x66400000<=a and a+n<=0x66404800),hex(a)
    u.hook_add(UC_HOOK_MEM_READ,read);u.hook_add(UC_HOOK_MEM_WRITE,write)
    # CPU geometry and MIU completion model; actual instruction primitives are
    # covered separately by check-cpu and QEMU. No whole-driver stubbing.
    def primitive(uc,a,n,name):
        r0=uc.reg_read(UC_ARM_REG_R0)
        value=0xc0181d if name=='leanboot_read_sctlr' else 0x1000 if name=='leanboot_load16' and r0==0x1f203140 else 0
        uc.reg_write(UC_ARM_REG_R0,value);uc.reg_write(UC_ARM_REG_PC,uc.reg_read(UC_ARM_REG_LR))
    for name in ['leanboot_read_sctlr','leanboot_read_clidr','leanboot_load16']:
        u.hook_add(UC_HOOK_CODE,primitive,name,begin=s[name],end=s[name])
    u.emu_start(BASE,STOP,count=20000000)
    assert u.reg_read(UC_ARM_REG_PC)==STOP
    result=u.reg_read(UC_ARM_REG_R0)
    expected=29 if busy else 30 if bad_id or mdio_timeout else 36 if corrupt and mode else 0
    assert result==expected,(result,expected)
    assert u.reg_read(UC_ARM_REG_SP)==0x28010000
    assert u.reg_read(UC_ARM_REG_CPSR)&255==0x13
    if busy: assert not mmio
    else: assert regs[0]&12==0
    if mode and not (busy or bad_id or corrupt or mdio_timeout):
        assert len(packets)==1 and packets[0]==core(arp())
    else:assert not packets
    recipe=[(a,n,v) for a,n,v in mmio if n==1 and a!=0x1f201300]
    return dict(mode=mode,bad_id=bad_id,busy=busy,corrupt=corrupt,mdio_timeout=mdio_timeout,result=result,stack_bytes=maximum[0],phy_writes=recipe)


def reference_phy():
    path=ROOT/'assets/backups/g13-v013-stock-20261031/MBOOT.img'
    if not path.exists():return None
    data=path.read_bytes()
    assert hashlib.sha256(data).hexdigest()=='643058879b84d044dea58a4572bd9ade1cc3140f3d3357eeaabae53be32ec382'
    u=Uc(UC_ARCH_ARM,UC_MODE_ARM)
    u.mem_map(0x273fd000,(len(data)+0xfff+0xc00)&~4095);u.mem_write(0x273fdc00,data)
    u.mem_map(0x28000000,65536);u.mem_map(0x1f000000,0x210000)
    u.reg_write(UC_ARM_REG_SP,0x28010000);u.reg_write(UC_ARM_REG_LR,0x28000000)
    writes=[]
    def record(uc,access,a,n,v,d):writes.append((a,n,v))
    u.hook_add(UC_HOOK_MEM_WRITE,record,begin=0x1f000000,end=0x1f210000)
    u.emu_start(0x2749fc18,0x28000000,count=100000)
    assert u.reg_read(UC_ARM_REG_PC)==0x28000000
    return writes


def main():
    core(arp()); count=1
    for n in [0,1,2,31,32,33,511,1472]:
        data=bytes(i%256 for i in range(n))
        for frame in [icmp(data),udp(data),udp(data,False)]:core(frame);count+=1
    for n in range(42):core(arp()[:n],False);count+=1
    for frame in [icmp(b'abc'),udp(b'abc')]:
        for offset in [0,12,14,16,20,24,26,30,36,38,41]:
            bad=bytearray(frame);bad[offset]^=0x80;core(bad,False);count+=1
    bad=bytearray(arp());bad[22]^=2;core(bad,False);count+=1
    for flags in [1,0x2000,0x8000]:core(ip_packet(1,icmp(b'a')[34:],flags),False);count+=1
    rng=random.Random(7533)
    for _ in range(100):
        frame=bytes(rng.randrange(256) for _ in range(rng.randrange(0,1600)))
        core(frame,False);count+=1
    cases=[driver(),driver(mode=0),driver(bad_id=True),driver(busy=True),driver(corrupt=True),driver(mdio_timeout=True)]
    reference=reference_phy()
    for case in cases:
        writes=case.pop('phy_writes')
        if not case['busy'] and reference is not None:assert writes==reference
    result=dict(packet_cases=count,driver_cases=cases,exact_mboot_phy_trace_matched=reference is not None,binary_sha256=hashlib.sha256((OUT/'g13-ethernet.bin').read_bytes()).hexdigest(),hardware_proven=False)
    (OUT/'ethernet-check.json').write_text(json.dumps(result,indent=2)+'\n')
    print(f'PASS: {count} packet/bounds cases and {len(cases)} compiled ARM driver cases')


if __name__=='__main__':main()
