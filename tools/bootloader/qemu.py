#!/usr/bin/env python3
"""Boot the Lean loader and a Linux shell on QEMU virt; never accesses the TV.

Default is interactive. --check runs a successful boot and a malformed-image
rejection. All downloaded/generated material stays under assets/build/leanboot.
"""
import argparse
import gzip
import hashlib
import json
from pathlib import Path
import selectors
import struct
import subprocess
import sys
import time
import urllib.request

sys.dont_write_bytecode = True
from build import build, OUT
from os_profile import encode_recipe

WORK = OUT / 'qemu-input'
URL = 'https://deb.debian.org/debian/dists/trixie/main/installer-armhf/20250803+deb13u7/images/netboot/'
PINS = {'vmlinuz': 'b508abca3127cb0d9e1b3aba37f400cbd7dac170040641756807e0e496039de8',
        'initrd.gz': 'e7ca77e9dfd998ef8a6f5a39ae3bf7d8c6cfcbdc3c4f59b84077ace585928fbb'}
MACHINE = 'virt-11.1,highmem=off,gic-version=2,dtb-randomness=on'


def sha(data): return hashlib.sha256(data).hexdigest()
def align(data, page): return data + bytes(-len(data) % page)


def inputs():
    WORK.mkdir(parents=True, exist_ok=True)
    for name, digest in PINS.items():
        path = WORK/name
        if not path.exists():
            print('Downloading pinned Debian ARM input:', name, flush=True)
            data = urllib.request.urlopen(URL+name, timeout=120).read()
            if sha(data) != digest: raise ValueError('Downloaded input hash mismatch: '+name)
            path.write_bytes(data)
        if sha(path.read_bytes()) != digest: raise ValueError('Input hash mismatch: '+name)


def read_cpio(data):
    pos, result = 0, {}
    while pos + 110 <= len(data):
        assert data[pos:pos+6] == b'070701'
        h = [int(data[pos+i:pos+i+8],16) for i in range(6,110,8)]
        name = data[pos+110:pos+110+h[11]-1].decode()
        pos = (pos+110+h[11]+3)&~3
        result[name] = (h[1],data[pos:pos+h[6]])
        pos = (pos+h[6]+3)&~3
        if name == 'TRAILER!!!': return result
    raise ValueError('Truncated initramfs')


def cpio(entries):
    out = b''
    for ino,(name,mode,data) in enumerate([*entries,('TRAILER!!!',0,b'')],1):
        major,minor = (5,1) if name == 'dev/console' else (0,0)
        h = [ino,mode,0,0,1,0,len(data),0,0,major,minor,len(name)+1,0]
        out = align(out+b'070701'+''.join(f'{x:08x}' for x in h).encode()+name.encode()+b'\0',4)
        out = align(out+data,4)
    return align(out,512)


def boot_image(kernel=b'', ramdisk=b''):
    header = bytearray(4096)
    header[:8] = b'ANDROID!'
    struct.pack_into('<4I',header,8,len(kernel),len(ramdisk),0,1584)
    struct.pack_into('<I',header,40,4)
    return bytes(header)+align(kernel,4096)+align(ramdisk,4096)


def vendor_image(ramdisk,dtb):
    wrapped = struct.pack('>8I',0xd7b7ab1e,len(dtb)+64,32,32,1,32,2048,0)
    wrapped += struct.pack('>8I',len(dtb),64,0,0,0,0,0,0)+dtb
    header = bytearray(4096)
    header[:8] = b'VNDRBOOT'
    struct.pack_into('<II',header,8,4,2048)
    struct.pack_into('<I',header,24,len(ramdisk))
    struct.pack_into('<II',header,2096,2128,len(wrapped))
    struct.pack_into('<4I',header,2112,108,1,108,0)
    table = struct.pack('<III',len(ramdisk),0,1)+bytes(96)
    return bytes(header)+align(ramdisk,2048)+align(wrapped,2048)+align(table,2048)


def command():
    return ['qemu-system-arm','-M',MACHINE,'-cpu','cortex-a15','-m','256M',
            '-smp','1','-display','none','-monitor','none','-serial','stdio','-nic','none']


def prepare(check):
    inputs()
    entries = read_cpio(gzip.decompress((WORK/'initrd.gz').read_bytes()))
    script = b'''#!/bin/busybox sh
export PATH=/bin
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev
echo LEANBOOT_USERSPACE_READY
uname -a
cat /proc/cmdline
if test -f /proc/bootconfig; then cat /proc/bootconfig; fi
if grep -q leanboot.test=1 /proc/cmdline; then
    grep -q 'Lean assembled both ramdisks' /leanboot-vendor || exec sh
    if test -f /proc/bootconfig; then
        grep -q 'source = "lean-qemu"' /proc/bootconfig || exec sh
    fi
    echo LEANBOOT_QEMU_PASS
    poweroff -f
fi
echo 'Lean bootloader test shell. Type poweroff -f to exit.'
exec sh
'''
    selected = [(n,0o40755,b'') for n in ('bin','lib','lib/arm-linux-gnueabihf','dev','proc','sys')]
    selected += [('dev/console',0o20600,b''),('init',0o100755,script),
                 ('bin/busybox',*entries['usr/bin/busybox']),
                 ('lib/ld-linux-armhf.so.3',*entries['usr/lib/arm-linux-gnueabihf/ld-linux-armhf.so.3']),
                 ('lib/arm-linux-gnueabihf/libc.so.6',*entries['usr/lib/arm-linux-gnueabihf/libc.so.6'])]
    selected += [('bin/'+name,0o120777,b'busybox') for name in
                 ('sh','mount','umount','cat','grep','ls','uname','poweroff','reboot',
                  'dmesg','hexdump','ps','sleep','echo','mkdir','touch','rm','cp','mv','vi','sed','wc','free')]
    generic = gzip.compress(cpio(selected),mtime=0)
    vendor = gzip.compress(cpio([('leanboot-vendor',0o100644,b'Lean assembled both ramdisks\n')]),mtime=0)
    subprocess.run(command()+['-M',MACHINE+',dumpdtb='+str(WORK/'virt-raw.dtb')],check=True,capture_output=True)
    subprocess.run(['dtc','-q','-I','dtb','-O','dtb','-s','-o',str(WORK/'virt.dtb'),str(WORK/'virt-raw.dtb')],check=True)
    dtb = (WORK/'virt.dtb').read_bytes()
    config = b'source = "lean-qemu"\n'
    length = (len(vendor)+len(generic)+len(config)+4)&~3
    length += 20
    args = b'console=ttyAMA0,115200 earlycon=pl011,0x09000000 rdinit=/init bootconfig panic=-1'
    if check: args += b' leanboot.test=1'
    recipe = encode_recipe([('/chosen/bootargs',args+b'\0'),
                            ('/chosen/linux,initrd-start',struct.pack('>I',0x44000000)),
                            ('/chosen/linux,initrd-end',struct.pack('>I',0x44000000+length))])
    blobs = {'boot':boot_image(kernel=(WORK/'vmlinuz').read_bytes()),
             'vendor':vendor_image(vendor,dtb),'init':boot_image(ramdisk=generic)}
    control = struct.pack('<16I',0x534f424c,1,1,64,len(blobs['boot']),len(blobs['vendor']),len(blobs['init']),
                          64,len(recipe),64+len(recipe),len(config),length,0,0,0,0)+recipe+config
    blobs['control'] = control
    for name,blob in blobs.items(): (WORK/(name+'.bin')).write_bytes(blob)
    manifest = {'machine':MACHINE,'kernel':'Debian ARM Linux 6.12.107+deb13-armmp (virtual board)',
                'downloads':{name:{'url':URL+name,'sha256':digest} for name,digest in PINS.items()},
                'images':{name:{'bytes':len(blob),'sha256':sha(blob)} for name,blob in blobs.items()},
                'loader_sha256':sha((OUT/'qemu.bin').read_bytes()), 'initrd_length':length}
    (WORK/'manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
    return manifest


def run_command(bad=False):
    cmd = command()
    for name,addr in [('boot',0x40207000),('vendor',0x44000000),('init',0x45400000),('control',0x46200000)]:
        path = WORK/(name+'.bin')
        if bad and name == 'boot':
            path = WORK/'bad-boot.bin'
            path.write_bytes(b'INVALID!'+(WORK/'boot.bin').read_bytes()[8:])
        cmd += ['-device',f'loader,file={path},addr={addr:#x},force-raw=on']
    return cmd+['-device',f'loader,file={OUT / "qemu.bin"},addr=0x46000000,cpu-num=0,force-raw=on']


def check_run(bad=False):
    proc = subprocess.Popen(run_command(bad),stdin=subprocess.DEVNULL,stdout=subprocess.PIPE,stderr=subprocess.STDOUT)
    log = bytearray()
    selector = selectors.DefaultSelector()
    selector.register(proc.stdout,selectors.EVENT_READ)
    start = time.monotonic()
    log_path = WORK/('reject.log' if bad else 'boot.log')
    log_path.write_bytes(b'')
    try:
        while time.monotonic()-start < 60:
            for key,_ in selector.select(0.25):
                chunk = key.fileobj.read1(65536)
                log += chunk
                with log_path.open('ab') as f: f.write(chunk)
            if bad and b'FAIL 4' in log: break
            if proc.poll() is not None: break
        else: raise TimeoutError('QEMU did not reach the expected result in 60 seconds')
        if bad:
            assert b'FAIL 4' in log and b'LEANBOOT Linux' not in log
        else:
            assert b'LEANBOOT Linux' in log and b'LEANBOOT_QEMU_PASS' in log, log[-2500:].decode(errors='replace')
            assert proc.wait(timeout=5) == 0
    finally:
        if proc.poll() is None: proc.terminate()
        proc.wait(timeout=5)
        selector.close()
        log_path.write_bytes(log)
    return {'case':'bad-header' if bad else 'linux-userspace','passed':True,'seconds':round(time.monotonic()-start,2)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check',action='store_true')
    parser.add_argument('--no-build',action='store_true')
    args = parser.parse_args()
    if not args.no_build: build()
    manifest = prepare(args.check)
    if args.check:
        results = [check_run(),check_run(True)]
        (WORK/'checks.json').write_text(json.dumps({'manifest':manifest,'results':results},indent=2)+'\n')
        for result in results: print('PASS:',result)
    else:
        print('QEMU virt: shared Lean loader -> Linux -> shell. Exit: poweroff -f',flush=True)
        raise SystemExit(subprocess.call(run_command()))


if __name__ == '__main__': main()
