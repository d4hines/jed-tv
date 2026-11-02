#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
out="$PWD/assets/build/audio-agent/tee-trace"
mkdir -p "$out"
rustc +1.92.0 --edition=2021 --test -C linker=/usr/bin/cc research/tee-trace/main.rs -o "$out/host-tests"
"$out/host-tests"
