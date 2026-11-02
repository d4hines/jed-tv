import JsbExact.F.Table
import JsbExact.C172Gen

/-! `JsbExact.Gen` at `Float`: the tables the step reads, copied into `F.Table`s once at start-up (same
numbers), and the c172p aerodynamic functions with unboxed inputs and outputs. The scalar constants and
plain arrays of `JsbExact.Gen` (`Gen.dt`, `Gen.gears`, ...) are read from there directly. -/

namespace JsbExact.F.Gen
open JsbExact.F.S

def tab_stdTemp : Table := .ofGeneric JsbExact.Gen.tab_stdTemp
def tab_mixEff : Table := .ofGeneric JsbExact.Gen.tab_mixEff
def tab_cThrust : Table := .ofGeneric JsbExact.Gen.tab_cThrust
def tab_cPower : Table := .ofGeneric JsbExact.Gen.tab_cPower
def tab_CtMach : Table := .ofGeneric JsbExact.Gen.tab_CtMach
def tab_CpMach : Table := .ofGeneric JsbExact.Gen.tab_CpMach
def tab_kCDge_0 : Table := .ofGeneric JsbExact.Gen.tab_kCDge_0
def tab_kCLge_1 : Table := .ofGeneric JsbExact.Gen.tab_kCLge_1
def tab_CDDf_2 : Table := .ofGeneric JsbExact.Gen.tab_CDDf_2
def tab_CDwbh_3 : Table := .ofGeneric JsbExact.Gen.tab_CDwbh_3
def tab_CYb_4 : Table := .ofGeneric JsbExact.Gen.tab_CYb_4
def tab_CYp_5 : Table := .ofGeneric JsbExact.Gen.tab_CYp_5
def tab_CYr_6 : Table := .ofGeneric JsbExact.Gen.tab_CYr_6
def tab_CLwbh_7 : Table := .ofGeneric JsbExact.Gen.tab_CLwbh_7
def tab_CLDf_8 : Table := .ofGeneric JsbExact.Gen.tab_CLDf_8
def tab_Clb_9 : Table := .ofGeneric JsbExact.Gen.tab_Clb_9
def tab_Clr_10 : Table := .ofGeneric JsbExact.Gen.tab_Clr_10
def tab_Cmdf_11 : Table := .ofGeneric JsbExact.Gen.tab_Cmdf_11
def tab_Cnb_12 : Table := .ofGeneric JsbExact.Gen.tab_Cnb_12

/-- Properties the c172p aerodynamic functions read. -/
structure AeroIn where
  Sw : Float
  adot : Float
  ailL : Float
  alpha : Float
  beta : Float
  bi2vel : Float
  bw : Float
  cbar : Float
  ci2vel : Float
  elev : Float
  flapDeg : Float
  hbMac : Float
  magBeta : Float
  magElev : Float
  pAero : Float
  qAero : Float
  qbar : Float
  rAero : Float
  rho : Float
  rud : Float
  stall : Float
  uAero : Float
  vind : Float

/-- Axis sums in document order (DRAG, SIDE, LIFT in wind axes; ROLL, PITCH, YAW in body axes). -/
structure AeroOut where
  drag : Float
  side : Float
  lift : Float
  roll : Float
  pitch : Float
  yaw : Float

def aero (i : AeroIn) : AeroOut :=
  let f_kCDge := (tab_kCDge_0.get1 i.hbMac)
  let f_kCLge := (tab_kCLge_1.get1 i.hbMac)
  let f_velocity_induced_fps := (S.zero + i.uAero + i.vind + i.vind)
  let f_qbar_induced_psf := (S.one * f_velocity_induced_fps * f_velocity_induced_fps * i.rho * (S.lit 0.5))
  let c_CDo := (S.one * i.qbar * i.Sw * (S.lit 0.027))
  let c_CDDf := (S.one * i.qbar * i.Sw * f_kCDge * (tab_CDDf_2.get1 i.flapDeg))
  let c_CDwbh := (S.one * i.qbar * i.Sw * f_kCDge * (tab_CDwbh_3.get2 i.alpha i.flapDeg))
  let c_CDDe := (S.one * i.qbar * i.Sw * i.magElev * (S.lit 0.06))
  let c_CDbeta := (S.one * i.qbar * i.Sw * i.magBeta * (S.lit 0.17))
  let c_CYb := (S.one * i.qbar * i.Sw * (tab_CYb_4.get2 i.beta i.flapDeg))
  let c_CYda := (S.one * i.qbar * i.Sw * i.ailL * (S.lit 0.0))
  let c_CYdr := (S.one * i.qbar * i.Sw * i.rud * (S.lit 0.187))
  let c_CYp := (S.one * i.qbar * i.Sw * i.bi2vel * i.pAero * (tab_CYp_5.get2 i.alpha i.flapDeg))
  let c_CYr := (S.one * i.qbar * i.Sw * i.bi2vel * i.rAero * (tab_CYr_6.get2 i.alpha i.flapDeg))
  let c_CLwbh := (S.one * i.qbar * i.Sw * f_kCLge * (tab_CLwbh_7.get2 i.alpha i.stall))
  let c_CLDf := (S.one * i.qbar * i.Sw * f_kCLge * (tab_CLDf_8.get1 i.flapDeg))
  let c_CLDe := (S.one * i.qbar * i.Sw * i.elev * (S.lit 0.43))
  let c_CLadot := (S.one * i.qbar * i.Sw * i.adot * i.ci2vel * (S.lit 1.7))
  let c_CLq := (S.one * i.qbar * i.Sw * i.qAero * i.ci2vel * (S.lit 3.9))
  let c_Clb := (S.one * i.qbar * i.Sw * i.bw * (tab_Clb_9.get1 i.beta))
  let c_Clp := (S.one * i.qbar * i.Sw * i.bw * i.bi2vel * i.pAero * (S.lit (-0.484)))
  let c_Clr := (S.one * i.qbar * i.Sw * i.bw * i.bi2vel * i.rAero * (tab_Clr_10.get2 i.alpha i.flapDeg))
  let c_ClDa := (S.one * i.qbar * i.Sw * i.bw * i.ailL * (S.lit 0.229))
  let c_Cldr := (S.one * i.qbar * i.Sw * i.bw * i.rud * (S.lit 0.0147))
  let c_Cmo := (S.one * i.qbar * i.Sw * i.cbar * (S.lit 0.1))
  let c_Cmalpha := (S.one * i.qbar * i.Sw * i.cbar * i.alpha * (S.lit (-1.8)))
  let c_Cmq := (S.one * i.qbar * i.Sw * i.cbar * i.ci2vel * i.qAero * (S.lit (-12.4)))
  let c_Cmadot := (S.one * i.qbar * i.Sw * i.cbar * i.ci2vel * i.adot * (S.lit (-7.27)))
  let c_Cmde := (S.one * f_qbar_induced_psf * i.Sw * i.cbar * i.elev * (S.lit (-1.122)))
  let c_Cmdf := (S.one * i.qbar * i.Sw * i.cbar * (tab_Cmdf_11.get1 i.flapDeg))
  let c_Cnb := (S.one * i.qbar * i.Sw * i.bw * (tab_Cnb_12.get1 i.beta))
  let c_Cnp := (S.one * i.qbar * i.Sw * i.bw * i.bi2vel * i.pAero * (S.lit (-0.0278)))
  let c_Cnr := (S.one * i.qbar * i.Sw * i.bw * i.bi2vel * i.rAero * (S.lit (-0.0937)))
  let c_Cnda := (S.one * i.qbar * i.Sw * i.bw * i.ailL * (S.lit (-0.0053)))
  let c_Cndr := (S.one * f_qbar_induced_psf * i.Sw * i.bw * i.rud * (S.lit (-0.043)))
  { drag := S.zero + c_CDo + c_CDDf + c_CDwbh + c_CDDe + c_CDbeta, side := S.zero + c_CYb + c_CYda + c_CYdr + c_CYp + c_CYr, lift := S.zero + c_CLwbh + c_CLDf + c_CLDe + c_CLadot + c_CLq, roll := S.zero + c_Clb + c_Clp + c_Clr + c_ClDa + c_Cldr, pitch := S.zero + c_Cmo + c_Cmalpha + c_Cmq + c_Cmadot + c_Cmde + c_Cmdf, yaw := S.zero + c_Cnb + c_Cnp + c_Cnr + c_Cnda + c_Cndr }

end JsbExact.F.Gen
