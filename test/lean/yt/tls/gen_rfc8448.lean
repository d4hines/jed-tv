/-! Extracts RFC 8448 trace values (section 3, simple 1-RTT handshake) into
TlsTest/Rfc8448.lean. Input: assets/build/lean-tls/rfc/rfc8448.txt
(rfc-editor.org).

    lake env lean --run gen_rfc8448.lean -/

/-- The text without page breaks: the blank lines, footer (`... [Page n]`),
form feed, running header (`RFC 8448 ...`) and blank lines between two pages
become one line break. -/
def dropPages (text : String) : String := Id.run do
  let ls := (text.splitOn "\n").toArray
  let mut out : Array String := #[]
  let mut i := 0
  while i < ls.size do
    let l := ls[i]!
    let isFooter := (l.splitOn "[Page ").length > 1 && l.endsWith "]"
    if isFooter && i + 2 < ls.size && (ls[i + 2]!).startsWith "RFC 8448" &&
        (ls[i + 1]! == "\x0c" || ls[i + 1]! == "") && out.size > 0 && out.back! == "" then
      while out.size > 0 && out.back! == "" do out := out.pop
      i := i + 3
      while i < ls.size && ls[i]! == "" do i := i + 1
    else
      out := out.push l
      i := i + 1
  return "\n".intercalate out.toList

/-- The text from the line starting with `start` up to the line starting with `stop`. -/
def sectionOf (text start stop : String) : String :=
  let after := (text.splitOn ("\n" ++ start)).drop 1
  let rest := "\n" ++ start ++ ("\n" ++ start).intercalate after
  (rest.splitOn ("\n" ++ stop)).headD rest

def isHex (c : Char) : Bool := c.isDigit || ('a' ≤ c && c ≤ 'f')

/-- A continuation line: at least 9 spaces, then a hex byte, then a space or the end. -/
def continuation (l : String) : Bool :=
  let cs := l.toList
  let sp := (cs.takeWhile (· == ' ')).length
  let rest := cs.drop sp
  sp ≥ 9 && match rest with
    | a :: b :: [] => isHex a && isHex b
    | a :: b :: c :: _ => isHex a && isHex b && c.isWhitespace
    | _ => false

/-- The hex value after `label`, following the `occurrence`-th `header` in `sec`. -/
def field (sec header : String) (occurrence : Nat) (label : String) : Except String String := do
  let parts := sec.splitOn header
  if parts.length ≤ occurrence then throw s!"missing {header}"
  let afterHeader := header.intercalate (parts.drop occurrence)
  let lparts := afterHeader.splitOn label
  if lparts.length < 2 then throw s!"missing {label}"
  let rest := label.intercalate (lparts.drop 1)
  let lines := rest.splitOn "\n"
  let mut out := ""
  let mut k := 0
  for line in lines do
    if k > 0 && !continuation line then break
    let part := if k == 0 && (line.splitOn ":").length > 1 then ":".intercalate ((line.splitOn ":").drop 1) else line
    out := out ++ String.ofList (part.toList.filter (!·.isWhitespace))
    k := k + 1
  if out.isEmpty || !out.toList.all isHex then throw s!"{header} {label}: not hex"
  -- "(N octets)" in the label: the value's length
  let n := (label.splitOn "(").getD 1 "" |>.splitOn " octets" |>.headD "" |>.toNat?
  if let some n := n then
    if out.length != 2 * n then throw s!"{header} {label}: {out.length / 2} octets"
  return out

def items3 : List (String × String × Nat × String) := [
  ("clientPriv", "{client}  create an ephemeral x25519 key pair:", 1, "private key (32 octets):"),
  ("clientHello", "{client}  construct a ClientHello handshake message:", 1, "ClientHello (196 octets):"),
  ("clientHelloRecord", "{client}  send handshake record:", 1, "complete record (201 octets):"),
  ("serverHelloRecord", "{server}  send handshake record:", 1, "complete record (95 octets):"),
  ("serverHsRecord", "{server}  send handshake record:", 2, "complete record (679 octets):"),
  ("serverHsPayload", "{server}  send handshake record:", 2, "payload (657 octets):"),
  ("certificateMsg", "{server}  construct a Certificate handshake message:", 1, "Certificate (445 octets):"),
  ("cHsTraffic", "{server}  derive secret \"tls13 c hs traffic\":", 1, "expanded (32 octets):"),
  ("sHsTraffic", "{server}  derive secret \"tls13 s hs traffic\":", 1, "expanded (32 octets):"),
  ("cApTraffic", "{server}  derive secret \"tls13 c ap traffic\":", 1, "expanded (32 octets):"),
  ("sApTraffic", "{server}  derive secret \"tls13 s ap traffic\":", 1, "expanded (32 octets):"),
  ("clientFinishedRecord", "{client}  send handshake record:", 2, "complete record (58 octets):"),
  ("ticketRecord", "{server}  send handshake record:", 3, "complete record (227 octets):"),
  ("clientAppRecord", "{client}  send application_data record:", 1, "complete record (72 octets):"),
  ("serverAppRecord", "{server}  send application_data record:", 1, "complete record (72 octets):"),
  ("clientAlertRecord", "{client}  send alert record:", 1, "complete record (24 octets):"),
  ("serverAlertRecord", "{server}  send alert record:", 1, "complete record (24 octets):")]

def main : IO UInt32 := do
  let text := dropPages (← IO.FS.readFile "../../../../assets/build/lean-tls/rfc/rfc8448.txt")
  let s3 := sectionOf text "3.  Simple 1-RTT Handshake" "4.  Resumed 0-RTT Handshake"
  let mut defs := #[]
  for (ident, header, occ, label) in items3 do
    match field s3 header occ label with
    | .ok h => defs := defs.push s!"def {ident} : String := \"{h}\""
    | .error e => IO.eprintln e; return 1
  let out := "\n".intercalate (["/-! RFC 8448 trace values (section 3, simple 1-RTT handshake). Generated by",
    "../gen_rfc8448.lean from the RFC text; do not edit. -/", "namespace TlsTest.Rfc8448", "",
    "namespace S3\n" ++ "\n".intercalate defs.toList ++ "\nend S3\n", "end TlsTest.Rfc8448", ""])
  IO.FS.writeFile "TlsTest/Rfc8448.lean" out
  IO.println "ok"
  return 0
