import ZrAsm.Isa

/-!
# ZrAsm.Sem: an executable semantics of the ZrAsm subset

A state (r0-r15, the flags, the rest of CPSR, memory as a function, an
abstract log) and `step`, one instruction. Everything is computable, so the
model can be run (`#eval`, test/yuv/ScaleRow.lean; dande-router's test/asm
tests it against QEMU),
and unfolded in proofs.

What is interpreted: the data-processing, multiply, `clz`, load/store (word
and byte, unaligned words as ARMv7 with SCTLR.A = 0), load/store multiple,
branch instructions and the flag/control fields of CPSR. What is not: caches,
MMU, barriers, coprocessors, VFP and the mode machinery. Those instructions
only append to `State.log` (newest first), so a proof can state that a routine
performed exactly these effects in this order; a coprocessor *read* returns
`State.cpIn` of the register, an input the proof quantifies over.

`step` returns `none` when the architecture calls the instruction
UNPREDICTABLE or the model does not cover it (Thumb interworking, `pc` as a
store source or multiply operand, an unaligned LDM/STM address, writeback
into the loaded register, ...). A proof that a routine runs therefore also
shows it stays inside the modelled subset.

PC: during `exec` the state's r15 is already the address of the next
instruction (`insn + 4`), so a read of `pc` as an operand is `r15 + 4`
(`insn + 8`) and a branch overwrites r15; between instructions r15 is the
address of the instruction about to run.
-/

namespace ZrAsm

/-- An effect on something the model leaves uninterpreted. -/
inductive Effect
  | mcr (r : CpReg) (v : UInt32)
  | mrc (r : CpReg) (v : UInt32)
  | vmsr (sys : VfpSys) (v : UInt32)
  | vmrs (sys : VfpSys) (v : UInt32)
  | cps (enable i f : Bool)
  | msrC (v : UInt32)
  | dsb | dmb | isb | wfe | wfi | sev
  deriving DecidableEq, Repr

structure State where
  /-- r0-r14; r15 as described above. -/
  r : Fin 16 → UInt32
  n : Bool
  z : Bool
  c : Bool
  v : Bool
  /-- CPSR's bits 27:0 (Q, GE, the I F A bits, the mode ...). -/
  cpsrRest : UInt32
  mem : UInt32 → UInt8
  /-- What a coprocessor / VFP system register read returns. -/
  cpIn : CpReg → UInt32
  vfpIn : VfpSys → UInt32
  /-- Effects on the uninterpreted parts, newest first. -/
  log : List Effect

namespace State

def upd (r : Fin 16 → UInt32) (i : Fin 16) (x : UInt32) : Fin 16 → UInt32 :=
  fun j => if j = i then x else r j

def setR (s : State) (i : Fin 16) (x : UInt32) : State := { s with r := upd s.r i x }

/-- An operand read: `pc` reads as the instruction's address + 8. -/
def get (s : State) (i : Fin 16) : UInt32 := if i = 15 then s.r 15 + 4 else s.r i

def flagsWord (s : State) : UInt32 :=
  (if s.n then 0x80000000 else 0) ||| (if s.z then 0x40000000 else 0) |||
  (if s.c then 0x20000000 else 0) ||| (if s.v then 0x10000000 else 0)

def cpsr (s : State) : UInt32 := s.flagsWord ||| s.cpsrRest

def setFlagsWord (s : State) (x : UInt32) : State :=
  { s with n := x &&& 0x80000000 != 0, z := x &&& 0x40000000 != 0,
           c := x &&& 0x20000000 != 0, v := x &&& 0x10000000 != 0 }

def logEff (s : State) (e : Effect) : State := { s with log := e :: s.log }

end State

/-! ### Memory -/

/-- A little-endian word at any alignment (arithmetic rather than bit
operations, which proofs about copies reason about with `omega`). -/
def rd32 (m : UInt32 → UInt8) (a : UInt32) : UInt32 :=
  UInt32.ofNat ((m a).toNat + 256 * (m (a + 1)).toNat + 65536 * (m (a + 2)).toNat +
    16777216 * (m (a + 3)).toNat)

def wr8 (m : UInt32 → UInt8) (a : UInt32) (x : UInt8) : UInt32 → UInt8 :=
  fun b => if b = a then x else m b

def wr32 (m : UInt32 → UInt8) (a : UInt32) (x : UInt32) : UInt32 → UInt8 :=
  wr8 (wr8 (wr8 (wr8 m a (UInt8.ofNat (x.toNat % 256))) (a + 1) (UInt8.ofNat (x.toNat / 256 % 256)))
    (a + 2) (UInt8.ofNat (x.toNat / 65536 % 256))) (a + 3) (UInt8.ofNat (x.toNat / 16777216 % 256))

/-! ### Conditions, shifts, arithmetic -/

def Cond.holds (s : State) : Cond → Bool
  | .eq => s.z
  | .ne => !s.z
  | .cs => s.c
  | .cc => !s.c
  | .mi => s.n
  | .pl => !s.n
  | .vs => s.v
  | .vc => !s.v
  | .hi => s.c && !s.z
  | .ls => !s.c || s.z
  | .ge => s.n == s.v
  | .lt => s.n != s.v
  | .gt => !s.z && s.n == s.v
  | .le => s.z || s.n != s.v
  | .al => true

def bit (x : UInt32) (i : Nat) : Bool := (x >>> i.toUInt32) &&& 1 != 0

/-- Shift `x` by `amt` bits (the amount already decoded: an immediate
form's 32 for `#32`, a register form's low byte), with the carry-out
(`cin` when `amt = 0`): ARM ARM `Shift_C`. -/
def shiftC (k : ShiftKind) (x : UInt32) (amt : Nat) (cin : Bool) : UInt32 × Bool :=
  if amt = 0 then (x, cin) else
  match k with
  | .lsl => if amt < 32 then (x <<< amt.toUInt32, bit x (32 - amt))
            else if amt = 32 then (0, bit x 0) else (0, false)
  | .lsr => if amt < 32 then (x >>> amt.toUInt32, bit x (amt - 1))
            else if amt = 32 then (0, bit x 31) else (0, false)
  | .asr => if amt < 32 then
              let l := x >>> amt.toUInt32
              ((if bit x 31 then l ||| ~~~(0xFFFFFFFF >>> amt.toUInt32) else l), bit x (amt - 1))
            else (if bit x 31 then 0xFFFFFFFF else 0, bit x 31)
  | .ror => let m := amt % 32
            if m = 0 then (x, bit x 31)
            else ((x >>> m.toUInt32) ||| (x <<< (32 - m).toUInt32), bit x (m - 1))

def Shift.apply (sh : Shift) (x : UInt32) (cin : Bool) : UInt32 × Bool :=
  match sh with
  | .lsl n => shiftC .lsl x n cin
  | .lsr n => shiftC .lsr x n cin
  | .asr n => shiftC .asr x n cin
  | .ror n => shiftC .ror x n cin
  | .rrx => (((if cin then 0x80000000 else 0 : UInt32)) ||| (x >>> 1), bit x 0)

/-- `x + y + cin` with the carry-out and the signed overflow. -/
def addC (x y : UInt32) (cin : Bool) : UInt32 × Bool × Bool :=
  let sum := x.toNat + y.toNat + (if cin then 1 else 0)
  let res := x + y + (if cin then 1 else 0)
  let sx := bit x 31
  let sy := bit y 31
  (res, decide (sum ≥ 2 ^ 32), sx == sy && bit res 31 != sx)

/-- The shifter operand's value and carry-out, or `none` for the forms with
`pc` where the model does not follow the architecture (`pc` as the register
of a register-specified shift). -/
def Op2.eval (s : State) : Op2 → Option (UInt32 × Bool)
  | .imm v rot =>
    let x := ror32 v.toUInt32 (2 * rot)
    some (x, if rot = 0 then s.c else bit x 31)
  | .reg rm sh => some (sh.apply (s.get rm) s.c)
  | .regShift rm k rs =>
    if rm = 15 ∨ rs = 15 then none
    else some (shiftC k (s.get rm) ((s.get rs) &&& 0xFF).toNat s.c)

/-- A branch (or a write of `pc`): ARM-mode target, word aligned. A set bit 0
(Thumb) or bit 1 is outside the model. -/
def branchTo (s : State) (x : UInt32) : Option State :=
  if x &&& 3 = 0 then some (s.setR 15 x) else none

/-- Write a destination register, `pc` being a branch. -/
def writeDest (s : State) (rd : Fin 16) (x : UInt32) : Option State :=
  if rd = 15 then branchTo s x else some (s.setR rd x)

def setNZ (s : State) (x : UInt32) : State := { s with n := bit x 31, z := x == 0 }

/-- Count leading zeros. -/
def clz32 (x : UInt32) : UInt32 :=
  go 32 x 0
where
  go : Nat → UInt32 → UInt32 → UInt32
    | 0, _, acc => acc
    | fuel + 1, x, acc => if bit x 31 then acc else go fuel (x <<< 1) (acc + 1)

def toInt (x : UInt32) : Int := if x ≥ 0x80000000 then (x.toNat : Int) - 2 ^ 32 else x.toNat

/-- The sorted, duplicate-free registers of a list. -/
def regsOf (l : List (Fin 16)) : List (Fin 16) :=
  (List.range 16).filterMap fun i => if h : i < 16 then (if l.contains ⟨i, h⟩ then some ⟨i, h⟩ else none) else none

/-! ### Data processing -/

def execDp (s : State) (op : DpOp) (sflag : Bool) (rd rn : Fin 16) (o : Op2) : Option State := do
  let (y, sc) ← o.eval s
  if sflag && rd = 15 && !op.isTest then none
  if rn = 15 && !op.isMove && (match o with | .regShift .. => true | _ => false) then none
  let x := s.get rn
  -- result, and the flags it sets: `some (n, z, c, v)` for arithmetic, logical via `sc`
  let logical (r : UInt32) : UInt32 × (Bool × Bool × Bool × Bool) :=
    (r, (bit r 31, r == 0, sc, s.v))
  let arith (r : UInt32) (c v : Bool) : UInt32 × (Bool × Bool × Bool × Bool) :=
    (r, (bit r 31, r == 0, c, v))
  let (res, fl) :=
    match op with
    | .and | .tst => logical (x &&& y)
    | .eor | .teq => logical (x ^^^ y)
    | .orr => logical (x ||| y)
    | .bic => logical (x &&& ~~~y)
    | .mov => logical y
    | .mvn => logical (~~~y)
    | .add | .cmn => let (r, c, v) := addC x y false; arith r c v
    | .adc => let (r, c, v) := addC x y s.c; arith r c v
    | .sub | .cmp => let (r, c, v) := addC x (~~~y) true; arith r c v
    | .sbc => let (r, c, v) := addC x (~~~y) s.c; arith r c v
    | .rsb => let (r, c, v) := addC y (~~~x) true; arith r c v
    | .rsc => let (r, c, v) := addC y (~~~x) s.c; arith r c v
  let s1 := if sflag || op.isTest then
      { s with n := fl.1, z := fl.2.1, c := fl.2.2.1, v := fl.2.2.2 } else s
  if op.isTest then some s1 else writeDest s1 rd res

/-! ### Multiply -/

def execMul (s : State) (sflag : Bool) (rd rm rs : Fin 16) (ra : Option (Fin 16)) : Option State := do
  if rd = 15 ∨ rm = 15 ∨ rs = 15 ∨ ra == some 15 then none
  let acc := match ra with | some a => s.get a | none => 0
  let res := s.get rm * s.get rs + acc
  let s1 := s.setR rd res
  some (if sflag then setNZ s1 res else s1)

def execMulLong (s : State) (signed acc sflag : Bool) (lo hi rm rs : Fin 16) : Option State := do
  if lo = 15 ∨ hi = 15 ∨ rm = 15 ∨ rs = 15 ∨ lo = hi then none
  let prod : Int :=
    if signed then toInt (s.get rm) * toInt (s.get rs)
    else ((s.get rm).toNat * (s.get rs).toNat : Nat)
  let a : Int := if acc then ((s.get hi).toNat * 2 ^ 32 + (s.get lo).toNat : Nat) else 0
  let total : Nat := ((prod + a) % 2 ^ 64).toNat
  let l : UInt32 := (total % 2 ^ 32).toUInt32
  let h : UInt32 := (total / 2 ^ 32).toUInt32
  let s1 := (s.setR lo l).setR hi h
  some (if sflag then { s1 with n := bit h 31, z := total == 0 } else s1)

/-! ### Load and store -/

/-- The accessed address and the base register's new value for an addressing
mode, or `none` for the forms with `pc` that are outside the model. -/
def Addr.eval (s : State) (a : Addr) : Option (UInt32 × UInt32) := do
  let base := s.get a.rn
  let offv ← match a.off with
    | .imm add v => pure (add, v.toUInt32)
    | .reg add rm sh =>
      if rm = 15 then none else pure (add, (sh.apply (s.get rm) s.c).1)
  let moved := if offv.1 then base + offv.2 else base - offv.2
  match a.idx with
  | .offset => some (moved, base)
  | .pre => some (moved, moved)
  | .post => some (base, moved)

def writesBack (a : Addr) : Bool := a.idx != .offset

def execLdr (s : State) (byte : Bool) (rt : Fin 16) (a : Addr) : Option State := do
  let (addr, nb) ← a.eval s
  if writesBack a && (a.rn = 15 ∨ a.rn = rt) then none
  if byte && rt = 15 then none
  let s1 := if writesBack a then s.setR a.rn nb else s
  let v := if byte then (s.mem addr).toUInt32 else rd32 s.mem addr
  writeDest s1 rt v

def execStr (s : State) (byte : Bool) (rt : Fin 16) (a : Addr) : Option State := do
  let (addr, nb) ← a.eval s
  if writesBack a && (a.rn = 15 ∨ a.rn = rt) then none
  if rt = 15 then none
  let s1 := if writesBack a then s.setR a.rn nb else s
  let v := s.get rt
  some { s1 with mem := if byte then wr8 s.mem addr v.toUInt8 else wr32 s.mem addr v }

/-- `ldm`/`stm`: the registers in ascending order at ascending word
addresses. The unpredictable forms (unaligned address, empty list, base in the
list with writeback, `pc` stored, base `pc`) are `none`. -/
def mmAddrs (m : MultMode) (base : UInt32) (n : Nat) : UInt32 × UInt32 :=
  let n4 : UInt32 := (4 * n).toUInt32
  match m with
  | .ia => (base, base + n4)
  | .ib => (base + 4, base + n4)
  | .da => (base - n4 + 4, base - n4)
  | .db => (base - n4, base - n4)

def execLdm (s : State) (m : MultMode) (rn : Fin 16) (wb : Bool) (l : List (Fin 16)) : Option State := do
  let regs := regsOf l
  if rn = 15 ∨ regs.isEmpty ∨ (wb && regs.contains rn) then none
  let base := s.r rn
  let (start, nb) := mmAddrs m base regs.length
  if start &&& 3 ≠ 0 then none
  let rec load (s : State) (a : UInt32) : List (Fin 16) → Option State
    | [] => some s
    | r :: rest =>
      let v := rd32 s.mem a
      if r = 15 then (branchTo s v) else load (s.setR r v) (a + 4) rest
  let s1 ← load s start regs
  some (if wb then s1.setR rn nb else s1)

def execStm (s : State) (m : MultMode) (rn : Fin 16) (wb : Bool) (l : List (Fin 16)) : Option State := do
  let regs := regsOf l
  if rn = 15 ∨ regs.isEmpty ∨ regs.contains 15 ∨ (wb && regs.contains rn) then none
  let base := s.r rn
  let (start, nb) := mmAddrs m base regs.length
  if start &&& 3 ≠ 0 then none
  let rec store (mem : UInt32 → UInt8) (a : UInt32) : List (Fin 16) → (UInt32 → UInt8)
    | [] => mem
    | r :: rest => store (wr32 mem a (s.r r)) (a + 4) rest
  let s1 := { s with mem := store s.mem start regs }
  some (if wb then s1.setR rn nb else s1)

/-! ### The instruction -/

/-- The target of a resolved branch (`pc + 8 + 4n`; `s.r 15` is `pc + 4` during `exec`). -/
def Target.addr (s : State) : Target → Option UInt32
  | .rel n => some (s.r 15 + 4 + ((4 * n) % 2 ^ 32).toNat.toUInt32)
  | .label _ | .local .. => none

/-- Execute one instruction whose condition held, in the state where r15
already is the next instruction's address. -/
def exec (s : State) : Instr → Option State
  | .dp op sf rd rn o => execDp s op sf rd rn o
  | .movw rd v => if rd = 15 then none else some (s.setR rd v.toUInt32)
  | .movt rd v => if rd = 15 then none else
      some (s.setR rd ((s.r rd &&& (0xFFFF : UInt32)) ||| (v.toUInt32 <<< 16)))
  | .movwSym .. | .movtSym .. => none   -- unresolved: `layout` replaces them
  | .mul sf rd rm rs => execMul s sf rd rm rs none
  | .mla sf rd rm rs ra => execMul s sf rd rm rs (some ra)
  | .mulLong sg ac sf lo hi rm rs => execMulLong s sg ac sf lo hi rm rs
  | .clz rd rm => if rd = 15 ∨ rm = 15 then none else some (s.setR rd (clz32 (s.r rm)))
  | .ldr byte rt a => execLdr s byte rt a
  | .str byte rt a => execStr s byte rt a
  | .ldm m rn wb l => execLdm s m rn wb l
  | .stm m rn wb l => execStm s m rn wb l
  | .b t => do
    let a ← t.addr s
    branchTo s a
  | .bl t => do
    let a ← t.addr s
    branchTo (s.setR 14 (s.r 15)) a
  | .bx rm => if rm = 15 then none else branchTo s (s.r rm)
  | .blx rm => if rm = 15 then none else branchTo (s.setR 14 (s.r 15)) (s.r rm)
  | .mrsCpsr rd => if rd = 15 then none else some (s.setR rd s.cpsr)
  | .msrCpsr f c src => do
    let v ← match src with
      | .imm v rot => some (ror32 v.toUInt32 (2 * rot))
      | .reg rm => if rm = 15 then none else some (s.r rm)
    let s1 := if f then
        { (s.setFlagsWord v) with cpsrRest := (s.cpsrRest &&& ~~~0x08000000) ||| (v &&& 0x08000000) }
      else s
    if c then
      some ({ s1 with cpsrRest := (s1.cpsrRest &&& ~~~0xDF) ||| (v &&& 0xDF) }.logEff (.msrC v))
    else some s1
  | .cps en i f =>
    let m : UInt32 := (if i then 0x80 else 0) ||| (if f then 0x40 else 0)
    let rest := if en then s.cpsrRest &&& ~~~m else s.cpsrRest ||| m
    some ({ s with cpsrRest := rest }.logEff (.cps en i f))
  | .mcr r rt => if rt = 15 then none else some (s.logEff (.mcr r (s.r rt)))
  | .mrc r rt =>
    if rt = 15 then none else
    let v := s.cpIn r
    some ((s.setR rt v).logEff (.mrc r v))
  | .vmsr sys rt => if rt = 15 then none else some (s.logEff (.vmsr sys (s.r rt)))
  | .vmrs rt sys =>
    if rt = 15 then none else
    let v := s.vfpIn sys
    some ((s.setR rt v).logEff (.vmrs sys v))
  | .dsb => some (s.logEff .dsb)
  | .dmb => some (s.logEff .dmb)
  | .isb => some (s.logEff .isb)
  | .nop => some s
  | .wfe => some (s.logEff .wfe)
  | .wfi => some (s.logEff .wfi)
  | .sev => some (s.logEff .sev)
  | .svc _ => none -- OS exception handling is outside this instruction model.
  -- FFmpeg's additions (`ZrAsm.Ffmpeg`) are printed and assembled but not
  -- interpreted: the FFmpeg-only load/store forms, literal and `adr` loads,
  -- `pld`, the ARMv6 media instructions, `setend` and all of NEON.
  | .movwSymOff .. | .movtSymOff .. | .ldrx .. | .strx .. | .ldrLit .. | .adr .. | .pld _
  | .par .. | .mul4 .. | .ext .. | .setend _ => none
  | .v3 .. | .v3s .. | .v2 .. | .vcmpz .. | .vsh .. | .vimm .. | .vext .. | .vdup .. | .vdupLane ..
  | .vmovToLane .. | .vmovFromLane .. | .vmovToD .. | .vmovFromD .. | .vld .. | .vst ..
  | .vpush _ | .vpop _ => none

/-- One step of the instruction at `s.r 15` with condition `c`: r15 advances
by 4 unless the instruction branches. `none`: outside the model. -/
def step (s : State) (c : Cond) (i : Instr) : Option State :=
  let s1 := s.setR 15 (s.r 15 + 4)
  if c.holds s then exec s1 i else some s1

/-! ### Programs -/

/-- A program image: instruction words and data words at consecutive
addresses from `base`. -/
inductive Cell
  | ins (c : Cond) (i : Instr)
  | data (w : UInt32)
  deriving Repr, DecidableEq

structure Prog where
  base : UInt32
  cells : Array Cell
  deriving Repr, DecidableEq

def Prog.cellAt (p : Prog) (a : UInt32) : Option Cell :=
  if a &&& 3 ≠ 0 ∨ a < p.base then none else p.cells[((a - p.base) >>> 2).toNat]?

/-- Run up to `fuel` instructions from the state's pc while it is inside
the program. The result: the state when pc left the program or fuel ran out
(`done = false`), or `none` if an instruction was outside the model or pc
pointed at data. -/
def run (p : Prog) : Nat → State → Option (Bool × State)
  | 0, s => some (false, s)
  | fuel + 1, s =>
    match p.cellAt (s.r 15) with
    | none => some (true, s)
    | some (.data _) => none
    | some (.ins c i) => do
      let s' ← step s c i
      run p fuel s'

/-- Resolve labels. The cells and the label addresses of items placed from
`base`; `none` if a symbol is unknown or a directive is not modelled
(sections, `ltorg`, symbol words). -/
private def find (l : List (String × UInt32)) (k : String) : Option UInt32 :=
  (l.find? (·.1 == k)).map (·.2)

def layout (base : UInt32) (items : List Item) : Option Prog := do
  let mut labels : List (String × UInt32) := []
  let mut addr := base
  -- pass 1: addresses
  for it in items do
    match it with
    | .insn .. => addr := addr + 4
    | .label s => labels := (s, addr) :: labels
    | .word _ => addr := addr + 4
    | .zero n => addr := addr + n.toUInt32
    | .balign n =>
      let m := n.toUInt32
      if m == 0 then none else addr := (addr + m - 1) / m * m
    | _ => pure ()
  -- pass 2: cells
  let mut cells : Array Cell := #[]
  let mut a := base
  for it in items do
    match it with
    | .insn c i =>
      let i' ← match i with
        | .b (.label t) => (find labels t).map fun x => Instr.b (.rel (((x.toNat : Int) - (a.toNat : Int) - 8) / 4))
        | .bl (.label t) => (find labels t).map fun x => Instr.bl (.rel (((x.toNat : Int) - (a.toNat : Int) - 8) / 4))
        | .movwSym rd t => (find labels t).map fun (x : UInt32) => Instr.movw rd (x &&& 0xFFFF).toUInt16
        | .movtSym rd t => (find labels t).map fun (x : UInt32) => Instr.movt rd (x >>> 16).toUInt16
        | i => some i
      cells := cells.push (.ins c i')
      a := a + 4
    | .word (.num v) => cells := cells.push (.data v); a := a + 4
    | .word (.sym t) =>
      let x ← find labels t
      cells := cells.push (.data x)
      a := a + 4
    | .zero n =>
      if n % 4 ≠ 0 then none else
      for _ in List.range (n / 4) do cells := cells.push (.data 0)
      a := a + n.toUInt32
    | .balign n =>
      let m := n.toUInt32
      let a' := (a + m - 1) / m * m
      if (a' - a) % 4 ≠ 0 then none else
      for _ in List.range ((a' - a) / 4).toNat do cells := cells.push (.data 0)
      a := a'
    | .label _ | .global _ | .weak _ | .funcType _ | .text | .arm => pure ()
    | .size _ | .arch _ | .objectArch _ | .fpu _ | .eabiAttribute .. | .syntaxUnified => pure ()
    -- Not modelled: sections, literal pools, numeric local labels, byte/halfword data.
    | .section .. | .ltorg | .noteGnuStack | .localLabel _ | .byte _ | .short _ => none
  return { base, cells }

end ZrAsm
