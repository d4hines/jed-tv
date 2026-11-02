import Axiom.Mesh
import Axiom.Physics
import Axiom.Effects
import Axiom.Tv

/-!
World presentation, sky, HUD and menus, as upstream's Render.lean, drawn with
the TV platform layer at 320x180 (upstream laid the HUD out for a 1280x760
window; positions here are about a quarter of those).
-/

namespace Axiom

@[inline] def mixByte (a b : UInt8) (t : Float32) : UInt8 :=
  UInt8.ofNat (min 255 (a.toFloat32 + (b.toFloat32 - a.toFloat32) * t).toUInt32.toNat)

def mixRgba (a b : Rgba) (t : Float32) : Rgba :=
  ⟨mixByte a.r b.r t, mixByte a.g b.g t, mixByte a.b b.b t, mixByte a.a b.a t⟩

def daylight (time : Float32) : Float32 :=
  let angle := 0.85 + time * 0.0052359877 -- one full cycle every 20 minutes
  clamp32 0 1 (angle.sin * 1.18 + 0.12)

def drawSky (time : Float32) : IO Unit := do
  let light := daylight time
  let top := mixRgba ⟨15, 23, 48, 255⟩ ⟨77, 126, 166, 255⟩ light
  let bottom := mixRgba ⟨47, 49, 72, 255⟩ ⟨211, 192, 157, 255⟩ light
  atvGradientV 0 0 screenW screenH top.pack bottom.pack
  let night := 1 - light
  if night > 0.05 then
    let starAlpha := UInt8.ofNat (min 220 (night * 230).toUInt32.toNat)
    for i in [0:38] do
      let x := Int32.ofNat ((hash2 0x57a2 i 9 % UInt64.ofNat screenW.toNatClampNeg).toNat)
      let starHeight := max 1 (screenH.toNatClampNeg * 2 / 3)
      let y := Int32.ofNat ((hash2 0x57a2 i 17 % UInt64.ofNat starHeight).toNat)
      let radius : Float32 := if i % 7 == 0 then 0.9 else 0.5
      atvCircle x y radius (rgba 226 233 220 starAlpha)
  let angle := 0.85 + time * 0.0052359877
  let sunX := ((0.5 + angle.cos * 0.42) * screenW.toFloat32).toInt32
  let sunY := ((0.66 - angle.sin * 0.49) * screenH.toFloat32).toInt32
  let sunAlpha := UInt8.ofNat (min 245 (35 + light * 220).toUInt32.toNat)
  atvCircle sunX sunY 18 (rgba 255 190 104 (sunAlpha / 8))
  atvCircle sunX sunY 10.5 (rgba 255 222 156 (sunAlpha / 3))
  atvCircle sunX sunY 4.5 (rgba 255 239 196 sunAlpha)

def drawWorldTint (time : Float32) : IO Unit := do
  let darkness := 1 - daylight time
  if darkness > 0.02 then
    let alpha := UInt8.ofNat (min 112 (darkness * 112).toUInt32.toNat)
    atvRect 0 0 screenW screenH (rgba 13 20 42 alpha)

@[inline] def wrap32 (value span : Float32) : Float32 :=
  value - (value / span).floor * span

def drawClouds (time : Float32) : IO Unit := do
  let white := rgba 245 240 225 185
  for i in [0:12] do
    let fi := i.toFloat32
    let x := wrap32 (fi * 17.3 + time * (0.38 + fi * 0.012)) 118 - 11
    let z := wrap32 (fi * 29.1 + 7) 112 - 8
    let y := 33 + (fi * 1.7).sin * 2.5
    atvCube x y z (8 + (i % 3).toFloat32 * 2.5) 0.65 3.8 white
    atvCube (x + 3.5) (y + 0.35) (z + 0.8) 5.5 0.7 3.1 white

def drawOcean : IO Unit :=
  atvPlane (worldWidth.toFloat32 / 2) (seaLevel.toFloat32 + 0.52) (worldDepth.toFloat32 / 2) 108 108
    (rgba 46 126 162 255)

def drawGlowHalos (time : Float32) : IO Unit := do
  let pulse := 0.13 * (time * 2.2).sin + 0.58
  let color := rgba 255 184 72 (UInt8.ofNat (80 + (pulse * 70).toUInt32.toNat))
  -- Upstream drew spheres; the TV renderer has boxes.
  for p in #[Vector3.mk 69.5 28.5 62.5, Vector3.mk 75.5 28.5 62.5, Vector3.mk 72.5 32.5 62.5] do
    atvCube p.x p.y p.z (pulse * 1.6) (pulse * 1.6) (pulse * 1.6) color

def drawSelection (hit : Option VoxelHit) : IO Unit := do
  if let some h := hit then
    atvCubeWires ((Int32.ofInt h.x).toFloat32 + 0.5) ((Int32.ofInt h.y).toFloat32 + 0.5)
      ((Int32.ofInt h.z).toFloat32 + 0.5) 1.014 (rgba 255 244 204 245)

def drawParticles (particles : Array Particle) : IO Unit := do
  for particle in particles do
    let alpha := UInt8.ofNat (min 255 (80 + (particle.life * 310).toUInt32.toNat))
    atvCube particle.position.x particle.position.y particle.position.z
      particle.size particle.size particle.size (rgba particle.color.r particle.color.g particle.color.b alpha)

/-- Every section's faces. The renderer skips sections outside the view. -/
def drawWorld (mesh : WorldMesh) : IO Unit := do
  for sy in [0:sectionCountY] do
    for sz in [0:sectionCountZ] do
      for sx in [0:sectionCountX] do
        let part := mesh.sections[sectionIndex sx sy sz]!
        if part.faces.size > 0 then
          atvSection (UInt32.ofNat (sx * sectionSize)) (UInt32.ofNat (sy * sectionSize))
            (UInt32.ofNat (sz * sectionSize)) (UInt32.ofNat sectionSize) part.faces

def drawCrosshair : IO Unit := do
  let cx := screenW / 2
  let cy := screenH / 2
  let shadow := rgba 12 18 24 170
  let light := rgba 255 249 224 245
  atvLine (cx - 3) (cy + 1) (cx + 3) (cy + 1) shadow
  atvLine (cx + 1) (cy - 3) (cx + 1) (cy + 3) shadow
  atvLine (cx - 3) cy (cx + 3) cy light
  atvLine cx (cy - 3) cx (cy + 3) light

def drawHotbar (player : Player) : IO Unit := do
  let slot : Int32 := 14
  let gap : Int32 := 2
  let total := slot * 9 + gap * 8
  let startX := (screenW - total) / 2
  let y := screenH - 22
  for i in [0:Block.palette.size] do
    let x := startX + Int32.ofNat i * (slot + gap)
    let selected := i == player.selected
    let panel := if selected then rgba 247 232 192 235 else rgba 20 28 35 190
    let border := if selected then rgba 255 244 210 255 else rgba 133 158 166 145
    atvRect x y slot slot panel
    atvRect x y slot 1 border
    atvRect x (y + slot - 1) slot 1 border
    atvRect x y 1 slot border
    atvRect (x + slot - 1) y 1 slot border
    let block := Block.palette[i]!
    atvRect (x + 3) (y + 3) 8 8 (Block.color block).pack
  let selectedBlock := Block.palette[player.selected]!
  let label := Block.name selectedBlock
  atvText ((screenW - textWidth 1 label) / 2) (y - 11) 1 (rgba 255 247 224 235) label

def drawStatus (player : Player) (showDebug : Bool) : IO Unit := do
  atvRect 5 4 112 12 (rgba 16 25 32 185)
  atvText 8 6 1 (rgba 246 232 193 245) "AXIOM // LEAN 4"
  if player.flying then
    atvRect 5 18 96 12 (rgba 170 111 55 205)
    atvText 8 20 1 (rgba 245 245 245 255) "FLIGHT MODE"
  if showDebug then
    atvRect 4 34 150 30 (rgba 10 15 20 190)
    atvText 7 37 1 (rgba 245 245 245 255) s!"x {player.position.x.toInt32} y {player.position.y.toInt32}"
    atvText 7 47 1 (rgba 245 245 245 255) s!"z {player.position.z.toInt32}"

def drawHud (player : Player) (showDebug : Bool) : IO Unit := do
  drawCrosshair
  drawHotbar player
  drawStatus player showDebug

def drawToast (message : String) : IO Unit := do
  let width := textWidth 1 message
  let x := (screenW - width) / 2
  atvRect (x - 4) 30 (width + 8) 12 (rgba 18 27 33 218)
  atvText x 32 1 (rgba 255 240 207 245) message

def drawTitleOverlay (hasSave : Bool) : IO Unit := do
  atvGradientH 0 0 (screenW * 3 / 5) screenH (rgba 10 18 25 235) (rgba 10 18 25 0)
  atvText 18 26 4 (rgba 255 240 203 255) "AXIOM"
  atvText 20 60 1 (rgba 235 176 104 255) "FIRST LIGHT"
  atvText 20 72 1 (rgba 203 220 220 235) "A voxel sandbox in Lean 4"
  atvRect 18 90 136 16 (rgba 235 215 173 226)
  atvText 24 94 1 (rgba 24 35 40 255) (if hasSave then "OK  CONTINUE" else "OK  BEGIN")
  atvText 20 116 1 (rgba 238 238 220 220) "UP/DOWN move  LEFT/RIGHT turn"
  atvText 20 126 1 (rgba 238 238 220 220) "CH+/CH- look  NETFLIX jump"
  atvText 20 136 1 (rgba 238 238 220 220) "OK mine  PRIME build  YOUTUBE block"
  atvText 20 146 1 (rgba 238 238 220 220) "BACK pause"
  atvText 20 (screenH - 14) 1 (rgba 180 199 200 180) "Every block rule runs in Lean."

def drawPauseOverlay : IO Unit := do
  atvRect 0 0 screenW screenH (rgba 7 12 17 178)
  let panelX := screenW / 2 - 110
  let panelY := screenH / 2 - 52
  atvRect panelX panelY 220 104 (rgba 22 31 38 244)
  atvText (panelX + 24) (panelY + 10) 2 (rgba 255 236 197 255) "PAUSED"
  atvText (panelX + 12) (panelY + 36) 1 (rgba 206 221 218 235) "OK       return to island"
  atvText (panelX + 12) (panelY + 48) 1 (rgba 206 221 218 235) "NETFLIX  respawn"
  atvText (panelX + 12) (panelY + 60) 1 (rgba 206 221 218 235) "PRIME    toggle flight"
  atvText (panelX + 12) (panelY + 72) 1 (rgba 206 221 218 235) "BACK     save and leave"
  atvText (panelX + 12) (panelY + 90) 1 (rgba 149 176 178 210) "Your world saves itself."

def drawLoading (headline detail : String) : IO Unit := do
  atvBeginFrame
  drawSky 0
  atvRect 0 0 screenW screenH (rgba 10 18 24 128)
  atvText 18 (screenH / 2 - 16) 2 (rgba 255 238 202 255) headline
  atvText 20 (screenH / 2 + 6) 1 (rgba 194 215 216 230) detail
  atvRect 20 (screenH / 2 + 18) 70 1 (rgba 224 160 86 225)
  let _ ← atvPresent

end Axiom
