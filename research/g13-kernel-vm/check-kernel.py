#!/usr/bin/env python3
"""Exercise VMA remapping, rebuilt modules and stock ABI rejection in QEMU."""
import gzip
import json
from pathlib import Path
import sys
import subprocess

sys.dont_write_bytecode = True
from run import ROOT, WORK, OLD, runner
from qemu import cpio, read_cpio


def main():
    build = json.loads((WORK / 'kernel-build.json').read_text())
    rebuilt = WORK / 'kernel-out/lib/crypto/libarc4.ko'
    assert runner.sha(rebuilt) == build['artifacts']['lib/crypto/libarc4.ko']['sha256']
    assert runner.sha(OLD / 'native-initramfs.cpio.gz') == 'e84b6a12448f9fc88eac50d050fe81c5fd22039028dd828f3e44ebe1a0d0f6f5'
    entries = read_cpio(gzip.decompress((OLD / 'native-initramfs.cpio.gz').read_bytes()))
    target = WORK / 'kernel-test'
    target.mkdir(exist_ok=True)
    # Link against the exact libc inside the verified test ramdisk.
    (target / 'libc.so').write_bytes(entries['system/lib/libc.so'][1])
    subprocess.run(['clang', '--target=armv7-linux-androideabi35', '-mcpu=cortex-a55',
                    '-Wall', '-Wextra', '-Werror', '-ffreestanding', '-fPIE', '-nostdlib',
                    '-fuse-ld=lld', '-pie', '-Wl,-e,_start',
                    '-Wl,--dynamic-linker,/system/bin/linker',
                    str(ROOT / 'src/audio/bionic-start.S'),
                    str(Path(__file__).with_name('kernel-probe.c')),
                    '-L' + str(target), '-lc', '-o', str(target / 'kernel-probe')], check=True)
    entries['system/bin/kernel-probe'] = (0o100755, (target / 'kernel-probe').read_bytes())
    entries['test-modules'] = (0o40755, b'')
    entries['test-modules/libarc4.ko'] = (0o100644, rebuilt.read_bytes())
    timer = WORK / build['test_module']['path']
    assert runner.sha(timer) == build['test_module']['sha256']
    entries['test-modules/timer_probe.ko'] = (0o100644, timer.read_bytes())
    script = '''
/system/bin/kernel-probe
/system/bin/toybox insmod /test-modules/libarc4.ko
/system/bin/toybox grep -q '^libarc4 ' /proc/modules
/system/bin/toybox rmmod libarc4
if /system/bin/toybox grep -q '^libarc4 ' /proc/modules; then exit 1; fi
echo SOURCE_MODULE_LOAD_UNLOAD_OK
/system/bin/toybox insmod /test-modules/timer_probe.ko
/system/bin/toybox dmesg | /system/bin/toybox grep -q KERNEL_TIMER_SHUTDOWN_OK
/system/bin/toybox rmmod timer_probe
echo TIMER_MODULE_LOAD_UNLOAD_OK
'''
    fixtures = {'rebuilt': runner.sha(rebuilt), 'probe': runner.sha(target / 'kernel-probe'),
                'timer': runner.sha(timer)}
    roots = {'v013': ROOT / 'assets/research/g13-firmware-trim-20261101/v013-extracted',
             'v033': ROOT / 'assets/research/g13-v033-offline-20261101/extracted'}
    markers = ['MREMAP_DONTUNMAP_CASES=8', 'SOURCE_MODULE_LOAD_UNLOAD_OK',
               'KERNEL_TIMER_SHUTDOWN_OK', 'TIMER_MODULE_LOAD_UNLOAD_OK']
    for label, root in roots.items():
        audit = json.loads((WORK / (label + '-module-audit.json')).read_text())
        item = next(x for x in audit['modules'] if x['name'] == 'devnode.ko')
        # Rejection is expected before module init: never strip versions or force-load.
        assert 'module_layout' in item['mismatched_crc']
        stock = root / item['path']
        assert runner.sha(stock) == item['sha256']
        entries['test-modules/' + label + '.ko'] = (0o100644, stock.read_bytes())
        fixtures[label] = item['sha256']
        marker = 'STOCK_' + label.upper() + '_MODULE_REJECTED'
        markers.append(marker)
        script += f'''
/system/bin/toybox dmesg -c > /tmp/before-{label}.log
if /system/bin/toybox insmod /test-modules/{label}.ko; then exit 1; fi
/system/bin/toybox dmesg > /tmp/reject-{label}.log
/system/bin/toybox cat /tmp/reject-{label}.log
/system/bin/toybox grep -q 'devnode: disagrees about version of symbol module_layout' /tmp/reject-{label}.log
echo {marker}
'''
    mode, body = entries['native-test.sh']
    assert body.count(b'trap - EXIT') == 1
    entries['native-test.sh'] = (mode, body.replace(b'trap - EXIT', script.encode() + b'trap - EXIT'))
    if not (target / 'kernel-out').exists():
        (target / 'kernel-out').symlink_to(WORK / 'kernel-out', target_is_directory=True)
    ramdisk = gzip.compress(cpio([(name, mode, data) for name, (mode, data) in sorted(entries.items())]), mtime=0)
    (target / 'native-initramfs.cpio.gz').write_bytes(ramdisk)
    manifest = json.loads((OLD / 'native-manifest.json').read_text())
    manifest.update(module_fixtures=fixtures, ramdisk_bytes=len(ramdisk),
                    ramdisk_sha256=runner.sha(target / 'native-initramfs.cpio.gz'))
    (target / 'native-manifest.json').write_text(json.dumps(manifest, indent=2) + '\n')
    before = set(target.glob('qemu-*'))
    runner.WORK = target
    status = runner.main()
    runs = set(target.glob('qemu-*')) - before
    assert len(runs) == 1
    run = runs.pop()
    result = json.loads((run / 'result.json').read_text())
    serial = (run / 'serial.log').read_text()
    result['checks'].update({marker: marker in serial for marker in markers})
    result['pass'] = result['pass'] and all(result['checks'].values())
    (run / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
    print('Kernel checks:', {marker: result['checks'][marker] for marker in markers})
    return status or int(not result['pass'])


if __name__ == '__main__':
    sys.exit(main())
