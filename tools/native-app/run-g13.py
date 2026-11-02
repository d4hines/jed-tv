#!/usr/bin/env python3
"""Run tvinit and its apps under stock G13 Android; no flash or reboot."""
import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import shlex
import socket
import subprocess
import sys
import time
from g13_device import SERIAL, TARGET, adb, root as ensure_root

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'assets/build/native-app-g13'
BASE = '/data/local/tmp/dande-g13'
APP_UIDS = [19001, 19002, 19003, 19005, 19006, 19007, 19008]
APP_BINARIES = ('tvinit', 'ytplay', 'slides', 'doomtv', 'tvosd', 'rpgtv', 'flighttv', 'snaketv')


def call(args, **kwargs):
    return subprocess.run([str(a) for a in args], check=True, **kwargs)


def shell(command):
    return adb('shell', command, capture_output=True, text=True, timeout=30).stdout


def push(source, destination):
    adb('push', source, destination, capture_output=True, timeout=120)


def root(allow_native=False):
    ensure_root()
    state = shell('id -u; getprop ro.product.device; getprop ro.build.version.incremental; getprop sys.boot_completed')
    if state.splitlines() != ['0', 'G13', 'AU13', '1']:
        raise RuntimeError(f'Expected rooted, booted G13/AU13: {state!r}')
    if not allow_native and shell('getprop sys.dande.native').strip() == '1':
        raise RuntimeError('Native boot already manages this session. Select Android with '
                           'tools/native-app/g13-boot.py android and reboot before using the manual runner.')


def build():
    result = call(['nix', 'build', '.#g13-apps', '.#pid1.passthru.media',
                   '.#native-app-tools', '--no-link', '--json'], cwd=ROOT,
                  stdout=subprocess.PIPE, text=True)
    paths = [Path(item['outputs']['out']) for item in json.loads(result.stdout)]
    apps = next(p for p in paths if (p / 'bin/tvinit').is_file())
    media = next(p for p in paths if (p / 'player.json').is_file())
    tools = next(p for p in paths if (p / 'bin/rustc').is_file())
    libraries = OUT / 'libraries'
    libraries.mkdir(exist_ok=True)
    for name in ['libc.so', 'libdl.so', 'libm.so', 'liblog.so']:
        source = ('/system/lib/' if name == 'liblog.so' else '/apex/com.android.runtime/lib/bionic/') + name
        adb('pull', source, libraries / name, capture_output=True, timeout=30)
    env = {**os.environ, 'TV_RUSTC': str(tools / 'bin/rustc'),
           'TV_ANDROID_CLANG': str(tools / 'bin/clang'),
           'TV_ANDROID_READELF': str(tools / 'bin/readelf'),
           'TV_ANDROID_LIB_DIR': str(libraries), 'TV_AUDIO_BUILD_DIR': str(OUT / 'objects')}
    display_archive = Path(call([sys.executable, 'tools/display/lean-library.py', '--target', 'armv7-linux-androideabi'],
                                cwd=ROOT, env=env, capture_output=True, text=True).stdout.strip())
    call(['sh', 'tools/audio/rust-build.sh', 'src/pid1/display/main.rs', OUT / 'native-display',
          '-L', f'native={display_archive.parent}', '-l', 'static=tv_display_lean'], cwd=ROOT, env=env)
    call(['sh', 'tools/audio/rust-build.sh', 'src/pid1/audio/bridge.rs', OUT / 'native-audio'], cwd=ROOT, env=env)
    rust = tools / 'bin/rustc'
    sysroot = call([rust, '--print', 'sysroot'], capture_output=True, text=True).stdout.strip()
    info = call([rust, '-vV'], capture_output=True, text=True).stdout
    host = next(line.split(': ', 1)[1] for line in info.splitlines() if line.startswith('host: '))
    linker = Path(sysroot) / 'lib/rustlib' / host / 'bin/rust-lld'
    for source, name in [('research/probes/android-client.rs', 'android-client'),
                         ('research/probes/adb-relay.rs', 'adb-relay'),
                         ('research/probes/audio/protocol-probe.rs', 'audio-protocol-probe')]:
        call([rust, '--edition=2021', '--target', 'armv7-unknown-linux-musleabihf',
              '-C', f'linker={linker}', '-C', 'target-cpu=cortex-a55', '-C', 'opt-level=2',
              '-C', 'panic=abort', ROOT / source, '-o', OUT / name])
    record = {'apps': str(apps), 'media': str(media), 'tools': str(tools)}
    (OUT / 'build.json').write_text(json.dumps(record, indent=2) + '\n')
    return record


def stop():
    # Match executable paths, rather than a process name or a stale saved PID.
    command = f'''for pid in $(pidof {' '.join(APP_BINARIES)} native-display native-audio audio-protocol-probe android-client adb-relay); do
target=$(readlink "/proc/$pid/exe" 2>/dev/null) || continue
case "$target" in {BASE}/*)
kill -TERM "$pid" 2>/dev/null || true;; esac
done'''
    shell(command)
    time.sleep(1)
    shell(command.replace('kill -TERM', 'kill -KILL'))
    saved = OUT / 'host-services.json'
    if saved.is_file():
        for name, item in json.loads(saved.read_text()).items():
            try:
                cmdline = Path(f'/proc/{item["pid"]}/cmdline').read_bytes().split(b'\0')
                expected = str(ROOT / 'tools/native-app' / ('tunnel.py' if name == 'proxy' else 'replay.py')).encode()
                if expected in cmdline and str(item['port']).encode() in cmdline:
                    os.kill(item['pid'], 15)
            except (FileNotFoundError, ProcessLookupError):
                pass
        for endpoint in ['tcp:8090', 'tcp:8899', 'localabstract:dande-g13-proxy', 'localabstract:dande-g13-replay']:
            subprocess.run(['adb', '-s', TARGET, 'reverse', '--remove', endpoint], capture_output=True)
        saved.unlink()


def stage(record):
    apps, media = Path(record['apps']), Path(record['media'])
    if json.loads((apps / 'app-uids.json').read_text()) != APP_UIDS:
        raise RuntimeError('Rebuild with .#g13-apps for the G13 application identities')
    packages = shell('cat /data/system/packages.list')
    if any(int(line.split()[1]) in APP_UIDS for line in packages.splitlines()):
        raise RuntimeError('G13 application UID collides with an installed Android package')
    stop()
    previous = OUT / 'previous-logs' / str(time.time_ns())
    previous.parent.mkdir(exist_ok=True)
    subprocess.run(['adb', '-s', TARGET, 'pull', f'{BASE}/logs', str(previous)], capture_output=True)
    shell(f'umask 022; mkdir -p {BASE}/logs {BASE}/root/bin {BASE}/root/tmp {BASE}/root/dev/input '
          f'{BASE}/root/proc {BASE}/root/run {BASE}/root/etc {BASE}/root/data/apps '
          f'{BASE}/root/share/tiles {BASE}/root/doom; chmod 755 {BASE} {BASE}/root; '
          f'chmod 1777 {BASE}/root/tmp; '
          f'test -f {BASE}/root/tmp/app-volume || echo 0.0075 > {BASE}/root/tmp/app-volume')
    directories = ['', '/bin', '/dev', '/dev/input', '/proc', '/run', '/etc', '/data', '/data/apps', '/share', '/share/tiles', '/doom']
    shell('chmod 755 ' + ' '.join(BASE + '/root' + p for p in directories) +
          f'; chmod 644 {BASE}/root/tmp/app-volume; chown 0:0 {BASE}/root/tmp/app-volume')
    shell(f'rm -f {BASE}/logs/*.log')
    for name in APP_BINARIES:
        push(apps / 'bin' / name, f'{BASE}/root/bin/{name}')
    for name in ['native-display', 'native-audio', 'android-client', 'adb-relay', 'audio-protocol-probe']:
        push(OUT / name, f'{BASE}/{name}')
    push(media / 'doom/freedoom1.wad', f'{BASE}/root/doom/freedoom1.wad')
    for tile in sorted((media / 'tiles').glob('*.rgb')):
        push(tile, f'{BASE}/root/share/tiles/{tile.name}')
    push(ROOT / 'src/pid1/scripts/native-app/start-android.sh', f'{BASE}/start.sh')
    push(ROOT / 'src/pid1/scripts/native-app/supervise-android.sh', f'{BASE}/supervise.sh')
    shell(f'chmod 755 {BASE}/root/bin/* {BASE}/native-* {BASE}/android-client {BASE}/adb-relay {BASE}/audio-protocol-probe; '
          f'restorecon -RF {BASE}')
    expected = {f'{BASE}/root/bin/{name}': hashlib.sha256((apps / 'bin' / name).read_bytes()).hexdigest()
                for name in APP_BINARIES}
    expected.update({f'{BASE}/{name}': hashlib.sha256((OUT / name).read_bytes()).hexdigest()
                     for name in ['native-display', 'native-audio', 'android-client', 'adb-relay', 'audio-protocol-probe']})
    observed = dict(line.split()[::-1] for line in shell('sha256sum ' + ' '.join(expected)).splitlines())
    if observed != expected:
        raise RuntimeError('Uploaded binary hash mismatch')
    (OUT / 'device-sha256.json').write_text(json.dumps(observed, indent=2) + '\n')


def client(program, *args, log):
    command = [f'{BASE}/android-client', f'{BASE}/logs/{log}', program, *args]
    return shell('LD_LIBRARY_PATH=/system/lib:/apex/com.android.art/lib:/apex/com.android.i18n/lib:/apex/com.android.vndk.v34/lib '
                 + ' '.join(shlex.quote(a) for a in command))


def service(script, arguments, name):
    with socket.socket() as sock:
        sock.bind(('127.0.0.1', 0))
        port = sock.getsockname()[1]
    with (OUT / f'{name}.log').open('ab') as log:
        process = subprocess.Popen([sys.executable, str(ROOT / script), *arguments,
                                    '--host', '127.0.0.1', '--port', str(port)],
                                   cwd=ROOT, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
    for _ in range(50):
        if process.poll() is not None:
            raise RuntimeError(f'{name} stopped; see {OUT / (name + ".log")}')
        try:
            with socket.create_connection(('127.0.0.1', port), timeout=.1):
                return {'pid': process.pid, 'port': port}
        except OSError:
            time.sleep(.1)
    raise RuntimeError(f'{name} did not listen')


def start_network(record):
    # The recovered TV's offline RTC is in 2025; TLS needs the current UTC time.
    now = datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%d %H:%M:%S')
    shell('date -u -s ' + shlex.quote(now))
    media = Path(record['media'])
    saved = OUT / 'host-services.json'
    services = json.loads(saved.read_text()) if saved.is_file() else {}
    for name, arguments in [('proxy', []), ('replay', ['--player', str(media / 'player.json'), '--fixtures', str(media / 'fixtures')])]:
        script = 'tunnel.py' if name == 'proxy' else 'replay.py'
        item = services.get(name)
        reusable = False
        if item:
            try:
                cmdline = Path(f'/proc/{item["pid"]}/cmdline').read_bytes().split(b'\0')
                expected = str(ROOT / 'tools/native-app' / script).encode()
                if expected in cmdline and str(item['port']).encode() in cmdline:
                    with socket.create_connection(('127.0.0.1', item['port']), timeout=1):
                        reusable = True
            except (OSError, KeyError):
                pass
        if not reusable:
            services[name] = service('tools/native-app/' + script, arguments, name)
        (OUT / 'host-services.json').write_text(json.dumps(services, indent=2) + '\n')
        adb('reverse', f'localabstract:dande-g13-{name}', f'tcp:{services[name]["port"]}', capture_output=True)


def start(record):
    start_network(record)
    print(client('/system/bin/sh', '-c', 'id; wm size', log='android-client.log'))
    print(client(f'{BASE}/native-display', '--hidden-probe', log='display-probe.log'))
    shell(f'nohup /system/bin/sh {BASE}/supervise.sh > {BASE}/logs/supervisor.log 2>&1 < /dev/null &')
    deadline = time.monotonic() + 30
    while time.monotonic() < deadline:
        if 'home screen:' in shell(f'cat {BASE}/logs/menu.log 2>/dev/null || true'):
            break
        time.sleep(.25)
    else:
        raise RuntimeError(shell(f'cat {BASE}/logs/supervisor.log {BASE}/logs/menu.log 2>/dev/null || true'))
    deadline = time.monotonic() + 15
    while time.monotonic() < deadline:
        logs = shell(f'cat {BASE}/logs/menu.log {BASE}/logs/display.log {BASE}/logs/audio.log 2>/dev/null || true')
        if 'NATIVE_DISPLAY frames=' in logs and 'RUST_AUDIO ready' in logs:
            break
        time.sleep(.25)
    print(logs)
    (OUT / 'startup.log').write_text(logs)
    if 'NATIVE_DISPLAY frames=' not in logs or 'RUST_AUDIO ready' not in logs:
        raise RuntimeError('Display frame or audio startup not confirmed; inspect saved logs')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--stop', action='store_true')
    parser.add_argument('--prepare-only', action='store_true')
    parser.add_argument('--no-build', action='store_true')
    parser.add_argument('--network-only', action='store_true',
                        help='restore host proxy/replay and clock for the running native boot')
    args = parser.parse_args()
    if args.network_only and (args.stop or args.prepare_only or args.no_build):
        parser.error('--network-only is a standalone action using the existing build record')
    OUT.mkdir(parents=True, exist_ok=True)
    root(allow_native=args.network_only)
    if args.network_only:
        start_network(json.loads((OUT / 'build.json').read_text()))
        print(f'Host proxy/replay ready over Ethernet ADB at {TARGET}; launcher left running.')
        return
    if args.stop:
        stop()
        print('Stopped G13 apps and helpers; Android remains running.')
        return
    record = json.loads((OUT / 'build.json').read_text()) if args.no_build else build()
    stage(record)
    if not args.prepare_only:
        try:
            start(record)
        except BaseException:
            subprocess.run(['adb', '-s', TARGET, 'pull', f'{BASE}/logs', str(OUT / 'failed-logs')], capture_output=True)
            stop()
            raise


if __name__ == '__main__':
    main()
