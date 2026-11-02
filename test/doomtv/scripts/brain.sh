#!/bin/sh
# Doom II boss-brain flow test, which random input never reaches: a copy of
# freedoom2.wad with the MAP30 player start moved into the brain's closet
# (plus a Commander Keen). The player shoots the Keen, then the brain; the
# noise wakes the eye (cube spitting, spawn fires), the brain explodes and
# exits, the intermission and MAP30 text are pressed through and the cast
# runs ~3000 tics without input. Compares every tic with the oracle.
#
# usage: ORACLE=.../run-oracle.sh brain.sh FREEDOOM2_WAD OUTDIR
set -eu
WAD=$1 OUT=$2
HERE=$(cd "$(dirname "$0")" && pwd)
ORACLE=${ORACLE:?set ORACLE to the oracle run-oracle.sh}
VERIFY=${VERIFY_BIN:-$HERE/../.lake/build/bin/verify}
mkdir -p "$OUT"
python3 -I "$HERE/make_testwad.py" "$WAD" "$OUT/freedoom2-brain.wad" MAP30 \
  move:1:-2208:4450:90 add:72:-2208:4510:270:7 > /dev/null
python3 -I "$HERE/gen_demo.py" "$OUT/brain.lmp" --episode 1 --map 30 --skill 3 --tics 4000 \
  --scripted fire:0-640,press:700,press:760,press:820,press:880,press:940,press:1000
"$ORACLE" -iwad "$OUT/freedoom2-brain.wad" -tracedemo "$OUT/brain.lmp" -trace /dev/null \
  -digest "$OUT/oracle.dig" -nosound -nomusic -nosfx > "$OUT/oracle.log" 2>&1
"$VERIFY" --iwad "$OUT/freedoom2-brain.wad" --demo "$OUT/brain.lmp" --ref-digest "$OUT/oracle.dig" \
  --out-dir "$OUT" --flow
