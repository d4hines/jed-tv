prelude
import Init.System.IO
import Init.Data.List.Lemmas
set_option compiler.extract_closed false

/-! The supervisor owns Wi-Fi policy and credential formatting. Workers only
perform WPA/DHCP operations; all calls into this module use its Lean executor. -/
namespace TV.Wifi

structure Network where
  ssid : String
  signal : String
  secure : Bool
  deriving BEq, Repr

private def lines (rest word : List Char) (out : List (List Char)) : List (List Char) :=
  match rest with
  | [] => (match word with | [] => out | _ => word.reverse :: out).reverse
  | '\n' :: xs =>
    let word := match word with | '\r' :: xs => xs | xs => xs
    lines xs [] (word.reverse :: out)
  | c :: xs => lines xs (c :: word) out

private def fields : Nat → List Char → List Char → List (List Char) → List (List Char)
  | 0, rest, word, out => ((word.reverse ++ rest) :: out).reverse
  | _, [], word, out => (word.reverse :: out).reverse
  | n + 1, '\t' :: xs, word, out => fields n xs [] (word.reverse :: out)
  | n, c :: xs, word, out => fields n xs (c :: word) out

private def startsWith : List Char → List Char → Bool
  | [], _ => true
  | c :: cs, d :: ds => c == d && startsWith cs ds
  | _, _ => false

private def contains (needle : List Char) : List Char → Bool
  | [] => startsWith needle []
  | c :: cs => startsWith needle (c :: cs) || contains needle cs

private def eligible (flags : List Char) : Option Bool :=
  let secure := contains ['P','S','K'] flags
  if contains ['W','E','P'] flags || (!secure &&
      (contains ['W','P','A'] flags || contains ['E','A','P'] flags || contains ['S','A','E'] flags)) then none
  else some secure

private def appendScan : List (List Char) → List Network → Nat → List Network
  | [], out, _ => out.reverse
  | _, out, 0 => out.reverse
  | row :: rest, out, n + 1 =>
    match fields 4 row [] [] with
    | [_, _, strength, flags, name] =>
      let ssid := String.ofList name
      if ssid.utf8ByteSize == 0 || ssid.utf8ByteSize > 32 || contains ['\\'] name ||
          out.any (fun network => network.ssid == ssid) then appendScan rest out (n + 1)
      else match eligible flags with
        | none => appendScan rest out (n + 1)
        | some secure => appendScan rest (⟨ssid, String.ofList strength, secure⟩ :: out) n
    | _ => appendScan rest out (n + 1)

/-- Keep WPA scan order, first eligible SSID occurrence, and at most 24 rows.
Only open and WPA/WPA2 PSK networks are supported by the current UI. Escaped
binary names are excluded so the displayed name is never a different SSID. -/
def parseScan (text : String) : List Network :=
  match lines text.toList [] [] with
  | [] => []
  | _ :: rows => appendScan rows [] 24

private def hexDigit (n : UInt8) : Char :=
  Char.ofNat (if n < 10 then 48 + n.toNat else 87 + n.toNat)

private def hex (text : String) : String := Id.run do
  let bytes := text.toUTF8
  let mut out := ""
  for i in List.range bytes.size do
    let b := bytes[i]!
    out := (out.push (hexDigit (b >>> 4))).push (hexDigit (b &&& 15))
  return out

private def quote : List Char → String → String
  | [], out => out
  | c :: cs, out => quote cs ((if c == '\\' || c == '"' then out.push '\\' else out).push c)

/-- SSIDs are hex bytes. Passphrases are printable ASCII with only quote and
backslash escaped, so neither field can introduce a WPA configuration line. -/
def configuration (ssid password : String) (secure : Bool) : Except String String := do
  if ssid.utf8ByteSize == 0 || ssid.utf8ByteSize > 32 then
    throw "Invalid Wi-Fi network name"
  if secure && (password.utf8ByteSize < 8 || password.utf8ByteSize > 63 ||
      !password.toList.all (fun c => c.toNat >= 32 && c.toNat <= 126)) then
    throw "WPA password must be 8 to 63 printable ASCII characters"
  let credentials := if secure then "key_mgmt=WPA-PSK\npsk=\"" ++ quote password.toList "" ++ "\"\n"
                     else "key_mgmt=NONE\n"
  return "ctrl_interface=/run/tv-network/wpa\nnetwork={\nssid=" ++ hex ssid ++ "\n" ++ credentials ++ "}\n"

@[extern "tv_wifi_network"]
opaque emitNetwork (ssid signal : @& String) (secure : UInt32) : IO Unit
@[extern "tv_wifi_result"]
opaque emitResult (success : UInt32) (text : @& String) : IO Unit

@[export tv_wifi_scan]
def scan (text : String) : IO Unit := do
  for network in parseScan text do
    emitNetwork network.ssid network.signal (if network.secure then 1 else 0)

@[export tv_wifi_configuration]
def prepare (ssid password : String) (secure : UInt32) : IO Unit :=
  match configuration ssid password (secure != 0) with
  | .ok config => emitResult 1 config
  | .error message => emitResult 0 message

@[export tv_wifi_io_unit]
def ioUnit (_ : UInt32) : IO Unit := pure ()

end TV.Wifi
