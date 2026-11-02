import Yt.Platform
import Yt.Ui.Text

/-! A platform smoke test: draws through the display lease (fill, an alpha
mask), reads remote keys and logs, for six seconds. -/
namespace Yt.App.Hello
open Yt.Platform Yt.Platform.Tv Yt.Ui

def colors : Array UInt32 := #[0x202840, 0x5a2a6e, 0x1f6e4a, 0x7a4b12]

/-- A 32x32 coverage mask: a disc with a half-covered rim. -/
def disc : ByteArray := Id.run do
  let mut b := ByteArray.emptyWithCapacity 1024
  for y in [0:32] do
    for x in [0:32] do
      let dx := if x ≥ 16 then x - 16 else 16 - x
      let dy := if y ≥ 16 then y - 16 else 16 - y
      let d2 := dx * dx + dy * dy
      b := b.push (if d2 < 196 then 255 else if d2 < 256 then 128 else 0)
  b

@[export yt_hello_main]
def main (_argc : UInt32) : BaseIO UInt32 := do
  Sys.log "hello-tv: Lean app on the Lean runtime"
  let d ← match ← Display.open with
    | .ok d => pure d
    | .error e => Sys.log ("hello-tv: display open failed, errno " ++ toString e); return 1
  let input ← Input.open
  match input with
  | .ok _ => Sys.log "hello-tv: input open"
  | .error e => Sys.log ("hello-tv: input open failed, errno " ++ toString e)
  -- two seconds of a 440 Hz square wave, through whichever audio backend
  match ← Audio.open 48000 2 with
  | .ok a =>
    let mut pcm := ByteArray.emptyWithCapacity (48000 * 4)
    for i in [0:48000] do
      let v : UInt16 := if (i * 440 / 24000) % 2 == 0 then 3000 else 0x10000 - 3000
      pcm := pcm.push v.toUInt8 |>.push (v >>> 8).toUInt8 |>.push v.toUInt8 |>.push (v >>> 8).toUInt8
    match ← a.write pcm 0 (pcm.size / 2) with
    | .ok () => Sys.log ("hello-tv: wrote " ++ toString (pcm.size / 4) ++ " audio frames")
    | .error e => Sys.log ("hello-tv: audio write failed, errno " ++ toString e)
    match ← a.write pcm 0 (pcm.size / 2) with
    | _ => pure ()
    match ← a.queued with
    | .ok q => Sys.log ("hello-tv: audio queued " ++ toString q ++ " frames")
    | .error e => Sys.log ("hello-tv: audio queued failed, errno " ++ toString e)
  | .error e => Sys.log ("hello-tv: audio open failed, errno " ++ toString e)
  let fonts := Fonts.load
  Sys.log ("hello-tv: fonts " ++ toString fonts.small.count ++ " glyphs, line heights " ++
    toString fonts.small.lineHeight ++ "/" ++ toString fonts.body.lineHeight ++ "/" ++ toString fonts.large.lineHeight)
  let t0 ← Sys.monotonicMs
  let mut frames := 0
  let mut color := 0
  let mut keys := 0
  let mut tAcq : UInt64 := 0
  let mut tDraw : UInt64 := 0
  let mut tPres : UInt64 := 0
  let mut tFill : UInt64 := 0
  while (← Sys.monotonicMs) - t0 < 6000 do
    let a0 ← Sys.clockNs 1
    match ← d.acquire with
    | .error e =>
      Sys.log ("hello-tv: acquire failed, errno " ++ toString e)
      break
    | .ok () =>
      let a1 ← Sys.clockNs 1
      let w := (← d.width).toNat
      let h := (← d.height).toNat
      d.fill 0 0 w h colors[color % 4]!
      let b1 ← Sys.clockNs 1
      tFill := tFill + (b1 - a1) / 1000
      let x := (frames * 8) % (w - 200)
      d.fill x 300 200 120 0xF0C020
      d.blitMask (x + 84) 344 disc 0 32 32 32 0x101010
      let _ ← fonts.large.draw d 80 60 "YouTube, in Lean".toUTF8 0xFFFFFF
      let title := "Café Müller — Pina Bausch’s “Tanztheater” and a title that is far too long to fit on one line".toUTF8
      let _ ← fonts.body.draw d 80 140 (fonts.body.ellipsize title 1000) 0xE0E0E0
      let _ ← fonts.small.draw d 80 190 ("frame " ++ toString frames ++ " · keys " ++ toString keys ++ " · ▶ ‖ ◀").toUTF8 0xA0A0B0
      let a2 ← Sys.clockNs 1
      let _ ← d.present
      let a3 ← Sys.clockNs 1
      tAcq := tAcq + (a1 - a0) / 1000
      tDraw := tDraw + (a2 - a1) / 1000
      tPres := tPres + (a3 - a2) / 1000
      frames := frames + 1
    if let .ok i := input then
      if let .ok (some k) ← i.poll then
        keys := keys + 1
        if k.value == 1 then color := color + 1
        Sys.log ("hello-tv: key " ++ toString k.code.toNat ++ " value " ++ toString k.value.toNat)
  Sys.log ("hello-tv: " ++ toString frames ++ " frames, " ++ toString keys ++ " key events")
  if frames > 0 then
    let f := frames.toUInt64
    Sys.log ("hello-tv: per frame us: acquire " ++ toString (tAcq / f).toNat ++ " draw " ++
      toString (tDraw / f).toNat ++ " present " ++ toString (tPres / f).toNat ++ " (full-screen fill " ++ toString (tFill / f).toNat ++ ")")
  d.close
  return 0

end Yt.App.Hello
