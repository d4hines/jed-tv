{ pkgs, apps, leanArm }:
# Axiom: First Light (github.com/petergpt/lean-3d-game-axiom), a hackathon
# project: a voxel game written in Lean 4, running on the TV itself. The Lean
# game (src/axiomtv/lean) is compiled to C, cross-compiled with zig for 32-bit
# ARM and linked with the Lean runtime from `leanArm` (demo/package.nix,
# tools/lean-arm) and the TV platform layer libaxiomgfx.a (src/axiomtv/gfx,
# built with the other apps). The upstream acceptance tests run on the build
# host first.
# Output: bin/axiomtv.
let
  lib = pkgs.lib;
  target = "armv7-unknown-linux-musleabihf";
in pkgs.stdenv.mkDerivation {
  pname = "axiomtv";
  version = "0.1.0";
  # The game and the cross-build scripts, which look for each other by path.
  src = lib.cleanSourceWith { src = ../.; filter = path: type:
    let rel = lib.removePrefix "${toString ../.}/" (toString path);
    in builtins.elem rel [ "src" "src/axiomtv" "tools" "tools/lean-arm" "tools/lean" ]
       || lib.hasPrefix "src/axiomtv/" rel || lib.hasPrefix "tools/lean-arm/" rel
       || lib.hasPrefix "tools/lean/" rel; };
  nativeBuildInputs = [ pkgs.lean4 pkgs.zig pkgs.findutils pkgs.python3 ];
  buildPhase = ''
    export HOME="$TMPDIR/home"; mkdir -p "$HOME"
    export ZIG_GLOBAL_CACHE_DIR="$TMPDIR/zig-cache" ZIG_LOCAL_CACHE_DIR="$TMPDIR/zig-local"
    export JOBS="$NIX_BUILD_CORES"
    (cd src/axiomtv/lean && lake build tests && .lake/build/bin/tests)
    mkdir -p "$out/bin"
    EXTRA_C="$PWD/src/axiomtv/native/lean_glue.c" \
    EXTRA_LINK="${apps}/libaxiomgfx.a -lpthread -ldl -lm" \
    LEAN_ARM=${leanArm} \
      bash tools/lean-arm/build-exe.sh ${target} src/axiomtv/lean axiomtv "$out/bin/axiomtv"
  '';
  dontInstall = true;
  dontFixup = true;
}
