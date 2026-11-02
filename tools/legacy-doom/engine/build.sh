#!/usr/bin/env bash
# Cross-compile the original Doom engine (id Software's GPL release, via
# doomgeneric) into a static library for doomtv, with `zig cc` on the Mac.
#   app/doom/build.sh [aarch64-unknown-linux-musl] [armv7-unknown-linux-musleabihf]
# Source: assets/third_party/doomgeneric (git clone of github.com/ozkl/doomgeneric,
# pinned below; fetched once, see the clone command at the bottom). Only engine
# files are compiled: no SDL/X11/Windows backend, none of its Makefiles/scripts.
# The platform hooks (DG_*) live in Rust: app/crates/doomtv.
# Output: assets/build/doom-minimal/engine/<rust-target>/libdoomgeneric.a
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../.." && pwd)
PIN=e257a09527062bfb986b6e5e0780839b6cbd5b71
SRC=$ROOT/assets/third_party/doomgeneric
if [ ! -d "$SRC/.git" ]; then
  echo "missing $SRC; fetch it with:" >&2
  echo "  git clone https://github.com/ozkl/doomgeneric $SRC && git -C $SRC checkout $PIN" >&2
  exit 1
fi
HEAD=$(git -C "$SRC" rev-parse HEAD)
[ "$HEAD" = "$PIN" ] || { echo "doomgeneric is at $HEAD, expected $PIN" >&2; exit 1; }
DG=$SRC/doomgeneric
export ZIG_GLOBAL_CACHE_DIR="$ROOT/assets/build/doom-minimal/zig-global"
export ZIG_LOCAL_CACHE_DIR="$ROOT/assets/build/doom-minimal/zig-local"

# The engine objects of doomgeneric's Makefile, minus the X11 backend
# (doomgeneric_xlib.o). FEATURE_SOUND is deliberately absent.
# include/SDL_mixer.h is an empty stand-in for an unused include.
ENGINE=(dummy am_map doomdef doomstat dstrings d_event d_items d_iwad d_loop d_main
  d_mode d_net f_finale f_wipe g_game hu_lib hu_stuff info i_cdmus i_endoom i_joystick
  i_scale i_sound i_system i_timer memio m_argv m_bbox m_cheat m_config m_controls
  m_fixed m_menu m_misc m_random p_ceilng p_doors p_enemy p_floor p_inter p_lights
  p_map p_maputl p_mobj p_plats p_pspr p_saveg p_setup p_sight p_spec p_switch
  p_telept p_tick p_user r_bsp r_data r_draw r_main r_plane r_segs r_sky r_things
  sha1 sounds statdump st_lib st_stuff tables v_video wi_stuff w_checksum
  w_file w_main w_wad z_zone w_file_stdc i_input i_video doomgeneric)
SOUND_SOURCE=silent_sound
ENGINE_ROOT=engine
if [ "${DOOM_SOUND:-0}" = 1 ]; then
  ENGINE+=(s_sound)
  SOUND_SOURCE=doomtv_sound
  ENGINE_ROOT=engine-sound
fi

# Doom renders 320x200; the Rust side scales that to the screen (4:3, 960x720),
# so ask doomgeneric for 320x200 XRGB8888 (fb_scaling 1) and skip its own 2x.
# exit()/system() are renamed so the engine can never end PID 1 (doomtv_shim.c).
# -fsigned-char: the engine was developed on x86, where char is signed.
# -fno-sanitize=all: zig cc may otherwise add UBSan traps, and vanilla Doom
# relies on wrapping fixed-point arithmetic.
CFLAGS=(-O2 -std=gnu99 -fsigned-char -fno-strict-aliasing -fno-sanitize=all
  -ffunction-sections -fdata-sections -w
  -DNORMALUNIX -DLINUX -D_DEFAULT_SOURCE
  -DDOOMGENERIC_RESX=320 -DDOOMGENERIC_RESY=200
  -Dexit=doomtv_exit -Dsystem=doomtv_system)
if [ "${DOOM_SOUND:-0}" = 1 ]; then CFLAGS+=(-DFEATURE_SOUND); fi

TARGETS=("$@")
[ ${#TARGETS[@]} -gt 0 ] || TARGETS=(aarch64-unknown-linux-musl armv7-unknown-linux-musleabihf)
for T in "${TARGETS[@]}"; do
  case $T in
    aarch64-unknown-linux-musl)     ZT=aarch64-linux-musl ;;
    armv7-unknown-linux-musleabihf) ZT=arm-linux-musleabihf ;;
    *) echo "unknown target $T" >&2; exit 2 ;;
  esac
  OUT=$ROOT/assets/build/doom-minimal/$ENGINE_ROOT/$T
  B=$ROOT/assets/build/doom-minimal/$ENGINE_ROOT/work-$T
  # Replace the library atomically at the end: other cargo builds of the
  # workspace (it builds every member) may be linking the old one meanwhile.
  rm -rf "$B"; mkdir -p "$B" "$OUT"
  # One compile per job: $B/cc1 SRC OBJ (prints nothing unless it fails).
  { printf '#!/bin/sh\n'
    printf 'zig cc -target %s -mcpu=cortex_a55' "$ZT"
    printf ' %q' "${CFLAGS[@]}" "-I$DG" "-I$ROOT/src/legacy-doom/engine/include"
    printf ' -c "$1" -o "$2" 2>"$2.log" || { cat "$2.log" >&2; exit 255; }\n'
  } > "$B/cc1"
  chmod +x "$B/cc1"
  JOBS=${DOOM_BUILD_JOBS:-4}
  { for f in "${ENGINE[@]}"; do printf '%s\0%s\0' "$DG/$f.c" "$B/$f.o"; done
    for f in doomtv_shim "$SOUND_SOURCE"; do printf '%s\0%s\0' "$ROOT/src/legacy-doom/engine/$f.c" "$B/$f.o"; done
  } | xargs -0 -n 2 -P "$JOBS" "$B/cc1"
  zig ar rcs "$B/libdoomgeneric.a" "$B"/*.o
  echo "$PIN" > "$OUT/DOOMGENERIC_COMMIT"
  mv -f "$B/libdoomgeneric.a" "$OUT/libdoomgeneric.a"
  echo "$T: $(ls -l "$OUT/libdoomgeneric.a" | awk '{print $5}') bytes, $(ls "$B"/*.o | wc -l | tr -d ' ') objects"
done
