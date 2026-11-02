import Axiom.World

namespace Axiom

/-- Visible faces of one world section, 8 bytes per face: block x y z, face
(0 top, 1 bottom, 2 north, 3 south, 4 west, 5 east), then r g b a. Upstream
expanded each face into six GPU vertices; the TV renderer (gfx) expands the
same faces itself, so the mesh stays compact in Lean. -/
structure MeshCpu where
  faces : ByteArray := ByteArray.empty
deriving Inhabited

def MeshCpu.faceCount (mesh : MeshCpu) : Nat := mesh.faces.size / 8

/-- Six vertices per face, as upstream counted them. -/
def MeshCpu.vertexCount (mesh : MeshCpu) : Nat := mesh.faceCount * 6

@[inline] def clampByte (value : Int) : UInt8 :=
  UInt8.ofNat (max 0 (min 255 value) |>.toNat)

def shadeColor (base : Rgba) (face : Nat) (x y z : Nat) : Rgba :=
  let faceLight : Int :=
    match face with
    | 0 => 24   -- top
    | 1 => -42  -- bottom
    | 2 => -12  -- north
    | 3 => 5    -- south
    | 4 => -24  -- west
    | _ => -4   -- east
  let grain := Int.ofNat ((hash2 0x51ade x (z + y * 31)) % 13).toNat - 6
  let adjust (channel : UInt8) := clampByte (Int.ofNat channel.toNat + faceLight + grain)
  ⟨adjust base.r, adjust base.g, adjust base.b, base.a⟩

def emitFace (mesh : MeshCpu) (block : UInt8) (face x y z : Nat) : MeshCpu :=
  let color := shadeColor (Block.color block) face x y z
  { faces := mesh.faces
      |>.push (UInt8.ofNat x) |>.push (UInt8.ofNat y) |>.push (UInt8.ofNat z) |>.push (UInt8.ofNat face)
      |>.push color.r |>.push color.g |>.push color.b |>.push color.a }

def neighborForFace (face x y z : Nat) : Int × Int × Int :=
  let xi := Int.ofNat x
  let yi := Int.ofNat y
  let zi := Int.ofNat z
  match face with
  | 0 => (xi, yi + 1, zi)
  | 1 => (xi, yi - 1, zi)
  | 2 => (xi, yi, zi - 1)
  | 3 => (xi, yi, zi + 1)
  | 4 => (xi - 1, yi, zi)
  | _ => (xi + 1, yi, zi)

def buildSectionCpu (world : World) (sectionX sectionY sectionZ : Nat) : MeshCpu := Id.run do
  let startX := sectionX * sectionSize
  let startY := sectionY * sectionSize
  let startZ := sectionZ * sectionSize
  let stopX := Nat.min worldWidth (startX + sectionSize)
  let stopY := Nat.min worldHeight (startY + sectionSize)
  let stopZ := Nat.min worldDepth (startZ + sectionSize)
  let mut mesh : MeshCpu := {}
  for x in [startX:stopX] do
    for y in [startY:stopY] do
      for z in [startZ:stopZ] do
        let block := world.get x y z
        if Block.isSolid block then
          for face in [0:6] do
            let (nx, ny, nz) := neighborForFace face x y z
            if !Block.isSolid (world.getI nx ny nz) then
              mesh := emitFace mesh block face x y z
  return mesh

def sectionCountX : Nat := (worldWidth + sectionSize - 1) / sectionSize
def sectionCountY : Nat := (worldHeight + sectionSize - 1) / sectionSize
def sectionCountZ : Nat := (worldDepth + sectionSize - 1) / sectionSize
def totalSections : Nat := sectionCountX * sectionCountY * sectionCountZ

@[inline] def sectionIndex (sx sy sz : Nat) : Nat :=
  (sy * sectionCountZ + sz) * sectionCountX + sx

/-- Every section's faces, ready to draw. Upstream kept GPU meshes; the TV keeps the bytes. -/
structure WorldMesh where
  sections : Array MeshCpu

def buildWorldMesh (world : World) : WorldMesh := Id.run do
  let mut sections : Array MeshCpu := Array.replicate totalSections {}
  for sx in [0:sectionCountX] do
    for sy in [0:sectionCountY] do
      for sz in [0:sectionCountZ] do
        sections := sections.set! (sectionIndex sx sy sz) (buildSectionCpu world sx sy sz)
  return { sections }

def WorldMesh.rebuildSection (mesh : WorldMesh) (world : World) (sx sy sz : Nat) : WorldMesh :=
  if sx ≥ sectionCountX || sy ≥ sectionCountY || sz ≥ sectionCountZ then mesh
  else { sections := mesh.sections.set! (sectionIndex sx sy sz) (buildSectionCpu world sx sy sz) }

def affectedSections (x y z : Nat) : Array (Nat × Nat × Nat) := Id.run do
  let sx := x / sectionSize
  let sy := y / sectionSize
  let sz := z / sectionSize
  let mut sections := #[(sx, sy, sz)]
  if x % sectionSize == 0 && sx > 0 then sections := sections.push (sx - 1, sy, sz)
  if x % sectionSize == sectionSize - 1 && sx + 1 < sectionCountX then
    sections := sections.push (sx + 1, sy, sz)
  if y % sectionSize == 0 && sy > 0 then sections := sections.push (sx, sy - 1, sz)
  if y % sectionSize == sectionSize - 1 && sy + 1 < sectionCountY then
    sections := sections.push (sx, sy + 1, sz)
  if z % sectionSize == 0 && sz > 0 then sections := sections.push (sx, sy, sz - 1)
  if z % sectionSize == sectionSize - 1 && sz + 1 < sectionCountZ then
    sections := sections.push (sx, sy, sz + 1)
  return sections

/-- Rebuild the sections an edit at (x, y, z) can change. Upstream did this on
worker tasks; the TV's Lean runtime is single-threaded, so it is done at once. -/
def WorldMesh.rebuildAround (mesh : WorldMesh) (world : World) (x y z : Nat) : WorldMesh := Id.run do
  let mut out := mesh
  for (sx, sy, sz) in affectedSections x y z do
    out := out.rebuildSection world sx sy sz
  return out

end Axiom
