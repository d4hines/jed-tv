#!/usr/bin/env python3
"""Headless QEMU check of the Lean Doom app under the real launcher.

  python3 test/doomtv/check-qemu.py --apps DIR --media DIR [--seconds N]

Boots the graphical virt32 surrogate (same machine as demo/run.py) with the
given app set, opens Doom from the home screen, plays for a while with QMP key
presses, and saves the serial log, timed screenshots and a summary under
assets/build/lean-doom/qemu-<time>/. Fails if Doom does not start, reports an
engine error, stops running tics, or the screen stays blank. TCG timing on the
Mac is not TV timing.
"""
import argparse, json, pathlib, socket, struct, subprocess, sys, tempfile, time, zlib

ROOT = pathlib.Path(__file__).resolve().parents[2]

def png_stats(path):
    """(width, height, distinct sampled colours) of a QEMU RGB PNG."""
    data = path.read_bytes()
    w, h = struct.unpack_from('>II', data, 16)
    idat, pos = b'', 8
    while pos < len(data):
        n, kind = struct.unpack_from('>I4s', data, pos)
        if kind == b'IDAT':
            idat += data[pos + 8:pos + 8 + n]
        pos += 12 + n
    raw = zlib.decompress(idat)
    stride = 1 + 3 * w
    colours = set()
    for y in range(0, h, 23):
        row = raw[y * stride + 1:(y + 1) * stride]  # filter byte ignored; enough for a variety count
        for x in range(0, w, 37):
            colours.add(row[3 * x:3 * x + 3])
    return w, h, len(colours)

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--apps', required=True, type=pathlib.Path)
    ap.add_argument('--media', required=True, type=pathlib.Path)
    ap.add_argument('--seconds', type=float, default=40)
    ap.add_argument('--wad', default=None, help='IWAD to install as /doom/freedoom1.wad')
    ap.add_argument('--extra', default='', help='extra kernel command line words')
    ap.add_argument('--attract', action='store_true', help='press nothing in Doom: watch the title pages and demos')
    ap.add_argument('--saveload', action='store_true', help='after playing, save to slot 1, play on, then load it')
    ap.add_argument('--out', type=pathlib.Path, default=ROOT / 'assets/build/lean-doom',
                    help='parent directory of the qemu-<time> output directory')
    a = ap.parse_args()
    out = a.out / time.strftime('qemu-%Y%m%dT%H%M%S')
    out.mkdir(parents=True)
    serial, rd = out / 'serial.log', out / 'initramfs.cpio'
    qmp = pathlib.Path(tempfile.mkdtemp(prefix='ld-', dir='/tmp')) / 'qmp'  # UNIX socket paths are short
    wad = a.wad or f'{a.media}/doom/freedoom1.wad'
    files = [f'{a.apps}/bin/{n}:/bin/{n}' for n in ['ytplay', 'slides', 'doomtv']]
    files += [f'{wad}:/doom/freedoom1.wad']
    files += [f'{p}:/share/tiles/{p.name}' for p in sorted((a.media / 'tiles').glob('*.rgb'))]
    subprocess.run([sys.executable, str(ROOT / 'tools/mkinitramfs.py'), str(a.apps / 'bin/tvinit'), str(rd), *files],
                   check=True, stdout=subprocess.DEVNULL)
    cmd = ['qemu-system-aarch64', '-M', 'virt-11.1,highmem=off,gic-version=2', '-cpu', 'cortex-a55,aarch64=off',
           '-smp', '4', '-m', '1430M', '-kernel', str(a.media / 'kernel/zImage'), '-initrd', str(rd),
           '-append', 'console=ttyAMA0 panic=-1 ip=off drm_kms_helper.drm_fbdev_overalloc=200 tvinit.demo=1 hello.qemu=1 ' + a.extra,
           '-global', 'virtio-mmio.force-legacy=false', '-device', 'virtio-gpu-device,xres=1280,yres=720',
           '-audiodev', f'wav,id=a0,path={out}/audio.wav', '-device', 'virtio-sound-device,audiodev=a0,streams=1',
           '-device', 'virtio-keyboard-device', '-device', 'virtio-rng-device',
           '-serial', f'file:{serial}', '-qmp', f'unix:{qmp},server=on,wait=off', '-no-reboot', '-display', 'none']
    p = subprocess.Popen(cmd, stdout=(out / 'qemu.log').open('w'), stderr=subprocess.STDOUT)
    summary = {'out': str(out), 'shots': []}
    prev = 3
    try:
        def text():
            return serial.read_text(errors='replace') if serial.exists() else ''
        def wait(marker, timeout=60):
            until = time.monotonic() + timeout
            while time.monotonic() < until:
                t = text()
                if marker in t:
                    return
                if 'doom: engine error' in t or 'doom: render error' in t or p.poll() is not None:
                    raise RuntimeError('Doom failed:\n' + t[-3000:])
                time.sleep(0.2)
            raise RuntimeError(f'timed out waiting for {marker!r}\n' + text()[-3000:])
        wait('home screen: 3 rows', 90)
        s = socket.socket(socket.AF_UNIX); s.settimeout(10); s.connect(str(qmp)); f = s.makefile('rw'); f.readline()
        def q(name, **args):
            f.write(json.dumps({'execute': name, 'arguments': args}) + '\n'); f.flush()
            while True:
                r = json.loads(f.readline())
                if 'return' in r: return r['return']
                if 'error' in r: raise RuntimeError(str(r))
        q('qmp_capabilities')
        def key(code, hold=0.1):
            q('input-send-event', events=[{'type': 'key', 'data': {'down': True, 'key': {'type': 'qcode', 'data': code}}}])
            time.sleep(hold)
            q('input-send-event', events=[{'type': 'key', 'data': {'down': False, 'key': {'type': 'qcode', 'data': code}}}])
            time.sleep(0.2)
        def shot(name):
            path = out / f'{name}.png'
            q('screendump', filename=str(path), format='png')
            time.sleep(0.3)
            w, h, colours = png_stats(path)
            summary['shots'].append({'name': name, 'size': [w, h], 'colours': colours})
            return colours
        key('down'); key('right'); key('right'); key('ret')  # demo row first; Doom is row 2, tile 3
        wait('doom: engine up', 120)
        if a.attract:
            for t in (3, 9, 16, 24, 32):
                time.sleep(t - (0 if t == 3 else prev)); prev = t
                shot(f'attract-{t:02d}s')
            wait('doom: demo DEMO1', 5)
            # Exercise normal exit too: the independent PCM worker must join.
            key('esc'); key('q'); key('ret'); key('ret')
            wait('Doom exited (0)', 10)
            summary['pass'] = True
            return
        time.sleep(3); shot('title')
        # Title -> menu -> New Game -> single player -> episode 1 (Doom 1 IWADs) -> default skill.
        key('ret'); time.sleep(1.5); shot('menu')
        key('ret'); time.sleep(1.5); shot('gametype')
        key('ret'); time.sleep(1.5); shot('episode')
        key('ret'); time.sleep(1.5); shot('skill')
        key('ret')
        wait('doom: level ', 60)
        t0 = time.monotonic()
        time.sleep(4); shot('start')
        key('up', 2.0); shot('walk')
        key('ctrl', 0.3); time.sleep(0.5); shot('fire')
        key('left', 0.8); key('up', 1.5); shot('turn')
        # Automap (Tab = the remote's Blue): open, zoom in, close.
        key('tab'); time.sleep(1); shot('automap')
        key('equal', 1.5); time.sleep(0.5); shot('automap-zoom')
        key('tab'); time.sleep(1); shot('after-automap')
        while time.monotonic() - t0 < a.seconds:
            time.sleep(1)
        shot('end')
        if a.saveload:
            # Menu (cursor on New Game) -> down 3 = Save Game -> slot 1.
            key('esc'); time.sleep(0.8)
            for _ in range(3): key('down')
            key('ret'); time.sleep(0.8); shot('save-menu')
            key('ret'); time.sleep(1.0); shot('saved')
            key('up', 1.5); key('left', 0.8); time.sleep(1.0); shot('moved-on')
            # Menu remembers Save Game; up = Load Game -> slot 1.
            key('esc'); time.sleep(0.8); key('up'); key('ret'); time.sleep(0.8); shot('load-menu')
            key('ret'); time.sleep(1.5); shot('loaded')
        stats = [l for l in text().splitlines() if 'tics/s' in l]
        summary['stats'] = stats[-3:]
        if not stats:
            raise RuntimeError('no tic-rate reports\n' + text()[-3000:])
        if max(s['colours'] for s in summary['shots']) < 20:
            raise RuntimeError(f'screens look blank: {summary["shots"]}')
        key('home'); time.sleep(3); shot('home')
        summary['pass'] = True
    except Exception as e:
        summary['pass'] = False
        summary['error'] = str(e)[-4000:]
    finally:
        if p.poll() is None:
            try:
                if 'f' in locals():
                    f.write(json.dumps({'execute': 'quit'}) + '\n'); f.flush()
                else:
                    p.terminate()
                p.wait(timeout=5)
            except (OSError, subprocess.TimeoutExpired):
                p.kill(); p.wait()
        (out / 'summary.json').write_text(json.dumps(summary, indent=1))
    doom_lines = [l for l in (serial.read_text(errors='replace') if serial.exists() else '').splitlines() if 'doom' in l.lower()][:40]
    print('\n'.join(doom_lines))
    print(json.dumps(summary, indent=1))
    return 0 if summary['pass'] else 1

if __name__ == '__main__':
    sys.exit(main())
