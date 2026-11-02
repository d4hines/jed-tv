prelude
import TV.Core
import TV.Volume.Native
import Init.Data.SInt.Float32

/- A hardware-free test of the real Core loop on the Zr runtime. Native
   callbacks use raw Linux effects only; no upstream C++ runtime is linked. -/
namespace CoreZr

initialize reference : IO.Ref (Array String) ← IO.mkRef #["original"]

@[extern "zr_tv_sys"] opaque sys (nr a0 a1 a2 a3 : USize) : BaseIO USize
@[extern "zr_tv_write"] opaque writeRaw (fd : USize) (b : @& ByteArray) (off len : USize) : BaseIO USize
@[extern "zr_tv_open"] opaque openPath (path : @& String) (flags mode : USize) : BaseIO USize
private def bad (r : USize) : Bool := r > (0 : USize) - 4096

private partial def putAll (b : ByteArray) (off : USize) : BaseIO Unit := do
  if off < b.size.toUSize then
    let n ← writeRaw 1 b off (b.size.toUSize - off)
    if n != 0 && !bad n then putAll b (off + n)

private def say (s : String) : IO Unit := putAll (s ++ "\n").toUTF8 0

private def slot : BaseIO USize := sys 19 42 0 1 0 -- lseek(current)

-- Event encoding is the actual Core native ABI. The kernel file offset is
-- test input position, so the Lean callbacks need no mutable reference shim.
private def key (code : UInt64) (held : UInt64 := 0) : UInt64 :=
  (1 : UInt64) ||| ((2 : UInt64) <<< 8) ||| (held <<< 16) ||| (code <<< 32)
private def launch (app : UInt64) : UInt64 := (2 : UInt64) ||| (app <<< 16)
private def enable (app pin : UInt64) : UInt64 :=
  (2 : UInt64) ||| ((3 : UInt64) <<< 8) ||| (app <<< 16) ||| (pin <<< 32)
private def changePin (old new : UInt64) : UInt64 :=
  (2 : UInt64) ||| ((1 : UInt64) <<< 8) ||| (old <<< 32) ||| (new <<< 48)

@[export tv_core_next]
def next : IO UInt64 := do
  let pos ← sys 19 42 1 1 0
  if pos > 100 && pos < 200 then
    if pos == 101 then return launch 2
    if pos == 102 then return 1
    if pos == 103 then return (2 : UInt64) ||| ((2 : UInt64) <<< 8) ||| ((2 : UInt64) <<< 16)
    say "FAIL next after save failure"
    return 255
  if pos > 200 then
    if pos % 100 == 1 then return launch 2
    return 255
  let event := match pos.toNat with
    | 1 => launch 2
    | 2 => key 103
    | 3 => key 103 1
    | 4 => (4 : UInt64) ||| ((103 : UInt64) <<< 32)
    | 5 => key 102 -- incorrectly classified system key must still be blocked
    | 6 => 1 -- Home
    | 7 => launch 1 -- app still stopping
    | 8 => 0 -- tick exactly at grace: no KILL
    | 9 => 1 -- repeated Home does not extend grace
    | 10 => 0 -- tick after grace: KILL
    | 11 => (3 : UInt64) ||| ((2 : UInt64) <<< 8) -- Doom exit
    | 12 => (2 : UInt64) ||| ((2 : UInt64) <<< 8) ||| ((1 : UInt64) <<< 16) -- disable YouTube
    | 13 => launch 1
    | 14 => enable 1 0 -- bad PIN
    | 15 => enable 1 1234
    | 16 => changePin 0 4321
    | 17 => changePin 1234 4321
    | 18 => launch 1
    | 19 => (1 : UInt64) ||| ((1 : UInt64) <<< 8) -- Power
    | 20 => 1
    | 21 => (3 : UInt64) ||| ((1 : UInt64) <<< 8)
    | 22 => changePin 1234 6789 -- old PIN must fail
    | 23 => changePin 4321 6789
    | _ => 255
  pure event

@[export tv_core_clock]
def clock : IO UInt64 := do
  let pos ← slot
  pure <| if pos < 8 then 0 else if pos < 10 then 1500000000 else 1500000001

@[export tv_core_load]
def load : IO UInt64 := do
  say "load"
  let pos ← slot
  if pos ≥ 300 then return 10000 -- invalid PIN
  if pos ≥ 200 then return (1 : UInt64) <<< 32 -- unreadable settings
  pure ((1234 : UInt64) ||| ((55 : UInt64) <<< 16))
@[export tv_core_save]
def save (pin enabled : UInt32) : IO Unit := do
  say s!"save {pin.toNat} {enabled.toNat}"
  let pos ← slot
  if pos > 100 && pos < 200 then TV.Core.Native.ioError 5
@[export tv_core_show_home]
def showHome : IO Unit := say "home"
@[export tv_core_show_app]
def showApp (app : UInt32) : IO Unit := say s!"show {app.toNat}"
@[export tv_core_menu_key]
def menuKey (kind : UInt8) (code : UInt32) (held : Bool) : IO Unit :=
  say s!"menu-key {kind.toNat} {code.toNat} {if held then 2 else 1}"
@[export tv_core_menu_apps]
def menuApps (enabled : UInt32) : IO Unit := say s!"menu-apps {enabled.toNat}"
@[export tv_core_tell]
def tell (kind : UInt8) (app : UInt32) : IO Unit := say s!"tell {kind.toNat} {app.toNat}"
@[export tv_core_start]
def start (app : UInt32) : IO Unit := say s!"start {app.toNat}"
@[export tv_core_signal]
def signal (app : UInt32) (kill : Bool) : IO Unit :=
  say s!"signal {app.toNat} {if kill then 9 else 15}"
@[export tv_core_forward]
def forward (app code : UInt32) (phase : UInt8) : IO Unit :=
  say s!"forward {app.toNat} {code.toNat} {phase.toNat}"
@[export tv_core_power_off]
def powerOff : IO Unit := say "power-off"

@[export tv_volume_initial]
def volumeInitial : IO UInt64 := pure 75000
@[export tv_volume_ready]
def volumeReady : IO UInt64 := pure 1
@[export tv_volume_poll]
def volumePoll : IO UInt64 := do
  let pos ← sys 19 42 1 1 0
  let key (code value : UInt64) := (2 : UInt64) ||| (value <<< 8) ||| (code <<< 32)
  pure <| match pos.toNat with
    | 1001 => key 115 1
    | 1002 => key 113 1
    | 1003 => key 113 2
    | 1004 => key 113 1
    | 1005 => (1 : UInt64) ||| ((123456 : UInt64) <<< 32)
    | 1006 => key 114 1
    | 1007 => key 113 1
    | 1008 => key 115 1
    | 1009 => key 113 1
    | _ => 0
@[export tv_volume_commit]
def volumeCommit (gain : UInt32) : IO UInt64 := do
  say s!"volume-write {gain.toNat}"
  pure <| if (← slot) == 1008 then 0 else 1
@[export tv_volume_publish]
def volumePublish (gain : UInt32) : IO Unit := say s!"volume-show {gain.toNat}"

private def runVolume : IO Unit := do
  say "finite-volume"
  let rec ticks : Nat → UInt64 → IO UInt64
    | 0, state => pure state
    | n + 1, state => do
      let next ← TV.Volume.Native.tick state
      say s!"volume-state {next.toUInt32.toNat} {(next >>> 32).toUInt32.toNat}"
      ticks n next
  let state ← ticks 9 (← TV.Volume.Native.start)
  if state != ((105000 : UInt64) ||| ((105000 : UInt64) <<< 32)) then
    say "FAIL volume final state"

private def runReferences : IO Unit := do
  let cell ← IO.mkRef #["one", "two"]
  let before ← cell.get
  cell.modify (·.push "three")
  let old ← cell.swap #["replacement"]
  if before.size != 2 || old.size != 3 || (← cell.get).size != 1 then
    say "FAIL reference alias/swap"
  let original ← reference.get
  reference.modify (·.push "next")
  if original.size != 1 || (← reference.get).size != 2 then
    say "FAIL persistent reference modification"
  reference.set #[]
  if (← reference.get).size != 0 then say "FAIL persistent reference set"
  say "references passed"

private def runFloat32 : IO Unit := do
  let check (bits : UInt32) (expected : UInt16) : IO Unit := do
    if (Float32.ofBits bits).toInt16.toUInt16 != expected then
      say "FAIL float32 saturation"
  check 0x00000000 0
  check 0x80000000 0
  check 0x7fc00000 0
  check 0x7f800000 32767
  check 0xff800000 32768
  check 0x42f78000 123
  check 0xc2f78000 65413
  check 0x46ffff80 32767
  check 0x47000000 32767
  check 0xc7000080 32768
  check 0xc7000100 32768
  let expected := Float32.ofBits 0x4b800000
  let rounded := (16777217 : UInt32).toFloat32
  if rounded < expected || expected < rounded then say "FAIL uint32 float32 rounding"
  let positive := -((65494 : UInt16).toInt16.toFloat32)
  let product := positive * Float32.ofBits 0x3f000000
  if product.toInt16.toUInt16 != 21 then say "FAIL float32 multiply/negate"
  say "float32 passed"

private def runCore (label expected : String) : BaseIO Unit := do
  putAll (label ++ "\n").toUTF8 0
  match ← TV.Core.main.toBaseIO with
  | .error (.userError message) =>
    if message == expected then putAll "stopped\n".toUTF8 0
    else putAll ("FAIL unexpected error: " ++ message ++ "\n").toUTF8 0
  | _ => putAll "FAIL unexpected result\n".toUTF8 0

@[export zr_core_test_main]
def main (_argc : UInt32) : BaseIO UInt32 := do
  let fd ← openPath "/core-state" (0o100 ||| 0o1000 ||| 2) 0o600
  if bad fd then return 2
  let dup ← sys 63 fd 42 0 0
  if bad dup then return 3
  let _ ← sys 6 fd 0 0 0
  putAll "==core-zr-begin==\n".toUTF8 0
  runCore "normal" "launcher stopped"
  let _ ← sys 19 42 100 0 0
  runCore "save-failure" "native core effect failed (errno 5)"
  let _ ← sys 19 42 200 0 0
  runCore "unreadable-policy" "launcher stopped"
  let _ ← sys 19 42 300 0 0
  runCore "invalid-pin" "launcher stopped"
  let _ ← sys 19 42 1000 0 0
  match ← runVolume.toBaseIO with
  | .ok _ => pure ()
  | .error _ => putAll "FAIL volume IO\n".toUTF8 0
  match ← runReferences.toBaseIO with
  | .ok _ => pure ()
  | .error _ => putAll "FAIL references IO\n".toUTF8 0
  match ← runFloat32.toBaseIO with
  | .ok _ => pure ()
  | .error _ => putAll "FAIL float32 IO\n".toUTF8 0
  putAll "==core-zr-end==\n".toUTF8 0
  if (← sys 20 0 0 0 0) == 1 then
    let _ ← sys 36 0 0 0 0
    let _ ← sys 88 0xfee1dead 672274793 0x4321fedc 0
  pure 0

end CoreZr
