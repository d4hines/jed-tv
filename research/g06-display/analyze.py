#!/usr/bin/env python3
"""Offline ELF32 ARM evidence helpers. No device access or target execution."""
import struct, pathlib, subprocess, json, hashlib, argparse, functools
LLVM='/opt/homebrew/opt/llvm/bin/'
class ELF:
 def __init__(self,p):
  self.path=pathlib.Path(p); self.b=self.path.read_bytes()
  assert self.b[:6]==b'\x7fELF\x01\x01'
  h=struct.unpack_from('<16sHHIIIIIHHHHHH',self.b); off,ents,n,si=h[6],h[11],h[12],h[13]
  self.ss=[struct.unpack_from('<IIIIIIIIII',self.b,off+i*ents) for i in range(n)]
  self.names=self.data(si); self.sn=[self.string(self.names,s[0]) for s in self.ss]
  self.syms={}
  for i,s in enumerate(self.ss):
   if s[1] in (2,11):
    st=self.data(s[6]); self.syms[i]=[(self.string(st,t[0]),*t[1:]) for t in struct.iter_unpack('<IIIBBH',self.data(i))]
 @functools.cache
 def data(self,i):
  s=self.ss[i];return self.b[s[4]:s[4]+s[5]]
 @staticmethod
 def string(b,i):return b[i:b.find(b'\0',i)].decode('utf8','replace')
 def symbols(self):return next(v for k,v in self.syms.items() if self.ss[k][1]==2)
 def exports(self):
  return {s[0].removeprefix('__crc_') for s in self.symbols() if s[0].startswith('__crc_') and s[-1]==65521}
 def imports(self):return {s[0] for s in self.symbols() if s[-1]==0 and s[0] and s[-3]>>4!=2}
 def annotate(self,functions):
  out=subprocess.check_output([LLVM+'llvm-objdump','-dr',str(self.path)],text=True)
  chosen=set(functions.split(','))
  ranges=[(self.sn[x[-1]],x[1],x[1]+x[2]) for x in self.symbols() if x[0] in chosen and x[-1]<len(self.ss)]
  # ARM REL relocations store the addend in the instruction or data word.
  refs={}
  for i,s in enumerate(self.ss):
   if s[1]!=9:continue
   syms=self.syms[s[6]]; target=s[7]
   for addr,info in struct.iter_unpack('<II',self.data(i)):
    sym=syms[info>>8]; typ=info&255
    if typ not in (2,43,44) or sym[-1]>=len(self.ss) or not sym[-1]:continue
    v=struct.unpack_from('<I',self.data(target),addr)[0]
    add=v if typ==2 else ((v>>4)&0xf000)|(v&0xfff)
    if typ==44:add<<=16
    pos=sym[1]+add; sec=self.data(sym[-1]); secname=self.sn[sym[-1]]
    if pos<len(sec) and ('rodata' in secname or 'data' in secname):
     raw=sec[pos:pos+180].split(b'\0')[0]
     if raw and all(x in (9,10,13) or 32<=x<127 for x in raw):refs[(self.sn[target],addr)]=repr(raw.decode())
  current=''
  lines=[]
  keep=False
  import re
  for l in out.splitlines():
   if l.startswith('Disassembly of section '):
    current=l.split('section ')[1].rstrip(':');keep=False
    lines.append(l)
   m=re.match(r'\s*([0-9a-f]+):',l)
   if m:keep=any(sec==current and start<=int(m[1],16)<end for sec,start,end in ranges)
   lab=re.match(r'^([0-9a-f]+) <',l)
   if lab:keep=any(sec==current and start<=int(lab[1],16)<end for sec,start,end in ranges)
   if not keep:continue
   if m and (current,int(m[1],16)) in refs:l+='  ; '+refs[(current,int(m[1],16))]
   lines.append(l)
  return '\n'.join(lines)+'\n'
def main():
 ap=argparse.ArgumentParser();ap.add_argument('elf');ap.add_argument('--functions');a=ap.parse_args();e=ELF(a.elf)
 if a.functions:print(e.annotate(a.functions),end='')
 else: print(json.dumps({'sha256':hashlib.sha256(e.b).hexdigest(),'exports':sorted(e.exports()),'imports':sorted(e.imports())},indent=2))
if __name__=='__main__':main()
