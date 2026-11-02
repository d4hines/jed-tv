

/-!
# Runtime conformance suite

Exercises the runtime's primitives and prints a deterministic transcript
(one `key=value` line per check). The same compiled C runs against upstream
Lean's runtime on the host (`zr-build conformance --record` records the
expected transcript) and against ours on 32-bit ARM in QEMU (`zr-build
conformance`); the transcripts must match byte for byte.

Everything takes a runtime argument `k` (always 0) so that the compiler
cannot fold the computations away at compile time. The suite avoids
`Init` functions that would have to be linked (only `@[extern]`
primitives, `@[inline]`/`@[specialize]` code, our own helpers and the
runtime's `Init` shim, ZrLean/Low/InitShim.lean), and values outside this
runtime's bounds (Nat below 2^128, Int in [-2^127, 2^127)).

Platform independence: values printed never depend on the word size
(`USize` results are reduced mod 2^32 first), so the 64-bit host and the
32-bit target must agree.
-/

namespace ZrLean.Conformance

theorem usize_le (t : ByteArray) : t.usize.toNat ≤ t.size := by
  simp only [ByteArray.usize, Nat.toUSize_eq]; exact Nat.mod_le _ _

/-- Bounds-checked byte read at a machine-word index (0 out of range). -/
@[inline] def getU (t : ByteArray) (i : USize) : UInt8 :=
  if h : i < t.usize then
    t.uget i (by have := USize.lt_iff_toNat_lt.mp h; have := usize_le t; omega)
  else 0

/-! ## Output -/

@[inline] def str (o : ByteArray) (s : String) : ByteArray := o ++ s.toUTF8

/-- Decimal digits of `n` (fuel: 2^128 has 39 digits). -/
def digits : Nat → Nat → ByteArray → ByteArray
  | 0, _, o => o
  | f + 1, n, o =>
    if n < 10 then o.push (48 + n.toUInt8)
    else (digits f (n / 10) o).push (48 + (n % 10).toUInt8)

def nat (o : ByteArray) (n : Nat) : ByteArray := digits 45 n o

def nl (o : ByteArray) : ByteArray := o.push 10

/-- Keys are bytes; string literals convert. -/
def utf8 (s : String) : ByteArray := s.toUTF8
instance : Coe String ByteArray := ⟨utf8⟩

/-- `"<s><i>."` -/
def pre (s : String) (i : Nat) : ByteArray := (nat s.toUTF8 i).push 46

/-- `"<s><i>,<j>."` -/
def pre2 (s : String) (i j : Nat) : ByteArray := (nat ((nat s.toUTF8 i).push 44) j).push 46

def kv (o : ByteArray) (key : ByteArray) (v : Nat) : ByteArray :=
  nl (nat ((o ++ key).push 61) v)

def kb (o : ByteArray) (key : ByteArray) (b : Bool) : ByteArray :=
  nl (str ((o ++ key).push 61) (if b then "true" else "false"))

def ks (o : ByteArray) (key : ByteArray) (s : String) : ByteArray :=
  nl (str ((o ++ key).push 61) s)

/-- Bytes as decimal numbers separated by commas. -/
def bytesLoop (b : ByteArray) (i : Nat) (o : ByteArray) : ByteArray :=
  if h : i < b.size then
    let o := if i > 0 then o.push 44 else o
    bytesLoop b (i + 1) (nat o b[i].toNat)
  else o
termination_by b.size - i

def kbytes (o : ByteArray) (key : ByteArray) (b : ByteArray) : ByteArray :=
  nl (bytesLoop b 0 ((o ++ key).push 61))

def natsLoop (a : Array Nat) (i : Nat) (o : ByteArray) : ByteArray :=
  if h : i < a.size then
    let o := if i > 0 then o.push 44 else o
    natsLoop a (i + 1) (nat o a[i])
  else o
termination_by a.size - i

def knats (o : ByteArray) (key : ByteArray) (a : Array Nat) : ByteArray :=
  nl (natsLoop a 0 ((o ++ key).push 61))

/-! ## Nat near the boxing boundaries (2^31 on 32-bit, 2^63 on 64-bit) and beyond 2^64 -/

/-- Boundary values (as literals: 32-bit targets box everything from 2^31). -/
def boundaries (k : Nat) : Array Nat :=
  #[0, 1, 2147483646, 2147483647, 2147483648, 2147483649, 4294967295, 4294967296, 4294967297,
    9223372036854775807, 9223372036854775808, 9223372036854775809,
    18446744073709551615, 18446744073709551616, 18446744073709551617,
    79228162514264337593543950335, 85070591730234615865843651857942052864].map (· + k)

def natOne (o : ByteArray) (i : Nat) (x : Nat) : ByteArray :=
  let p := pre "nat." i
  let o := kv o (p ++ "val") x
  let o := kv o (p ++ "pred") (x - 1)
  let o := kv o (p ++ "half") (x / 2)
  let o := kv o (p ++ "div7") (x / 7)
  let o := kv o (p ++ "mod7") (x % 7)
  let o := kv o (p ++ "div0") (x / 0)
  let o := kv o (p ++ "mod0") (x % 0)
  let o := kv o (p ++ "mod2^32") (x % 4294967296)
  let o := kv o (p ++ "shr1") (x >>> 1)
  let o := kv o (p ++ "shr33") (x >>> 33)
  let o := kv o (p ++ "shr200") (x >>> 200)
  let o := kv o (p ++ "land") (x &&& 0xFFFFFFFFFFFF)
  let o := kv o (p ++ "lor") (x ||| 1)
  let o := kv o (p ++ "xor") (x ^^^ 2147483648)
  let o := kv o (p ++ "log2") x.log2
  let o := kv o (p ++ "gcd12") (Nat.gcd x 12)
  let o := kv o (p ++ "u8") x.toUInt8.toNat
  let o := kv o (p ++ "u16") x.toUInt16.toNat
  let o := kv o (p ++ "u32") x.toUInt32.toNat
  let o := kv o (p ++ "u64") x.toUInt64.toNat
  let o := kv o (p ++ "usize32") (x.toUSize.toNat % 4294967296)
  let o := kb o (p ++ "even") (x % 2 == 0)
  o

/-- Operations that stay below 2^128 for `x` below 2^64. -/
def natSmallish (o : ByteArray) (i : Nat) (x : Nat) : ByteArray :=
  let p := pre "nat." i
  let o := kv o (p ++ "succ") (x + 1)
  let o := kv o (p ++ "Nat.succ") (Nat.succ x)
  let o := kv o (p ++ "dbl") (x + x)
  let o := kv o (p ++ "mul3") (x * 3)
  let o := kv o (p ++ "sq") (x * x)
  let o := kv o (p ++ "shl3") (x <<< 3)
  let o := kv o (p ++ "shl40") (x <<< 40)
  o

def natPairs (o : ByteArray) (xs : Array Nat) (i j : Nat) : ByteArray :=
  let x := xs[i]!
  let y := xs[j]!
  let p := pre2 "nat." i j
  let o := kv o (p ++ "add") (x + y)
  let o := kv o (p ++ "sub") (x - y)
  let o := kb o (p ++ "eq") (x == y)
  let o := kb o (p ++ "lt") (decide (x < y))
  let o := kb o (p ++ "le") (decide (x ≤ y))
  let o := kv o (p ++ "div") (x / y)
  let o := kv o (p ++ "mod") (x % y)
  let o := kv o (p ++ "and") (x &&& y)
  let o := kv o (p ++ "or") (x ||| y)
  let o := kv o (p ++ "xor") (x ^^^ y)
  let o := kv o (p ++ "gcd") (Nat.gcd x y)
  o

def natAll (o : ByteArray) (i : Nat) (xs : Array Nat) : Nat → ByteArray
  | 0 => o
  | fuel + 1 =>
    if h : i < xs.size then
      let x := xs[i]
      let o := natOne o i x
      let o := if x ≤ 18446744073709551615 then natSmallish o i x else o
      natAll o (i + 1) xs fuel
    else o

def pairLoop (o : ByteArray) (xs : Array Nat) (i j : Nat) : Nat → ByteArray
  | 0 => o
  | fuel + 1 =>
    if i ≥ xs.size then o
    else if j ≥ xs.size then pairLoop o xs (i + 1) 0 fuel
    else
      -- products only where they stay below 2^128
      let o := natPairs o xs i j
      let o := if xs[i]! ≤ 18446744073709551615 && xs[j]! ≤ 18446744073709551615 then
        kv o (pre2 "nat." i j ++ "mul") (xs[i]! * xs[j]!) else o
      pairLoop o xs i (j + 1) fuel

def natPow (o : ByteArray) (k : Nat) : ByteArray :=
  let o := kv o "nat.pow.2^31" (2 ^ (31 + k))
  let o := kv o "nat.pow.2^64" (2 ^ (64 + k))
  let o := kv o "nat.pow.3^40" (3 ^ (40 + k))
  let o := kv o "nat.pow.0^0" (0 ^ k)
  kv o "nat.pow.10^38" (10 ^ (38 + k))

/-- Many big Nats allocated and dropped: a running sum of 2^62 + i. -/
def natChurn (k : Nat) : Nat → Nat → Nat
  | 0, acc => acc
  | n + 1, acc => natChurn k n ((acc + 4611686018427387904 + n + k) % 1000000000000000000000)

def natSection (o : ByteArray) (k : Nat) : ByteArray :=
  let xs := boundaries k
  let o := natAll o 0 xs 100
  let o := pairLoop o xs 0 0 1000
  let o := natPow o k
  kv o "nat.churn" (natChurn k 5000 0)

/-! ## Fixed-width integers -/

def u64s (k : Nat) : Array UInt64 :=
  #[0, 1, 0x7FFFFFFF, 0x80000000, 0xFFFFFFFF, 0x100000000, 0x7FFFFFFFFFFFFFFF, 0x8000000000000000,
    0xFFFFFFFFFFFFFFFF].map (· + k.toUInt64)

def u64One (o : ByteArray) (i : Nat) (x : UInt64) : ByteArray :=
  let p := pre "u64." i
  let o := kv o (p ++ "toNat") x.toNat
  let o := kv o (p ++ "succ") (x + 1).toNat
  let o := kv o (p ++ "pred") (x - 1).toNat
  let o := kv o (p ++ "mul3") (x * 3).toNat
  let o := kv o (p ++ "sq") (x * x).toNat
  let o := kv o (p ++ "div7") (x / 7).toNat
  let o := kv o (p ++ "mod7") (x % 7).toNat
  let o := kv o (p ++ "div0") (x / 0).toNat
  let o := kv o (p ++ "mod0") (x % 0).toNat
  let o := kv o (p ++ "shl1") (x <<< 1).toNat
  let o := kv o (p ++ "shl65") (x <<< 65).toNat
  let o := kv o (p ++ "shr63") (x >>> 63).toNat
  let o := kv o (p ++ "not") (~~~x).toNat
  let o := kv o (p ++ "neg") (0 - x).toNat
  let o := kv o (p ++ "u32") x.toUInt32.toNat
  let o := kv o (p ++ "u16") x.toUInt16.toNat
  let o := kv o (p ++ "u8") x.toUInt8.toNat
  let o := kv o (p ++ "ofNatRound") (UInt64.ofNat (x.toNat * 3 + 1)).toNat
  let o := kb o (p ++ "lt.max") (x < 0xFFFFFFFFFFFFFFFF)
  o

def u64Loop (o : ByteArray) (xs : Array UInt64) (i : Nat) : ByteArray :=
  if h : i < xs.size then u64Loop (u64One o i xs[i]) xs (i + 1) else o
termination_by xs.size - i

def u32s (k : Nat) : Array UInt32 :=
  #[0, 1, 0x7FFFFFFE, 0x7FFFFFFF, 0x80000000, 0x80000001, 0xFFFFFFFE, 0xFFFFFFFF].map (· + k.toUInt32)

def u32One (o : ByteArray) (i : Nat) (x : UInt32) : ByteArray :=
  let p := pre "u32." i
  let o := kv o (p ++ "toNat") x.toNat
  let o := kv o (p ++ "succ") (x + 1).toNat
  let o := kv o (p ++ "pred") (x - 1).toNat
  let o := kv o (p ++ "mul5") (x * 5).toNat
  let o := kv o (p ++ "div3") (x / 3).toNat
  let o := kv o (p ++ "mod0") (x % 0).toNat
  let o := kv o (p ++ "shl33") (x <<< 33).toNat
  let o := kv o (p ++ "shr31") (x >>> 31).toNat
  let o := kv o (p ++ "rot") ((x <<< 8) ||| (x >>> 24)).toNat
  let o := kv o (p ++ "u64") x.toUInt64.toNat
  let o := kv o (p ++ "u8") x.toUInt8.toNat
  let o := kv o (p ++ "usize32") (x.toUSize.toNat % 4294967296)
  let o := kv o (p ++ "ofNat") (UInt32.ofNat (x.toNat + 4294967296)).toNat
  o

def u32Loop (o : ByteArray) (xs : Array UInt32) (i : Nat) : ByteArray :=
  if h : i < xs.size then u32Loop (u32One o i xs[i]) xs (i + 1) else o
termination_by xs.size - i

def smallInts (o : ByteArray) (k : Nat) : ByteArray :=
  let a : UInt8 := 250 + k.toUInt8
  let b : UInt16 := 65530 + k.toUInt16
  let s : USize := 1000 + k.toUSize
  let o := kv o "u8.add" (a + 10).toNat
  let o := kv o "u8.sub" (a - 251).toNat
  let o := kv o "u8.mul" (a * 3).toNat
  let o := kv o "u8.shl9" (a <<< 9).toNat
  let o := kv o "u8.div0" (a / 0).toNat
  let o := kv o "u16.add" (b + 10).toNat
  let o := kv o "u16.mul" (b * b).toNat
  let o := kv o "u16.shr17" (b >>> 17).toNat
  let o := kv o "usize.mul" (s * 7).toNat
  let o := kv o "usize.div" (s / 7).toNat
  let o := kv o "usize.mod" (s % 7).toNat
  let o := kv o "usize.sub" (s - 999).toNat
  let o := kv o "usize.u32" (s * 4000000).toUInt32.toNat
  o

/-- UInt64s through boxed positions (array elements, structure fields). -/
structure Mixed where
  a : UInt8
  b : UInt16
  c : UInt32
  d : UInt64
  e : USize
  n : Nat
  arr : Array UInt64

def mixedSum (m : Mixed) : Nat :=
  m.a.toNat + m.b.toNat + m.c.toNat + m.d.toNat + (m.e.toNat % 4294967296) + m.n +
    m.arr.foldl (fun s x => s + x.toNat) 0

def uintSection (o : ByteArray) (k : Nat) : ByteArray :=
  let o := u64Loop o (u64s k) 0
  let o := u32Loop o (u32s k) 0
  let o := smallInts o k
  let m : Mixed := { a := 200, b := 60000, c := 4000000000, d := 0xFFFFFFFFFFFFFFF0 + k.toUInt64,
                     e := 77, n := 2147483648 + k, arr := u64s k }
  let o := kv o "mixed.sum" (mixedSum m)
  let m2 := { m with d := m.d + 0x20, c := m.c + 300000000 }
  let o := kv o "mixed.sum2" (mixedSum m2)
  kv o "mixed.sum1again" (mixedSum m)

/-! ## Arrays: push, set!, copy-on-write, out of bounds, conversions -/

def pushN (a : Array Nat) (k : Nat) : Nat → Array Nat
  | 0 => a
  | n + 1 => pushN (a.push (n * n + k)) k n

def sumArr (a : Array Nat) : Nat := a.foldl (fun s x => s + x) 0

unsafe def arraySection (o : ByteArray) (k : Nat) : ByteArray :=
  let a := pushN (Array.emptyWithCapacity k) k 20
  let o := kv o "arr.size" a.size
  let o := kv o "arr.sum" (sumArr a)
  let o := kb o "arr.exclusive" (isExclusiveUnsafe a)
  -- sharing: `b` keeps the old version
  let b := a
  let a := a.set! 3 (1000 + k)
  let o := kv o "arr.cow.a3" a[3]!
  let o := kv o "arr.cow.b3" b[3]!
  let o := kb o "arr.cow.b.exclusive" (isExclusiveUnsafe b)
  let a := a.push 7
  let o := kv o "arr.pushed.size" a.size
  let o := kv o "arr.old.size" b.size
  -- out of bounds (panics, then the default / the array unchanged)
  let o := kv o "arr.oob.get" (a[100 + k]!)
  let a := a.set! (200 + k) 5
  let o := kv o "arr.oob.set.size" a.size
  let o := kv o "arr.oob.big.get" (a[18446744073709551616 + k]!)
  -- in place while exclusive with spare capacity, copied when full
  let c := (Array.emptyWithCapacity (2 + k)).push (1 : Nat) |>.push 2
  let pc := ptrAddrUnsafe c
  let c := c.push 3
  let o := kb o "arr.grow.moved" (ptrAddrUnsafe c != pc)
  let pc := ptrAddrUnsafe c
  let c := c.push 4
  let o := kb o "arr.inplace" (ptrAddrUnsafe c == pc)
  let o := knats o "arr.c" c
  let o := kv o "arr.pop.size" c.pop.size
  let o := knats o "arr.replicate" (Array.replicate (5 + k) (2147483648 + k))
  let l := c.toList
  let o := knats o "arr.toList.toArray" l.toArray
  let o := knats o "arr.mk" (Array.mk [9 + k, 8, 7])
  -- nested arrays (children freed with the parent)
  let nested := Array.replicate 50 (pushN #[] k 10)
  let o := kv o "arr.nested" (nested.foldl (fun s x => s + sumArr x) 0)
  -- a big array (over the small-object size)
  let big := pushN #[] k 1000
  let o := kv o "arr.big.sum" (sumArr big)
  let o := kv o "arr.big.last" big[999]!
  o

/-! ## ByteArray -/

def bytesN (b : ByteArray) (k : Nat) : Nat → ByteArray
  | 0 => b
  | n + 1 => bytesN (b.push (n * 7 + k).toUInt8) k n

unsafe def byteArraySection (o : ByteArray) (k : Nat) : ByteArray :=
  let a := bytesN (ByteArray.emptyWithCapacity k) k 10
  let o := kbytes o "ba.a" a
  let o := kb o "ba.exclusive" (isExclusiveUnsafe a)
  let b := a
  let a := a.set! 0 99
  let o := kbytes o "ba.cow.a" a
  let o := kbytes o "ba.cow.b" b
  let o := kv o "ba.get.oob" (a.get! (50 + k)).toNat
  let o := kv o "ba.get.big" (a.get! (18446744073709551616 + k)).toNat
  let o := kbytes o "ba.set.oob" (a.set! (50 + k) 1)
  let o := kbytes o "ba.append" (a ++ b)
  let o := kbytes o "ba.slice" (a.copySlice (2 + k) ByteArray.empty 0 4)
  let o := kbytes o "ba.slice.oob" (a.copySlice (8 + k) ByteArray.empty 0 100)
  let o := kbytes o "ba.copySlice" (a.copySlice (1 + k) b 3 4)
  let o := kbytes o "ba.copySlice.past" (a.copySlice (1 + k) b 30 4)
  let o := kbytes o "ba.copySlice.srcoob" (a.copySlice (40 + k) b 3 4)
  let o := kbytes o "ba.copySlice.self" (a.copySlice (k) a 2 5)
  let o := kbytes o "ba.mk" (ByteArray.mk #[1, 2, 255, (3 + k).toUInt8])
  let o := knats o "ba.data" (a.data.map (·.toNat))
  let o := kv o "ba.uget" (getU a (3 + k.toUSize)).toNat
  let big := bytesN ByteArray.empty k 5000
  let o := kv o "ba.big.size" big.size
  let o := kv o "ba.big.2500" (big.get! 2500).toNat
  o

/-! ## Closures: exact, partial, over-application, > 16 arguments -/

@[noinline] def add (a b : Nat) : Nat := a + b
@[noinline] def mul (a b : Nat) : Nat := a * b
@[noinline] def sub (a b : Nat) : Nat := a - b
def max2 (a b : Nat) : Nat := if a < b then b else a

@[noinline] def ops (k : Nat) : Array (Nat → Nat → Nat) :=
  #[add, mul, fun a b => a * 10 + b + k, sub, max2]

@[noinline] def adder (n : Nat) : Nat → Nat := fun x => x + n

/-- Arity 1, returns a function: applying it to two arguments over-applies. -/
@[noinline] def curried (k : Nat) : Array (Nat → Nat → Nat) :=
  #[fun a => adder (a + k), fun a => fun b => a * b + 1]

def f18 (a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 : Nat) : Nat :=
  a1 + 2*a2 + 3*a3 + 4*a4 + 5*a5 + 6*a6 + 7*a7 + 8*a8 + 9*a9 + 10*a10 + 11*a11 + 12*a12 +
    13*a13 + 14*a14 + 15*a15 + 16*a16 + 17*a17 + 18*a18

@[noinline] def bigClosures (k : Nat) : Array (Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat →
    Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat) :=
  #[f18, fun a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 a17 a18 =>
    f18 a18 a17 a16 a15 a14 a13 a12 a11 a10 a9 a8 a7 a6 a5 a4 a3 a2 (a1 + k)]

@[noinline] def capture (xs : Array Nat) : Nat → Nat := fun i => xs[i]! + xs.size

@[noinline] def applyTwice (f : Nat → Nat) (x : Nat) : Nat := f (f x)

@[noinline] def apply1 (f : Nat → Nat → Nat) (x : Nat) : Nat → Nat := f x

def closureLoop (o : ByteArray) (fs : Array (Nat → Nat → Nat)) (k i : Nat) : ByteArray :=
  if h : i < fs.size then
    let f := fs[i]
    let p := pre "clo." i
    let o := kv o (p ++ "exact") (f (7 + k) 3)
    let g := apply1 f (2147483647 + k)
    let o := kv o (p ++ "partial") (g 5)
    let o := kv o (p ++ "partial.again") (g 6)
    closureLoop o fs k (i + 1)
  else o
termination_by fs.size - i

def f16 (a1 a2 a3 a4 a5 a6 a7 a8 a9 a10 a11 a12 a13 a14 a15 a16 : Nat) : Nat :=
  a1 + 3*a2 + 5*a3 + 7*a4 + 11*a5 + 13*a6 + 17*a7 + 19*a8 + 23*a9 + 29*a10 + 31*a11 + 37*a12 +
    41*a13 + 43*a14 + 47*a15 + 53*a16

abbrev F16 := Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat → Nat

@[noinline] def f16s (k : Nat) : Array F16 := #[f16, fun a1 => f16 (a1 + k)]

/-- Hides a value from the optimiser (forces calls through `lean_apply_*`). -/
@[noinline] def hide {α : Type} (a : α) : α := a

/-- `lean_apply_1` .. `lean_apply_16`: a 16-ary closure with `j` arguments
fixed, applied to the other `16 - j`. -/
def applyN (o : ByteArray) (k : Nat) : ByteArray :=
  let f := hide ((f16s k).getD 0 f16)
  let o := kv o "clo.16.apply16" (f 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16)
  let g1 := hide (f 1)
  let o := kv o "clo.16.apply15" (g1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16)
  let g2 := hide (g1 2)
  let o := kv o "clo.16.apply14" (g2 3 4 5 6 7 8 9 10 11 12 13 14 15 16)
  let g3 := hide (g2 3)
  let o := kv o "clo.16.apply13" (g3 4 5 6 7 8 9 10 11 12 13 14 15 16)
  let g4 := hide (g3 4)
  let o := kv o "clo.16.apply12" (g4 5 6 7 8 9 10 11 12 13 14 15 16)
  let g5 := hide (g4 5)
  let o := kv o "clo.16.apply11" (g5 6 7 8 9 10 11 12 13 14 15 16)
  let g6 := hide (g5 6)
  let o := kv o "clo.16.apply10" (g6 7 8 9 10 11 12 13 14 15 16)
  let g7 := hide (g6 7)
  let o := kv o "clo.16.apply9" (g7 8 9 10 11 12 13 14 15 16)
  let g8 := hide (g7 8)
  let o := kv o "clo.16.apply8" (g8 9 10 11 12 13 14 15 16)
  let g9 := hide (g8 9)
  let o := kv o "clo.16.apply7" (g9 10 11 12 13 14 15 16)
  let g10 := hide (g9 10)
  let o := kv o "clo.16.apply6" (g10 11 12 13 14 15 16)
  let g11 := hide (g10 11)
  let o := kv o "clo.16.apply5" (g11 12 13 14 15 16)
  let g12 := hide (g11 12)
  let o := kv o "clo.16.apply4" (g12 13 14 15 16)
  let g13 := hide (g12 13)
  let o := kv o "clo.16.apply3" (g13 14 15 16)
  let g14 := hide (g13 14)
  let o := kv o "clo.16.apply2" (g14 15 16)
  let g15 := hide (g14 15)
  let o := kv o "clo.16.apply1" (g15 16)
  -- the shared partial applications are still usable
  let o := kv o "clo.16.shared" (g8 0 0 0 0 0 0 0 (2147483648 + k))
  -- the second closure fixes its first argument itself
  let h := (f16s k).getD 1 f16
  kv o "clo.16.wrapped" (h 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1)

def closureSection (o : ByteArray) (k : Nat) : ByteArray :=
  let o := applyN o k
  let o := closureLoop o (ops k) k 0
  let cs := curried k
  let o := kv o "clo.over.0" ((cs.getD 0 mul) 10 20)
  let o := kv o "clo.over.1" ((cs.getD 1 mul) 6 7)
  let bs := bigClosures k
  let o := kv o "clo.18.exact" ((bs.getD 0 f18) 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18)
  let o := kv o "clo.18.rev" ((bs.getD 1 f18) 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18)
  let part := (bs.getD 0 f18) 100 200 300
  let o := kv o "clo.18.partial" (part 1 1 1 1 1 1 1 1 1 1 1 1 1 1 1)
  let o := kv o "clo.18.partial.again" (part 2 2 2 2 2 2 2 2 2 2 2 2 2 2 2)
  let c := capture (pushN #[] k 5)
  let o := kv o "clo.capture" (applyTwice c 1)
  let o := kv o "clo.adder" (applyTwice (adder (4294967296 + k)) 1)
  o

/-! ## Strings -/

unsafe def stringSection (o : ByteArray) (k : Nat) : ByteArray :=
  let s := "héllo ✓ 𝔸"
  let o := kv o "str.length" s.length
  let o := kv o "str.bytes" s.utf8ByteSize
  let o := kbytes o "str.utf8" s.toUTF8
  let t := s.push 'x' |>.push '✓' |>.push (if k == 0 then '😀' else 'z')
  let o := ks o "str.push" t
  let o := kv o "str.push.length" t.length
  let o := ks o "str.append" (s ++ " / " ++ t)
  let u := "abc".push (if k == 0 then '0' else '1')
  let o := kb o "str.eq" (u == "abc0")
  let o := kb o "str.ne" (u == "abc1")
  let o := kb o "str.lt" (decide (u < "abd"))
  let o := kb o "str.lt.prefix" (decide ("ab" < u))
  let o := kb o "str.lt.not" (decide (u < "ab"))
  let o := kb o "str.valid" (ByteArray.validateUTF8 s.toUTF8)
  let o := kb o "str.invalid" (ByteArray.validateUTF8 (ByteArray.mk #[0xC3, 0x28 + k.toUInt8]))
  let raw := ByteArray.mk #[0xE2, 0x9C, 0x93, 0x41 + k.toUInt8]
  let o := ks o "str.fromUTF8" ((String.fromUTF8? raw).getD "invalid")
  let o := kv o "str.fromUTF8.length" ((String.fromUTF8? raw).map String.length |>.getD 0)
  let shared := s ++ ""
  let o := kb o "str.exclusive" (isExclusiveUnsafe shared)
  let v := shared
  let w := shared.push '!'
  let o := ks o "str.cow.v" v
  let o := ks o "str.cow.w" w
  o

/-! ## Closed terms (once cells, persistent objects) -/

def closedBig : Nat := 340282366920938463463374607431768211455
def closedArr : Array Nat := #[2147483648, 4294967296, 1, 2, 3]
def closedU64 : UInt64 := 0xDEADBEEFCAFEF00D
def closedStr : String := "closed ✓"

unsafe def closedSection (o : ByteArray) (k : Nat) : ByteArray :=
  let o := kv o "closed.big" (closedBig + k)
  let o := kv o "closed.big.div" (closedBig / (3 + k))
  let o := knats o "closed.arr" closedArr
  let o := kb o "closed.arr.exclusive" (isExclusiveUnsafe closedArr)
  -- pushing to a persistent array copies it; the closed term is unchanged
  let o := knats o "closed.arr.pushed" (closedArr.push (5 + k))
  let o := knats o "closed.arr.after" closedArr
  let o := kv o "closed.u64" closedU64.toNat
  let o := ks o "closed.str" (closedStr.push '!')
  ks o "closed.str.after" closedStr

/-! ## Panics -/

def panicSection (o : ByteArray) (k : Nat) : ByteArray :=
  let o := kv o "panic.core" (panicCore (α := Nat) ("conformance: explicit panic ".push (if k == 0 then '0' else '1')))
  let o := kv o "panic.core.after" (41 + k + 1)
  o

/-! ## Allocation churn (lists, structures) -/

inductive Tree where
  | leaf : Nat → Tree
  | node : Tree → Tree → Tree

def build : Nat → Nat → Tree
  | 0, n => .leaf n
  | d + 1, n => .node (build d (2 * n)) (build d (2 * n + 1))

def Tree.sum : Tree → Nat
  | .leaf n => n
  | .node a b => a.sum + b.sum

def Tree.mirror : Tree → Tree
  | .leaf n => .leaf (n + 2147483648)
  | .node a b => .node b.mirror a.mirror

def Tree.leftmost : Tree → Nat
  | .leaf n => n
  | .node a _ => a.leftmost

def revList : List Nat → List Nat → List Nat
  | [], acc => acc
  | x :: xs, acc => revList xs (x :: acc)

def mkList : Nat → List Nat → List Nat
  | 0, acc => acc
  | n + 1, acc => mkList n ((n * 3) :: acc)

def sumList : List Nat → Nat → Nat
  | [], s => s
  | x :: xs, s => sumList xs (s + x)

def churnSection (o : ByteArray) (k : Nat) : ByteArray :=
  let t := build (12 + k) k
  let o := kv o "tree.sum" t.sum
  let m := t.mirror
  let o := kv o "tree.mirror.sum" m.sum
  let o := kv o "tree.mirror.leftmost" m.leftmost
  let o := kv o "tree.sum.again" t.sum
  let l := mkList (3000 + k) []
  let o := kv o "list.sum" (sumList (revList l []) 0)
  o

/-! ## Lean 4.35 runtime additions

Linearity marks (`markLinear`: the top bit of `m_other`, which a scalar
array shares with its element size), reference-count increments above
2^16 (`lean_inc_ref_huge_n`), `ByteArray` equality (`lean_sarray_eq_cold`),
`String.compare`, and `Nat.pow` with an exponent of 2^32 or more. -/

def ordIdx : Ordering → Nat
  | .lt => 0
  | .eq => 1
  | .gt => 2

unsafe def v435Section (o : ByteArray) (k : Nat) : ByteArray :=
  -- arrays: marked, then copied while shared, then grown while exclusive
  let a := (pushN (Array.emptyWithCapacity k) k 3).markLinear
  let b := a
  let a := a.push (40 + k)
  let o := knats o "lin.arr.a" a
  let o := knats o "lin.arr.b" b
  let a := pushN a k 40
  let o := kv o "lin.arr.grown.size" a.size
  let o := kv o "lin.arr.grown.sum" (sumArr a)
  let o := knats o "lin.arr.propagate" (a.propagateMark #[1 + k, 2])
  -- byte arrays: the element size must be read without the mark
  let x := (bytesN (ByteArray.emptyWithCapacity k) k 4).markLinear
  let y := x
  let x := x.push (9 + k).toUInt8
  let o := kbytes o "lin.ba.x" x
  let o := kbytes o "lin.ba.y" y
  let x := bytesN x k 600
  let o := kv o "lin.ba.grown.size" x.size
  let o := kv o "lin.ba.grown.599" (x.get! 599).toNat
  let o := kbytes o "lin.ba.copySlice" (x.copySlice (1 + k) y 2 3)
  let o := kbytes o "lin.ba.append" (y ++ y)
  let o := kbytes o "lin.ba.propagate" (y.propagateMark (ByteArray.mk #[7, (8 + k).toUInt8]))
  -- strings
  let s := ("lin" ++ (if k == 0 then "ear" else "e")).markLinear
  let t := s
  let s := s.push '✓'
  let o := ks o "lin.str.s" s
  let o := ks o "lin.str.t" t
  let o := ks o "lin.str.append" (s ++ t ++ s)
  let o := ks o "lin.str.propagate" (t.propagateMark ("p".push 'q'))
  -- ByteArray ==
  let e1 := bytesN ByteArray.empty k 10
  let e2 := bytesN (ByteArray.emptyWithCapacity 64) k 10
  let o := kb o "ba.eq" (e1 == e2)
  let o := kb o "ba.eq.self" (e1 == e1)
  let o := kb o "ba.eq.diff" (e1 == e2.set! 0 1)
  let o := kb o "ba.eq.len" (e1 == e2.push 0)
  let o := kb o "ba.eq.marked" (e1.markLinear == e2)
  let o := kb o "ba.eq.empty" (ByteArray.emptyWithCapacity k == ByteArray.empty)
  -- String.compare (Ordering: lt 0, eq 1, gt 2)
  let u := "abc".push (if k == 0 then '0' else '1')
  let o := kv o "str.compare.eq" (ordIdx (compare u "abc0"))
  let o := kv o "str.compare.lt" (ordIdx (compare u "abd"))
  let o := kv o "str.compare.gt" (ordIdx (compare u "abb"))
  let o := kv o "str.compare.prefix" (ordIdx (compare u "abc"))
  let o := kv o "str.compare.longer" (ordIdx (compare u "abc00"))
  let o := kv o "str.compare.utf8" (ordIdx (compare "é" u))
  -- 0 ^ e and 1 ^ e for huge e (4.30 panicked)
  let o := kv o "nat.pow.0^2^64" (0 ^ (18446744073709551616 + k))
  let o := kv o "nat.pow.1^2^64" (1 ^ (18446744073709551616 + k))
  let o := kv o "nat.pow.1^2^40" (1 ^ (1099511627776 + k))
  -- Array.replicate of a heap object: one increment of n - 1 > 2^16
  let elem := pushN #[] k 3
  let big := Array.replicate (70000 + k) elem
  let o := kb o "rc.huge.elem.exclusive" (isExclusiveUnsafe (big.getD (69999 + k) #[]))
  let o := kv o "rc.huge.size" big.size
  let o := kv o "rc.huge.sum" (big.foldl (fun acc x => acc + sumArr x) 0)
  -- `big` is dead now: its 70000 references are gone again
  let o := kb o "rc.huge.after.exclusive" (isExclusiveUnsafe elem)
  o

/-! ## Int

Small Ints are scalars ([-2^30, 2^30) on 32-bit, [-2^31, 2^31) on 64-bit);
others are objects, bounded by this runtime to [-2^127, 2^127). Values are
printed in decimal with a sign; no computed value leaves the bound. -/

def int (o : ByteArray) (x : Int) : ByteArray :=
  if x < 0 then nat (o.push 45) x.natAbs else nat o x.natAbs

def ki (o : ByteArray) (key : ByteArray) (v : Int) : ByteArray :=
  nl (int ((o ++ key).push 61) v)

/-- 2^127 - 1, the largest Int of this runtime (its negation minus 1 is the
smallest; the literal 2^127 itself is out of bounds). -/
def intMaxLit : Int := 170141183460469231731687303715884105727

/-- Around 0, both small-Int boundaries, 2^32, 2^63, 2^64, 2^100. -/
def intVals (k : Int) : Array Int :=
  #[0, 1, -1, 2, -2, 7, -7,
    1073741823, 1073741824, 1073741825, -1073741823, -1073741824, -1073741825,
    2147483647, 2147483648, -2147483647, -2147483648, -2147483649,
    4294967296, -4294967296, 9223372036854775807, 9223372036854775808, -9223372036854775808,
    -9223372036854775809, 18446744073709551621, -18446744073709551621,
    1267650600228229401496703205376, -1267650600228229401496703205376].map (· + k)

/-- Divisors: every sign combination, small and big. -/
def intDivisors (k : Int) : Array Int :=
  #[7, -7, 1, -1, 0, 1073741824, -1073741825, 18446744073709551621, -18446744073709551621].map (· + k)

def intDivLoop (o : ByteArray) (p : ByteArray) (x : Int) (ds : Array Int) (j : Nat) : ByteArray :=
  if h : j < ds.size then
    let d := ds[j]
    let q := (nat (p ++ "d") j).push 46
    let o := ki o (q ++ "ediv") (x / d)
    let o := ki o (q ++ "emod") (x % d)
    let o := ki o (q ++ "tdiv") (x.tdiv d)
    let o := ki o (q ++ "tmod") (x.tmod d)
    intDivLoop o p x ds (j + 1)
  else o
termination_by ds.size - j

def intOne (o : ByteArray) (i : Nat) (x : Int) (ds : Array Int) (kn : Nat) : ByteArray :=
  let p := pre "int." i
  let o := ki o (p ++ "val") x
  let o := kv o (p ++ "natAbs") x.natAbs
  let o := kv o (p ++ "toNat") x.toNat
  let o := kb o (p ++ "lt0") (decide (x < 0))
  let o := kb o (p ++ "le0") (decide (x ≤ 0))
  let o := kb o (p ++ "eq0") (x == 0)
  let o := intDivLoop o p x ds 0
  let o := ki o (p ++ "shr0") (x >>> (0 + kn))
  let o := ki o (p ++ "shr1") (x >>> (1 + kn))
  let o := ki o (p ++ "shr3") (x >>> (3 + kn))
  let o := ki o (p ++ "shr30") (x >>> (30 + kn))
  let o := ki o (p ++ "shr31") (x >>> (31 + kn))
  let o := ki o (p ++ "shr64") (x >>> (64 + kn))
  let o := ki o (p ++ "shr200") (x >>> (200 + kn))
  let o := ki o (p ++ "shr2^64") (x >>> (18446744073709551616 + kn))
  let o := ki o (p ++ "i8") x.toInt8.toInt
  let o := ki o (p ++ "i16") x.toInt16.toInt
  let o := ki o (p ++ "i32") x.toInt32.toInt
  let o := ki o (p ++ "i64") x.toInt64.toInt
  ki o (p ++ "isize32") x.toISize.toInt.toInt32.toInt

/-- Arithmetic that stays within the bound for `|x| ≤ 2^100` (all of `intVals`). -/
def intArith (o : ByteArray) (i : Nat) (x : Int) (kn : Nat) : ByteArray :=
  let p := pre "int." i
  let o := ki o (p ++ "neg") (-x)
  let o := ki o (p ++ "succ") (x + 1)
  let o := ki o (p ++ "pred") (x - 1)
  let o := ki o (p ++ "dbl") (x + x)
  let o := ki o (p ++ "mul3") (x * 3)
  let o := ki o (p ++ "mul-3") (x * -3)
  let o := ki o (p ++ "shl1") (x <<< (1 + kn))
  let o := ki o (p ++ "shl20") (x <<< (20 + kn))
  let o := ki o (p ++ "shl26") (x <<< (26 + kn))
  let o := ki o (p ++ "pow0") (x ^ (0 + kn))
  let o := ki o (p ++ "pow1") (x ^ (1 + kn))
  if x.natAbs.log2 < 62 then
    let o := ki o (p ++ "sq") (x * x)
    let o := ki o (p ++ "pow2") (x ^ (2 + kn))
    if x.natAbs.log2 < 41 then ki o (p ++ "pow3") (x ^ (3 + kn)) else o
  else o

def intAll (o : ByteArray) (i : Nat) (xs ds : Array Int) (kn : Nat) : Nat → ByteArray
  | 0 => o
  | fuel + 1 =>
    if h : i < xs.size then
      let x := xs[i]
      intAll (intArith (intOne o i x ds kn) i x kn) (i + 1) xs ds kn fuel
    else o

/-- Pairs: every sign combination, small/big mixes, the small boundaries. -/
def intPairVals (k : Int) : Array Int :=
  #[0, 1, -1, 3, -3, 7, -7, 1073741823, 1073741824, -1073741824, -1073741825, 2147483648,
    -2147483649, 18446744073709551621, -18446744073709551621, 1267650600228229401496703205376,
    -1267650600228229401496703205376].map (· + k)

def intPairs (o : ByteArray) (xs : Array Int) (i j : Nat) : ByteArray :=
  let x := xs[i]!
  let y := xs[j]!
  let p := pre2 "int." i j
  let o := ki o (p ++ "add") (x + y)
  let o := ki o (p ++ "sub") (x - y)
  let o := kb o (p ++ "eq") (x == y)
  let o := kb o (p ++ "ne") (x != y)
  let o := kb o (p ++ "lt") (decide (x < y))
  let o := kb o (p ++ "le") (decide (x ≤ y))
  let o := ki o (p ++ "ediv") (x / y)
  let o := ki o (p ++ "emod") (x % y)
  let o := ki o (p ++ "tdiv") (x.tdiv y)
  let o := ki o (p ++ "tmod") (x.tmod y)
  if x.natAbs.log2 + y.natAbs.log2 < 125 then ki o (p ++ "mul") (x * y) else o

def intPairLoop (o : ByteArray) (xs : Array Int) (i j : Nat) : Nat → ByteArray
  | 0 => o
  | fuel + 1 =>
    if i ≥ xs.size then o
    else if j ≥ xs.size then intPairLoop o xs (i + 1) 0 fuel
    else intPairLoop (intPairs o xs i j) xs i (j + 1) fuel

/-- Nat to Int across both small boundaries and up to the bound. -/
def intNats (kn : Nat) : Array Nat :=
  #[0, 1, 1073741822, 1073741823, 1073741824, 2147483647, 2147483648, 4294967296,
    18446744073709551616, 170141183460469231731687303715884105727].map (· + kn)

def intOfNatLoop (o : ByteArray) (ns : Array Nat) (i : Nat) : ByteArray :=
  if h : i < ns.size then
    let n := ns[i]
    let p := pre "int.nat." i
    let o := ki o (p ++ "ofNat") (Int.ofNat n)
    let o := ki o (p ++ "negSucc") (Int.negSucc n)
    let o := kv o (p ++ "ofNat.toNat") (Int.ofNat n).toNat
    let o := kv o (p ++ "negSucc.natAbs") (Int.negSucc n).natAbs
    let o := kv o (p ++ "negSucc.toNat") (Int.negSucc n).toNat
    intOfNatLoop o ns (i + 1)
  else o
termination_by ns.size - i

/-- At the bound [-2^127, 2^127). -/
def intBound (o : ByteArray) (k : Int) (kn : Nat) : ByteArray :=
  let mx := intMaxLit + k
  let mn := -mx - 1
  let o := ki o "int.max" mx
  let o := ki o "int.min" mn
  let o := ki o "int.max.neg" (-mx)
  let o := ki o "int.max.pred" (mx - 1)
  let o := ki o "int.min.succ" (mn + 1)
  let o := ki o "int.min+max" (mn + mx)
  let o := ki o "int.max-max" (mx - mx)
  let o := kv o "int.min.natAbs" mn.natAbs
  let o := kv o "int.min.toNat" mn.toNat
  let o := kv o "int.max.toNat" mx.toNat
  let o := ki o "int.min.mul1" (mn * 1)
  let o := ki o "int.max.mul-1" (mx * -1)
  let o := ki o "int.2^126.mul-2" ((mx / 2 + 1) * -2)
  let o := ki o "int.min.ediv2" (mn / 2)
  let o := ki o "int.min.ediv-2" (mn / -2)
  let o := ki o "int.min.ediv3" (mn / 3)
  let o := ki o "int.min.emod3" (mn % 3)
  let o := ki o "int.min.emod-3" (mn % -3)
  let o := ki o "int.min.tdiv3" (mn.tdiv 3)
  let o := ki o "int.min.tmod3" (mn.tmod 3)
  let o := ki o "int.min.ediv1" (mn / 1)
  let o := ki o "int.min.emodmin" (mn % mn)
  let o := ki o "int.min.edivmax" (mn / mx)
  let o := ki o "int.min.emodmax" (mn % mx)
  let o := ki o "int.max.edivmin" (mx / mn)
  let o := ki o "int.max.emodmin" (mx % mn)
  let o := ki o "int.max.tmodmin" (mx.tmod mn)
  let o := ki o "int.min.shr1" (mn >>> (1 + kn))
  let o := ki o "int.min.shr126" (mn >>> (126 + kn))
  let o := ki o "int.min.shr127" (mn >>> (127 + kn))
  let o := ki o "int.min.shr500" (mn >>> (500 + kn))
  let o := ki o "int.max.shr126" (mx >>> (126 + kn))
  let o := ki o "int.max.shr127" (mx >>> (127 + kn))
  let o := ki o "int.min.i64" mn.toInt64.toInt
  let o := ki o "int.max.i64" mx.toInt64.toInt
  let o := ki o "int.max.i32" mx.toInt32.toInt
  let o := ki o "int.max.i8" mx.toInt8.toInt
  let o := kb o "int.min<max" (decide (mn < mx))
  let o := kb o "int.max<min" (decide (mx < mn))
  let o := kb o "int.min<-1" (decide (mn < -1))
  let o := kb o "int.min==min" (mn == -mx - 1)
  let o := ki o "int.pow.-2^127" ((-2 + k) ^ (127 + kn))
  let o := ki o "int.pow.2^126" ((2 + k) ^ (126 + kn))
  let o := ki o "int.pow.-2^63" ((-2 + k) ^ (63 + kn))
  let o := ki o "int.pow.-3^41" ((-3 + k) ^ (41 + kn))
  let o := ki o "int.pow.-7^5" ((-7 + k) ^ (5 + kn))
  let o := ki o "int.pow.-1^2^64" ((-1 + k) ^ (18446744073709551616 + kn))
  let o := ki o "int.pow.-1^2^64+1" ((-1 + k) ^ (18446744073709551617 + kn))
  let o := ki o "int.pow.0^2^64" ((0 + k) ^ (18446744073709551616 + kn))
  let o := ki o "int.pow.0^0" (k ^ kn)
  let o := ki o "int.shl.-1<<<127" ((-1 + k) <<< (127 + kn))
  let o := ki o "int.shl.1<<<126" ((1 + k) <<< (126 + kn))
  let o := ki o "int.shl.-3<<<64" ((-3 + k) <<< (64 + kn))
  let o := ki o "int.shl.min<<<0" (mn <<< kn)
  let o := ki o "int.shl.0<<<200" (k <<< (200 + kn))
  ki o "int.negSucc.max" (Int.negSucc (170141183460469231731687303715884105727 + kn))

def closedIntSmall : Int := -5
def closedIntBig : Int := -18446744073709551621
def closedIntArr : Array Int := #[-1, 1073741824, -1073741825, 18446744073709551621]

structure IntRow where
  xs : Array Int
  n : Int
  deriving Inhabited

/-- Many big Ints allocated and dropped, through negative values. -/
def intChurn (k : Int) : Nat → Int → Int
  | 0, acc => acc
  | n + 1, acc => intChurn k n ((acc * 3 - 1180591620717411303424 + k - n) % 1267650600228229401496703205383)

def intSection (o : ByteArray) (k : Nat) : ByteArray :=
  let ki0 : Int := Int.ofNat k
  let o := intAll o 0 (intVals ki0) (intDivisors ki0) k 100
  let o := intPairLoop o (intPairVals ki0) 0 0 1000
  let o := intOfNatLoop o (intNats k) 0
  let o := intBound o ki0 k
  let o := ki o "int.closed.small" (closedIntSmall * (3 + ki0))
  let o := ki o "int.closed.big" (closedIntBig - ki0)
  let o := ki o "int.closed.big.again" closedIntBig
  let o := ki o "int.closed.arr.3" closedIntArr[3]!
  let o := ki o "int.closed.arr.oob" closedIntArr[7 + k]!
  let rows : Array IntRow := #[{ xs := closedIntArr, n := -2 + ki0 }]
  let r := rows[3 + k]!
  let o := kv o "int.row.default.size" r.xs.size
  let o := ki o "int.row.default.n" r.n
  let o := ki o "int.row.0.n" (rows[k]!).n
  -- the default of an array element that is an array (Array.instInhabited)
  let aa : Array (Array Int) := #[closedIntArr, #[ki0]]
  let o := kv o "int.arrarr.oob.size" (aa[5 + k]!).size
  let o := kv o "int.arrarr.1.size" (aa[1 + k]!).size
  ki o "int.churn" (intChurn ki0 3000 (-1))

/-! ## The transcript -/

unsafe def transcriptImpl (k : Nat) : ByteArray :=
  let o := ByteArray.emptyWithCapacity 65536
  let o := str o "lean-conformance v1\n"
  let o := natSection o k
  let o := uintSection o k
  let o := arraySection o k
  let o := byteArraySection o k
  let o := closureSection o k
  let o := stringSection o k
  let o := closedSection o k
  let o := panicSection o k
  let o := churnSection o k
  let o := v435Section o k
  let o := intSection o k
  str o "end\n"

@[implemented_by transcriptImpl]
opaque transcript (k : Nat) : ByteArray

end ZrLean.Conformance
