import Doom.Playsim.Player
import Doom.Net.Types

/-!
# Doom.Net.Bytes

Total big-endian byte access (out-of-range reads yield 0, out-of-range writes
are no-ops, nothing panics) and the packed 5-byte `TicCmd` used on the wire and
in every tic ring:

| byte | field                                   |
|------|-----------------------------------------|
| 0    | forwardmove, signed 8-bit, clamped      |
| 1    | sidemove, signed 8-bit, clamped         |
| 2-3  | angleturn, signed 16-bit BE, clamped    |
| 4    | buttons, low 8 bits                     |
-/

namespace Doom.Net

open Doom.Playsim.Player (TicCmd)

@[inline] def rd8 (b : ByteArray) (i : Nat) : UInt8 :=
  if h : i < b.size then b.get i h else 0

@[inline] def rd16 (b : ByteArray) (i : Nat) : UInt16 :=
  ((rd8 b i).toUInt16 <<< 8) ||| (rd8 b (i + 1)).toUInt16

@[inline] def rd32 (b : ByteArray) (i : Nat) : UInt32 :=
  ((rd8 b i).toUInt32 <<< 24) ||| ((rd8 b (i + 1)).toUInt32 <<< 16)
    ||| ((rd8 b (i + 2)).toUInt32 <<< 8) ||| (rd8 b (i + 3)).toUInt32

@[inline] def wr8 (b : ByteArray) (v : UInt8) : ByteArray := b.push v

@[inline] def wr16 (b : ByteArray) (v : UInt16) : ByteArray :=
  (b.push (v >>> 8).toUInt8).push v.toUInt8

@[inline] def wr32 (b : ByteArray) (v : UInt32) : ByteArray :=
  (((b.push (v >>> 24).toUInt8).push (v >>> 16).toUInt8).push (v >>> 8).toUInt8).push v.toUInt8

/-- In-place store; ignored when out of range. -/
@[inline] def put8 (b : ByteArray) (i : Nat) (v : UInt8) : ByteArray :=
  if h : i < b.size then b.set i v h else b

@[inline] def put32 (b : ByteArray) (i : Nat) (v : UInt32) : ByteArray :=
  put8 (put8 (put8 (put8 b i (v >>> 24).toUInt8) (i + 1) (v >>> 16).toUInt8)
    (i + 2) (v >>> 8).toUInt8) (i + 3) v.toUInt8

/-- A zero-filled buffer of `n` bytes. -/
def zeroBytes (n : Nat) : ByteArray := Id.run do
  let mut b := ByteArray.emptyWithCapacity n
  for _ in [0:n] do
    b := b.push 0
  return b

/-- Copy `n` bytes `src[so..]` to `dst[do..]` (byte-wise, total). -/
@[inline] def copyBytes (dst : ByteArray) (dOff : Nat) (src : ByteArray) (sOff n : Nat) :
    ByteArray := Id.run do
  let mut d := dst
  for k in [0:n] do
    d := put8 d (dOff + k) (rd8 src (sOff + k))
  return d

/-- Zero `n` bytes at `off`. -/
@[inline] def zeroRange (dst : ByteArray) (off n : Nat) : ByteArray := Id.run do
  let mut d := dst
  for k in [0:n] do
    d := put8 d (off + k) 0
  return d

/-- Append `src[off, off+n)`. -/
def appendRange (dst : ByteArray) (src : ByteArray) (off n : Nat) : ByteArray := Id.run do
  let mut d := dst
  for k in [0:n] do
    d := d.push (rd8 src (off + k))
  return d

/-- Number of set bits among the low 4 bits (player masks). -/
@[inline] def popcount4 (m : UInt8) : Nat :=
  (m &&& 1).toNat + ((m >>> 1) &&& 1).toNat + ((m >>> 2) &&& 1).toNat + ((m >>> 3) &&& 1).toNat

@[inline] def bit (m : UInt8) (p : Nat) : Bool := ((m >>> p.toUInt8) &&& 1) != 0

/-! ## TicCmd packing -/

def CMD_BYTES : Nat := 5

@[inline] def clampI32 (v lo hi : Int32) : Int32 :=
  if v < lo then lo else if hi < v then hi else v

/-- Clamp to the vanilla `ticcmd_t` ranges the wire can carry. -/
def clampCmd (c : TicCmd) : TicCmd := {
  forwardmove := clampI32 c.forwardmove (-128) 127
  sidemove := clampI32 c.sidemove (-128) 127
  angleturn := clampI32 c.angleturn (-32768) 32767
  buttons := c.buttons &&& 0xff
}

@[inline] def sext8 (b : UInt8) : Int32 :=
  let u := b.toUInt32
  (if u ≥ 128 then u ||| 0xffffff00 else u).toInt32

@[inline] def sext16 (h : UInt16) : Int32 :=
  let u := h.toUInt32
  (if u ≥ 0x8000 then u ||| 0xffff0000 else u).toInt32

/-- Append a clamped command. -/
@[inline] def pushCmd (b : ByteArray) (c0 : TicCmd) : ByteArray :=
  let c := clampCmd c0
  let a := c.angleturn.toUInt32
  ((((b.push c.forwardmove.toUInt32.toUInt8).push c.sidemove.toUInt32.toUInt8).push
    (a >>> 8).toUInt8).push a.toUInt8).push c.buttons.toUInt8

/-- Store a clamped command at `off` (in place, total). -/
@[inline] def putCmd (b : ByteArray) (off : Nat) (c0 : TicCmd) : ByteArray :=
  let c := clampCmd c0
  let a := c.angleturn.toUInt32
  put8 (put8 (put8 (put8 (put8 b off c.forwardmove.toUInt32.toUInt8) (off + 1)
    c.sidemove.toUInt32.toUInt8) (off + 2) (a >>> 8).toUInt8) (off + 3) a.toUInt8)
    (off + 4) c.buttons.toUInt8

@[inline] def readCmd (b : ByteArray) (off : Nat) : TicCmd := {
  forwardmove := sext8 (rd8 b off)
  sidemove := sext8 (rd8 b (off + 1))
  angleturn := sext16 (rd16 b (off + 2))
  buttons := (rd8 b (off + 4)).toUInt32
}

/-- The packed form of a command. -/
def packCmd (c : TicCmd) : ByteArray := pushCmd (ByteArray.emptyWithCapacity CMD_BYTES) c

/-- Strings of printable ASCII up to `NAME_MAX` bytes, as bytes. -/
def nameBytes (s : String) : ByteArray := Id.run do
  let mut b := ByteArray.emptyWithCapacity NAME_MAX
  for c in (sanitizeName s).toList do
    b := b.push c.toNat.toUInt8
  return b

/-- `some name` iff `b[off, off+n)` is printable ASCII and `n ≤ NAME_MAX`. -/
def readName? (b : ByteArray) (off n : Nat) : Option String := Id.run do
  if n > NAME_MAX then return none
  let mut s := ""
  for k in [0:n] do
    let v := rd8 b (off + k)
    if v < 0x20 || v > 0x7e then return none
    s := s.push (Char.ofNat v.toNat)
  return some s

end Doom.Net
