#!/bin/sh
set -eu
test "$(id -u)" = 0
grep -q 'androidboot.serialno=4C0490290FA05A0C0' /proc/cmdline
umask 077
test -d /run && test ! -L /run
test "$(stat -c %u /run)" = 0
mode=$(stat -c %a /run)
test "$((0$mode & 022))" = 0
root=/run/tv-network
test ! -L "$root"
if ! test -d "$root"; then mkdir -m 700 "$root"; fi
test "$(stat -c %u:%g:%a "$root")" = 0:0:700
for pair in lean-wpa-supplicant:wpa_supplicant lean-wpa-cli:wpa_cli wifi-prepare.sh:prepare.sh lean-lease.sh:lease.sh lean-join.sh:join.sh; do
    source=/tmp/${pair%%:*}
    target=$root/${pair#*:}
    test -f "$source" && test ! -L "$source"
    test "$(stat -c %u:%h "$source")" = 0:1
    mode=$(stat -c %a "$source")
    test "$((0$mode & 022))" = 0
    cp "$source" "$target.new"
    chmod 700 "$target.new"
    mv "$target.new" "$target"
done
if test -f /tmp/lean-router.conf; then
    test ! -L /tmp/lean-router.conf
    test "$(stat -c %u:%h /tmp/lean-router.conf)" = 0:1
    mode=$(stat -c %a /tmp/lean-router.conf)
    test "$((0$mode & 022))" = 0
    cp /tmp/lean-router.conf "$root/preset.conf.new"
    chmod 600 "$root/preset.conf.new"
    mv "$root/preset.conf.new" "$root/preset.conf"
    rm /tmp/lean-router.conf
fi
printf 'Installed private G06 network helpers\n'
