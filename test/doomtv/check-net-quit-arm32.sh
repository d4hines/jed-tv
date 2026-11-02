#!/bin/sh
# A LAN player leaving, on 32-bit ARM: cross-build `net-quit` against the
# cross-built Lean runtime (LEAN_ARM, default assets/build/lean-arm/TARGET)
# and run it as PID 1 in the ARM32 QEMU surrogate with Freedoom's IWAD.
set -eu
cd "$(dirname "$0")/../.."
T=armv7-unknown-linux-musleabihf
B=${OUT:-assets/build/doom-test/net-quit-armv7}
WAD=assets/third_party/freedoom/freedoom1.wad
tools/lean-arm/build-exe.sh $T test/doomtv net-quit $B
ok=1
out=$(python3 tools/lean-arm/qemu-run.py $B --file "$WAD:/f/iwad.wad" --arg /f/iwad.wad \
  --expect "net-quit: PASS" --timeout 600) || ok=0
printf '%s\n' "$out" | grep -E 'net-quit|qemu-run|Kernel panic|segfault|signal' || true
[ $ok = 1 ]
