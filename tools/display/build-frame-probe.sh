#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
exec sh tools/toolchain/build-static-rust.sh research/probes/display/frame_probe.rs "$PWD/assets/build/display-agent/frame-probe"
