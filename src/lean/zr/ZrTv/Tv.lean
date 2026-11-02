/-
The app contract (`tv_app_v1.h`, research/APP_API_V1.md), its client side
written here in Lean (ZrLow style), so Lean apps link no native platform
library. It speaks the same backend protocols as src/platform:

- display: the shared-frame file (TV_FB_PATH: a 4096-byte header, magic
  "TVF2", width, height, stride, published page, sequence; two RGBA pages;
  Linux byte-range locks as in research/APP_API_V1.md), or fbdev
  (/dev/fb0, page flipping);
- audio: ALSA PCM through its ioctls (interleaved S16_LE), or the bridge's
  TCP PCM protocol (TV_AUDIO_ADDR: O/W/D/R/F requests, 4-byte status);
- input: the launch-bound Unix datagram socket from PID 1 (16-byte events).
Choosing a backend from the environment is the typed layer's job
(src/lean/yt/Yt/Platform.lean).

Handles are blocks of this runtime's heap (byte arrays the app never sees as
objects), kept by the app as `USize`. Results are "raw" (ZrTv.Linux): an
error is a value above SIZE_MAX - 4096, i.e. -errno. The leased frame's pixel
pointer never leaves this module (native video conversion gets it only
through `zr_tv_frame_pixels`, during a lease).

A display block: word 0 the backend (1 shared file, 2 fbdev), word 1 whether
a lease is open; from byte 16 the frame (pixels pointer, then width, height,
stride in bytes, red/green/blue shifts and the opaque alpha bits, uint32
each); from byte 64 words: fd, mapping, mapping length, back page, pages,
held page (SIZE_MAX: none), page bytes.
-/
import ZrTv.Linux

set_option compiler.extract_closed false

namespace ZrTv.Tv
open ZrLean.Low ZrLean.Low.Obj ZrLean.Low.Rc ZrTv.Linux

/-- A scratch cell of `n` bytes. -/
@[inline] def scratch (n : USize) : BaseIO USize := allocSArray 1 n n

@[inline] def errno (e : USize) : USize := 0 - e
/-- A C `int32_t` result as a raw word (sign-extended). -/
@[inline] def ofI32 (rc : UInt32) : USize :=
  if rc ≥ 0x80000000 then 0 - (0 - rc).toUSize else rc.toUSize
@[inline] def ioctl (fd req arg : USize) : BaseIO USize := sys6 54 fd req arg 0 0 0
@[inline] def closeFd (fd : USize) : BaseIO Unit := do let _ ← sys6 6 fd 0 0 0 0 0
@[inline] def mmapShared (len fd : USize) : BaseIO USize := sys6 192 0 len 3 1 fd 0

def sleepMs (ms : USize) : BaseIO Unit := do
  let ts ← scratch 8
  st32 (ts + sarrData 0) 0
  st32 (ts + sarrData 0 + 4) (ms * 1000000).toUInt32
  let _ ← sys6 162 (ts + sarrData 0) 0 0 0 0 0
  dec ts

/-! ### Display -/

@[inline] def st (h : USize) : USize := h + sarrData 0
@[inline] def frame (h : USize) : USize := st h + 16
@[inline] def pixels (h : USize) : BaseIO USize := ldp (frame h)
@[inline] def field (h i : USize) : BaseIO UInt32 := ld32 (frame h + wordBytes 0 + 4 * i)
@[inline] def setField (h i : USize) (v : UInt32) : BaseIO Unit := st32 (frame h + wordBytes 0 + 4 * i) v
/-- Word `i` of the backend state (fd 0, mapping 1, length 2, back 3, pages 4, held 5, page bytes 6). -/
@[inline] def wordAt (h i : USize) : USize := st h + 64 + wordBytes 0 * i
@[inline] def leased (h : USize) : BaseIO Bool := do pure ((← ldp (st h + wordBytes 0)) != 0)

partial def lockLoop (fd fl : USize) (deadline : UInt64) : BaseIO USize := do
  let r ← sys6 221 fd 6 fl 0 0 0
  if !isErr r then return 0
  let e := errno r
  if (e == 11 || e == 13 || e == 4) && (← clockNs 1) < deadline then
    sleepMs 1
    lockLoop fd fl deadline
  else return r

/-- fcntl64(F_SETLK) of `[start, start + len)`, retrying for up to 2 s while
the other side holds it; `kind` 0 shared, 1 exclusive, 2 unlock. Uses the
kernel's 32-bit `struct flock` (type, whence, start, len, pid). -/
def regionLock (fd start len kind : USize) : BaseIO USize := do
  let fl ← scratch 16
  let p := fl + sarrData 0
  st16 p kind.toUInt16
  st16 (p + 2) 0
  st32 (p + 4) start.toUInt32
  st32 (p + 8) len.toUInt32
  st32 (p + 12) 0
  let r ← lockLoop fd p ((← clockNs 1) + 2000000000)
  dec fl
  pure r

def newDisplay (kind fd mem len back pages pageBytes w hgt stride : USize) (rs gs bs alpha : UInt32) :
    BaseIO USize := do
  let h ← allocSArray 1 128 128
  stp (st h) kind
  stp (st h + wordBytes 0) 0
  stp (frame h) 0
  setField h 0 w.toUInt32
  setField h 1 hgt.toUInt32
  setField h 2 stride.toUInt32
  setField h 3 rs
  setField h 4 gs
  setField h 5 bs
  setField h 6 alpha
  stp (wordAt h 0) fd
  stp (wordAt h 1) mem
  stp (wordAt h 2) len
  stp (wordAt h 3) back
  stp (wordAt h 4) pages
  stp (wordAt h 5) (sizeMax 0)
  stp (wordAt h 6) pageBytes
  pure h

/-- Open the shared-frame display file at `path` (a Lean string): a handle,
or a raw error. 1280x720 RGBA, two pages after a 4096-byte header. -/
@[export zr_tv_display_open_shared]
def openShared (path : USize) : BaseIO USize := do
  let w : USize := 1280
  let hgt : USize := 720
  let pageBytes := w * hgt * 4
  let total := 4096 + 2 * pageBytes
  let fd ← openPath path (2 ||| 0x40 ||| 0x80000) 438
  if isErr fd then return fd
  let r ← sys6 93 fd total 0 0 0 0
  if isErr r then closeFd fd; return r
  let mem ← mmapShared total fd
  if isErr mem then closeFd fd; return mem
  let ok := (← ld32 mem) == 0x32465654 && (← ld32 (mem + 4)) == w.toUInt32 &&
    (← ld32 (mem + 8)) == hgt.toUInt32 && (← ld32 (mem + 12)) == (w * 4).toUInt32 && (← ld32 (mem + 16)) < 2
  if !ok then
    st32 mem 0x32465654
    st32 (mem + 4) w.toUInt32
    st32 (mem + 8) hgt.toUInt32
    st32 (mem + 12) (w * 4).toUInt32
    st32 (mem + 16) 0
    st32 (mem + 20) 0
  let back := ((← ld32 (mem + 16)).toUSize + 1) % 2
  newDisplay 1 fd mem total back 2 pageBytes w hgt (w * 4) 0 8 16 0xFF000000

/-- Open an fbdev display (`path`, a Lean string, e.g. /dev/fb0): a handle,
or a raw error. 32-bit pixels; page flipping when the driver has room. -/
@[export zr_tv_display_open_fb]
def openFb (path : USize) : BaseIO USize := do
  let fd ← openPath path (2 ||| 0x80000) 0
  if isErr fd then return fd
  let var ← scratch 160
  let fix ← scratch 68
  let v := var + sarrData 0
  let f := fix + sarrData 0
  let r1 ← ioctl fd 0x4600 v
  let r2 ← ioctl fd 0x4602 f
  let w := (← ld32 v).toUSize
  let hgt := (← ld32 (v + 4)).toUSize
  let yvirt := (← ld32 (v + 12)).toUSize
  let bpp ← ld32 (v + 24)
  let rs ← ld32 (v + 32)
  let gs ← ld32 (v + 44)
  let bs ← ld32 (v + 56)
  let toff ← ld32 (v + 68)
  let tlen ← ld32 (v + 72)
  let smem := (← ld32 (f + 20)).toUSize
  let line := (← ld32 (f + 44)).toUSize
  dec var
  dec fix
  if isErr r1 then closeFd fd; return r1
  if isErr r2 then closeFd fd; return r2
  -- As src/platform's mapping_pages: 32 bpp; rows inside the line length;
  -- one frame inside the memory (`hgt ≤ smem / line` is `line * hgt ≤ smem`
  -- without overflow); a virtual height of at least one frame.
  if bpp != 32 || w == 0 || hgt == 0 || line == 0 || line % 4 != 0 || w > line / 4 ||
      hgt > smem / line || yvirt < hgt || smem > sizeMax 0 / 2 then
    closeFd fd; return (0 - 22)
  let mem ← mmapShared smem fd
  if isErr mem then closeFd fd; return mem
  let pageBytes := line * hgt
  let byMem := smem / pageBytes
  let byVirt := yvirt / hgt
  let pages := max 1 (min byMem byVirt)
  let alpha : UInt32 := if tlen > 0 && tlen < 32 then ((1 <<< tlen) - 1) <<< toff else 0
  newDisplay 2 fd mem smem (if pages > 1 then 1 else 0) pages pageBytes w hgt line rs gs bs alpha

/-- Acquire an exclusive writable frame (on the shared file this may wait
for the consumer to release the page). -/
@[export zr_tv_display_acquire]
def acquire (h : USize) : BaseIO USize := do
  if ← leased h then return (0 - 16)
  let back ← ldp (wordAt h 3)
  let pageBytes ← ldp (wordAt h 6)
  let mem ← ldp (wordAt h 1)
  if (← ldp (st h)) == 1 then
    let r ← regionLock (← ldp (wordAt h 0)) (4096 + back * pageBytes) pageBytes 1
    if isErr r then return r
    stp (wordAt h 5) back
    stp (frame h) (mem + 4096 + back * pageBytes)
  else
    stp (frame h) (mem + back * pageBytes)
  stp (st h + wordBytes 0) 1
  pure 0

/-- Release the held page (shared file). -/
def releasePage (h : USize) : BaseIO Unit := do
  let held ← ldp (wordAt h 5)
  if held != sizeMax 0 then
    let _ ← regionLock (← ldp (wordAt h 0)) (4096 + held * (← ldp (wordAt h 6))) (← ldp (wordAt h 6)) 2
    stp (wordAt h 5) (sizeMax 0)

/-- Publish the leased frame and release it. -/
@[export zr_tv_display_present]
def present (h : USize) : BaseIO USize := do
  if !(← leased h) then return (0 - 22)
  stp (st h + wordBytes 0) 0
  let fd ← ldp (wordAt h 0)
  let mem ← ldp (wordAt h 1)
  let back ← ldp (wordAt h 3)
  let pages ← ldp (wordAt h 4)
  if (← ldp (st h)) == 1 then
    let r ← regionLock fd 0 4096 1
    if isErr r then releasePage h; return r
    st32 (mem + 16) back.toUInt32
    st32 (mem + 20) ((← ld32 (mem + 20)) + 1)
    let _ ← regionLock fd 0 4096 2
    releasePage h
    stp (wordAt h 3) ((back + 1) % 2)
    return 0
  if pages > 1 then
    let var ← scratch 160
    let v := var + sarrData 0
    let r ← ioctl fd 0x4600 v
    if !isErr r then
      st32 (v + 20) (back * (← field h 1).toUSize).toUInt32
      let _ ← ioctl fd 0x4606 v
    dec var
    stp (wordAt h 3) ((back + 1) % pages)
  pure 0

/-- Release the lease without publishing. -/
@[export zr_tv_display_cancel]
def cancel (h : USize) : BaseIO UInt8 := do
  stp (st h + wordBytes 0) 0
  releasePage h
  pure 0

/-- Close the display and free the handle. -/
@[export zr_tv_display_close]
def close (h : USize) : BaseIO UInt8 := do
  let _ ← cancel h
  let _ ← sys6 91 (← ldp (wordAt h 1)) (← ldp (wordAt h 2)) 0 0 0 0
  closeFd (← ldp (wordAt h 0))
  dec h
  pure 0

/-- Frame width and height (valid during a lease; 0 otherwise). -/
@[export zr_tv_frame_width]
def width (h : USize) : BaseIO UInt32 := do
  if ← leased h then field h 0 else pure 0
@[export zr_tv_frame_height]
def height (h : USize) : BaseIO UInt32 := do
  if ← leased h then field h 1 else pure 0

/-- The leased frame's pixel address and stride in bytes, for native video
conversion only; 0 without a lease. -/
@[export zr_tv_frame_pixels]
def framePixels (h : USize) : BaseIO USize := do
  if ← leased h then pixels h else pure 0
@[export zr_tv_frame_stride]
def frameStride (h : USize) : BaseIO UInt32 := do
  if ← leased h then field h 2 else pure 0

/-- The frame's pixel word for `rgb` = 0xRRGGBB. -/
def pixelOf (h : USize) (rgb : UInt32) : BaseIO UInt32 := do
  let r := (rgb >>> 16) &&& 0xff
  let g := (rgb >>> 8) &&& 0xff
  let b := rgb &&& 0xff
  pure ((← field h 6) ||| (r <<< (← field h 3)) ||| (g <<< (← field h 4)) ||| (b <<< (← field h 5)))

partial def fillRow (p v i n : USize) (px : UInt32) : BaseIO Unit := do
  if i < n then
    st32 (p + 4 * i) px
    fillRow p v (i + 1) n px

partial def copyRows (first stride rowBytes i n : USize) : BaseIO Unit := do
  if i < n then
    copy (first + i * stride) first rowBytes
    copyRows first stride rowBytes (i + 1) n

/-- Clip `[a, a + len)` to `[0, lim)`: the end (≥ a when visible). -/
@[inline] def clipEnd (a len lim : USize) : USize :=
  if a ≥ lim then a else if len > lim - a then lim else a + len

/-- Fill a rectangle (clipped to the frame) with `rgb`. -/
@[export zr_tv_fill]
def fill (h x y w hh : USize) (rgb : UInt32) : BaseIO UInt8 := do
  if ← leased h then
    let fw := (← field h 0).toUSize
    let fh := (← field h 1).toUSize
    let stride := (← field h 2).toUSize
    let x1 := clipEnd x w fw
    let y1 := clipEnd y hh fh
    if x < x1 && y < y1 then
      let first := (← pixels h) + y * stride + 4 * x
      fillRow first 0 0 (x1 - x) (← pixelOf h rgb)
      copyRows first stride (4 * (x1 - x)) 1 (y1 - y)
  pure 0

partial def rgbRow (h dst src i n : USize) : BaseIO Unit := do
  if i < n then
    let r ← ld8 (src + 3 * i)
    let g ← ld8 (src + 3 * i + 1)
    let b ← ld8 (src + 3 * i + 2)
    st32 (dst + 4 * i) (← pixelOf h ((r.toUInt32 <<< 16) ||| (g.toUInt32 <<< 8) ||| b.toUInt32))
    rgbRow h dst src (i + 1) n

partial def rgbRows (h dst stride src sstride rowPixels i n : USize) : BaseIO Unit := do
  if i < n then
    rgbRow h (dst + i * stride) (src + i * sstride) 0 rowPixels
    rgbRows h dst stride src sstride rowPixels (i + 1) n

/-- Copy an RGB888 image (bytes `off ..` of `src`, borrowed; rows of `sw`
pixels every `sstride` bytes, `sh` rows) to `(x, y)`, clipped. -/
@[export zr_tv_blit_rgb]
def blitRgb (h x y src off sw sh sstride : USize) : BaseIO UInt8 := do
  let size ← ldp (src + sarrSizeOff 0)
  let need := if sh == 0 then 0 else off + (sh - 1) * sstride + 3 * sw
  if (← leased h) && need ≤ size && off ≤ size then
    let fw := (← field h 0).toUSize
    let fh := (← field h 1).toUSize
    let stride := (← field h 2).toUSize
    let x1 := clipEnd x sw fw
    let y1 := clipEnd y sh fh
    if x < x1 && y < y1 then
      rgbRows h ((← pixels h) + y * stride + 4 * x) stride (src + sarrData 0 + off) sstride (x1 - x) 0 (y1 - y)
  pure 0

/-- `(src * a + dst * (255 - a)) / 255` for one channel. -/
@[inline] def mix (s d a : UInt32) : UInt32 := (s * a + d * (255 - a) + 127) / 255

def blend (h p : USize) (a : UInt32) (rgb : UInt32) : BaseIO Unit := do
  let d ← ld32 p
  let rs ← field h 3
  let gs ← field h 4
  let bs ← field h 5
  let r := mix ((rgb >>> 16) &&& 0xff) ((d >>> rs) &&& 0xff) a
  let g := mix ((rgb >>> 8) &&& 0xff) ((d >>> gs) &&& 0xff) a
  let b := mix (rgb &&& 0xff) ((d >>> bs) &&& 0xff) a
  st32 p ((← field h 6) ||| (r <<< rs) ||| (g <<< gs) ||| (b <<< bs))

partial def maskRow (h dst src i n : USize) (rgb px : UInt32) : BaseIO Unit := do
  if i < n then
    let a := (← ld8 (src + i)).toUInt32
    if a == 255 then st32 (dst + 4 * i) px
    else if a != 0 then blend h (dst + 4 * i) a rgb
    maskRow h dst src (i + 1) n rgb px

partial def maskRows (h dst stride src sstride rowPixels i n : USize) (rgb px : UInt32) : BaseIO Unit := do
  if i < n then
    maskRow h (dst + i * stride) (src + i * sstride) 0 rowPixels rgb px
    maskRows h dst stride src sstride rowPixels (i + 1) n rgb px

/-- Paint `rgb` through an 8-bit coverage mask (text, icons): bytes `off ..`
of `mask` (borrowed), `mw` per row every `mstride` bytes, `mh` rows. -/
@[export zr_tv_blit_mask]
def blitMask (h x y mask off mw mh mstride : USize) (rgb : UInt32) : BaseIO UInt8 := do
  let size ← ldp (mask + sarrSizeOff 0)
  let need := if mh == 0 then 0 else off + (mh - 1) * mstride + mw
  if (← leased h) && need ≤ size && off ≤ size then
    let fw := (← field h 0).toUSize
    let fh := (← field h 1).toUSize
    let stride := (← field h 2).toUSize
    let x1 := clipEnd x mw fw
    let y1 := clipEnd y mh fh
    if x < x1 && y < y1 then
      maskRows h ((← pixels h) + y * stride + 4 * x) stride (mask + sarrData 0 + off) mstride (x1 - x) 0 (y1 - y)
        rgb (← pixelOf h rgb)
  pure 0

/-! ### Audio

An audio block: word 0 the backend (1 ALSA, 2 TCP), then fd, rate,
channels, the device path (a Lean string the block owns, ALSA) and the boost
(TCP). -/

@[inline] def aw (a i : USize) : USize := a + sarrData 0 + wordBytes 0 * i

def newAudio (kind fd rate ch path boost : USize) : BaseIO USize := do
  let a ← allocSArray 1 64 64
  stp (aw a 0) kind
  stp (aw a 1) fd
  stp (aw a 2) rate
  stp (aw a 3) ch
  stp (aw a 4) path
  stp (aw a 5) boost
  pure a

/-- Set interval `i` (hw param index - 8) of a `struct snd_pcm_hw_params`. -/
def setRange (p i lo hi : USize) : BaseIO Unit := do
  st32 (p + 260 + 12 * i) lo.toUInt32
  st32 (p + 264 + 12 * i) hi.toUInt32
  st32 (p + 268 + 12 * i) 0

/-- Mask `m` to the single value `v`. -/
partial def setMask (p m v : USize) (k : USize := 0) : BaseIO Unit := do
  if k < 8 then
    st32 (p + 4 + 32 * m + 4 * k) (if k == v / 32 then ((1 : UInt32) <<< (v % 32).toUInt32) else 0)
    setMask p m v (k + 1)

/-- Every interval: any value (`[0, max]`). -/
partial def anyRanges (p i : USize) : BaseIO Unit := do
  if i < 12 then
    setRange p i 0 (sizeMax 0)
    anyRanges p (i + 1)

partial def fillWords (p : USize) (v : UInt32) (i n : USize) : BaseIO Unit := do
  if i < n then
    st32 (p + 4 * i) v
    fillWords p v (i + 1) n

/-- HW_PARAMS (interleaved S16_LE, `ch` channels at `rate`, 40-100 ms
periods, a 200-500 ms buffer, as src/platform/src/alsa.rs) and PREPARE. -/
def alsaConfigure (fd rate ch : USize) : BaseIO USize := do
  let hp ← scratch 604
  let p := hp + sarrData 0
  fillWords p 0 0 151
  fillWords (p + 4) 0xFFFFFFFF 0 24
  anyRanges p 0
  st32 (p + 512) 0xFFFFFFFF
  st32 (p + 520) 0xFFFFFFFF
  setMask p 0 3
  setMask p 1 2
  setMask p 2 0
  setRange p 2 ch ch
  setRange p 3 rate rate
  setRange p 5 (rate / 25) (rate / 10)
  setRange p 7 2 16
  setRange p 9 (rate / 5) (rate / 2)
  let r ← ioctl fd 0xC25C4111 p
  dec hp
  if isErr r then return r
  ioctl fd 0x4140 0

def alsaOpenFd (path rate ch : USize) : BaseIO USize := do
  let fd ← openPath path (2 ||| 0x80000) 0
  if isErr fd then return fd
  let r ← alsaConfigure fd rate ch
  if isErr r then closeFd fd; return r
  pure fd

/-- Open ALSA PCM playback at `path` (a Lean string, borrowed): a handle,
or a raw error. -/
@[export zr_tv_audio_open_alsa]
def audioOpenAlsa (path rate ch : USize) : BaseIO USize := do
  let fd ← alsaOpenFd path rate ch
  if isErr fd then return fd
  inc path
  newAudio 1 fd rate ch path 0

/-- Read exactly `n` bytes into `p` (blocking socket). -/
partial def readFull (fd p n : USize) : BaseIO USize := do
  if n == 0 then return 0
  let r ← sys6 3 fd p n 0 0 0
  if isErr r then return r
  if r == 0 then return (0 - 104)
  readFull fd (p + r) (n - r)

partial def writeFull (fd p n : USize) : BaseIO USize := do
  if n == 0 then return 0
  let r ← sys6 4 fd p n 0 0 0
  if isErr r then return r
  writeFull fd (p + r) (n - r)

/-- Send `n` bytes from `p`, then read the bridge's 4-byte status (0 = ok). -/
def tcpExchange (fd p n : USize) : BaseIO USize := do
  let w ← writeFull fd p n
  if isErr w then return w
  let s ← scratch 4
  let r ← readFull fd (s + sarrData 0) 4
  let v ← ld32 (s + sarrData 0)
  dec s
  if isErr r then return r
  pure (if v == 0 then 0 else (0 - 5))

/-- A request header: command byte and a little-endian u32. -/
def tcpHeader (cmd : UInt8) (len : UInt32) : BaseIO USize := do
  let b ← scratch 5
  st8 (b + sarrData 0) cmd
  st8 (b + sarrData 0 + 1) len.toUInt8
  st8 (b + sarrData 0 + 2) (len >>> 8).toUInt8
  st8 (b + sarrData 0 + 3) (len >>> 16).toUInt8
  st8 (b + sarrData 0 + 4) (len >>> 24).toUInt8
  pure b

/-- `W`: `bytes` of PCM from `p`. -/
def tcpWrite (fd p bytes : USize) : BaseIO USize := do
  let h ← tcpHeader 87 bytes.toUInt32
  let w ← writeFull fd (h + sarrData 0) 5
  dec h
  if isErr w then return w
  tcpExchange fd p bytes

/-- Prime the bridge with a quarter second of silence (its start threshold). -/
def tcpPrime (fd rate ch : USize) : BaseIO USize := do
  let n := rate * ch / 4 * 2
  let z ← allocSArray 1 n n
  fillWords (z + sarrData 0) 0 0 (n / 4)
  let r ← tcpWrite fd (z + sarrData 0) n
  dec z
  pure r

/-- Connect to the bridge's PCM service at `ip:port` (IPv4, host order
a.b.c.d = a << 24 ...) and open a stream: a handle, or a raw error. -/
@[export zr_tv_audio_open_tcp]
def audioOpenTcp (ip port rate ch boost : USize) : BaseIO USize := do
  let fd ← sys6 281 2 (1 ||| 0x80000) 0 0 0 0
  if isErr fd then return fd
  let sa ← scratch 16
  let s := sa + sarrData 0
  fillWords s 0 0 4
  st16 s 2
  st8 (s + 2) (port >>> 8).toUInt8
  st8 (s + 3) port.toUInt8
  st8 (s + 4) (ip >>> 24).toUInt8
  st8 (s + 5) (ip >>> 16).toUInt8
  st8 (s + 6) (ip >>> 8).toUInt8
  st8 (s + 7) ip.toUInt8
  let c ← sys6 283 fd s 16 0 0 0
  dec sa
  if isErr c then closeFd fd; return c
  let _ ← setsockoptInt fd 6 1 1
  let o ← scratch 13
  let p := o + sarrData 0
  st8 p 79
  st32 (p + 1) rate.toUInt32
  st32 (p + 5) ch.toUInt32
  st32 (p + 9) boost.toUInt32
  let r ← tcpExchange fd p 13
  dec o
  if isErr r then closeFd fd; return r
  let r ← tcpPrime fd rate ch
  if isErr r then closeFd fd; return r
  newAudio 2 fd rate ch 0 boost

partial def alsaWrite (fd p ch frames : USize) : BaseIO USize := do
  if frames == 0 then return 0
  let x ← scratch 12
  let xp := x + sarrData 0
  st32 xp 0
  stp (xp + 4) p
  st32 (xp + 8) frames.toUInt32
  let r ← ioctl fd 0x400C4150 xp
  let done := (← ld32 xp).toUSize
  dec x
  if isErr r then
    let e := errno r
    if e == 32 then
      let _ ← ioctl fd 0x4140 0
      alsaWrite fd p ch frames
    else if e == 4 then alsaWrite fd p ch frames
    else return r
  else alsaWrite fd (p + done * ch * 2) ch (frames - done)

/-- Write `count` interleaved 16-bit samples from byte `off` of `pcm`
(borrowed, little-endian). May block under backpressure. -/
@[export zr_tv_audio_write]
def audioWrite (a pcm off count : USize) : BaseIO USize := do
  let size ← ldp (pcm + sarrSizeOff 0)
  let ch ← ldp (aw a 3)
  if off &&& 1 != 0 || off > size || count > (size - off) / 2 || ch == 0 || count % ch != 0 then
    return (0 - 22)
  let p := pcm + sarrData 0 + off
  if (← ldp (aw a 0)) == 1 then alsaWrite (← ldp (aw a 1)) p ch (count / ch)
  else tcpWrite (← ldp (aw a 1)) p (count * 2)

/-- Frames queued for playback (the playback clock), or a raw error. -/
@[export zr_tv_audio_queued]
def audioQueued (a : USize) : BaseIO USize := do
  let fd ← ldp (aw a 1)
  let c ← scratch 16
  let p := c + sarrData 0
  if (← ldp (aw a 0)) == 1 then
    let r ← ioctl fd 0x80044121 p
    let d ← ld32 p
    dec c
    if isErr r then return r
    pure (if d ≥ 0x80000000 then 0 else d.toUSize)
  else
    -- `D`: frames sent and played so far (two u64; their low words suffice)
    st8 p 68
    let w ← writeFull fd p 1
    let r ← if isErr w then pure w else readFull fd p 16
    let sent ← ld32 p
    let played ← ld32 (p + 8)
    dec c
    if isErr r then return r
    pure (if sent ≥ played then (sent - played).toUSize else 0)

/-- Stop now and discard queued audio (ALSA: reopen; QEMU's virtio-snd stalls
after DROP+PREPARE on a running stream). -/
@[export zr_tv_audio_reset]
def audioReset (a : USize) : BaseIO USize := do
  let fd ← ldp (aw a 1)
  let rate ← ldp (aw a 2)
  let ch ← ldp (aw a 3)
  if (← ldp (aw a 0)) == 1 then
    closeFd fd
    let nfd ← alsaOpenFd (← ldp (aw a 4)) rate ch
    if isErr nfd then return nfd
    stp (aw a 1) nfd
    return 0
  let c ← scratch 1
  st8 (c + sarrData 0) 82
  let r ← tcpExchange fd (c + sarrData 0) 1
  dec c
  if isErr r then return r
  tcpPrime fd rate ch

@[export zr_tv_audio_drain]
def audioDrain (a : USize) : BaseIO USize := do
  let fd ← ldp (aw a 1)
  if (← ldp (aw a 0)) == 1 then ioctl fd 0x4144 0
  else
    let c ← scratch 1
    st8 (c + sarrData 0) 70
    let r ← tcpExchange fd (c + sarrData 0) 1
    dec c
    pure r

@[export zr_tv_audio_close]
def audioClose (a : USize) : BaseIO UInt8 := do
  closeFd (← ldp (aw a 1))
  if (← ldp (aw a 0)) == 1 then dec (← ldp (aw a 4))
  dec a
  pure 0

/-! ### Input

An input block: the socket fd, then a 16-byte event buffer. -/

/-- Read keys from PID 1's launch-bound socket `fd`: a handle. -/
@[export zr_tv_input_open]
def inputOpen (fd : USize) : BaseIO USize := do
  let i ← allocSArray 1 32 32
  stp (i + sarrData 0) fd
  pure i

/-- One key event, nonblocking: 0 none; bit 63 set: an event with the Linux
key code in bits 0..31 and the value (0 release, 1 press, 2 repeat) in bits
32..39; bit 62 set: an error with the errno in the low bits. Non-key events
are skipped. -/
@[export zr_tv_input_poll]
partial def inputPoll (i : USize) : BaseIO UInt64 := do
  let fd ← ldp (i + sarrData 0)
  let buf := i + sarrData 0 + 16
  -- poll(2) with no wait: a struct pollfd in the first 8 bytes of the buffer
  st32 buf fd.toUInt32
  st16 (buf + 4) 1
  st16 (buf + 6) 0
  let r ← sys6 168 buf 1 0 0 0 0
  if isErr r then return ((1 : UInt64) <<< 62) ||| (errno r).toUInt64
  if r == 0 || (← ld16 (buf + 6)) &&& 1 == 0 then return 0
  let n ← sys6 3 fd buf 16 0 0 0
  if isErr n then
    let e := errno n
    return if e == 11 then 0 else ((1 : UInt64) <<< 62) ||| e.toUInt64
  if n < 16 then return 0
  -- struct input_event (32-bit): time (8 bytes), type u16, code u16, value s32
  if (← ld16 (buf + 8)) != 1 then return ← inputPoll i
  let code ← ld16 (buf + 10)
  let value ← ld32 (buf + 12)
  return ((1 : UInt64) <<< 63) ||| code.toUInt64 ||| ((value &&& 0xff).toUInt64 <<< 32)

@[export zr_tv_input_close] def inputClose (i : USize) : BaseIO UInt8 := do dec i; pure 0

end ZrTv.Tv
