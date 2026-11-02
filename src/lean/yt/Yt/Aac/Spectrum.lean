import Yt.Aac.Ics

/-! Spectral reconstruction in block floating point: inverse quantization,
perceptual noise substitution, mid/side and intensity stereo, and TNS.

A channel's 1024 coefficients are `Int32` words sharing one exponent `e`
(value = word · 2^e in the spec's units, where the 2/N-normalised IMDCT gives
16-bit PCM). The exponent is chosen from per-band magnitude bounds so that
every word is below 2^28 after inverse quantization, leaving room for M/S.
TNS renormalises the channel first (`Decoder.tnsChannel`) because its
all-pole filters amplify quantization error; outputs saturate rather than
wrap, and the caller retries with more headroom if one did. -/

namespace Yt.Aac

structure Consts where
  /-- Words: x^(4/3) mantissas, Q30 normalised (see `Math.pow43`). -/
  pow43M : ByteArray
  /-- Bytes: x^(4/3) exponents. -/
  pow43E : ByteArray
  /-- Words: 2^(r/4), Q30. -/
  frac : ByteArray

/-- No bound (band is silent). -/
def noBound : Int32 := -100000

/-- Upper bound `b` with |value| < 2^b for every coefficient of band `i`. -/
def bandBound (c : Consts) (ics : Ics) (i : USize) : Int32 :=
  let t := ld8 ics.bandType i
  if t ≥ 1 && t ≤ 11 then
    let m := ld32 ics.bmax i
    if m == 0 then noBound
    else (ld8 c.pow43E m.toUSize).toUInt32.toInt32 + ((ldI ics.sf i - 100) >>> (2 : Int32)) + 2
  else if t == noiseHcb then (ldI ics.sf i >>> (2 : Int32)) + 1
  else noBound

@[inline] private def imax (a b : Int32) : Int32 := if a > b then a else b

/-- Largest band bound of a channel (bands `g * 64 + sfb`). -/
def channelBound (c : Consts) (ics : Ics) : Int32 := Id.run do
  let mut b := noBound
  for g in [0:ics.info.groupLen.size] do
    for sfb in [0:ics.info.maxSfb] do
      b := imax b (bandBound c ics (g * 64 + sfb).toUSize)
  return b

/-- Bound of the intensity-coded bands of the second channel of a pair. -/
def intensityBound (c : Consts) (l r : Ics) : Int32 := Id.run do
  let mut b := noBound
  for g in [0:r.info.groupLen.size] do
    for sfb in [0:r.info.maxSfb] do
      let i := (g * 64 + sfb).toUSize
      let t := ld8 r.bandType i
      if t == intensityHcb || t == intensityHcb2 then
        let bl := bandBound c l i
        if bl != noBound then b := imax b (bl + ((0 - ldI r.sf i) >>> (2 : Int32)) + 1)
  return b

/-- Exponent for a bound: words stay below 2^28. -/
def expForBound (b : Int32) : Int32 := if b == noBound then 0 else b - 28

/-- round(p / 2^sh) for a nonnegative p < 2^62, saturated to Int32. -/
@[inline] private def scaleDown (p : UInt64) (sh : Int32) : Int32 :=
  if sh ≥ 63 then 0
  else if sh ≤ 1 then 2147483647
  else
    let s := sh.toUInt32.toUInt64
    let v := (p + ((1 : UInt64) <<< (s - 1))) >>> s
    if v ≥ 2147483647 then 2147483647 else v.toUInt32.toInt32

/-- Inverse quantization of words `[k, kEnd)`: x ↦ sign(x)·|x|^(4/3)·F·2^(−base). -/
private partial def deqLoop (m e8 q : ByteArray) (k kEnd : USize) (f : UInt64) (base : Int32) :
    ByteArray :=
  if k < kEnd then
    let v := ldI q k
    if v == 0 then deqLoop m e8 q (k + 1) kEnd f base
    else
      let a := iabs v
      let p := (ld32 m a.toUSize).toUInt64 * f
      let x := scaleDown p (base - (ld8 e8 a.toUSize).toUInt32.toInt32)
      deqLoop m e8 (stI q k (if v < 0 then 0 - x else x)) (k + 1) kEnd f base
  else q

/-- FFmpeg's PNS generator: state ← state·1664525 + 1013904223. Words
`[k, kEnd)` get the generated values (as Int32, shifted right 3); returns the
energy of the shifted values. -/
private partial def noiseFill (q : ByteArray) (k kEnd : USize) (rand : UInt32) (en : UInt64) :
    ByteArray × UInt32 × UInt64 :=
  if k < kEnd then
    let rand := rand * 1664525 + 1013904223
    let v := rand.toInt32 >>> (3 : Int32)
    noiseFill (stI q k v) (k + 1) kEnd rand (en + sx v * sx v)
  else (q, rand, en)

private partial def isqrt64 (n : UInt64) (x : UInt64) : UInt64 :=
  let y := (x + n / x) / 2
  if y < x then isqrt64 n y else x

private partial def noiseScale (q : ByteArray) (k kEnd : USize) (inv f : UInt64) (sh : Int32) :
    ByteArray :=
  if k < kEnd then
    let v := ldI q k
    let u := ((iabs v).toUInt64 * inv) >>> 31
    let x := scaleDown (u * f) sh
    noiseScale (stI q k (if v < 0 then 0 - x else x)) (k + 1) kEnd inv f sh
  else q

/-- Inverse-quantize a channel in place with exponent `e`, generating noise
for PNS bands (in FFmpeg's order, so the generator state matches). -/
def dequantize (c : Consts) (ics : Ics) (e : Int32) (rand : UInt32) : ByteArray × UInt32 := Id.run do
  let mut q := ics.q
  let mut rand := rand
  let mut win := 0
  let info := ics.info
  for g in [0:info.groupLen.size] do
    let glen := info.groupLen[g]!
    for sfb in [0:info.maxSfb] do
      let i := (g * 64 + sfb).toUSize
      let t := ld8 ics.bandType i
      let lo := info.swb[sfb]!
      let hi := info.swb[sfb + 1]!
      if t ≥ 1 && t ≤ 11 then
        let s := ldI ics.sf i - 100
        let f := (ld32 c.frac (s &&& 3).toUInt32.toUSize).toUInt64
        let base : Int32 := 60 + e - (s >>> (2 : Int32))
        for w in [win:win + glen] do
          q := deqLoop c.pow43M c.pow43E q (w * 128 + lo).toUSize (w * 128 + hi).toUSize f base
      else if t == noiseHcb then
        let nrg := ldI ics.sf i
        let f := (ld32 c.frac (nrg &&& 3).toUInt32.toUSize).toUInt64
        let sh : Int32 := 60 + e - (nrg >>> (2 : Int32))
        for w in [win:win + glen] do
          let k0 := (w * 128 + lo).toUSize
          let k1 := (w * 128 + hi).toUSize
          let (q1, r1, en) := noiseFill q k0 k1 rand 0
          rand := r1
          let s := if en == 0 then 0 else isqrt64 en 0x100000000
          q := if s == 0 then clear32 q1 k0 k1 else noiseScale q1 k0 k1 (0x2000000000000000 / s) f sh
    win := win + glen
  return (q, rand)

private partial def msLoop (l r : ByteArray) (k kEnd : USize) : ByteArray × ByteArray :=
  if k < kEnd then
    let m := ldI l k
    let s := ldI r k
    msLoop (stI l k (m + s)) (stI r k (m - s)) (k + 1) kEnd
  else (l, r)

/-- Mid/side stereo on bands with `ms_used` where neither channel is noise
or intensity coded. -/
def midSide (info : IcsInfo) (ms btL btR : ByteArray) (l r : ByteArray) : ByteArray × ByteArray := Id.run do
  let mut l := l
  let mut r := r
  let mut win := 0
  for g in [0:info.groupLen.size] do
    let glen := info.groupLen[g]!
    for sfb in [0:info.maxSfb] do
      let i := (g * 64 + sfb).toUSize
      if ld8 ms i == 1 && ld8 btL i < noiseHcb && ld8 btR i < noiseHcb then
        for w in [win:win + glen] do
          let (l1, r1) := msLoop l r (w * 128 + info.swb[sfb]!).toUSize (w * 128 + info.swb[sfb + 1]!).toUSize
          l := l1
          r := r1
    win := win + glen
  return (l, r)

private partial def isLoop (l r : ByteArray) (k kEnd : USize) (f : UInt64) (neg : Bool) (sh : Int32) :
    ByteArray :=
  if k < kEnd then
    let p := sx (ldI l k) * f
    let x : Int32 :=
      if sh ≥ 63 then 0
      else if sh ≥ 1 then rshr64 p sh.toUInt32.toUInt64
      else sat32 (p <<< (0 - sh).toUInt32.toUInt64)
    isLoop l (stI r k (if neg then 0 - x else x)) (k + 1) kEnd f neg sh
  else r

/-- Intensity stereo: the right channel's intensity bands are the scaled left
channel (using the right channel's window grouping, as FFmpeg). -/
def intensity (c : Consts) (rics : Ics) (ms : ByteArray) (msPresent : Bool) (l r : ByteArray) : ByteArray := Id.run do
  let info := rics.info
  let mut r := r
  let mut win := 0
  for g in [0:info.groupLen.size] do
    let glen := info.groupLen[g]!
    for sfb in [0:info.maxSfb] do
      let i := (g * 64 + sfb).toUSize
      let t := ld8 rics.bandType i
      if t == intensityHcb || t == intensityHcb2 then
        let flip := msPresent && ld8 ms i == 1
        let neg := (t == intensityHcb2) != flip
        -- scale 2^(−pos/4) = F[(−pos) & 3] · 2^(((−pos) >> 2) − 30)
        let gpos := 0 - ldI rics.sf i
        let f := (ld32 c.frac (gpos &&& 3).toUInt32.toUSize).toUInt64
        let sh : Int32 := 30 - (gpos >>> (2 : Int32))
        for w in [win:win + glen] do
          r := isLoop l r (w * 128 + info.swb[sfb]!).toUSize (w * 128 + info.swb[sfb + 1]!).toUSize f neg sh
    win := win + glen
  return r

/-! ### TNS -/

/-- (k·a) >> 31 for a Q31 `k` and a 64-bit two's complement `a` (|a| < 2^62). -/
@[inline] private def mulK (k : Int32) (a : UInt64) : UInt64 :=
  let hi := sar64 a 32
  let lo := a &&& 0xFFFFFFFF
  ((sx k * hi) <<< 1) + sar64 (sx k * lo) 31

/-- LPC coefficients a[1..order] in Q40 from reflection coefficients (the
spec's step-up recursion). -/
def lpcQ40 (ks : Array Int32) : Array UInt64 := Id.run do
  let mut a : Array UInt64 := #[]
  for m in [0:ks.size] do
    let k := ks.getD m 0
    let mut b : Array UInt64 := #[]
    for i in [0:m] do
      b := b.push (a.getD i 0 + mulK k (a.getD (m - 1 - i) 0))
    a := b.push (sx k <<< 9)
  return a

@[inline] private def absU64 (x : UInt64) : UInt64 := if x >>> 63 == 1 then 0 - x else x

private partial def tnsTaps (x : ByteArray) (pos : USize) (down : Bool) (lpc : Array Int32)
    (i n : Nat) (acc : UInt64) : UInt64 :=
  if i < n then
    let off := (i + 1).toUSize
    let y := ldI x (if down then pos + off else pos - off)
    tnsTaps x pos down lpc (i + 1) n (acc - sx (lpc.getD i 0) * sx y)
  else acc

/-- All-pole filtering of `size` words from `pos` (upwards or downwards).
The flag records whether any output had to be saturated. -/
private partial def tnsRun (x : ByteArray) (pos : USize) (down : Bool) (lpc : Array Int32) (qf : UInt64)
    (m size : Nat) (sat : Bool) : ByteArray × Bool :=
  if m < size then
    let n := if m < lpc.size then m else lpc.size
    let acc := tnsTaps x pos down lpc 0 n (sx (ldI x pos) <<< qf)
    let v := sar64 (acc + ((1 : UInt64) <<< (qf - 1))) qf
    let y := sat32 v
    tnsRun (stI x pos y) (if down then pos - 1 else pos + 1) down lpc qf (m + 1) size
      (sat || sx y != v)
  else (x, sat)

/-- Apply the TNS filters of a channel in place (FFmpeg's band limits).
Returns whether an output saturated (the caller then retries with more
headroom). -/
def applyTns (ics : Ics) (rateIndex : Nat) (x : ByteArray) : ByteArray × Bool := Id.run do
  let info := ics.info
  let maxBands := if info.short then Tables.tnsMaxBandsShort.getD rateIndex 14
    else Tables.tnsMaxBandsLong.getD rateIndex 39
  let mmm := min maxBands info.maxSfb
  if mmm == 0 then return (x, false)
  let mut x := x
  let mut sat := false
  let mut bottom := info.numSwb
  let mut lastWin := 1000
  for f in ics.tns do
    if f.win != lastWin then
      bottom := info.numSwb
      lastWin := f.win
    let top := bottom
    bottom := if top > f.length then top - f.length else 0
    if f.order == 0 then continue
    let start := info.swb[min bottom mmm]!
    let stop := info.swb[min top mmm]!
    if stop ≤ start then continue
    let a := lpcQ40 f.coefs
    -- Q of the int32 taps: as many fraction bits (≤ 31) as keep |tap| < 2^30.
    let big := a.foldl (fun m v => let u := absU64 v; if u > m then u else m) 0
    let mut shift : UInt64 := 9
    for _ in [0:40] do
      if (big >>> shift) ≥ 0x40000000 then shift := shift + 1
    let lpc := a.map (fun v => lo32 (sar64 (v + ((1 : UInt64) <<< (shift - 1))) shift))
    let qf := 40 - shift
    let base := f.win * 128
    let (x1, s1) := if f.down then tnsRun x (base + stop - 1).toUSize true lpc qf 0 (stop - start) false
      else tnsRun x (base + start).toUSize false lpc qf 0 (stop - start) false
    x := x1
    sat := sat || s1
  return (x, sat)

end Yt.Aac
