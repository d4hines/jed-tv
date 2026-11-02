import ZrAsm.Ffmpeg.Common
import ZrAsm.Ffmpeg.H264Cmc
import ZrAsm.Ffmpeg.H264Dsp
import ZrAsm.Ffmpeg.H264Idct
import ZrAsm.Ffmpeg.H264Pred
import ZrAsm.Ffmpeg.H264Qpel
import ZrAsm.Ffmpeg.HpelDsp
import ZrAsm.Ffmpeg.MeCmpArmv6
import ZrAsm.Ffmpeg.StartcodeArmv6
import ZrAsm.Ffmpeg.VideodspArmv5te

/-!
# ZrAsm.Ffmpeg: FFmpeg's 32-bit ARM H.264 assembly, in Lean

One module per file of FFmpeg 9.0.1's libavcodec/arm assembly for H.264, in
FFmpeg's build order (the order of `files`). `zr-asm-gen ffmpeg-arm OUT`
writes them as OUT/<name>.S plus OUT/SOURCES; `zr-build rt` assembles the ones
the Lean decoder calls (`ZrTv.H264Kernels`) into apps, and
test/ffmpeg/AsmCheck.lean proves that each assembles to the same machine
code, data, relocations and symbols as FFmpeg's original.
-/

namespace ZrAsm.Ffmpeg

def files : List AsmFile :=
  [H264Cmc.file, H264Dsp.file, H264Idct.file, H264Pred.file, H264Qpel.file, HpelDsp.file,
   MeCmpArmv6.file, StartcodeArmv6.file, VideodspArmv5te.file]

end ZrAsm.Ffmpeg
