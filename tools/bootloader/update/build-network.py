#!/usr/bin/env python3
"""Build the signed network-boot RAM entry and a returnable Lean payload."""
import sys
sys.dont_write_bytecode=True
import argparse
import hashlib
import json
import os
import importlib.util
from pathlib import Path
import shutil
import subprocess
import build as core

OUT=core.ROOT/'assets/build/leanboot-network'
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--public-key',type=Path,required=True)
    args=p.parse_args();record=core.OUT/'build.json'
    old=json.loads(record.read_text()) if record.exists() else {}
    if old.get('public_key_sha256')!=sha(args.public_key) or any(sha(core.ROOT/n)!=v for n,v in old.get('source_sha256',{}).items()):
        core.build(args.public_key);old=json.loads(record.read_text())
    OUT.mkdir(parents=True,exist_ok=True);oleans=OUT/'lean'
    standard=core.ROOT/'assets/build/leanboot'
    shutil.copytree(standard/'lean',oleans,dirs_exist_ok=True)
    shutil.copytree(core.OUT/'lean',oleans,dirs_exist_ok=True)
    env=dict(os.environ,LEAN_PATH=str(oleans))
    prefix=subprocess.check_output(['lean','--print-prefix'],text=True).strip()
    modules=['Cpu','EntryCheck','Ethernet','Ethernet/Descriptor','EmacConfig','Unsafe/Emac','Unsafe/Flash','NetworkBoot/Plan','NetworkBoot/Runtime','NetworkBoot/Receiver','Unsafe/Network','NetworkBoot','NetworkBootEntry','NetworkExample','Update/Handoff','NetworkGate']
    objects=[p for p in core.OUT.glob('*.o') if p.name not in ('Unsafe-UpdateModel.o','update-model.o') and not p.name.startswith('Hacl_')]
    shared=['images','fdt','bootconfig','plan','verifiedboot','os','bootpolicy','bootjournal-plan',
            'bootjournal','lz4','unsafe-bootstrap','bootbundle','bootgate']
    objects += [standard/(name+'.o') for name in shared]
    for module in modules:
        target=oleans/(module+'.olean');target.parent.mkdir(parents=True,exist_ok=True)
        c=OUT/(module.replace('/','-')+'.c');obj=c.with_suffix('.o')
        core.run('lean','-Dcompiler.extract_closed=false','-o',target,'-c',c,module+'.lean',cwd=core.SRC,env=env)
        core.run('clang',*old['compiler_flags'],'-I'+prefix+'/include','-c',c,'-o',obj);objects.append(obj)
    harness=core.HOST/'Unsafe/FlashHarness.lean'
    c=OUT/'flash-harness.c';harness_obj=OUT/'flash-harness.o'
    core.run('lean','-Dcompiler.extract_closed=false','-c',c,harness,cwd=core.HOST,env=env)
    core.run('clang',*old['compiler_flags'],'-I'+prefix+'/include','-c',c,'-o',harness_obj)
    vendor=core.OUT/'hacl'
    includes=[core.SRC/'Unsafe/update/include',core.OUT,vendor/'dist/portable-gcc-compatible',
              vendor/'dist/karamel/include',vendor/'dist/karamel/krmllib/dist/minimal']
    for name in ['Hacl_Ed25519','Hacl_Curve25519_51','Hacl_Hash_SHA2']:
        obj=OUT/(name+'.o')
        core.run('clang',*old['compiler_flags'],'-Os',*['-I'+str(p) for p in includes],
                 '-DKRML_VERIFIED_UINT128','-DKRML_HOST_TIME=leanboot_forbidden_time',
                 '-c',vendor/('dist/portable-gcc-compatible/'+name+'.c'),'-o',obj);objects.append(obj)
    for name,script in [('network-ram','ethernet.ld'),('network-example','network-example.ld'),('network-gate','gate.ld'),('flash-model','ethernet.ld')]:
        elf=OUT/(name+'.elf')
        roots=['--undefined=leanboot_network_window'] if name=='network-ram' else []
        extra=[]
        if name=='flash-model':
            roots=['--undefined='+s for s in ['leanboot_flash_install','leanboot_flash_version','leanboot_flash_transfer','leanboot_flash_frame','leanboot_flash_handoff']]
            extra=[harness_obj]
        core.run('ld.lld','--gc-sections','--no-undefined','-T',core.SRC/script,*roots,*objects,*extra,'-o',elf)
        core.run('llvm-objcopy','-O','binary',elf,OUT/(name+'.bin'))
    sources=dict(old['source_sha256'])
    standard_sources=json.loads((standard/'build.json').read_text())['source_sha256']
    for module in ['Images','Fdt','Bootconfig','Plan','VerifiedBoot','Os','BootPolicy','BootJournal/Plan',
                   'BootJournal','Lz4','Unsafe/Bootstrap','BootBundle','BootGate','Stage0']:
        path=core.SRC/(module+'.lean')
        assert sha(path)==standard_sources[module+'.lean'],'Rebuild the standard bootloader first'
        sources[str(path.relative_to(core.ROOT))]=sha(path)
    for path in [*[core.SRC/(m+'.lean') for m in modules],core.HOST/'Unsafe/Arm.lean',Path(__file__),core.SRC/'ethernet.ld',core.SRC/'network-example.ld',core.SRC/'gate.ld',core.SRC/'stage0.ld',core.HOST/'build-boot-image.py']:
        sources[str(path.relative_to(core.ROOT))]=sha(path)
    sources[str(harness.relative_to(core.ROOT))]=sha(harness)
    result={'source_sha256':sources,'public_key_sha256':sha(args.public_key),'crypto_optimization':'-Os',
            'outputs':{name:{'bytes':(OUT/(name+'.bin')).stat().st_size,'sha256':sha(OUT/(name+'.bin'))}
                       for name in ['network-ram','network-example','network-gate','flash-model']}}
    (OUT/'build.json').write_text(json.dumps(result,indent=2)+'\n');print(json.dumps(result['outputs']))
    sys.path.append(str(core.HOST))
    spec=importlib.util.spec_from_file_location('image_builder',core.HOST/'build-boot-image.py')
    builder=importlib.util.module_from_spec(spec);spec.loader.exec_module(builder)
    builder.build((OUT/'network-gate.bin').read_bytes(),OUT)
    shutil.copyfile(standard/'g13-stage0.bin',OUT/'g13-stage0.bin')
    shutil.copyfile(standard/'g13-stage0.elf',OUT/'g13-stage0.elf')
    shutil.copyfile(core.OUT/'entry.o',OUT/'entry.o')

if __name__=='__main__':main()
