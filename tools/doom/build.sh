#!/usr/bin/env bash
# Cross-compile the original Doom engine (id Software's GPL release, via
# doomgeneric) into a static library for doomtv, with `zig cc` on the Mac.
#   tools/doom/build.sh [aarch64-unknown-linux-musl] [armv7-unknown-linux-musleabihf]
# Source: src/third_party/doomgeneric (the engine files of github.com/ozkl/doomgeneric
# at the commit in its UPSTREAM file, vendored unmodified). No SDL/X11/Windows
# backend, none of its Makefiles/scripts. Our C glue is src/doomtv/native; the
# platform hooks (DG_*) live in Rust: src/doomtv.
# Output: assets/build/doom/<rust-target>/libdoomgeneric.a
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../.." && pwd)
PIN=e257a09527062bfb986b6e5e0780839b6cbd5b71
DG=$ROOT/src/third_party/doomgeneric
grep -q "$PIN" "$DG/UPSTREAM" || { echo "vendored doomgeneric is not at $PIN" >&2; exit 1; }
NATIVE=$ROOT/src/doomtv/native

# The engine objects of doomgeneric's Makefile, minus the X11 backend
# (doomgeneric_xlib.o). Sound: FEATURE_SOUND with our own DG_sound_module
# (doomtv_sound.c; music silent) instead of the SDL one; include/SDL_mixer.h is
# an empty stand-in for i_sound.c's unused include.
ENGINE=(dummy am_map doomdef doomstat dstrings d_event d_items d_iwad d_loop d_main
  d_mode d_net f_finale f_wipe g_game hu_lib hu_stuff info i_cdmus i_endoom i_joystick
  i_scale i_sound i_system i_timer memio m_argv m_bbox m_cheat m_config m_controls
  m_fixed m_menu m_misc m_random p_ceilng p_doors p_enemy p_floor p_inter p_lights
  p_map p_maputl p_mobj p_plats p_pspr p_saveg p_setup p_sight p_spec p_switch
  p_telept p_tick p_user r_bsp r_data r_draw r_main r_plane r_segs r_sky r_things
  net_client net_common net_io net_loop net_packet net_server net_structrw
  sha1 sounds statdump st_lib st_stuff s_sound tables v_video wi_stuff w_checksum
  w_file w_main w_wad z_zone w_file_stdc i_input i_video doomgeneric)

# Doom renders 320x200; the Rust side scales that to the screen (4:3, 960x720),
# so ask doomgeneric for 320x200 XRGB8888 (fb_scaling 1) and skip its own 2x.
# exit()/system() are renamed so the engine can never end PID 1 (doomtv_shim.c).
# -fsigned-char: the engine was developed on x86, where char is signed.
# -fno-sanitize=all: zig cc may otherwise add UBSan traps, and vanilla Doom
# relies on wrapping fixed-point arithmetic.
CFLAGS=(-O2 -std=gnu99 -fsigned-char -fno-strict-aliasing -fno-sanitize=all
  -ffunction-sections -fdata-sections -w
  -DNORMALUNIX -DLINUX -D_DEFAULT_SOURCE
  -DDOOMGENERIC_RESX=320 -DDOOMGENERIC_RESY=200 -DFEATURE_SOUND
  -Dexit=doomtv_exit -Dsystem=doomtv_system)

TARGETS=("$@")
[ ${#TARGETS[@]} -gt 0 ] || TARGETS=(aarch64-unknown-linux-musl armv7-unknown-linux-musleabihf)
for T in "${TARGETS[@]}"; do
  case $T in
    aarch64-unknown-linux-musl)     ZT=aarch64-linux-musl ;;
    armv7-unknown-linux-musleabihf) ZT=arm-linux-musleabihf ;;
    *) echo "unknown target $T" >&2; exit 2 ;;
  esac
  DOOM_OUTPUT=${DOOM_BUILD_ROOT:-$ROOT/assets/build/doom}
  OUT=$DOOM_OUTPUT/$T
  B=$DOOM_OUTPUT/work-$T
  # Replace the library atomically at the end: other cargo builds of the
  # workspace (it builds every member) may be linking the old one meanwhile.
  rm -rf "$B"; mkdir -p "$B" "$OUT"
  # One compile per job: $B/cc1 SRC OBJ (prints nothing unless it fails).
  { printf '#!/bin/sh\n'
    printf 'zig cc -target %s -mcpu=cortex_a55' "$ZT"
    printf ' %q' "${CFLAGS[@]}" "-I$DG" "-I$NATIVE/include"
    printf ' -c "$1" -o "$2" 2>"$2.log" || { cat "$2.log" >&2; exit 255; }\n'
  } > "$B/cc1"
  chmod +x "$B/cc1"
  JOBS=${DOOM_BUILD_JOBS:-4}
  { for f in "${ENGINE[@]}"; do printf '%s\0%s\0' "$DG/$f.c" "$B/$f.o"; done
    for f in doomtv_shim doomtv_sound doomtv_net; do printf '%s\0%s\0' "$NATIVE/$f.c" "$B/$f.o"; done
  } | xargs -0 -n 2 -P "$JOBS" "$B/cc1"
  zig ar rcs "$B/libdoomgeneric.a" "$B"/*.o
  echo "$PIN" > "$OUT/DOOMGENERIC_COMMIT"
  mv -f "$B/libdoomgeneric.a" "$OUT/libdoomgeneric.a"
  printf '%s\n' chocolate-2.2.1-native-udp-v1 > "$OUT/NETWORK_FEATURE"
  echo "$T: $(ls -l "$OUT/libdoomgeneric.a" | awk '{print $5}') bytes, $(ls "$B"/*.o | wc -l | tr -d ' ') objects"
done
