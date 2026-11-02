#!/usr/bin/env bash
set -euo pipefail
root=$(cd "$(dirname "$0")/../.." && pwd)
cd "$root/src/pid1"
HELLO_BUILD_ID=$(python3 "$root/tools/pid1/source_hash.py") \
  cargo build --locked --offline --release --target armv7-unknown-linux-musleabihf
cd "$root"
python3 tools/pid1/package.py
nix-store --add-fixed sha256 assets/build/hello-pid1/g06-hello-pid1-kit.tar
