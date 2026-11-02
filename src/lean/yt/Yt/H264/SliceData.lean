/-
Decoding the macroblocks of one slice (FFmpeg `ff_h264_decode_slice`,
CABAC). Part of the Lean H.264 decoder (follows FFmpeg 9.0.1,
LGPL-2.1-or-later).
-/
import Yt.H264.Recon

namespace Yt.H264

/-- Set up the state for a slice: CABAC on its data, contexts, QP, lists. -/
def Dec.startSlice (d : Dec) (rbsp : ByteArray) (h : SliceHeader) (sps : Sps) (pps : Pps) (sliceNum : Nat) : Dec :=
  let qp := h.qp.toNat
  let ctx := initContexts qp (if h.type == .i then none else some h.cabacInitIdc)
  let listCount := match h.type with | .i => 0 | .p => 1 | .b => 2
  let sl : SliceCtx :=
    { d.sl with
      sliceType := h.type, listCount := listCount, refCount0 := h.numRefIdx.1, refCount1 := h.numRefIdx.2,
      directSpatial := h.directSpatial, cqpOff0 := pps.chromaQpIndexOffset, cqpOff1 := pps.secondChromaQpIndexOffset,
      cqpTab0 := chromaQpTab pps.chromaQpIndexOffset, cqpTab1 := chromaQpTab pps.secondChromaQpIndexOffset,
      transform8x8Mode := pps.transform8x8, constrainedIntra := pps.constrainedIntraPred,
      direct8x8Inference := sps.direct8x8Inference, scaling4 := pps.scaling4, scaling8 := pps.scaling8 }
  { d with
    cabac := Cabac.init rbsp h.dataPos ctx, sliceNum, sl, qscale := qp,
    chromaQp0 := chromaQp pps.chromaQpIndexOffset qp, chromaQp1 := chromaQp pps.secondChromaQpIndexOffset qp,
    lastQscaleDiff := 0, prevMbSkipped := false, mbX := h.firstMb % d.mbW, mbY := h.firstMb / d.mbW }

/-- Position the state at macroblock (x, y). -/
@[noinline] def Dec.setMbPos (d : Dec) (x y : Nat) : Dec := { d with mbX := x, mbY := y }

@[noinline] def Dec.terminateAt (d : Dec) : Dec := d.terminate

/-- Decode the macroblocks of the slice, reconstructing each one when
`recon`. Returns the state and the number of macroblocks decoded. -/
partial def Dec.decodeSlice (d : Dec) (recon : Bool) : Dec × Nat :=
  go d 0
where
  go (d : Dec) (n : Nat) : Dec × Nat :=
    let d := decodeMb d
    if d.err.isSome then (d, n)
    else
      let d := if recon then reconMb d else d
      let d := d.terminateAt
      let eos := d.v == 1
      let x := d.mbX + 1
      let d := if x == d.mbW then d.setMbPos 0 (d.mbY + 1) else d.setMbPos x d.mbY
      if eos || d.mbY ≥ d.mbH then (d, n + 1) else go d (n + 1)

end Yt.H264
