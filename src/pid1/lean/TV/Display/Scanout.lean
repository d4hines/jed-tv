prelude
import Init.System.IO
set_option compiler.extract_closed false

/-! OSD transaction policy, independent of native handles and file descriptors.
The native caller holds the display ownership lock and executes each requested
ioctl synchronously. These scalar effects use no allocator or shared runtime.
State bit 0 is the next back buffer; bit 1 records that the layer was shown.
The returned low byte is the new state, and upper bits report failed effects. -/
namespace TV.Display.Scanout

@[extern "tv_display_osd_call"]
private opaque call (context : USize) (request first second : UInt32) (count : UInt8) : BaseIO UInt8

private def one (ctx : USize) (request value : UInt32) : BaseIO UInt8 :=
  call ctx request value 0 1
private def two (ctx : USize) (request first second : UInt32) : BaseIO UInt8 :=
  call ctx request first second 2

private def failed (state flags : UInt32) : UInt32 := state ||| (flags <<< 8)

@[export tv_scanout_present]
def present (ctx : USize) (layer window surface state : UInt32) : BaseIO UInt32 := do
  if state >= 4 then return failed state 256
  if (← one ctx 0xc0041416 layer) != 0 then return failed state 1
  let flip ← two ctx 0xc008142d window surface
  -- Once Begin succeeds, Apply must run even if the flip fails.
  let apply ← one ctx 0xc0041417 layer
  let failures := (if flip == 0 then 0 else 2) ||| (if apply == 0 then 0 else 4)
  if failures != 0 then return failed state failures
  if state &&& 2 == 0 then
    if (← one ctx 0xc0041405 layer) != 0 then return failed state 8
  return (state ^^^ 1) ||| 2

@[export tv_scanout_hide]
def hide (ctx : USize) (layer state : UInt32) : BaseIO UInt32 := do
  if state >= 4 then return failed state 256
  if state &&& 2 != 0 then
    if (← one ctx 0xc0041406 layer) != 0 then return failed state 16
  return state &&& 1

@[export tv_scanout_configure_alpha]
def configureAlpha (ctx : USize) (layer window : UInt32) : BaseIO UInt32 := do
  -- Match the three required fbdev controls: no transparent color key,
  -- per-pixel alpha with global value 255, no layer global-alpha override.
  if (← one ctx 0xc004140c layer) != 0 then return 1
  if (← two ctx 0xc0081425 window 0x0000ff01) != 0 then return 2
  if (← one ctx 0xc0041415 layer) != 0 then return 3
  return 0

end TV.Display.Scanout
