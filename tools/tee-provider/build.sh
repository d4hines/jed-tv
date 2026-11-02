#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
out="$PWD/assets/build/audio-agent/tee-provider"
mkdir -p "$out"
CARGO_TARGET_DIR="$out" RUSTFLAGS='-C linker=rust-lld -C target-cpu=cortex-a55' cargo +1.92.0 build --offline --locked --release --manifest-path research/tee-provider/Cargo.toml --target armv7-unknown-linux-musleabihf
printf '%s\n' "$out/armv7-unknown-linux-musleabihf/release/g06-tee-provider-candidate"
