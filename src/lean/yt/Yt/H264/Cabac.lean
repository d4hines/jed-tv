/-
The CABAC arithmetic decoding engine (ITU-T H.264 9.3.1.2, 9.3.3.2). Part of
the Lean H.264 decoder (follows FFmpeg 9.0.1, LGPL-2.1-or-later).

The engine is a structure updated in place: every decoding function returns
the updated engine and leaves the decoded bin in `bin` (a returned pair would
allocate per bin). Context states are bytes `(pStateIdx << 1) | valMPS`.
-/
import Yt.H264.Tables

namespace Yt.H264

structure Cabac where
  data : ByteArray
  /-- Next bit to read, from the start of `data`. -/
  bitPos : USize
  range : UInt32
  offset : UInt32
  ctx : ByteArray
  /-- The last decoded bin (0 or 1). -/
  bin : UInt32 := 0
  deriving Inhabited

-- Hot code uses fixed-width arithmetic only: Nat shifts go through the
-- arbitrary-precision path.

@[inline] def readBit (data : ByteArray) (pos : USize) : UInt32 :=
  let i := pos >>> 3
  if i < data.usize then ((data[i.toNat]!).toUInt32 >>> (7 - (pos &&& 7)).toUInt32) &&& 1 else 0

/-- `n` bits from `pos`, most significant first (n ≤ 9). -/
partial def readBitsGo (data : ByteArray) (pos n k : USize) (v : UInt32) : UInt32 :=
  if k < n then readBitsGo data pos n (k + 1) ((v <<< 1) ||| readBit data (pos + k)) else v

def readBits (data : ByteArray) (pos : USize) (n : USize) : UInt32 := readBitsGo data pos n 0 0

/-- Number of doublings that bring `r` (≥ 2) to at least 256. -/
@[inline] def renormShift (r : UInt32) : USize :=
  if r ≥ 256 then 0 else if r ≥ 128 then 1 else if r ≥ 64 then 2 else if r ≥ 32 then 3
  else if r ≥ 16 then 4 else if r ≥ 8 then 5 else if r ≥ 4 then 6 else 7

/-- Context initialization (9.3.1.1) for a slice of QP `qp`: the (m, n)
pairs of cabac_init_idc `initIdc` (none: an I slice). -/
def initContexts (qp : Nat) (initIdc : Option Nat) : ByteArray := Id.run do
  let tab := match initIdc with
    | none => Tables.cabacInitI
    | some k => Tables.cabacInitPB.extract (k * 2048) (k * 2048 + 2048)
  let q : Int := min 51 qp
  let mut out := ByteArray.emptyWithCapacity 1024
  for i in [0:1024] do
    let i8 (b : UInt8) : Int := if b ≥ 128 then b.toNat - 256 else b.toNat
    let m := i8 tab[2 * i]!
    let n := i8 tab[2 * i + 1]!
    let pre : Int := max 1 (min 126 (((m * q) >>> 4) + n))
    let st : Nat := if pre ≤ 63 then (63 - pre).toNat <<< 1 else ((pre - 64).toNat <<< 1) ||| 1
    out := out.push st.toUInt8
  return out

/-- Start decoding at byte `pos` of `data` (9.3.1.2). -/
def Cabac.init (data : ByteArray) (pos : Nat) (ctx : ByteArray) : Cabac :=
  { data, bitPos := (pos * 8 + 9).toUSize, range := 510, offset := readBits data (pos * 8).toUSize 9, ctx }

/-- transIdxLPS (Table 9-45). -/
def transIdxLPS : ByteArray := ⟨#[0, 0, 1, 2, 2, 4, 4, 5, 6, 7, 8, 9, 9, 11, 11, 12, 13, 13, 15, 15, 16, 16, 18, 18, 19,
  19, 21, 21, 22, 22, 23, 24, 24, 25, 26, 26, 27, 27, 28, 29, 29, 30, 30, 30, 31, 32, 32, 33, 33, 33, 34, 34, 35,
  35, 35, 36, 36, 36, 37, 37, 37, 38, 38, 63]⟩

-- Each engine step ends in exactly one structure update. Updates in several
-- branches (or one before an inlined renormalization) leave all but one
-- without the compiler's in-place reuse, so a step would allocate a new
-- engine and free the old one.

/-- DecodeDecision with context `i` (9.3.3.2.1), renormalized. -/
def Cabac.decision (c : Cabac) (i : Nat) : Cabac :=
  let st := c.ctx[i]!.toUInt32
  let p := st >>> 1
  let mps := st &&& 1
  let lps := Tables.rangeTabLPS[(p * (4 : UInt32) + ((c.range >>> 6) &&& 3)).toNat]!.toUInt32
  let r := c.range - lps
  let isLps := c.offset ≥ r
  let range := if isLps then lps else r
  let offset := if isLps then c.offset - r else c.offset
  let bin := if isLps then 1 - mps else mps
  let st' :=
    if isLps then (transIdxLPS[p.toNat]!.toUInt32 <<< 1) ||| (if p == 0 then 1 - mps else mps)
    else ((if p < 62 then p + 1 else p) <<< 1) ||| mps
  let s := renormShift range
  let ctx := c.ctx
  let c := { c with ctx := .empty }
  { c with ctx := ctx.set! i st'.toUInt8, range := range <<< s.toUInt32,
           offset := (offset <<< s.toUInt32) ||| readBits c.data c.bitPos s, bitPos := c.bitPos + s, bin }

/-- DecodeBypass (9.3.3.2.3). -/
def Cabac.bypass (c : Cabac) : Cabac :=
  let o := (c.offset <<< 1) ||| readBit c.data c.bitPos
  let one := o ≥ c.range
  { c with offset := if one then o - c.range else o, bitPos := c.bitPos + 1, bin := if one then 1 else 0 }

/-- DecodeTerminate (9.3.3.2.2): no renormalization after a 1 (end of slice). -/
def Cabac.terminate (c : Cabac) : Cabac :=
  let r := c.range - 2
  let one := c.offset ≥ r
  let s := if one then 0 else renormShift r
  { c with range := r <<< s.toUInt32, offset := (c.offset <<< s.toUInt32) ||| readBits c.data c.bitPos s,
           bitPos := c.bitPos + s, bin := if one then 1 else 0 }

end Yt.H264
