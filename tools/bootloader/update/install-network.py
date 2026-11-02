#!/usr/bin/env python3
"""Install the exact RAM-tested network gate, preserving the existing journal.

Only boot_a's 128 KiB kernel prefix is written. Requires exact device/hash and
successful same-image RAM boot with signed network execution. No reboot here.
"""
import sys
sys.dont_write_bytecode=True
import argparse
import hashlib
import json
from datetime import datetime,timezone
from pathlib import Path

ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'tools/native-app'))
import g13_device as d

BUILD=ROOT/'assets/build/leanboot-network'
EVIDENCE=ROOT/'assets/research/g13-network-default'
PINNED={
    'boot_a':'53d3a25639b7bf5ecd3f493d0a958e9eac33a4f026f3ffdc4aee53a49a4ba3dd',
    'init_boot_a':'0ebb78f7eb550f8b2f524cf661342baeeaa55a70e3c3e5708d71f26381fc5e8b',
    'MBOOT':'a342fcdd62b2ce7121d977b9410d2c05aa51757bf7d26d6b445423e97122439b',
    'MBOOT_b':'643058879b84d044dea58a4572bd9ade1cc3140f3d3357eeaabae53be32ec382',
    'MBOOTBAK':'643058879b84d044dea58a4572bd9ade1cc3140f3d3357eeaabae53be32ec382'}

def sha(data):return hashlib.sha256(data).hexdigest()

def main():
    p=argparse.ArgumentParser(description=__doc__);p.add_argument('--apply',action='store_true')
    args=p.parse_args()
    image=(BUILD/'g13-boot_a.img').read_bytes();digest=sha(image)
    build=json.loads((BUILD/'build.json').read_text())
    for name,want in build['source_sha256'].items():assert sha((ROOT/name).read_bytes())==want,name
    assert digest==json.loads((BUILD/'boot-image.json').read_text())['image_sha256']
    public=(ROOT/'assets/private/network-boot/public.raw').read_bytes()
    assert len(public)==32 and sha(public)==build['public_key_sha256']
    assert sha(public)!='21fe31dfa154a261626bf854046fd2271b7bed4b6abe45aa58877ef47f9721b9','Never install the public test key'
    assert json.loads((BUILD/'audit.json').read_text())['outputs']==build['outputs']
    assert json.loads((BUILD/'checks.json').read_text())['sha256']==build['outputs']['network-ram']['sha256']
    cases=json.loads((BUILD/'gate-checks.json').read_text());assert len(cases)==6
    trial=json.loads((EVIDENCE/'ram-trial/result.json').read_text())
    serial=(EVIDENCE/'ram-trial/boot.bin').read_bytes()
    assert trial['image_sha256']==digest and trial['ready'] and trial['route']=='lean'
    assert b'LEANBOOT NET' in serial and b'NETWORK LEAN' in serial,'Same-image signed network execution required'
    d.connect();d.root()
    assert d.shell('getprop ro.boot.slot_suffix').strip()=='_a'
    assert d.shell('pidof tvinit').strip(),'Normal native userspace must be running'
    for name,want in PINNED.items():
        assert d.shell('sha256sum /dev/block/by-name/'+name).split()[0]==want,name
    before=d.adb('exec-out','cat /dev/block/by-name/boot_a',capture_output=True,timeout=120).stdout
    assert sha(before)==PINNED['boot_a'] and len(before)==len(image)==55574528
    assert before[:4096]==image[:4096] and before[0x21000:]==image[0x21000:]
    print('PASS: exact target, recovery images, source/key hashes, signed RAM trial and preserved boot-image body')
    if not args.apply:return
    out=EVIDENCE/('install-'+datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ'))
    out.mkdir(parents=True)
    (out/'boot_a-before.img').write_bytes(before)
    for name,data in [('before-prefix',before[4096:0x21000]),('candidate-prefix',image[4096:0x21000])]:
        path=out/(name+'.bin');path.write_bytes(data)
        remote='/data/local/tmp/leanboot-'+name+'.bin'
        d.adb('push',path,remote,capture_output=True,timeout=30)
        assert d.shell('sha256sum '+remote).split()[0]==sha(data)
    d.shell('dd if=/data/local/tmp/leanboot-candidate-prefix.bin of=/dev/block/by-name/boot_a bs=4096 seek=1 count=32 conv=fsync 2>/dev/null; sync; blockdev --flushbufs /dev/block/by-name/boot_a')
    actual=d.shell('sha256sum /dev/block/by-name/boot_a').split()[0]
    if actual!=digest:
        d.shell('dd if=/data/local/tmp/leanboot-before-prefix.bin of=/dev/block/by-name/boot_a bs=4096 seek=1 count=32 conv=fsync 2>/dev/null; sync; blockdev --flushbufs /dev/block/by-name/boot_a')
        restored=d.shell('sha256sum /dev/block/by-name/boot_a').split()[0]
        raise RuntimeError('Candidate mismatch; previous-image restore hash='+restored)
    result={'boot_a_before':PINNED['boot_a'],'boot_a_after':actual,'public_key_sha256':sha(public),
            'offset':4096,'bytes':131072,'journal_reseeded':False,'other_partitions_written':False,'rebooted':False}
    (out/'result.json').write_text(json.dumps(result,indent=2)+'\n')
    print(json.dumps(result));print('Evidence:',out)

if __name__=='__main__':main()
