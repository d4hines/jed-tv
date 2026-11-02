prelude
import TV.SettingsView
set_option compiler.extract_closed false

/-! Launcher and Settings presentation. All navigation, choices and draw-command
construction live here; native code only renders commands and carries WiFi I/O. -/
namespace TV.MenuView
open TV.View

inductive Action where
  | app (id : UInt32) (lean demo : Bool)
  | settings | about | powerOff
  deriving BEq, Repr, Inhabited

structure Tile where
  title : String
  subtitle : String
  art : Option String
  color : UInt32
  action : Action
  deriving BEq, Repr, Inhabited

structure Row where
  title : String
  tiles : Array Tile
  deriving BEq, Repr, Inhabited

def appTiles : Array Tile := #[
  ⟨"YouTube","Search YouTube and browse suggested videos",some "youtube",0xbe1e28,.app 1 false false⟩,
  ⟨"Slides","Our hackathon deck, built from Markdown - arrows move, Back exits",some "slides",0x285aa0,.app 3 false false⟩,
  ⟨"Doom","Original GPL engine + Freedoom - Ctrl fire, Space use",some "doom",0x781414,.app 2 false false⟩,
  ⟨"Infinite RPG","Offline Lean adventure - arrows choose, OK acts",some "rpg",0x442662,.app 5 false false⟩,
  ⟨"City flight","Cessna over a city with a Lean JSBSim plant and safety filter, all on this TV",some "flight",0x3c5a8c,.app 6 false false⟩,
  ⟨"YouTube (Lean)","Lean YouTube client - search, suggestions and playback",some "youtube",0xbe1e28,.app 1 true false⟩,
  ⟨"Doom (Lean)","Lean Doom engine + Freedoom - compare with Doom",some "doom",0x781414,.app 2 true false⟩,
  ⟨"Axiom","A voxel island written in Lean 4 (petergpt/lean-3d-game-axiom), running on this TV",none,0x467850,.app 7 false false⟩,
  ⟨"Snake","Snake on a torus: the ESP32 prototype's RV32I game, ported - Left/Right turn, OK restarts",none,0x286e3c,.app 8 false false⟩]

def systemTiles : Array Tile := #[
  ⟨"Settings","Parental PIN, app access, Wi-Fi and policy proofs",none,0x23645a,.settings⟩,
  ⟨"About","What this TV is made of",none,0x1e4678,.about⟩,
  ⟨"Power off","Shut the TV down",none,0x464650,.powerOff⟩]

def catalog (installed : UInt32) (demo : Bool) : Array Row := Id.run do
  let mut tiles := #[]
  for i in List.range appTiles.size do
    if installed &&& (1 <<< i.toUInt32) != 0 then tiles := tiles.push appTiles[i]!
  let mut rows := #[]
  if demo && installed &&& 1 != 0 then
    rows := rows.push ⟨"Demo", #[⟨"Demo video","Generated picture and tone - offline, 16 seconds",some "demo-video",0x282830,.app 1 false true⟩]⟩
  if !tiles.isEmpty then rows := rows.push ⟨"Apps",tiles⟩
  return rows.push ⟨"System",systemTiles⟩

inductive View where
  | home | settings (state : SettingsView.State) | about
  deriving BEq, Repr, Inhabited

structure State where
  rows : Array Row
  row : Nat := 0
  columns : Array Nat
  view : View := .home
  active : Bool := false
  enabled : UInt32 := 0
  available : Bool := false
  launch : Option Action := none
  status : String := ""
  clock : String := "00:00"
  width : Nat := 1280
  height : Nat := 720
  wifi : Wifi := {}
  about : List String := ["PID 1 decisions and menus: Lean", "Native effects: display, audio, input and storage", "Home and parental policy belong to PID 1"]
  deriving BEq, Repr, Inhabited

def initial (installed : UInt32) (demo : Bool) (width : Nat := 1280) (height : Nat := 720) : State :=
  let rows := catalog installed demo
  { rows, columns := rows.map (fun _ => 0), width, height }

def setEnabled (s : State) (enabled : UInt32) : State :=
  { s with enabled, view := match s.view with
      | .settings page => .settings (SettingsView.setEnabled page enabled) | page => page }

def setAvailable (s : State) (available : Bool) : State :=
  { s with available, view := match s.view with
      | .settings page => .settings (SettingsView.setAvailable page available) | page => page }

def setWifi (s : State) (wifi : Wifi) : State :=
  { s with wifi, view := match s.view with
      | .settings page => .settings (SettingsView.setWifi page wifi) | page => page }

def appName (id : UInt32) : String :=
  ((SettingsView.apps.find? (fun pair => pair.2 == id)).map (·.1)).getD "App"

def tell (s : State) (kind : UInt8) (app : UInt32) : State :=
  let name := appName app
  let message := match kind.toNat with
    | 0 => "Incorrect PIN" | 1 => name ++ " is disabled in parental settings"
    | 2 => name ++ " is still closing" | 3 => name ++ " exited"
    | 4 => "Settings could not be saved" | _ => "Operation failed"
  { s with status := message, launch := if kind == 1 || kind == 2 then none else s.launch, view := match s.view with
      | .settings page => if kind == 0 || kind == 4 then .settings (SettingsView.tell page kind) else s.view
      | _ => s.view }

structure Result where
  state : State
  effects : List Effect := []
  deriving Repr, Inhabited

def select (s : State) (tile : Tile) : Result :=
  match tile.action with
  | .app id lean demo => ⟨{ s with launch := some (.app id lean demo) }, [.choice (choiceWord 0 id 0 0)]⟩
  | .settings => ⟨{ s with view := .settings (SettingsView.initial s.enabled s.available s.wifi) }, []⟩
  | .about => ⟨{ s with view := .about }, []⟩
  | .powerOff => ⟨s, [.choice (choiceWord 4 0 0 0)]⟩

def move (position count : Nat) (positive : Bool) : Nat :=
  min (if positive then position + 1 else position - 1) (count - 1)

def handle (s : State) (kind code value : UInt32) : Result := Id.run do
  if !s.active || value == 0 then return ⟨s, []⟩
  let key := keyOfCode kind code
  let repeated := value == 2
  if key == .home then return ⟨{ s with view := .home, row := 0, columns := s.columns.map (fun _ => 0), status := "", launch := none }, []⟩
  if key == .power then return ⟨s, [.choice (choiceWord 4 0 0 0)]⟩
  match s.view with
  | .settings page =>
    let result := SettingsView.step page key repeated
    return ⟨{ s with view := if result.close then .home else .settings result.state },result.effects⟩
  | .about => return ⟨{ s with view := if !repeated && (key == .back || key == .ok) then .home else .about },[]⟩
  | .home =>
    let row := min s.row (s.rows.size - 1)
    let tiles := (s.rows[row]?.getD default).tiles
    let col := min (s.columns[row]?.getD 0) (tiles.size - 1)
    if key == .up || key == .down then
      return ⟨{ s with row := move row s.rows.size (key == .down) }, []⟩
    if key == .left || key == .right then
      return ⟨{ s with columns := s.columns.set! row (move col tiles.size (key == .right)) }, []⟩
    if repeated then return ⟨s, []⟩
    if key == .ok then
      if let some tile := tiles[col]? then return select s tile
    if key == .youtube then
      for row in s.rows do
        if let some tile := row.tiles.find? (fun tile => tile.action == .app 1 false false) then return select s tile
    return ⟨s, []⟩

/-- App id plus variant bits: bit8 selects Lean, bit9 selects offline demo. -/
def takeLaunch (s : State) (app : UInt32) : State × UInt32 :=
  match s.launch with
  | some (.app id lean demo) =>
      if app == id then ({s with launch := none},id ||| (if lean then 256 else 0) ||| (if demo then 512 else 0))
      else (s,0)
  | _ => (s,0)

def twoDigits (n : Nat) : String := (if n < 10 then "0" else "") ++ toString n

def setClock (s : State) (unix : UInt64) (offsetMinutes : Int32) : State :=
  let minutes := ((Int.ofNat (unix.toNat / 60) + offsetMinutes.toInt) % 1440).toNat
  { s with clock := twoDigits (minutes / 60) ++ ":" ++ twoDigits (minutes % 60) }

def drawHome (s : State) : List Draw := Id.run do
  let mut out : List Draw := [.gradient 0x161a2c 0x06060c]
  for r in List.range s.rows.size do
    let row := s.rows[r]!
    let y := 150 + (Int.ofNat r - Int.ofNat s.row) * 300
    if y < -300 || y > Int.ofNat s.height then continue
    let focusedRow := r == s.row
    out := out ++ [.text 72 y 3 (if focusedRow then white else grey) row.title]
    let first := if focusedRow then (s.columns[r]?.getD 0) - 3 else 0
    for c in List.range row.tiles.size do
      if c < first then continue
      let x := 72 + (c-first)*288
      if x > s.width then continue
      let tile := row.tiles[c]!
      let focused := focusedRow && c == (s.columns[r]?.getD 0)
      let width := if focused then 288 else 256
      let height := if focused then 162 else 144
      let x := Int.ofNat x - (if focused then 16 else 0)
      let y := y + 40 - (if focused then 9 else 0)
      out := out ++ [.rect x y width height tile.color]
      if let some art := tile.art then
        let label := clipped tile.title ((width - 16) / 16)
        out := out ++ [.art x y width height art,
          .rect x (y + Int.ofNat height - 34) width 34 0x10131e,
          .text (x + (Int.ofNat width - Int.ofNat (label.length*16))/2)
            (y + Int.ofNat height - 25) 2 white label]
      else
        let label := clipped tile.title (width / 24)
        out := out ++ [.text (x + (Int.ofNat width - Int.ofNat (label.length*24))/2)
          (y + Int.ofNat height/2-12) 3 white label]
      if focused then out := out ++ [.frame x y width height 5 white]
      else if !focusedRow then out := out ++ [.dim x y width height]
    if focusedRow then
      if let some tile := row.tiles[s.columns[r]?.getD 0]? then
        out := out ++ [.text 72 (y+40+162+6) 3 white tile.title,
          .text 72 (y+40+162+40) 2 grey tile.subtitle]
  let width := s.width
  let height := Int.ofNat s.height
  out := out ++ [.rect 0 0 width 96 0x0e101c, .rect 0 0 10 96 0xe62832,
    .text 42 30 4 white "dande-tv",
    .text (Int.ofNat width-72-Int.ofNat (s.clock.length*32)) 30 4 white s.clock,
    .rect 0 (height-76) width 76 0x0e101c,
    .text 42 (height-60) 2 grey "Arrows move - OK opens - Home returns"]
  if !s.status.isEmpty then
    let maxChars := (s.width - 84)/16
    let status := if s.status.length > maxChars && maxChars >= 3 then clipped s.status (maxChars-3) ++ "..." else s.status
    out := out ++ [.text 42 (height-30) 2 grey status]
  return out

def draw (s : State) : List Draw :=
  if !s.active then [] else
  match s.view with
  | .home => drawHome s
  | .settings page => SettingsView.draw page
  | .about => [.gradient 0x161a2c 0x06060c, .text 72 60 4 white "About this TV"] ++
      (s.about.zipIdx.map fun (line,i) => .text 72 (150+Int.ofNat i*36) 2 (if line.startsWith " " then grey else white) line) ++
      [.text 72 660 2 grey "Press Back to return"]

end TV.MenuView
