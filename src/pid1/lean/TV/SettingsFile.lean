prelude
import Init.Data.String.Basic
import Init.Data.List.Basic
import Init.Data.UInt
import TV.Apps
set_option compiler.extract_closed false

/-! The private parental-policy file. Only Core's storage adapter receives its
packed PIN/mask value; menus receive enabled-app bits separately from Core.
TV.Storage owns identity acceptance and the atomic commit sequence; native code
performs bounded filesystem operations. This module owns the format, decimal
validation, defaults and serialization. -/
namespace TV.SettingsFile

structure Policy where
  pin : UInt32
  enabled : UInt32
  /-- Three bits per app at `3 * (id - 1)`: 1 internet, 2 local network,
      4 saved data (TV.Core.grantBits). -/
  grants : UInt32
  offline : Bool
  deriving BEq, Repr

/-- Rust's `str::split_whitespace` uses the Unicode White_Space property.
Keep that existing input format, including vertical tab, form feed and NBSP. -/
def whitespace (c : Char) : Bool :=
  let n := c.toNat
  (9 ≤ n && n ≤ 13) || n == 32 || n == 133 || n == 160 || n == 5760 ||
  (8192 ≤ n && n ≤ 8202) || n == 8232 || n == 8233 || n == 8239 ||
  n == 8287 || n == 12288

private def tokens : List Char → List Char → List (List Char) → List (List Char)
  | [], word, words => (match word with | [] => words | _ => word.reverse :: words).reverse
  | c :: rest, word, words =>
    if whitespace c then
      tokens rest [] (match word with | [] => words | _ => word.reverse :: words)
    else tokens rest (c :: word) words

private def digit (c : Char) : Option UInt32 :=
  let n := c.toNat
  if 48 ≤ n && n ≤ 57 then some (n - 48).toUInt32 else none

private def decimal (limit : UInt32) : List Char → UInt32 → Option UInt32
  | [], n => some n
  | c :: rest, n => do
    let d ← digit c
    let next := n * 10 + d
    if next > limit then none else decimal limit rest next

private def pin : List Char → Option UInt32
  | [a,b,c,d] => decimal 9999 [a,b,c,d] 0
  | _ => none

private def mask (word : List Char) : Option UInt32 :=
  let word := match word with | '+' :: rest => rest | other => other
  match word with | [] => none | _ => decimal 255 word 0

private def grantWord (word : List Char) : Option UInt32 :=
  match word with | [] => none | _ => decimal 0xffffff word 0

private def flag : List Char → Option Bool
  | ['0'] => some false | ['1'] => some true | _ => none

/-- Every capability switched on, for files written before grants existed. -/
def allGrants : UInt32 := 0xffffff

/-- Preserve every stored mask bit, including the retired app's bit. Existing
files must never silently enable a newly added app or lose unrecognized bits.
`DANDE1` files predate grants and the offline switch: they load with every
capability on and online, which is what those TVs had. -/
def parse (text : String) : Option Policy := do
  match tokens text.toList [] [] with
  | [version, digits, bits] =>
    if version != ['D','A','N','D','E','1'] then none
    else return ⟨← pin digits, ← mask bits, allGrants, false⟩
  | [version, digits, bits, grants, offline] =>
    if version != ['D','A','N','D','E','2'] then none
    else return ⟨← pin digits, ← mask bits, ← grantWord grants, ← flag offline⟩
  | _ => none

private def defaults : Nat → UInt32
  | 0 => 0
  | n + 1 =>
    defaults n ||| if TV.Apps.valid (n + 1).toUInt32 then (1 <<< n.toUInt32) else 0

def defaultPolicy : Policy := ⟨0, defaults 8, allGrants, false⟩

def valid (p : Policy) : Bool := p.pin < 10000 && p.enabled < 256 && p.grants ≤ 0xffffff

/-- TV.Core.decodePolicy's word: PIN bits 0–15, enabled 16–23, offline bit
    33, grants bits 40–63. Bit 32 means unreadable. -/
def pack (p : Policy) : UInt64 :=
  p.pin.toUInt64 ||| (p.enabled.toUInt64 <<< 16) ||| ((if p.offline then (1 : UInt64) else 0) <<< 33) |||
  (p.grants.toUInt64 <<< 40)

private def char (n : UInt32) : Char := Char.ofNat (48 + n.toNat)

/-- Canonical bytes: four ASCII PIN digits, decimal mask and exactly one LF. -/
def format (p : Policy) : Option String :=
  if !valid p then none else
    let digits := String.ofList [char (p.pin / 1000), char (p.pin / 100 % 10),
      char (p.pin / 10 % 10), char (p.pin % 10)]
    some ("DANDE2 " ++ digits ++ " " ++ toString p.enabled.toNat ++ " " ++ toString p.grants.toNat ++
      (if p.offline then " 1\n" else " 0\n"))

/-- Core's startup word. A parse failure is Core's load error (bit 32), never
the factory default policy. -/
@[export tv_settings_file_parse]
def parseWord (text : String) : UInt64 :=
  match parse text with | some p => pack p | none => (1 : UInt64) <<< 32

@[export tv_settings_file_default]
def defaultWord (_ : UInt32) : UInt64 := pack defaultPolicy

/-- Empty output is the invalid-save sentinel; it is never a valid file. -/
@[export tv_settings_file_format]
def formatWord (pin enabled grants : UInt32) (offline : UInt8) : String :=
  (format ⟨pin, enabled, grants, offline != 0⟩).getD ""

end TV.SettingsFile
