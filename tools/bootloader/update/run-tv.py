#!/usr/bin/env python3
"""RAM-only signed update trial at an already verified G13 MBOOT prompt.

No automatic reboot, flash, saveenv, eMMC write, boot selection or serial open.
Uses existing Screen ownership, checked LZ4/YMODEM uploads and bounded UDP tests.
"""
import sys
sys.dont_write_bytecode=True
import hashlib
import json
from pathlib import Path
import shutil
import struct
import subprocess
import time
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime,timezone
from fixtures_tv import ROOT, OUT, BANK_SIZE, bank_image

sys.path.append(str(ROOT/'tools/bootloader'))
import tv_session as t

def main():
    assert Path('/proc/'+t.SESSION+'/fd/5').resolve()==Path('/dev/ttyUSB1')
    assert Path('/proc/'+t.SESSION+'/fd/6').resolve()==t.LOG
    code=(OUT/'g13-update-ram.bin').read_bytes();digest=hashlib.sha256(code).hexdigest()
    build=json.loads((OUT/'build.json').read_text())
    for name in ('build','checks','audit'):
        assert json.loads((OUT/(name+'.json')).read_text())['binary_sha256']==digest
    for name,expected in build['source_sha256'].items():
        assert hashlib.sha256((ROOT/name).read_bytes()).hexdigest()==expected,name
    assert build['public_key_sha256']=='21fe31dfa154a261626bf854046fd2271b7bed4b6abe45aa58877ef47f9721b9'
    fixtures=json.loads((OUT/'fixtures.json').read_text())
    for name,expected in fixtures.items():assert hashlib.sha256((OUT/(name+'.lbu')).read_bytes()).hexdigest()==expected
    seed=(OUT/'seed.lbu').read_bytes()
    stamp=datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    out=ROOT/'assets/research/g13-update-ram'/stamp;out.mkdir(parents=True)
    t.OUT=out
    for name in ('g13-update-ram.bin','g13-update-ram.elf','build.json','checks.json','audit.json','fixtures.json'):
        shutil.copyfile(OUT/name,out/name)
    failed=None
    with t.Session() as session:
        session.guard()
        for addr,value in [(0x297f0980,0x26000c0e),(0x297f0988,0x26200c0e),(0x297f098c,0x26300c0e),
                           (0x297f0990,0x26400c0e),(0x297f1990,0x26400c06)]:
            assert struct.unpack('<I',session.read(addr,4))[0]==value,hex(addr)
        session.upload(0x26000000,code)
        control=struct.pack('<II',0x55504431,len(seed))+bytes(4096-8)+seed
        session.upload(0x26200000,control)
        session.ask('mw.l 26200100 0 18')
        session.sync();start=t.LOG.stat().st_size
        with ThreadPoolExecutor(max_workers=1) as pool:
            future=pool.submit(session.ask,'go 26000000',240)
            try:
                deadline=time.monotonic()+20
                while time.monotonic()<deadline:
                    if b'LEANBOOT RAM READY' in t.LOG.read_bytes()[start:]:break
                    if future.done():raise RuntimeError('Returned before READY: '+future.result())
                    time.sleep(.02)
                else:raise TimeoutError('RAM updater READY')
                host=subprocess.run([sys.executable,str(Path(__file__).with_name('host-tv.py')),
                                     '--host','192.168.77.3','--out',str(out)],capture_output=True,text=True,timeout=120)
                (out/'host.log').write_text(host.stdout+host.stderr);print(host.stdout,flush=True)
                if host.returncode:raise RuntimeError('UDP checks failed; see host.log')
            except Exception as exc:failed=str(exc)
            finally:
                if not future.done() and t.PROMPT not in t.LOG.read_bytes()[start:]:
                    subprocess.run([t.SCREEN,'-S',t.SESSION,'-p','0','-X','stuff','q'],check=True)
                reply=future.result(timeout=20)
        (out/'serial.bin').write_bytes(t.LOG.read_bytes()[start:]);print(reply,flush=True)
        assert 'rc = 0x0' in reply,reply
        report=struct.unpack('<10I',session.read(0x26200100,40))
        stats=struct.unpack('<4I',session.read(0x26200140,16))
        registers=session.read(0x1f204000,8)
        assert (struct.unpack('<II',registers)[0]&12)==0,'MAC still active'
        assert report[8:]==(0,0),report
        if not failed:
            assert stats==(72,0,0,3),stats
            session.crc(0x26210000,bank_image((OUT/'v3.lbu').read_bytes()))
            session.crc(0x26220000,bank_image((OUT/'v2.lbu').read_bytes()))
        result={'binary_sha256':digest,'report':report,'ram_writes':stats[0],'last_status':stats[1],
                'selected_bank':stats[2],'selected_version':stats[3],'mac_disabled':True,
                'returned_to_mboot':True,'error':failed,'storage':'RAM only'}
        (out/'result.json').write_text(json.dumps(result,indent=2)+'\n')
        print('Evidence:',out,flush=True)
    if failed:raise SystemExit(failed)

if __name__=='__main__':main()
