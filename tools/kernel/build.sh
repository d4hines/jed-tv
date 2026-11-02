#!/usr/bin/env bash
# Build the minimal QEMU-virt kernel: tinyconfig + common.config + <arch>.config.
# Runs on Linux with nix (we use eduardo-vm; macOS cannot build Linux easily).
#   tools/kernel/build.sh arm64|arm OUTDIR
# Source: nixpkgs#linux.src (6.18 LTS when written). Work tree: $KBUILD_WORK
# (default ~/Downloads/dande-tv-kernel). Output: OUTDIR/{Image|zImage,config}.
set -euo pipefail
ARCH=${1:?arm64 or arm}
OUT=$(realpath -m "${2:?output dir}")
HERE=$(cd "$(dirname "$0")" && pwd)
WORK=${KBUILD_WORK:-$HOME/Downloads/dande-tv-kernel}

case $ARCH in
  arm64) CROSS=aarch64-unknown-linux-gnu-
         SET=pkgsCross.aarch64-multiplatform.buildPackages; IMG=arch/arm64/boot/Image ;;
  arm)   CROSS=armv7l-unknown-linux-gnueabihf-
         SET=pkgsCross.armv7l-hf-multiplatform.buildPackages; IMG=arch/arm/boot/zImage ;;
  *) echo "unknown arch $ARCH" >&2; exit 2 ;;
esac

SRC_TAR=$(nix build --no-link --print-out-paths nixpkgs#linux.src)
SRC=$WORK/src
if [ ! -f "$SRC/Makefile" ]; then
  mkdir -p "$SRC"
  tar -xf "$SRC_TAR" -C "$SRC" --strip-components=1
fi
B=$WORK/build-$ARCH
mkdir -p "$B" "$OUT"

nix shell nixpkgs#gnumake nixpkgs#gcc nixpkgs#flex nixpkgs#bison nixpkgs#bc \
  nixpkgs#perl nixpkgs#python3 "nixpkgs#$SET.gcc" "nixpkgs#$SET.binutils" \
  -c bash -euo pipefail -c '
  cd "$0"
  K="make -s O=$1 ARCH=$2 CROSS_COMPILE=$3"
  $K tinyconfig
  ARCH=$2 scripts/kconfig/merge_config.sh -m -O "$1" "$1/.config" \
    "$4/common.config" "$4/$2.config" >/dev/null
  $K olddefconfig
  # Every requested option must survive olddefconfig (unmet deps drop silently).
  missing=0
  for frag in "$4/common.config" "$4/$2.config"; do
    while IFS= read -r line; do
      case $line in
        CONFIG_*=*) grep -qxF "$line" "$1/.config" || { echo "MISSING: $line"; missing=1; } ;;
        "# CONFIG_"*" is not set") opt=${line#"# "}; opt=${opt%% *}
          grep -q "^$opt=y" "$1/.config" && { echo "STILL SET: $opt"; missing=1; } ;;
      esac
    done < "$frag"
  done
  [ $missing = 0 ] || { echo "config check failed" >&2; exit 1; }
  $K -j"$(nproc)" "$(basename "$5")"
  if grep -q "^CONFIG_MODULES=y" "$1/.config"; then $K -j"$(nproc)" modules; fi
' "$SRC" "$B" "$ARCH" "$CROSS" "$HERE" "$IMG"

cp "$B/$IMG" "$OUT/"
cp "$B/.config" "$OUT/config"
rm -rf "$OUT/modules"
mkdir -p "$OUT/modules"
find "$B" -name "*.ko" -exec cp {} "$OUT/modules/" \;
ls -l "$OUT"
