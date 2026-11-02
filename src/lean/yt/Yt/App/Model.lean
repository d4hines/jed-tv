/-! The YouTube app's screens as a pure state machine: remote actions and
network results in, new state plus effects out (`update`). No IO here, so it
runs and is tested the same on the host and on the TV. Behaviour follows
the Rust app (src/ytplay/src/hub.rs, browse.rs). -/
namespace Yt.App

/-! ### Remote -/

inductive Action where
  | up | down | left | right | ok | back | exit | playPause
  | digit (n : Nat) | fastForward | rewind
  deriving BEq, Inhabited

/-- The action of a Linux key code (as src/platform/src/remote.rs). Home and
Power never reach apps: PID 1 consumes them. -/
def Action.ofCode (code : UInt32) : Option Action :=
  if code == 103 then some .up
  else if code == 108 then some .down
  else if code == 105 then some .left
  else if code == 106 then some .right
  else if code == 28 || code == 96 || code == 352 || code == 353 then some .ok
  else if code == 158 || code == 1 || code == 14 then some .back
  else if code == 174 then some .exit
  else if code == 164 || code == 57 || code == 207 || code == 119 || code == 201 then some .playPause
  else if code ≥ 2 && code ≤ 10 then some (.digit (code - 1).toNat)
  else if code == 11 then some (.digit 0)
  else if code == 159 then some .fastForward
  else if code == 168 then some .rewind
  else none

/-! ### Configuration -/

structure Config where
  /-- `app` (home screen) or `play` (one video, then exit). -/
  mode : String := "app"
  /-- Base URL: the replay server in QEMU, https://www.youtube.com live. -/
  server : String := ""
  video : String := ""
  /-- `host:port` of the HTTP CONNECT proxy (TV_HTTP_PROXY), if any. -/
  proxy : Option String := none
  /-- Stop a video after this many pictures (0: play to the end; tests). -/
  frames : Nat := 0

/-- The value of `key=value` in `arg`, if `arg` has that key. -/
def argValue (key : String) (arg : String) : Option String :=
  let a := arg.toUTF8
  let p := (key ++ "=").toUTF8
  if a.size ≥ p.size && a.extract 0 p.size == p then String.fromUTF8? (a.extract p.size a.size) else none

/-- The decimal number `s` (digits only; 0 otherwise). Bytes, not `String.toNat?`,
which the runtime does not link. -/
def decimal (s : String) : Nat :=
  s.toUTF8.foldl (fun (acc : Option Nat) c =>
    acc.bind fun n => if 48 ≤ c && c ≤ 57 then some (10 * n + (c - 48).toNat) else none) (some 0) |>.getD 0

def Config.parse (args : Array String) (proxy : Option String) : Config := Id.run do
  let mut c : Config := { proxy }
  for a in args do
    if let some v := argValue "ytplay.mode" a then c := { c with mode := v }
    if let some v := argValue "ytplay.server" a then c := { c with server := v }
    if let some v := argValue "ytplay.video" a then c := { c with video := v, mode := "play" }
    if let some v := argValue "ytplay.frames" a then c := { c with frames := decimal v }
  c

/-! ### Data -/

/-- One search result; `thumb` is an RGB888 picture of `thumbW`×`thumbH`. -/
structure Item where
  id : String
  title : String
  channel : String := ""
  length : String := ""
  thumb : Option ByteArray := none
  /-- Position in the search reply (results show in YouTube's order). -/
  rank : Nat := 0

def thumbW : Nat := 288
def thumbH : Nat := 162

/-- The home screen: Search, then four suggestions (as hub.rs). -/
def suggestions : Array Item := #[
  { id := "PVGeM40dABA", title := "Coffee Run", channel := "Blender Studio", length := "3:05" },
  { id := "aqz-KE-bpKQ", title := "Big Buck Bunny", channel := "Blender Foundation", length := "10:35" },
  { id := "WhWc3b3KhnY", title := "Spring", channel := "Blender Studio", length := "7:44" },
  { id := "_cMxraX_5RE", title := "Sprite Fright", channel := "Blender Studio", length := "10:30" }]

/-! ### Keyboard: 4 rows of 10 cells; row 3 ends with SPACE (two cells), DEL, GO -/

def keyRows : Array String := #["abcdefghij", "klmnopqrst", "uvwxyz0123", "456789"]

inductive Key where
  | char (c : UInt8) | space | del | go
  deriving BEq, Inhabited

def keyAt (r c : Nat) : Key :=
  if r == 3 && c ≥ 6 then (if c ≤ 7 then .space else if c == 8 then .del else .go)
  else .char ((keyRows[r]!).toUTF8[c]!)

/-! ### State -/

inductive Focus where
  | keys | results
  deriving BEq, Inhabited

structure Search where
  /-- ASCII (the keyboard only types ASCII). -/
  query : ByteArray := .empty
  row : Nat := 0
  col : Nat := 0
  focus : Focus := .keys
  /-- The results shown: those that passed the playability check. -/
  items : Array Item := #[]
  /-- Results whose playability check has not come back yet. -/
  pending : Array Item := #[]
  /-- How many results were hidden because they cannot play here. -/
  hidden : Nat := 0
  /-- The query the results are for. -/
  resultsFor : String := ""
  /-- Focus moves to the results with the first one shown, unless a key
  was pressed since GO. -/
  autoFocus : Bool := false
  sel : Nat := 0
  status : String := "Type with the remote; number keys work too"
  /-- What `status` returns to after an error is no longer current: the
  hint, or the last results. -/
  summary : String := "Type with the remote; number keys work too"
  busy : Bool := false

inductive Screen where
  | home | search | player
  deriving BEq, Inhabited

structure Model where
  screen : Screen := .home
  /-- Where the player returns to. -/
  prev : Screen := .home
  /-- 0 is Search, 1.. the suggestions. -/
  homeSel : Nat := 0
  status : String := ""
  search : Search := {}
  /-- "Link with TV code": the code to enter on a phone, and connected phones. -/
  castCode : Option String := none
  castRemotes : Array String := #[]
  /-- The phone link runs (it needs the TV's proxy to reach YouTube). -/
  castAvailable : Bool := true

inductive Effect where
  | search (query : String)
  | thumbnail (index : Nat) (videoId : String)
  /-- Ask YouTube whether a result can play here (the same /player request
  as playing it); the answer comes back as `.checked`. -/
  | check (item : Item)
  | play (item : Item)
  /-- A key for the running player (pause, seek, ...). -/
  | playerKey (a : Action)
  | stopPlayer
  | exit

inductive Event where
  | key (a : Action) (repeated : Bool)
  | searchDone (query : String) (items : Array Item)
  | searchFailed (msg : String)
  | thumbDone (videoId : String) (rgb : ByteArray)
  /-- A result's playability check: `false` only if YouTube refuses it for
  this client or it has no stream this app can decode. -/
  | checked (videoId : String) (playable : Bool)
  /-- The phone link's state changed. -/
  | castStatus (code : Option String) (remotes : Array String)
  /-- A phone asked to play a video. -/
  | castPlay (item : Item)
  /-- Playback started (the player now owns the screen). -/
  | playerStarted (title : String)
  | playerDone (msg : String)
  deriving Inhabited

def Search.queryString (s : Search) : String := (String.fromUTF8? s.query).getD ""

/-- Trim ASCII spaces at both ends. -/
def trimBytes (b : ByteArray) : ByteArray := Id.run do
  let mut i := 0
  let mut j := b.size
  while i < j && b[i]! == 32 do i := i + 1
  while j > i && b[j - 1]! == 32 do j := j - 1
  b.extract i j

def Search.onKey (s : Search) (a : Action) (repeated : Bool) : Search × Array Effect × Bool :=
  -- the Bool: leave the search screen
  match s.focus with
  | .keys =>
    let s := { s with autoFocus := false }
    match a with
    | .up => ({ s with row := s.row - 1 }, #[], false)
    | .down =>
      if s.row < 3 then ({ s with row := s.row + 1 }, #[], false)
      else if !s.items.isEmpty then ({ s with focus := .results }, #[], false)
      else (s, #[], false)
    | .left => ({ s with col := s.col - 1 }, #[], false)
    | .right => ({ s with col := min 9 (s.col + 1) }, #[], false)
    | .digit d => if repeated then (s, #[], false) else ({ s with query := s.query.push (48 + d).toUInt8 }, #[], false)
    | .ok =>
      if repeated then (s, #[], false) else
      match keyAt s.row s.col with
      | .char c => ({ s with query := s.query.push c }, #[], false)
      | .space => ({ s with query := s.query.push 32 }, #[], false)
      | .del => ({ s with query := s.query.extract 0 (s.query.size - 1) }, #[], false)
      | .go =>
        let q := trimBytes s.query
        if q.size == 0 || s.busy then (s, #[], false)
        else
          let qs := (String.fromUTF8? q).getD ""
          ({ s with busy := true, status := "Searching for \"" ++ qs ++ "\"…" }, #[.search qs], false)
    | .back | .exit => if repeated then (s, #[], false) else (s, #[], true)
    | _ => (s, #[], false)
  | .results =>
    match a with
    | .left => ({ s with sel := s.sel - 1 }, #[], false)
    | .right => ({ s with sel := min (s.items.size - 1) (s.sel + 1) }, #[], false)
    | .up | .back => if repeated then (s, #[], false) else ({ s with focus := .keys }, #[], false)
    | .ok | .playPause =>
      if repeated then (s, #[], false) else
      match s.items[s.sel]? with
      | some it => (s, #[.play it], false)
      | none => (s, #[], false)
    | .exit => if repeated then (s, #[], false) else (s, #[], true)
    | _ => (s, #[], false)

/-- The status line for the current results. -/
def Search.resultsStatus (s : Search) : String :=
  let n := s.items.size
  let shown := toString n ++ (if n == 1 then " result" else " results") ++ " for \"" ++ s.resultsFor ++ "\""
  if !s.pending.isEmpty then
    shown ++ ", checking " ++ toString s.pending.size ++ " more…"
  else if s.hidden == 0 then shown
  else shown ++ " (" ++ toString s.hidden ++ " more can't play on this TV)"

/-- The results of a search: at most 32 (a first reply page). None is shown
until its playability check passes: YouTube refuses many videos to this
client (most of the big studio and children's channels), and live streams
cannot play, so often only a few remain. -/
def Search.onResults (s : Search) (q : String) (items : Array Item) : Search × Array Effect :=
  let pending := (items.extract 0 32).mapIdx fun i it => { it with rank := i }
  let s := { s with items := #[], pending, hidden := 0, resultsFor := q, sel := 0, busy := false,
                    focus := .keys, autoFocus := true }
  let status := if pending.isEmpty then "No results for \"" ++ q ++ "\"" else s.resultsStatus
  ({ s with status, summary := status }, pending.map .check)

/-- A check came back: show the result in its place, or count it hidden.
The selected result stays selected. Answers for results no longer pending
(an older search) are ignored. -/
def Search.onChecked (s : Search) (id : String) (playable : Bool) : Search × Array Effect :=
  match s.pending.find? (·.id == id) with
  | none => (s, #[])
  | some it =>
    let s := { s with pending := s.pending.filter (·.id != id) }
    let (s, fx) :=
      if playable then
        let pos := (s.items.findIdx? (·.rank > it.rank)).getD s.items.size
        let sel := if s.focus == .results && pos ≤ s.sel then s.sel + 1 else s.sel
        let s := { s with items := (s.items.extract 0 pos).push it ++ s.items.extract pos s.items.size, sel }
        let s := if s.autoFocus then { s with focus := .results, autoFocus := false } else s
        (s, #[Effect.thumbnail pos it.id])
      else ({ s with hidden := s.hidden + 1 }, #[])
    let status := s.resultsStatus
    ({ s with status, summary := status }, fx)

def update (m : Model) (e : Event) : Model × Array Effect :=
  match e, m.screen with
  | .key a r, .home =>
    match a with
    | .up => ({ m with homeSel := m.homeSel - 1 }, #[])
    | .down => ({ m with homeSel := min suggestions.size (m.homeSel + 1) }, #[])
    | .back | .exit => if r then (m, #[]) else (m, #[.exit])
    | .ok =>
      if r then (m, #[])
      else if m.homeSel == 0 then
        let search := if m.search.busy then m.search else { m.search with status := m.search.summary }
        ({ m with screen := .search, status := "", search }, #[])
      else match suggestions[m.homeSel - 1]? with
        | some it => ({ m with status := "Loading " ++ it.title ++ "…" }, #[.play it])
        | none => (m, #[])
    | _ => (m, #[])
  | .key a r, .search =>
    let (s, fx, leave) := m.search.onKey a r
    if leave then ({ m with search := s, screen := .home }, fx) else ({ m with search := s }, fx)
  | .key a r, .player =>
    match a with
    | .back | .exit => if r then (m, #[]) else (m, #[.stopPlayer])
    | _ => (m, #[.playerKey a])
  | .playerStarted _, s => if s == .player then (m, #[]) else ({ m with screen := .player, prev := s }, #[])
  | .searchDone q items, _ =>
    let (s, fx) := m.search.onResults q items
    ({ m with search := s }, fx)
  | .checked id ok, _ =>
    let (s, fx) := m.search.onChecked id ok
    ({ m with search := s }, fx)
  | .searchFailed msg, _ => ({ m with search := { m.search with busy := false, status := "Search failed: " ++ msg } }, #[])
  | .thumbDone id rgb, _ =>
    let items := m.search.items.map fun it => if it.id == id then { it with thumb := some rgb } else it
    ({ m with search := { m.search with items } }, #[])
  | .castStatus code remotes, _ => ({ m with castCode := code, castRemotes := remotes }, #[])
  | .castPlay it, _ => (m, #[.play it])
  | .playerDone msg, s =>
    let screen := if s == .player then m.prev else s
    -- an error stays until the next play or search; a good play restores the summary
    let status := if msg.isEmpty && !m.search.busy then m.search.summary else if msg.isEmpty then m.search.status else msg
    ({ m with screen, status := msg, search := { m.search with status } }, #[])

end Yt.App
