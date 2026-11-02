#!/usr/bin/env python3
"""Bounded raw-Ethernet checks for the G13 RAM payload, on its direct cable only.

Needs CAP_NET_RAW (for example sudo). Does not configure interfaces or routes.
Saves a small pcap and JSON result. This is not a firmware uploader.
"""
import argparse
import json
from pathlib import Path
import socket
import struct
import time

TV=bytes.fromhex('18acc22d1c47')
TV_IP=bytes([192,168,77,3])
HOST_IP=bytes([192,168,77,1])


def checksum(data):
    data+=bytes(len(data)%2)
    total=sum(struct.unpack('!'+'H'*(len(data)//2),data))
    while total>>16:total=(total&65535)+(total>>16)
    return total^65535


def main():
    p=argparse.ArgumentParser(description=__doc__)
    p.add_argument('--interface',default='enu1u3')
    p.add_argument('--out',type=Path,required=True)
    a=p.parse_args();a.out.mkdir(parents=True,exist_ok=True)
    mac=bytes.fromhex((Path('/sys/class/net')/a.interface/'address').read_text().strip().replace(':',''))
    s=socket.socket(socket.AF_PACKET,socket.SOCK_RAW,socket.htons(3));s.bind((a.interface,0));s.settimeout(.2)
    records=[];cases=[]
    def exchange(name,frame,match):
        for attempt in range(3):
            s.send(frame);records.append((time.time(),frame));deadline=time.monotonic()+2
            while time.monotonic()<deadline:
                try:data,source=s.recvfrom(2048)
                except socket.timeout:continue
                if source[2]==socket.PACKET_OUTGOING:continue
                if data[6:12]!=TV:continue
                records.append((time.time(),data))
                if match(data):
                    cases.append(dict(name=name,passed=True,attempt=attempt+1,request_bytes=len(frame),response_bytes=len(data)))
                    print(name+': PASS',flush=True);return
        cases.append(dict(name=name,passed=False));raise RuntimeError(name+': no valid reply')
    try:
        arp=b'\xff'*6+mac+bytes.fromhex('08060001080006040001')+mac+HOST_IP+bytes(6)+TV_IP
        exchange('ARP',arp,lambda d:len(d)>=42 and d[:12]==mac+TV and d[12:22]==bytes.fromhex('08060001080006040002') and d[22:42]==TV+TV_IP+mac+HOST_IP)
        seq=0
        for n in [0,1,31,256,1472]:
            payload=bytes((i*13+7)%256 for i in range(n))
            for proto in [1,17]:
                seq+=1
                if proto==1:
                    body=bytearray(struct.pack('!BBHHH',8,0,0,0x1ea4,seq)+payload)
                    struct.pack_into('!H',body,2,checksum(bytes(body)))
                else:
                    body=bytearray(struct.pack('!HHHH',43210,7533,8+n,0)+payload)
                    pseudo=HOST_IP+TV_IP+struct.pack('!BBH',0,17,len(body))
                    struct.pack_into('!H',body,6,checksum(pseudo+body) or 65535)
                header=bytearray(struct.pack('!BBHHHBBH4s4s',0x45,0,20+len(body),seq,0,64,proto,0,HOST_IP,TV_IP))
                struct.pack_into('!H',header,10,checksum(bytes(header)))
                request=TV+mac+bytes.fromhex('0800')+header+body
                def match(d):
                    if len(d)<42+n or d[:14]!=mac+TV+bytes.fromhex('0800') or d[14]!=0x45 or d[23]!=proto or d[26:34]!=TV_IP+HOST_IP:return False
                    if struct.unpack_from('!H',d,16)[0]!=28+n or checksum(d[14:34])!=0:return False
                    response=d[34:42+n]
                    if response[8:]!=payload:return False
                    if proto==1:return response[:2]==bytes(2) and response[4:8]==body[4:8] and checksum(response)==0
                    return struct.unpack_from('!HHH',response)==(7533,43210,8+n) and checksum(TV_IP+HOST_IP+struct.pack('!BBH',0,17,len(response))+response)==0
                exchange(('ICMP' if proto==1 else 'UDP')+f' {n}',request,match)
    finally:
        s.close()
        (a.out/'host.json').write_text(json.dumps(dict(interface=a.interface,host_mac=mac.hex(':'),cases=cases),indent=2)+'\n')
        with (a.out/'ethernet.pcap').open('wb') as f:
            f.write(struct.pack('<IHHIIII',0xa1b2c3d4,2,4,0,0,65535,1))
            for stamp,data in records:f.write(struct.pack('<IIII',int(stamp),int(stamp%1*1e6),len(data),len(data))+data)


if __name__=='__main__':main()
