#!/usr/bin/env python3
"""Differential check: V109 ARM firmware instructions vs our Rust TEE provider.

Requires unicorn==2.1.4 and pyelftools==0.32. The OS/memory/file hooks are
synthetic; no TEE device or secure-world code is executed. TA bytes stay local.
"""
import argparse
import hashlib
import json
from pathlib import Path
import struct
import subprocess
import time
from firmware_oracle import Stock, packet, FIRMWARE_SHA256, check_kernel_bounds

ROOT = Path(__file__).resolve().parents[2]
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--rust-peer',type=Path,default=ROOT/'assets/build/tee-firmware-rust/debug/examples/firmware-replay')
parser.add_argument('--out',type=Path,default=ROOT/'assets/research/tee-firmware-comparison'/str(time.time_ns()))
args=parser.parse_args()
firmware=ROOT/'assets/research/native-vendor-stack/tee-supplicant'
root=ROOT/'assets/research/audio-agent-direct/cold-firmware'
manifest=json.loads((root/'manifest.json').read_text())
files={}
for entry in manifest:
    name=entry['source_path'].rsplit('/',1)[-1]
    if not name.endswith('.ta'): continue
    data=(root/name).read_bytes()
    assert len(data)==entry['bytes'] and hashlib.sha256(data).hexdigest()==entry['sha256']
    files[name]=data
assert len(files)==4
kernel_bounds=check_kernel_bounds(ROOT/'assets/research/kexec/kernel-disassembly.elf')
report={'kernel_length_guards':kernel_bounds,'firmware_sha256':FIRMWARE_SHA256,'rust_peer_sha256':hashlib.sha256(args.rust_peer.read_bytes()).hexdigest(),
        'hardware_tested':False,'secure_world_tested':False,
        'emulated':'exact stock Thumb dispatcher, memref lookup/bounds, SHM list bookkeeping and TA path/size/copy code',
        'synthetic':'mutexes, ioctl peer, zeroed mmap memory, allocation and in-memory read-only stdio',
        'agreement':[],'differences':[]}

def compare(name, requests, expect_difference=False):
    stock=Stock(firmware,files)
    observed=[]
    for request in requests:
        raw=stock.request(request)
        observed.append((None if raw is None else raw.hex(),stock.snapshots()))
    process=subprocess.run([str(args.rust_peer),str(root)],input=''.join(r.hex()+'\n' for r in requests),
                           text=True,capture_output=True,check=True)
    rust=[]
    for line in process.stdout.splitlines():
        raw,hashes=line.split(';')
        rust.append((raw,dict(pair.split(':') for pair in hashes.split(',') if pair)))
    assert len(rust)==len(observed)
    rows=[{'request':r.hex(),'stock':s,'rust':v} for r,s,v in zip(requests,observed,rust)]
    equal=observed==rust
    assert equal != expect_difference, f'{name}: unexpected {"agreement" if equal else "difference"}: {rows}'
    report['differences' if expect_difference else 'agreement'].append({'name':name,'packets':rows})
    count = len(report['agreement']) + len(report['differences'])
    if count % 25 == 0: print(f'Checked {count} firmware scenarios', flush=True)

for name,data in files.items():
    uuid=bytes.fromhex(name[:-3].replace('-','')); a,b=struct.unpack('<QQ',uuid)
    for attr in (1,2,3):
        for mem in (5,6,7):
            for capacity in (0,1,len(data)-1,len(data),len(data)+16):
                compare(f'{name}:value={attr}:memref={mem}:capacity={capacity}',[
                    packet(6,[(attr,0,len(data)+16,0)]),
                    packet(0,[(attr,a,b,0),(mem,0,capacity,17)]),
                    packet(7,[(attr,0,17,0)]),
                    packet(7,[(attr,0,17,0)]),
                ])
    for mem in (5,6,7):
        compare(f'{name}:absent-query:memref={mem}',[packet(0,[(1,a,b,0),(mem,0,0,0xffffffff)])])
    compare(f'{name}:unowned-nonempty-output-rust-refuses',[packet(0,[(1,a,b,0),(6,0,len(data),0xffffffff)])],True)
    compare(f'{name}:nonzero-offset-stock-undercounts',[
        packet(6,[(1,0,len(data)+32,0)]),packet(0,[(1,a,b,0),(6,8,len(data),17)])],True)

for prefix in ([],[(0x100,11,22,33)],[(0x100,11,22,33),(0x101,44,55,66)]):
    compare(f'metadata-prefix-{len(prefix)}',[packet(6,prefix+[(1,0,128,0)]),packet(7,prefix+[(1,0,17,0)])])
for function,params in [(0,[]),(0,[(1,0,0,0)]),(6,[]),(6,[(4,0,128,0)]),(7,[]),(7,[(1,0,17,0)]),(99,[]),(99,[(1,0,0,0)]),(0,[(1,0,0,0),(6,0,0,0)])]:
    compare(f'rejection-{function}-{params}',[packet(function,params)])
compare('misplaced-metadata-stock-closes-loop',[packet(6,[(1,0,128,0),(0x100,0,0,0)])],True)
compare('attr-high-bits-rust-rejects',[packet(6,[(0x100000001,0,128,0)])],True)
# The stock handler truncates size to 32 bits; Rust retains and bounds all 64.
compare('size-high-bits-rust-rejects',[packet(6,[(1,0,(1<<32)+128,0)])],True)
report['summary']={'agreeing_scenarios':len(report['agreement']),'documented_difference_scenarios':len(report['differences']),
                   'packets':sum(len(s['packets']) for group in ('agreement','differences') for s in report[group]),
                   'stock_receive_slots':5,'rust_receive_slots':31,'kernel_slot_ceiling':31}
args.out.mkdir(parents=True,exist_ok=True)
(args.out/'comparison.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps(report['summary']))
print('PASS: firmware differential check; evidence '+str(args.out))
