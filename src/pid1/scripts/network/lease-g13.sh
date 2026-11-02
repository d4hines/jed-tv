#!/system/bin/sh
# Native G13 Wi-Fi DHCP; retain the dedicated Ethernet management route.
set -eu
test "$(getprop ro.serialno)" = 18ACC22D1C47
test "$interface" = wlan0
base=/data/local/tmp/dande-g13
case "$1" in
    deconfig)
        ip addr flush dev wlan0 scope global
        setprop sys.dande.wifi_ready 0
        ;;
    bound|renew)
        case "$ip" in
            192.168.77.*) echo 'Wi-Fi overlaps the Ethernet management subnet' >&2; exit 1 ;;
        esac
        : "${subnet:?DHCP offered no subnet mask}"
        ifconfig wlan0 "$ip" netmask "$subnet" up
        # Android's final unreachable rule otherwise hides the main table.
        # The earlier priority-9000 Ethernet rule remains authoritative.
        if ! ip rule show | grep -q '9010:.*lookup main'; then
            ip rule add priority 9010 lookup main
        fi
        gateway=${router:-}
        gateway=${gateway%% *}
        if [ -n "$gateway" ]; then
            ip route replace default via "$gateway" dev wlan0 metric 200
        fi
        if [ -n "${dns:-}" ]; then
            umask 022
            mkdir -p "$base/root/etc"
            : > "$base/root/etc/resolv.conf.next"
            for server in $dns; do
                printf 'nameserver %s\n' "$server" >> "$base/root/etc/resolv.conf.next"
            done
            mv "$base/root/etc/resolv.conf.next" "$base/root/etc/resolv.conf"
        fi
        setprop sys.dande.wifi_ready 1
        printf 'G13 Wi-Fi lease: %s, gateway %s\n' "$ip" "$gateway"
        ;;
esac
