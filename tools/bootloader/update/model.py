"""Run the real ARM updater against two bounded, optionally persistent banks."""
import os
import struct
from pathlib import Path
from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_CODE, UC_HOOK_MEM_WRITE, UC_PROT_READ, UC_PROT_EXEC
from unicorn.arm_const import *

ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'assets/build/leanboot-update'
BASE, SOURCE, SCRATCH, STATE, PACKET, STACK, STOP = 0x1000000,0x20000000,0x21000000,0x23000000,0x23001000,0x24000000,0x25000000
CAP=2*1024*1024
BANK_SIZE=CAP+512


class PowerCut(Exception): pass


def bank_image(package):
    assert len(package)>=160
    raw=package[:160]+bytes(352)+package[160:]
    if len(raw)>BANK_SIZE: raise ValueError('Bank overflow')
    return bytearray(raw+bytes(BANK_SIZE-len(raw)))


class Machine:
    max_stack_bytes=0
    def __init__(self,banks=None):
        self.banks=banks if banks is not None else [bytearray(BANK_SIZE),bytearray(BANK_SIZE)]
        self.u=Uc(UC_ARCH_ARM,UC_MODE_ARM)
        code=(OUT/'updater-model.bin').read_bytes()
        self.u.mem_map(BASE,(len(code)+4095)&~4095)
        self.u.mem_write(BASE,code)
        self.u.mem_protect(BASE,(len(code)+4095)&~4095,UC_PROT_READ|UC_PROT_EXEC)
        for p,n in [(SOURCE,CAP+4096),(SCRATCH,CAP+4096),(STATE,12288),(STACK,65536),(STOP,4096)]: self.u.mem_map(p,n)
        with (OUT/'updater-model.elf').open('rb') as f:
            self.symbols={s.name:BASE+s['st_value'] for s in ELFFile(f).get_section_by_name('.symtab').iter_symbols()}
        self.writes=[];self.reads=[];self.flushes=0;self.cut_write=None;self.torn=0
        self.fail_read=None;self.corrupt_read=None;self.fail_flush=None;self.cut=False;self.allowed=[];self.stack_low=STACK+65536
        for name,kind in [('leanboot_model_read','read'),('leanboot_model_write','write'),('leanboot_model_flush','flush')]:
            address=self.symbols[name]
            self.u.hook_add(UC_HOOK_CODE,self.io,kind,begin=address,end=address)
        self.u.hook_add(UC_HOOK_MEM_WRITE,self.memory_write)

    def memory_write(self,u,access,address,size,value,data):
        assert any(a<=address and address+size<=b for a,b in self.allowed),(hex(address),size)
        if STACK<=address<STACK+65536:
            self.stack_low=min(self.stack_low,address)
            Machine.max_stack_bytes=max(Machine.max_stack_bytes,STACK+65536-address)

    def io(self,u,address,size,kind):
        ok=True
        if kind=='flush':
            self.flushes+=1;ok=self.flushes!=self.fail_flush
        else:
            b,s,p=[u.reg_read(r) for r in [UC_ARM_REG_R0,UC_ARM_REG_R1,UC_ARM_REG_R2]]
            assert b in (0,1) and 0<=s<BANK_SIZE//512,(b,s)
            assert SCRATCH<=p and p+512<=SCRATCH+CAP+4096,hex(p)
            if kind=='read':
                self.reads.append((b,s));ok=len(self.reads)!=self.fail_read
                if ok:
                    block=bytearray(self.banks[b][s*512:(s+1)*512])
                    if len(self.reads)==self.corrupt_read:block[0]^=1
                    u.mem_write(p,bytes(block))
            else:
                block=bytes(u.mem_read(p,512));self.writes.append((b,s))
                count=self.torn if len(self.writes)==self.cut_write else 512
                self.banks[b][s*512:s*512+count]=block[:count]
                if len(self.writes)==self.cut_write:
                    self.cut=True;u.emu_stop();return
        u.reg_write(UC_ARM_REG_R0,int(ok));u.reg_write(UC_ARM_REG_PC,u.reg_read(UC_ARM_REG_LR))

    def call(self,name,*args,receiver=False):
        self.allowed=[(STACK,STACK+65536),(SCRATCH,SCRATCH+CAP+4096)]
        if receiver:self.allowed += [(STATE,STATE+40),(STATE+8192,STATE+8212),(SOURCE,SOURCE+CAP+160)]
        for r in range(UC_ARM_REG_R0,UC_ARM_REG_R12+1):self.u.reg_write(r,0)
        sp=STACK+65536-32
        self.u.reg_write(UC_ARM_REG_CPSR,0x1d3);self.u.reg_write(UC_ARM_REG_SP,sp);self.u.reg_write(UC_ARM_REG_LR,STOP)
        for r,a in zip([UC_ARM_REG_R0,UC_ARM_REG_R1,UC_ARM_REG_R2,UC_ARM_REG_R3],args[:4]):self.u.reg_write(r,a)
        if len(args)>4:self.u.mem_write(sp,struct.pack('<'+'I'*(len(args)-4),*args[4:]))
        self.u.emu_start(self.symbols[name],STOP,count=100_000_000)
        if self.cut:raise PowerCut
        assert self.u.reg_read(UC_ARM_REG_PC)==STOP,('instruction budget',name)
        return self.u.reg_read(UC_ARM_REG_R0)|(self.u.reg_read(UC_ARM_REG_R1)<<32)

    def install(self,package):
        self.u.mem_write(SOURCE,package)
        return self.call('leanboot_update_install',SOURCE,len(package),SCRATCH,CAP+4096)&0xffffffff

    def version(self,bank):
        return self.call('leanboot_update_bank',bank,SCRATCH,CAP+4096)>>32

    def receive(self,packet):
        self.u.mem_write(PACKET,packet)
        return self.call('leanboot_update_receive',STATE,SOURCE,CAP+160,PACKET,len(packet),receiver=True)

    def serve(self,packet):
        self.u.mem_write(PACKET,packet)
        size=self.call('leanboot_update_serve',STATE,SOURCE,PACKET,len(packet),receiver=True)&0xffffffff
        assert size in (0,20)
        return bytes(self.u.mem_read(STATE+8192,size)) if size else b''

    def save(self,directory):
        directory=Path(directory);directory.mkdir(parents=True,exist_ok=True)
        for i,data in enumerate(self.banks):
            path=directory/f'bank{i}.bin'
            if path.exists() and path.read_bytes()==data:continue
            temporary=directory/f'.bank{i}.next'
            with temporary.open('wb') as stream:
                stream.write(data);stream.flush();os.fsync(stream.fileno())
            temporary.replace(path)
        descriptor=os.open(directory,os.O_RDONLY|os.O_DIRECTORY)
        try:os.fsync(descriptor)
        finally:os.close(descriptor)
