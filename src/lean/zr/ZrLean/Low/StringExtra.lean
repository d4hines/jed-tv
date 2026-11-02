import ZrLean.Low.Str
import ZrLean.Low.InitShim
import ZrLean.Low.HeaderInt

/-! Additional standard-library ABI operations. Arguments use the pinned Lean
runtime representation; borrowed inputs are never consumed. All allocation,
reference counting and memory reads are explicit BaseIO effects. -/
set_option compiler.extract_closed false
namespace ZrLean.Low.StringExtra
open ZrLean.Low ZrLean.Low.Obj ZrLean.Low.Rc

private def emptyString : BaseIO USize := do
  let text ← HeaderSupport.allocString 1 1 0
  st8 (text + strData 0) 0
  pure text

private partial def intercalateLoop (separator rest text : USize) (first : Bool) : BaseIO USize := do
  if rest &&& 1 != 0 then return text
  let text ← if first then pure text else Str.append text separator
  let head ← ldp (rest + ctorObjs 0)
  let text ← Str.append text head
  intercalateLoop separator (← ldp (rest + ctorObjs 0 + wordBytes 0)) text false

/-- `String.intercalate (separator : @& obj) (parts : tobj) : obj`.
Keep the complete owned list alive while borrowing its strings, then release it. -/
@[export zr_low_string_intercalate]
def intercalate (separator parts : USize) : BaseIO USize := do
  let text ← intercalateLoop separator parts (← emptyString) true
  dec parts
  pure text

/-- `Int.repr (value : @& tobj) : obj`. Zr's signed 128-bit bound includes
-2^127; its magnitude is a valid unsigned Nat and is consumed by natRepr. -/
@[export zr_low_int_repr]
def intRepr (value : USize) : BaseIO USize := do
  let nonnegative ← HeaderInt.decNonneg value
  let text ← InitShim.natRepr (← HeaderInt.natAbs value)
  if nonnegative then return text
  let minus ← HeaderSupport.allocString 2 2 1
  st8 (minus + strData 0) 45
  st8 (minus + strData 0 + 1) 0
  let result ← Str.append minus text
  dec text
  pure result

/-- Pinned `lean_string_utf8_get_fast`: both inputs borrowed. The byte position
must be an in-bounds small Nat, as guaranteed by its Lean proof argument.
The terminal NUL is readable; malformed encodings use upstream's default 'A'. -/
@[export zr_low_string_utf8_get_fast]
def utf8GetFast (text position : USize) : BaseIO UInt32 := do
  let index := position >>> 1
  let data := text + strData 0
  let first := (← ld8 (data + index)).toUInt32
  if first &&& 0x80 == 0 then return first
  let size ← ldp (text + strSizeOff 0)
  if first &&& 0xe0 == 0xc0 && index + 1 < size then
    let second := (← ld8 (data + index + 1)).toUInt32
    let value := ((first &&& 0x1f) <<< 6) ||| (second &&& 0x3f)
    if value >= 0x80 then return value
  if first &&& 0xf0 == 0xe0 && index + 2 < size then
    let second := (← ld8 (data + index + 1)).toUInt32
    let third := (← ld8 (data + index + 2)).toUInt32
    let value := ((first &&& 0x0f) <<< 12) ||| ((second &&& 0x3f) <<< 6) ||| (third &&& 0x3f)
    if value >= 0x800 && (value < 0xd800 || value > 0xdfff) then return value
  if first &&& 0xf8 == 0xf0 && index + 3 < size then
    let second := (← ld8 (data + index + 1)).toUInt32
    let third := (← ld8 (data + index + 2)).toUInt32
    let fourth := (← ld8 (data + index + 3)).toUInt32
    let value := ((first &&& 7) <<< 18) ||| ((second &&& 0x3f) <<< 12) |||
      ((third &&& 0x3f) <<< 6) ||| (fourth &&& 0x3f)
    if value >= 0x10000 && value <= 0x10ffff then return value
  return 65

/-- Pinned `lean_string_utf8_next_fast`: both inputs borrowed. Advance an
in-bounds small byte position by the width indicated by its leading byte;
upstream also advances by one for an invalid lead or the terminal NUL. -/
@[export zr_low_string_utf8_next_fast]
def utf8NextFast (text position : USize) : BaseIO USize := do
  let index := position >>> 1
  let first ← ld8 (text + strData 0 + index)
  let width : USize :=
    if first &&& 0x80 == 0 then 1 else if first &&& 0xe0 == 0xc0 then 2
    else if first &&& 0xf0 == 0xe0 then 3 else if first &&& 0xf8 == 0xf0 then 4 else 1
  pure (BigNat.boxed (index + width))

end ZrLean.Low.StringExtra
