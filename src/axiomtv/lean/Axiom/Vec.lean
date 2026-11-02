namespace Axiom

/-- A point or direction. Upstream used Raymath's `Vector3`; this is the same
record, so the physics code is unchanged. -/
structure Vector3 where
  x : Float32
  y : Float32
  z : Float32
deriving Inhabited, Repr

end Axiom
