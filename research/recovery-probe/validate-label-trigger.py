"""Offline emulation of V122 MBOOT label copying and Android option parsing.
No device access. Hardware/FAT discovery and simple libc routines are stubbed;
label-copy, BCB-building and getopt_long code execute from official binaries.
"""
from pathlib import Path
import struct,io,json
from unicorn import *
from unicorn.arm_const import *
from elftools.elf.elffile import ELFFile
ROOT=Path(__file__).resolve().parents[2]
BASE=ROOT/'assets/research/firmware/V8-T221T04-LF1V122'
LABEL=b'\n--fast\n   '
def u32(x):return struct.pack('<I',x)
def word(u,a):return struct.unpack('<I',u.mem_read(a,4))[0]
def cstr(u,a):
 if not a:return b''
 out=bytearray()
 for i in range(4096):
  c=u.mem_read(a+i,1)[0]
  if not c:return bytes(out)
  out.append(c)
 raise RuntimeError('Unterminated string')
def ret(u,value):
 u.reg_write(UC_ARM_REG_R0,value&0xffffffff);u.reg_write(UC_ARM_REG_PC,u.reg_read(UC_ARM_REG_LR))
def simple(u,kind):
 a=u.reg_read(UC_ARM_REG_R0);b=u.reg_read(UC_ARM_REG_R1);n=u.reg_read(UC_ARM_REG_R2)
 if kind=='memset':u.mem_write(a,bytes([b&255])*n);ret(u,a)
 elif kind=='strlen':ret(u,len(cstr(u,a)))
 elif kind=='strncpy':u.mem_write(a,cstr(u,b)[:n].ljust(n,b'\0'));ret(u,a)
 elif kind=='strncat':u.mem_write(a+len(cstr(u,a)),cstr(u,b)[:n]+b'\0');ret(u,a)
 elif kind=='strchr':
  s=cstr(u,a)+b'\0';i=s.find(bytes([b&255]));ret(u,a+i if i>=0 else 0)
 elif kind=='strncmp':
  x=cstr(u,a)[:n];y=cstr(u,b)[:n];ret(u,(x>y)-(x<y))
 elif kind=='getenv':ret(u,0)
 else:raise ValueError(kind)
# MBOOT is ARM, loaded at its linked address.
u=Uc(UC_ARCH_ARM,UC_MODE_ARM);u.mem_map(0x27400000,0x1000000)
b=(BASE/'mboot.bin').read_bytes();u.mem_write(0x27400000,b[0x2400:])
u.mem_map(0x10000000,0x20000)
entry=0x10001000;out=0x10002000;serial=0x10003000;fp=0x10010000
u.mem_write(entry,LABEL+b'\x08'+bytes(20));u.mem_write(serial+0x100,bytes.fromhex('f421cb6e'))
u.mem_write(fp-0x2c,u32(serial+0x100-3))
for reg,val in [(UC_ARM_REG_R3,entry+32),(UC_ARM_REG_R6,entry),(UC_ARM_REG_R5,out),(UC_ARM_REG_R10,serial),(UC_ARM_REG_FP,fp),(UC_ARM_REG_SP,fp-0x1000)]:u.reg_write(reg,val)
hooks={0x274c7670:'strncmp',0x274c75b4:'strncpy',0x274c76f0:'strlen',0x274c7600:'strncat',0x274c78f4:'memset'}
def mhook(u,pc,size,data):
 if pc in hooks:simple(u,hooks[pc])
u.hook_add(UC_HOOK_CODE,mhook)
u.emu_start(0x2743b698,0x2743b804,count=10000)
actual_label=cstr(u,out);assert actual_label==LABEL[:8],repr(actual_label)
# Emulate the actual BCB string assembly with the extracted label and UUID.
u.mem_write(fp-0x19c,actual_label+b'\0');u.mem_write(fp-0x29c,b'6ecb-21f4\0')
u.mem_write(fp-0x11c,b'--update_package=/storage/6ecb-21f4//V8-T221T04-LF1V109.000732.zip\n\0')
u.emu_start(0x274917e0,0x27491918,count=10000)
bcb=cstr(u,fp-0xb5c);args=[x for x in bcb.split(b'\n') if x]
assert args[0]==b'recovery' and args[3]==b'--fast',args
print('MBOOT output label:',repr(actual_label));print('BCB arguments:',[x.decode() for x in args])
# Extract the exact main() OPTIONS table from the TCL recovery ELF.
rb=(BASE/'recovery-ramdisk/system/bin/recovery').read_bytes();re=ELFFile(io.BytesIO(rb))
def rget(addr,n):
 seg=next(s for s in re.iter_segments() if s['p_type']=='PT_LOAD' and s['p_vaddr']<=addr<s['p_vaddr']+s['p_filesz'])
 off=addr-seg['p_vaddr']+seg['p_offset'];return rb[off:off+n]
table=(struct.unpack('<I',rget(0x143d4,4))[0]+0x14192)&0xffffffff
options=[]
for i in range(20):
 name,has_arg,flag,val=struct.unpack('<4I',rget(table+i*16,16))
 if not name:break
 assert flag==0
 options.append((rget(name,100).split(b'\0')[0],has_arg,val))
print('Actual recovery OPTIONS:',[(a.decode(),b,c) for a,b,c in options])
# Execute actual ARM libc getopt_long; relocate the ELF in an isolated emulator.
lb=(BASE/'recovery-ramdisk/system/lib/libc.so').read_bytes();le=ELFFile(io.BytesIO(lb));load=0x40000000
v=Uc(UC_ARCH_ARM,UC_MODE_ARM);v.mem_map(load,0x200000)
for seg in le.iter_segments():
 if seg['p_type']=='PT_LOAD':v.mem_write(load+seg['p_vaddr'],seg.data())
ds=le.get_section_by_name('.dynsym')
for rn in ['.rel.dyn','.rel.plt']:
 for rel in le.get_section_by_name(rn).iter_relocations():
  addr=load+rel['r_offset'];typ=rel['r_info_type'];s=ds.get_symbol(rel['r_info_sym']);a=word(v,addr)
  if typ==23:val=load+a
  elif typ in (21,22):val=load+s['st_value'] if s['st_value'] else 0
  elif typ==2:val=load+s['st_value']+a
  elif typ in (19,160):continue # TLS and IFUNC relocations unused by this parser path
  else:raise RuntimeError(('unknown relocation',typ))
  v.mem_write(addr,u32(val&0xffffffff))
syms={s.name:s['st_value'] for s in le.get_section_by_name('.symtab').iter_symbols() if s['st_value']}
v.mem_map(0x50000000,0x20000);arena=0x50001000
v.reg_write(UC_ARM_REG_SP,0x5001f000);v.reg_write(UC_ARM_REG_LR,0x50000000)
v.reg_write(UC_ARM_REG_C1_C0_2,0xf00000);v.reg_write(UC_ARM_REG_FPEXC,0x40000000)
v.mem_write(load+syms['opterr'],u32(0));v.mem_write(load+syms['optind'],u32(1))
argv=arena;cursor=arena+0x200
for i,a in enumerate(args):v.mem_write(argv+4*i,u32(cursor));v.mem_write(cursor,a+b'\0');cursor+=len(a)+1
v.mem_write(argv+len(args)*4,u32(0));opts=arena+0x1000;cursor=arena+0x1200
for i,(name,has_arg,val) in enumerate(options):
 v.mem_write(cursor,name+b'\0');v.mem_write(opts+16*i,struct.pack('<4I',cursor,has_arg,0,val));cursor+=len(name)+1
optstr=arena+0x1800;idx=arena+0x1810
plt=le.get_section_by_name('.plt');imports={}
for i,rel in enumerate(le.get_section_by_name('.rel.plt').iter_relocations()):imports[load+plt['sh_addr']+32+16*i]=ds.get_symbol(rel['r_info_sym']).name
calls=[]
def chook(v,pc,size,data):
 if pc in imports:
  name=imports[pc];calls.append(name)
  if name not in ['getenv','strchr','strncmp','strlen']:raise RuntimeError(('unstubbed libc call',hex(pc),name))
  simple(v,name)
v.hook_add(UC_HOOK_CODE,chook)
parsed=[]
for _ in range(20):
 for reg,val in [(UC_ARM_REG_R0,len(args)),(UC_ARM_REG_R1,argv),(UC_ARM_REG_R2,optstr),(UC_ARM_REG_R3,opts),(UC_ARM_REG_SP,0x5001f000),(UC_ARM_REG_LR,0x50000000)]:v.reg_write(reg,val)
 v.mem_write(0x5001f000,u32(idx))
 try:v.emu_start(load+syms['getopt_long'],0x50000000,count=100000)
 except Exception:
  print('PC',hex(v.reg_read(UC_ARM_REG_PC)),'calls',calls);raise
 result=v.reg_read(UC_ARM_REG_R0)
 if result==0xffffffff:break
 if result==0:parsed.append(options[word(v,idx)][0].decode())
 else:parsed.append(result)
else:raise RuntimeError('getopt did not terminate')
assert 'fastboot' in parsed,parsed
result={'firmware':'V122 (not installed V109)','label_hex':LABEL.hex(),'mboot_label_hex':actual_label.hex(),'bcb_args':[a.decode() for a in args],'getopt_results':parsed,'emulated':['MBOOT label-copy path','MBOOT BCB assembly','TCL recovery libc getopt_long'],'limitations':['FAT discovery and simple libc routines stubbed','Main dispatch statically inspected, not booted','Not tested on physical TV']}
destination=ROOT/'assets/research/recovery-probe/label-trigger-validation.json'
destination.parent.mkdir(parents=True,exist_ok=True)
destination.write_text(json.dumps(result,indent=2)+'\n')
print('PASS: actual V122 getopt_long selected fastboot:',parsed)
