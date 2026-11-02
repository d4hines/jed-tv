The normal TV and QEMU builds currently use the previous Rust/C Doom app,
with sound effects. Its music backend is silent. The Lean version is preserved
below for later work; its audio delivery still needs repair on the TV.

# Retained Lean Doom implementation

The retained implementation is a Lean 4 program. The engine (playsim, renderer, menus,
intermission, finale, automap, sound and music synthesis, LAN netcode) is
Lean; `lean/native/tv_ffi.c` only moves bytes between Lean and the TV app ABI
(`src/platform/include/tv_app_v1.h`): it converts the 320x200 indexed frame
into the leased display buffer, passes key events and PCM, and opens a UDP
socket for LAN games. The engine started from a Lean port of Chocolate Doom
(see `lean/UPSTREAM.txt`, GPL-2.0-or-later). Game data is Freedoom.

Music starts automatically at the in-game level 15/15. The TV's master volume
still limits all output; Doom's Sound menu can lower music independently.

## Layout

| Path | What |
|---|---|
| `lean/Doom/Playsim/`, `lean/Doom/Game.lean` | Game simulation and game flow, a port of Chocolate Doom tic-identical on all shareware and Freedoom demos |
| `lean/Doom/Render/` | Software renderer (BSP, walls, planes, sprites, status bar, HUD) |
| `lean/Doom/Front/` | Menu, intermission, finale, automap, DEHACKED strings |
| `lean/Doom/Audio/` | DMX sfx, 8-channel mixer, MUS/MIDI, GENMIDI, Nuked OPL3 port, `s_sound.c` logic |
| `lean/Doom/Net/` | Lockstep LAN session for up to 4 players (pure, explicit time) |
| `lean/Doom/TV/` | Remote/keyboard key map, platform externs, LAN discovery and checksums |
| `lean/DoomTV.lean` | The app: title pages, menu, game loop, audio pump, LAN lobby |
| `tools/doomtv/build-lean.sh` (repository root) | Cross-build the ARM32 app (Lean runtime from `tools/lean-arm`, `libplatform.a`) |

## Build

    tools/doomtv/build-lean.sh          # -> assets/build/lean-doom/armv7-unknown-linux-musleabihf/doomtv

The first run cross-compiles Lean 4.35.0's runtime and Init for armv7 musl
with zig (`tools/lean-arm/build.sh`). The normal Nix, native-app, and QEMU
builds select Rust/C Doom; this command explicitly builds the retained Lean app.

## Checks

| Command | What it shows |
|---|---|
| `test/doomtv/check-demos.sh` | all 14 demos (shareware, Freedoom 1/2, FreeDM) identical to Chocolate Doom at every tic (host, ~15 s) |
| `tools/lean-arm/check-demos-arm32.sh` | the same as an ARM32 binary in QEMU |
| `test/doomtv/scripts/fuzz.sh`, `exits.sh`, `brain.sh` | synthetic demos, forced exits/finales and the boss brain vs the oracle (`scripts/build-oracle.sh`) |
| `lake exe map-sweep IWAD [TICS]` | every map played with scripted input, reporting engine errors |
| `fb-test` (run from `assets/build/lean-doom-check/fbroot`) | rendered frames identical to Chocolate Doom |
| `lake exe front-test` | intermission/finale/automap renders and state machines (316 checks) |
| `test/doomtv/check-audio.sh` | OPL3 bit-exact vs Nuked/Chocolate C, music end to end vs Chocolate's player |
| `lake exe net-test` / `net-sim` / `net-fuzz` | netcode codecs, lossy multi-peer simulation, decoder fuzzing |
| `lake exe keys-test` | remote/keyboard bindings |
| `lake exe bench WAD TICS [compare]` | host render/playsim timing; `compare` checks live vs harness frames |
| `test/doomtv/check-qemu.py --apps DIR --media DIR [--attract] [--saveload]` | the app under the real launcher in ARM32 QEMU: title, menu, play, Home (or the attract demos; or save, play on, load) |
| `test/doomtv/check-multiplayer.py --apps DIR --media DIR [--mode dm] [--menu] [--leave join\|host]` | two VMs on a LAN: discovery, lobby, lockstep play, matching checksums; `--menu` starts the game through the menus as on the TV, `--leave` presses Home on one TV and checks that the other plays on or returns to the title |
| `test/doomtv/check-net-quit-arm32.sh` | a LAN player leaving (`PlayerQuitGame`'s message), run as PID 1 on ARM32 |
| `nix run .#pid1 -- --check` | the repository's whole graphical demo, Doom included |

`--apps` is a directory with `bin/{tvinit,ytplay,slides,doomtv}` (for
example `nix build .#pid1.passthru.apps` with your own `doomtv` copied in);
`--media` is `nix build .#pid1.passthru.media`.

## Running on the TV

    tools/native-app/build.sh && python3 tools/native-app/run.py

Headless timing of the shipping binary, no display or root needed:

    adb push .../release/doomtv /data/local/tmp/doomtv
    adb push assets/third_party/freedoom/freedoom1.wad /data/local/tmp/
    adb shell /data/local/tmp/doomtv doom.bench=300 doom.iwad=/data/local/tmp/freedoom1.wad

## Options (program arguments, or `doom.*` on the kernel command line /
## `/etc/tvinit.conf`, which the launcher forwards)

| Option | Meaning |
|---|---|
| `doom.iwad=PATH` | game data (default `/doom/freedoom1.wad`) |
| `doom.warp=E,M` / `doom.skill=0..4` | skip the title and start a map |
| `doom.sound=0` | no audio |
| `doom.net=host[,dm]` / `doom.net=join[,IP]` | go straight to a LAN lobby (join finds a host by broadcast unless an IP is given) |
| `doom.netauto=N` | host starts once N players are in |
| `doom.name=NAME` / `doom.netlead=N` | lobby name; command window (default 6) |
| `doom.stats=SECONDS` / `doom.bench=N` | rate reports; headless benchmark |

## LAN games

New Game > host co-op / host deathmatch lan game opens a lobby; the other
TVs pick join lan game, which finds the host by UDP broadcast on port 2342,
and the host presses OK. Every TV must run Doom (Lean) with the same IWAD:
the classic Doom tile speaks Chocolate Doom's protocol, which this one does
not. A player who presses Home is dropped after 8 s of silence and the
others play on; if the host leaves, the others return to the title pages.

## Remote

The TCL remote (`src/platform/src/remote.rs`; PID 1 keeps Power, Home,
Volume and Mute): OK fires and selects (and answers "yes" to prompts), Back
opens the menu or steps back, arrows move/turn, Ch+ uses, Ch- runs, YouTube
opens the automap and the Ch rocker zooms it. In the Rust/C build NETFLIX /
prime video pick the previous / next weapon; the Lean playsim has no weapon
cycling yet, so there those two buttons are unassigned and only keyboard
digits pick weapons. Home is the launcher's.

## Not done yet

Saving to disk (the six Load/Save slots are kept in memory until the app
exits); HU chat; the vanilla netgame consistency check (the lockstep
session compares `Doom.Game.checksum` instead). Freedoom's DEHACKED is
applied as far as Freedoom uses it (frames and BEX strings). Nothing here has
run on the TV yet; frame time on the Cortex-A55 is the open question
(`doom.bench=N`).

## Integration checks

Integrated from `d4hines/lean-doom` at `08166c7` while preserving the current
PID1 and direct-media backends. The 14 demo comparisons and audio reference
checks pass. The ARM32 launcher test covers gameplay, automap, save/load and
Home; evidence is in `assets/build/lean-doom/qemu-20261101T070103/`. This is
QEMU evidence, not a measurement of TV performance.

`tools/native-app/run.py` transfers executables compressed so the static Lean
binary fits the existing bounded upload endpoint, then expands them in RAM
before the normal application restart. No boot image or firmware is changed.
