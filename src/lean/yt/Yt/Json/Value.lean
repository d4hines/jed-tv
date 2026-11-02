import Yt.Json.Doc

/-! A JSON tree for building request bodies (and for small documents), its
compact serializer, and lookups. Numbers are kept as their literal text, so
nothing is rounded. `render` writes what serde_json's `to_string` writes for
the same tree: no spaces; `"`, `\` and control characters escaped (`\n`,
`\t`, ..., `\u001f`), everything else (including non-ASCII) raw. Objects keep
their member order: serde_json (without `preserve_order`, as in ytplay)
sorts keys, so build objects in sorted order to get byte-identical bodies. -/
namespace Yt.Json
open Yt.Fmt

inductive Json where
  | null
  | bool (b : Bool)
  /-- A number, as its JSON literal text (e.g. `"136"`, `"-1.5e3"`). -/
  | num (lit : String)
  | str (s : String)
  | arr (xs : Array Json)
  | obj (kvs : Array (String × Json))
  deriving Inhabited

namespace Json

def ofU64 (x : UInt64) : Json := .num (dec x)
def ofNat (n : Nat) : Json := .num (toString n)

/-- Append `s` as a JSON string literal (with quotes). -/
def pushStr (o : ByteArray) (s : ByteArray) : ByteArray := Id.run do
  let mut o := o.push 34
  let mut run := 0
  for i in [0:s.size] do
    let c := byteAt s i
    if c == 34 || c == 92 || c < 32 then
      if i > run then o := s.copySlice run o o.size (i - run) false
      run := i + 1
      o := o.push 92
      if c == 34 || c == 92 then o := o.push c
      else if c == 10 then o := o.push 110
      else if c == 9 then o := o.push 116
      else if c == 13 then o := o.push 114
      else if c == 8 then o := o.push 98
      else if c == 12 then o := o.push 102
      else o := ((((o.push 117).push 48).push 48).push (hexDigit (c >>> 4))).push (hexDigit (c &&& 15))
  if s.size > run then o := s.copySlice run o o.size (s.size - run) false
  return o.push 34

/-- Append the compact serialization of `j`. -/
partial def renderTo (o : ByteArray) : Json → ByteArray
  | .null => o ++ "null".toUTF8
  | .bool b => o ++ (if b then "true" else "false").toUTF8
  | .num lit => o ++ lit.toUTF8
  | .str s => pushStr o s.toUTF8
  | .arr xs => Id.run do
    let mut o := o.push 91
    for i in [0:xs.size] do
      if i > 0 then o := o.push 44
      o := renderTo o xs[i]!
    return o.push 93
  | .obj kvs => Id.run do
    let mut o := o.push 123
    for i in [0:kvs.size] do
      if i > 0 then o := o.push 44
      let (k, v) := kvs[i]!
      o := renderTo ((pushStr o k.toUTF8).push 58) v
    return o.push 125

/-- The compact serialization of `j` (UTF-8). -/
def render (j : Json) : ByteArray := renderTo ByteArray.empty j

/-- Member `key` (the last one if repeated). -/
def get? (j : Json) (key : String) : Option Json :=
  match j with
  | .obj kvs => Id.run do
    let mut r := none
    for (k, v) in kvs do
      if k == key then r := some v
    return r
  | _ => none

/-- Element `n` of an array. -/
def at? (j : Json) (n : Nat) : Option Json :=
  match j with
  | .arr xs => xs[n]?
  | _ => none

/-- Follow object keys. -/
def path? (j : Json) (keys : Array String) : Option Json := Id.run do
  let mut cur := j
  for k in keys do
    match cur.get? k with
    | some x => cur := x
    | none => return none
  return some cur

def str? : Json → Option String
  | .str s => some s
  | _ => none

def bool? : Json → Option Bool
  | .bool b => some b
  | _ => none

def isNull : Json → Bool
  | .null => true
  | _ => false

/-- serde_json's `as_u64` (see `Doc.u64?`). -/
def u64? : Json → Option UInt64
  | .num lit => Id.run do
    let b := lit.toUTF8
    if b.size == 0 then return none
    let mut x : Nat := 0
    for c in b do
      if !isDigit c then return none
      x := x * 10 + (c - 48).toNat
      if x ≥ 18446744073709551616 then return none
    return some x.toUInt64
  | _ => none

/-- Set member `key` of an object: replace it in place if present, else
append it. Non-objects are returned unchanged. -/
def set (j : Json) (key : String) (v : Json) : Json :=
  match j with
  | .obj kvs => Id.run do
    for i in [0:kvs.size] do
      if kvs[i]!.1 == key then return .obj (kvs.set! i (key, v))
    return .obj (kvs.push (key, v))
  | j => j

end Json

/-- The value `v` of `d` as a tree. -/
partial def Doc.toJson (d : Doc) (v : Val) : Json :=
  match d.kind v with
  | .null => .null
  | .bool => .bool ((d.bool? v).getD false)
  | .num => .num (str (d.raw v))
  | .str => .str ((d.str? v).getD "")
  | .arr => .arr ((d.elems v).map d.toJson)
  | .obj => .obj ((d.members v).map fun (k, x) => ((d.str? k).getD "", d.toJson x))

/-- Parse a whole document into a tree. -/
def Json.parse (b : ByteArray) : Except String Json := do
  let d ← Doc.parse b
  pure (d.toJson d.root)

end Yt.Json
