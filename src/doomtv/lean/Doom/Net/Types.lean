/-!
# Doom.Net.Types

Shared vocabulary for the multiplayer session: addresses, protocol constants,
configuration, game settings and the reasons a player can be refused, dropped or
leave. Everything here is plain data; see `Doom.Net.Session` for the protocol.
-/

namespace Doom.Net

/-- An IPv4 endpoint. `ip` is the address in host order (`10.0.0.2` = `0x0A000002`). -/
structure Addr where
  ip : UInt32
  port : UInt16
  deriving BEq, DecidableEq, Repr, Inhabited

def Addr.toString (a : Addr) : String :=
  s!"{a.ip >>> 24}.{(a.ip >>> 16) &&& 0xff}.{(a.ip >>> 8) &&& 0xff}.{a.ip &&& 0xff}:{a.port}"

instance : ToString Addr := ⟨Addr.toString⟩

/-- Fixed UDP port for every peer. -/
def PORT : UInt16 := 2342
/-- Wire protocol version. Bump on any layout change; HELLO/REJECT prefixes stay fixed. -/
def PROTOCOL_VERSION : UInt8 := 1
/-- Largest datagram we build or accept. -/
def MAX_PACKET : Nat := 1400
def MAX_PLAYERS : Nat := 4
/-- Tic ring capacity (power of two). Bounds every per-tic buffer. -/
def RING : Nat := 64
def RING32 : UInt32 := 64
def RING_MASK : UInt32 := 63
def MAX_CMDS_PER_PACKET : Nat := 64
def MAX_SETS_PER_PACKET : Nat := 48
def MAX_SUMS_PER_PACKET : Nat := 64
def NAME_MAX : Nat := 16
def DIGEST_MAX : Nat := 32
/-- Bound on queued out-of-band replies (rejections, kicks, leave notices). -/
def OUTBOX_MAX : Nat := 16
/-- Wire value meaning "no tic". -/
def NO_TIC : UInt32 := 0xFFFFFFFF
/-- Largest accepted input delay / lead window, in tics. -/
def MAX_INPUT_DELAY : Nat := 8
def MAX_LEAD : Nat := 32

/-- Why the host refused a HELLO. -/
inductive RejectReason where
  | version
  | iwad
  | full
  | started
  deriving BEq, DecidableEq, Repr, Inhabited

def RejectReason.toByte : RejectReason → UInt8
  | .version => 1 | .iwad => 2 | .full => 3 | .started => 4

def RejectReason.ofByte? : UInt8 → Option RejectReason
  | 1 => some .version | 2 => some .iwad | 3 => some .full | 4 => some .started
  | _ => none

/-- Why a player left a running game (also the LEAVE reason byte). -/
inductive DropReason where
  /-- Host heard nothing for `dropTimeoutMs`. -/
  | timeout
  /-- The player announced it was leaving. -/
  | quit
  /-- The host removed the player (sent to a dropped peer that keeps talking). -/
  | kicked
  deriving BEq, DecidableEq, Repr, Inhabited

def DropReason.toByte : DropReason → UInt8
  | .timeout => 1 | .quit => 2 | .kicked => 3

def DropReason.ofByte? : UInt8 → Option DropReason
  | 1 => some .timeout | 2 => some .quit | 3 => some .kicked
  | _ => none

/-- Player `slot` is absent from every tic set at or after `tic`. -/
structure DropInfo where
  slot : UInt8
  tic : UInt32
  reason : DropReason
  deriving BEq, Repr, Inhabited

/-- First tic at which the host saw differing checksums, and from which slots. -/
structure Desync where
  tic : UInt32
  /-- Bit `p` set: player `p`'s checksum differed from the host's. -/
  slots : UInt8
  deriving BEq, Repr, Inhabited

/-- Why the local session stopped. -/
inductive EndReason where
  /-- The host refused our HELLO. -/
  | rejected (r : RejectReason)
  /-- No answer from the host within `joinTimeoutMs`. -/
  | noResponse
  /-- The host went silent past the timeout. -/
  | hostLost
  /-- The host announced it left. -/
  | hostQuit
  /-- The host removed us from the game. -/
  | dropped
  /-- `Session.leave` was called locally. -/
  | localQuit
  deriving BEq, Repr, Inhabited

def EndReason.describe : EndReason → String
  | .rejected .version => "host runs a different protocol version"
  | .rejected .iwad => "host uses a different IWAD"
  | .rejected .full => "game is full"
  | .rejected .started => "game already started"
  | .noResponse => "no response from host"
  | .hostLost => "lost connection to host"
  | .hostQuit => "host ended the game"
  | .dropped => "removed from the game by the host"
  | .localQuit => "left the game"

/-- Game options owned by the host and sent in START. -/
structure GameSettings where
  episode : UInt8 := 1
  map : UInt8 := 1
  /-- `sk_baby` = 0 .. `sk_nightmare` = 4. -/
  skill : UInt8 := 2
  /-- 0 coop, 1 deathmatch, 2 altdeath. -/
  deathmatch : UInt8 := 0
  nomonsters : Bool := false
  respawn : Bool := false
  fast : Bool := false
  /-- Number of the first game tic. -/
  startTic : UInt32 := 0
  deriving BEq, Repr, Inhabited

def GameSettings.valid (g : GameSettings) : Bool :=
  g.episode ≤ 9 && g.map ≤ 99 && g.skill ≤ 4 && g.deathmatch ≤ 2
    && g.startTic < 0x10000000

/-- Everything a peer needs to set up the level, as carried by START. -/
structure StartInfo where
  settings : GameSettings
  /-- Bit `p` set: player `p` is in the game at `startTic`. -/
  playerMask : UInt8
  /-- The receiving peer's player number. -/
  consoleplayer : UInt8
  inputDelay : UInt8
  /-- Player names indexed by player number; `""` for absent players. -/
  names : Array String
  deriving BEq, Repr, Inhabited

/--
Session configuration supplied by the caller. Times are milliseconds on the
caller's monotonic clock.
-/
structure Config where
  /-- Shown in lobbies; reduced to at most 16 printable ASCII bytes. -/
  name : String := "player"
  /-- Caller-computed IWAD identity (at most 32 bytes are used); joiners must match the host. -/
  iwadDigest : ByteArray := .empty
  /-- Caller randomness: the host's session id, or the joiner's HELLO nonce. -/
  nonce : UInt32 := 1
  /-- Local commands are scheduled this many tics ahead of the tic being run (host decides). -/
  inputDelay : Nat := 2
  /-- Most local commands that may be outstanding (generated, not yet run). -/
  maxLead : Nat := 6
  /-- Silence after which the host drops a player, or a joiner gives up on the host. -/
  dropTimeoutMs : UInt64 := 8000
  /--
  Silence timeout used instead of `dropTimeoutMs` until the first tic with real
  (non-prefill) commands from everyone, covering level loads after START.
  -/
  loadTimeoutMs : UInt64 := 30000
  /-- Joining: give up if the host never answers; lobby: silence timeout. -/
  joinTimeoutMs : UInt64 := 5000
  /-- Resend interval while data is unacknowledged. -/
  resendMs : UInt64 := 25
  /-- Idle keepalive interval (also lobby refresh). -/
  keepaliveMs : UInt64 := 250
  /-- HELLO interval while joining / in the lobby. -/
  helloMs : UInt64 := 250
  /-- START resend interval until the joiner's first command packet arrives. -/
  startResendMs : UInt64 := 100
  deriving Inhabited

/-- Printable ASCII only, at most `NAME_MAX` bytes; other characters become `?`. -/
def sanitizeName (s : String) : String := Id.run do
  let mut out := ""
  let mut n := 0
  for c in s.toList do
    if n ≥ NAME_MAX then break
    let v := c.toNat
    out := out.push (if 0x20 ≤ v && v ≤ 0x7e then c else '?')
    n := n + 1
  return out

end Doom.Net
