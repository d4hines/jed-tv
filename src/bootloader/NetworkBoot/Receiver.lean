import NetworkBoot.Plan
import Update.Server
import Update.Flash
import Ethernet

/- Protocol and receiver state use the same abstract memory/crypto effects
   as the updater. No register accesses or physical execution occur here. -/
namespace LeanBoot.NetworkBoot.Receiver
variable {m : Type → Type} [Monad m] [Memory.Reader m] [Memory.Writer m]
  [Update.Crypto m] [Update.Storage m]

@[specialize] private def admit (source : CpuAddress) (size : ByteCount) : m Status :=
  Update.withManifest source.value size.value (pure ⟨2⟩) fun manifest => do
    if manifest.component == Update.Flash.component then
      if manifest.entry != 0 then return ⟨2⟩
      let result ← Update.Flash.installImage source.value size.value
      if result != 0 then return ⟨result⟩
      let _ ← Memory.Writer.word (state.value + 72) 1
      let _ ← Memory.Writer.word (state.value + 68) 4000000
      return .ok
    Update.withAuthenticated policy source.value size.value (pure ⟨2⟩) fun _ => do
      let _ ← Memory.Writer.word (state.value + 64) size.value
    -- Allow a short, bounded final-ACK retry interval, then execute even if
    -- no further datagram arrives. Accepted bytes are immutable in this phase.
      let _ ← Memory.Writer.word (state.value + 68) 4000000
      return .ok

@[specialize] def finished : m Bool := do
  if (← Memory.Reader.word (state.value + 64)) == 0 &&
      (← Memory.Reader.word (state.value + 72)) == 0 then return false
  let remaining ← Memory.Reader.word (state.value + 68)
  if remaining == 0 then return true
  let _ ← Memory.Writer.word (state.value + 68) (remaining - 1)
  return false

@[specialize] def respond (rx : CpuAddress) (length : ByteCount) : m ByteCount := do
  let tx : CpuAddress := ⟨(rx.value &&& 0xfff00000) + 0x4000⟩
  let n ← Ethernet.respond rx.value length.value tx.value 2048 0xc0a84d03 0x18acc22d1c47
  if n == 0 then return ⟨0⟩
  if (← Memory.read8 tx.value 2048 23) != 17 then return ⟨n⟩
  let hi ← Memory.read8 rx.value length.value 38
  let lo ← Memory.read8 rx.value length.value 39
  let size : ByteCount := ⟨(hi <<< 8 ||| lo) - 8⟩
  if (← Memory.Reader.word (state.value + 64)) != 0 ||
      (← Memory.Reader.word (state.value + 72)) != 0 then
    if size.value != 20 || (← Memory.readLE32 (rx.value + 42) size.value 4) != 3 then return ⟨0⟩
  let accept := fun source size => do return (← admit ⟨source⟩ ⟨size⟩).code
  let out ← Update.Server.dispatch accept state.value buffer.value (Update.Flash.capacity + 160)
    (rx.value + 42) size.value (tx.value + 42) workspace.value 4096
  if out == 0 then return ⟨0⟩
  return ⟨← Ethernet.finishUdp tx.value 2048 out⟩

end LeanBoot.NetworkBoot.Receiver
