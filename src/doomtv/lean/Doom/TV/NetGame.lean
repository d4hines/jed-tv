import Doom.Net.Session
import Doom.Playsim.GameState
import Doom.Wad

/-!
# Doom.TV.NetGame

LAN game pieces around the lockstep session (`Doom.Net`): finding a host
without typing an address, the IWAD identity peers must share, and the
per-tic state checksum used for desync detection.

Discovery shares UDP port 2342 with the session but never its framing
(sessions start with `'L' 'D'`): a joiner broadcasts `DQRY v`; a host in its
lobby answers `DHST v mode players nameLen name`. Decoders are total and
bounded like the session's.
-/

namespace Doom.TV.NetGame

open Doom.Net
open Doom.Playsim.GameState
open Doom.Wad

def discoveryVersion : UInt8 := 1

def query : ByteArray := ⟨#[0x44, 0x51, 0x52, 0x59, discoveryVersion]⟩  -- "DQRY" v

def isQuery (b : ByteArray) : Bool :=
  b.size == 5 && b[0]! == 0x44 && b[1]! == 0x51 && b[2]! == 0x52 && b[3]! == 0x59 && b[4]! == discoveryVersion

/-- A host found on the LAN. -/
structure Found where
  addr : Addr
  /-- 1 co-op, 2 deathmatch. -/
  mode : Nat
  players : Nat
  name : String
  deriving Repr, BEq

def answer (mode players : Nat) (name : String) : ByteArray := Id.run do
  let n := (name.toUTF8.extract 0 16)
  let mut b : ByteArray := ⟨#[0x44, 0x48, 0x53, 0x54, discoveryVersion]⟩  -- "DHST" v
  b := b.push mode.toUInt8 |>.push players.toUInt8 |>.push n.size.toUInt8
  return b ++ n

def parseAnswer (src : Addr) (b : ByteArray) : Option Found := do
  guard (b.size ≥ 8 && b[0]! == 0x44 && b[1]! == 0x48 && b[2]! == 0x53 && b[3]! == 0x54 && b[4]! == discoveryVersion)
  let len := b[7]!.toNat
  guard (len ≤ 16 && b.size == 8 + len)
  let name ← String.fromUTF8? (b.extract 8 (8 + len))
  guard (name.all fun c => c.toNat ≥ 32 && c.toNat < 127)
  return { addr := src, mode := b[5]!.toNat, players := b[6]!.toNat, name }

/-- IWAD identity for the HELLO check: FNV-1a over the directory (names and
sizes), so peers with different game data refuse each other. -/
def iwadDigest (wad : WadDirectory) : ByteArray := Id.run do
  let mut h : UInt64 := 14695981039346656037
  for e in wad.entries do
    for i in [0:8] do
      h := (h ^^^ (e.name[i]?.getD 0).toUInt64) * 1099511628211
    for k in [0:4] do
      h := (h ^^^ ((e.size >>> (8 * k).toUInt32) &&& 0xff).toUInt64) * 1099511628211
  let mut out : ByteArray := .empty
  for k in [0:8] do out := out.push ((h >>> (8 * k).toUInt64) &&& 0xff).toUInt8
  return out

/-- Per-tic checksum compared between peers (`consistancy` in vanilla is the
console player's x; this folds every player's position, the RNG index and the
level time, so most divergences show within a tic). -/
def checksum (gs : GameState) : UInt32 := Id.run do
  let mut h : UInt32 := 2166136261
  let mix (h v : UInt32) : UInt32 := (h ^^^ v) * 16777619
  h := mix h gs.leveltime
  h := mix h gs.rng.prndindex
  for p in gs.players do
    if p.mo >= 0 then
      match gs.mobjs[p.mo.toNatClampNeg]? with
      | some m => h := mix (mix (mix h m.x.toUInt32) m.y.toUInt32) m.angle
      | none => pure ()
  return h

end Doom.TV.NetGame
