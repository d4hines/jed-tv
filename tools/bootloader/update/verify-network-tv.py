#!/usr/bin/env python3
"""Full G13 restarts covering fallback, no upload and forged+valid network boot."""
import sys
sys.dont_write_bytecode=True
import argparse
import json
from pathlib import Path
import struct
import subprocess
import time
import zlib
from concurrent.futures import ThreadPoolExecutor
from fixtures_tv import ROOT
from send import send
sys.path.insert(0,str(ROOT/'tools/native-app'))
import g13_device as d

BUILD=ROOT/'assets/build/leanboot-network'
OUT=ROOT/'assets/research/g13-network-default/verification'
LOG=ROOT/'assets/research/g13-ethernet-20261101/serial.log'

def journal():
    data=d.adb('exec-out','dd if=/dev/block/by-name/upgrade bs=512 skip=102398 count=2 2>/dev/null',capture_output=True).stdout
    assert len(data)==1024
    records=[]
    for off in (0,512):
        block=data[off:off+512];magic,version,seq,state,nseq,nstate=struct.unpack_from('<6I',block)
        assert magic==0x314a424c and version==1 and nseq==seq^0xffffffff and nstate==state^0xffffffff
        assert zlib.crc32(block[:508])==struct.unpack_from('<I',block,508)[0]
        records.append((seq,state))
    a,b=records
    return a if 0<(a[0]-b[0])&0xffffffff<0x80000000 else b

def reconnect():
    deadline=time.monotonic()+40
    while time.monotonic()<deadline:
        try:d.connect();d.root();return
        except (subprocess.SubprocessError,RuntimeError):time.sleep(.5)
    raise RuntimeError('G13 Ethernet ADB did not return')

def offer(package):
    blob=package.read_bytes();bad=bytearray(blob);bad[96]^=1
    try:send(bytes(bad),'192.168.77.3',timeout=.2,retries=15,wait=90)
    except RuntimeError as error:assert 'status=18' in str(error),error
    else:raise AssertionError('Forged signature accepted')
    began=time.monotonic()
    send(blob,'192.168.77.3',timeout=.2,retries=15)
    return {'forged_rejected':True,'signed_accepted':True,'package_bytes':len(blob),
            'signed_transfer_seconds':round(time.monotonic()-began,6)}

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--count',type=int,choices=(1,3),default=3)
    parser.add_argument('--out',type=Path,default=OUT)
    parser.add_argument('--package',type=Path,default=BUILD/'example.lbu')
    parser.add_argument('--expected-hash',default=None,
                        help='verify an installed predecessor while a new candidate is being built')
    args=parser.parse_args()
    out=args.out;out.mkdir(parents=True,exist_ok=True);reconnect()
    expected=args.expected_hash or json.loads((BUILD/'boot-image.json').read_text())['image_sha256']
    assert d.shell('sha256sum /dev/block/by-name/boot_a').split()[0]==expected
    seq,state=journal();assert state in (0,1,2,4),(seq,state)
    results=[]
    for index in range(1,args.count+1):
        next_state={0:1,1:2,2:4,4:1}[state]
        network=next_state==2
        before=d.shell('cat /proc/sys/kernel/random/boot_id').strip()
        start=LOG.stat().st_size;began=time.monotonic();times={}
        with ThreadPoolExecutor(max_workers=1) as pool:
            future=pool.submit(offer,args.package) if network else None
            d.adb('reboot',capture_output=True,timeout=15)
            deadline=time.monotonic()+100
            while time.monotonic()<deadline:
                with LOG.open('rb') as stream:stream.seek(start);data=stream.read()
                for marker in ['LEANBOOT WAIT','LEANBOOT NET','LEANBOOT OS prep','LEANBOOT Linux','READY: bootloader=']:
                    if marker.encode() in data and marker not in times:times[marker]=round(time.monotonic()-began,3)
                if b'DANDE_BOOT' in data and b'READY: bootloader=' in data:break
                time.sleep(.1)
            else:raise TimeoutError('Native READY after restart '+str(index))
            transfer=future.result(timeout=5) if future else None
        (out/f'boot-{index}.bin').write_bytes(data)
        reconnect()
        boot=d.shell('cat /proc/sys/kernel/random/boot_id').strip();assert boot!=before
        actual=d.shell('sha256sum /dev/block/by-name/boot_a').split()[0];assert actual==expected
        got_seq,got_state=journal();assert (got_seq,got_state)==((seq+1)&0xffffffff,next_state)
        lean=d.shell('getprop ro.boot.leanboot').strip()
        assert lean in (('','0') if next_state==4 else ('1',)),lean
        for service in ['tvinit','surfaceflinger','audioserver']:assert d.shell('pidof '+service).strip()
        if next_state==4:assert b'LEANBOOT WAIT' not in data
        else:assert b'LEANBOOT WAIT' in data
        assert (b'NETWORK LEAN' in data)==network
        result={'restart':index,'boot_id':boot,'journal_sequence':got_seq,'journal_state':got_state,
                'bootloader':'MBOOT' if next_state==4 else 'Lean','boot_a_sha256':actual,
                'markers_seconds':times,'transfer':transfer,'native_services':True}
        results.append(result);(out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
        print(json.dumps(result),flush=True);seq,state=got_seq,got_state

if __name__=='__main__':main()
