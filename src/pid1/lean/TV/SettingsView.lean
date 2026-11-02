prelude
import TV.MenuView.Types
import TV.Apps
set_option compiler.extract_closed false

/-! The Settings UI owns only typed digits and a view of enabled app bits.
It cannot read a stored PIN or change policy; those are TV.Core operations. -/
namespace TV.SettingsView
open TV.View

inductive Purpose where
  | enable (app : UInt32)
  | oldPin
  | newPin (typed : UInt32)
  | confirm (typed fresh : UInt32)
  deriving BEq, Repr, Inhabited

inductive Page where
  | root | apps | proofs | wifi
  | pin (purpose : Purpose) (digits : Array Nat) (position : Nat)
  | password (network : Network) (value : String) (cursor : Nat)
  deriving BEq, Repr, Inhabited

structure State where
  page : Page := .root
  selected : Nat := 0
  enabled : UInt32 := 0
  available : Bool := false
  wifi : Wifi := {}
  notice : String := "Settings storage unavailable"
  awaitingPolicy : Bool := false
  deriving BEq, Repr, Inhabited

def apps : Array (String × UInt32) :=
  #[("YouTube",1),("Doom",2),("Slides",3),("Infinite RPG",5),("City flight",6),("Axiom",7),("Snake",8)]

def chars : Array String := #["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "-", "_", ".", "!", "@", "#", "$", "%", "&", "*", "+", "?", " "]

def initial (enabled : UInt32) (available : Bool) (wifi : Wifi) : State :=
  { enabled, available, wifi, notice := if available then "" else "Settings storage unavailable" }

def pinPage (purpose : Purpose) : Page := .pin purpose #[0,0,0,0] 0

def pinValue (digits : Array Nat) : UInt32 :=
  (digits.foldl (fun n d => n * 10 + d % 10) 0).toUInt32

def setEnabled (s : State) (enabled : UInt32) : State :=
  { s with enabled, notice := if s.awaitingPolicy then "Saved" else s.notice, awaitingPolicy := false }

def setAvailable (s : State) (available : Bool) : State :=
  if available then { s with available } else
    { s with available, page := .root, selected := 0, awaitingPolicy := false, notice := "Settings storage unavailable" }

def tell (s : State) (kind : UInt8) : State :=
  { s with awaitingPolicy := false, notice := if kind == 0 then "Incorrect PIN"
      else if kind == 4 then "Settings could not be saved" else "Operation was not completed" }

def setWifi (s : State) (wifi : Wifi) : State :=
  { s with wifi, selected := if s.page == .wifi then min s.selected (wifi.networks.size + 1) else s.selected }

structure Result where
  state : State
  close : Bool := false
  effects : List Effect := []
  deriving Repr, Inhabited

def emit (s : State) (kind app typed fresh : UInt32) : Result :=
  ⟨{ s with awaitingPolicy := true }, false, [.choice (choiceWord kind app typed fresh)]⟩

/-- One key in the Settings UI. Releases are filtered by MenuView. -/
def step (s : State) (key : Key) (repeated : Bool := false) : Result := Id.run do
  if repeated && !directional key then return ⟨s, false, []⟩
  if key == .home then return ⟨s, true, []⟩
  if key == .back then
    let page ← match s.page with
      | .root => return ⟨s, true, []⟩
      | .password .. => pure Page.wifi
      | .pin (.enable _) .. => pure Page.apps
      | _ => pure Page.root
    return ⟨{ s with page, selected := 0, notice := if s.available then "" else s.notice }, false, []⟩
  match s.page with
  | .pin purpose digits position =>
    let position := min position 3
    let mut digits := if digits.size == 4 then digits else #[0,0,0,0]
    let mut position := position
    match key with
    | .digit d => digits := digits.set! position (d % 10); position := min (position + 1) 3
    | .up => digits := digits.set! position ((digits[position]! + 1) % 10)
    | .down => digits := digits.set! position ((digits[position]! + 9) % 10)
    | .left => position := position - 1
    | .right => position := min (position + 1) 3
    | .ok =>
      if s.available then
        let value := pinValue digits
        match purpose with
        | .enable app => return emit { s with page := .apps } 3 app value 0
        | .oldPin => return ⟨{ s with page := pinPage (.newPin value), notice := "" }, false, []⟩
        | .newPin typed => return ⟨{ s with page := pinPage (.confirm typed value), notice := "" }, false, []⟩
        | .confirm typed fresh =>
          if value == fresh then return emit { s with page := .root } 1 0 typed fresh
          else return ⟨{ s with page := pinPage (.newPin typed), notice := "PINs did not match" }, false, []⟩
    | _ => pure ()
    return ⟨{ s with page := .pin purpose digits position }, false, []⟩
  | .password network value cursor =>
    let mut cursor := min cursor (chars.size - 1)
    let mut value := value
    match key with
    | .left => cursor := cursor - 1
    | .right => cursor := min (cursor + 1) (chars.size - 1)
    | .up => cursor := cursor - 16
    | .down => cursor := min (cursor + 16) (chars.size - 1)
    | .ok => if value.length < 63 then value := value ++ chars[cursor]!
    | .digit d => if value.length < 63 then value := value ++ toString (d % 10)
    | .green => value := clipped value (value.length - 1)
    | .red => return ⟨{ s with page := .wifi, selected := 0 }, false, [.connect network value]⟩
    | _ => pure ()
    return ⟨{ s with page := .password network value cursor }, false, []⟩
  | page =>
    let count := match page with
      | .root => 4 | .apps => apps.size | .wifi => 2 + s.wifi.networks.size
      | .proofs => apps.size + 9 | _ => 0
    match key with
    | .up => return ⟨{ s with selected := s.selected - 1 }, false, []⟩
    | .down => return ⟨{ s with selected := min (s.selected + 1) (count - 1) }, false, []⟩
    | .ok =>
      match page with
      | .root =>
        let next := match s.selected with
          | 0 => Page.apps | 1 => if s.available then pinPage .oldPin else .root
          | 2 => .wifi | _ => .proofs
        let notice := if s.selected == 1 && !s.available then "Parental settings are unavailable"
          else if s.available then "" else s.notice
        return ⟨{ s with page := next, selected := 0, notice }, false, []⟩
      | .apps =>
        if s.available then
          let app := (apps[s.selected]?.getD ("",0)).2
          if s.enabled &&& (1 <<< (app - 1)) != 0 then return emit s 2 app 0 0
          else return ⟨{ s with page := pinPage (.enable app), notice := "" }, false, []⟩
      | .wifi =>
        if s.selected == 0 then return ⟨s, false, [.scan]⟩
        if s.selected == 1 then return ⟨s, false, [.disconnect]⟩
        if let some network := s.wifi.networks[s.selected - 2]? then
          if network.secure then return ⟨{ s with page := .password network "" 0 }, false, []⟩
          else return ⟨s, false, [.connect network ""]⟩
      | _ => pure ()
    | _ => pure ()
    return ⟨s, false, []⟩


def proofLines : List String :=
  ["Policy: TV/Core.lean (compiled into PID 1)",
   "Wrong PIN cannot change the PIN (proved)",
   "Wrong PIN cannot re-enable an app (proved)",
   "Invalid new PIN cannot replace the PIN (proved)",
   "The correct PIN can change it (proved)",
   "Disabling an app removes its access (proved)"] ++
  (apps.toList.map fun (name, app) => s!"{name}: UID {TV.Apps.uid app}  network {match TV.Apps.network app 7 with | 1 => "home" | 2 => "via PID 1" | 3 => "via PID 1 + home" | _ => "none"}  threads {TV.Apps.threads app}") ++
  ["Storage, input, display and Linux remain trusted",
   "Home belongs to the supervisor; apps cannot receive it",
   "Menus never receive the stored parental PIN"]

def lines (s : State) : String × List String :=
  match s.page with
  | .root => ("Owned by PID 1", ["App access","Change parental PIN","Wi-Fi","System and proofs"])
  | .apps => ("Disable freely; re-enable with the parental PIN", apps.toList.map fun (name, app) =>
      name ++ ": " ++ if !s.available then "UNAVAILABLE"
      else if s.enabled &&& (1 <<< (app - 1)) != 0 then "ENABLED" else "DISABLED")
  | .wifi => ("WPA2 personal and open networks", ["Scan for networks","Disconnect Wi-Fi"] ++
      s.wifi.networks.toList.map fun n => n.ssid ++ "  " ++ n.signal ++ " dBm  " ++ if n.secure then "password" else "open")
  | .proofs => ("Checked policy; trusted Linux and native I/O", proofLines)
  | .pin purpose digits position =>
      let title := match purpose with
        | .enable _ => "Parental PIN to re-enable" | .oldPin => "Current parental PIN"
        | .newPin _ => "Choose a new four-digit PIN" | .confirm .. => "Confirm the new PIN"
      let text := String.join ((List.range 4).map fun i =>
        if i == position then "[" ++ toString (digits[i]?.getD 0) ++ "]" else " * ")
      (title, [text,"Digits or arrows to edit; OK submits"])
  | .password .. => ("",[])

def draw (s : State) : List Draw := Id.run do
  let mut out : List Draw := [.gradient 0x161a2c 0x06060c, .text 60 35 4 white "Settings"]
  match s.page with
  | .password network value cursor =>
      out := out ++ [.text 60 110 2 grey ("Join " ++ network.ssid),
        .text 60 155 2 white s!"Password: {String.join (List.replicate value.length "*")} ({value.length} characters)"]
      for i in List.range chars.size do
        let x := 64 + (i % 16) * 65
        let y := 230 + (i / 16) * 55
        if i == cursor then out := out ++ [.rect (Int.ofNat x - 8) (Int.ofNat y - 8) 40 40 accent]
        let char := chars[i]!
        out := out ++ [.text (Int.ofNat x) (Int.ofNat y) 3 white (if char == " " then "_" else char)]
      return out ++ [.text 60 575 2 grey "OK adds - Green deletes - Red connects - Back cancels"]
  | _ => pure ()
  let (title, textLines) := lines s
  out := out ++ [.text 60 110 2 grey title]
  let start := s.selected - 8
  for (line, i) in textLines.zipIdx do
    if i < start || i >= start + 10 then continue
    let y := 170 + (i-start)*40
    if s.page != .proofs && !(match s.page with | .pin .. => true | _ => false) && i == s.selected then
      out := out ++ [.rect 48 (Int.ofNat y - 7) 1180 34 0x234a53]
    out := out ++ [.text 60 (Int.ofNat y) 2 white (clipped line 72)]
  if s.page == .wifi then out := out ++ [.text 60 575 2 grey (clipped s.wifi.connection 72)]
  let message := if s.page == .wifi then s.wifi.status else s.notice
  return out ++ [.text 60 610 2 accent (clipped message 72),
    .text 60 665 2 grey "Arrows select - OK opens - Back returns - Home exits"]

end TV.SettingsView
