import Yt.Aac.Buf

/-! Integer generation of the decoder's constant tables.

Everything is computed from integers on first use, so the host and the TV get
bit-identical tables: π by Machin's formula and 2^(r/4) by nested integer
square roots (`Nat`, a handful of operations), then every per-entry table in
`UInt64` fixed point with an exact 64×64→128-bit product (`mulShr`), which
allocates nothing (our runtime's big naturals allocate per operation):
sine/cosine by Taylor series in Q62, Kaiser-Bessel-derived windows from the
Bessel series, x^(4/3) from a Newton iteration for x^(−1/3). Results are
rounded to Q31 (Q30 for x^(4/3)). -/

namespace Yt.Aac.Math

/-- `⌊a·b / 2^s⌋` for `s < 64`, assuming the result fits in 64 bits. -/
@[inline] def mulShr (a b : UInt64) (s : UInt64) : UInt64 :=
  let a0 := a &&& 0xFFFFFFFF
  let a1 := a >>> 32
  let b0 := b &&& 0xFFFFFFFF
  let b1 := b >>> 32
  let p00 := a0 * b0
  let p01 := a0 * b1
  let p10 := a1 * b0
  let mid := (p00 >>> 32) + (p01 &&& 0xFFFFFFFF) + (p10 &&& 0xFFFFFFFF)
  let lo := (mid <<< 32) ||| (p00 &&& 0xFFFFFFFF)
  let hi := a1 * b1 + (p01 >>> 32) + (p10 >>> 32) + (mid >>> 32)
  if s == 0 then lo else (hi <<< (64 - s)) ||| (lo >>> s)

/-- `⌊√n⌋`. -/
def isqrt64 (n : UInt64) : UInt64 := go 0xFFFFFFFF 64
where
  go (x : UInt64) : Nat → UInt64
    | 0 => x
    | fuel + 1 =>
      let y := (x + n / x) / 2
      if y < x then go y fuel else x

/-- `⌊cum · 2^62 / total⌋` for `cum ≤ total < 2^62`, by long division. -/
def ratioQ62 (cum total : UInt64) : UInt64 := go cum 0 62
where
  go (rem q : UInt64) : Nat → UInt64
    | 0 => q
    | k + 1 =>
      let rem := rem <<< 1
      if rem ≥ total then go (rem - total) ((q <<< 1) ||| 1) k else go rem (q <<< 1) k

/-- `2^k` as a `Nat`. -/
def pow2 (k : Nat) : Nat := go 1 k
where go (a : Nat) : Nat → Nat
  | 0 => a
  | n + 1 => go (2 * a) n

/-- Integer square root of a `Nat`. -/
def isqrt (n : Nat) : Nat :=
  if n < 2 then n else
  let rec up (g : Nat) (fuel : Nat) : Nat :=
    match fuel with
    | 0 => g
    | f + 1 => if g * g < n then up (2 * g) f else g
  let rec newton (x : Nat) (fuel : Nat) : Nat :=
    match fuel with
    | 0 => x
    | f + 1 =>
      let y := (x + n / x) / 2
      if y < x then newton y f else x
  newton (up 1 70) 200

/-- atan(1/x) in fixed point `one` (truncated series; `x ≥ 2`). -/
def atanInv (x : Nat) (one : Nat) : Nat := go (one / x) 1 0 0 true 400
where
  go (pw : Nat) (k : Nat) (pos neg : Nat) (plus : Bool) : Nat → Nat
    | 0 => pos - neg
    | fuel + 1 =>
      if pw == 0 then pos - neg
      else
        let t := pw / k
        let pos := if plus then pos + t else pos
        let neg := if plus then neg else neg + t
        go (pw / (x * x)) (k + 2) pos neg (!plus) fuel

/-- π · 2^62, rounded (Machin: π = 16 atan(1/5) − 4 atan(1/239), computed in Q72). -/
def piQ62 (_ : Unit) : UInt64 :=
  let one := pow2 72
  let p := 16 * atanInv 5 one - 4 * atanInv 239 one
  ((p + 512) / 1024).toUInt64

def q62 : UInt64 := 0x4000000000000000

/-- Taylor sum continuing from term `t` (index k): next term `t·x²/(k(k+1))`,
alternating signs. -/
def series (x2 t : UInt64) (k : UInt64) (sum : UInt64) (plus : Bool) : Nat → UInt64
  | 0 => sum
  | fuel + 1 =>
    let t := mulShr t x2 62 / (k * (k + 1))
    if t == 0 then sum
    else series x2 t (k + 2) (if plus then sum + t else sum - t) (!plus) fuel

/-- `(cos x, sin x)` in Q62 for x in Q62, 0 ≤ x ≤ π/4. -/
def cosSinQ62 (x : UInt64) : UInt64 × UInt64 :=
  let x2 := mulShr x x 62
  (series x2 q62 1 q62 false 40, series x2 x 2 x false 40)

/-- `π·p/q` in Q62, rounded (for results below 2^63). -/
@[inline] def piFrac (pi p q : UInt64) : UInt64 :=
  (pi / q) * p + ((pi % q) * p * 2 + q) / (2 * q)

/-- `(cos θ, sin θ)` in Q62 for θ = π·p/q ∈ [0, π/2]. -/
def cosSinPi (pi : UInt64) (p q : Nat) : UInt64 × UInt64 :=
  if 4 * p ≤ q then cosSinQ62 (piFrac pi p.toUInt64 q.toUInt64)
  else
    let (c, s) := cosSinQ62 (piFrac pi (q - 2 * p).toUInt64 (2 * q).toUInt64)
    (s, c)

/-- Q62 → Q31, rounded, at most 2^31 − 1. -/
def toQ31 (v : UInt64) : Int32 :=
  let r := (v + 0x40000000) >>> 31
  (if r ≥ 0x80000000 then 0x7FFFFFFF else r).toUInt32.toInt32

@[inline] def negQ31 (v : Int32) : Int32 := 0 - v

/-- Words `[0, n)`: `sin(π(i + 1/2)/(2n))` in Q31, the rising half of a
2n-point sine window. -/
def sineWindow (n : Nat) : ByteArray := Id.run do
  let pi := piQ62 ()
  let mut b := zeros32 n
  for i in [0:n] do
    b := stI b i.toUSize (toQ31 (cosSinPi pi (2 * i + 1) (4 * n)).2)
  return b

/-- Σ_k t^k/(k!)² in Q28 (t in Q48), continuing from `term` at index `k`. -/
def bessel (t term sum k : UInt64) : Nat → UInt64
  | 0 => sum
  | fuel + 1 =>
    let term := mulShr term t 48 / (k * k)
    if term == 0 then sum else bessel t term (sum + term) (k + 1) fuel

/-- Words `[0, n)`: the rising half of a 2n-point Kaiser-Bessel-derived
window with parameter `alpha` (ISO/IEC 14496-3 4.6.11.3.2), Q31; `n` a power
of two up to 1024. W'(j) = I0(πα·sqrt(1 − (2j/n − 1)²)) = Σ t^k/(k!)² with
t = (πα)²·j(n−j)/n², and the window is sqrt(Σ_{i≤j} W'(i) / Σ_{i≤n} W'(i)). -/
def kbdWindow (n alpha : Nat) : ByteArray := Id.run do
  let pa := (piQ62 () >>> 14) * alpha.toUInt64         -- πα, Q48
  let pa2 := mulShr pa pa 48                           -- (πα)², Q48
  let shift := (2 * n.log2).toUInt64
  let one : UInt64 := 0x10000000                       -- 1.0 in Q28
  let mut w := zeros32 (2 * (n + 1))
  let mut total : UInt64 := 0
  for j in [0:n + 1] do
    let t := mulShr pa2 (j * (n - j)).toUInt64 shift
    let v := bessel t one one 1 300
    w := st32 (st32 w (2 * j).toUSize v.toUInt32) (2 * j + 1).toUSize (v >>> 32).toUInt32
    total := total + v
  let mut b := zeros32 n
  let mut cum : UInt64 := 0
  for j in [0:n] do
    cum := cum + ((ld32 w (2 * j).toUSize).toUInt64 ||| ((ld32 w (2 * j + 1).toUSize).toUInt64 <<< 32))
    let v := isqrt64 (ratioQ62 cum total)
    b := stI b j.toUSize (if v ≥ 0x80000000 then 0x7FFFFFFF else v.toUInt32.toInt32)
  return b

/-- IMDCT pre/post-rotation for an N-point transform: words `2i, 2i+1` are
`cos, sin` of `2π(i + 1/8)/N`, `i < N/4`, Q31. -/
def imdctTwiddles (bigN : Nat) : ByteArray := Id.run do
  let pi := piQ62 ()
  let n4 := bigN / 4
  let mut b := zeros32 (2 * n4)
  for i in [0:n4] do
    let (c, s) := cosSinPi pi (8 * i + 1) (4 * bigN)
    b := stI b (2 * i).toUSize (toQ31 c)
    b := stI b (2 * i + 1).toUSize (toQ31 s)
  return b

/-- FFT twiddles for an n-point transform: words `2k, 2k+1` are `cos, sin`
of `2πk/n`, `k < n/2`, Q31. -/
def fftTwiddles (n : Nat) : ByteArray := Id.run do
  let pi := piQ62 ()
  let mut b := zeros32 n
  for k in [0:n / 2] do
    let (c, s) :=
      if 4 * k ≤ n then
        let (c, s) := cosSinPi pi (2 * k) n
        (toQ31 c, toQ31 s)
      else
        -- θ − π/2 = π(4k − n)/(2n): cos θ = −sin(θ − π/2), sin θ = cos(θ − π/2)
        let (c, s) := cosSinPi pi (4 * k - n) (2 * n)
        (negQ31 (toQ31 s), toQ31 c)
    b := stI b (2 * k).toUSize c
    b := stI b (2 * k + 1).toUSize s
  return b

/-- Bit-reversal permutation of `log2 n` bits, as words. -/
def bitReverse (n : Nat) : ByteArray := Id.run do
  let bits := n.log2
  let mut b := zeros32 n
  for k in [0:n] do
    let mut r := 0
    let mut x := k
    for _ in [0:bits] do
      r := 2 * r + x % 2
      x := x / 2
    b := st32 b k.toUSize r.toUInt32
  return b

/-- Newton for r = x^(−1/3) in Q62 from a nearby estimate:
r ← r + r(1 − x r³)/3. -/
def rcbrt (x : UInt64) (r : UInt64) : Nat → UInt64
  | 0 => r
  | fuel + 1 =>
    let r3 := mulShr (mulShr r r 62) r 62
    let xr3 := x * r3
    let up := xr3 ≤ q62
    let d := mulShr r (if up then q62 - xr3 else xr3 - q62) 62 / 3
    -- Stop once the step is below 2^-50 relative: x·r³ carries about x ulps of
    -- truncation, so the step never reaches 0.
    if d ≤ 4096 then r else rcbrt x (if up then r + d else r - d) fuel

/-- `x^(4/3) = M[x] · 2^(E[x] − 30)` for `0 < x < 8208` (escape values plus
pulse amplitudes; `size` = 8208), with `2^30 ≤ M[x] < 2^31` (words of the first array,
rounded) and `E[x] = ⌊log2 x^(4/3)⌋` (bytes of the second). -/
def pow43Table (size : Nat) : ByteArray × ByteArray := Id.run do
  let mut m := st32 (zeros32 size) 1 0x40000000
  let mut e := zeros size
  let mut r := q62
  for x in [2:size] do
    let xu := x.toUInt64
    -- the previous x's root is a close estimate from above
    r := rcbrt xu r 60
    -- x^(4/3) = x² r²; with E = ⌊log2(x⁴)/3⌋, M = round(x² r² / 2^(32 + E))
    let ex := (xu * xu * xu * xu).log2 / 3
    let v := mulShr (xu * xu) (mulShr r r 62) (31 + ex)
    let mv := (v + 1) >>> 1
    m := st32 m x.toUSize (if mv ≥ 0x80000000 then 0x7FFFFFFF else mv.toUInt32)
    e := st8 e x.toUSize ex.toUInt8
  return (m, e)

def pow43 (_ : Unit) : ByteArray × ByteArray := pow43Table 8208

/-- `2^(r/4)` in Q30 for r = 0..3 (floor): √√(2^(120+r)). -/
def fracPow2 (_ : Unit) : ByteArray := Id.run do
  let mut b := zeros32 4
  for r in [0:4] do
    b := st32 b r.toUSize (isqrt (isqrt (pow2 (120 + r)))).toUInt32
  return b

/-- TNS reflection coefficients (ISO/IEC 14496-3 4.6.9.3), Q31: for a
coefficient resolution of `res` bits (3 or 4), entry `c + 8` holds
`sin(c / iqfac)` with `iqfac = (2^(res−1) ∓ 1/2)/(π/2)` for `c ≥ 0` / `c < 0`,
for `−8 ≤ c < 8`. -/
def tnsCoefs (res : Nat) : ByteArray := Id.run do
  let pi := piQ62 ()
  let half := if res == 4 then 8 else 4
  let mut b := zeros32 16
  for i in [0:16] do
    if i ≥ 8 then
      let c := i - 8
      -- c/iqfac = π c / (2 half − 1)
      if c < half then
        b := stI b i.toUSize (toQ31 (cosSinPi pi c (2 * half - 1)).2)
    else
      let c := 8 - i
      if c ≤ half then
        b := stI b i.toUSize (negQ31 (toQ31 (cosSinPi pi c (2 * half + 1)).2))
  return b

end Yt.Aac.Math
