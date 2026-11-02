import Doom.Playsim.Input
import Doom.TV.Keys
open Doom.TV.Keys Doom.Playsim.Input

/-- The TCL remote and keyboard reach the engine's own bindings. -/
def main : IO UInt32 := do
  let has (code : UInt32) (k : Nat) := (doomKeys code).contains k.toUInt32
  let checks := [
    ("OK fires and selects", has 28 key_fire && has 28 key_enter),
    ("Ctrl fires", has 29 key_fire), ("Space uses", has 57 key_use),
    ("Play/Pause uses", has 164 key_use), ("Green uses", has 0x18f key_use),
    ("Yellow runs", has 0x190 key_speed), ("Back is escape", has 158 key_escape),
    ("arrows", has 103 key_up && has 108 key_down && has 105 key_left && has 106 key_right),
    ("strafe keys", has 51 key_strafeleft && has 52 key_straferight),
    ("Alt strafes", has 56 key_strafe), ("digits pick weapons", has 3 key_weapon2),
    -- The TCL remote: Ch+ uses, Ch- runs, YouTube toggles the automap,
    -- the Ch rocker zooms it.
    ("Ch+ uses", has 402 key_use), ("Ch- runs", has 403 key_speed),
    ("YouTube toggles the automap", has 569 9), ("Blue toggles the automap", has 0x191 9),
    ("Ch/Page zoom the automap", has 402 '='.toNat && has 403 '-'.toNat &&
      has 104 '='.toNat && has 109 '-'.toNat && has 13 '='.toNat && has 12 '-'.toNat),
    ("unknown codes ignored", (doomKeys 0).isEmpty)]
  let bad := checks.filter (!·.2)
  for (name, _) in bad do IO.eprintln s!"keys-test FAIL: {name}"
  if bad.isEmpty then IO.println s!"keys-test: {checks.length} mappings OK"
  return if bad.isEmpty then 0 else 1
