#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
mkdir -p assets/build/display-agent
exec sh ./tools/toolchain/build-static-rust.sh research/probes/display/mi_inspect.rs "$PWD/assets/build/display-agent/mi-inspect"
