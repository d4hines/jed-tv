"""Observe one startup and send only CR for a bounded autoboot-entry test.

Requires the operator's connected UART RX/TX/GND setup. No bootloader commands,
break signals, or modem-control pulses are deliberately transmitted.
"""

import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import re
import time

import serial


PROMPT_RE = re.compile(
    rb"(?:^|[\r\n])[^\r\n]{0,60}(?:MT5867|M[Ss]tar|[Mm][Bb][Oo][Oo][Tt]|nugget)"
    rb"[^\r\n]{0,30}[>#] ?$"
)
STOP_MARKERS = (b"UART BUS OFF", b"Starting kernel", b"Power Down")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("port")
    parser.add_argument("--seconds", type=float, default=95)
    parser.add_argument("--tx-window", type=float, default=12)
    args = parser.parse_args()
    if not 0 < args.seconds <= 1800 or not 0 < args.tx_window <= 12:
        parser.error("seconds must be in (0, 1800], tx-window in (0, 12]")
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    base = Path(__file__).resolve().parents[1] / "assets/research/device"
    base.mkdir(parents=True, exist_ok=True)
    raw_path = base / f"uart-enter-{stamp}.bin"
    events_path = base / f"uart-enter-{stamp}.jsonl"
    uart = serial.Serial(
        port=None, baudrate=115200, bytesize=serial.EIGHTBITS,
        parity=serial.PARITY_NONE, stopbits=serial.STOPBITS_ONE,
        timeout=0.01, write_timeout=0.5, xonxoff=False, rtscts=False,
        dsrdtr=False, exclusive=True,
    )
    uart.dtr = uart.rts = False
    uart.port = args.port
    start = time.monotonic()
    trigger = None
    next_tx = 0
    sent = 0
    received = 0
    tail = b""
    stopped = False
    with raw_path.open("xb") as raw, events_path.open("x") as events:
        def event(kind, **fields):
            record = dict(utc=datetime.now(timezone.utc).isoformat(),
                          elapsed=round(time.monotonic() - start, 6),
                          kind=kind, **fields)
            events.write(json.dumps(record) + "\n")
            events.flush()

        uart.open()
        event("open", port=args.port, baud=115200, duration_limit=args.seconds,
              tx_window_seconds=args.tx_window, tx_interval_seconds=0.05,
              tx_hex="0d", trigger="U-Boot 2011.06")
        print(f"Listening; one startup only, Enter only. Raw: {raw_path}", flush=True)
        try:
            while time.monotonic() - start < args.seconds:
                chunk = uart.read(max(1, min(uart.in_waiting, 4096)))
                now = time.monotonic()
                if chunk:
                    raw.write(chunk)
                    raw.flush()
                    event("rx", offset=received, length=len(chunk))
                    received += len(chunk)
                    tail = (tail + chunk)[-4096:]
                    print(chunk.decode("utf-8", errors="backslashreplace")
                          .encode("unicode_escape").decode("ascii"), flush=True)
                    if trigger is None and b"U-Boot 2011.06" in tail:
                        trigger = now
                        event("startup_trigger")
                        print(f"U-Boot detected; sending CR at most 20 times/second for {args.tx_window:g} seconds.", flush=True)
                    if trigger is not None and PROMPT_RE.search(tail):
                        event("possible_prompt", tail_hex=tail[-120:].hex())
                        stopped = True
                        print("Possible prompt detected; transmission stopped.", flush=True)
                        break
                    if trigger is not None and not stopped:
                        for marker in STOP_MARKERS:
                            if marker in tail:
                                stopped = True
                                event("tx_stopped", reason=marker.decode(), count=sent)
                                print(f"{marker.decode()} detected; receive-only for the rest of the test.", flush=True)
                                break
                if trigger is not None and not stopped and now - trigger < args.tx_window and now >= next_tx and sent < 240:
                    uart.write(b"\r")
                    event("tx", hex="0d")
                    sent += 1
                    next_tx = now + 0.05
                if trigger is not None and now - trigger >= args.tx_window and not stopped:
                    stopped = True
                    event("tx_complete", count=sent)
                    print(f"Enter window ended: {sent} CR bytes; receive-only for the rest of the test.", flush=True)
        finally:
            uart.close()
            event("close", rx_bytes=received, tx_cr_bytes=sent)
            print(f"Closed: {received} received bytes, {sent} CR bytes sent. Events: {events_path}", flush=True)


if __name__ == "__main__":
    main()
