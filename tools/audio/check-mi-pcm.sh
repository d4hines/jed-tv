#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
out=${TV_AUDIO_BUILD_DIR:-$PWD/assets/build/audio-agent}
mkdir -p "$out"
rustc +1.92.0 --edition=2021 --test src/pid1/audio/mi-pcm.rs -C linker=/usr/bin/cc -o "$out/mi-pcm-host-tests"
"$out/mi-pcm-host-tests"
