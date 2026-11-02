import ZrAsm.Dsl

/-!
# ZrAsm.Neon: writing Advanced SIMD (NEON) in the `Asm` monad

The registers `d0`-`d31` and `q0`-`q15` (`open ZrAsm.VReg`), element types
(a numeral is the untyped size, `vld1 8 ...` for `vld1.8`; `.i16`, `.u8`,
`.s16`, ... the typed ones), scalars (`d6[1]`), and one function per
mnemonic, operands in assembler order:

    vld1 8 [d0, d1] (al r0 128) r1      -- vld1.8   {d0, d1}, [r0:128], r1
    vld1 32 [d16[0]] r0 .wb             -- vld1.32  {d16[0]}, [r0]!
    vld1 16 (dup [d22, d23]) (al r6 16) -- vld1.16  {d22[], d23[]}, [r6:16]
    vmull .u8 q8 d4 d0                  -- vmull.u8 q8, d4, d0
    vmla .i16 q8 q1 d6[1]               -- vmla.i16 q8, q1, d6[1]
    vrshrn .u16 d16 q8 6                -- vrshrn.u16 d16, q8, #6
    vclt .s8 q7 q12 0                   -- vclt.s8  q7, q12, #0
    vand q6 q6 q14                      -- vand     q6, q6, q14

Shapes the assembler rejects (a `.d` where a `.q` belongs, a scalar for an
operation without a by-scalar form) are printed as written and fail there.
None of this is interpreted by `ZrAsm.Sem`: a NEON step is stuck.
-/

namespace ZrAsm

namespace VReg
def d0 : VReg := .d 0
def d1 : VReg := .d 1
def d2 : VReg := .d 2
def d3 : VReg := .d 3
def d4 : VReg := .d 4
def d5 : VReg := .d 5
def d6 : VReg := .d 6
def d7 : VReg := .d 7
def d8 : VReg := .d 8
def d9 : VReg := .d 9
def d10 : VReg := .d 10
def d11 : VReg := .d 11
def d12 : VReg := .d 12
def d13 : VReg := .d 13
def d14 : VReg := .d 14
def d15 : VReg := .d 15
def d16 : VReg := .d 16
def d17 : VReg := .d 17
def d18 : VReg := .d 18
def d19 : VReg := .d 19
def d20 : VReg := .d 20
def d21 : VReg := .d 21
def d22 : VReg := .d 22
def d23 : VReg := .d 23
def d24 : VReg := .d 24
def d25 : VReg := .d 25
def d26 : VReg := .d 26
def d27 : VReg := .d 27
def d28 : VReg := .d 28
def d29 : VReg := .d 29
def d30 : VReg := .d 30
def d31 : VReg := .d 31
def q0 : VReg := .q 0
def q1 : VReg := .q 1
def q2 : VReg := .q 2
def q3 : VReg := .q 3
def q4 : VReg := .q 4
def q5 : VReg := .q 5
def q6 : VReg := .q 6
def q7 : VReg := .q 7
def q8 : VReg := .q 8
def q9 : VReg := .q 9
def q10 : VReg := .q 10
def q11 : VReg := .q 11
def q12 : VReg := .q 12
def q13 : VReg := .q 13
def q14 : VReg := .q 14
def q15 : VReg := .q 15

/-- The low and high halves of a quadword (`q1.lo` is `d2`). -/
def lo : VReg → VReg
  | .q n => .d ⟨2 * n.val, by omega⟩
  | r => panic! s!"ZrAsm.VReg.lo: {repr r} is not a quadword"
def hi : VReg → VReg
  | .q n => .d ⟨2 * n.val + 1, by omega⟩
  | r => panic! s!"ZrAsm.VReg.hi: {repr r} is not a quadword"
end VReg

/-- The doublewords `d<lo>`-`d<hi>`, as GNU's `{d8-d15}`. -/
def drange (lo hi : Nat) : List VReg :=
  (List.range (hi + 1 - lo)).map fun i => .d (Fin.ofNat _ (lo + i))

/-- A numeral is an untyped element size: `vld1 8` is `vld1.8`. -/
instance : OfNat Dt n := ⟨⟨.x, n⟩⟩

namespace Dt
def i8 : Dt := ⟨.i, 8⟩
def i16 : Dt := ⟨.i, 16⟩
def i32 : Dt := ⟨.i, 32⟩
def i64 : Dt := ⟨.i, 64⟩
def u8 : Dt := ⟨.u, 8⟩
def u16 : Dt := ⟨.u, 16⟩
def u32 : Dt := ⟨.u, 32⟩
def u64 : Dt := ⟨.u, 64⟩
def s8 : Dt := ⟨.s, 8⟩
def s16 : Dt := ⟨.s, 16⟩
def s32 : Dt := ⟨.s, 32⟩
def s64 : Dt := ⟨.s, 64⟩
def f32 : Dt := ⟨.f, 32⟩
def p8 : Dt := ⟨.p, 8⟩
end Dt

/-- `d6[1]`: a lane of a doubleword register. -/
instance : GetElem VReg Nat Scalar (fun _ _ => True) where
  getElem v i _ := match v with
    | .d n => ⟨n, i⟩
    | .q n => panic! s!"ZrAsm: q{n.val}[{i}] is not a scalar (lanes are of d registers)"

/-- Register lists: whole registers (a `q` is its two `d`s), lanes
(`[d16[0], d17[0]]`, the same lane of each) or `dup [d0, d1]` (`{d0[], d1[]}`). -/
instance : Coe (List VReg) VList := ⟨fun l => .regs (l.flatMap VReg.dregs)⟩
instance : Coe (List Scalar) VList := ⟨fun l =>
  match l with
  | [] => panic! "ZrAsm: empty lane list"
  | s :: rest =>
    if rest.all (·.lane == s.lane) then .lane (l.map (·.reg)) s.lane
    else panic! "ZrAsm: a lane list must use one lane"⟩
def dup (l : List VReg) : VList := .dup (l.flatMap VReg.dregs)

/-- `[rn]`, and `al rn 128` is `[rn:128]`. -/
instance : Coe Reg VAddr := ⟨fun r => { rn := r }⟩
def al (rn : Reg) (bits : Nat) : VAddr := { rn, align := bits }
/-- Post-increment by a register (`, rm`); `.wb` is `!`. -/
instance : Coe Reg VPost := ⟨.reg⟩

/-- The last operand of a three-register form: a register, a scalar
(`vmla.i16 q0, q1, d6[1]`) or, for the comparisons, `0` (`vclt.s8 q0, q1, #0`). -/
inductive VSrc
  | reg (r : VReg)
  | lane (s : Scalar)
  | zero
instance : Coe VReg VSrc := ⟨.reg⟩
instance : Coe Scalar VSrc := ⟨.lane⟩
instance : OfNat VSrc 0 := ⟨.zero⟩

/-- The shift operand of `vshl`/`vqshl`: a register or an immediate. -/
inductive VShift
  | reg (r : VReg)
  | imm (n : Nat)
instance : Coe VReg VShift := ⟨.reg⟩
instance : OfNat VShift n := ⟨.imm n⟩

/-- The source of `vdup`: a core register or a scalar. -/
inductive VDupSrc
  | core (r : Reg)
  | lane (s : Scalar)
instance : Coe Reg VDupSrc := ⟨.core⟩
instance : Coe Scalar VDupSrc := ⟨.lane⟩

namespace Asm

private def cmpZ : V3Op → Option VCmpZOp
  | .vceq => some .vceq | .vcge => some .vcge | .vcgt => some .vcgt
  | .vcle => some .vcle | .vclt => some .vclt | _ => none

/-- A three-register (or by-scalar, or compare-with-zero) form. -/
def v3 (op : V3Op) (dt : Option Dt) (d n : VReg) (m : VSrc) : Asm Unit :=
  match m, dt with
  | .reg r, _ => ins (.v3 op dt d n r)
  | .lane s, some t => ins (.v3s op t d n s)
  | .zero, some t =>
    match cmpZ op with
    | some z => ins (.vcmpz z t d n)
    | none => panic! s!"ZrAsm.{op.name}: #0 is only an operand of the comparisons"
  | _, none => panic! s!"ZrAsm.{op.name}: a scalar or #0 operand needs an element type"

/-! Three registers, an element type. -/
def vadd (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vadd t d n m
def vsub (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vsub t d n m
def vaddl (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vaddl t d n m
def vsubl (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vsubl t d n m
def vaddw (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vaddw t d n m
def vsubw (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vsubw t d n m
def vaddhn (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vaddhn t d n m
def vhadd (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vhadd t d n m
def vhsub (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vhsub t d n m
def vrhadd (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vrhadd t d n m
def vqadd (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vqadd t d n m
def vqsub (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vqsub t d n m
def vabd (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vabd t d n m
def vabdl (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vabdl t d n m
def vaba (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vaba t d n m
def vabal (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vabal t d n m
def vmax (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vmax t d n m
def vmin (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vmin t d n m
def vpadd (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vpadd t d n m
def vpmax (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vpmax t d n m
def vpmin (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vpmin t d n m
def vmul (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vmul t d n m
def vmla (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vmla t d n m
def vmls (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vmls t d n m
def vmull (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vmull t d n m
def vmlal (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vmlal t d n m
def vmlsl (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vmlsl t d n m
def vqdmulh (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vqdmulh t d n m
def vqrdmulh (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vqrdmulh t d n m
def vceq (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vceq t d n m
def vcge (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vcge t d n m
def vcgt (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vcgt t d n m
def vcle (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vcle t d n m
def vclt (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vclt t d n m
def vtst (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vtst t d n m
def vrshl (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vrshl t d n m
def vqrshl (t : Dt) (d n : VReg) (m : VSrc) : Asm Unit := v3 .vqrshl t d n m

/-! Bitwise: no element type. -/
def vand (d n m : VReg) : Asm Unit := ins (.v3 .vand none d n m)
def vbic (d n m : VReg) : Asm Unit := ins (.v3 .vbic none d n m)
def vorr (d n m : VReg) : Asm Unit := ins (.v3 .vorr none d n m)
def vorn (d n m : VReg) : Asm Unit := ins (.v3 .vorn none d n m)
def veor (d n m : VReg) : Asm Unit := ins (.v3 .veor none d n m)
def vbsl (d n m : VReg) : Asm Unit := ins (.v3 .vbsl none d n m)
def vbit (d n m : VReg) : Asm Unit := ins (.v3 .vbit none d n m)
def vbif (d n m : VReg) : Asm Unit := ins (.v3 .vbif none d n m)

/-! Two registers. -/
def vmov (d m : VReg) : Asm Unit := ins (.v2 .vmov none d m)
def vmvn (d m : VReg) : Asm Unit := ins (.v2 .vmvn none d m)
def vswp (d m : VReg) : Asm Unit := ins (.v2 .vswp none d m)
def vtrn (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vtrn t d m)
def vuzp (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vuzp t d m)
def vzip (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vzip t d m)
def vrev16 (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vrev16 t d m)
def vrev32 (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vrev32 t d m)
def vrev64 (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vrev64 t d m)
def vpaddl (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vpaddl t d m)
def vpadal (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vpadal t d m)
def vabs (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vabs t d m)
def vneg (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vneg t d m)
def vqabs (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vqabs t d m)
def vqneg (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vqneg t d m)
def vclz (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vclz t d m)
def vcnt (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vcnt t d m)
def vmovl (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vmovl t d m)
def vmovn (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vmovn t d m)
def vqmovn (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vqmovn t d m)
def vqmovun (t : Dt) (d m : VReg) : Asm Unit := ins (.v2 .vqmovun t d m)

/-! Two registers and a shift. `vshl`/`vqshl` also take a shift register. -/
def vshl (t : Dt) (d m : VReg) (s : VShift) : Asm Unit :=
  match s with
  | .reg r => ins (.v3 .vshl t d m r)
  | .imm n => ins (.vsh .vshl t d m n)
def vqshl (t : Dt) (d m : VReg) (s : VShift) : Asm Unit :=
  match s with
  | .reg r => ins (.v3 .vqshl t d m r)
  | .imm n => ins (.vsh .vqshl t d m n)
def vshr (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vshr t d m n)
def vrshr (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vrshr t d m n)
def vsra (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vsra t d m n)
def vrsra (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vrsra t d m n)
def vsli (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vsli t d m n)
def vsri (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vsri t d m n)
def vqshlu (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vqshlu t d m n)
def vshrn (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vshrn t d m n)
def vrshrn (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vrshrn t d m n)
def vqshrn (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vqshrn t d m n)
def vqrshrn (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vqrshrn t d m n)
def vqshrun (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vqshrun t d m n)
def vqrshrun (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vqrshrun t d m n)
def vshll (t : Dt) (d m : VReg) (n : Nat) : Asm Unit := ins (.vsh .vshll t d m n)

/-! One register and an immediate: `vmov.i16 q11, #28` is `vmovImm .i16 q11 28`. -/
def vmovImm (t : Dt) (d : VReg) (v : Nat) : Asm Unit := ins (.vimm .vmov t d v)
def vmvnImm (t : Dt) (d : VReg) (v : Nat) : Asm Unit := ins (.vimm .vmvn t d v)
def vorrImm (t : Dt) (d : VReg) (v : Nat) : Asm Unit := ins (.vimm .vorr t d v)
def vbicImm (t : Dt) (d : VReg) (v : Nat) : Asm Unit := ins (.vimm .vbic t d v)

/-- `vext.<size> d, n, m, #imm` -/
def vext (t : Dt) (d n m : VReg) (imm : Nat) : Asm Unit := ins (.vext t d n m imm)

/-- `vdup.<size> d, rt` or `vdup.<size> d, dm[x]`. -/
def vdup (t : Dt) (d : VReg) (s : VDupSrc) : Asm Unit :=
  match s with
  | .core r => ins (.vdup t d r)
  | .lane x => ins (.vdupLane t d x)

/-- `vmov.<size> dn[x], rt` -/
def vmovLane (t : Dt) (s : Scalar) (rt : Reg) : Asm Unit := ins (.vmovToLane t s rt)
/-- `vmov.<dt> rt, dn[x]` -/
def vmovFromLane (t : Dt) (rt : Reg) (s : Scalar) : Asm Unit := ins (.vmovFromLane t rt s)
/-- `vmov dm, rt, rt2` / `vmov rt, rt2, dm` -/
def vmovToD (dm : VReg) (rt rt2 : Reg) : Asm Unit :=
  match dm with
  | .d n => ins (.vmovToD n rt rt2)
  | _ => panic! "ZrAsm.vmovToD: a doubleword register"
def vmovFromD (rt rt2 : Reg) (dm : VReg) : Asm Unit :=
  match dm with
  | .d n => ins (.vmovFromD rt rt2 n)
  | _ => panic! "ZrAsm.vmovFromD: a doubleword register"

/-! Element and structure loads and stores: the list, the address, then the
post-increment (a register, or `.wb` for `!`). -/
private def vaddr (a : VAddr) (p : VPost) : VAddr := { a with post := p }
def vld1 (t : Dt) (l : VList) (a : VAddr) (p : VPost := .none) : Asm Unit := ins (.vld 1 t l (vaddr a p))
def vld2 (t : Dt) (l : VList) (a : VAddr) (p : VPost := .none) : Asm Unit := ins (.vld 2 t l (vaddr a p))
def vld3 (t : Dt) (l : VList) (a : VAddr) (p : VPost := .none) : Asm Unit := ins (.vld 3 t l (vaddr a p))
def vld4 (t : Dt) (l : VList) (a : VAddr) (p : VPost := .none) : Asm Unit := ins (.vld 4 t l (vaddr a p))
def vst1 (t : Dt) (l : VList) (a : VAddr) (p : VPost := .none) : Asm Unit := ins (.vst 1 t l (vaddr a p))
def vst2 (t : Dt) (l : VList) (a : VAddr) (p : VPost := .none) : Asm Unit := ins (.vst 2 t l (vaddr a p))
def vst3 (t : Dt) (l : VList) (a : VAddr) (p : VPost := .none) : Asm Unit := ins (.vst 3 t l (vaddr a p))
def vst4 (t : Dt) (l : VList) (a : VAddr) (p : VPost := .none) : Asm Unit := ins (.vst 4 t l (vaddr a p))

/-- `vpush {list}` / `vpop {list}` (a `q` is its two `d`s). -/
def vpush (l : List VReg) : Asm Unit := ins (.vpush (l.flatMap VReg.dregs))
def vpop (l : List VReg) : Asm Unit := ins (.vpop (l.flatMap VReg.dregs))

end Asm

end ZrAsm
