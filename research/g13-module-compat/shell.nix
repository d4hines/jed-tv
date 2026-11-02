let
  lock = builtins.fromJSON (builtins.readFile ../../flake.lock);
  pkgs = import (builtins.fetchTree lock.nodes.nixpkgs.locked) {
    system = builtins.currentSystem;
  };
in pkgs.mkShell {
  packages = with pkgs; [
    (python3.withPackages (p: [ p.pyelftools ]))
    llvmPackages.clang-unwrapped llvmPackages.lld llvmPackages.llvm
    gnumake bc flex bison perl openssl pkg-config dtc
    cpio gzip xz patch git curl unzip e2fsprogs
  ];
}
