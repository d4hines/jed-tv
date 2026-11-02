prelude
import Init.Data.UInt
import Init.Data.ToString.Basic
import Init.Data.Repr

/- Why a mechanism (an ioctl, a socket read, the clock) did not do what the
driver asked. Rust reports each outcome as one word, so no Rust code has to
interpret what happened; Lean does. -/
namespace TV.Audio

inductive Fault where
  /-- The MI call completed but returned a nonzero status. -/
  | vendor (request status : UInt32)
  /-- The system call itself failed. -/
  | os (errno : UInt32)
  /-- The mechanism answered, but with a value the driver refuses to trust. -/
  | invalid (what : String)
  | timeout (what : String)
  /-- The hardware is owned by someone else right now. -/
  | busy (what : String)
  deriving BEq, Repr, Inhabited

/-- `0x` followed by eight hexadecimal digits. -/
def hex (value : UInt32) : String :=
  let digit (v : UInt32) : Char := Char.ofNat (if v < 10 then 48 + v.toNat else 87 + v.toNat)
  let rec go : Nat → UInt32 → List Char → List Char
    | 0, _, acc => acc
    | n + 1, v, acc => go n (v >>> 4) (digit (v &&& 15) :: acc)
  "0x" ++ String.ofList (go 8 value [])

def Fault.describe : Fault → String
  | .vendor request status => "MI request " ++ hex request ++ ": status " ++ toString status
  | .os errno => "errno " ++ toString errno
  | .invalid what => what
  | .timeout what => what ++ ": timed out"
  | .busy what => what

/-- Bits 0..7 of a result word are the tag (0 success, 1 the MI call's nonzero
status, anything else an errno); bits 32..63 carry the value. -/
@[inline] def outcome (request : UInt32) (word : UInt64) : Except Fault UInt32 :=
  match (word &&& 0xff).toUInt32 with
  | 0 => .ok (word >>> 32).toUInt32
  | 1 => .error (.vendor request (word >>> 32).toUInt32)
  | _ => .error (.os (word >>> 32).toUInt32)

instance [BEq ε] [BEq α] : BEq (Except ε α) where
  beq
    | .ok x, .ok y => x == y
    | .error x, .error y => x == y
    | _, _ => false

end TV.Audio
