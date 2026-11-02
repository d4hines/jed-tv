#!/bin/sh
set -eu
grep -q 'androidboot.serialno=4C0490290FA05A0C0' /proc/cmdline
[ "$(blockdev --getro /dev/mmcblk0p28)" = 1 ]
[ "$(cat /sys/class/block/mmcblk0p28/size)" = 7065600 ]
mkdir -p /stock-system /stock-vendor /product /system_ext
losetup -r -o 1048576 /dev/loop0 /dev/mmcblk0p28
losetup -r -o 734003200 /dev/loop1 /dev/mmcblk0p28
losetup -r -o 1084227584 /dev/loop2 /dev/mmcblk0p28
losetup -r -o 2887778304 /dev/loop3 /dev/mmcblk0p28
mount -t ext4 -o ro,noload /dev/loop0 /stock-system
mount -t ext4 -o ro,noload /dev/loop1 /stock-vendor
mount -t ext4 -o ro,noload /dev/loop2 /product
mount -t ext4 -o ro,noload /dev/loop3 /system_ext
mkdir -p /system /vendor/bin /vendor/lib /vendor/etc
mount -o bind /stock-system/system /system
mount -o bind /stock-vendor/bin /vendor/bin
mount -o bind /stock-vendor/lib /vendor/lib
mount -o bind /stock-vendor/etc /vendor/etc
mkdir -p /apex
for path in /stock-system/system/apex/*; do ln -s "$path" "/apex/${path##*/}"; done
ln -s /stock-system/system/apex/com.android.vndk.current /apex/com.android.vndk.v30
ln -s /stock-system/system/apex/com.android.art.release /apex/com.android.art
mkdir -p /dev/socket /linkerconfig
echo 'Original system and vendor runtime mounted read-only; userdata remains unmounted'
