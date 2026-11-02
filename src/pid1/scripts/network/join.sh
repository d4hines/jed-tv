#!/bin/sh
set -eu
umask 077
root=/run/tv-network
test "$(id -u)" = 0
test -d "$root" && test ! -L "$root"
test "$(stat -c %u:%g:%a "$root")" = 0:0:700
grep -q 'androidboot.serialno=4C0490290FA05A0C0' /proc/cmdline
if test -f /settings/dande-tv/wifi.conf; then
    sed 's|^ctrl_interface=.*|ctrl_interface=/run/tv-network/wpa|' /settings/dande-tv/wifi.conf > "$root/wifi.conf"
elif test -f "$root/preset.conf"; then
    sed 's|^ctrl_interface=.*|ctrl_interface=/run/tv-network/wpa|' "$root/preset.conf" > "$root/wifi.conf"
else
    echo 'No saved Wi-Fi network' >&2
    exit 1
fi
chmod 600 "$root/wifi.conf"
sh "$root/prepare.sh"
ip route replace 192.168.77.1/32 dev eth0 src 192.168.77.2
attempt=0
until "$root/wpa_cli" -p "$root/wpa" -s "$root" -i wlan0 status | grep -q '^wpa_state=COMPLETED$'; do
    attempt=$((attempt + 1))
    if test "$attempt" -ge 30; then
        echo "Wi-Fi association did not complete; inspect $root/wpa.log" >&2
        exit 1
    fi
    sleep 1
done
if test -f "$root/udhcpc.pid"; then
    pid=$(cat "$root/udhcpc.pid")
    if test -f "/proc/$pid/cmdline" && tr '\000' ' ' < "/proc/$pid/cmdline" | grep -q 'udhcpc -i wlan0'; then
        ip route get 1.1.1.1
        exit 0
    fi
fi
udhcpc -i wlan0 -s "$root/lease.sh" -p "$root/udhcpc.pid" -n -t 5 -T 3 </dev/null >"$root/udhcpc.log" 2>&1
cat "$root/udhcpc.log"
ip route get 1.1.1.1
