/-!
# ZrAsm.Isa: the ARMv7-A (ARM mode) instructions our hand-written assembly needs

The abstract syntax of the supported subset: the
data-processing, multiply, load/store, branch, status-register, coprocessor
and barrier instructions the startup code, the exception vectors, the
netload trampoline, the FSBL handoff stub, the memcpy/memset bursts and the
division helpers use, and the directives around them; plus what FFmpeg's
32-bit ARM H.264 assembly (`ZrAsm.Ffmpeg`) needs: the halfword, signed and
doubleword loads and stores, `pld`, `adr`, literal loads, the ARMv6 media
instructions (parallel add/subtract, `usada8`, `smlad`, `uxtb16`, `setend`)
and Advanced SIMD (NEON): the 64- and 128-bit registers, element types,
scalars, the element and structure loads/stores with alignment and
post-increment, and the data-processing forms by operand shape.

Nothing here is Thumb or VFP arithmetic. `ZrAsm.Sem` interprets the scalar
core subset and leaves a step stuck (`none`) on the corners the architecture
calls UNPREDICTABLE and on everything it does not model: NEON, the media
instructions and the FFmpeg-only load/store forms are printed and assembled
(`ZrAsm.Print`) but not interpreted.

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
  /-- A numeric local label of GNU syntax: `1b` (the nearest `1:` before)
  or `1f` (the nearest after). `ZrAsm.layout` does not resolve them. -/
  | local (n : Nat) (forward : Bool)
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

/-! ### FFmpeg's scalar additions -/

/-- The "miscellaneous" loads and stores (8-bit immediate or plain register
offset): halfword, signed halfword, signed byte, and doubleword (`rt` and
`rt + 1`, `rt` even). -/
inductive MemX
  | h | sh | sb | d
  deriving DecidableEq, Repr, Inhabited

/-- ARMv6 parallel add/subtract and `sel`: `op rd, rn, rm`. -/
inductive ParOp
  | uadd8 | usub8 | uadd16 | usub16 | uhadd8 | uhsub8 | uhadd16 | uhsub16
  | uqadd8 | uqsub8 | sadd8 | ssub8 | shadd8 | qadd8 | qsub8 | usad8 | sel
  deriving DecidableEq, Repr, Inhabited

/-- Four-register multiply-accumulate forms: `op rd, rn, rm, ra`. -/
inductive Mul4Op
  | usada8 | smlad | smladx | smlsd | smlabb | smlabt | smlatb | smlatt | smlawb | smlawt
  deriving DecidableEq, Repr, Inhabited

/-- The extend instructions: `op rd, rm{, ror #rot}` (rot 0, 8, 16, 24). -/
inductive ExtOp
  | uxtb | uxth | uxtb16 | sxtb | sxth | sxtb16
  deriving DecidableEq, Repr, Inhabited

/-! ### Advanced SIMD (NEON) -/

/-- A 64-bit extension register, d0-d31. -/
abbrev DReg := Fin 32

/-- A 128-bit NEON register, q0-q15; `qn` is `d(2n)` (low) and `d(2n+1)`. -/
abbrev QReg := Fin 16

/-- A NEON register operand: a doubleword or a quadword. -/
inductive VReg
  | d (n : DReg)
  | q (n : QReg)
  deriving DecidableEq, Repr, Inhabited

/-- The doublewords a register operand covers, low first. -/
def VReg.dregs : VReg → List DReg
  | .d n => [n]
  | .q n => [⟨2 * n.val, by omega⟩, ⟨2 * n.val + 1, by omega⟩]

/-- The kind of an element type: `x` the untyped size alone (`vld1.8`,
`vext.8`), `i` integer of either sign, `s`/`u` signed/unsigned, `f` float,
`p` polynomial. -/
inductive DtKind
  | x | i | s | u | f | p
  deriving DecidableEq, Repr, Inhabited

/-- The element data type suffix of a NEON instruction: `.8`, `.i16`, `.u8`,
`.s32`, ... -/
structure Dt where
  kind : DtKind
  bits : Nat
  deriving DecidableEq, Repr, Inhabited

/-- A scalar: one lane of a doubleword register, `d0[1]`. -/
structure Scalar where
  reg : DReg
  lane : Nat
  deriving DecidableEq, Repr, Inhabited

/-- The register list of an element or structure load/store:
whole registers `{d0, d1}`, one lane of each `{d0[1], d1[1]}`, or one element
loaded into all lanes `{d0[], d1[]}`. -/
inductive VList
  | regs (l : List DReg)
  | lane (l : List DReg) (lane : Nat)
  | dup (l : List DReg)
  deriving DecidableEq, Repr, Inhabited

/-- Post-increment of a NEON load/store: none, by the transfer size (`!`),
or by a register (`, rm`). -/
inductive VPost
  | none
  | wb
  | reg (rm : Reg)
  deriving DecidableEq, Repr, Inhabited

/-- `[rn{:align}]` and its post-increment. `align` is in bits (0: none). -/
structure VAddr where
  rn : Reg
  align : Nat := 0
  post : VPost := .none
  deriving DecidableEq, Repr, Inhabited

/-- Three-register forms, all shapes: same length (`vadd.i16 q0, q1, q2`),
long (`vaddl.u8 q0, d2, d3`), wide (`vaddw.u8 q0, q0, d2`) and narrow
(`vaddhn.i16 d0, q1, q2`). The assembler checks the shape. -/
inductive V3Op
  | vadd | vsub | vaddl | vsubl | vaddw | vsubw | vaddhn | vsubhn | vraddhn | vrsubhn
  | vhadd | vhsub | vrhadd | vqadd | vqsub
  | vabd | vabdl | vaba | vabal | vmax | vmin | vpadd | vpmax | vpmin
  | vmul | vmla | vmls | vmull | vmlal | vmlsl | vqdmulh | vqrdmulh | vqdmull
  | vand | vbic | vorr | vorn | veor | vbsl | vbit | vbif
  | vceq | vcge | vcgt | vcle | vclt | vtst
  | vshl | vrshl | vqshl | vqrshl
  deriving DecidableEq, Repr, Inhabited

/-- Two-register forms (`vmovl.u8 q0, d0`, `vtrn.32 d0, d1`, `vmov q0, q1`). -/
inductive V2Op
  | vmov | vmvn | vswp | vtrn | vuzp | vzip | vrev16 | vrev32 | vrev64
  | vpaddl | vpadal | vabs | vneg | vqabs | vqneg | vcls | vclz | vcnt
  | vmovl | vmovn | vqmovn | vqmovun
  deriving DecidableEq, Repr, Inhabited

/-- Comparisons against zero, `vclt.s8 q0, q1, #0`. -/
inductive VCmpZOp
  | vceq | vcge | vcgt | vcle | vclt
  deriving DecidableEq, Repr, Inhabited

/-- Two registers and a shift (`vshr.s16 q0, q1, #2`, `vrshrn.i16 d0, q1, #3`). -/
inductive VShOp
  | vshl | vshr | vrshr | vsra | vrsra | vsli | vsri | vqshl | vqshlu
  | vshrn | vrshrn | vqshrn | vqrshrn | vqshrun | vqrshrun | vshll
  deriving DecidableEq, Repr, Inhabited

/-- One register and a modified immediate (`vmov.i16 q0, #28`). -/
inductive VImmOp
  | vmov | vmvn | vorr | vbic
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
  /-- Supervisor exception (Linux system calls, the Linux-user test entry). -/
  | svc (imm : UInt32)
  -- FFmpeg's scalar additions.
  /-- `movw rd, #:lower16:sym+off` / `movt rd, #:upper16:sym+off`. -/
  | movwSymOff (rd : Reg) (sym : String) (off : Int)
  | movtSymOff (rd : Reg) (sym : String) (off : Int)
  /-- `ldrh ldrsh ldrsb ldrd` / `strh strd` (`MemX`); `ldrd` loads `rt`, `rt + 1`. -/
  | ldrx (k : MemX) (rt : Reg) (a : Addr)
  | strx (k : MemX) (rt : Reg) (a : Addr)
  /-- `ldr rt, =v`: a load from the literal pool the assembler places at
  the end of the section. -/
  | ldrLit (rt : Reg) (v : UInt32)
  /-- `adr rd, label`: `rd := label` as `add`/`sub rd, pc, #k`. -/
  | adr (rd : Reg) (t : Target)
  /-- Preload data (a hint). -/
  | pld (a : Addr)
  | par (op : ParOp) (rd rn rm : Reg)
  | mul4 (op : Mul4Op) (rd rn rm ra : Reg)
  | ext (op : ExtOp) (rd rm : Reg) (rot : Nat)
  /-- `setend be` / `setend le`: data endianness. -/
  | setend (be : Bool)
  -- Advanced SIMD. Unconditional in ARM mode: the condition must be `al`.
  | v3 (op : V3Op) (dt : Option Dt) (d n m : VReg)
  /-- By scalar: `vmla.i16 q0, q1, d6[1]`, `vmull.u16 q0, d1, d2[0]`. -/
  | v3s (op : V3Op) (dt : Dt) (d n : VReg) (m : Scalar)
  | v2 (op : V2Op) (dt : Option Dt) (d m : VReg)
  | vcmpz (op : VCmpZOp) (dt : Dt) (d m : VReg)
  | vsh (op : VShOp) (dt : Dt) (d m : VReg) (imm : Nat)
  | vimm (op : VImmOp) (dt : Dt) (d : VReg) (v : Nat)
  /-- `vext.<size> d, n, m, #imm` -/
  | vext (dt : Dt) (d n m : VReg) (imm : Nat)
  /-- `vdup.<size> d, rt` and `vdup.<size> d, dm[x]`. -/
  | vdup (dt : Dt) (d : VReg) (rt : Reg)
  | vdupLane (dt : Dt) (d : VReg) (m : Scalar)
  /-- `vmov.<dt> dn[x], rt` and `vmov.<dt> rt, dn[x]`. -/
  | vmovToLane (dt : Dt) (s : Scalar) (rt : Reg)
  | vmovFromLane (dt : Dt) (rt : Reg) (s : Scalar)
  /-- `vmov dm, rt, rt2` and `vmov rt, rt2, dm`. -/
  | vmovToD (dm : DReg) (rt rt2 : Reg)
  | vmovFromD (rt rt2 : Reg) (dm : DReg)
  /-- `vld<n>.<size> list, addr` / `vst<n>`: element and structure loads and stores, `n` 1-4. -/
  | vld (n : Nat) (dt : Dt) (l : VList) (a : VAddr)
  | vst (n : Nat) (dt : Dt) (l : VList) (a : VAddr)
  /-- `vpush {list}` = `vstmdb sp!`, `vpop {list}` = `vldmia sp!`. -/
  | vpush (l : List DReg)
  | vpop (l : List DReg)
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
  -- Used by FFmpeg's files (`ZrAsm.Ffmpeg`).
  /-- A numeric local label `n:` (`Target.local`). -/
  | localLabel (n : Nat)
  /-- `.size s, . - s`: the symbol's size is up to here. -/
  | size (s : String)
  | byte (l : List UInt8)
  | short (l : List UInt16)
  /-- `.arch`, `.object_arch`, `.fpu`, `.eabi_attribute tag, v`, `.syntax unified`. -/
  | arch (s : String)
  | objectArch (s : String)
  | fpu (s : String)
  | eabiAttribute (tag v : Nat)
  | syntaxUnified
  /-- `.section .note.GNU-stack,"",%progbits`: the stack is not executable. -/
  | noteGnuStack
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
