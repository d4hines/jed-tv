import Init

namespace LeanBoot.EntryCheck

/-- This is an admission check, not proof of RAM ownership or cache coherency.
    Bootstrap must already provide executable code and mapped private stacks. -/
@[inline] def mode (cpsr : UInt32) : Bool :=
  cpsr &&& 0x3f == 0x13 && cpsr &&& 0xc0 == 0xc0

@[inline] def stack (sp low high : UInt32) : Bool :=
  sp &&& 7 == 0 && sp >= low + 4096 && sp <= high

@[inline] def g13 (cpsr sp sctlr : UInt32) : Bool :=
  mode cpsr && stack sp 0x26330000 0x26340000 && sctlr == 0x00c0181d

end LeanBoot.EntryCheck
