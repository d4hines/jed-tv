#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
export CARGO_TARGET_DIR="$PWD/assets/build/tee-firmware-rust"
export UV_CACHE_DIR="$PWD/assets/build/uv-cache"
RUSTFLAGS='-C linker=/usr/bin/cc' cargo +1.92.0 build --offline --locked --manifest-path research/tee-provider/Cargo.toml --example firmware-replay
uv run --python 3.12 --with unicorn==2.1.4 --with pyelftools==0.32 python tools/tee-provider/check-firmware.py "$@"
