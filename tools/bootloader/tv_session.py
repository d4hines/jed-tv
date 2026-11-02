"""G13 experiment helpers; use only at a freshly verified MBOOT prompt.

No automatic reboot, flashing, environment saving or payload execution.
Screen remains the only serial owner. Transactions are captured for review.
"""
import fcntl
import json
from pathlib import Path
import re
import shutil
import struct
import subprocess
import sys
import tempfile
import time
import zlib

import lz4.block

ROOT=Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/research/g13-leanboot-os'
LOG=ROOT/'assets/research/g13-ethernet-20261101/serial.log'
SCREEN='/nix/store/q9k0rlny9cwxc2sba5lripkm66pwmx7b-screen-5.0.2/bin/screen'
SESSION='96550'
PROMPT=b'<< MT5867 >>#'
STAGING=0x26100000


def upload_limit(address):
    # Code and compressed staging each own a disjoint 1 MiB reservation.
    # The crypto-enabled RAM trial needs more than the original 64 KiB cap.
    assert address in (0x26000000,STAGING,0x26200000)
    return 128*1024 if address in (0x26000000,STAGING) else 65536


def pack(data):
    """MBOOT expects a raw LZ4 block, with its exact output length separately."""
    compressed=lz4.block.compress(data,mode='high_compression',store_size=False)
    assert lz4.block.decompress(compressed,uncompressed_size=len(data))==data
    return compressed


class Session:
    def __enter__(self):
        self.lock=LOG.with_suffix('.lock').open('w')
        fcntl.flock(self.lock,fcntl.LOCK_EX|fcntl.LOCK_NB)
        return self

    def __exit__(self,*args):self.lock.close()

    def ask(self,command,timeout=8):
        assert '\n' not in command and '\r' not in command and len(command)<240
        start=LOG.stat().st_size
        subprocess.run([SCREEN,'-S',SESSION,'-p','0','-X','stuff',command+'\r'],check=True)
        deadline=time.monotonic()+timeout
        while time.monotonic()<deadline:
            with LOG.open('rb') as f:f.seek(start);data=f.read()
            if PROMPT in data:
                reply=data.decode(errors='replace')
                with (OUT/'transactions.jsonl').open('a') as f:
                    f.write(json.dumps({'time':time.time(),'command':command,'reply':reply})+'\n')
                assert 'Unknown command' not in reply and ('Usage:' not in reply or command.startswith('help ')),reply
                return reply
            time.sleep(.01)
        (OUT/'last-timeout.bin').write_bytes(data)
        raise TimeoutError(command)

    def guard(self):
        info=self.ask('bdinfo')
        for line in ('relocaddr   = 0x28D00000','reloc off   = 0x01900000',
                     'TLB addr    = 0x297F0000','sp start    = 0x284DDF60'):
            assert line in info,info
        assert 'androidboot.serialno=18ACC22D1C47' in self.ask('printenv bootargs')
        assert 'C92369AF' in self.ask('crc32 28daf290 168')
        return info

    def read(self,address,size):
        assert address%4==0 and size%4==0 and size<=65536
        result=bytearray()
        for offset in range(0,size,128):
            count=min(128,size-offset)//4
            reply=self.ask(f'md.l {address+offset:08x} {count:x}')
            words=[]
            for line in reply.splitlines():
                m=re.match(r'([0-9A-F]{8}):((?: [0-9A-F]{8}){1,4}) ',line)
                if m:
                    assert int(m[1],16)==address+offset+len(words)*4
                    words.extend(int(x,16) for x in m[2].split())
            assert len(words)==count,reply
            result+=struct.pack('<'+'I'*count,*words)
        return bytes(result)

    def upload(self,address,data):
        # Only the reserved experiment code/control regions may be uploaded.
        assert address in (0x26000000,0x26200000) and 0<len(data)<=upload_limit(address)
        compressed=pack(data) if len(data)>=1024 else b''
        if compressed and len(compressed)+64<len(data):
            # The stock decoder trusts input bounds: only feed locally generated,
            # round-trip-checked bytes, and verify their CRC before invoking it.
            assert 'LZ4 decompress' in self.ask('help lz4')
            self._upload_bytes(STAGING,compressed+bytes(16))
            self.crc(STAGING,compressed)
            reply=self.ask(f'lz4 -d {STAGING:x} {len(compressed):x} {address:x} {len(data):x}',timeout=15)
            assert 'fail' not in reply.lower(),reply
            self.crc(address,data)
            print(f'LZ4 RAM upload: {len(data)} -> {len(compressed)} bytes',flush=True)
            return
        self._upload_bytes(address,data)
        self.crc(address,data)

    def _upload_bytes(self,address,data):
        if len(data)<1024:
            self._upload_words(address,data)
        else:
            self._upload_ymodem(address,data)

    def _upload_ymodem(self,address,data):
        assert 0<len(data)<=upload_limit(address)
        sender=shutil.which('sb')
        if sender is None:
            raise RuntimeError('YMODEM requires sb; use nix develop .#bootloader')
        assert 'ymodem' in self.ask('help loady').lower()
        directory=Path(tempfile.mkdtemp(prefix='ymodem-',dir=OUT))
        payload=directory/'payload.bin'
        result=directory/'result.json'
        payload.write_bytes(data)
        start=LOG.stat().st_size
        began=time.monotonic()
        command=f'loady {address:x} 115200'
        # Screen routes the serial stream to this subprocess and resumes normal
        # display afterward. Neither Python nor sb opens another serial reader.
        # Start the sender first so Screen does not consume the initial C; user
        # input still goes to the original window with this fd pattern.
        subprocess.run([SCREEN,'-S',SESSION,'-p','0','-X','exec','!!',
                        sys.executable,str(ROOT/'tools/bootloader/serial_send.py'),
                        '--sender',sender,'--file',str(payload),'--result',str(result)],check=True)
        deadline=time.monotonic()+5
        while not result.with_suffix('.ready').exists():
            if result.exists() or time.monotonic()>=deadline:
                raise RuntimeError(f'YMODEM sender did not start: {directory}')
            time.sleep(.01)
        subprocess.run([SCREEN,'-S',SESSION,'-p','0','-X','stuff',command+'\r'],check=True)
        deadline=time.monotonic()+50
        while not result.exists() and time.monotonic()<deadline:time.sleep(.02)
        if not result.exists():raise TimeoutError(f'YMODEM sender: {directory}')
        record=json.loads(result.read_text())
        if record.get('returncode')!=0:
            raise RuntimeError(f'YMODEM failed; inspect receiver before retry: {record}')
        # MBOOT finishes/drains the receiver after the final protocol ACK.
        # Sending a command earlier loses its tail in that drain; sb success
        # alone is not a console-ready indication.
        deadline=time.monotonic()+8
        while time.monotonic()<deadline:
            with LOG.open('rb') as f:f.seek(start);reply=f.read()
            if PROMPT in reply:break
            time.sleep(.02)
        else:raise TimeoutError('MBOOT prompt after YMODEM')
        time.sleep(.2)
        self.crc(address,data)
        record.update(address=address,verified_crc32=f'{zlib.crc32(data):08x}',
                      verified_seconds=time.monotonic()-began)
        (directory/'verified.json').write_text(json.dumps(record,indent=2)+'\n')
        print(f'YMODEM RAM upload: {len(data)} bytes in {record["verified_seconds"]:.2f}s, CRC verified',flush=True)

    def _upload_words(self,address,data):
        assert address in (0x26000000,STAGING,0x26200000) and len(data)<=65536
        data=data+bytes(-len(data)%4)
        for offset in range(0,len(data),4):
            word=struct.unpack_from('<I',data,offset)[0]
            self.ask(f'mw.l {address+offset:08x} {word:08x} 1')
            if offset%4096==0:print(f'RAM upload {address:#x}: {offset}/{len(data)}',flush=True)
        self.crc(address,data)

    def crc(self,address,data):
        reply=self.ask(f'crc32 {address:08x} {len(data):x}',timeout=15)
        assert f'{zlib.crc32(data):08X}' in reply,reply
        return reply

    def sync(self):
        # Bootstrap-only: code has not run yet, so use already-resident helpers.
        assert 'C92369AF' in self.ask('crc32 28daf290 168')
        assert 'rc = 0x0' in self.ask('go 28daf330')
        assert 'rc = 0x1' in self.ask('go 28daf318')
        assert 'rc = 0xC0181D' in self.ask('go 28db0a20')
