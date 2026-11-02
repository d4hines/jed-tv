#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
exec sh tools/toolchain/build-static-rust.sh research/probes/audio/direct-bridge.rs "$PWD/assets/build/audio-agent/direct-audio-bridge"
