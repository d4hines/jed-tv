#!/usr/bin/env python3
"""Check actual Lean-generated seccomp programs with a bounded BPF interpreter.
Kernel enforcement and musl/thread compatibility are separately checked in QEMU.
"""
import os,pathlib,platform,struct,subprocess
ROOT=pathlib.Path(__file__).resolve().parents[3];out=ROOT/'assets/build/lean';out.mkdir(parents=True,exist_ok=True)
prefix=subprocess.check_output(['lean','--print-prefix'],cwd=ROOT/'src/pid1/lean',text=True).strip()
sources=[]
for name in ['Apps','Sandbox']:
 c=out/(name.lower()+'.c');sources.append(str(c))
 subprocess.run(['lake','env','lean','-c',str(c),f'TV/{name}.lean'],cwd=ROOT/'src/pid1/lean',check=True)
env=os.environ.copy();cc=env.get('CC','cc');flags=['-Wl,--gc-sections']
if platform.system()=='Darwin':
 flags=['-Wl,-dead_strip']
 if 'CC' not in env:cc='/Library/Developer/CommandLineTools/usr/bin/clang';env['SDKROOT']='/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk'
exe=out/'sandbox-driver'
subprocess.run([cc,'-O2','-DNDEBUG','-ffunction-sections','-fdata-sections','-I'+prefix+'/include',*sources,str(ROOT/'test/lean/tv/sandbox-driver.c'),*flags,'-o',str(exe)],check=True,env=env)
imports=subprocess.check_output(['nm','-u',str(exe)],text=True)
assert not any(s in imports for s in ['lean_','malloc','realloc','pthread','initialize_']),imports
ALLOW=0x7fff0000;DENY=0x50001;KILL=0x80000000;NOSYS=0x50026;checks=0

def run(program,nr,arch,*args):
 global checks
 checks+=1;data=struct.pack('<IIQ6Q',nr,arch,0,*((list(args)+[0]*6)[:6]));pc=0;acc=0
 for _ in range(len(program)+1):
  assert 0<=pc<len(program)
  word=program[pc];code=word&65535;jt=(word>>16)&255;jf=(word>>24)&255;k=word>>32;pc+=1
  if code==0x20:acc=struct.unpack_from('<I',data,k)[0]
  elif code==0x54:acc &= k
  elif code==0x15:pc += jt if acc==k else jf
  elif code==6:return k
  else:raise AssertionError(f'unsupported instruction {code}')
 raise AssertionError('BPF did not terminate')

for line in subprocess.check_output([str(exe)],text=True).splitlines():
 app,arch,uid,threads,nofile,n,*program=map(int,line.split());assert len(program)==n
 audit=0x40000028 if arch==0 else 0xc00000b7
 if app not in [1,2,3,5,6,7,8] or arch>1:
  assert uid==0 if app not in [1,2,3,5,6,7,8] else uid==10000+app
  assert run(program,0,audit)==KILL;continue
 assert uid==10000+app and nofile<1023 and threads=={1:32,2:8,3:1,5:1,6:4,7:4,8:1}[app]
 clone,execveat=(120,387) if arch==0 else (220,281)
 forbidden=[2,190,11] if arch==0 else [221]
 forbidden += [26,21,23,46,164,310,311,318] if arch==0 else [117,40,146,144,142,97,104,268]
 for nr in forbidden:assert run(program,nr,audit)==DENY,(app,arch,nr)
 assert run(program,435,audit)==NOSYS
 assert run(program,clone,audit,0x7d0f00)==ALLOW
 assert run(program,clone,audit,0x3d0f00)==ALLOW
 # No process-producing clone or namespace flag can pass. Test every single
 # bit modification of the normal pthread flags and every low 16-bit flag set.
 for flags in range(65536):assert run(program,clone,audit,flags)==DENY
 for bit in range(32):
  flags=0x7d0f00^(1<<bit)
  expected=ALLOW if flags&~0x7d0f00==0 and flags&0x10900==0x10900 else DENY
  assert run(program,clone,audit,flags)==expected
 for fd in [0,3,255,256,1022,1023,1024,0xffffffff]:
  assert run(program,execveat,audit,fd)==(ALLOW if fd==1023 else DENY)
 for nr in ([93,194] if arch==0 else [46]):
  assert run(program,nr,audit)==ALLOW, (app,arch,nr)
 assert run(program,clone,audit^1,0x7d0f00)==KILL
 # socket, connect, bind, with every grant on: YouTube and Doom have local
 # network sockets; the rest have none.
 for nr in ([281,283,282] if arch==0 else [198,203,200]):
  assert run(program,nr,audit)==(ALLOW if app in [1,2] else DENY),(app,arch,nr)
 # recvmsg, sendto, recvfrom, setsockopt: also YouTube, which uses the sockets
 # PID 1 passes it. Infinite RPG and the rest have no networking at all.
 for nr in ([297,290,292,294] if arch==0 else [212,206,207,208]):
  assert run(program,nr,audit)==(ALLOW if app in [1,2] else DENY),(app,arch,nr)
print(f'PASS: Lean confinement; {checks} BPF executions; app identities, process/exec denial, thread flags and launch descriptor; no Lean runtime imports')
