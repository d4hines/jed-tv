#!/usr/bin/env python3
"""Reviewed resolutions for the pinned three-way merge; see README.txt."""
import hashlib
import json
from pathlib import Path
import re
import shutil

WORK = Path(__file__).resolve().parents[2] / 'assets/build/g13-android16'
SOURCE = WORK / 'kernel-source'
PATTERN = re.compile(r'^<<<<<<< MT5867-vendor\n(.*?)^\|\|\|\|\|\|\| Android-5.15.148-base\n(.*?)^=======\n(.*?)^>>>>>>> Android-5.15.220\n', re.M | re.S)


def usb_enumeration(text):
    # The vendor's expanded new-scheme loop predates the helper now used by
    # common. Carry its validation fixes into that loop as well.
    text = text.replace('usb_rcvaddr0pipe()', 'usb_rcvctrlpipe(udev, 0)')
    text = text.replace('buf->bMaxPacketSize0 = 0;', 'buf->bDescriptorType = buf->bMaxPacketSize0 = 0;')
    text = text.replace('buf->bDescriptorType = buf->bDescriptorType = ', 'buf->bDescriptorType = ')
    text = text.replace('case 8: case 16: case 32: case 64: case 255:', 'case 8: case 16: case 32: case 64: case 9:')
    text = text.replace('if (r == 0)\n\t\t\t\t\t\tr = -EPROTO;', 'if (r >= 0)\n\t\t\t\t\t\tr = -EPROTO;')
    statement = '\t\t\tmaxp0 = buf->bMaxPacketSize0;'
    check = '''
\t\t\tif (maxp0 > 0 && !initial && maxp0 != udev->descriptor.bMaxPacketSize0) {
\t\t\t\tdev_err(&udev->dev, "device reset changed ep0 maxpacket size!\\n");
\t\t\t\tretval = -ENODEV;
\t\t\t\tgoto fail;
\t\t\t}
'''
    assert text.count(statement) == 1
    text = text.replace(statement, statement + check)
    statement = '\t\t\tretval = 0;\n\n\t\t\tdelay = udev->parent->hub_delay;'
    assert text.count(statement) == 1
    text = text.replace(statement, check + '\n' + statement)
    return text


def main():
    output = WORK / 'resolution-report.json'
    if output.exists():
        raise SystemExit('Resolutions already recorded; preserving the working tree')
    report = {}
    conflicts = json.loads((WORK / 'merge-report.json').read_text())['conflicts']
    backup = WORK / 'unresolved'
    backup.mkdir()
    for name in conflicts:
        dest = backup / name
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(SOURCE / name, dest)

    def write(name, text, reason):
        assert not PATTERN.search(text), name
        (SOURCE / name).write_text(text)
        report[name] = {'reason': reason, 'sha256': hashlib.sha256(text.encode()).hexdigest()}

    # These vendor changes are earlier versions of Android/stable backports,
    # with no MT5867-specific code. Use the coherent newer implementation.
    upstream = [
        'android/abi_gki_aarch64.stg',
        'drivers/hwtracing/coresight/coresight-etm4x-core.c',
        'drivers/hwtracing/coresight/coresight-etm4x.h',
        'fs/f2fs/compress.c', 'fs/f2fs/data.c',
        'include/linux/pgsize_migration.h', 'mm/pgsize_migration.c',
        'kernel/rcu/tree_nocb.h', 'net/core/dst.c', 'net/ipv6/route.c',
        'net/netfilter/nft_set_pipapo.c', 'net/netlink/af_netlink.c',
        'net/unix/af_unix.c', 'net/unix/garbage.c',
    ]
    for name in upstream:
        write(name, (WORK / 'source-new' / name).read_text(),
              'New Android common implementation supersedes earlier backport; ARM64 STG is reference metadata only')

    for name in ['android/abi_gki_aarch64_amlogic', 'android/abi_gki_aarch64_mtktv']:
        def union(match):
            assert not match[2].strip()
            return ''.join(sorted(set((match[1] + match[3]).splitlines(True))))
        write(name, PATTERN.sub(union, (SOURCE / name).read_text()),
              'Union of added symbol names; this is not a verified MT5867 binary ABI')

    name = 'fs/proc/task_mmu.c'
    text = (WORK / 'source-new' / name).read_text()
    old = '\tmm = proc_mem_open(inode, PTRACE_MODE_READ);'
    assert text.count(old) == 1
    text = text.replace(old, '#ifdef CONFIG_PLAT_MSTAR\n\t/* Preserve vendor restriction on physical address disclosure. */\n\tif (!capable(CAP_SYS_ADMIN))\n\t\treturn -EPERM;\n#endif\n' + old)
    write(name, text, 'Use new page-size migration API; retain MStar pagemap CAP_SYS_ADMIN restriction')

    name = 'include/linux/dma-buf.h'
    write(name, PATTERN.sub(lambda m: m[1] + m[3], (SOURCE / name).read_text()),
          'Retain vendor kmap/kunmap declarations and add upstream unlocked attachment API')

    name = 'kernel/exit.c'
    def exit_resolution(match):
        # Upstream moved perf_event_exit_task earlier; keep the Utopia callback here.
        return match[1].split('\n\t/*')[0] + '\n'
    write(name, PATTERN.sub(exit_resolution, (SOURCE / name).read_text()),
          'Keep Utopia exit callback; perf_event_exit_task runs once at its new upstream location')

    name = 'kernel/locking/semaphore.c'
    write(name, PATTERN.sub(lambda m: m[3] + m[1][m[1].index('#ifdef'):], (SOURCE / name).read_text()),
          'Use wake_q-aware upstream __up declaration and retain vendor interruptible timeout helper')

    name = 'mm/cma.c'
    write(name, PATTERN.sub(lambda m: m[1] + m[3], (SOURCE / name).read_text()),
          'Retain vendor CMA includes and timing hook; add upstream tracepoint exports')

    name = 'mm/memory.c'
    write(name, PATTERN.sub(lambda m: m[3], (SOURCE / name).read_text()),
          'Use new nr_data_pages calculation in updated page-size migration API')

    name = 'drivers/usb/core/hub.c'
    text = (SOURCE / name).read_text()
    blocks = list(PATTERN.finditer(text))
    assert len(blocks) == 3
    replacement = [blocks[0][3], blocks[1][1] + blocks[1][3], '''\t\t\tdev_dbg(&port_dev->dev, "do warm reset, port only\\n");
\t\t\terr = hub_port_reset(hub, port1, NULL,
\t\t\t\t\t     HUB_BH_RESET_TIME, true);
\t\t\tif (!udev && err == -ENOTCONN)
\t\t\t\tconnect_change = 0;
#if (MP_USB_MSTAR==1)
\t\t\telse if (err < 0 ||
\t\t\t\t (portstatus & USB_PORT_STAT_LINK_STATE) == USB_SS_PORT_LS_LOOPBACK)
\t\t\t\thub_port_disable(hub, port1, 1, 1);
#else
\t\t\telse if (err < 0)
''']
    it = iter(replacement)
    text = PATTERN.sub(lambda m: next(it), text)
    # Vendor had restored the older descriptor field assignment. The new core
    # reads maxp0 before fetching the full descriptor; initialize it on both paths.
    old = '\t\t\tudev->descriptor.bMaxPacketSize0 =\n\t\t\t\t\tbuf->bMaxPacketSize0;'
    assert text.count(old) == 1
    text = text.replace(old, '\t\t\tmaxp0 = buf->bMaxPacketSize0;')
    text = usb_enumeration(text)
    write(name, text, 'Keep MStar port-reset API/loopback handling and upstream disconnect/maxpacket changes')

    name = 'drivers/usb/core/message.c'
    text = (SOURCE / name).read_text()
    blocks = list(PATTERN.finditer(text))
    assert len(blocks) == 1
    vendor = blocks[0][1]
    vendor = vendor[vendor.index('#if (MP_USB_MSTAR==1)'):vendor.rindex('#else')]
    vendor = vendor.replace('if (!wait_for_completion_timeout(&ctx.done, timeout_step)) {', '''unsigned long step = min(timeout_step, expire - total_time);
\t\t\trc = killable ? wait_for_completion_killable_timeout(&ctx.done, step) :
\t\t\t\twait_for_completion_timeout(&ctx.done, step);
\t\t\tif (rc < 0) {
\t\t\t\tusb_kill_urb(urb);
\t\t\t\tretval = ctx.status == -ENOENT ? rc : ctx.status;
\t\t\t\tbreak;
\t\t\t}
\t\t\tif (!rc) {''').replace('total_time += timeout_step;', 'total_time += step;')
    common, wait = blocks[0][3].split('\tif (killable)', 1)
    text = PATTERN.sub(lambda m: common + vendor + '#else\n\tif (killable)' + wait, text)
    write(name, text, 'Keep MStar CSC/suspend polling; apply timeout cap and killable waits to both branches')

    name = 'lib/lzo/lzo1x_compress.c'
    text = (WORK / 'source-new' / name).read_text()
    vendor = (WORK / 'source-vendor' / name).read_text()
    extra = vendor[vendor.index('#ifdef CONFIG_MP_ZSM\nint lzo1x_1_compress_crc'):vendor.index('EXPORT_SYMBOL_GPL(lzo1x_1_compress);')]
    extra = extra.replace('#ifdef CONFIG_MP_ZSM', '#if defined(CONFIG_MP_ZSM) && defined(LZO_UNSAFE)')
    extra = extra.replace('\t\tuintptr_t ll_end = (uintptr_t) ip + ll;', '\t\tuintptr_t ll_end = (uintptr_t) ip + ll;\n\t\tunsigned char *chunk_out = op;\n\t\tint err;')
    old = '\t\tt = lzo1x_1_do_compress(ip, ll, op, out_len, t, wrkmem, &state_offset, 0, \t &tmp_hash);'
    assert old in extra
    extra = extra.replace(old, '''\t\terr = lzo1x_1_do_compress(ip, ll, &op, NULL, &t, wrkmem, &state_offset, 0);
\t\tif (err != LZO_E_OK)
\t\t\treturn err;
\t\t*out_len = op - chunk_out;''').replace('\t\top += *out_len;\n', '')
    # The new compressor advances op itself; the vendor checksum samples the
    # first byte of the chunk, not the next unwritten byte. Its internal
    # t_total was always zero, so tmp_hash remains zero as in the vendor code.
    extra = extra.replace('unsigned char *tmp_op = op;', 'unsigned char *tmp_op = chunk_out;')
    text = text.replace('EXPORT_SYMBOL_GPL(LZO_SAFE(lzo1x_1_compress));', extra + '\nEXPORT_SYMBOL_GPL(LZO_SAFE(lzo1x_1_compress));')
    write(name, text, 'Use updated standard/safe compressors; adapt optional MStar CRC wrapper to new internal API (disabled in captured config)')

    assert set(report) == set(conflicts), (set(conflicts) - set(report))
    output.write_text(json.dumps(report, indent=2) + '\n')
    print(f'Resolved {len(report)} files; pre-resolution files saved in {backup}')


if __name__ == '__main__':
    main()
