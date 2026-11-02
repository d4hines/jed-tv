"""Public-key test fixtures for the RAM trial. None of these images is executed."""
import hashlib
import json
from pathlib import Path
import struct
import subprocess
from package import package

ROOT=Path(__file__).resolve().parents[3]
OUT=ROOT/'assets/build/leanboot-update-tv'
BANK_SIZE=32768+512

def bank_image(blob):
    data=blob[:160]+bytes(352)+blob[160:]
    assert len(data)<=BANK_SIZE
    return data+bytes(BANK_SIZE-len(data))

def prepare():
    OUT.mkdir(parents=True,exist_ok=True)
    seed=bytes.fromhex('9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60')
    der=OUT/'test-key.der';pem=OUT/'test-key.pem'
    der.write_bytes(bytes.fromhex('302e020100300506032b657004220420')+seed)
    subprocess.run(['openssl','pkey','-inform','DER','-in',str(der),'-out',str(pem)],check=True)
    pem.chmod(0o600)
    old=package(bytes(range(256))*4,pem,1)
    new=package(bytes((i*31+17)%256 for i in range(1537)),pem,2,entry=4)
    third=package(bytes((i*13+29)%256 for i in range(32768)),pem,3)
    forged=bytearray(third);forged[96]^=1
    changed=bytearray(third);changed[160]^=1
    fixtures={'seed':old,'v2':new,'v3':third,'forged':bytes(forged),'changed':bytes(changed)}
    for name,data in fixtures.items():(OUT/(name+'.lbu')).write_bytes(data)
    (OUT/'fixtures.json').write_text(json.dumps({k:hashlib.sha256(v).hexdigest() for k,v in fixtures.items()},indent=2)+'\n')
    return fixtures

def request(op,session,offset,count,data=b''):
    return struct.pack('<5I',0x3152554c,op,session,offset,count)+data

def transfer(blob,session,status=0,duplicate=False):
    """Request/expected-status pairs for an independently checked packet run."""
    packets=[(request(1,session,0,len(blob)),0)]
    for off in range(0,len(blob),1024):
        data=blob[off:off+1024]
        packets.append((request(2,session,off,len(data),data),0))
        if duplicate and off==0:packets.append(packets[-1])
    seal=(request(3,session,len(blob),0),status)
    packets.append(seal)
    if duplicate:packets.append(seal)
    return packets
