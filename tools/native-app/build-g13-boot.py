#!/usr/bin/env python3
"""Prepare a G13/AU13 native-startup candidate. No device writes or reboot."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import runpy
import shutil
import struct
import subprocess

ROOT = Path(__file__).resolve().parents[2]
ROOT_IMAGE = ROOT / 'assets/research/g13-adb-root-20261101/candidate/init_boot-adb-root.img'
ROOT_SHA = 'afc34332ba1b0f4b737eecdc512016e94bb3b9c058c920f411ef53c0393a70a4'
POLICY_RULES = b'''
; G13 native startup: only our explicitly labelled root shell uses su.
(allow init su (process (transition)))
(allow su shell_exec (file (entrypoint)))
; Config survives stock restorecon_recursive /metadata with its normal label.
(allow init metadata_file (dir (search open read getattr)))
(allow init metadata_file (file (open read getattr map)))
'''


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    original = ROOT_IMAGE.read_bytes()
    if len(original) != 8388608 or sha(original) != ROOT_SHA:
        raise RuntimeError('Previously boot-tested G13 root image does not match')
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    work, check = out / 'work', out / 'verification'
    work.mkdir(); check.mkdir()
    commands = []

    def run(command, cwd=ROOT, env=None):
        command = list(map(str, command))
        result = subprocess.run(command, cwd=cwd, env=env, capture_output=True, text=True)
        commands.append(dict(command=command, cwd=str(cwd), returncode=result.returncode,
                             stdout=result.stdout, stderr=result.stderr))
        (out / 'commands.json').write_text(json.dumps(commands, indent=2) + '\n')
        if result.returncode:
            raise RuntimeError(result.stdout + result.stderr)
        return result.stdout

    old_builder = runpy.run_path(str(ROOT / 'research/g13-adb-root/build-image.py'))
    entries = old_builder['entries']
    magiskboot = ROOT / 'assets/research/g13-magisk-v31.1-20261031/host-validation/magiskboot'
    run([magiskboot, 'unpack', ROOT_IMAGE], work)
    raw = (work / 'ramdisk.cpio').read_bytes()
    before = entries(raw)
    members = {name: body for name, _, body, _ in before}
    if members['adb_debug.prop'] != b'ro.debuggable=1\nro.force.debuggable=1\n':
        raise RuntimeError('Unexpected existing debug properties')
    replacements = {
        'adb_debug.prop': members['adb_debug.prop'] + b'ro.boot.init_rc=/metadata/dande-tv/entry.rc\n',
        'userdebug_plat_sepolicy.cil': members['userdebug_plat_sepolicy.cil'] + POLICY_RULES,
    }
    changed = bytearray()
    for name, fields, body, _ in before:
        body = replacements.get(name, body)
        fields = list(fields)
        fields[6] = len(body)
        changed += b'070701' + b''.join(f'{v:08x}'.encode() for v in fields)
        changed += name.encode() + b'\0'
        changed += b'\0' * (-len(changed) % 4)
        changed += body
        changed += b'\0' * (-len(changed) % 4)
    changed += b'\0' * (-len(changed) % 512)
    (work / 'ramdisk.cpio').write_bytes(changed)
    policy = out / 'userdebug_plat_sepolicy.cil'
    policy.write_bytes(replacements['userdebug_plat_sepolicy.cil'])
    # Same exact captured vendor policy inputs as the successfully booted root
    # image. -N is intentional: this is an owner-debug policy, not production.
    compile_args = json.loads((ROOT / 'assets/research/g13-adb-root-20261101/debug-compile.json').read_text())['args']
    compile_args[1] = str(policy)
    compile_args[compile_args.index('-o') + 1] = str(out / 'compiled-policy')
    compile_args[compile_args.index('-f') + 1] = str(out / 'file-contexts')
    run(compile_args)
    image = out / 'init_boot-native-startup.img'
    run([magiskboot, 'repack', ROOT_IMAGE, image], work)
    built = image.read_bytes()
    if len(built) != len(original) or built[:12] != original[:12] or built[16:4096] != original[16:4096]:
        raise RuntimeError('Unexpected image length/header changes')
    run([magiskboot, 'unpack', image], check)
    after = entries((check / 'ramdisk.cpio').read_bytes())
    if len(after) != len(before):
        raise RuntimeError('Ramdisk member count changed')
    for (name, fields, body, _), (new_name, new_fields, new_body, _) in zip(before, after):
        expected = replacements.get(name, body)
        metadata = list(fields); metadata[6] = len(expected)
        if name != new_name or tuple(metadata) != new_fields or expected != new_body:
            raise RuntimeError(f'Ramdisk round-trip mismatch: {name}')
    scripts = ROOT / 'src/pid1/scripts/native-app'
    config = out / 'config'
    config.mkdir()
    for source, dest in [('android-g13.rc', 'android.rc'), ('native-g13.rc', 'native.rc'),
                         ('early-g13.sh', 'early.sh'), ('boot-g13.sh', 'boot.sh'),
                         ('network-g13.sh', 'network.sh'), ('boot-log-g13.sh', 'boot-log.sh'),
                         ('apps-g13.sh', 'apps.sh')]:
        (config / dest).write_bytes((scripts / source).read_bytes())
    android = (config / 'android.rc').read_bytes()
    (config / 'native-entry.rc').write_bytes(android + b'\nimport /metadata/dande-tv/native.rc\n')
    (config / 'entry.rc').write_bytes(android)
    (config / 'mode').write_text('disabled\n')
    tools = Path(json.loads((ROOT / 'assets/build/native-app-g13/build.json').read_text())['tools'])
    rust = tools / 'bin/rustc'
    sysroot = Path(run([rust, '--print', 'sysroot']).strip())
    host = next(line.split(': ', 1)[1] for line in run([rust, '-vV']).splitlines() if line.startswith('host: '))
    linker = sysroot / 'lib/rustlib' / host / 'bin/rust-lld'
    run([tools / 'bin/clang', '--target=armv7-linux-androideabi34', '-fPIC',
         '-fno-stack-protector', '-O2', '-c', ROOT / 'src/pid1/audio/standalone-app-state.c',
         '-o', out / 'standalone-app-state.o'])
    run([linker, '-flavor', 'gnu', '-m', 'armelf_linux_eabi', '-shared', '-z', 'global',
         out / 'standalone-app-state.o', '-o', config / 'standalone-app-state.so'])
    stock_audio = ROOT / 'assets/research/g13-native-pid1-20261101/audioserver-stock'
    if sha(stock_audio.read_bytes()) != '004de992143d287d53ca555f23a925625062a1c82200cd7bab11cd8c2e8d548b':
        raise RuntimeError('Not the captured G13 AU13 audioserver')
    shutil.copyfile(stock_audio, config / 'audioserver-standalone')
    run(['nix', 'shell', 'nixpkgs#patchelf', '-c', 'patchelf', '--add-needed',
         '/dev/dande-native/standalone-app-state.so', config / 'audioserver-standalone'])
    dynamic = run([tools / 'bin/readelf', '-d', config / 'standalone-app-state.so'])
    if 'GLOBAL' not in dynamic:
        raise RuntimeError('Shim must interpose calls from dlopened audio HAL libraries')
    env = {**os.environ, 'TV_RUSTC': str(rust), 'TV_ANDROID_CLANG': str(tools / 'bin/clang'),
           'TV_ANDROID_READELF': str(tools / 'bin/readelf'),
           'TV_ANDROID_LIB_DIR': str(ROOT / 'assets/build/native-app-g13/libraries'),
           'TV_AUDIO_BUILD_DIR': str(out / 'audio-objects')}
    run(['sh', 'tools/audio/rust-build.sh', 'src/pid1/audio/bridge.rs', out / 'native-audio'], env=env)
    shutil.copyfile(scripts / 'supervise-android.sh', out / 'supervise.sh')
    manifest = dict(image=str(image), image_sha256=sha(built), image_size=len(built),
                    boot_tested=False, flashed=False, installed_config='Android fallback',
                    base_image=str(ROOT_IMAGE), base_sha256=ROOT_SHA,
                    stock_init_sha256=sha(members['init']), unchanged_member_count=len(before)-3,
                    modified_members={n: sha(b) for n,b in replacements.items()},
                    ramdisk_size=struct.unpack_from('<I', built, 12)[0],
                    policy_check='Compiled with matching captured policy and -N (neverallow checks disabled)',
                    limitation='Stock init remains PID 1; native hardware services retained')
    manifest['config_sha256'] = {p.name: sha(p.read_bytes()) for p in sorted(config.iterdir())}
    manifest['runtime_sha256'] = {n: sha((out / n).read_bytes()) for n in ['native-audio', 'supervise.sh']}
    (out / 'manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    print(json.dumps(manifest, indent=2))


if __name__ == '__main__':
    main()
