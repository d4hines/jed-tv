/-
`Yt.H264.Kernels` on the host (upstream Lean): the decoder's Lean sample
code exported under the kernels' symbols. Host executables import this
module; builds for the TV never link it (the runtime's `ZrTv.H264Kernels`
provides the symbols there; test scripts skip `Yt/Host`).
-/
import Yt.H264.Inter
import Yt.H264.Filter
import Yt.H264.State
import Yt.H264.Transform

namespace Yt.Host.H264Kernels
open Yt.H264

@[export zr_h264_qpel]
def qpel (dst : ByteArray) (dOff : USize) (src : ByteArray) (sOff stride w h mode : USize) : ByteArray :=
  let xy := (mode &&& 15).toNat
  let s := stride.toNat
  -- lumaFrom takes the window from 2 rows and 2 columns before the block,
  -- which the caller only guarantees when the fraction needs them: before
  -- the plane's start, unused bytes stand in for them
  let (src, o) := if sOff.toNat ≥ 2 * s + 2 then (src, sOff.toNat) else (Yt.Bytes.zeros (2 * s + 2) ++ src, sOff.toNat + 2 * s + 2)
  Inter.lumaFrom dst dOff.toNat s src (o - 2 * s - 2) s (xy % 4) (xy / 4) w.toNat h.toNat (mode &&& 16 != 0)

@[export zr_h264_chroma]
def chroma (dst : ByteArray) (dOff : USize) (src : ByteArray) (sOff stride w h fx fy avg : USize) : ByteArray :=
  let x := fx.toUInt32
  let y := fy.toUInt32
  Inter.chromaRows dst dOff.toNat stride.toNat src sOff.toNat stride.toNat w.toNat
    ((8 - x) * (8 - y)) (x * (8 - y)) ((8 - x) * y) (x * y) (avg != 0) h.toNat

/-- Byte `i` of `tc` as a 32-bit two's-complement int8. -/
@[inline] def tcOf (tc : USize) (i : Nat) : UInt32 :=
  let v := ((tc >>> (8 * i).toUSize) &&& 0xFF).toUInt32
  if v ≥ 128 then v ||| 0xFFFFFF00 else v

@[export zr_h264_loop_filter]
def loopFilter (plane : ByteArray) (off stride alpha beta tc dir chroma : USize) : ByteArray :=
  let (xs, ys) := if dir == 0 then (1, stride.toNat) else (stride.toNat, 1)
  if chroma != 0 then
    Filter.chroma32 plane off.toNat xs ys alpha.toNat beta.toNat (tcOf tc 0) (tcOf tc 1) (tcOf tc 2) (tcOf tc 3)
  else Filter.luma32 plane off.toNat xs ys alpha.toNat beta.toNat (tcOf tc 0) (tcOf tc 1) (tcOf tc 2) (tcOf tc 3)

def fillRow (b : ByteArray) (o : Nat) (v : UInt32) (esize : Nat) : Nat → ByteArray
  | 0 => b
  | w + 1 =>
    let b := b.set! o v.toUInt8
    let b := if esize ≥ 2 then b.set! (o + 1) (v >>> 8).toUInt8 else b
    let b := if esize == 4 then (b.set! (o + 2) (v >>> 16).toUInt8).set! (o + 3) (v >>> 24).toUInt8 else b
    fillRow b (o + esize) v esize w

def fillRows (b : ByteArray) (o stride w : Nat) (v : UInt32) (esize : Nat) : Nat → ByteArray
  | 0 => b
  | h + 1 => fillRows (fillRow b o v esize w) (o + stride) stride w v esize h

/-- ZrTv.H264Kernels.blockInside. -/
def blockInside (size off stride n h : Nat) : Bool :=
  h != 0 && h ≤ 4096 && stride ≤ 65536 && n ≤ 65536 && off + (h - 1) * stride + n ≤ size

@[export zr_h264_fill]
def fill (b : ByteArray) (off stride w h v esize : USize) : ByteArray :=
  if !blockInside b.size off.toNat stride.toNat (w.toNat * esize.toNat) h.toNat || w.toNat > 4096 then b
  else fillRows b off.toNat stride.toNat w.toNat v.toUInt32 esize.toNat h.toNat

def copyRowsGo (dst : ByteArray) (d ds : Nat) (src : ByteArray) (s ss w : Nat) : Nat → ByteArray
  | 0 => dst
  | h + 1 => copyRowsGo (src.copySlice s dst d w) (d + ds) ds src (s + ss) ss w h

@[export zr_h264_copy_rows]
def copyRows (dst : ByteArray) (dOff dStride : USize) (src : ByteArray) (sOff sStride w h : USize) : ByteArray :=
  if !blockInside dst.size dOff.toNat dStride.toNat w.toNat h.toNat ||
     !blockInside src.size sOff.toNat sStride.toNat w.toNat h.toNat then dst
  else copyRowsGo dst dOff.toNat dStride.toNat src sOff.toNat sStride.toNat w.toNat h.toNat

/-- `n` int16 coefficients from `coef[2 * off]`. -/
def coefsAt (coef : ByteArray) (off n : Nat) : Array Int :=
  (List.range n).foldl (fun a i => a.push (getI16 coef (off + i))) (Array.mkEmpty n)

@[export zr_h264_idct]
def idct (plane : ByteArray) (po stride : USize) (coef : ByteArray) (off kind : USize) : ByteArray :=
  let n := if kind ≥ 2 then 64 else 16
  let rows := if kind ≥ 2 then 8 else 4
  if plane.size < 512 || !blockInside (plane.size - 256) po.toNat stride.toNat rows rows || off > 4096 ||
     2 * (off.toNat + n) > coef.size then plane
  else
    let c := coefsAt coef off.toNat n
    let (p, s) := (po.toNat, stride.toNat)
    if kind == 0 then Transform.idct4Add plane p s c
    else if kind == 1 then Transform.idct4DcAdd plane p s c[0]!
    else if kind == 2 then Transform.idct8Add plane p s c
    else Transform.idct8DcAdd plane p s c[0]!

def predRows (b : ByteArray) (o s w : Nat) (dir : USize) : Nat → ByteArray
  | 0 => b
  | h + 1 =>
    let b := if dir == 0 then b.copySlice (o - s) b o w else fillRows b o 0 w (b[o - 1]!).toUInt32 1 1
    predRows b (o + s) s w dir h

@[export zr_h264_pred_copy]
def predCopy (b : ByteArray) (off stride w h dir : USize) : ByteArray :=
  let ok :=
    if dir == 0 then off ≥ stride && blockInside b.size (off - stride).toNat stride.toNat w.toNat (h.toNat + 1)
    else off ≥ 1 && blockInside b.size (off - 1).toNat stride.toNat (w.toNat + 1) h.toNat
  if !ok || w > 64 then b
  else
    -- vertical: every row copies the row above the block
    if dir == 0 then
      let above := b.extract (off - stride).toNat (off - stride + w).toNat
      (List.range h.toNat).foldl (fun b i => above.copySlice 0 b (off.toNat + i * stride.toNat) w.toNat) b
    else predRows b off.toNat stride.toNat w.toNat dir h.toNat

@[export zr_h264_intra_filter]
def intraFilter (b : ByteArray) (off xs ys alpha beta chroma : USize) : ByteArray :=
  let n : Nat := if chroma != 0 then 8 else 16
  let reach : Nat := if chroma != 0 then 2 else 4
  if xs > 65536 || ys > 65536 || off.toNat < reach * xs.toNat ||
     off.toNat + (n - 1) * ys.toNat + (reach - 1) * xs.toNat ≥ b.size then b
  else if chroma != 0 then Filter.chromaIntraU b off xs ys alpha.toUInt32 beta.toUInt32
  else Filter.lumaIntraU b off xs ys alpha.toUInt32 beta.toUInt32

/-! `zr_h264_mb_bs`: FFmpeg's filter_mb_dir boundary strengths of a frame
macroblock from the picture's tables (see Yt.H264.Kernels.mbBs). -/

namespace Bs

structure Tabs where
  nz : ByteArray
  r0 : ByteArray
  r1 : ByteArray
  v0 : ByteArray
  v1 : ByteArray
  maps : ByteArray
  two : Bool

/-- The picture of stored reference byte `r` through the map at `maps[m]`. -/
def mapRef (tb : Tabs) (m r : Nat) : Nat := if r ≥ 128 then 0xFF else tb.maps[m + r]!.toNat

/-- Whether packed vectors `a`, `b` (x, y int16) differ by 4 or more in a component. -/
def far (a b : UInt32) : Bool :=
  let s16 (v : UInt32) : Int := let v := Int.ofNat v.toNat; if v ≥ 32768 then v - 65536 else v
  (s16 (a &&& 0xFFFF) - s16 (b &&& 0xFFFF)).natAbs ≥ 4 || (s16 (a >>> 16) - s16 (b >>> 16)).natAbs ≥ 4

/-- `check_mv` between block (qc, qr) of macroblock `qxy` (maps at `qa`, `qb`)
and (pc, pr) of `pxy` (maps at `pa`, `pb`). -/
def checkMv (tb : Tabs) (qa qb pa pb qxy qc qr pxy pc pr : Nat) : Bool :=
  let qri := 4 * qxy + 2 * (qr / 2) + qc / 2
  let pri := 4 * pxy + 2 * (pr / 2) + pc / 2
  let qm := 16 * qxy + qc + 4 * qr
  let pm := 16 * pxy + pc + 4 * pr
  let q0 := mapRef tb qa tb.r0[qri]!.toNat
  let p0 := mapRef tb pa tb.r0[pri]!.toNat
  let a0 := getU32 tb.v0 qm
  let b0 := getU32 tb.v0 pm
  let v := q0 != p0 || (q0 != 0xFF && far a0 b0)
  if !tb.two then v
  else
    let q1 := mapRef tb qb tb.r1[qri]!.toNat
    let p1 := mapRef tb pb tb.r1[pri]!.toNat
    let a1 := getU32 tb.v1 qm
    let b1 := getU32 tb.v1 pm
    let v := v || q1 != p1 || far a1 b1
    if !v then false
    else if q0 != p1 || q1 != p0 then true
    else far a0 b1 || far a1 b0

/-- bS of segment `i` of edge `edge` (P across edge 0 in `nxy`). -/
def segBs (tb : Tabs) (qa qb pa pb dir edge i xy nxy : Nat) (known : Bool) (v : Nat) : Nat :=
  let qc := if dir == 0 then edge else i
  let qr := if dir == 0 then i else edge
  let pc := if dir == 0 then (if edge == 0 then 3 else edge - 1) else i
  let pr := if dir == 0 then i else (if edge == 0 then 3 else edge - 1)
  let pxy := if edge == 0 then nxy else xy
  if tb.nz[48 * xy + 4 * qr + qc]! != 0 || tb.nz[48 * pxy + 4 * pr + pc]! != 0 then 2
  else if known then v
  else if checkMv tb qa qb pa pb xy qc qr pxy pc pr then 1 else 0

def edgeBs (tb : Tabs) (qa qb pa pb dir edge xy nxy : Nat) (known : Bool) (v : Nat) : UInt32 :=
  (segBs tb qa qb pa pb dir edge 0 xy nxy known v).toUInt32 |||
    ((segBs tb qa qb pa pb dir edge 1 xy nxy known v).toUInt32 <<< 8) |||
    ((segBs tb qa qb pa pb dir edge 2 xy nxy known v).toUInt32 <<< 16) |||
    ((segBs tb qa qb pa pb dir edge 3 xy nxy known v).toUInt32 <<< 24)

def maskEdgeTab : Array (Array Nat) := #[#[0, 3, 3, 3, 1, 1, 1, 1], #[0, 3, 1, 1, 3, 3, 3, 3]]

/-- The four edges of direction `dir` (strengths of edge e at entry e). -/
def dirBs (tb : Tabs) (qa la : Nat) (dir xy : Nat) (t mbmT : UInt32) (mbmXY cbp cbpN : Nat) : Array UInt32 :=
  let me := maskEdgeTab[dir]![((t >>> 3) &&& 7).toNat]!
  let edges := if me == 3 && cbp == 0 then 1 else 4
  let part : UInt32 := MB_16x16 ||| (MB_8x16 >>> dir.toUInt32)
  let par0 := t &&& part != 0
  let e0 : UInt32 :=
    if mbmT == 0 then 0
    else if isIntra (t ||| mbmT) then 0x04040404
    else
      let known := par0 && mbmT &&& part != 0
      let v := if known && checkMv tb qa (qa + 128) la (la + 128) xy 0 0 mbmXY (if dir == 0 then 3 else 0)
        (if dir == 0 then 0 else 3) then 1 else 0
      if known && cbp == 0 && cbpN == 0 then v.toUInt32 * 0x01010101
      else edgeBs tb qa (qa + 128) la (la + 128) dir 0 xy mbmXY known v
  let inner (edge : Nat) : UInt32 :=
    if edge ≥ edges || (is8x8dct t && edge &&& 1 == 1) then 0
    else if isIntra t then 0x03030303
    else
      let skipMv := edge &&& me != 0
      let v := if !skipMv && par0 && checkMv tb qa (qa + 128) qa (qa + 128) xy (if dir == 0 then edge else 0)
        (if dir == 0 then 0 else edge) xy (if dir == 0 then edge - 1 else 0) (if dir == 0 then 0 else edge - 1)
        then 1 else 0
      if (skipMv || par0) && cbp == 0 then v.toUInt32 * 0x01010101
      else edgeBs tb qa (qa + 128) qa (qa + 128) dir edge xy xy (skipMv || par0) v
  #[e0, inner 1, inner 2, inner 3]

end Bs

@[export zr_h264_mb_bs]
def mbBs (out nnz ref0 ref1 mv0 mv1 maps : ByteArray) (xy mbW t leftT topT sc sl st two cbps : USize) : ByteArray :=
  let n := xy.toNat + 1
  let nm := maps.size / 256
  let ok := out.size ≥ 32 && nnz.size ≥ 48 * n && ref0.size ≥ 4 * n && mv0.size ≥ 64 * n &&
    (two == 0 || (ref1.size ≥ 4 * n && mv1.size ≥ 64 * n)) && sc.toNat < nm && sl.toNat < nm && st.toNat < nm &&
    (leftT == 0 || xy ≥ 1) && (topT == 0 || xy ≥ mbW)
  if !ok then out
  else
    let tb : Bs.Tabs := { nz := nnz, r0 := ref0, r1 := ref1, v0 := mv0, v1 := mv1, maps, two := two != 0 }
    let cbp := (cbps &&& 15).toNat
    let x := xy.toNat
    let d0 := Bs.dirBs tb (256 * sc.toNat) (256 * sl.toNat) 0 x t.toUInt32 leftT.toUInt32 (x - 1) cbp ((cbps >>> 4) &&& 15).toNat
    let d1 := Bs.dirBs tb (256 * sc.toNat) (256 * st.toNat) 1 x t.toUInt32 topT.toUInt32 (x - mbW.toNat) cbp
      ((cbps >>> 8) &&& 15).toNat
    (d0 ++ d1).foldl (fun (b, k) v => (setU32 b k v, k + 1)) (out, 0) |>.1

end Yt.Host.H264Kernels
