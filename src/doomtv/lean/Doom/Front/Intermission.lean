import Doom.Front.Draw
import Doom.Playsim.GameState
import Doom.Playsim.Intermission
import Doom.Playsim.Random
import Doom.Render.Types
import Doom.Wad

/-!
# Doom.Front.Intermission

The intermission screen (`wi_stuff.c` `WI_Drawer`): the episode map or
`INTERPIC` background with its animations, "<level> finished" /
"entering <level>", the "you are here" splats and pointer, and the
single-player, cooperative and deathmatch tallies.

The ticker (`WI_Ticker`) belongs to the playsim (`gs.wi`). Drawing reads a
`View` holding exactly the `wi_stuff.c` statics the drawer uses, so it does
not depend on how the game state stores them; `view` builds it from the
current `GameState`.

The current playsim ticker animates only the episode 1 background
(`epsd0animinfo`). A `Tracker`, run once per tic after `G_Ticker`, mirrors
`WI_initAnimatedBack` / `WI_updateAnimatedBack` for the other episodes on the
drawer side, drawing the same `M_Random` values the C code would (from a copy
of the generator, so the game's own state is untouched). It steps aside when
`gs.wi.anims` holds the episode's full table.
-/

namespace Doom.Front.Intermission

open Doom.Front.Draw
open Doom.Playsim.GameState
open Doom.Playsim.Random
open Doom.Render.Types
open Doom.Wad

def TICRATE : Int32 := 35
def WI_TITLEY : Int32 := 2
def WI_SPACINGY : Int32 := 33
def SP_STATSX : Int32 := 50
def SP_STATSY : Int32 := 50
def SP_TIMEX : Int32 := 16
def SP_TIMEY : Int32 := 200 - 32
def NG_STATSY : Int32 := 50
def NG_SPACINGX : Int32 := 64
def DM_MATRIXX : Int32 := 42
def DM_MATRIXY : Int32 := 68
def DM_SPACINGX : Int32 := 40
def DM_TOTALSX : Int32 := 269
def DM_KILLERSX : Int32 := 10
def DM_KILLERSY : Int32 := 100
def DM_VICTIMSX : Int32 := 5
def DM_VICTIMSY : Int32 := 50
def NUMCMAPS : Nat := 32
def NUMMAPS : Nat := 9
def MAXPLAYERS : Nat := 4

/-- `animenum_t`. -/
def ANIM_ALWAYS : Int32 := 0
def ANIM_RANDOM : Int32 := 1
def ANIM_LEVEL : Int32 := 2

/-- Static part of `anim_t`. -/
structure AnimInfo where
  type_ : Int32
  period : Int32
  nanims : Int32
  x : Int32
  y : Int32
  data1 : Int32 := 0
  data2 : Int32 := 0
  deriving Inhabited

/-- `epsd0animinfo` .. `epsd2animinfo`. -/
def animInfo : Array (Array AnimInfo) :=
  let a (x y : Int32) (p : Int32 := TICRATE / 3) : AnimInfo :=
    { type_ := ANIM_ALWAYS, period := p, nanims := 3, x, y }
  let l (n : Int32) (x y d : Int32) : AnimInfo :=
    { type_ := ANIM_LEVEL, period := TICRATE / 3, nanims := n, x, y, data1 := d }
  #[#[a 224 104, a 184 160, a 112 136, a 72 112, a 88 96, a 64 48, a 192 40, a 136 16,
      a 80 16, a 64 24],
    #[l 1 128 136 1, l 1 128 136 2, l 1 128 136 3, l 1 128 136 4, l 1 128 136 5,
      l 1 128 136 6, l 1 128 136 7, l 3 192 144 8, l 1 128 136 8],
    #[a 104 168, a 40 136, a 160 96, a 104 80, a 120 32, a 40 0 (TICRATE / 4)]]

/-- `NUMANIMS[epsd]` (0 beyond episode 3). -/
def numAnims (epsd : Nat) : Nat := (animInfo[epsd]?.map (·.size)).getD 0

/-- `lnodes[epsd][map]`: level locations on the episode maps, as (x, y). -/
def lnodes : Array (Array (Int32 × Int32)) := #[
  #[(185, 164), (148, 143), (69, 122), (209, 102), (116, 89), (166, 55), (71, 56), (135, 29), (71, 24)],
  #[(254, 25), (97, 50), (188, 64), (128, 78), (214, 92), (133, 130), (208, 136), (148, 140), (235, 158)],
  #[(156, 168), (48, 154), (174, 95), (265, 75), (130, 48), (279, 23), (198, 48), (140, 25), (281, 136)]]

/-- `stateenum_t`. -/
inductive Stage | statCount | showNextLoc | noState
  deriving BEq, Repr, Inhabited

/-- Cooperative and deathmatch tallies (`cnt_*[MAXPLAYERS]`, `dm_frags`, ...). -/
structure NetView where
  playeringame : Array Bool := Array.replicate 4 false
  me : Nat := 0
  dofrags : Bool := false
  cntKills : Array Int32 := Array.replicate 4 0
  cntItems : Array Int32 := Array.replicate 4 0
  cntSecret : Array Int32 := Array.replicate 4 0
  cntFrags : Array Int32 := Array.replicate 4 0
  dmFrags : Array (Array Int32) := Array.replicate 4 (Array.replicate 4 0)
  dmTotals : Array Int32 := Array.replicate 4 0
  deriving Inhabited

/-- What `WI_Drawer` reads: `wbs`, `state`, the `cnt_*` counters, the
background animation frames and `snl_pointeron`. -/
structure View where
  stage : Stage := .statCount
  commercial : Bool := false
  /-- `gameversion >= exe_ultimate`. -/
  retail : Bool := false
  deathmatch : Bool := false
  netgame : Bool := false
  /-- `wbs->epsd` after `WI_initVariables` (0-based). -/
  epsd : Nat := 0
  /-- `wbs->last` / `wbs->next` (0-based maps). -/
  last : Nat := 0
  next : Nat := 0
  didsecret : Bool := false
  pointerOn : Bool := true
  cntKills : Int32 := -1
  cntItems : Int32 := -1
  cntSecret : Int32 := -1
  cntTime : Int32 := -1
  cntPar : Int32 := -1
  /-- `anims[epsd][i].ctr + 1` per animation (0 = not drawn yet). -/
  animFrames : ByteArray := ByteArray.empty
  net : NetView := {}
  deriving Inhabited

/-! ## Drawer-side background animation -/

/-- Mutable part of `anim_t`. -/
structure AnimRt where
  ctr : Int32
  nexttic : Int32
  deriving Inhabited

structure Tracker where
  active : Bool := false
  epsd : Nat := 0
  /-- Last `bcnt` processed. -/
  bcnt : Int32 := 0
  /-- `gs.wi.state` seen at the last step. -/
  wiState : Int32 := 0
  anims : Array AnimRt := #[]
  /-- `M_Random` draws made here that the playsim did not make. -/
  extraDraws : Nat := 0
  deriving Inhabited

private def advance (rng : RandomState) (n : Nat) : RandomState :=
  { rng with rndindex := (rng.rndindex + n.toUInt32) &&& 0xff }

/-- `WI_initAnimatedBack` at `bcnt`. Returns the anims and the draws made. -/
def initAnims (epsd : Nat) (bcnt : Int32) (rng0 : RandomState) : Array AnimRt × Nat := Id.run do
  let infos := animInfo[epsd]?.getD #[]
  let mut rng := rng0
  let mut draws := 0
  let mut out : Array AnimRt := Array.emptyWithCapacity infos.size
  for a in infos do
    if a.type_ == ANIM_ALWAYS then
      let (r, rng') := mRandom rng
      rng := rng'; draws := draws + 1
      out := out.push { ctr := -1, nexttic := bcnt + 1 + r % (if a.period == 0 then 1 else a.period) }
    else if a.type_ == ANIM_RANDOM then
      let (r, rng') := mRandom rng
      rng := rng'; draws := draws + 1
      out := out.push { ctr := -1, nexttic := bcnt + 1 + a.data2 + r % (if a.data1 == 0 then 1 else a.data1) }
    else
      out := out.push { ctr := -1, nexttic := bcnt + 1 }
  return (out, draws)

/-- `WI_updateAnimatedBack` at `bcnt`; `statCount` is `state == StatCount`.
(No `ANIM_RANDOM` entries exist, so no draws happen here.) -/
def updateAnims (epsd : Nat) (anims : Array AnimRt) (bcnt : Int32) (statCount : Bool)
    (next : Int32) : Array AnimRt := Id.run do
  let infos := animInfo[epsd]?.getD #[]
  let mut out := anims
  for i in [0:min infos.size anims.size] do
    let a := infos[i]!
    let r := out[i]!
    if bcnt == r.nexttic then
      if a.type_ == ANIM_ALWAYS then
        let c := r.ctr + 1
        out := out.set! i { ctr := if c >= a.nanims then 0 else c, nexttic := bcnt + a.period }
      else if a.type_ == ANIM_RANDOM then
        let c := r.ctr + 1
        out := out.set! i (if c == a.nanims then { ctr := -1, nexttic := bcnt + a.data2 }
          else { ctr := c, nexttic := bcnt + a.period })
      else if !(statCount && i == 7) && next == a.data1 then
        let c := r.ctr + 1
        out := out.set! i { ctr := if c == a.nanims then c - 1 else c, nexttic := bcnt + a.period }
  return out

/-- `wbs->epsd` as the drawer uses it (`WI_initVariables`). -/
def drawEpisode (gs : GameState) (retail : Bool) : Nat :=
  let e := gs.wi.epsd.toNatClampNeg
  if !retail && e > 2 then e - 3 else e

/-- Whether `gs.wi.anims` already animates this episode's background. -/
def playsimAnimates (gs : GameState) (commercial retail : Bool) : Bool :=
  !commercial && gs.wi.anims.size == numAnims (drawEpisode gs retail) && gs.wi.anims.size > 0

/-- Follow one tic of `WI_Ticker` (call after `G_Ticker`). -/
def Tracker.step (t : Tracker) (gs : GameState) (commercial retail : Bool) : Tracker := Id.run do
  if gs.gamestate != gamestateIntermission then return { t with active := false }
  let wi := gs.wi
  let epsd := drawEpisode gs retail
  if commercial || epsd > 2 || playsimAnimates gs commercial retail then
    return { active := true, epsd, bcnt := wi.bcnt, wiState := wi.state }
  let mut t := t
  if !t.active || wi.bcnt < t.bcnt || t.epsd != epsd then
    -- `WI_Start` -> `WI_initStats` at bcnt 0, from the generator as it was.
    let (anims, draws) := initAnims epsd 0 gs.rng
    t := { active := true, epsd, bcnt := 0, wiState := Doom.Playsim.Intermission.StatCount,
           anims, extraDraws := draws }
  let statCount := t.wiState == Doom.Playsim.Intermission.StatCount
  let mut anims := t.anims
  let mut b := t.bcnt + 1
  while b <= wi.bcnt do
    anims := updateAnims epsd anims b statCount wi.next
    b := b + 1
  let mut extra := t.extraDraws
  if statCount && wi.state == Doom.Playsim.Intermission.ShowNextLoc then
    -- `WI_initShowNextLoc`: after this tic's sounds, plus the draws the
    -- playsim did not make.
    let (a2, draws) := initAnims epsd wi.bcnt (advance gs.rng extra)
    anims := a2
    extra := extra + draws
  return { t with bcnt := wi.bcnt, wiState := wi.state, anims, extraDraws := extra }

/-- The drawer's inputs from the current game state (`wbs`, the `wi_stuff.c`
statics the playsim keeps in `gs.wi`, and `snl_pointeron`). -/
def view (gs : GameState) (_t : Tracker) (commercial retail : Bool) : View := Id.run do
  let wi := gs.wi
  let epsd := drawEpisode gs retail
  let stage :=
    if wi.state == Doom.Playsim.Intermission.StatCount then Stage.statCount
    else if wi.state == Doom.Playsim.Intermission.ShowNextLoc then .showNextLoc
    else .noState
  let mut frames := ByteArray.empty
  if !commercial && epsd <= 2 then
    for a in wi.anims do frames := frames.push (a.ctr + 1).toUInt32.toUInt8
  let net : NetView := {
    playeringame := gs.playeringame, me := wi.me.toNatClampNeg, dofrags := wi.dofrags != 0
    cntKills := wi.cntKillsA, cntItems := wi.cntItemsA, cntSecret := wi.cntSecretA
    cntFrags := wi.cntFrags, dmFrags := wi.dmFrags, dmTotals := wi.dmTotals }
  return {
    stage, commercial, retail, epsd
    deathmatch := gs.deathmatch, netgame := gs.netgame
    last := wi.last.toNatClampNeg, next := wi.next.toNatClampNeg
    didsecret := gs.wminfo.didsecret
    pointerOn := stage != .showNextLoc || wi.snlPointeron
    cntKills := wi.cntKills, cntItems := wi.cntItems, cntSecret := wi.cntSecret
    cntTime := wi.cntTime, cntPar := wi.cntPar
    animFrames := frames, net }

/-! ## Drawing -/

/-- Patches used by every intermission screen (`WI_loadData`). -/
structure Pics where
  num : Array PatchInfo
  wiminus : Option PatchInfo
  percent : Option PatchInfo
  colon : Option PatchInfo

private def loadPics (wad : WadDirectory) : Pics :=
  { num := (List.range 10).toArray.filterMap (fun i => patchNamed? wad s!"WINUM{i}")
    -- "More hacks on minus sign": only when present (`W_CheckNumForName > 0`).
    wiminus := patchNamed? wad "WIMINUS"
    percent := patchNamed? wad "WIPCNT"
    colon := patchNamed? wad "WICOLON" }

private def opt (wad : WadDirectory) (fb : Framebuffer) (p? : Option PatchInfo) (x y : Int32) :
    Except String Framebuffer :=
  match p? with
  | some p => patch wad fb p x y
  | none => pure fb

/-- `WI_drawNum`: right-aligned at `x`; `digits < 0` means as many as needed.
Returns the new x. -/
def drawNum (wad : WadDirectory) (pics : Pics) (fb : Framebuffer) (x y n : Int32) (digits : Int32) :
    Except String (Framebuffer × Int32) := do
  let fontwidth := (pics.num[0]?.map (·.width)).getD 0
  let mut digits := digits
  if digits < 0 then
    if n == 0 then digits := 1
    else
      digits := 0
      let mut temp := n
      while temp != 0 do
        temp := temp / 10
        digits := digits + 1
  let neg := n < 0
  let mut n := if neg then -n else n
  if n == 1994 then return (fb, 0)
  let mut fb := fb
  let mut x := x
  while digits > 0 do
    digits := digits - 1
    x := x - fontwidth
    if let some p := pics.num[(n % 10).toNatClampNeg]? then fb ← patch wad fb p x y
    n := n / 10
  if neg then
    if let some m := pics.wiminus then
      x := x - 8
      fb ← patch wad fb m x y
  return (fb, x)

/-- `WI_drawPercent`. -/
def drawPercent (wad : WadDirectory) (pics : Pics) (fb : Framebuffer) (x y p : Int32) :
    Except String Framebuffer := do
  if p < 0 then return fb
  let fb ← opt wad fb pics.percent x y
  return (← drawNum wad pics fb x y p (-1)).1

/-- `WI_drawTime`: m:ss (or h:mm:ss), or "sucks" past 61:59. -/
def drawTime (wad : WadDirectory) (pics : Pics) (fb : Framebuffer) (x y t : Int32) :
    Except String Framebuffer := do
  if t < 0 then return fb
  if t <= 61 * 59 then
    let colonW := (pics.colon.map (·.width)).getD 0
    let mut div : Int32 := 1
    let mut x := x
    let mut fb := fb
    repeat
      let n := (t / div) % 60
      let (fb', x') ← drawNum wad pics fb x y n 2
      fb := fb'
      x := x' - colonW
      div := div * 60
      if div == 60 || t / div != 0 then fb ← opt wad fb pics.colon x y
      if t / div == 0 then break
    return fb
  else
    match patchNamed? wad "WISUCKS" with
    | some s => patch wad fb s (x - s.width) y
    | none => pure fb

private def lname (v : View) (n : Nat) : String :=
  if v.commercial then s!"CWILV{if n < 10 then "0" else ""}{n}" else s!"WILV{v.epsd}{n}"

/-- `WI_slamBackground`. -/
def background (v : View) : String :=
  if v.commercial || (v.retail && v.epsd == 3) then "INTERPIC" else s!"WIMAP{v.epsd}"

/-- `WI_drawLF`: "<level> finished". -/
def drawLF (wad : WadDirectory) (fb : Framebuffer) (v : View) : Except String Framebuffer := do
  let y := WI_TITLEY
  let fin := patchNamed? wad "WIF"
  let finX := (320 - (fin.map (·.width)).getD 0) / 2
  if !v.commercial || v.last < NUMCMAPS then
    let some ln := patchNamed? wad (lname v v.last) | return fb
    let fb ← patch wad fb ln ((320 - ln.width) / 2) y
    opt wad fb fin finX (y + (5 * ln.height) / 4)
  else if v.last == NUMCMAPS then
    opt wad fb fin finX y
  else
    -- Past MAP33 vanilla reads a bogus patch and errors; draw nothing.
    pure fb

/-- `WI_drawEL`: "entering <level>". -/
def drawEL (wad : WadDirectory) (fb : Framebuffer) (v : View) : Except String Framebuffer := do
  let y := WI_TITLEY
  let ent := patchNamed? wad "WIENTER"
  let fb ← opt wad fb ent ((320 - (ent.map (·.width)).getD 0) / 2) y
  let some ln := patchNamed? wad (lname v v.next) | return fb
  patch wad fb ln ((320 - ln.width) / 2) (y + (5 * ln.height) / 4)

/-- `WI_drawOnLnode`: the first of `cs` that fits on screen at node `n`. -/
def drawOnLnode (wad : WadDirectory) (fb : Framebuffer) (v : View) (n : Nat)
    (cs : Array PatchInfo) : Except String Framebuffer := do
  let some (nx, ny) := (lnodes[v.epsd]?.bind (·[n]?)) | return fb
  let mut i := 0
  let mut fits := false
  while !fits && i < cs.size && i < 2 do
    let c := cs[i]!
    let left := nx - c.leftoffset
    let top := ny - c.topoffset
    if left >= 0 && left + c.width < 320 && top >= 0 && top + c.height < 200 then fits := true
    else i := i + 1
  if fits && i < 2 then patch wad fb cs[i]! nx ny else pure fb

/-- `WI_drawAnimatedBack`. -/
def drawAnimatedBack (wad : WadDirectory) (fb : Framebuffer) (v : View) : Except String Framebuffer := do
  if v.commercial || v.epsd > 2 then return fb
  let infos := animInfo[v.epsd]?.getD #[]
  let mut fb := fb
  for i in [0:min infos.size v.animFrames.size] do
    let f := v.animFrames[i]!.toNat
    if f > 0 then
      let a := infos[i]!
      -- "MONDO HACK": episode 2's last animation reuses animation 4's frames.
      let j := if v.epsd == 1 && i == 8 then 4 else i
      let name := s!"WIA{v.epsd}{if j < 10 then "0" else ""}{j}0{f - 1}"
      fb ← named wad fb name a.x a.y
  return fb

/-- `WI_drawShowNextLoc` (`WI_drawNoState` passes `pointerOn = true`). -/
def drawShowNextLoc (wad : WadDirectory) (fb : Framebuffer) (v : View) : Except String Framebuffer := do
  let mut fb ← named wad fb (background v) 0 0
  fb ← drawAnimatedBack wad fb v
  if !v.commercial then
    if v.epsd > 2 then return (← drawEL wad fb v)
    let last := if v.last == 8 then v.next - 1 else v.last
    let splat := (patchNamed? wad "WISPLAT").toArray
    for i in [0:last + 1] do
      fb ← drawOnLnode wad fb v i splat
    if v.didsecret then fb ← drawOnLnode wad fb v 8 splat
    if v.pointerOn then
      let yah := (patchNamed? wad "WIURH0").toArray ++ (patchNamed? wad "WIURH1").toArray
      fb ← drawOnLnode wad fb v v.next yah
  if !v.commercial || v.next != 30 then fb ← drawEL wad fb v
  return fb

/-- `WI_drawStats`: single-player kills, items, secrets, time and par. -/
def drawStats (wad : WadDirectory) (fb : Framebuffer) (v : View) : Except String Framebuffer := do
  let pics := loadPics wad
  let lh := (3 * (pics.num[0]?.map (·.height)).getD 0) / 2
  let mut fb ← drawAnimatedBack wad fb v
  fb ← drawLF wad fb v
  fb ← named wad fb "WIOSTK" SP_STATSX SP_STATSY
  fb ← drawPercent wad pics fb (320 - SP_STATSX) SP_STATSY v.cntKills
  fb ← named wad fb "WIOSTI" SP_STATSX (SP_STATSY + lh)
  fb ← drawPercent wad pics fb (320 - SP_STATSX) (SP_STATSY + lh) v.cntItems
  fb ← named wad fb "WISCRT2" SP_STATSX (SP_STATSY + 2 * lh)
  fb ← drawPercent wad pics fb (320 - SP_STATSX) (SP_STATSY + 2 * lh) v.cntSecret
  fb ← named wad fb "WITIME" SP_TIMEX SP_TIMEY
  fb ← drawTime wad pics fb (320 / 2 - SP_TIMEX) SP_TIMEY v.cntTime
  if v.epsd < 3 then
    fb ← named wad fb "WIPAR" (320 / 2 + SP_TIMEX) SP_TIMEY
    fb ← drawTime wad pics fb (320 - SP_TIMEX) SP_TIMEY v.cntPar
  return fb

private def ingame (n : NetView) (i : Nat) : Bool := n.playeringame.getD i false
private def at4 (a : Array Int32) (i : Nat) : Int32 := a.getD i 0

/-- `WI_drawNetgameStats`: cooperative tallies per player. -/
def drawNetgameStats (wad : WadDirectory) (fb : Framebuffer) (v : View) : Except String Framebuffer := do
  let pics := loadPics wad
  let n := v.net
  let star := patchNamed? wad "STFST01"
  let starW := (star.map (·.width)).getD 0
  let statsx := 32 + starW / 2 + (if n.dofrags then 0 else 32)
  let pwidth := (pics.percent.map (·.width)).getD 0
  let kills := patchNamed? wad "WIOSTK"
  -- The French IWAD's WIOBJ "items" title is used for cooperative games.
  let items := if (checkNumForName wad "WIOBJ").isSome && !v.deathmatch then patchNamed? wad "WIOBJ"
    else patchNamed? wad "WIOSTI"
  let secret := patchNamed? wad "WIOSTS"
  let frags := patchNamed? wad "WIFRGS"
  let w (p : Option PatchInfo) := (p.map (·.width)).getD 0
  let mut fb ← drawAnimatedBack wad fb v
  fb ← drawLF wad fb v
  fb ← opt wad fb kills (statsx + NG_SPACINGX - w kills) NG_STATSY
  fb ← opt wad fb items (statsx + 2 * NG_SPACINGX - w items) NG_STATSY
  fb ← opt wad fb secret (statsx + 3 * NG_SPACINGX - w secret) NG_STATSY
  if n.dofrags then fb ← opt wad fb frags (statsx + 4 * NG_SPACINGX - w frags) NG_STATSY
  let mut y := NG_STATSY + (kills.map (·.height)).getD 0
  for i in [0:MAXPLAYERS] do
    if !ingame n i then continue
    let mut x := statsx
    let p := patchNamed? wad s!"STPB{i}"
    fb ← opt wad fb p (x - w p) y
    if i == n.me then fb ← opt wad fb star (x - w p) y
    x := x + NG_SPACINGX
    fb ← drawPercent wad pics fb (x - pwidth) (y + 10) (at4 n.cntKills i)
    x := x + NG_SPACINGX
    fb ← drawPercent wad pics fb (x - pwidth) (y + 10) (at4 n.cntItems i)
    x := x + NG_SPACINGX
    fb ← drawPercent wad pics fb (x - pwidth) (y + 10) (at4 n.cntSecret i)
    x := x + NG_SPACINGX
    if n.dofrags then fb := (← drawNum wad pics fb x (y + 10) (at4 n.cntFrags i) (-1)).1
    y := y + WI_SPACINGY
  return fb

/-- `WI_drawDeathmatchStats`: the frag matrix. -/
def drawDeathmatchStats (wad : WadDirectory) (fb : Framebuffer) (v : View) : Except String Framebuffer := do
  let pics := loadPics wad
  let n := v.net
  let total := patchNamed? wad "WIMSTT"
  let w (p : Option PatchInfo) := (p.map (·.width)).getD 0
  let mut fb ← drawAnimatedBack wad fb v
  fb ← drawLF wad fb v
  fb ← opt wad fb total (DM_TOTALSX - w total / 2) (DM_MATRIXY - WI_SPACINGY + 10)
  fb ← named wad fb "WIKILRS" DM_KILLERSX DM_KILLERSY
  fb ← named wad fb "WIVCTMS" DM_VICTIMSX DM_VICTIMSY
  let mut x := DM_MATRIXX + DM_SPACINGX
  let mut y := DM_MATRIXY
  for i in [0:MAXPLAYERS] do
    if ingame n i then
      let p := patchNamed? wad s!"STPB{i}"
      fb ← opt wad fb p (x - w p / 2) (DM_MATRIXY - WI_SPACINGY)
      fb ← opt wad fb p (DM_MATRIXX - w p / 2) y
      if i == n.me then
        fb ← named wad fb "STFDEAD0" (x - w p / 2) (DM_MATRIXY - WI_SPACINGY)
        fb ← named wad fb "STFST01" (DM_MATRIXX - w p / 2) y
    x := x + DM_SPACINGX
    y := y + WI_SPACINGY
  y := DM_MATRIXY + 10
  let nw := (pics.num[0]?.map (·.width)).getD 0
  for i in [0:MAXPLAYERS] do
    x := DM_MATRIXX + DM_SPACINGX
    if ingame n i then
      for j in [0:MAXPLAYERS] do
        if ingame n j then
          fb := (← drawNum wad pics fb (x + nw) y (at4 (n.dmFrags.getD i #[]) j) 2).1
        x := x + DM_SPACINGX
      fb := (← drawNum wad pics fb (DM_TOTALSX + nw) y (at4 n.dmTotals i) 2).1
    y := y + WI_SPACINGY
  return fb

/-- `WI_Drawer` into `fb` (the whole screen is repainted). -/
def draw (wad : WadDirectory) (fb : Framebuffer) (v : View) : Except String Framebuffer := do
  match v.stage with
  | .statCount =>
    let fb ← named wad fb (background v) 0 0
    if v.deathmatch then drawDeathmatchStats wad fb v
    else if v.netgame then drawNetgameStats wad fb v
    else drawStats wad fb v
  | .showNextLoc => drawShowNextLoc wad fb v
  | .noState => drawShowNextLoc wad fb { v with pointerOn := true }

end Doom.Front.Intermission
