"""Bounded YMODEM sender run by Screen's exec !!, never opening a serial port."""
import argparse
import json
from pathlib import Path
import subprocess
import termios
import time


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--sender', required=True)
    parser.add_argument('--file', required=True, type=Path)
    parser.add_argument('--result', required=True, type=Path)
    args = parser.parse_args()
    before = termios.tcgetattr(0)
    start = time.monotonic()
    result = {'file': str(args.file), 'bytes': args.file.stat().st_size}
    try:
        with args.result.with_suffix('.stderr').open('wb') as errors:
            child = subprocess.Popen([args.sender, '--ymodem', '--binary',
                                      '--1k', str(args.file)], stderr=errors)
            try:
                # Let sb enter raw mode before the receiver sends its first C.
                time.sleep(.1)
                if child.poll() is not None:
                    raise RuntimeError(f'sb exited before receiver start: {child.returncode}')
                args.result.with_suffix('.ready').write_text(str(child.pid) + '\n')
                result['returncode'] = child.wait(timeout=45)
            finally:
                if child.poll() is None:
                    child.kill()
                    child.wait()
    except Exception as exc:
        result['error'] = repr(exc)
    finally:
        termios.tcsetattr(0, termios.TCSANOW, before)
        result['seconds'] = time.monotonic() - start
        temp = args.result.with_suffix('.tmp')
        temp.write_text(json.dumps(result, indent=2) + '\n')
        temp.replace(args.result)


if __name__ == '__main__':
    main()
