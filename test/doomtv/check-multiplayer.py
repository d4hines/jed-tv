#!/usr/bin/env python3
"""Two-TV LAN game in QEMU: Lean Doom host and joiner under the real launcher.

  python3 test/doomtv/check-multiplayer.py --apps DIR --media DIR [--seconds N]

Boots two graphical ARM32 surrogates on one virtual LAN (QEMU multicast
socket netdev, no host networking). The host runs with doom.net=host and
doom.netauto=2, the joiner with doom.net=join and finds the host by UDP
broadcast. Both open Doom from the home screen; the game starts when both are
in. Each peer then gets its own movement/fire key presses. Passes when both
logged the same state checksum at every common tic (at least `--min-tics`
worth), with no desync report. Emulated frames take ~250 ms, so both peers
use a larger command window (doom.netlead=24) than the TV default. Saves serial logs and screenshots under
assets/build/lean-doom/mp-<time>/.

--menu starts the game the way players on the TV do (the launcher passes Doom
no arguments): the host picks New Game > host co-op/deathmatch lan game >
episode > skill and presses OK in the lobby once the joiner, who picked
join lan game, is in. --leave join|host then presses Home on that TV: the
host must drop the joiner and play on, or the joiner must end the game and
return to the title pages; either way the remaining app keeps running.
"""
import argparse, json, pathlib, re, socket, subprocess, sys, tempfile, time

ROOT = pathlib.Path(__file__).resolve().parents[2]

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument('--apps', required=True, type=pathlib.Path)
    ap.add_argument('--media', required=True, type=pathlib.Path)
    ap.add_argument('--seconds', type=float, default=40)
    ap.add_argument('--min-tics', type=int, default=350)
    ap.add_argument('--mode', default='coop', choices=['coop', 'dm'])
    ap.add_argument('--port', type=int, default=23420, help='loopback port for the VM-to-VM link')
    ap.add_argument('--menu', action='store_true', help='start the game through the in-game menus')
    ap.add_argument('--leave', choices=['join', 'host'], help='press Home on this peer after the game window')
    a = ap.parse_args()
    out = ROOT / 'assets/build/lean-doom' / time.strftime('mp-%Y%m%dT%H%M%S')
    out.mkdir(parents=True)
    tmp = pathlib.Path(tempfile.mkdtemp(prefix='mp-', dir='/tmp'))
    files = [f'{a.apps}/bin/{n}:/bin/{n}' for n in ['ytplay', 'slides', 'doomtv']]
    files += [f'{a.media}/doom/freedoom1.wad:/doom/freedoom1.wad']
    files += [f'{p}:/share/tiles/{p.name}' for p in sorted((a.media / 'tiles').glob('*.rgb'))]
    rd = out / 'initramfs.cpio'
    subprocess.run([sys.executable, str(ROOT / 'tools/mkinitramfs.py'), str(a.apps / 'bin/tvinit'), str(rd), *files],
                   check=True, stdout=subprocess.DEVNULL)
    host_net = '' if a.menu else 'doom.net=host' + (',dm' if a.mode == 'dm' else '') + ' doom.netauto=2'
    join_net = '' if a.menu else 'doom.net=join'
    peers = {
        # A point-to-point Ethernet link over a loopback TCP socket (QEMU's
        # multicast netdev does not carry traffic reliably on macOS).
        'host': dict(ip='192.168.77.2', mac='52:54:00:77:00:02', netdev=f'socket,id=n0,listen=127.0.0.1:{a.port}',
                     extra=f'{host_net} doom.name=alpha doom.netlead=24'),
        'join': dict(ip='192.168.77.3', mac='52:54:00:77:00:03', netdev=f'socket,id=n0,connect=127.0.0.1:{a.port}',
                     extra=f'{join_net} doom.name=beta doom.netlead=24'),
    }
    procs, qmps = {}, {}
    for name, p in peers.items():
        (out / name).mkdir()
        qmps[name] = tmp / f'{name}.qmp'
        cmd = ['qemu-system-aarch64', '-M', 'virt-11.1,highmem=off,gic-version=2', '-cpu', 'cortex-a55,aarch64=off',
               '-smp', '4', '-m', '1430M', '-kernel', str(a.media / 'kernel/zImage'), '-initrd', str(rd),
               '-append', f'console=ttyAMA0 panic=-1 ip=off drm_kms_helper.drm_fbdev_overalloc=200 tvinit.demo=1 '
                          f'tvinit.iface=eth0 tvinit.ip={p["ip"]}/24 {p["extra"]}',
               '-global', 'virtio-mmio.force-legacy=false',
               '-netdev', p['netdev'], '-device', f'virtio-net-device,netdev=n0,mac={p["mac"]}',
               '-device', 'virtio-gpu-device,xres=1280,yres=720',
               '-audiodev', 'none,id=a0', '-device', 'virtio-sound-device,audiodev=a0,streams=1',
               '-device', 'virtio-keyboard-device', '-device', 'virtio-rng-device',
               '-serial', f'file:{out / name / "serial.log"}', '-qmp', f'unix:{qmps[name]},server=on,wait=off',
               '-no-reboot', '-display', 'none']
        procs[name] = subprocess.Popen(cmd, stdout=(out / name / 'qemu.log').open('w'), stderr=subprocess.STDOUT)
        time.sleep(1.0)  # the listening VM must be up before the other connects
    summary = {'out': str(out), 'mode': a.mode}
    try:
        def text(name):
            f = out / name / 'serial.log'
            return f.read_text(errors='replace') if f.exists() else ''
        def wait(name, marker, timeout):
            until = time.monotonic() + timeout
            while time.monotonic() < until:
                t = text(name)
                if marker in t: return
                if 'engine error' in t or procs[name].poll() is not None:
                    raise RuntimeError(f'{name} failed:\n' + t[-3000:])
                time.sleep(0.3)
            raise RuntimeError(f'{name}: timed out waiting for {marker!r}\n' + text(name)[-3000:])
        conns = {}
        def q(name, cmd, **args):
            f = conns[name]
            f.write(json.dumps({'execute': cmd, 'arguments': args}) + '\n'); f.flush()
            while True:
                r = json.loads(f.readline())
                if 'return' in r: return r['return']
                if 'error' in r: raise RuntimeError(str(r))
        def key(name, code, hold=0.1):
            for down in (True, False):
                q(name, 'input-send-event', events=[{'type': 'key', 'data': {'down': down, 'key': {'type': 'qcode', 'data': code}}}])
                time.sleep(hold if down else 0.15)
        for name in peers:
            wait(name, 'home screen: 3 rows', 120)
            s = socket.socket(socket.AF_UNIX); s.settimeout(10); s.connect(str(qmps[name]))
            conns[name] = s.makefile('rw'); conns[name].readline(); q(name, 'qmp_capabilities')
        for name in peers:  # open Doom: demo row first, Doom is row 2 tile 3
            for k in ('down', 'right', 'right', 'ret'): key(name, k)
        if a.menu:
            for name in peers: wait(name, 'doom: engine up', 120)
            time.sleep(2)
            # Title -> menu -> New Game -> game type (single player is first).
            host_type = ['down'] * (2 if a.mode == 'dm' else 1)
            for k in ['ret', 'ret'] + host_type + ['ret', 'ret', 'ret']: key('host', k); time.sleep(0.5)
            wait('host', 'net: hosting', 30)
            for k in ['ret', 'ret', 'down', 'down', 'down', 'ret']: key('join', k); time.sleep(0.5)
            wait('join', 'net: found alpha', 30)
            time.sleep(3)  # HELLO / LOBBY round trips
            q('host', 'screendump', filename=str(out / 'host' / 'lobby.png'), format='png')
            key('host', 'ret')  # OK in the lobby starts the game
        for name in peers: wait(name, 'net: game started as player', 180)
        t0 = time.monotonic()
        time.sleep(3)
        for name in peers:
            q(name, 'screendump', filename=str(out / name / 'start.png'), format='png')
        key('host', 'up', 1.5); key('join', 'left', 1.0); key('join', 'up', 1.2)
        key('host', 'ctrl', 0.3); key('join', 'ctrl', 0.3)
        key('host', 'right', 0.7); key('host', 'up', 1.0)
        while time.monotonic() - t0 < a.seconds: time.sleep(1)
        for name in peers:
            q(name, 'screendump', filename=str(out / name / 'end.png'), format='png')
        time.sleep(0.5)
        sums = {}
        for name in peers:
            sums[name] = {int(m[1]): int(m[2]) for m in re.finditer(r'net: tic (\d+) sum (\d+)', text(name))}
        common = sorted(set(sums['host']) & set(sums['join']))
        diverged = [t for t in common if sums['host'][t] != sums['join'][t]]
        desync = [n for n in peers if 'net: desync' in text(n)]
        summary.update(common_tics=len(common), last_common_tic=common[-1] if common else None,
                       diverged=diverged[:5], desync_reported=desync,
                       players={n: re.findall(r'net: game started as player \d+, players mask \d+', text(n)) for n in peers},
                       stats={n: [l for l in text(n).splitlines() if 'tics/s' in l][-2:] for n in peers})
        if not common or common[-1] < a.min_tics:
            raise RuntimeError(f'too few common tics: {common[-1:] if common else None}')
        if diverged or desync:
            raise RuntimeError(f'peers diverged at {diverged[:3]} desync={desync}')
        if a.leave:
            stay = 'join' if a.leave == 'host' else 'host'
            before = len(text(stay))
            key(a.leave, 'home')
            wait(a.leave, 'Doom stopped', 30)
            # The joiner sees host silence; the host drops the joiner after its timeout.
            marker = 'net: game ended' if a.leave == 'host' else 'net: player 2 left the game'
            wait(stay, marker, 60)
            time.sleep(8)  # the remaining game goes on (host) or shows the title pages (joiner)
            after = text(stay)[before:]
            if 'Doom stopped' in after or 'engine error' in after:
                raise RuntimeError(f'{stay} stopped after {a.leave} left:\n' + after[-2000:])
            if a.leave == 'join' and not re.search(r'net: tic \d+ sum', after.split(marker, 1)[1]):
                raise RuntimeError('host stopped running tics after the joiner left:\n' + after[-2000:])
            q(stay, 'screendump', filename=str(out / stay / 'after-leave.png'), format='png')
            summary['leave'] = [l for l in after.splitlines() if 'net:' in l][:6]
        summary['pass'] = True
    except Exception as e:
        summary['pass'] = False
        summary['error'] = str(e)[-3000:]
    finally:
        for p in procs.values():
            p.kill(); p.wait()
        (out / 'summary.json').write_text(json.dumps(summary, indent=1))
    for name in peers:
        lines = [l for l in (out / name / 'serial.log').read_text(errors='replace').splitlines() if 'net:' in l or 'doom:' in l]
        print(f'--- {name}'); print('\n'.join(lines[:12]))
    print(json.dumps(summary, indent=1))
    return 0 if summary['pass'] else 1

if __name__ == '__main__':
    sys.exit(main())
