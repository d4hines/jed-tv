import MemoryAccess
import Types

namespace LeanBoot.Update

/-- Fixed Ed25519 verification key belongs to the interpreter, never the peer.
    The signed message is exactly 96 bytes, followed by its 64-byte signature.
    Hashing compares SHA-256 of the supplied image with 32 manifest bytes. -/
class Crypto (m : Type → Type) where
  signature : CpuAddress → m Bool
  digest : CpuAddress → ByteCount → CpuAddress → m Bool

/-- Bank-scoped sector I/O. No partition name or physical address is supplied
    by a package. Successful flush must make prior writes durable in the
    chosen storage model. Implementations must reject bank/sector overflow. -/
class Storage (m : Type → Type) where
  read : UInt32 → UInt32 → UInt32 → m Bool
  write : UInt32 → UInt32 → UInt32 → m Bool
  flush : m Bool

end LeanBoot.Update
