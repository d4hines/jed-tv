/-
H.264 sequence and picture parameter sets (7.3.2.1, 7.3.2.2), with the
scaling matrices resolved by the fall-back rules (Table 7-2). Part of the
Lean H.264 decoder (follows FFmpeg 9.0.1, LGPL-2.1-or-later).

Only 8-bit 4:2:0 frame-coded video is accepted, as by src/ffh264's shrink.py:
other SPS are rejected here.
-/
import Yt.H264.Bits
import Yt.H264.Tables

namespace Yt.H264

structure Sps where
  id : Nat
  profileIdc : Nat
  log2MaxFrameNum : Nat
  pocType : Nat
  log2MaxPocLsb : Nat := 0
  deltaPicOrderAlwaysZero : Bool := false
  offsetForNonRefPic : Int := 0
  offsetForTopToBottomField : Int := 0
  offsetsForRefFrame : Array Int := #[]
  maxNumRefFrames : Nat
  gapsAllowed : Bool
  mbWidth : Nat
  mbHeight : Nat
  direct8x8Inference : Bool
  /-- Cropping in luma samples: left, right, top, bottom. -/
  crop : Nat × Nat × Nat × Nat := (0, 0, 0, 0)
  /-- VUI bitstream_restriction: max_num_reorder_frames (none: not given). -/
  numReorderFrames : Option Nat := none
  maxDecFrameBuffering : Option Nat := none
  /-- Scaling lists (raster order): six 4x4 (16 each) then two 8x8 (64 each). -/
  scaling4 : ByteArray
  scaling8 : ByteArray
  /-- seq_scaling_matrix_present_flag: a PPS's absent lists fall back to
  these (rule B) rather than to the defaults (rule A). -/
  scalingPresent : Bool := false
  deriving Inhabited

structure Pps where
  id : Nat
  spsId : Nat
  cabac : Bool
  bottomFieldPicOrderPresent : Bool
  numRefIdxL0Default : Nat
  numRefIdxL1Default : Nat
  weightedPred : Bool
  weightedBipredIdc : Nat
  picInitQp : Int
  chromaQpIndexOffset : Int
  secondChromaQpIndexOffset : Int
  deblockingFilterControlPresent : Bool
  constrainedIntraPred : Bool
  redundantPicCntPresent : Bool
  transform8x8 : Bool
  scaling4 : ByteArray
  scaling8 : ByteArray
  deriving Inhabited

def flat4 : ByteArray := ⟨Array.replicate (6 * 16) 16⟩
def flat8 : ByteArray := ⟨Array.replicate (2 * 64) 16⟩

/-- scaling_list() (7.3.2.1.1.1) into raster order; `none` when
useDefaultScalingMatrixFlag. -/
def scalingList (size : Nat) : BitM (Option ByteArray) := do
  let scan := if size == 16 then Tables.zigzag4x4 else Tables.zigzag8x8
  let mut out := ByteArray.mk (Array.replicate size 0)
  let mut last : Int := 8
  let mut next : Int := 8
  for j in [0:size] do
    if next != 0 then
      let delta ← se
      next := (last + delta + 256) % 256
      if j == 0 && next == 0 then return none
    let v := if next == 0 then last else next
    out := out.set! scan[j]!.toNat v.toNat.toUInt8
    last := v
  return some out

def slice16 (b : ByteArray) (i n : Nat) : ByteArray := b.extract (i * n) (i * n + n)

/-- Fall-back rule A for the six 4x4 lists: Default_4x4_Intra (Y, Cb, Cr),
then Default_4x4_Inter. -/
def defaultScaling4x6 : ByteArray :=
  let intra := Tables.defaultScaling4.extract 0 16
  let inter := Tables.defaultScaling4.extract 16 32
  intra ++ intra ++ intra ++ inter ++ inter ++ inter

/-- The scaling matrices of an SPS or PPS: `count` lists (6, or 8 with the
8x8 ones), each present or falling back (rule A for the SPS, rule B to the
SPS's lists for a PPS). -/
def scalingMatrices (count : Nat) (fallback4 fallback8 : ByteArray) : BitM (ByteArray × ByteArray) := do
  let mut l4 : Array ByteArray := #[]
  let mut l8 : Array ByteArray := #[]
  for i in [0:count] do
    let present ← flag
    let size := if i < 6 then 16 else 64
    let parsed ← if present then scalingList size else pure none
    let default := if i < 6 then slice16 Tables.defaultScaling4 (if i < 3 then 0 else 1) 16
                   else slice16 Tables.defaultScaling8 (i - 6) 64
    let value :=
      if present then parsed.getD default
      else if i < 6 then
        if i == 0 || i == 3 then slice16 fallback4 i 16 else l4[i - 1]!
      else slice16 fallback8 (i - 6) 64
    if i < 6 then l4 := l4.push value else l8 := l8.push value
  -- without the 8x8 lists in the syntax they fall back too
  for i in [l8.size:2] do l8 := l8.push (slice16 fallback8 i 64)
  return (l4.foldl (· ++ ·) .empty, l8.foldl (· ++ ·) .empty)

def hrd : BitM Unit := do
  let n ← ue
  let _ ← u 8
  for _ in [0:n + 1] do
    let _ ← ue; let _ ← ue; let _ ← bit
  let _ ← u 20

/-- vui_parameters() up to bitstream_restriction (E.1.1). -/
def vui : BitM (Option Nat × Option Nat) := do
  if ← flag then if (← u 8) == 255 then let _ ← u 32
  if ← flag then let _ ← bit
  if ← flag then
    let _ ← u 4
    if ← flag then let _ ← u 24
  if ← flag then let _ ← ue; let _ ← ue
  if ← flag then let _ ← u 32; let _ ← u 32; let _ ← bit
  let nal ← flag
  if nal then hrd
  let vcl ← flag
  if vcl then hrd
  if nal || vcl then let _ ← bit
  let _ ← bit
  if ← flag then
    let _ ← bit; let _ ← ue; let _ ← ue; let _ ← ue; let _ ← ue
    let reorder ← ue
    let dpb ← ue
    return (some reorder, some dpb)
  return (none, none)

def highProfiles : List Nat := [100, 110, 122, 244, 44, 83, 86, 118, 128, 138, 139, 134, 135]

/-- seq_parameter_set_rbsp(). -/
def parseSps : BitM Sps := do
  let profileIdc := (← u 8).toNat
  let _ ← u 16
  let id ← ue
  let mut chromaFormatIdc := 1
  let mut depthY := 8
  let mut depthC := 8
  let mut scaling := (flat4, flat8)
  let mut scalingPresent := false
  if highProfiles.any (· == profileIdc) then
    chromaFormatIdc ← ue
    if chromaFormatIdc == 3 then let _ ← bit
    depthY := (← ue) + 8
    depthC := (← ue) + 8
    if ← flag then throw "H.264: transform bypass is not supported"
    if ← flag then
      if chromaFormatIdc == 3 then throw "H.264: only 8-bit 4:2:0 is supported (chroma_format_idc 3)"
      -- fall-back rule A: the default lists
      scaling ← scalingMatrices 8 (defaultScaling4x6) Tables.defaultScaling8
      scalingPresent := true
  if chromaFormatIdc != 1 || depthY != 8 || depthC != 8 then
    throw s!"H.264: only 8-bit 4:2:0 is supported (chroma_format_idc {chromaFormatIdc}, depth {depthY}/{depthC})"
  let log2MaxFrameNum := (← ue) + 4
  let pocType ← ue
  let mut sps : Sps := { id, profileIdc, log2MaxFrameNum, pocType, maxNumRefFrames := 0, gapsAllowed := false,
                         mbWidth := 0, mbHeight := 0, direct8x8Inference := false,
                         scaling4 := scaling.1, scaling8 := scaling.2, scalingPresent }
  if pocType == 0 then
    sps := { sps with log2MaxPocLsb := (← ue) + 4 }
  else if pocType == 1 then
    let always ← flag
    let nonRef ← se
    let topBottom ← se
    let n ← ue
    let mut offs := #[]
    for _ in [0:n] do offs := offs.push (← se)
    sps := { sps with deltaPicOrderAlwaysZero := always, offsetForNonRefPic := nonRef,
                      offsetForTopToBottomField := topBottom, offsetsForRefFrame := offs }
  else if pocType != 2 then throw "H.264: bad pic_order_cnt_type"
  let maxNumRefFrames ← ue
  let gaps ← flag
  let mbWidth := (← ue) + 1
  let mbHeight := (← ue) + 1
  unless ← flag do throw "H.264: interlaced (field) coding is not supported"
  let direct8x8 ← flag
  let mut crop := (0, 0, 0, 0)
  if ← flag then
    let l ← ue; let r ← ue; let t ← ue; let b ← ue
    crop := (2 * l, 2 * r, 2 * t, 2 * b)
  let (reorder, dpb) ← if ← flag then vui else pure (none, none)
  return { sps with maxNumRefFrames, gapsAllowed := gaps, mbWidth, mbHeight, direct8x8Inference := direct8x8,
                    crop, numReorderFrames := reorder, maxDecFrameBuffering := dpb }

/-- pic_parameter_set_rbsp(), with the SPSs seen so far (by id). -/
def parsePps (spss : Array (Option Sps)) : BitM Pps := do
  let id ← ue
  let spsId ← ue
  let some (some sps) := spss[spsId]? | throw s!"H.264: PPS {id} refers to unknown SPS {spsId}"
  let cabac ← flag
  if !cabac then throw "H.264: CAVLC entropy coding is not supported yet"
  let bottom ← flag
  if (← ue) != 0 then throw "H.264: slice groups are not supported"
  let l0 := (← ue) + 1
  let l1 := (← ue) + 1
  let wp ← flag
  let wbi := (← u 2).toNat
  let qp := 26 + (← se)
  let _ ← se
  let cqp ← se
  let dfc ← flag
  let cip ← flag
  let rpc ← flag
  let mut t8 := false
  let mut scaling := (sps.scaling4, sps.scaling8)
  let mut second := cqp
  if ← moreRbspData then
    t8 ← flag
    if ← flag then
      -- fall-back rule B (the SPS's lists) only if the SPS has a matrix, else A
      scaling ← if sps.scalingPresent then scalingMatrices (if t8 then 8 else 6) sps.scaling4 sps.scaling8
                else scalingMatrices (if t8 then 8 else 6) defaultScaling4x6 Tables.defaultScaling8
    second ← se
  return { id, spsId, cabac, bottomFieldPicOrderPresent := bottom, numRefIdxL0Default := l0,
           numRefIdxL1Default := l1, weightedPred := wp, weightedBipredIdc := wbi, picInitQp := qp,
           chromaQpIndexOffset := cqp, secondChromaQpIndexOffset := second,
           deblockingFilterControlPresent := dfc, constrainedIntraPred := cip, redundantPicCntPresent := rpc,
           transform8x8 := t8, scaling4 := scaling.1, scaling8 := scaling.2 }

end Yt.H264
