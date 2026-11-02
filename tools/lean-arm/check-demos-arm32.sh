#!/bin/sh
# Playsim identity on 32-bit ARM: cross-build `verify`, then replay every demo
# (shareware DEMO1-3, Freedoom 1 DEMO1-4, Freedoom 2 DEMO1-4, FreeDM DEMO1-3)
# as PID 1 in the ARM32 QEMU surrogate against the Chocolate Doom digests.
# Absent WADs or digests are skipped, as in test/doomtv/check-demos.sh.
set -eu
cd "$(dirname "$0")/../.."
T=armv7-unknown-linux-musleabihf
A=assets
F=$A/third_party/doom_lean/src/fixtures
FD=$A/third_party/freedoom
FR=$A/research/lean-doom-playsim/oracle-digests
B=$A/build/lean-arm/verify-armv7
[ -f $A/build/lean-arm/$T/libleanrt.a ] || tools/lean-arm/build.sh $T $A/build/lean-arm/$T
tools/lean-arm/build-exe.sh $T test/doomtv verify $B
status=0
run() {
  wad=$1 tag=$2 name=$3 dig=$4
  if [ ! -f "$wad" ] || [ ! -f "$dig" ]; then echo "$tag $name: skipped"; return; fi
  out=$(python3 tools/lean-arm/qemu-run.py $B --memory 1024 --file "$wad:/f/iwad.wad" --file "$dig:/f/ref.dig" \
    --arg=--iwad --arg=/f/iwad.wad --arg=--demo --arg="$name" --arg=--ref-digest --arg=/f/ref.dig \
    --arg=--out-dir --arg=/out --expect "identical to the oracle" --timeout 1800 \
    | grep -E 'identical to the oracle|diverges|engine error' || true)
  echo "$tag ${out:-$name: NO RESULT}"
  case "$out" in *identical*) ;; *) status=1 ;; esac
}
run $F/wads/doom1.wad shareware DEMO1 $F/demo1.dig
run $F/wads/doom1.wad shareware DEMO2 $F/demo2.dig
run $F/wads/doom1.wad shareware DEMO3 $F/holdout/demo3.dig
for d in DEMO1 DEMO2 DEMO3 DEMO4; do run $FD/freedoom1.wad freedoom1 $d $FR/freedoom1/$d.dig; done
for d in DEMO1 DEMO2 DEMO3 DEMO4; do run $FD/extract/freedoom-0.13.0/freedoom2.wad freedoom2 $d $FR/freedoom2/$d.dig; done
for d in DEMO1 DEMO2 DEMO3; do run $FD/freedm.wad freedm $d $FR/freedm/$d.dig; done
exit $status
