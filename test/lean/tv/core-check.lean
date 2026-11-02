import TV.Core

open TV.Core

def view : Output → String
  | .show .home => "home"
  | .show (.app a) => s!"show:{a.id}"
  | .toMenu k held => s!"menu:{k.kind}:{k.code}:{held}"
  | .forward a code held => s!"key:{a.id}:{code}:{if held then 2 else 1}"
  | .release a code => s!"key:{a.id}:{code}:0"
  | .start a l g => s!"start:{a.id}:{l}:{g.bits}"
  | .signal a .term => s!"term:{a.id}"
  | .signal a .kill => s!"kill:{a.id}"
  | .reply a l _ granted => s!"reply:{a.id}:{l}:{granted}"
  | .save p => s!"save:{p.pin.val}:{enabledBits p.enabled}"
  | .tell n => s!"notice:{n.kind}:{n.app}"
  | .network op up => s!"network:{op}:{up}"
  | .daemon .wifi go => s!"wifi:{go}"
  | .daemon .dhcp go => s!"dhcp:{go}"
  | .saveWifi => "savewifi"
  | .reboot => "reboot"
  | .powerOff => "power"

def ensure (ok : Bool) (label : String) : IO Unit := do
  unless ok do throw (IO.userError label)

def outputs (s : State) (i : Input) : List String := ((step s i).2).map view

/-- Started online and the network confirmed. -/
def ready : State := { State.boot Policy.initial with net := .online }

def front (a : App) : State := { ready with screen := .app a }

def main : IO Unit := do
  let boot := State.boot Policy.initial
  ensure ((bootOutputs Policy.initial).map view == ["network:0:true"]) "online boot brings the link up"
  ensure (outputs boot (.choose (.launch .doom)) == ["notice:8:0"]) "no launch while the network is changing"
  ensure (outputs boot (.netDone 0 true true) == ["wifi:true", "dhcp:true", "notice:5:0"]) "confirmed link starts Wi-Fi"
  ensure ((step boot (.netDone 0 true true)).1.net == .online) "confirmed link is online"
  ensure (outputs boot (.netDone 1 true true) == []) "report for another change is stale"
  ensure (outputs ready (.choose (.launch .doom)) == ["start:2:1:6", "show:2"]) "launch orders start before app presentation"
  let launched := (step ready (.choose (.launch .youtube))).1
  ensure (outputs launched (.ask .youtube 1 (.connect "www.youtube.com" 443)) == ["reply:1:1:true"]) "listed host granted"
  ensure (outputs launched (.ask .youtube 1 (.connect "evil.example" 443)) == ["reply:1:1:false"]) "unlisted host refused"
  ensure (outputs launched (.ask .youtube 0 (.connect "www.youtube.com" 443)) == ["reply:1:0:false"]) "old launch refused"
  ensure (outputs launched (.ask .youtube 1 .teeSession) == ["reply:1:1:false"]) "no app gets the TEE"
  let playing := front .doom
  ensure (outputs playing (.key (.other 103) false) == ["key:2:103:1"]) "press reaches foreground"
  ensure (outputs playing (.key (.other 103) true) == ["key:2:103:2"]) "repeat reaches foreground"
  ensure (outputs playing (.release 103) == ["key:2:103:0"]) "release reaches foreground"
  ensure (outputs boot (.release 103) == []) "release ignored by menu"
  let (closing, actions) := step { playing with clock := 100 } (.key .home false)
  ensure (actions.map view == ["home", "term:2"]) "Home revokes presentation before TERM"
  ensure (outputs closing (.release 103) == []) "no releases forwarded after Home"
  let repeated := (step closing (.key .home true)).1
  ensure ((repeated.stopping.map (·.deadline)) == some (100 + grace)) "repeated Home preserves grace deadline"
  ensure (outputs closing (.choose (.launch .youtube)) == ["notice:2:2"]) "another launch blocked while closing"
  ensure (outputs closing (.tick (100 + grace)) == []) "deadline is strict"
  let (killed, killActions) := step closing (.tick (101 + grace))
  ensure (killActions.map view == ["kill:2"]) "first late tick sends KILL"
  ensure (outputs killed (.tick (102 + grace)) == []) "KILL sent once"
  ensure ((step killed (.exited .youtube 0)).1.stopping.isSome) "unrelated app exit preserves stopping child"
  ensure ((step killed (.exited .doom 7)).1.stopping.isSome) "exit of an old launch preserves stopping child"
  ensure ((step killed (.exited .doom 0)).1.stopping.isNone) "matching app exit permits launch"
  let (stuck, stuckActions) := step killed (.tick (102 + grace + stuckGrace))
  ensure (stuckActions.map view == ["notice:4:2"]) "KILL that fails asks about restarting"
  ensure (outputs stuck (.choose .restart) == ["reboot"]) "restart offered once stuck"
  ensure (outputs ready (.choose .restart) == []) "no restart unless offered"

  for a in App.all do
    for code in [102, 172, 116, 113, 114, 115, 0x100000066] do
      ensure (outputs (front a) (.key (.other code) false) == []) "reserved numeric press blocked"
      ensure (outputs (front a) (.key (.other code) true) == []) "reserved numeric repeat blocked"
      ensure (outputs (front a) (.release code) == []) "reserved numeric release blocked"
    ensure (outputs (front a) (.key .power false) == ["power"]) "power is global"
    ensure (outputs (front a) (.key .power true) == ["power"]) "power repeat is global"

  match decodeInput (((103 : UInt64) <<< 32) ||| 4) 7 with
  | some (.release 103) => pure ()
  | _ => throw (IO.userError "release wire decoding")
  match decodeInput (((103 : UInt64) <<< 32) ||| ((1 : UInt64) <<< 16) ||| ((2 : UInt64) <<< 8) ||| 1) 7 with
  | some (.key (.other 103) true) => pure ()
  | _ => throw (IO.userError "repeat wire decoding")
  ensure ((decodeInput (((9 : UInt64) <<< 16) ||| 2) 0).isNone) "invalid app ID rejected"
  ensure ((decodePolicy (((31 : UInt64) <<< 16) ||| 10000)).enabled .doom == false) "invalid stored PIN fails closed"
  ensure ((decodePolicy ((1 : UInt64) <<< 32)).enabled .youtube == false) "storage error fails closed"
  ensure (outputs boot (.choose (.changePin 1234 9876)) == ["notice:0:0"]) "wrong PIN never saves"
  ensure ((step boot (.choose (.disable .doom))).1.policy.enabled .doom == false) "disable applies"
  let (offline, offActions) := step ready (.choose .goOffline)
  ensure (offActions.map view == ["save:0:247", "wifi:false", "dhcp:false", "network:1:false", "notice:6:0"]) "going offline takes everything down"
  ensure (outputs offline (.choose (.goOnline 1234)) == ["notice:0:0"]) "going online needs the PIN"
  ensure (outputs offline (.choose (.joinWifi 0)) == ["savewifi"]) "offline Wi-Fi save starts no daemon"
  let offlineBoot := State.boot { Policy.initial with offline := true }
  ensure (offlineBoot.net == .offline && (bootOutputs { Policy.initial with offline := true }).isEmpty) "offline boot leaves the link down"
  ensure (outputs { offlineBoot with net := .goingOnline 0 } (.netDone 0 true true) == []) "offline policy ignores link-up reports"

  -- The effect interpreter must stop on a failed write. It must neither
  -- consume a later launch choice nor carry on using the candidate policy.
  let readCount ← IO.mkRef (0 : Nat)
  let trace ← IO.mkRef ([] : List String)
  let next : IO Input := do
    readCount.modify (· + 1)
    return .choose (.changePin 0 1234)
  let perform : Output → IO Unit := fun action => do
    trace.modify (· ++ [view action])
    match action with
    | .save _ => throw (IO.userError "injected storage failure")
    | _ => pure ()
  let failed ← try
    loop next perform boot
    pure false
  catch _ => pure true
  ensure failed "failed save propagates"
  ensure ((← readCount.get) == 1 && (← trace.get) == ["save:1234:247"]) "failed save prevents next input"
  let nativeFailed ← try
    Native.ioError 5
    pure false
  catch _ => pure true
  ensure nativeFailed "native error constructor returns failed IO"
  IO.println "PASS: Core lifecycle, launches, requests, network changes, reserved keys, storage failure and scalar decoding"
