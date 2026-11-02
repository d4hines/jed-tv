/-! The platform for Lean apps: Linux system calls and the app contract
(`tv_app_v1`), as typed wrappers over the runtime's ZrTv functions
(`src/lean/zr/ZrTv`). These link only on the TV side (32-bit ARM, our
runtime); host tests should not call them.

Raw results follow the kernel's convention: an error is a value above
`SIZE_MAX - 4096`, i.e. `-errno`. -/
namespace Yt.Platform

@[inline] def isErr (r : USize) : Bool := r > (0 : USize) - 4096
@[inline] def errnoOf (r : USize) : Nat := ((0 : USize) - r).toNat

/-- A raw result as `Except errno value`. -/
@[inline] def check (r : USize) : Except Nat USize :=
  if isErr r then .error (errnoOf r) else .ok r

namespace Sys

@[extern "zr_tv_sys"] opaque sys (nr a0 a1 a2 a3 : USize) : BaseIO USize
@[extern "zr_tv_read"] opaque read (fd n : USize) : BaseIO (Except Nat ByteArray)
@[extern "zr_tv_write"] opaque writeRaw (fd : USize) (b : @& ByteArray) (off len : USize) : BaseIO USize
@[extern "zr_tv_connect"] opaque connectRaw (fd : USize) (sa : @& ByteArray) : BaseIO USize
@[extern "zr_tv_setsockopt_int"] opaque setsockoptInt (fd level name value : USize) : BaseIO USize
@[extern "zr_tv_getsockopt_int"] opaque getsockoptInt (fd level name : USize) : BaseIO USize
/-- poll(2): `fds` holds `struct pollfd` records (fd: 4 bytes LE, events
and revents: 2 bytes each); returned with revents filled in. -/
@[extern "zr_tv_poll"] opaque poll (fds : ByteArray) (timeoutMs : USize) : BaseIO (Except Nat ByteArray)
@[extern "zr_tv_getrandom"] opaque getrandom (n : USize) : BaseIO (Except Nat ByteArray)
/-- Clock 0 is realtime (UNIX time), 1 monotonic; nanoseconds. -/
@[extern "zr_tv_clock_ns"] opaque clockNs (clock : USize) : BaseIO UInt64

def write (fd : USize) (b : ByteArray) (off len : USize) : BaseIO (Except Nat USize) := do
  pure (check (← writeRaw fd b off len))

/-- Write all of `b` (retrying short writes). -/
partial def writeAll (fd : USize) (b : ByteArray) (off : USize := 0) : BaseIO (Except Nat Unit) := do
  if off ≥ b.size.toUSize then return .ok ()
  match ← write fd b off (b.size.toUSize - off) with
  | .ok n => writeAll fd b (off + n)
  | .error e => return .error e

/-- A line on standard output (the app's log). -/
def log (s : String) : BaseIO Unit := do
  let _ ← writeAll 1 (s ++ "\n").toUTF8

def monotonicMs : BaseIO UInt64 := do pure ((← clockNs 1) / 1000000)

/-- A nonblocking TCP socket. -/
def tcpSocket : BaseIO (Except Nat USize) := do
  -- socket(AF_INET, SOCK_STREAM | SOCK_NONBLOCK, 0); 281 on ARM EABI
  pure (check (← sys 281 2 (1 ||| 0x800) 0 0))

/-- `struct sockaddr_in` for an IPv4 address (a.b.c.d as bytes) and port. -/
def sockaddrIn (a b c d : UInt8) (port : UInt16) : ByteArray :=
  ⟨#[2, 0, (port >>> 8).toUInt8, port.toUInt8, a, b, c, d, 0, 0, 0, 0, 0, 0, 0, 0]⟩

/-- Start connecting (EINPROGRESS, 115, is success for a nonblocking socket). -/
def connect (fd : USize) (sa : ByteArray) : BaseIO (Except Nat Unit) := do
  let r ← connectRaw fd sa
  pure (if isErr r && errnoOf r != 115 then .error (errnoOf r) else .ok ())

def close (fd : USize) : BaseIO Unit := do let _ ← sys 6 fd 0 0 0

@[extern "zr_tv_arg"] opaque argRaw (i : USize) : BaseIO String
@[extern "zr_tv_env"] opaque envRaw (i : USize) : BaseIO String

/-- The program's arguments (argv, including argv[0]). -/
partial def args (i : Nat := 0) (acc : Array String := #[]) : BaseIO (Array String) := do
  let a ← argRaw i.toUSize
  if a.toUTF8.size == 0 then return acc else args (i + 1) (acc.push a)

/-- The value of `KEY=value` in the environment. -/
partial def getenv (key : String) (i : Nat := 0) : BaseIO (Option String) := do
  let e := (← envRaw i.toUSize).toUTF8
  if e.size == 0 then return none
  let p := (key ++ "=").toUTF8
  if e.size ≥ p.size && e.extract 0 p.size == p then
    return String.fromUTF8? (e.extract p.size e.size)
  getenv key (i + 1)

end Sys

namespace Tv

/-- An open display. -/
structure Display where
  h : USize

@[extern "zr_tv_display_open_shared"] opaque displayOpenSharedRaw (path : @& String) : BaseIO USize
@[extern "zr_tv_display_open_fb"] opaque displayOpenFbRaw (path : @& String) : BaseIO USize
@[extern "zr_tv_display_acquire"] opaque acquireRaw (h : USize) : BaseIO USize
@[extern "zr_tv_display_present"] opaque presentRaw (h : USize) : BaseIO USize
@[extern "zr_tv_display_cancel"] opaque cancelRaw (h : USize) : BaseIO UInt8
@[extern "zr_tv_display_close"] opaque closeRaw (h : USize) : BaseIO UInt8
@[extern "zr_tv_frame_width"] opaque widthRaw (h : USize) : BaseIO UInt32
@[extern "zr_tv_frame_height"] opaque heightRaw (h : USize) : BaseIO UInt32
@[extern "zr_tv_frame_pixels"] opaque pixelsRaw (h : USize) : BaseIO USize
@[extern "zr_tv_frame_stride"] opaque strideRaw (h : USize) : BaseIO UInt32
@[extern "zr_tv_fill"] opaque fillRaw (h x y w hh : USize) (rgb : UInt32) : BaseIO UInt8
@[extern "zr_tv_blit_rgb"] opaque blitRgbRaw (h x y : USize) (src : @& ByteArray) (off sw sh sstride : USize) : BaseIO UInt8
@[extern "zr_tv_blit_mask"] opaque blitMaskRaw (h x y : USize) (mask : @& ByteArray) (off mw mh mstride : USize) (rgb : UInt32) : BaseIO UInt8

/-- The display: the shared-frame file named by TV_FB_PATH (the TV's
bridge), else /dev/fb0. -/
def Display.open : BaseIO (Except Nat Display) := do
  let r ← match ← Sys.getenv "TV_FB_PATH" with
    | some path => displayOpenSharedRaw path
    | none => displayOpenFbRaw "/dev/fb0"
  pure ((check r).map (⟨·⟩))
/-- Acquire a frame to draw into (all drawing happens between `acquire` and
`present`/`cancel`). -/
def Display.acquire (d : Display) : BaseIO (Except Nat Unit) := do
  pure ((check (← acquireRaw d.h)).map (fun _ => ()))
def Display.present (d : Display) : BaseIO (Except Nat Unit) := do
  pure ((check (← presentRaw d.h)).map (fun _ => ()))
def Display.cancel (d : Display) : BaseIO Unit := do let _ ← cancelRaw d.h
def Display.close (d : Display) : BaseIO Unit := do let _ ← closeRaw d.h
def Display.width (d : Display) : BaseIO UInt32 := widthRaw d.h
def Display.height (d : Display) : BaseIO UInt32 := heightRaw d.h
def Display.fill (d : Display) (x y w h : Nat) (rgb : UInt32) : BaseIO Unit := do
  let _ ← fillRaw d.h x.toUSize y.toUSize w.toUSize h.toUSize rgb
/-- Draw an RGB888 image: `w`×`h` pixels from byte `off`, rows `stride` bytes apart. -/
def Display.blitRgb (d : Display) (x y : Nat) (src : ByteArray) (off w h stride : Nat) : BaseIO Unit := do
  let _ ← blitRgbRaw d.h x.toUSize y.toUSize src off.toUSize w.toUSize h.toUSize stride.toUSize
/-- Paint `rgb` through an 8-bit coverage mask (text). -/
def Display.blitMask (d : Display) (x y : Nat) (mask : ByteArray) (off w h stride : Nat) (rgb : UInt32) : BaseIO Unit := do
  let _ ← blitMaskRaw d.h x.toUSize y.toUSize mask off.toUSize w.toUSize h.toUSize stride.toUSize rgb

/-- PCM output. -/
structure Audio where
  h : USize

@[extern "zr_tv_audio_open_alsa"] opaque audioOpenAlsaRaw (path : @& String) (rate channels : USize) : BaseIO USize
@[extern "zr_tv_audio_open_tcp"] opaque audioOpenTcpRaw (ip port rate channels boost : USize) : BaseIO USize
@[extern "zr_tv_audio_write"] opaque audioWriteRaw (a : USize) (pcm : @& ByteArray) (off count : USize) : BaseIO USize
@[extern "zr_tv_audio_queued"] opaque audioQueuedRaw (a : USize) : BaseIO USize
@[extern "zr_tv_audio_reset"] opaque audioResetRaw (a : USize) : BaseIO USize
@[extern "zr_tv_audio_drain"] opaque audioDrainRaw (a : USize) : BaseIO USize
@[extern "zr_tv_audio_close"] opaque audioCloseRaw (a : USize) : BaseIO UInt8

/-- A decimal natural in a string (none if not all digits). -/
def parseNat (s : String) : Option Nat := Id.run do
  let b := s.toUTF8
  if b.size == 0 then return none
  let mut n := 0
  for c in b do
    if c < 48 || c > 57 then return none
    n := n * 10 + (c - 48).toNat
  return some n

/-- `a.b.c.d:port`, as (address a << 24 ..., port). -/
def parseAddr (s : String) : Option (Nat × Nat) := Id.run do
  let b := s.toUTF8
  let mut parts : Array Nat := #[]
  let mut cur := 0
  let mut digits := 0
  for c in b do
    if c == 46 || c == 58 then
      if digits == 0 then return none
      parts := parts.push cur
      cur := 0
      digits := 0
    else if c ≥ 48 && c ≤ 57 then
      cur := cur * 10 + (c - 48).toNat
      digits := digits + 1
    else return none
  if digits == 0 then return none
  parts := parts.push cur
  if parts.size != 5 || parts[0]! > 255 || parts[1]! > 255 || parts[2]! > 255 || parts[3]! > 255 then return none
  return some ((parts[0]! <<< 24) ||| (parts[1]! <<< 16) ||| (parts[2]! <<< 8) ||| parts[3]!, parts[4]!)

/-- PCM output: the bridge's PCM service at TV_AUDIO_ADDR (ip:port; with
TV_AUDIO_BOOST 1..4), else ALSA's first playback device. -/
def Audio.open (rate channels : Nat) : BaseIO (Except Nat Audio) := do
  let r ← match (← Sys.getenv "TV_AUDIO_ADDR").bind parseAddr with
    | some (ip, port) =>
      let boost := (((← Sys.getenv "TV_AUDIO_BOOST").bind parseNat).getD 1).max 1 |>.min 4
      audioOpenTcpRaw ip.toUSize port.toUSize rate.toUSize channels.toUSize boost.toUSize
    | none => audioOpenAlsaRaw "/dev/snd/pcmC0D0p" rate.toUSize channels.toUSize
  pure ((check r).map (⟨·⟩))
/-- Write `count` interleaved 16-bit little-endian samples from byte `off`. -/
def Audio.write (a : Audio) (pcm : ByteArray) (off count : Nat) : BaseIO (Except Nat Unit) := do
  pure ((check (← audioWriteRaw a.h pcm off.toUSize count.toUSize)).map (fun _ => ()))
/-- Frames queued for playback: the playback clock. -/
def Audio.queued (a : Audio) : BaseIO (Except Nat Nat) := do
  pure ((check (← audioQueuedRaw a.h)).map (·.toNat))
def Audio.reset (a : Audio) : BaseIO Unit := do let _ ← audioResetRaw a.h
def Audio.drain (a : Audio) : BaseIO Unit := do let _ ← audioDrainRaw a.h
def Audio.close (a : Audio) : BaseIO Unit := do let _ ← audioCloseRaw a.h

/-- Remote input. -/
structure Input where
  h : USize

/-- A key event: Linux key code; value 0 release, 1 press, 2 repeat. -/
structure Key where
  code : UInt32
  value : UInt32

@[extern "zr_tv_input_open"] opaque inputOpenRaw (fd : USize) : BaseIO USize
@[extern "zr_tv_input_poll"] opaque inputPollRaw (i : USize) : BaseIO UInt64
@[extern "zr_tv_input_close"] opaque inputCloseRaw (i : USize) : BaseIO UInt8

/-- Remote input: the launch-bound socket PID 1 passes (TVAPP_REMOTE=N in
the environment, or `tvapp.remote=N` in argv). Apps never open input
devices themselves. -/
def Input.open : BaseIO (Except Nat Input) := do
  let fromArgs := (← Sys.args).foldl (fun acc a =>
    let b := a.toUTF8
    let p := "tvapp.remote=".toUTF8
    if acc.isNone && b.size > p.size && b.extract 0 p.size == p then
      (String.fromUTF8? (b.extract p.size b.size)).bind parseNat else acc) none
  match (← Sys.getenv "TVAPP_REMOTE").bind parseNat <|> fromArgs with
  | some fd => pure (.ok ⟨← inputOpenRaw fd.toUSize⟩)
  | none => pure (.error 19)
/-- The next event, if any (nonblocking). -/
def Input.poll (i : Input) : BaseIO (Except Nat (Option Key)) := do
  let v ← inputPollRaw i.h
  if v == 0 then return .ok none
  if v >>> 63 == 1 then return .ok (some ⟨v.toUInt32, ((v >>> 32) &&& 0xff).toUInt32⟩)
  return .error (v &&& 0xffffffff).toNat
def Input.close (i : Input) : BaseIO Unit := do let _ ← inputCloseRaw i.h

@[extern "zr_tv_video_blit_yuv"]
opaque blitYuvRaw (disp : USize) (y cb cr : @& ByteArray) (sw sh dx dy w hh : USize) : BaseIO USize

/-- Draw a 4:2:0 picture (packed planes: Y `sw`×`sh`, Cb and Cr half that in
each direction, as Yt.H264 outputs) scaled into the `w`×`h` rectangle at
(x, y) of the display's leased frame. -/
def Display.blitYuv (d : Display) (y cb cr : ByteArray) (sw sh x yy w h : Nat) : BaseIO Unit := do
  let _ ← blitYuvRaw d.h y cb cr sw.toUSize sh.toUSize x.toUSize yy.toUSize w.toUSize h.toUSize

end Tv
end Yt.Platform
