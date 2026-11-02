prelude
import Init.Data.UInt
import Init.Data.SInt.Basic
import Init.Data.ByteArray.Basic

/- The socket protocol apps use to play sound: `O rate channels boost`, then
`W bytes data...`, `D` (counters), `R` (reset), `F` (flush). Replies are a
little-endian `i32` status (0, -1 or -22) or two little-endian `u64`
counters. Each decision is a pure function, so the Rust side only moves
bytes. -/
namespace TV.Audio.Protocol

def ok : Int32 := 0
def failure : Int32 := -1
def einval : Int32 := -22
def maxBlockBytes : UInt32 := 1048576

/-- Rates and channel counts the direct hardware path supports. -/
@[inline] def supports (rate channels : UInt32) : Bool :=
  (rate == 44100 || rate == 48000) && (channels == 1 || channels == 2)

/-- What an `O` request may ask for: the wire's own limits, then ours. -/
@[inline] def validFormat (rate channels boost : UInt32) : Bool :=
  8000 <= rate && rate <= 96000 && 1 <= channels && channels <= 2 && 1 <= boost && boost <= 4
    && supports rate channels

/-- A block is at most 1 MiB and holds whole frames. -/
@[inline] def validBlock (channels bytes : UInt32) : Bool :=
  bytes <= maxBlockBytes && bytes % (channels * 2) == 0

@[inline] def readU32 (bytes : ByteArray) (i : Nat) : UInt32 :=
  (bytes.get! i).toUInt32 ||| ((bytes.get! (i + 1)).toUInt32 <<< 8)
    ||| ((bytes.get! (i + 2)).toUInt32 <<< 16) ||| ((bytes.get! (i + 3)).toUInt32 <<< 24)

@[inline] def readU64 (bytes : ByteArray) (i : Nat) : UInt64 :=
  (readU32 bytes i).toUInt64 ||| ((readU32 bytes (i + 4)).toUInt64 <<< 32)

@[inline] def pushU32 (bytes : ByteArray) (v : UInt32) : ByteArray :=
  (((bytes.push v.toUInt8).push (v >>> 8).toUInt8).push (v >>> 16).toUInt8).push (v >>> 24).toUInt8

@[inline] def pushU64 (bytes : ByteArray) (v : UInt64) : ByteArray :=
  pushU32 (pushU32 bytes v.toUInt32) (v >>> 32).toUInt32

@[inline] def statusBytes (status : Int32) : ByteArray := pushU32 ByteArray.empty status.toUInt32
@[inline] def counterBytes (written played : UInt64) : ByteArray := pushU64 (pushU64 ByteArray.empty written) played

end TV.Audio.Protocol
