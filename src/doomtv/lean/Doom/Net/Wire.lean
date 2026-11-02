import Doom.Net.Bytes

/-!
# Doom.Net.Wire

Datagram formats. All integers are big-endian. Every datagram starts with an
8-byte header:

```
0  'L' (0x4C)
1  'D' (0x44)
2  protocol version (PROTOCOL_VERSION)
3  message type
4  session id u32 (0 in HELLO/REJECT; host-chosen afterwards)
```

Bodies (after the header):

```
1 HELLO   joiner→host  nonce u32 | dlen u8 ≤32 | digest[dlen] | nlen u8 ≤16 | name[nlen]
2 LOBBY   host→joiner  nonce u32 | yourSlot u8 | mask u8 | per set bit p: nlen u8 | name
3 REJECT  host→joiner  nonce u32 | reason u8 (1 version,2 iwad,3 full,4 started) | hostVersion u8
4 START   host→joiner  nonce u32 | episode | map | skill | mode | flags (b0 nomonsters,
                       b1 respawn, b2 fast) | playerMask | consoleplayer | inputDelay
                       | startTic u32 | per player in mask: nlen u8 | name
5 CMDS    joiner→host  slot u8 | setAck u32 | firstTic u32 | n u8 ≤64 | n × cmd(5)
                       | sumFirst u32 | m u8 ≤64 | m × u32
6 SETS    host→joiner  cmdAck u32 | sumAck u32 | ahead i8 | firstTic u32 | n u8 ≤48
                       | n × (mask u8 1..15 | popcount(mask) × cmd(5), ascending slot)
                       | d u8 ≤4 | d × (slot u8 | tic u32 | reason u8)
                       | desyncTic u32 (0xFFFFFFFF none) | desyncMask u8
7 LEAVE   either       slot u8 | reason u8 (1 timeout, 2 quit, 3 kicked)
```

`cmd(5)` is the packed `TicCmd` of `Doom.Net.Bytes`. Decoding is strict and
total: wrong magic, unknown type, out-of-range field, non-printable name,
inconsistent count or trailing byte rejects the datagram, so every accepted
datagram re-encodes to the same bytes. Datagrams over `MAX_PACKET` bytes are
rejected before parsing. The only cross-version guarantee is the HELLO prefix
(header + nonce) and the whole REJECT layout, so a host can tell a joiner of
another version why it was refused.
-/

namespace Doom.Net

def MAGIC0 : UInt8 := 0x4C
def MAGIC1 : UInt8 := 0x44
def HEADER_BYTES : Nat := 8

def T_HELLO : UInt8 := 1
def T_LOBBY : UInt8 := 2
def T_REJECT : UInt8 := 3
def T_START : UInt8 := 4
def T_CMDS : UInt8 := 5
def T_SETS : UInt8 := 6
def T_LEAVE : UInt8 := 7

/-- Joiner → host command stream (with redundancy) and checksum reports. -/
structure CmdsMsg where
  slot : UInt8
  /-- The joiner holds every tic set below this tic. -/
  setAck : UInt32
  firstTic : UInt32
  count : Nat
  /-- `count` packed commands for tics `firstTic ..`. -/
  cmds : ByteArray
  sumFirst : UInt32
  sumCount : Nat
  /-- `sumCount` big-endian checksums for tics `sumFirst ..`. -/
  sums : ByteArray
  deriving BEq, Inhabited

/-- Host → joiner merged tic sets, acknowledgements and game events. -/
structure SetsMsg where
  /-- The host holds every command of the recipient below this tic. -/
  cmdAck : UInt32
  /-- The host holds every checksum of the recipient below this tic. -/
  sumAck : UInt32
  /-- Recipient's command stream minus the slowest player's, int8 two's complement. -/
  ahead : UInt8
  firstTic : UInt32
  count : Nat
  /-- `count` encoded sets (mask, then the commands of the players in it). -/
  body : ByteArray
  drops : Array DropInfo
  desyncTic : UInt32
  desyncMask : UInt8
  deriving BEq, Inhabited

inductive Msg where
  | hello (nonce : UInt32) (digest : ByteArray) (name : String)
  /-- `names` has 4 entries; `mask` says which slots are occupied (absent names are ""). -/
  | lobby (nonce : UInt32) (yourSlot : UInt8) (mask : UInt8) (names : Array String)
  | reject (nonce : UInt32) (reason : RejectReason) (hostVersion : UInt8)
  | start (nonce : UInt32) (info : StartInfo)
  | cmds (m : CmdsMsg)
  | sets (m : SetsMsg)
  | leave (slot : UInt8) (reason : DropReason)
  deriving BEq, Inhabited

inductive Decoded where
  | ok (session : UInt32) (msg : Msg)
  /-- A well-formed HELLO prefix from a different protocol version. -/
  | wrongVersion (version : UInt8) (nonce : UInt32)
  | invalid
  deriving BEq, Inhabited

/-! ## Encoding -/

private def header (ty : UInt8) (session : UInt32) (cap : Nat) : ByteArray :=
  wr32 ((((ByteArray.emptyWithCapacity cap).push MAGIC0).push MAGIC1).push PROTOCOL_VERSION |>.push ty)
    session

private def pushName (b : ByteArray) (s : String) : ByteArray :=
  let nb := nameBytes s
  appendRange (b.push nb.size.toUInt8) nb 0 nb.size

private def flagsOf (g : GameSettings) : UInt8 :=
  (if g.nomonsters then 1 else 0) ||| (if g.respawn then 2 else 0) ||| (if g.fast then 4 else 0)

/-- Size in bytes of the set encodings for one tic with player mask `m`. -/
@[inline] def setBytes (m : UInt8) : Nat := 1 + CMD_BYTES * popcount4 m

def encode (session : UInt32) : Msg → ByteArray
  | .hello nonce digest name =>
    let dl := min digest.size DIGEST_MAX
    let b := wr32 (header T_HELLO session 64) nonce
    let b := appendRange (b.push dl.toUInt8) digest 0 dl
    pushName b name
  | .lobby nonce yourSlot mask names => Id.run do
    let m := mask &&& 0x0f
    let mut b := wr32 (header T_LOBBY session 96) nonce
    b := (b.push yourSlot).push m
    for p in [0:MAX_PLAYERS] do
      if bit m p then b := pushName b (names.getD p "")
    return b
  | .reject nonce reason ver =>
    (wr32 (header T_REJECT session 16) nonce |>.push reason.toByte).push ver
  | .start nonce info => Id.run do
    let g := info.settings
    let m := info.playerMask &&& 0x0f
    let mut b := wr32 (header T_START session 96) nonce
    b := b.push g.episode |>.push g.map |>.push g.skill |>.push g.deathmatch
      |>.push (flagsOf g) |>.push m |>.push info.consoleplayer |>.push info.inputDelay
    b := wr32 b g.startTic
    for p in [0:MAX_PLAYERS] do
      if bit m p then b := pushName b (info.names.getD p "")
    return b
  | .cmds c =>
    let n := min c.count MAX_CMDS_PER_PACKET
    let k := min c.sumCount MAX_SUMS_PER_PACKET
    let b := header T_CMDS session (HEADER_BYTES + 19 + n * CMD_BYTES + 4 * k)
    let b := wr32 (wr32 (b.push c.slot) c.setAck) c.firstTic
    let b := appendRange (b.push n.toUInt8) c.cmds 0 (n * CMD_BYTES)
    let b := wr32 b c.sumFirst
    appendRange (b.push k.toUInt8) c.sums 0 (4 * k)
  | .sets s => Id.run do
    let n := min s.count MAX_SETS_PER_PACKET
    let mut b := header T_SETS session (HEADER_BYTES + 23 + s.body.size + 30)
    b := wr32 (wr32 b s.cmdAck) s.sumAck
    b := wr32 (b.push s.ahead) s.firstTic
    b := b.push n.toUInt8
    -- copy exactly `n` well-formed sets from the body
    let mut pos := 0
    for _ in [0:n] do
      let m := rd8 s.body pos &&& 0x0f
      let len := setBytes m
      b := b.push m
      b := appendRange b s.body (pos + 1) (len - 1)
      pos := pos + len
    let d := min s.drops.size MAX_PLAYERS
    b := b.push d.toUInt8
    for i in [0:d] do
      let x := s.drops.getD i default
      b := wr32 (b.push x.slot) x.tic |>.push x.reason.toByte
    b := wr32 b s.desyncTic
    return b.push s.desyncMask
  | .leave slot reason =>
    (header T_LEAVE session 10 |>.push slot).push reason.toByte

/-! ## Decoding -/

/-- Parse `count` length-prefixed names for the bits of `mask`, from `pos`. -/
private def readNames? (b : ByteArray) (pos0 : Nat) (mask : UInt8) :
    Option (Array String × Nat) := Id.run do
  let mut pos := pos0
  let mut names : Array String := #["", "", "", ""]
  for p in [0:MAX_PLAYERS] do
    if bit mask p then
      let n := (rd8 b pos).toNat
      match readName? b (pos + 1) n with
      | none => return none
      | some s =>
        if pos + 1 + n > b.size then return none
        names := names.setIfInBounds p s
        pos := pos + 1 + n
  return some (names, pos)

private def decodeHello (b : ByteArray) : Option Msg := do
  let nonce := rd32 b 8
  let dl := (rd8 b 12).toNat
  guard (dl ≤ DIGEST_MAX)
  guard (13 + dl + 1 ≤ b.size)
  let digest := b.extract 13 (13 + dl)
  let nl := (rd8 b (13 + dl)).toNat
  let name ← readName? b (14 + dl) nl
  guard (14 + dl + nl == b.size)
  pure (.hello nonce digest name)

private def decodeLobby (b : ByteArray) : Option Msg := do
  guard (b.size ≥ 14)
  let nonce := rd32 b 8
  let you := rd8 b 12
  let mask := rd8 b 13
  guard (mask != 0 && mask ≤ 0x0f && you < 4 && bit mask you.toNat)
  let (names, pos) ← readNames? b 14 mask
  guard (pos == b.size)
  pure (.lobby nonce you mask names)

private def decodeReject (b : ByteArray) : Option Msg := do
  guard (b.size == 14)
  let r ← RejectReason.ofByte? (rd8 b 12)
  pure (.reject (rd32 b 8) r (rd8 b 13))

private def decodeStart (b : ByteArray) : Option Msg := do
  guard (b.size ≥ 24)
  let flags := rd8 b 16
  let mask := rd8 b 17
  let cp := rd8 b 18
  let delay := rd8 b 19
  let g : GameSettings := {
    episode := rd8 b 12, map := rd8 b 13, skill := rd8 b 14, deathmatch := rd8 b 15
    nomonsters := flags &&& 1 != 0, respawn := flags &&& 2 != 0, fast := flags &&& 4 != 0
    startTic := rd32 b 20 }
  guard (g.valid && flags ≤ 7 && mask != 0 && mask ≤ 0x0f && cp < 4 && bit mask cp.toNat
    && delay.toNat ≤ MAX_INPUT_DELAY)
  let (names, pos) ← readNames? b 24 mask
  guard (pos == b.size)
  pure (.start (rd32 b 8) {
    settings := g, playerMask := mask, consoleplayer := cp, inputDelay := delay, names })

private def decodeCmds (b : ByteArray) : Option Msg := do
  guard (b.size ≥ 23)
  let slot := rd8 b 8
  guard (slot < 4)
  let first := rd32 b 13
  let n := (rd8 b 17).toNat
  guard (n ≤ MAX_CMDS_PER_PACKET && first.toNat + n ≤ 0x100000000)
  let p := 18 + n * CMD_BYTES
  guard (p + 5 ≤ b.size)
  let sumFirst := rd32 b p
  let k := (rd8 b (p + 4)).toNat
  guard (k ≤ MAX_SUMS_PER_PACKET && sumFirst.toNat + k ≤ 0x100000000)
  guard (p + 5 + 4 * k == b.size)
  pure (.cmds {
    slot, setAck := rd32 b 9, firstTic := first, count := n
    cmds := b.extract 18 p, sumFirst, sumCount := k, sums := b.extract (p + 5) b.size })

private def decodeSets (b : ByteArray) : Option Msg := do
  guard (b.size ≥ 22)
  let first := rd32 b 17
  let n := (rd8 b 21).toNat
  guard (n ≤ MAX_SETS_PER_PACKET && first.toNat + n ≤ 0x100000000)
  -- walk the sets
  let walk : Option Nat := Id.run do
    let mut pos := 22
    for _ in [0:n] do
      let m := rd8 b pos
      if m == 0 || m > 0x0f || pos ≥ b.size then return none
      pos := pos + setBytes m
    return some pos
  let bodyEnd ← walk
  guard (bodyEnd + 1 ≤ b.size)
  let d := (rd8 b bodyEnd).toNat
  guard (d ≤ MAX_PLAYERS && bodyEnd + 1 + 6 * d + 5 == b.size)
  let drops : Option (Array DropInfo) := Id.run do
    let mut out : Array DropInfo := #[]
    for i in [0:d] do
      let o := bodyEnd + 1 + 6 * i
      let slot := rd8 b o
      match DropReason.ofByte? (rd8 b (o + 5)) with
      | some r => if slot < 4 then out := out.push { slot, tic := rd32 b (o + 1), reason := r }
                  else return none
      | none => return none
    return some out
  let drops ← drops
  let dpos := bodyEnd + 1 + 6 * d
  let dTic := rd32 b dpos
  let dMask := rd8 b (dpos + 4)
  guard (dMask ≤ 0x0f && (dTic != NO_TIC || dMask == 0))
  pure (.sets {
    cmdAck := rd32 b 8, sumAck := rd32 b 12, ahead := rd8 b 16, firstTic := first
    count := n, body := b.extract 22 bodyEnd, drops, desyncTic := dTic, desyncMask := dMask })

private def decodeLeave (b : ByteArray) : Option Msg := do
  guard (b.size == 10)
  let slot := rd8 b 8
  guard (slot < 4)
  let r ← DropReason.ofByte? (rd8 b 9)
  pure (.leave slot r)

/-- Strict, total decoder. -/
def decode (b : ByteArray) : Decoded :=
  if b.size < HEADER_BYTES || b.size > MAX_PACKET || rd8 b 0 != MAGIC0 || rd8 b 1 != MAGIC1 then
    .invalid
  else
    let ver := rd8 b 2
    let ty := rd8 b 3
    let session := rd32 b 4
    if ver != PROTOCOL_VERSION then
      if ty == T_HELLO && b.size ≥ 12 then .wrongVersion ver (rd32 b 8)
      else if ty == T_REJECT then
        match decodeReject b with
        | some m => .ok session m
        | none => .invalid
      else .invalid
    else
      let r : Option Msg :=
        if ty == T_HELLO then decodeHello b
        else if ty == T_LOBBY then decodeLobby b
        else if ty == T_REJECT then decodeReject b
        else if ty == T_START then decodeStart b
        else if ty == T_CMDS then decodeCmds b
        else if ty == T_SETS then decodeSets b
        else if ty == T_LEAVE then decodeLeave b
        else none
      match r with
      | some m => .ok session m
      | none => .invalid

end Doom.Net
