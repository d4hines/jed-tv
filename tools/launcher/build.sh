#!/usr/bin/env bash
set -euo pipefail
repo_root=$(cd "$(dirname "$0")/../.." && pwd)
cd "$repo_root/research/launcher"
launcher_id=$(cat src/main.rs Cargo.lock ../../tools/pid1/runtime.json | shasum -a 256 | cut -d ' ' -f 1)
TV_LAUNCHER_ID="$launcher_id" CARGO_TARGET_DIR="$repo_root/assets/build/launcher/target-prod" \
  cargo build --locked --offline --release --target armv7-unknown-linux-musleabihf --bin tv-launcher
cd "$repo_root"
python3 tools/launcher/package.py "$launcher_id"
nix-store --add-fixed sha256 assets/build/launcher/g06-launcher-runtime.tar
