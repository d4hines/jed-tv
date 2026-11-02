#!/usr/bin/env python3
"""Prepare an OFFLINE candidate config tree. Does not load modules or access TV."""
import pathlib,re,json,hashlib,shutil
O=pathlib.Path('assets/research/g06-display-offline');root=O/'candidate-root'
def source(target):
 if target.startswith('/vendor/tvconfig/config/'):return O/'tvconfig-config'/target.removeprefix('/vendor/tvconfig/')
 if target.startswith('/tclconfig/hardware/'):return O/'tclconfig-hardware'/target.removeprefix('/tclconfig/')
 return None
core=['/vendor/tvconfig/config/'+x for x in ['mik_init.ini','sys.ini','board.ini','MMAP_MI.h','OSD/OsdConfig.ini','OSD/OsdDefine.ini','PQ_general/OSDPQConfig.ini']]
queue=list(core)+['/vendor/tvconfig/config/Qos.bin','/vendor/tvconfig/config/QosCustomer.bin'];seen=set();missing=[];manifest=[];edges=[]
# Conservative transitive literal-path superset; not a proof each file is needed.
while queue:
 t=queue.pop(0)
 if t in seen:continue
 seen.add(t);p=source(t)
 if p is None or not p.exists():missing.append(t);continue
 if p.is_dir():
  queue += [t.rstrip('/')+'/'+str(x.relative_to(p)) for x in p.rglob('*') if x.is_file()];continue
 dst=root/t.lstrip('/');dst.parent.mkdir(parents=True,exist_ok=True);shutil.copyfile(p,dst)
 if p.suffix in ['.ini','.h']:
  for line in p.read_text(errors='replace').splitlines():
   if line.lstrip().startswith(('#','//')):continue
   m=re.search(r'=\s*[\"\']?(/[^\"\';#\s]+)',line)
   if m:
    ref=m[1].replace('//','/');edges.append({'from':t,'to':ref});queue.append(ref)
 manifest.append({'target':t,'source':str(p),'bytes':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest(),'tier':'bootstrap' if t in core else 'literal-reference-superset'})
p=root/'vendor/tvconfig/config/mik_init.ini';s=p.read_text();edits={'MI_DISP_INIT':1,'MI_OSD_INIT':1,'MI_TUNER_INIT':0}
for k,v in edits.items():s,n=re.subn(r'(?m)^('+k+r'\s*=\s*)\d+',lambda m:m[1]+str(v),s);assert n==1
p.write_text(s)
alias=root/'config'
if not alias.is_symlink():alias.symlink_to('vendor/tvconfig/config')
(O/'candidate-manifest.json').write_text(json.dumps({'status':'OFFLINE UNTESTED CANDIDATE, not minimality proof','edits':edits,'aliases':{'/config':'vendor/tvconfig/config'},'notes':['FLASH_INIT remains 1 for SYS flash environment reads','DISPOUT_INIT remains stock 0; cold panel init unproven'],'files':manifest,'literal_edges':edges,'missing_or_outside_extracted_trees':sorted(missing)},indent=2)+'\n')
print(len(manifest),'files;',sum(x['bytes'] for x in manifest),'source bytes;',len(missing),'unresolved literal paths')
print('\n'.join(sorted(missing)))
