#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
mkdir -p assets/build/display-agent
exec sh ./tools/toolchain/build-static-rust.sh research/probes/display/direct_trial.rs "$PWD/assets/build/display-agent/direct-display-trial"
