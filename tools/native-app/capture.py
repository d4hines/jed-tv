#!/usr/bin/env python3
"""Capture the shared application frame, preserving alpha for regression checks."""
import argparse
import base64
import json
from pathlib import Path
import struct
import subprocess
import zlib

ROOT=Path(__file__).resolve().parents[2]
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--require-opaque',action='store_true')
parser.add_argument('--name',default='native-app-frame')
args=parser.parse_args()
import sys
sys.path.insert(0, str(ROOT / 'tools/pid1'))
from watch import request
source = json.loads(request('GET', '/status')).get('display_source', '/tmp/app.frames')
assert source in ('/tmp/app.frames', '/tmp/core-app.frames', '/run/tv-core/menu.frames')
reply=subprocess.check_output([str(ROOT/'repl.sh'),'-c',
    f'echo FRAME_BEGIN; base64 {source}; echo FRAME_END'],cwd=ROOT).replace(b'\r',b'')
start=reply.index(b'\nFRAME_BEGIN\n')+len(b'\nFRAME_BEGIN\n')
end=reply.index(b'\nFRAME_END',start)
data=base64.b64decode(reply[start:end])
magic,w,h,stride,page,sequence=struct.unpack_from('<6I',data)
assert magic==0x32465654 and page<2 and stride==w*4 and len(data)==4096+stride*h*2
pixels=data[4096+page*stride*h:4096+(page+1)*stride*h]
opaque=sum(a==255 for a in pixels[3::4])
report={'width':w,'height':h,'page':page,'sequence':sequence,'opaque_pixels':opaque,'total_pixels':w*h}
base=ROOT/'assets/build'/args.name
base.with_suffix('.json').write_text(json.dumps(report,indent=2)+'\n')
raw=b''.join(b'\0'+pixels[y*stride:(y+1)*stride] for y in range(h))
def chunk(kind,body):
    return struct.pack('>I',len(body))+kind+body+struct.pack('>I',zlib.crc32(kind+body))
base.with_suffix('.png').write_bytes(b'\x89PNG\r\n\x1a\n'
    +chunk(b'IHDR',struct.pack('>2I5B',w,h,8,6,0,0,0))
    +chunk(b'IDAT',zlib.compress(raw))+chunk(b'IEND',b''))
print(json.dumps(report))
if args.require_opaque and opaque!=w*h: raise SystemExit('FAIL: transparent UI pixels')
