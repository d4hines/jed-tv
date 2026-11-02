#!/usr/bin/env python3
"""Run the full app in the existing G06 native shell; no reboot or flash."""
from pathlib import Path
import argparse
import gzip
import json
import datetime
import socket
import time
import subprocess
import sys

ROOT=Path(__file__).resolve().parents[2]
sys.path.insert(0,str(ROOT/'tools/pid1'))
from graphics import upload, console
from watch import request
sys.path.insert(0,str(ROOT/'tools/network'))
from stage import stage_network

def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--prepare-only',action='store_true',help='build/upload without restarting the running UI')
    args=parser.parse_args()
    status=json.loads(request('GET','/status'))
    services=status.get('services',{})
    if (status.get('pid') != 1 or services.get('hardware') != 'ready'
            or services.get('launcher') != 'resident'
            or services.get('audio') != 'listening'
            or services.get('display') not in ('waiting','presenting')):
        raise RuntimeError('Boot the current PID1 REPL with direct services before starting the app')
    media=ROOT/'assets/build/native-app-media'
    if not (media/'player.json').is_file():
        (media/'fixtures').mkdir(parents=True,exist_ok=True)
        subprocess.run([sys.executable,'tools/native-app/make-fixtures.py',str(media)],cwd=ROOT,check=True)
    result=subprocess.run(['nix','build','.#pid1.passthru.rpg','--no-link','--json'],cwd=ROOT,check=True,capture_output=True,text=True)
    rpg=Path(json.loads(result.stdout)[0]['outputs']['out'])
    services=[('192.168.77.1',8090,['tools/native-app/replay.py','--player',str(media/'player.json'),
                       '--fixtures',str(media/'fixtures'),'--host','192.168.77.1','--port','8090']),
              ('192.168.2.1',8899,['tools/native-app/tunnel.py','--host','192.168.2.1'])]
    for host,port,arguments in services:
        try:
            with socket.create_connection((host,port),timeout=1): pass
        except OSError:
            log=open(ROOT/f'assets/build/native-app-host-{port}.log','ab')
            process=subprocess.Popen([sys.executable,*arguments],cwd=ROOT,stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
            log.close()
            (ROOT/f'assets/build/native-app-host-{port}.pid').write_text(str(process.pid)+'\n')
            time.sleep(0.5)
            if process.poll() is not None: raise RuntimeError(f'Host service {port} failed')
    subprocess.run(['sh','tools/native-app/build.sh'],cwd=ROOT,check=True)
    unpack=[]
    for name in ('slides','ytplay','doomtv','rpgtv','flighttv','snaketv','ytplay-lean','doomtv-lean'):
        source=ROOT/'assets/build/native-app-target/armv7-unknown-linux-musleabihf/release'/name
        packed=ROOT/'assets/build'/('app-next-'+name+'.gz')
        packed.write_bytes(gzip.compress(source.read_bytes(),compresslevel=1,mtime=0))
        upload(packed)
        unpack.append(f'gzip -dc /tmp/{packed.name} > /tmp/app-next-{name}')
        unpack.append(f'chmod 755 /tmp/app-next-{name}')
        unpack.append(f'rm /tmp/{packed.name}')
    unpack_script = ROOT / 'assets/build/unpack-apps.sh'
    unpack_script.write_text('set -e\n' + '\n'.join(unpack) + '\n')
    upload(unpack_script)
    console('sh /tmp/unpack-apps.sh')
    packed=gzip.compress((ROOT/'assets/third_party/freedoom/freedoom1.wad').read_bytes(),compresslevel=1,mtime=0)
    parts=[]
    for index,offset in enumerate(range(0,len(packed),4*1024*1024)):
        part=ROOT/f'assets/build/app-wad-{index:03d}'
        part.write_bytes(packed[offset:offset+4*1024*1024])
        upload(part); parts.append('/tmp/'+part.name)
    console('cat '+' '.join(parts)+' > /tmp/app-wad.gz; rm '+' '.join(parts))
    flight_tile=media/'tiles/flight.rgb'
    flight_tile.parent.mkdir(parents=True,exist_ok=True)
    subprocess.run(['ffmpeg','-v','error','-y','-i',str(ROOT/'demo/flight-tile.png'),
                    '-vf','scale=288:162','-pix_fmt','rgb24','-f','rawvideo',str(flight_tile)],check=True)
    subprocess.run([sys.executable, 'tools/make-menu-tiles.py', str(media/'tiles')],cwd=ROOT,check=True)
    for name in ('flight', 'demo-video', 'youtube', 'doom'):
        upload(media/'tiles'/f'{name}.rgb')
    upload(rpg/'tile.rgb','rpg-tile.rgb')
    for picture in sorted((rpg/'pictures').glob('*.rgb')):
        upload(picture,'rpg-picture-'+picture.name)
    preset=ROOT/'assets/private/lean-router.conf'
    stage_network(preset if preset.is_file() else None)
    upload(ROOT/'src/pid1/scripts/native-app/start.sh','start-app.sh')
    now=datetime.datetime.now(datetime.timezone.utc).strftime('%Y-%m-%d %H:%M:%S')
    console('date -u -s "'+now+'"')
    command='sh /tmp/start-app.sh'
    if args.prepare_only: print('Prepared in RAM. Stop the launcher with POST /launcher/stop, run '+command+', then POST /launcher/reload')
    else:
        request('POST','/launcher/stop',timeout=6)
        console(command)
        request('POST','/launcher/reload')
        for _ in range(50):
            services=json.loads(request('GET','/status')).get('services',{})
            if services.get('audio') == 'listening' and services.get('display') == 'presenting':
                print('PID1 direct audio is listening and display is presenting')
                break
            if 'failed' in services.values():
                raise RuntimeError(f'PID1 media service failed: {services}')
            time.sleep(0.1)
        else:
            raise RuntimeError(f'PID1 media did not become ready: {services}')
        console('if ! sh /run/tv-network/join.sh; then echo "Wi-Fi is not ready; local apps and wired debug remain available"; fi')

if __name__=='__main__': main()
