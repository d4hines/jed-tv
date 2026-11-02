#!/usr/bin/env python3
"""Build a returnable G13 RAM-storage updater; no device access."""
import sys
sys.dont_write_bytecode = True
import argparse
import hashlib
import json
import os
import shutil
import subprocess
from pathlib import Path
import build as core

OUT = core.ROOT/'assets/build/leanboot-update-tv'

def sha(path): return hashlib.sha256(path.read_bytes()).hexdigest()

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--public-key',required=True,type=Path)
    args=p.parse_args()
    record=core.OUT/'build.json'
    old=json.loads(record.read_text()) if record.exists() else {}
    if old.get('public_key_sha256')!=sha(args.public_key) or any(
        sha(core.ROOT/name)!=digest for name,digest in old.get('source_sha256',{}).items()):
        core.build(args.public_key)
        old=json.loads(record.read_text())
    OUT.mkdir(parents=True,exist_ok=True)
    oleans=OUT/'lean'
    shutil.copytree(core.OUT/'lean',oleans,dirs_exist_ok=True)
    env=dict(os.environ,LEAN_PATH=str(oleans))
    prefix=subprocess.check_output(['lean','--print-prefix'],text=True).strip()
    modules=['Cpu','EntryCheck','Ethernet','EmacConfig','Unsafe/Emac','Unsafe/UpdateRam','UpdateRamEntry']
    objects=[p for p in core.OUT.glob('*.o') if p.name not in ('Unsafe-UpdateModel.o','update-model.o')]
    for module in modules:
        target=oleans/(module+'.olean');target.parent.mkdir(parents=True,exist_ok=True)
        c=OUT/(module.replace('/','-')+'.c');obj=c.with_suffix('.o')
        core.run('lean','-Dcompiler.extract_closed=false','-o',target,'-c',c,module+'.lean',cwd=core.SRC,env=env)
        core.run('clang',*old['compiler_flags'],'-I'+prefix+'/include','-c',c,'-o',obj)
        objects.append(obj)
    elf=OUT/'g13-update-ram.elf'
    core.run('ld.lld','--gc-sections','--no-undefined','-T',core.SRC/'ethernet.ld',
             '--undefined=leanboot_ram_respond','--undefined=leanboot_ram_initialize',
             '--undefined=leanboot_ram_version',*objects,'-o',elf)
    binary=OUT/'g13-update-ram.bin'
    core.run('llvm-objcopy','-O','binary',elf,binary)
    assert binary.stat().st_size <= 128*1024
    sources=dict(old['source_sha256'])
    sources.update({str((core.SRC/(m+'.lean')).relative_to(core.ROOT)):sha(core.SRC/(m+'.lean')) for m in modules})
    for path in [core.SRC/'ethernet.ld',core.HOST/'Unsafe/Arm.lean',Path(__file__)]:
        sources[str(path.relative_to(core.ROOT))]=sha(path)
    result={'bytes':binary.stat().st_size,'binary_sha256':sha(binary),'source_sha256':sources,
            'public_key_sha256':sha(args.public_key),'bank_capacity':32768,
            'storage':'RAM only','new_assembly':0,'load_address':'0x26000000'}
    (OUT/'build.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps({k:v for k,v in result.items() if k!='source_sha256'}))

if __name__=='__main__': main()
