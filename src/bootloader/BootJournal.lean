import Memory
import Flow
import BootJournal.Plan

/- Record codec and storage interpreter for the pure BootJournal.Plan.
   The outer adapter supplies memory and block capabilities. -/
namespace LeanBoot.BootJournal
open Memory

class Blocks (m : Type → Type) where
  read : UInt32 → UInt32 → m UInt32
  write : UInt32 → UInt32 → m UInt32

private partial def bits (value count : UInt32) : UInt32 :=
  if count == 0 then value
  else bits ((value >>> 1) ^^^ (if value &&& 1 == 0 then 0 else 0xedb88320)) (count - 1)

variable {m : Type → Type} [Monad m] [Reader m]

@[specialize] private partial def crcLoop (base size crc : UInt32) : m UInt32 := do
  if size == 0 then return crc ^^^ 0xffffffff
  let byte ← Reader.byte base
  crcLoop (base + 1) (size - 1) (bits (crc ^^^ byte.toUInt32) 8)

@[specialize] def crc (base size : UInt32) : m UInt32 :=
  crcLoop base size 0xffffffff

@[specialize] private def field (base offset : UInt32) : m UInt32 := do
  return (← readLE32 base 512 offset).toUInt32

@[specialize] def decode (base : UInt32) : m UInt64 := do
  let seq ← field base 8
  let state ← field base 12
  if (← field base 0) != 0x314a424c || (← field base 4) != 1 ||
      (← field base 16) != (seq ^^^ 0xffffffff) ||
      (← field base 20) != (state ^^^ 0xffffffff) || !BootPolicy.valid state ||
      (← crc base 508) != (← field base 508) then return error 1
  return pack seq state

@[specialize] def validRecord (base : UInt32) : m Bool := do
  let seq ← field base 8
  let state ← field base 12
  return (← field base 0) == 0x314a424c && (← field base 4) == 1 &&
    (← field base 16) == (seq ^^^ 0xffffffff) &&
    (← field base 20) == (state ^^^ 0xffffffff) && BootPolicy.valid state &&
    (← crc base 508) == (← field base 508)

@[specialize] def encode [Writer m] (base seq state : UInt32) : m UInt32 := do
  if !BootPolicy.valid state then return 1
  let _ ← fill base 512 0 512 0
  let _ ← writeLE32 base 512 0 0x314a424c
  let _ ← writeLE32 base 512 4 1
  let _ ← writeLE32 base 512 8 seq
  let _ ← writeLE32 base 512 12 state
  let _ ← writeLE32 base 512 16 (seq ^^^ 0xffffffff)
  let _ ← writeLE32 base 512 20 (state ^^^ 0xffffffff)
  writeLE32 base 512 508 (← crc base 508)

/-- Commit the next selection before entering the larger payload. A missing,
    inconsistent, unreadable or unwritable journal always selects stock.
    No boot success callback resets the counter. -/
@[specialize] def advance [Writer m] [Blocks m] : m UInt32 := do
  let a : UInt32 := 0x26350000
  let b : UInt32 := 0x26350200
  let next : UInt32 := 0x26350800
  let verify : UInt32 := 0x26350a00
  if (← Blocks.read 0 a) != 0 || (← Blocks.read 1 b) != 0 then return BootPolicy.rejected
  let ra ← decode a
  let rb ← decode b
  withNext ra rb (← validRecord a) (← validRecord b) pure fun plan => do
    let result ← Flow.run .journal fun stage => do
      match stage with
      | .encodeRecord => return ⟨← encode next plan.sequence plan.state⟩
      | .writeRecord => return ⟨← Blocks.write plan.target next⟩
      | .readRecord => return ⟨← Blocks.read plan.target verify⟩
      | .verifyRecord =>
        return if (← validRecord verify) && (← decode verify) == plan.record then .ok else ⟨1⟩
      | _ => pure ⟨45⟩
    return if result.succeeded then plan.state else BootPolicy.rejected

end LeanBoot.BootJournal
