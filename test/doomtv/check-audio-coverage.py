#!/usr/bin/env python3
"""Catch gross PCM starvation in a completed Doom QEMU run directory.

This checks supplied sample duration against logged active wall time, not
speaker audibility or perfect continuity. Half coverage is intentionally a
coarse regression floor: the old frame-coupled mixer supplied about 17%.
QEMU can leave RIFF/data lengths zero in its streaming WAV output.
"""
import array
import json
from pathlib import Path
import re
import struct
import sys


def check(directory):
    directory = Path(directory)
    events = []
    for line in (directory / 'serial.log').read_text(errors='replace').splitlines():
        match = re.search(r'\[([0-9]+\.[0-9]+)\] (.*)', line)
        if match:
            events.append((float(match[1]), match[2]))
    start = next(t for t, text in events if text.startswith('audio: 48000 Hz'))
    end = max(t for t, text in events if 'tics/s' in text or text == 'doom: quit from the menu')
    data = (directory / 'audio.wav').read_bytes()
    assert data[:4] == b'RIFF' and data[8:12] == b'WAVE', 'not QEMU PCM WAV'
    position = 12
    rate = channels = width = 0
    pcm = b''
    while position + 8 <= len(data):
        kind = data[position:position + 4]
        size = struct.unpack_from('<I', data, position + 4)[0]
        position += 8
        if kind == b'fmt ':
            format_, channels, rate = struct.unpack_from('<HHI', data, position)
            width = struct.unpack_from('<H', data, position + 14)[0]
            assert format_ == 1 and width == 16, 'expected signed 16-bit PCM'
        if kind == b'data':
            pcm = data[position:position + size] if size else data[position:]
            break
        position += size + (size & 1)
    assert rate and channels and pcm, 'no PCM samples'
    samples = array.array('h', pcm)
    if sys.byteorder != 'little':
        samples.byteswap()
    audio_seconds = len(samples) / channels / rate
    wall_seconds = end - start
    assert wall_seconds > 1, 'run too short to measure audio supply'
    coverage = audio_seconds / wall_seconds
    nonzero = any(samples)
    result = {'audio_seconds': audio_seconds, 'logged_active_seconds': wall_seconds,
              'coverage': coverage, 'nonzero_pcm': nonzero,
              'passed': coverage >= .5 and nonzero}
    print(json.dumps(result, indent=2))
    return result['passed']


if __name__ == '__main__':
    raise SystemExit(0 if check(sys.argv[1]) else 1)
