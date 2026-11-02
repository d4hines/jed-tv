#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
root=$PWD
out=${TV_AUDIO_BUILD_DIR:-$root/assets/build/rust-audio-toolchain}
mkdir -p "$out"
rust() { if [ -n "${TV_RUSTC:-}" ]; then "$TV_RUSTC" "$@"; else rustc +1.92.0 "$@"; fi; }
# Preserve the original macOS LLVM choice; Apple clang cannot target this ABI.
llvm_tool() {
    if [ -x "/opt/homebrew/opt/llvm/bin/$1" ]; then
        printf '%s' "/opt/homebrew/opt/llvm/bin/$1"
    else
        command -v "$1"
    fi
}
clang=${TV_ANDROID_CLANG:-$(llvm_tool clang)}
readelf=${TV_ANDROID_READELF:-$(llvm_tool llvm-readelf)}
rustroot=$(rust --print sysroot)
test -d "$rustroot/lib/rustlib/armv7-linux-androideabi/lib" || {
    echo 'Install the pinned target: rustup target add armv7-linux-androideabi --toolchain 1.92.0' >&2
    exit 1
}
"$clang" --target=armv7-linux-androideabi30 -fPIE -c src/pid1/audio/bionic-start.S -o "$out/bionic-start.o"
export TV_AUDIO_START_OBJECT="$out/bionic-start.o"
source=${1:-research/probes/audio/toolchain-probe.rs}
output=${2:-$out/toolchain-probe}
if [ "$#" -gt 0 ]; then shift; fi
if [ "$#" -gt 0 ]; then shift; fi
mkdir -p "$(dirname "$output")"
for library in libc.so libdl.so libm.so liblog.so; do
    test -f "${TV_ANDROID_LIB_DIR:-assets/research/native-vendor-stack/bionic}/$library" || { echo "Missing exact TV library: $library" >&2; exit 1; }
done
rust --edition=2021 --target armv7-linux-androideabi \
  -C linker="$root/tools/audio/android-linker.sh" \
  -C panic=abort -C opt-level=2 -C debuginfo=0 -C strip=symbols \
  -C link-arg=-Wl,--gc-sections "$source" -o "$output" "$@"
"$readelf" -h -l -d "$output" > "$output.elf.txt"
printf '%s\n' "$output"
