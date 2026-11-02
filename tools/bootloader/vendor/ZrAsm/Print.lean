import ZrAsm.Isa

/-!
# ZrAsm.Print: the AST as GNU/LLVM unified assembler syntax (`.S`)

One line per `Item`. The text is what clang (`--target=armv7a`) assembles;
test/asm assembles what it prints and runs it against `ZrAsm.Sem` in QEMU.
-/

namespace ZrAsm

def Reg.name (r : Reg) : String :=
  match r.val with
  | 13 => "sp"
  | 14 => "lr"
  | 15 => "pc"
  | n => s!"r{n}"

def Cond.suffix : Cond → String
  | .eq => "eq" | .ne => "ne" | .cs => "cs" | .cc => "cc" | .mi => "mi" | .pl => "pl"
  | .vs => "vs" | .vc => "vc" | .hi => "hi" | .ls => "ls" | .ge => "ge" | .lt => "lt"
  | .gt => "gt" | .le => "le" | .al => ""

def ShiftKind.name : ShiftKind → String
  | .lsl => "lsl" | .lsr => "lsr" | .asr => "asr" | .ror => "ror"

def Shift.print : Shift → String
  | .lsl 0 => ""
  | .lsl n => s!", lsl #{n}"
  | .lsr n => s!", lsr #{n}"
  | .asr n => s!", asr #{n}"
  | .ror n => s!", ror #{n}"
  | .rrx => ", rrx"

def Op2.print : Op2 → String
  | .imm v rot => s!"#{(ror32 v.toUInt32 (2 * rot)).toNat}"
  | .reg rm sh => rm.name ++ sh.print
  | .regShift rm k rs => s!"{rm.name}, {k.name} {rs.name}"

def DpOp.name : DpOp → String
  | .and => "and" | .eor => "eor" | .sub => "sub" | .rsb => "rsb" | .add => "add"
  | .adc => "adc" | .sbc => "sbc" | .rsc => "rsc" | .tst => "tst" | .teq => "teq"
  | .cmp => "cmp" | .cmn => "cmn" | .orr => "orr" | .mov => "mov" | .bic => "bic" | .mvn => "mvn"

def Offset.print : Offset → String
  | .imm add v => s!"#{if add then "" else "-"}{v}"
  | .reg add rm sh => s!"{if add then "" else "-"}{rm.name}{sh.print}"

def Addr.print (a : Addr) : String :=
  match a.idx with
  | .offset => s!"[{a.rn.name}, {a.off.print}]"
  | .pre => s!"[{a.rn.name}, {a.off.print}]!"
  | .post => s!"[{a.rn.name}], {a.off.print}"

def MultMode.name : MultMode → String
  | .ia => "ia" | .ib => "ib" | .da => "da" | .db => "db"

/-- The list in ascending register order, as assemblers want it (the
instruction transfers in that order whatever the source order). -/
def regList (l : List Reg) : String :=
  let sorted := (List.range 16).filterMap fun i => if h : i < 16 then (if l.contains ⟨i, h⟩ then some (Reg.name ⟨i, h⟩) else none) else none
  "{" ++ ", ".intercalate sorted ++ "}"

def Target.print : Target → String
  | .label s => s
  | .rel n => s!". + {8 + 4 * n}"

def CpReg.print (r : CpReg) (rt : Reg) : String :=
  s!"p{r.cp}, {r.opc1}, {rt.name}, c{r.crn}, c{r.crm}, {r.opc2}"

def VfpSys.name : VfpSys → String
  | .fpscr => "fpscr" | .fpexc => "fpexc"

def Instr.print (c : Cond) (i : Instr) : String :=
  let cs := c.suffix
  let sf (s : Bool) := if s then "s" else ""
  match i with
  | .dp op s rd rn o =>
    let n := op.name ++ (if op.isTest then "" else sf s) ++ cs
    if op.isTest then s!"{n} {rn.name}, {o.print}"
    else if op.isMove then s!"{n} {rd.name}, {o.print}"
    else s!"{n} {rd.name}, {rn.name}, {o.print}"
  | .movw rd v => s!"movw{cs} {rd.name}, #{v}"
  | .movt rd v => s!"movt{cs} {rd.name}, #{v}"
  | .movwSym rd s => s!"movw{cs} {rd.name}, #:lower16:{s}"
  | .movtSym rd s => s!"movt{cs} {rd.name}, #:upper16:{s}"
  | .mul s rd rm rs => s!"mul{sf s}{cs} {rd.name}, {rm.name}, {rs.name}"
  | .mla s rd rm rs ra => s!"mla{sf s}{cs} {rd.name}, {rm.name}, {rs.name}, {ra.name}"
  | .mulLong sg ac s lo hi rm rs =>
    let n := (if sg then "s" else "u") ++ (if ac then "mlal" else "mull")
    s!"{n}{sf s}{cs} {lo.name}, {hi.name}, {rm.name}, {rs.name}"
  | .clz rd rm => s!"clz{cs} {rd.name}, {rm.name}"
  | .ldr byte rt a => s!"ldr{if byte then "b" else ""}{cs} {rt.name}, {a.print}"
  | .str byte rt a => s!"str{if byte then "b" else ""}{cs} {rt.name}, {a.print}"
  | .ldrh rt a => s!"ldrh{cs} {rt.name}, {a.print}"
  | .strh rt a => s!"strh{cs} {rt.name}, {a.print}"
  | .ldm m rn wb l =>
    if m == .ia && wb && rn == Reg.sp then s!"pop{cs} {regList l}"
    else s!"ldm{m.name}{cs} {rn.name}{if wb then "!" else ""}, {regList l}"
  | .stm m rn wb l =>
    if m == .db && wb && rn == Reg.sp then s!"push{cs} {regList l}"
    else s!"stm{m.name}{cs} {rn.name}{if wb then "!" else ""}, {regList l}"
  | .b t => s!"b{cs} {t.print}"
  | .bl t => s!"bl{cs} {t.print}"
  | .bx rm => s!"bx{cs} {rm.name}"
  | .blx rm => s!"blx{cs} {rm.name}"
  | .mrsCpsr rd => s!"mrs{cs} {rd.name}, cpsr"
  | .msrCpsr f c' src =>
    let fields := (if f then "f" else "") ++ (if c' then "c" else "")
    let s := match src with
      | .imm v rot => s!"#{(ror32 v.toUInt32 (2 * rot)).toNat}"
      | .reg rm => rm.name
    s!"msr{cs} cpsr_{fields}, {s}"
  | .cps en i f => s!"cps{if en then "ie" else "id"} {if i then "i" else ""}{if f then "f" else ""}"
  | .mcr r rt => s!"mcr{cs} {r.print rt}"
  | .mrc r rt => s!"mrc{cs} {r.print rt}"
  | .vmsr sys rt => s!"vmsr{cs} {sys.name}, {rt.name}"
  | .vmrs rt sys => s!"vmrs{cs} {rt.name}, {sys.name}"
  | .dsb => "dsb"
  | .dmb => "dmb"
  | .isb => "isb"
  | .nop => s!"nop{cs}"
  | .wfe => s!"wfe{cs}"
  | .wfi => s!"wfi{cs}"
  | .sev => s!"sev{cs}"
  | .svc imm => s!"svc{cs} #{imm}"
  | .smc imm => s!"smc{cs} #{imm}"

def Const.print : Const → String
  | .num v => s!"{v.toNat}"
  | .sym s => s

def Item.print : Item → String
  | .insn c i => "  " ++ i.print c
  | .label s => s ++ ":"
  | .global s => s!".global {s}"
  | .weak s => s!".weak {s}"
  | .funcType s => s!".type {s}, %function"
  | .section n f => s!".section {n}, \"{f}\""
  | .text => ".text"
  | .arm => ".arm"
  | .balign n => s!".balign {n}"
  | .word v => s!"  .word {v.print}"
  | .zero n => s!"  .zero {n}"
  | .ltorg => ".ltorg"

/-- A whole file, one item per line. -/
def printItems (l : List Item) : String :=
  "\n".intercalate (l.map Item.print) ++ "\n"

end ZrAsm
