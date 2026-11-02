#!/usr/bin/env python3
"""Check host LZ4 blocks against the exact G13 MBOOT ARM decoder, offline."""
import hashlib
import json
from pathlib import Path
import random
import struct

from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_MEM_READ, UC_HOOK_MEM_WRITE
from unicorn.arm_const import UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3, UC_ARM_REG_SP, UC_ARM_REG_LR, UC_ARM_REG_PC
from tv_session import pack

ROOT=Path(__file__).resolve().parents[2]
BASE=0x273fdc00
SHA='643058879b84d044dea58a4572bd9ade1cc3140f3d3357eeaabae53be32ec382'


def main():
    firmware=(ROOT/'assets/backups/g13-v013-stock-20261031/MBOOT.img').read_bytes()
    assert hashlib.sha256(firmware).hexdigest()==SHA
    rng=random.Random(1729)
    cases=[bytes(n) for n in (1,4,15,64,1024,65536)]
    cases += [rng.randbytes(n) for n in (15,1024,16384,65536)]
    cases += [bytes(range(256))*256,b'LEANBOOT serial transfer\n'*1024]
    cases += [(ROOT/'assets/build/leanboot'/name).read_bytes() for name in ('g13-os.bin','os-control.bin')]
    report=[]
    for raw in cases:
        data=pack(raw)
        uc=Uc(UC_ARCH_ARM,UC_MODE_ARM)
        page=BASE&~4095
        uc.mem_map(page,(len(firmware)+BASE-page+4095)&~4095)
        uc.mem_write(BASE,firmware)
        src,dst,stack,stop=0x1000000,0x2000000,0x3000000,0x4000000
        for addr in (src,dst,stack,stop):uc.mem_map(addr,0x20000)
        uc.mem_write(src,data+bytes(16))
        uc.mem_write(stack,struct.pack('<I',len(data)))
        sp=stack+0x10000
        def memory(emu,access,addr,size,value,context):
            # Stock wild-copy decoder may read up to seven bytes past a block.
            valid=[(src,src+len(data)+16),(dst,dst+len(raw)),
                   (stack,stack+4),(sp-512,sp)]
            if context=='read':valid.append((BASE,BASE+len(firmware)))
            else:valid=valid[1:]
            assert any(lo<=addr and addr+size<=hi for lo,hi in valid),(context,hex(addr),size)
        uc.hook_add(UC_HOOK_MEM_READ,memory,'read')
        uc.hook_add(UC_HOOK_MEM_WRITE,memory,'write')
        for reg,value in ((UC_ARM_REG_R0,src),(UC_ARM_REG_R1,stack),
                          (UC_ARM_REG_R2,dst),(UC_ARM_REG_R3,len(raw)),
                          (UC_ARM_REG_SP,sp),(UC_ARM_REG_LR,stop)):
            uc.reg_write(reg,value)
        uc.emu_start(0x274436a4,stop,count=2000000)
        assert uc.reg_read(UC_ARM_REG_PC)==stop
        assert uc.reg_read(UC_ARM_REG_R0)==0
        assert struct.unpack('<I',uc.mem_read(stack,4))[0]==len(data)
        assert uc.mem_read(dst,len(raw))==raw
        report.append({'bytes':len(raw),'compressed_bytes':len(data),'sha256':hashlib.sha256(raw).hexdigest()})
    out=ROOT/'assets/build/leanboot/transfer-check.json'
    out.write_text(json.dumps({'firmware_sha256':SHA,'cases':report},indent=2)+'\n')
    print(f'PASS: {len(cases)} host LZ4 blocks decoded by exact MBOOT ARM; read/write bounds checked')


if __name__=='__main__':main()
