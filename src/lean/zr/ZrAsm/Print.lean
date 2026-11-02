import ZrAsm.Isa

/-!
# ZrAsm.Print: the AST as GNU/LLVM unified assembler syntax (`.S`)

One line per `Item`. The text is what clang (`--target=armv7a`) assembles;
dande-router's test/asm assembles what it prints and runs it against
`ZrAsm.Sem` in QEMU.
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
  | .local n fwd => s!"{n}{if fwd then "f" else "b"}"

def CpReg.print (r : CpReg) (rt : Reg) : String :=
  s!"p{r.cp}, {r.opc1}, {rt.name}, c{r.crn}, c{r.crm}, {r.opc2}"

def VfpSys.name : VfpSys → String
  | .fpscr => "fpscr" | .fpexc => "fpexc"

def MemX.name : MemX → String
  | .h => "h" | .sh => "sh" | .sb => "sb" | .d => "d"

def ParOp.name : ParOp → String
  | .uadd8 => "uadd8" | .usub8 => "usub8" | .uadd16 => "uadd16" | .usub16 => "usub16"
  | .uhadd8 => "uhadd8" | .uhsub8 => "uhsub8" | .uhadd16 => "uhadd16" | .uhsub16 => "uhsub16"
  | .uqadd8 => "uqadd8" | .uqsub8 => "uqsub8" | .sadd8 => "sadd8" | .ssub8 => "ssub8"
  | .shadd8 => "shadd8" | .qadd8 => "qadd8" | .qsub8 => "qsub8" | .usad8 => "usad8" | .sel => "sel"

def Mul4Op.name : Mul4Op → String
  | .usada8 => "usada8" | .smlad => "smlad" | .smladx => "smladx" | .smlsd => "smlsd"
  | .smlabb => "smlabb" | .smlabt => "smlabt" | .smlatb => "smlatb" | .smlatt => "smlatt"
  | .smlawb => "smlawb" | .smlawt => "smlawt"

def ExtOp.name : ExtOp → String
  | .uxtb => "uxtb" | .uxth => "uxth" | .uxtb16 => "uxtb16"
  | .sxtb => "sxtb" | .sxth => "sxth" | .sxtb16 => "sxtb16"

def DReg.name (d : DReg) : String := s!"d{d.val}"

def VReg.name : VReg → String
  | .d n => s!"d{n.val}"
  | .q n => s!"q{n.val}"

def DtKind.name : DtKind → String
  | .x => "" | .i => "i" | .s => "s" | .u => "u" | .f => "f" | .p => "p"

/-- `.i16`, `.8`, ... -/
def Dt.suffix (t : Dt) : String := s!".{t.kind.name}{t.bits}"

def Dt.opt : Option Dt → String
  | some t => t.suffix
  | none => ""

def Scalar.print (s : Scalar) : String := s!"d{s.reg.val}[{s.lane}]"

def VList.print : VList → String
  | .regs l => "{" ++ ", ".intercalate (l.map DReg.name) ++ "}"
  | .lane l x => "{" ++ ", ".intercalate (l.map fun d => s!"{d.name}[{x}]") ++ "}"
  | .dup l => "{" ++ ", ".intercalate (l.map fun d => s!"{d.name}[]") ++ "}"

def VAddr.print (a : VAddr) : String :=
  let al := if a.align = 0 then "" else s!":{a.align}"
  let base := s!"[{a.rn.name}{al}]"
  match a.post with
  | .none => base
  | .wb => base ++ "!"
  | .reg rm => s!"{base}, {rm.name}"

def V3Op.name : V3Op → String
  | .vadd => "vadd" | .vsub => "vsub" | .vaddl => "vaddl" | .vsubl => "vsubl"
  | .vaddw => "vaddw" | .vsubw => "vsubw" | .vaddhn => "vaddhn" | .vsubhn => "vsubhn"
  | .vraddhn => "vraddhn" | .vrsubhn => "vrsubhn"
  | .vhadd => "vhadd" | .vhsub => "vhsub" | .vrhadd => "vrhadd" | .vqadd => "vqadd" | .vqsub => "vqsub"
  | .vabd => "vabd" | .vabdl => "vabdl" | .vaba => "vaba" | .vabal => "vabal"
  | .vmax => "vmax" | .vmin => "vmin" | .vpadd => "vpadd" | .vpmax => "vpmax" | .vpmin => "vpmin"
  | .vmul => "vmul" | .vmla => "vmla" | .vmls => "vmls" | .vmull => "vmull" | .vmlal => "vmlal"
  | .vmlsl => "vmlsl" | .vqdmulh => "vqdmulh" | .vqrdmulh => "vqrdmulh" | .vqdmull => "vqdmull"
  | .vand => "vand" | .vbic => "vbic" | .vorr => "vorr" | .vorn => "vorn" | .veor => "veor"
  | .vbsl => "vbsl" | .vbit => "vbit" | .vbif => "vbif"
  | .vceq => "vceq" | .vcge => "vcge" | .vcgt => "vcgt" | .vcle => "vcle" | .vclt => "vclt" | .vtst => "vtst"
  | .vshl => "vshl" | .vrshl => "vrshl" | .vqshl => "vqshl" | .vqrshl => "vqrshl"

def V2Op.name : V2Op → String
  | .vmov => "vmov" | .vmvn => "vmvn" | .vswp => "vswp" | .vtrn => "vtrn" | .vuzp => "vuzp"
  | .vzip => "vzip" | .vrev16 => "vrev16" | .vrev32 => "vrev32" | .vrev64 => "vrev64"
  | .vpaddl => "vpaddl" | .vpadal => "vpadal" | .vabs => "vabs" | .vneg => "vneg"
  | .vqabs => "vqabs" | .vqneg => "vqneg" | .vcls => "vcls" | .vclz => "vclz" | .vcnt => "vcnt"
  | .vmovl => "vmovl" | .vmovn => "vmovn" | .vqmovn => "vqmovn" | .vqmovun => "vqmovun"

def VCmpZOp.name : VCmpZOp → String
  | .vceq => "vceq" | .vcge => "vcge" | .vcgt => "vcgt" | .vcle => "vcle" | .vclt => "vclt"

def VShOp.name : VShOp → String
  | .vshl => "vshl" | .vshr => "vshr" | .vrshr => "vrshr" | .vsra => "vsra" | .vrsra => "vrsra"
  | .vsli => "vsli" | .vsri => "vsri" | .vqshl => "vqshl" | .vqshlu => "vqshlu"
  | .vshrn => "vshrn" | .vrshrn => "vrshrn" | .vqshrn => "vqshrn" | .vqrshrn => "vqrshrn"
  | .vqshrun => "vqshrun" | .vqrshrun => "vqrshrun" | .vshll => "vshll"

def VImmOp.name : VImmOp → String
  | .vmov => "vmov" | .vmvn => "vmvn" | .vorr => "vorr" | .vbic => "vbic"

def dList (l : List DReg) : String := "{" ++ ", ".intercalate (l.map DReg.name) ++ "}"

def symOff (s : String) (off : Int) : String :=
  if off = 0 then s else if off > 0 then s!"{s}+{off}" else s!"{s}-{off.natAbs}"

def Offset.isZero : Offset → Bool
  | .imm true 0 => true
  | _ => false

/-- `[rn]` for a zero immediate offset (so `pld [r1]` prints as written). -/
def Addr.print' (a : Addr) : String :=
  if a.idx == .offset && a.off.isZero then s!"[{a.rn.name}]" else a.print

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
  | .movwSymOff rd s off => s!"movw{cs} {rd.name}, #:lower16:{symOff s off}"
  | .movtSymOff rd s off => s!"movt{cs} {rd.name}, #:upper16:{symOff s off}"
  | .ldrx k rt a =>
    if k == .d then s!"ldrd{cs} {rt.name}, {Reg.name (rt + 1)}, {a.print'}"
    else s!"ldr{k.name}{cs} {rt.name}, {a.print'}"
  | .strx k rt a =>
    if k == .d then s!"strd{cs} {rt.name}, {Reg.name (rt + 1)}, {a.print'}"
    else s!"str{k.name}{cs} {rt.name}, {a.print'}"
  | .ldrLit rt v => s!"ldr{cs} {rt.name}, ={v.toNat}"
  | .adr rd t => s!"adr{cs} {rd.name}, {t.print}"
  | .pld a => s!"pld {a.print'}"
  | .par op rd rn rm => s!"{op.name}{cs} {rd.name}, {rn.name}, {rm.name}"
  | .mul4 op rd rn rm ra => s!"{op.name}{cs} {rd.name}, {rn.name}, {rm.name}, {ra.name}"
  | .ext op rd rm rot => s!"{op.name}{cs} {rd.name}, {rm.name}{if rot = 0 then "" else s!", ror #{rot}"}"
  | .setend be => s!"setend {if be then "be" else "le"}"
  | .v3 op dt d n m => s!"{op.name}{Dt.opt dt} {d.name}, {n.name}, {m.name}"
  | .v3s op dt d n m => s!"{op.name}{dt.suffix} {d.name}, {n.name}, {m.print}"
  | .v2 op dt d m => s!"{op.name}{Dt.opt dt} {d.name}, {m.name}"
  | .vcmpz op dt d m => s!"{op.name}{dt.suffix} {d.name}, {m.name}, #0"
  | .vsh op dt d m imm => s!"{op.name}{dt.suffix} {d.name}, {m.name}, #{imm}"
  | .vimm op dt d v => s!"{op.name}{dt.suffix} {d.name}, #{v}"
  | .vext dt d n m imm => s!"vext{dt.suffix} {d.name}, {n.name}, {m.name}, #{imm}"
  | .vdup dt d rt => s!"vdup{dt.suffix} {d.name}, {rt.name}"
  | .vdupLane dt d m => s!"vdup{dt.suffix} {d.name}, {m.print}"
  | .vmovToLane dt sc rt => s!"vmov{dt.suffix} {sc.print}, {rt.name}"
  | .vmovFromLane dt rt sc => s!"vmov{dt.suffix} {rt.name}, {sc.print}"
  | .vmovToD dm rt rt2 => s!"vmov {dm.name}, {rt.name}, {rt2.name}"
  | .vmovFromD rt rt2 dm => s!"vmov {rt.name}, {rt2.name}, {dm.name}"
  | .vld n dt l a => s!"vld{n}{dt.suffix} {l.print}, {a.print}"
  | .vst n dt l a => s!"vst{n}{dt.suffix} {l.print}, {a.print}"
  | .vpush l => s!"vpush {dList l}"
  | .vpop l => s!"vpop {dList l}"

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
  | .localLabel n => s!"{n}:"
  | .size s => s!".size {s}, . - {s}"
  | .byte l => "  .byte " ++ ", ".intercalate (l.map fun b => toString b.toNat)
  | .short l => "  .short " ++ ", ".intercalate (l.map fun b => toString b.toNat)
  | .arch s => s!".arch {s}"
  | .objectArch s => s!".object_arch {s}"
  | .fpu s => s!".fpu {s}"
  | .eabiAttribute t v => s!".eabi_attribute {t}, {v}"
  | .syntaxUnified => ".syntax unified"
  | .noteGnuStack => ".section .note.GNU-stack,\"\",%progbits"

/-- A whole file, one item per line. -/
def printItems (l : List Item) : String :=
  "\n".intercalate (l.map Item.print) ++ "\n"

end ZrAsm
