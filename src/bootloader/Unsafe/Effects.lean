import Init

namespace LeanBoot.Unsafe

inductive Effect where
  | memory
  | hardware

/- A compile-time effect color, not a proof of hardware safety. The proposition
   argument is erased by Lean, so recursive operations retain BaseIO's machine
   representation. Different colors do not unify or coerce to each other. -/
inductive Permission (effect : Effect) : Prop where
  | token : Permission effect

def EffectM (effect : Effect) (α : Type) : Type := Permission effect → BaseIO α
abbrev MemoryM := EffectM .memory
abbrev HardwareM := EffectM .hardware

@[inline] private def pureEffect {e : Effect} {α : Type} (value : α) : EffectM e α :=
  fun _ => pure value

@[inline] private def bindEffect {e : Effect} {α β : Type}
    (action : EffectM e α) (next : α → EffectM e β) : EffectM e β :=
  fun permission => do next (← action permission) permission

@[always_inline] instance {e : Effect} : Monad (EffectM e) where
  pure := pureEffect
  bind := bindEffect

/-- Explicit memory-to-hardware lift; there is deliberately no reverse lift. -/
@[inline] def liftMemory {α : Type} (action : MemoryM α) : HardwareM α :=
  fun _ => action .token

/- Only reviewed outer entry/export adapters discharge a color to BaseIO.
   These runners are not available to the generic deterministic core. -/
@[inline] def runMemory {α : Type} (action : MemoryM α) : BaseIO α := action .token
@[inline] def runHardware {α : Type} (action : HardwareM α) : BaseIO α := action .token

end LeanBoot.Unsafe
