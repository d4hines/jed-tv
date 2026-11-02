{ pkgs, checkedLean, appUidBase ? 10000 }:
assert builtins.isInt appUidBase && appUidBase >= 10000 && appUidBase <= 19993;
let
  inherit (pkgs) lib;
  target = "armv7-unknown-linux-musleabihf";
  hosts = {
    aarch64-darwin = "aarch64-apple-darwin";
    x86_64-darwin = "x86_64-apple-darwin";
    aarch64-linux = "aarch64-unknown-linux-gnu";
    x86_64-linux = "x86_64-unknown-linux-gnu";
  };
  pins = builtins.fromJSON (builtins.readFile ./rust-toolchain.json);
  host = hosts.${pkgs.stdenv.hostPlatform.system};
  downloads = (builtins.attrValues pins.${host}) ++ [ pins.${target}.rust-std ];
  toolchain = pkgs.stdenvNoCC.mkDerivation {
    name = "pid1-rust-1.92.0";
    archives = map (p: pkgs.fetchurl { inherit (p) url; sha256 = p.hash; }) downloads;
    nativeBuildInputs = [ pkgs.xz ] ++ lib.optionals pkgs.stdenv.hostPlatform.isLinux [ pkgs.autoPatchelfHook ];
    buildInputs = lib.optionals pkgs.stdenv.hostPlatform.isLinux [ pkgs.stdenv.cc.cc.lib pkgs.zlib pkgs.curl pkgs.openssl ];
    dontUnpack = true;
    dontConfigure = true;
    dontBuild = true;
    installPhase = ''
      mkdir -p "$out"
      for archive in $archives; do
        mkdir unpack
        tar -xf "$archive" -C unpack --strip-components=1
        bash unpack/install.sh --prefix="$out" --disable-ldconfig
        rm -rf unpack
      done
    '';
  };
  androidToolchain = toolchain.overrideAttrs (old: {
    name = "pid1-rust-1.92.0-android";
    archives = old.archives ++ [ (pkgs.fetchurl {
      url = "https://static.rust-lang.org/dist/2025-12-11/rust-std-1.92.0-armv7-linux-androideabi.tar.xz";
      sha256 = "06ac2f08dcf5c480e7767c0541c9bd7460754ec3a369a4b7097b328eca28fab9";
    }) ];
  });
  source = lib.cleanSourceWith {
    src = ../.;
    filter = path: type:
      let rel = lib.removePrefix "${toString ../.}/" (toString path);
      in (toString path == toString ../.) ||
        (builtins.elem (builtins.head (lib.splitString "/" rel)) [ "src" "tools" "test" "demo" ]
         && !(builtins.elem (baseNameOf path) [ ".lake" "__pycache__" ]));
  };
  deps = pkgs.rustPlatform.fetchCargoVendor {
    src = source;
    cargoRoot = "src";
    hash = "sha256-zNSCqBNKL5RGEtVairHcd0qukjaSjFJ5DCbmyCkAdBE=";
  };
  # Lean 4.35.0's runtime and Init cross-built for the TV (tools/lean-arm).
  # libuv is the release Lean pins (nixpkgs has a newer one).
  libuv148 = pkgs.fetchFromGitHub {
    owner = "libuv"; repo = "libuv"; rev = "v1.48.0";
    hash = "sha256-U68BmIQNpmIy3prS7LkYl+wvDJQNikoeFiKh50yQFoA=";
  };
  leanArm = pkgs.stdenvNoCC.mkDerivation {
    name = "lean-4.35.0-runtime-${target}";
    src = lib.cleanSourceWith { src = ../.; filter = path: type:
      let rel = lib.removePrefix "${toString ../.}/" (toString path);
      in rel == "tools" || rel == "tools/lean-arm" || lib.hasPrefix "tools/lean-arm/" rel; };
    nativeBuildInputs = [ pkgs.lean4 pkgs.zig ];
    dontConfigure = true;
    dontStrip = true;
    buildPhase = ''
      export ZIG_GLOBAL_CACHE_DIR="$TMPDIR/zig-cache" ZIG_LOCAL_CACHE_DIR="$TMPDIR/zig-local"
      LEAN_SRC=${pkgs.lean4.src} UV_SRC=${libuv148} bash tools/lean-arm/build.sh ${target} "$TMPDIR/rt"
    '';
    installPhase = ''
      mkdir -p "$out"
      cp -R "$TMPDIR/rt/include" "$TMPDIR/rt/libleanrt.a" "$TMPDIR/rt/libInit.a" "$out/"
    '';
  };
  apps = pkgs.stdenvNoCC.mkDerivation {
    pname = "pid1-demo-apps";
    version = "0.1.0";
    src = source;
    cargoDeps = deps;
    cargoRoot = "src";
    dontStrip = true; # Cargo already strips; these are foreign static ELF binaries.
    dontPatchELF = true;
    nativeBuildInputs = [ toolchain pkgs.rustPlatform.cargoSetupHook pkgs.zig pkgs.ffmpeg-headless pkgs.pkg-config pkgs.lean4 pkgs.python3 pkgs.stdenv.cc ];
    dontConfigure = true;
    postPatch = ''
      patchShebangs tools/pid1/linker-armv7 tools/pid1/linker-aarch64
    '';
    buildPhase = ''
      test -f ${checkedLean}/result.txt
      # Android hosting needs identities that do not belong to installed APKs.
      # Keep the same fixed Lean policy and capabilities with a host-specific base.
      substituteInPlace src/pid1/lean/TV/Apps.lean \
        --replace-fail 'then 10000 + app else 0' 'then ${toString appUidBase} + app else 0'
      substituteInPlace src/pid1/lean/TV/Core.lean \
        --replace-fail 'def uidBase : Nat := 10000' 'def uidBase : Nat := ${toString appUidBase}'
      export HOME="$TMPDIR/home"
      mkdir -p "$HOME"
      export ZIG_GLOBAL_CACHE_DIR="$TMPDIR/zig-cache"
      export ZIG_LOCAL_CACHE_DIR="$TMPDIR/zig-local"
      export CARGO_TARGET_DIR="$TMPDIR/target"
      export LEAN_ARM=${leanArm}
      export DOOM_BUILD_ROOT="$TMPDIR/doom"
      export DOOMGENERIC_DIR="$DOOM_BUILD_ROOT/${target}"
      bash tools/doom/build.sh ${target}
      cd src
      cargo build --offline --locked --release --target ${target} -p doomtv -p platform -p tvinit -p ytplay -p tvosd -p rpgtv -p axiomgfx -p snaketv
      # The city flight demo carries the Lean runtime (C++), so zig c++ links it
      # with its own musl and libc++ instead of rust-lld.
      CITYJSB_DIR=${flight} \
      CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_LINKER="$PWD/../tools/toolchain/zcxx-${target}" \
      CARGO_TARGET_ARMV7_UNKNOWN_LINUX_MUSLEABIHF_RUSTFLAGS="-C target-cpu=cortex-a55 -C link-self-contained=no" \
        cargo build --offline --locked --release --target ${target} -p flighttv
      LEAN_ARM=${leanArm} PLATFORM_LIB="$CARGO_TARGET_DIR/${target}/release/libplatform.a" \
        bash ../tools/slides/build.sh ${target} "$CARGO_TARGET_DIR/${target}/release/slides"
      rustc --edition=2021 --target ${target} -C linker=rust-lld -C target-cpu=cortex-a55 \
        -C panic=abort -C opt-level=2 ../test/fixtures/tvinit-child.rs -o "$CARGO_TARGET_DIR/uncooperative"
      for archive in "$CARGO_TARGET_DIR/${target}/release/build"/tvinit-*/out/libtv_control_lean.a; do
        rustc --edition=2021 --target ${target} -C linker=rust-lld -C target-cpu=cortex-a55 \
          -C panic=abort -C opt-level=2 -L "native=$(dirname "$archive")" -l static=tv_control_lean \
          ../test/fixtures/confinement-init.rs -o "$CARGO_TARGET_DIR/confinement-init"
      done
    '';
    installPhase = ''
      mkdir -p "$out/bin"
      cp "$CARGO_TARGET_DIR/${target}/release/"{tvinit,bridge-init,ytplay,doomtv,slides,tvosd,rpgtv,flighttv,snaketv,contract-probe} "$out/bin/"
      cp "$CARGO_TARGET_DIR/${target}/release/"{libplatform.a,libaxiomgfx.a} "$out/"
      mkdir -p "$out/include"
      cp platform/include/tv_app_v1.h "$out/include/"
      cp "$CARGO_TARGET_DIR/"{uncooperative,confinement-init} "$out/bin/"
      echo '${builtins.toJSON (map (id: appUidBase + id) [ 1 2 3 5 6 7 8 ])}' > "$out/app-uids.json"
    '';
  };
  freedoomZip = pkgs.fetchurl {
    url = "https://github.com/freedoom/freedoom/releases/download/v0.13.0/freedoom-0.13.0.zip";
    sha256 = "3f9b264f3e3ce503b4fb7f6bdcb1f419d93c7b546f4df3e874dd878db9688f59";
  };
  media = pkgs.runCommand "pid1-demo-media" {
    nativeBuildInputs = [ pkgs.ffmpeg-headless pkgs.python3 pkgs.unzip pkgs.xz ];
  } ''
    mkdir -p "$out/fixtures" "$out/doom" "$out/kernel"
    xz -dc ${./kernel/zImage.xz} > "$out/kernel/zImage"
    echo '996d7306ab2e13e61c37636903378c3ce37caaa9e7605e640693015f1995784d  '"$out/kernel/zImage" | sha256sum -c -
    unzip -q ${freedoomZip} -d wad
    cp wad/freedoom-0.13.0/freedoom1.wad "$out/doom/"
    cp wad/freedoom-0.13.0/COPYING.txt "$out/doom/"
    python3 ${./make-fixtures.py} "$out"
    python3 ${../tools/make-menu-tiles.py} "$out/tiles" --source-dir ${./tiles}
    ffmpeg -loglevel error -i ${./flight-tile.png} -vf scale=288:162 -f rawvideo -pix_fmt rgb24 "$out/tiles/flight.rgb"
  '';
  launcher = pkgs.writeShellApplication {
    name = "pid1";
    runtimeInputs = [ (pkgs.python3.withPackages (ps: [ ps.tkinter ])) pkgs.qemu ];
    text = ''
      exec python3 ${./run.py} --apps ${apps} --media ${media} --source ${source} --rpg ${rpg} --axiom ${axiom} "$@"
    '';
  };
  rpg = import ./rpg.nix { inherit pkgs; };
  flight = import ./flight.nix { inherit pkgs leanArm; };
  axiom = import ./axiom.nix { inherit pkgs apps leanArm; };
in launcher // { passthru = { inherit apps media toolchain leanArm androidToolchain rpg flight axiom; }; }
