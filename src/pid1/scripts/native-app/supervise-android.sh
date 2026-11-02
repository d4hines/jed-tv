#!/system/bin/sh
set -eu
umask 022
base=/data/local/tmp/dande-g13
menu= audio= display= osd= proxy= replay=
cleanup() {
    for pid in $display $audio $osd $menu $proxy $replay; do kill -TERM "$pid" 2>/dev/null || true; done
    menu= audio= display= osd= proxy= replay=
}
trap cleanup EXIT
trap 'exit 0' INT TERM
"$base/adb-relay" 8899 dande-g13-proxy > "$base/logs/proxy-relay.log" 2>&1 &
proxy=$!
"$base/adb-relay" 8090 dande-g13-replay > "$base/logs/replay-relay.log" 2>&1 &
replay=$!
rm -f "$base/root/tmp/app.frames"
unshare -m /system/bin/sh "$base/start.sh" > "$base/logs/menu.log" 2>&1 &
menu=$!
# Audio initialization does not consume the frame file or the chroot's /dev.
# Let it wait for Android's audio services while the launcher builds its
# private namespace and first frame. Display and OSD still need that frame.
export TV_RUNTIME_DIR=$base/root/tmp
export TV_DISPLAY_SIZE=${TV_DISPLAY_SIZE:-1280x720}
export LD_LIBRARY_PATH=/system/lib:/apex/com.android.art/lib:/apex/com.android.i18n/lib:/apex/com.android.vndk.v34/lib
set -- --port 8877
if [ "${TV_AUDIO_INIT_POLICY:-0}" = 1 ]; then set -- "$@" --init-policy; fi
"$base/android-client" "$base/logs/audio.log" "$base/native-audio" "$@" &
audio=$!
ready=0
for attempt in $(seq 1 100); do
    kill -0 "$menu"
    if [ "$(stat -c %s "$base/root/tmp/app.frames" 2>/dev/null || echo 0)" -gt 4096 ]; then
        ready=1; break
    fi
    sleep .1
done
[ "$ready" = 1 ]
chroot "$base/root" /bin/tvosd > "$base/logs/osd.log" 2>&1 &
osd=$!
"$base/android-client" "$base/logs/display.log" "$base/native-display" --run "$base/root/tmp/app.frames" &
display=$!
wait "$menu"
