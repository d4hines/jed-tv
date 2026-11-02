{ pkgs }:
# Static artwork for the offline RPG. The game engine is linked into rpgtv;
# this derivation builds no server or runtime executable.
pkgs.stdenv.mkDerivation {
  pname = "infinite-rpg-art";
  version = "0-unstable-2026-11-01";
  src = pkgs.fetchFromGitHub {
    owner = "agostbiro";
    repo = "vibecheck-hackathon";
    rev = "13a10c7dfb8b81eccadba36b7842170bcd949c69";
    hash = "sha256-JaRShsfY5qTa9CFYeaIIGImXMZNuS7RfZovsKHrrd40=";
  };
  nativeBuildInputs = [ pkgs.librsvg pkgs.ffmpeg-headless ];
  dontBuild = true;
  installPhase = ''
    mkdir -p "$out/pictures"
    for svg in web/pictures/*.svg; do
      name=$(basename "$svg" .svg)
      rsvg-convert -w 720 -h 240 -b '#000000' "$svg" \
        | ffmpeg -loglevel error -i - -f rawvideo -pix_fmt rgb24 "$out/pictures/$name.rgb"
    done
    rsvg-convert -w 486 -h 162 -b '#000000' web/pictures/shore.svg \
      | ffmpeg -loglevel error -i - -vf crop=288:162:150:0 -f rawvideo -pix_fmt rgb24 "$out/tile.rgb"
  '';
}
