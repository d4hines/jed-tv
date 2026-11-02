#!/usr/bin/env python3
"""Generate redistributable synthetic fMP4 demo media; no private recordings."""
import json,pathlib,struct,subprocess,sys
out=pathlib.Path(sys.argv[1]);fixtures=out/'fixtures'
def encode(extra,path):
 subprocess.run(['ffmpeg','-v','error','-y',*extra,'-t','16','-movflags','dash+global_sidx+frag_keyframe+empty_moov+default_base_moof','-frag_duration','1000000',str(path)],check=True)
encode(['-f','lavfi','-i','testsrc2=size=640x360:rate=24','-an','-c:v','libx264','-preset','fast','-profile:v','high','-pix_fmt','yuv420p','-g','24','-bf','2'],fixtures/'136.mp4')
encode(['-f','lavfi','-i','sine=frequency=440:sample_rate=48000','-vn','-c:a','aac','-b:a','96k'],fixtures/'140.m4a')
def format(itag,name,mime,**kwargs):
 p=fixtures/name;b=p.read_bytes();at=0
 while at+8<=len(b):
  size,kind=struct.unpack_from('>I4s',b,at)
  if size<8 or at+size>len(b):raise ValueError('invalid generated MP4')
  if kind==b'sidx':break
  at+=size
 else:raise ValueError('global sidx missing')
 return dict(itag=itag,mimeType=mime,contentLength=str(len(b)),bitrate=len(b)*8//16,initRange={'start':'0','end':str(at-1)},indexRange={'start':str(at),'end':str(at+size-1)},**kwargs)
player={'playabilityStatus':{'status':'OK'},'videoDetails':{'title':'Generated demo video','lengthSeconds':'16'},'streamingData':{'adaptiveFormats':[format(136,'136.mp4','video/mp4; codecs="avc1.64001e"',width=640,height=360,fps=24),format(140,'140.m4a','audio/mp4; codecs="mp4a.40.2"')]}}
(out/'player.json').write_text(json.dumps(player,indent=2)+'\n')

(out/'tiles').mkdir()
subprocess.run(['ffmpeg','-v','error','-i',str(fixtures/'136.mp4'),'-frames:v','1','-vf','scale=288:162','-pix_fmt','rgb24','-f','rawvideo',str(out/'tiles/demo-video.rgb')],check=True)
