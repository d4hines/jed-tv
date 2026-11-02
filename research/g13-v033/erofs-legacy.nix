# V033 uses the legacy LZ4 layout removed from erofs-utils 1.9.
# Keep this reader local to firmware research; never modify the input images.
let
  # Exclude ignored firmware/build assets from the flake source snapshot.
  pkgs = import (builtins.getFlake ("git+file://" + toString ../..)).inputs.nixpkgs {
    system = builtins.currentSystem;
  };
in pkgs.erofs-utils.overrideAttrs (_: {
  version = "1.8.10";
  src = pkgs.fetchurl {
    url = "https://github.com/erofs/erofs-utils/archive/refs/tags/v1.8.10.tar.gz";
    sha256 = "05eb4edebe11decce6ecb34e98d2f80c8cd283c2f2967d8ba7efd58418570514";
  };
})
