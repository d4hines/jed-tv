# Offline RPG

`rpgtv` runs The Drowned Lighthouse locally. The app contains the original Lean
rules, seeded dice, combat engine, adventure and legality/input proofs. Rust
draws the existing interface and forwards a selected option index to Lean.
There is no HTTP client, host server, periodic remote refresh or network
permission. PID 1 still launches the app as ID 5 and owns Home.

The files under `lean/Rpg` come from
`agostbiro/vibecheck-hackathon` commit
`13a10c7dfb8b81eccadba36b7842170bcd949c69`, the source of the former host game.
Game logic and story are unchanged. JSON derivations/imports were removed
from `Report.lean` and `Game.lean`; the unused probability/fairness import was
removed from `Adventures/Lighthouse/Definition.lean`. `RpgTv/Engine.lean` is the
local session and display adapter. All generated C and binaries stay under
`assets/build`.

The build-time `demo/rpg.nix` derivation supplies only the original static
artwork. It builds and runs no server.

Validation: `cargo +1.92.0 test --offline --manifest-path src/Cargo.toml -p
rpgtv --test offline -- --nocapture` exercises paired seeded games, invalid
choices, combat, endings/death and restart. `test/rpg/check-qemu.py` compares
the complete UI transcript of that test binary against its ARM32/Zr build.
`lean --run test/rpg/check-runtime-abi.lean` checks the pinned standard-library
ownership ABI. The string/Int runtime oracle is under
`test/lean/zr/string-extra/`.
