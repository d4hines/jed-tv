#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
out=${TV_AUDIO_BUILD_DIR:-$PWD/assets/build/audio-agent}
mkdir -p "$out"
rustc +1.92.0 --edition=2021 -C linker=/usr/bin/cc --test research/probes/audio/direct-bridge.rs -C linker=/usr/bin/cc -o "$out/direct-audio-host-tests"
"$out/direct-audio-host-tests"
