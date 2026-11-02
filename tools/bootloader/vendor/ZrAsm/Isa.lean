/-!
# ZrAsm.Isa: the ARMv7-A (ARM mode) instructions our hand-written assembly needs

The abstract syntax of the supported subset: the
data-processing, multiply, load/store, branch, status-register, coprocessor
and barrier instructions the startup code, the exception vectors, the
netload trampoline, the FSBL handoff stub, the memcpy/memset bursts and the
division helpers use, and the directives around them. Nothing here is Thumb,
NEON, VFP arithmetic or an exception-return form: those are not in the
subset, and `ZrAsm.Sem` leaves a step stuck (`none`) on the corners of the
subset that the architecture calls UNPREDICTABLE.

The AST is what the proofs (ZrAsm.Sem) and the printer (ZrAsm.Print, the `.S`
text the assembler reads) share, so that what is proved is what is assembled.
-/

namespace ZrAsm

/-- A core register: 0-12, 13 = sp, 14 = lr, 15 = pc. -/
abbrev Reg := Fin 16

namespace Reg
def r0 : Reg := 0
def r1 : Reg := 1
def r2 : Reg := 2
def r3 : Reg := 3
def r4 : Reg := 4
def r5 : Reg := 5
def r12 : Reg := 12
def sp : Reg := 13
def lr : Reg := 14
def pc : Reg := 15
end Reg

/-- The condition field of every ARM-mode instruction. -/
inductive Cond
  | eq | ne | cs | cc | mi | pl | vs | vc | hi | ls | ge | lt | gt | le | al
  deriving DecidableEq, Repr, Inhabited

inductive ShiftKind
  | lsl | lsr | asr | ror
  deriving DecidableEq, Repr, Inhabited

/-- A shift by an immediate amount, as the register operand forms encode
it: `lsl 0..31`, `lsr 1..32`, `asr 1..32`, `ror 1..31`, and `rrx`
(`ror #0`'s encoding). `lsl 0` is "no shift". -/
inductive Shift
  | lsl (n : Nat)
  | lsr (n : Nat)
  | asr (n : Nat)
  | ror (n : Nat)
  | rrx
  deriving DecidableEq, Repr, Inhabited

/-- The flexible second operand of a data-processing instruction. `imm v rot`
is the 8-bit constant `v` rotated right by `2 * rot` (the encoding's own
fields, since the shifter's carry-out depends on `rot = 0`); the printer
writes the value and the assembler picks the encoding with the smallest
`rot`, which `Op2.ofImm32` reproduces. -/
inductive Op2
  | imm (v : UInt8) (rot : Nat)
  | reg (rm : Reg) (sh : Shift)
  | regShift (rm : Reg) (k : ShiftKind) (rs : Reg)
  deriving DecidableEq, Repr, Inhabited

inductive DpOp
  | and | eor | sub | rsb | add | adc | sbc | rsc
  | tst | teq | cmp | cmn | orr | mov | bic | mvn
  deriving DecidableEq, Repr, Inhabited

/-- `tst teq cmp cmn` write no register and always set the flags. -/
def DpOp.isTest : DpOp → Bool
  | .tst | .teq | .cmp | .cmn => true
  | _ => false

/-- `mov mvn` have no first operand. -/
def DpOp.isMove : DpOp → Bool
  | .mov | .mvn => true
  | _ => false

/-- The offset of a load/store: a 12-bit immediate, or a (shifted) register,
added or subtracted. -/
inductive Offset
  | imm (add : Bool) (v : Nat)
  | reg (add : Bool) (rm : Reg) (sh : Shift)
  deriving DecidableEq, Repr, Inhabited

/-- `[rn, off]`, `[rn, off]!` and `[rn], off`. -/
inductive Index
  | offset | pre | post
  deriving DecidableEq, Repr, Inhabited

structure Addr where
  rn : Reg
  off : Offset
  idx : Index
  deriving DecidableEq, Repr, Inhabited

/-- Load/store multiple addressing: increment after/before, decrement
after/before. `push` is `stm db sp!`, `pop` is `ldm ia sp!`. -/
inductive MultMode
  | ia | ib | da | db
  deriving DecidableEq, Repr, Inhabited

/-- A branch target: a symbol (resolved by `ZrAsm.layout`), or the encoded
offset itself: the target is `pc + 8 + 4 * n` (`pc` the address of the
branch). -/
inductive Target
  | label (s : String)
  | rel (n : Int)
  deriving DecidableEq, Repr, Inhabited

/-- A coprocessor register by its operands, `p<cp>, <opc1>, rt, c<crn>, c<crm>, <opc2>`. -/
structure CpReg where
  cp : Nat
  opc1 : Nat
  crn : Nat
  crm : Nat
  opc2 : Nat
  deriving DecidableEq, Repr, Inhabited

/-- The VFP system registers `vmrs`/`vmsr` reach (enable_fpu: FPEXC). -/
inductive VfpSys
  | fpscr | fpexc
  deriving DecidableEq, Repr, Inhabited

/-- The source of `msr`: an immediate or a plain register. -/
inductive MsrSrc
  | imm (v : UInt8) (rot : Nat)
  | reg (rm : Reg)
  deriving DecidableEq, Repr, Inhabited

/-- The instructions (the condition is `Item.insn`'s). Operand orders as
in assembly: destination first. For `tst teq cmp cmn` the `rd` of `dp` is
ignored (print as 0), for `mov mvn` the `rn` is. -/
inductive Instr
  | dp (op : DpOp) (s : Bool) (rd rn : Reg) (o : Op2)
  | movw (rd : Reg) (v : UInt16)
  | movt (rd : Reg) (v : UInt16)
  /-- `movw rd, #:lower16:sym` / `movt rd, #:upper16:sym`: an address of a
  label, which `ZrAsm.layout` resolves (position-independent code loads
  a link-time address with this pair instead of a literal pool). -/
  | movwSym (rd : Reg) (sym : String)
  | movtSym (rd : Reg) (sym : String)
  | mul (s : Bool) (rd rm rs : Reg)
  | mla (s : Bool) (rd rm rs ra : Reg)
  /-- `umull umlal smull smlal`: `rdlo, rdhi, rm, rs`. -/
  | mulLong (signed acc s : Bool) (rdlo rdhi rm rs : Reg)
  | clz (rd rm : Reg)
  /-- `ldr`/`ldrb`: `byte` selects the byte form. -/
  | ldr (byte : Bool) (rt : Reg) (a : Addr)
  | str (byte : Bool) (rt : Reg) (a : Addr)
  /-- Local G13 extension: volatile halfword accesses. -/
  | ldrh (rt : Reg) (a : Addr)
  | strh (rt : Reg) (a : Addr)
  | ldm (m : MultMode) (rn : Reg) (wb : Bool) (regs : List Reg)
  | stm (m : MultMode) (rn : Reg) (wb : Bool) (regs : List Reg)
  | b (t : Target)
  | bl (t : Target)
  | bx (rm : Reg)
  | blx (rm : Reg)
  | mrsCpsr (rd : Reg)
  /-- `msr cpsr_<fields>, src`: `f` the flag field (bits 31:24), `c` the control field (7:0). -/
  | msrCpsr (f c : Bool) (src : MsrSrc)
  /-- `cpsie`/`cpsid` with the `i` and `f` bits. -/
  | cps (enable i f : Bool)
  | mcr (r : CpReg) (rt : Reg)
  | mrc (r : CpReg) (rt : Reg)
  | vmsr (sys : VfpSys) (rt : Reg)
  | vmrs (rt : Reg) (sys : VfpSys)
  | dsb | dmb | isb
  | nop | wfe | wfi | sev
  /-- Supervisor exception; used by the Linux-user test entry. -/
  | svc (imm : UInt32)
  /-- Local G13 extension: secure monitor call. -/
  | smc (imm : UInt32)
  deriving DecidableEq, Repr, Inhabited

/-- `push {regs}` and `pop {regs}`. -/
def Instr.push (regs : List Reg) : Instr := .stm .db Reg.sp true regs
def Instr.pop (regs : List Reg) : Instr := .ldm .ia Reg.sp true regs

/-- A constant for the data directives. -/
inductive Const
  | num (v : UInt32)
  | sym (s : String)
  deriving DecidableEq, Repr, Inhabited

/-- One line of an assembly file. -/
inductive Item
  | insn (c : Cond) (i : Instr)
  | label (s : String)
  | global (s : String)
  | weak (s : String)
  /-- `.type s, %function` -/
  | funcType (s : String)
  /-- `.section name, "flags"` -/
  | section (name flags : String)
  | text
  | arm
  | balign (n : Nat)
  | word (v : Const)
  | zero (n : Nat)
  | ltorg
  deriving DecidableEq, Repr, Inhabited

/-- Unconditional instruction. -/
def Item.i (i : Instr) : Item := .insn .al i

/-- `rd := v` for any 32-bit `v`: the `movw`/`movt` pair (no literal pool,
so the code stays position independent). -/
def loadImm32 (rd : Reg) (v : UInt32) : List Item :=
  [.i (.movw rd (v &&& 0xFFFF).toUInt16), .i (.movt rd (v >>> 16).toUInt16)]

/-- Rotate right / left by `n` bits (mod 32). -/
def ror32 (x : UInt32) (n : Nat) : UInt32 :=
  let m := (n % 32).toUInt32
  if m = 0 then x else (x >>> m) ||| (x <<< (32 - m))

def rol32 (x : UInt32) (n : Nat) : UInt32 := ror32 x (32 - n % 32)

/-- The smallest-rotation encoding of a 32-bit constant as a rotated 8-bit
immediate, if there is one (what the assembler picks). -/
def Op2.ofImm32 (x : UInt32) : Option Op2 :=
  go 0 16
where
  go (rot : Nat) : Nat → Option Op2
    | 0 => none
    | fuel + 1 =>
      let r := rol32 x (2 * rot)
      if r < 256 then some (.imm r.toUInt8 rot) else go (rot + 1) fuel

end ZrAsm
