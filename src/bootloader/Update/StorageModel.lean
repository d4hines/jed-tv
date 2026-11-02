import Update.FlashProofs

/- Specification only, not linked into firmware. The card may retain a
   pending write, finish it before flush, lose it on power failure, or tear
   it into arbitrary bytes. The one physical assumption is write isolation:
   a write/tear does not corrupt another sector. Real eMMC FTL behavior under
   power loss is an explicit hardware obligation, not a theorem here. -/
namespace LeanBoot.Update.StorageModel
open Flash

abbrev Sector := Fin 512 → UInt8
abbrev Media := UInt32 → Sector

structure Pending where
  address : UInt32
  bytes : Sector

structure Card where
  durable : Media
  pending : Option Pending := none

def replace (media : Media) (address : UInt32) (bytes : Sector) : Media :=
  fun a => if a = address then bytes else media a

def finish (card : Card) : Card :=
  match card.pending with
  | none => card
  | some p => ⟨replace card.durable p.address p.bytes, none⟩

/-- Arbitrary sector bytes include no write, full write, non-prefix tears,
    and bit corruption. Only the in-flight sector may change. -/
def powerLoss (card : Card) (torn : Sector) : Card :=
  match card.pending with
  | none => card
  | some p => ⟨replace card.durable p.address torn, none⟩

inductive Event
  | write (sector : UInt32) (bytes : Sector)
  | flush
  | powerLoss (torn : Sector)

/-- Writes use the production address guard. A single-command card cannot
    accept a second write while one is pending. -/
def step (target : UInt32) (card : Card) : Event → Card
  | .write sector bytes =>
    if card.pending.isSome then card else
      withSector target sector card fun address =>
        ⟨card.durable, some ⟨address.value, bytes⟩⟩
  | .flush => finish card
  | .powerLoss torn => powerLoss card torn

def run (target : UInt32) (card : Card) (events : List Event) : Card :=
  events.foldl (step target) card

/-- A protected sector is neither being programmed nor already changed. -/
def Protected (card : Card) (address : UInt32) (original : Sector) : Prop :=
  card.durable address = original ∧
    ∀ p, card.pending = some p → p.address ≠ address

theorem step_preserves (target address : UInt32) (original : Sector)
    (outside : ∀ sector, sectorAllowed target sector = true →
      (sectorAddress target sector).value ≠ address)
    (card : Card) (safe : Protected card address original) (event : Event) :
    Protected (step target card event) address original := by
  rcases safe with ⟨same, pending⟩
  cases event with
  | write sector bytes =>
    simp only [step]
    split
    · exact ⟨same, pending⟩
    · unfold withSector
      split
      next allowed =>
        constructor
        · exact same
        · intro p eq
          cases eq
          exact outside sector allowed
      · exact ⟨same, pending⟩
  | flush =>
    simp only [step, finish]
    split
    · exact ⟨same, pending⟩
    next p eq =>
      have different := pending p eq
      simp [Protected, replace, Ne.symm different, same]
  | powerLoss torn =>
    simp only [step, powerLoss]
    split
    · exact ⟨same, pending⟩
    next p eq =>
      have different := pending p eq
      simp [Protected, replace, Ne.symm different, same]

theorem arbitrary_trace_preserves (target address : UInt32) (original : Sector)
    (outside : ∀ sector, sectorAllowed target sector = true →
      (sectorAddress target sector).value ≠ address)
    (events : List Event) (card : Card) (safe : Protected card address original) :
    Protected (run target card events) address original := by
  induction events generalizing card with
  | nil => exact safe
  | cons event rest ih =>
    exact ih (step target card event) (step_preserves target address original outside card safe event)

/-- Any number of power losses, arbitrary torn sectors, rejected addresses
    and flushes preserve every byte of the other bank, without atomic-sector
    writes or a finite enumeration of tear patterns. -/
theorem other_bank_survives (target active sector : UInt32) (media : Media)
    (valid : sectorAllowed active sector = true) (different : target ≠ active)
    (events : List Event) :
    (run target ⟨media, none⟩ events).durable (sectorAddress active sector).value =
      media (sectorAddress active sector).value := by
  exact (arbitrary_trace_preserves target (sectorAddress active sector).value _
    (fun s ok => different_banks_disjoint target active s sector ok valid different)
    events ⟨media, none⟩ ⟨rfl, by simp⟩).1

theorem journal_survives (target : UInt32) (media : Media) (events : List Event) :
    (run target ⟨media, none⟩ events).durable 0x3217fe = media 0x3217fe ∧
    (run target ⟨media, none⟩ events).durable 0x3217ff = media 0x3217ff := by
  constructor
  · exact (arbitrary_trace_preserves target 0x3217fe _
      (fun s ok => (sector_excludes_journal target s ok).1)
      events ⟨media, none⟩ ⟨rfl, by simp⟩).1
  · exact (arbitrary_trace_preserves target 0x3217ff _
      (fun s ok => (sector_excludes_journal target s ok).2)
      events ⟨media, none⟩ ⟨rfl, by simp⟩).1

/-- A completed durability barrier leaves nothing for a later power cut to
    tear. This is the abstract Storage.flush contract, not a proof that DAT0
    implements it on this card. -/
theorem flushed_survives_power_loss (card : Card) (torn : Sector) :
    powerLoss (finish card) torn = finish card := by
  cases card with
  | mk durable pending => cases pending <;> rfl

end LeanBoot.Update.StorageModel
