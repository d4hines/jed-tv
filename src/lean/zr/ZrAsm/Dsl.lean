import ZrAsm.Isa
import ZrAsm.Print

/-!
# ZrAsm.Dsl: writing routines in Lean

The assembly the firmware and the FSBL need (src/lean/ZrAsm/Isa.lean) is written
here as ordinary Lean in the `Asm` monad, one instruction per line, over
the AST of `ZrAsm.Isa`:

    def zr_hw_isb : Routine := routine "zr_hw_isb" do
      isb
      mov r0 (imm 1)
      bx lr

A `Routine` is a name, an ELF section, an alignment and its items (labels,
instructions, directives). `Routine.items` adds the header (`.section`,
`.arm`, `.global`, `.type`, the label) and `ZrAsm.printItems` prints the
`.S` text the generator (`ZrAsmGen`, the Nix package `asm-gen`) writes and
clang assembles: so what the model (`ZrAsm.Sem`) runs and the proofs are
about is what the board runs. Branch targets inside a routine are named
labels (`.L...`, unique within a file; the model's `layout` resolves them,
numeric `1b` forms it cannot); a routine meant to be run in the model has no
`.section` directive in its `body`.

Immediates are the assembler's: `imm v` is the 8-bit-rotated encoding of
`v`, and fails (at generation time) for a value that has none, which is the
error the assembler would give. A constant of any size is `loadImm32` or
`loadSym` (a `movw`/`movt` pair: no literal pools, so no `.ltorg` and the
code stays position independent).
-/

namespace ZrAsm

namespace Reg
/-- `ip` is r12, the intra-procedure scratch register. -/
def ip : Reg := 12
def r6 : Reg := 6
def r7 : Reg := 7
def r8 : Reg := 8
def r9 : Reg := 9
def r10 : Reg := 10
def r11 : Reg := 11
end Reg

/- The CP15 registers our assembly reads and writes, by operands
(`mcr p15, opc1, rt, c<crn>, c<crm>, opc2`). -/
namespace Cp15
private def r (opc1 crn crm opc2 : Nat) : CpReg := ⟨15, opc1, crn, crm, opc2⟩
/-- TLBIALL, ICIALLU, BPIALL: invalidate all (the value written is ignored). -/
def tlbiall := r 0 8 7 0
def iciallu := r 0 7 5 0
def bpiall := r 0 7 5 6
/-- Data cache maintenance: by set/way (DCISW invalidate, DCCISW clean and
invalidate) and by address (DCCMVAC clean, DCIMVAC invalidate, DCCIMVAC both). -/
def dcisw := r 0 7 6 2
def dccisw := r 0 7 14 2
def dccmvac := r 0 7 10 1
def dcimvac := r 0 7 6 1
def dccimvac := r 0 7 14 1
/-- Cache size selection. -/
def csselr := r 2 0 0 0
def sctlr := r 0 1 0 0
def actlr := r 0 1 0 1
def cpacr := r 0 1 0 2
def ttbr0 := r 0 2 0 0
def dacr := r 0 3 0 0
def dfsr := r 0 5 0 0
def ifsr := r 0 5 0 1
def dfar := r 0 6 0 0
def ifar := r 0 6 0 2
def mpidr := r 0 0 0 5
def vbar := r 0 12 0 0
end Cp15

/-- The 8-bit rotated immediate with the value `v`; a value with no such
encoding is an error at generation time. -/
def imm (v : UInt32) : Op2 :=
  match Op2.ofImm32 v with
  | some o => o
  | none => panic! s!"ZrAsm.imm: {v} is not a rotated 8-bit immediate"

instance : Coe Reg Op2 := ⟨fun r => .reg r (.lsl 0)⟩

/-- A numeral is an immediate operand: `add r4, r4, #64` is `add r4 r4 64`. -/
instance : OfNat Op2 n := ⟨imm (UInt32.ofNat n)⟩
/-- An immediate operand from a natural-number expression. -/
def Op2.nat (n : Nat) : Op2 := ZrAsm.imm (UInt32.ofNat n)

/-- A register with an immediate shift: `r5.lsl 3` is `r5, lsl #3`, and
`lsr`, `asr`, `ror`. (Not `<<<`: on `Reg = Fin 16` that is `Fin`'s own
shift, a register number.) -/
structure ShReg where
  rm : Reg
  sh : Shift
  deriving Inhabited

def Reg.lsl (r : Reg) (n : Nat) : ShReg := ⟨r, .lsl n⟩
def Reg.lsr (r : Reg) (n : Nat) : ShReg := ⟨r, .lsr n⟩
def Reg.asr (r : Reg) (n : Nat) : ShReg := ⟨r, .asr n⟩
def Reg.ror (r : Reg) (n : Nat) : ShReg := ⟨r, .ror n⟩
instance : Coe ShReg Op2 := ⟨fun s => .reg s.rm s.sh⟩

/-- A load/store offset: an immediate (a numeral, negative allowed) or a
(shifted) register, added or (`-r2`) subtracted. -/
instance : OfNat Offset n := ⟨.imm true n⟩
instance : Coe Reg Offset := ⟨fun r => .reg true r (.lsl 0)⟩
instance : Coe ShReg Offset := ⟨fun s => .reg true s.rm s.sh⟩
instance : Neg Offset := ⟨fun
  | .imm add v => if v = 0 then .imm true 0 else .imm (!add) v
  | .reg add rm sh => .reg (!add) rm sh⟩
/-- An immediate offset from an integer expression. -/
def Offset.int (v : Int) : Offset := .imm (v ≥ 0) v.natAbs

/-- `[rn, off]`, `[rn, off]!`, `[rn], off`; a register alone is `[rn]`. -/
def mem (rn : Reg) (off : Offset := 0) : Addr := { rn, off, idx := .offset }
def pre (rn : Reg) (off : Offset) : Addr := { rn, off, idx := .pre }
def post (rn : Reg) (off : Offset) : Addr := { rn, off, idx := .post }
instance : Coe Reg Addr := ⟨fun r => mem r⟩

/-- A symbol is a branch target. -/
instance : Coe String Target := ⟨.label⟩
/-- GNU numeric local labels: `back 1` is `1b`, `fwd 2` is `2f`. -/
def back (n : Nat) : Target := .local n false
def fwd (n : Nat) : Target := .local n true

/-- The instructions written so far, in order. -/
abbrev Asm := StateM (Array Item)

/-- An assembly routine: a symbol in its own ELF section (so the linker's
`--gc-sections` drops it when nothing refers to it), its alignment, and the
items after the label. -/
structure Routine where
  name : String
  sect : String
  align : Nat := 4
  body : List Item
  /-- Data uses writable/readonly sections without a function type. -/
  flags : String := "ax"
  isCode : Bool := true
  deriving Inhabited

/-- The directives that start a routine, then its body. -/
def Routine.items (r : Routine) : List Item :=
  [.section r.sect r.flags] ++ (if r.isCode then [.arm] else []) ++
  (if r.align > 4 || !r.isCode then [.balign r.align] else []) ++
  [.global r.name] ++ (if r.isCode then [.funcType r.name] else []) ++ [.label r.name] ++ r.body

/-- Build a routine from `Asm` code. The section defaults to `.text.<name>`. -/
def routine (name : String) (code : Asm Unit) (sect : Option String := none) (align : Nat := 4) : Routine :=
  { name, sect := sect.getD s!".text.{name}", align, body := (code.run #[]).2.toList }

/-- Fixed zero-initialized storage in its own BSS section (ARM byte sizes). -/
def storage (name : String) (bytes : Nat) (align : Nat := 4)
    (sect : Option String := none) : Routine :=
  { name, sect := sect.getD s!".bss.{name}", align, body := [.zero bytes], flags := "aw", isCode := false }

/-- Readonly words; useful for byte strings and ABI constants without C data. -/
def roWords (name : String) (words : List UInt32) (sect : Option String := none) : Routine :=
  { name, sect := sect.getD s!".rodata.{name}", body := words.map (Item.word ∘ Const.num),
    flags := "a", isCode := false }

/-- A byte string, NUL-terminated and padded to a word boundary. -/
def roString (name value : String) (sect : Option String := none) : Routine :=
  let b := value.toUTF8.push 0
  let words := (List.range ((b.size + 3) / 4)).map fun i =>
    let octet (j : Nat) : UInt32 := (b[j]?.getD 0).toUInt32
    octet (4*i) ||| (octet (4*i+1) <<< 8) ||| (octet (4*i+2) <<< 16) ||| (octet (4*i+3) <<< 24)
  roWords name words sect

namespace Asm

def emit (i : Item) : Asm Unit := modify (·.push i)
def emitAll (l : List Item) : Asm Unit := l.forM emit
def ins (i : Instr) (c : Cond := .al) : Asm Unit := emit (.insn c i)
def label (s : String) : Asm Unit := emit (.label s)
/-- A numeric local label `n:`, the target of `back n` / `fwd n`. -/
def lbl (n : Nat) : Asm Unit := emit (.localLabel n)
def global (s : String) : Asm Unit := emit (.global s)

private def dp (op : DpOp) (s : Bool) (rd rn : Reg) (o : Op2) (c : Cond) : Asm Unit := ins (.dp op s rd rn o) c

/-! Data processing. The final `c` argument is the condition (`(c := .ne)`
or positionally, `subs r0 r0 1 .eq`); the `s` forms set the flags. -/
def mov (rd : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .mov false rd 0 o c
def movs (rd : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .mov true rd 0 o c
def mvn (rd : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .mvn false rd 0 o c
def add (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .add false rd rn o c
def adds (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .add true rd rn o c
def sub (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .sub false rd rn o c
def subs (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .sub true rd rn o c
def rsb (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .rsb false rd rn o c
def rsbs (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .rsb true rd rn o c
def and (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .and false rd rn o c
def ands (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .and true rd rn o c
def eor (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .eor false rd rn o c
def eors (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .eor true rd rn o c
def orr (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .orr false rd rn o c
def orrs (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .orr true rd rn o c
def bic (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .bic false rd rn o c
def bics (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .bic true rd rn o c
def tst (rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .tst true 0 rn o c
def teq (rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .teq true 0 rn o c
def cmp (rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .cmp true 0 rn o c
def cmn (rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .cmn true 0 rn o c

/-- A shift amount: an immediate (a numeral) or a register. -/
inductive ShAmt
  | imm (n : Nat)
  | reg (rs : Reg)
instance : OfNat ShAmt n := ⟨.imm n⟩
instance : Coe Reg ShAmt := ⟨.reg⟩

private def shiftOp (k : ShiftKind) (rm : Reg) : ShAmt → Op2
  | .imm n => .reg rm (match k with | .lsl => .lsl n | .lsr => .lsr n | .asr => .asr n | .ror => .ror n)
  | .reg rs => .regShift rm k rs
/-- `lsl rd, rm, #n` / `lsl rd, rm, rs` (UAL for `mov rd, rm, lsl ...`), and `lsr`, `asr`, `ror`. -/
def lsl (rd rm : Reg) (a : ShAmt) (c : Cond := .al) : Asm Unit := mov rd (shiftOp .lsl rm a) c
def lsr (rd rm : Reg) (a : ShAmt) (c : Cond := .al) : Asm Unit := mov rd (shiftOp .lsr rm a) c
def asr (rd rm : Reg) (a : ShAmt) (c : Cond := .al) : Asm Unit := mov rd (shiftOp .asr rm a) c
def ror (rd rm : Reg) (a : ShAmt) (c : Cond := .al) : Asm Unit := mov rd (shiftOp .ror rm a) c

/-! Multiplies and the ARMv6 media instructions. -/
def mul (rd rm rs : Reg) (c : Cond := .al) : Asm Unit := ins (.mul false rd rm rs) c
def muls (rd rm rs : Reg) (c : Cond := .al) : Asm Unit := ins (.mul true rd rm rs) c
def mla (rd rm rs ra : Reg) (c : Cond := .al) : Asm Unit := ins (.mla false rd rm rs ra) c
def usada8 (rd rn rm ra : Reg) (c : Cond := .al) : Asm Unit := ins (.mul4 .usada8 rd rn rm ra) c
def smlad (rd rn rm ra : Reg) (c : Cond := .al) : Asm Unit := ins (.mul4 .smlad rd rn rm ra) c
def uadd8 (rd rn rm : Reg) (c : Cond := .al) : Asm Unit := ins (.par .uadd8 rd rn rm) c
def uhadd8 (rd rn rm : Reg) (c : Cond := .al) : Asm Unit := ins (.par .uhadd8 rd rn rm) c
def usub16 (rd rn rm : Reg) (c : Cond := .al) : Asm Unit := ins (.par .usub16 rd rn rm) c
/-- `uxtb16 rd, rm{, ror #rot}` -/
def uxtb16 (rd rm : Reg) (rot : Nat := 0) (c : Cond := .al) : Asm Unit := ins (.ext .uxtb16 rd rm rot) c
def setend (be : Bool) : Asm Unit := ins (.setend be)
def movw (rd : Reg) (v : UInt16) : Asm Unit := ins (.movw rd v)
def movt (rd : Reg) (v : UInt16) : Asm Unit := ins (.movt rd v)
/-- `rd := v` (a `movw`/`movt` pair). -/
def loadImm (rd : Reg) (v : UInt32) : Asm Unit := emitAll (loadImm32 rd v)
/-- `rd := &sym` (a `movw`/`movt` pair with the symbol's halves). -/
def loadSym (rd : Reg) (sym : String) : Asm Unit := do
  ins (.movwSym rd sym)
  ins (.movtSym rd sym)

/-! Loads and stores. The address is an `Addr`: a register alone is
`[rn]`, and `mem rn off` is `[rn, off]`, `pre rn off` is `[rn, off]!`,
`post rn off` is `[rn], off`, where `off` is an immediate (`4`, `-1`), a
register (`r2`, `-r2`) or a shifted register (`r2.lsl 2`). -/
def ldr (rt : Reg) (a : Addr) (c : Cond := .al) : Asm Unit := ins (.ldr false rt a) c
def ldrb (rt : Reg) (a : Addr) (c : Cond := .al) : Asm Unit := ins (.ldr true rt a) c
def str (rt : Reg) (a : Addr) (c : Cond := .al) : Asm Unit := ins (.str false rt a) c
def strb (rt : Reg) (a : Addr) (c : Cond := .al) : Asm Unit := ins (.str true rt a) c
/-- `ldr rt, [rn], #off` -/
def ldrPost (rt rn : Reg) (off : Int) : Asm Unit := ldr rt (post rn (Offset.int off))
/-- `str rt, [rn], #off` -/
def strPost (rt rn : Reg) (off : Int) : Asm Unit := str rt (post rn (Offset.int off))
/-- `ldmia rn!, {regs}` -/
def ldrh (rt : Reg) (a : Addr) (c : Cond := .al) : Asm Unit := ins (.ldrx .h rt a) c
def ldrsh (rt : Reg) (a : Addr) (c : Cond := .al) : Asm Unit := ins (.ldrx .sh rt a) c
def ldrsb (rt : Reg) (a : Addr) (c : Cond := .al) : Asm Unit := ins (.ldrx .sb rt a) c
def strh (rt : Reg) (a : Addr) (c : Cond := .al) : Asm Unit := ins (.strx .h rt a) c
/-- `ldrd rt, rt2, addr`: `rt` even and `rt2 = rt + 1` (the encoding has no `rt2`). -/
def ldrd (rt rt2 : Reg) (a : Addr) (c : Cond := .al) : Asm Unit :=
  if rt.val % 2 = 0 ∧ rt2.val = rt.val + 1 then ins (.ldrx .d rt a) c
  else panic! s!"ZrAsm.ldrd: {rt.name}, {rt2.name} is not an even/odd pair"
def strd (rt rt2 : Reg) (a : Addr) (c : Cond := .al) : Asm Unit :=
  if rt.val % 2 = 0 ∧ rt2.val = rt.val + 1 then ins (.strx .d rt a) c
  else panic! s!"ZrAsm.strd: {rt.name}, {rt2.name} is not an even/odd pair"
/-- `ldr rt, =v`: a literal-pool load (the assembler places the pool at the
end of the section). -/
def ldrConst (rt : Reg) (v : UInt32) (c : Cond := .al) : Asm Unit := ins (.ldrLit rt v) c
/-- `adr rd, label` -/
def adr (rd : Reg) (t : Target) (c : Cond := .al) : Asm Unit := ins (.adr rd t) c
def pld (a : Addr) : Asm Unit := ins (.pld a)
def ldmia (rn : Reg) (regs : List Reg) : Asm Unit := ins (.ldm .ia rn true regs)
/-- `stmia rn!, {regs}` -/
def stmia (rn : Reg) (regs : List Reg) : Asm Unit := ins (.stm .ia rn true regs)
/-- `ldm rn, {regs}` (no writeback). -/
def ldm (rn : Reg) (regs : List Reg) : Asm Unit := ins (.ldm .ia rn false regs)
def push (regs : List Reg) : Asm Unit := ins (.push regs)
def pop (regs : List Reg) : Asm Unit := ins (.pop regs)

/-! Branches. The target is a symbol (a string) or a numeric local label,
`back 1` (GNU `1b`) or `fwd 2` (`2f`), placed with `lbl 1`. -/
def b (t : Target) (c : Cond := .al) : Asm Unit := ins (.b t) c
def bl (t : Target) (c : Cond := .al) : Asm Unit := ins (.bl t) c
def bx (r : Reg) (c : Cond := .al) : Asm Unit := ins (.bx r) c
def blx (r : Reg) (c : Cond := .al) : Asm Unit := ins (.blx r) c
def beq (t : Target) : Asm Unit := b t .eq
def bne (t : Target) : Asm Unit := b t .ne
def bcs (t : Target) : Asm Unit := b t .cs
def bcc (t : Target) : Asm Unit := b t .cc
def bhs (t : Target) : Asm Unit := b t .cs
def blo (t : Target) : Asm Unit := b t .cc
def bmi (t : Target) : Asm Unit := b t .mi
def bpl (t : Target) : Asm Unit := b t .pl
def bhi (t : Target) : Asm Unit := b t .hi
def bls (t : Target) : Asm Unit := b t .ls
def bge (t : Target) : Asm Unit := b t .ge
def blt (t : Target) : Asm Unit := b t .lt
def bgt (t : Target) : Asm Unit := b t .gt
def ble (t : Target) : Asm Unit := b t .le

/-! System. -/
def mcr (r : CpReg) (rt : Reg) : Asm Unit := ins (.mcr r rt)
def mrc (r : CpReg) (rt : Reg) : Asm Unit := ins (.mrc r rt)
def vmrs (rt : Reg) (s : VfpSys) : Asm Unit := ins (.vmrs rt s)
def vmsr (s : VfpSys) (rt : Reg) : Asm Unit := ins (.vmsr s rt)
def dsb : Asm Unit := ins .dsb
def dmb : Asm Unit := ins .dmb
def isb : Asm Unit := ins .isb
def nop : Asm Unit := ins .nop
def wfe : Asm Unit := ins .wfe
def cpsidIF : Asm Unit := ins (.cps false true true)

end Asm

/-- A file of routines as `.S` text: the unified-syntax header, a note on
where it came from, then each routine. `ZrAsm.printItems` per item. -/
def Routine.file (what : String) (rs : List Routine) : String :=
  s!"/* {what}\n * Generated by zr-asm-gen (src/lean/ZrAsmGen.lean) from the routines in\n * src/lean/ZrAsm/; not edited by hand (src/lean/ZrAsm/Isa.lean). */\n.syntax unified\n\n" ++
  "\n".intercalate (rs.map fun r => printItems r.items)

end ZrAsm
