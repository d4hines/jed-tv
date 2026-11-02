#!/usr/bin/env python3
"""Run a Lean app as /bin/ytplay in the graphical QEMU demo (virt32
surrogate, the real launcher and confinement), launch it from the home
screen, take a screenshot while it runs and print the serial log.

    test/lean/yt/demo-app.py APP_BINARY --apps DEMO_APPS --media DEMO_MEDIA [--seconds N] [--shot PNG] [--keys k1,k2]

DEMO_APPS/DEMO_MEDIA: outputs of the Nix demo (demo/package.nix: the
pid1-demo-apps and pid1-demo-media store paths).
"""
import argparse, http.server, json, pathlib, socket, subprocess, sys, tempfile, threading, time

ROOT = pathlib.Path(__file__).resolve().parents[3]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('app', type=pathlib.Path)
    ap.add_argument('--apps', required=True, type=pathlib.Path)
    ap.add_argument('--media', required=True, type=pathlib.Path)
    ap.add_argument('--seconds', type=float, default=4)
    ap.add_argument('--shot', type=pathlib.Path, default=ROOT / 'assets/build/lean-yt/demo-app.png')
    ap.add_argument('--keys', default='', help='qcodes to send after launch, comma separated; sleep:N waits N s')
    ap.add_argument('--launch', default='ret', help='qcodes on the home screen that launch /bin/ytplay (down,ret: the YouTube tile)')
    ap.add_argument('--append', default='', help='extra kernel command line')
    ap.add_argument('--shot-after', type=float, default=0, help='seconds between the last key and the screenshot')
    ap.add_argument('--live', action='store_true', help='real YouTube through a CONNECT proxy on the host (TLS in the app)')
    ap.add_argument('--cast-remote', action='store_true',
                    help='with --live: when the app shows its TV code, link research/dial-lounge/fake_remote.py to it')
    ap.add_argument('--window', action='store_true',
                    help='interactive: a QEMU window with host audio; runs until you close it')
    ap.add_argument('--bridge', action='store_true',
                    help="the TV's backends: bridge-init (next to --tvinit) as PID 1 with shared frames and a TCP PCM stub")
    ap.add_argument('--bridge-init', type=pathlib.Path, help='bridge-init binary (default: next to --tvinit)')
    ap.add_argument('--wait-for', default='home screen: 3 rows', help='serial marker before sending launch keys')
    ap.add_argument('--audio-out', type=pathlib.Path, help='keep the guest audio (WAV) here')
    ap.add_argument('--tvinit', type=pathlib.Path, help='launcher binary (default: DEMO_APPS/bin/tvinit)')
    a = ap.parse_args()
    if a.live:
        # tools/native-app/tunnel.py's CONNECT handler (YouTube hosts only),
        # on the host loopback that QEMU's 192.168.77.1 reaches.
        sys.path.insert(0, str(ROOT / 'tools/native-app'))
        import tunnel
        server = tunnel.Server(('127.0.0.1', 0), tunnel.Tunnel)
        a.append += f' ytplay.server=https://www.youtube.com TV_HTTP_PROXY=192.168.77.1:{server.server_address[1]}'
    else:
        # Offline YouTube: test/replay.py plus search and thumbnails (server.py).
        sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent))
        import server as offline
        server = http.server.ThreadingHTTPServer(('127.0.0.1', 0), offline.make_handler(a.media))
        a.append += f' ytplay.server=http://192.168.77.1:{server.server_port}'
    threading.Thread(target=server.serve_forever, daemon=True).start()
    with tempfile.TemporaryDirectory(prefix='lean-yt-demo-') as temp:
        out = pathlib.Path(temp)
        serial, qmp, rd = out / 'serial.log', out / 'qmp', out / 'initramfs.cpio'
        files = [f'{a.app}:/bin/ytplay', f'{a.apps}/bin/slides:/bin/slides', f'{a.apps}/bin/doomtv:/bin/doomtv']
        files += [f'{p}:/share/tiles/{p.name}' for p in sorted((a.media / 'tiles').glob('*.rgb'))]
        init = a.tvinit or a.apps / 'bin/tvinit'
        pcm = None
        if a.bridge:
            sys.path.insert(0, str(ROOT / 'demo'))
            from pcm_stub import PcmStub
            pcm = PcmStub()
            files.append(f'{init}:/bin/tvinit')
            init = a.bridge_init or pathlib.Path(init).parent / 'bridge-init'
            a.append += f' bridge.audio=192.168.77.1:{pcm.server_address[1]}'
        subprocess.run([sys.executable, str(ROOT / 'tools/mkinitramfs.py'), str(init), str(rd), *files], check=True)
        cmd = ['qemu-system-aarch64', '-M', 'virt-11.1,highmem=off,gic-version=2', '-cpu', 'cortex-a55,aarch64=off',
               '-smp', '4', '-m', '1430M', '-kernel', str(a.media / 'kernel/zImage'), '-initrd', str(rd),
               '-append', 'console=ttyAMA0 panic=-1 ip=off drm_kms_helper.drm_fbdev_overalloc=200 tvinit.demo=1 '
                          'tvinit.iface=eth0 tvinit.ip=192.168.77.2/24 ' + a.append,
               '-global', 'virtio-mmio.force-legacy=false', '-netdev', 'user,id=n0,net=192.168.77.0/24,host=192.168.77.1',
               '-device', 'virtio-net-device,netdev=n0', '-device', 'virtio-gpu-device,xres=1280,yres=720',
               '-audiodev', ('coreaudio,id=a0' if a.window else f'wav,id=a0,path={out}/audio.wav'),
               '-device', 'virtio-sound-device,audiodev=a0,streams=1',
               '-device', 'virtio-keyboard-device', '-device', 'virtio-rng-device',
               '-serial', f'file:{serial}', '-qmp', f'unix:{qmp},server=on,wait=off', '-no-reboot',
               '-display', 'cocoa' if a.window else 'none']
        p = subprocess.Popen(cmd, stdout=open(out / 'qemu.log', 'w'), stderr=subprocess.STDOUT)
        try:
            def text():
                return serial.read_text(errors='replace') if serial.exists() else ''
            def wait(marker, timeout=40):
                until = time.monotonic() + timeout
                while time.monotonic() < until:
                    if marker in text(): return
                    if p.poll() is not None: break
                    time.sleep(.1)
                raise RuntimeError(f'timed out waiting for {marker!r}\n' + text()[-3000:])
            wait(a.wait_for)
            s = socket.socket(socket.AF_UNIX); s.settimeout(5); s.connect(str(qmp)); f = s.makefile('rw'); f.readline()
            def q(name, **args):
                f.write(json.dumps({'execute': name, 'arguments': args}) + '\n'); f.flush()
                while True:
                    r = json.loads(f.readline())
                    if 'return' in r: return r['return']
                    if 'error' in r: raise RuntimeError(str(r))
            q('qmp_capabilities')
            def key(code): q('send-key', keys=[{'type': 'qcode', 'data': code}]); time.sleep(.25)
            for k in filter(None, a.launch.split(',')): key(k)
            wait('(/bin/ytplay ')
            wait('app 1: confined pid')
            if a.window:
                print('[demo-app] the Lean YouTube app is open in the QEMU window; close it to finish', flush=True)
                p.wait()
                return
            remote = None
            if a.cast_remote:
                import re
                until = time.monotonic() + 60
                code = None
                while time.monotonic() < until and not code:
                    m = re.search(r'ytlean: phone link code (\d{12})', text())
                    code = m and m.group(1)
                    time.sleep(.5)
                if code:
                    remote = subprocess.Popen([sys.executable, str(ROOT / 'research/dial-lounge/fake_remote.py'),
                                               '--code', code, '--name', 'fake phone (QEMU test)'],
                                              stdout=open(out / 'remote.log', 'w'), stderr=subprocess.STDOUT,
                                              cwd=str(ROOT / 'research/dial-lounge'))
            time.sleep(a.seconds / 2)
            for k in filter(None, a.keys.split(',')):
                if k.startswith('sleep:'): time.sleep(float(k[6:]))
                else: key(k)
            time.sleep(a.shot_after)
            a.shot.parent.mkdir(parents=True, exist_ok=True)
            q('screendump', filename=str(a.shot), format='png')
            if not any(marker in text() for marker in ('ytlean:', 'hello-tv:')):
                raise RuntimeError('Launcher spawned app 1, but the Lean app never reached its entry log')
            time.sleep(a.seconds / 2 + 2)
        finally:
            if 'remote' in locals() and remote is not None and remote.poll() is None:
                remote.terminate()
                remote.wait(timeout=10)
            if p.poll() is None:
                try:
                    if 'f' in locals():
                        f.write(json.dumps({'execute': 'quit'}) + '\n'); f.flush()
                    else:
                        p.terminate()
                    p.wait(timeout=5)
                except (OSError, subprocess.TimeoutExpired):
                    p.kill(); p.wait()
            print(text())
            print(f'[demo-app] screenshot: {a.shot}')
            if pcm:
                print(f'[demo-app] pcm stub: opens {pcm.opens}, frames {pcm.frames}, resets {pcm.resets}, errors {pcm.errors}')
            if (out / 'remote.log').exists():
                print('[demo-app] remote log:')
                print((out / 'remote.log').read_text())
            if a.audio_out and (out / 'audio.wav').exists():
                import shutil
                a.audio_out.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(out / 'audio.wav', a.audio_out)
                print(f'[demo-app] audio: {a.audio_out}')


if __name__ == '__main__':
    main()
