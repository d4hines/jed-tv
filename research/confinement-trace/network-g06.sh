#!/bin/sh
set -eu
umask 077
test "$(id -u)" = 0
grep -q 'androidboot.serialno=4C0490290FA05A0C0' /proc/cmdline
root=/run/tv-network
test "$(stat -c %u:%g:%a "$root")" = 0:0:700
test -x /tmp/tee-trace
test ! -f /tmp/wpa-syscalls.jsonl
test ! -f /tmp/dhcp-syscalls.jsonl
for module in ehci-hcd phy-ms-usb firmware_class cfg80211; do
    name=$(printf '%s' "$module" | tr - _)
    if ! grep -q "^$name " /proc/modules; then insmod "/vendor/lib/modules/$module.ko"; fi
done
if ! test -d /sys/class/net/wlan0; then
    insmod /vendor/lib/modules/8733bu.ko ifname=wlan0 if2name=p2p0
fi
attempt=0
while ! test -d /sys/class/net/wlan0; do
    attempt=$((attempt + 1)); test "$attempt" -lt 50; sleep 0.1
done
ip link set wlan0 up
mkdir -p "$root/wpa"
chmod 700 "$root/wpa"
if test -f /settings/dande-tv/wifi.conf; then
    sed 's|^ctrl_interface=.*|ctrl_interface=/run/tv-network/wpa|' /settings/dande-tv/wifi.conf > "$root/wifi.conf"
else
    sed 's|^ctrl_interface=.*|ctrl_interface=/run/tv-network/wpa|' "$root/preset.conf" > "$root/wifi.conf"
fi
chmod 600 "$root/wifi.conf"
/tmp/tee-trace --log /tmp/wpa-syscalls.jsonl -- "$root/wpa_supplicant" -Dwext -i wlan0 -c "$root/wifi.conf" -P "$root/wpa.pid" -f "$root/wpa.log" </dev/null >/tmp/wpa-tracer.log 2>&1 &
wpa_trace=$!
attempt=0
until "$root/wpa_cli" -p "$root/wpa" -s "$root" -i wlan0 status 2>/dev/null | grep -q '^wpa_state=COMPLETED$'; do
    attempt=$((attempt + 1))
    if test "$attempt" -ge 40 || ! kill -0 "$wpa_trace" 2>/dev/null; then
        echo 'WPA did not associate; inspect private logs'
        exit 1
    fi
    sleep 1
done
echo 'WPA associated under tracing'
ip route replace 192.168.77.1/32 dev eth0 src 192.168.77.2
/tmp/tee-trace --log /tmp/dhcp-syscalls.jsonl -- /bin/udhcpc -f -i wlan0 -s "$root/lease.sh" -p "$root/udhcpc.pid" -n -t 5 -T 3 </dev/null >/tmp/dhcp-tracer.log 2>&1 &
dhcp_trace=$!
attempt=0
until ip -4 addr show wlan0 | grep -q 'inet '; do
    attempt=$((attempt + 1))
    if test "$attempt" -ge 30 || ! kill -0 "$dhcp_trace" 2>/dev/null; then
        echo 'DHCP did not bind; inspect private logs'
        exit 1
    fi
    sleep 1
done
echo 'DHCP bound; exercising renew and release'
dhcp_pid=$(cat "$root/udhcpc.pid")
kill -USR1 "$dhcp_pid"
sleep 3
kill -USR2 "$dhcp_pid"
sleep 2
kill -TERM "$dhcp_pid"
set +e
wait "$dhcp_trace"
dhcp_result=$?
set -e
"$root/wpa_cli" -p "$root/wpa" -s "$root" -i wlan0 scan > /tmp/wpa-scan-command.txt
sleep 3
"$root/wpa_cli" -p "$root/wpa" -s "$root" -i wlan0 scan_results > /tmp/wpa-scan-results.txt
"$root/wpa_cli" -p "$root/wpa" -s "$root" -i wlan0 terminate
set +e
wait "$wpa_trace"
wpa_result=$?
set -e
printf 'Trace exits: WPA=%s DHCP=%s\n' "$wpa_result" "$dhcp_result"
if test -d "$root/wpa"; then
    ls -lan "$root/wpa"
else
    printf 'WPA removed its control directory during normal termination\n'
fi
ls -lan /etc/resolv.conf
ls -la /bin/udhcpc
