import Yt.Lounge.Session
open Yt.Lounge Yt.Fmt

/-! Host-only live driver (not part of the transcript or the TV build): runs
`Yt.Lounge.Session` with a simulated player, talking a line protocol on
stdin/stdout to `live.py`, which performs the HTTPS requests against real
YouTube. Everything lives in this one root module so its C (which has
`main`) is skipped by the 32-bit ARM transcript build.

In (one line per event, `<nowMs>` first):
  S <b64 entropy> <screenId|-> <deviceId> <b64 name>   start (first line)
  R <id> <status> <b64 body|->                          response (status 0: failed)
  C <id> <b64 bytes>                                     long-poll body bytes
  E <id> <status|->                                      long poll ended
  T                                                      tick
  D <code>                                               a DIAL pairing code
  Q                                                      stop
Out: `REQ <id> <tag> <method> <stream 0|1> <url> <hex headers> <hex body|->`,
`CMD <text>`, `CODE <12 digits>`, `ID <screenId>`, `REMOTES <names>`,
`ERR <text>`, and `OK` after each event. -/

def b64 (s : String) : ByteArray := if s == "-" then ByteArray.empty else (base64Decode s.toUTF8).getD ByteArray.empty

structure St where
  s : Session
  p : PlayerState := {}
  loadAt : UInt64 := 0
  code : Option String := none
  screenId : Option String := none
  remotes : Array String := #[]
  err : String := ""
  done : Bool := false

def apply (p : PlayerState) (c : Command) : PlayerState :=
  match c with
  | .playVideo v t _ => { p with videoId := some v, state := .buffering, currentTimeMs := t, durationMs := 0 }
  | .play => { p with state := .playing }
  | .pause => { p with state := .paused }
  | .seek ms => { p with currentTimeMs := ms }
  | .stop => { p with state := .unstarted, currentTimeMs := 0 }
  | .setVolume v m => { p with volume := v, muted := m }
  | _ => p

def emitReqs (o : IO.FS.Stream) (rs : Array Request) : IO Unit := do
  for r in rs do
    let hdrs := r.headers.foldl (fun a (k, v) => a ++ k ++ ": " ++ v ++ "\n") ""
    o.putStrLn ("REQ " ++ toString r.id ++ " " ++ r.tag.name ++ " " ++ r.method ++ " " ++
      (if r.streaming then "1" else "0") ++ " " ++ r.url ++ " " ++ hex hdrs.toUTF8 ++ " " ++
      (if r.body.size == 0 then "-" else hex r.body))

/-- The player position now (for the simulated load-complete report). -/
def handleCmds (o : IO.FS.Stream) (st : St) (cs : Array Command) (now : UInt64) : IO St := do
  let mut st := st
  for c in cs do
    o.putStrLn ("CMD " ++ c.describe)
    -- Extrapolate the simulated position before applying the command.
    let p := if st.p.state == .playing then { st.p with currentTimeMs := st.s.current.currentTimeMs } else st.p
    let p := apply p c
    let (s, rs) := st.s.report p now
    emitReqs o rs
    st := { st with s, p, loadAt := match c with | .playVideo .. => now + 1000 | _ => st.loadAt }
  return st

def after (o : IO.FS.Stream) (st : St) (now : UInt64) : IO St := do
  let mut st := st
  if st.loadAt != 0 && now ≥ st.loadAt then
    let p := { st.p with state := .playing, durationMs := 600000 }
    let (s, rs) := st.s.report p now
    emitReqs o rs
    st := { st with s, p, loadAt := 0 }
  if st.s.pairingCode != st.code then
    match st.s.pairingCode with
    | some c => o.putStrLn ("CODE " ++ c)
    | none => pure ()
    st := { st with code := st.s.pairingCode }
  if st.s.identity.screenId != st.screenId then
    o.putStrLn ("ID " ++ st.s.identity.screenId.getD "-")
    st := { st with screenId := st.s.identity.screenId }
  if st.s.remoteNames != st.remotes then
    o.putStrLn ("REMOTES " ++ st.s.remoteNames.foldl (fun a n => a ++ n ++ "; ") "")
    st := { st with remotes := st.s.remoteNames }
  if st.s.lastError != st.err then
    o.putStrLn ("ERR " ++ st.s.lastError)
    st := { st with err := st.s.lastError }
  return st

def step (o : IO.FS.Stream) (st : St) (w : Array String) (now : UInt64) : IO St := do
  let arg (i : Nat) := w[i]?.getD "-"
  let num (i : Nat) := (parseNat? (arg i)).getD 0
  match arg 1 with
  | "R" =>
    let (s, rs, cs) := st.s.onResponse (num 2) (num 3) (b64 (arg 4)) now
    emitReqs o rs
    handleCmds o { st with s } cs now
  | "C" =>
    let (s, rs, cs) := st.s.onChunk (num 2) (b64 (arg 3)) now
    emitReqs o rs
    handleCmds o { st with s } cs now
  | "E" =>
    let (s, rs) := st.s.onPollEnd (num 2) (parseNat? (arg 3)) now
    emitReqs o rs
    pure { st with s }
  | "T" =>
    let (s, rs) := st.s.tick now
    emitReqs o rs
    pure { st with s }
  | "D" =>
    let (s, rs) := st.s.registerPairingCode (arg 2) now
    emitReqs o rs
    pure { st with s }
  | "Q" =>
    let (s, rs) := st.s.stop
    emitReqs o rs
    pure { st with s, done := true }
  | _ => pure st

partial def loop (i o : IO.FS.Stream) (st : St) : IO Unit := do
  let line ← i.getLine
  if line.isEmpty then return
  let w := splitOn (trimmed line.toUTF8) 32
  let now := ((parseNat? (w[0]?.getD "0")).getD 0).toUInt64
  let st ← step o st w now
  let st ← after o st now
  o.putStrLn "OK"
  o.flush
  if !st.done then loop i o st

def main : IO Unit := do
  let i ← IO.getStdin
  let o ← IO.getStdout
  let line ← i.getLine
  let w := splitOn (trimmed line.toUTF8) 32
  let now := ((parseNat? (w[0]?.getD "0")).getD 0).toUInt64
  if w[1]? != some "S" then
    o.putStrLn "ERR expected S"
    return
  let sid := w[3]?.getD "-"
  let ident : Identity := { screenId := if sid == "-" then none else some sid, deviceId := w[4]?.getD "",
                            name := str (b64 (w[5]?.getD "-")) }
  let (s, rs) := Session.start {} ident (b64 (w[2]?.getD "-")) now
  emitReqs o rs
  let st ← after o { s } now
  o.putStrLn "OK"
  o.flush
  loop i o st
