import JsbExact.F.Model
import CityJsb.F.Template

/-! `CityJsb.Plant` over the `Float` model: the same `init`, `obs` and step as `CityJsb/Plant.lean`, with the
state as `F.State`. Nothing is exported from here; `CityJsb/Plant.lean`'s exported entry points call these. -/

namespace CityJsb.F
open JsbExact.F

-- The same literals as `CityJsb.ft` and `CityJsb.g0` (that file imports this one, so they are repeated here).
def ft : Float := 0.3048
def g0 : Float := 32.174

/-- `FGColumnVector3`-style Euler angles (roll, pitch, heading) to the local-to-body matrix. -/
def tl2bOf (phi tht psi : Float) : M33 :=
  let (cf, sf, ct, st, cp, sp) := (phi.cos, phi.sin, tht.cos, tht.sin, psi.cos, psi.sin)
  M33.mk' (ct * cp) (ct * sp) (-st)
    (sf * st * cp - cf * sp) (sf * st * sp + cf * cp) (sf * ct)
    (cf * st * cp + sf * sp) (cf * st * sp - sf * cp) (cf * ct)

/-- An in-flight state `hM` metres above sea level: true airspeed `v` m/s, flight-path angle `gam`, heading
`psi` (clockwise from north), bank `phi`, angle of attack `alpha`; engine and mass state from the template. -/
def init (hM v gam psi phi alpha : Float) : State :=
  let ri : V3 := ⟨JsbExact.Gen.a + hM / ft, 0, 0⟩
  let tec2l := (locDerive ri).tec2l
  let l2b := tl2bOf phi (gam + alpha) psi
  let ti2b := l2b.mul tec2l
  let q := ti2b.toQuat
  let uvw : V3 := ⟨v / ft * alpha.cos, 0, v / ft * alpha.sin⟩
  let omega : V3 := ⟨0, 0, JsbExact.Gen.omega[2]!⟩
  let vi := V3.add (tec2l.T.mulV (l2b.T.mulV uvw)) (V3.cross omega ri)
  let pqri := ti2b.mulV omega
  let t := template
  { t with
    q := q
    qdot := q.qdot pqri
    pqri := pqri
    ri := ri
    vi := vi
    vih0 := vi
    vih1 := vi
    aih0 := V3.zero
    epa := (0 : Float)
    pqridot := V3.zero
    uvwidot := V3.zero
    uvwdot := V3.zero
    stall := (0 : Float)
    wind := V3.zero }

/-- North, east (from the origin), altitude above sea level, NED velocity, airspeed, Euler angles, body rates,
alpha, beta, normal load factor, height above terrain, dynamic pressure (Pa), engine rpm. -/
def obs (s : State) (d : Diag) : FloatArray :=
  let nz := -(d.faero.z + d.fprop.z) / (d.mass * g0)
  ⟨#[d.loc.z * ft, d.loc.y * ft, d.hsl * ft, d.vned.x * ft, d.vned.y * ft, d.vned.z * ft, d.vt * ft,
     d.euler.x, d.euler.y, d.euler.z, d.pqr.x, d.pqr.y, d.pqr.z, d.alpha, d.beta, nz, d.agl * ft,
     d.qbar * 47.880258, s.rpm]⟩

/-- One frame with the demo's fixed mixture and no brakes, and its observation. -/
def stepE (s : State) (elev ail rud thr : Float) : State × FloatArray :=
  let (s', d) := step s { elev, ail, rud, thr, mix := 0.9, brkL := 0, brkR := 0 }
  (s', obs s' d)

end CityJsb.F
