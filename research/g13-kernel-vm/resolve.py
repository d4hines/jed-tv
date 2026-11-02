#!/usr/bin/env python3
"""Resolve the pinned stable merge against the Android/MT5867 interfaces.

This is an experimental source port, not a claim of stock module ABI parity.
The report records every manual adjustment, including changes outside the
textual conflicts. Inputs are checked before anything is overwritten.
"""
import hashlib
import json
import re
import sys

sys.dont_write_bytecode = True
from prepare import WORK, PREVIOUS, merge

SOURCE = WORK / 'kernel-source'
PATTERN = re.compile(r'^<<<<<<< MT5867-Android\n(.*?)^\|\|\|\|\|\|\| stable-5.15.220\n(.*?)^=======\n(.*?)^>>>>>>> stable-5.15.224\n', re.M | re.S)


def replace(text, old, new, count=1):
    assert text.count(old) == count, (old, text.count(old), count)
    return text.replace(old, new)


def main():
    manifest = json.loads((WORK / 'stable-merge.json').read_text())
    output = WORK / 'stable-resolutions.json'
    if output.exists():
        raise SystemExit('Resolutions already recorded; preserving source')
    expected = {p: tuple(v) for p, v in manifest['source_hashes'].items()}
    assert merge.inventory(SOURCE) == expected, 'Merge input changed'
    result, texts = {}, {}

    def read(name):
        return texts.get(name, (SOURCE / name).read_text())

    def save(name, text, reason):
        assert not PATTERN.search(text), name
        texts[name] = text
        result[name] = {'reason': reason,
                        'sha256': hashlib.sha256(text.encode()).hexdigest()}

    def select(name, choices, reason):
        blocks = list(PATTERN.finditer(read(name)))
        if isinstance(choices, int):
            choices = [choices] * len(blocks)
        assert len(choices) == len(blocks), name
        parts = iter(m[c] if isinstance(c, int) else c(m)
                     for m, c in zip(blocks, choices))
        save(name, PATTERN.sub(lambda m: next(parts), read(name)), reason)

    for name, reason in {
        'drivers/hid/hid-core.c': 'Use extracted probe helper; it already retains Android IO-stop cleanup',
        'drivers/infiniband/hw/cxgb4/device.c': 'Cancel registration before teardown and avoid duplicate debugfs cleanup',
        'drivers/remoteproc/remoteproc_core.c': 'Use mandatory recovery workqueue and queue crash work under its lock',
        'drivers/usb/typec/ucsi/displayport.c': 'Reject invalid CAM index while retaining the no-mode sentinel',
        'fs/f2fs/sysfs.c': 'Use embedded GC thread state; Android urgent-mid case adjusted below',
        'net/ipv6/seg6_local.c': 'Preserve L3 slave metadata for both IPv4 and IPv6 decapsulation',
        'net/sunrpc/svc.c': 'Select a populated RPC worker pool',
    }.items():
        select(name, 3, reason)

    # Android moved these ARM64 errata down the file. Update the existing
    # declarations rather than adding duplicate options or capability entries.
    name = 'arch/arm64/Kconfig'
    blocks = list(PATTERN.finditer(read(name)))
    incoming = blocks[0][3]
    text = PATTERN.sub(lambda m: m[1], read(name))
    old = re.search(r'config ARM64_ERRATUM_2457168\n.*?(?=config ARM64_ERRATUM_)', text, re.S)[0]
    save(name, replace(text, old, incoming), 'Replace moved AMU options with shared broken-counter workaround')
    name = 'arch/arm64/kernel/cpu_errata.c'
    block = list(PATTERN.finditer(read(name)))[0][3]
    block = block[:block.index('#ifdef CONFIG_ARM64_ERRATUM_1742098')]
    text = PATTERN.sub(lambda m: m[1], read(name))
    old = re.search(r'#ifdef CONFIG_ARM64_ERRATUM_2457168\n\t\{.*?#endif\n', text, re.S)[0]
    save(name, replace(text, old, block), 'Replace moved AMU capability entry; keep Android 2658417 and existing AES erratum')
    select('arch/arm64/tools/cpucaps', [lambda m: m[1].replace('WORKAROUND_2457168\n', ''), lambda m: m[3] + m[1]],
           'Replace old AMU capability; retain Android erratum and reserved identifiers')

    select('drivers/mtd/ubi/io.c', [lambda m: m[1].replace('if (torture', 'if (*torture')],
           'Preserve vendor mark-bad versus torture distinction through new pointer API')
    name = 'drivers/mtd/ubi/io.c'
    text = read(name)
    text = replace(text, '\telse if (torture == 2)', '\telse if (*torture == 2)') if '\telse if (torture == 2)' in text else text
    # A completed vendor torture must clear the retry flag too.
    old = '\telse if (*torture == 2) {\n\t\tret = torture_peb(ubi, pnum);\n\t\tif (ret < 0)\n\t\t\treturn ret;\n'
    text = replace(text, old, old + '\t\t*torture = 0;\n')
    save(name, text, result[name]['reason'])

    select('drivers/net/wireless/marvell/libertas_tf/main.c',
           [lambda m: m[3].replace('timer_delete_sync', 'del_timer_sync')],
           'Cancel timer synchronously before freeing commands using this tree timer API')
    select('drivers/spi/spi.c', 1, 'Null-check fix targets a match-data helper absent from this Android tree')
    select('drivers/usb/typec/tcpm/tcpm.c',
           [lambda m: m[1].replace('tcpm_port_is_debug(port)', 'tcpm_port_is_debug_source(port)')],
           'Keep Android timing hook and distinguish debug source from sink')
    select('drivers/virtio/virtio_balloon.c',
           [lambda m: m[3].replace('\t__virtio_clear_bit(vdev, VIRTIO_F_ACCESS_PLATFORM);\n', '')],
           'Disable indirect page-reporting descriptors; retain Android ACCESS_PLATFORM policy')
    select('sound/usb/quirks.c', 1,
           'Fixed-rate JBL quirk mechanism is absent from this Android tree; defer the dependent Quantum650 entry')
    select('fs/erofs/zdata.c', [lambda m: m[1] + m[3], lambda m: m[1].replace('in_atomic() || irqs_disabled()', 'z_erofs_in_atomic()')],
           'Use RCU-aware atomic-context test while retaining per-CPU decompression workers')

    # Android already parses nonuniform zone descriptors and advances by their
    # actual ends. Retain that loop; use sectors for the capacity boundary.
    name = 'drivers/scsi/sd_zbc.c'
    select(name, [1, 1], 'Retain Android variable-zone traversal and validate against sector capacity')
    text = replace(read(name), 'sector_t capacity = logical_to_sectors(sdkp->device, sdkp->capacity);',
                   'sector_t capacity = logical_to_sectors(sdkp->device, sdkp->capacity);\n\tsector_t lba = sectors_to_logical(sdkp->device, sector);')
    text = replace(text, 'while (zone_idx < nr_zones && lba < sdkp->capacity)',
                   'while (zone_idx < nr_zones && logical_to_sectors(sdkp->device, lba) < capacity)')
    save(name, text, result[name]['reason'])

    # The new mac80211 and HID teardown fixes require shutdown semantics,
    # including prevention of rearming. A del_timer_sync alias is insufficient.
    # The stable timer implementation differs here only in this prerequisite,
    # the new CPU-migration debug fix, and the Android hooks restored below.
    name = 'kernel/time/timer.c'
    text = (WORK / 'stable-new' / name).read_text()
    text = replace(text, '#include <trace/events/timer.h>\n',
                   '#include <trace/events/timer.h>\n#undef CREATE_TRACE_POINTS\n'
                   '#include <trace/hooks/timer.h>\n\n'
                   'EXPORT_TRACEPOINT_SYMBOL_GPL(hrtimer_expire_entry);\n'
                   'EXPORT_TRACEPOINT_SYMBOL_GPL(hrtimer_expire_exit);\n')
    text = replace(text, '\texpires = (expires + LVL_GRAN(lvl)) >> LVL_SHIFT(lvl);',
                   '\ttrace_android_vh_timer_calc_index(lvl, &expires);\n'
                   '\texpires = (expires + LVL_GRAN(lvl)) >> LVL_SHIFT(lvl);')
    wrappers = '''/* Preserve the exported Android 5.15 entry points. */
int del_timer(struct timer_list *timer)
{
\treturn timer_delete(timer);
}
EXPORT_SYMBOL(del_timer);

int del_timer_sync(struct timer_list *timer)
{
\treturn timer_delete_sync(timer);
}
EXPORT_SYMBOL(del_timer_sync);

'''
    text = replace(text, 'static void call_timer_fn(', wrappers + 'static void call_timer_fn(')
    save(name, text, 'Backport complete stable timer shutdown/rearm semantics and migration fix; retain Android hooks and exported names')
    name = 'include/linux/timer.h'
    text = replace(read(name), 'extern int del_timer_sync(struct timer_list *timer);',
                   'extern int del_timer_sync(struct timer_list *timer);\n'
                   'extern int timer_delete_sync(struct timer_list *timer);\n'
                   'extern int timer_delete(struct timer_list *timer);\n'
                   'extern int timer_shutdown_sync(struct timer_list *timer);\n'
                   'extern int timer_shutdown(struct timer_list *timer);')
    save(name, text, 'Expose stable timer APIs while retaining Android timer layout and existing declarations')

    select('fs/f2fs/gc.c', [3, lambda m: '\t\tint ' + m[3].lstrip(), 1],
           'Embed GC lifetime in superblock; retain Android GC control interface')
    name = 'fs/f2fs/gc.c'
    text = replace(read(name), '\t\tgc_th->f2fs_gc_task = NULL;\n\t}', '\t\tgc_th->f2fs_gc_task = NULL;\n\t\treturn err;\n\t}')
    save(name, text, result[name]['reason'])
    select('fs/f2fs/gc.h', [lambda m: m[1].split('struct f2fs_gc_kthread')[0]],
           'Keep Android checkpoint constant; GC structure moved to f2fs.h')
    select('fs/f2fs/segment.c', 1, 'Keep Android GC control and threshold calculation; adapt embedded state below')
    for name in ('fs/f2fs/segment.c', 'fs/f2fs/sysfs.c', 'fs/f2fs/gc.c', 'fs/f2fs/super.c', 'fs/f2fs/debug.c'):
        text = read(name)
        text = text.replace('sbi->gc_thread &&\n\t\t\t\tsbi->gc_thread->f2fs_gc_task', 'sbi->gc_thread.f2fs_gc_task')
        text = text.replace('if (sbi->gc_thread)', 'if (sbi->gc_thread.f2fs_gc_task)')
        text = text.replace('!sbi->gc_thread)', '!sbi->gc_thread.f2fs_gc_task)')
        text = text.replace('sbi->gc_thread->', 'sbi->gc_thread.')
        if text != read(name):
            save(name, text, 'Convert remaining Android-only GC thread references to embedded state')

    # BPF owner is embedded in array_aux here, not dynamically allocated in
    # bpf_map. Carry the attach-type check to that representation.
    select('include/linux/bpf.h', 1, 'Retain Android BPF interfaces; add attach type to the actual array owner')
    name = 'include/linux/bpf.h'
    text = replace(read(name), '\t\tbool jited;\n\t} owner;', '\t\tbool jited;\n\t\tenum bpf_attach_type expected_attach_type;\n\t} owner;')
    save(name, text, result[name]['reason'])
    select('kernel/bpf/core.c', [lambda m: m[1] + '\t\tarray->aux->owner.expected_attach_type = fp->expected_attach_type;\n',
           lambda m: m[1].replace('array->aux->owner.jited == fp->jited;', 'array->aux->owner.jited == fp->jited &&\n\t\t      array->aux->owner.expected_attach_type == fp->expected_attach_type;')],
           'Check tail-call attach-type compatibility using Android array ownership')

    for name in ('include/linux/cgroup-defs.h', 'kernel/cgroup/cgroup.c'):
        select(name, 1, 'cgroup.kill and kill_seq are absent; retain existing fork interface')
    name = 'kernel/cgroup/cgroup.c'
    save(name, replace(read(name), '\tkargs->kill_seq = dst_cgrp->kill_seq;\n', ''), result[name]['reason'])
    select('include/linux/fsnotify_backend.h', [lambda m: m[3].replace('#define FSNOTIFY_CONN_FLAG_HAS_IREF\t0x02\n', '')],
           'Add connector-destruction flag without introducing absent inode-reference representation')
    select('include/net/inet_connection_sock.h', [lambda m: ''],
           'Prepare-for-destroy helper moves out of line; reqsk_timeout is absent here')
    select('mm/damon/vaddr-test.h', [lambda m: 'out:\n' + m[1]],
           'Use new failure exit with the existing target destructor')
    select('mm/mmap.c', [lambda m: m[3].replace('vma_policy(vma), vma->vm_userfaultfd_ctx);', 'vma_policy(vma), vma->vm_userfaultfd_ctx,\n\t\t\t\t    anon_vma_name(vma));')],
           'Prevent DONTUNMAP self-merge while retaining Android anonymous-VMA names')
    select('mm/mremap.c', [lambda m: m[3] + m[1]],
           'Reset unfaulted VMA offset within existing speculative-page-fault guard')
    select('mm/vmscan.c', 1, 'Retain Android MGLRU definitions; port RCU quiescence into actual reclaim loop below')
    name = 'mm/vmscan.c'
    text = read(name)
    start = text.index('static void shrink_node_memcgs(')
    end = text.index('\nstatic ', start + 1)
    part = text[start:end].replace('cond_resched();', 'cond_resched_tasks_rcu_qs();')
    save(name, text[:start] + part + text[end:], result[name]['reason'])
    select('net/bluetooth/hci_core.c', 1, 'Android fixed-size name formatting cannot fail or leak an allocated ID')
    select('net/core/flow_dissector.c', [lambda m: '\tu32 thoff;\n' + m[1]],
           'Retain initialized return value and add checked transport offset')
    select('net/ipv4/route.c', 1, 'Multipath exception loop is absent; existing selected-nexthop path retained')
    # The teardown fix is already present, but the merge matched its moved
    # functions twice. Replace only that region with the original equivalent.
    name = 'net/openvswitch/conntrack.c'
    old = (PREVIOUS / 'kernel-source' / name).read_text()
    text = read(name)
    begin = text.index('<<<<<<< MT5867-Android')
    prefix = text[:begin]
    old_begin = old.index('static void *ovs_ct_limit_exit_start(')
    save(name, prefix + old[old_begin:], 'Android already has split RCU teardown and error cleanup; retain it once')
    select('security/security.c', [lambda m: m[3].replace('call_int_hook(path_rename, ', 'call_int_hook(path_rename, 0, ')] * 2,
           'Pass rename flags through LSM using Android call_int_hook default argument')
    select('sound/core/pcm_native.c', [lambda m: m[3].replace('runtime->state', 'runtime->status->state')],
           'Protect mmap against buffer reallocation using the existing PCM state field')

    select('fs/notify/fanotify/fanotify_user.c', 1,
           'Keep older ignore-mask API; recompute connector mask unconditionally under mark mutex')
    name = 'fs/notify/fanotify/fanotify_user.c'
    text = read(name)
    text = replace(text, '\tif (added & ~fsnotify_conn_mask(fsn_mark->connector))\n\t\tfsnotify_recalc_mask(fsn_mark->connector);',
                   '\tfsnotify_recalc_mask(fsn_mark->connector);')
    text = replace(text, '\t__u32 added;\n', '')
    text = replace(text, '\tadded = fanotify_mark_add_to_mask(', '\tfanotify_mark_add_to_mask(')
    text = replace(text, '\t\tlist_for_each_entry(fsn_event, &group->notification_list, list)\n'
                   '\t\t\tsend_len += fanotify_event_len(info_mode,\n'
                   '\t\t\t\t\t\t       FANOTIFY_E(fsn_event));',
                   '\t\tlist_for_each_entry(fsn_event, &group->notification_list, list) {\n'
                   '\t\t\tsend_len += FAN_EVENT_METADATA_LEN;\n'
                   '\t\t\tif (info_mode)\n'
                   '\t\t\t\tsend_len += fanotify_event_info_len(info_mode,\n'
                   '\t\t\t\t\t\t\t    FANOTIFY_E(fsn_event));\n\t\t}')
    save(name, text, result[name]['reason'])
    select('fs/notify/inotify/inotify_user.c', 1,
           'Keep older mask API; merged update always recalculates connector mask')
    name = 'fs/notify/inotify/inotify_user.c'
    text = read(name).replace('\t__u32 old_mask, new_mask;\n', '').replace('\told_mask = fsn_mark->mask;\n', '').replace('\tnew_mask = fsn_mark->mask;\n', '')
    save(name, text, result[name]['reason'])

    # NFS server keeps the older COPY field names and synchronous flag. Port
    # publication/refcount and error-path fixes without importing unrelated APIs.
    def copy_fields(text):
        return text.replace('cs_stid', 'stid').replace('cs_count', 'sc_count').replace('cs_type', 'sc_type')

    select('fs/nfsd/nfs4proc.c', [1, 1,
           lambda m: m[1].replace('\t\tsince = READ_ONCE(dst->f_wb_err);\n', ''),
           lambda m: m[1] + m[3][m[3].index('\t\telse if'):],
           lambda m: m[3].replace('cstate, copy);', 'cstate, copy, &copy->ss_mnt);'),
           lambda m: copy_fields(m[3])],
           'Port COPY error and ownership fixes; absent ACL-to-attrs API needs no error-path change')
    name = 'fs/nfsd/nfs4proc.c'
    save(name, replace(read(name), 'nfsd_reset_write_verifier(copy->cp_nn);',
                       'nfsd_reset_boot_verifier(net_generic(copy->cp_clp->net,\n\t\t\t\t\t\t  nfsd_net_id));'),
         result[name]['reason'] + '; use the existing namespace boot-verifier API')
    select('fs/nfsd/nfs4state.c', [lambda m: copy_fields(m[3])] * 6 +
           [lambda m: m[3][:m[3].index('\ttrace_nfsd_deleg_return')]],
           'Publish initialized COPY state under lock, validate cancellation ownership and delegation filehandle')
    name = 'fs/nfsd/nfs4state.c'
    save(name, copy_fields(read(name)), result[name]['reason'])
    select('fs/nfsd/nfscache.c', 1, 'Older shrink loop has no max bound; off-by-one fix does not apply')
    select('fs/nfsd/trace.h', 1, 'The affected fh-verify tracepoints are absent in this tree')

    assert set(manifest['conflicts']) <= result.keys()
    for name, text in texts.items():
        (SOURCE / name).write_text(text)
    output.write_text(json.dumps(result, indent=2) + '\n')
    print('Resolved', len(manifest['conflicts']), 'conflicts;', len(result), 'reviewed files')


if __name__ == '__main__':
    main()
