import Yt.App.Model
import AppTest.KeepAlive
open Yt.App

namespace AppTest

def screenName : Screen → String
  | .home => "home" | .search => "search" | .player => "player"

def effectName : Effect → String
  | .search q => "search(" ++ q ++ ")"
  | .thumbnail i id => "thumb(" ++ toString i ++ "," ++ id ++ ")"
  | .check it => "check(" ++ it.id ++ ")"
  | .play it => "play(" ++ it.id ++ ")"
  | .playerKey _ => "playerKey"
  | .stopPlayer => "stopPlayer"
  | .exit => "exit"

def summary (m : Model) : String :=
  let s := m.search
  screenName m.screen ++ " home=" ++ toString m.homeSel ++ " q=\"" ++ s.queryString ++ "\" key=" ++
    toString s.row ++ "," ++ toString s.col ++ " focus=" ++ (if s.focus == .keys then "keys" else "results") ++
    " items=" ++ toString s.items.size ++ s.items.foldl (fun acc it => acc ++ "," ++ it.id) "" ++
    " pending=" ++ toString s.pending.size ++ " hidden=" ++ toString s.hidden ++
    " sel=" ++ toString s.sel ++ " busy=" ++ toString s.busy ++
    " status=\"" ++ s.status ++ "\""

def step (o : ByteArray) (m : Model) (e : Event) (label : String) : ByteArray × Model :=
  let (m, fx) := update m e
  let fxs := fx.foldl (fun acc f => acc ++ " " ++ effectName f) ""
  (o ++ (label ++ ": " ++ summary m ++ " fx:" ++ fxs ++ "\n").toUTF8, m)

def keys (o : ByteArray) (m : Model) (as : List (String × Action)) : ByteArray × Model :=
  as.foldl (fun (o, m) (n, a) => step o m (.key a false) n) (o, m)

@[export zr_test_transcript]
def transcript (k : Nat) : ByteArray := Id.run do
  let m : Model := { homeSel := k }
  let o := ByteArray.empty
  -- home: down past the end, back up, open a suggestion, open search
  let (o, m) := keys o m [("down", .down), ("down", .down), ("down", .down), ("down", .down), ("down", .down),
    ("up", .up), ("ok", .ok), ("up", .up), ("up", .up), ("up", .up), ("up", .up), ("up", .up), ("ok", .ok)]
  -- type "cat 7": c(0,2) a(0,0) t(1,9) space(3,6) digit 7, then Go with an empty grid move
  let (o, m) := keys o m [("right", .right), ("right", .right), ("ok", .ok), ("left", .left), ("left", .left),
    ("left", .left), ("ok", .ok), ("down", .down), ("right", .right), ("right", .right), ("right", .right),
    ("right", .right), ("right", .right), ("right", .right), ("right", .right), ("right", .right),
    ("right", .right), ("right", .right), ("ok", .ok), ("down", .down), ("down", .down), ("left", .left),
    ("left", .left), ("left", .left), ("ok", .ok), ("digit7", .digit 7), ("right", .right), ("right", .right),
    ("right", .right), ("ok", .ok)]
  -- results arrive, then thumbnails; navigate and play; back to keys, delete, leave
  let mut items : Array Item := #[]
  for i in [0:20] do items := items.push { id := "v" ++ toString i, title := "Video " ++ toString i }
  let (o, m) := step o m (.searchDone "cat 7" items) "results"
  -- playability checks come back out of order; refused results stay hidden,
  -- the first playable one takes the focus, the selection stays on its item
  let (o, m) := step o m (.checked "v2" true) "checked-v2"
  let (o, m) := step o m (.checked "v0" false) "checked-v0"
  let (o, m) := keys o m [("right", .right)]
  let (o, m) := step o m (.checked "v1" true) "checked-v1"
  let (o, m) := step o m (.checked "v1" true) "checked-v1-again"
  let (o, m) := step o m (.checked "v99" true) "checked-stale"
  let mut om := (o, m)
  for i in [3:16] do
    om := step om.1 om.2 (.checked ("v" ++ toString i) (i % 4 != 1)) ("checked-v" ++ toString i)
  let (o, m) := om
  let (o, m) := step o m (.thumbDone "v3" (ByteArray.mk #[1, 2, 3])) "thumb"
  let thumbs := m.search.items.foldl (fun n it => if it.thumb.isSome then n + 1 else n) 0
  let o := o ++ ("thumbs=" ++ toString thumbs ++ "\n").toUTF8
  let (o, m) := keys o m [("left", .left), ("right", .right), ("right", .right), ("ok", .ok), ("back", .back),
    ("left", .left), ("ok", .ok), ("back", .back)]
  let (o, m) := step o m (.key .ok true) "repeat-ok"
  let (o, m) := step o m (.key .back false) "back-home"
  -- From Search's results: a play that fails, then one that works; the
  -- error shows until the good play, and re-entering Search shows the
  -- results summary, not an old error.
  let (o, m) := keys o m [("up", .up), ("up", .up), ("up", .up), ("up", .up), ("up", .up), ("ok", .ok)]
  let (o, m) := step o m (.playerDone "Not playable: ERROR Video unavailable") "play-failed"
  let (o, m) := step o m (.playerStarted "Video 0") "play-started"
  let (o, m) := step o m (.playerDone "") "play-ended"
  let (o, m) := step o m (.playerDone "Could not open: TLS: leaf certificate expired") "play-failed-2"
  let (o, m) := step o m (.key .back false) "back-home-2"
  let (o, _) := step o m (.key .ok false) "reenter"
  o ++ KeepAlive.transcript

end AppTest
