prelude
import Init.Data.UInt
set_option compiler.extract_closed false

/-! ASCII kernel ownership records. Native code only supplies bytes and PID;
these entry points have scalar ABIs and never enter the allocating runtime. -/
namespace TV.Hardware.Ownership

private def space (b : UInt32) : Bool := b == 32 || (9 ≤ b && b ≤ 13)
private def decimal (b : UInt32) : Bool := 48 ≤ b && b ≤ 57
private def length (kind : UInt32) : UInt32 :=
  match kind with
  | 0 => 38 -- androidboot.serialno=4C0490290FA05A0C0
  | 1 => 14 -- surfaceflinger
  | 2 => 15 -- composer@2.4-se
  | 3 => 11 -- audioserver
  | 4 => 15 -- mstar.hardware.
  | 5 => 8 -- midaemon
  | 6 => 15 -- app-rust-displa
  | 7 => 15 -- app-rust-audio-
  | 8 => 6 -- State:
  | _ => 0

private def byte (kind position : UInt32) : UInt32 :=
  match kind with
  | 0 => -- androidboot.serialno=4C0490290FA05A0C0
    match position with
    | 0 => 97
    | 1 => 110
    | 2 => 100
    | 3 => 114
    | 4 => 111
    | 5 => 105
    | 6 => 100
    | 7 => 98
    | 8 => 111
    | 9 => 111
    | 10 => 116
    | 11 => 46
    | 12 => 115
    | 13 => 101
    | 14 => 114
    | 15 => 105
    | 16 => 97
    | 17 => 108
    | 18 => 110
    | 19 => 111
    | 20 => 61
    | 21 => 52
    | 22 => 67
    | 23 => 48
    | 24 => 52
    | 25 => 57
    | 26 => 48
    | 27 => 50
    | 28 => 57
    | 29 => 48
    | 30 => 70
    | 31 => 65
    | 32 => 48
    | 33 => 53
    | 34 => 65
    | 35 => 48
    | 36 => 67
    | 37 => 48
    | _ => 0
  | 1 => -- surfaceflinger
    match position with
    | 0 => 115
    | 1 => 117
    | 2 => 114
    | 3 => 102
    | 4 => 97
    | 5 => 99
    | 6 => 101
    | 7 => 102
    | 8 => 108
    | 9 => 105
    | 10 => 110
    | 11 => 103
    | 12 => 101
    | 13 => 114
    | _ => 0
  | 2 => -- composer@2.4-se
    match position with
    | 0 => 99
    | 1 => 111
    | 2 => 109
    | 3 => 112
    | 4 => 111
    | 5 => 115
    | 6 => 101
    | 7 => 114
    | 8 => 64
    | 9 => 50
    | 10 => 46
    | 11 => 52
    | 12 => 45
    | 13 => 115
    | 14 => 101
    | _ => 0
  | 3 => -- audioserver
    match position with
    | 0 => 97
    | 1 => 117
    | 2 => 100
    | 3 => 105
    | 4 => 111
    | 5 => 115
    | 6 => 101
    | 7 => 114
    | 8 => 118
    | 9 => 101
    | 10 => 114
    | _ => 0
  | 4 => -- mstar.hardware.
    match position with
    | 0 => 109
    | 1 => 115
    | 2 => 116
    | 3 => 97
    | 4 => 114
    | 5 => 46
    | 6 => 104
    | 7 => 97
    | 8 => 114
    | 9 => 100
    | 10 => 119
    | 11 => 97
    | 12 => 114
    | 13 => 101
    | 14 => 46
    | _ => 0
  | 5 => -- midaemon
    match position with
    | 0 => 109
    | 1 => 105
    | 2 => 100
    | 3 => 97
    | 4 => 101
    | 5 => 109
    | 6 => 111
    | 7 => 110
    | _ => 0
  | 6 => -- app-rust-displa
    match position with
    | 0 => 97
    | 1 => 112
    | 2 => 112
    | 3 => 45
    | 4 => 114
    | 5 => 117
    | 6 => 115
    | 7 => 116
    | 8 => 45
    | 9 => 100
    | 10 => 105
    | 11 => 115
    | 12 => 112
    | 13 => 108
    | 14 => 97
    | _ => 0
  | 7 => -- app-rust-audio-
    match position with
    | 0 => 97
    | 1 => 112
    | 2 => 112
    | 3 => 45
    | 4 => 114
    | 5 => 117
    | 6 => 115
    | 7 => 116
    | 8 => 45
    | 9 => 97
    | 10 => 117
    | 11 => 100
    | 12 => 105
    | 13 => 111
    | 14 => 45
    | _ => 0
  | 8 => -- State:
    match position with
    | 0 => 83
    | 1 => 116
    | 2 => 97
    | 3 => 116
    | 4 => 101
    | 5 => 58
    | _ => 0
  | _ => 0

/- Serial state: low byte is current-token matched length (255 mismatch),
bit8 remembers a complete earlier token; all-ones means non-ASCII input. -/
@[export tv_owner_serial_step]
def serialStep (state b : UInt32) : UInt32 :=
  if state == 0xffffffff || b > 127 then 0xffffffff else
  let index := state &&& 255
  let found := state &&& 256
  if space b then found ||| (if index == length 0 then 256 else 0)
  else found ||| (if index < length 0 && b == byte 0 index then index + 1 else 255)

@[export tv_owner_identity]
def identity (pid state : UInt32) : Bool :=
  pid == 1 && state != 0xffffffff && ((state &&& 256) != 0 || (state &&& 255) == length 0)

/- PID-name state: phase in high word (0 start,1 '+',2 digits); low word
holds the parsed u32. All-ones is absorbing invalid. -/
@[export tv_owner_pid_step]
def pidStep (state : UInt64) (b : UInt32) : UInt64 :=
  let phase := state >>> 32
  let value := state &&& 0xffffffff
  if state == 0xffffffffffffffff || b > 127 then 0xffffffffffffffff
  else if phase == 0 && b == 43 then (1 : UInt64) <<< 32
  else if phase ≤ 2 && decimal b then
    let digit := (b - 48).toUInt64
    if value ≤ (0xffffffff - digit) / 10 then ((2 : UInt64) <<< 32) ||| (value * 10 + digit)
    else 0xffffffffffffffff
  else 0xffffffffffffffff

@[export tv_owner_pid_finish]
def pidFinish (state : UInt64) : Bool := state >>> 32 == 2

private def retain (mask position b kind bit : UInt32) : UInt32 :=
  if mask &&& bit != 0 && position < length kind && b == byte kind position then bit else 0
private def narrow (mask position b : UInt32) : UInt32 :=
  retain mask position b 1 1 ||| retain mask position b 2 2 ||| retain mask position b 3 4 |||
  retain mask position b 4 8 ||| retain mask position b 5 16 ||| retain mask position b 6 32 |||
  retain mask position b 7 64
private def complete (mask position : UInt32) : UInt32 :=
  if mask &&& 1 != 0 && position == length 1 then 1
  else if mask &&& 2 != 0 && position == length 2 then 2
  else if mask &&& 4 != 0 && position == length 3 then 3
  else if mask &&& 8 != 0 && position == length 4 then 4
  else if mask &&& 16 != 0 && position == length 5 then 5
  else if mask &&& 32 != 0 && position == length 6 then 6
  else if mask &&& 64 != 0 && position == length 7 then 7
  else 0

/- comm state: high byte 0 leading,1 token,2 trailing,3 unknown; low
seven bits are candidates while in-token and final kind while trailing. -/
@[export tv_owner_comm_step]
def commStep (state b : UInt32) : UInt32 :=
  if state == 0xffffffff || b > 127 then 0xffffffff else
  let phase := state >>> 24
  let position := (state >>> 8) &&& 255
  let mask := state &&& 127
  if phase == 0 then
    if space b then 0 else ((1 : UInt32) <<< 24) ||| ((1 : UInt32) <<< 8) ||| narrow 127 0 b
  else if phase == 1 then
    if space b then ((2 : UInt32) <<< 24) ||| complete mask position
    else if position < 16 then ((1 : UInt32) <<< 24) ||| ((position + 1) <<< 8) ||| narrow mask position b
    else (3 : UInt32) <<< 24
  else if phase == 2 then if space b then state else (3 : UInt32) <<< 24
  else (3 : UInt32) <<< 24

/-- 0 is an unrelated name; 1..7 are recognized owners; all-ones is invalid. -/
@[export tv_owner_comm_finish]
def commFinish (state : UInt32) : UInt32 :=
  if state == 0xffffffff then 0xffffffff
  else if state >>> 24 == 1 then complete (state &&& 127) ((state >>> 8) &&& 255)
  else if state >>> 24 == 2 then state &&& 127 else 0

/- Status states0..5 match State: at line start,6 awaits its first value,
7 skips unrelated line,8 retains a nonstopped first State:,9 stopped.
Even after finding a value, every byte is checked for ASCII. -/
@[export tv_owner_status_step]
def statusStep (state b : UInt32) : UInt32 :=
  if state == 255 || b > 127 then 255
  else if state == 8 || state == 9 then state
  else if state == 6 then
    if b == 10 then 8 else if space b then 6 else if b == 84 || b == 116 then 9 else 8
  else if state == 7 then if b == 10 then 0 else 7
  else if state < 6 then
    if b == 10 then 0 else if b == byte 8 state then state + 1 else 7
  else 255

@[export tv_owner_status_finish]
def statusFinish (state : UInt32) : Bool := state == 9

@[export tv_owner_permitted]
def permitted (owner : UInt32) (stopped : Bool) : Bool :=
  owner == 0 || (1 ≤ owner && owner ≤ 7 && stopped)

theorem identity_requires_pid1 (pid state : UInt32) (h : identity pid state = true) : pid = 1 := by
  simp only [identity, Bool.and_eq_true, beq_iff_eq] at h
  exact h.1.1

theorem recognized_owner_requires_stopped (owner : UInt32) (hknown : owner ≠ 0)
    (h : permitted owner stopped = true) : stopped = true := by
  simp only [permitted, Bool.or_eq_true, Bool.and_eq_true, beq_iff_eq] at h
  rcases h with same | running
  · exact False.elim (hknown same)
  · exact running.2

theorem invalid_owner_refused (stopped : Bool) : permitted 0xffffffff stopped = false := by
  simp [permitted]

theorem invalid_pid_absorbs (b : UInt32) : pidStep 0xffffffffffffffff b = 0xffffffffffffffff := by
  simp [pidStep]

theorem invalid_serial_absorbs (b : UInt32) : serialStep 0xffffffff b = 0xffffffff := by
  simp [serialStep]
theorem invalid_comm_absorbs (b : UInt32) : commStep 0xffffffff b = 0xffffffff := by
  simp [commStep]
theorem invalid_status_absorbs (b : UInt32) : statusStep 255 b = 255 := by
  simp [statusStep]
theorem stopped_status_required (state : UInt32) (h : statusFinish state = true) : state = 9 := by
  simpa [statusFinish] using h

end TV.Hardware.Ownership
