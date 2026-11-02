#!/usr/bin/env bash
# Step 1 of the C -> Rust translation of FFmpeg's H.264 decoder (see regenerate.sh):
# configure and build the same static H.264-only FFmpeg as tools/ffh264/build-ffmpeg.sh,
# but with inline asm and NEON intrinsics disabled (so the C fallbacks are what
# c2rust sees), into
#   assets/build/ffh264/work-<rust-target>/   (config.h, objects, libs)
#   assets/build/ffh264/c/<rust-target>/lib/   (libavcodec.a, libavutil.a, libytdec.a)
# The second tree is a drop-in YTDEC_DIR for ytplay, used to make the link map
# (prune.py) and as the C baseline for the bit-exact gate.
#   tools/ffh264/build-ffmpeg.sh [aarch64-unknown-linux-musl] [armv7-unknown-linux-musleabihf]
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../.." && pwd)
SRC=${FFMPEG_SRC:-/nix/store/cizg7bxjfvcska1fjnpa68cf7l5ysapf-ffmpeg}
TARGETS=("$@")
[ ${#TARGETS[@]} -gt 0 ] || TARGETS=(aarch64-unknown-linux-musl armv7-unknown-linux-musleabihf)

for T in "${TARGETS[@]}"; do
  case $T in
    aarch64-unknown-linux-musl)     ZT=aarch64-linux-musl;     ARCH=aarch64; CPU=cortex_a55; EXTRA=() ;;
    armv7-unknown-linux-musleabihf) ZT=arm-linux-musleabihf;   ARCH=arm;     CPU=cortex_a55
                                    EXTRA=(--enable-neon --enable-vfp) ;;
    *) echo "unknown target $T" >&2; exit 2 ;;
  esac
  B=$ROOT/assets/build/ffh264/work-$T
  OUT=$ROOT/assets/build/ffh264/c/$T
  rm -rf "$B" "$OUT"; mkdir -p "$B/bin" "$OUT/lib"
  printf '#!/bin/sh\nexec zig cc -target %s -mcpu=%s "$@"\n' "$ZT" "$CPU" > "$B/bin/cc"
  printf '#!/bin/sh\nexec zig ar "$@"\n' > "$B/bin/ar"
  printf '#!/bin/sh\nexec zig ranlib "$@"\n' > "$B/bin/ranlib"
  chmod +x "$B/bin/"*
  (
    cd "$B"
    "$SRC/configure" --prefix="$OUT" --enable-cross-compile --target-os=linux --arch=$ARCH \
      --cc="$B/bin/cc" --ar="$B/bin/ar" --ranlib="$B/bin/ranlib" --nm=nm --strip=true \
      --pkg-config=false --disable-everything --disable-programs --disable-doc \
      --disable-network --disable-autodetect --disable-avdevice --disable-avformat \
      --disable-swscale --disable-swresample --disable-avfilter --disable-debug \
      --enable-static --disable-shared --enable-pthreads --disable-inline-asm \
      --enable-decoder=h264 --enable-parser=h264 "${EXTRA[@]}" > configure.log 2>&1 \
      || { tail -30 configure.log; exit 1; }
    # NEON intrinsics (only AV_COPY128/AV_ZERO128 on aarch64) can't be
    # translated; FFmpeg's generic fallback is plain 64-bit copies.
    sed -i '' 's/^#define HAVE_INTRINSICS_NEON 1$/#define HAVE_INTRINSICS_NEON 0/' config.h
    make -j"$(sysctl -n hw.ncpu)" V=1 > make.log 2>&1 || { tail -30 make.log; exit 1; }
    make install > install.log 2>&1
    "$B/bin/cc" -O2 -I"$OUT/include" -c "$ROOT/src/ffh264/ytdec.c" -o ytdec.o
    "$B/bin/ar" rcs "$OUT/lib/libytdec.a" ytdec.o
  )
  echo "$T:"; ls -l "$OUT/lib/"*.a
done
