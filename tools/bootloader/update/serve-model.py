#!/usr/bin/env python3
"""Serve the ARM updater on loopback; all protocol/install decisions are Lean."""
import argparse
import socket
import sys
sys.dont_write_bytecode=True
from pathlib import Path
from model import Machine, BANK_SIZE


def serve(directory,port):
    banks=[bytearray((directory/f'bank{i}.bin').read_bytes()) for i in range(2)]
    if any(len(b)!=BANK_SIZE for b in banks):raise ValueError('Wrong bank size')
    machine=Machine(banks)
    with socket.socket(socket.AF_INET,socket.SOCK_DGRAM) as sock:
        sock.bind(('127.0.0.1',port));print(f'LISTEN {sock.getsockname()[1]}',flush=True)
        while True:
            packet,peer=sock.recvfrom(4096)
            writes=len(machine.writes)
            reply=machine.serve(packet)
            if len(machine.writes)!=writes:machine.save(directory)
            if reply:sock.sendto(reply,peer)


if __name__=='__main__':
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('banks',type=Path);p.add_argument('--port',type=int,default=7533)
    a=p.parse_args();serve(a.banks,a.port)
