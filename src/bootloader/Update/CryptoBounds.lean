import Types
import Memory
import Update.Policy

namespace LeanBoot.Update.CryptoBounds

/-- Arithmetic preconditions for the fixed-size verifier ABI. Physical RAM
    mapping, ownership and stability remain the adapter's responsibility. -/
@[inline] def signatureInput (header : CpuAddress) : Bool :=
  header.value != 0 && Memory.addressable header.value headerSize

@[inline] def digestInput (input : CpuAddress) (length : ByteCount)
    (expected : CpuAddress) : Bool :=
  input.value != 0 && expected.value != 0 && length.value <= maxPayload &&
    Memory.addressable input.value length.value && Memory.addressable expected.value 32

@[inline] def signature {m : Type → Type} [Monad m]
    (verify : CpuAddress → m Bool) (header : CpuAddress) : m Bool :=
  if signatureInput header then verify header else pure false

@[inline] def digest {m : Type → Type} [Monad m]
    (verify : CpuAddress → ByteCount → CpuAddress → m Bool)
    (input : CpuAddress) (length : ByteCount) (expected : CpuAddress) : m Bool :=
  if digestInput input length expected then verify input length expected else pure false

end LeanBoot.Update.CryptoBounds
