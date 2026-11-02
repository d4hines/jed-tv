/-
H.264 slice header (7.3.3). Part of the Lean H.264 decoder (follows FFmpeg
9.0.1, LGPL-2.1-or-later).
-/
import Yt.H264.Params

namespace Yt.H264

inductive SliceType where
  | p | b | i
  deriving BEq, Inhabited

/-- One weight/offset pair (pred_weight_table): luma, then Cb, Cr. -/
structure Weight where
  luma : Option (Int × Int) := none
  chroma : Option ((Int × Int) × (Int × Int)) := none
  deriving Inhabited

/-- A memory_management_control_operation with its arguments. -/
structure Mmco where
  op : Nat
  a : Nat := 0
  b : Nat := 0
  deriving Inhabited

structure SliceHeader where
  nalRefIdc : Nat
  idr : Bool
  firstMb : Nat
  type : SliceType
  ppsId : Nat
  frameNum : Nat
  idrPicId : Nat := 0
  pocLsb : Nat := 0
  deltaPocBottom : Int := 0
  deltaPoc0 : Int := 0
  deltaPoc1 : Int := 0
  directSpatial : Bool := false
  numRefIdx : Nat × Nat := (0, 0)
  /-- ref_pic_list_modification for l0, l1: (modification_of_pic_nums_idc, value). -/
  modifications : Array (Nat × Nat) × Array (Nat × Nat) := (#[], #[])
  lumaLog2Denom : Nat := 0
  chromaLog2Denom : Nat := 0
  weights : Array Weight × Array Weight := (#[], #[])
  explicitWeights : Bool := false
  noOutputOfPriorPics : Bool := false
  longTermReference : Bool := false
  mmcos : Option (Array Mmco) := none
  cabacInitIdc : Nat := 0
  qp : Int
  disableDeblocking : Nat := 0
  alphaOffset : Int := 0
  betaOffset : Int := 0
  /-- Byte position of slice_data() in the RBSP (CABAC: after the alignment bits). -/
  dataPos : Nat := 0
  deriving Inhabited

def modificationList : BitM (Array (Nat × Nat)) := do
  let mut out := #[]
  if ← flag then
    repeat
      let idc ← ue
      if idc == 3 then break
      if idc > 5 then throw "H.264: bad modification_of_pic_nums_idc"
      out := out.push (idc, ← ue)
  return out

def weightTable (n : Nat) : BitM (Array Weight) := do
  let mut out := #[]
  for _ in [0:n] do
    let luma ← if ← flag then pure (some (← se, ← se)) else pure none
    let chroma ← if ← flag then pure (some ((← se, ← se), (← se, ← se))) else pure none
    out := out.push { luma, chroma }
  return out

/-- slice_header(); `nalType` 5 is IDR. -/
def parseSliceHeader (nalType nalRefIdc : Nat) (spss : Array (Option Sps)) (ppss : Array (Option Pps)) :
    BitM (SliceHeader × Sps × Pps) := do
  let firstMb ← ue
  let st ← ue
  let type ← match st % 5 with
    | 0 => pure SliceType.p | 1 => pure .b | 2 => pure .i
    | _ => throw "H.264: SP/SI slices are not supported"
  let ppsId ← ue
  let some (some pps) := ppss[ppsId]? | throw s!"H.264: unknown PPS {ppsId}"
  let some (some sps) := spss[pps.spsId]? | throw s!"H.264: unknown SPS {pps.spsId}"
  let frameNum := (← u sps.log2MaxFrameNum).toNat
  let idr := nalType == 5
  let mut h : SliceHeader := { nalRefIdc, idr, firstMb, type, ppsId, frameNum, qp := 0 }
  if idr then h := { h with idrPicId := ← ue }
  if sps.pocType == 0 then
    h := { h with pocLsb := (← u sps.log2MaxPocLsb).toNat }
    if pps.bottomFieldPicOrderPresent then h := { h with deltaPocBottom := ← se }
  if sps.pocType == 1 && !sps.deltaPicOrderAlwaysZero then
    h := { h with deltaPoc0 := ← se }
    if pps.bottomFieldPicOrderPresent then h := { h with deltaPoc1 := ← se }
  if pps.redundantPicCntPresent then
    if (← ue) != 0 then throw "H.264: redundant pictures are not supported"
  if type == .b then h := { h with directSpatial := ← flag }
  let mut nref := (pps.numRefIdxL0Default, pps.numRefIdxL1Default)
  if type != .i then
    if ← flag then
      let l0 := (← ue) + 1
      let l1 ← if type == .b then pure ((← ue) + 1) else pure nref.2
      nref := (l0, l1)
    if nref.1 > 32 || nref.2 > 32 then throw "H.264: too many reference indices"
  h := { h with numRefIdx := if type == .i then (0, 0) else if type == .p then (nref.1, 0) else nref }
  if type != .i then
    let m0 ← modificationList
    let m1 ← if type == .b then modificationList else pure #[]
    h := { h with modifications := (m0, m1) }
  if (pps.weightedPred && type == .p) || (pps.weightedBipredIdc == 1 && type == .b) then
    let ld ← ue
    let cd ← ue
    let w0 ← weightTable h.numRefIdx.1
    let w1 ← if type == .b then weightTable h.numRefIdx.2 else pure #[]
    h := { h with lumaLog2Denom := ld, chromaLog2Denom := cd, weights := (w0, w1), explicitWeights := true }
  if nalRefIdc != 0 then
    if idr then
      h := { h with noOutputOfPriorPics := ← flag, longTermReference := ← flag }
    else if ← flag then
      let mut ops := #[]
      repeat
        let op ← ue
        if op == 0 then break
        let m ← match op with
          | 1 => pure { op, a := ← ue : Mmco }
          | 2 => pure { op, a := ← ue }
          | 3 => pure { op, a := ← ue, b := ← ue }
          | 4 => pure { op, a := ← ue }
          | 5 => pure { op }
          | 6 => pure { op, a := ← ue }
          | _ => throw "H.264: bad memory_management_control_operation"
        ops := ops.push m
      h := { h with mmcos := some ops }
  if pps.cabac && type != .i then h := { h with cabacInitIdc := ← ue }
  h := { h with qp := pps.picInitQp + (← se) }
  if h.qp < 0 || h.qp > 51 then throw "H.264: slice QP out of range"
  if pps.deblockingFilterControlPresent then
    let d ← ue
    h := { h with disableDeblocking := d }
    if d != 1 then h := { h with alphaOffset := 2 * (← se), betaOffset := 2 * (← se) }
  -- cabac_alignment_one_bit
  let s ← get
  h := { h with dataPos := (s.pos + 7) / 8 }
  return (h, sps, pps)

end Yt.H264
