prelude
import Init.System.IO
import Init.Data.List.Lemmas
set_option compiler.extract_closed false

/-! Data exchanged by the UI model and its rendering/transport effects. -/
namespace TV.View

inductive Key where
  | up | down | left | right | ok | back | home | power | youtube
  | digit (value : Nat)
  | red | green | other
  deriving BEq, Repr, Inhabited

def keyOfCode (kind code : UInt32) : Key :=
  if kind == 0 then .home else if kind == 1 then .power else
  match code.toNat with
  | 103 => .up | 108 => .down | 105 => .left | 106 => .right
  | 28 | 96 | 352 | 353 => .ok
  | 158 | 1 | 14 | 174 => .back
  | 102 | 172 => .home | 116 => .power | 569 => .youtube
  | 2 => .digit 1 | 3 => .digit 2 | 4 => .digit 3 | 5 => .digit 4
  | 6 => .digit 5 | 7 => .digit 6 | 8 => .digit 7 | 9 => .digit 8
  | 10 => .digit 9 | 11 => .digit 0
  | 398 => .red | 399 => .green | _ => .other

def directional : Key → Bool
  | .up | .down | .left | .right => true
  | _ => false

/-- Split scalar wire words avoid an unaligned boxed UInt64 on 32-bit Lean. -/
structure Choice where
  low : UInt32
  high : UInt32
  deriving BEq, Repr, Inhabited

/-- The existing Core menu-choice wire format; PIN fields are user input only. -/
def choiceWord (kind app typed fresh : UInt32) : Choice :=
  ⟨(2 : UInt32) ||| (kind <<< 8) ||| (app <<< 16), typed ||| (fresh <<< 16)⟩

/-- Colors are RGB24; the renderer supplies the display format's opaque alpha. -/
inductive Draw where
  | gradient (top bottom : UInt32)
  | rect (x y : Int) (width height : Nat) (rgb : UInt32)
  | frame (x y : Int) (width height thickness : Nat) (rgb : UInt32)
  | dim (x y : Int) (width height : Nat)
  | text (x y : Int) (scale : Nat) (rgb : UInt32) (text : String)
  | art (x y : Int) (width height : Nat) (name : String)
  deriving BEq, Repr, Inhabited

def white : UInt32 := 0xf0f0f5
def grey : UInt32 := 0xaaadc3
def accent : UInt32 := 0x43c1ba

def clipped (text : String) (count : Nat) : String :=
  (text.toSlice.take count).toString

structure Network where
  ssid : String
  signal : String
  secure : Bool
  deriving BEq, Repr, Inhabited

structure Wifi where
  networks : Array Network := #[]
  status : String := "Select Scan to find Wi-Fi networks"
  connection : String := "Wi-Fi state has not been read"
  deriving BEq, Repr, Inhabited

inductive Effect where
  | choice (word : Choice)
  | scan | disconnect
  | connect (network : Network) (password : String)
  deriving BEq, Repr, Inhabited

end TV.View
