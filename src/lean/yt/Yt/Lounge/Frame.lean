import Yt.Json.Doc
import Yt.Lounge.Form

/-! BrowserChannel framing on the Lounge bind channel, incrementally.

The body of a bind response (the initial POST and every long poll) is
`<decimal length>\n<JSON array>\n`, repeated. The length counts the bytes
after its own newline, including the newline after the array. Arrays may
span several lines. Every observed message was pure ASCII (the server sends
non-ASCII as `\uXXXX` escapes), so bytes and characters agree.

`Framer.feed` accepts the bytes in arbitrary pieces. It finds each array by
string-aware bracket matching (as the host prototype does) and only checks
the declared length (`mismatches` counts disagreements), so a wrong length
cannot desynchronize it. The scan state survives between feeds, so the work
is linear in the input.

Each array entry is `[arrayId, [name, args...]]`; `parseFrame` turns one
frame into `Msg`s with the payload object's members as strings. -/
namespace Yt.Lounge
open Yt.Json Yt.Fmt

/-- One message: array ID, name, positional arguments (strings decoded,
other JSON as source text), and the payload object's members if the first
argument is an object (string values decoded, others as source text). -/
structure Msg where
  aid : Nat
  name : String
  args : Array String := #[]
  fields : Array (String × String) := #[]
  deriving Inhabited, BEq

/-- Payload member `k` (the last one if repeated). -/
def Msg.get? (m : Msg) (k : String) : Option String := Id.run do
  let mut r := none
  for (a, b) in m.fields do
    if a == k then r := some b
  return r

def Msg.getD (m : Msg) (k : String) (d : String := "") : String := (m.get? k).getD d

/-- Scan state. Phases: 0 before the length (whitespace allowed), 1 in the
length digits, 2 after the length line, before `[`, 3 inside the array. -/
structure Framer where
  buf : ByteArray := ByteArray.empty
  /-- Next byte of `buf` to examine. -/
  pos : Nat := 0
  phase : UInt8 := 0
  declared : Nat := 0
  /-- Offset in `buf` just past the length line's newline (phase 2 and 3). -/
  bodyStart : Nat := 0
  /-- Offset of the array's `[` (phase 3). -/
  start : Nat := 0
  depth : Nat := 0
  inStr : Bool := false
  esc : Bool := false
  failed : Bool := false
  /-- Frames whose declared length disagreed with the bytes (diagnostic). -/
  mismatches : Nat := 0
  deriving Inhabited

@[inline] def isWsByte (c : UInt8) : Bool := c == 32 || c == 10 || c == 13 || c == 9

/-- Advance over `f.buf` from `f.pos`, collecting complete frames. -/
partial def Framer.scan (f : Framer) (out : Array ByteArray) : Framer × Array ByteArray :=
  if f.failed || f.pos ≥ f.buf.size then (f, out) else
  let c := byteAt f.buf f.pos
  let i := f.pos
  match f.phase with
  | 0 =>
    if isWsByte c then Framer.scan { f with pos := i + 1 } out
    else if 48 ≤ c && c ≤ 57 then
      Framer.scan { f with pos := i + 1, phase := 1, declared := (c - 48).toNat } out
    else ({ f with failed := true }, out)
  | 1 =>
    if 48 ≤ c && c ≤ 57 then
      if f.declared > 100000000 then ({ f with failed := true }, out)
      else Framer.scan { f with pos := i + 1, declared := f.declared * 10 + (c - 48).toNat } out
    else if c == 10 then Framer.scan { f with pos := i + 1, phase := 2, bodyStart := i + 1 } out
    else if c == 13 then Framer.scan { f with pos := i + 1 } out
    else ({ f with failed := true }, out)
  | 2 =>
    if isWsByte c then Framer.scan { f with pos := i + 1 } out
    else if c == 91 then
      Framer.scan { f with pos := i + 1, phase := 3, start := i, depth := 1, inStr := false, esc := false } out
    else ({ f with failed := true }, out)
  | _ =>
    if f.inStr then
      if f.esc then Framer.scan { f with pos := i + 1, esc := false } out
      else if c == 92 then Framer.scan { f with pos := i + 1, esc := true } out
      else if c == 34 then Framer.scan { f with pos := i + 1, inStr := false } out
      else Framer.scan { f with pos := i + 1 } out
    else if c == 34 then Framer.scan { f with pos := i + 1, inStr := true } out
    else if c == 91 then Framer.scan { f with pos := i + 1, depth := f.depth + 1 } out
    else if c == 93 then
      if f.depth == 1 then
        let frame := f.buf.extract f.start (i + 1)
        -- The length covers the body through the newline after the array.
        let mism := if f.declared == i + 2 - f.bodyStart then f.mismatches else f.mismatches + 1
        Framer.scan { f with pos := i + 1, phase := 0, depth := 0, mismatches := mism } (out.push frame)
      else Framer.scan { f with pos := i + 1, depth := f.depth - 1 } out
    else Framer.scan { f with pos := i + 1 } out

/-- Feed the next bytes; the complete frames (each a JSON array's bytes).
After a framing error `failed` is set and nothing more is produced. -/
def Framer.feed (f : Framer) (data : ByteArray) : Framer × Array ByteArray :=
  let f := { f with buf := f.buf ++ data }
  let (f, frames) := f.scan #[]
  -- Keep only the unfinished frame's body (the length is already a number).
  let keep := if f.phase ≤ 1 then f.pos else f.bodyStart
  if keep == 0 then (f, frames) else
  ({ f with buf := f.buf.extract keep f.buf.size, pos := f.pos - keep,
            start := f.start - min f.start keep, bodyStart := f.bodyStart - keep }, frames)

/-- A JSON value as a string argument: strings decoded, others as source text. -/
def valText (d : Doc) (v : Val) : String :=
  match d.kind v with
  | .str => (d.str? v).getD ""
  | _ => str (d.raw v)

/-- The messages in one frame. -/
def parseFrame (frame : ByteArray) : Except String (Array Msg) := do
  let d ← Doc.parse frame
  if d.kind d.root != .arr then throw "lounge: frame is not an array"
  let mut out : Array Msg := #[]
  for e in d.elems d.root do
    let some idv := d.at? e 0 | throw "lounge: entry without an id"
    let some aid := d.u64? idv | throw "lounge: bad array id"
    let some inner := d.at? e 1 | throw "lounge: entry without a body"
    let parts := d.elems inner
    if parts.size == 0 then throw "lounge: empty message"
    let some name := d.str? parts[0]! | throw "lounge: message name is not a string"
    let mut args : Array String := #[]
    let mut fields : Array (String × String) := #[]
    for k in [1:parts.size] do
      let p := parts[k]!
      if k == 1 && d.kind p == .obj then
        for (kv, vv) in d.members p do
          fields := fields.push ((d.str? kv).getD "", valText d vv)
      else args := args.push (valText d p)
    out := out.push { aid := aid.toNat, name, args, fields }
  return out

/-- Every message in a complete body (the initial bind POST's reply). -/
def parseBody (body : ByteArray) : Except String (Array Msg) := do
  let (f, frames) := ({} : Framer).feed body
  if f.failed then throw "lounge: framing error"
  let mut out : Array Msg := #[]
  for fr in frames do
    out := out ++ (← parseFrame fr)
  return out

end Yt.Lounge
