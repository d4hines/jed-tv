import NetworkBoot.Runtime
import Update.CryptoBounds

/- Execute the production controller with deterministic observations and an
   effect trace. Cryptography is modeled, not reimplemented by this test. -/
open LeanBoot LeanBoot.NetworkBoot

structure Scenario where
  busy : Bool := false
  reset : Status := .ok
  start : Status := .ok
  receive : Status := .ok
  stop : Status := .ok
  empty : Bool := false
  signature : Bool := true
  digest : Bool := true
  copiedDigest : Bool := true
  copy : Status := .ok
  prepare : Status := .ok
  invoke : Status := .ok
  length : UInt32 := 8
  entry : UInt32 := 0

structure World where
  scenario : Scenario
  trace : List String := []

abbrev Model := StateM World

def observe (name : String) (value : Scenario → α) : Model α := do
  modify fun w => { w with trace := w.trace ++ [name] }
  return value (← get).scenario

def headerWord (s : Scenario) (index : UInt32) : UInt32 :=
  match index with
  | 0 => 0x4f4f424c
  | 1 => 0x31505554
  | 2 => 1
  | 3 => 0x00130001
  | 4 => 1
  | 5 => 2
  | 6 => s.length
  | 7 => s.entry
  | 9 => 1
  | _ => 0

instance : Memory.Reader Model where
  byte := fun address => do
    let offset := address - buffer.value
    return ((headerWord (← get).scenario (offset / 4)) >>> ((offset % 4) * 8)).toUInt8
  word := fun address => do
    return headerWord (← get).scenario ((address - buffer.value) / 4)

instance : Update.Crypto Model where
  signature := fun _ => observe "signature" (·.signature)
  digest := fun input _ _ =>
    if input == image then observe "copied-digest" (·.copiedDigest)
    else observe "digest" (·.digest)

def effects : Effects Model where
  busy := observe "busy" (·.busy)
  reset := observe "reset" (·.reset)
  start := observe "start" (·.start)
  receive := fun _ => observe "receive" (·.receive)
  stop := observe "stop" (·.stop)
  accepted := observe "accepted" fun s => ⟨if s.empty then 0 else s.length + 160⟩
  copy := fun _ _ => observe "copy" (·.copy)
  prepare := fun _ _ => observe "prepare" (·.prepare)
  invoke := fun _ => observe "invoke" (·.invoke)

def check (label : String) (scenario : Scenario) (expected : Status)
    (trace : List String) : IO Unit := do
  let (result, world) := (NetworkBoot.run effects ⟨⟨0x12340000⟩⟩ ⟨100⟩).run { scenario := scenario }
  unless result == expected && world.trace == trace do
    throw <| IO.userError s!"{label}: result={repr result}, trace={repr world.trace}"

def checkBounds : IO Unit := do
  let verifySignature (_ : CpuAddress) : StateM Nat Bool := do
    modify (· + 1); return true
  let verifyDigest (_ : CpuAddress) (_ : ByteCount) (_ : CpuAddress) : StateM Nat Bool := do
    modify (· + 1); return true
  for p in ([0, 0xffffff60, 0xffffffff] : List UInt32) do
    unless ((Update.CryptoBounds.signature verifySignature ⟨p⟩).run 0 |>.run) == (false, 0) do
      throw <| IO.userError "invalid signature span reached verifier"
  unless ((Update.CryptoBounds.signature verifySignature ⟨0xffffff5f⟩).run 0 |>.run) == (true, 1) do
    throw <| IO.userError "valid signature boundary rejected"
  for (p, n, h) in ([(0, 1, 4096), (4096, 1, 0), (4096, 0x1000001, 8192),
      (0xfffffff0, 32, 4096), (4096, 8, 0xfffffff0)] : List (UInt32 × UInt32 × UInt32)) do
    unless ((Update.CryptoBounds.digest verifyDigest ⟨p⟩ ⟨n⟩ ⟨h⟩).run 0 |>.run) == (false, 0) do
      throw <| IO.userError "invalid digest span reached verifier"
  unless ((Update.CryptoBounds.digest verifyDigest ⟨4096⟩ ⟨0x200000⟩ ⟨8192⟩).run 0 |>.run) == (true, 1) do
    throw <| IO.userError "valid digest boundary rejected"
  for (n, e, expected) in ([(4, 0, true), (5, 4, false), (8, 4, true),
      (8, 1, false), (8, 8, false), (0x100000, 0xffffc, true),
      (0x100001, 0, false), (0xffffffff, 0xfffffffc, false)] : List (UInt32 × UInt32 × Bool)) do
    unless entryFits ⟨n⟩ ⟨e⟩ == expected do
      throw <| IO.userError s!"entry bounds mismatch: {n}/{e}"

def main : IO Unit := do
  let received := ["busy", "reset", "start", "receive", "stop", "accepted"]
  let authenticated := received ++ ["signature", "digest"]
  let copied := authenticated ++ ["copy", "copied-digest"]
  check "busy" { busy := true } ⟨29⟩ ["busy"]
  check "reset failure" { reset := ⟨1⟩ } ⟨1⟩ ["busy", "reset"]
  check "start failure stops" { start := ⟨30⟩ } ⟨30⟩ ["busy", "reset", "start", "stop"]
  check "failed start and stop" { start := ⟨30⟩, stop := ⟨1⟩ } ⟨20⟩ ["busy", "reset", "start", "stop"]
  check "receive failure stops" { receive := ⟨31⟩ } ⟨31⟩ ["busy", "reset", "start", "receive", "stop"]
  check "stop failure blocks auth" { stop := ⟨1⟩ } ⟨20⟩ ["busy", "reset", "start", "receive", "stop"]
  check "empty window" { empty := true } .ok received
  check "bad signature" { signature := false } ⟨43⟩ (received ++ ["signature"])
  check "bad source digest" { digest := false } ⟨43⟩ authenticated
  check "partial instruction" { length := 5, entry := 4 } ⟨42⟩ authenticated
  check "bad entry alignment" { entry := 1 } ⟨43⟩ received
  check "copy failure" { copy := ⟨1⟩ } ⟨44⟩ (authenticated ++ ["copy"])
  check "bad copied digest" { copiedDigest := false } ⟨43⟩ copied
  check "prepare failure" { prepare := ⟨20⟩ } ⟨20⟩ (copied ++ ["prepare"])
  check "success" {} .ok (copied ++ ["prepare", "invoke"])
  check "payload return" { invoke := ⟨37⟩ } ⟨37⟩ (copied ++ ["prepare", "invoke"])
  checkBounds
  IO.println "PASS: 16 controller traces, 10 crypto-span cases, 8 executable-boundary cases"
