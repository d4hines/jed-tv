import JsbExact.Scalar

/-! `JsbExact.S` at `Float`.

The generic model (`JsbExact.Model` and friends) is written over `Scalar α`, and Lean stores the fields of a
polymorphic structure as boxed objects: one heap allocation per number. The `JsbExact.F` modules are the same
model with `α := Float` written out, so every field is an unboxed `double`.

Each helper here is the operation the `Scalar Float` instance in `Scalar.lean` uses, under the same name, so
`F/LinAlg.lean`, `F/Table.lean`, `F/C172Gen.lean` and `F/Model.lean` read the same as the generic files. The
trigonometric functions go through the same `@[extern]` `jx_*` opaques, so the two models call the same C. -/

namespace JsbExact.F

namespace S
/-- `Scalar.ofFloat` for `Float` is the identity; kept so literals read as in the generic code. -/
@[inline] def lit (x : Float) : Float := x
@[inline] def zero : Float := lit 0.0
@[inline] def one : Float := lit 1.0
/-- C `a*b + c` under contraction. -/
@[inline] def mac (a b c : Float) : Float := Float.fma a b c
/-- C `a*b - c` under contraction. -/
@[inline] def msc (a b c : Float) : Float := Float.fma a b (-c)
/-- C `c - a*b` under contraction. -/
@[inline] def nmac (a b c : Float) : Float := Float.fma (-a) b c
@[inline] def lt (a b : Float) : Bool := a < b
@[inline] def le (a b : Float) : Bool := a ≤ b
@[inline] def gt (a b : Float) : Bool := b < a
@[inline] def ge (a b : Float) : Bool := b ≤ a
@[inline] def eq (a b : Float) : Bool := a == b
@[inline] def ne (a b : Float) : Bool := !(a == b)
/-- `std::max(a, b)`. -/
@[inline] def max (a b : Float) : Float := if lt a b then b else a
/-- `std::min(a, b)`. -/
@[inline] def min (a b : Float) : Float := if lt b a then b else a
/-- `FGJSBBase::Constrain(lo, x, hi)`. -/
@[inline] def constrain (lo x hi : Float) : Float := if lt x lo then lo else if gt x hi then hi else x
/-- `FGJSBBase::sign`. -/
@[inline] def sign (x : Float) : Float := if ge x zero then one else lit (-1.0)
@[inline] def sqrt (x : Float) : Float := Float.sqrt x
@[inline] def sin (x : Float) : Float := jxSin x
@[inline] def cos (x : Float) : Float := jxCos x
/-- `sin` where the C function also takes `cos` of the same argument (clang emits `__sincos_stret`). -/
@[inline] def sinS (x : Float) : Float := jxSincosSin x
@[inline] def cosS (x : Float) : Float := jxSincosCos x
@[inline] def atan (x : Float) : Float := Float.atan x
@[inline] def asin (x : Float) : Float := Float.asin x
@[inline] def atan2 (y x : Float) : Float := Float.atan2 y x
@[inline] def pow (x y : Float) : Float := Float.pow x y
@[inline] def exp (x : Float) : Float := Float.exp x
@[inline] def abs (x : Float) : Float := Float.abs x
end S

/-! ### The two helper sets agree bit for bit at `Float`

Literals in the generic code are `S.lit c` = `Scalar.ofFloat c`; here they are `c`. Every arithmetic helper
below is compared against the generic one instantiated at `Float` on the same inputs. (`jx_*` are external C
and cannot run at elaboration time; they are the same opaque constants on both sides by construction.) -/

private def sameBits (a b : Float) : Bool := a.toBits == b.toBits
/-- `1 + 2^-30`: its square is `1 + 2^-29 + 2^-60`, which a fused multiply-add keeps and a separate multiply
rounds away, so the `mac`/`msc`/`nmac` checks below tell `Float.fma` from `a*b ± c`. -/
private def d : Float := 1.0 + 9.313225746154785e-10
#guard !(sameBits (Float.fma d d (-1.0)) (d * d - 1.0))

#guard sameBits (JsbExact.S.lit 0.5) (S.lit 0.5)
#guard sameBits (JsbExact.S.lit (-1.0)) (S.lit (-1.0))
#guard sameBits (JsbExact.S.lit 1e-10) (S.lit 1e-10)
#guard sameBits (JsbExact.S.lit (1.0 / 12.0)) (S.lit (1.0 / 12.0))
#guard sameBits (JsbExact.S.lit (0.5 * 3.141592653589793)) (S.lit (0.5 * 3.141592653589793))
#guard sameBits (JsbExact.S.lit (2 * 3.141592653589793)) (S.lit (2 * 3.141592653589793))
#guard sameBits (JsbExact.S.lit (1e-15 * 0.0208854342)) (S.lit (1e-15 * 0.0208854342))
#guard sameBits (JsbExact.S.lit 0.01745) (S.lit 0.01745)
#guard sameBits (JsbExact.S.lit 999999.0) (S.lit 999999.0)
#guard sameBits (JsbExact.S.lit (-0.484)) (S.lit (-0.484))
#guard sameBits (JsbExact.S.zero : Float) S.zero
#guard sameBits (JsbExact.S.one : Float) S.one
#guard sameBits (JsbExact.S.mac d d (-1.0)) (S.mac d d (-1.0))
#guard sameBits (JsbExact.S.msc d d 1.0) (S.msc d d 1.0)
#guard sameBits (JsbExact.S.nmac d d 1.0) (S.nmac d d 1.0)
#guard !(sameBits (S.msc d d 1.0) (d * d - 1.0))
#guard !(sameBits (S.mac d d (-1.0)) (d * d + (-1.0)))
#guard !(sameBits (S.nmac d d 1.0) (1.0 - d * d))
#guard sameBits (JsbExact.S.sign (-0.0 : Float)) (S.sign (-0.0))
#guard sameBits (JsbExact.S.sign (-2.0 : Float)) (S.sign (-2.0))
#guard sameBits (JsbExact.S.constrain (-1.0 : Float) 2.5 1.0) (S.constrain (-1.0) 2.5 1.0)
#guard sameBits (JsbExact.S.max (1.0 : Float) 2.0) (S.max 1.0 2.0)
#guard sameBits (JsbExact.S.min (1.0 : Float) 2.0) (S.min 1.0 2.0)
#guard sameBits (JsbExact.S.sqrt (2.0 : Float)) (S.sqrt 2.0)
#guard sameBits (JsbExact.S.atan (0.7 : Float)) (S.atan 0.7)
#guard sameBits (JsbExact.S.asin (0.7 : Float)) (S.asin 0.7)
#guard sameBits (JsbExact.S.atan2 (0.7 : Float) (-0.2)) (S.atan2 0.7 (-0.2))
#guard sameBits (JsbExact.S.pow (0.7 : Float) 3.5) (S.pow 0.7 3.5)
#guard sameBits (JsbExact.S.exp (0.7 : Float)) (S.exp 0.7)
#guard sameBits (JsbExact.S.abs (-0.7 : Float)) (S.abs (-0.7))
#guard JsbExact.S.lt (1.0 : Float) 2.0 == S.lt 1.0 2.0
#guard JsbExact.S.le (2.0 : Float) 2.0 == S.le 2.0 2.0
#guard JsbExact.S.gt (1.0 : Float) 2.0 == S.gt 1.0 2.0
#guard JsbExact.S.ge (2.0 : Float) 2.0 == S.ge 2.0 2.0
#guard JsbExact.S.eq (2.0 : Float) 2.0 == S.eq 2.0 2.0
#guard JsbExact.S.ne (2.0 : Float) 2.0 == S.ne 2.0 2.0

end JsbExact.F
