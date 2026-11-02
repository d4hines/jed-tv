#!/system/bin/sh
# Replaces the zygote service command; no VM or system_server is started.
set -eu
umask 022
base=/data/local/tmp/dande-g13
config=/metadata/dande-tv

fallback() {
    dande_boot_mark "FAILED: ${1:-native startup}; selecting Android for the next boot"
    cp "$config/android.rc" "$config/entry.next"
    chmod 600 "$config/entry.next"
    chcon u:object_r:metadata_file:s0 "$config/entry.next"
    sync
    mv "$config/entry.next" "$config/entry.rc"
    sync
    setprop sys.powerctl reboot
    exit 1
}

test "$(getprop ro.serialno)" = 18ACC22D1C47
test "$(getprop ro.build.version.incremental)" = AU13
. "$config/boot-log.sh"
dande_boot_mark 'LAUNCHER: starting native services'
setprop sys.dande.launcher_ready 0
[ "$(getprop sys.dande.native_ready)" = 1 ] || fallback
# The bootloader publishes a bank only after signature and full-image checks.
# The mount is inherited by the supervisor and its private app namespaces.
if [ -f "$config/apps.sh" ]; then
    . "$config/apps.sh" || fallback 'authenticated app image mount failed'
fi
# Reapply in the post-fs service namespace. The early-init remount does not
# survive the stock mount setup as seen by services on this G13.
mount -o remount,bind,suid /system/bin/audioserver || fallback
for executable in tvinit ytplay slides doomtv tvosd; do
    [ -x "$base/root/bin/$executable" ] || fallback
done
for executable in android-client native-display native-audio adb-relay; do
    [ -x "$base/$executable" ] || fallback
done
mkdir -p "$base/logs"
exec >> "$base/logs/boot.log" 2>&1
echo "dande native startup: $(date -u), shell PID $$, kernel PID1 remains stock init"
# Readiness must come from this startup, never a previous successful session.
for log in audio display; do
    [ ! -f "$base/logs/$log.log" ] || mv "$base/logs/$log.log" "$base/logs/$log.previous.log"
done

setprop ctl.stop bootanim
setprop service.bootanim.exit 1
setprop sys.boot_completed 1
setprop sys.dande.native 1

# Wait for the exact hardware IR receiver rather than racing module loading.
dande_boot_mark 'INPUT: waiting for the IR receiver'
ready=0
for attempt in $(seq 1 300); do
    for name in /sys/class/input/event*/device/name; do
        if [ "$(cat "$name" 2>/dev/null)" = 'MStar Smart TV IR Receiver' ]; then
            ready=1; break
        fi
    done
    [ "$ready" = 1 ] && break
    sleep .1
done
[ "$ready" = 1 ] || fallback 'IR receiver timeout'
dande_boot_mark 'INPUT: IR receiver available; starting launcher, display and audio'

# G13 SurfaceFlinger retains a 1280x720 layer stack even without WindowManager;
# the hardware composer scales that to the physical 1920x1080 panel.
export TV_DISPLAY_SIZE=1280x720 TV_AUDIO_INIT_POLICY=1
# The G13 composer closes its boot-logo plane after three presentations.
# Submit a short, paced burst even when the launcher's first frame is static.
export TV_DISPLAY_STARTUP_FRAMES=8
/system/bin/sh "$base/supervise.sh" &
supervisor=$!
trap 'kill -TERM "$supervisor" 2>/dev/null || true' EXIT
trap 'exit 1' INT TERM
# These provide APK/framework functionality, not our display/audio path.
# Overlap their serial control requests with launcher setup and vendor
# display/audio initialization instead of delaying the first frame.
for service in artd installd media mediaextractor media.swcodec mediametrics \
    android-hardware-media-c2-hal-1-1 statsd storaged mdnsd; do
    setprop ctl.stop "$service"
done
ready=0
display_ready=0
audio_ready=0
for attempt in $(seq 1 300); do
    kill -0 "$supervisor" 2>/dev/null || fallback 'launcher supervisor exited'
    if [ "$display_ready" = 0 ] && grep -q 'NATIVE_DISPLAY startup complete' "$base/logs/display.log" 2>/dev/null; then
        display_ready=1
        dande_boot_mark 'DISPLAY: startup frames submitted'
    fi
    if [ "$audio_ready" = 0 ] && grep -q 'RUST_AUDIO ready' "$base/logs/audio.log" 2>/dev/null; then
        audio_ready=1
        dande_boot_mark 'AUDIO: bridge listening'
    fi
    if [ "$display_ready" = 1 ] && [ "$audio_ready" = 1 ]; then
        ready=1; break
    fi
    sleep .1
done
[ "$ready" = 1 ] || fallback 'display/audio readiness timeout'
setprop sys.dande.launcher_ready 1
case "$(getprop ro.boot.leanboot 2>/dev/null || printf unknown)" in
    1) dande_bootloader=Lean ;;
    '') dande_bootloader=MBOOT ;;
    *) dande_bootloader=unknown ;;
esac
dande_boot_mark "READY: bootloader=$dande_bootloader; home frames submitted, audio listening, IR available"
echo 'dande native launcher submitted its startup frames; no Android framework'

# The launcher exits successfully for Power. Preserve the native selection
# and let stock init perform the board's orderly shutdown.
if wait "$supervisor"; then
    setprop sys.powerctl shutdown
else
    fallback
fi
