"""G13 identity is independent of its ADB transport (Ethernet by default)."""
import os
import subprocess

SERIAL = '18ACC22D1C47'
TARGET = os.environ.get('TV_ADB_SERIAL', '192.168.77.3:5555')


def adb(*args, **kwargs):
    return subprocess.run(['adb', '-s', TARGET, *map(str, args)], check=True, **kwargs)


def shell(command):
    return adb('shell', 'set -eu; ' + command, capture_output=True, text=True, timeout=120).stdout


def connect():
    if ':' in TARGET:
        subprocess.run(['adb', 'connect', TARGET], check=True, capture_output=True, text=True, timeout=15)
    state = shell('getprop ro.serialno; getprop ro.product.device; getprop ro.build.version.incremental')
    if state.splitlines() != [SERIAL, 'G13', 'AU13']:
        raise RuntimeError(f'Expected exact G13/AU13 at {TARGET}, got {state!r}')


def root():
    # Identify the device before sending a state-changing command.
    connect()
    adb('root', capture_output=True, timeout=15)
    adb('wait-for-device', timeout=30)
    connect()
    if shell('id -u').strip() != '0':
        raise RuntimeError('G13 adb root is unavailable')
