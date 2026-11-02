#!/usr/bin/env python3
from analyze import ELF
import pathlib,json
O=pathlib.Path('assets/research/g06-display-offline');mods={p.name:ELF(p) for p in (O/'vendor/lib/modules').glob('*.ko')};providers={}
for n,e in mods.items():
 for s in e.exports():providers.setdefault(s,[]).append(n)
result={}
for n,e in mods.items():
 deps={};unresolved=[]
 for s in sorted(e.imports()):
  ps=[p for p in providers.get(s,[]) if p!=n]
  if ps:
   for p in ps:deps.setdefault(p,[]).append(s)
  else:unresolved.append(s)
 mi=e.data(e.sn.index('.modinfo')).decode().split('\0')
 result[n]={'sha256':__import__('hashlib').sha256(e.b).hexdigest(),'modinfo':[x for x in mi if x.startswith(('depends=','name=','vermagic='))],'providers':deps,'kernel_or_unprovided':unresolved}
(O/'module-dependencies.json').write_text(json.dumps(result,indent=2)+'\n')
for n,r in result.items():print(n,':',', '.join(p+' ('+str(len(ss))+')' for p,ss in r['providers'].items()))
