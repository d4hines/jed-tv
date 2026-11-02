#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
bash "$root/tools/legacy-doom/engine/build.sh" armv7-unknown-linux-musleabihf
cd "$root/src/legacy-doom"
DOOMGENERIC_DIR="$root/assets/build/doom-minimal/engine/armv7-unknown-linux-musleabihf" cargo build --locked --release --target armv7-unknown-linux-musleabihf -p doomtv
cd "$root"
python3 tools/legacy-doom/prepare-runtime.py "${DEBUGFS:-debugfs}"
nix-store --add-fixed sha256 assets/build/doom-minimal/g06-doom-runtime.tar
