#!/usr/bin/env python3
"""Build the actual Lean updater + pinned crypto for ARM, without TV access."""
import sys
sys.dont_write_bytecode = True
import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).parent
SRC = ROOT / 'src/bootloader'
HOST = ROOT / 'tools/bootloader'
OUT = ROOT / 'assets/build/leanboot-update'


def run(*args, **kw):
    subprocess.run(list(map(str, args)), check=True, **kw)


def build(public_key):
    key = Path(public_key).read_bytes()
    if len(key) != 32 or not any(key):
        raise ValueError('A 32-byte raw trusted Ed25519 public key is required')
    OUT.mkdir(parents=True, exist_ok=True)
    spec = importlib.util.spec_from_file_location('fetch', HERE/'fetch-crypto.py')
    fetch = importlib.util.module_from_spec(spec); spec.loader.exec_module(fetch)
    vendor = fetch.fetch()
    (OUT/'update-key.h').write_text('static const uint8_t leanboot_update_key[32] = {' + ','.join(map(str,key)) + '};\n')
    oleans = OUT/'lean'; oleans.mkdir(exist_ok=True)
    env = dict(os.environ, LEAN_PATH=str(oleans))
    prefix = subprocess.check_output(['lean','--print-prefix'],text=True).strip()
    flags = ['--target=armv7a-none-eabi','-marm','-mfloat-abi=soft','-O2','-DNDEBUG',
             '-ffreestanding','-fno-builtin','-fno-stack-protector','-fno-unwind-tables',
             '-fno-asynchronous-unwind-tables','-fpie','-ffunction-sections','-fdata-sections']
    modules = ['Types','Flow','Unsafe/Effects','Unsafe/Primitives','MemoryAccess','Unsafe/Ram','Memory',
               'Update/Policy','Update/FlashPlan','Update/Access','Update/Manifest','Update/Install','Update/Flash',
               'Update/Receiver/Plan','Update/Receiver','Update/Server','Update/CryptoBounds','Unsafe/UpdateCrypto','Unsafe/UpdateModel']
    objects=[]
    for module in modules:
        target = oleans/(module+'.olean'); target.parent.mkdir(parents=True,exist_ok=True)
        c = OUT/(module.replace('/','-')+'.c'); o=c.with_suffix('.o')
        run('lean','-Dcompiler.extract_closed=false','-o',target,'-c',c,module+'.lean',
            cwd=HOST if module.endswith('UpdateModel') else SRC,env=env)
        run('clang',*flags,'-I'+prefix+'/include','-c',c,'-o',o)
        objects.append(o)
    # Reuse only existing ZrAsm memory primitives. No new handwritten assembly.
    for name in ['Isa','Print','Dsl']:
        (oleans/'ZrAsm').mkdir(exist_ok=True)
        run('lean','-o',oleans/f'ZrAsm/{name}.olean',f'ZrAsm/{name}.lean',cwd=HOST/'vendor',env=env)
    assembly = subprocess.check_output(['lean','--run','Unsafe/Arm.lean'],cwd=HOST,env=env)
    (OUT/'entry.S').write_bytes(assembly)
    run('clang',*flags,'-c',OUT/'entry.S','-o',OUT/'entry.o'); objects.append(OUT/'entry.o')
    includes = [SRC/'Unsafe/update/include', OUT, vendor/'dist/portable-gcc-compatible',
                vendor/'dist/karamel/include',vendor/'dist/karamel/krmllib/dist/minimal']
    for path in [vendor/'dist/portable-gcc-compatible/Hacl_Ed25519.c',
                 vendor/'dist/portable-gcc-compatible/Hacl_Curve25519_51.c',
                 vendor/'dist/portable-gcc-compatible/Hacl_Hash_SHA2.c',
                 SRC/'Unsafe/update/crypto.c', HOST/'Unsafe/update-model.c']:
        obj=OUT/(path.stem+'.o')
        run('clang',*flags,*['-I'+str(p) for p in includes],'-DKRML_VERIFIED_UINT128',
            '-DKRML_HOST_TIME=leanboot_forbidden_time',
            '-c',path,'-o',obj)
        objects.append(obj)
    elf=OUT/'updater-model.elf'
    run('ld.lld','--gc-sections','--no-undefined','-T',HERE/'model.ld',
        '--undefined=leanboot_update_install','--undefined=leanboot_update_bank',
        '--undefined=leanboot_update_receive','--undefined=leanboot_update_serve',*objects,'-o',elf)
    run('llvm-objcopy','-O','binary',elf,OUT/'updater-model.bin')
    undefined=subprocess.check_output(['llvm-nm','--undefined-only',elf],text=True)
    if undefined.strip(): raise RuntimeError(undefined)
    report={'public_key_sha256':hashlib.sha256(key).hexdigest(),
            'binary_sha256':hashlib.sha256((OUT/'updater-model.bin').read_bytes()).hexdigest(),
            'bytes':(OUT/'updater-model.bin').stat().st_size,
            'hardware_backend':False,'authored_new_arm_instructions':0,
            'crypto_revision':json.loads((HERE/'crypto-lock.json').read_text())['revision']}
    sources = [SRC/(m+'.lean') if m!='Unsafe/UpdateModel' else HOST/(m+'.lean') for m in modules]
    sources += list((SRC/'Unsafe/update').rglob('*')) + [HOST/'Unsafe/update-model.c', HERE/'model.ld', HERE/'crypto-lock.json']
    report['source_sha256'] = {str(p.relative_to(ROOT)):hashlib.sha256(p.read_bytes()).hexdigest()
                               for p in sorted(sources) if p.is_file()}
    report['compiler_flags'] = flags
    (OUT/'build.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(report,indent=2))


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--public-key',required=True,type=Path,help='trusted raw 32-byte Ed25519 public key')
    build(p.parse_args().public_key)
