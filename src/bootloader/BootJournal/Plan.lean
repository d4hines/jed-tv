import BootPolicy

/- Pure journal decisions. Words retain the installed sequence/state format;
   Commit names the values the storage interpreter needs. No memory reads or
   block operations occur here. -/
namespace LeanBoot.BootJournal

@[inline] def sequence (record : UInt64) : UInt32 := (record >>> 32).toUInt32

@[inline] def pack (seq state : UInt32) : UInt64 :=
  (seq.toUInt64 <<< 32) ||| state.toUInt64

/-- 0/1 selects a record; 2 rejects both. Adjacent sequence numbers include
    UInt32 wrap. Equal words select slot 0; inconsistent words reject. -/
@[export leanboot_journal_select]
def select (a b : UInt64) (goodA goodB : Bool) : UInt32 :=
  if !goodA && !goodB then 2
  else if !goodA then 1
  else if !goodB then 0
  else if a == b then 0
  else if sequence a + 1 == sequence b then 1
  else if sequence b + 1 == sequence a then 0
  else 2

structure Commit where
  source : UInt32
  sequence : UInt32
  state : UInt32

@[inline] def Commit.target (plan : Commit) : UInt32 := plan.source ^^^ 1
@[inline] def Commit.record (plan : Commit) : UInt64 := pack plan.sequence plan.state

/-- Produce values for one inactive-slot write, or reject before storage is
    touched. Tail callbacks let the freestanding build scalarize Commit. -/
@[inline] def withNext {α : Type} (a b : UInt64) (goodA goodB : Bool)
    (reject : UInt32 → α) (accept : Commit → α) : α :=
  let source := select a b goodA goodB
  if source > 1 then reject BootPolicy.rejected else
  let old := if source == 0 then a else b
  let state := BootPolicy.step old.toUInt32 1
  if BootPolicy.failed state then reject state else
  accept { source, sequence := sequence old + 1, state }

end LeanBoot.BootJournal
