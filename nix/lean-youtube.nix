{ pkgs }:
let
  source = pkgs.lib.cleanSourceWith {
    src = ../.;
    filter = path: type:
      let
        rel = pkgs.lib.removePrefix "${toString ../.}/" (toString path);
        trees = [ "src/lean/zr" "src/lean/yt" "tools/lean/zr" ];
      in toString path == toString ../. ||
        (builtins.any (tree: rel == tree || pkgs.lib.hasPrefix "${tree}/" rel ||
          pkgs.lib.hasPrefix "${rel}/" tree) trees &&
          !(builtins.elem (baseNameOf path) [ ".lake" "__pycache__" ]));
  };
in pkgs.stdenvNoCC.mkDerivation {
  pname = "lean-youtube";
  version = "0.1.0";
  src = source;
  nativeBuildInputs = [ pkgs.lean4 pkgs.zig pkgs.stdenv.cc ];
  strictDeps = true;
  dontConfigure = true;
  dontStrip = true;
  dontPatchELF = true;
  buildPhase = ''
    runHook preBuild
    export HOME="$TMPDIR/home"
    mkdir -p "$HOME"
    export ZIG_GLOBAL_CACHE_DIR="$TMPDIR/zig-cache"
    export ZIG_LOCAL_CACHE_DIR="$TMPDIR/zig-local"
    lake -d tools/lean/zr exe zr-build app src/lean/yt Yt.App.Main yt_app_main "$TMPDIR/ytplay-lean"
    runHook postBuild
  '';
  installPhase = ''
    runHook preInstall
    mkdir -p "$out/bin"
    cp "$TMPDIR/ytplay-lean" "$out/bin/ytplay-lean"
    runHook postInstall
  '';
  meta = {
    description = "Experimental standalone Lean YouTube app on the Zr ARM32 runtime";
    mainProgram = "ytplay-lean";
    platforms = pkgs.lib.platforms.unix;
  };
}
