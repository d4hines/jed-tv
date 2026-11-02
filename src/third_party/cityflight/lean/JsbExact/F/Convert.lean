import JsbExact.Model
import JsbExact.F.Model

/-! Copies between the generic model at `Float` and the `F` model, and the numbers of a state or diagnostic
record as a named list. Used to build the `F` template from the generic one and by the differential test
(`Tools/JsbDiff.lean`), which compares every field of both models by `Float.toBits` after every frame.
Nothing here computes: each field is copied as is. -/

namespace JsbExact.F

def V3.ofGeneric (v : JsbExact.V3 Float) : V3 := ⟨v.x, v.y, v.z⟩
def M33.ofGeneric (m : JsbExact.M33 Float) : M33 :=
  ⟨m.a11, m.a12, m.a13, m.a21, m.a22, m.a23, m.a31, m.a32, m.a33⟩
def Quat.ofGeneric (q : JsbExact.Quat Float) : Quat := ⟨q.q0, q.q1, q.q2, q.q3⟩

def GearState.ofGeneric (g : JsbExact.GearState Float) : GearState :=
  { wow := g.wow, lm0 := g.lm0, lm1 := g.lm1, lm2 := g.lm2, slip := g.slip, steer := g.steer,
    act := .ofGeneric g.act, mT := .ofGeneric g.mT }

def State.ofGeneric (s : JsbExact.State Float) : State :=
  { q := .ofGeneric s.q, qdot := .ofGeneric s.qdot, pqri := .ofGeneric s.pqri, ri := .ofGeneric s.ri,
    vi := .ofGeneric s.vi, vih0 := .ofGeneric s.vih0, vih1 := .ofGeneric s.vih1, aih0 := .ofGeneric s.aih0,
    epa := s.epa, pqridot := .ofGeneric s.pqridot, uvwidot := .ofGeneric s.uvwidot,
    uvwdot := .ofGeneric s.uvwdot, stall := s.stall, cg := .ofGeneric s.cg, tank0 := s.tank0,
    tank1 := s.tank1, tmap := s.tmap, running := s.running, fuelflow := s.fuelflow, starved := s.starved,
    rpm := s.rpm, act := .ofGeneric s.act, lastcg := .ofGeneric s.lastcg,
    gears := s.gears.map GearState.ofGeneric, wind := .ofGeneric s.wind }

def Diag.ofGeneric (d : JsbExact.Diag Float) : Diag :=
  { uvw := .ofGeneric d.uvw, pqr := .ofGeneric d.pqr, loc := .ofGeneric d.loc, euler := .ofGeneric d.euler,
    hsl := d.hsl, agl := d.agl, vned := .ofGeneric d.vned, alpha := d.alpha, beta := d.beta, adot := d.adot,
    qbar := d.qbar, vt := d.vt, mach := d.mach, pt := d.pt, hmac := d.hmac, rho := d.rho, p := d.p, t := d.t,
    a := d.a, grav := .ofGeneric d.grav, fnat := .ofGeneric d.fnat, mmrc := .ofGeneric d.mmrc,
    faero := .ofGeneric d.faero, maero := .ofGeneric d.maero, fprop := .ofGeneric d.fprop,
    mprop := .ofGeneric d.mprop, ftot := .ofGeneric d.ftot, mtot := .ofGeneric d.mtot, thrust := d.thrust,
    hp := d.hp, mass := d.mass, de := d.de, da := d.da, dr := d.dr, fgear := .ofGeneric d.fgear,
    mgear := .ofGeneric d.mgear, ffric := .ofGeneric d.ffric, gearFz := d.gearFz }

/-! ### Every number of a record, with its field name

Booleans are listed as `1.0`/`0.0` so one comparison covers them too. -/

/-- A named list of numbers. -/
abbrev Fields := Array (String × Float)

def V3.fields (n : String) (v : V3) : Fields := #[(n ++ ".x", v.x), (n ++ ".y", v.y), (n ++ ".z", v.z)]
def M33.fields (n : String) (m : M33) : Fields :=
  #[(n ++ ".a11", m.a11), (n ++ ".a12", m.a12), (n ++ ".a13", m.a13), (n ++ ".a21", m.a21), (n ++ ".a22", m.a22),
    (n ++ ".a23", m.a23), (n ++ ".a31", m.a31), (n ++ ".a32", m.a32), (n ++ ".a33", m.a33)]
def Quat.fields (n : String) (q : Quat) : Fields :=
  #[(n ++ ".q0", q.q0), (n ++ ".q1", q.q1), (n ++ ".q2", q.q2), (n ++ ".q3", q.q3)]
private def boolField (n : String) (b : Bool) : Fields := #[(n, if b then 1.0 else 0.0)]

def GearState.fields (n : String) (g : GearState) : Fields :=
  boolField (n ++ ".wow") g.wow ++ #[(n ++ ".lm0", g.lm0), (n ++ ".lm1", g.lm1), (n ++ ".lm2", g.lm2),
    (n ++ ".slip", g.slip), (n ++ ".steer", g.steer)] ++ V3.fields (n ++ ".act") g.act ++ M33.fields (n ++ ".mT") g.mT

def State.fields (s : State) : Fields :=
  Quat.fields "q" s.q ++ Quat.fields "qdot" s.qdot ++ V3.fields "pqri" s.pqri ++ V3.fields "ri" s.ri ++
  V3.fields "vi" s.vi ++ V3.fields "vih0" s.vih0 ++ V3.fields "vih1" s.vih1 ++ V3.fields "aih0" s.aih0 ++
  #[("epa", s.epa)] ++ V3.fields "pqridot" s.pqridot ++ V3.fields "uvwidot" s.uvwidot ++
  V3.fields "uvwdot" s.uvwdot ++ #[("stall", s.stall)] ++ V3.fields "cg" s.cg ++
  #[("tank0", s.tank0), ("tank1", s.tank1), ("tmap", s.tmap)] ++ boolField "running" s.running ++
  #[("fuelflow", s.fuelflow)] ++ boolField "starved" s.starved ++ #[("rpm", s.rpm)] ++
  V3.fields "act" s.act ++ V3.fields "lastcg" s.lastcg ++
  (s.gears.zipIdx.foldl (fun acc (g, i) => acc ++ GearState.fields s!"gears[{i}]" g) #[]) ++
  #[("gears.size", Float.ofNat s.gears.size)] ++ V3.fields "wind" s.wind

def Diag.fields (d : Diag) : Fields :=
  V3.fields "uvw" d.uvw ++ V3.fields "pqr" d.pqr ++ V3.fields "loc" d.loc ++ V3.fields "euler" d.euler ++
  #[("hsl", d.hsl), ("agl", d.agl)] ++ V3.fields "vned" d.vned ++
  #[("alpha", d.alpha), ("beta", d.beta), ("adot", d.adot), ("qbar", d.qbar), ("vt", d.vt), ("mach", d.mach),
    ("pt", d.pt), ("hmac", d.hmac), ("rho", d.rho), ("p", d.p), ("t", d.t), ("a", d.a)] ++
  V3.fields "grav" d.grav ++ V3.fields "fnat" d.fnat ++ V3.fields "mmrc" d.mmrc ++ V3.fields "faero" d.faero ++
  V3.fields "maero" d.maero ++ V3.fields "fprop" d.fprop ++ V3.fields "mprop" d.mprop ++
  V3.fields "ftot" d.ftot ++ V3.fields "mtot" d.mtot ++
  #[("thrust", d.thrust), ("hp", d.hp), ("mass", d.mass), ("de", d.de), ("da", d.da), ("dr", d.dr)] ++
  V3.fields "fgear" d.fgear ++ V3.fields "mgear" d.mgear ++ V3.fields "ffric" d.ffric ++
  (d.gearFz.zipIdx.map fun (fz, i) => (s!"gearFz[{i}]", fz)) ++ #[("gearFz.size", Float.ofNat d.gearFz.size)]

end JsbExact.F
