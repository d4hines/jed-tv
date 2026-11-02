#!/usr/bin/env python3
"""Bounded, offline execution of EXACT G06 kernel bytes. Never contacts the TV.
This is a diagnostic probe, not a claim of MediaTek platform emulation.
"""
import hashlib,json,pathlib,socket,struct,subprocess,time
ROOT=pathlib.Path(__file__).resolve().parents[1]
OUT=ROOT/'assets/research/kernel-parity'

class Rsp:
    def __init__(self,path):
        self.s=socket.socket(socket.AF_UNIX);self.s.settimeout(3)
        for _ in range(100):
            try:self.s.connect(str(path));break
            except (FileNotFoundError,ConnectionRefusedError):time.sleep(.02)
        self.call('qSupported:qXfer:features:read+')
    def call(self,cmd):
        b=cmd.encode();self.s.sendall(b'$'+b+b'#'+f'{sum(b)%256:02x}'.encode())
        while True:
            ch=self.s.recv(1)
            if not ch:raise RuntimeError('GDB closed before reply')
            if ch==b'$':break
        data=bytearray()
        while True:
            b=self.s.recv(1)
            if b==b'#':break
            if not b:raise RuntimeError('GDB closed')
            data+=b
        self.s.recv(2);self.s.sendall(b'+');return data.decode()
    def regs(self):
        data=self.call('g');return [int.from_bytes(bytes.fromhex(data[i:i+8]),'little') for i in range(0,128,8)],data
    def bootargs(self):
        regs,data=self.regs()
        # ARM core r0-r15 are the first 16 registers in the target description.
        for r,value in [(0,0),(1,0xffffffff),(2,0x23000000)]:
            at=r*8;data=data[:at]+value.to_bytes(4,'little').hex()+data[at+8:]
        reply=self.call('G'+data)
        if reply!='OK':raise RuntimeError('set registers failed: '+reply)


def run(name,cmd,manual=False,seconds=2):
    dest=OUT/'probes'/name;dest.mkdir(parents=True,exist_ok=True)
    path=pathlib.Path('/tmp')/f'g06-parity-{time.time_ns()}.sock'
    cmd += ['-display','none','-serial',f'file:{dest / "serial.log"}',
            '-monitor','none','-no-reboot','-S','-gdb',f'unix:{path},server=on,wait=off']
    (dest/'cmd.json').write_text(json.dumps(cmd,indent=2)+'\n')
    with open(dest/'stderr.log','w') as err:
        p=subprocess.Popen(cmd,stdout=err,stderr=err)
        report={'case':name,'kernel_bytes_modified':False,'stock_boot_success':False}
        try:
            r=Rsp(path)
            if manual:r.bootargs()
            first,_=r.regs();report['initial_core_registers']=[hex(v) for v in first]
            steps=[]
            for i in range(256):
                reply=r.call('s');regs,_=r.regs()
                if i<16 or i in [31,63,127,255]:steps.append({'step':i+1,'pc':hex(regs[15]),'r0':hex(regs[0]),'r1':hex(regs[1]),'r2':hex(regs[2])})
            report['steps']=steps
            # Continue for a bounded interval, then interrupt via the debug port.
            r.s.sendall(b'$c#63');time.sleep(seconds);r.s.sendall(b'\x03')
            # Collect stop reply without another command being outstanding.
            while True:
                ch=r.s.recv(1)
                if not ch:raise RuntimeError('QEMU exited before debug stop')
                if ch==b'$':break
            while True:
                ch=r.s.recv(1)
                if not ch:raise RuntimeError('QEMU exited during debug stop')
                if ch==b'#':break
            r.s.recv(2);r.s.sendall(b'+')
            final,_=r.regs();report['final_core_registers']=[hex(v) for v in final]
            report['serial_bytes']=(dest/'serial.log').stat().st_size
            physical=r.call('Qqemu.PhyMemMode:1')
            report['physical_debug_memory']=physical
            if physical=='OK':
                # Exact stock kallsyms: __log_buf=c12fdc78; fixed virtual/physical
                # delta is c0000000-20200000. Read the static ring, not secrets.
                buf=bytearray()
                for at in range(0,0x20000,0x400):
                    value=r.call(f'm{0x214fdc78+at:x},400')
                    if value.startswith('E'):break
                    buf+=bytes.fromhex(value)
                (dest/'static-printk-buffer.bin').write_bytes(buf)
                messages=[];at=0
                while at+16<=len(buf):
                    ts,length,textlen,dictlen,facility,flags=struct.unpack_from('<QHHHBB',buf,at)
                    if not length or length<16 or at+length>len(buf) or textlen>length-16:break
                    messages.append(buf[at+16:at+16+textlen].decode(errors='replace'));at+=length
                (dest/'kernel-messages.json').write_text(json.dumps(messages,indent=2)+'\n')
                report['kernel_message_count']=len(messages)

            report['result']='bounded execution stopped; inspect register trace, not a boot-success assertion'
        except Exception as e:report['error']=str(e)
        finally:
            p.terminate()
            try:p.wait(timeout=2)
            except subprocess.TimeoutExpired:p.kill();p.wait()
            try:path.unlink()
            except FileNotFoundError:pass
        (dest/'result.json').write_text(json.dumps(report,indent=2)+'\n');print(json.dumps(report,indent=2))


def main():
    OUT.mkdir(parents=True,exist_ok=True)
    boot=(ROOT/'assets/backups/v109-stock-20261031/boot.img').read_bytes()
    if hashlib.sha256(boot).hexdigest()!='d116dfa370d8e5dd5809f6f65266c86671b5e9bec9d62f95c0977bca0ff30085':
        raise RuntimeError('stock boot backup hash mismatch')
    size=struct.unpack_from('<I',boot,8)[0];page=struct.unpack_from('<I',boot,36)[0]
    (OUT/'stock.Image').write_bytes(boot[page:page+size])
    live=(ROOT/'assets/research/kexec/live.dtb').read_bytes()
    if hashlib.sha256(live).hexdigest()!='0a1ebf816890411ad9ab3652b9fc187a3ab09aa978df6d5e6c66c4f5e742649b':
        raise RuntimeError('captured live DTB hash mismatch')
    ramdisk=ROOT/'assets/build/tv-boot/m1a/ramdisk.cpio.gz'
    rd=ramdisk.read_bytes()
    if hashlib.sha256(rd).hexdigest()!='bd43ecf5e93e5bef305d1bb89a05af9f6f94990a1726c9a456c68132960cddbb':
        raise RuntimeError('diagnostic initramfs changed; review before updating pin')
    dtb=OUT/'probe-live.dtb';dtb.write_bytes(live)
    args=subprocess.check_output(['fdtget','-t','s',str(dtb),'/chosen','bootargs'],text=True).strip()
    # Retain MBOOT memory/pool parameters. Generated DTB and log stay private:
    # captured bootargs can contain device identifiers.
    subprocess.run(['fdtput','-t','s',str(dtb),'/chosen','bootargs',args+' rdinit=/init maxcpus=1 loglevel=8'],check=True)
    for prop,value in [('linux,initrd-start',0x24000000),('linux,initrd-end',0x24000000+len(rd))]:
        subprocess.run(['fdtput','-t','x',str(dtb),'/chosen',prop,f'{value:x}'],check=True)
    kernel=OUT/'stock.Image';expected='01e36bda0554aed46ad3aa1d7fdf688174f46241117f62b4b95f02e25c47c11e'
    if hashlib.sha256(kernel.read_bytes()).hexdigest()!=expected:raise RuntimeError('not exact dumped kernel')
    base=['qemu-system-aarch64','-M','virt-11.1,highmem=off,gic-version=2','-cpu','cortex-a55,aarch64=off','-smp','4','-m','1430M']
    run('virt-generated-dtb',base+['-kernel',str(kernel),'-append','console=ttyAMA0 loglevel=8'])
    run('virt-captured-dtb',base+['-kernel',str(kernel),'-dtb',str(OUT/'probe-live.dtb'),'-initrd',str(ROOT/'assets/build/tv-boot/m1a/ramdisk.cpio.gz')])
    # none maps RAM from zero (including would-be MMIO). This isolates early
    # address/entry problems only; it is NOT a usable TV machine model.
    run('none-fixed-dtb-address',['qemu-system-aarch64','-M','none','-cpu','cortex-a55,aarch64=off','-m','2048M',
        '-device',f'loader,file={kernel},addr=0x20208000,cpu-num=0,force-raw=on',
        '-device',f'loader,file={OUT / "probe-live.dtb"},addr=0x23000000,force-raw=on',
        '-device',f'loader,file={ROOT / "assets/build/tv-boot/m1a/ramdisk.cpio.gz"},addr=0x24000000,force-raw=on'],True)

if __name__=='__main__':main()
