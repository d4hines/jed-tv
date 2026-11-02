import Yt.Jpeg.Idct

/-! Baseline and progressive Huffman JPEG to RGB, for thumbnails, and the
nearest-neighbour downscale of src/ytplay/src/browse.rs.

Supported: 8-bit sequential (SOF0/SOF1) and progressive (SOF2) Huffman
JPEGs, 1 (grayscale) or 3 components (YCbCr, or RGB per the Adobe marker
or component IDs, as libjpeg decides), any sampling factors of 1..4 with
integral ratios, restart markers, DQT/DHT between scans. Not supported:
arithmetic coding, lossless, hierarchical, 12-bit, CMYK/4 components, DNL.

The output equals libjpeg-turbo's `djpeg` (default settings: ISLOW
IDCT, fancy upsampling; see Idct.lean) byte for byte on the test images:
the same IDCT and colour arithmetic, the same upsampling (h2v1, h1v2,
h2v2 "fancy" triangle filters with edge replication; box filters
otherwise) and the same progressive refinement. Corrupt entropy data
decodes like libjpeg's (zeros past a marker or the end), not as an error. -/
namespace Yt.Jpeg
open Yt.Fmt

structure Image where
  width : Nat
  height : Nat
  /-- RGB24, row-major, `width * height * 3` bytes. -/
  rgb : ByteArray
  deriving Inhabited

/-- Zigzag index → natural (row-major) index, with libjpeg's 16 extra
entries so that corrupt runs past 63 land on 63. -/
def natural : ByteArray := ⟨#[0, 1, 8, 16, 9, 2, 3, 10, 17, 24, 32, 25, 18, 11, 4, 5,
  12, 19, 26, 33, 40, 48, 41, 34, 27, 20, 13, 6, 7, 14, 21, 28,
  35, 42, 49, 56, 57, 50, 43, 36, 29, 22, 15, 23, 30, 37, 44, 51,
  58, 59, 52, 45, 38, 31, 39, 46, 53, 60, 61, 54, 47, 55, 62, 63,
  63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63, 63]⟩

/-! ## Huffman tables (jdhuff.c's derived tables) -/

structure Huff where
  /-- 9-bit lookahead: entry `i` is (code length, symbol) at bytes 2i, 2i+1;
  length 0 means a longer code. -/
  lut : ByteArray
  /-- `maxcode[l] + 1` for lengths 1..17 (0: no code of that length). -/
  maxcode : Array Nat
  /-- `valoffset[l] + 65536`. -/
  valoff : Array Nat
  vals : ByteArray
  deriving Inhabited

/-- Build the decoding table from the 16 code-length counts and the symbols. -/
def Huff.build (bits : ByteArray) (vals : ByteArray) : Except String Huff := do
  -- Code lengths and codes, in symbol order.
  let mut sizes : Array Nat := #[]
  for l in [1:17] do
    for _ in [0:(byteAt bits (l - 1)).toNat] do sizes := sizes.push l
  if sizes.size > 256 || sizes.size > vals.size then throw "JPEG: bad Huffman table"
  let mut codes : Array Nat := #[]
  let mut code := 0
  let mut si := if sizes.size > 0 then sizes[0]! else 0
  let mut p := 0
  while p < sizes.size do
    while p < sizes.size && sizes[p]! == si do
      codes := codes.push code
      code := code + 1
      p := p + 1
    if code ≥ 1 <<< si then throw "JPEG: bad Huffman table"
    code := code * 2
    si := si + 1
  let mut maxcode : Array Nat := Array.replicate 18 0
  let mut valoff : Array Nat := Array.replicate 18 0
  p := 0
  for l in [1:17] do
    let cnt := (byteAt bits (l - 1)).toNat
    if cnt > 0 then
      valoff := valoff.set! l (p + 65536 - codes[p]!)
      p := p + cnt
      maxcode := maxcode.set! l (codes[p - 1]! + 1)
  maxcode := maxcode.set! 17 0x100000
  let mut lut := zeros 1024
  p := 0
  for l in [1:10] do
    for _ in [0:(byteAt bits (l - 1)).toNat] do
      let c := codes[p]! <<< (9 - l)
      for k in [0:1 <<< (9 - l)] do
        lut := (lut.set! (2 * (c + k)) l.toUInt8).set! (2 * (c + k) + 1) (byteAt vals p)
      p := p + 1
  pure { lut, maxcode, valoff, vals }

/-! ## Bit reader -/

/-- Entropy-coded data reader: `buf` holds `cnt` valid bits (low bits). At a
marker (or the end) it supplies zero bits, as libjpeg does; the lowest
`fake` bits of `buf` are such zeros. Consuming any of them sets `bad`
(libjpeg's `insufficient_data`: the rest of the MCU is decoded from zeros,
later MCUs are left alone until the next restart). -/
structure Bits where
  p : Nat
  buf : UInt64
  cnt : UInt32
  fake : UInt32 := 0
  bad : Bool := false
  deriving Inhabited

/-- Shift in the next byte (or zeros at a marker or the end). -/
@[inline] def step (d : ByteArray) (b : Bits) : Bits :=
  let c := byteAt d b.p
  if b.p ≥ d.size then { b with buf := b.buf <<< 8, cnt := b.cnt + 8, fake := b.fake + 8 }
  else if c == 0xFF then
    -- 0xFF00 is a stuffed 0xFF; anything else is a marker: don't pass it.
    if byteAt d (b.p + 1) == 0 then { b with p := b.p + 2, buf := (b.buf <<< 8) ||| 0xFF, cnt := b.cnt + 8 }
    else { b with buf := b.buf <<< 8, cnt := b.cnt + 8, fake := b.fake + 8 }
  else { b with p := b.p + 1, buf := (b.buf <<< 8) ||| c.toUInt64, cnt := b.cnt + 8 }

/-- Make at least 25 bits available (at most 4 bytes are needed). -/
@[inline] def fill (d : ByteArray) (b : Bits) : Bits :=
  if b.cnt ≥ 25 then b else
  let b := step d b
  if b.cnt ≥ 25 then b else
  let b := step d b
  if b.cnt ≥ 25 then b else
  let b := step d b
  if b.cnt ≥ 25 then b else step d b

/-- Drop `n` bits from the top of the buffer. -/
@[inline] def consume (b : Bits) (n : UInt32) : Bits :=
  let c := b.cnt - n
  if c ≥ b.fake then { b with cnt := c } else { b with cnt := c, fake := c, bad := true }

/-- The next `n` bits (n ≤ 16), as an unsigned value. -/
@[inline] def getBits (d : ByteArray) (b : Bits) (n : UInt32) : UInt32 × Bits :=
  if n == 0 then (0, b) else
  let b := fill d b
  let c := b.cnt - n
  (((b.buf >>> c.toUInt64) &&& ((1 <<< n.toUInt64) - 1)).toUInt32, consume b n)

/-- libjpeg's HUFF_EXTEND: the signed value of `n` received bits. -/
@[inline] def extend (v n : UInt32) : UInt32 :=
  if n == 0 then 0 else if v < (1 <<< (n - 1)) then v - (1 <<< n) + 1 else v

/-- Codes longer than the 9-bit lookahead: try lengths `l..16`; returns
(symbol, length). A bad code gives symbol 0 (libjpeg warns and does so). -/
partial def slowCode (h : Huff) (buf : UInt64) (cnt : UInt32) (l : Nat) : UInt32 × UInt32 :=
  if l > 16 then (0, 16) else
  let code := ((buf >>> (cnt - l.toUInt32).toUInt64) &&& ((1 <<< l.toUInt64) - 1)).toNat
  if code + 1 ≤ h.maxcode[l]! then ((byteAt h.vals (code + h.valoff[l]! - 65536)).toUInt32, l.toUInt32)
  else slowCode h buf cnt (l + 1)

/-- Decode one Huffman symbol. -/
@[inline] def huffDecode (d : ByteArray) (h : Huff) (b : Bits) : UInt32 × Bits :=
  let b := fill d b
  let look := ((b.buf >>> (b.cnt - 9).toUInt64) &&& 511).toNat
  let len := byteAt h.lut (2 * look)
  if len != 0 then
    ((byteAt h.lut (2 * look + 1)).toUInt32, consume b len.toUInt32)
  else
    let (sym, l) := slowCode h b.buf b.cnt 10
    (sym, consume b l)

/-! ## Frame and decoder state -/

structure Comp where
  id : UInt8
  h : Nat
  v : Nat
  tq : Nat
  /-- Blocks per row and rows of blocks in the coefficient store (the
  interleaved MCU grid, which covers the component's own blocks). -/
  bw : Nat
  bh : Nat
  /-- The component's own blocks (non-interleaved scans). -/
  wib : Nat
  hib : Nat
  /-- Downsampled size in samples. -/
  dw : Nat
  dh : Nat
  /-- Its first block in the coefficient store. -/
  block0 : Nat
  /-- Quantization table latched at its first scan (64 int16, natural order). -/
  q : ByteArray
  deriving Inhabited

structure St where
  width : Nat := 0
  height : Nat := 0
  progressive : Bool := false
  comps : Array Comp := #[]
  hmax : Nat := 1
  vmax : Nat := 1
  mcusX : Nat := 0
  mcusY : Nat := 0
  qt : Array ByteArray := Array.replicate 4 ByteArray.empty
  dc : Array Huff := Array.replicate 4 default
  ac : Array Huff := Array.replicate 4 default
  restart : Nat := 0
  jfif : Bool := false
  adobe : Option UInt8 := none
  /-- int16 coefficients, 64 per block, natural order. -/
  coefs : ByteArray := ByteArray.empty
  /-- Per block, the highest zigzag index holding a non-zero AC value (0:
  DC only). -/
  maxk : ByteArray := ByteArray.empty
  deriving Inhabited

@[inline] def be16 (d : ByteArray) (i : Nat) : Nat := (byteAt d i).toNat * 256 + (byteAt d (i + 1)).toNat

/-- Store the int16 coefficient `v` at natural index `k` of block `blk`. -/
@[inline] def setCoef (c : ByteArray) (blk k : Nat) (v : UInt32) : ByteArray :=
  let i := 2 * (64 * blk + k)
  (c.set! i v.toUInt8).set! (i + 1) (v >>> 8).toUInt8

@[inline] def getCoef (c : ByteArray) (blk k : Nat) : UInt32 := s16 c (64 * blk + k)

@[inline] def bumpMaxk (m : ByteArray) (blk : Nat) (k : Nat) : ByteArray :=
  if (byteAt m blk).toNat < k then m.set! blk k.toUInt8 else m

/-! ## Block decoders -/

/-- Mutable scan state threaded through the block decoders. -/
structure Scan where
  bits : Bits
  coefs : ByteArray
  maxk : ByteArray
  /-- DC predictors of the scan's components. -/
  pred : Array UInt32
  eobrun : Nat

/-- The AC values of a sequential block from zigzag index `k`; returns the
reader, the coefficients and the last index written. (Hot loops are written
as recursive functions: a `for`/`while` with several mutable variables packs
them into a tuple that is allocated on every iteration.) -/
partial def acSeq (d : ByteArray) (acT : Huff) (blk : Nat) (b : Bits) (coefs : ByteArray) (k last : Nat) :
    Bits × ByteArray × Nat :=
  if k ≥ 64 then (b, coefs, last) else
  let (rs, b) := huffDecode d acT b
  let run := (rs >>> 4).toNat
  let sz := rs &&& 15
  if sz != 0 then
    let k := k + run
    let (v, b) := getBits d b sz
    acSeq d acT blk b (setCoef coefs blk (byteAt natural k).toNat (extend v sz)) (k + 1) k
  else if run != 15 then (b, coefs, last)
  else acSeq d acT blk b coefs (k + 16) last

/-- A sequential (baseline) block: DC difference and the 63 AC values. -/
def blockSeq (d : ByteArray) (dcT acT : Huff) (s : Scan) (ci blk : Nat) : Scan :=
  -- Take the fields out of `s` first, so that the arrays are not shared
  -- (a shared coefficient array would be copied by every write).
  let ⟨bits, coefs, maxk, pred, eobrun⟩ := s
  let (t, b) := huffDecode d dcT bits
  let (r, b) := getBits d b t
  let dc := pred[ci]! + extend r t
  let (b, coefs, last) := acSeq d acT blk b (setCoef coefs blk 0 dc) 1 0
  ⟨b, coefs, bumpMaxk maxk blk (min last 63), pred.set! ci dc, eobrun⟩

/-- Progressive DC scan, first pass or refinement. -/
def blockDC (d : ByteArray) (dcT : Huff) (ah al : Nat) (s : Scan) (ci blk : Nat) : Scan :=
  let ⟨bits, coefs, maxk, pred, eobrun⟩ := s
  if ah == 0 then
    let (t, b) := huffDecode d dcT bits
    let (r, b) := getBits d b t
    let dc := pred[ci]! + extend r t
    ⟨b, setCoef coefs blk 0 (dc <<< al.toUInt32), maxk, pred.set! ci dc, eobrun⟩
  else
    let (bit, b) := getBits d bits 1
    if bit != 0 then
      let v := getCoef coefs blk 0 ||| ((1 : UInt32) <<< al.toUInt32)
      ⟨b, setCoef coefs blk 0 v, maxk, pred, eobrun⟩
    else ⟨b, coefs, maxk, pred, eobrun⟩

/-- The band of a progressive first-pass AC block from zigzag `k`; returns
the reader, coefficients, highest index written and the new EOB run. -/
partial def acFirst (d : ByteArray) (acT : Huff) (se al blk : Nat) (b : Bits) (coefs : ByteArray)
    (k top : Nat) : Bits × ByteArray × Nat × Nat :=
  if k > se then (b, coefs, top, 0) else
  let (rs, b) := huffDecode d acT b
  let r := (rs >>> 4).toNat
  let sz := rs &&& 15
  if sz != 0 then
    let k := k + r
    let (v, b) := getBits d b sz
    acFirst d acT se al blk b (setCoef coefs blk (byteAt natural k).toNat (extend v sz <<< al.toUInt32))
      (k + 1) (max top (min k 63))
  else if r == 15 then acFirst d acT se al blk b coefs (k + 16) top
  else
    let (e, b) := getBits d b r.toUInt32
    (b, coefs, top, (1 <<< r) + e.toNat - 1)

/-- Progressive AC scan, first pass (spectral band ss..se). -/
def blockACFirst (d : ByteArray) (acT : Huff) (ss se al : Nat) (s : Scan) (blk : Nat) : Scan :=
  let ⟨bits, coefs, maxk, pred, eobrun⟩ := s
  if eobrun > 0 then ⟨bits, coefs, maxk, pred, eobrun - 1⟩ else
  let (b, coefs, top, eobrun) := acFirst d acT se al blk bits coefs ss 0
  ⟨b, coefs, bumpMaxk maxk blk top, pred, eobrun⟩

/-- Correction bits for the already non-zero coefficients in zigzag `k ..`
until the `r`-th zero (`r` = none: to `se`); returns the position reached. -/
def refineRun (d : ByteArray) (blk : Nat) (p1 : UInt32) (se : Nat)
    (b : Bits) (coefs : ByteArray) (k : Nat) (zeros : Option Nat) : Bits × ByteArray × Nat := Id.run do
  let mut b := b
  let mut coefs := coefs
  let mut k := k
  let mut r := zeros.getD 0
  while k ≤ se do
    let pos := (byteAt natural k).toNat
    let c := getCoef coefs blk pos
    if c != 0 then
      let (bit, b1) := getBits d b 1
      b := b1
      if bit != 0 && c &&& p1 == 0 then
        coefs := setCoef coefs blk pos (if c >>> 31 == 0 then c + p1 else c - p1)
    else if zeros.isSome then
      if r == 0 then break
      r := r - 1
    k := k + 1
  return (b, coefs, k)

/-- Progressive AC scan, refinement (jdphuff.c's decode_mcu_AC_refine). -/
def blockACRefine (d : ByteArray) (acT : Huff) (ss se al : Nat) (s : Scan) (blk : Nat) : Scan := Id.run do
  let p1 : UInt32 := (1 : UInt32) <<< al.toUInt32
  let ⟨bits, coefs0, maxk0, pred, eobrun0⟩ := s
  let mut b := bits
  let mut coefs := coefs0
  let mut maxk := maxk0
  let mut eobrun := eobrun0
  let mut k := ss
  if eobrun == 0 then
    while k ≤ se do
      let (rs, b1) := huffDecode d acT b
      b := b1
      let r := (rs >>> 4).toNat
      let sz := rs &&& 15
      let mut newv : UInt32 := 0
      if sz != 0 then
        let (bit, b2) := getBits d b 1
        b := b2
        newv := if bit != 0 then p1 else (0 : UInt32) - p1
      else if r != 15 then
        let (e, b2) := getBits d b r.toUInt32
        b := b2
        eobrun := (1 <<< r) + e.toNat
        break
      -- Skip r zero coefficients (and correct the non-zero ones on the way).
      let (b3, c3, k3) := refineRun d blk p1 se b coefs k (some r)
      b := b3
      coefs := c3
      k := k3
      if newv != 0 then
        let kk := min k 63
        coefs := setCoef coefs blk (byteAt natural k).toNat newv
        maxk := bumpMaxk maxk blk kk
      k := k + 1
  if eobrun > 0 then
    let (b3, c3, _) := refineRun d blk p1 se b coefs k none
    b := b3
    coefs := c3
    eobrun := eobrun - 1
  return ⟨b, coefs, maxk, pred, eobrun⟩

/-! ## Markers and scans -/

/-- After a restart interval: drop the remaining bits and pass the RSTn
marker; also whether one was found. -/
def restartAt (d : ByteArray) (p : Nat) : Nat × Bool := Id.run do
  let mut p := p
  while p + 1 < d.size && !(byteAt d p == 0xFF && 0xD0 ≤ byteAt d (p + 1) && byteAt d (p + 1) ≤ 0xD7) do
    -- Stop at any other marker (libjpeg would resynchronize; we just go on).
    if byteAt d p == 0xFF && byteAt d (p + 1) != 0 && byteAt d (p + 1) != 0xFF then return (p, false)
    p := p + 1
  return if p + 1 < d.size then (p + 2, true) else (p, false)

/-- The end of the entropy-coded segment starting at `p`: the next marker
other than RSTn (and other than stuffed 0xFF00). -/
def segmentEnd (d : ByteArray) (p : Nat) : Nat := Id.run do
  let mut p := p
  while p + 1 < d.size do
    if byteAt d p == 0xFF then
      let m := byteAt d (p + 1)
      if m != 0 && !(0xD0 ≤ m && m ≤ 0xD7) && m != 0xFF then return p
    p := p + 1
  return d.size

/-- Decode one scan (SOS header at `p`, after the marker and length). -/
def scan (d : ByteArray) (st : St) (p len : Nat) : Except String (St × Nat) := do
  let ns := (byteAt d p).toNat
  if ns < 1 || ns > 4 || len != 6 + 2 * ns then throw "JPEG: bad SOS"
  let mut sc : Array (Nat × Nat × Nat) := #[]  -- (component index, dc table, ac table)
  let mut st := st
  for i in [0:ns] do
    let cid := byteAt d (p + 1 + 2 * i)
    let t := byteAt d (p + 2 + 2 * i)
    let mut found := none
    for k in [0:st.comps.size] do
      if st.comps[k]!.id == cid then found := some k
    let some ci := found | throw "JPEG: SOS names an unknown component"
    -- Latch the quantization table at the component's first scan.
    let c := st.comps[ci]!
    if c.q.size == 0 then
      let q := st.qt[c.tq]!
      if q.size == 0 then throw "JPEG: missing quantization table"
      st := { st with comps := st.comps.set! ci { c with q } }
    sc := sc.push (ci, (t >>> 4).toNat &&& 3, (t &&& 15).toNat &&& 3)
  let ss := (byteAt d (p + 1 + 2 * ns)).toNat
  let se := (byteAt d (p + 2 + 2 * ns)).toNat
  let ah := (byteAt d (p + 3 + 2 * ns) >>> 4).toNat
  let al := (byteAt d (p + 3 + 2 * ns) &&& 15).toNat
  if st.progressive then
    if se > 63 || ss > se || (ss == 0 && se != 0) || (ss > 0 && ns != 1) || al > 13 then
      throw "JPEG: bad progressive scan parameters"
  let start := p + len - 2
  -- Move the coefficient arrays out of `st` so that they are not shared.
  let coefs := st.coefs
  let maxk := st.maxk
  st := { st with coefs := ByteArray.empty, maxk := ByteArray.empty }
  let mut s : Scan := { bits := { p := start, buf := 0, cnt := 0 }, coefs, maxk, pred := Array.replicate ns 0,
                        eobrun := 0 }
  let kind : Nat := if !st.progressive then 0 else if ss == 0 then 1 else if ah == 0 then 2 else 3
  -- MCUs: one block of the component if non-interleaved, else the MCU grid.
  let (mx, my) := if ns == 1 then
      let c := st.comps[sc[0]!.1]!
      (c.wib, c.hib)
    else (st.mcusX, st.mcusY)
  let total := mx * my
  let mut todo := st.restart
  for m in [0:total] do
    if st.restart > 0 then
      if todo == 0 then
        let (q, rst) := restartAt d s.bits.p
        -- libjpeg keeps the out-of-data state unless a real RSTn followed.
        s := { s with bits := { p := q, buf := 0, cnt := 0, bad := s.bits.bad && !rst },
                      pred := Array.replicate ns 0, eobrun := 0 }
        todo := st.restart
      todo := todo - 1
    -- Out of data: leave this MCU alone (libjpeg's insufficient_data).
    if s.bits.bad then continue
    let ux := m % mx
    let uy := m / mx
    for j in [0:ns] do
      let (ci, td, ta) := sc[j]!
      let c := st.comps[ci]!
      let (bh, bv) := if ns == 1 then (1, 1) else (c.h, c.v)
      for by_ in [0:bv] do
        for bx in [0:bh] do
          let blk := c.block0 + (uy * bv + by_) * c.bw + (ux * bh + bx)
          s := match kind with
            | 0 => blockSeq d st.dc[td]! st.ac[ta]! s j blk
            | 1 => blockDC d st.dc[td]! ah al s j blk
            | 2 => blockACFirst d st.ac[ta]! ss se al s blk
            | _ => blockACRefine d st.ac[ta]! ss se al s blk
  pure ({ st with coefs := s.coefs, maxk := s.maxk }, segmentEnd d s.bits.p)

def frame (d : ByteArray) (st : St) (p len : Nat) (progressive : Bool) : Except String St := do
  if byteAt d p != 8 then throw "JPEG: only 8-bit samples are supported"
  let height := be16 d (p + 1)
  let width := be16 d (p + 3)
  let nc := (byteAt d (p + 5)).toNat
  if width == 0 || height == 0 then throw "JPEG: zero image size (DNL is not supported)"
  if nc != 1 && nc != 3 then throw "JPEG: only 1 or 3 components are supported"
  if len != 8 + 3 * nc then throw "JPEG: bad SOF length"
  let mut comps : Array Comp := #[]
  for i in [0:nc] do
    let o := p + 6 + 3 * i
    let hv := byteAt d (o + 1)
    let h := (hv >>> 4).toNat
    let v := (hv &&& 15).toNat
    if h < 1 || h > 4 || v < 1 || v > 4 then throw "JPEG: bad sampling factors"
    comps := comps.push { id := byteAt d o, h, v, tq := (byteAt d (o + 2)).toNat &&& 3,
                          bw := 0, bh := 0, wib := 0, hib := 0, dw := 0, dh := 0, block0 := 0, q := ByteArray.empty }
  let hmax := comps.foldl (fun m c => max m c.h) 1
  let vmax := comps.foldl (fun m c => max m c.v) 1
  for c in comps do
    if hmax % c.h != 0 || vmax % c.v != 0 then throw "JPEG: fractional sampling ratios are not supported"
  let mcusX := (width + 8 * hmax - 1) / (8 * hmax)
  let mcusY := (height + 8 * vmax - 1) / (8 * vmax)
  let mut block0 := 0
  let mut cs : Array Comp := #[]
  for c in comps do
    let dw := (width * c.h + hmax - 1) / hmax
    let dh := (height * c.v + vmax - 1) / vmax
    let c := { c with dw, dh, wib := (dw + 7) / 8, hib := (dh + 7) / 8,
                      bw := mcusX * c.h, bh := mcusY * c.v, block0 }
    block0 := block0 + c.bw * c.bh
    cs := cs.push c
  if block0 > 1 <<< 20 then throw "JPEG: image too large"
  pure { st with width, height, progressive, comps := cs, hmax, vmax, mcusX, mcusY,
                 coefs := zeros (128 * block0), maxk := zeros block0 }

/-! ## Output -/

/-- The IDCT of every block of component `c` into a padded plane of
`(bw*8) x (bh*8)` samples. -/
def plane (st : St) (c : Comp) : ByteArray := Id.run do
  let stride := c.bw * 8
  let mut pl := zeros (stride * c.bh * 8)
  -- Only the component's own blocks are needed.
  for by_ in [0:c.hib] do
    for bx in [0:c.wib] do
      let blk := c.block0 + by_ * c.bw + bx
      let o := by_ * 8 * stride + bx * 8
      if byteAt st.maxk blk == 0 then
        pl := dcBlock (getCoef st.coefs blk 0 * s16 c.q 0) pl o stride
      else
        pl := idctBlock st.coefs (64 * blk) c.q pl o stride
  return pl

/-- 3 * nearer + further sample of column `cx` (rows at offsets `no`, `fo`). -/
@[inline] def colsum (pl : ByteArray) (no fo cx : Nat) : UInt32 :=
  (byteAt pl (no + cx)).toUInt32 * 3 + (byteAt pl (fo + cx)).toUInt32

/-- One output row of h2v2_fancy_upsample from input column `cx`
(`last`/`this`: the column sums of columns cx-1 and cx). -/
partial def h2v2Row (pl : ByteArray) (no fo lastX w o : Nat) (out : ByteArray) (cx : Nat)
    (last this : UInt32) : ByteArray :=
  if cx > lastX then out else
  let next := if cx < lastX then colsum pl no fo (cx + 1) else this
  let out := out.set! (o + 2 * cx) ((this * 3 + last + 8) >>> 4).toUInt8
  let out := if 2 * cx + 1 < w then out.set! (o + 2 * cx + 1) ((this * 3 + next + 7) >>> 4).toUInt8 else out
  h2v2Row pl no fo lastX w o out (cx + 1) this next

/-- One output row of h2v1_fancy_upsample (input row at `r`). -/
partial def h2v1Row (pl : ByteArray) (r lastX w o : Nat) (out : ByteArray) (cx : Nat) (last this : UInt32) :
    ByteArray :=
  if cx > lastX then out else
  let next := if cx < lastX then (byteAt pl (r + cx + 1)).toUInt32 else this
  let out := out.set! (o + 2 * cx) ((this * 3 + last + 1) >>> 2).toUInt8
  let out := if 2 * cx + 1 < w then out.set! (o + 2 * cx + 1) ((this * 3 + next + 2) >>> 2).toUInt8 else out
  h2v1Row pl r lastX w o out (cx + 1) this next

/-- Upsample component `c` (plane `pl`) to full size, libjpeg's way. Writes
into a preallocated array (`set!` is inline; `push` is a runtime call). -/
def upsample (st : St) (c : Comp) (pl : ByteArray) : ByteArray := Id.run do
  let w := st.width
  let h := st.height
  let stride := c.bw * 8
  let hx := st.hmax / c.h
  let vx := st.vmax / c.v
  if hx == 1 && vx == 1 then
    if stride == w then return pl.extract 0 (w * h)
    let mut out := ByteArray.emptyWithCapacity (w * h)
    for y in [0:h] do
      out := pl.copySlice (y * stride) out out.size w false
    return out
  let mut out := zeros (w * h)
  let lastX := c.dw - 1
  let lastY := c.dh - 1
  if hx == 2 && vx == 2 && c.dw > 2 then
    -- h2v2_fancy_upsample: 9/16, 3/16, 3/16, 1/16 with edge replication,
    -- one column sum per input column (sliding window, as libjpeg).
    for y in [0:h] do
      let near := y / 2
      let far := if y % 2 == 0 then (if near == 0 then 0 else near - 1) else min (near + 1) lastY
      let c0 := colsum pl (near * stride) (far * stride) 0
      out := h2v2Row pl (near * stride) (far * stride) lastX w (y * w) out 0 c0 c0
  else if hx == 2 && vx == 1 && c.dw > 2 then
    -- h2v1_fancy_upsample: 3/4 nearer + 1/4 further.
    for y in [0:h] do
      let r := y * stride
      let c0 := (byteAt pl r).toUInt32
      out := h2v1Row pl r lastX w (y * w) out 0 c0 c0
  else if hx == 1 && vx == 2 then
    -- h1v2_fancy_upsample.
    for y in [0:h] do
      let near := y / 2
      let (far, bias) := if y % 2 == 0 then ((if near == 0 then 0 else near - 1), 1)
        else (min (near + 1) lastY, 2)
      let o := y * w
      let no := near * stride
      let fo := far * stride
      for x in [0:w] do
        out := out.set! (o + x) ((colsum pl no fo x + bias) >>> 2).toUInt8
  else
    -- Box filter (h2v1_upsample, h2v2_upsample, int_upsample).
    for y in [0:h] do
      let o := y * w
      let r := (y / vx) * stride
      for x in [0:w] do
        out := out.set! (o + x) (byteAt pl (r + x / hx))
  return out

/-- Interleave the full-size components into RGB. -/
def toRgb (st : St) (planes : Array ByteArray) : ByteArray := Id.run do
  let n := st.width * st.height
  let mut out := zeros (3 * n)
  if planes.size == 1 then
    let y := planes[0]!
    for i in [0:n] do
      let v := byteAt y i
      out := ((out.set! (3 * i) v).set! (3 * i + 1) v).set! (3 * i + 2) v
    return out
  let y := planes[0]!
  let cb := planes[1]!
  let cr := planes[2]!
  let isRgb := match st.adobe with
    | some t => !st.jfif && t == 0
    | none => !st.jfif && st.comps[0]!.id == 82 && st.comps[1]!.id == 71 && st.comps[2]!.id == 66
  if isRgb then
    for i in [0:n] do
      out := ((out.set! (3 * i) (byteAt y i)).set! (3 * i + 1) (byteAt cb i)).set! (3 * i + 2) (byteAt cr i)
  else
    for i in [0:n] do
      out := setRgb out (3 * i) (byteAt y i) (byteAt cb i) (byteAt cr i)
  return out

/-! ## Entry points -/

/-- Decode a JPEG file to RGB. -/
def decode (d : ByteArray) : Except String Image := do
  if byteAt d 0 != 0xFF || byteAt d 1 != 0xD8 then throw "JPEG: no SOI marker"
  let mut st : St := {}
  let mut p := 2
  let mut sawFrame := false
  let mut sawScan := false
  -- Markers until EOI (or the end of the data).
  while p + 3 < d.size do
    if byteAt d p != 0xFF then
      p := p + 1  -- junk between markers: skip, as libjpeg (with a warning)
      continue
    let m := byteAt d (p + 1)
    if m == 0xFF then
      p := p + 1  -- fill byte
      continue
    if m == 0xD9 then break
    if m == 0xD8 || m == 0x01 || (0xD0 ≤ m && m ≤ 0xD7) then
      p := p + 2
      continue
    let len := be16 d (p + 2)
    if len < 2 || p + 2 + len > d.size then throw "JPEG: truncated marker segment"
    let q := p + 4
    if m == 0xC0 || m == 0xC1 || m == 0xC2 then
      if sawFrame then throw "JPEG: more than one frame"
      st ← frame d st q len (m == 0xC2)
      sawFrame := true
      p := p + 2 + len
    else if m == 0xC3 || (0xC5 ≤ m && m ≤ 0xCF && m != 0xC8 && m != 0xCC) then
      throw "JPEG: lossless, hierarchical and arithmetic-coded JPEGs are not supported"
    else if m == 0xC4 then
      let mut i := q
      while i < p + 2 + len do
        let tc := byteAt d i >>> 4
        let th := (byteAt d i &&& 15).toNat
        if th > 3 || tc > 1 then throw "JPEG: bad DHT"
        let bits := d.extract (i + 1) (i + 17)
        let mut n := 0
        for k in [0:16] do n := n + (byteAt bits k).toNat
        if i + 17 + n > p + 2 + len then throw "JPEG: bad DHT"
        let t ← Huff.build bits (d.extract (i + 17) (i + 17 + n))
        st := if tc == 0 then { st with dc := st.dc.set! th t } else { st with ac := st.ac.set! th t }
        i := i + 17 + n
      p := p + 2 + len
    else if m == 0xDB then
      let mut i := q
      while i < p + 2 + len do
        let pq := byteAt d i >>> 4
        let tq := (byteAt d i &&& 15).toNat
        if tq > 3 || pq > 1 then throw "JPEG: bad DQT"
        let mut t := zeros 128
        for k in [0:64] do
          let v := if pq == 0 then (byteAt d (i + 1 + k)).toNat else be16 d (i + 1 + 2 * k)
          let nk := (byteAt natural k).toNat
          t := (t.set! (2 * nk) v.toUInt8).set! (2 * nk + 1) (v >>> 8).toUInt8
        st := { st with qt := st.qt.set! tq t }
        i := i + 1 + (if pq == 0 then 64 else 128)
      p := p + 2 + len
    else if m == 0xDD then
      st := { st with restart := be16 d q }
      p := p + 2 + len
    else if m == 0xDA then
      if !sawFrame then throw "JPEG: SOS before SOF"
      let (st2, e) ← scan d st q len
      st := st2
      sawScan := true
      p := e
    else
      if m == 0xE0 && len ≥ 7 && d.extract q (q + 5) == "JFIF\x00".toUTF8 then st := { st with jfif := true }
      if m == 0xEE && len ≥ 12 && d.extract q (q + 5) == "Adobe".toUTF8 then
        st := { st with adobe := some (byteAt d (q + 11)) }
      p := p + 2 + len
  if !sawScan then throw "JPEG: no image data"
  let planes := st.comps.map fun c => upsample st c (plane st c)
  pure { width := st.width, height := st.height, rgb := toRgb st planes }

/-- browse.rs's nearest-neighbour scale of an RGB image to `dw` x `dh`:
source pixel `(x * w / dw, y * h / dh)`. -/
def scaleNearest (img : Image) (dw dh : Nat) : ByteArray := Id.run do
  let mut out := zeros (dw * dh * 3)
  for y in [0:dh] do
    let row := (y * img.height / dh) * img.width
    let o := y * dw * 3
    for x in [0:dw] do
      let s := (row + x * img.width / dw) * 3
      out := ((out.set! (o + 3 * x) (byteAt img.rgb s)).set! (o + 3 * x + 1) (byteAt img.rgb (s + 1))).set!
        (o + 3 * x + 2) (byteAt img.rgb (s + 2))
  return out

end Yt.Jpeg
