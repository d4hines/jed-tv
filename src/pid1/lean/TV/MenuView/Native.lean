prelude
import TV.MenuView
set_option compiler.extract_closed false

/-! The menu effect interpreter. Native code supplies pixels and Wi-Fi transport;
this module owns state, transitions, draw order, and requests to the policy core. -/
namespace TV.MenuView.Native
open TV.View

@[extern "tv_menu_paint"] opaque paint (op : UInt32) (x y : Int32)
    (width height a b : UInt32) (text : @& String) : IO Unit
@[extern "tv_menu_present"] opaque present : IO Unit
@[extern "tv_menu_reply"] opaque reply (low high : UInt32) : IO Unit
@[extern "tv_menu_wifi"] opaque wifi (op secure : UInt32)
    (ssid password : @& String) : IO Unit

structure Runtime where
  state : State := initial 0 false
  offset : Int32 := 0
  deriving Inhabited

initialize runtime : IO.Ref Runtime ← IO.mkRef {}

@[export tv_menu_io_unit]
def ioUnit (_ : UInt32) : IO Unit := pure ()

private def emitDraw : Draw → IO Unit
  | .gradient top bottom => paint 0 0 0 0 0 top bottom ""
  | .rect x y w h color => paint 1 x.toInt32 y.toInt32 w.toUInt32 h.toUInt32 color 0 ""
  | .frame x y w h t color => paint 2 x.toInt32 y.toInt32 w.toUInt32 h.toUInt32 color t.toUInt32 ""
  | .dim x y w h => paint 3 x.toInt32 y.toInt32 w.toUInt32 h.toUInt32 0 0 ""
  | .text x y scale color text => paint 4 x.toInt32 y.toInt32 scale.toUInt32 0 color 0 text
  | .art x y w h name => paint 5 x.toInt32 y.toInt32 w.toUInt32 h.toUInt32 0 0 name

private def render : IO Unit := do
  let s := (← runtime.get).state
  if !s.active then return
  for command in draw s do emitDraw command
  present

private def applyEffects (effects : List Effect) : IO Unit := do
  for effect in effects do
    match effect with
    | .choice word => reply word.low word.high
    | .scan => wifi 0 0 "" ""
    | .disconnect => wifi 1 0 "" ""
    | .connect network password => wifi 2 (if network.secure then 1 else 0) network.ssid password

@[export tv_menu_create]
def create (installed demo width height : UInt32) (offset : Int32) : IO Unit := do
  runtime.set ⟨initial installed (demo != 0) width.toNat height.toNat, offset⟩
  return ()

@[export tv_menu_config]
def config (kind value : UInt32) : IO Unit := do
  runtime.modify fun r => {r with state := if kind == 0 then setEnabled r.state value else setAvailable r.state (value != 0)}
  render
  return ()

@[export tv_menu_show]
def setShown (active : UInt32) : IO Unit := do
  runtime.modify fun r => {r with state := {r.state with active := active != 0}}
  render
  if active != 0 then render
  let s := (← runtime.get).state
  reply (s.rows.foldl (fun n (row : Row) => n + row.tiles.size) 0).toUInt32 s.rows.size.toUInt32

@[export tv_menu_key]
def key (kind code value : UInt32) : IO Unit := do
  if value == 0 then return ()
  let r ← runtime.get
  let result := handle r.state kind code value
  runtime.set {r with state := result.state}
  applyEffects result.effects
  render

@[export tv_menu_notice]
def notice (kind app : UInt32) : IO Unit := do
  runtime.modify fun r => {r with state := tell r.state kind.toUInt8 app}
  render
  return ()

@[export tv_menu_tick]
def tick (unix : UInt64) (dirty : UInt32) : IO Unit := do
  let r ← runtime.get
  let state := setClock r.state unix r.offset
  let changed := state.clock != r.state.clock || dirty != 0
  runtime.set {r with state}
  if changed then render
  reply (if changed then 1 else 0) 0

@[export tv_menu_launch]
def launch (app : UInt32) : IO Unit := do
  let r ← runtime.get
  let (state, token) := takeLaunch r.state app
  runtime.set {r with state}
  reply token 0

@[export tv_menu_wifi_begin]
def wifiBegin (status connection : String) : IO Unit := do
  runtime.modify fun r => {r with state := setWifi r.state {status, connection}}
  return ()

@[export tv_menu_wifi_network]
def wifiNetwork (ssid signal : String) (secure : UInt32) : IO Unit := do
  runtime.modify fun r => {r with state := setWifi r.state {r.state.wifi with networks := r.state.wifi.networks.push ⟨ssid, signal, secure != 0⟩}}
  return ()

@[export tv_menu_about_begin]
def aboutBegin : IO Unit := do
  runtime.modify fun r => {r with state := {r.state with about := []}}
  return ()

@[export tv_menu_about_line]
def aboutLine (line : String) : IO Unit := do
  runtime.modify fun r => {r with state := {r.state with about := r.state.about ++ [line]}}
  return ()

end TV.MenuView.Native
