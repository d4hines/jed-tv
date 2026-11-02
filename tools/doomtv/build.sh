#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
target=${1:-armv7-unknown-linux-musleabihf}
out=${2:-$root/assets/build/doom/$target/doomtv}
export DOOM_BUILD_ROOT=${DOOM_BUILD_ROOT:-$root/assets/build/doom}
export DOOMGENERIC_DIR="$DOOM_BUILD_ROOT/$target"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-$root/assets/build/app-target}
case "$CARGO_TARGET_DIR" in /*) ;; *) CARGO_TARGET_DIR="$root/src/$CARGO_TARGET_DIR" ;; esac
bash "$root/tools/doom/build.sh" "$target"
(cd "$root/src" && cargo build --offline --locked --release --target "$target" -p doomtv)
mkdir -p "$(dirname "$out")"
built=$CARGO_TARGET_DIR/$target/release/doomtv
if [ "$built" != "$out" ]; then cp "$built" "$out"; fi
