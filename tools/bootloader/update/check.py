#!/usr/bin/env python3
"""Signed-update/receiver/fault tests against compiled ARM; never contacts a TV."""
import sys
sys.dont_write_bytecode = True
import argparse
import hashlib
import json
import struct
import subprocess
from pathlib import Path
from model import Machine, PowerCut, bank_image, OUT, SOURCE
from package import package


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--key',type=Path,help='host-only key matching the locally built verifier; default uses the public test seed')
    args=parser.parse_args()
    fixtures=OUT/'fixtures';fixtures.mkdir(parents=True,exist_ok=True)
    # Public RFC 8032 test seed; this is deliberately not a release key.
    seed=bytes.fromhex('9d61b19deffd5a60ba844af492ec2cc44449c5697b326919703bac031cae7f60')
    der=fixtures/'test-key.der';pem=fixtures/'test-key.pem'
    der.write_bytes(bytes.fromhex('302e020100300506032b657004220420')+seed)
    subprocess.run(['openssl','pkey','-inform','DER','-in',str(der),'-out',str(pem)],check=True)
    pem.chmod(0o600)
    pub=bytes.fromhex('d75a980182b10ab7d54bfed3c964073a0ee172f3daa62325af021a68f707511a')
    if args.key:
        pem=args.key
        public=subprocess.check_output(['openssl','pkey','-in',str(pem),'-pubout','-outform','DER'])
        assert len(public)==44 and public[:12]==bytes.fromhex('302a300506032b6570032100'), 'Expected Ed25519'
        pub=public[12:]
    assert json.loads((OUT/'build.json').read_text())['public_key_sha256']==hashlib.sha256(pub).hexdigest(),'Key does not match the locally built verifier'
    old=package(bytes(range(256))*4,pem,1)
    new=package(bytes((i*31+17)%256 for i in range(1537)),pem,2,entry=4)
    (fixtures/'v1.lbu').write_bytes(old);(fixtures/'v2.lbu').write_bytes(new)
    original=bank_image(old)
    def fresh():return Machine([bytearray(original),bytearray(len(original))])
    cases=0
    m=fresh();assert m.version(0)==1 and m.version(1)==0
    assert m.install(new)==0
    assert m.banks[0]==original and m.version(1)==2
    writes=len(m.writes);assert all(b==1 for b,s in m.writes)
    assert m.writes[0]==(1,0) and m.writes[-1]==(1,0)
    assert [s for b,s in m.writes[1:-1]]==[1,2,3,4]
    m.save(OUT/'demo-banks');cases+=1
    print('PASS: actual Ed25519/SHA-256, ARM install, bank preservation and readback',flush=True)
    for offset in [0,4,8,12,16,20,24,28,32,36,40,71,72,95,96,127,159,160,len(new)-1]:
        bad=bytearray(new);bad[offset]^=1
        m=fresh();assert m.install(bytes(bad))!=0 and not m.writes,offset;cases+=1
    for bad in [new[:-1],new+b'\0',new[:100],package(new[160:],pem,2,board=1),package(new[160:],pem,2,component=2)]:
        m=fresh();assert m.install(bad)!=0 and not m.writes;cases+=1
    m=Machine([bytearray(original),bank_image(new)])
    assert m.install(old)!=0 and not m.writes;cases+=1
    m=Machine([bytearray(original),bytearray(original)])
    assert m.install(new)==7 and not m.writes;cases+=1
    m=Machine([bytearray(original),bank_image(new)]);m.banks[1][512]^=1
    assert m.version(1)==0 and m.version(0)==1;cases+=1
    print('PASS: unsigned/tampered/wrong-target/truncated/downgrade/ambiguous/corrupt updates reject',flush=True)
    # Power loss is injected in every write, including final commit. Prefix
    # tears model a subset of real media failures; no hardware atomicity claim.
    for cut in range(1,writes+1):
        for torn in [0,1,159,160,511,512]:
            m=fresh();m.cut_write=cut;m.torn=torn
            try:m.install(new);assert False,'fault was not reached'
            except PowerCut:pass
            after=Machine(m.banks)
            assert m.banks[0]==original and after.version(0)==1,(cut,torn)
            assert after.version(1) in (0,2),(cut,torn)
            cases+=1
    for flush in range(1,5):
        m=fresh();m.fail_flush=flush;assert m.install(new)!=0
        assert m.banks[0]==original and Machine(m.banks).version(0)==1;cases+=1
    for fault in ['fail_read','corrupt_read']:
        m=fresh();setattr(m,fault,5)
        assert m.install(new)==5 and m.writes[-1]!=(1,0)
        assert m.banks[0]==original;cases+=1
    print('PASS: interrupted writes and flush failures preserve the previous authenticated bank',flush=True)
    # Exact receiver bytes, retransmissions, ordering and session binding.
    def packet(op,session,offset,n,data=b''):return struct.pack('<5I',0x3152554c,op,session,offset,n)+data
    m=fresh();n=len(new)
    assert m.receive(packet(1,42,0,n))==0
    assert m.receive(packet(1,43,0,n))>>32==2
    assert m.receive(packet(3,42,n,0))>>32==4
    assert m.receive(packet(2,42,1,1,b'x'))>>32==3
    for off in range(0,n,1024):
        data=new[off:off+1024];p=packet(2,42,off,len(data),data)
        ack=off+len(data);assert m.receive(p)==ack and m.receive(p)==ack
        if off==0:
            bad=bytearray(p);bad[-1]^=1
            assert m.receive(bytes(bad))>>32==5
    assert m.receive(packet(3,42,n,0))==n
    assert bytes(m.u.mem_read(SOURCE,n))==new and not m.writes
    assert m.install(new)==0 and m.version(1)==2
    assert m.receive(packet(4,42,0,0))==0
    cases+=1
    report={'cases':cases,'power_cuts':writes*6,'real_arm_crypto':True,'hardware_tested':False,
            'new_authored_assembly':0,'max_observed_stack_bytes':Machine.max_stack_bytes}
    (OUT/'checks.json').write_text(json.dumps(report,indent=2)+'\n')
    print('PASS:',json.dumps(report),flush=True)


if __name__=='__main__':main()
