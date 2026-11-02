/-!
# Doom.TV.Platform

The TV app ABI (`tv_app_v1.h`) as seen from Lean, through `native/tv_ffi.c`.
Status results are 0 or a negative errno as `UInt32`. There is exactly one
display, input and audio handle per process, owned by the game loop.
-/

namespace Doom.TV.Platform

@[extern "dtv_display_open"] opaque displayOpen : IO UInt32
/-- Convert an indexed 320x200 frame through a 768-byte RGB palette, scale it
to 4:3 on the TV's frame, and publish it. -/
@[extern "dtv_display_present"] opaque displayPresent (fb : @& ByteArray) (pal : @& ByteArray) : IO UInt32

/-- Benchmark: convert a frame like `displayPresent` into a private buffer. -/
@[extern "dtv_bench_blit"] opaque benchBlit (fb : @& ByteArray) (pal : @& ByteArray) : IO UInt32

@[extern "dtv_input_open"] opaque inputOpen : IO UInt32
/-- 0 when idle, else `0x80000000 ||| value <<< 16 ||| linuxKeycode`. -/
@[extern "dtv_input_poll"] opaque inputPoll : IO UInt32

@[extern "dtv_audio_open"] opaque audioOpen (rate channels : UInt32) : IO UInt32
/-- Interleaved signed 16-bit little-endian stereo. -/
@[extern "dtv_audio_write"] opaque audioWrite (pcm : @& ByteArray) : IO UInt32
/-- Frames still queued for playback; `0xffffffff` when unknown. -/
@[extern "dtv_audio_queued"] opaque audioQueued : IO UInt32

@[extern "dtv_udp_open"] opaque udpOpen (port : UInt16) : IO UInt32
/-- Send to every interface's broadcast address and 255.255.255.255; returns
the number of successful sends, or a negative errno. -/
@[extern "dtv_udp_broadcast"] opaque udpBroadcast (port : UInt16) (bytes : @& ByteArray) : IO UInt32
@[extern "dtv_udp_send"] opaque udpSend (ip : UInt32) (port : UInt16) (bytes : @& ByteArray) : IO UInt32
/-- Empty when idle, else 4 bytes IPv4 + 2 bytes port (big-endian) + payload. -/
@[extern "dtv_udp_recv"] opaque udpRecv : IO ByteArray

/-- Error text for a negative-errno status. -/
def statusText (s : UInt32) : String :=
  if s == 0 then "ok" else s!"errno {(0 : UInt32) - s}"

end Doom.TV.Platform
