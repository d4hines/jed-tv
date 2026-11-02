"""Host client for the bounded, read-only G13 HAL bridge."""
import hashlib
import json
import socket
import subprocess

EXPECTED='TCL/G13_2K_US/G13:14/UKN2.241110.001/AU13:user/release-keys'

class Client:
    def __init__(self,target='18ACC22D1C47'):
        self.adb=['adb','-s',target]
        self.port=None
        self.socket=None
        self.stream=None
        fingerprint=self.command('shell','getprop','ro.build.fingerprint').decode().strip()
        if fingerprint!=EXPECTED:
            raise ValueError('Unexpected live firmware fingerprint')
        logs=self.command('logcat','-d','-v','raw','-s','TVReadback:I','*:S').decode()
        candidates=[]
        for line in logs.splitlines():
            try:
                item=json.loads(line)
                if isinstance(item,dict) and item.get('fingerprint')==EXPECTED and 'socket' in item:
                    candidates.append(item)
            except ValueError:
                pass
        if not candidates:
            raise ValueError('No current readback bridge announcement; launch ServerActivity')
        self.info=candidates[-1]
        if self.info['fingerprint']!=EXPECTED:
            raise ValueError('Unexpected helper fingerprint')
        try:
            self.port=int(self.command('forward','tcp:0','localabstract:'+self.info['socket']).strip())
            self.socket=socket.create_connection(('127.0.0.1',self.port),timeout=60)
            self.socket.settimeout(120)
            self.stream=self.socket.makefile('rwb',buffering=1024*1024)
            self.send('AUTH '+self.info['token'])
            if self.line()!='OK '+EXPECTED:
                raise ValueError('Bridge authentication/identity failed')
        except BaseException:
            self.close()
            raise

    def command(self,*args):
        return subprocess.check_output(self.adb+list(args),timeout=20)

    def send(self,text):
        self.stream.write((text+'\n').encode())
        self.stream.flush()

    def line(self):
        line=self.stream.readline(4097)
        if not line.endswith(b'\n') or len(line)>4096:
            raise IOError('Truncated/oversized bridge response: '+repr(line[:120]))
        return line.decode().rstrip('\n')

    def exact(self,length):
        data=bytearray()
        while len(data)<length:
            part=self.stream.read(length-len(data))
            if not part: raise EOFError('Truncated binary read')
            data.extend(part)
        return bytes(data)

    def text(self,path):
        self.send('TEXT '+path)
        header=self.line().split()
        if len(header)!=2 or header[0]!='DATA' or not 0<=int(header[1])<=16384:
            raise ValueError('Invalid text response')
        return self.exact(int(header[1])).decode()

    def read(self,source,offset,length):
        self.send(f'READ {source} {offset} {length}')
        if self.line()!=f'DATA {length}': raise ValueError('Invalid read header')
        data=self.exact(length)
        digest=hashlib.sha256(data).hexdigest()
        if self.line()!='SHA256 '+digest: raise ValueError('Transfer checksum mismatch')
        return data,digest

    def hash(self,source,offset,length):
        self.send(f'HASH {source} {offset} {length}')
        if self.line()!=f'HASHING {length}': raise ValueError('Invalid hash header')
        result=self.line().split()
        if len(result)!=2 or result[0]!='SHA256' or len(result[1])!=64:
            raise ValueError('Invalid source hash')
        return result[1]

    def close(self):
        if self.stream:
            self.stream.close(); self.stream=None
        if self.socket:
            self.socket.close(); self.socket=None
        if self.port:
            subprocess.run(self.adb+['forward','--remove',f'tcp:{self.port}'],timeout=10,check=False)
            self.port=None

    def __enter__(self): return self
    def __exit__(self,*args): self.close()
