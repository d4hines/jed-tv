/-!
# Doom.Playsim.NameMap

A string-keyed index for lump and texture names, in core Lean only (the TV
runtime ships `Init`, not `Std`). Fixed bucket count; `insert` replaces an
existing key, as `Std.HashMap.insert` does.
-/

namespace Doom.Playsim

structure NameMap where
  buckets : Array (Array (String × Nat)) := Array.replicate 2048 #[]
  deriving Inhabited

namespace NameMap

instance : Repr NameMap := ⟨fun m _ => s!"NameMap({(m.buckets.foldl (· + ·.size) 0)} entries)"⟩

@[inline] private def slot (m : NameMap) (k : String) : Nat :=
  (hash k).toNat % m.buckets.size

def get? (m : NameMap) (k : String) : Option Nat :=
  if m.buckets.size == 0 then none
  else (m.buckets[m.slot k]!.find? (·.1 == k)).map (·.2)

def contains (m : NameMap) (k : String) : Bool := (m.get? k).isSome

def insert (m : NameMap) (k : String) (v : Nat) : NameMap :=
  if m.buckets.size == 0 then m
  else
    let i := m.slot k
    { buckets := m.buckets.modify i fun b =>
        match b.findIdx? (·.1 == k) with
        | some j => b.set! j (k, v)
        | none => b.push (k, v) }

end NameMap
end Doom.Playsim
