"""Catch one U-Boot startup through an existing GNU Screen serial session.

Reads only newly appended log data. Screen remains the sole serial-port owner.
The only transmitted input is CR, at most 60 times over a three-second window.
Set Screen's logfile flush interval to zero before starting this helper.
"""

import argparse
from datetime import datetime, timezone
import json
from pathlib import Path
import subprocess
import time


def stop_reason(data):
    if b"<< MT5867 >>#" in data:
        return "bootloader_prompt"
    for marker in (b"UART BUS OFF", b"Starting kernel", b"Power Down"):
        if marker in data:
            return marker.decode()
    return None


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--screen", required=True)
    parser.add_argument("--session", required=True)
    parser.add_argument("--log", type=Path, required=True)
    parser.add_argument("--seconds", type=float, default=600)
    args = parser.parse_args()
    if not 0 < args.seconds <= 1800:
        parser.error("seconds must be in (0, 1800]")
    start = time.monotonic()
    stamp = datetime.now(timezone.utc).strftime("%Y%m%dT%H%M%S.%fZ")
    output = Path(__file__).resolve().parents[1] / "assets/research/device"
    output.mkdir(parents=True, exist_ok=True)
    events_path = output / f"screen-enter-{stamp}.jsonl"
    sent = 0
    trigger = None
    next_tx = 0
    tail = b""
    with args.log.open("rb") as source, events_path.open("x") as events:
        source.seek(0, 2)

        def event(kind, **fields):
            record = dict(utc=datetime.now(timezone.utc).isoformat(),
                          elapsed=round(time.monotonic() - start, 6),
                          kind=kind, **fields)
            events.write(json.dumps(record) + "\n")
            events.flush()

        event("armed", session=args.session, log=str(args.log),
              start_offset=source.tell(), duration_limit=args.seconds,
              tx_window_seconds=3, tx_limit=60, tx_hex="0d")
        print(f"Armed on {args.session}; watching new bytes in {args.log}.", flush=True)
        print(f"Events: {events_path}", flush=True)
        try:
            while time.monotonic() - start < args.seconds:
                offset = source.tell()
                if args.log.stat().st_size < offset:
                    event("stopped", reason="log_truncated", tx_cr_bytes=sent)
                    return
                chunk = source.read(4096)
                now = time.monotonic()
                if chunk:
                    event("rx_log", offset=offset, length=len(chunk))
                    tail = (tail + chunk)[-8192:]
                    reason = stop_reason(tail)
                    if reason:
                        event("stopped", reason=reason, tx_cr_bytes=sent)
                        print(f"Stopped: {reason}; {sent} CR bytes sent. Screen remains open.", flush=True)
                        return
                    if trigger is None and b"U-Boot 2011.06" in tail:
                        trigger = now
                        event("startup_trigger")
                if trigger is not None:
                    if now - trigger >= 3 or sent >= 60:
                        event("stopped", reason="tx_window_finished", tx_cr_bytes=sent)
                        print(f"Enter window finished: {sent} CR bytes. Screen remains open.", flush=True)
                        return
                    if now >= next_tx:
                        subprocess.run([args.screen, "-S", args.session, "-p", "0",
                                        "-X", "stuff", "\r"], check=True,
                                       capture_output=True, timeout=2)
                        sent += 1
                        event("tx", hex="0d", count=sent)
                        next_tx = time.monotonic() + 0.05
                time.sleep(0.005)
            event("stopped", reason="wait_timeout", tx_cr_bytes=sent)
            print("Wait timed out; Screen remains open.", flush=True)
        finally:
            event("closed", tx_cr_bytes=sent)


if __name__ == "__main__":
    main()
