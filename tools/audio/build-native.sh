#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
out=assets/research/audio-native
mkdir -p "$out"
clang=/opt/homebrew/opt/llvm/bin/clang
rustroot=$(rustc +1.92.0 --print sysroot)
linker=$rustroot/lib/rustlib/aarch64-apple-darwin/bin/rust-lld
"$clang" --target=armv7-linux-androideabi30 -fPIC -O2 -ffreestanding -c src/pid1/audio/standalone-app-state.c -o "$out/standalone-app-state.o"
"$linker" -flavor gnu -shared "$out/standalone-app-state.o" -o "$out/standalone-app-state.so"
"$clang" --target=armv7-linux-androideabi30 -fPIE -O2 -ffreestanding -c research/probes/audio/middleware-probe.c -o "$out/middleware-probe.o"
"$clang" --target=armv7-linux-androideabi30 -fPIE -c src/pid1/audio/bionic-start.S -o "$out/bionic-start.o"
"$linker" -flavor gnu -pie --dynamic-linker /system/bin/linker -e _start --hash-style=both "$out/bionic-start.o" "$out/middleware-probe.o" assets/research/native-vendor-stack/bionic/libc.so assets/research/native-vendor-stack/bionic/libdl.so -o "$out/middleware-probe"
