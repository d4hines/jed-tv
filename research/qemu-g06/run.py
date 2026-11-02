#!/usr/bin/env python3
"""Boot the exact dumped G06 kernel on the local partial G06 machine (offline)."""
import argparse,gzip,hashlib,json,pathlib,socket,struct,subprocess,tempfile,time
from contract import verified_snapshot
ROOT=pathlib.Path(__file__).resolve().parents[2]
HERE=pathlib.Path(__file__).resolve().parent
BUILD=ROOT/'assets/build/qemu-g06'
KERNEL_SHA='01e36bda0554aed46ad3aa1d7fdf688174f46241117f62b4b95f02e25c47c11e'
BOOT_SHA='d116dfa370d8e5dd5809f6f65266c86671b5e9bec9d62f95c0977bca0ff30085'
LIVE_SHA='0a1ebf816890411ad9ab3652b9fc187a3ab09aa978df6d5e6c66c4f5e742649b'
SKIP='mstar_mci_init,mmc_sdio_init,mtktv_regulator_init,mtktv_thermal_init,ir_dynamic_config_init'
APP=ROOT if (ROOT/'src').is_dir() else ROOT.parent/'agent1'
def digest(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def checked(p,sha):
    if digest(p)!=sha:raise RuntimeError('Unreviewed input hash: '+str(p))
    return p.read_bytes()
def watchdog_observation(path):
    with socket.socket(socket.AF_UNIX) as sock:
        sock.settimeout(2)
        sock.connect(str(path))
        with sock.makefile('rw') as stream:
            json.loads(stream.readline())
            def command(name, **args):
                stream.write(json.dumps({'execute':name,'arguments':args})+'\n');stream.flush()
                while True:
                    reply=json.loads(stream.readline())
                    if 'return' in reply:return reply['return']
                    if 'error' in reply:raise RuntimeError(str(reply['error']))
            command('qmp_capabilities')
            return {name:command('qom-get',path='/machine',property=name) for name in
                    ['watchdog-clear-count','watchdog-other-write-count','watchdog-last-clear-ns']}
def main():
    ap=argparse.ArgumentParser(description=__doc__)
    ap.add_argument('--init',type=pathlib.Path,default=ROOT/'assets/build/app-target/armv7-unknown-linux-musleabihf/release/tvinit')
    ap.add_argument('--smoke',action='store_true',help='test stock ABI, then exec real tvinit in hold mode')
    ap.add_argument('--timeout',type=float,default=60)
    ap.add_argument('--cpus',type=int,choices=[1,4],default=4)
    ap.add_argument('--extra',default='')
    ap.add_argument('--add-file',action='append',default=[])
    ap.add_argument('--out',type=pathlib.Path,default=ROOT/'assets/build/qemu-runs')
    a=ap.parse_args()
    hardware = verified_snapshot()
    qemu=BUILD/'build/qemu-system-aarch64'
    if not qemu.exists():raise SystemExit('First build: python3 research/qemu-g06/build.py')
    out=a.out/f'g06-stock-{time.time_ns()}';out.mkdir(parents=True)
    boot=checked(ROOT/'assets/backups/v109-stock-20261031/boot.img',BOOT_SHA)
    kernel=out/'Image';page=struct.unpack_from('<I',boot,36)[0];size=struct.unpack_from('<I',boot,8)[0]
    kernel.write_bytes(boot[page:page+size]);checked(kernel,KERNEL_SHA)
    dtb=out/'board.dtb';dtb.write_bytes(checked(ROOT/'assets/research/kexec/live.dtb',LIVE_SHA))
    existing=subprocess.check_output(['fdtget','-t','s',str(dtb),'/chosen','bootargs'],text=True).split()
    keep={'LX_MEM','LX_MEM2','EMAC_MEM','CMA0','CMA1','CMA2','DMA_SIZE','SD_CONFIG','platform','Pmode','SAR5'}
    args=' '.join(s for s in existing if s.split('=')[0] in keep)
    args+=' console=ttyS0,115200 loglevel=5 rdinit=/init root=/dev/ram rw panic=-1 initcall_blacklist='+SKIP
    args+=' '+a.extra
    init=a.init.resolve();extras=list(a.add_file)
    if a.smoke:
        smoke=BUILD/'init-smoke'
        subprocess.run(['zig','cc','-target','arm-linux-musleabihf','-mcpu=cortex_a55','-static','-Os','-pthread',str(ROOT/'test/fixtures/qemu/init-smoke.c'),'-o',str(smoke)],check=True)
        conf=out/'tvinit.conf';conf.write_text('tvinit.hold=1\n')
        extras += [f'{init}:/bin/tvinit',f'{conf}:/etc/tvinit.conf']
        init=smoke
    cpio=out/'ramdisk.cpio'
    subprocess.run(['python3',str(APP/'tools/mkinitramfs.py'),str(init),str(cpio),*extras],check=True)
    ram=out/'ramdisk.cpio.gz';ram.write_bytes(gzip.compress(cpio.read_bytes(),mtime=0))
    for prop,value in [('linux,initrd-start',0x24000000),('linux,initrd-end',0x24000000+ram.stat().st_size)]:
        subprocess.run(['fdtput','-t','x',str(dtb),'/chosen',prop,f'{value:x}'],check=True)
    subprocess.run(['fdtput','-t','s',str(dtb),'/chosen','bootargs',args],check=True)
    stub=out/'entry.bin'
    stub.write_bytes(struct.pack('<6I',0xe3a00000,0xe3e01000,0xe59f2000,0xe59ff000,0x23000000,0x20208000))
    serial=out/'serial.log'
    control=tempfile.TemporaryDirectory(prefix='g06-qmp-')
    qmp_path=pathlib.Path(control.name)/'control'
    cmd=[str(qemu),'-M','g06','-smp',str(a.cpus),'-m','1430M','-display','none','-monitor','none','-no-reboot',
         '-qmp',f'unix:{qmp_path},server=on,wait=off',
         '-serial',f'file:{serial}',
         '-device',f'loader,file={kernel},addr=0x20208000,force-raw=on',
         '-device',f'loader,file={dtb},addr=0x23000000,force-raw=on',
         '-device',f'loader,file={ram},addr=0x24000000,force-raw=on',
         '-device',f'loader,file={stub},addr=0x20200000,cpu-num=0,force-raw=on']
    (out/'cmd.json').write_text(json.dumps(cmd,indent=2)+'\n')
    print(f'kernel: exact dumped G06 4.19.116+ {KERNEL_SHA}',flush=True)
    print('platform: partial G06 model; no working display/audio/network emulation',flush=True)
    print(f'run dir: {out}',flush=True)
    report={'kernel_sha256':digest(kernel),'stock_boot_sha256':BOOT_SHA,'kernel_bytes_modified':False,
            'qemu_sha256':digest(qemu),'machine_source_sha256':digest(HERE/'g06.c'),'dtb_sha256':digest(dtb),
            'init_sha256':digest(init),'ramdisk_sha256':digest(ram),'cpus_requested':a.cpus,
            'disabled_initcalls':SKIP.split(','),'stock_kernel':True,'abi_smoke_pass':False,'tvinit_started':False,
            'devices_ready':False, 'hardware_observations':hardware,
            'watchdog_model':'service writes counted; expiration unmodeled',
            'display_model':'absent; observed fbdev/MMA contract recorded only',
            'network_model':'no packet backend; native hardware control channel proven separately'}
    with (out/'qemu.log').open('w') as err:
        p=subprocess.Popen(cmd,stdout=err,stderr=err)
        start=time.monotonic();marked=None
        try:
            while p.poll() is None and time.monotonic()-start<a.timeout:
                text=serial.read_text(errors='replace') if serial.exists() else ''
                if 'Kernel panic' in text or 'Internal error:' in text:break
                report['abi_smoke_pass']='G06-SMOKE ABI PASS' in text
                report['tvinit_started']='tvinit: hold: bring-up only' in text
                report['four_cpus_online']='cpus=4' in text
                if a.smoke and report['abi_smoke_pass'] and report['tvinit_started']:
                    if marked is None:marked=time.monotonic()
                    if time.monotonic()-marked>=2:break
                time.sleep(.1)
            report['guest_alive_at_stop']=p.poll() is None
        finally:
            try:report['watchdog_observation']=watchdog_observation(qmp_path)
            except Exception as e:report['watchdog_observation_error']=str(e)
            p.terminate()
            try:p.wait(timeout=3)
            except subprocess.TimeoutExpired:p.kill();p.wait()
            control.cleanup()
    final_text=serial.read_text(errors='replace') if serial.exists() else ''
    report['kernel_fault']=('Kernel panic' in final_text or 'Internal error:' in final_text)
    report['elapsed_seconds']=round(time.monotonic()-start,3)
    (out/'fidelity.json').write_text(json.dumps(report,indent=2)+'\n')
    text=serial.read_text(errors='replace') if serial.exists() else ''
    if a.smoke:
        lines=[s for s in text.splitlines() if 'G06-SMOKE' in s or 'tvinit: hold:' in s]
        print('\n'.join(lines),flush=True)
        ok=report['abi_smoke_pass'] and report['tvinit_started'] and report['guest_alive_at_stop'] and not report['kernel_fault']
        if a.cpus==4:ok=ok and report['four_cpus_online']
        print('PASS' if ok else 'FAIL',flush=True)
        return 0 if ok else 1
    print('\n'.join(text.splitlines()[-20:]),flush=True)
    return 0 if report['guest_alive_at_stop'] and not report['kernel_fault'] else 1
if __name__=='__main__':raise SystemExit(main())
