#!/bin/sh
set -eu
grep -q 'androidboot.serialno=4C0490290FA05A0C0' /proc/cmdline
[ "$(id -u)" = 0 ]
if pidof midaemon >/dev/null; then echo 'midaemon is already running'; exit 1; fi
if grep -q '^mik ' /proc/modules; then echo 'Use a fresh native boot: mik is already initialized'; exit 1; fi
/tmp/mount-stock-runtime.sh
for name in mmcblk0p7 mmcblk0p19 mmcblk0p27; do
    [ "$(blockdev --getro /dev/$name)" = 1 ]
done
mkdir -p /vendor/tvconfig /mnt/vendor/tvservice /tclconfig /dev/block
mount -t ext4 -o ro,noload /dev/mmcblk0p7 /vendor/tvconfig
mount -t ext4 -o ro,noload /dev/mmcblk0p19 /mnt/vendor/tvservice
mount -t ext4 -o ro,noload /dev/mmcblk0p27 /tclconfig
ln -s /mnt/vendor/tvservice/glibc /lib
ln -s /vendor/tvconfig/config /config
ln -s /tclconfig /vendor/tclconfig
mkdir -p /var/run /var/tmp /dev/shm /mnt/vendor/tmp /data/vendor
mkdir -p /data/tee /sys/fs/selinux
mount -t selinuxfs selinuxfs /sys/fs/selinux
[ "$(cat /sys/fs/selinux/enforce)" = 0 ]
dd if=/vendor/etc/selinux/precompiled_sepolicy of=/sys/fs/selinux/load bs=1048576
tar -xzf /tmp/android-properties.tar.gz -C /dev
/tmp/tv-binder
export PATH=/bin:/sbin:/system/bin:/vendor/bin
export ANDROID_ROOT=/system
export ANDROID_DATA=/data
export ANDROID_STORAGE=/storage
export ANDROID_TZDATA_ROOT=/apex/com.android.tzdata
export ANDROID_BOOTLOGO=1
start_child() {
    name=$1
    shift
    nohup "$@" > "/tmp/$name.log" 2>&1 &
    echo "$!" > "/tmp/$name.pid"
    echo "started $name pid=$!"
}
start_child logd /system/bin/logd
start_child servicemanager /system/bin/servicemanager
start_child vndservicemanager /vendor/bin/vndservicemanager /dev/vndbinder
start_child hwservicemanager /system/bin/hwservicemanager
for node in /dev/mmcblk0*; do ln -s "$node" "/dev/block/${node##*/}"; done
mkdir -p /dev/block/by-name /dev/block/platform/mstar_mci.0/by-name
for entry in /sys/class/block/mmcblk0p*/uevent; do
    name=$(sed -n 's/^PARTNAME=//p' "$entry")
    node=$(sed -n 's/^DEVNAME=//p' "$entry")
    [ -n "$name" ] || continue
    [ "$(blockdev --getro /dev/$node)" = 1 ]
    ln -s "/dev/$node" "/dev/block/by-name/$name"
    ln -s "/dev/$node" "/dev/block/platform/mstar_mci.0/by-name/$name"
done
for name in iniparser kdrv_platform kdrv_xc kdrv_alg libdmdbraslia utpa2k hdcp1x hdcp2x xcker mwgifker misck mik mstar_fbdev_mi dtv_driver devnode mali_kbase; do
    if ! grep -q "^$name " /proc/modules; then
        echo "loading $name"
        insmod "/vendor/lib/modules/$name.ko"
    fi
done
for node in /dev/mik/*; do ln -s "$node" "/dev/mik!${node##*/}"; done
start_child tee-supplicant /vendor/bin/tee-supplicant
start_child midaemon env LD_LIBRARY_PATH=/mnt/vendor/tvservice/glibc:/vendor/tvconfig/config:/vendor/lib LINUX_TMP_PATH=/mnt/vendor/tmp /mnt/vendor/tvservice/bin/midaemon
sleep 2
start_child allocator /vendor/bin/hw/android.hardware.graphics.allocator@4.0-service
start_child hwcomposer /vendor/bin/hw/android.hardware.graphics.composer@2.4-service
sleep 2
start_child surfaceflinger env LD_LIBRARY_PATH=/system/lib:/apex/com.android.os.statsd/lib:/vendor/lib:/apex/com.android.vndk.v30/lib /system/bin/surfaceflinger
echo 'Stock display processes started as children; logs are in /tmp/*.log'
