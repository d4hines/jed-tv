import Init
namespace Slides
inductive Style where
  | plain | bold | italic | boldItalic | code
  deriving BEq, Repr, Inhabited
structure Span where
  text : String
  style : Style := .plain
  deriving BEq, Repr, Inhabited
inductive Block where
  | subtitle (spans : Array Span)
  | para (spans : Array Span)
  | item (depth : Nat) (marker : String) (spans : Array Span)
  | code (language : String) (lines : Array String)
  | image (caption path : String)
  deriving BEq, Repr, Inhabited
structure Slide where
  line : Nat := 1
  title : Array Span := #[]
  blocks : Array Block := #[]
  notes : Array String := #[]
  deriving Repr, Inhabited
/-- Host-generated hex keeps image payloads compact for Lean elaboration. -/
def decodeHex (s : String) : ByteArray := Id.run do
  let mut out := ByteArray.emptyWithCapacity (s.utf8ByteSize/2)
  let bytes := s.toUTF8
  let nibble (c : UInt8) : UInt8 := if c ≤ 57 then c-48 else c-87
  for i in [:bytes.size/2] do
    out := out.push (nibble bytes[2*i]! * 16 + nibble bytes[2*i+1]!)
  return out
structure Image where
  path : String
  width : Nat
  height : Nat
  rgba : ByteArray
  deriving Inhabited
structure Deck where
  slides : Array Slide
  images : Array Image := #[]
  deriving Inhabited
def plain (xs : Array Span) : String := String.join (xs.toList.map (·.text))
end Slides
