import MemoryAccess
import Unsafe.Primitives

/- The only adapter from core memory capabilities to physical RAM. The entry
   contract establishes the mappings/ownership; Memory checks every span.
   Test interpreters can provide a different instance with no physical effects. -/
namespace LeanBoot.Unsafe

@[always_inline] instance : Memory.Reader MemoryM where
  byte := load8
  word := load32

@[always_inline] instance : Memory.Writer MemoryM where
  byte := store8
  word := store32

-- Explicitly embed memory operations when the outer driver composes stages.
@[always_inline] instance : Memory.Reader HardwareM where
  byte := fun a => liftMemory (load8 a)
  word := fun a => liftMemory (load32 a)

@[always_inline] instance : Memory.Writer HardwareM where
  byte := fun a v => liftMemory (store8 a v)
  word := fun a v => liftMemory (store32 a v)

end LeanBoot.Unsafe
