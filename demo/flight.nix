{ pkgs, leanArm }:
# City flight demo (github.com/uyhcire/vibecheck-tsa, branch share/smart-tv-starter),
# a hackathon project that runs on the TV itself. Its aircraft dynamics are
# JsbExact, a Lean port of JSBSim, built here for 32-bit ARM: Lean emits C,
# zig compiles that C for the TV, and the result is a static library behind
# the project's own C API (c/cityjsb.h), linked with the Lean runtime from
# `leanArm` (demo/package.nix, tools/lean-arm). The Lean, C and Rust sources
# are vendored under src/third_party/cityflight.
# Output: lib/ with libcityjsb.a, libInit.a, libleanrt.a and zig's C++ runtime
# archives for the target (the Lean runtime is C++); include/cityjsb.h.
let
  zt = "-target arm-linux-musleabihf -mcpu=cortex_a55";
  # zig's libc++, libc++abi and libunwind for the target, so a Rust link can carry the Lean runtime.
  cxx = pkgs.stdenvNoCC.mkDerivation {
    name = "zig-cxx-runtime-armv7-musl";
    dontUnpack = true;
    nativeBuildInputs = [ pkgs.zig pkgs.findutils ];
    buildPhase = ''
      export ZIG_GLOBAL_CACHE_DIR="$TMPDIR/zig-cache" ZIG_LOCAL_CACHE_DIR="$TMPDIR/zig-local"
      echo 'int main(void) { return 0; }' > probe.cpp
      zig c++ ${zt} -static probe.cpp -o probe
      mkdir -p "$out"
      for lib in libc++ libc++abi libunwind; do
        cp "$(find "$ZIG_GLOBAL_CACHE_DIR" -name "$lib.a" | head -n 1)" "$out/"
      done
    '';
    dontInstall = true;
  };
in pkgs.stdenvNoCC.mkDerivation {
  pname = "cityflight-jsb";
  version = "c8f5e98";
  src = pkgs.lib.cleanSource ../src/third_party/cityflight;
  nativeBuildInputs = [ pkgs.lean4 pkgs.zig pkgs.findutils ];
  buildPhase = ''
    export HOME="$TMPDIR/home"; mkdir -p "$HOME"
    export ZIG_GLOBAL_CACHE_DIR="$TMPDIR/zig-cache" ZIG_LOCAL_CACHE_DIR="$TMPDIR/zig-local"
    substituteInPlace lean/lakefile.lean \
      --replace-fail "../../../../assets/build/lake/cityflight" "../build"
    # This isolated source archive uses build/ outside its Lean source tree.
    (cd lean && lake build JsbExact CityJsb)
    flags="-O3 -DNDEBUG -ffunction-sections -fdata-sections -ffp-contract=off -fwrapv -w -I${leanArm}/include"
    mkdir -p obj
    (cd build/ir && find JsbExact CityJsb -name '*.c') > obj/list
    while read -r c; do
      echo "zig cc ${zt} $flags -c build/ir/$c -o obj/$(echo "$c" | tr / _ | sed 's/\.c$/.o/')"
    done < obj/list | xargs -P "$NIX_BUILD_CORES" -I{} sh -c '{}'
    # The project's C glue. Their jxmath.c uses Darwin's combined sin/cos; on Linux these are sin and cos.
    cat > obj/jxmath.c <<'EOF'
#include <math.h>
double jx_sin(double x) { return sin(x); }
double jx_cos(double x) { return cos(x); }
double jx_sincos_sin(double x) { return sin(x); }
double jx_sincos_cos(double x) { return cos(x); }
EOF
    zig cc ${zt} -O2 -fno-builtin -w -c obj/jxmath.c -o obj/jxmath.o
    zig cc ${zt} -O2 -w -I${leanArm}/include -Ic -c c/cityjsb_abi.c -o obj/cityjsb_abi.o
    zig ar rcs libcityjsb.a obj/*.o
  '';
  installPhase = ''
    mkdir -p "$out/lib" "$out/include"
    cp libcityjsb.a ${leanArm}/libInit.a ${leanArm}/libleanrt.a ${cxx}/*.a "$out/lib/"
    cp c/cityjsb.h "$out/include/"
  '';
}
