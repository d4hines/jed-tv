#!/usr/bin/env python3
"""Send a signed package via bounded stop-and-wait UDP. No target is implicit."""
import argparse
import errno
import math
from pathlib import Path
import secrets
import socket
import struct
import time

MAGIC=0x3152554c


def send(package,host,port=7533,timeout=1.0,retries=5,wait=0.0,commit_timeout=600.0):
    if not 160<=len(package)<=16*1024*1024+160:raise ValueError('Invalid package size')
    if timeout<=0 or retries<1 or wait<0 or commit_timeout<=0:raise ValueError('Invalid retry budget')
    session=secrets.randbits(32) or 1
    with socket.socket(socket.AF_INET,socket.SOCK_DGRAM) as sock:
        sock.settimeout(timeout)
        def exchange(op,offset,count,data=b''):
            request=struct.pack('<5I',MAGIC,op,session,offset,count)+data
            attempts=max(retries,int(wait/timeout)+1) if op==1 else retries
            deadline=time.monotonic()+commit_timeout if op==3 else None
            if deadline is not None:attempts=max(retries,math.ceil(commit_timeout/timeout)+1)
            for _ in range(attempts):
                remaining=timeout if deadline is None else deadline-time.monotonic()
                if remaining<=0:break
                sock.settimeout(min(timeout,remaining))
                try:
                    if op==1:sock.connect((host,port))
                    sock.send(request)
                    reply=sock.recv(128)
                except TimeoutError:continue
                except OSError as error:
                    if op!=1 or wait<=0 or error.errno not in (
                        errno.ECONNREFUSED,errno.ENETUNREACH,errno.EHOSTUNREACH,errno.ENETDOWN):raise
                    time.sleep(timeout)
                    continue
                if len(reply)!=20:continue
                magic,got_session,got_op,status,ack=struct.unpack('<5I',reply)
                if (magic,got_session,got_op)!=(MAGIC,session,op):continue
                if status:raise RuntimeError(f'Update rejected: status={status}, acknowledged={ack}')
                expected=0 if op==1 else offset+count if op==2 else len(package)
                if ack!=expected:continue
                return
            raise TimeoutError(f'No valid acknowledgement for opcode {op}, offset {offset}')
        exchange(1,0,len(package))
        for offset in range(0,len(package),1024):
            data=package[offset:offset+1024];exchange(2,offset,len(data),data)
        exchange(3,len(package),0)
    return session


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('package',type=Path);p.add_argument('--host',required=True)
    p.add_argument('--port',type=int,default=7533)
    p.add_argument('--wait',type=float,default=0,help='seconds to retry Begin while the TV starts')
    p.add_argument('--commit-timeout',type=float,default=600,help='total seconds to retry Seal during flashing and full readback')
    a=p.parse_args();send(a.package.read_bytes(),a.host,a.port,wait=a.wait,commit_timeout=a.commit_timeout)
    print('Signed transfer accepted')
