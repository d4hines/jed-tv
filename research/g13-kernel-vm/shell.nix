let
  lock = builtins.fromJSON (builtins.readFile ../../flake.lock);
  pkgs = import (builtins.fetchTree lock.nodes.nixpkgs.locked) {
    system = builtins.currentSystem;
  };
in pkgs.mkShell {
  packages = with pkgs; [
    llvmPackages.clang-unwrapped llvmPackages.lld llvmPackages.llvm
    gnumake bc flex bison perl python3 openssl pkg-config dtc
    cpio gzip xz patch git curl unzip e2fsprogs android-tools
    glib pixman ninja meson python3Packages.packaging zlib libslirp
    checkpolicy libsepol rsync
  ];
}
