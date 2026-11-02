#!/usr/bin/env python3
"""Start the proven native audio path without playing any sound or rebooting."""
import datetime
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/pid1'))
from graphics import console, upload


def main():
    subprocess.run(['sh', 'tools/audio/build-native.sh'], cwd=ROOT, check=True)
    for name in ('standalone-app-state.so', 'middleware-probe'):
        upload(ROOT / 'assets/research/audio-native' / name)
    upload(ROOT / 'research/probes/audio/start-native.sh')
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    print('Native audio startup:', stamp, flush=True)
    console('set -e; '
            "grep -q 'androidboot.serialno=4C0490290FA05A0C0' /proc/cmdline; "
            'if [ -f /tmp/native-amplifier.ready ]; then '
            'echo "Audio hardware bootstrap already completed"; else '
            'mkdir -p /impdata; '
            'mountpoint -q /impdata || mount -t ext4 -o ro,noload /dev/mmcblk0p24 /impdata; '
            '[ -e /vendor/tvos ] || ln -s /stock-vendor/tvos /vendor/tvos; '
            'sh /tmp/start-native.sh; '
            'if [ ! -f /tmp/native-amplifier.ready ]; then '
            'F1_CONFIG=/tclconfig/json_sys.ini '
            'LD_LIBRARY_PATH=/vendor/tvos/libBionic:/system/lib:/vendor/lib '
            'timeout 15 /tmp/middleware-probe --amp && touch /tmp/native-amplifier.ready; '
            'fi; test -f /tmp/native-amplifier.ready; fi')


if __name__ == '__main__':
    main()
