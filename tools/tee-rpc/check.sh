#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
out="$PWD/assets/build/audio-agent/tee-rpc"
mkdir -p "$out"
rustc +1.92.0 --edition=2021 --test -C linker=/usr/bin/cc research/tee-rpc/lib.rs -o "$out/host-tests"
"$out/host-tests"
rustc +1.92.0 --edition=2021 --crate-name g06_tee_rpc --crate-type rlib --target armv7-unknown-linux-musleabihf -C panic=abort -C opt-level=2 research/tee-rpc/lib.rs -o "$out/libg06_tee_rpc.rlib"
