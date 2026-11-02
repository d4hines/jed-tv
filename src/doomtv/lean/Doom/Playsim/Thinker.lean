/-!
# Doom.Playsim.Thinker

Thinker function ids matching `docs/TRACE.md` §6.2 (`THF_*`), and the
thinker-list entry encoding.

The list is an `Array Nat` in C list order. Each entry is
`kind * 2^26 + payload index`; small `Nat`s stay unboxed on 32-bit targets.
The C `function` pointer lives in the payload (`func` field), so removal
and stasis are O(1) and the list is compacted lazily by `P_RunThinkers`,
exactly when C frees the node.
-/

namespace Doom.Playsim.Thinker

def THF_REMOVED : UInt32 := 0
def THF_MOBJ : UInt32 := 1
def THF_VERTICALDOOR : UInt32 := 2
def THF_MOVECEILING : UInt32 := 3
def THF_MOVEFLOOR : UInt32 := 4
def THF_PLATRAISE : UInt32 := 5
def THF_LIGHTFLASH : UInt32 := 6
def THF_STROBEFLASH : UInt32 := 7
def THF_GLOW : UInt32 := 8
def THF_FIREFLICKER : UInt32 := 9
def THF_NULL : UInt32 := 10

/-- Thinker payload kinds. -/
def TK_MOBJ : Nat := 0
def TK_DOOR : Nat := 1
def TK_CEILING : Nat := 2
def TK_FLOOR : Nat := 3
def TK_PLAT : Nat := 4
def TK_FLASH : Nat := 5
def TK_STROBE : Nat := 6
def TK_GLOW : Nat := 7
def TK_FIRE : Nat := 8

def TK_SHIFT : Nat := 67108864  -- 2^26

@[inline] def thRef (kind idx : Nat) : Nat := kind * TK_SHIFT + idx
@[inline] def thKind (r : Nat) : Nat := r / TK_SHIFT
@[inline] def thIdx (r : Nat) : Nat := r % TK_SHIFT

/-- One thinker list entry as emitted by the trace (`TraceEmit`). -/
structure Thinker where
  traceId : UInt32
  func : UInt32
  payload : UInt32
  deriving Repr

end Doom.Playsim.Thinker
