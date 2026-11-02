/-! Integer buffers and fixed-point arithmetic for the AAC decoder.

The decoder is integer-only: our runtime implements no `Float` arithmetic
(`lean_float_add` and friends are absent), and integer code gives the same
bits on the host and on 32-bit ARM. Working buffers are `ByteArray`s holding
little-endian 32-bit words (an `Array Int32` would box every element on a
32-bit target). Accessors are bounds-checked once per word; out-of-range
reads give 0 and out-of-range writes are ignored.

Signed 64-bit arithmetic uses `UInt64` two's complement (our runtime lacks
most `Int64` operations): `sx` sign-extends, and products, sums and
right shifts of at most 32 bits followed by truncation to 32 bits agree with
the signed results. -/

namespace Yt.Aac

theorem two_pow_nb_ge : 2 ^ System.Platform.numBits ≥ 2 ^ 32 := by
  rcases System.Platform.numBits_eq with h | h <;> rw [h] <;> decide

theorem usize_lit (n : Nat) (h : n < 2 ^ 32) : (OfNat.ofNat n : USize).toNat = n := by
  rw [USize.toNat_ofNat]; exact Nat.mod_eq_of_lt (Nat.lt_of_lt_of_le h two_pow_nb_ge)

theorem usize_lt (a : ByteArray) (i : USize) (h : i < a.usize) : i.toNat < a.size := by
  have h1 : i.toNat < a.usize.toNat := h
  simp [ByteArray.usize] at h1
  exact Nat.lt_of_lt_of_le h1 (Nat.mod_le _ _)

theorem idx_main (a : ByteArray) (i : USize) (h : i < a.usize >>> 2) :
    i.toNat * 4 + 3 < a.size % 2 ^ System.Platform.numBits ∧
    a.size % 2 ^ System.Platform.numBits ≤ a.size ∧
    a.size % 2 ^ System.Platform.numBits < 2 ^ System.Platform.numBits := by
  have hu : a.usize.toNat = a.size % 2 ^ System.Platform.numBits := by simp [ByteArray.usize]
  have hM := two_pow_nb_ge
  have hlt : a.size % 2 ^ System.Platform.numBits < 2 ^ System.Platform.numBits :=
    Nat.mod_lt _ (by omega)
  have hle : a.size % 2 ^ System.Platform.numBits ≤ a.size := Nat.mod_le _ _
  have h1 : i.toNat < a.usize.toNat >>> (2 % System.Platform.numBits) := by
    have := (USize.lt_iff_toNat_lt).mp h
    simpa [USize.toNat_shiftRight] using this
  have h2 : 2 % System.Platform.numBits = 2 := by
    rcases System.Platform.numBits_eq with h | h <;> rw [h]
  rw [h2, Nat.shiftRight_eq_div_pow, hu] at h1
  exact ⟨by omega, hle, hlt⟩

theorem idx4 (a : ByteArray) (i k : USize) (hk : k.toNat < 4) (h : i < a.usize >>> 2) :
    (i * 4 + k).toNat < a.size := by
  obtain ⟨h3, hle, hlt⟩ := idx_main a i h
  rw [USize.toNat_add, USize.toNat_mul, usize_lit 4 (by decide)]
  rw [Nat.mod_eq_of_lt (by omega : i.toNat * 4 < 2 ^ System.Platform.numBits)]
  rw [Nat.mod_eq_of_lt (by omega : i.toNat * 4 + k.toNat < 2 ^ System.Platform.numBits)]
  omega

theorem idx4z (a : ByteArray) (i : USize) (h : i < a.usize >>> 2) : (i * 4).toNat < a.size := by
  obtain ⟨h3, hle, hlt⟩ := idx_main a i h
  rw [USize.toNat_mul, usize_lit 4 (by decide)]
  rw [Nat.mod_eq_of_lt (by omega : i.toNat * 4 < 2 ^ System.Platform.numBits)]
  omega

theorem size_uset (a : ByteArray) (i : USize) (v : UInt8) (h : i.toNat < a.size) :
    (a.uset i v h).size = a.size := by
  cases a; simp [ByteArray.uset, ByteArray.size]

theorem k1 : (1 : USize).toNat < 4 := by rw [usize_lit 1 (by decide)]; decide
theorem k2 : (2 : USize).toNat < 4 := by rw [usize_lit 2 (by decide)]; decide
theorem k3 : (3 : USize).toNat < 4 := by rw [usize_lit 3 (by decide)]; decide

/-- Byte `i`, or 0 past the end. -/
@[inline] def ld8 (a : ByteArray) (i : USize) : UInt8 :=
  if h : i < a.usize then a.uget i (usize_lt a i h) else 0

@[inline] def st8 (a : ByteArray) (i : USize) (v : UInt8) : ByteArray :=
  if h : i < a.usize then a.uset i v (usize_lt a i h) else a

/-- Little-endian 32-bit word `i` (bytes `4i .. 4i+3`), or 0 past the end. -/
@[inline] def ld32 (a : ByteArray) (i : USize) : UInt32 :=
  if h : i < a.usize >>> 2 then
    (a.uget (i * 4) (idx4z a i h)).toUInt32 |||
    ((a.uget (i * 4 + 1) (idx4 a i 1 k1 h)).toUInt32 <<< 8) |||
    ((a.uget (i * 4 + 2) (idx4 a i 2 k2 h)).toUInt32 <<< 16) |||
    ((a.uget (i * 4 + 3) (idx4 a i 3 k3 h)).toUInt32 <<< 24)
  else 0

/-- Replace word `i` (ignored past the end). In place when `a` is unshared. -/
@[inline] def st32 (a : ByteArray) (i : USize) (v : UInt32) : ByteArray :=
  if h : i < a.usize >>> 2 then
    let a1 := a.uset (i * 4) v.toUInt8 (idx4z a i h)
    have e1 : a1.size = a.size := size_uset _ _ _ _
    let a2 := a1.uset (i * 4 + 1) (v >>> 8).toUInt8 (by rw [e1]; exact idx4 a i 1 k1 h)
    have e2 : a2.size = a.size := by rw [size_uset]; exact e1
    let a3 := a2.uset (i * 4 + 2) (v >>> 16).toUInt8 (by rw [e2]; exact idx4 a i 2 k2 h)
    have e3 : a3.size = a.size := by rw [size_uset]; exact e2
    a3.uset (i * 4 + 3) (v >>> 24).toUInt8 (by rw [e3]; exact idx4 a i 3 k3 h)
  else a

@[inline] def ldI (a : ByteArray) (i : USize) : Int32 := (ld32 a i).toInt32
@[inline] def stI (a : ByteArray) (i : USize) (v : Int32) : ByteArray := st32 a i v.toUInt32

/-- `n` zero bytes, by pushing. -/
def zerosSlow (n : Nat) : ByteArray :=
  go (ByteArray.emptyWithCapacity n) n
where
  go (a : ByteArray) : Nat → ByteArray
    | 0 => a
    | k + 1 => go (a.push 0) k

/-- A shared block of zeros to copy from. -/
def zeroBlock (_ : Unit) : ByteArray := zerosSlow 16384

/-- `n` zero bytes. -/
def zeros (n : Nat) : ByteArray :=
  if n ≤ 16384 then ByteArray.copySlice (zeroBlock ()) 0 (ByteArray.emptyWithCapacity n) 0 n
  else zerosSlow n

/-- `n` zero words. -/
@[inline] def zeros32 (n : Nat) : ByteArray := zeros (4 * n)

/-- Set words `[i, j)` to 0. -/
partial def clear32 (a : ByteArray) (i j : USize) : ByteArray :=
  if i < j then clear32 (st32 a i 0) (i + 1) j else a

/-- Copy words `[s, s+n)` of `src` to `[d, d+n)` of `dst`. -/
partial def copy32 (src : ByteArray) (s : USize) (dst : ByteArray) (d n : USize) : ByteArray :=
  if n == 0 then dst else copy32 src (s + 1) (st32 dst d (ld32 src s)) (d + 1) (n - 1)

/-! ### Signed fixed point -/

/-- Sign-extend to 64-bit two's complement. -/
@[inline] def sx (a : Int32) : UInt64 := a.toInt64.toUInt64

/-- Low 32 bits of a 64-bit two's complement value. -/
@[inline] def lo32 (x : UInt64) : Int32 := x.toUInt32.toInt32

/-- Arithmetic right shift of a 64-bit two's complement value (`s < 64`). -/
@[inline] def sar64 (x : UInt64) (s : UInt64) : UInt64 :=
  if x >>> 63 == 0 then x >>> s else ~~~((~~~x) >>> s)

/-- Saturate a 64-bit two's complement value to `Int32`. -/
@[inline] def sat32 (x : UInt64) : Int32 :=
  let l := lo32 x
  if sx l == x then l else if x >>> 63 == 0 then 2147483647 else -2147483647 - 1

/-- `round(a * b / 2^31)`: product with a Q31 factor. -/
@[inline] def mulQ31 (a b : Int32) : Int32 := lo32 ((sx a * sx b + 0x40000000) >>> 31)

/-- `round(x / 2^s)` for a 64-bit two's complement `x`, `1 ≤ s ≤ 32`, saturated. -/
@[inline] def rshr64 (x : UInt64) (s : UInt64) : Int32 :=
  sat32 (sar64 (x + ((1 : UInt64) <<< (s - 1))) s)

@[inline] def iabs (a : Int32) : UInt32 := if a < 0 then (0 - a).toUInt32 else a.toUInt32

/-- Shift by `s` bits: left (saturating) when `s ≥ 0`, else right with rounding. -/
@[inline] def shiftRound (a : Int32) (s : Int32) : Int32 :=
  if s ≥ 0 then
    if s ≥ 31 then (if a == 0 then 0 else if a < 0 then -2147483647 - 1 else 2147483647)
    else sat32 (sx a <<< s.toUInt32.toUInt64)
  else
    let r := 0 - s
    if r ≥ 32 then 0 else lo32 (sar64 (sx a + ((1 : UInt64) <<< (r.toUInt32.toUInt64 - 1))) r.toUInt32.toUInt64)

end Yt.Aac
