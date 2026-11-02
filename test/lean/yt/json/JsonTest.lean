import Yt.Json.YouTube
import JsonTest.Fixtures

/-! The JSON library against Python's json module (ref.py): for each
fixture, the canonical re-serialization of the whole document and the
fields the app extracts (yt.rs rules); edge cases (accept/reject and
canonical form); the request bodies; and unit checks of the lookups. -/
namespace JsonTest
open Yt.Json Yt.Json.YouTube Yt.Fmt

def js (s : String) : String := str (Json.pushStr ByteArray.empty s.toUTF8)
def n (x : Nat) : String := toString x
def u (x : UInt64) : String := dec x

def lines (s : String) : Array String := Id.run do
  let b := s.toUTF8
  let mut out := #[]
  let mut p := 0
  for i in [0:b.size] do
    if b[i]! == 10 then
      out := out.push (lossy (b.extract p i))
      p := i + 1
  if p < b.size then out := out.push (lossy (b.extract p b.size))
  return out

/-- yt.rs's walk with serde_json's sorted keys (to see whether it differs). -/
partial def collectSorted (d : Doc) (v : Val) (out : Array SearchItem) : Array SearchItem :=
  match d.kind v with
  | .obj =>
    let out := match d.get? v "videoRenderer" with
      | some r => match strAt d (d.get? r "videoId") with
        | some id => out.push { id, title := textOf d (d.get? r "title"),
                                channel := textOf d (d.get? r "ownerText"),
                                length := textOf d (d.get? r "lengthText") }
        | none => out
      | none => out
    let ms := (d.members v).map (fun (k, x) => ((d.str? k).getD "", x))
    -- Insertion sort (Array.qsort needs lean_array_fswap, missing on our runtime).
    let ms := Id.run do
      let mut a := ms
      for i in [1:a.size] do
        let mut j := i
        while j > 0 && a[j]!.1 < a[j - 1]!.1 do
          let t := a[j]!
          a := (a.set! j a[j - 1]!).set! (j - 1) t
          j := j - 1
      return a
    ms.foldl (fun o (_, x) => collectSorted d x o) out
  | .arr => (d.elems v).foldl (fun o x => collectSorted d x o) out
  | _ => out

def dedup (all : Array SearchItem) : Array SearchItem := Id.run do
  let mut out : Array SearchItem := #[]
  for i in all do
    if i.length.isEmpty then continue
    if out.any (·.id == i.id) then continue
    out := out.push i
  return out

def sameItems (a b : Array SearchItem) : Bool := Id.run do
  if a.size != b.size then return false
  for i in [0:a.size] do
    let x := a[i]!
    let y := b[i]!
    if x.id != y.id || x.title != y.title || x.channel != y.channel || x.length != y.length then
      return false
  return true

/-- The text ref.py prints for a document. -/
def extraction (d : Doc) : String := Id.run do
  let p := playerOf d
  let mut o := "status " ++ js p.status ++ "\nreason " ++ js p.reason ++ "\nvisitor "
    ++ (match p.visitorData with | some v => js v | none => "none") ++ "\ntitle " ++ js p.title
    ++ "\nlength " ++ u p.lengthS ++ "\n"
  for f in p.formats do
    o := o ++ "format itag" ++ n f.itag.toNat ++ " mime" ++ js f.mime ++ " w" ++ n f.width.toNat
      ++ " h" ++ n f.height.toNat ++ " fps" ++ n f.fps.toNat ++ " br" ++ u f.bitrate ++ " clen"
      ++ u f.contentLength ++ " init" ++ u f.initRange.1 ++ "-" ++ u f.initRange.2 ++ " index"
      ++ u f.indexRange.1 ++ "-" ++ u f.indexRange.2 ++ " url" ++ js f.url ++ "\n"
  for mh in [720, 480, 144, 100] do
    o := o ++ "pick-video " ++ n mh ++ " " ++ (match pickVideo p.formats mh.toUInt32 with
      | some f => n f.itag.toNat | none => "none") ++ "\n"
  o := o ++ "pick-audio " ++ (match pickAudio p.formats with
    | some f => n f.itag.toNat | none => "none") ++ "\n"
  let items := searchOf d
  let sorted := dedup (collectSorted d d.root #[])
  o := o ++ "search " ++ n items.size ++ " (rust sorted-key walk: "
    ++ (if sameItems items sorted then "same order" else "different order") ++ ")\n"
  for i in items do
    o := o ++ "item " ++ js i.id ++ " title" ++ js i.title ++ " channel" ++ js i.channel ++ " length"
      ++ js i.length ++ "\n"
  return o

def firstDiff (a b : String) : String := Id.run do
  let la := lines a
  let lb := lines b
  for i in [0:max la.size lb.size] do
    let x := la[i]?.getD "<none>"
    let y := lb[i]?.getD "<none>"
    if x != y then
      return "line " ++ n (i + 1) ++ "\n    python: " ++ x ++ "\n    lean:   " ++ y
  return "no line differs"

def fixture (name : String) (json : ByteArray) (canon expect : String) : String :=
  match Doc.parse json with
  | .error e => "json " ++ name ++ ": FAIL " ++ e ++ "\n"
  | .ok d =>
    let c := str (Json.render (d.toJson d.root)) ++ "\n"
    let x := extraction d
    let p := playerOf d
    let items := searchOf d
    "json " ++ name ++ ": " ++ n json.size ++ " bytes, " ++ n d.count ++ " values, "
      ++ n p.formats.size ++ " formats, " ++ n items.size ++ " search items; canonical form "
      ++ (if c == canon then "identical" else "DIFFERS at " ++ firstDiff canon c)
      ++ "; extraction " ++ (if x == expect then "identical" else "DIFFERS at " ++ firstDiff expect x)
      ++ "\n"

def cases : String := Id.run do
  let cs := lines Fixtures.cases
  let want := lines Fixtures.casesExpect
  let mut ok := 0
  let mut o := ""
  for i in [0:cs.size] do
    let b := (base64Decode cs[i]!.toUTF8).getD ByteArray.empty
    let got := match Doc.parse b with
      | .ok d => "ok " ++ str (Json.render (d.toJson d.root))
      | .error _ => "err"
    if got == want[i]! then ok := ok + 1
    else o := o ++ "  case " ++ n i ++ " " ++ cs[i]! ++ ": python " ++ want[i]! ++ ", lean " ++ got ++ "\n"
  return "json edge cases: " ++ n ok ++ " of " ++ n cs.size ++ " agree with Python\n" ++ o

def bodies : String := Id.run do
  let want := (lines Fixtures.bodies).map fun l => str ((base64Decode l.toUTF8).getD ByteArray.empty)
  let vis := "CgtTWVg5bUIwa0VZWSjK7pvXBjIKCgJVUxIEGgAgJg%3D%3D"
  let got := #[playerBody "PVGeM40dABA" none, playerBody "PVGeM40dABA" (some vis),
    searchBody "big buck bunny", searchBody "say \"hi\" \\ back\tslash\n",
    searchBody "café 日本 😀", searchBody "\x01\x1f\x7f"].map str
  let mut ok := 0
  let mut o := ""
  for i in [0:got.size] do
    if got[i]! == want[i]?.getD "" then ok := ok + 1
    else o := o ++ "  body " ++ n i ++ ":\n    python " ++ want[i]?.getD "" ++ "\n    lean   " ++ got[i]! ++ "\n"
  return "json request bodies: " ++ n ok ++ " of " ++ n got.size ++ " byte-identical to Python (sorted keys)\n" ++ o

def check (name : String) (b : Bool) : String := "json unit " ++ name ++ ": " ++ (if b then "ok" else "FAIL") ++ "\n"

def nest (k : Nat) : ByteArray := Id.run do
  let mut b := ByteArray.empty
  for _ in [0:k] do b := b.push 91
  for _ in [0:k] do b := b.push 93
  return b

def units : String := Id.run do
  let mut o := ""
  o := o ++ check "depth 1000 accepted, 1001 rejected"
    ((Doc.parse (nest 1000)).toOption.isSome && (Doc.parse (nest 1001)).toOption.isNone)
  let src := "{\"x\":[[1], {\"k\" : \"v\"}] ,\"y\":3,\"y\":4, \"s\":\"a\\nb\", \"n\":-1.5e3, \"t\":true}".toUTF8
  match Doc.parse src with
  | .error e => o := o ++ "json unit doc: FAIL " ++ e ++ "\n"
  | .ok d =>
    let r := d.root
    let x := (d.get? r "x").getD r
    o := o ++ check "get? last duplicate" ((d.get? r "y").bind d.u64? == some 4)
    o := o ++ check "size/members/elems" (d.size r == 6 && (d.members r).size == 6 && d.size x == 2
      && (d.elems x).size == 2)
    o := o ++ check "raw of nested containers"
      (str (d.raw x) == "[[1], {\"k\" : \"v\"}]" && str (d.raw ((d.at? x 0).getD r)) == "[1]"
       && str (d.raw ((d.at? x 1).getD r)) == "{\"k\" : \"v\"}")
    o := o ++ check "path? and at?" ((d.path? r #["x"]).isSome && (d.at? x 2).isNone
       && (((d.at? x 1).bind (d.get? · "k")).bind d.str?) == some "v")
    o := o ++ check "escaped string" ((d.get? r "s").bind d.str? == some "a\nb")
    o := o ++ check "number raw and u64?" (((d.get? r "n").map (str ∘ d.raw)) == some "-1.5e3"
       && (d.get? r "n").bind d.u64? == none && (d.get? r "t").bind d.bool? == some true)
    o := o ++ check "kinds" (d.kind r == .obj && d.kind x == .arr && (d.get? r "t").map d.kind == some .bool)
  let j : Json := .obj #[("a", .num "1")]
  o := o ++ check "Json.set replaces and appends"
    (str (((j.set "a" (.str "z")).set "b" .null).render) == "{\"a\":\"z\",\"b\":null}")
  o := o ++ check "u64 bounds" ((Json.num "18446744073709551615").u64? == some 18446744073709551615
    && (Json.num "18446744073709551616").u64? == none && (Json.num "-0").u64? == none
    && (Json.num "1.0").u64? == none)
  o := o ++ check "parseU64 (Rust str::parse)" (parseU64 "+12".toUTF8 == some 12 && parseU64 "+".toUTF8 == none
    && parseU64 "".toUTF8 == none && parseU64 " 1".toUTF8 == none && parseU64 "007".toUTF8 == some 7)
  return o

@[export zr_test_transcript] def transcript (k : Nat) : ByteArray := Id.run do
  let mut o := ""
  for (name, json, canon, expect) in Fixtures.all do
    o := o ++ fixture name (json.toUTF8.extract k json.utf8ByteSize) canon expect
  o := o ++ cases ++ bodies ++ units
  return o.toUTF8

end JsonTest
