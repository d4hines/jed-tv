#!/bin/sh
# Build the Chocolate Doom trace oracle (doom_lean's patched tree at
# 353cf5001dfd5777c13327010fa58acb57b913b2, plus oracle-traceexits.patch)
# into assets/build without
# system-wide installs: SDL2/SDL2_mixer/SDL2_net/libpng come from nixpkgs.
#
# usage: build-oracle.sh [OUTDIR]   (default assets/build/lean-doom-playsim/oracle)
# Prints the binary's SHA-256. Run it with OUTDIR/run-oracle.sh.
set -eu
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../.." && pwd)
SRC=$ROOT/assets/third_party/doom_lean/src/oracle
OUT=${1:-$ROOT/assets/build/lean-doom-playsim/oracle}
mkdir -p "$OUT"
rsync -a --exclude build --exclude runtime "$SRC/" "$OUT/"
# Test hook for forced level exits (-traceexits), inert unless given.
if ! grep -q TraceForcedExits "$OUT/chocolate-doom/src/doom/g_game.c"; then
  patch -d "$OUT/chocolate-doom" -p1 < "$HERE/oracle-traceexits.patch"
fi
DEV=$(nix build --no-link --print-out-paths nixpkgs#SDL2.dev nixpkgs#SDL2_mixer.dev \
        nixpkgs#SDL2_net.dev nixpkgs#libpng.dev | tr '\n' ';')
# Xcode 16's clang cannot link against the macOS 27 CLT SDK; use the
# Command Line Tools clang with its own SDK on macOS.
if [ "$(uname)" = Darwin ]; then
  export CC=/Library/Developer/CommandLineTools/usr/bin/clang
  export SDKROOT=$(ls -d /Library/Developer/CommandLineTools/SDKs/MacOSX[0-9]*.sdk | sort -V | tail -1)
fi
CMAKE_PREFIX_PATH="$DEV" nix shell nixpkgs#cmake nixpkgs#pkg-config nixpkgs#SDL2.dev \
  nixpkgs#SDL2_mixer.dev nixpkgs#SDL2_net.dev nixpkgs#libpng.dev -c "$OUT/build.sh"
