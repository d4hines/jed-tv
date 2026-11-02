#!/usr/bin/env python3
"""Make an offline ARM32 Android 16 native test ramdisk, without the framework."""
import gzip
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import subprocess
import sys

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
WORK = ROOT / 'assets/build/g13-android16'
sys.path.insert(0, str(ROOT / 'tools/bootloader'))
from qemu import cpio, read_cpio, PINS
from storage import require_space


def sha(path):
    with path.open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def policy(donor):
    """Single-domain allow-all policy, only for this isolated QEMU experiment."""
    lines = (donor / 'system/etc/selinux/plat_sepolicy.cil').read_text().splitlines()
    declarations = [line for line in lines if re.match(r'^\((class |classorder |common |classcommon |sid |sidorder )', line)]
    classes = [line.split()[1] for line in lines if line.startswith('(class ')]
    sids = [line.removeprefix('(sid ').removesuffix(')') for line in lines if line.startswith('(sid ')]
    text = '\n'.join(declarations) + '''
(handleunknown deny)
(mls true)
(sensitivity s0)
(sensitivityorder (s0))
(level systemlow (s0))
(levelrange systemrange (systemlow systemlow))
(user u)
(role r)
(role object_r)
(type native_t)
(typealias default_prop)
(typealiasactual default_prop native_t)
(typealias properties_serial)
(typealiasactual properties_serial native_t)
(typealias init)
(typealiasactual init native_t)
(roletype r native_t)
(roletype object_r native_t)
(userrole u r)
(userrole u object_r)
(userlevel u systemlow)
(userrange u systemrange)
'''
    for name in classes:
        text += f'(allow native_t native_t ({name} (all)))\n'
    for sid in sids:
        text += f'(sidcontext {sid} (u {"r" if sid == "kernel" else "object_r"} native_t (systemlow systemlow)))\n'
    for fs in ('proc', 'sysfs', 'selinuxfs', 'binder', 'debugfs', 'rootfs'):
        text += f'(genfscon {fs} / (u object_r native_t (systemlow systemlow)))\n'
    for fs in ('tmpfs', 'devpts'):
        text += f'(fsuse trans {fs} (u object_r native_t (systemlow systemlow)))\n'
    source = WORK / 'native-test-policy.cil'
    source.write_text(text)
    subprocess.run([str(WORK / 'secilc'), '-c', '30', '-o', str(WORK / 'native-test-policy'),
                    '-f', str(WORK / 'native-test-file-contexts'), str(source)], check=True)
    return (WORK / 'native-test-policy').read_bytes()


def main():
    require_space(0.25)
    donor = WORK / 'android16-extracted'
    runtime = WORK / 'runtime-apex'
    assert sha(WORK / 'system-td-arm32_binder64-vanilla.img.xz') == '3f4fc02daf6f7d0d1333dab3fb2e70f3b30c268dfc419eb42c1c2b3f6d9cc666'
    donor_props = dict(line.split('=', 1) for line in (donor / 'system/build.prop').read_text().splitlines()
                       if line and not line.startswith('#') and '=' in line)
    assert donor_props['ro.build.version.sdk'] == '36'
    assert donor_props['ro.build.version.release'] == '16'
    entries, manifest = {}, {}

    def put(name, mode, data):
        name = name.lstrip('/')
        for parent in reversed(PurePosixPath(name).parents):
            if str(parent) != '.':
                entries.setdefault(str(parent), (0o40755, b''))
        entries[name] = (mode, data)

    def host(name):
        if name == '/system/bin/native-probe':
            return WORK / 'native-probe'
        if name.startswith('/apex/com.android.runtime/'):
            return runtime / name.removeprefix('/apex/com.android.runtime/')
        return donor / name.lstrip('/')

    def add(name):
        if name.lstrip('/') in entries:
            return
        path = host(name)
        if path.is_symlink():
            target = str(path.readlink())
            put(name, 0o120777, target.encode())
            add(target if target.startswith('/') else str(PurePosixPath(name).parent / target))
            return
        data = path.read_bytes()
        put(name, 0o100755 if path.stat().st_mode & 0o111 else 0o100644, data)
        manifest[name] = {'sha256': sha(path), 'bytes': len(data)}
        if data[:4] != b'\x7fELF':
            return
        assert data[4:6] == b'\x01\x01', f'Expected ELF32 little endian: {name}'
        info = subprocess.check_output(['llvm-readelf', '-l', '-d', str(path)], text=True)
        interp = re.search(r'Requesting program interpreter: ([^\]]+)', info)
        if interp:
            add(interp[1])
        for needed in re.findall(r'Shared library: \[([^\]]+)\]', info):
            choices = [f'/apex/com.android.runtime/lib/bionic/{needed}', f'/system/lib/{needed}']
            found = next((p for p in choices if host(p).is_file() or host(p).is_symlink()), None)
            if found is None:
                raise RuntimeError(f'Missing dependency {needed} required by {name}')
            add(found)

    subprocess.run(['clang', '--target=armv7-linux-androideabi35', '-mcpu=cortex-a55',
                    '-Wall', '-Wextra', '-Werror', '-ffreestanding', '-fPIE', '-nostdlib', '-fuse-ld=lld', '-pie',
                    '-Wl,-e,_start', '-Wl,--dynamic-linker,/system/bin/linker',
                    str(ROOT / 'src/pid1/audio/bionic-start.S'),
                    str(Path(__file__).with_name('native-probe.c')),
                    '-L' + str(runtime / 'lib/bionic'), '-L' + str(donor / 'system/lib'),
                    '-lc', '-lbinder_ndk', '-o', str(WORK / 'native-probe')], check=True)
    for binary in ('toybox', 'sh', 'servicemanager', 'service', 'native-probe', 'getprop', 'setprop', 'init'):
        add('/system/bin/' + binary)
    # The dynamic linker loads this private Bionic library itself.
    add('/apex/com.android.runtime/lib/bionic/libdl_android.so')
    for directory in ('dev', 'proc', 'sys', 'tmp', 'data', 'data/local/tmp', 'dev/socket', 'dev/pts', 'dev/binderfs'):
        put(directory, 0o40755, b'')
    put('dev/console', 0o20600, b'')
    put('native-test-policy', 0o100644, policy(donor))
    put('system/etc/selinux/plat_service_contexts', 0o100644, b'* u:object_r:native_t:s0\n')
    busybox = ROOT / 'assets/build/leanboot/qemu-input/initrd.gz'
    assert sha(busybox) == PINS['initrd.gz']
    old = read_cpio(gzip.decompress(busybox.read_bytes()))
    for target, source in {
        'bin/busybox': 'usr/bin/busybox',
        'lib/ld-linux-armhf.so.3': 'usr/lib/arm-linux-gnueabihf/ld-linux-armhf.so.3',
        'lib/arm-linux-gnueabihf/libc.so.6': 'usr/lib/arm-linux-gnueabihf/libc.so.6',
    }.items():
        put(target, *old[source])
    for command in ('sh', 'mount', 'mknod', 'cat', 'grep', 'sleep', 'mkdir', 'chmod', 'uname', 'kill', 'wc', 'ln'):
        put('bin/' + command, 0o120777, b'busybox')
    # A deliberately small shared namespace for this isolated native test.
    # Real vendor/APEX separation will need the generated production linkerconfig.
    put('linkerconfig/ld.config.txt', 0o100644, b'''dir.system = /system/bin
[system]
namespace.default.isolated = false
namespace.default.search.paths = /apex/com.android.runtime/lib/bionic:/system/lib
''')
    put('init', 0o100755, b'''#!/bin/busybox sh
export PATH=/bin
set -eu
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t tmpfs tmpfs /tmp
mknod /dev/null c 1 3
mknod /dev/zero c 1 5
mknod /dev/random c 1 8
mknod /dev/urandom c 1 9
mknod /dev/kmsg c 1 11
mknod /dev/kmsg_debug c 1 11
chmod 666 /dev/null /dev/zero /dev/random /dev/urandom
mount -t binder binder /dev/binderfs
for node in binder hwbinder vndbinder; do ln -s binderfs/$node /dev/$node; done
echo NATIVE_TEST_PID1=$$
uname -a
export LD_LIBRARY_PATH=/apex/com.android.runtime/lib/bionic:/system/lib
export ANDROID_LOG_TAGS='*:v'
/system/bin/toybox --version
/system/bin/toybox uname -m
mkdir -p /sys/fs/selinux
mount -t selinuxfs selinuxfs /sys/fs/selinux
/system/bin/toybox load_policy /native-test-policy
echo 1 > /sys/fs/selinux/enforce
exec /system/bin/init second_stage
''')
    put('native-test.sh', 0o100755, b'''#!/system/bin/sh
exec > /dev/console 2>&1
set -eu
trap 'echo ANDROID16_NATIVE_QEMU_FAIL' EXIT
test "$(/system/bin/toybox readlink /proc/1/exe)" = /system/bin/init
echo ANDROID16_INIT_PID1_OK
test "$(/system/bin/getprop ro.build.version.sdk)" = 36
/system/bin/setprop dande.native_probe 42
/system/bin/toybox sleep 1
test "$(/system/bin/getprop dande.init_response)" = ok
echo ANDROID16_PROPERTY_TRIGGER_OK
test "$(/system/bin/toybox cat /sys/devices/system/cpu/online)" = 0-3
echo ONLINE_CPUS=0-3
echo KERNEL_RELEASE=$(/system/bin/toybox uname -r)
test "$(/system/bin/toybox cat /sys/fs/selinux/enforce)" = 1
echo QEMU_TEST_POLICY_ENFORCING
echo ANDROID16_BIONIC_SHELL_OK
/system/bin/toybox dd if=/dev/zero of=/tmp/android16-test bs=1024 count=64
test "$(/system/bin/toybox wc -c < /tmp/android16-test)" = 65536
/system/bin/service list > /tmp/services.txt 2>&1
/system/bin/toybox cat /tmp/services.txt
/system/bin/toybox grep -q 'Found .* services:' /tmp/services.txt
/system/bin/native-probe server > /tmp/native-server.log 2>&1 &
server=$!
/system/bin/native-probe || { /system/bin/toybox cat /tmp/native-server.log; exit 1; }
kill -0 $server
/system/bin/toybox cat /tmp/native-server.log
trap - EXIT
echo ANDROID16_NATIVE_QEMU_PASS
''')
    # Android init mounts fresh tmpfs on /apex and /linkerconfig. This
    # experiment uses flattened native libraries, without apexd packaging.
    for name in list(entries):
        if name.startswith('apex/com.android.runtime/') and entries[name][0] & 0o170000 == 0o100000:
            target = 'system/bin/linker' if name.endswith('/bin/linker') else 'system/lib/' + PurePosixPath(name).name
            put(target, *entries[name])
            manifest['/' + target] = manifest.pop('/' + name)
            del entries[name]
    put('bootstrap-apex', 0o40755, b'')
    put('dev/.coldboot_done', 0o100644, b'')
    put('system/etc/selinux/plat_property_contexts', 0o100644,
        b'* u:object_r:native_t:s0\n')
    put('system/etc/selinux/plat_file_contexts', 0o100644,
        b'/.* u:object_r:native_t:s0\n')
    put('system/etc/prop.default', 0o100644, b'''ro.build.version.sdk=36
ro.build.version.release=16
ro.build.version.codename=REL
ro.build.version.security_patch=2025-06-05
ro.product.cpu.abilist=armeabi-v7a,armeabi
ro.product.cpu.abilist32=armeabi-v7a,armeabi
ro.hardware=dande_qemu
ro.debuggable=1
ro.boot.selinux=enforcing
ro.boot.init_rc=/system/etc/init/hw/init.rc
''')
    put('system/build.prop', *entries['system/etc/prop.default'])
    put('system/etc/init/hw/init.rc', 0o100644, b'''on early-init
    # Device nodes for this bounded QEMU test were supplied by /init.
    setprop ro.cold_boot_done true
    copy /system/etc/native-ld.config.txt /linkerconfig/ld.config.txt

on init
    mkdir /dev/cpuset 0755 root root
    mount cgroup none /dev/cpuset cpuset
    mkdir /dev/cg2_bpf 0755 root root
    mount cgroup2 none /dev/cg2_bpf
    start servicemanager

on property:servicemanager.ready=true
    start native-test

on property:dande.native_probe=42
    setprop dande.init_response ok

service servicemanager /system/bin/servicemanager
    user root
    group root
    seclabel u:r:native_t:s0
    stdio_to_kmsg

service native-test /system/bin/sh /native-test.sh
    user root
    group root
    seclabel u:r:native_t:s0
    stdio_to_kmsg
    oneshot
''')
    put('system/etc/native-ld.config.txt', *entries['linkerconfig/ld.config.txt'])
    ramdisk = gzip.compress(cpio([(name, mode, data) for name, (mode, data) in sorted(entries.items())]), mtime=0)
    (WORK / 'native-initramfs.cpio.gz').write_bytes(ramdisk)
    report = {'donor': 'TrebleDroid ci-20250617, pinned manifest Android 16 r2',
              'donor_fingerprint': donor_props['ro.system.build.fingerprint'],
              'files': manifest, 'android_file_bytes': sum(v['bytes'] for v in manifest.values()),
              'android_file_count': len(manifest), 'ramdisk_bytes': len(ramdisk),
              'ramdisk_sha256': sha(WORK / 'native-initramfs.cpio.gz'),
              'android_init': True,
              'scope': 'Native Bionic shell and Binder smoke test; no Android framework or HALs'}
    (WORK / 'native-manifest.json').write_text(json.dumps(report, indent=2) + '\n')
    print(json.dumps({k:v for k,v in report.items() if k != 'files'}, indent=2))


if __name__ == '__main__':
    main()
