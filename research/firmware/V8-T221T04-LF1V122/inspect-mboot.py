"""Offline-only ARM disassembly helper; run with python3 + capstone."""
from pathlib import Path
import argparse, struct
from capstone import Cs, CS_ARCH_ARM, CS_MODE_ARM
from capstone.arm import ARM_OP_REG, ARM_OP_IMM, ARM_OP_MEM, ARM_REG_PC

B = (Path(__file__).resolve().parents[3]/'assets/research/firmware/V8-T221T04-LF1V122/mboot.bin').read_bytes()
DELTA = 0x27400000 - 0x2400
def word(a):
    o = a - DELTA
    return struct.unpack_from('<I', B, o)[0] if 0 <= o <= len(B)-4 else None
def string(a):
    o = a - DELTA
    if 0 <= o < len(B):
        s = B[o:o+240].split(b'\0', 1)[0]
        if s and all(32 <= c < 127 or c in (9,10,13) for c in s):
            return s.decode()
    return ''

p = argparse.ArgumentParser()
p.add_argument('--start', type=lambda s:int(s,0), default=0x27400000)
p.add_argument('--length', type=lambda s:int(s,0), default=0x249000)
p.add_argument('--xrefs', nargs='*')
a=p.parse_args()
c=Cs(CS_ARCH_ARM, CS_MODE_ARM); c.detail=True; c.skipdata=True
known={}
for i in c.disasm(B[a.start-DELTA:a.start-DELTA+a.length],a.start):
    if i.id == 0: continue
    ops=i.operands; note=''
    if i.mnemonic.startswith('ldr') and len(ops)==2 and ops[0].type==ARM_OP_REG and ops[1].type==ARM_OP_MEM and ops[1].mem.base==ARM_REG_PC:
        v=word(i.address+8+ops[1].mem.disp)
        if v is not None:
            known[ops[0].reg]=(v,i.address)
            note=f'literal {v:#x} {string(v)!r}'
    elif i.mnemonic.startswith('add') and len(ops)==3 and ops[0].type==ARM_OP_REG and ops[1].type==ARM_OP_REG and ops[1].reg==ARM_REG_PC and ops[2].type==ARM_OP_REG:
        val,at=known.get(ops[2].reg,(None,0))
        if val is not None and i.address-at<48:
            addr=(i.address+8+val)&0xffffffff
            s=string(addr)
            note=f'address {addr:#x} {s!r}'
            if a.xrefs is not None and any(x in s for x in a.xrefs):
                print(f'{i.address:#010x}: {i.mnemonic} {i.op_str} ; {note}')
            known[ops[0].reg]=(addr,i.address)
    if a.xrefs is None:
        print(f'{i.address:#010x}: {i.mnemonic:8s} {i.op_str:40s}'+(' ; '+note if note else ''))
