#!/usr/bin/env python3
"""Run the Ethernet-only RAM experiment at an already stopped, verified MBOOT prompt.

Uses the existing Screen session; never opens UART. Does not reboot, flash,
change boot selection or save environment. See research/G13_LEANBOOT_ETHERNET.txt.
"""
import argparse
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone
import json
import hashlib
from pathlib import Path
import shutil
import struct
import subprocess
import sys
import time

import tv_session as t
from build import OUT as BUILD


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--mode',choices=['phy','echo'],default='phy')
    parser.add_argument('--interface',default='enu1u3')
    args=parser.parse_args()
    if Path('/proc/'+t.SESSION+'/fd/5').resolve()!=Path('/dev/ttyUSB1'):
        raise RuntimeError('Re-establish Screen UART ownership before using this session-specific tool')
    if Path('/proc/'+t.SESSION+'/fd/6').resolve()!=t.LOG:
        raise RuntimeError('Screen log does not match the verified session')
    stamp=datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    out=t.ROOT/'assets/research/g13-leanboot-ethernet'/f'{args.mode}-{stamp}'
    out.mkdir(parents=True);t.OUT=out
    for name in ['g13-ethernet.bin','g13-ethernet.elf','build.json','boundary-audit.json','ethernet-check.json']:
        shutil.copyfile(BUILD/name,out/name)
    code=(out/'g13-ethernet.bin').read_bytes()
    checked=json.loads((out/'ethernet-check.json').read_text())
    assert checked['binary_sha256']==hashlib.sha256(code).hexdigest(), 'Run check-ethernet.py on this build first'
    failed=None
    with t.Session() as s:
        s.guard()
        assert '26400C0E' in s.ask('md.l 297f0990 1')
        assert '26400C06' in s.ask('md.l 297f1990 1')
        s.upload(0x26000000,code)
        s.upload(0x26200000,struct.pack('<II',0x45544831,int(args.mode=='echo')))
        # Report slots are scratch; clear them so an early return is apparent.
        s.ask('mw.l 26200100 0 a')
        s.sync();start=t.LOG.stat().st_size
        if args.mode=='phy':reply=s.ask('go 26000000',timeout=15)
        else:
            with ThreadPoolExecutor(max_workers=1) as pool:
                future=pool.submit(s.ask,'go 26000000',90)
                deadline=time.monotonic()+15
                try:
                    while time.monotonic()<deadline:
                        data=t.LOG.read_bytes()[start:]
                        if b'READY' in data:break
                        if future.done():raise RuntimeError('Payload returned before READY: '+future.result())
                        time.sleep(.02)
                    else:raise TimeoutError('Ethernet READY')
                    host=subprocess.run(['sudo','-n',sys.executable,str(t.ROOT/'tools/bootloader/ethernet-host.py'),
                                         '--interface',args.interface,'--out',str(out)],capture_output=True,text=True,timeout=70)
                    (out/'host.log').write_text(host.stdout+host.stderr);print(host.stdout,flush=True)
                    if host.returncode:failed='Host packet test failed; see host.log'
                except Exception as exc:
                    failed=str(exc)
                finally:
                    if not future.done() and t.PROMPT not in t.LOG.read_bytes()[start:]:
                        subprocess.run([t.SCREEN,'-S',t.SESSION,'-p','0','-X','stuff','q'],check=True)
                    reply=future.result(timeout=15)
        print(reply,flush=True);(out/'result.txt').write_text(reply)
        (out/'serial.bin').write_bytes(t.LOG.read_bytes()[start:])
        report=s.read(0x26200100,40);(out/'report.bin').write_bytes(report)
        values=struct.unpack('<10I',report)
        print('Report:',[hex(x) for x in values],flush=True)
        (out/'report.json').write_text(json.dumps(dict(zip(['phy_id1','phy_id2','bmsr','init_status','received','sent','last_descriptor_status','last_descriptor_address','result','miu_stop_status'],values)),indent=2)+'\n')
        (out/'registers.bin').write_bytes(s.read(0x1f204000,0x300))
        if args.mode=='echo':
            (out/'descriptors.bin').write_bytes(s.read(0x66400000,64))
            (out/'rx-first.bin').write_bytes(s.read(0x66401000,128))
        print('Evidence:',out,flush=True)
    if failed:raise SystemExit(failed)


if __name__=='__main__':main()
