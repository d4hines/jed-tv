#!/usr/bin/env python3
"""Compare ALL guest frame hashes, or an explicitly requested frame prefix.
  python3 test/check_framemd5.py SERIAL_LOG FIXTURE.mp4 [FRAME_LIMIT]
A truncated run must fail instead of validating only the frames it produced.
"""
import re
import subprocess
import sys


def compare(log, reference, limit=None):
    records = re.findall(r"framemd5 (\d+) -?\d+ \d+x\d+ ([0-9a-f]{32})", log)
    got = [h for _, h in records]
    want = [line.rsplit(",", 1)[1].strip() for line in reference.splitlines()
            if line and not line.startswith("#")]
    expected = min(limit, len(want)) if limit is not None else len(want)
    n = min(len(got), expected)
    bad = [i for i in range(n) if got[i] != want[i] or int(records[i][0]) != i]
    ok = expected > 0 and len(got) == expected and not bad
    return ok, (f"guest frames {len(got)}, ffmpeg frames {len(want)}, expected {expected}, "
                f"compared {n}, mismatches {len(bad)}"), bad


def main():
    log, fixture = sys.argv[1:3]
    limit = int(sys.argv[3]) if len(sys.argv) > 3 else None
    reference = subprocess.run(["ffmpeg", "-v", "error", "-i", fixture, "-map", "0:v",
                                "-pix_fmt", "yuv420p", "-f", "framemd5", "-"],
                               capture_output=True, text=True, check=True).stdout
    ok, summary, bad = compare(open(log).read(), reference, limit)
    print(summary)
    if bad:
        print("first mismatches:", bad[:10])
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
