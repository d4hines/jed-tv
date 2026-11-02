import Unsafe.Ram
import Update.Access
import Update.CryptoBounds

/- Stable, owned input ranges are required. These primitives access RAM only;
   no verification key is accepted from the network or mutable input state. -/
namespace LeanBoot.Unsafe

@[extern "leanboot_update_signature"]
private opaque rawSignature (header : UInt32) : BaseIO Bool
@[extern "leanboot_update_digest"]
private opaque rawDigest (input length expected : UInt32) : BaseIO Bool

@[inline] def updateSignature (header : CpuAddress) : MemoryM Bool :=
  fun _ => Update.CryptoBounds.signature (fun p => rawSignature p.value) header
@[inline] def updateDigest (input : CpuAddress) (length : ByteCount) (expected : CpuAddress) : MemoryM Bool :=
  fun _ => Update.CryptoBounds.digest (fun p n h => rawDigest p.value n.value h.value) input length expected

@[always_inline] instance : Update.Crypto MemoryM where
  signature := updateSignature
  digest := updateDigest
@[always_inline] instance : Update.Crypto HardwareM where
  signature := fun p => liftMemory (updateSignature p)
  digest := fun p n h => liftMemory (updateDigest p n h)

end LeanBoot.Unsafe
