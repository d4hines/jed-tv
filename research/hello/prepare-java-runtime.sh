#!/bin/sh
set -eu
for pair in 'com.android.permission.gms com.android.permission' 'com.android.tethering.inprocess com.android.tethering'; do
    set -- $pair
    [ -e /apex/$2 ] || ln -s /apex/$1 /apex/$2
done
cp /tmp/java-ld.config.txt /linkerconfig/ld.config.txt
awk '$1 == "export" {print "export " $2 "=" $3}' /stock-system/init.environ.rc >/tmp/android-env.sh
echo 'export BOOTCLASSPATH=$BOOTCLASSPATH:/system_ext/framework/tcl-framework.jar' >>/tmp/android-env.sh
mkdir -p /data/dalvik-cache/arm /data/resource-cache
