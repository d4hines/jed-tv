#!/usr/bin/env python3
"""Build the local G06 QEMU machine without modifying any guest kernel."""
import hashlib,os,pathlib,shutil,subprocess,tarfile,tempfile,urllib.request
ROOT=pathlib.Path(__file__).resolve().parents[2]
OUT=ROOT/'assets/build/qemu-g06';VERSION='11.1.3'
SHA='5a8289c7940dc77d3873c81a6c29ba3afe1446fc9d36e77fa1658cdbecb84b22'
def main():
    OUT.mkdir(parents=True,exist_ok=True)
    archive=OUT/f'qemu-{VERSION}.tar.xz'
    if not archive.exists():
        urllib.request.urlretrieve(f'https://download.qemu.org/qemu-{VERSION}.tar.xz',archive)
    if hashlib.sha256(archive.read_bytes()).hexdigest()!=SHA:raise RuntimeError('QEMU source hash mismatch')
    src=OUT/f'qemu-{VERSION}'
    # Python's safe extractor correctly rejects EDK2's macOS-only absolute
    # X11 include link. It is not used by this Linux system emulator. Never
    # mistake an interrupted extraction for a complete source tree.
    marker=src/'.dande-source-complete'
    if not marker.exists() or marker.read_text().strip()!=SHA:
        def source_filter(member,destination):
            if member.name==f'qemu-{VERSION}/roms/edk2/EmulatorPkg/Unix/Host/X11IncludeHack':
                if not member.issym() or member.linkname!='/opt/X11/include':
                    raise RuntimeError('unexpected EDK2 link')
                return None
            return tarfile.data_filter(member,destination)
        with tempfile.TemporaryDirectory(prefix='qemu-unpack-',dir=OUT) as temp:
            with tarfile.open(archive) as tf:tf.extractall(temp,filter=source_filter)
            fresh=pathlib.Path(temp)/f'qemu-{VERSION}'
            (fresh/marker.name).write_text(SHA+'\n')
            if src.exists():
                previous=OUT/f'qemu-{VERSION}.previous'
                if previous.exists():raise RuntimeError('inspect previous source extraction first')
                src.rename(previous)
            fresh.rename(src)
    shutil.copyfile(pathlib.Path(__file__).with_name('g06.c'),src/'hw/arm/g06.c')
    p=src/'hw/arm/meson.build';s=p.read_text()
    if "files('virt.c', 'g06.c')" not in s:
        if s.count("files('virt.c')")!=1:raise RuntimeError('unexpected QEMU source layout')
        p.write_text(s.replace("files('virt.c')","files('virt.c', 'g06.c')",1))
    build=OUT/'build';build.mkdir(exist_ok=True)
    env=os.environ.copy()
    args=[str(src/'configure'),'--target-list=aarch64-softmmu','--disable-docs','--disable-werror',
          '--disable-gtk','--disable-sdl','--disable-cocoa','--disable-tools','--disable-guest-agent',
          '--disable-capstone','--disable-pvg','--enable-slirp','--enable-fdt=system']
    if os.uname().sysname=='Darwin':
        clt='/Library/Developer/CommandLineTools'
        env.update(SDKROOT=clt+'/SDKs/MacOSX.sdk',CC=clt+'/usr/bin/clang',
                   CXX=clt+'/usr/bin/clang++',OBJC=clt+'/usr/bin/clang',PATH=clt+'/usr/bin:'+env['PATH'])
        args+=['--python=/opt/homebrew/bin/python3','--extra-cflags=-I/opt/homebrew/opt/dtc/include',
               '--extra-ldflags=-L/opt/homebrew/opt/dtc/lib']
    if not (build/'build.ninja').exists():subprocess.run(args,cwd=build,env=env,check=True)
    subprocess.run(['ninja','-j6','qemu-system-aarch64'],cwd=build,env=env,check=True)
if __name__=='__main__':main()
