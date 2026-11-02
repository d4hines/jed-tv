#!/usr/bin/env bash
# Cross-compile one executable of a Lake package for a static musl target.
#
#   tools/lean-arm/build-exe.sh TARGET PACKAGE_DIR EXE OUT_BINARY
#   EXTRA_C      native C files compiled for TARGET and linked in
#   EXTRA_CFLAGS flags for EXTRA_C (e.g. -I for headers)
#   EXTRA_LINK   further link inputs (static libraries, -l flags)
#
# Lake (host) generates C for every module; zig compiles that C for TARGET and
# links it with the incrementally refreshed local runtime. LEAN_ARM may instead
# supply a complete prebuilt runtime (for example an immutable Nix package).
set -euo pipefail
target=${1:?target}; pkg=$(cd "${2:?package dir}" && pwd); exe=${3:?exe}; bin=${4:?output}
root=$(cd "$(dirname "$0")/../.." && pwd)
export ZIG_GLOBAL_CACHE_DIR=${ZIG_GLOBAL_CACHE_DIR:-$root/assets/build/lean-arm/zig-global}
export ZIG_LOCAL_CACHE_DIR=${ZIG_LOCAL_CACHE_DIR:-$root/assets/build/lean-arm/zig-local}
rt=${LEAN_ARM:-$root/assets/build/lean-arm/$target}
jobs=${JOBS:-$(getconf _NPROCESSORS_ONLN 2>/dev/null || echo 4)}
case $target in
  armv7-unknown-linux-musleabihf) zt="-target arm-linux-musleabihf -mcpu=cortex_a55" ;;
  aarch64-unknown-linux-musl) zt="-target aarch64-linux-musl -mcpu=cortex_a55" ;;
  *) echo "unsupported target $target" >&2; exit 2 ;;
esac
if [ -z "${LEAN_ARM:-}" ]; then
  "$root/tools/lean-arm/build.sh" "$target" "$rt"
fi
for input in libleanrt.a libInit.a include/lean/lean.h; do
  test -f "$rt/$input" || { echo "incomplete Lean runtime: $rt/$input" >&2; exit 2; }
done
root_dotted=$(python3 "$root/tools/lean-arm/exe-inputs.py" "$pkg" "$exe" --root)
# Only the C is needed: build the root module (and its imports), not a host binary.
build=$(python3 "$root/tools/lean/build_paths.py" "$pkg")
mkdir -p "$build"
(cd "$pkg" && lake build "$root_dotted" > "$build/build-exe.log" 2>&1) ||
  { cat "$build/build-exe.log" >&2; echo "lake build $root_dotted failed" >&2; exit 1; }
obj="$build/$target/obj"
mkdir -p "$obj"
python3 "$root/tools/lean-arm/exe-inputs.py" "$pkg" "$exe" > "$obj/list"
# (absolute paths, the package's or a path dependency's; objects are named by
# the module path below ir, unique because module names are)
export OBJ="$obj" ZT="$zt" RT="$rt"
xargs -P "$jobs" -n 1 sh -c '
  o="$OBJ/$(echo "${1#*/ir/}" | tr / _ | sed "s/\.c$/.o/")"
  [ "$o" -nt "$1" ] || zig cc $ZT -O3 -DNDEBUG -ffunction-sections -fdata-sections \
    -ffp-contract=off -fwrapv -w -I"$RT/include" -c "$1" -o "$o"
' _ < "$obj/list"
mkdir -p "$(dirname "$bin")"
# Use the same manifest for compilation and linking; stale objects from other
# executables or previously removed modules must never enter this program.
objs=""
while IFS= read -r source; do
  name=$(echo "${source#*/ir/}" | tr / _ | sed 's/\.c$/.o/')
  objs="$objs $obj/$name"
done < "$obj/list"
mkdir -p "$obj/native"
for c in ${EXTRA_C:-}; do
  o="$obj/native/$(basename "$c" .c).o"
  zig cc $zt -O2 -DNDEBUG -ffunction-sections -fdata-sections -I"$rt/include" ${EXTRA_CFLAGS:-} -c "$c" -o "$o"
  objs="$objs $o"
done
zig c++ $zt -static -s -Wl,--gc-sections $objs "$rt/libInit.a" "$rt/libleanrt.a" ${EXTRA_LINK:-} -o "$bin"
echo "built $bin ($(wc -c < "$bin") bytes)"
