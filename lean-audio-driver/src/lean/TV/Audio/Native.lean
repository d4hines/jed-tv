prelude
import Init.System.IO
import TV.Audio.Session

/- The mechanisms on the TV. Each `@[extern]` is one Rust function in
`src/pid1/src/audio_service.rs` (scalars) or one C function in
`src/lean/native/audio.c` (byte arrays and strings, which then call Rust).
All of them run on the connection's own thread. -/
namespace TV.Audio.Native
open TV.Audio.Session

/-! MI request numbers, named here only so that faults say which call failed. -/
def pcmOpenRequest : UInt32 := 0xc0803402
def pcmStartRequest : UInt32 := 0xc0083404
def pcmWriteRequest : UInt32 := 0xc0383408
def pcmAttrRequest : UInt32 := 0xc018340b
def pcmStopRequest : UInt32 := 0xc0043405
def pcmCloseRequest : UInt32 := 0xc0043403
def aoutHandleRequest : UInt32 := 0xc0081116
def aoutCountRequest : UInt32 := 0xc010111e
def aoutConnectedRequest : UInt32 := 0xc018111f
def aoutConnectRequest : UInt32 := 0xc00c111c
def aoutDisconnectRequest : UInt32 := 0xc008111d
def aoutVolumeRequest : UInt32 := 0xc010110a
def aoutSetVolumeRequest : UInt32 := 0x40101109

@[extern "tv_audio_recv_command"] opaque recvCommandWord : IO UInt64
/-- `len` bytes, or fewer when the socket closed, timed out or failed. -/
@[extern "tv_audio_recv_exact"] opaque recvExactRaw (len : UInt32) : IO ByteArray
@[extern "tv_audio_send"] opaque sendRaw (bytes : @& ByteArray) : IO UInt64
@[extern "tv_audio_gain"] opaque gainWord : IO UInt64
@[extern "tv_audio_now"] opaque nowWord : IO UInt64
@[extern "tv_audio_sleep"] opaque sleepMs (ms : UInt32) : IO Unit
@[extern "tv_audio_log"] opaque logRaw (message : @& String) : IO Unit
@[extern "tv_audio_acquire"] opaque acquireWord : IO UInt64
@[extern "tv_audio_release"] opaque releaseLease : IO Unit
@[extern "tv_audio_poison"] opaque poisonLease : IO Unit
@[extern "tv_audio_pcm_open"] opaque pcmOpenWord : IO UInt64
@[extern "tv_audio_pcm_start"] opaque pcmStartWord (handle : UInt32) : IO UInt64
@[extern "tv_audio_pcm_write"] opaque pcmWriteWord (handle : UInt32) (bytes : @& ByteArray) (offset len : UInt32) : IO UInt64
@[extern "tv_audio_pcm_queued"] opaque pcmQueuedWord (handle : UInt32) : IO UInt64
@[extern "tv_audio_pcm_stop"] opaque pcmStopWord (handle : UInt32) : IO UInt64
@[extern "tv_audio_pcm_close"] opaque pcmCloseWord (handle : UInt32) : IO UInt64
@[extern "tv_audio_pcm_retain"] opaque pcmRetain (handle : UInt32) : IO Unit
@[extern "tv_audio_aout_speaker"] opaque aoutSpeakerWord : IO UInt64
@[extern "tv_audio_aout_count"] opaque aoutCountWord (output : UInt32) : IO UInt64
/-- An 8-byte result word, then `count` little-endian handles. -/
@[extern "tv_audio_aout_connected"] opaque aoutConnectedRaw (output count : UInt32) : IO ByteArray
@[extern "tv_audio_aout_connect"] opaque aoutConnectWord (output input : UInt32) : IO UInt64
@[extern "tv_audio_aout_disconnect"] opaque aoutDisconnectWord (output input : UInt32) : IO UInt64
/-- An 8-byte result word, then the integer and thousandths as little-endian 32-bit values. -/
@[extern "tv_audio_aout_volume"] opaque aoutVolumeRaw (output : UInt32) : IO ByteArray
@[extern "tv_audio_aout_set_volume"] opaque aoutSetVolumeWord (output integer thousandths : UInt32) : IO UInt64

def unit (request : UInt32) (word : UInt64) : Except Fault Unit :=
  match outcome request word with
  | .ok _ => .ok ()
  | .error f => .error f

def effects : Effects IO where
  recvCommand := do
    let word ← recvCommandWord
    return if word &&& 0xff == 0 then some (word >>> 32).toUInt8 else none
  recvExact := fun len => do
    let bytes ← recvExactRaw len
    return if bytes.size == len.toNat then some bytes else none
  send := fun bytes => return (← sendRaw bytes) == 0
  gain := return (← gainWord).toUInt32
  now := nowWord
  sleep := sleepMs
  log := logRaw
  acquire := return (← acquireWord).toUInt8
  release := releaseLease
  poison := poisonLease
  pcmOpen := return outcome pcmOpenRequest (← pcmOpenWord)
  pcmStart := fun h => return unit pcmStartRequest (← pcmStartWord h)
  pcmWrite := fun h b off len => return outcome pcmWriteRequest (← pcmWriteWord h b off len)
  pcmQueued := fun h => return outcome pcmAttrRequest (← pcmQueuedWord h)
  pcmStop := fun h => return unit pcmStopRequest (← pcmStopWord h)
  pcmClose := fun h => return unit pcmCloseRequest (← pcmCloseWord h)
  pcmRetain
  aoutSpeaker := return outcome aoutHandleRequest (← aoutSpeakerWord)
  aoutCount := fun o => return outcome aoutCountRequest (← aoutCountWord o)
  aoutConnected := fun o n => do
    let raw ← aoutConnectedRaw o n
    if raw.size != 8 + 4 * n.toNat then return .error (.invalid "AOUT connection list is short")
    match outcome aoutConnectedRequest (Protocol.readU64 raw 0) with
    | .error f => return .error f
    | .ok _ => return .ok ((Array.range n.toNat).map fun i => Protocol.readU32 raw (8 + 4 * i))
  aoutConnect := fun o i => return unit aoutConnectRequest (← aoutConnectWord o i)
  aoutDisconnect := fun o i => return unit aoutDisconnectRequest (← aoutDisconnectWord o i)
  aoutVolume := fun o => do
    let raw ← aoutVolumeRaw o
    if raw.size != 16 then return .error (.invalid "AOUT volume reply is short")
    match outcome aoutVolumeRequest (Protocol.readU64 raw 0) with
    | .error f => return .error f
    | .ok _ => return .ok ⟨(Protocol.readU32 raw 8).toInt32, (Protocol.readU32 raw 12).toInt32⟩
  aoutSetVolume := fun o v => return unit aoutSetVolumeRequest (← aoutSetVolumeWord o v.integer.toUInt32 v.thousandths.toUInt32)

/-- Construct the real IO success objects at the Rust boundary, as `TV.Pid1` does. -/
@[export tv_audio_io_u64] def ioU64 (value : UInt64) : IO UInt64 := pure value
@[export tv_audio_io_unit] def ioUnit (_ : UInt32) : IO Unit := pure ()

/-- One connection, on its own thread. 0 when the app closed the socket
normally, 1 when the session ended with a fault (already logged). -/
@[export tv_audio_client]
def client : IO UInt64 := do
  match ← Session.client effects with
  | .ok () => return 0
  | .error fault =>
    logRaw ("LEAN_AUDIO client: " ++ fault.describe)
    return 1

end TV.Audio.Native
