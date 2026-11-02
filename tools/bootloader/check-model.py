#!/usr/bin/env python3
"""Pure-state execution of the shared Lean core and effect-color type checks."""
import json
import os
from pathlib import Path
import struct
import subprocess

from build import ROOT, OUT
from os_profile import decode_fdt, encode_recipe
from qemu import boot_image, vendor_image


def main():
    work = OUT / 'pure-model'
    work.mkdir(parents=True, exist_ok=True)
    (work/'input.dts').write_text('/dts-v1/; / { compatible = "leanboot,model"; #address-cells = <1>; #size-cells = <1>; };\n')
    subprocess.run(['dtc','-q','-s','-I','dts','-O','dtb','-o',str(work/'input.dtb'),str(work/'input.dts')],check=True)
    dtb = (work/'input.dtb').read_bytes()
    vendor, generic, config = b'vendor-initramfs', b'generic-initramfs', b'source = "pure-model"\n'
    length = ((len(vendor)+len(generic)+len(config)+4)&~3)+20
    changes = [('/chosen/bootargs',b'console=ttyAMA0 leanboot.model=1\0'),
               ('/chosen/linux,initrd-start',struct.pack('>I',0x44000000)),
               ('/chosen/linux,initrd-end',struct.pack('>I',0x44000000+length))]
    recipe = encode_recipe(changes)
    blobs = {'boot':boot_image(kernel=bytes(range(64))), 'vendor':vendor_image(vendor,dtb),
             'init':boot_image(ramdisk=generic)}
    control = struct.pack('<16I',0x534f424c,1,0,64,len(blobs['boot']),len(blobs['vendor']),len(blobs['init']),
                          64,len(recipe),64+len(recipe),len(config),length,0,0,0,0)+recipe+config
    blobs['control'] = control
    for name, blob in blobs.items(): (work/(name+'.bin')).write_bytes(blob)
    env = dict(os.environ,LEAN_PATH=str(OUT/'lean'))
    subprocess.run(['lean','-DmaxHeartbeats=2000000','-DmaxRecDepth=4096','--run',
                    str(ROOT/'tools/bootloader/ModelCheck.lean'),str(work)],env=env,check=True)
    expected = decode_fdt(dtb)[0];expected.update(changes)
    actual,_,reservations = decode_fdt((work/'model.dtb').read_bytes())
    assert actual == expected and reservations == [(0x44000000,length)]
    padding = config + bytes(length-len(vendor)-len(generic)-20-len(config))
    assert (work/'model-initrd.bin').read_bytes() == vendor+generic+padding+struct.pack('<II',len(padding),sum(padding))+b'#BOOTCONFIG\n'
    # These must fail because of different types, rather than missing imports.
    bad = {
        'hardware_in_memory':'def bad : LeanBoot.Unsafe.MemoryM UInt32 := LeanBoot.Unsafe.readSctlr',
        'memory_in_pure':'def bad : Id UInt8 := LeanBoot.Unsafe.load8 0',
        'hardware_in_baseio':'def bad : BaseIO UInt32 := LeanBoot.Unsafe.readSctlr',
        'memory_in_hardware_without_lift':'def bad : LeanBoot.Unsafe.HardwareM UInt8 := LeanBoot.Unsafe.load8 0',
    }
    for name, source in bad.items():
        path=work/(name+'.lean');path.write_text('import Unsafe.Primitives\n'+source+'\n')
        result=subprocess.run(['lean',str(path)],env=env,capture_output=True,text=True)
        assert result.returncode != 0 and 'Type mismatch' in result.stdout, result.stdout+result.stderr
    good=work/'explicit-lift.lean'
    good.write_text('import Unsafe.Primitives\ndef good : LeanBoot.Unsafe.HardwareM UInt8 := LeanBoot.Unsafe.liftMemory (LeanBoot.Unsafe.load8 0)\n')
    subprocess.run(['lean',str(good)],env=env,check=True)
    report={'pure_state_replay':True,'reader_only_inspection':True,'callback_count':1,
            'rejects_before_writes':5,'effect_color_rejections':len(bad),'explicit_lift':True,
            'fdt_properties':len(actual),'initrd_bytes':length,'hardware_access':False}
    (work/'check.json').write_text(json.dumps(report,indent=2)+'\n')
    print('PASS: shared pure-memory interpreter vs independent byte oracle; four effect-color compile failures and explicit lift')


if __name__ == '__main__': main()
