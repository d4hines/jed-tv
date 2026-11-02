#!/bin/sh
# Playsim identity against Chocolate Doom: every tic digest of each demo.
# Shareware DEMO1-3 need the upstream fixtures (doom1.wad, *.dig); the
# Freedoom demos need the WADs and the recorded oracle digests
# (scripts/record-oracle.sh). Absent WADs or digests are skipped.
set -eu
cd "$(dirname "$0")"
A=../../assets
F=${DOOM_FIXTURES:-$A/third_party/doom_lean/src/fixtures}
FD=${FREEDOOM_DIR:-$A/third_party/freedoom}
FR=${FREEDOOM_REFS:-$A/research/lean-doom-playsim/oracle-digests}
OUT=${DOOM_CHECK_OUT:-$A/build/lean-doom-check}
BIN=${VERIFY_BIN:-.lake/build/bin/verify}
[ -n "${VERIFY_BIN:-}" ] || lake build verify >/dev/null
mkdir -p "$OUT"
status=0
run() {
  wad=$1 tag=$2 name=$3 dig=$4
  if [ ! -f "$wad" ]; then echo "$tag $name: skipped ($wad absent)"; return; fi
  if [ ! -f "$dig" ]; then echo "$tag $name: skipped (no oracle digest $dig)"; return; fi
  printf "%s " "$tag"
  "$BIN" --iwad "$wad" --demo "$name" --ref-digest "$dig" --out-dir "$OUT/$tag/$name" || status=1
}
run "$F/wads/doom1.wad" shareware DEMO1 "$F/demo1.dig"
run "$F/wads/doom1.wad" shareware DEMO2 "$F/demo2.dig"
run "$F/wads/doom1.wad" shareware DEMO3 "$F/holdout/demo3.dig"
for d in DEMO1 DEMO2 DEMO3 DEMO4; do run "$FD/freedoom1.wad" freedoom1 $d "$FR/freedoom1/$d.dig"; done
for d in DEMO1 DEMO2 DEMO3 DEMO4; do
  run "$FD/extract/freedoom-0.13.0/freedoom2.wad" freedoom2 $d "$FR/freedoom2/$d.dig"
done
for d in DEMO1 DEMO2 DEMO3; do run "$FD/freedm.wad" freedm $d "$FR/freedm/$d.dig"; done
exit $status
