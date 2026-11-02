#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
test "$#" -ge 2 || { echo 'usage: build-static-rust.sh SOURCE OUTPUT [RUSTC_ARGS...]' >&2; exit 2; }
source=$1
output=$2
shift 2
mkdir -p "$(dirname "$output")"
rustc +1.92.0 --edition=2021 --target armv7-unknown-linux-musleabihf \
    -C linker=rust-lld -C target-cpu=cortex-a55 -C opt-level=2 \
    -C panic=abort -C strip=symbols "$source" -o "$output" "$@"
/opt/homebrew/opt/llvm/bin/llvm-readelf -h -l -d "$output" > "$output.elf.txt"
if /usr/bin/grep -Eq 'INTERP|NEEDED' "$output.elf.txt"; then
    echo 'Unexpected dynamic runtime dependency' >&2
    exit 1
fi
printf '%s\n' "$output"
