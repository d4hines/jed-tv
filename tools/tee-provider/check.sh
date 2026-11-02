#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
out="$PWD/assets/build/audio-agent/tee-provider-host"
mkdir -p "$out"
CARGO_TARGET_DIR="$out" RUSTFLAGS='-C linker=/usr/bin/cc' cargo +1.92.0 test --offline --locked --manifest-path research/tee-provider/Cargo.toml
