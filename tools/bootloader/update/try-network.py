#!/usr/bin/env python3
"""Sign a position-independent ARM bootloader and offer it for one boot over UDP."""
import sys
sys.dont_write_bytecode=True
import argparse
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
import subprocess
from package import package
from send import send

def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('image',type=Path);p.add_argument('--key',type=Path,required=True)
    p.add_argument('--host',required=True);p.add_argument('--version',type=int,default=1)
    p.add_argument('--entry',type=lambda v:int(v,0),default=0)
    p.add_argument('--wait',type=float,default=60)
    p.add_argument('--reboot',action='store_true',help='reboot the exact G13 over existing Ethernet ADB')
    a=p.parse_args()
    blob=package(a.image.read_bytes(),a.key,a.version,a.entry)
    if len(blob)>0x100000+160:raise ValueError('Network execution is limited to 1 MiB')
    if a.reboot:
        identity=subprocess.check_output(['adb','-s',a.host+':5555','shell','getprop ro.serialno'],text=True).strip()
        if identity!='18ACC22D1C47':raise RuntimeError('Unexpected or unreachable G13 identity')
    with ThreadPoolExecutor(max_workers=1) as pool:
        result=pool.submit(send,blob,a.host,7533,0.2,15,a.wait)
        if a.reboot:subprocess.run(['adb','-s',a.host+':5555','reboot'],check=True)
        result.result()
    print('Signed image accepted for temporary execution. UART reports the handoff.')

if __name__=='__main__':main()
