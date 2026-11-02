import Init

/- The deterministic core knows only these memory capabilities. Reader-only
   code cannot obtain a Writer, UART, CPU register or secure-firmware service.
   Implementations supply stable, owned RAM or a pure memory model. Arithmetic
   bounds checks live in Memory; physical ownership remains a caller contract. -/
namespace LeanBoot.Memory

class Reader (m : Type → Type) where
  byte : UInt32 → m UInt8
  word : UInt32 → m UInt32

class Writer (m : Type → Type) where
  byte : UInt32 → UInt8 → m UInt32
  word : UInt32 → UInt32 → m UInt32

end LeanBoot.Memory
