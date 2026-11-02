#!/bin/sh
set -eu
grep -q 'androidboot.serialno=4C0490290FA05A0C0' /proc/cmdline
[ "$(id -u)" = 0 ]
[ -e /tmp/standalone-app-state.so ]
mkdir -p /dev/socket/audioserver /data/misc/audioserver /data/vendor/audio
start() {
    name=$1; shift
    nohup "$@" >"/tmp/$name.log" 2>&1 &
    echo $! >"/tmp/$name.pid"
}
if ! /system/bin/service check media.metrics | grep -q ': found'; then
    start mediametrics env LD_LIBRARY_PATH=/system/lib:/apex/com.android.os.statsd/lib:/vendor/lib:/apex/com.android.vndk.v30/lib /system/bin/mediametrics
fi
if ! /system/bin/service check suspend_control | grep -q ': found'; then
    start suspend-service /system/bin/hw/android.system.suspend@1.0-service
fi
if ! pidof mstar.hardware.audio.service >/dev/null; then
    start audio-hal env LD_LIBRARY_PATH=/vendor/lib:/apex/com.android.vndk.v30/lib:/system/lib:/apex/com.android.i18n/lib:/apex/com.android.runtime/lib /vendor/bin/hw/mstar.hardware.audio.service
fi
if ! pidof audioserver >/dev/null; then
    start audioserver env LD_PRELOAD=/tmp/standalone-app-state.so LD_LIBRARY_PATH=/system/lib:/apex/com.android.os.statsd/lib:/apex/com.android.i18n/lib:/vendor/lib:/apex/com.android.vndk.v30/lib /system/bin/audioserver
fi
for i in $(seq 1 30); do
    if /system/bin/service check media.audio_policy | grep -q ': found'; then
        echo 'Native AudioFlinger and audio policy ready'
        exit 0
    fi
    sleep 1
done
echo 'Audio policy did not register' >&2
exit 1
