#!/usr/bin/env python3
"""Executable state/ABI checks: supervision, Linux wait observations and statuses."""
import os,pathlib,platform,random,subprocess
ROOT=pathlib.Path(__file__).resolve().parents[3];out=ROOT/'assets/build/lean';out.mkdir(parents=True,exist_ok=True)
c=out/'supervisor.c'
subprocess.run(['lake','env','lean','-c',str(c),'TV/Supervisor.lean'],cwd=ROOT/'src/pid1/lean',check=True)
prefix=subprocess.check_output(['lean','--print-prefix'],cwd=ROOT/'src/pid1/lean',text=True).strip()
env=os.environ.copy();cc=env.get('CC','cc');flags=['-Wl,--gc-sections']
if platform.system()=='Darwin':
 flags=['-Wl,-dead_strip']
 if 'CC' not in env:cc='/Library/Developer/CommandLineTools/usr/bin/clang';env['SDKROOT']='/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk'
exe=out/'supervisor-driver'
subprocess.run([cc,'-O2','-DNDEBUG','-ffunction-sections','-fdata-sections','-I'+prefix+'/include',str(c),str(ROOT/'test/lean/tv/supervisor-driver.c'),*flags,'-o',str(exe)],check=True,env=env)
imports=subprocess.check_output(['nm','-u',str(exe)],text=True)
assert not any(s in imports for s in ['lean_','malloc','realloc','pthread','initialize_']),imports
cases=[];expected=[];grace=1_500_000_000;maximum=2**64-1

def transition(phase,deadline,now,event,repeated):
 command=0
 if phase in [4,5]:pass
 elif event==4:phase,command=4,4
 elif event==5:phase,command=5,5
 elif event==0 and phase==0:phase=1
 elif event==2 and not repeated:command=3
 elif event==1 and not repeated and phase==1:phase,command=2,1
 elif event==3 and phase==2 and now>deadline:phase,command=3,2
 if command==1:deadline=min(maximum,now+grace)
 elif phase!=2:deadline=0
 return phase,deadline,command

def case(op,*args,result):
 values=[*args]+[0]*(5-len(args));cases.append(' '.join(map(str,[op,*values]))+'\n');expected.append(result)
def step(phase,deadline,now,event,repeated):
 p,d,cmd=transition(phase,deadline,now,event,repeated)
 case(0,phase,deadline,now,event,repeated,result=(p|(cmd<<32),d));return p,d,cmd
for phase in range(6):
 for deadline in [0,grace,2**32+grace,2**63,maximum-500]:
  for now in sorted(set([0,max(0,deadline-1),deadline,min(maximum,deadline+1),2**32,2**63,maximum])):
   for event in range(8):
    for repeated in [0,1,2]:step(phase,deadline,now,event,repeated)
# Long traces check exactly one TERM/KILL, no deadline rearming on Home,
# and no commands after completion even when input/ticks keep arriving.
# Explicit temporal scenarios, independent of the differential matrix.
for base in [0,2**32,2**63]:
 p,d,cmd=step(0,0,base,0,0);assert (p,d,cmd)==(1,0,0)
 p,d,cmd=step(p,d,base,1,0);assert (p,d,cmd)==(2,base+grace,1)
 p,d,cmd=step(p,d,base+100,1,0);assert (p,d,cmd)==(2,base+grace,0)
 p,d,cmd=step(p,d,base+grace,3,0);assert (p,d,cmd)==(2,base+grace,0)
 p,d,cmd=step(p,d,base+grace+1,3,0);assert (p,d,cmd)==(3,0,2)
 p,d,cmd=step(p,d,base+grace+2,1,0);assert (p,d,cmd)==(3,0,0)
 p,d,cmd=step(p,d,base+grace+3,4,0);assert (p,d,cmd)==(4,0,4)
 p,d,cmd=step(p,d,base+grace+4,2,0);assert (p,d,cmd)==(4,0,0)
 # Exit before the timer wins; no late KILL after the target was reaped.
 p,d,cmd=step(2,base+grace,base+10,4,0);assert (p,d,cmd)==(4,0,4)
 p,d,cmd=step(p,d,base+grace+1,3,0);assert (p,d,cmd)==(4,0,0)
rng=random.Random(20261101)
for trial in range(100):
 phase,deadline,now=0,0,2**32+trial
 phase,deadline,_=step(phase,deadline,now,0,0)
 terms=kills=0
 for i in range(100):
  now+=rng.randrange(1,100_000_000)
  event=rng.choice([1,1,2,3,3,3,4,5,6,7]);repeated=rng.randrange(3)
  phase,deadline,command=step(phase,deadline,now,event,repeated)
  terms+=command==1;kills+=command==2
  assert terms<=1 and kills<=1
for pid in [0,1,42,2**31-1,2**31,2**32-1]:
 case(1,pid,result=(0 if pid==0 else 2 if pid>=2**31 else 1,0))
for child in [1,42,2**31-1]:
 for observed in [0,1,42,99,2**31-1,2**31,2**32-1]:
  for error in [0,4,10,11,22]:
   result=1 if observed==child else 0 if observed==0 else 2 if observed<2**31 or error==4 else 3
   case(2,child,observed,error,result=(result,0))
for status in [*range(65536),2**32-1,0x1234007f]:
 signal=status&127
 kind,detail=(0,(status>>8)&255) if signal==0 else (3,status) if signal==127 else (1,signal) if signal in [9,15] else (2,signal)
 case(3,status,result=(detail|(kind<<32),0))
result=subprocess.run([str(exe)],input=''.join(cases),text=True,capture_output=True,check=True)
actual=[tuple(map(int,l.split())) for l in result.stdout.splitlines()];assert len(actual)==len(expected)
for i,(a,b) in enumerate(zip(actual,expected)):
 if a!=b:raise AssertionError(f'{cases[i].strip()}: Lean {a} != reference {b}')
print(f'PASS: Lean supervisor; {len(cases)} transition/wait/status ABI cases; no Lean runtime or allocator imports')
