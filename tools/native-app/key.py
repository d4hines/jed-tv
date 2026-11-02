#!/usr/bin/env python3
"""Send normal evdev remote keys to the native TV app for repeatable checks."""
import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[2]/'tools/pid1'))
from graphics import console, upload
keys={'up':103,'down':108,'left':105,'right':106,'ok':28,'home':102,'back':158,
      'volume-up':115,'volume-down':114,'mute':113}
commands=[]
for name in sys.argv[1:]:
    code=keys[name]
    commands.extend([f'/system/bin/sendevent /dev/input/event2 1 {code} 1',
                     '/system/bin/sendevent /dev/input/event2 0 0 0','sleep 0.1',
                     f'/system/bin/sendevent /dev/input/event2 1 {code} 0',
                     '/system/bin/sendevent /dev/input/event2 0 0 0','sleep 0.2'])
script=Path(__file__).resolve().parents[2]/'assets/build/native-app-keys.sh'
script.write_text('set -e\n'+'\n'.join(commands)+'\n')
upload(script)
console('sh /tmp/native-app-keys.sh')
