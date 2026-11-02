{
  description = "TV firmware tools, graphical PID1 demo and Lean userspace";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

  outputs = { self, nixpkgs }:
    let
      systems = [ "aarch64-darwin" "x86_64-darwin" "aarch64-linux" "x86_64-linux" ];
      eachSystem = nixpkgs.lib.genAttrs systems;
    in {
      packages = eachSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            config.allowUnfreePredicate = pkg: builtins.elem (nixpkgs.lib.getName pkg) [ "boot.img" "g06-doom-runtime.tar" "g06-hello-pid1-kit.tar" "g06-launcher-runtime.tar" ];
          };
          originalBoot = pkgs.requireFile {
            name = "boot.img";
            sha256 = "d116dfa370d8e5dd5809f6f65266c86671b5e9bec9d62f95c0977bca0ff30085";
            message = ''
              Import this TV's original G06 V109 boot image once, from the repo root:
                nix-store --add-fixed sha256 assets/backups/v109-stock-20261031/boot.img
              Then run: nix build .#stock-boot
            '';
          };
          runtimePin = builtins.fromJSON (builtins.readFile ./tools/legacy-doom/runtime.json);
          doomRuntime = pkgs.requireFile {
            inherit (runtimePin) name sha256;
            message = ''
              Prepare the local Doom runtime with tools/legacy-doom/build.sh, or import its preserved artifact:
                nix-store --add-fixed sha256 assets/build/doom-minimal/g06-doom-runtime.tar
            '';
          };
          helloPid1Pin = builtins.fromJSON (builtins.readFile ./tools/pid1/runtime.json);
          helloPid1Kit = pkgs.requireFile {
            inherit (helloPid1Pin) name sha256;
            message = ''
              Prepare this TV's local RAM-boot kit with tools/pid1/build.sh, or import it:
                nix-store --add-fixed sha256 assets/build/hello-pid1/g06-hello-pid1-kit.tar
            '';
          };
          r8 = pkgs.fetchurl {
            url = "https://dl.google.com/dl/android/maven2/com/android/tools/r8/8.3.37/r8-8.3.37.jar";
            sha256 = "59753e70a74f918389cc87f1b7d66b5c0862932559167425708ded159e3de439";
          };
          magiskApk = pkgs.fetchurl {
            url = "https://github.com/topjohnwu/Magisk/releases/download/v28.1/Magisk-v28.1.apk";
            sha256 = "8bfd3346b3da5814f82eff6f1b1b5fedd0ad585f39a25709b23eb54aac45691d";
          };
        in {
          lean-youtube = import ./nix/lean-youtube.nix { inherit pkgs; };
          pid1 = import ./demo/package.nix { inherit pkgs; checkedLean = self.checks.${system}.lean; };
          wifi-supplicant = import ./tools/network/wpa-supplicant.nix {
            pkgs = import nixpkgs {
              system = if pkgs.stdenv.hostPlatform.isDarwin then "aarch64-linux" else system;
            };
          };
          g13-apps = let
            bundle = (import ./demo/package.nix {
              inherit pkgs;
              checkedLean = self.checks.${system}.lean;
              appUidBase = 19000;
            }).passthru;
          in pkgs.symlinkJoin {
            name = "g13-native-apps";
            paths = [ bundle.apps bundle.axiom ];
            postBuild = ''
              mkdir -p "$out/share/tiles" "$out/share/rpg"
              cp ${bundle.rpg}/tile.rgb "$out/share/tiles/rpg.rgb"
              cp ${bundle.rpg}/pictures/*.rgb "$out/share/rpg/"
            '';
          };
          native-app-tools = pkgs.symlinkJoin {
            name = "tv-native-app-build-tools";
            paths = [ self.packages.${system}.pid1.passthru.androidToolchain
              pkgs.llvmPackages.clang-unwrapped pkgs.llvmPackages.bintools ];
          };
          lean-check = pkgs.writeShellApplication {
            name = "lean-check";
            runtimeInputs = [ pkgs.coreutils ];
            text = "cat ${self.checks.${system}.lean}/result.txt";
          };
          stock-plus-loader = import ./tools/launcher/package.nix {
            inherit pkgs originalBoot;
            stockPlusRoot = self.packages.${system}.stock-plus-root;
            helloKit = self.packages.${system}.hello-pid1-kit;
          };
          stock-plus-root = pkgs.runCommand "g06-v109-stock-plus-root" {
            nativeBuildInputs = [ pkgs.python3 pkgs.android-tools ];
          } ''
            python3 ${./tools/make-stock-plus-root.py} ${originalBoot} ${magiskApk} ${./src/runtime-scripts/root/adb-root.rc} ${./src/runtime-scripts/root/adb-root.sh} "$out"
          '';
          hello-pid1-kit = pkgs.runCommand "g06-hello-pid1-kit" { } ''
            mkdir -p "$out"
            tar -xf ${helloPid1Kit} -C "$out" init kernel dtb initramfs.cpio.gz cmdline.txt manifest.json
          '';
          hello-pid1-boot = pkgs.runCommand "g06-hello-pid1-boot" {
            nativeBuildInputs = [ pkgs.python3 pkgs.android-tools ];
          } ''
            python3 ${./tools/make-hello-pid1-boot.py} ${originalBoot} ${self.packages.${system}.hello-pid1-kit} "$out"
          '';
          hello-screen = pkgs.runCommand "g06-hello-screen" {
            nativeBuildInputs = [ pkgs.jdk21_headless ];
          } ''
            mkdir -p classes "$out"
            cp ${./research/hello/HelloScreen.java} HelloScreen.java
            cp ${./research/hello/AudioTone.java} AudioTone.java
            javac --release 8 -Xlint:-options -d classes HelloScreen.java AudioTone.java
            java -cp ${r8} com.android.tools.r8.D8 --min-api 30 --output "$out/hello.jar" classes/*.class
          '';
          boot-probe = pkgs.runCommand "g06-boot-probe" {
            nativeBuildInputs = [ pkgs.jdk21_headless ];
          } ''
            mkdir -p classes "$out"
            cp ${./research/hello/FactoryBootProbe.java} FactoryBootProbe.java
            javac --release 8 -Xlint:-options -d classes FactoryBootProbe.java
            java -cp ${r8} com.android.tools.r8.D8 --min-api 30 --output "$out/probe.jar" classes/*.class
          '';
          hello = pkgs.writeShellApplication {
            name = "tv-hello";
            runtimeInputs = [ pkgs.android-tools pkgs.python3 ];
            text = ''
              exec python3 ${./tools/hello.py} ${self.packages.${system}.hello-screen}/hello.jar "$@"
            '';
          };
          stock-boot = pkgs.runCommand "g06-v109-stock-boot" { } ''
            mkdir -p "$out"
            cp ${originalBoot} "$out/boot.img"
          '';
          poc-boot = pkgs.runCommand "g06-v109-poc-boot" {
            nativeBuildInputs = [ pkgs.python3 pkgs.android-tools ];
          } ''
            mkdir -p "$out"
            python3 ${./tools/make-boot-poc.py} ${originalBoot} "$out/boot.img" > "$out/change.json"
            avbtool verify_image --image "$out/boot.img"
            avbtool info_image --image "$out/boot.img" > "$out/avb-info.txt"
          '';
          doom-boot = pkgs.runCommand "g06-v109-doom-boot" {
            nativeBuildInputs = [ pkgs.python3 pkgs.android-tools ];
          } ''
            python3 ${./tools/make-doom-boot.py} ${originalBoot} ${doomRuntime} "$out"
          '';
          flash = pkgs.writeShellApplication {
            name = "tv-flash";
            runtimeInputs = [ pkgs.python3 pkgs.android-tools ];
            text = ''
              exec python3 ${./tools/flash.py} "$@"
            '';
          };
        });
      checks = eachSystem (system:
        let pkgs = import nixpkgs { inherit system; };
        in { lean = import ./nix/lean-check.nix { inherit pkgs; }; });
      apps = eachSystem (system: {
        lean-check = {
          type = "app";
          program = "${self.packages.${system}.lean-check}/bin/lean-check";
        };
        pid1 = {
          type = "app";
          program = "${self.packages.${system}.pid1}/bin/pid1";
        };
        hello = {
          type = "app";
          program = "${self.packages.${system}.hello}/bin/tv-hello";
        };
        flash = {
          type = "app";
          program = "${self.packages.${system}.flash}/bin/tv-flash";
        };
      });
      devShells = eachSystem (system:
        let pkgs = import nixpkgs { inherit system; };
        in {
          default = pkgs.mkShellNoCC {
            packages = [ pkgs.android-tools pkgs.e2fsprogs pkgs.python3 ];
          };
          bootloader = pkgs.mkShellNoCC {
            packages = [ pkgs.lean4 pkgs.llvmPackages.clang-unwrapped
              pkgs.llvmPackages.lld pkgs.llvmPackages.llvm pkgs.qemu pkgs.dtc pkgs.lrzsz pkgs.openssl pkgs.erofs-utils
              (pkgs.python3.withPackages (p: [ p.capstone p.unicorn p.pyelftools p.lz4 ])) ];
          };
        });
    };
}
