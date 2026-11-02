#!/system/bin/sh
# Optional native G13 Wi-Fi, separate from Ethernet and launcher readiness.
set -eu
test "$(getprop ro.serialno)" = 18ACC22D1C47
test "$(getprop ro.product.device)" = G13
test "$(getprop ro.build.version.incremental)" = AU13
config=/metadata/dande-tv/wifi
base=/data/local/tmp/dande-g13/wifi
test -f "$config/wpa.conf"
setprop sys.dande.wifi_ready 0
umask 077
mkdir -p "$base/run"
chown 0:1010 "$base"
chmod 750 "$base"
chown 1010:1010 "$base/run"
chmod 700 "$base/run"
# The matching vendor supplicant drops to Android's wifi UID itself.
sed "s|^ctrl_interface=.*|ctrl_interface=$base/run/control|" \
    "$config/wpa.conf" > "$base/run/wpa.conf"
chown 1010:1010 "$base/run/wpa.conf"
chmod 600 "$base/run/wpa.conf"
for attempt in $(seq 1 300); do
    [ ! -d /sys/class/net/wlan0 ] || break
    sleep .1
done
test -d /sys/class/net/wlan0
ip link set wlan0 up
supplicant= dhcp=
cleanup() {
    for pid in $dhcp $supplicant; do kill -TERM "$pid" 2>/dev/null || true; done
    setprop sys.dande.wifi_ready 0
}
trap cleanup EXIT
trap 'exit 0' INT TERM
/vendor/bin/hw/wpa_supplicant -Dnl80211 -i wlan0 \
    -c "$base/run/wpa.conf" > "$base/vendor-wpa.log" 2>&1 &
supplicant=$!
cli() {
    # The wifi UID must be able to reply to the client's socket. The enclosing
    # run directory is private to wifi/root, even with this socket's mode.
    (umask 000; "$base/wpa_cli" -p "$base/run/control" -s "$base/run" -i wlan0 "$@")
}
ready=0
for attempt in $(seq 1 100); do
    kill -0 "$supplicant"
    if [ -S "$base/run/control/wlan0" ] && cli ping | grep -q PONG; then
        ready=1; break
    fi
    sleep .1
done
test "$ready" = 1
cli select_network 0 | grep -q OK
ready=0
for attempt in $(seq 1 120); do
    kill -0 "$supplicant"
    if cli status | grep -q '^wpa_state=COMPLETED$'; then ready=1; break; fi
    sleep .5
done
test "$ready" = 1
"$base/busybox" udhcpc -f -B -i wlan0 -s "$base/lease.sh" \
    -p "$base/udhcpc.pid" -n -t 5 -T 3 > "$base/dhcp.log" 2>&1 &
dhcp=$!
wait "$dhcp"
