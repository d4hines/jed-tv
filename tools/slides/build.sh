#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
target=${1:-armv7-unknown-linux-musleabihf}
out=${2:-$root/assets/build/lean-slides/$target/slides}
lib=${PLATFORM_LIB:-}
if [ -z "$lib" ]; then
  (cd "$root/src" && cargo build --offline --release --target "$target" -p platform --target-dir "$root/assets/build/lean-slides/cargo")
  lib=$root/assets/build/lean-slides/cargo/$target/release/libplatform.a
fi
python3 "$root/tools/slides/prepare.py"
(cd "$root/test/slides" && lake build Check && lake env lean --run Check.lean --validate)
EXTRA_C="$root/src/slides/native/tv_ffi.c" EXTRA_CFLAGS="-I$root/src/platform/include" \
  EXTRA_LINK="$lib -lpthread -ldl -lm -Wl,--wrap=lean_init_task_manager,--wrap=lean_run_main" \
  bash "$root/tools/lean-arm/build-exe.sh" "$target" "$root/src/slides" slides "$out"
