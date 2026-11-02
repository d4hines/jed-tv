import Yt.Platform

/-! Nonblocking TCP connections driven by poll(2): a send queue and a
receive buffer per connection; the protocol on top (CONNECT proxy, TLS,
HTTP) consumes `inbox` and queues bytes with `send`. Single-threaded: the
event loop polls all connections and the remote together. -/
namespace Yt.Net
open Yt.Platform

def POLLIN : UInt16 := 1
def POLLOUT : UInt16 := 4
def POLLERR : UInt16 := 8
def POLLHUP : UInt16 := 0x10

inductive State where
  | connecting | connected | closed | failed (errno : Nat)
  deriving BEq, Inhabited

structure Conn where
  fd : USize
  state : State := .connecting
  out : ByteArray := .empty
  outOff : Nat := 0
  inbox : ByteArray := .empty
  eof : Bool := false
  deriving Inhabited

/-- An IPv4 address: four bytes, or none if `s` isn't a dotted quad. -/
def parseIPv4 (s : String) : Option (UInt8 × UInt8 × UInt8 × UInt8) := Id.run do
  let b := s.toUTF8
  let mut parts : Array Nat := #[]
  let mut cur := 0
  let mut digits := 0
  for c in b do
    if c == 46 then
      if digits == 0 then return none
      parts := parts.push cur
      cur := 0
      digits := 0
    else if c ≥ 48 && c ≤ 57 then
      cur := cur * 10 + (c - 48).toNat
      digits := digits + 1
      if cur > 255 then return none
    else return none
  if digits == 0 then return none
  parts := parts.push cur
  if parts.size != 4 then return none
  return some (parts[0]!.toUInt8, parts[1]!.toUInt8, parts[2]!.toUInt8, parts[3]!.toUInt8)

/-- Start connecting to `ip:port`. -/
def Conn.open (ip : UInt8 × UInt8 × UInt8 × UInt8) (port : UInt16) : BaseIO (Except Nat Conn) := do
  match ← Sys.tcpSocket with
  | .error e => return .error e
  | .ok fd =>
    -- TCP_NODELAY: requests are small and latency matters
    let _ ← Sys.setsockoptInt fd 6 1 1
    let (a, b, c, d) := ip
    match ← Sys.connect fd (Sys.sockaddrIn a b c d port) with
    | .error e => Sys.close fd; return .error e
    | .ok () => return .ok { fd }

/-- Queue bytes to send. -/
def Conn.send (c : Conn) (b : ByteArray) : Conn :=
  if c.outOff == c.out.size then { c with out := b, outOff := 0 }
  else { c with out := c.out ++ b }

/-- The poll(2) events this connection waits for. -/
def Conn.events (c : Conn) : UInt16 :=
  match c.state with
  | .connecting => POLLOUT
  | .connected => POLLIN ||| (if c.outOff < c.out.size then POLLOUT else 0)
  | _ => 0

def Conn.isOpen (c : Conn) : Bool := c.state == .connecting || c.state == .connected

/-- Write what the socket accepts. -/
partial def Conn.flush (c : Conn) : BaseIO Conn := do
  if c.outOff ≥ c.out.size then return { c with out := .empty, outOff := 0 }
  match ← Sys.write c.fd c.out c.outOff.toUSize (c.out.size - c.outOff).toUSize with
  | .ok n => if n == 0 then return c else Conn.flush { c with outOff := c.outOff + n.toNat }
  | .error 11 => return c
  | .error e => return { c with state := .failed e }

/-- Read what has arrived (EOF sets `eof`). -/
partial def Conn.fill (c : Conn) : BaseIO Conn := do
  match ← Sys.read c.fd 65536 with
  | .ok b =>
    if b.size == 0 then return { c with eof := true }
    let c := { c with inbox := c.inbox ++ b }
    if b.size == 65536 then Conn.fill c else return c
  | .error 11 => return c
  | .error e => return { c with state := .failed e }

/-- Handle poll(2)'s `revents` for this connection. -/
def Conn.ready (c : Conn) (revents : UInt16) : BaseIO Conn := do
  if revents == 0 then return c
  match c.state with
  | .connecting =>
    -- connect finished: SO_ERROR says how
    let err ← Sys.getsockoptInt c.fd 1 4
    if isErr err || err != 0 then
      return { c with state := .failed (if isErr err then errnoOf err else err.toNat) }
    Conn.flush { c with state := .connected }
  | .connected =>
    let c ← if revents &&& (POLLIN ||| POLLHUP ||| POLLERR) != 0 then Conn.fill c else pure c
    if c.outOff < c.out.size then Conn.flush c else return c
  | _ => return c

/-- Take `n` bytes from the front of the inbox. -/
def Conn.take (c : Conn) (n : Nat) : Conn × ByteArray :=
  ({ c with inbox := c.inbox.extract n c.inbox.size }, c.inbox.extract 0 n)

def Conn.close (c : Conn) : BaseIO Conn := do
  if c.isOpen then Sys.close c.fd
  return { c with state := .closed }

/-- Wait up to `timeoutMs` for any of `conns`; each connection handled. -/
def pollAll (conns : Array Conn) (timeoutMs : Nat) : BaseIO (Array Conn) := do
  let mut fds := ByteArray.emptyWithCapacity (8 * conns.size)
  for c in conns do
    let fd := c.fd.toUInt32
    let ev := c.events
    -- struct pollfd { int fd; short events; short revents; }, little-endian
    fds := fds.push fd.toUInt8 |>.push (fd >>> 8).toUInt8 |>.push (fd >>> 16).toUInt8 |>.push (fd >>> 24).toUInt8
    fds := fds.push ev.toUInt8 |>.push (ev >>> 8).toUInt8 |>.push 0 |>.push 0
  match ← Sys.poll fds timeoutMs.toUSize with
  | .error _ => return conns
  | .ok got =>
    let mut out := conns
    for i in [0:conns.size] do
      let rev := got[8 * i + 6]!.toUInt16 ||| (got[8 * i + 7]!.toUInt16 <<< 8)
      out := out.set! i (← conns[i]!.ready rev)
    return out

end Yt.Net
