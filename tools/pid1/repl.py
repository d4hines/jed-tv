#!/usr/bin/env python3
"""Connect to our native TV PID-1 console, or upload a file into its /tmp."""
import argparse
import json
import select
import socket
import sys
import termios
import tty
from pathlib import Path

from watch import request
from direct_link import connect


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--put", type=Path, help="upload a local file into the native TV's RAM")
    parser.add_argument("--name", help="destination filename in /tmp (default: local filename)")
    parser.add_argument("-c", "--command", help="run commands in a fresh native shell, then exit")
    args = parser.parse_args()
    status = json.loads(request("GET", "/status"))
    if status.get("app") != "hello-pid1" or status.get("pid") != 1 or status.get("phase") != "shell-ready":
        parser.error(f"Not the native PID-1 shell environment: {status}")
    if args.put:
        name = args.name or args.put.name
        data = args.put.read_bytes()
        if len(data) > 8 * 1024 * 1024:
            parser.error("upload must be at most 8 MiB")
        print(request("PUT", "/files/" + name, data, timeout=5).decode(), end="")
        return
    connection = connect(8766, 5)
    connection.settimeout(None)
    if args.command:
        connection.sendall(args.command.encode() + b"\nexit\n")
    saved = termios.tcgetattr(sys.stdin) if sys.stdin.isatty() and not args.command else None
    input_open = not args.command
    try:
        if saved: tty.setraw(sys.stdin.fileno())
        while True:
            ready, _, _ = select.select([connection, sys.stdin] if input_open else [connection], [], [])
            if connection in ready:
                data = connection.recv(65536)
                if not data: break
                sys.stdout.buffer.write(data); sys.stdout.buffer.flush()
            if sys.stdin in ready:
                import os
                data = os.read(sys.stdin.fileno(), 4096)
                if not data or b"\x1d" in data: break
                connection.sendall(data)
    finally:
        if saved: termios.tcsetattr(sys.stdin, termios.TCSADRAIN, saved)
        connection.close()


if __name__ == "__main__":
    main()
