import Yt.Crypto.Util
open Yt.Crypto
/-! Transcript helpers: vector parsing and result lines. -/
namespace CryptoTest

def line (o : ByteArray) (s : String) : ByteArray := o ++ (s ++ "\n").toUTF8

/-- `name ok` when `got` equals the expected hex, else both values. -/
def check (o : ByteArray) (name : String) (got : ByteArray) (want : String) : ByteArray :=
  if got == ofHex want then line o (name ++ " ok")
  else line o (name ++ " FAIL got " ++ toHex got ++ " want " ++ want)

def checkB (o : ByteArray) (name : String) (ok : Bool) : ByteArray :=
  line o (name ++ (if ok then " ok" else " FAIL"))

def rep (s : String) (n : Nat) : ByteArray := Id.run do
  let u := s.toUTF8
  let mut r := ByteArray.empty
  for _ in [0:n] do r := r ++ u
  r

/-- One vector: its raw space-separated fields. -/
abbrev Vec := Array ByteArray

/-- Lines of space-separated fields. -/
def parse (s : String) : Array Vec := Id.run do
  let mut out : Array Vec := #[]
  let mut cur : Vec := #[]
  let mut fld := ByteArray.empty
  for c in s.toUTF8 do
    if c == 32 then
      cur := cur.push fld
      fld := ByteArray.empty
    else if c == 10 then
      out := out.push (cur.push fld)
      cur := #[]
      fld := ByteArray.empty
    else fld := fld.push c
  if fld.size > 0 || cur.size > 0 then out := out.push (cur.push fld)
  out

def fieldStr (v : Vec) (i : Nat) : String := Id.run do
  let mut s := ""
  for c in v[i]! do s := s.push (Char.ofNat c.toNat)
  s
def fieldHex (v : Vec) (i : Nat) : ByteArray :=
  let f := v[i]!
  if f.size == 1 && f[0]! == 45 then ByteArray.empty else Id.run do
    let mut r := ByteArray.emptyWithCapacity (f.size / 2)
    for j in [0:f.size / 2] do r := r.push ((hexVal f[2*j]! <<< 4) ||| hexVal f[2*j+1]!)
    r
def fieldNat (v : Vec) (i : Nat) : Nat := Id.run do
  let mut n := 0
  for c in v[i]! do n := n * 10 + (c - 48).toNat
  n
/-- Expected outcome: `v` valid, `i` invalid, `a` acceptable (either). -/
def fieldRes (v : Vec) : UInt8 := (v[1]!)[0]!

/-- Run `test` (returning whether the vector was accepted) over every vector;
one summary line with the ids of mismatches. Acceptable vectors count as
passing either way. -/
def suite (o : ByteArray) (name : String) (data : String) (test : Vec → Bool) : ByteArray := Id.run do
  let vs := parse data
  let mut pass := 0
  let mut bad := ""
  for v in vs do
    let ok := test v
    let r := fieldRes v
    if r == 97 || (r == 118) == ok then pass := pass + 1
    else bad := bad ++ " " ++ fieldStr v 0
  line o (name ++ " " ++ toString pass ++ "/" ++ toString vs.size ++ (if bad == "" then " ok" else " FAIL ids" ++ bad))

end CryptoTest
