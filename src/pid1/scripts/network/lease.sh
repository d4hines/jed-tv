#!/bin/sh
set -eu
umask 077
root=/run/tv-network
test "$(id -u)" = 0
test -d "$root" && test ! -L "$root"
test "$(stat -c %u:%g:%a "$root")" = 0:0:700
test "$interface" = wlan0
case "$1" in
    deconfig)
        ip addr flush dev wlan0
        ;;
    bound|renew)
        case "$ip" in
            192.168.77.*) echo 'Refusing Wi-Fi address overlapping the wired debug subnet' >&2; exit 1 ;;
        esac
        : "${subnet:?DHCP offered no subnet mask}"
        ifconfig wlan0 "$ip" netmask "$subnet" up
        gateway=${router:-}
        gateway=${gateway%% *}
        if test -n "$gateway"; then
            ip route replace default via "$gateway" dev wlan0 metric 200
        fi
        if test -n "${dns:-}"; then
            mkdir -p /etc
            : > /etc/resolv.conf
            for server in $dns; do printf 'nameserver %s\n' "$server" >> /etc/resolv.conf; done
        fi
        printf 'Lean Router lease: %s, gateway %s\n' "$ip" "$gateway"
        ;;
esac
