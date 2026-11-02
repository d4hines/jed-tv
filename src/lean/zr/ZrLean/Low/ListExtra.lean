/-
Additional Init list/array entry points used by the offline RPG. Each USize
is a raw upstream Lean value. Borrow/consume contracts are checked against
upstream IR by test/rpg/check-runtime-abi.lean.
-/
import ZrLean.Low.InitShim
import ZrLean.Low.Apply

set_option compiler.extract_closed false

namespace ZrLean.Low.ListExtra
open ZrLean.Low ZrLean.Low.Obj ZrLean.Low.Rc

private def cons (head tail : USize) : BaseIO USize := do
  let node ← HeaderObj.allocCtor 1 2 0
  stp (node + ctorObjs 0) head
  stp (node + ctorObjs 0 + wordBytes 0) tail
  pure node

private partial def nodeAt (xs n : USize) : BaseIO USize := do
  if xs == 1 then pure 0
  else if n == 0 then pure xs
  else nodeAt (← ldp (xs + ctorObjs 0 + wordBytes 0)) (n - 1)

/-- List.get?Internal: borrowed list, consumed Nat; owned Option element. -/
@[export zr_low_shim_list_get_opt]
def getOpt (xs index : USize) : BaseIO USize := do
  let node ← nodeAt xs (Arr.natToSizeSat index)
  dec index
  if node == 0 then pure 1 else
    let value ← ldp (node + ctorObjs 0)
    inc value
    let result ← HeaderObj.allocCtor 1 1 0
    stp (result + ctorObjs 0) value
    pure result

/-- List.getD: borrowed list/default, consumed Nat. -/
@[export zr_low_shim_list_get_d]
def getD (xs index fallback : USize) : BaseIO USize := do
  let node ← nodeAt xs (Arr.natToSizeSat index)
  dec index
  let result ← if node == 0 then pure fallback else ldp (node + ctorObjs 0)
  inc result
  pure result

/-- List.lengthTR: borrowed list. -/
@[export zr_low_shim_list_length]
def length (xs : USize) : BaseIO USize := do
  BigNat.ofWords (← Str.listLength xs 0).toUInt64 0

/-- List.drop: consumed count, borrowed list, owned result. -/
@[export zr_low_shim_list_drop]
def drop (count xs : USize) : BaseIO USize := do
  let node ← nodeAt xs (Arr.natToSizeSat count)
  dec count
  let result := if node == 0 then 1 else node
  inc result
  pure result

/-- List.isEmpty: borrowed list. -/
@[export zr_low_shim_list_is_empty]
def isEmpty (xs : USize) : BaseIO UInt8 := pure (if xs == 1 then 1 else 0)

/-- Array.appendList's specialized foldl: consumed array and list. -/
@[export zr_low_shim_array_append_list]
partial def appendList (array xs : USize) : BaseIO USize := do
  if xs == 1 then pure array else
    let head ← ldp (xs + ctorObjs 0)
    let tail ← ldp (xs + ctorObjs 0 + wordBytes 0)
    inc head
    inc tail
    dec xs
    appendList (← Arr.arrayPush array head) tail

/-- Nat's equality closure consumes both arguments and returns boxed Bool. -/
@[export zr_low_shim_nat_dec_eq_boxed]
def natDecEq (a b : USize) : BaseIO USize := do
  let same ← BigNat.eq a b
  dec a
  dec b
  pure (if same then 3 else 1)

/-- Option.decEq consumes its equality closure and both optional values. -/
@[export zr_low_shim_option_dec_eq]
def optionDecEq (eq a b : USize) : BaseIO UInt8 := do
  if a == 1 || b == 1 then
    let same := a == b
    dec eq
    dec a
    dec b
    pure (if same then 1 else 0)
  else
    let x ← ldp (a + ctorObjs 0)
    let y ← ldp (b + ctorObjs 0)
    inc x
    inc y
    dec a
    dec b
    let result ← Apply.apply2 eq x y
    let same := result != 1
    dec result
    pure (if same then 1 else 0)

private partial def modifyLoop (xs index f revHead : USize) : BaseIO USize := do
  if xs == 1 then
    dec f
    InitShim.listAppend (← InitShim.listReverse revHead) 1
  else
    let head ← ldp (xs + ctorObjs 0)
    let tail ← ldp (xs + ctorObjs 0 + wordBytes 0)
    inc head
    inc tail
    dec xs
    if index == 0 then
      let changed ← Apply.apply1 f head
      InitShim.listAppend (← InitShim.listReverse revHead) (← cons changed tail)
    else modifyLoop tail (index - 1) f (← cons head revHead)

/-- List.modifyTR consumes list/index/function. Out-of-range is unchanged. -/
@[export zr_low_shim_list_modify]
def modify (xs index f : USize) : BaseIO USize := do
  let n := Arr.natToSizeSat index
  dec index
  modifyLoop xs n f 1

private partial def merge (left right cmp : USize) : BaseIO USize := do
  if left == 1 then pure right else if right == 1 then pure left else
    let a ← ldp (left + ctorObjs 0)
    let b ← ldp (right + ctorObjs 0)
    inc cmp
    inc a
    inc b
    let answer ← Apply.apply2 cmp a b
    let chooseLeft := answer != 1
    dec answer
    let source := if chooseLeft then left else right
    let head ← ldp (source + ctorObjs 0)
    let tail ← ldp (source + ctorObjs 0 + wordBytes 0)
    inc head
    inc tail
    dec source
    let rest ← if chooseLeft then merge tail right cmp else merge left tail cmp
    cons head rest

private partial def sortN (xs n cmp : USize) : BaseIO USize := do
  if n == 0 then pure 1 else if n == 1 then
    let head ← ldp (xs + ctorObjs 0)
    inc head
    cons head 1
  else
    let half := n / 2
    let middle ← nodeAt xs half
    let left ← sortN xs half cmp
    let right ← sortN middle (n - half) cmp
    merge left right cmp

/-- Stable merge sort with upstream's consumed list/comparator ABI. -/
@[export zr_low_shim_list_merge_sort]
def mergeSort (xs cmp : USize) : BaseIO USize := do
  let result ← sortN xs (← Str.listLength xs 0) cmp
  dec xs
  dec cmp
  pure result
end ZrLean.Low.ListExtra
