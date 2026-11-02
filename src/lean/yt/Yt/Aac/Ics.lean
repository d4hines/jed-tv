import Yt.Aac.Config
import Yt.Aac.Huffman

/-! individual_channel_stream parsing (ISO/IEC 14496-3 4.4.2.7): ics_info,
section data, scalefactors, pulse and TNS data, and the Huffman-coded
quantized spectrum. Band-indexed arrays use index `g * 64 + sfb`; spectral
arrays hold 1024 words with short window `w` at `w * 128`. -/

namespace Yt.Aac

/-- Band types (Huffman codebook numbers and the special types). -/
def zeroHcb : UInt8 := 0
def noiseHcb : UInt8 := 13
def intensityHcb2 : UInt8 := 14
def intensityHcb : UInt8 := 15

structure IcsInfo where
  /-- window_sequence: 0 only-long, 1 long-start, 2 eight-short, 3 long-stop. -/
  seq : UInt8
  /-- window_shape: KBD when set, sine otherwise. -/
  kbd : Bool
  maxSfb : Nat
  /-- Window-group lengths (one group of one window for long blocks). -/
  groupLen : Array Nat
  /-- Scalefactor-band offsets for this window length. -/
  swb : Array Nat
  deriving Inhabited

namespace IcsInfo
@[inline] def short (i : IcsInfo) : Bool := i.seq == 2
@[inline] def numWindows (i : IcsInfo) : Nat := if i.short then 8 else 1
@[inline] def numSwb (i : IcsInfo) : Nat := i.swb.size - 1
end IcsInfo

structure TnsFilter where
  win : Nat
  length : Nat
  order : Nat
  /-- Filter downwards in frequency. -/
  down : Bool
  /-- Reflection coefficients, Q31. -/
  coefs : Array Int32
  deriving Inhabited

structure Ics where
  info : IcsInfo
  globalGain : Nat
  /-- Bytes: band type of band `g * 64 + sfb`. -/
  bandType : ByteArray
  /-- Words: scalefactor, noise energy or intensity position of each band. -/
  sf : ByteArray
  /-- Words: largest |quantized value| of each band. -/
  bmax : ByteArray
  /-- 1024 words: quantized spectrum. -/
  q : ByteArray
  /-- TNS filters in bitstream order. -/
  tns : Array TnsFilter
  /-- Pulse data was present. -/
  pulses : Bool
  deriving Inhabited

private def err (s : String) : Except DecodeError α := throw (.invalid s)

def parseIcsInfo (r : BitReader) (rateIndex : Nat) : Except DecodeError (IcsInfo × BitReader) := do
  let (_reserved, r) := r.read 1
  let (seq, r) := r.read 2
  let (kbd, r) := r.readBool
  if seq == 2 then
    let (maxSfb, r) := r.readNat 4
    let (grp, r) := r.read 7
    let mut lens : Array Nat := #[1]
    for i in [0:7] do
      if (grp >>> (6 - i).toUInt32) &&& 1 == 1 then
        lens := lens.set! (lens.size - 1) (lens[lens.size - 1]! + 1)
      else
        lens := lens.push 1
    let swb := Tables.swbShort rateIndex
    if maxSfb > swb.size - 1 then err "max_sfb exceeds the number of bands"
    return ({ seq := seq.toUInt8, kbd, maxSfb, groupLen := lens, swb }, r)
  else
    let (maxSfb, r) := r.readNat 6
    let (pred, r) := r.readBool
    if pred then throw (.unsupported "prediction (AAC Main)")
    let swb := Tables.swbLong rateIndex
    if maxSfb > swb.size - 1 then err "max_sfb exceeds the number of bands"
    return ({ seq := seq.toUInt8, kbd, maxSfb, groupLen := #[1], swb }, r)

/-- section_data: band types. -/
def parseSections (r : BitReader) (info : IcsInfo) : Except DecodeError (ByteArray × BitReader) := do
  let bits : UInt32 := if info.short then 3 else 5
  let esc : Nat := if info.short then 7 else 31
  let mut bt := zeros (8 * 64)
  let mut r := r
  for g in [0:info.groupLen.size] do
    let mut k := 0
    -- Each section consumes at least 4 + bits bits, so the overrun check ends the loop.
    for _ in [0:100000] do
      if k ≥ info.maxSfb then break
      let (cb, r1) := r.readNat 4
      r := r1
      if cb == 12 then err "reserved band type"
      let mut len := 0
      for _ in [0:100000] do
        let (incr, r2) := r.readNat bits
        r := r2
        len := len + incr
        if incr != esc || r.overrun then break
      if r.overrun then err "section data truncated"
      if k + len > info.maxSfb then err "section exceeds max_sfb"
      for sfb in [k:k + len] do
        bt := st8 bt (g * 64 + sfb).toUSize cb.toUInt8
      k := k + len
  return (bt, r)

/-- scale_factor_data. Normal bands get their scalefactor (0..255), noise
bands their energy, intensity bands their position (FFmpeg's clipping). -/
def parseScalefactors (r : BitReader) (info : IcsInfo) (bt : ByteArray) (globalGain : Nat)
    (scf : HuffTable) : Except DecodeError (ByteArray × BitReader) := do
  let mut sf := zeros32 (8 * 64)
  let mut r := r
  let mut normal : Int32 := Int32.ofNat globalGain
  let mut noise : Int32 := Int32.ofNat globalGain - 90
  let mut inten : Int32 := 0
  let mut firstNoise := true
  for g in [0:info.groupLen.size] do
    for sfb in [0:info.maxSfb] do
      let i := (g * 64 + sfb).toUSize
      let t := ld8 bt i
      if t == zeroHcb then
        pure ()
      else
        let delta : Int32 ←
          if t == noiseHcb && firstNoise then
            let (v, r1) := r.read 9
            r := r1
            pure (v.toInt32 - 256)
          else
            let e := scf.decode r.data r.pos
            if e >>> 16 == 0 then err "scalefactor codeword"
            r := { r with pos := r.pos + (e >>> 16).toUSize }
            pure ((e &&& 0xFFFF).toInt32 - 60)
        if t == intensityHcb || t == intensityHcb2 then
          inten := inten + delta
          let v := if inten < -155 then -155 else if inten > 100 then 100 else inten
          sf := stI sf i v
        else if t == noiseHcb then
          firstNoise := false
          noise := noise + delta
          let v := if noise < -100 then -100 else if noise > 155 then 155 else noise
          sf := stI sf i v
        else
          normal := normal + delta
          if normal < 0 || normal > 255 then err "scalefactor out of range"
          sf := stI sf i normal
  return (sf, r)

/-- pulse_data: added to the quantized spectrum. -/
def parsePulses (r : BitReader) (info : IcsInfo) : Except DecodeError (Array (Nat × Nat) × BitReader) := do
  let (n, r) := r.readNat 2
  let (startSfb, r) := r.readNat 6
  if startSfb ≥ info.numSwb then err "pulse start band"
  let top := info.swb[info.numSwb]!
  let mut pos := info.swb[startSfb]!
  let mut out := #[]
  let mut r := r
  for i in [0:n + 1] do
    let (off, r1) := r.readNat 5
    let (amp, r2) := r1.readNat 4
    r := r2
    pos := pos + off
    if pos ≥ top then err "pulse position"
    out := out.push (pos, amp)
    let _ := i
  return (out, r)

/-- tns_data. -/
def parseTns (r : BitReader) (info : IcsInfo) (coef3 coef4 : ByteArray) :
    Except DecodeError (Array TnsFilter × BitReader) := do
  let short := info.short
  let maxOrder := if short then 7 else 12
  let mut r := r
  let mut out := #[]
  for w in [0:info.numWindows] do
    let (nFilt, r1) := r.readNat (if short then 1 else 2)
    r := r1
    if nFilt > 0 then
      let (res4, r1) := r.readBool
      r := r1
      for _ in [0:nFilt] do
        let (length, r1) := r.readNat (if short then 4 else 6)
        let (order, r2) := r1.readNat (if short then 3 else 5)
        r := r2
        if order > maxOrder then err "TNS filter order"
        if order > 0 then
          let (down, r1) := r.readBool
          let (compress, r2) := r1.readBool
          r := r2
          let bits : UInt32 := (if res4 then 4 else 3) - (if compress then 1 else 0)
          let table := if res4 then coef4 else coef3
          let mut coefs := #[]
          for _ in [0:order] do
            let (v, r1) := r.read bits
            r := r1
            -- sign-extend `bits` bits, then index c + 8
            let c : Int32 := if v >>> (bits - 1) == 1 then v.toInt32 - ((1 : Int32) <<< bits.toInt32) else v.toInt32
            coefs := coefs.push (ldI table (c + 8).toUInt32.toUSize)
          out := out.push { win := w, length, order, down, coefs }
        else
          out := out.push { win := w, length, order := 0, down := false, coefs := #[] }
  return (out, r)

/-! ### Spectral data -/

structure Band where
  pos : USize
  q : ByteArray
  maxq : UInt32
  ok : Bool
  deriving Inhabited

@[inline] private def sgn (v : UInt32) (w : UInt32) (i : UInt32) : Int32 :=
  if (w <<< i) >>> 31 == 1 then 0 - v.toInt32 else v.toInt32

@[inline] private def umax (a b : UInt32) : UInt32 := if a > b then a else b

/-- Codebooks 1, 2: signed quads, values −1..1. -/
private partial def quadsSigned (t : HuffTable) (d : ByteArray) (pos : USize) (q : ByteArray)
    (k kEnd : USize) (mx : UInt32) : Band :=
  if k < kEnd then
    let e := t.decode d pos
    if e >>> 16 == 0 then ⟨pos, q, mx, false⟩ else
    let s := e &&& 0xFFFF
    let q := stI q k ((s / 27).toInt32 - 1)
    let q := stI q (k + 1) ((s / 9 % 3).toInt32 - 1)
    let q := stI q (k + 2) ((s / 3 % 3).toInt32 - 1)
    let q := stI q (k + 3) ((s % 3).toInt32 - 1)
    quadsSigned t d (pos + (e >>> 16).toUSize) q (k + 4) kEnd (if s != 40 then 1 else mx)
  else ⟨pos, q, mx, true⟩

/-- Codebooks 3, 4: unsigned quads, values 0..2, then sign bits. -/
private partial def quadsUnsigned (t : HuffTable) (d : ByteArray) (pos : USize) (q : ByteArray)
    (k kEnd : USize) (mx : UInt32) : Band :=
  if k < kEnd then
    let e := t.decode d pos
    if e >>> 16 == 0 then ⟨pos, q, mx, false⟩ else
    let s := e &&& 0xFFFF
    let pos := pos + (e >>> 16).toUSize
    let a := s / 27
    let b := s / 9 % 3
    let c := s / 3 % 3
    let dd := s % 3
    let w := peek32 d pos
    let na : UInt32 := if a != 0 then 1 else 0
    let nb : UInt32 := if b != 0 then 1 else 0
    let nc : UInt32 := if c != 0 then 1 else 0
    let nd : UInt32 := if dd != 0 then 1 else 0
    let q := stI q k (sgn a w 0)
    let q := stI q (k + 1) (sgn b w na)
    let q := stI q (k + 2) (sgn c w (na + nb))
    let q := stI q (k + 3) (sgn dd w (na + nb + nc))
    quadsUnsigned t d (pos + (na + nb + nc + nd).toUSize) q (k + 4) kEnd
      (umax mx (umax (umax a b) (umax c dd)))
  else ⟨pos, q, mx, true⟩

/-- Codebooks 5, 6: signed pairs, values −4..4. -/
private partial def pairsSigned (t : HuffTable) (d : ByteArray) (pos : USize) (q : ByteArray)
    (k kEnd : USize) (mx : UInt32) : Band :=
  if k < kEnd then
    let e := t.decode d pos
    if e >>> 16 == 0 then ⟨pos, q, mx, false⟩ else
    let s := e &&& 0xFFFF
    let y := (s / 9).toInt32 - 4
    let z := (s % 9).toInt32 - 4
    let q := stI q k y
    let q := stI q (k + 1) z
    pairsSigned t d (pos + (e >>> 16).toUSize) q (k + 2) kEnd (umax mx (umax (iabs y) (iabs z)))
  else ⟨pos, q, mx, true⟩

/-- Codebooks 7..10: unsigned pairs, values 0..m−1, then sign bits. -/
private partial def pairsUnsigned (t : HuffTable) (m : UInt32) (d : ByteArray) (pos : USize)
    (q : ByteArray) (k kEnd : USize) (mx : UInt32) : Band :=
  if k < kEnd then
    let e := t.decode d pos
    if e >>> 16 == 0 then ⟨pos, q, mx, false⟩ else
    let s := e &&& 0xFFFF
    let pos := pos + (e >>> 16).toUSize
    let y := s / m
    let z := s % m
    let w := peek32 d pos
    let ny : UInt32 := if y != 0 then 1 else 0
    let nz : UInt32 := if z != 0 then 1 else 0
    let q := stI q k (sgn y w 0)
    let q := stI q (k + 1) (sgn z w ny)
    pairsUnsigned t m d (pos + (ny + nz).toUSize) q (k + 2) kEnd (umax mx (umax y z))
  else ⟨pos, q, mx, true⟩

/-- escape_code after a 16 in codebook 11: `(value <<< 8) ||| bits`, or 0 on overflow. -/
@[inline] private def escape (d : ByteArray) (pos : USize) : UInt32 :=
  let w := peek32 d pos
  let inv := ~~~w
  let n : UInt32 := if inv == 0 then 32 else 31 - inv.log2
  if n > 8 then 0
  else
    let b := n + 4
    let v := ((1 : UInt32) <<< b) + ((w <<< (n + 1)) >>> (32 - b))
    (v <<< 8) ||| (n + 1 + b)

/-- Codebook 11: unsigned pairs 0..16 with escapes, then sign bits. -/
private partial def pairsEsc (t : HuffTable) (d : ByteArray) (pos : USize) (q : ByteArray)
    (k kEnd : USize) (mx : UInt32) : Band :=
  if k < kEnd then
    let e := t.decode d pos
    if e >>> 16 == 0 then ⟨pos, q, mx, false⟩ else
    let s := e &&& 0xFFFF
    let pos := pos + (e >>> 16).toUSize
    let y := s / 17
    let z := s % 17
    let w := peek32 d pos
    let ny : UInt32 := if y != 0 then 1 else 0
    let nz : UInt32 := if z != 0 then 1 else 0
    let pos := pos + (ny + nz).toUSize
    let ey := if y == 16 then escape d pos else 0
    if y == 16 && ey == 0 then ⟨pos, q, mx, false⟩ else
    let y := if y == 16 then ey >>> 8 else y
    let pos := pos + (ey &&& 0xFF).toUSize
    let ez := if z == 16 then escape d pos else 0
    if z == 16 && ez == 0 then ⟨pos, q, mx, false⟩ else
    let z := if z == 16 then ez >>> 8 else z
    let pos := pos + (ez &&& 0xFF).toUSize
    let q := stI q k (sgn y w 0)
    let q := stI q (k + 1) (sgn z w ny)
    pairsEsc t d pos q (k + 2) kEnd (umax mx (umax y z))
  else ⟨pos, q, mx, true⟩

/-- Decode the coefficients `[k, kEnd)` coded with codebook `cb` (1..11). -/
def decodeBand (bk : Books) (cb : UInt8) (d : ByteArray) (pos : USize) (q : ByteArray)
    (k kEnd : USize) : Band :=
  let t := bk.spec[cb.toNat - 1]!
  if cb ≤ 2 then quadsSigned t d pos q k kEnd 0
  else if cb ≤ 4 then quadsUnsigned t d pos q k kEnd 0
  else if cb ≤ 6 then pairsSigned t d pos q k kEnd 0
  else if cb ≤ 8 then pairsUnsigned t 8 d pos q k kEnd 0
  else if cb ≤ 10 then pairsUnsigned t 13 d pos q k kEnd 0
  else pairsEsc t d pos q k kEnd 0

/-- spectral_data: quantized values and per-band maxima. -/
def parseSpectrum (r : BitReader) (info : IcsInfo) (bt : ByteArray) (bk : Books) :
    Except DecodeError (ByteArray × ByteArray × BitReader) := do
  let mut q := zeros32 1024
  let mut bmax := zeros32 (8 * 64)
  let mut pos := r.pos
  let mut win := 0
  for g in [0:info.groupLen.size] do
    let glen := info.groupLen[g]!
    for sfb in [0:info.maxSfb] do
      let i := (g * 64 + sfb).toUSize
      let cb := ld8 bt i
      if cb ≥ 1 && cb ≤ 11 then
        let lo := info.swb[sfb]!
        let hi := info.swb[sfb + 1]!
        let mut mx : UInt32 := 0
        for w in [win:win + glen] do
          let b := decodeBand bk cb r.data pos q (w * 128 + lo).toUSize (w * 128 + hi).toUSize
          if !b.ok then err "spectral codeword"
          q := b.q
          pos := b.pos
          if b.maxq > mx then mx := b.maxq
        bmax := st32 bmax i mx
    win := win + glen
  return (q, bmax, { r with pos })

/-- Add pulses to the quantized spectrum (normal bands only, as FFmpeg) and
refresh the band maxima. -/
def applyPulses (q bmax bt : ByteArray) (info : IcsInfo) (pulses : Array (Nat × Nat)) :
    ByteArray × ByteArray := Id.run do
  let mut q := q
  let mut bmax := bmax
  let mut band := 0
  for (p, amp) in pulses do
    for _ in [0:64] do
      if band + 1 < info.swb.size && info.swb[band + 1]! ≤ p then band := band + 1
    let t := ld8 bt band.toUSize
    if t ≥ 1 && t ≤ 11 then
      let v := ldI q p.toUSize
      let a := Int32.ofNat amp
      let v := if v > 0 then v + a else v - a
      q := stI q p.toUSize v
      if iabs v > ld32 bmax band.toUSize then bmax := st32 bmax band.toUSize (iabs v)
  return (q, bmax)

/-- individual_channel_stream. With a common window the CPE's shared
`ics_info` is passed in. -/
def parseIcs (r : BitReader) (rateIndex : Nat) (common : Option IcsInfo) (bk : Books)
    (coef3 coef4 : ByteArray) : Except DecodeError (Ics × BitReader) := do
  let (globalGain, r) := r.readNat 8
  let (info, r) ← match common with
    | some i => pure (i, r)
    | none => parseIcsInfo r rateIndex
  let (bt, r) ← parseSections r info
  let (sf, r) ← parseScalefactors r info bt globalGain bk.scf
  let (pulsePresent, r) := r.readBool
  let (pulses, r) ← if pulsePresent then
      (if info.short then err "pulse data in short windows" else parsePulses r info)
    else pure (#[], r)
  let (tnsPresent, r) := r.readBool
  let (tns, r) ← if tnsPresent then parseTns r info coef3 coef4 else pure (#[], r)
  let (gainControl, r) := r.readBool
  if gainControl then throw (.unsupported "gain control (AAC SSR)")
  if r.overrun then err "channel stream truncated"
  let (q, bmax, r) ← parseSpectrum r info bt bk
  let (q, bmax) := if pulses.isEmpty then (q, bmax) else applyPulses q bmax bt info pulses
  return ({ info, globalGain, bandType := bt, sf, bmax, q, tns, pulses := pulsePresent }, r)

end Yt.Aac
