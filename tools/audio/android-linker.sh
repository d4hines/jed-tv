#!/bin/sh
set -eu
root=$(CDPATH= cd -- "$(dirname "$0")/../.." && pwd)
rust() { if [ -n "${TV_RUSTC:-}" ]; then "$TV_RUSTC" "$@"; else rustc +1.92.0 "$@"; fi; }
rustroot=$(rust --print sysroot)
host=$(rust -vV | sed -n 's/^host: //p')
linker=$rustroot/lib/rustlib/$host/bin/rust-lld
for argument in "$@"; do
    shift
    case "$argument" in
        # Objects are already ARM EABI. Bionic API 30 exports the unwind
        # functions Rust needs, so a separate libunwind is unnecessary.
        -march=*|-nodefaultlibs|-lunwind) continue ;;
        -Wl,*)
            old_ifs=$IFS
            IFS=,
            for part in ${argument#-Wl,}; do set -- "$@" "$part"; done
            IFS=$old_ifs
            ;;
        *) set -- "$@" "$argument" ;;
    esac
done
exec "$linker" -flavor gnu -m armelf_linux_eabi \
    "${TV_AUDIO_START_OBJECT:-$root/assets/build/rust-audio-toolchain/bionic-start.o}" \
    -L "${TV_ANDROID_LIB_DIR:-$root/assets/research/native-vendor-stack/bionic}" \
    -e _start --dynamic-linker /system/bin/linker "$@"
