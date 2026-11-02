import Axiom

/-!
Axiom: First Light on the TV. The game loop of upstream `Main.lean`, with the
remote (or a keyboard in the emulator) in place of mouse and keyboard, and
the TV platform layer in place of Raylib.

Program arguments: `tvapp.remote=FD` is the launcher's input socket.
-/

open Axiom

inductive Screen where
  | title
  | playing
  | paused
deriving BEq, Inhabited

def log (msg : String) : IO Unit := do
  let t ← atvNowMs
  IO.println s!"[{t / 1000}.{t % 1000}] axiomtv: {msg}"

def coordInWorld (x y z : Int) : Bool :=
  x ≥ 0 && y ≥ 0 && z ≥ 0 && x < Int.ofNat worldWidth &&
    y < Int.ofNat worldHeight && z < Int.ofNat worldDepth

def saveGameSafe (world : World) (player : Player) : IO Bool := do
  try
    saveGame world player
    return true
  catch _ =>
    return false

/-- Turn speed in radians per second while Left/Right are held, and the matching
look speed. `updatePlayer` scales mouse deltas by 0.0022, so these are expressed
as mouse movement. -/
def turnRate : Float32 := 1.7
def lookRate : Float32 := 1.2

def controlsOf (held : Held) (jumpPressed : Bool) (dt : Float32) : Controls := {
  forward := held.forward
  backward := held.backward
  left := held.strafeLeft
  right := held.strafeRight
  jumpDown := held.jump
  jumpPressed := jumpPressed
  descend := held.descend
  sprint := held.sprint
  toggleFly := false
  mouseX := (boolFloat held.turnRight - boolFloat held.turnLeft) * turnRate * dt / 0.0022
  mouseY := (boolFloat held.lookDown - boolFloat held.lookUp) * lookRate * dt / 0.0022
}

def titleCamera (time : Float32) : Vector3 × Vector3 :=
  let angle := time * 0.085
  let center : Vector3 := ⟨56, 17, 53⟩
  (⟨center.x + angle.cos * 48, 34 + (time * 0.21).sin * 2.5, center.z + angle.sin * 48⟩, ⟨62, 15, 57⟩)

def main (args : List String) : IO Unit := do
  let remoteFd := args.findSome? (fun a =>
    if a.startsWith "tvapp.remote=" then (a.drop "tvapp.remote=".length).toNat? else none) |>.getD 0xffffffff
  let opened ← atvOpen (UInt32.ofNat remoteFd) (UInt32.ofNat audioRate)
  if opened != 0 then
    IO.eprintln s!"axiomtv: display: error {opened.toNat}"
    IO.Process.exit 1
  log "display open"
  drawLoading "AXIOM: FIRST LIGHT" "Recovering the island from a proof..."
  let t0 ← atvNowMs
  let loaded ← loadGame?
  let mut hasSave := loaded.isSome
  let mut world := match loaded with
    | some state => state.1
    | none => generateWorld
  let mut player :=
    match loaded with
    | some state =>
      let pitch := if state.2.pitch.abs > 1.0 then -0.14 else state.2.pitch
      let candidate := { state.2 with pitch := pitch, velocity := ⟨0,0,0⟩ }
      if playerCollides world candidate.position then newPlayer world else candidate
    | none => newPlayer world
  let t1 ← atvNowMs
  log s!"world {if hasSave then "loaded" else "generated"} in {t1 - t0} ms"
  drawLoading "BUILDING THE HORIZON" "Lean is meshing every visible face..."
  let mut mesh := buildWorldMesh world
  let t2 ← atvNowMs
  let faces := mesh.sections.foldl (fun n s => n + s.faceCount) 0
  log s!"meshed {faces} faces in {t2 - t1} ms"
  let audio := loadAudioBank
  let mut screen := Screen.title
  let mut held : Held := {}
  let mut showDebug := false
  let mut hit : Option VoxelHit := none
  let mut lastBreak : Float32 := -10
  let mut lastSave : Float32 := 0
  let mut toast := ""
  let mut toastUntil : Float32 := 0
  let mut particles : Array Particle := #[]
  let mut frames : Nat := 0
  let mut lastReport ← atvNowMs
  let mut lastMs ← atvNowMs
  let start := lastMs
  -- Presses not yet handled. A frame can take longer than the gap between two
  -- presses, and a screen change must not swallow the presses after it, so the
  -- title and pause screens take one press per frame and play stops at Back.
  let mut queue : Array Press := #[]

  repeat do
    let nowMs ← atvNowMs
    let time : Float32 := (nowMs - start).toFloat32 / 1000
    let dt : Float32 := min 0.1 ((nowMs - lastMs).toFloat32 / 1000)
    lastMs := nowMs
    let (nextHeld, polled) ← pollKeys held
    held := nextHeld
    queue := queue ++ polled

    match screen with
    | .title =>
      let press := queue[0]?
      queue := queue.extract 1 queue.size
      if press == some .ok then
        playSound audio.ui
        screen := .playing
        toast := "Welcome to First Light"
        toastUntil := time + 2.2
      else if press == some .back then
        log "quit from title"
        let _ ← saveGameSafe world player
        IO.Process.exit 0
    | .playing =>
      let stop := queue.findIdx? (· == .back)
      let presses := match stop with
        | some i => queue.extract 0 i
        | none => queue
      queue := match stop with
        | some i => queue.extract (i + 1) queue.size
        | none => #[]
      if stop.isSome then
        screen := .paused
        held := {}
        if ← saveGameSafe world player then
          hasSave := true
        else
          toast := "Save failed - world remains open"
          toastUntil := time + 3
        lastSave := time
      else
        showDebug := if presses.contains .debug then !showDebug else showDebug
        let jumpPressed := presses.contains .jump
        if jumpPressed && player.grounded then playSound audio.jump
        let controls := { controlsOf held jumpPressed dt with toggleFly := presses.contains .fly }
        player := updatePlayer world controls dt player
        for p in presses do
          if let .digit n := p then
            if n ≥ 1 && n ≤ Block.palette.size then player := { player with selected := n - 1 }
          if p == .nextBlock then
            player := { player with selected := (player.selected + 1) % Block.palette.size }
            toast := "Block: " ++ Block.name Block.palette[player.selected]!
            toastUntil := time + 1.15
        if player.position.y < -8 || player.position.x < -12 || player.position.z < -12 ||
            player.position.x > worldWidth.toFloat32 + 12 || player.position.z > worldDepth.toFloat32 + 12 then
          player := newPlayer world
          toast := "Returned to first light"
          toastUntil := time + 2

        hit := raycastVoxel world player.eye player.lookDirection
        if presses.contains .ok && time - lastBreak > 0.145 then
          if let some h := hit then
            let block := world.getI h.x h.y h.z
            if block != Block.bedrock && block != Block.air && coordInWorld h.x h.y h.z then
              let x := h.x.toNat
              let y := h.y.toNat
              let z := h.z.toNat
              world := world.set x y z Block.air
              mesh := mesh.rebuildAround world x y z
              playSound audio.breakBlock
              particles := particles ++ spawnBlockParticles block x y z (time * 1000).toUInt32.toNat 18
              lastBreak := time
              toast := "Mined " ++ Block.name block
              toastUntil := time + 1.15

        if presses.contains .place then
          if let some h := hit then
            if coordInWorld h.placeX h.placeY h.placeZ &&
                !blockIntersectsPlayer player h.placeX h.placeY h.placeZ then
              let x := h.placeX.toNat
              let y := h.placeY.toNat
              let z := h.placeZ.toNat
              let block := Block.palette[player.selected]!
              world := world.set x y z block
              mesh := mesh.rebuildAround world x y z
              playSound audio.placeBlock
              particles := particles ++ spawnBlockParticles block x y z (time * 1000).toUInt32.toNat 10
              toast := "Placed " ++ Block.name block
              toastUntil := time + 1.15

        if time - lastSave > 18 then
          if ← saveGameSafe world player then
            hasSave := true
          else
            toast := "Autosave failed"
            toastUntil := time + 3
          lastSave := time
    | .paused =>
      let press := queue[0]?
      queue := queue.extract 1 queue.size
      if press == some .ok then
        playSound audio.ui
        screen := .playing
      else if press == some .back then
        if ← saveGameSafe world player then
          hasSave := true
          screen := .title
          hit := none
        else
          toast := "Could not save - staying in world"
          toastUntil := time + 3
      else if press == some (.digit 1) || press == some .jump then      -- 1, NETFLIX
        player := newPlayer world
        toast := "Returned to first light"
        toastUntil := time + 2
      else if press == some (.digit 2) || press == some .place then     -- 2, prime video
        player := { player with flying := !player.flying }
        toast := if player.flying then "Flight on" else "Flight off"
        toastUntil := time + 2

    particles := updateParticles particles dt
    let (camPos, camTarget) := if screen == .title then titleCamera time else (player.eye, player.cameraTarget)
    let fovy : Float32 := if screen == .title then 52 else 61

    atvBeginFrame
    drawSky time
    atvBegin3d camPos.x camPos.y camPos.z camTarget.x camTarget.y camTarget.z fovy
    drawWorld mesh
    drawOcean
    drawClouds time
    drawGlowHalos time
    drawParticles particles
    if screen == .playing then drawSelection hit
    drawWorldTint time
    match screen with
    | .title => drawTitleOverlay hasSave
    | .playing =>
      drawHud player showDebug
      if time < toastUntil then drawToast toast
    | .paused =>
      drawHud player false
      drawPauseOverlay
      if time < toastUntil then drawToast toast
    let presented ← atvPresent
    if presented != 0 then
      log s!"present failed: {presented.toNat}"
      IO.Process.exit 1
    frames := frames + 1
    if nowMs - lastReport ≥ 5000 then
      log s!"{frames * 1000 / (nowMs - lastReport).toNat} fps, {screen |> fun s => match s with | .title => "title" | .playing => "playing" | .paused => "paused"}, {particles.size} particles"
      frames := 0
      lastReport := nowMs
