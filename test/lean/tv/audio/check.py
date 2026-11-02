#!/usr/bin/env python3
"""Host differential gate: compiled Lean vs the existing Rust PCM protocol."""
from pathlib import Path
import struct
import subprocess

ROOT = Path(__file__).resolve().parents[4]
OUT = ROOT / 'assets/build/lake/audio-tests'


def run(*command):
    subprocess.run(command, cwd=ROOT, check=True, stdout=subprocess.DEVNULL)


def u32(n):
    return struct.pack('<I', n)


def opening(rate=48000, channels=2, boost=1):
    return b'O' + u32(rate) + u32(channels) + u32(boost)


def write(data):
    return b'W' + u32(len(data)) + data


def compare(name, stream, gain, convert=False):
    source, lean, rust = (OUT / n for n in ('input.bin', 'lean.bin', 'rust.bin'))
    source.write_bytes(stream if convert else struct.pack('<f', gain) + stream)
    mode = ['convert'] if convert else []
    run(str(OUT / 'bin/audioTests'), *mode, str(source), str(lean))
    run(str(OUT / 'oracle'), *mode, str(source), str(rust))
    actual, expected = lean.read_bytes(), rust.read_bytes()
    if actual != expected:
        offset = next((i for i, pair in enumerate(zip(actual, expected)) if pair[0] != pair[1]),
                      min(len(actual), len(expected)))
        raise AssertionError(f'{name}: first difference at byte {offset}; '
                             f'Lean={actual[offset:offset+16].hex()} '
                             f'Rust={expected[offset:offset+16].hex()}')


def main():
    run('lake', '-d', 'test/lean/tv/audio', 'build', 'audioTests')
    run('rustc', '+1.92.0', '--edition=2021', '-O', '-C', 'linker=/usr/bin/cc',
        'test/lean/tv/audio/oracle.rs', '-o', str(OUT / 'oracle'))
    subprocess.run([str(OUT / 'bin/audioTests')], check=True)
    # Every signed 16-bit input, every supported integer boost, seven actual
    # binary32 gain values. Includes negative extrema and half-way round cases.
    all_samples = b''.join(struct.pack('<H', n) for n in range(65536))
    cases = 0
    for boost in range(1, 5):
        for gain in (0.0, 0.0025, 0.015, 0.075, 0.15, 0.25, 0.3):
            stream = opening(boost=boost) + write(all_samples) + b'DR' + write(b'\xff\x7f\x00\x80') + b'FD'
            compare(f'boost={boost} gain={gain}', stream, gain)
            cases += 1
    fixtures = [
        b'', b'D', b'O', opening() + b'?', opening(7999), opening(96001),
        opening(48000, 0), opening(48000, 3), opening(48000, 1, 0),
        opening(48000, 1, 5), opening(32000), opening() + b'W' + u32(3),
        opening() + b'W' + u32(1048577), opening() + b'W' + u32(4) + b'\0\0',
        opening(44100, 1, 2) + write(b'') + b'DRFD',
        opening(48000, 2, 4) + write(bytes(1048576)) + b'D',
    ]
    for index, stream in enumerate(fixtures):
        compare(f'fixture={index}', stream, 0.0025)
    converter_cases = 0
    for rate in (44100, 48000):
        for channels in (1, 2):
            frame_size = channels * 2
            for fragmented in (False, True):
                if fragmented:
                    chunks, offset, turn = [], 0, 0
                    while offset < len(all_samples):
                        size = ((turn * 17) % 373 + 1) * frame_size
                        chunks.append(all_samples[offset:offset+size])
                        offset += size
                        turn += 1
                else:
                    chunks = [all_samples]
                stream = u32(rate) + u32(channels) + u32(len(chunks))
                stream += b''.join(u32(len(chunk)) + chunk for chunk in chunks)
                compare(f'converter rate={rate} channels={channels} split={fragmented}', stream, 0, True)
                converter_cases += 1
    print(f'PASS: byte-identical Lean/Rust converters in {converter_cases} cases; '
          'all sample values, fragmented chunks and every consumed-frame count')
    print(f'PASS: byte-identical Lean/Rust protocol outputs in {cases + len(fixtures)} cases; '
          f'{65536 * cases:,} sample conversions plus maximum-size block')


if __name__ == '__main__':
    main()
