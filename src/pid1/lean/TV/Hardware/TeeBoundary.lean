prelude
import Init.Data.UInt
import Init.Omega
set_option compiler.extract_closed false

/-! Parsers for the kernel's ASCII TEE identity/readiness records. Every
transition uses scalar values only: the hardware worker may call this code
without entering the boxed Lean runtime. Linux effects supply bytes and metadata.
Malformed, non-ASCII, overflowing and incomplete records fail closed. -/
namespace TV.Hardware.TeeBoundary

private def space (b : UInt32) : Bool := b == 32 || (b >= 9 && b <= 13)
private def dec (b : UInt32) : Bool := b >= 48 && b <= 57
private def hex (b : UInt32) : Bool := dec b || (b >= 65 && b <= 70) || (b >= 97 && b <= 102)
private def digit (b : UInt32) : UInt64 :=
  if dec b then (b - 48).toUInt64
  else if b <= 70 then (b - 55).toUInt64 else (b - 87).toUInt64
private def fits (n base d : UInt64) : Bool := n <= (0xffffffffffffffff - d) / base

/- Device states: 0 leading space, 1 major sign, 2 major digits,
3 colon, 4 minor sign, 5 minor digits, 6 trailing space, 255 invalid. -/
@[export tv_tee_device_step]
def deviceStep (state : UInt32) (major minor : UInt64) (b : UInt32) : UInt32 :=
  if b > 127 then 255 else
  if state == 0 then
    if space b then 0 else if b == 43 then 1 else if dec b then 2 else 255
  else if state == 1 then if dec b then 2 else 255
  else if state == 2 then
    if dec b then if fits major 10 (digit b) then 2 else 255
    else if b == 58 then 3 else 255
  else if state == 3 then if b == 43 then 4 else if dec b then 5 else 255
  else if state == 4 then if dec b then 5 else 255
  else if state == 5 then
    if dec b then if fits minor 10 (digit b) then 5 else 255
    else if space b then 6 else 255
  else if state == 6 then if space b then 6 else 255
  else 255

@[export tv_tee_device_major]
def deviceMajor (state : UInt32) (major : UInt64) (b : UInt32) : UInt64 :=
  if (state == 0 || state == 1 || state == 2) && dec b then major * 10 + digit b else major

@[export tv_tee_device_minor]
def deviceMinor (state : UInt32) (minor : UInt64) (b : UInt32) : UInt64 :=
  if (state == 3 || state == 4 || state == 5) && dec b then minor * 10 + digit b else minor

@[export tv_tee_device_finish]
def deviceFinish (state : UInt32) : Bool := state == 5 || state == 6

@[export tv_tee_device_matches]
def deviceMatches (major minor raw : UInt64) (character : Bool) : Bool :=
  character && major == (((raw >>> 8) &&& (0xfff : UInt64)) ||| ((raw >>> 32) &&& (0xfffff000 : UInt64))) &&
    minor == ((raw &&& (0xff : UInt64)) ||| ((raw >>> 12) &&& (0xffffff00 : UInt64)))

/- Unsigned token states: 0 start, 1 '+', 2 leading zero, 3 decimal,
4 '0x', 5 hex digits, 6 '0x+', 8 signed leading zero, 255 invalid. -/
private def numberGood (state : UInt32) : Bool := state == 2 || state == 3 || state == 5 || state == 8
private def numberStep (state : UInt32) (n : UInt64) (b : UInt32) : UInt32 :=
  if state == 0 || state == 1 then
    if b == 43 && state == 0 then 1
    else if b == 48 then if state == 1 then 8 else 2
    else if dec b then 3 else 255
  else if state == 2 || state == 3 || state == 8 then
    if b == 120 && state == 2 then 4
    else if dec b && fits n 10 (digit b) then 3 else 255
  else if state == 4 || state == 5 || state == 6 then
    if b == 43 && state == 4 then 6
    else if hex b && fits n 16 (digit b) then 5 else 255
  else 255
private def numberValue (state : UInt32) (n : UInt64) (b : UInt32) : UInt64 :=
  if numberStep state n b == 255 then n
  else if (state == 4 || state == 5 || state == 6) && hex b then n * 16 + digit b
  else if dec b then n * 10 + digit b else n

/- Low byte: 0 leading space, 1 first '0', 2 first '5', 3 first '54',
4 before fd, 5 fd token, 6 before request, 7 request token, 8 remaining
kernel arguments, 255 invalid. High byte is the unsigned-token state.
The vendor reports syscall 0 without ptrace; standard ARM ioctl is 54. -/
private def phase (state : UInt32) : UInt32 := state &&& 255
private def numeric (state : UInt32) : UInt32 := state >>> 8
private def tokenState (p n : UInt32) : UInt32 := if n == 255 then 255 else p ||| (n <<< 8)

@[export tv_tee_receive_step]
def receiveStep (state : UInt32) (fd request : UInt64) (b : UInt32) : UInt32 :=
  let p := phase state
  if b > 127 then 255 else
  if p == 0 then
    if space b then 0 else if b == 48 then 1 else if b == 53 then 2 else 255
  else if p == 1 || p == 3 then if space b then 4 else 255
  else if p == 2 then if b == 52 then 3 else 255
  else if p == 4 then
    if space b then 4 else tokenState 5 (numberStep 0 0 b)
  else if p == 5 then
    if space b then if numberGood (numeric state) then 6 else 255
    else tokenState 5 (numberStep (numeric state) fd b)
  else if p == 6 then
    if space b then 6 else tokenState 7 (numberStep 0 0 b)
  else if p == 7 then
    if space b then if numberGood (numeric state) then 8 else 255
    else tokenState 7 (numberStep (numeric state) request b)
  else if p == 8 then 8 else 255

@[export tv_tee_receive_fd]
def receiveFd (state : UInt32) (fd : UInt64) (b : UInt32) : UInt64 :=
  if phase state == 4 then if space b then fd else numberValue 0 0 b
  else if phase state == 5 then if space b then fd else numberValue (numeric state) fd b
  else fd

@[export tv_tee_receive_request]
def receiveRequest (state : UInt32) (request : UInt64) (b : UInt32) : UInt64 :=
  if phase state == 6 then if space b then request else numberValue 0 0 b
  else if phase state == 7 then if space b then request else numberValue (numeric state) request b
  else request

def receiveReady (state : UInt32) (fd request : UInt64) : Bool :=
  (phase state == 8 || (phase state == 7 && numberGood (numeric state))) &&
    fd <= 0xffffffff && request == 0x8010a406

/-- Zero means refusal; an accepted fd is encoded as fd + 1, including fd 0. -/
@[export tv_tee_receive_finish]
def receiveFinish (state : UInt32) (fd request : UInt64) : UInt64 :=
  if receiveReady state fd request then fd + 1 else 0

theorem receiveRequiresRequest (state : UInt32) (fd request : UInt64)
    (h : receiveReady state fd request = true) : request = 0x8010a406 := by
  simp only [receiveReady, Bool.and_eq_true, beq_iff_eq] at h
  exact h.2

theorem receiveRequiresDescriptorBound (state : UInt32) (fd request : UInt64)
    (h : receiveReady state fd request = true) : fd <= 0xffffffff := by
  simp only [receiveReady, Bool.and_eq_true, decide_eq_true_eq] at h
  exact h.1.2

theorem rejectedReceiveHasNoDescriptor (state : UInt32) (fd request : UInt64)
    (h : receiveReady state fd request = false) : receiveFinish state fd request = 0 := by
  simp [receiveFinish, h]

theorem deviceRequiresCharacter (major minor raw : UInt64)
    (h : deviceMatches major minor raw false = true) : False := by
  simp [deviceMatches] at h

theorem invalidDeviceStaysInvalid (major minor : UInt64) (b : UInt32) :
    deviceStep 255 major minor b = 255 := by
  simp [deviceStep]

theorem invalidReceiveStaysInvalid (fd request : UInt64) (b : UInt32) :
    receiveStep 255 fd request b = 255 := by
  simp [receiveStep, phase]

end TV.Hardware.TeeBoundary
