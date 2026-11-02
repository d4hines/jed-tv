#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
out="$PWD/assets/build/audio-agent/tee-trace"
mkdir -p "$out"
rustc +1.92.0 --edition=2021 --target armv7-unknown-linux-musleabihf -C linker=rust-lld -C target-cpu=cortex-a55 -C panic=abort -C opt-level=2 -C strip=symbols research/tee-trace/main.rs -o "$out/tee-trace"
rustc +1.92.0 --edition=2021 --target armv7-unknown-linux-musleabihf -C linker=rust-lld -C target-cpu=cortex-a55 -C panic=abort -C opt-level=2 -C strip=symbols research/tee-trace/smoke.rs -o "$out/tee-trace-smoke-v2"
printf '%s\n' "$out/tee-trace"
