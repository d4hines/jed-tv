#!/usr/bin/env bash
# Regenerate src/ffh264 (FFmpeg 9.0.1's H.264 decoder in unsafe Rust,
# NEON assembly kept) from scratch, for both targets:
#   tools/ffh264/regenerate.sh [TARGET...]
#
# 1. build-ffmpeg.sh   configure/build the same H.264-only FFmpeg as
#                      tools/ffh264/build-ffmpeg.sh, but with inline asm and NEON
#                      intrinsics off (C fallbacks are bit-exact by design)
# 2. translate.sh      link ytplay against it with a rust-lld -Map, keep only
#                      the translation units that survive --gc-sections
#                      (prune.py), preprocess them with the exact build command
#                      lines (preprocess.py), run c2rust 0.22.1 on eduardo-vm
#                      (transpile-remote.sh), fetch to assets/build/ffh264/raw/
# 3. postprocess.py    mechanical fixes for stable Rust 1.92 + C layout parity
#                      (see its docstring), then mandatory shrink.py: reject
#                      unsupported SPS formats and prune high-depth templates
#                      -> src/ffh264/src/<arch>/
# 4. vendor-asm.py     copy the surviving .S files (unmodified) and their
#                      includes -> src/ffh264/asm/<arch>/
# 5. flatten-asm.py    (optional) macro-free copies for the Lean asm DSL ->
#                      assets/build/ffh264/flat-asm/<target>/
# Then: cargo build -p ytplay (in src/),
# and gate with test/run.py ... --mode framemd5 + test/check_framemd5.py.
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
TARGETS=("$@")
[ ${#TARGETS[@]} -gt 0 ] || TARGETS=(aarch64-unknown-linux-musl armv7-unknown-linux-musleabihf)
"$HERE/build-ffmpeg.sh" "${TARGETS[@]}"
"$HERE/translate.sh" "${TARGETS[@]}"
for T in "${TARGETS[@]}"; do
  python3 "$HERE/postprocess.py" "$T"
  python3 "$HERE/vendor-asm.py" "$T"
  python3 "$HERE/flatten-asm.py" "$T"
done
