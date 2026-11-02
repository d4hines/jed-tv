"""Execute the pinned V109 ARM supplicant dispatcher with fake OS services.

Only firmware instructions implement dispatch, parameter validation, SHM list
management and TA loading. Hooks supply inert memory/files and capture ioctls;
no host TEE, devices, network, mounts or firmware writes are used.
"""
import hashlib
import io
import re
import struct
from pathlib import Path
from elftools.elf.elffile import ELFFile
from unicorn import Uc, UC_ARCH_ARM, UC_MODE_THUMB, UC_HOOK_CODE
from unicorn.arm_const import UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3, UC_ARM_REG_SP, UC_ARM_REG_LR, UC_ARM_REG_PC, UC_ARM_REG_C1_C0_2, UC_ARM_REG_FPEXC

FIRMWARE_SHA256 = 'c36e55f67c472e7e7435a29d5d3361148e713ec84b3439930fd08f97339803ef'
REGS = [UC_ARM_REG_R0, UC_ARM_REG_R1, UC_ARM_REG_R2, UC_ARM_REG_R3]

def packet(function, params):
    return struct.pack('<II', function, len(params)) + b''.join(struct.pack('<4Q', *p) for p in params)

class Stock:
    def __init__(self, binary, files):
        data = Path(binary).read_bytes()
        assert hashlib.sha256(data).hexdigest() == FIRMWARE_SHA256
        elf = ELFFile(io.BytesIO(data))
        self.cpu = Uc(UC_ARCH_ARM, UC_MODE_THUMB)
        self.cpu.mem_map(0, 0x20000)
        self.cpu.mem_map(0x200000, 0x600000)
        for segment in elf.iter_segments():
            if segment['p_type'] == 'PT_LOAD':
                self.cpu.mem_write(segment['p_vaddr'], segment.data())
        self.cpu.reg_write(UC_ARM_REG_C1_C0_2, 0xf << 20)
        self.cpu.reg_write(UC_ARM_REG_FPEXC, 0x40000000)
        symbols = elf.get_section_by_name('.dynsym')
        self.imports = {0x1650 + 12*i: symbols.get_symbol(r['r_info_sym']).name
                        for i, r in enumerate(elf.get_section_by_name('.rel.plt').iter_relocations())}
        self.files = files
        self.handles = {}
        self.next_handle = 100
        self.next_address = 0x300000
        self.next_id = 17
        self.regions = {}
        self.events = []
        # One other receive worker remains pending, so metadata processing
        # exercises dispatch without needing to create another OS thread.
        self.write32(0x21000c, 1)
        self.cpu.hook_add(UC_HOOK_CODE, self.instruction)

    def read32(self, address): return struct.unpack('<I', self.cpu.mem_read(address, 4))[0]
    def write32(self, address, value): self.cpu.mem_write(address, struct.pack('<I', value & 0xffffffff))
    def cstr(self, address):
        result = bytearray()
        while len(result) < 4096:
            byte = self.cpu.mem_read(address + len(result), 1)[0]
            if not byte: return result.decode('ascii')
            result.append(byte)
        raise AssertionError('unterminated firmware string')
    def alloc(self, size):
        assert 0 <= size <= 0x100000
        address = self.next_address
        self.next_address += (size + 0xfff) & ~0xfff
        assert self.next_address < 0x780000
        self.cpu.mem_write(address, bytes(size))
        return address
    def result(self, value):
        self.cpu.reg_write(UC_ARM_REG_R0, value & 0xffffffff)
        self.cpu.reg_write(UC_ARM_REG_PC, self.cpu.reg_read(UC_ARM_REG_LR))

    def instruction(self, cpu, address, size, unused):
        if address == 0x200000:
            cpu.emu_stop(); return
        # Single-thread execution does not model pthread scheduling/locks.
        if address in (0x2ff4, 0x3050):
            self.result(0); return
        name = self.imports.get(address)
        if name is None: return
        a,b,c,d = [cpu.reg_read(r) for r in REGS]
        sp = cpu.reg_read(UC_ARM_REG_SP)
        if name in ('__aeabi_memclr8', '__aeabi_memclr'):
            cpu.mem_write(a, bytes(b)); result = 0
        elif name == 'ioctl':
            if b == 0x8010a406:
                ptr, capacity = struct.unpack('<QQ', cpu.mem_read(c, 16))
                self.events.append({'ioctl':'recv', 'capacity':capacity, 'slots':self.read32(ptr+4), 'meta':self.read32(ptr+8)})
                assert len(self.incoming) <= capacity
                cpu.mem_write(ptr, self.incoming); result = 0
            elif b == 0x8010a407:
                ptr, length = struct.unpack('<QQ', cpu.mem_read(c, 16))
                self.outgoing = bytes(cpu.mem_read(ptr, length)); result = 0
            elif b == 0xc010a401:
                size,flags,_ = struct.unpack('<QIi', cpu.mem_read(c,16))
                assert flags == 0
                ident = self.next_id; self.next_id += 1
                fd = ident + 1000
                self.regions[ident] = {'size':size,'fd':fd,'address':None}
                self.write32(c+12,ident); result = fd
            else: raise AssertionError(f'unmodeled ioctl {b:#x}')
        elif name == 'calloc': result = self.alloc(a*b)
        elif name == 'mmap':
            fd = self.read32(sp)
            entry = next(v for v in self.regions.values() if v['fd']==fd)
            assert b == entry['size'] and c == 3 and d == 1 and self.read32(sp+4)==0
            result = self.alloc(b); entry['address'] = result
        elif name == 'munmap':
            assert any(v['address']==a and v['size']==b for v in self.regions.values()); result = 0
        elif name == 'close':
            for ident,region in list(self.regions.items()):
                if region['fd']==a: del self.regions[ident]
            result=0
        elif name in ('free','_dprintf','puts'): result=0
        elif name == 'strcmp': result = 0 if self.cstr(a)==self.cstr(b) else 1
        elif name == '__vsnprintf_chk':
            fmt = self.cstr(self.read32(sp)); va = self.read32(sp+4); index=0
            def replace(m):
                nonlocal index
                value=self.read32(va+4*index); index+=1
                spec=m.group()[1:]
                return self.cstr(value) if spec=='s' else format(value,spec)
            text = re.sub(r'%0?\d*[sux]',replace,fmt).encode()
            assert len(text)<b
            cpu.mem_write(a,text+b'\0'); result=len(text)
        elif name == 'fopen':
            path=self.cstr(a); filename=path.rsplit('/',1)[-1]
            self.events.append({'open':filename})
            if filename not in self.files: result=0
            else:
                result=self.next_handle; self.next_handle+=1
                self.handles[result]={'data':self.files[filename],'position':0}
        elif name == 'fseek':
            h=self.handles[a]; assert b == 0 and c in (0, 2)
            h['position'] = 0 if c == 0 else len(h['data'])
            result=0
        elif name == 'ftell': result=self.handles[a]['position']
        elif name == 'fread':
            h=self.handles[d]; raw=h['data'][h['position']:h['position']+b*c]
            cpu.mem_write(a,raw); h['position']+=len(raw); result=len(raw)//b
        elif name == 'fclose': del self.handles[a]; result=0
        else: raise AssertionError(f'unmodeled imported call {name} at {address:#x}')
        self.result(result)

    def request(self, raw):
        assert len(raw)<=168, 'stock dispatcher advertises five parameters'
        self.incoming=raw; self.outgoing=None
        self.cpu.reg_write(UC_ARM_REG_SP,0x2ff000)
        self.cpu.reg_write(UC_ARM_REG_LR,0x200001)
        self.cpu.reg_write(UC_ARM_REG_R0,0x210000)
        self.cpu.emu_start(0x2931,0,count=200000)
        assert self.cpu.reg_read(UC_ARM_REG_PC)==0x200000, 'firmware did not return'
        return self.outgoing

    def snapshots(self):
        return {str(k):hashlib.sha256(bytes(self.cpu.mem_read(v['address'],v['size']))).hexdigest()
                for k,v in self.regions.items()}


def check_kernel_bounds(binary):
    """Execute the stock kernel's exact 64-bit RECV/SEND length guards."""
    from unicorn import UC_MODE_ARM
    data = Path(binary).read_bytes()
    elf = ELFFile(io.BytesIO(data))
    text = elf.get_section_by_name('.text')
    rows = []
    for name, start in [('recv', 0xc0890234), ('send', 0xc08904ac)]:
        offset = start - text['sh_addr']
        code = text.data()[offset:offset+20]
        words = struct.unpack('<5I', code)
        assert words[:4] == (0xe2500008, 0xe2c11000, 0xe2700ffe, 0xe2f10000)
        for length in [0, 7, 8, 168, 1000, 1024, 1025, (1<<32)+1000, (1<<64)-1]:
            cpu=Uc(UC_ARCH_ARM,UC_MODE_ARM)
            cpu.mem_map(0xc0890000,0x2000)
            cpu.mem_write(start,code)
            cpu.reg_write(UC_ARM_REG_R0,length & 0xffffffff)
            cpu.reg_write(UC_ARM_REG_R1,length >> 32)
            cpu.emu_start(start,0,count=5)
            accepted=cpu.reg_read(UC_ARM_REG_PC)==start+20
            assert accepted == (8 <= length <= 1024)
            rows.append({'operation':name,'bytes':length,'accepted':accepted})
    return {'elf_sha256':hashlib.sha256(data).hexdigest(),'cases':rows}
