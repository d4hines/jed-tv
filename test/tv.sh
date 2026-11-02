#!/usr/bin/env bash
# Boot the whole TV in QEMU: tvinit (home screen, PID 1) + /bin/ytplay +
# /bin/doomtv + Freedoom + tile art. Extra arguments go to test/run.py.
#   test/tv.sh                 # exact G06 kernel; serial, peripherals still incomplete
#   test/tv.sh virt32          # surrogate media window + CoreAudio
#   test/tv.sh fast --headless --at 3:shot:home ...
#   test/tv.sh fast            # accelerated arm64 development loop
# Start the replay server first for the offline video (Coffee Run):
#   python3 test/replay.py --player ... --fixtures ...
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/.." && pwd)
PROFILE=faithful
if [ $# -gt 0 ] && [[ $1 != -* ]]; then PROFILE=$1; shift; fi
case $PROFILE in
  fast) T=aarch64-unknown-linux-musl ;;
  faithful|virt32) T=armv7-unknown-linux-musleabihf ;;
  *) echo "profile: faithful (stock), virt32 (surrogate), or fast" >&2; exit 2 ;;
esac
(cd "$ROOT/src" && cargo build --release --target $T -p platform -p tvinit -p ytplay)
[ -f "$ROOT/assets/build/tiles/doom.rgb" ] || python3 "$ROOT/tools/make_tiles.py"
R=$ROOT/assets/build/app-target/$T/release
PLATFORM_LIB="$R/libplatform.a" bash "$ROOT/tools/doomtv/build.sh" $T "$R/doomtv"
PLATFORM_LIB="$R/libplatform.a" bash "$ROOT/tools/slides/build.sh" $T "$R/slides"
FILES=(--add-file "$R/ytplay:/bin/ytplay" --add-file "$R/doomtv:/bin/doomtv"
       --add-file "$R/slides:/bin/slides")
WAD=$ROOT/assets/third_party/freedoom/freedoom1.wad
[ -f "$WAD" ] && FILES+=(--add-file "$WAD:/doom/freedoom1.wad")
for t in "$ROOT"/assets/build/tiles/*.rgb; do FILES+=(--add-file "$t:/share/tiles/$(basename "$t")"); done
# Local time for the clock: minutes east of UTC.
OFF=$(( $(date +%z | sed -E 's/^([+-])0?([0-9]{1,2})([0-9]{2})$/\1(\2*60+\3)/') ))
exec python3 "$ROOT/test/run.py" "$PROFILE" --bin tvinit --no-build "${FILES[@]}" \
  --extra "tvinit.utc_offset=$OFF" --timeout 86400 "$@"
