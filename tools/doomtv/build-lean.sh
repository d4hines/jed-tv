#!/usr/bin/env bash
# Build the Lean Doom TV app: the engine (src/doomtv/lean) cross-compiled to C,
# native/tv_ffi.c over the TV app ABI, the cross-built Lean runtime and the
# platform library (libplatform.a), linked statically with zig.
#
#   tools/doomtv/build-lean.sh [TARGET] [OUT]
#     default armv7-unknown-linux-musleabihf, assets/build/lean-doom/TARGET/doomtv
#   PLATFORM_LIB  prebuilt libplatform.a for TARGET (default: build it with cargo)
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
target=${1:-armv7-unknown-linux-musleabihf}
out=${2:-$root/assets/build/lean-doom/$target/doomtv}
lib=${PLATFORM_LIB:-}
if [ -z "$lib" ]; then
  (cd "$root/src" && cargo build --release --target "$target" -p platform \
    --target-dir "$root/assets/build/lean-doom/cargo" >/dev/null)
  lib=$root/assets/build/lean-doom/cargo/$target/release/libplatform.a
fi
EXTRA_C="$root/src/doomtv/lean/native/tv_ffi.c" \
EXTRA_CFLAGS="-I$root/src/platform/include" \
EXTRA_LINK="$lib -lpthread -ldl -lm" \
  "$root/tools/lean-arm/build-exe.sh" "$target" "$root/src/doomtv/lean" doomtv "$out"
