#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
export CARGO_TARGET_AARCH64_APPLE_DARWIN_LINKER=/usr/bin/cc
export DOOM_BUILD_ROOT="$PWD/assets/build/native-app-doom"
export DOOMGENERIC_DIR="$DOOM_BUILD_ROOT/armv7-unknown-linux-musleabihf"
bash tools/doom/build.sh armv7-unknown-linux-musleabihf
R=$PWD/assets/build/native-app-target/armv7-unknown-linux-musleabihf/release
(cd src && cargo build --offline --locked --release --target armv7-unknown-linux-musleabihf --target-dir ../assets/build/native-app-target -p doomtv -p platform -p ytplay -p tvosd -p rpgtv -p snaketv)

PLATFORM_LIB="$R/libplatform.a" bash tools/slides/build.sh armv7-unknown-linux-musleabihf "$R/slides"

CITYJSB_DIR=$(nix build .#pid1.passthru.flight --no-link --print-out-paths)
export CITYJSB_DIR
(cd src && CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER="$PWD/../tools/toolchain/zcxx-armv7-unknown-linux-musleabihf" \
    CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_RUSTFLAGS="-C target-cpu=cortex-a55 -C link-self-contained=no" \
    cargo build --offline --locked --release --target armv7-unknown-linux-musleabihf --target-dir ../assets/build/native-app-target -p flighttv)

PLATFORM_LIB="$R/libplatform.a" bash tools/doomtv/build-lean.sh armv7-unknown-linux-musleabihf "$R/doomtv-lean"
LEAN_YOUTUBE=$(nix build .#lean-youtube --no-link --print-out-paths)
cp -f "$LEAN_YOUTUBE/bin/ytplay-lean" "$R/ytplay-lean"
chmod 755 "$R/ytplay-lean"
