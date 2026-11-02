#!/usr/bin/env python3
"""End-to-end ARM execution of Lean OS preparation using exact G13 images."""
import hashlib
import json
import struct
import time

from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_ARM, UC_HOOK_MEM_WRITE, UC_HOOK_CODE
from unicorn.arm_const import *
from os_profile import profile, decode_fdt, OUT


def run(mode,secure_status=0,invalid=None):
    p = profile()
    control=bytearray(p['control']);struct.pack_into('<I',control,8,mode)
    if invalid=='length':struct.pack_into('<I',control,44,1)
    elif invalid=='config':control[struct.unpack_from('<I',control,36)[0]+struct.unpack_from('<I',control,40)[0]-1]=0
    elif invalid=='digest':control[struct.unpack_from('<I',control,48)[0]]=ord('g')
    code = (OUT/'g13-os.bin').read_bytes()
    u = Uc(UC_ARCH_ARM, UC_MODE_ARM)
    spans = [(0x20207000, p['boot']), (0x23000000, bytes(65536)),
             (0x24000000, p['vendor'] + bytes(0x1400000-len(p['vendor']))),
             (0x25400000, p['init']), (0x26000000, code),
             (0x26200000, bytes(control) + bytes(65536-len(control))),
             (0x26320000,bytes(4096)), (0x1f201000,bytes(4096)),
             (0x26330000,bytes(65536)),
             (0x28000000, bytes(0x20000)), (0x27f00000, bytes(4096))]
    for address, data in spans:
        u.mem_map(address, (len(data)+4095)&~4095)
        u.mem_write(address, data)
    u.mem_write(0x1f201328,b'\x60\x00\x00\x00')
    with (OUT/'g13-os.elf').open('rb') as f:
        elf = ELFFile(f)
        symbols={s.name:0x26000000+s['st_value'] for s in elf.get_section_by_name('.symtab').iter_symbols()}
        entry=symbols['leanboot_os_start']
    # Lean erases the effect permission from internal calls. The exported
    # adapters are deliberately not retained in this production image.
    for label, name in (('clean_caches','cleanCaches'),('flush_miu','flushMiu'),
                        ('enter_linux','enterLinux')):
        symbols['leanboot_'+label]=symbols['l_LeanBoot_Cpu_'+name+'___redArg']
    # Protect original kernel/init, control data and code against any writes.
    stack_use=[0];output_writes=[0];uart=bytearray()
    def writes(uc, access, address, size, value, unused):
        if address==0x1f201300:
            assert size==1
            uart.append(value & 255)
            return
        assert any(a <= address and address+size <= a+n for a,n in
                   [(0x23000000,65536),(0x24000000,0x1400000),(0x26320000,132),
                    (0x26330000,65536),(0x2800fff8,8)]), hex(address)
        if 0x26330000<=address<0x26340000:stack_use[0]=max(stack_use[0],0x26340000-address)
        elif address<0x2800fff8:output_writes[0]+=1
    hook = u.hook_add(UC_HOOK_MEM_WRITE, writes)
    events=[]
    names=('leanboot_read_sctlr','leanboot_clean_caches',
           'leanboot_flush_miu','leanboot_send_verified_boot','leanboot_enter_linux')
    handlers={symbols[n]:n for n in names}
    expected_packet=(OUT/'verified-boot-packet.bin').read_bytes()
    def primitive(uc,address,size,data):
        name=handlers[address];result=0
        if name=='leanboot_read_sctlr':result=0xc0181d
        else:
            events.append(name)
            if name=='leanboot_send_verified_boot':
                assert u.reg_read(UC_ARM_REG_R0)==0x26320000
                assert bytes(u.mem_read(0x26320000,132))==expected_packet
                result=secure_status
            elif name=='leanboot_enter_linux':
                assert u.reg_read(UC_ARM_REG_R0)==0x20208000
                assert u.reg_read(UC_ARM_REG_R1)==0x23000000
                u.emu_stop();return
        u.reg_write(UC_ARM_REG_R0,result);u.reg_write(UC_ARM_REG_PC,u.reg_read(UC_ARM_REG_LR))
    for address in handlers:u.hook_add(UC_HOOK_CODE,primitive,begin=address,end=address)
    started = time.monotonic()
    u.reg_write(UC_ARM_REG_CPSR, 0x1d3)
    u.reg_write(UC_ARM_REG_SP, 0x28010000)
    u.reg_write(UC_ARM_REG_LR, 0x27f00000)
    u.emu_start(entry, 0x27f00000, count=500000000)
    if invalid:
        assert u.reg_read(UC_ARM_REG_PC)==0x27f00000
        assert u.reg_read(UC_ARM_REG_R0)==(24 if invalid=='digest' else 13)
        assert u.reg_read(UC_ARM_REG_SP)==0x28010000
        assert not events and output_writes[0]==0
        report={'invalid':invalid,'no_output_writes':True,'no_hardware_commit':True,'stack_bytes':stack_use[0]}
        print('PASS:',report)
        return report
    if mode==1 and secure_status==0:
        assert u.reg_read(UC_ARM_REG_PC)==symbols['leanboot_enter_linux']
    else:
        assert u.reg_read(UC_ARM_REG_PC) == 0x27f00000, hex(u.reg_read(UC_ARM_REG_PC))
        assert u.reg_read(UC_ARM_REG_R0) == (25 if secure_status else 0),u.reg_read(UC_ARM_REG_R0)
        assert u.reg_read(UC_ARM_REG_SP) == 0x28010000
    expected_events=[] if mode==0 else ['leanboot_clean_caches','leanboot_flush_miu','leanboot_send_verified_boot']
    if mode==1 and secure_status==0:expected_events.append('leanboot_enter_linux')
    assert events==expected_events,events
    assert bytes(u.mem_read(0x26320000,132))==expected_packet
    dtb = bytes(u.mem_read(0x23000000,65536))
    props, nodes, reservations = decode_fdt(dtb)
    assert props == p['expected_properties']
    assert reservations == [(0x24000000,p['initrd_length'])]
    raw = p['vendor'][4096:4096+18128651]+p['init'][4096:4096+2262498]
    padded = p['bootconfig'] + bytes(p['initrd_length']-len(raw)-20-len(p['bootconfig']))
    expected = raw+padded+struct.pack('<II',len(padded),sum(padded))+b'#BOOTCONFIG\n'
    ramdisk = bytes(u.mem_read(0x24000000,p['initrd_length']))
    assert ramdisk == expected
    assert bytes(u.mem_read(0x20208000,15871916)) == p['boot'][4096:4096+15871916]
    (OUT/'lean-prepared.dtb').write_bytes(dtb[:struct.unpack_from('>I',dtb,4)[0]])
    report = {'mode':mode,'secure_status':secure_status,'seconds':round(time.monotonic()-started,2),'properties':len(props),
              'initrd_length':len(ramdisk),'initrd_sha256':hashlib.sha256(ramdisk).hexdigest(),
              'dtb_sha256':hashlib.sha256((OUT/'lean-prepared.dtb').read_bytes()).hexdigest(),
              'kernel_unchanged':True,'booted_linux':False,'events':events,'uart':uart.decode(),
              'private_stack_bytes':stack_use[0]}
    print('PASS:',report)
    return report


def main():
    report=[run(0),run(1),run(1,0xffffffff),*[run(0,invalid=x) for x in ('length','config','digest')]]
    (OUT/'os-check.json').write_text(json.dumps(report,indent=2)+'\n')


if __name__ == '__main__': main()
