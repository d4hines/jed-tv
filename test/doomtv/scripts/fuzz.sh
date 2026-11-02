#!/bin/sh
# Play synthetic demos through the oracle and the Lean engine and compare
# their digest streams, JOBS cases in parallel. Prints one line per case;
# exits non-zero on any mismatch.
#
# usage: ORACLE=.../run-oracle.sh fuzz.sh WAD CASES_FILE OUTDIR
#   CASES_FILE lines: name episode map skill players deathmatch flags tics seed [timer]
#   flags: any of r (respawn) f (fast) n (nomonsters), or -
set -u
WAD=$1 CASES=$2 OUT=$3
HERE=$(cd "$(dirname "$0")" && pwd)
export WAD OUT HERE
export ORACLE=${ORACLE:?set ORACLE to the oracle run-oracle.sh}
export VERIFY=${VERIFY_BIN:-$HERE/../.lake/build/bin/verify}
mkdir -p "$OUT"
grep -v '^\s*\(#\|$\)' "$CASES" | xargs -P "${JOBS:-8}" -L 1 sh -c '
  name=$0 ep=$1 map=$2 skill=$3 players=$4 dm=$5 flags=$6 tics=$7 seed=$8 timer=${9:-}
  extra=""
  case "$flags" in *r*) extra="$extra --respawn";; esac
  case "$flags" in *f*) extra="$extra --fast";; esac
  case "$flags" in *n*) extra="$extra --nomonsters";; esac
  d="$OUT/$name"; mkdir -p "$d"
  python3 -I "$HERE/gen_demo.py" "$d/demo.lmp" --episode "$ep" --map "$map" --skill "$skill" \
    --players "$players" --deathmatch "$dm" --tics "$tics" --seed "$seed" $extra
  topt=""; vopt=""
  if [ -n "$timer" ]; then topt="-timer $timer"; vopt="--timer $timer"; fi
  "$ORACLE" -iwad "$WAD" -tracedemo "$d/demo.lmp" -trace /dev/null -digest "$d/oracle.dig" \
    -nosound -nomusic -nosfx $topt > "$d/oracle.log" 2>&1
  orc=$?
  res=$("$VERIFY" --iwad "$WAD" --demo "$d/demo.lmp" --ref-digest "$d/oracle.dig" --out-dir "$d" $vopt 2>&1)
  vrc=$?
  echo "$name oracle_rc=$orc verify_rc=$vrc ${res##*/}"
  [ $vrc -eq 0 ]
'
