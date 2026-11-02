#!/usr/bin/env python3
"""Compare the bounded tone on stock G06 Android; never reboot or flash."""
import argparse
import datetime
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[2]
SERIAL = '4C0490290FA05A0C0'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--percent', type=int, default=30)
    parser.add_argument('--jingle', action='store_true')
    args = parser.parse_args()
    if not 1 <= args.percent <= 30:
        parser.error('percent must be 1..30')
    adb = ['adb','-s',SERIAL]
    identity = subprocess.check_output(adb+['shell','getprop ro.serialno; getprop ro.product.device; getprop ro.software.version_id; getprop sys.boot_completed'],timeout=10,text=True).splitlines()
    if identity != [SERIAL,'G06','V8-T221T04-LF1V109','1']:
        raise RuntimeError(f'Stock Android not ready or wrong device: {identity!r}')
    package = subprocess.check_output(['nix','build','.#hello-screen','--no-link','--print-out-paths'],cwd=ROOT,text=True).strip()
    jar = str(Path(package)/'hello.jar')
    remote = '/data/local/tmp/native-audio-test.jar'
    subprocess.run(adb+['push',jar,remote],check=True)
    stamp=datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    out=ROOT/'assets/research/audio-native'/('stock-'+stamp)
    out.mkdir(parents=True)
    out.joinpath('identity.txt').write_text('\n'.join(identity)+'\n')
    command=f'CLASSPATH={remote} app_process32 /system/bin --nice-name=tv.tone AudioTone {args.percent}'
    if args.jingle:
        command += ' jingle'
    result=subprocess.run(adb+['shell',command],stdout=subprocess.PIPE,stderr=subprocess.STDOUT,timeout=15)
    out.joinpath('tone.log').write_bytes(result.stdout)
    print(result.stdout.decode(errors='replace'),end='')
    print('Evidence:',out)
    result.check_returncode()


if __name__ == '__main__':
    main()
