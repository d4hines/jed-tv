#!/bin/sh
# Build src/flightgpu (the city flight demo's GPU renderer) for the Android ABI,
# like tools/gpu/build-probe.sh: Rust 1.92 for armv7-linux-androideabi, linked
# by tools/audio/android-linker.sh against the exact TV Bionic libraries.
#   TV_ANDROID_LIB_DIR  Bionic libraries (libc.so, libdl.so, libm.so, liblog.so)
#   TV_GPU_BUILD_DIR    output directory (default assets/build/gpu-probe)
set -eu
cd "$(dirname "$0")/../.."
root=$PWD
out=${TV_GPU_BUILD_DIR:-$root/assets/build/gpu-probe}
mkdir -p "$out"
llvm_tool() {
    if [ -x "/opt/homebrew/opt/llvm/bin/$1" ]; then printf '%s' "/opt/homebrew/opt/llvm/bin/$1"; else command -v "$1"; fi
}
clang=${TV_ANDROID_CLANG:-$(llvm_tool clang)}
lib=${TV_ANDROID_LIB_DIR:-$root/assets/research/native-vendor-stack/bionic}
for library in libc.so libdl.so libm.so liblog.so; do
    test -f "$lib/$library" || { echo "Missing exact TV library $lib/$library (extract it from the firmware backup)" >&2; exit 1; }
done
"$clang" --target=armv7-linux-androideabi30 -fPIE -c src/pid1/audio/bionic-start.S -o "$out/bionic-start.o"
cd src
TV_AUDIO_START_OBJECT="$out/bionic-start.o" TV_ANDROID_LIB_DIR="$lib" \
CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_LINKER="$root/tools/audio/android-linker.sh" \
CARGO_TARGET_ARMV7_LINUX_ANDROIDEABI_RUSTFLAGS="-C panic=abort -C link-arg=-Wl,--gc-sections -C link-arg=-ldl" \
  cargo build --release --target armv7-linux-androideabi --target-dir "$out/target" -p flightgpu
cp "$out/target/armv7-linux-androideabi/release/flightgpu" "$out/flightgpu"
printf '%s\n' "$out/flightgpu"
