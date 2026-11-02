#!/bin/sh
# Record the oracle digest streams of every Freedoom demo and write a
# manifest with the oracle binary and WAD hashes.
#
# usage: record-oracle.sh [ORACLE_DIR] [OUTDIR]
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../.." && pwd)
ORA=${1:-$ROOT/assets/build/lean-doom-playsim/oracle}
OUT=${2:-$ROOT/assets/research/lean-doom-playsim/oracle-digests}
FD=$ROOT/assets/third_party/freedoom
mkdir -p "$OUT"
rec() { wad=$1 tag=$2; shift 2
  [ -f "$wad" ] || { echo "skip $tag: $wad absent"; return 0; }
  mkdir -p "$OUT/$tag"
  for d in "$@"; do
    "$ORA/run-oracle.sh" -iwad "$wad" -tracedemo "$d" -trace /dev/null -digest "$OUT/$tag/$d.dig" \
      -nosound -nomusic -nosfx > "$OUT/$tag/$d.log" 2>&1
    echo "$tag $d $(( ($(wc -c < "$OUT/$tag/$d.dig") - 16) / 8 )) tics"
  done
}
rec "$FD/freedoom1.wad" freedoom1 DEMO1 DEMO2 DEMO3 DEMO4
rec "$FD/extract/freedoom-0.13.0/freedoom2.wad" freedoom2 DEMO1 DEMO2 DEMO3 DEMO4
rec "$FD/freedm.wad" freedm DEMO1 DEMO2 DEMO3
{
  echo "# Chocolate Doom oracle digests (TRACE.md v1), recorded $(date -u +%Y-%m-%dT%H:%MZ)"
  echo "# oracle: doom_lean trace.patch + scripts/oracle-traceexits.patch on chocolate-doom 353cf5001dfd5777c13327010fa58acb57b913b2"
  shasum -a 256 "$ORA/build/src/chocolate-doom" | sed "s|$ROOT/||"
  for w in "$FD/freedoom1.wad" "$FD/extract/freedoom-0.13.0/freedoom2.wad" "$FD/freedm.wad"; do
    [ -f "$w" ] && shasum -a 256 "$w" | sed "s|$ROOT/||"
  done
  (cd "$OUT" && shasum -a 256 */*.dig)
} > "$OUT/MANIFEST.txt"
cat "$OUT/MANIFEST.txt"
