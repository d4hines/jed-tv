import Yt.Tls.Client
import Std.Internal.UV.TCP
import Std.Internal.UV.DNS
/-! Host-only test driver (upstream Lean runtime; never built for the TV):
runs the pure `Yt.Tls` client over a real TCP socket, sends one HTTP/1.1
request and reports what happened. Optionally records the session
(entropy, time, every byte received and sent) as a Lean module so that it
can be replayed deterministically by the transcript test.

    tlsprobe HOST PORT [--sni NAME] [--time UNIX] [--ca CERT.der]...
             [--path /] [--suite 1301|1302|1303]... [--record FILE.lean NAME]
             [--record-data] [--max BYTES] [--echo]

The recording keeps the received records up to the end of the handshake
(with `--record-data`, through the first application data record).
-/
open Std.Internal.UV Std.Net Yt.Crypto Yt.Tls

def await (act : IO (IO.Promise (Except IO.Error α))) : IO α := do
  let p ← act
  match ← IO.wait p.result? with
  | some (.ok a) => pure a
  | some (.error e) => throw e
  | none => throw (IO.userError "promise dropped")

structure Opts where
  host : String := ""
  port : Nat := 443
  sni : String := ""
  time : UInt64 := 0
  cas : Array String := #[]
  path : String := "/"
  suites : Array Suite := #[]
  record : String := ""
  recordName : String := ""
  maxBytes : Nat := 1000000
  recordData : Bool := false
  echo : Bool := false

partial def parseArgs (o : Opts) : List String → IO Opts
  | [] => pure o
  | "--sni" :: v :: r => parseArgs { o with sni := v } r
  | "--time" :: v :: r => parseArgs { o with time := v.toNat!.toUInt64 } r
  | "--ca" :: v :: r => parseArgs { o with cas := o.cas.push v } r
  | "--path" :: v :: r => parseArgs { o with path := v } r
  | "--suite" :: v :: r =>
    let s := match v with | "1302" => Suite.aes256gcm | "1303" => .chacha20 | _ => .aes128gcm
    parseArgs { o with suites := o.suites.push s } r
  | "--record" :: f :: n :: r => parseArgs { o with record := f, recordName := n } r
  | "--max" :: v :: r => parseArgs { o with maxBytes := v.toNat! } r
  | "--record-data" :: r => parseArgs { o with recordData := true } r
  | "--echo" :: r => parseArgs { o with echo := true } r
  | h :: p :: r => if o.host == "" then parseArgs { o with host := h, port := p.toNat! } r
                   else throw (IO.userError s!"unexpected argument {h}")
  | [x] => throw (IO.userError s!"unexpected argument {x}")

def statusName : Status → String
  | .handshaking => "handshaking"
  | .connected => "connected"
  | .closed => "closed"
  | .failed r => "failed: " ++ r

def hexLines (b : ByteArray) : String := Id.run do
  let h := toHex b
  let mut s := ""
  let mut i := 0
  while i < h.length do
    s := s ++ "\n  " ++ (h.drop i |>.take 120).toString
    i := i + 120
  s

def main (args : List String) : IO UInt32 := do
  let o ← parseArgs {} args
  let sni := if o.sni == "" then o.host else o.sni
  let mut anchors := Yt.X509.defaultAnchors
  if o.cas.size > 0 then
    anchors := #[]
    for f in o.cas do
      match Yt.X509.Anchor.ofDer (← IO.FS.readBinFile f) with
      | some a => anchors := anchors.push a
      | none => throw (IO.userError s!"cannot parse {f}")
  let entropy ← IO.getRandomBytes 32
  let cfg : Config := { serverName := sni, unixTime := o.time, entropy := entropy, anchors := anchors,
                        suites := if o.suites.size > 0 then o.suites else defaultSuites }
  -- resolve and connect
  let addrs ← await (DNS.getAddrInfo o.host (toString o.port) 0)
  let some ip := addrs.find? (fun a => match a with | .v4 _ => true | _ => false) | throw (IO.userError "no IPv4 address")
  let addr : SocketAddress := match ip with
    | .v4 a => .v4 ⟨a, o.port.toUInt16⟩
    | .v6 a => .v6 ⟨a, o.port.toUInt16⟩
  let sock ← TCP.Socket.new
  let t0 ← IO.monoMsNow
  await (sock.connect addr)
  let tConn ← IO.monoMsNow
  let (c, hello) := start cfg
  let mut c := c
  let mut received := ByteArray.empty
  let mut sentAll := hello
  await (sock.send #[hello])
  let mut requested := false
  let mut body := ByteArray.empty
  let mut tHs : Nat := 0
  let mut eof := false
  let mut status := c.status
  -- bytes fed so far (whole records); where the handshake and the first
  -- application data record ended
  let mut fed := 0
  let mut hsEnd := 0
  let mut dataEnd := 0
  while !eof do
    let chunk? ← await (sock.recv? 65536)
    match chunk? with
    | none => eof := true
    | some chunk =>
      received := received ++ chunk
      -- feed one whole record at a time
      while fed + 5 ≤ received.size && !eof do
        let n := 5 + ((received[fed+3]!.toNat <<< 8) ||| received[fed+4]!.toNat)
        if fed + n > received.size then break
        let r := feed c (received.extract fed (fed + n))
        fed := fed + n
        if !(o.echo && r.data.size > 0 && r.status == .connected) then c := r.client
        status := r.status
        body := body ++ r.data
        if r.data.size > 0 && dataEnd == 0 then dataEnd := fed
        if r.send.size > 0 then
          sentAll := sentAll ++ r.send
          await (sock.send #[r.send])
        if o.echo && r.data.size > 0 && r.status == .connected then
          let (c', recs) := send r.client "ack\n".toUTF8
          c := c'
          sentAll := sentAll ++ recs
          await (sock.send #[recs])
        match r.status with
        | .connected =>
          if !requested then
            hsEnd := fed
            tHs ← IO.monoMsNow
            let req := s!"GET {o.path} HTTP/1.1\r\nHost: {sni}\r\nUser-Agent: yt-lean-tls-probe\r\nAccept: */*\r\nConnection: close\r\n\r\n"
            let (c', recs) := send c req.toUTF8
            c := c'
            sentAll := sentAll ++ recs
            await (sock.send #[recs])
            requested := true
          if body.size ≥ o.maxBytes then eof := true
        | .handshaking => pure ()
        | _ => eof := true
  let tEnd ← IO.monoMsNow
  let (_, bye) := close c
  if bye.size > 0 && status == .connected then
    try await (sock.send #[bye]) catch _ => pure ()
  -- report
  IO.println s!"host {o.host}:{o.port} sni {sni}"
  IO.println s!"status {statusName status}"
  IO.println s!"suite {c.cipherSuite} alpn {c.protocol} certs {c.certs.size} hrr {c.hrr}"
  IO.println s!"tcp connect {tConn - t0} ms, handshake {if tHs > 0 then tHs - tConn else 0} ms"
  IO.println s!"received {received.size} bytes, plaintext {body.size} bytes in {tEnd - tConn} ms"
  let text := String.fromUTF8? (body.extract 0 (min body.size 400)) |>.getD "(binary)"
  let firstLine := (text.splitOn "\r\n").headD ""
  IO.println s!"response: {firstLine}"
  if o.record != "" then
    let mod := s!"/-! A recorded session from `tlsprobe` (test/tls/Probe.lean): {o.host}:{o.port}, " ++
      s!"SNI {sni}. Generated; do not edit. -/\nnamespace TlsTest.{o.recordName}\n" ++
      s!"def serverName : String := \"{sni}\"\ndef unixTime : UInt64 := {o.time}\n" ++
      s!"def entropy : String := \"{toHex entropy}\"\n" ++
      s!"def received : String := \"{hexLines (received.extract 0 (if o.recordData then dataEnd else hsEnd))}\"\n" ++
      s!"def sent : String := \"{hexLines sentAll}\"\n" ++
      s!"def request : String := \"GET {o.path} HTTP/1.1\\r\\nHost: {sni}\\r\\nUser-Agent: yt-lean-tls-probe\\r\\nAccept: */*\\r\\nConnection: close\\r\\n\\r\\n\"\n" ++
      s!"end TlsTest.{o.recordName}\n"
    IO.FS.writeFile o.record mod
    IO.println s!"recorded to {o.record}"
  return (if status == .connected || status == .closed then 0 else 1)
