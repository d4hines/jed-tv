#!/bin/sh
# Level-exit flow test: synthetic demos plus forced exits at given tics
# (oracle -traceexits / verify --exits, see scripts/oracle-traceexits.patch).
# Covers intermissions, next-map rules, secret levels and finales.
#
# usage: ORACLE=.../run-oracle.sh exits.sh WAD CASES_FILE OUTDIR
#   CASES_FILE lines: name episode map skill players deathmatch tics seed exits
#   exits: TIC:n|s,... (n = G_ExitLevel, s = G_SecretExitLevel)
set -u
WAD=$1 CASES=$2 OUT=$3
HERE=$(cd "$(dirname "$0")" && pwd)
export WAD OUT HERE
export ORACLE=${ORACLE:?set ORACLE to the oracle run-oracle.sh}
export VERIFY=${VERIFY_BIN:-$HERE/../.lake/build/bin/verify}
mkdir -p "$OUT"
grep -v '^\s*\(#\|$\)' "$CASES" | xargs -P "${JOBS:-8}" -L 1 sh -c '
  name=$0 ep=$1 map=$2 skill=$3 players=$4 dm=$5 tics=$6 seed=$7 exits=$8
  d="$OUT/$name"; mkdir -p "$d"
  python3 -I "$HERE/gen_demo.py" "$d/demo.lmp" --episode "$ep" --map "$map" --skill "$skill" \
    --players "$players" --deathmatch "$dm" --tics "$tics" --seed "$seed"
  "$ORACLE" -iwad "$WAD" -tracedemo "$d/demo.lmp" -trace /dev/null -digest "$d/oracle.dig" \
    -nosound -nomusic -nosfx -traceexits "$exits" > "$d/oracle.log" 2>&1
  orc=$?
  res=$("$VERIFY" --iwad "$WAD" --demo "$d/demo.lmp" --ref-digest "$d/oracle.dig" --out-dir "$d" \
    --exits "$exits" 2>&1)
  vrc=$?
  echo "$name oracle_rc=$orc verify_rc=$vrc ${res##*/}"
  [ $vrc -eq 0 ]
'
