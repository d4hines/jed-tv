let
  # Keep ignored firmware/build artifacts out of Nix's source snapshot.
  flake = builtins.getFlake ("git+file://" + toString ../..);
  pkgs = import flake.inputs.nixpkgs { system = builtins.currentSystem; };
in pkgs.mkShell {
  packages = with pkgs; [
    llvmPackages.clang-unwrapped llvmPackages.lld llvmPackages.llvm
    gnumake bc flex bison perl python3
    openssl pkg-config dtc cpio gzip xz patch
    glib pixman ninja meson python3Packages.packaging zlib libslirp
  ];
}
