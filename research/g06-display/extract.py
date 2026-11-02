#!/usr/bin/env python3
"""Read-only debugfs extraction from exact V109 inputs; output stays in assets."""
import pathlib,subprocess,json,hashlib
S=pathlib.Path('/Users/d4hines/repos/dande-tv/main/assets/research/firmware-implications')
O=pathlib.Path('assets/research/g06-display-offline'); D='/nix/store/gly23cvy7pbx4ngy3ac2x2n51sxg57i9-e2fsprogs-1.47.4-bin/bin/debugfs'
B=pathlib.Path('/Users/d4hines/repos/dande-tv/main/assets/backups/v109-stock-20261031')
req=[]
for n in ['iniparser','utpa2k','kdrv_platform','kdrv_xc','kdrv_alg','libdmdbraslia','hdcp1x','hdcp2x','xcker','mwgifker','misck','mik','mstar_fbdev_mi']:
 req.append((S/'super/vendor.img','/lib/modules/'+n+'.ko','vendor/lib/modules/'+n+'.ko'))
for n in ['modules.dep','modules.load','modules.softdep']:req.append((S/'super/vendor.img','/lib/modules/'+n,'vendor/lib/modules/'+n))
for image,src,dst in req:
 p=O/dst;p.parent.mkdir(parents=True,exist_ok=True)
 r=subprocess.run([D,'-R',f'dump {src} {p}',str(image)],capture_output=True,text=True)
 if not p.exists():raise RuntimeError(r.stderr)
# Exact configuration trees; no writes to source images.
for img,src,dst in [('tvconfig.img','/config','tvconfig-config'),('tclconfig.img','/hardware','tclconfig-hardware')]:
 p=O/dst;p.mkdir(exist_ok=True)
 r=subprocess.run([D,'-R',f'rdump {src} {p}',str(B/img)],capture_output=True,text=True)
 (O/(img+'.debugfs.txt')).write_text(r.stdout+r.stderr)
manifest=[]
for p in sorted(O.rglob('*')):
 if p.is_file() and (p.suffix in ['.ko','.ini','.h'] or p.name.startswith('modules.')):
  manifest.append({'path':str(p),'size':p.stat().st_size,'sha256':hashlib.sha256(p.read_bytes()).hexdigest()})
(O/'extracted-manifest.json').write_text(json.dumps(manifest,indent=2)+'\n')
print('Extracted',len(manifest),'files')
