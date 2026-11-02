#!/usr/bin/env bash
set -euo pipefail

[[ $# -eq 1 ]] || { echo "Usage: $0 ./result/boot.img | .#stock-boot" >&2; exit 1; }
image="$1"
if [[ "$image" == *'#'* || "$image" == *.drv ]]; then
    output=$(nix build --no-link --print-out-paths "$image")
    [[ -n "$output" && "$output" != *$'\n'* ]] || { echo "Expected exactly one Nix output" >&2; exit 1; }
    image="$output/boot.img"
fi
[[ -f "$image" ]] || { echo "Missing $image" >&2; exit 1; }
[[ $(head -c 8 "$image") == 'ANDROID!' ]] || { echo "Not an Android boot image" >&2; exit 1; }
bytes=$(wc -c < "$image")
(( bytes > 0 && bytes <= 32505856 )) || { echo "Boot image exceeds the G06 boot partition" >&2; exit 1; }

target=$(fastboot devices -l | awk '$2 == "fastboot" { for (i = 3; i <= NF; i++) if ($i ~ /^usb:/) print $i }')
[[ -n "$target" && "$target" != *$'\n'* ]] || { echo "Connect exactly one TV in USB bootloader Fastboot mode" >&2; exit 1; }

check() {
    local value
    value=$(fastboot -s "$target" getvar "$1" 2>&1)
    printf '%s\n' "$value"
    printf '%s\n' "$value" | sed 's/^(bootloader) *//' | tr -d '\r' | grep -Eq "^$1: *$2 *$"
}

check product C06_eu
check version-bootloader '01\.01\.250118'
check is-userspace no
check unlocked yes
check partition-size:boot '0x0*1[fF]00000'

fastboot -s "$target" flash boot "$image"
fastboot -s "$target" reboot
