#!/usr/bin/env bash
# Cross-compile the Lean 4.35.0 runtime and its Init library for a static musl
# target with zig, so Lean programs (the Doom engine) run on the TV.
#
#   tools/lean-arm/build.sh TARGET OUT
#     TARGET  armv7-unknown-linux-musleabihf | aarch64-unknown-linux-musl
#     OUT     output directory: include/, libleanrt.a, libInit.a
#   LEAN_SRC  Lean 4 v4.35.0 source tree (only src/runtime and src/include are
#             read); default assets/third_party/lean4-v4.35.0/src
#   UV_SRC    libuv v1.48.0 source tree (the version Lean 4.35.0 pins);
#             default assets/third_party/libuv-v1.48.0/src
#
# Init's C is regenerated from the toolchain's own Init sources with the pinned
# `lean -c`, so it matches the .olean files our modules are compiled against.
# Differences from the official build: Lean's own small-object allocator
# instead of mimalloc, built-in bignums instead of GMP, no OpenSSL, and
# libuv only for the synchronous file helpers in io.cpp: the runtime's libuv
# networking and its event-loop thread are left out (stubbed below).
set -euo pipefail
target=${1:?target}; out=${2:?output directory}
root=$(cd "$(dirname "$0")/../.." && pwd)
export ZIG_GLOBAL_CACHE_DIR=${ZIG_GLOBAL_CACHE_DIR:-$root/assets/build/lean-arm/zig-global}
export ZIG_LOCAL_CACHE_DIR=${ZIG_LOCAL_CACHE_DIR:-$root/assets/build/lean-arm/zig-local}
src=${LEAN_SRC:-$root/assets/third_party/lean4-v4.35.0/src}
uv=${UV_SRC:-$root/assets/third_party/libuv-v1.48.0/src}
jobs=${JOBS:-$(getconf _NPROCESSORS_ONLN 2>/dev/null || echo 4)}
case $target in
  # The runtime's default 1 GiB thread stacks do not fit a 32-bit address space;
  # 8 MiB is what Lean itself uses on 32-bit WebAssembly.
  armv7-unknown-linux-musleabihf) zt="-target arm-linux-musleabihf -mcpu=cortex_a55"
    rtflags="-DLEAN_DEFAULT_THREAD_STACK_SIZE=8*1024*1024" ;;
  aarch64-unknown-linux-musl) zt="-target aarch64-linux-musl -mcpu=cortex_a55"
    rtflags="-DLEAN_DEFAULT_THREAD_STACK_SIZE=64*1024*1024" ;;
  *) echo "unsupported target $target" >&2; exit 2 ;;
esac
lean --version | grep -q 'version 4\.35\.0' || { echo "need Lean 4.35.0 on PATH" >&2; exit 2; }
prefix=$(lean --print-prefix)
test -f "$src/src/runtime/object.cpp" || { echo "missing Lean runtime sources at $src" >&2; exit 2; }
cmp -s "$src/src/include/lean/lean.h" "$prefix/include/lean/lean.h" ||
  { echo "lean.h in $src differs from the toolchain's" >&2; exit 2; }
grep -q 'UV_VERSION_MINOR 48$' "$uv/include/uv/version.h" || { echo "need libuv 1.48 at $uv" >&2; exit 2; }

mkdir -p "$out/include/lean" "$out/obj/rt" "$out/obj/init" "$out/obj/uv" "$out/initc"
out=$(cd "$out" && pwd)
# Keep unchanged generated inputs' mtimes so the object timestamp checks below
# remain useful when each native executable refreshes this shared runtime.
write_if_changed() {
  local destination=$1 temporary=$1.tmp.$$
  cat > "$temporary"
  if cmp -s "$temporary" "$destination"; then rm -f "$temporary"
  else mv -f "$temporary" "$destination"; fi
}
write_if_changed "$out/include/lean/lean.h" < "$prefix/include/lean/lean.h"
sed "s|LEAN_PLATFORM_TARGET .*|LEAN_PLATFORM_TARGET \"$target\"|" \
  "$prefix/include/lean/version.h" | write_if_changed "$out/include/lean/version.h"
printf '#pragma once\n#include <lean/version.h>\n#define LEAN_IS_STAGE0 0\n' | write_if_changed "$out/include/lean/config.h"
echo '#define LEAN_GITHASH "eb6cafb7290e8bd5f2937522d8d804f58f02bafc"' | write_if_changed "$out/include/githash.h"
write_if_changed "$out/nouv.cpp" <<'EOF'
// The runtime's libuv networking and OpenSSL are not linked; Init does not use them.
#include <lean/lean.h>
extern "C" void initialize_libuv() {}
extern "C" LEAN_EXPORT char **lean_setup_args(int, char **argv) { return argv; }
EOF

# 1. Init's C, generated in parallel (about a second per module).
if [ ! -f "$out/initc/.complete" ]; then
  (cd "$prefix/src/lean" && { echo Init.lean; find Init -name '*.lean'; }) | sort > "$out/initc/modules"
  xargs -P "$jobs" -n 1 sh -c '
    f=$2; c="$0/initc/${f%.lean}.c"; mkdir -p "$(dirname "$c")"
    lean --root="$1" -c "$c" "$1/$f" >/dev/null || { echo "lean -c failed: $f" >&2; exit 255; }
  ' "$out" "$prefix/src/lean" < "$out/initc/modules"
  touch "$out/initc/.complete"
fi

# 2. Compile.
common="$zt -O2 -DNDEBUG -fno-omit-frame-pointer -ffunction-sections -fdata-sections -I$out/include"
rt="debug thread mpz utf8 object apply exception interrupt memory stackinfo compact init_module io hash
    byteslice platform alloc allocprof sharecommon stack_overflow process object_ref mpn mutex"
# Random bytes via getrandom(2): confined apps and a bare PID 1 have no /dev/urandom.
{ echo '#include <sys/random.h>'
  sed -e 's|int fd_urandom = open("/dev/urandom", O_RDONLY \| O_CLOEXEC);|int fd_urandom = 0;|' \
      -e 's|read(fd_urandom, dst, read_sz)|getrandom(dst, read_sz, 0)|' \
      -e 's|close(fd_urandom);|(void)fd_urandom;|' "$src/src/runtime/io.cpp"; } | write_if_changed "$out/io.cpp"
grep -q 'getrandom(dst' "$out/io.cpp" && ! grep -q '/dev/urandom", O_RDONLY' "$out/io.cpp" ||
  { echo "io.cpp getrandom patch did not apply" >&2; exit 2; }
# String.toList ends its list with lean_box_uint32(0), which on 32-bit targets
# is a heap object instead of the scalar [] (upstream object.cpp,
# string_to_list_core). Compiled Lean matches still see [], but runtime
# consumers that test lean_is_scalar walk past the end: String.ofList of such
# a list crashed Lean Doom on ARM32 when a LAN player left.
sed -e 's|obj_res  r = lean_box_uint32(0);|obj_res  r = lean_box(0);|' "$src/src/runtime/object.cpp" | write_if_changed "$out/object.cpp"
grep -A5 'string_to_list_core(std::string' "$out/object.cpp" | grep -q 'r = lean_box(0);' ||
  { echo "object.cpp list terminator patch did not apply" >&2; exit 2; }
for f in $rt; do echo "$src/src/runtime/$f.cpp"; done |
  sed -e "s|$src/src/runtime/io.cpp|$out/io.cpp|" -e "s|$src/src/runtime/object.cpp|$out/object.cpp|" > "$out/obj/rt.list"
echo "$out/nouv.cpp" >> "$out/obj/rt.list"
xargs -P "$jobs" -n 1 sh -c '
  o="$0/obj/rt/$(basename "$4" .cpp).o"
  [ "$o" -nt "$4" ] || zig c++ $1 -std=c++20 -DLEAN_EXPORTING -DLEAN_MULTI_THREAD \
    -DLEAN_BUILD_TYPE=\"Release\" -ftls-model=local-exec -I"$2/src" -I"$3/include" -w -c "$4" -o "$o"
'  "$out" "$common $rtflags" "$src" "$uv" < "$out/obj/rt.list"
(cd "$out/initc" && find . -name '*.c' | sed 's|^\./||') > "$out/obj/init.list"
xargs -P "$jobs" -n 1 sh -c '
  o="$0/obj/init/$(echo "$2" | tr / _ | sed "s/\.c$/.o/")"
  [ "$o" -nt "$0/initc/$2" ] || zig cc $1 -ffp-contract=off -fwrapv -w -c "$0/initc/$2" -o "$o"
' "$out" "$common" < "$out/obj/init.list"

uvsrc="fs-poll idna inet random strscpy strtok thread-common threadpool timer uv-common
       uv-data-getter-setters version unix/async unix/core unix/dl unix/fs unix/getaddrinfo
       unix/getnameinfo unix/loop-watcher unix/loop unix/pipe unix/poll unix/process
       unix/random-devurandom unix/signal unix/stream unix/tcp unix/thread unix/tty unix/udp
       unix/linux unix/proctitle unix/procfs-exepath unix/random-getrandom unix/random-sysctl-linux"
for f in $uvsrc; do echo "$uv/src/$f.c"; done > "$out/obj/uv.list"
xargs -P "$jobs" -n 1 sh -c '
  o="$0/obj/uv/$(echo "$3" | sed "s|.*/src/||; s|/|_|g; s|\.c$|.o|")"
  [ "$o" -nt "$3" ] || zig cc $1 -D_GNU_SOURCE -D_POSIX_C_SOURCE=200112 -D_FILE_OFFSET_BITS=64 \
    -D_LARGEFILE_SOURCE -I"$2/include" -I"$2/src" -w -c "$3" -o "$o"
' "$out" "$common" "$uv" < "$out/obj/uv.list"

# 3. Archive (libuv objects go into libleanrt.a).
rm -f "$out/libleanrt.a" "$out/libInit.a"
zig ar rcs "$out/libleanrt.a" "$out"/obj/rt/*.o "$out"/obj/uv/*.o
zig ar rcs "$out/libInit.a" "$out"/obj/init/*.o
echo "lean-arm: $target runtime $(wc -c < "$out/libleanrt.a") bytes, Init $(wc -c < "$out/libInit.a") bytes ($(wc -l < "$out/obj/init.list") modules) in $out"
