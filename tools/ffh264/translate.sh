#!/usr/bin/env bash
# Steps 2-4 of the C -> Rust translation (see regenerate.sh), for one or both targets:
#   tools/ffh264/translate.sh [TARGET...]
# Needs build-ffmpeg.sh output. Links ytplay against the C libs to get a link
# map, prunes to the surviving translation units, preprocesses them, runs c2rust
# on eduardo-vm, and fetches the raw translation to assets/build/ffh264/raw/TARGET.
# postprocess.py then turns raw/ into src/ffh264/src/TARGET.
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../.." && pwd)
B=$ROOT/assets/build/ffh264
SRC=${FFMPEG_SRC:-/nix/store/cizg7bxjfvcska1fjnpa68cf7l5ysapf-ffmpeg}
VM=${FFH264_VM:-eduardo-vm}
TARGETS=("$@")
[ ${#TARGETS[@]} -gt 0 ] || TARGETS=(aarch64-unknown-linux-musl armv7-unknown-linux-musleabihf)

for T in "${TARGETS[@]}"; do
  # 2. link map of the C build, pruned to the surviving objects
  ( cd "$ROOT/src" && YTDEC_DIR=$B/c/$T CARGO_TARGET_DIR=$B/target \
      cargo rustc -q --release --target "$T" -p ytplay --bin ytplay -- \
      -C link-arg=-Map="$B/ytplay-c-$T.map" )
  python3 "$HERE/prune.py" "$B/ytplay-c-$T.map" "$B/work-$T" "$SRC" "$B/tus-$T.json"
  # 3. preprocess with the exact build command lines
  rm -rf "$B/pp/$T"
  python3 "$HERE/preprocess.py" "$T"
  # 4. c2rust on the VM
  ssh "$VM" mkdir -p Downloads/ffh264/pp
  rsync -a --delete "$B/pp/$T/" "$VM:Downloads/ffh264/pp/$T/"
  ssh "$VM" bash -s -- "$T" 11 < "$HERE/transpile-remote.sh"
  rm -rf "$B/raw/$T"; mkdir -p "$B/raw"
  rsync -a "$VM:Downloads/ffh264/rs/$T/" "$B/raw/$T/"
  rsync -a "$VM:Downloads/ffh264/logs/$T/" "$B/raw/$T-logs/"
done
