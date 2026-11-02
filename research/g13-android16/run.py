#!/usr/bin/env python3
"""Boot the minimal Android native ramdisk on the partial G13 QEMU model."""
import argparse
import importlib.util
import json
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import time

sys.dont_write_bytecode = True
from minimal import ROOT, WORK, sha
from storage import require_space

spec = importlib.util.spec_from_file_location('kernel_run', ROOT / 'research/g13-kernel/run.py')
kernel_run = importlib.util.module_from_spec(spec)
spec.loader.exec_module(kernel_run)


def main():
    require_space(0.1)
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--baseline', action='store_true', help='Use previously built 5.15.148 for comparison')
    parser.add_argument('--timeout', type=float, default=90)
    args = parser.parse_args()
    if args.timeout <= 0:
        parser.error('--timeout must be positive')
    kernel_out = ROOT / 'assets/build/g13-kernel/out' if args.baseline else WORK / 'kernel-out'
    qemu = ROOT / 'assets/build/qemu-g06/build/qemu-system-aarch64'
    assert sha(kernel_run.LIVE) == kernel_run.LIVE_SHA
    out = WORK / ('qemu-' + time.strftime('%Y%m%dT%H%M%S', time.gmtime()))
    out.mkdir()
    kernel = out / 'Image'
    kernel.write_bytes((kernel_out / 'arch/arm/boot/Image').read_bytes())
    (out / 'kernel.config').write_bytes((kernel_out / '.config').read_bytes())
    ram = out / 'initramfs.cpio.gz'
    ram.write_bytes((WORK / 'native-initramfs.cpio.gz').read_bytes())
    dtb = out / 'board.dtb'
    dtb.write_bytes(kernel_run.LIVE.read_bytes())
    original = subprocess.check_output(['fdtget', '-t', 's', str(dtb), '/chosen', 'bootargs'], text=True).split()
    keep = {'LX_MEM', 'LX_MEM2', 'EMAC_MEM', 'CMA0', 'CMA1', 'CMA2', 'DMA_SIZE',
            'SD_CONFIG', 'platform', 'Pmode', 'SAR5', 'PM51_ADDR', 'PM51_LEN', 'cma', 'vmalloc'}
    cmdline = ' '.join(s for s in original if s.split('=')[0] in keep)
    cmdline += ' console=ttyS0,115200 loglevel=7 rdinit=/init root=/dev/ram rw panic=-1'
    cmdline += ' initcall_blacklist=' + ','.join(kernel_run.SKIP)
    for prop, value in [('linux,initrd-start', 0x24000000),
                        ('linux,initrd-end', 0x24000000 + ram.stat().st_size)]:
        subprocess.run(['fdtput', '-t', 'x', str(dtb), '/chosen', prop, f'{value:x}'], check=True)
    subprocess.run(['fdtput', '-t', 's', str(dtb), '/chosen', 'bootargs', cmdline], check=True)
    stub = out / 'entry.bin'
    stub.write_bytes(struct.pack('<6I', 0xe3a00000, 0xe3e01000, 0xe59f2000,
                                 0xe59ff000, 0x23000000, 0x20208000))
    serial = out / 'serial.log'
    report = {'baseline': args.baseline, 'kernel_release': (kernel_out / 'include/config/kernel.release').read_text().strip(),
              'kernel_sha256': sha(kernel), 'kernel_config_sha256': sha(out / 'kernel.config'),
              'qemu_sha256': sha(qemu), 'dtb_sha256': sha(dtb), 'ramdisk_sha256': sha(ram),
              'native_manifest': json.loads((WORK / 'native-manifest.json').read_text()),
              'tv_accessed': False, 'disabled_initcalls': kernel_run.SKIP,
              'limitations': 'Partial RIU model, no physical peripherals, no Android framework or vendor HAL test'}
    with tempfile.TemporaryDirectory(prefix='g13-native-qmp-') as control:
        qmp = Path(control) / 'socket'
        cmd = [str(qemu), '-M', 'g06', '-smp', '4', '-display', 'none', '-monitor', 'none',
               '-nic', 'none', '-no-reboot', '-serial', 'file:' + str(serial),
               '-qmp', f'unix:{qmp},server=on,wait=off']
        for path, address in [(kernel, 0x20208000), (dtb, 0x23000000), (ram, 0x24000000), (stub, 0x20200000)]:
            cmd += ['-device', f'loader,file={path},addr={address:#x},force-raw=on' + (',cpu-num=0' if path == stub else '')]
        (out / 'command.json').write_text(json.dumps(cmd, indent=2) + '\n')
        print(out, flush=True)
        started = time.monotonic()
        with (out / 'qemu.log').open('w') as log:
            proc = subprocess.Popen(cmd, stdout=log, stderr=log)
            try:
                while proc.poll() is None and time.monotonic() - started < args.timeout:
                    text = serial.read_text(errors='replace') if serial.exists() else ''
                    if any(x in text for x in ('ANDROID16_NATIVE_QEMU_PASS', 'ANDROID16_NATIVE_QEMU_FAIL', 'Kernel panic', 'Internal error:', 'InitFatalReboot', 'Reboot failed -- System halted')):
                        break
                    time.sleep(0.1)
            finally:
                try:
                    (out / 'registers.txt').write_text(kernel_run.registers(qmp))
                except (OSError, ValueError, RuntimeError) as error:
                    report['register_error'] = str(error)
                proc.terminate()
                try:
                    proc.wait(timeout=3)
                except subprocess.TimeoutExpired:
                    proc.kill()
                    proc.wait()
        report['elapsed_seconds'] = round(time.monotonic() - started, 3)
    text = serial.read_text(errors='replace') if serial.exists() else ''
    required = ('ANDROID16_NATIVE_QEMU_PASS', 'ANDROID16_INIT_PID1_OK',
                'ANDROID16_PROPERTY_TRIGGER_OK', 'ONLINE_CPUS=0-3',
                'KERNEL_RELEASE=' + report['kernel_release'],
                'QEMU_TEST_POLICY_ENFORCING', 'ANDROID16_BIONIC_SHELL_OK',
                'ANDROID16_THREADS_SOCKETS_OK', 'ANDROID16_BINDER_SERVER_READY',
                'ANDROID16_BINDER_ROUNDTRIP=42', 'ANDROID16_BINDER_SHARED_MEMORY_OK',
                'ANDROID16_BINDER_CONCURRENT_512_OK')
    report['checks'] = {marker: marker in text for marker in required}
    report['pass'] = all(report['checks'].values()) and not any(x in text for x in (
        'Kernel panic', 'Internal error:', 'ANDROID16_NATIVE_QEMU_FAIL', 'InitFatalReboot'))
    (out / 'result.json').write_text(json.dumps(report, indent=2) + '\n')
    print('\n'.join(text.splitlines()[-45:]))
    return 0 if report['pass'] else 1


if __name__ == '__main__':
    raise SystemExit(main())
