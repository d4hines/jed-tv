#!/usr/bin/env python3
"""Generate tiny local AVC fixtures and verify the decoder's narrowed contract.
Requires release ytplay binaries for both targets, ffmpeg with libx264, and
QEMU. No network/replay server is needed. Artifacts stay under assets/.
Run: python3 tools/ffh264/check-formats.py
"""
import hashlib
import json
import pathlib
import re
import subprocess
import sys
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]


def main():
    out = ROOT / 'assets/build/ffh264/format-checks' / str(time.time_ns())
    out.mkdir(parents=True)
    fixtures = [('yuv420p', None), ('yuv420p10le', 'illegal bit depth value'),
                ('yuv444p', 'chroma_format_idc 3')]
    results = []
    for fmt, reject in fixtures:
        fixture = out / f'{fmt}.mp4'
        subprocess.run(['ffmpeg', '-hide_banner', '-loglevel', 'error', '-f', 'lavfi',
                        '-i', 'testsrc2=size=64x64:rate=4', '-frames:v', '64', '-c:v', 'libx264',
                        '-pix_fmt', fmt, '-movflags', 'dash+frag_keyframe+empty_moov+default_base_moof',
                        '-y', str(fixture)], check=True)
        for profile, target in [('fast', 'aarch64-unknown-linux-musl'),
                                ('virt32', 'armv7-unknown-linux-musleabihf')]:
            p = subprocess.run([sys.executable, str(ROOT / 'test/run.py'), profile,
                                '--headless', '--no-build', '--mode', 'bench', '--frames', '64',
                                '--add-file', f'{fixture}:/fixture.mp4', '--extra',
                                'ytplay.file=/fixture.mp4', '--out', str(out / f'{fmt}-{profile}'),
                                '--timeout', '30'], capture_output=True, text=True, timeout=45)
            log = p.stdout + p.stderr
            logpath = out / f'{fmt}-{profile}.log'
            logpath.write_text(log)
            failed = any(x in log for x in ['PANIC', 'panicked at', 'blocked syscall', 'Traceback'])
            if reject:
                ok = (p.returncode == 0 and reject in log and 'bench: error: h264 send error' in log
                      and not re.search(r'bench: decode only,.*: [1-9]\d* frames', log))
            else:
                ok = (p.returncode == 0 and len(re.findall(r'bench: decode only,.*: 64 frames', log)) == 3
                      and 'bench: decode+convert' in log and 'bench: error:' not in log)
            ok = ok and not failed
            binary = ROOT / 'assets/build/app-target' / target / 'release/ytplay'
            results.append(dict(profile=profile, pixel_format=fmt, passed=ok,
                                fixture_sha256=hashlib.sha256(fixture.read_bytes()).hexdigest(),
                                binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest(),
                                log=str(logpath.relative_to(ROOT))))
            print(f'{"PASS" if ok else "FAIL"}: {profile} {fmt}', flush=True)
    (out / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
    print(f'evidence: {out}')
    return 0 if all(r['passed'] for r in results) else 1


if __name__ == '__main__':
    sys.exit(main())
