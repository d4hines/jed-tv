"""Patch only the approved PNY stick's FAT32 volume-label fields.
The nonstandard label injects --fast into V122 recovery's newline-delimited
BCB options. No executable or firmware payload is added. Default is dry-run.
"""
from pathlib import Path
import os,stat,struct,fcntl,json,base64,datetime,sys
DEVICE=Path('/dev/disk/by-id/usb-PNY_USB_3.1_FD_07210C69972A5005-0:0')
EXPECTED_SIZE=31042043904
OLD=b'TVRECOVERY '
NEW=b'\n--fast\n   '
assert len(OLD)==len(NEW)==11
apply=sys.argv[1:]==['--apply']
assert not sys.argv[1:] or apply,'Use --apply or no arguments'
real=DEVICE.resolve(strict=True)
assert real.name.startswith('sd'),real
for part in [DEVICE,Path(str(DEVICE)+'-part1')]:
 dev=os.stat(part).st_rdev;key=f'{os.major(dev)}:{os.minor(dev)}'
 assert all(line.split()[2]!=key for line in Path('/proc/self/mountinfo').read_text().splitlines()),f'{part} is mounted'
fd=os.open(DEVICE,os.O_RDWR|os.O_EXCL if apply else os.O_RDONLY)
try:
 assert stat.S_ISBLK(os.fstat(fd).st_mode)
 size=struct.unpack('<Q',fcntl.ioctl(fd,0x80081272,bytes(8)))[0]
 assert size==EXPECTED_SIZE,(size,EXPECTED_SIZE)
 def read(offset,n):
  data=os.pread(fd,n,offset);assert len(data)==n;return data
 mbr=read(0,512);assert mbr[510:]==b'\x55\xaa'
 assert mbr[450] in (0x0b,0x0c)
 start,count=struct.unpack_from('<II',mbr,454)
 assert start==2048 and (start+count)*512<=size
 assert not any(mbr[462:510]),'Unexpected additional partitions'
 off=start*512;boot=read(off,512)
 assert boot[510:]==b'\x55\xaa' and boot[82:90]==b'FAT32   '
 bps=struct.unpack_from('<H',boot,11)[0];spc=boot[13]
 reserved=struct.unpack_from('<H',boot,14)[0];fats=boot[16]
 fatsz=struct.unpack_from('<I',boot,36)[0];root=struct.unpack_from('<I',boot,44)[0]
 backup=struct.unpack_from('<H',boot,50)[0]
 assert bps==512 and spc>0 and spc&(spc-1)==0 and fats==2 and root>=2
 assert boot[67:71]==bytes.fromhex('f421cb6e'),'Unexpected filesystem UUID'
 assert 0<backup<reserved and boot[71:82]==OLD
 backoff=off+backup*bps;back=read(backoff,bps)
 assert back[71:82]==OLD and back[:65]==boot[:65] and back[66:]==boot[66:],'Unexpected backup BPB'
 rootoff=off+(reserved+fats*fatsz+(root-2)*spc)*bps
 rootbytes=read(rootoff,spc*bps);labels=[]
 for pos in range(0,len(rootbytes),32):
  entry=rootbytes[pos:pos+32]
  if entry[0]==0:break
  if entry[0]!=0xe5 and entry[11]==8:labels.append(pos)
 assert len(labels)==1,labels
 pos=labels[0];assert rootbytes[pos:pos+11]==OLD
 assert rootbytes[pos+20:pos+22]==bytes(2) and rootbytes[pos+26:pos+32]==bytes(6)
 rootsector=rootoff+(pos//bps)*bps;within=pos%bps
 plans=[]
 for sector,at in [(off,71),(backoff,71),(rootsector,within)]:
  old=read(sector,bps);assert old[at:at+11]==OLD
  new=old[:at]+NEW+old[at+11:]
  assert len(new)==bps
  plans.append({'offset':sector,'label_offset':at,'before_b64':base64.b64encode(old).decode(),'after_b64':base64.b64encode(new).decode()})
 result={'device':str(DEVICE),'resolved_device':str(real),'size':size,'partition_start':start,'filesystem_uuid':'6ECB-21F4','old_label_hex':OLD.hex(),'new_label_hex':NEW.hex(),'sectors':plans,'applied':False}
 print(json.dumps({k:v for k,v in result.items() if k!='sectors'},indent=2))
 print('Sector offsets:',[p['offset'] for p in plans])
 if apply:
  stamp=datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
  path=Path(__file__).resolve().parents[2]/'assets/research/recovery-probe/usb-backup'/f'fastboot-label-{stamp}.json'
  path.parent.mkdir(parents=True,exist_ok=True)
  with path.open('x') as f:json.dump(result,f,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
  for p in plans:
   before=base64.b64decode(p['before_b64']);after=base64.b64decode(p['after_b64'])
   assert read(p['offset'],bps)==before,'Device changed during preparation'
   assert os.pwrite(fd,after,p['offset'])==bps
  os.fsync(fd)
  # Invalidate the block cache, then read back the device sectors.
  fcntl.ioctl(fd,0x1261)
  for p in plans:assert read(p['offset'],bps)==base64.b64decode(p['after_b64'])
  result['applied']=True
  with path.open('w') as f:json.dump(result,f,indent=2);f.write('\n');f.flush();os.fsync(f.fileno())
  print('APPLIED and read-back verified; sector backup:',path)
 else:print('DRY RUN: no bytes written')
finally:os.close(fd)
