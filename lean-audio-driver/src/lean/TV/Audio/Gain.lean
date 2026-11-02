prelude
import Init.Data.UInt
import Init.Data.SInt.Basic
import Init.Data.ByteArray.Basic
import Init.Data.Range

/- Global gain for everything the TV plays. A sample is first boosted (the
app asks for 1..4), clamped to full scale, then attenuated by the global gain.
The gain unit is millionths of full scale, the same as `TV.Volume`; the
calibrated ceiling is 0.30, so no output sample can exceed 9830 = 32767 * 0.3. -/
namespace TV.Audio.Gain

def ceiling : UInt32 := 300000
def unit : Int64 := 1000000
/-- The largest magnitude `scale` can produce: floor(32767 * 300000 / 1000000). -/
def maxOutput : Int64 := 9830

/-- The little-endian signed 16-bit sample at byte index `i`. -/
@[inline] def readInt16 (bytes : ByteArray) (i : Nat) : Int16 :=
  ((bytes.get! i).toUInt16 ||| ((bytes.get! (i + 1)).toUInt16 <<< 8)).toInt16

@[inline] def pushInt16 (bytes : ByteArray) (v : Int16) : ByteArray :=
  let raw := v.toUInt16
  (bytes.push raw.toUInt8).push (raw >>> 8).toUInt8

/-- `clamp(sample * boost, -32767, 32767) * min(gain, ceiling) / 1000000`,
with the division truncating toward zero. -/
@[inline] def scale (boost gain : UInt32) (sample : Int16) : Int16 :=
  let boosted : Int64 := sample.toInt64 * boost.toUInt64.toInt64
  let clamped : Int64 := if boosted < -32767 then -32767 else if boosted > 32767 then 32767 else boosted
  let gain : Int64 := (if gain > ceiling then ceiling else gain).toUInt64.toInt64
  (clamped * gain / unit).toInt16

structure Scaled where
  output : ByteArray
  sourcePeak : UInt32
  outputPeak : UInt32

@[inline] def magnitude (v : Int16) : UInt32 :=
  if v < 0 then (0 - v.toInt32).toUInt32 else v.toInt32.toUInt32

/-- Every pair of bytes is one little-endian sample; a trailing odd byte is dropped. -/
def scaleBlock (boost gain : UInt32) (pcm : ByteArray) : Scaled := Id.run do
  let mut out := ByteArray.emptyWithCapacity pcm.size
  let mut sourcePeak : UInt32 := 0
  let mut outputPeak : UInt32 := 0
  for i in [0:pcm.size / 2] do
    let sample := readInt16 pcm (2 * i)
    let scaled := scale boost gain sample
    sourcePeak := max sourcePeak (magnitude sample)
    outputPeak := max outputPeak (magnitude scaled)
    out := pushInt16 out scaled
  return { output := out, sourcePeak, outputPeak }

end TV.Audio.Gain
