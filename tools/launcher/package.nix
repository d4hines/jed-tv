{ pkgs, originalBoot, stockPlusRoot, helloKit }:
let
  pin = builtins.fromJSON (builtins.readFile ./runtime.json);
  runtime = pkgs.requireFile {
    inherit (pin) name sha256;
    message = ''
      Build/import the local production launcher once from this repository:
        ./launcher/build.sh
      Or import the preserved artifact:
        nix-store --add-fixed sha256 assets/build/launcher/g06-launcher-runtime.tar
    '';
  };
in pkgs.runCommand "g06-v109-stock-plus-loader" {
  nativeBuildInputs = [ pkgs.python3 pkgs.android-tools ];
} ''
  python3 ${../make-stock-plus-loader.py} ${originalBoot} ${stockPlusRoot} ${helloKit} ${runtime} ${../make-stock-plus-root.py} "$out"
''
