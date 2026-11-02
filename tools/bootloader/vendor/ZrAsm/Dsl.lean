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
def global (s : String) : Asm Unit := emit (.global s)

private def dp (op : DpOp) (s : Bool) (rd rn : Reg) (o : Op2) (c : Cond) : Asm Unit := ins (.dp op s rd rn o) c

/-! Data processing. The final `c` argument is the condition (`(c := .ne)`). -/
def mov (rd : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .mov false rd 0 o c
def mvn (rd : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .mvn false rd 0 o c
def add (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .add false rd rn o c
def sub (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .sub false rd rn o c
def subs (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .sub true rd rn o c
def orr (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .orr false rd rn o c
def bic (rd rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .bic false rd rn o c
def tst (rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .tst true 0 rn o c
def cmp (rn : Reg) (o : Op2) (c : Cond := .al) : Asm Unit := dp .cmp true 0 rn o c
def movw (rd : Reg) (v : UInt16) : Asm Unit := ins (.movw rd v)
def movt (rd : Reg) (v : UInt16) : Asm Unit := ins (.movt rd v)
/-- `rd := v` (a `movw`/`movt` pair). -/
def loadImm (rd : Reg) (v : UInt32) : Asm Unit := emitAll (loadImm32 rd v)
/-- `rd := &sym` (a `movw`/`movt` pair with the symbol's halves). -/
def loadSym (rd : Reg) (sym : String) : Asm Unit := do
  ins (.movwSym rd sym)
  ins (.movtSym rd sym)

/-! Loads and stores, word and byte, immediate offset. -/
private def at_ (rn : Reg) (off : Int) (idx : Index) : Addr :=
  { rn, off := .imm (off ≥ 0) off.natAbs, idx }
/-- `ldr rt, [rn, #off]` -/
def ldr (rt rn : Reg) (off : Int := 0) : Asm Unit := ins (.ldr false rt (at_ rn off .offset))
/-- `str rt, [rn, #off]` -/
def str (rt rn : Reg) (off : Int := 0) : Asm Unit := ins (.str false rt (at_ rn off .offset))
/-- `ldr rt, [rn], #off` -/
def ldrPost (rt rn : Reg) (off : Int) : Asm Unit := ins (.ldr false rt (at_ rn off .post))
/-- `str rt, [rn], #off` -/
def strPost (rt rn : Reg) (off : Int) : Asm Unit := ins (.str false rt (at_ rn off .post))
/-- `ldmia rn!, {regs}` -/
def ldmia (rn : Reg) (regs : List Reg) : Asm Unit := ins (.ldm .ia rn true regs)
/-- `stmia rn!, {regs}` -/
def stmia (rn : Reg) (regs : List Reg) : Asm Unit := ins (.stm .ia rn true regs)
def push (regs : List Reg) : Asm Unit := ins (.push regs)
def pop (regs : List Reg) : Asm Unit := ins (.pop regs)

/-! Branches. -/
def b (l : String) (c : Cond := .al) : Asm Unit := ins (.b (.label l)) c
def bl (l : String) (c : Cond := .al) : Asm Unit := ins (.bl (.label l)) c
def bx (r : Reg) (c : Cond := .al) : Asm Unit := ins (.bx r) c

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
