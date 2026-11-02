{ pkgs }:
pkgs.runCommand "pid1-lean-runtime-check" {
  nativeBuildInputs = [ pkgs.lean4 pkgs.python3 pkgs.stdenv.cc pkgs.binutils ];
  CC = "${pkgs.stdenv.cc}/bin/cc";
} ''
  mkdir -p work/src/pid1 work/test/lean
  cp -R ${pkgs.lib.cleanSource ../src/pid1/lean} work/src/pid1/lean
  cp -R ${pkgs.lib.cleanSource ../test/lean/tv} work/test/lean/tv
  chmod -R u+w work
  python3 work/test/lean/tv/check.py | tee result.txt
  python3 work/test/lean/tv/core-check.py | tee -a result.txt
  (cd work/src/pid1/lean; lake build TV.MenuView; lake env lean --run ../../../test/lean/tv/menu/Check.lean) | tee -a result.txt
  python3 work/test/lean/tv/check-volume.py | tee -a result.txt
  mkdir -p "$out"
  cp result.txt "$out/"
''
