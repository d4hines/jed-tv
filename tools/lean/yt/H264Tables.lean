/-!
Writes `Yt/H264/Tables.lean`: the H.264 decoding tables (ITU-T H.264 9.3 and
8.7), taken as numbers from FFmpeg's C source so that none is retyped.

    lake env lean --run tools/H264Tables.lean FFMPEG_SRC Yt/H264/Tables.lean

FFMPEG_SRC: the FFmpeg 9.0.1 source tree (the one src/ffh264 was translated
from; tools/ffh264/postprocess.py names its Nix store path).
-/

def isIdentChar (c : Char) : Bool := c.isAlphanum || c == '_'

/-- The text of the C initializer `{ ... }` of table `name` in `src`
(without the outer braces and comments). The name must be a whole word. -/
def initializerText (src name : String) : IO String := do
  -- `name[...] = {` or, inside DECLARE_ASM_ALIGNED(...), `name)[...] = {`
  let mut found : Option String := none
  for key in [name ++ "[", name ++ ")["] do
    let parts := (src.splitOn key).toArray
    for k in [1:parts.size] do
      let before := parts[k - 1]!
      if found.isNone && (before.isEmpty || !isIdentChar (before.toList.getLast!)) then
        found := some (key.intercalate (parts.extract k parts.size).toList)
  let some after := found | throw (IO.userError s!"no table {name}")
  -- from the first `{` to its matching `}`
  let cs := ("{" ++ "{".intercalate (after.splitOn "{").tail).toList
  let mut depth := 0
  let mut text := ""
  for c in cs do
    if c == '{' then depth := depth + 1
    if c == '}' then depth := depth - 1
    if depth == 0 then break
    text := text.push c
  let code := "\n".intercalate ((text.splitOn "\n").map fun (l : String) => (l.splitOn "//").head!)
  let parts := code.splitOn "/*"
  return (parts.tail.foldl (fun acc (part : String) => acc ++ ((part.splitOn "*/").tail.headD "")) parts.head!).drop 1 |>.toString

def numbers (text : String) : Array Int := Id.run do
  let mut out := #[]
  let mut cur := ""
  for c in (text ++ " ").toList do
    if c.isDigit || (c == '-' && cur.isEmpty) then cur := cur.push c
    else
      if !cur.isEmpty && cur != "-" then out := out.push (cur.toInt?.getD 0)
      cur := ""
  return out

/-- The integers of table `name`'s initializer, flat. -/
def initializer (src name : String) : IO (Array Int) := return numbers (← initializerText src name)

/-- A two-dimensional table whose rows `{ ... }` may be shorter than
`width` (C zero-fills them): the rows, padded, flat. -/
def rows (src name : String) (width : Nat) : IO (Array Int) := do
  let text ← initializerText src name
  let mut out := #[]
  for part in (text.splitOn "{").tail do
    let row := numbers ((part.splitOn "}").head!)
    out := out ++ row ++ Array.replicate (width - row.size) 0
  return out

def bytes (name doc : String) (xs : Array Int) : String :=
  let vals := ", ".intercalate (xs.toList.map fun x => toString (if x < 0 then x + 256 else x))
  s!"/-- {doc} -/\ndef {name} : ByteArray := ⟨#[{vals}]⟩\n\n"

def main (args : List String) : IO UInt32 := do
  let [src, out] := args | IO.eprintln "usage: H264Tables FFMPEG_SRC OUT"; return 2
  let rd (f : String) := IO.FS.readFile (System.FilePath.mk src / "libavcodec" / f)
  let cabacH264 ← rd "h264_cabac.c"
  let cabac ← rd "cabac.c"
  let data ← rd "h264data.c"
  let lf ← rd "h264_loopfilter.c"
  let math ← rd "mathtables.c"
  let ps ← rd "h264_ps.c"
  let cavlc ← rd "h264_cavlc.c"
  let initI ← initializer cabacH264 "cabac_context_init_I"
  let initPB ← initializer cabacH264 "cabac_context_init_PB"
  let tabs ← initializer cabac "ff_h264_cabac_tables"
  let sig8 ← initializer cabacH264 "significant_coeff_flag_offset_8x8"
  let alpha ← initializer lf "alpha_table"
  let beta ← initializer lf "beta_table"
  let tc0 ← initializer lf "tc0_table"
  let zz8 ← initializer math "ff_zigzag_direct"
  let dq4 ← initializer data "ff_h264_dequant4_coeff_init"
  let dq8 ← initializer data "ff_h264_dequant8_coeff_init"
  let dq8scan ← initializer data "ff_h264_dequant8_coeff_init_scan"
  let def4 ← initializer ps "default_scaling4"
  let def8 ← initializer ps "default_scaling8"
  let ctLen ← rows cavlc "coeff_token_len" 68
  let ctBits ← rows cavlc "coeff_token_bits" 68
  let cdcLen ← initializer cavlc "chroma_dc_coeff_token_len"
  let cdcBits ← initializer cavlc "chroma_dc_coeff_token_bits"
  let tzLen := Array.replicate 16 0 ++ (← rows cavlc "total_zeros_len" 16)
  let tzBits := Array.replicate 16 0 ++ (← rows cavlc "total_zeros_bits" 16)
  let ctzLen ← rows cavlc "chroma_dc_total_zeros_len" 4
  let ctzBits ← rows cavlc "chroma_dc_total_zeros_bits" 4
  let runLen ← rows cavlc "run_len" 16
  let runBits ← rows cavlc "run_bits" 16
  let cbpIntra ← initializer data "ff_h264_golomb_to_intra4x4_cbp"
  let cbpInter ← initializer data "ff_h264_golomb_to_inter_cbp"
  unless initI.size == 2048 && initPB.size == 3 * 2048 && tabs.size == 512 + 512 + 256 + 63 &&
      sig8.size == 126 && alpha.size == 156 && beta.size == 156 && tc0.size == 624 && zz8.size == 64 &&
      dq4.size == 18 && dq8.size == 36 && dq8scan.size == 16 &&
      def4.size == 32 && def8.size == 128 && ctLen.size == 4 * 68 && ctBits.size == 4 * 68 &&
      cdcLen.size == 20 && tzLen.size == 256 && ctzLen.size == 12 && runLen.size == 112 &&
      cbpIntra.size == 48 && cbpInter.size == 48 do
    IO.eprintln s!"unexpected table sizes {[initI.size, initPB.size, tabs.size, sig8.size, alpha.size, tc0.size]} cavlc {[ctLen.size, ctBits.size, cdcLen.size, tzLen.size, ctzLen.size, runLen.size, cbpIntra.size, cbpInter.size]}"
    return 1
  -- rangeTabLPS[pStateIdx][qRangeIdx]: FFmpeg's lps_range is indexed by
  -- qRangeIdx * 128 + 2 * pStateIdx (+ valMPS).
  let lps := (List.range 64).foldl (fun acc p => (List.range 4).foldl (fun acc q =>
    acc.push tabs[512 + q * 128 + 2 * p]!) acc) #[]
  let last8 := tabs.extract (512 + 512 + 256) (512 + 512 + 256 + 63)
  let header := "/-\nGenerated by tools/H264Tables.lean from FFmpeg 9.0.1 (LGPL-2.1-or-later): the\n" ++
    "tables of ITU-T H.264. Do not edit.\n-/\n\nnamespace Yt.H264.Tables\n\n"
  let body :=
    bytes "cabacInitI" "CABAC (m, n) for I slices, ctxIdx 0..1023, as int8 (9.3.1.1)." initI ++
    bytes "cabacInitPB" "CABAC (m, n) for P/B slices by cabac_init_idc 0..2, ctxIdx 0..1023, as int8." initPB ++
    bytes "rangeTabLPS" "rangeTabLPS[pStateIdx * 4 + qRangeIdx] (Table 9-44)." lps ++
    bytes "sigCoeffFlagOffset8x8" "ctxIdxInc of significant_coeff_flag in 8x8 blocks by position: frame, field (Table 9-43)." sig8 ++
    bytes "lastCoeffFlagOffset8x8" "ctxIdxInc of last_significant_coeff_flag in 8x8 blocks by position." last8 ++
    bytes "alpha" "alpha' by indexA 0..51 (Table 8-16)." (alpha.extract 52 104) ++
    bytes "beta" "beta' by indexB 0..51 (Table 8-16)." (beta.extract 52 104) ++
    bytes "tc0" "tC0' by indexA 0..51 and bS 0..3 (index 4 * indexA + bS; Table 8-17)." (tc0.extract (52 * 4) (104 * 4)) ++
    bytes "zigzag8x8" "Frame zigzag scan of 8x8 blocks: raster index by scan position." zz8 ++
    -- ff_zigzag_scan is written as `x + y * 4` expressions: this one is Table 8-12.
    bytes "zigzag4x4" "Frame zigzag scan of 4x4 blocks: raster index by scan position (Table 8-12)."
      #[0, 1, 4, 8, 5, 2, 3, 6, 9, 12, 13, 10, 7, 11, 14, 15] ++
    bytes "dequant4Init" "normAdjust4x4 by qP % 6 and position class (8.5.9)." dq4 ++
    bytes "dequant8Init" "normAdjust8x8 by qP % 6 and position class (8.5.9)." dq8 ++
    bytes "dequant8Class" "The position class of each 8x8 coefficient, by (row & 3) * 4 + (col & 3)." dq8scan ++
    bytes "defaultScaling4" "Default_4x4_Intra then Default_4x4_Inter, raster order (Table 7-3)." def4 ++
    bytes "defaultScaling8" "Default_8x8_Intra then Default_8x8_Inter, raster order (Table 7-4)." def8 ++
    bytes "coeffTokenLen" "CAVLC coeff_token code lengths: 4 tables (nC 0-1, 2-3, 4-7, 8+) of 4 * TotalCoeff + TrailingOnes (Table 9-5)." ctLen ++
    bytes "coeffTokenBits" "CAVLC coeff_token codes, same layout." ctBits ++
    bytes "chromaDcCoeffTokenLen" "CAVLC coeff_token lengths for 4:2:0 chroma DC (nC -1)." cdcLen ++
    bytes "chromaDcCoeffTokenBits" "CAVLC coeff_token codes for 4:2:0 chroma DC." cdcBits ++
    bytes "totalZerosLen" "CAVLC total_zeros lengths: 16 by TotalCoeff (row 0 unused) of total_zeros (Tables 9-7, 9-8)." tzLen ++
    bytes "totalZerosBits" "CAVLC total_zeros codes, same layout." tzBits ++
    bytes "chromaDcTotalZerosLen" "CAVLC total_zeros lengths for 4:2:0 chroma DC: 3 by TotalCoeff - 1 of 4 (Table 9-9)." ctzLen ++
    bytes "chromaDcTotalZerosBits" "CAVLC total_zeros codes for 4:2:0 chroma DC." ctzBits ++
    bytes "runLen" "CAVLC run_before lengths: 7 by min(zerosLeft, 7) - 1 of 16 (Table 9-10)." runLen ++
    bytes "runBits" "CAVLC run_before codes, same layout." runBits ++
    bytes "golombToIntraCbp" "coded_block_pattern of an intra macroblock by its me(v) code number (Table 9-4)." cbpIntra ++
    bytes "golombToInterCbp" "coded_block_pattern of an inter macroblock by its me(v) code number." cbpInter
  IO.FS.writeFile out (header ++ body ++ "end Yt.H264.Tables\n")
  IO.println s!"{out}: written"
  return 0
