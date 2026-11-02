#!/bin/sh
# Auditable TV source counts. Lean-generated C stays in assets/build/lake.
# The retained FFmpeg-to-Rust translation and vendored source are shown separately.
# crates.io dependencies are listed by name only; they are not in src/.
set -e
cd "$(dirname "$0")/../src"
cloc() { nix shell nixpkgs#cloc -c cloc --quiet --hide-rate "$@"; }

echo "== ours: pid1 ytplay platform mp4 slides doomtv (+ doomtv's C shim)"
cloc --exclude-ext=lean pid1 ytplay platform mp4 slides doomtv
echo "== generated: ffh264 (FFmpeg 9.0.1 H.264 via c2rust; asm is FFmpeg's NEON, unmodified)"
cloc ffh264
echo "== Lean: runtime, experimental YouTube and TV control"
cloc --include-ext=lean lean pid1/lean
echo "== vendored: doomgeneric (Doom engine, C)"
cloc third_party
echo "== crates.io dependencies (not in src/; pinned by Cargo.lock)"
cargo tree -q --target armv7-unknown-linux-musleabihf -e normal,no-proc-macro --prefix none --format '{p}' \
  | sed 's/ (\*)//; s/ v/@/' | grep -v ' (/' | sort -u > /tmp/loc-deps.txt
echo "$(wc -l < /tmp/loc-deps.txt | tr -d ' ') crates:"
tr '\n' ' ' < /tmp/loc-deps.txt | fold -s -w 100; echo
