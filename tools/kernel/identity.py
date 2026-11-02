#!/usr/bin/env python3
"""Pin the exact dumped kernel and identify QEMU surrogates without relabeling them.
No args: extract verified stock inputs and report identities. fast|virt32|faithful:
additionally require that profile's kernel to be byte-identical to the dump.
"""
import hashlib,json,pathlib,struct,sys,zlib
ROOT=pathlib.Path(__file__).resolve().parents[2]
PIN={
 'boot_sha256':'d116dfa370d8e5dd5809f6f65266c86671b5e9bec9d62f95c0977bca0ff30085',
 'kernel_sha256':'01e36bda0554aed46ad3aa1d7fdf688174f46241117f62b4b95f02e25c47c11e',
 'packed_dtb_sha256':'b1b23ad40f3fe0b4bf02fedd57313329ffa3e8578421cbe7f40b121459bf4fca',
 'surrogates':{
  # the repo's kernel (demo/kernel/zImage.xz, tools/kernel configs: AF_UNIX for
  # the launcher's remote sockets, file locks for shared frames)
  'virt32':{'path':'assets/build/kernel/arm/zImage-repo','sha256':'996d7306ab2e13e61c37636903378c3ce37caaa9e7605e640693015f1995784d','version':'6.18.57'},
  'fast':{'path':'assets/build/kernel/arm64/Image','sha256':'94dbaae0492f0a31528dfece4f5f2f9eee08626914bcf98e6d4355eced88f98b','version':'6.18.57'}}}

def sha(data):return hashlib.sha256(data).hexdigest()

def verified(data,expected,label):
    if sha(data)!=expected:raise ValueError(f'{label} hash mismatch; refusing unreviewed kernel input')
    return data

def extract(boot):
    if boot[:8]!=b'ANDROID!' or len(boot)<1660:raise ValueError('not an Android v2 boot image')
    size=struct.unpack_from('<I',boot,8)[0];page=struct.unpack_from('<I',boot,36)[0]
    version=struct.unpack_from('<I',boot,40)[0]
    if version!=2 or page<2048 or page&(page-1) or page+size>len(boot):raise ValueError('invalid boot header or truncated kernel')
    rd=struct.unpack_from('<I',boot,16)[0];second=struct.unpack_from('<I',boot,24)[0]
    align=lambda n:(n+page-1)//page*page
    offset=page+align(size)+align(rd)+align(second)
    dt_size=struct.unpack_from('<I',boot,1648)[0]
    if offset+dt_size>len(boot):raise ValueError('truncated packed device tree')
    return boot[page:page+size],boot[offset:offset+dt_size]

def prepare(root=ROOT):
    root=pathlib.Path(root)
    boot=verified((root/'assets/backups/v109-stock-20261031/boot.img').read_bytes(),PIN['boot_sha256'],'stock boot')
    kernel,packed=extract(boot)
    verified(kernel,PIN['kernel_sha256'],'stock kernel');verified(packed,PIN['packed_dtb_sha256'],'stock DTB')
    marker=kernel.find(b'IKCFG_ST')
    if marker<0:raise ValueError('stock kernel has no embedded config')
    config=zlib.decompress(kernel[marker+8:],31)
    captured=root/'assets/research/g06-capabilities-20261031/kernel-config'
    if config!=captured.read_bytes():raise ValueError('dumped config differs from captured /proc/config.gz')
    if packed[:4]!=bytes.fromhex('d7b7ab1e'):raise ValueError('unexpected Android DT table')
    count,entries=struct.unpack_from('>II',packed,16)
    if count!=1:raise ValueError('expected exactly one G06 device-tree entry')
    size,offset=struct.unpack_from('>II',packed,entries)
    board=packed[offset:offset+size]
    if len(board)!=size or board[:4]!=bytes.fromhex('d00dfeed'):raise ValueError('invalid board FDT')
    out=root/'assets/build/kernel/g06';out.mkdir(parents=True,exist_ok=True)
    for name,data in [('Image',kernel),('config',config),('dtb.img',packed),('board.dtb',board)]:
        p=out/name
        if not p.exists() or p.read_bytes()!=data:p.write_bytes(data)
    report={'target':'original G06/V109; not G13','stock_version':'4.19.116+',
        'stock_boot_sha256':PIN['boot_sha256'],'stock_kernel_sha256':sha(kernel),
        'stock_kernel_bytes':len(kernel),'stock_config_sha256':sha(config),'stock_board_dtb_sha256':sha(board),
        'embedded_config_matches_captured_proc':True,'profiles':{}}
    for name,pin in PIN['surrogates'].items():
        p=root/pin['path']
        if not p.exists():continue
        data=p.read_bytes();verified(data,pin['sha256'],name+' surrogate')
        report['profiles'][name]={'path':pin['path'],'version':pin['version'],'sha256':sha(data),
                                  'is_exact_dump':data==kernel,'bytes':len(data)}
    report['profiles']['faithful']={'path':'assets/build/kernel/g06/Image','version':'4.19.116+',
        'sha256':sha(kernel),'is_exact_dump':True,'bytes':len(kernel)}
    (out/'identity.json').write_text(json.dumps(report,indent=2)+'\n')
    return report

if __name__=='__main__':
    report=prepare();print(json.dumps(report,indent=2))
    if len(sys.argv)>1:
        profile=sys.argv[1]
        if profile not in report['profiles']:raise SystemExit('unknown/missing kernel profile')
        if not report['profiles'][profile]['is_exact_dump']:
            raise SystemExit(f'{profile}: NOT the dumped G06 kernel; exact-kernel QEMU boot is not established')
