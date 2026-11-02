import Doom.Playsim.Defs
import Doom.Playsim.GameState
import Doom.Playsim.Player
import Doom.Playsim.Sound

/-!
# Doom.Playsim.Intermission

`wi_stuff.c` logic: `WI_Start` and `WI_Ticker` for single-player, coop and
deathmatch stats, background animation timers (their `M_Random` draws) and
the stage sounds. Drawing is the front end's.
-/

namespace Doom.Playsim.Intermission

open Doom.Playsim.Defs
open Doom.Playsim.GameState
open Doom.Playsim.Player
open Doom.Playsim.Sound

def StatCount : Int32 := 0
def ShowNextLoc : Int32 := 1
def NoState : Int32 := 2
def ANIM_ALWAYS : Int32 := 0
def ANIM_RANDOM : Int32 := 1
def ANIM_LEVEL : Int32 := 2
def TICRATE : Int32 := 35
def SHOWNEXTLOCDELAY : Int32 := 4

/-- `anims[episode]` logic fields (type, period, nanims, data1). -/
def animTable (epsd : Nat) : Array WiAnim :=
  let a (t p n d : Int32) : WiAnim := { type_ := t, period := p, nanims := n, data1 := d, ctr := -1, nexttic := 0 }
  if epsd == 0 then Array.replicate 10 (a ANIM_ALWAYS (TICRATE / 3) 3 0)
  else if epsd == 1 then
    #[a ANIM_LEVEL (TICRATE/3) 1 1, a ANIM_LEVEL (TICRATE/3) 1 2, a ANIM_LEVEL (TICRATE/3) 1 3,
      a ANIM_LEVEL (TICRATE/3) 1 4, a ANIM_LEVEL (TICRATE/3) 1 5, a ANIM_LEVEL (TICRATE/3) 1 6,
      a ANIM_LEVEL (TICRATE/3) 1 7, a ANIM_LEVEL (TICRATE/3) 3 8, a ANIM_LEVEL (TICRATE/3) 1 8]
  else if epsd == 2 then
    #[a ANIM_ALWAYS (TICRATE/3) 3 0, a ANIM_ALWAYS (TICRATE/3) 3 0, a ANIM_ALWAYS (TICRATE/3) 3 0,
      a ANIM_ALWAYS (TICRATE/3) 3 0, a ANIM_ALWAYS (TICRATE/3) 3 0, a ANIM_ALWAYS (TICRATE/4) 3 0]
  else #[]

@[inline] def modWi (f : WiState → WiState) : M Unit := modify fun gs => { gs with wi := f gs.wi }

/-- Keep the renderer's single-player counters in step with `cnt_*[0]`. -/
def syncSp : M Unit := modWi fun w => {
  w with
  cntKills := w.cntKillsA[0]!, cntItems := w.cntItemsA[0]!, cntSecret := w.cntSecretA[0]! }

def initAnimatedBack : M Unit := do
  let gs ← get
  if gs.res.gamemode == gm_commercial then return
  if gs.wminfo.epsd > 2 then return
  let mut anims := animTable gs.wminfo.epsd.toNatClampNeg
  for i in [0:anims.size] do
    let a := anims[i]!
    let mut nt := a.nexttic
    if a.type_ == ANIM_ALWAYS then nt := (← get).wi.bcnt + 1 + ((← mRandomM) % a.period)
    else if a.type_ == ANIM_RANDOM then nt := (← get).wi.bcnt + 1 + a.data2 + ((← mRandomM) % a.data1)
    else if a.type_ == ANIM_LEVEL then nt := (← get).wi.bcnt + 1
    anims := anims.set! i { a with ctr := -1, nexttic := nt }
  modWi fun w => { w with anims }

def updateAnimatedBack : M Unit := do
  let gs ← get
  if gs.res.gamemode == gm_commercial then return
  if gs.wminfo.epsd > 2 then return
  let mut anims := gs.wi.anims
  let bcnt := gs.wi.bcnt
  for i in [0:anims.size] do
    let a := anims[i]!
    if bcnt == a.nexttic then
      if a.type_ == ANIM_ALWAYS then
        let c := a.ctr + 1
        anims := anims.set! i { a with ctr := if c >= a.nanims then 0 else c, nexttic := bcnt + a.period }
      else if a.type_ == ANIM_RANDOM then
        let c := a.ctr + 1
        if c == a.nanims then
          let r ← mRandomM
          anims := anims.set! i { a with ctr := -1, nexttic := bcnt + a.data2 + (r % a.data1) }
        else anims := anims.set! i { a with ctr := c, nexttic := bcnt + a.period }
      else if a.type_ == ANIM_LEVEL then
        if !((← get).wi.state == StatCount && i == 7) && gs.wminfo.next == a.data1 then
          let c := a.ctr + 1
          anims := anims.set! i { a with ctr := if c == a.nanims then c - 1 else c, nexttic := bcnt + a.period }
  modWi fun w => { w with anims }

def fragSum (gs : GameState) (pn : Nat) : Int32 := Id.run do
  let mut frags : Int32 := 0
  let pl := gs.wminfo.plyr[pn]!
  for i in [0:4] do
    if gs.playeringame[i]! && i != pn then frags := frags + pl.frags[i]!
  return frags - pl.frags[pn]!

def initNoState : M Unit := modWi fun w => { w with state := NoState, acceleratestage := 0, cnt := 10 }

def initShowNextLoc : M Unit := do
  modWi fun w => { w with state := ShowNextLoc, acceleratestage := 0, cnt := SHOWNEXTLOCDELAY * TICRATE }
  initAnimatedBack

def initStats : M Unit := do
  modWi fun w => {
    w with
    state := StatCount, acceleratestage := 0, spState := 1
    cntKillsA := w.cntKillsA.set! 0 (-1), cntItemsA := w.cntItemsA.set! 0 (-1)
    cntSecretA := w.cntSecretA.set! 0 (-1), cntTime := -1, cntPar := -1, cntPause := TICRATE }
  initAnimatedBack

def initNetgameStats : M Unit := do
  modWi fun w => { w with state := StatCount, acceleratestage := 0, ngState := 1, cntPause := TICRATE }
  for i in [0:4] do
    let gs ← get
    if !gs.playeringame[i]! then continue
    let fs := fragSum gs i
    modWi fun w => {
      w with
      cntKillsA := w.cntKillsA.set! i 0, cntItemsA := w.cntItemsA.set! i 0
      cntSecretA := w.cntSecretA.set! i 0, cntFrags := w.cntFrags.set! i 0, dofrags := w.dofrags + fs }
  modWi fun w => { w with dofrags := if w.dofrags != 0 then 1 else 0 }
  initAnimatedBack

def initDeathmatchStats : M Unit := do
  modWi fun w => { w with state := StatCount, acceleratestage := 0, dmState := 1, cntPause := TICRATE }
  for i in [0:4] do
    let gs ← get
    if gs.playeringame[i]! then
      for j in [0:4] do
        if gs.playeringame[j]! then
          modWi fun w => { w with dmFrags := w.dmFrags.modify i fun r => r.set! j 0 }
      modWi fun w => { w with dmTotals := w.dmTotals.set! i 0 }
  initAnimatedBack

/-- Percentage `(n * 100) / max`. -/
@[inline] def pct (n m : Int32) : Int32 := (n * 100) / m

def updateStats : M Unit := do
  updateAnimatedBack
  let gs ← get
  let me := gs.wi.me.toNatClampNeg
  let pl := gs.wminfo.plyr[me]!
  let wb := gs.wminfo
  if gs.wi.acceleratestage != 0 && gs.wi.spState != 10 then
    modWi fun w => {
      w with
      acceleratestage := 0
      cntKillsA := w.cntKillsA.set! 0 (pct pl.skills wb.maxkills)
      cntItemsA := w.cntItemsA.set! 0 (pct pl.sitems wb.maxitems)
      cntSecretA := w.cntSecretA.set! 0 (pct pl.ssecret wb.maxsecret)
      cntTime := pl.stime / TICRATE, cntPar := wb.partime / TICRATE, spState := 10 }
    startSoundNull sfx_barexp
  let w := (← get).wi
  let tick4 := (w.bcnt &&& 3) == 0
  if w.spState == 2 || w.spState == 4 || w.spState == 6 then
    let (cur, target) :=
      if w.spState == 2 then (w.cntKillsA[0]! + 2, pct pl.skills wb.maxkills)
      else if w.spState == 4 then (w.cntItemsA[0]! + 2, pct pl.sitems wb.maxitems)
      else (w.cntSecretA[0]! + 2, pct pl.ssecret wb.maxsecret)
    let setc (v : Int32) : M Unit := modWi fun w =>
      if w.spState == 2 then { w with cntKillsA := w.cntKillsA.set! 0 v }
      else if w.spState == 4 then { w with cntItemsA := w.cntItemsA.set! 0 v }
      else { w with cntSecretA := w.cntSecretA.set! 0 v }
    setc cur
    if tick4 then startSoundNull sfx_pistol
    if cur >= target then
      setc target
      startSoundNull sfx_barexp
      modWi fun w => { w with spState := w.spState + 1 }
  else if w.spState == 8 then
    if tick4 then startSoundNull sfx_pistol
    let mut ct := w.cntTime + 3
    if ct >= pl.stime / TICRATE then ct := pl.stime / TICRATE
    let mut cp := w.cntPar + 3
    modWi fun w => { w with cntTime := ct, cntPar := cp }
    if cp >= wb.partime / TICRATE then
      cp := wb.partime / TICRATE
      modWi fun w => { w with cntPar := cp }
      if ct >= pl.stime / TICRATE then
        startSoundNull sfx_barexp
        modWi fun w => { w with spState := w.spState + 1 }
  else if w.spState == 10 then
    if w.acceleratestage != 0 then
      startSoundNull sfx_sgcock
      if gs.res.gamemode == gm_commercial then initNoState else initShowNextLoc
  else if (w.spState &&& 1) != 0 then
    let cp := w.cntPause - 1
    if cp == 0 then modWi fun w => { w with spState := w.spState + 1, cntPause := TICRATE }
    else modWi fun w => { w with cntPause := cp }

def updateNetgameStats : M Unit := do
  updateAnimatedBack
  let gs ← get
  let wb := gs.wminfo
  if gs.wi.acceleratestage != 0 && gs.wi.ngState != 10 then
    modWi fun w => { w with acceleratestage := 0 }
    for i in [0:4] do
      let gs ← get
      if !gs.playeringame[i]! then continue
      let pl := wb.plyr[i]!
      let fs := fragSum gs i
      modWi fun w => {
        w with
        cntKillsA := w.cntKillsA.set! i (pct pl.skills wb.maxkills)
        cntItemsA := w.cntItemsA.set! i (pct pl.sitems wb.maxitems)
        cntSecretA := w.cntSecretA.set! i (pct pl.ssecret wb.maxsecret)
        cntFrags := if w.dofrags != 0 then w.cntFrags.set! i fs else w.cntFrags }
    startSoundNull sfx_barexp
    modWi fun w => { w with ngState := 10 }
  let w := (← get).wi
  let tick4 := (w.bcnt &&& 3) == 0
  if w.ngState == 2 || w.ngState == 4 || w.ngState == 6 then
    if tick4 then startSoundNull sfx_pistol
    let mut still := false
    for i in [0:4] do
      let gs ← get
      if !gs.playeringame[i]! then continue
      let pl := wb.plyr[i]!
      let w := gs.wi
      let (cur, target) :=
        if w.ngState == 2 then (w.cntKillsA[i]! + 2, pct pl.skills wb.maxkills)
        else if w.ngState == 4 then (w.cntItemsA[i]! + 2, pct pl.sitems wb.maxitems)
        else (w.cntSecretA[i]! + 2, pct pl.ssecret wb.maxsecret)
      let v := if cur >= target then target else cur
      if cur < target then still := true
      modWi fun w =>
        if w.ngState == 2 then { w with cntKillsA := w.cntKillsA.set! i v }
        else if w.ngState == 4 then { w with cntItemsA := w.cntItemsA.set! i v }
        else { w with cntSecretA := w.cntSecretA.set! i v }
    if !still then
      startSoundNull sfx_barexp
      modWi fun w => {
        w with
        ngState :=
        if w.ngState == 6 then w.ngState + 1 + 2 * (if w.dofrags == 0 then 1 else 0) else w.ngState + 1 }
  else if w.ngState == 8 then
    if tick4 then startSoundNull sfx_pistol
    let mut still := false
    for i in [0:4] do
      let gs ← get
      if !gs.playeringame[i]! then continue
      let fsum := fragSum gs i
      let cur := gs.wi.cntFrags[i]! + 1
      let v := if cur >= fsum then fsum else cur
      if cur < fsum then still := true
      modWi fun w => { w with cntFrags := w.cntFrags.set! i v }
    if !still then
      startSoundNull sfx_pldeth
      modWi fun w => { w with ngState := w.ngState + 1 }
  else if w.ngState == 10 then
    if w.acceleratestage != 0 then
      startSoundNull sfx_sgcock
      if gs.res.gamemode == gm_commercial then initNoState else initShowNextLoc
  else if (w.ngState &&& 1) != 0 then
    let cp := w.cntPause - 1
    if cp == 0 then modWi fun w => { w with ngState := w.ngState + 1, cntPause := TICRATE }
    else modWi fun w => { w with cntPause := cp }

def clamp99 (v : Int32) : Int32 := if v > 99 then 99 else if v < -99 then -99 else v

def updateDeathmatchStats : M Unit := do
  updateAnimatedBack
  let gs ← get
  let wb := gs.wminfo
  if gs.wi.acceleratestage != 0 && gs.wi.dmState != 4 then
    modWi fun w => { w with acceleratestage := 0 }
    for i in [0:4] do
      let gs ← get
      if !gs.playeringame[i]! then continue
      for j in [0:4] do
        if gs.playeringame[j]! then
          modWi fun w => { w with dmFrags := w.dmFrags.modify i fun r => r.set! j wb.plyr[i]!.frags[j]! }
      let fs := fragSum gs i
      modWi fun w => { w with dmTotals := w.dmTotals.set! i fs }
    startSoundNull sfx_barexp
    modWi fun w => { w with dmState := 4 }
  let w := (← get).wi
  if w.dmState == 2 then
    if (w.bcnt &&& 3) == 0 then startSoundNull sfx_pistol
    let mut still := false
    for i in [0:4] do
      let gs ← get
      if !gs.playeringame[i]! then continue
      for j in [0:4] do
        let gs ← get
        let target := wb.plyr[i]!.frags[j]!
        let cur := gs.wi.dmFrags[i]![j]!
        if gs.playeringame[j]! && cur != target then
          let v := clamp99 (if target < 0 then cur - 1 else cur + 1)
          modWi fun w => { w with dmFrags := w.dmFrags.modify i fun r => r.set! j v }
          still := true
      let fs := clamp99 (fragSum (← get) i)
      modWi fun w => { w with dmTotals := w.dmTotals.set! i fs }
    if !still then
      startSoundNull sfx_barexp
      modWi fun w => { w with dmState := w.dmState + 1 }
  else if w.dmState == 4 then
    if w.acceleratestage != 0 then
      startSoundNull sfx_slop
      if gs.res.gamemode == gm_commercial then initNoState else initShowNextLoc
  else if (w.dmState &&& 1) != 0 then
    let cp := w.cntPause - 1
    if cp == 0 then modWi fun w => { w with dmState := w.dmState + 1, cntPause := TICRATE }
    else modWi fun w => { w with cntPause := cp }

def checkForAccelerate : M Unit := do
  for i in [0:4] do
    let gs ← get
    if !gs.playeringame[i]! then continue
    let p := gs.players[i]!
    let mut accel := false
    let mut p' := p
    if (p.cmd.buttons &&& BT_ATTACK) != 0 then
      if !p.attackdown then accel := true
      p' := { p' with attackdown := true }
    else p' := { p' with attackdown := false }
    if (p.cmd.buttons &&& BT_USE) != 0 then
      if !p.usedown then accel := true
      p' := { p' with usedown := true }
    else p' := { p' with usedown := false }
    modifyPl i fun _ => p'
    if accel then modWi fun w => { w with acceleratestage := 1 }

/-- `WI_Start(&wminfo)`. -/
def wiStart : M Unit := do
  modify fun gs =>
    let wb := gs.wminfo
    let wb := {
      wb with
      maxkills := if wb.maxkills == 0 then 1 else wb.maxkills
      maxitems := if wb.maxitems == 0 then 1 else wb.maxitems
      maxsecret := if wb.maxsecret == 0 then 1 else wb.maxsecret
      epsd := if gs.res.gameversion < exe_ultimate && wb.epsd > 2 then wb.epsd - 3 else wb.epsd }
    { gs with wminfo := wb
              wi := { gs.wi with
                      acceleratestage := 0, cnt := 0, bcnt := 0, me := wb.pnum
                      maxkills := wb.maxkills, maxitems := wb.maxitems, maxsecret := wb.maxsecret
                      partime := wb.partime, next := wb.next, last := wb.last, epsd := wb.epsd
                      skills := wb.plyr[wb.pnum.toNatClampNeg]!.skills
                      sitems := wb.plyr[wb.pnum.toNatClampNeg]!.sitems
                      ssecret := wb.plyr[wb.pnum.toNatClampNeg]!.ssecret
                      stime := wb.plyr[wb.pnum.toNatClampNeg]!.stime } }
  let gs ← get
  if gs.deathmatchMode != 0 then initDeathmatchStats
  else if gs.netgame then initNetgameStats
  else initStats
  syncSp

/-- `WI_Ticker`; `worldDone` is `G_WorldDone`. -/
def wiTicker (worldDone : M Unit) : M Unit := do
  modWi fun w => { w with bcnt := w.bcnt + 1 }
  if (← get).wi.bcnt == 1 then
    if (← get).res.gamemode == gm_commercial then changeMusic mus_dm2int true
    else changeMusic mus_inter true
  checkForAccelerate
  let gs ← get
  let st := gs.wi.state
  if st == StatCount then
    if gs.deathmatchMode != 0 then updateDeathmatchStats
    else if gs.netgame then updateNetgameStats
    else updateStats
  else if st == ShowNextLoc then
    updateAnimatedBack
    let w := (← get).wi
    let c := w.cnt - 1
    modWi fun w => { w with cnt := c }
    if c == 0 || w.acceleratestage != 0 then initNoState
    else modWi fun w => { w with snlPointeron := (c &&& 31) < 20 }
  else if st == NoState then
    updateAnimatedBack
    let c := (← get).wi.cnt - 1
    modWi fun w => { w with cnt := c }
    if c == 0 then worldDone
  syncSp

end Doom.Playsim.Intermission
