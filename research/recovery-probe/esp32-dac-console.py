#!/usr/bin/env python3
"""Classic ESP32 GPIO25 DAC via ROM register commands; never writes flash.

Run only with the ESP disconnected from the TV until electrical measurements
establish a suitable connection. Starts with DAC disabled. Commands: status,
set 0..63, off, quit. A DAC code is NOT a verified voltage under load.

Register definitions: Espressif ESP-IDF v5.5.1, esp32 rtc_io_reg.h,
sens_reg.h, reg_base.h and hal/dac_ll.h. Copies are in
assets/research/g13-esp32-uart-enable-20261101/.
Requires esptool 5.x. --plan needs only the Python standard library.
"""

import argparse
import datetime
import json
from pathlib import Path

PAD = 0x3FF48484  # RTC_IO_PAD_DAC1_REG, classic ESP32 GPIO25 / RTCIO6
RTC_DISABLE = 0x3FF48414  # RTC_GPIO_ENABLE_W1TC_REG
CTRL1 = 0x3FF48898  # SENS_SAR_DAC_CTRL1_REG
CTRL2 = 0x3FF4889C  # SENS_SAR_DAC_CTRL2_REG
ADC_CTRL = 0x3FF488A0  # SENS_SAR_MEAS_CTRL2_REG
PAD_MASK = sum(1 << bit for bit in range(10, 30))
PAD_OFF = (1 << 17) | (1 << 10)  # RTC mux, force DAC power control, power off
MAX_CODE = 63  # Nominal maximum ~0.82 V at 3.3 V; measure actual voltage.
READABLE = (PAD, 0x3FF4840C, CTRL1, CTRL2, ADC_CTRL)


def settings(code):
    if code is not None and not 0 <= code <= MAX_CODE:
        raise ValueError(f"Code must be 0..{MAX_CODE}")
    return [
        (RTC_DISABLE, 1 << (6 + 14), 0xFFFFFFFF),
        (ADC_CTRL, 0, 0xF),
        (CTRL1, 0, 1 << 22),
        (CTRL2, 0, 1 << 24),
        (PAD, PAD_OFF | (0 if code is None else (1 << 18) | (code << 19)), PAD_MASK),
    ]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--port', help='Explicit ESP serial path; never the TV UART')
    parser.add_argument('--plan', action='store_true', help='Show register plan without hardware')
    parser.add_argument('--log', type=Path)
    args = parser.parse_args()
    if args.plan:
        print(json.dumps({'off': settings(None), 'example_code_32': settings(32),
                          'maximum_code': MAX_CODE, 'gpio': 25}, indent=2))
        return
    if not args.port:
        parser.error('--port is required')
    from esptool.cmds import detect_chip
    from serial.tools import list_ports

    port = Path(args.port).resolve(strict=True)
    matches = [p for p in list_ports.comports() if Path(p.device).resolve() == port]
    if len(matches) != 1:
        raise SystemExit('Cannot uniquely identify the ESP serial port')
    device = matches[0]
    if device.serial_number == 'AU0563JT' or 'DCSD' in (device.description or ''):
        raise SystemExit('Refusing the TV UART adapter')
    stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
    logfile = args.log or Path('assets/research/g13-esp32-uart-enable-20261101') / f'console-{stamp}.jsonl'
    logfile.parent.mkdir(parents=True, exist_ok=True)

    def record(event, **data):
        row = {'utc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
               'event': event, **data}
        with logfile.open('a') as f:
            f.write(json.dumps(row) + '\n')

    record('connect', port=str(port), usb_serial=device.serial_number, vid=device.vid, pid=device.pid)
    # Reset into the ESP ROM downloader; no stub upload, flash operations or
    # eFuse writes. Reject non-classic chips before any peripheral writes.
    esp = detect_chip(str(port), baud=115200, connect_attempts=3)
    if esp.CHIP_NAME != 'ESP32':
        esp._port.close()
        raise SystemExit(f'Requires classic ESP32; detected {esp.CHIP_NAME}')

    def status():
        values = {hex(address): hex(esp.read_reg(address)) for address in READABLE}
        record('registers', values=values)
        print(json.dumps(values), flush=True)

    def apply(code):
        for address, value, mask in settings(code):
            esp.write_reg(address, value, mask)
            if address != RTC_DISABLE and esp.read_reg(address) & mask != value & mask:
                raise RuntimeError(f'Register readback mismatch at {address:#x}')
        if esp.read_reg(0x3FF4840C) & (1 << 20):
            raise RuntimeError('RTC digital output is still enabled')
        record('dac', code=code)
        print('DAC disabled' if code is None else f'DAC code {code}; verify GPIO25 voltage with meter', flush=True)

    try:
        status()
        apply(None)
        print('GPIO25 starts disabled. Commands: status | set 0..63 | off | quit', flush=True)
        while True:
            try:
                command = input('dac> ').strip().split()
            except EOFError:
                break
            if command == ['quit']:
                break
            if command == ['status']:
                status()
            elif command == ['off']:
                apply(None)
            elif len(command) == 2 and command[0] == 'set':
                try:
                    code = int(command[1])
                    settings(code)
                except ValueError as error:
                    print(error)
                    continue
                apply(code)
            elif command:
                print('Commands: status | set 0..63 | off | quit')
    except KeyboardInterrupt:
        pass
    finally:
        try:
            apply(None)
        finally:
            # Leave processor in ROM download mode; do not boot saved firmware
            # while the owner may still have an experimental wire attached.
            esp._port.close()


if __name__ == '__main__':
    main()
