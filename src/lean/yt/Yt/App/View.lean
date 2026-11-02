import Yt.App.Model
import Yt.Ui.Text
import Yt.Lounge.Session

/-! Drawing the app's screens from the model (layout after the Rust app's
hub.rs and browse.rs, 1280×720). Called between `acquire` and `present`. -/
namespace Yt.App
open Yt.Platform.Tv Yt.Ui

def bg : UInt32 := 0x101219
def card : UInt32 := 0x212530
def accent : UInt32 := 0xAA1826
def white : UInt32 := 0xF5F5FA
def grey : UInt32 := 0xAFB4C3
def keyBg : UInt32 := 0x222432

def drawHome (d : Display) (f : Fonts) (m : Model) : BaseIO Unit := do
  let _ ← f.large.draw d 72 40 "YouTube".toUTF8 white
  let _ ← f.small.draw d 74 98 "Search and suggestions".toUTF8 grey
  let rows : Array (String × String) := #[("Search", "Find a video with the remote")] ++
    suggestions.map fun it => (it.title, it.channel ++ " · " ++ it.length)
  for i in [0:rows.size] do
    let (title, sub) := rows[i]!
    let y := 148 + i * 94
    d.fill 64 y 760 82 (if i == m.homeSel then accent else card)
    let _ ← f.body.draw d 88 (y + 10) title.toUTF8 white
    let _ ← f.small.draw d 88 (y + 48) sub.toUTF8 grey
  -- casting from a phone ("Link with TV code")
  d.fill 856 148 360 458 card
  let _ ← f.body.draw d 880 164 "From your phone".toUTF8 white
  let _ ← f.small.draw d 880 212 "YouTube app: Settings >".toUTF8 grey
  let _ ← f.small.draw d 880 244 "Watch on TV >".toUTF8 grey
  let _ ← f.small.draw d 880 276 "Enter TV code".toUTF8 grey
  match m.castCode with
  | some code =>
    let shown := (Yt.Lounge.formatCode code).toUTF8
    let font := if f.large.measure shown ≤ 312 then f.large else f.body
    let _ ← font.draw d (880 + (312 - min 312 (font.measure shown)) / 2) 320 shown white
  | none =>
    let t := if m.castAvailable then "Getting a code…" else "Not available: no proxy"
    let _ ← f.small.draw d 880 336 t.toUTF8 grey
  if !m.castAvailable then pure ()
  else if m.castRemotes.isEmpty then
    let _ ← f.small.draw d 880 400 "No phone connected".toUTF8 grey
  else
    let _ ← f.small.draw d 880 400 "Connected:".toUTF8 grey
    for i in [0:min 4 m.castRemotes.size] do
      let _ ← f.small.draw d 880 (436 + 34 * i) (f.small.ellipsize m.castRemotes[i]!.toUTF8 312) white
  let _ ← f.small.draw d 72 640 "Up/Down: choose   OK: open   Back: leave".toUTF8 grey
  let _ ← f.small.draw d 72 676 (f.small.ellipsize m.status.toUTF8 1100) white

def keyLabel : Key → String
  | .char c => (String.fromUTF8? (ByteArray.mk #[c])).getD "?"
  | .space => "space"
  | .del => "del"
  | .go => "GO"

def drawSearch (d : Display) (f : Fonts) (s : Search) : BaseIO Unit := do
  let keysFocused := s.focus == .keys
  -- query box
  d.fill 72 36 1136 64 card
  let q := if s.query.size == 0 then "Search YouTube".toUTF8 else s.query
  let x ← f.body.draw d 92 50 (f.body.ellipsize q 1060) (if s.query.size == 0 then grey else white)
  if keysFocused then d.fill (x + 2) 52 3 34 white
  -- keyboard
  for r in [0:4] do
    for c in [0:10] do
      let key := keyAt r c
      -- the two space cells draw as one wide key
      if !(key == .space && c == 7) then
        let w := if key == .space then 2 * 104 - 8 else 96
        let kx := 72 + c * 104
        let ky := 124 + r * 64
        let focused := keysFocused && r == s.row &&
          (c == s.col || (key == .space && (s.col == 6 || s.col == 7)))
        d.fill kx ky w 56 (if focused then accent else keyBg)
        let label := (keyLabel key).toUTF8
        let font := if label.size > 1 then f.small else f.body
        let lw := font.measure label
        let _ ← font.draw d (kx + (w - min w lw) / 2) (ky + (56 - font.lineHeight) / 2) label white
  -- results: four tiles from the selection's page
  let _ ← f.body.draw d 40 392 "Results".toUTF8 (if keysFocused then grey else white)
  let first := s.sel - min s.sel 3
  for k in [0:4] do
    let i := first + k
    if h : i < s.items.size then
      let it := s.items[i]
      let tx := 40 + k * 308
      let ty := 440
      let focused := !keysFocused && i == s.sel
      if focused then d.fill (tx - 6) (ty - 6) (thumbW + 12) (thumbH + 12) accent
      match it.thumb with
      | some rgb => d.blitRgb tx ty rgb 0 thumbW thumbH (thumbW * 3)
      | none => d.fill tx ty thumbW thumbH card
      let _ ← f.small.draw d tx (ty + thumbH + 8) (f.small.ellipsize it.title.toUTF8 thumbW) (if focused then white else grey)
  if let some it := s.items[s.sel]? then
    if !keysFocused then
      let x ← f.small.draw d 40 652 (f.small.ellipsize it.title.toUTF8 900) white
      let _ ← f.small.draw d (x + 16) 652 (f.small.ellipsize (it.channel ++ " · " ++ it.length).toUTF8 (1224 - x)) grey
  let _ ← f.small.draw d 40 684 (f.small.ellipsize s.status.toUTF8 1200) white

/-- Draw the whole screen for `m` into the leased frame. -/
def draw (d : Display) (f : Fonts) (m : Model) : BaseIO Unit := do
  let w := (← d.width).toNat
  let h := (← d.height).toNat
  d.fill 0 0 w h bg
  match m.screen with
  | .home => drawHome d f m
  | .search => drawSearch d f m.search
  | .player => pure ()

end Yt.App
