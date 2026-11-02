#!/system/bin/sh
# Invoked only by the explicit native startup configuration, before class core.
set -eu
umask 022
config=/metadata/dande-tv
runtime=/dev/dande-native

restore_next_boot() {
    cp "$config/android.rc" "$config/entry.next"
    chmod 600 "$config/entry.next"
    chcon u:object_r:metadata_file:s0 "$config/entry.next"
    sync
    mv "$config/entry.next" "$config/entry.rc"
    sync
    cmp -s "$config/entry.rc" "$config/android.rc"
}

test "$(getprop ro.serialno)" = 18ACC22D1C47
test "$(getprop ro.build.version.incremental)" = AU13
test "$(stat -c '%u:%g:%a' "$config")" = 0:0:700
test "$(stat -c '%u:%g:%a' "$config/mode")" = 0:0:600
. "$config/boot-log.sh"
dande_boot_mark 'EARLY: preparing native startup'
case "$(cat "$config/mode")" in
    once) restore_next_boot ;;
    persistent) ;;
    *) restore_next_boot; exit 1 ;;
esac

# No metrics daemon in native mode. Disable its client lookup before any
# audio process caches it, avoiding the first stream's ten-second timeout.
setprop media.metrics.enabled 0
mkdir -p "$runtime"
chmod 755 "$runtime"
chcon u:object_r:system_file:s0 "$runtime"
cp "$config/standalone-app-state.so" "$runtime/standalone-app-state.so"
chmod 644 "$runtime/standalone-app-state.so"
chcon u:object_r:system_lib_file:s0 "$runtime/standalone-app-state.so"
# A DT_NEEDED dependency also works for a secure init -> audioserver exec.
# The executable and mount exist only until reboot; /system stays unchanged.
cp "$config/audioserver-standalone" "$runtime/audioserver-standalone"
chmod 755 "$runtime/audioserver-standalone"
chcon u:object_r:audioserver_exec:s0 "$runtime/audioserver-standalone"
mount -o bind "$runtime/audioserver-standalone" /system/bin/audioserver
mount -o remount,bind,suid /system/bin/audioserver
echo dande_g13_native_boot > /sys/power/wake_lock
setprop sys.dande.native_ready 1
dande_boot_mark 'EARLY: audio shim and wake lock ready'
# Vendor driver chatter later overwrites the small kernel ring buffer.
# Retain the earlier init/policy timings in RAM for startup diagnosis.
dmesg > "$runtime/early-dmesg.log" 2>/dev/null || :
