#!/usr/bin/env python3
"""Replace the running native supervisor with the exact QEMU-tested build."""
import datetime
import hashlib
import json
from pathlib import Path
import re
import subprocess
import time
from watch import request

ROOT=Path(__file__).resolve().parents[2]
def shell(command):
    return subprocess.check_output([str(ROOT/'repl.sh'),'-c',command],text=True)

manifest=json.loads((ROOT/'assets/build/pid1-repl/manifest.json').read_text())
binary=(ROOT/'assets/build/pid1-repl/target/armv7-unknown-linux-musleabihf/release/hello-pid1').read_bytes()
assert hashlib.sha256(binary).hexdigest()==manifest['init_sha256']
checks=[json.loads(p.read_text()) for p in (ROOT/'assets/build/pid1-repl/qemu').glob('*/result.json')]
assert any(c.get('passed') and c.get('init_sha256')==manifest['init_sha256']
           and 'live supervisor reload preserves RAM and console' in c.get('checks',[]) for c in checks)
before=json.loads(request('GET','/status'))
assert before['pid']==1 and before['phase']=='shell-ready'
identity=shell('cat /proc/cmdline; cat /proc/sys/kernel/random/boot_id')
assert 'androidboot.serialno=4C0490290FA05A0C0' in identity
boot=re.findall(r'(?m)^([a-f0-9-]{36})\s*$',identity)[0]
out=ROOT/'assets/research/pid1-volume'/datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
out.mkdir(parents=True)
(out/'before.txt').write_text(identity)
request('PUT','/files/pid1-live-handoff',b'verified live supervisor handoff\n')
print(request('POST','/exec',binary,timeout=5).decode(),flush=True)
time.sleep(.3)
deadline=time.monotonic()+15
while time.monotonic()<deadline:
    try:
        status=json.loads(request('GET','/status'))
        if status['build']==manifest['build'] and status['phase']=='shell-ready': break
    except (OSError,ValueError): pass
    time.sleep(.2)
else: raise RuntimeError('Supervisor did not reconnect; check Android/TV state')
after=shell('cat /proc/sys/kernel/random/boot_id; sha256sum /proc/1/exe')
assert boot in after and manifest['init_sha256'] in after
volume=json.loads(request('GET','/volume'))
assert volume['owner_pid']==1 and volume['ready']
(out/'after.txt').write_text(after)
(out/'result.json').write_text(json.dumps({'passed':True,'boot_id_unchanged':True,
    'init_sha256':manifest['init_sha256'],'status':status,'volume':volume},indent=2)+'\n')
print('PASS: supervisor upgraded without reboot; PID 1 owns volume; root console retained')
print('Evidence:',out)
