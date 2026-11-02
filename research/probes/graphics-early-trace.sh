#!/system/bin/sh
# One-shot Magisk post-fs-data capture, before vendor early-boot module loading.
BASE=/data/local/tmp/g06-early-trace
TRACE=/sys/kernel/tracing
INSTANCE=$TRACE/instances/g06_early
ARM=$BASE/armed
umask 077
[ -f "$ARM" ] || exit 0
mv "$ARM" "$BASE/consumed"
exec >> "$BASE/startup.log" 2>&1
echo "stage=post-fs-data uptime=$(cat /proc/uptime)"
id
cat /proc/self/attr/current
mkdir -p "$TRACE"
mount -t tracefs tracefs "$TRACE" 2>/dev/null || true
mkdir "$INSTANCE" || exit 1
echo 0 > "$INSTANCE/tracing_on"
echo nop > "$INSTANCE/current_tracer"
echo 4096 > "$INSTANCE/buffer_size_kb"
echo mono > "$INSTANCE/trace_clock"
cat /proc/modules > "$BASE/modules-at-arm.txt"
cat /proc/mounts > "$BASE/mounts-at-arm.txt"
cat "$TRACE/available_events" > "$BASE/available-events.txt"

add_probe() {
    name=$1
    library=$2
    offset=$3
    kind=$4
    [ -f "$library" ] || return 1
    if [ "$kind" = ioctl ]; then
        echo "p:g06early/$name $library:$offset fd=%r0 cmd=%r1 data=%r2 a0=+0(%r2):x32 a1=+4(%r2):x32 a2=+8(%r2):x32 a3=+12(%r2):x32 a4=+16(%r2):x32 a5=+20(%r2):x32 a6=+24(%r2):x32 a7=+28(%r2):x32" >> "$TRACE/uprobe_events" || return 1
    else
        echo "p:g06early/$name $library:$offset r0=%r0 r1=%r1 r2=%r2 r3=%r3 p0=+0(%r0):x32 p1=+4(%r0):x32 p2=+8(%r0):x32 p3=+12(%r0):x32" >> "$TRACE/uprobe_events" || return 1
    fi
    echo 1 > "$INSTANCE/events/g06early/$name/enable"
}

# File offsets preserve the Thumb bit in the Android library's ELF symbols.
if [ "$(sha256sum /vendor/lib/libmi3.so | cut -d ' ' -f 1)" = a10f65b026358c7acbbb5e8767c6b7a6901695f8c620b7e02abed6ea31b890a9 ]; then
    add_probe android_ioctl /vendor/lib/libmi3.so 0x4f90d ioctl
    add_probe android_disp_init /vendor/lib/libmi3.so 0x5d87d args
    add_probe android_osd_init /vendor/lib/libmi3.so 0x778e5 args
    add_probe android_layer_create /vendor/lib/libmi3.so 0x77b6d args
    add_probe android_layer_show /vendor/lib/libmi3.so 0x77d85 args
else
    echo 'Android libmi3 hash mismatch; its uprobes were not installed'
fi

for event in sys_enter_ioctl sys_exit_ioctl sys_enter_openat sys_exit_openat sys_enter_mmap2 sys_exit_mmap2 sys_enter_munmap sys_enter_execve sys_enter_init_module sys_exit_init_module sys_enter_finit_module sys_exit_finit_module; do
    path=$INSTANCE/events/syscalls/$event/enable
    [ ! -f "$path" ] || echo 1 > "$path"
done
echo 1 > "$INSTANCE/events/sched/sched_process_exec/enable"
echo 1 > "$INSTANCE/events/sched/sched_process_exit/enable"
echo 1 > "$INSTANCE/tracing_on"
echo G06_EARLY_TRACE_ARMED_BEFORE_VENDOR_EARLY_BOOT > "$INSTANCE/trace_marker"
echo "armed uptime=$(cat /proc/uptime)"

(
    # The native vendor library may be mounted later in startup. Probe the
    # original inode as soon as it appears, and log when registration occurred.
    count=0
    while [ "$count" -lt 100 ]; do
        library=/mnt/vendor/tvservice/glibc/libmi.so
        if [ -f "$library" ]; then
            if [ "$(sha256sum "$library" | cut -d ' ' -f 1)" = 08a850e63442245ed1534168465cffe9ff87686b696f292f9355fcd3f8096a58 ]; then
                add_probe native_ioctl "$library" 0x43d70 ioctl
                add_probe native_disp_init "$library" 0x5ac30 args
                add_probe native_osd_init "$library" 0x86f58 args
                add_probe native_layer_create "$library" 0x87430 args
                add_probe native_layer_show "$library" 0x877dc args
                echo "G06_NATIVE_LIBRARY_PROBES_READY" > "$INSTANCE/trace_marker"
            fi
            break
        fi
        count=$((count + 1))
        sleep 0.1
    done
) &

(
    (ulimit -f 131072; exec cat "$INSTANCE/trace_pipe" > "$BASE/events.txt") &
    reader=$!
    sleep 120
    echo 0 > "$INSTANCE/tracing_on"
    kill "$reader" 2>/dev/null || true
    echo 0 > "$INSTANCE/events/enable"
    for cpu in "$INSTANCE"/per_cpu/cpu*; do
        echo "$cpu"
        cat "$cpu/stats"
    done > "$BASE/buffer-stats.txt"
    cat "$INSTANCE/trace" > "$BASE/tail.txt"
    cat /proc/modules > "$BASE/modules-after.txt"
    dmesg > "$BASE/kernel.log"
    getprop > "$BASE/properties.txt"
    ps -A -T > "$BASE/processes.txt"
    cat "$TRACE/uprobe_events" > "$BASE/probe-definitions.txt"
    for probe in android_ioctl android_disp_init android_osd_init android_layer_create android_layer_show native_ioctl native_disp_init native_osd_init native_layer_create native_layer_show; do
        echo "-:g06early/$probe" >> "$TRACE/uprobe_events" 2>/dev/null || true
    done
    echo complete > "$BASE/complete"
) &
exit 0
