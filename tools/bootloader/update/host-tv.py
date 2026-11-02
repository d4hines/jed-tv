#!/usr/bin/env python3
"""Bounded UDP test client for the RAM-only G13 updater. Explicit target required."""
import sys
sys.dont_write_bytecode=True
import argparse
import json
import socket
import struct
import time
from pathlib import Path
from fixtures_tv import OUT, transfer

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--host',required=True);p.add_argument('--out',type=Path,required=True)
    args=p.parse_args();args.out.mkdir(parents=True,exist_ok=True)
    results=[]
    with socket.socket(socket.AF_INET,socket.SOCK_DGRAM) as sock:
        sock.connect((args.host,7533));sock.settimeout(3)
        for name,session,status,dup in [('v2',101,0,True),('forged',102,18,False),
                ('changed',103,18,False),('seed',104,18,False),('v3',105,0,True)]:
            blob=(OUT/(name+'.lbu')).read_bytes();began=time.monotonic()
            for packet,expected in transfer(blob,session,status,dup):
                _,op,_,offset,count=struct.unpack('<5I',packet[:20])
                acknowledged=0 if op==1 else offset+count if op==2 else len(blob)
                want=struct.pack('<5I',0x3152554c,session,op,expected,acknowledged)
                for attempt in range(6):
                    sock.send(packet)
                    try:reply=sock.recv(128)
                    except TimeoutError:continue
                    if len(reply)==20 and struct.unpack_from('<II',reply,4)!=(session,op):continue
                    assert reply==want,(name,op,offset,reply.hex(),want.hex())
                    break
                else:raise TimeoutError((name,op,offset))
            result={'case':name,'status':status,'bytes':len(blob),'seconds':time.monotonic()-began}
            results.append(result);print(json.dumps(result),flush=True)
    (args.out/'host.json').write_text(json.dumps(results,indent=2)+'\n')

if __name__=='__main__':main()
