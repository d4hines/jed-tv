from elftools.elf.elffile import ELFFile
from pathlib import Path
from capstone import *
import io,sys,struct
base=Path(__file__).resolve().parents[3]/'assets/research/firmware/V8-T221T04-LF1V122/recovery-ramdisk/system'
p=base/sys.argv[1];b=p.read_bytes();e=ELFFile(io.BytesIO(b));sym={}; sizes={}
for pp in [p,Path(str(p)+'.debug')]:
 if not pp.exists():continue
 ee=ELFFile(io.BytesIO(pp.read_bytes()))
 for sec in ee.iter_sections():
  if sec['sh_type'] in ('SHT_SYMTAB','SHT_DYNSYM'):
   for s in sec.iter_symbols():
    if s['st_value']:sym[s['st_value']&~1]=s.name;sizes[s['st_value']&~1]=s['st_size']
rel=e.get_section_by_name('.rel.plt');ds=e.get_section_by_name('.dynsym');plt=e.get_section_by_name('.plt')
if rel:
 # LLVM ARM PLT has 32-byte header and 16-byte entries here.
 for i,r in enumerate(rel.iter_relocations()):sym[plt['sh_addr']+32+16*i]=ds.get_symbol(r['r_info_sym']).name+'@plt'
def get(addr,size):
 for seg in e.iter_segments():
  if seg['p_type']=='PT_LOAD' and seg['p_vaddr']<=addr<seg['p_vaddr']+seg['p_filesz']:
   off=addr-seg['p_vaddr']+seg['p_offset'];return b[off:off+size]
 return b''
for addr in sys.argv[2:]:
 if ':' in addr:start,size=[int(x,0) for x in addr.split(':')]
 else:start=int(addr,0);size=sizes[start]
 print('\nFUNCTION',sym.get(start),hex(start),size)
 regs={}
 md=Cs(CS_ARCH_ARM,CS_MODE_THUMB);md.skipdata=True
 for ins in md.disasm(get(start,size),start):
  note=''
  if ins.mnemonic.startswith('bl') and ins.op_str.startswith('#'):note=sym.get(int(ins.op_str[1:],16),'')
  if ins.mnemonic=='ldr' and '[pc, #' in ins.op_str:
   reg=ins.op_str.split(',')[0];imm=int(ins.op_str.split('#')[1].split(']')[0],0);v=get(((ins.address+4)&~3)+imm,4)
   if len(v)==4:regs[reg]=struct.unpack('<I',v)[0]
  if ins.mnemonic=='add' and ins.op_str.endswith(', pc'):
   reg=ins.op_str.split(',')[0]
   if reg in regs:
    ptr=(regs[reg]+ins.address+4)&0xffffffff;raw=get(ptr,160).split(b'\0')[0]
    if raw and all(9<=c<127 for c in raw):note+=f' STR {ptr:x} {raw!r}'
  print(f'{ins.address:08x}: {ins.mnemonic:8} {ins.op_str:35} {note}')
