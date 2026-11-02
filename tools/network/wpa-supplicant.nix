{ pkgs }:
pkgs.stdenvNoCC.mkDerivation {
  pname = "g06-wpa-supplicant";
  inherit (pkgs.wpa_supplicant) version src;
  nativeBuildInputs = [ pkgs.zig pkgs.gnumake ];
  buildPhase = ''
    runHook preBuild
    export ZIG_GLOBAL_CACHE_DIR="$TMPDIR/zig-global"
    export ZIG_LOCAL_CACHE_DIR="$TMPDIR/zig-local"
    cp ${./wpa_supplicant.config} wpa_supplicant/.config
    make -C wpa_supplicant -j$NIX_BUILD_CORES \
      CC="zig cc -target arm-linux-musleabihf -mcpu=cortex_a55 -isystem ${pkgs.linuxHeaders}/include" \
      LDFLAGS="-static -s" wpa_supplicant wpa_cli
    runHook postBuild
  '';
  installPhase = ''
    mkdir -p "$out/bin"
    cp wpa_supplicant/wpa_supplicant wpa_supplicant/wpa_cli "$out/bin/"
  '';
}
