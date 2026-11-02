#!/system/bin/sh
# Linux must not reuse the vendor OTA workspace occupied by Lean app banks.
# This kernel's partition read-only flag does not affect pre-Linux FCIE I/O.
set -eu
test "$(getprop ro.serialno)" = 18ACC22D1C47
test "$(getprop ro.build.version.incremental)" = AU13
device=$(readlink -f /dev/block/by-name/upgrade)
block=${device##*/}
test "$(cat /sys/class/block/"$block"/start)" = 3180544
test "$(cat /sys/class/block/"$block"/size)" = 102400
setprop ctl.stop update_engine
blockdev --setro "$device"
test "$(blockdev --getro "$device")" = 1
