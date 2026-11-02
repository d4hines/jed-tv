import Doom.Playsim.Tables

/-!
# Doom.Render.Packed

Read-mostly 32-bit tables stored unboxed. On 32-bit ARM, `Array Int32` holds
every element as its own heap object: a read loads a pointer, touches two
reference counts and loads again. These tables keep native-endian words in a
`ByteArray` and read one with a single load (bounds checked in Lean). The
interface mirrors the `Array` calls the renderer uses (`getD`, `size`).
-/

namespace Doom.Render.Packed

/-- One aligned 32-bit load from a byte array; `get`/`getD` check the bounds. -/
@[extern c inline "((uint32_t*)lean_sarray_cptr(#1))[lean_unbox(#2)]"]
opaque loadU32 (a : @& ByteArray) (i : @& Nat) : UInt32

structure U32Table where
  bytes : ByteArray := .empty
  deriving Inhabited

private def packU32 (n : Nat) (f : Nat → UInt32) : ByteArray := Id.run do
  let mut b := ByteArray.emptyWithCapacity (n * 4)
  for i in [0:n] do
    let u := f i
    b := b.push u.toUInt8 |>.push (u >>> 8).toUInt8 |>.push (u >>> 16).toUInt8 |>.push (u >>> 24).toUInt8
  return b

namespace U32Table
def ofArray (a : Array UInt32) : U32Table := ⟨packU32 a.size fun i => a[i]!⟩
@[inline] def size (t : U32Table) : Nat := t.bytes.size / 4
@[inline] def getD (t : U32Table) (i : Nat) (d : UInt32) : UInt32 :=
  if i * 4 + 4 ≤ t.bytes.size then loadU32 t.bytes i else d
end U32Table

structure I32Table where
  bytes : ByteArray := .empty
  deriving Inhabited

namespace I32Table
def ofArray (a : Array Int32) : I32Table := ⟨packU32 a.size fun i => a[i]!.toUInt32⟩
@[inline] def size (t : I32Table) : Nat := t.bytes.size / 4
@[inline] def getD (t : I32Table) (i : Nat) (d : Int32) : Int32 :=
  if i * 4 + 4 ≤ t.bytes.size then (loadU32 t.bytes i).toInt32 else d
end I32Table

/-- `tables.c` lookups for the renderer, packed once at startup. -/
def finesine : I32Table := .ofArray Doom.Playsim.Tables.finesine
def finecosine : I32Table := .ofArray Doom.Playsim.Tables.finecosine
def finetangent : I32Table := .ofArray Doom.Playsim.Tables.finetangent
def tantoangle : U32Table := .ofArray Doom.Playsim.Tables.tantoangle

end Doom.Render.Packed
