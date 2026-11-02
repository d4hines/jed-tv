prelude
import Init.Data.UInt.Lemmas
import Init.Omega
import TV.Hardware.TeeBoundary
set_option compiler.extract_closed false

/-! Kernel sysfs text for private settings storage. These are scalar byte
transitions, with no borrowed pointers, UTF-8 decoder or boxed runtime state.
Kernel records are ASCII: unlike the old Unicode trim, non-ASCII whitespace
now fails closed. Decimal accepts one leading '+', never hexadecimal. -/
namespace TV.StorageText

def space (byte : UInt32) : Bool := byte == 32 || (byte >= 9 && byte <= 13)
def decimalDigit (byte : UInt32) : Bool := byte >= 48 && byte <= 57
def digit (byte : UInt32) : UInt64 := (byte - 48).toUInt64

/-- `maxUInt64 = 10 * 1844674407370955161 + 5`. -/
def fits (number digit : UInt64) : Bool :=
  number < 1844674407370955161 || (number == 1844674407370955161 && digit <= 5)

/- States: 0 leading whitespace, 1 leading '+', 2 digits, 3 trailing
whitespace, 255 invalid. Invalid states are sticky. -/
@[export tv_storage_decimal_step]
def decimalStep (state : UInt32) (number : UInt64) (byte : UInt32) : UInt32 :=
  if byte > 127 then 255 else
  if state == 0 then
    if space byte then 0 else if byte == 43 then 1
    else if decimalDigit byte && fits number (digit byte) then 2 else 255
  else if state == 1 then
    if decimalDigit byte && fits number (digit byte) then 2 else 255
  else if state == 2 then
    if decimalDigit byte then if fits number (digit byte) then 2 else 255
    else if space byte then 3 else 255
  else if state == 3 then if space byte then 3 else 255
  else 255

@[export tv_storage_decimal_value]
def decimalValue (state : UInt32) (number : UInt64) (byte : UInt32) : UInt64 :=
  if decimalStep state number byte == 2 && decimalDigit byte then
    number * 10 + digit byte
  else number

@[export tv_storage_decimal_finish]
def decimalFinish (state : UInt32) : Bool := state == 2 || state == 3

/-- Reuse the checked TEE major:minor grammar, but Linux makedev's two inputs
are UInt32. The native adapter casts only after this acceptance check. -/
@[export tv_storage_device_finish]
def deviceFinish (state : UInt32) (major minor : UInt64) : Bool :=
  TV.Hardware.TeeBoundary.deviceFinish state && major <= 0xffffffff && minor <= 0xffffffff

theorem invalidDecimalStaysInvalid (number : UInt64) (byte : UInt32) :
    decimalStep 255 number byte = 255 := by
  simp [decimalStep]

theorem nonAsciiRejected (state : UInt32) (number : UInt64) (byte : UInt32)
    (h : byte > 127) : decimalStep state number byte = 255 := by
  simp [decimalStep, h]

theorem incompleteDecimalRejected : decimalFinish 0 = false ∧ decimalFinish 1 = false := by
  decide

theorem acceptedDigitFits (state : UInt32) (number : UInt64) (byte : UInt32)
    (h : decimalStep state number byte = 2) : fits number (digit byte) = true := by
  unfold decimalStep at h
  repeat' (first | contradiction | split at h)
  all_goals simp_all [Bool.and_eq_true]

theorem acceptedIsDigit (state : UInt32) (number : UInt64) (byte : UInt32)
    (h : decimalStep state number byte = 2) : decimalDigit byte = true := by
  unfold decimalStep at h
  repeat' (first | contradiction | split at h)
  all_goals simp_all [Bool.and_eq_true]

theorem fitsBounds (number d : UInt64) (hd : d <= 9) (h : fits number d = true) :
    number.toNat * 10 + d.toNat < 18446744073709551616 := by
  simp only [fits, Bool.or_eq_true, Bool.and_eq_true, decide_eq_true_eq, beq_iff_eq] at h
  rcases h with h | ⟨h, ht⟩
  · change number.toNat < 1844674407370955161 at h
    change d.toNat <= 9 at hd
    omega
  · subst number
    change d.toNat <= 5 at ht
    change 1844674407370955161 * 10 + d.toNat < 18446744073709551616
    omega

/-- Every accepted accumulator update is within UInt64 before machine
arithmetic is applied; overflow cannot turn a rejected integer into a value. -/
theorem acceptedAppendBounded (state : UInt32) (number : UInt64) (byte : UInt32)
    (h : decimalStep state number byte = 2) :
    number.toNat * 10 + (digit byte).toNat < 18446744073709551616 := by
  have hd := acceptedIsDigit state number byte h
  simp only [decimalDigit, Bool.and_eq_true, decide_eq_true_eq] at hd
  have bound : digit byte <= 9 := by
    rw [UInt64.le_iff_toNat_le]
    simp only [digit, UInt32.toNat_toUInt64, UInt32.toNat_sub_of_le byte 48 hd.1]
    change byte.toNat - 48 <= 9
    have upper : byte.toNat <= 57 := hd.2
    omega
  exact fitsBounds number (digit byte) bound (acceptedDigitFits state number byte h)

theorem acceptedAppendExact (state : UInt32) (number : UInt64) (byte : UInt32)
    (h : decimalStep state number byte = 2) :
    (decimalValue state number byte).toNat = number.toNat * 10 + (digit byte).toNat := by
  have hd := acceptedIsDigit state number byte h
  have bound := acceptedAppendBounded state number byte h
  simp only [decimalValue, h, hd, beq_self_eq_true, Bool.and_self, ite_true]
  rw [UInt64.toNat_add, UInt64.toNat_mul]
  change ((number.toNat * 10) % 18446744073709551616 + (digit byte).toNat) %
    18446744073709551616 = number.toNat * 10 + (digit byte).toNat
  have product : number.toNat * 10 < 18446744073709551616 := by omega
  rw [Nat.mod_eq_of_lt product, Nat.mod_eq_of_lt bound]

theorem deviceRequiresMajorBound (state : UInt32) (major minor : UInt64)
    (h : deviceFinish state major minor = true) : major <= 0xffffffff := by
  simp only [deviceFinish, Bool.and_eq_true, decide_eq_true_eq] at h
  exact h.1.2

theorem deviceRequiresMinorBound (state : UInt32) (major minor : UInt64)
    (h : deviceFinish state major minor = true) : minor <= 0xffffffff := by
  simp only [deviceFinish, Bool.and_eq_true, decide_eq_true_eq] at h
  exact h.2

end TV.StorageText
