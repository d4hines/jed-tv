#!/usr/bin/env python3
"""Exercise the real sender and ARM-backed receiver over loopback UDP only."""
import sys
sys.dont_write_bytecode=True
import json
from pathlib import Path
import select
import socket
import struct
import subprocess
from model import OUT, Machine, bank_image, BANK_SIZE
from package import package
from send import send, MAGIC


def main():
    directory=OUT/'udp-banks';directory.mkdir(parents=True,exist_ok=True)
    old=(OUT/'fixtures/v1.lbu').read_bytes();new=(OUT/'fixtures/v2.lbu').read_bytes()
    (directory/'bank0.bin').write_bytes(bank_image(old))
    (directory/'bank1.bin').write_bytes(bytes(BANK_SIZE))
    server=subprocess.Popen([sys.executable,str(Path(__file__).with_name('serve-model.py')),
                             str(directory),'--port','0'],stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
    try:
        if not select.select([server.stdout],[],[],15)[0]:raise TimeoutError('Local server did not start')
        line=server.stdout.readline().strip()
        if not line.startswith('LISTEN '):raise RuntimeError(line)
        port=int(line.split()[1]);session=send(new,'127.0.0.1',port,timeout=30,retries=3)
        with socket.socket(socket.AF_INET,socket.SOCK_DGRAM) as retry:
            retry.settimeout(5)
            retry.sendto(struct.pack('<5I',MAGIC,3,session,len(new),0),('127.0.0.1',port))
            assert retry.recv(128)==struct.pack('<5I',MAGIC,session,3,0,len(new))
        banks=[bytearray((directory/f'bank{i}.bin').read_bytes()) for i in range(2)]
        m=Machine(banks)
        assert m.version(0)==1 and m.version(1)==2
        assert banks[0]==bank_image(old)
        third=package(b'next-stage-image'*80,OUT/'fixtures/test-key.pem',3)
        send(third,'127.0.0.1',port,timeout=30,retries=3)
        banks=[bytearray((directory/f'bank{i}.bin').read_bytes()) for i in range(2)]
        m=Machine(banks)
        assert m.version(0)==3 and m.version(1)==2 and banks[1]==bank_image(new)
        forged=bytearray(package(b'unsigned replacement',OUT/'fixtures/test-key.pem',4))
        forged[96]^=1
        try:
            send(bytes(forged),'127.0.0.1',port,timeout=30,retries=3)
            raise AssertionError('Server accepted a forged signature')
        except RuntimeError as error:
            assert 'status=18' in str(error),error
        assert all((directory/f'bank{i}.bin').read_bytes()==bank for i,bank in enumerate(banks))
        report={'loopback_udp':True,'arm_receiver':True,'arm_installer':True,
                'previous_bank_preserved':True,'duplicate_commit_ack':True,
                'consecutive_updates':2,'forged_signature_no_writes':True,
                'selected_version':3,'tv_access':False}
        (OUT/'udp-check.json').write_text(json.dumps(report,indent=2)+'\n')
        print('PASS:',json.dumps(report))
    finally:
        server.terminate()
        try:server.wait(timeout=5)
        except subprocess.TimeoutExpired:server.kill();server.wait()
        errors=server.stderr.read()
        if errors:print(errors,file=sys.stderr)


if __name__=='__main__':main()
