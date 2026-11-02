#!/vendor/bin/sh
set -u
if [ "$(id -u)" != 0 ] || [ "$(cat /sys/kernel/kexec_loaded)" != 1 ]; then
    echo TV_KEXEC_REFUSED
    exit 1
fi
test_directory=${0%/*}
cleanup() {
    if [ -f "$test_directory/single-core" ]; then
        for cpu in 1 2 3; do
            echo 1 > "/sys/devices/system/cpu/cpu$cpu/online"
        done
    fi
    /vendor/bin/kexec -u
}
trap cleanup EXIT
if [ -f "$test_directory/single-core" ]; then
    echo TV_KEXEC_SINGLE_CORE
    taskset -p 1 $$ || exit 1
    for cpu in 3 2 1; do
        echo 0 > "/sys/devices/system/cpu/cpu$cpu/online" || exit 1
    done
    printf 'TV_KEXEC_ONLINE='
    cat /sys/devices/system/cpu/online
    [ "$(cat /sys/devices/system/cpu/online)" = 0 ] || exit 1
fi
echo TV_KEXEC_STOP_FRAMEWORK
/system/bin/stop zygote surfaceflinger audioserver
sync
echo TV_KEXEC_REMOUNT_DISKS_READONLY
echo u > /proc/sysrq-trigger
attempt=0
while [ "$attempt" -lt 100 ]; do
    writable=$(awk '$3 ~ /^(ext4|f2fs|vfat|exfat)$/ && $4 ~ /(^|,)rw(,|$)/ {print $2}' /proc/mounts) || break
    if [ -z "$writable" ]; then
        echo TV_KEXEC_DISKS_READONLY
        cat /proc/mounts
        echo TV_KEXEC_EXECUTE
        /vendor/bin/kexec -e
        printf 'TV_KEXEC_RETURNED rc=%s\n' "$?"
        break
    fi
    attempt=$((attempt + 1))
    sleep 0.2
done
echo TV_KEXEC_ABORT_REBOOT_ANDROID
cat /proc/mounts
/vendor/bin/kexec -u
setprop sys.powerctl reboot
