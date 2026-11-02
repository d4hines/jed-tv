#!/usr/bin/env python3
"""Check executable Lean C ABI and compatibility with the original menu."""
import itertools,os,pathlib,platform,subprocess
ROOT=pathlib.Path(__file__).resolve().parents[3]
out=ROOT/'assets/build/lean';out.mkdir(parents=True,exist_ok=True)
subprocess.run(['lake','build','TV.Menu','TV.Supervisor','TV.Apps','TV.Sandbox','TV.Settings','TV.Proofs','TV.Core','TV.CoreProofs'],cwd=ROOT/'src/pid1/lean',check=True)
c=out/'menu.c'
subprocess.run(['lake','env','lean','-c',str(c),'TV/Menu.lean'],cwd=ROOT/'src/pid1/lean',check=True)
prefix=subprocess.check_output(['lean','--print-prefix'],cwd=ROOT/'src/pid1/lean',text=True).strip()
env=os.environ.copy();cc=env.get('CC','cc');flags=['-Wl,--gc-sections']
if platform.system()=='Darwin':
 flags=['-Wl,-dead_strip']
 if 'CC' not in env:
  cc='/Library/Developer/CommandLineTools/usr/bin/clang';env['SDKROOT']='/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk'
exe=out/'menu-driver'
subprocess.run([cc,'-O2','-DNDEBUG','-ffunction-sections','-fdata-sections','-I'+prefix+'/include',str(c),str(ROOT/'test/lean/tv/driver.c'),*flags,'-o',str(exe)],check=True,env=env)
imports=subprocess.check_output(['nm','-u',str(exe)],text=True)
assert not any(s in imports for s in ['lean_','malloc','realloc','pthread','initialize_']),imports

def pack(row,cols):return row|sum(v<<(8+16*i) for i,v in enumerate(cols))
def reference(row,cols,widths,key,repeated):
 cols=list(cols);effect=0
 if key==0:cols[row]=max(0,cols[row]-1)
 elif key==1:cols[row]=min(widths[row]-1,cols[row]+1)
 elif key==2:row=max(0,row-1)
 elif key==3:row=min(len(widths)-1,row+1)
 elif not repeated:effect={4:1,5:2,6:3}.get(key,0)
 return pack(row,cols)|(effect<<56)
cases=[];expected=[]
def case(row,cols,widths,key,repeated):
 cases.append(f'{pack(row,cols)} {pack(len(widths),widths)} {key} {repeated}\n')
 expected.append(reference(row,cols,widths,key,repeated))
for rows in range(1,4):
 for widths in itertools.product(range(1,6),repeat=rows):
  for cols in itertools.product(*(range(w) for w in widths)):
   for row in range(rows):
    for key in range(9):
     for repeated in [0,1]:case(row,cols,widths,key,repeated)
for cols in itertools.product([0,65533,65534],repeat=3):
 for row in range(3):
  for key in range(9):
   for repeated in [0,1,2,0xffffffff]:case(row,cols,[65535]*3,key,repeated)
p=subprocess.run([str(exe)],input=''.join(cases),text=True,capture_output=True,check=True)
actual=list(map(int,p.stdout.splitlines()));assert len(actual)==len(expected)
for i,(a,b) in enumerate(zip(actual,expected)):
 if a!=b:raise AssertionError(f'{cases[i].strip()}: Lean {a} != original {b}')
print(f'PASS: Lean menu; {len(cases)} C-ABI differential cases; no Lean runtime or allocator imports')

subprocess.run(['python3',str(ROOT/'test/lean/tv/check-supervisor.py')],check=True)
subprocess.run(['python3',str(ROOT/'test/lean/tv/check-sandbox.py')],check=True)
