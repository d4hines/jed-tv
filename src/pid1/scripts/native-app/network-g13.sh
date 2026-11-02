#!/system/bin/sh
# G13 native boot: dedicated Ethernet management link, independent of the UI.
set -eu
test "$(getprop ro.serialno)" = 18ACC22D1C47
test "$(getprop ro.product.device)" = G13
test "$(getprop ro.build.version.incremental)" = AU13
. /metadata/dande-tv/boot-log.sh
dande_boot_mark 'NETWORK: waiting for netd and Ethernet'

setprop sys.dande.network_ready 0
# netd flushes all non-local policy rules during initialization. Its terminal
# unreachable rule is installed after that flush; service "running" alone is
# too early. Wait for this before publishing our connected-subnet route.
ready=0
for attempt in $(seq 1 300); do
    if [ "$(getprop init.svc.netd)" = running ] &&
       ip rule show | grep -q '32000:.*unreachable'; then
        ready=1
        break
    fi
    sleep .1
done
test "$ready" = 1

for attempt in $(seq 1 300); do
    [ ! -d /sys/class/net/eth0 ] || break
    sleep .1
done
test -d /sys/class/net/eth0
ip link set eth0 up
ip address replace 192.168.77.3/24 brd + dev eth0
# Android's policy rules can bypass the connected routes in the main table.
# Route only the management subnet here; do not install an Internet gateway.
ip route replace 192.168.77.0/24 dev eth0 scope link src 192.168.77.3 table main
if ! ip rule show | grep -q 'to 192.168.77.0/24 lookup main'; then
    ip rule add priority 9000 to 192.168.77.0/24 lookup main
fi
if [ "$(getprop service.adb.tcp.port)" != 5555 ]; then
    setprop service.adb.tcp.port 5555
    setprop ctl.restart adbd
fi
setprop sys.dande.network_ready 1
dande_boot_mark 'NETWORK: 192.168.77.3/24 configured; authenticated ADB :5555 enabled'
