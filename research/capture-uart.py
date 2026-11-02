"""Capture UART output without sending bytes to the TV.

Run with Python and pyserial after confirming wiring and logic levels.
Connect only adapter RX and GND for the first capture.
"""

import argparse
from datetime import datetime, timezone
from pathlib import Path
import sys
import time

import serial


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("port", help="Confirmed USB-to-TTL device, preferably /dev/serial/by-id/...")
    parser.add_argument("--baud", type=int, default=115200)
    parser.add_argument("--seconds", type=float, default=120)
    args = parser.parse_args()
    if args.seconds <= 0:
        parser.error("--seconds must be positive")

    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    destination = Path(__file__).resolve().parents[1] / "assets/research/device" / f"uart-{stamp}.bin"
    destination.parent.mkdir(parents=True, exist_ok=True)
    uart = serial.Serial(
        port=None, baudrate=args.baud, bytesize=serial.EIGHTBITS,
        parity=serial.PARITY_NONE, stopbits=serial.STOPBITS_ONE,
        timeout=0.2, xonxoff=False, rtscts=False, dsrdtr=False,
        exclusive=True,
    )
    uart.dtr = False
    uart.rts = False
    uart.port = args.port
    uart.open()
    count = 0
    print(f"Listening on {args.port}, {args.baud} 8N1; no serial writes.", flush=True)
    print(f"Raw capture: {destination}", flush=True)
    deadline = time.monotonic() + args.seconds
    try:
        with uart, destination.open("xb") as output:
            while time.monotonic() < deadline:
                chunk = uart.read(max(1, min(uart.in_waiting, 4096)))
                if chunk:
                    output.write(chunk)
                    output.flush()
                    count += len(chunk)
                    text = chunk.decode("utf-8", errors="replace")
                    # Keep device escape sequences out of the host terminal.
                    sys.stdout.write("".join(
                        c if c in "\n\r\t" or c.isprintable() else f"\\x{ord(c):02x}"
                        for c in text
                    ))
                    sys.stdout.flush()
    except KeyboardInterrupt:
        pass
    finally:
        uart.close()
        print(f"\nCaptured {count} bytes in {destination}", flush=True)


if __name__ == "__main__":
    main()
