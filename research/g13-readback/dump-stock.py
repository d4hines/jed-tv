"""Dump the running G13 V013 firmware via its read-only factory HAL.

Every transferred chunk is verified against its app-side SHA256 and a second
independent source read. Userdata and RPMB are never requested. Only one host
dumper may run at a time. Host images are protected read-only on completion.
"""
import argparse
import datetime
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import time
from client import Client,EXPECTED

def now(): return datetime.datetime.now(datetime.timezone.utc).isoformat()
def save(path,value):
    temporary=path.with_suffix(path.suffix+'.tmp')
    temporary.write_text(json.dumps(value,indent=2)+'\n')
    temporary.replace(path)

parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--output',type=Path,required=True)
parser.add_argument('--target',default='18ACC22D1C47')
parser.add_argument('--resume',action='store_true')
args=parser.parse_args()
lock_dir=Path(__file__).resolve().parents[2]/'assets/research/g13-readback'
lock_dir.mkdir(parents=True,exist_ok=True)
lock=open(lock_dir/'.dump.lock','w')
fcntl.flock(lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
output=args.output
if output.exists() and not args.resume: raise SystemExit('Output exists; use --resume after inspecting its manifest')
if args.resume and not (output/'manifest.json').exists(): raise SystemExit('No manifest to resume')
output.mkdir(mode=0o700,parents=True,exist_ok=args.resume)
output.chmod(0o700)
manifest_path=output/'manifest.json'
with Client(args.target) as client:
    links=client.command('shell','ls','-l','/dev/block/by-name').decode()
    (output/'by-name.txt').write_text(links)
    (output/'getprop.txt').write_bytes(client.command('shell','getprop'))
    (output/'mounts.txt').write_bytes(client.command('shell','cat','/proc/mounts'))
    geometry={}
    plan=[]
    for line in links.splitlines():
        match=re.search(r' ([A-Za-z0-9_]+) -> /dev/block/(mmcblk0p\d+)$',line)
        if not match: continue
        name,node=match.groups()
        start=int(client.text('/sys/class/block/'+node+'/start').strip())*512
        size=int(client.text('/sys/class/block/'+node+'/size').strip())*512
        if size<=0 or start%512 or size%512: raise ValueError('Invalid geometry')
        geometry[name]={'node':node,'start_bytes':start,'size_bytes':size}
        if name=='userdata': continue
        if start+size>client.info['raw_limit_excluding_userdata']:
            raise ValueError('Partition overlaps excluded userdata')
        plan.append({'name':name,'source':'/dev/block/mmcblk0','offset':start,'size':size})
    if len(geometry)!=51 or geometry.get('userdata',{}).get('node')!='mmcblk0p51':
        raise ValueError('Unexpected G13 partition map')
    by_offset=sorted(plan,key=lambda x:x['offset'])
    for previous,current in zip(by_offset,by_offset[1:]):
        if previous['offset']+previous['size']>current['offset']:
            raise ValueError('Overlapping firmware partitions')
    first=min(x['offset'] for x in plan)
    if first!=2*1024*1024: raise ValueError('Unexpected eMMC head geometry')
    plan.append({'name':'mmcblk0-head','source':'/dev/block/mmcblk0','offset':0,'size':first})
    for index in (0,1):
        size=int(client.text(f'/sys/class/block/mmcblk0boot{index}/size').strip())*512
        if size!=client.info[f'boot{index}_size']: raise ValueError('Hardware boot size mismatch')
        plan.append({'name':f'mmcblk0boot{index}','source':f'/dev/block/mmcblk0boot{index}','offset':0,'size':size})
    priority=['MBOOT','MBOOT_b','MBOOTBAK','mmcblk0boot0','mmcblk0boot1','boot_a','boot_b','init_boot_a',
              'init_boot_b','vendor_boot_a','vendor_boot_b','vbmeta_a','vbmeta_b','vbmeta_system_a','vbmeta_system_b','dtbo_a','dtbo_b']
    plan.sort(key=lambda p:(priority.index(p['name']) if p['name'] in priority else len(priority),p['offset'],p['name']))
    if args.resume:
        manifest=json.loads(manifest_path.read_text())
        if manifest['fingerprint']!=EXPECTED or manifest['plan']!=plan or manifest['geometry']!=geometry:
            raise ValueError('Resume identity/geometry mismatch')
    else:
        manifest={'fingerprint':EXPECTED,'target':args.target,'created_at':now(),'complete':False,
                  'method':'targetSdk32 app -> stock MediaTek file-read HIDL -> raw eMMC offsets',
                  'geometry':geometry,'plan':plan,'total_bytes':sum(x['size'] for x in plan),'images':[],
                  'excluded':['userdata','RPMB','eMMC user-area tail inside userdata','hardware fuses'],
                  'source_verification':'Each image chunk is independently reread and SHA256-compared',
                  'restore_tested':False}
    manifest['complete']=False
    manifest.pop('error',None)
    manifest.pop('interrupted_at',None)
    save(manifest_path,manifest)
    print(f'Plan: {len(plan)} images, {manifest["total_bytes"]/2**20:.1f} MiB; userdata/RPMB excluded.',flush=True)
    start_time=time.monotonic()
    try:
        for position,item in enumerate(plan,1):
            target=output/(item['name']+'.img')
            previous=next((x for x in manifest['images'] if x['name']==item['name']),None)
            if previous and previous['status'] in ('verified','source_changed'):
                if not target.exists() or target.stat().st_size!=item['size']:
                    raise ValueError('Resume completed image missing/wrong size')
                with target.open('rb') as f: fresh=hashlib.file_digest(f,'sha256').hexdigest()
                if fresh!=previous['sha256']: raise ValueError('Resume host hash mismatch')
                print(f'SKIP {item["name"]} ({previous["status"]})',flush=True)
                continue
            partial=target.with_suffix('.img.partial')
            if partial.exists():
                partial.rename(partial.with_name(partial.name+'.interrupted-'+str(int(time.time()))))
            record=dict(item,status='reading',started_at=now(),chunks=[])
            manifest['images']=[x for x in manifest['images'] if x['name']!=item['name']]+[record]
            save(manifest_path,manifest)
            full=hashlib.sha256()
            print(f'[{position}/{len(plan)}] {item["name"]}: {item["size"]/2**20:.1f} MiB',flush=True)
            with partial.open('xb') as f:
                for relative in range(0,item['size'],32*1024*1024):
                    length=min(32*1024*1024,item['size']-relative)
                    data,digest=client.read(item['source'],item['offset']+relative,length)
                    f.write(data); full.update(data)
                    second=client.hash(item['source'],item['offset']+relative,length)
                    record['chunks'].append({'relative_offset':relative,'length':length,'sha256':digest,
                                             'second_source_sha256':second,'source_matches':digest==second})
                    record['bytes_done']=relative+length
                    save(manifest_path,manifest)
                    if item['size']>32*1024*1024:
                        print(f'  {(relative+length)/2**20:.0f}/{item["size"]/2**20:.0f} MiB; second-read match={digest==second}',flush=True)
                f.flush(); os.fsync(f.fileno())
            partial.rename(target)
            target.chmod(0o400)
            record.update(sha256=full.hexdigest(),finished_at=now(),
                          source_matches=all(c['source_matches'] for c in record['chunks']))
            record['status']='verified' if record['source_matches'] else 'source_changed'
            with target.open('rb') as f: record['host_reread_sha256']=hashlib.file_digest(f,'sha256').hexdigest()
            if record['host_reread_sha256']!=record['sha256']: raise ValueError('Fresh host hash mismatch')
            save(manifest_path,manifest)
            print('  '+record['status']+' '+record['sha256'],flush=True)
        manifest.update(complete=True,finished_at=now(),elapsed_seconds=time.monotonic()-start_time)
        save(manifest_path,manifest)
        (output/'SHA256SUMS').write_text(''.join(x['sha256']+'  '+x['name']+'.img\n' for x in manifest['images']))
        print('COMPLETE '+str(output),flush=True)
    except BaseException as error:
        manifest.update(error=repr(error),interrupted_at=now())
        save(manifest_path,manifest)
        raise
