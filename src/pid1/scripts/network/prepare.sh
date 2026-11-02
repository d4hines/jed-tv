#!/bin/sh
set -eu
umask 077
root=/run/tv-network
test "$(id -u)" = 0
test -d "$root" && test ! -L "$root"
test "$(stat -c %u:%g:%a "$root")" = 0:0:700
grep -q 'androidboot.serialno=4C0490290FA05A0C0' /proc/cmdline
for module in ehci-hcd phy-ms-usb firmware_class cfg80211; do
    name=$(printf '%s' "$module" | tr - _)
    if ! grep -q "^$name " /proc/modules; then insmod "/vendor/lib/modules/$module.ko"; fi
done
if ! test -d /sys/class/net/wlan0; then
    insmod /vendor/lib/modules/8733bu.ko ifname=wlan0 if2name=p2p0
fi
attempt=0
while ! test -d /sys/class/net/wlan0; do
    attempt=$((attempt + 1))
    test "$attempt" -lt 50 || { echo 'Wi-Fi adapter did not appear' >&2; exit 1; }
    sleep 0.1
done
ip link set wlan0 up
mkdir -p "$root/wpa"
chmod 700 "$root/wpa"
if "$root/wpa_cli" -p "$root/wpa" -s "$root" -i wlan0 ping 2>/dev/null | grep -q PONG; then exit 0; fi
if ! test -f "$root/wifi.conf"; then
    if test -f /settings/dande-tv/wifi.conf; then
        sed 's|^ctrl_interface=.*|ctrl_interface=/run/tv-network/wpa|' /settings/dande-tv/wifi.conf > "$root/wifi.conf"
    else printf 'ctrl_interface=/run/tv-network/wpa\n' > "$root/wifi.conf"; fi
fi
chmod 600 "$root/wifi.conf"
"$root/wpa_supplicant" -B -Dwext -i wlan0 -c "$root/wifi.conf" -P "$root/wpa.pid" -f "$root/wpa.log"
attempt=0
until "$root/wpa_cli" -p "$root/wpa" -s "$root" -i wlan0 ping 2>/dev/null | grep -q PONG; do
    attempt=$((attempt + 1))
    test "$attempt" -lt 50 || { echo 'Wi-Fi control socket did not become ready' >&2; exit 1; }
    sleep 0.1
done
