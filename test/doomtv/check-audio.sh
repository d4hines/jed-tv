#!/bin/sh
# Host checks for the Lean audio engine (Doom/Audio/*): parsing, OPL
# bit-exactness against Nuked OPL3 (upstream and Chocolate Doom's fork),
# end-to-end music bit-exactness against Chocolate Doom's own OPL music code,
# WAV renders with signal checks, and throughput.
#
# Needs: lake and a C compiler (zig cc preferred; CC=... to override).
# Inputs are read-only: the Freedoom WADs, Chocolate Doom's vendored sources
# (assets/third_party/doom_lean/src/oracle/chocolate-doom) and upstream
# Nuked OPL3 (assets/third_party/nuked-opl3, see SOURCE.txt there; commit
# cfedb09). The shareware doom1.wad fixture adds MUS coverage if present.
# Outputs (WAVs, logs, harness binaries) go to assets/build/lean-doom-audio/.
set -eu
cd "$(dirname "$0")"
ROOT=../..
A=$ROOT/assets/third_party
OUT=${AUDIO_CHECK_OUT:-$ROOT/assets/build/lean-doom-audio}
CHOC=$A/doom_lean/src/oracle/chocolate-doom
NUKED=$A/nuked-opl3/upstream
FD1=$A/freedoom/freedoom1.wad
FDM=$A/freedoom/freedm.wad
SW=$A/doom_lean/src/fixtures/wads/doom1.wad
if [ -z "${CC:-}" ]; then
  if command -v zig >/dev/null 2>&1; then CC="zig cc"; else CC=cc; fi
fi
mkdir -p "$OUT/bin" "$OUT/opl" "$OUT/music" "$OUT/lumps"
lake build audio-check >/dev/null
E=.lake/build/bin/audio-check
fail=0
note() { echo "== $*"; }

note "build C references ($CC)"
$CC -O2 -w -o "$OUT/bin/opl_ref_upstream" -I"$NUKED" AudioCheck/c/opl_ref.c "$NUKED/opl3.c"
$CC -O2 -w -o "$OUT/bin/opl_ref_choc" -I"$CHOC/opl" AudioCheck/c/opl_ref.c "$CHOC/opl/opl3.c"
$CC -O2 -w -o "$OUT/bin/music_ref" -IAudioCheck/c/stub -I"$CHOC/src" -I"$CHOC/opl" \
  AudioCheck/c/music_ref.c "$CHOC/src/i_oplmusic.c" "$CHOC/src/midifile.c" "$CHOC/src/mus2mid.c" \
  "$CHOC/src/memio.c" "$CHOC/opl/opl_queue.c" "$CHOC/opl/opl3.c"

note "parse every music / sfx / GENMIDI lump"
WADS="$FD1 $FDM"
[ -f "$SW" ] && WADS="$WADS $SW"
$E parse-all $WADS || fail=1

note "OPL: random register traces vs Nuked OPL3 (upstream cfedb09 and Chocolate's fork)"
for seed in 1 2 3; do
  for bursty in "" bursty; do
    $E opl-trace "$OUT/opl/t.txt" 300000 $seed $bursty
    for cfg in "native 48000" "stream 48000" "stream 44100" "stream 11025"; do
      set -- $cfg
      "$OUT/bin/opl_ref_upstream" $1 $2 300000 "$OUT/opl/t.txt" "$OUT/opl/up.raw"
      "$OUT/bin/opl_ref_choc" $1 $2 300000 "$OUT/opl/t.txt" "$OUT/opl/choc.raw"
      $E opl-render $1 $2 300000 "$OUT/opl/t.txt" "$OUT/opl/lean.raw" 2>/dev/null
      if cmp -s "$OUT/opl/up.raw" "$OUT/opl/lean.raw" && cmp -s "$OUT/opl/choc.raw" "$OUT/opl/lean.raw"; then
        echo "ok   seed=$seed ${bursty:-plain} $1 $2 Hz (300000 frames)"
      else
        echo "FAIL seed=$seed ${bursty:-plain} $1 $2 Hz"; fail=1
      fi
    done
  done
done

note "music: Lean player vs Chocolate Doom's i_oplmusic.c (60 s, sample-exact)"
$E lump "$FD1" GENMIDI "$OUT/lumps/GENMIDI"
$E lump "$FD1" D_E1M1 "$OUT/lumps/D_E1M1"
$E lump "$FDM" D_RUNNIN "$OUT/lumps/D_RUNNIN"
SONGS="D_E1M1 D_RUNNIN"
if [ -f "$SW" ]; then $E lump "$SW" D_E1M1 "$OUT/lumps/MUS_E1M1"; SONGS="$SONGS MUS_E1M1"; fi
for song in $SONGS; do
  for cfg in "48000 1024 2 0 64" "44100 1024 2 0 64" "48000 333 2 0 100" "48000 1024 0 0 64" \
             "48000 1024 1 0 64" "48000 1024 2 1 127"; do
    set -- $cfg
    frames=$(($1 * 60))
    "$OUT/bin/music_ref" "$OUT/lumps/GENMIDI" "$OUT/lumps/$song" $1 $frames $2 $3 $4 $5 1 "$OUT/music/c.raw" 2>/dev/null
    $E music "$OUT/lumps/GENMIDI" "$OUT/lumps/$song" $1 $frames $2 $3 $4 $5 1 "$OUT/music/l.raw" 2>/dev/null
    if cmp -s "$OUT/music/c.raw" "$OUT/music/l.raw"; then
      echo "ok   $song rate=$1 buffer=$2 driver=$3 opl3=$4 volume=$5"
    else
      echo "FAIL $song rate=$1 buffer=$2 driver=$3 opl3=$4 volume=$5"; fail=1
    fi
  done
done

note "WAV renders"
$E render-music "$FD1" D_E1M1 180 48000 "$OUT/d_e1m1.wav" || fail=1
$E render-music "$FDM" D_RUNNIN 180 48000 "$OUT/d_runnin.wav" || fail=1
$E sfx-scene "$FD1" 48000 "$OUT/sfx-scene.wav" || fail=1
$E sfx-scene "$FD1" 48000 "$OUT/game-scene.wav" music || fail=1

note "throughput (host)"
$E bench "$FD1"

if [ $fail -eq 0 ]; then echo "check-audio: PASS"; else echo "check-audio: FAIL"; exit 1; fi
