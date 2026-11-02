#!/usr/bin/env python3
"""Run the signed network-call experiment at an existing MBOOT prompt; no flash."""
import sys
sys.dont_write_bytecode=True
import hashlib
import json
from pathlib import Path
import struct
import subprocess
import time
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime,timezone
from fixtures_tv import ROOT
from send import send

sys.path.append(str(ROOT/'tools/bootloader'))
import tv_session as t
BUILD=ROOT/'assets/build/leanboot-network'

def main():
    assert Path('/proc/'+t.SESSION+'/fd/5').resolve()==Path('/dev/ttyUSB1')
    assert Path('/proc/'+t.SESSION+'/fd/6').resolve()==t.LOG
    code=(BUILD/'network-ram.bin').read_bytes();digest=hashlib.sha256(code).hexdigest()
    build=json.loads((BUILD/'build.json').read_text())
    assert build['outputs']['network-ram']['sha256']==digest
    assert json.loads((BUILD/'checks.json').read_text())['sha256']==digest
    assert json.loads((BUILD/'audit.json').read_text())['outputs']['network-ram']['sha256']==digest
    for name,want in build['source_sha256'].items():assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==want,name
    assert build['public_key_sha256']=='21fe31dfa154a261626bf854046fd2271b7bed4b6abe45aa58877ef47f9721b9'
    out=ROOT/'assets/research/g13-update-ram'/('network-'+datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ'))
    out.mkdir(parents=True);t.OUT=out
    for name in ('network-ram.bin','network-ram.elf','network-example.bin','example.lbu','build.json','checks.json','audit.json'):
        (out/name).write_bytes((BUILD/name).read_bytes())
    with t.Session() as session:
        session.guard()
        for section in [0x260,0x263,0x264,0x403,0x404,0x405,0x406]:
            assert struct.unpack('<I',session.read(0x297f0000+section*4,4))[0]==section*0x100000+0xc0e,hex(section)
        assert struct.unpack('<I',session.read(0x297f1990,4))[0]==0x26400c06
        session.upload(0x26000000,code);session.sync()
        start=t.LOG.stat().st_size
        with ThreadPoolExecutor(max_workers=1) as pool:
            future=pool.submit(session.ask,'go 26000000',180)
            try:
                deadline=time.monotonic()+20
                while time.monotonic()<deadline:
                    if b'LEANBOOT WAIT' in t.LOG.read_bytes()[start:]:break
                    if future.done():raise RuntimeError(future.result())
                    time.sleep(.02)
                else:raise TimeoutError('Network boot window')
                package=(BUILD/'example.lbu').read_bytes()
                bad=bytearray(package);bad[96]^=1
                try:send(bytes(bad),'192.168.77.3',timeout=1,retries=6)
                except RuntimeError as error:assert 'status=18' in str(error),error
                else:raise AssertionError('Forged signature accepted')
                began=time.monotonic();send(package,'192.168.77.3',timeout=1,retries=6)
                transfer_seconds=time.monotonic()-began
                reply=future.result(timeout=20)
            finally:
                if not future.done() and t.PROMPT not in t.LOG.read_bytes()[start:]:
                    subprocess.run([t.SCREEN,'-S',t.SESSION,'-p','0','-X','stuff','q'],check=True)
        (out/'serial.bin').write_bytes(t.LOG.read_bytes()[start:])
        assert 'NETWORK LEAN' in reply and 'rc = 0x0' in reply,reply
        context=struct.unpack('<9I',session.read(0x40301000,36))
        assert context[0]==0x31424e4c and context[5]==2 and context[8]==0x1ea4,context
        assert struct.unpack('<I',session.read(0x1f204000,4))[0]&12==0
        session.crc(0x40600000,(BUILD/'network-example.bin').read_bytes())
        result={'sha256':digest,'invalid_signature_rejected':True,'lean_payload_executed':True,
                'returned_to_mboot':True,'mac_disabled':True,'context':context,'transfer_seconds':transfer_seconds}
        (out/'result.json').write_text(json.dumps(result,indent=2)+'\n')
        print(reply);print(json.dumps(result));print('Evidence:',out)

if __name__=='__main__':main()
