#!/usr/bin/env python3
"""Headless ARM32 launcher/Slides check; never opens host sound or display."""
import argparse
import hashlib
import json
import pathlib
import socket
import subprocess
import sys
import tempfile
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--launcher', type=pathlib.Path, default=ROOT/'assets/build/app-target/armv7-unknown-linux-musleabihf/release/tvinit')
    ap.add_argument('--slides', type=pathlib.Path, default=ROOT/'assets/build/app-target/armv7-unknown-linux-musleabihf/release/slides')
    ap.add_argument('--kernel', type=pathlib.Path, default=ROOT/'assets/build/kernel/arm/zImage-repo',
                    help='virt32 kernel with AF_UNIX for the remote socket (test/run.py extracts demo/kernel/zImage.xz)')
    ap.add_argument('--tiles', type=pathlib.Path, default=ROOT/'assets/build/tiles')
    a = ap.parse_args()
    out = ROOT/'assets/build/lean-slides'/time.strftime('qemu-%Y%m%dT%H%M%S')
    out.mkdir(parents=True)
    serial = out/'serial.log'
    rd = out/'initramfs.cpio'
    qpath = pathlib.Path(tempfile.mkdtemp(prefix='slides-',dir='/tmp'))/'qmp'
    files = [f'{a.slides.resolve()}:/bin/slides']
    files += [f'{p.resolve()}:/share/tiles/{p.name}' for p in a.tiles.glob('*.rgb')]
    subprocess.run([sys.executable, ROOT/'tools/mkinitramfs.py', a.launcher, rd, *files],check=True)
    cmd = ['qemu-system-aarch64','-M','virt-11.1,highmem=off,gic-version=2','-cpu','cortex-a55,aarch64=off',
           '-smp','4','-m','1430M','-kernel',str(a.kernel),'-initrd',str(rd),
           '-append','console=ttyAMA0 panic=-1 ip=off drm_kms_helper.drm_fbdev_overalloc=200 tvinit.demo=1',
           '-global','virtio-mmio.force-legacy=false','-device','virtio-gpu-device,xres=1280,yres=720',
           '-device','virtio-keyboard-device','-device','virtio-rng-device',
           '-serial',f'file:{serial}','-qmp',f'unix:{qpath},server=on,wait=off','-no-reboot','-display','none']
    p = subprocess.Popen(cmd,stdout=(out/'qemu.log').open('w'),stderr=subprocess.STDOUT)
    report = {'pass':False,'evidence':str(out),'kernel':'virt32 surrogate, not stock G06'}
    try:
        def log(): return serial.read_text(errors='replace') if serial.exists() else ''
        def wait(marker,count=1):
            end=time.monotonic()+90
            while time.monotonic()<end:
                if log().count(marker)>=count:return
                if p.poll() is not None: raise RuntimeError('QEMU stopped: '+log()[-2500:])
                time.sleep(.1)
            raise RuntimeError('timeout '+marker+'\n'+log()[-2500:])
        wait('home screen: 2 rows')
        s=socket.socket(socket.AF_UNIX);s.settimeout(10);s.connect(str(qpath));f=s.makefile('rw');f.readline()
        def q(name,**kw):
            f.write(json.dumps({'execute':name,'arguments':kw})+'\n');f.flush()
            while True:
                r=json.loads(f.readline())
                if 'return' in r:return r['return']
                if 'error' in r:raise RuntimeError(r)
        q('qmp_capabilities')
        def key(k):
            for pressed in (True,False):
                q('input-send-event',events=[{'type':'key','data':{'down':pressed,'key':{'type':'qcode','data':k}}}]);time.sleep(.12)
            time.sleep(.2)
        def shot(name):q('screendump',filename=str(out/f'{name}.png'),format='png')
        shot('home-before');key('ret')
        wait('slides: slide 1/12:');shot('slide-1')
        key('right');wait('slides: slide 2/12:');shot('slide-2')
        key('left');wait('slides: slide 1/12:',2)
        key('1');key('2');wait('slides: slide 12/12:');shot('slide-12')
        key('esc');wait('slides: quit');time.sleep(1);shot('home-after-back')
        key('ret');wait('slides: slide 1/12:',3)
        key('home');time.sleep(1);shot('home-after-home')
        if 'PANIC' in log() or 'blocked syscall' in log(): raise RuntimeError(log()[-2500:])
        subprocess.run([sys.executable, ROOT/'tools/slides/prepare.py'],check=True)
        subprocess.run(['lake','exe','preview',str(out/'reference')],cwd=ROOT/'test/slides',check=True)
        parity=[]
        for i in (1,2,12):
            expected=(out/f'reference/slide-{i}.ppm').read_bytes().split(b'\n',3)[3]
            actual=subprocess.check_output(['ffmpeg','-v','error','-i',str(out/f'slide-{i}.png'),'-f','rawvideo','-pix_fmt','rgb24','-'])
            if actual != expected: raise RuntimeError(f'slide {i}: ARM framebuffer differs from host rendering')
            parity.append({'slide':i,'sha256':hashlib.sha256(actual).hexdigest()})
        report['pixel_parity']=parity
        report['pass']=True
    except Exception as e:report['error']=str(e)
    finally:
        p.kill();p.wait()
        (out/'summary.json').write_text(json.dumps(report,indent=2))
    print(json.dumps(report,indent=2))
    return 0 if report['pass'] else 1
if __name__=='__main__':sys.exit(main())
