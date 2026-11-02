import Std.Time

/-!
Interop: the Lean TLS client (the host-only `tlsprobe` driver, Probe.lean)
against a local `openssl s_server` (TLS 1.3 only) with a throwaway test PKI:

- ECDSA: P-384 root -> P-256 intermediate -> P-256 leaf (localhost)
- RSA: RSA-4096 root -> RSA-2048 intermediate -> RSA-2048 leaf (localhost)

Every cipher suite, ALPN, a server KeyUpdate, and the expected failures
(wrong name, untrusted root, expired, wrong suite, no common group).
OpenSSL is the independent oracle: it makes the PKI and is the server.
(OpenSSL 4.1 sends no HelloRetryRequest to a client offering only x25519,
even with -stateless; HRR is covered by TlsTest/Hrr.lean instead.)

    cd test/lean/yt/tls && lake exe tlsinterop

Work files: assets/build/lean-tls/interop (the PKI is made once).
-/

open System

def run (cmd : String) (args : Array String) (cwd : Option FilePath := none) : IO String := do
  let o ← IO.Process.output { cmd, args, cwd }
  unless o.exitCode == 0 do
    throw <| IO.userError s!"{cmd} {" ".intercalate args.toList} failed:\n{o.stdout}{o.stderr}"
  return o.stdout

def contains (s sub : String) : Bool := (s.splitOn sub).length > 1

/-- The test PKI: for each kind a root, an intermediate and a localhost leaf. -/
def makePki (work : FilePath) : IO Unit := do
  let ext := [("ca", "basicConstraints=critical,CA:TRUE\nkeyUsage=critical,keyCertSign,cRLSign\nsubjectKeyIdentifier=hash\n"),
    ("inter", "basicConstraints=critical,CA:TRUE,pathlen:0\nkeyUsage=critical,keyCertSign,cRLSign\nextendedKeyUsage=serverAuth\n"),
    ("leaf", "basicConstraints=critical,CA:FALSE\nkeyUsage=critical,digitalSignature\nextendedKeyUsage=serverAuth\n" ++
      "subjectAltName=DNS:localhost,DNS:*.test.example\n")]
  for (n, body) in ext do IO.FS.writeFile (work / s!"{n}.ext") body
  IO.FS.writeFile (work / "root.cnf") ("[req]\ndistinguished_name=d\n[d]\n[v3]\n" ++ (ext.lookup "ca").get!)
  let ssl (args : Array String) := discard <| run "openssl" args work
  for kind in ["ec", "rsa"] do
    let key (role : String) := s!"{kind}-{role}.key"
    if kind == "ec" then
      for (role, curve) in [("root", "secp384r1"), ("inter", "prime256v1"), ("leaf", "prime256v1")] do
        ssl #["ecparam", "-name", curve, "-genkey", "-noout", "-out", key role]
    else
      for (role, bits) in [("root", "4096"), ("inter", "2048"), ("leaf", "2048")] do
        ssl #["genrsa", "-out", key role, bits]
    let dg := if kind == "ec" then "-sha384" else "-sha256"
    ssl #["req", "-x509", "-new", "-key", key "root", "-subj", s!"/O=Lean TLS Test/CN=Test Root {kind}", "-days", "3650",
      dg, "-extensions", "v3", "-config", "root.cnf", "-out", s!"{kind}-root.pem"]
    ssl #["req", "-new", "-key", key "inter", "-subj", s!"/O=Lean TLS Test/CN=Test Intermediate {kind}", "-out", s!"{kind}-inter.csr"]
    ssl #["x509", "-req", "-in", s!"{kind}-inter.csr", "-CA", s!"{kind}-root.pem", "-CAkey", key "root", "-CAcreateserial",
      "-days", "1825", dg, "-extfile", "inter.ext", "-out", s!"{kind}-inter.pem"]
    ssl #["req", "-new", "-key", key "leaf", "-subj", "/CN=localhost", "-out", s!"{kind}-leaf.csr"]
    ssl #["x509", "-req", "-in", s!"{kind}-leaf.csr", "-CA", s!"{kind}-inter.pem", "-CAkey", key "inter", "-CAcreateserial",
      "-days", "365", "-sha256", "-extfile", "leaf.ext", "-out", s!"{kind}-leaf.pem"]
    ssl #["x509", "-in", s!"{kind}-root.pem", "-outform", "der", "-out", s!"{kind}-root.der"]

/-- `openssl s_server` on `port` with its output in `log` (and stdin piped for
its interactive commands), once it accepts connections. -/
def server (work : FilePath) (port : Nat) (args : Array String) (log : String) :
    IO (IO.Process.Child ⟨.piped, .null, .null⟩) := do
  let quoted := " ".intercalate (args.toList.map fun a => "'" ++ a ++ "'")
  let script := s!"exec openssl s_server -accept 127.0.0.1:{port} -tls1_3 -naccept 1 {quoted} > '{log}' 2>&1"
  -- not a previous run's "ACCEPT"
  if ← (work / log).pathExists then IO.FS.removeFile (work / log)
  let child ← IO.Process.spawn
    { cmd := "sh", args := #["-c", script], cwd := work, stdin := .piped, stdout := .null, stderr := .null }
  for _ in [0:50] do
    if ← (work / log).pathExists then
      if contains (← IO.FS.readFile (work / log)) "ACCEPT" then break
    IO.sleep 100
  return child

/-- The probe's summary lines, indented. -/
def report (out : String) : IO Unit :=
  for l in out.splitOn "\n" do
    if ["status", "suite", "response", "handshake"].any (contains l ·) then IO.println s!"    {l}"

def main : IO UInt32 := do
  let root : FilePath := (← run "git" #["rev-parse", "--show-toplevel"]).trimAscii.toString
  let here := root / "test/lean/yt/tls"
  let work := root / "assets/build/lean-tls/interop"
  IO.FS.createDirAll work
  IO.print (← run "openssl" #["version"])
  unless ← (work / "rsa-leaf.pem").pathExists do makePki work
  discard <| run "lake" #["build", "tlsprobe"] here
  let probe := (here / ".lake/build/bin/tlsprobe").toString
  let now := toString (← Std.Time.Timestamp.now).toSecondsSinceUnixEpoch.val
  let port ← IO.mkRef 44330
  let passed ← IO.mkRef 0
  let failed ← IO.mkRef 0
  let tally (name : String) (ok : Bool) : IO Unit := do
    if ok then passed.modify (· + 1) else failed.modify (· + 1)
    IO.println s!"{if ok then "PASS" else "FAIL"} {name}"
  -- one connection: `s_server` with `sargs`, the probe with `pargs`; `ok`: should it succeed?
  let case (name : String) (ok : Bool) (sargs pargs : Array String) : IO Unit := do
    port.modify (· + 1)
    let p ← port.get
    let srv ← server work p (#["-www"] ++ sargs) s!"s_server-{name}.log"
    let o ← IO.Process.output
      { cmd := probe, args := #["127.0.0.1", toString p, "--sni", "localhost", "--time", now] ++ pargs, cwd := work }
    srv.kill
    discard srv.wait
    let good := (o.exitCode == 0) == ok
    tally name good
    if good then report (o.stdout ++ o.stderr)
    else IO.println s!"    probe exit {o.exitCode}:\n{o.stdout}{o.stderr}"
  let ec := #["-cert", "ec-leaf.pem", "-key", "ec-leaf.key", "-cert_chain", "ec-inter.pem"]
  let rsa := #["-cert", "rsa-leaf.pem", "-key", "rsa-leaf.key", "-cert_chain", "rsa-inter.pem"]
  for s in ["TLS_AES_128_GCM_SHA256", "TLS_AES_256_GCM_SHA384", "TLS_CHACHA20_POLY1305_SHA256"] do
    case s!"ecdsa-{s}" true (ec ++ #["-ciphersuites", s]) #["--ca", "ec-root.der"]
    case s!"rsa-{s}" true (rsa ++ #["-ciphersuites", s]) #["--ca", "rsa-root.der"]
  case "ecdsa-alpn" true (ec ++ #["-alpn", "http/1.1"]) #["--ca", "ec-root.der"]
  case "rsa-pss-sha384-only" true (rsa ++ #["-sigalgs", "rsa_pss_rsae_sha384"]) #["--ca", "rsa-root.der"]
  case "wildcard-name" true ec #["--ca", "ec-root.der", "--sni", "a.test.example"]
  case "wrong-name" false ec #["--ca", "ec-root.der", "--sni", "example.com"]
  case "wildcard-two-labels" false ec #["--ca", "ec-root.der", "--sni", "a.b.test.example"]
  case "untrusted-root" false ec #["--ca", "rsa-root.der"]
  case "default-anchors" false ec #[]
  case "expired" false ec #["--ca", "ec-root.der", "--time", toString (now.toNat! + 400 * 86400)]
  case "no-common-suite" false (ec ++ #["-ciphersuites", "TLS_AES_256_GCM_SHA384"]) #["--ca", "ec-root.der", "--suite", "1301"]
  case "missing-intermediate" false #["-cert", "ec-leaf.pem", "-key", "ec-leaf.key"] #["--ca", "ec-root.der"]
  case "p256-group-only" false (ec ++ #["-groups", "P-256"]) #["--ca", "ec-root.der"]
  -- Server-initiated KeyUpdate with update_requested (s_server's interactive
  -- "K" command): the client must update its read key, answer with its own
  -- KeyUpdate, decrypt the data sent after it, and its reply ("ack", sent
  -- under the new write key) must decrypt at the server.
  port.modify (· + 1)
  let p ← port.get
  let srv ← server work p (#["-msg"] ++ ec) "s_server-keyupdate.log"
  let probeArgs := #["127.0.0.1", toString p, "--sni", "localhost", "--time", now, "--ca", "ec-root.der", "--echo"]
  let probing ← IO.asTask (IO.Process.output { cmd := probe, args := probeArgs, cwd := work })
  for (wait, line) in [(1500, "K\n"), (500, "hello after key update\n"), (1000, "Q\n")] do
    IO.sleep wait
    srv.stdin.putStr line
    srv.stdin.flush
  let out ← match ← IO.wait probing with
    | .ok o => pure (o.stdout ++ o.stderr)
    | .error e => pure (toString e)
  IO.sleep 500
  srv.kill
  discard srv.wait
  let log ← IO.FS.readFile (work / "s_server-keyupdate.log")
  let logLines := log.splitOn "\n"
  tally "key-update" (contains out "response: hello after key update" && logLines.any (·.startsWith "ack") &&
    (log.splitOn "KeyUpdate").length - 1 ≥ 2)
  for l in logLines do
    if contains l "KeyUpdate" || l.startsWith "ack" then IO.println s!"    {l}"
  IO.println s!"interop: {← passed.get} passed, {← failed.get} failed"
  return if (← failed.get) == 0 then 0 else 1
