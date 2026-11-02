#!/vendor/bin/sh
set -eu
[ "$(id -u)" = 0 ]
[ "$(cat /sys/devices/system/cpu/online)" = 0-3 ]
[ "$(cat /sys/kernel/kexec_loaded)" = 0 ]
restore() {
    for cpu in 1 2 3; do
        echo 1 > "/sys/devices/system/cpu/cpu$cpu/online"
    done
    printf 'restored_online='
    cat /sys/devices/system/cpu/online
}
trap restore EXIT
taskset -p 1 $$
for cpu in 3 2 1; do
    echo 0 > "/sys/devices/system/cpu/cpu$cpu/online"
    printf 'after_cpu%s_offline=' "$cpu"
    cat /sys/devices/system/cpu/online
done
[ "$(cat /sys/devices/system/cpu/online)" = 0 ]
echo PASS_SINGLE_CORE
