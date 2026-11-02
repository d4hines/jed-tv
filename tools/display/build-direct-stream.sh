#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
mkdir -p assets/build/display-agent
archive=$(python3 tools/display/lean-library.py)
exec sh ./tools/toolchain/build-static-rust.sh research/probes/display/direct_stream.rs "$PWD/assets/build/display-agent/direct-display-stream" \
    -L "native=$(dirname "$archive")" -l static=tv_display_lean
