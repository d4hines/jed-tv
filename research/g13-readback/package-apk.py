"""Build the minimal binary manifest and APK without an x86-only aapt binary.

No resources are needed. Android attribute IDs are stable framework IDs;
the app targets API 32 to use the TV's explicitly allowed SELinux domain.
"""
from pathlib import Path
import struct
import zipfile

import argparse
parser=argparse.ArgumentParser(description=__doc__)
parser.add_argument('--build-dir',type=Path,
    default=Path(__file__).resolve().parents[2]/'assets/research/g13-readback/build')
build=parser.parse_args().build_dir
build.mkdir(parents=True,exist_ok=True)
strings=[]
def idx(s):
    if s not in strings: strings.append(s)
    return strings.index(s)
attrs={'label':0x01010001,'name':0x01010003,'debuggable':0x0101000f,
       'exported':0x01010010,'minSdkVersion':0x0101020c,
       'targetSdkVersion':0x01010270,'required':0x0101028e}
for name in attrs: idx(name)
ns='http://schemas.android.com/apk/res/android'
NO=0xffffffff
def node(kind,ext): return struct.pack('<HHIII',kind,16,16+len(ext),1,NO)+ext
def attr(name,value,kind=3,android=True):
    return struct.pack('<IIIHBBI',idx(ns) if android else NO,idx(name),idx(value) if kind==3 else NO,
                       8,0,kind,idx(value) if kind==3 else value)
def start(name,attributes=()):
    extension=struct.pack('<IIHHHHHH',NO,idx(name),20,20,len(attributes),0,0,0)
    return node(0x102,extension+b''.join(attributes))
def end(name): return node(0x103,struct.pack('<II',NO,idx(name)))
chunks=[node(0x100,struct.pack('<II',idx('android'),idx(ns))),
 start('manifest',[attr('package','local.tvreadback',android=False)]),
 start('uses-sdk',[attr('minSdkVersion',21,0x10),attr('targetSdkVersion',32,0x10)]),end('uses-sdk'),
 start('application',[attr('label','TV firmware readback'),attr('debuggable',0,0x12)]),
 start('uses-library',[attr('name','tcl.adapter'),attr('required',0xffffffff,0x12)]),end('uses-library'),
 start('activity',[attr('name','local.tvreadback.ServerActivity'),attr('exported',0xffffffff,0x12)]),
 end('activity'),end('application'),end('manifest'),
 node(0x101,struct.pack('<II',idx('android'),idx(ns)))]
pool=b''
offsets=[]
for s in strings:
    encoded=s.encode('utf-16le')
    offsets.append(len(pool))
    pool+=struct.pack('<H',len(encoded)//2)+encoded+b'\0\0'
pool+=b'\0'*((-len(pool))%4)
start_offset=28+4*len(strings)
string_chunk=struct.pack('<HHIIIIII',1,28,start_offset+len(pool),len(strings),0,0,start_offset,0)
string_chunk+=struct.pack('<'+'I'*len(offsets),*offsets)+pool
resource_chunk=struct.pack('<HHI',0x180,8,8+4*len(attrs))+struct.pack('<'+'I'*len(attrs),*attrs.values())
body=string_chunk+resource_chunk+b''.join(chunks)
manifest=struct.pack('<HHI',3,8,len(body)+8)+body
(build/'AndroidManifest.xml').write_bytes(manifest)
with zipfile.ZipFile(build/'readback-unsigned.apk','w',zipfile.ZIP_DEFLATED) as archive:
    archive.writestr('AndroidManifest.xml',manifest)
    with zipfile.ZipFile(build/'classes.zip') as dex:
        archive.writestr('classes.dex',dex.read('classes.dex'))
print('APK packaged; manifest targets API 32 and uses installed tcl.adapter.')
