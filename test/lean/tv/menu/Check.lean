import TV.MenuView
open TV.View

def check (condition : Bool) (label : String) : IO Unit := do
  unless condition do throw (IO.userError label)
  IO.println ("PASS " ++ label)

def enter (s : TV.SettingsView.State) (digits : List Nat) : TV.SettingsView.Result :=
  let s := digits.foldl (fun s digit => (TV.SettingsView.step s (.digit digit)).state) s
  TV.SettingsView.step s .ok

def main : IO Unit := do
  let base := { TV.MenuView.initial 127 false with active := true }
  check ((TV.MenuView.initial 511 false).rows[0]!.tiles.size == 9) "Axiom and Snake follow the original app variants"
  check (base.rows.size == 2 && base.rows[0]!.tiles.size == 7) "seven installed apps and system row"
  let launch := TV.MenuView.handle {base with enabled := 0} 2 28 1
  check (launch.effects == [.choice (choiceWord 0 1 0 0)]) "menu emits launch even when policy mask disabled; Core decides"
  let (_, classic) := TV.MenuView.takeLaunch launch.state 1
  check (classic == 1) "classic YouTube launch token"
  let lean := TV.MenuView.handle {base with columns := #[5,0]} 2 28 1
  let (_, token) := TV.MenuView.takeLaunch lean.state 1
  check (token == 257) "Lean YouTube uses same app identity"
  let released := TV.MenuView.handle base 2 106 0
  check (released.state == base && released.effects.isEmpty) "key release is inert in menus"
  let hidden := TV.MenuView.handle {base with active := false} 2 28 1
  check (hidden.effects.isEmpty) "inactive menu cannot launch"
  let moved := (TV.MenuView.handle base 2 106 1).state
  check (moved.columns[0]! == 1) "right navigation"
  let home := (TV.MenuView.handle {moved with view := .about} 0 0 1).state
  check (home.view == .home && home.columns[0]! == 0) "Home resets page and cursor"
  check ((TV.MenuView.initial 0 false).rows.size == 1) "no installed apps still has system menu"
  check ((TV.MenuView.setClock base 0 (-60)).clock == "23:00") "clock wraps negative UTC offset"
  check (!(TV.MenuView.draw base).isEmpty && (TV.MenuView.draw {base with active := false}).isEmpty) "draw output follows presentation ownership"

  let mut settings := TV.SettingsView.initial 55 true {}
  settings := {settings with page := TV.SettingsView.pinPage .oldPin}
  let old := enter settings [9,9,9,9]
  check (old.effects.isEmpty) "old PIN is collected without local authentication"
  let fresh := enter old.state [1,2,3,4]
  let badConfirm := enter fresh.state [1,2,3,5]
  check (badConfirm.effects.isEmpty && badConfirm.state.notice == "PINs did not match") "mismatched confirmation never emits policy update"
  let again := enter badConfirm.state [2,2,2,2]
  let confirmed := enter again.state [2,2,2,2]
  check (confirmed.effects == [.choice (choiceWord 1 0 9999 2222)]) "complete typed PIN request goes to Core"

  settings := { TV.SettingsView.initial 55 true {} with page := .apps, selected := 1 }
  let disabled := TV.SettingsView.step settings .ok
  check (disabled.effects == [.choice (choiceWord 2 2 0 0)] && disabled.state.enabled == 55) "disable emits request without mutating policy snapshot"
  let ack := TV.SettingsView.setEnabled disabled.state 53
  check (ack.notice == "Saved") "Core acknowledgement updates view"
  let prompt := TV.SettingsView.step ack .ok
  let enabled := enter prompt.state [7,7,7,7]
  check (enabled.effects == [.choice (choiceWord 3 2 7777 0)] && enabled.state.enabled == 53) "enable emits user PIN without trusting it"
  let unavailable := TV.SettingsView.step {settings with available := false} .ok
  check (unavailable.effects.isEmpty) "unreadable policy cannot emit mutation"
  let n : Network := ⟨"example","-30",true⟩
  settings := { settings with page := .password n "secret" 0 }
  let back := TV.SettingsView.step settings .back
  check (back.state.page == .wifi && back.effects.isEmpty) "password Back returns to WiFi without submission"
  let connect := TV.SettingsView.step settings .red
  check (connect.effects == [.connect n "secret"]) "WiFi connect is an explicit effect"
  check (!(TV.SettingsView.draw confirmed.state).isEmpty) "Settings rendering is a pure command list"
  IO.println "PASS: Lean menu and Settings model and rendering"
