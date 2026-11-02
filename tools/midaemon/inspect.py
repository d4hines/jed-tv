#!/usr/bin/env python3
"""Offline exact-V109 ELF evidence extraction; never accesses a device."""
import argparse, hashlib, importlib.util, json, pathlib, re, struct, subprocess
ROOT=pathlib.Path(__file__).resolve().parents[2]
spec=importlib.util.spec_from_file_location('elf_analysis',ROOT/'research/g06-display/analyze.py')
m=importlib.util.module_from_spec(spec);spec.loader.exec_module(m)
LLVM=pathlib.Path('/opt/homebrew/opt/llvm/bin')
def inspect(path):
 e=m.ELF(path)
 def at(addr,n=160):
  for i,s in enumerate(e.ss):
   if s[3]<=addr<s[3]+s[5] and s[1]!=8:return e.data(i)[addr-s[3]:addr-s[3]+n]
  return b''
 symbols={s[1]:s[0] for s in e.symbols() if s[0]}
 dis=subprocess.check_output([str(LLVM/'llvm-objdump'),'-d',str(path)],text=True)
 regs={};lines=[]
 for line in dis.splitlines():
  ins=re.match(r'\s*([0-9a-f]+):\s*[0-9a-f]+\s+(.+)',line)
  if ins:
   pc=int(ins[1],16);text=ins[2]
   load=re.match(r'ldr\s+(r\d+|lr), \[pc, #(-?0x[0-9a-f]+|-?\d+)\]',text)
   add=re.match(r'add\s+(r\d+|lr), pc, (r\d+|lr)',text)
   if load:
    data=at(pc+8+int(load[2],0),4)
    if len(data)==4:regs[load[1]]=struct.unpack('<I',data)[0]
   if add and add[2] in regs:
    addr=(regs[add[2]]+pc+8)&0xffffffff; regs[add[1]]=addr
    raw=at(addr).split(b'\0')[0]
    if raw and all(c in (9,10,13) or 32<=c<127 for c in raw):line+=' ; '+repr(raw.decode())
    elif addr in symbols:line+=' ; '+symbols[addr]
  lines.append(line)
 return '\n'.join(lines)+'\n'
def main():
 p=argparse.ArgumentParser();p.add_argument('--out',type=pathlib.Path,default=ROOT/'assets/research/midaemon-replacement');a=p.parse_args();a.out.mkdir(parents=True,exist_ok=True)
 base=ROOT/'assets/research/firmware-implications/tvservice'
 manifest={}
 for name,path in {'midaemon':base/'bin/midaemon','libmi':base/'glibc/libmi.so','libmi_server':base/'glibc/libmi_server.so'}.items():
  (a.out/(name+'.annotated.txt')).write_text(inspect(path))
  manifest[name]={'path':str(path.relative_to(ROOT)),'sha256':hashlib.sha256(path.read_bytes()).hexdigest()}
 (a.out/'sources.json').write_text(json.dumps(manifest,indent=2)+'\n')
if __name__=='__main__':main()
