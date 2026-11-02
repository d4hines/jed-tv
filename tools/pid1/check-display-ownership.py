#!/usr/bin/env python3
"""Adversarial ARM32/QEMU check of Core's private-menu display ownership.

The selected app ignores TERM, continually truncates/corrupts its TVF2 file,
and probes the root-only menu and raw framebuffer. No TV is accessed.

  python3 tools/pid1/check-display-ownership.py --init PATH_TO_PID1 \
      --apps PATH_TO_ARM32_APPS --media PATH_TO_MEDIA_DERIVATION

Evidence is written below assets/build/core-display-adversary/. It records
the exact PID1/fixture hashes and the QEMU kernel/trampoline fidelity limits.
"""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import socket
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/qemu'))
import g06
spec = importlib.util.spec_from_file_location('ui_session', ROOT / 'tools/ui-audit/session.py')
ui = importlib.util.module_from_spec(spec)
spec.loader.exec_module(ui)


def ppm(path):
    with path.open('rb') as f:
        assert f.readline().strip() == b'P6'
        width, height = map(int, f.readline().split())
        assert f.readline().strip() == b'255'
        data = f.read()
    assert (width, height, len(data)) == (1280, 720, 1280 * 720 * 3)
    return data


def menu_region(data):
    # Exclude the changing clock at top-right and notice/footer at the bottom.
    return b''.join(data[y*1280*3:y*1280*3+1000*3] for y in range(600))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--init', type=Path, required=True)
    parser.add_argument('--apps', type=Path, required=True, help='directory containing slides and doomtv')
    parser.add_argument('--media', type=Path, required=True)
    args = parser.parse_args()
    binary, apps, media = args.init.resolve(), args.apps.resolve(), args.media.resolve()
    g06.require_frame_kernel(media / 'kernel/zImage')
    out = ROOT / 'assets/build/core-display-adversary' / str(time.time_ns())
    out.mkdir(parents=True)
    (out/'serial.log').touch()
    fixture = out / 'hostile-ytplay'
    subprocess.run(['rustc', '+1.92.0', '--edition=2021', '--target', 'armv7-unknown-linux-musleabihf',
                    '-C', 'target-cpu=cortex-a55', '-C', 'linker=rust-lld', '-C', 'opt-level=2',
                    '-C', 'panic=abort', str(ROOT / 'test/fixtures/core-display-adversary.rs'),
                    '-o', str(fixture)], check=True)
    trampoline = out / 'g06-init'
    g06.build_trampoline(trampoline)
    cpio = out / 'initramfs.cpio'
    files = [f'{fixture}:/bin/ytplay'] + [f'{apps/name}:/bin/{name}' for name in ('slides', 'doomtv')]
    files += [f'{tile}:/share/tiles/{tile.name}' for tile in sorted((media / 'tiles').glob('*.rgb'))]
    subprocess.run([sys.executable, str(ROOT/'tools/mkinitramfs.py'), str(binary), str(cpio), *files], check=True)
    cpio.write_bytes(g06.wrap_ramdisk(cpio.read_bytes(), trampoline, binary))
    http, console = ui.free_ports(2)
    qmp = Path('/private/tmp') / f'core-display-{out.name}.sock'
    append = ('console=ttyAMA0 rdinit=/init ip=off panic=3 '
              'drm_kms_helper.drm_fbdev_overalloc=200 hello.ip=192.168.77.2 '
              'hello.peer=192.168.77.1 hello.qemu=1 hello.timeout=0 hello.repl_timeout=0')
    network = ('user,id=n0,restrict=on,net=192.168.77.0/24,host=192.168.77.1,dhcpstart=192.168.77.15,'
               f'hostfwd=tcp:127.0.0.1:{http}-192.168.77.2:8765,'
               f'hostfwd=tcp:127.0.0.1:{console}-192.168.77.2:8766')
    command = ['qemu-system-arm', '-M', 'virt,highmem=off,gic-version=2', '-cpu', 'max', '-smp', '4', '-m', '1430M',
               '-global', 'virtio-mmio.force-legacy=false', '-kernel', str(media/'kernel/zImage'),
               '-initrd', str(cpio), '-append', append, '-netdev', network, '-device', 'virtio-net-device,netdev=n0',
               '-rtc', f'base={g06.RTC},clock=vm', '-device', 'virtio-gpu-device,xres=1280,yres=720',
               '-device', 'virtio-keyboard-device', '-device', 'virtio-rng-device', '-display', 'none',
               '-monitor', 'none', '-serial', f'file:{out}/serial.log', '-qmp', f'unix:{qmp},server=on,wait=off', '-no-reboot']
    (out/'command.json').write_text(json.dumps(command, indent=2)+'\n')
    (out/'session.json').write_text(json.dumps({'qmp':str(qmp),'http':http,'console':console})+'\n')
    g06.record(out, media/'kernel/zImage', trampoline, binary, True)
    ui.BASE = out.parent
    result = {'passed':False, 'checks':{}, 'pid1_sha256':g06.sha(binary),
              'fixture_sha256':g06.sha(fixture), 'hardware_tested':False}
    session = ui.Session(out.name)
    def check(name, value):
        result['checks'][name] = bool(value)
        print(('PASS ' if value else 'FAIL ')+name, flush=True)
        if not value: raise AssertionError(name)
    def wait(marker, timeout=30):
        deadline = time.monotonic()+timeout
        while time.monotonic() < deadline:
            text = session.serial()
            if marker in text: return text
            if process.poll() is not None: raise RuntimeError(text[-4000:])
            time.sleep(.03)
        raise TimeoutError(f'{marker}: {text[-4000:]}')
    def status():
        code, text = session.http('GET','/status')
        assert code == 200
        return json.loads(text)
    def shot(name):
        path = out / (name+'.ppm')
        session.qmp('screendump', filename=str(path))
        return ppm(path)
    with (out/'qemu.log').open('w') as log:
        process = subprocess.Popen(command, stdout=log, stderr=subprocess.STDOUT)
        try:
            wait('home screen:', 60)
            deadline = time.monotonic()+10
            while time.monotonic() < deadline:
                try:
                    initial = status()
                    if initial['phase']=='shell-ready': break
                except (OSError, ValueError): pass
                time.sleep(.1)
            else: raise TimeoutError('root console readiness')
            time.sleep(.5)
            home = shot('home-before')
            check('root-private-menu-selected', initial['display_source']=='/run/tv-core/menu.frames')
            session.press('ret')
            text = wait('hostile: VALID_MAGENTA_FRAME')
            check('app-cannot-open-menu-or-raw-framebuffer', 'hostile: PRIVATE_MENU_AND_RAW_FB_DENIED' in text)
            pid = int(re.findall(r'Core app 1: confined pid (\d+)', text)[-1])
            deadline = time.monotonic()+4.5
            while time.monotonic()<deadline:
                pixels = shot('app-valid-frame')
                magenta = sum(pixels[i:i+3]==bytes([240,16,224]) for i in range(0,len(pixels),3))
                if magenta>1280*720*.85: break
                time.sleep(.1)
            result['visible_app_pixel_ratio']=magenta/(1280*720)
            check('app-frame-reaches-scanout', magenta>1280*720*.85)
            wait('hostile: CORRUPTION_STARTED')
            time.sleep(.1)
            check('console-survives-active-truncation', 'CONSOLE_DURING_CORRUPTION' in session.shell('echo CONSOLE_DURING_CORRUPTION'))
            shell = session.shell(f"cat /proc/{pid}/status; stat -c '%u:%g %a %n' /run/tv-core /run/tv-core/menu.frames /dev/fb0")
            (out/'app-boundary.txt').write_text(shell)
            check('confined-uid-and-seccomp', re.search(r'Uid:\s+10001\s+10001',shell) and re.search(r'Seccomp:\s+2',shell))
            check('menu-and-framebuffer-remain-root-only', all(line in shell for line in [
                '0:0 700 /run/tv-core', '0:0 600 /run/tv-core/menu.frames', '0:0 600 /dev/fb0']))
            session.event('home', True)
            start = time.monotonic()
            deadline = start+3
            while time.monotonic()<deadline:
                current=status()
                if current['display_source']=='/run/tv-core/menu.frames': break
                time.sleep(.01)
            else: raise AssertionError('Home did not revoke app presentation')
            result['home_revocation_seconds'] = time.monotonic()-start
            wait('hostile: WRITE_AFTER_TERM', 3)
            during = shot('home-while-app-still-writing')
            session.event('home', False)
            check('home-scanout-matches-private-menu-while-child-writes', menu_region(during)==menu_region(home))
            live = session.shell(f'cat /proc/{pid}/status; echo CONSOLE_AFTER_HOME')
            (out/'child-after-home.txt').write_text(live)
            check('console-survives-home-before-kill', 'CONSOLE_AFTER_HOME' in live)
            check('child-is-still-live-after-home-scanout', re.search(r'State:\s+[RS]',live))
            text = wait(f'Lean PID1 reaped child {pid} status=9', 10)
            result['kill_wait_status']=9
            result['child_reap_seconds_after_home'] = time.monotonic()-start
            check('term-ignored-then-kill-observed', 'hostile: IGNORED_TERM' in text and 'hostile: WRITE_AFTER_TERM' in text)
            check('writer-is-reaped', 'CHILD_GONE' in session.shell(f'test ! -e /proc/{pid} && echo CHILD_GONE'))
            after = shot('home-after-kill')
            check('menu-stays-visible-after-kill', menu_region(after)==menu_region(home))
            final=status()
            check('pid1-and-http-survive', final['pid']==1 and final['phase']=='shell-ready' and final['uptime_ms']>initial['uptime_ms'])
            check('no-kernel-panic', 'Kernel panic' not in session.serial())
            (out/'status.json').write_text(json.dumps(final,indent=2)+'\n')
            result['passed']=True
            print('PASS: adversarial Core display ownership; evidence '+str(out), flush=True)
        finally:
            (out/'result.json').write_text(json.dumps(result,indent=2)+'\n')
            if session.wire is not None: session.wire.close(); session.sock.close()
            if process.poll() is None:
                process.terminate()
                try: process.wait(timeout=5)
                except subprocess.TimeoutExpired: process.kill(); process.wait()
            qmp.unlink(missing_ok=True)


if __name__=='__main__': main()
