#!/bin/sh
# Build research/gpu/egl-probe.rs for the Android ABI, the way tools/audio/rust-build.sh
# builds the audio probes: Rust 1.92 for armv7-linux-androideabi, linked by
# tools/audio/android-linker.sh against the exact TV Bionic libraries in
# assets/research/native-vendor-stack/bionic (extracted from the firmware).
#   TV_ANDROID_LIB_DIR  Bionic libraries (libc.so, libdl.so, libm.so, liblog.so)
#   TV_GPU_BUILD_DIR    output directory (default assets/build/gpu-probe)
set -eu
cd "$(dirname "$0")/../.."
root=$PWD
out=${TV_GPU_BUILD_DIR:-$root/assets/build/gpu-probe}
mkdir -p "$out"
rust() { if [ -n "${TV_RUSTC:-}" ]; then "$TV_RUSTC" "$@"; else rustc +1.92.0 "$@"; fi; }
llvm_tool() {
    if [ -x "/opt/homebrew/opt/llvm/bin/$1" ]; then printf '%s' "/opt/homebrew/opt/llvm/bin/$1"; else command -v "$1"; fi
}
clang=${TV_ANDROID_CLANG:-$(llvm_tool clang)}
lib=${TV_ANDROID_LIB_DIR:-$root/assets/research/native-vendor-stack/bionic}
for library in libc.so libdl.so libm.so liblog.so; do
    test -f "$lib/$library" || { echo "Missing exact TV library $lib/$library (extract it from the firmware backup)" >&2; exit 1; }
done
"$clang" --target=armv7-linux-androideabi30 -fPIE -c src/pid1/audio/bionic-start.S -o "$out/bionic-start.o"
TV_AUDIO_START_OBJECT="$out/bionic-start.o" TV_ANDROID_LIB_DIR="$lib" \
rust --edition=2021 --target armv7-linux-androideabi \
  -C linker="$root/tools/audio/android-linker.sh" \
  -C panic=abort -C opt-level=2 -C debuginfo=0 -C strip=symbols \
  -C link-arg=-Wl,--gc-sections -C link-arg=-ldl research/gpu/egl-probe.rs -o "$out/egl-probe"
printf '%s\n' "$out/egl-probe"
