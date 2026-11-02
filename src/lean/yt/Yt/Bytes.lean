/-! Small byte helpers shared by the libraries. Everything here must link
against our runtime (see ../RULES.md): no standard-library calls that are
not inlined. -/
namespace Yt.Bytes

/-- Big-endian 32-bit word at `i` (0 past the end). -/
@[inline] def u32be (b : ByteArray) (i : Nat) : UInt32 :=
  (b[i]!.toUInt32 <<< 24) ||| (b[i+1]!.toUInt32 <<< 16) ||| (b[i+2]!.toUInt32 <<< 8) ||| b[i+3]!.toUInt32

/-- `b` doubled (block copies) until it has `n` bytes. -/
partial def growTo (b : ByteArray) (n : Nat) : ByteArray :=
  if b.size ≥ n then b
  else if 2 * b.size ≤ n then growTo (b ++ b) n
  else b.copySlice 0 b b.size (n - b.size)

/-- `n` bytes of value `v`, built by doubling. `⟨Array.replicate n v⟩`
instead builds and frees an `Array` of `n` boxed elements: for a megabyte,
millions of allocator and reference-count steps. -/
def filled (n : Nat) (v : UInt8) : ByteArray :=
  if n == 0 then .empty else growTo (ByteArray.empty.push v) n

/-- 4 KiB of zeros, the source of `zeros`. -/
def zeroBlock : ByteArray := filled 4096 0

/-- Zeros appended to `b` up to `n` bytes, in blocks from `zeroBlock`. -/
partial def zerosTo (b : ByteArray) (n : Nat) : ByteArray :=
  if n ≤ b.size then b else zerosTo (zeroBlock.copySlice 0 b b.size (min 4096 (n - b.size))) n

/-- `n` zero bytes in one allocation (`filled n 0` allocates at each doubling:
the doubled array is shared between source and destination). -/
def zeros (n : Nat) : ByteArray := zerosTo (ByteArray.emptyWithCapacity n) n

/-! Byte access at `USize` positions, for hot code: `Nat` arithmetic on
indices costs overflow checks and boxing at every step, `USize` arithmetic
one instruction, and these reads check the position against the size
once (0 past the end; writes past the end leave the array unchanged). -/

theorem usize_lt_size {b : ByteArray} {i : USize} (h : i < b.usize) : i.toNat < b.size := by
  have h1 : i.toNat < b.usize.toNat := h
  have h2 : b.usize.toNat ≤ b.size := by simp [ByteArray.usize]; exact Nat.mod_le _ _
  omega

/-- The byte at `i`. -/
@[inline] def u8 (b : @& ByteArray) (i : USize) : UInt8 := if h : i < b.usize then b.uget i (usize_lt_size h) else 0
/-- The little-endian 16-bit value at byte `i`. -/
@[inline] def u16 (b : @& ByteArray) (i : USize) : UInt32 := (u8 b i).toUInt32 ||| ((u8 b (i + 1)).toUInt32 <<< 8)
/-- The little-endian 32-bit value at byte `i`. -/
@[inline] def u32 (b : @& ByteArray) (i : USize) : UInt32 :=
  (u8 b i).toUInt32 ||| ((u8 b (i + 1)).toUInt32 <<< 8) ||| ((u8 b (i + 2)).toUInt32 <<< 16) ||| ((u8 b (i + 3)).toUInt32 <<< 24)
/-- `b` with byte `i` set to `v`. -/
@[inline] def u8set (b : ByteArray) (i : USize) (v : UInt8) : ByteArray :=
  if h : i < b.usize then b.uset i v (usize_lt_size h) else b
/-- `b` with the little-endian 16-bit value `v` at byte `i`. -/
@[inline] def u16set (b : ByteArray) (i : USize) (v : UInt32) : ByteArray :=
  u8set (u8set b i v.toUInt8) (i + 1) (v >>> 8).toUInt8
/-- `b` with the little-endian 32-bit value `v` at byte `i`. -/
@[inline] def u32set (b : ByteArray) (i : USize) (v : UInt32) : ByteArray :=
  u8set (u8set (u8set (u8set b i v.toUInt8) (i + 1) (v >>> 8).toUInt8) (i + 2) (v >>> 16).toUInt8) (i + 3)
    (v >>> 24).toUInt8

end Yt.Bytes
