#!/usr/bin/env python3
"""Launch the self-contained graphical PID-1 demo (generic virtio surrogate)."""
import argparse,hashlib,os,shutil,http.server,json,pathlib,platform,socket,struct,subprocess,sys,tempfile,threading,time

def main():
 ap=argparse.ArgumentParser(description=__doc__)
 ap.add_argument('--apps',required=True,type=pathlib.Path,help=argparse.SUPPRESS)
 ap.add_argument('--media',required=True,type=pathlib.Path,help=argparse.SUPPRESS)
 ap.add_argument('--source',required=True,type=pathlib.Path,help=argparse.SUPPRESS)
 ap.add_argument('--rpg',required=True,type=pathlib.Path,help=argparse.SUPPRESS)
 ap.add_argument('--axiom',required=True,type=pathlib.Path,help=argparse.SUPPRESS)
 ap.add_argument('--check',action='store_true',help='run a headless menu/video/slides/Doom smoke test')
 ap.add_argument('--check-supervisor',action='store_true',help='test confinement and Home TERM-to-KILL against an uncooperative app')
 ap.add_argument('--check-contract',action='store_true',help='test app ABI and cross-process display ownership')
 ap.add_argument('--check-tv-bridge',action='store_true',help='test shared RGBA and TCP PCM adapters with confined ARM32 apps')
 a=ap.parse_args();a.check = a.check or a.check_supervisor or a.check_tv_bridge or a.check_contract
 pcm=None
 if a.check_tv_bridge:
  sys.path.insert(0,str(a.source/"demo"))
  from pcm_stub import PcmStub
  pcm=PcmStub()
 sys.path.insert(0,str(a.source/'test'));import replay
 server=http.server.ThreadingHTTPServer(('127.0.0.1',0),replay.make_handler(json.loads((a.media/'player.json').read_text()),replay.load_fixtures(a.media/'fixtures'),None))
 threading.Thread(target=server.serve_forever,daemon=True).start();port=server.server_port
 with tempfile.TemporaryDirectory(prefix='pid1-demo-') as temp:
  out=pathlib.Path(temp);serial=out/'serial.log';qmp=out/'qmp';rd=out/'initramfs.cpio'
  files=[f'{a.apps}/bin/{"uncooperative" if a.check_supervisor and n=="ytplay" else n}:/bin/{n}' for n in ['ytplay','slides','doomtv','rpgtv','flighttv','snaketv']]+[f'{a.axiom}/bin/axiomtv:/bin/axiomtv']
  files += [f'{a.media}/doom/freedoom1.wad:/doom/freedoom1.wad']
  for p in sorted((a.media/'tiles').glob('*.rgb')):files.append(f'{p}:/share/tiles/{p.name}')
  files += [f'{a.rpg}/tile.rgb:/share/tiles/rpg.rgb']+[f'{p}:/share/rpg/{p.name}' for p in sorted((a.rpg/'pictures').glob('*.rgb'))]
  init=a.apps/'bin/contract-probe' if a.check_contract else a.apps/'bin/tvinit'
  if pcm:
   files.append(f"{init}:/bin/tvinit");init=a.apps/'bin/bridge-init'
  subprocess.run([sys.executable,str(a.source/'tools/mkinitramfs.py'),str(init),str(rd),*files],check=True)
  audio=f'wav,id=a0,path={out}/audio.wav' if a.check else ('coreaudio,id=a0' if platform.system()=='Darwin' else 'sdl,id=a0')
  # FLIGHT_SELFTEST=1: City flight compares its gate filter's parallel rollouts with the serial ones
  # (src/flighttv/src/main.rs, `flight.selftest=1`) instead of flying; the check waits for its verdict.
  # FLIGHT_ARGS: extra City flight options for experiments, e.g. FLIGHT_ARGS='flight.check_every=1'.
  selftest=a.check and os.environ.get('FLIGHT_SELFTEST')=='1'
  cmd=['qemu-system-aarch64','-name','PID1 demo (virt32 surrogate)','-M','virt-11.1,highmem=off,gic-version=2','-cpu','cortex-a55,aarch64=off','-smp','4','-m','1430M',
       '-kernel',str(a.media/'kernel/zImage'),'-initrd',str(rd),
       '-append',f'console=ttyAMA0 panic=-1 ip=off drm_kms_helper.drm_fbdev_overalloc=200 tvinit.demo=1 tvinit.iface=eth0 tvinit.ip=192.168.77.2/24 ytplay.server=http://192.168.77.1:{port}'+(f' bridge.audio=192.168.77.1:{pcm.server_address[1]}' if pcm else '')+(' flight.selftest=1' if selftest else '')+(' '+os.environ['FLIGHT_ARGS'] if os.environ.get('FLIGHT_ARGS') else ''),
       '-global','virtio-mmio.force-legacy=false','-netdev','user,id=n0,net=192.168.77.0/24,host=192.168.77.1','-device','virtio-net-device,netdev=n0',
       '-device','virtio-gpu-device,xres=1280,yres=720','-audiodev',audio,'-device','virtio-sound-device,audiodev=a0,streams=1','-device','virtio-keyboard-device','-device','virtio-rng-device',
       '-serial',f'file:{serial}','-qmp',f'unix:{qmp},server=on,wait=off','-no-reboot','-display','none' if a.check else ('cocoa' if platform.system()=='Darwin' else 'gtk')]
  print('Graphical PID-1 demo: Linux 6.18.57 / virtio surrogate (not the TV kernel).',flush=True)
  print('Arrows + Enter select; Home returns to launcher. Demo video is generated locally.',flush=True)
  if not a.check:print('A TV remote opens in its own window; its buttons send remote keys to the emulator.',flush=True)
  with (out/'qemu.log').open('w') as err:
   p=subprocess.Popen(cmd,stdout=err,stderr=err)
   try:
    if not a.check:
     remote=subprocess.Popen([sys.executable,str(a.source/'demo/remote.py'),str(qmp)])
     try:code=p.wait()
     finally:remote.terminate();remote.wait()
     if code:print((out/'qemu.log').read_text(),file=sys.stderr)
     return code
    def wait(marker,timeout=30):
     until=time.monotonic()+timeout
     while time.monotonic()<until:
      text=serial.read_text(errors='replace') if serial.exists() else ''
      if marker in text:return
      if p.poll() is not None:raise RuntimeError((out/'qemu.log').read_text()+'\n'+text[-4000:])
      if 'PANIC' in text or 'play: error:' in text:raise RuntimeError(text[-3000:])
      time.sleep(.1)
     raise RuntimeError('Timed out waiting for '+marker+'\n'+text[-4000:])
    if a.check_contract:
     wait('APP-API PASS:');print('PASS: app ABI and cross-process display ownership',flush=True);return 0
    wait('home screen: 3 rows')
    s=socket.socket(socket.AF_UNIX);s.settimeout(5);s.connect(str(qmp));f=s.makefile('rw');f.readline()
    def q(name,**args):
     f.write(json.dumps({'execute':name,'arguments':args})+'\n');f.flush()
     while True:
      r=json.loads(f.readline())
      if 'return' in r:return r['return']
      if 'error' in r:raise RuntimeError(str(r))
    q('qmp_capabilities')
    def key(code):q('send-key',keys=[{'type':'qcode','data':code}]);time.sleep(.25)
    shot=pathlib.Path.cwd()/'assets/build/pid1-demo-check.png'
    shot.parent.mkdir(parents=True,exist_ok=True)
    for _ in range(40):
     q('screendump',filename=str(shot),format='png');png=shot.read_bytes()
     if len(png)>5000 and struct.unpack_from('>II',png,16)==(1280,720):break
     time.sleep(.1)
    else:raise RuntimeError('Menu never produced an active 1280x720 display')
    def snapshot(name):
     time.sleep(.4)
     path=out/(name+'.png');q('screendump',filename=str(path),format='png')
     image=path.read_bytes()
     assert len(image)>5000 and struct.unpack_from('>II',image,16)==(1280,720), name
    snapshot('menu')
    if a.check_supervisor:
     key('ret');wait('fixture: uncooperative app running');wait('fixture: confinement PASS')
     key('home');wait('fixture: ignored SIGTERM');key('home');wait('Demo video stopped')
     text=serial.read_text(errors='replace')
     assert text.count('home button: stopping the app')==1
     assert text.count('fixture: ignored SIGTERM')==1
     key('down');key('right');key('ret');wait('slide 1/');key('esc');wait('Slides exited')
     print('PASS: Lean confinement blocks subprocesses, permits threads, ignores duplicate Home, escalates TERM to KILL and launches the next app',flush=True)
     return 0
    key('ret');wait('audio: AAC');time.sleep(2);snapshot('video');key('home');wait('Demo video stopped')
    key('down');key('ret');wait('YouTube app: search and four suggestions');snapshot('youtube')
    key('ret');time.sleep(.4);snapshot('youtube-search');key('esc');wait('YouTube app: returned to suggestions')
    time.sleep(.3)
    assert 'YouTube exited' not in serial.read_text(errors='replace'), 'Back escaped the app'
    key('esc');wait('YouTube exited');key('up')
    # YouTube has no socket syscall: every connection above came from PID 1's broker.
    text=serial.read_text(errors='replace')
    assert 'broker: granted YouTube 192.168.77.1:' in text and 'broker: connected 192.168.77.1:' in text, 'YouTube did not connect through PID 1'
    assert not any('broker:' in l and ('refused' in l or 'failed' in l) for l in text.splitlines()), 'a broker request was refused or failed'
    key('down');key('right');key('ret');wait('slide 1/');snapshot('slides');key('esc');wait('Slides exited')
    key('right');key('ret');wait('doom: engine up');wait('tics/s (35 = full speed)');snapshot('doom');key('home');wait('Doom stopped')
    key('right');key('ret');wait('rpgtv: phase title');key('ret');wait('rpgtv: phase scene "The Shore"');snapshot('rpg')
    key('ret');wait('rpgtv: phase scene "The Iron Door"');key('esc');wait('Infinite RPG exited')
    # City flight: the Lean JSBSim plant and the gate filter run on the TV; the city takes a while to build.
    key('right');key('ret');wait('flighttv: JSBSim plant and gate filter ready',timeout=180)
    if selftest:
     wait('flighttv: selftest ',timeout=900)
     assert 'flighttv: selftest PASS' in serial.read_text(errors='replace'), 'the parallel gate differs from the serial one'
    else:
     wait('flighttv: sim ',timeout=120);wait(' fps, ',timeout=60);snapshot('flight')
    key('esc');wait('City flight exited')
    # Axiom: the Lean voxel game generates and meshes its island on the TV first.
    key('right');key('ret');wait('axiomtv: meshed ',timeout=300);wait('fps, title',timeout=120)
    key('ret');wait('fps, playing',timeout=60)
    snapshot('axiom');key('esc');key('esc');key('esc');wait('Axiom exited')
    # Snake on a torus: a step every 125 ms; Left turns the snake outward, and it must keep going.
    key('right');key('ret');wait('snaketv: new game: length 3');wait('snaketv: first frame')
    key('left');wait('snaketv: turn left: heading out');time.sleep(1.5);snapshot('snake')
    key('esc');wait('Snake exited')
    text=serial.read_text(errors='replace');assert 'play: error:' not in text
    assert 'axiomtv: PANIC' not in text and 'present failed' not in text
    assert 'snaketv: PANIC' not in text and 'snaketv: crashed' not in text
    assert 'flighttv: PANIC' not in text and 'crashed at' not in text, 'the filter let the scenario crash'
    if pcm:
     wait('bridge-init: opaque shared frames presented')
     assert pcm.opens>=2 and pcm.frames>24000 and not pcm.errors, vars(pcm)
    print('PASS: graphical menu, video/audio, nested YouTube Back, Home, Slides, Doom, Infinite RPG, City flight, Axiom and Snake'+(' through TV bridge protocols' if pcm else '')+'; '+str(shot),flush=True)
    return 0
   finally:
    for proc in [p]:
     if proc.poll() is None:
      proc.terminate()
      try:proc.wait(timeout=5)
      except subprocess.TimeoutExpired:proc.kill();proc.wait()
    evidence=pathlib.Path.cwd()/f'assets/build/integration-qemu-{time.time_ns()}'
    evidence.mkdir(parents=True,exist_ok=True)
    for name in ['serial.log','qemu.log','audio.wav']+[p.name for p in out.glob('*.png')]:
     if (out/name).exists():shutil.copyfile(out/name,evidence/name)
    report={'tv_bridge_protocols':bool(pcm),'vendor_hardware_emulated':False,'kernel_sha256':hashlib.sha256((a.media/'kernel/zImage').read_bytes()).hexdigest(),'apps':{n:hashlib.sha256((a.apps/'bin'/n).read_bytes()).hexdigest() for n in ['tvinit','ytplay','slides','doomtv','rpgtv','flighttv','snaketv']}}
    if pcm:
     report['pcm']={'opens':pcm.opens,'frames':pcm.frames,'resets':pcm.resets,'errors':pcm.errors}
     pcm.shutdown();pcm.server_close()
    (evidence/'fidelity.json').write_text(json.dumps(report,indent=2)+'\n')
    print(f'Evidence: {evidence}',flush=True)
    server.shutdown();server.server_close()
if __name__=='__main__':
 try:raise SystemExit(main())
 except KeyboardInterrupt:raise SystemExit(130)
