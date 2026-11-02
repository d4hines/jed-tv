import Doom.Net.Session
/-!
# Doom.Net.Test.Rng

Seeded splitmix64 generator and random message generators shared by the
Doom.Net test executables. Host-side test support only.
-/

namespace Doom.Net.Test

open Doom.Playsim.Player (TicCmd)

structure Rng where
  s : UInt64
  deriving Inhabited

abbrev R := StateM Rng

def Rng.ofSeed (seed : UInt64) : Rng := ⟨seed * 0x9E3779B97F4A7C15 + 0x1234567⟩

def Rng.step (r : Rng) : UInt64 × Rng :=
  let s : UInt64 := r.s + 0x9E3779B97F4A7C15
  let z : UInt64 := (s ^^^ (s >>> 30)) * 0xBF58476D1CE4E5B9
  let z : UInt64 := (z ^^^ (z >>> 27)) * 0x94D049BB133111EB
  (z ^^^ (z >>> 31), ⟨s⟩)

def rand : R UInt64 := modifyGet Rng.step

/-- Uniform in `[0, n)` (`n = 0` yields 0). -/
def below (n : Nat) : R Nat := do
  if n == 0 then return 0
  return (← rand).toNat % n

/-- Uniform in `[lo, hi]`. -/
def range (lo hi : Nat) : R Nat := do return lo + (← below (hi + 1 - lo))

def chance (permille : Nat) : R Bool := do return (← below 1000) < permille

def randU8 : R UInt8 := do return (← rand).toUInt8
def randU32 : R UInt32 := do return (← rand).toUInt32

def randInt (lo hi : Int) : R Int := do
  let span := (hi - lo + 1).toNat
  return lo + (← below span)

def randBytes (n : Nat) : R ByteArray := do
  let mut b := ByteArray.emptyWithCapacity n
  for _ in [0:n] do b := b.push (← randU8)
  return b

def randName : R String := do
  let n ← below (NAME_MAX + 1)
  let mut s := ""
  for _ in [0:n] do s := s.push (Char.ofNat (← range 0x20 0x7e))
  return s

/-- A command possibly outside the wire ranges (exercises clamping). -/
def randCmdWide : R TicCmd := do
  return { forwardmove := Int32.ofInt (← randInt (-300) 300)
           sidemove := Int32.ofInt (← randInt (-300) 300)
           angleturn := Int32.ofInt (← randInt (-70000) 70000)
           buttons := (← randU32) &&& 0x1ff }

def randMask : R UInt8 := do return (← range 1 15).toUInt8

def randBitOf (m : UInt8) : R UInt8 := do
  let mut bits : Array Nat := #[]
  for p in [0:4] do if bit m p then bits := bits.push p
  return (bits.getD (← below bits.size) 0).toUInt8

def randNamesFor (m : UInt8) : R (Array String) := do
  let mut a : Array String := #["", "", "", ""]
  for p in [0:4] do
    if bit m p then a := a.setIfInBounds p (← randName)
  return a

def randSettings : R GameSettings := do
  return { episode := (← below 10).toUInt8, map := (← below 100).toUInt8
           skill := (← below 5).toUInt8, deathmatch := (← below 3).toUInt8
           nomonsters := ← chance 500, respawn := ← chance 500, fast := ← chance 500
           startTic := (← below 0x10000000).toUInt32 }

def randTic (span : Nat) : R UInt32 := do
  if ← chance 100 then return (0xFFFFFFFF - span).toUInt32
  if ← chance 100 then return 0
  return (← below (0x100000000 - span)).toUInt32

def randDrop : R DropInfo := do
  let r := match ← below 3 with | 0 => DropReason.timeout | 1 => .quit | _ => .kicked
  return { slot := (← below 4).toUInt8, tic := ← randU32, reason := r }

def randSetsBody (n : Nat) : R ByteArray := do
  let mut b := ByteArray.empty
  for _ in [0:n] do
    let m ← randMask
    b := b.push m
    b := b.append (← randBytes (CMD_BYTES * popcount4 m))
  return b

/-- A random, canonical (encodable and strictly decodable) message. -/
def randMsg : R Msg := do
  match ← below 7 with
  | 0 => return .hello (← randU32) (← randBytes (← below (DIGEST_MAX + 1))) (← randName)
  | 1 =>
    let m ← randMask
    return .lobby (← randU32) (← randBitOf m) m (← randNamesFor m)
  | 2 =>
    let r := match ← below 4 with
      | 0 => RejectReason.version | 1 => .iwad | 2 => .full | _ => .started
    return .reject (← randU32) r (← randU8)
  | 3 =>
    let m ← randMask
    return .start (← randU32) { settings := ← randSettings, playerMask := m
                                consoleplayer := ← randBitOf m
                                inputDelay := (← below (MAX_INPUT_DELAY + 1)).toUInt8
                                names := ← randNamesFor m }
  | 4 =>
    let n ← below (MAX_CMDS_PER_PACKET + 1)
    let k ← below (MAX_SUMS_PER_PACKET + 1)
    return .cmds { slot := (← below 4).toUInt8, setAck := ← randU32, firstTic := ← randTic n
                   count := n, cmds := ← randBytes (n * CMD_BYTES), sumFirst := ← randTic k
                   sumCount := k, sums := ← randBytes (4 * k) }
  | 5 =>
    let n ← below (MAX_SETS_PER_PACKET + 1)
    let d ← below (MAX_PLAYERS + 1)
    let mut drops := #[]
    for _ in [0:d] do drops := drops.push (← randDrop)
    let desync ← chance 300
    let dt := (← randU32) &&& 0x7fffffff
    let dm := (← below 16).toUInt8
    return .sets { cmdAck := ← randU32, sumAck := ← randU32, ahead := ← randU8
                   firstTic := ← randTic n, count := n, body := ← randSetsBody n, drops
                   desyncTic := if desync then dt else NO_TIC
                   desyncMask := if desync then dm else 0 }
  | _ =>
    let r := match ← below 3 with | 0 => DropReason.timeout | 1 => .quit | _ => .kicked
    return .leave (← below 4).toUInt8 r

/-- The largest message of every variable-size type. -/
def maxMsgs : Array Msg :=
  let name := "ABCDEFGHIJKLMNOP"
  let names := #[name, name, name, name]
  let body := Id.run do
    let mut b := ByteArray.empty
    for _ in [0:MAX_SETS_PER_PACKET] do
      b := b.push 15
      b := b.append (zeroBytes (4 * CMD_BYTES))
    return b
  #[.hello 0xFFFFFFFF (zeroBytes DIGEST_MAX) name,
    .lobby 1 3 15 names,
    .start 1 { settings := {}, playerMask := 15, consoleplayer := 3, inputDelay := 8, names },
    .cmds { slot := 3, setAck := 0, firstTic := 0, count := MAX_CMDS_PER_PACKET
            cmds := zeroBytes (MAX_CMDS_PER_PACKET * CMD_BYTES), sumFirst := 0
            sumCount := MAX_SUMS_PER_PACKET, sums := zeroBytes (4 * MAX_SUMS_PER_PACKET) },
    .sets { cmdAck := 0, sumAck := 0, ahead := 0, firstTic := 0, count := MAX_SETS_PER_PACKET
            body, drops := #[⟨0, 1, .quit⟩, ⟨1, 2, .quit⟩, ⟨2, 3, .quit⟩, ⟨3, 4, .quit⟩]
            desyncTic := 5, desyncMask := 15 }]

end Doom.Net.Test
