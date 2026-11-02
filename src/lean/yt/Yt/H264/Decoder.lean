/-
The H.264 decoding process around the macroblock layer: parameter sets,
pictures, picture order counts (8.2.1, types 0 and 2), reference lists
(8.2.4, FFmpeg h264_refs.c), reference marking (8.2.5) and output in POC
order. Part of the Lean H.264 decoder (follows FFmpeg 9.0.1,
LGPL-2.1-or-later).
-/
import Yt.H264.SliceData
import Yt.H264.Recon
import Yt.H264.Loop

namespace Yt.H264

/-- A decoded picture kept as a reference. -/
structure StoredPic where
  pic : RefPic
  frameNum : Nat
  /-- Long-term frame index, or none for short-term. -/
  longIdx : Option Nat := none
  deriving Inhabited

/-- A decoded frame waiting for output: the cropped planes and its POC. -/
structure OutFrame where
  poc : Int
  /-- The presentation time given with the picture's first slice. -/
  pts : UInt64 := 0
  width : Nat
  height : Nat
  y : ByteArray
  cb : ByteArray
  cr : ByteArray
  deriving Inhabited

structure Decoder where
  spss : Array (Option Sps) := Array.replicate 32 none
  ppss : Array (Option Pps) := Array.replicate 256 none
  refs : Array StoredPic := #[]
  nextId : Nat := 0
  prevPocMsb : Int := 0
  prevPocLsb : Int := 0
  prevFrameNumOffset : Int := 0
  prevFrameNum : Nat := 0
  /-- The picture being decoded, with its header facts. -/
  cur : Option (Dec × SliceHeader × Sps × Int) := none
  /-- Presentation time of the picture being decoded (set by the caller). -/
  pts : UInt64 := 0
  curPts : UInt64 := 0
  sliceCount : Nat := 0
  /-- Frames decoded but not output yet, and frames ready. -/
  pending : Array OutFrame := #[]
  out : Array OutFrame := #[]
  /-- Macroblocks decoded in the current picture, and the largest number of
  slice-data bits left after an end_of_slice_flag (a parsing check). -/
  mbsDecoded : Nat := 0
  maxTailBits : Nat := 0
  /-- Reconstruction (after each macroblock) and loop filtering (per
  picture); off for parsing-only tools and per-stage measurements. -/
  recon : Bool := true
  deblock : Bool := true
  /-- Skip non-reference pictures (a player behind its clock; FFmpeg's
  skip_frame=nonref). Only with POC type 0, where they do not affect the
  order of later pictures. -/
  skipNonRef : Bool := false

/-- Insert `x` after the last element not greater than it. -/
partial def insertSorted {α : Type} [Inhabited α] (out : Array α) (x : α) (le : α → α → Bool) (i : Nat) : Array α :=
  if i > 0 && !le out[i - 1]! x then insertSorted out x le (i - 1)
  else (out.extract 0 i).push x ++ out.extract i out.size

partial def sortGo {α : Type} [Inhabited α] (xs : Array α) (le : α → α → Bool) (k : Nat) (out : Array α) : Array α :=
  if k < xs.size then sortGo xs le (k + 1) (insertSorted out xs[k]! le out.size) else out

/-- Stable insertion sort (the lists are a few pictures long), written as
plain recursion: generic monadic loops do not link on our runtime. -/
def sortBy {α : Type} [Inhabited α] (xs : Array α) (le : α → α → Bool) : Array α := sortGo xs le 0 #[]

/-! ## Picture order count -/

/-- POC of a frame (8.2.1.1, 8.2.1.3), and the Msb/Lsb to remember. -/
def Decoder.poc (dec : Decoder) (h : SliceHeader) (sps : Sps) : Except String (Int × Int × Int × Int) := do
  match sps.pocType with
  | 0 =>
    let maxLsb : Int := (2 : Int) ^ sps.log2MaxPocLsb
    let (pMsb, pLsb) := if h.idr then (0, 0) else (dec.prevPocMsb, dec.prevPocLsb)
    let lsb : Int := h.pocLsb
    let msb :=
      if lsb < pLsb && pLsb - lsb ≥ maxLsb / 2 then pMsb + maxLsb
      else if lsb > pLsb && lsb - pLsb > maxLsb / 2 then pMsb - maxLsb
      else pMsb
    let top := msb + lsb
    let bottom := top + h.deltaPocBottom
    return (min top bottom, msb, lsb, 0)
  | 2 =>
    let maxFrameNum : Int := (2 : Int) ^ sps.log2MaxFrameNum
    let offset : Int :=
      if h.idr then 0
      else if dec.prevFrameNum > h.frameNum then dec.prevFrameNumOffset + maxFrameNum
      else dec.prevFrameNumOffset
    let poc := if h.idr then 0 else if h.nalRefIdc == 0 then 2 * (offset + h.frameNum) - 1 else 2 * (offset + h.frameNum)
    return (poc, 0, 0, offset)
  | _ => throw "H.264: pic_order_cnt_type 1 is not supported yet"

/-! ## Reference lists -/

def Decoder.maxFrameNum (sps : Sps) : Nat := 2 ^ sps.log2MaxFrameNum

/-- FrameNumWrap of a short-term reference while decoding frame `cur`. -/
def frameNumWrap (sps : Sps) (cur fn : Nat) : Int :=
  if fn > cur then Int.ofNat fn - Decoder.maxFrameNum sps else fn

/-- The initial lists (FFmpeg h264_initialise_ref_list) and the
modifications (8.2.4.3), truncated to the active counts. -/
def Decoder.refLists (dec : Decoder) (h : SliceHeader) (sps : Sps) (curPoc : Int) :
    Except String (Array RefPic × Array RefPic) := do
  let shorts := dec.refs.filter (·.longIdx.isNone)
  let longs := sortBy (dec.refs.filter (·.longIdx.isSome)) (fun a b => a.longIdx.getD 0 ≤ b.longIdx.getD 0)
  let byRecency := shorts.reverse  -- most recently decoded first (refs are appended)
  let sortPoc (xs : Array StoredPic) (desc : Bool) :=
    sortBy xs (fun a b => if desc then a.pic.poc ≥ b.pic.poc else a.pic.poc ≤ b.pic.poc)
  let (l0, l1) : Array StoredPic × Array StoredPic :=
    if h.type == .b then
      let before := sortPoc (shorts.filter (·.pic.poc ≤ curPoc)) true
      let after := sortPoc (shorts.filter (·.pic.poc > curPoc)) false
      let l0 := before ++ after ++ longs
      let l1 := after ++ before ++ longs
      let same := Id.run do
        if l0.size != l1.size || l0.size ≤ 1 then return false
        for k in [0:l0.size] do
          if l0[k]!.pic.id != l1[k]!.pic.id then return false
        return true
      (l0, if same then (l1.set! 0 l0[1]!).set! 1 l0[0]! else l1)
    else (byRecency ++ longs, #[])
  -- 8.2.4.3: each modification inserts a picture at refIdx and removes its
  -- later duplicates only (a picture may appear twice, e.g. with two weights).
  -- Entries still missing get the default reference (FFmpeg's default_ref).
  let modify (init : Array StoredPic) (mods : Array (Nat × Nat)) (count : Nat) : Except String (Array RefPic) := do
    let mut list : Array (Option StoredPic) := #[]
    for k in [0:count + 1] do list := list.push init[k]?
    let maxFn : Int := Decoder.maxFrameNum sps
    let currPicNum : Int := h.frameNum
    let mut pred := currPicNum
    let mut idx := 0
    for (op, v) in mods do
      if idx ≥ count then throw "H.264: too many reference list modifications"
      let target ←
        if op == 0 || op == 1 then
          let delta : Int := v + 1
          let mut noWrap := if op == 0 then pred - delta else pred + delta
          if noWrap < 0 then noWrap := noWrap + maxFn
          if noWrap ≥ maxFn then noWrap := noWrap - maxFn
          pred := noWrap
          let picNum := if noWrap > currPicNum then noWrap - maxFn else noWrap
          match shorts.find? (fun s => frameNumWrap sps h.frameNum s.frameNum == picNum) with
          | some s => pure s
          | none => throw "H.264: reference list modification names a missing picture"
        else
          match longs.find? (fun s => s.longIdx == some v) with
          | some s => pure s
          | none => throw "H.264: reference list modification names a missing long-term picture"
      let mut c := count
      while c > idx do
        list := list.set! c list[c - 1]!
        c := c - 1
      list := list.set! idx (some target)
      idx := idx + 1
      let mut n := idx
      for k in [idx:count + 1] do
        let keep := match list[k]! with
          | some p => p.pic.id != target.pic.id || p.longIdx.isSome != target.longIdx.isSome
          | none => true
        if keep then
          list := list.set! n list[k]!
          n := n + 1
    if count == 0 then return #[]
    let some default := init[0]? | throw "H.264: no reference pictures"
    return (list.extract 0 count).map fun e => (e.getD default).pic
  let r0 ← modify l0 h.modifications.1 h.numRefIdx.1
  let r1 ← if h.type == .b then modify l1 h.modifications.2 h.numRefIdx.2 else pure #[]
  return (r0, r1)

/-! ## Weighted prediction -/

/-- C integer division (truncating toward zero). -/
def cdiv (a b : Int) : Int :=
  let q := Int.ofNat (a.natAbs / b.natAbs)
  if (a < 0) != (b < 0) then -q else q

def clipI8 (v : Int) : Int := max (-128) (min 127 v)

/-- The weighted prediction tables of a slice (FFmpeg ff_h264_pred_weight_table
and implicit_weight_table). -/
def weightTables (d : Dec) (h : SliceHeader) (pps : Pps) (curPoc : Int) : Dec := Id.run do
  let d := { d with sl := { d.sl with useWeight := 0, useWeightChroma := false } }
  if h.explicitWeights then
    let ld : Int := (2 : Int) ^ h.lumaLog2Denom
    let cd : Int := (2 : Int) ^ h.chromaLog2Denom
    let lumaOf (w : Weight) : Int × Int := w.luma.getD (ld, 0)
    let chromaOf (w : Weight) : (Int × Int) × (Int × Int) := w.chroma.getD ((cd, 0), (cd, 0))
    let ws0 := h.weights.1
    let ws1 := h.weights.2
    let l0 := ws0.map lumaOf
    let l1 := ws1.map lumaOf
    let c0 := ws0.map chromaOf
    let c1 := ws1.map chromaOf
    let lumaUsed := (l0 ++ l1).any (· != (ld, 0))
    let chromaUsed := (c0 ++ c1).any fun (a, b) => a != (cd, 0) || b != (cd, 0)
    return { d with sl := { d.sl with useWeight := if lumaUsed || chromaUsed then 1 else 0, useWeightChroma := chromaUsed,
                                      lumaDenom := h.lumaLog2Denom, chromaDenom := h.chromaLog2Denom,
                                      lumaW0 := l0, lumaW1 := l1, chromaW0 := c0, chromaW1 := c1 } }
  if pps.weightedBipredIdc == 2 && h.type == .b then
    let n0 := d.sl.refList0.size
    let n1 := d.sl.refList1.size
    if n0 == 1 && n1 == 1 && d.sl.refList0[0]!.poc + d.sl.refList1[0]!.poc == 2 * curPoc then return d
    let mut w : Array Int := Array.replicate (32 * 32) 32
    for r0 in [0:n0] do
      for r1 in [0:n1] do
        let p0 := d.sl.refList0[r0]!
        let p1 := d.sl.refList1[r1]!
        let mut v : Int := 32
        if !p0.longTerm && !p1.longTerm then
          let td := clipI8 (p1.poc - p0.poc)
          if td != 0 then
            let tb := clipI8 (curPoc - p0.poc)
            let tx := cdiv (16384 + td.natAbs / 2) td
            let dsf := (tb * tx + 32) >>> 8
            if dsf ≥ -64 && dsf ≤ 128 then v := 64 - dsf
        w := w.set! (32 * r0 + r1) v
    return { d with sl := { d.sl with useWeight := 2, useWeightChroma := true, lumaDenom := 5, chromaDenom := 5,
                                      implicitW := w } }
  return d

/-- Temporal direct's tables for a B slice (FFmpeg
ff_h264_direct_dist_scale_factor and ff_h264_direct_ref_list_init, frames). -/
def directTables (d : Dec) (h : SliceHeader) (curPoc : Int) : Dec := Id.run do
  if h.type != .b || h.directSpatial || d.sl.refList1.isEmpty then return d
  let poc1 := d.sl.refList1[0]!.poc
  let scale := d.sl.refList0.map fun p =>
    let td := clipI8 (poc1 - p.poc)
    if td == 0 || p.longTerm then (256 : Int)
    else
      let tb := clipI8 (curPoc - p.poc)
      let tx := cdiv (16384 + td.natAbs / 2) td
      max (-1024) (min 1023 ((tb * tx + 32) >>> 6))
  let col := d.sl.refList1[0]!
  let ident (p : RefPic) : Int := 4 * Int.ofNat p.frameNum + 3
  let mapOf (refPocs : Array Int) : Array Nat := Id.run do
    let mut m : Array Nat := Array.replicate 48 0
    for k in [0:refPocs.size] do
      let poc := refPocs[k]!  -- already 4 * frame_num + 3 (frames)
      for j in [0:d.sl.refList0.size] do
        if ident d.sl.refList0[j]! == poc then
          m := m.set! k j
          break
    return m
  return { d with sl := { d.sl with distScale := scale, mapCol0 := mapOf col.refPoc0, mapCol1 := mapOf col.refPoc1 } }

/-! ## Pictures -/

/-- `n` rows of `ow` bytes of `src` (from `so`, stride `ss`) appended to `dst`. -/
def cropRows (dst : ByteArray) (src : @& ByteArray) (so ss ow : Nat) : Nat → ByteArray
  | 0 => dst
  | n + 1 => cropRows (src.copySlice so dst dst.size ow) src (so + ss) ss ow n

/-- Crop the planes for output. -/
def cropPlanes (d : Dec) (sps : Sps) (poc : Int) : OutFrame :=
  let (cl, cr, ct, cb) := sps.crop
  let w := d.mbW * 16
  let ow := w - cl - cr
  let oh := d.mbH * 16 - ct - cb
  let cw := w / 2
  let co := ct / 2 * cw + cl / 2
  let y := cropRows (.emptyWithCapacity (ow * oh)) d.pic.y (ct * w + cl) w ow oh
  let u := cropRows (.emptyWithCapacity (ow * oh / 4)) d.pic.cb co cw (ow / 2) (oh / 2)
  let v := cropRows (.emptyWithCapacity (ow * oh / 4)) d.pic.cr co cw (ow / 2) (oh / 2)
  { poc, width := ow, height := oh, y, cb := u, cr := v }

/-- Reference marking after a reference picture (8.2.5): sliding window or
MMCO, then the current picture. -/
def Decoder.mark (dec : Decoder) (h : SliceHeader) (sps : Sps) (pic : RefPic) : Decoder := Id.run do
  let mut refs := dec.refs
  if h.idr then
    refs := #[]
    return { dec with refs := #[{ pic, frameNum := h.frameNum, longIdx := if h.longTermReference then some 0 else none }] }
  let mut curLong : Option Nat := none
  match h.mmcos with
  | none =>
    let shorts := refs.filter (·.longIdx.isNone)
    if refs.size ≥ max sps.maxNumRefFrames 1 && shorts.size > 0 then
      -- drop the short-term reference with the smallest FrameNumWrap
      let oldest := shorts.foldl (fun a b =>
        if frameNumWrap sps h.frameNum b.frameNum < frameNumWrap sps h.frameNum a.frameNum then b else a) shorts[0]!
      refs := refs.filter (·.pic.id != oldest.pic.id)
  | some ops =>
    for m in ops do
      let picNumOf (diff : Nat) : Int := Int.ofNat h.frameNum - (diff + 1)
      match m.op with
      | 1 => refs := refs.filter fun s => !(s.longIdx.isNone && frameNumWrap sps h.frameNum s.frameNum == picNumOf m.a)
      | 2 => refs := refs.filter fun s => s.longIdx != some m.a
      | 3 =>
        refs := refs.filter fun s => s.longIdx != some m.b
        refs := refs.map fun s =>
          if s.longIdx.isNone && frameNumWrap sps h.frameNum s.frameNum == picNumOf m.a then { s with longIdx := some m.b } else s
      | 4 => refs := refs.filter fun s => match s.longIdx with | some i => i < m.a | none => true
      | 5 => refs := #[]
      | 6 =>
        refs := refs.filter fun s => s.longIdx != some m.a
        curLong := some m.a
      | _ => pure ()
  let mut pic := pic
  if (h.mmcos.getD #[]).any (·.op == 5) then pic := { pic with poc := 0 }
  refs := refs.map fun s => { s with pic := { s.pic with longTerm := s.longIdx.isSome } }
  return { dec with refs := refs.push { pic := { pic with longTerm := curLong.isSome }, frameNum := h.frameNum, longIdx := curLong } }

/-- Finish the current picture: loop filter, keep it as a reference if it
is one, queue it for output. -/
def Decoder.finishPicture (dec : Decoder) : Decoder := Id.run do
  let some (d, h, sps, poc) := dec.cur | return dec
  if dec.mbsDecoded != d.mbW * d.mbH then
    return { dec with cur := none, pending := dec.pending, maxTailBits := 1000000 }
  let d := if dec.deblock then deblockPicture d else d
  let mut dec := { dec with cur := none }
  let frame := { cropPlanes d sps poc with pts := dec.curPts }
  -- output in POC order: an IDR or MMCO 5 flushes the frames before it
  let flush := h.idr || (h.mmcos.getD #[]).any (·.op == 5)
  if flush then
    dec := { dec with out := dec.out ++ sortBy dec.pending (·.poc ≤ ·.poc), pending := #[] }
  dec := { dec with pending := dec.pending.push frame }
  -- bumping: more pending than the stream may reorder releases the first in POC order
  let reorder := sps.numReorderFrames.getD 16
  while dec.pending.size > reorder do
    let sorted := sortBy dec.pending (·.poc ≤ ·.poc)
    dec := { dec with out := dec.out.push sorted[0]!, pending := sorted.extract 1 sorted.size }
  if h.nalRefIdc != 0 then
    let fnId (p : RefPic) : Int := 4 * Int.ofNat p.frameNum + 3
    let pic : RefPic := { y := d.pic.y, cb := d.pic.cb, cr := d.pic.cr, poc, id := dec.nextId, mbType := d.pic.mbType,
                          mv0 := d.pic.mv0, mv1 := d.pic.mv1, ref0 := d.pic.ref0, ref1 := d.pic.ref1, frameNum := h.frameNum,
                          refPoc0 := d.sl.refList0.map fnId, refPoc1 := d.sl.refList1.map fnId }
    dec := { dec.mark h sps pic with nextId := dec.nextId + 1 }
    let (_, msb, lsb, off) := (Decoder.poc dec h sps).toOption.getD (0, 0, 0, 0)
    let mmco5 := (h.mmcos.getD #[]).any (·.op == 5)
    dec := { dec with prevPocMsb := if mmco5 then 0 else msb, prevPocLsb := if mmco5 then 0 else lsb }
    let _ := off
  dec := { dec with prevFrameNum := h.frameNum }
  return dec

/-- All frames still waiting, in POC order. -/
def Decoder.flushOutput (dec : Decoder) : Decoder :=
  let dec := dec.finishPicture
  { dec with out := dec.out ++ sortBy dec.pending (·.poc ≤ ·.poc), pending := #[] }

/-- The loop filter's identity of a reference picture: its index in the
DPB (FFmpeg ref2frm), small enough for the int8 caches; 60 if absent. -/
def Decoder.dpbIndex (dec : Decoder) (p : RefPic) : Nat :=
  (dec.refs.findIdx? (·.pic.id == p.id)).getD 60

/-- Decode one NAL unit (with its header byte). -/
def Decoder.decodeNal (dec : Decoder) (nal : ByteArray) : Except String Decoder := do
  if nal.size == 0 then return dec
  let t := (nal[0]! &&& 31).toNat
  let refIdc := ((nal[0]! >>> 5) &&& 3).toNat
  let rbsp := unescape nal
  match t with
  | 7 =>
    let sps ← parseSps.run' { data := rbsp }
    return { dec with spss := dec.spss.set! sps.id (some sps) }
  | 8 =>
    let pps ← (parsePps dec.spss).run' { data := rbsp }
    return { dec with ppss := dec.ppss.set! pps.id (some pps) }
  | 1 | 5 =>
    let (h, sps, pps) ← (parseSliceHeader t refIdc dec.spss dec.ppss).run' { data := rbsp }
    if refIdc == 0 && dec.skipNonRef && sps.pocType == 0 then return dec
    let mut dec := dec
    if h.firstMb == 0 then dec := dec.finishPicture
    let (d, poc) ← match dec.cur with
      | some (d, _, _, poc) => pure (d, poc)
      | none =>
        let (poc, _, _, off) ← dec.poc h sps
        if sps.pocType == 2 then dec := { dec with prevFrameNumOffset := off }
        dec := { dec with sliceCount := 0, curPts := dec.pts }
        pure (Dec.new sps.mbWidth sps.mbHeight, poc)
    let (l0, l1) ← dec.refLists h sps poc
    let d := d.startSlice rbsp h sps pps (dec.sliceCount + 1)
    let d := { d with sl := { d.sl with refList0 := l0, refList1 := l1 } }
    let info : SliceInfo := { deblock := (match h.disableDeblocking with | 0 => 1 | 1 => 0 | k => k),
                              alphaOffset := h.alphaOffset, betaOffset := h.betaOffset, listCount := d.sl.listCount,
                              refIds0 := l0.map dec.dpbIndex, refIds1 := l1.map dec.dpbIndex,
                              refMap0 := refMapOf (l0.map dec.dpbIndex), refMap1 := refMapOf (l1.map dec.dpbIndex),
                              cqpOff0 := pps.chromaQpIndexOffset, cqpOff1 := pps.secondChromaQpIndexOffset,
                              cqpTab0 := d.sl.cqpTab0, cqpTab1 := d.sl.cqpTab1 }
    let d := directTables (weightTables { d with sl := { d.sl with slices := d.sl.slices.push info } } h pps poc) h poc
    if h.firstMb != (if h.firstMb == 0 then 0 else dec.mbsDecoded) then
      throw s!"H.264: slice starts at macroblock {h.firstMb}, previous slice ended at {dec.mbsDecoded}"
    let (d, n) := d.decodeSlice dec.recon
    if let some e := d.err then throw s!"{e} (at macroblock {d.mbX}, {d.mbY})"
    let tail := rbsp.size * 8 - min (rbsp.size * 8) d.cabac.bitPos.toNat
    return { dec with cur := some (d, h, sps, poc), sliceCount := dec.sliceCount + 1,
                      mbsDecoded := (if h.firstMb == 0 then 0 else dec.mbsDecoded) + n,
                      maxTailBits := max dec.maxTailBits tail }
  | _ => return dec

end Yt.H264
