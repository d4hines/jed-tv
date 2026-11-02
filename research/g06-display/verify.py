#!/usr/bin/env python3
"""Offline consistency checks for evidence, module ABI CRCs and prepared pack."""
from analyze import ELF
import pathlib,json,struct,hashlib
O=pathlib.Path('assets/research/g06-display-offline');d=json.loads((O/'module-dependencies.json').read_text());modules={p.name:ELF(p) for p in (O/'vendor/lib/modules').glob('*.ko')};checks=[];missing=[]
for n,e in modules.items():
 versions={}
 if '__versions' in e.sn:
  raw=e.data(e.sn.index('__versions'))
  assert len(raw)%64==0
  for pos in range(0,len(raw),64):versions[e.string(raw,pos+4)]=struct.unpack_from('<I',raw,pos)[0]
 for provider,symbols in d[n]['providers'].items():
  crc={s[0][6:]:s[1] for s in modules[provider].symbols() if s[0].startswith('__crc_') and s[-1]==65521}
  for sym in symbols:
   if sym not in versions:
    missing.append((n,provider,sym));continue
   assert versions[sym]==crc[sym],(n,provider,sym,'CRC mismatch')
   checks.append((n,provider,sym))
seen=set()
def visit(n):
 if n in seen:return
 seen.add(n)
 for p in d[n]['providers']:visit(p)
visit('mstar_fbdev_mi.ko')
assert len(seen)==10,seen
pack=json.loads((O/'candidate-manifest.json').read_text());changed=[]
for x in pack['files']:
 p=O/'candidate-root'/x['target'].lstrip('/');h=hashlib.sha256(p.read_bytes()).hexdigest()
 if h!=x['sha256']:changed.append(x['target'])
assert changed==['/vendor/tvconfig/config/mik_init.ini'],changed
assert (O/'candidate-root/config').resolve()==(O/'candidate-root/vendor/tvconfig/config').resolve()
r={'module_provider_crc_matches':len(checks),'imports_without_crc_record':missing,'fb_module_link_closure':sorted(seen),'candidate_files':len(pack['files']),'candidate_changed_files':changed,'hardware_tested':False}
(O/'verification.json').write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r,indent=2))
