#!/usr/bin/env python3
from analyze import ELF
import pathlib,struct,json,bisect
O=pathlib.Path('assets/research/g06-display-offline');e=ELF(O/'vendor/lib/modules/mik.ko');funcs=sorted((s[1],s[0]) for s in e.symbols() if s[-1]<len(e.ss) and e.sn[s[-1]]=='.text' and s[3]&15==2);starts=[a for a,n in funcs];out=[]
for i,s in enumerate(e.ss):
 if s[1]!=9 or e.sn[s[7]] not in ['.text','.init.text']:continue
 for addr,info in struct.iter_unpack('<II',e.data(i)):
  if info&255!=43:continue
  sym=e.syms[s[6]][info>>8]
  if not 0<sym[-1]<len(e.ss):continue
  v=struct.unpack_from('<I',e.data(s[7]),addr)[0];pos=sym[1]+((v>>4)&0xf000)+(v&0xfff);b=e.data(sym[-1]);raw=b[pos:pos+512].split(b'\0')[0]
  if not raw.startswith(b'/') or b'jenkins' in raw or not all(32<=c<127 for c in raw):continue
  name=funcs[bisect.bisect_right(starts,addr)-1][1] if e.sn[s[7]]=='.text' else 'init_module'
  out.append({'section':e.sn[s[7]],'address':hex(addr),'function':name,'path':raw.decode()})
(O/'kernel-path-references.json').write_text(json.dumps(out,indent=2)+'\n')
for x in out:
 if any(t in x['path'] for t in ['sys.ini','mik_init','MMAP_MI','Osd','OSDPQ','Qos']):print(x)
