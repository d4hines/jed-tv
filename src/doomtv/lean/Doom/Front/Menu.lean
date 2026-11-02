import Doom.Render.Hud
import Doom.Render.Types
import Doom.Render.Video
import Doom.Wad

/-!
# Doom.Front.Menu

The game menu (`m_menu.c`) as a pure state machine: `respond` handles one key,
`tick` animates the skull, `draw` paints the menu over a frame. Actions that
change the game come back as `Effect`s for the game loop.

Ported from Chocolate Doom with TV changes: in yes/no prompts OK (Enter) also
confirms and Back/Escape declines; Back steps up one menu level instead of
closing every menu; Load/Save and the mouse/detail options are omitted.
-/

namespace Doom.Front.Menu

open Doom.Render.Types
open Doom.Render.Video
open Doom.Wad

def KEY_ESCAPE : Nat := 27
def KEY_ENTER : Nat := 13
def KEY_BACKSPACE : Nat := 0x7f
def KEY_UPARROW : Nat := 0xad
def KEY_DOWNARROW : Nat := 0xaf
def KEY_LEFTARROW : Nat := 0xac
def KEY_RIGHTARROW : Nat := 0xae
def key_menu_confirm : Nat := 'y'.toNat
def key_menu_abort : Nat := 'n'.toNat

/-- `gamemode_t` as far as the menu cares. -/
inductive GameMode | shareware | registered | retail | commercial
  deriving BEq, Repr, Inhabited

/-- Identify the IWAD like `D_IdentifyVersion` does, from its map lumps. -/
def gameMode (wad : WadDirectory) : GameMode :=
  if (checkNumForName wad "MAP01").isSome then .commercial
  else if (checkNumForName wad "E4M1").isSome then .retail
  else if (checkNumForName wad "E3M1").isSome then .registered
  else .shareware

inductive MenuId | main | episode | skill | options | sound | readThis1 | readThis2 | gameType | load | save
  deriving BEq, Repr, Inhabited

def MenuId.index : MenuId → Nat
  | .main => 0 | .episode => 1 | .skill => 2 | .options => 3 | .sound => 4
  | .readThis1 => 5 | .readThis2 => 6 | .gameType => 7 | .load => 8 | .save => 9

/-- `menuitem_t`: status -1 none, 1 select, 2 left/right; patch; hotkey.
Items without a patch in the IWAD (our LAN game menu) show `text` instead. -/
structure Item where
  status : Int
  patch : String
  hotkey : Char := ' '
  text : String := ""

structure Def where
  items : Array Item
  prev : Option MenuId
  x : Int32
  y : Int32
  defaultOn : Nat := 0

def lineHeight : Int32 := 16
def skullXOff : Int32 := -32

/-- Options rows (`options_e` without detail/mouse). -/
def optEnd : Nat := 0
def optMessages : Nat := 1
def optScreenSize : Nat := 2
def optSoundVol : Nat := 4

def episodeItems : Array Item := #[
  { status := 1, patch := "M_EPI1", hotkey := 'k' },
  { status := 1, patch := "M_EPI2", hotkey := 't' },
  { status := 1, patch := "M_EPI3", hotkey := 'i' },
  { status := 1, patch := "M_EPI4", hotkey := 't' }]

/-- The six save slots (`LoadMenu` / `SaveMenu`). -/
def slotItems : Array Item := (List.range 6).toArray.map fun i =>
  { status := 1, patch := "", hotkey := Char.ofNat ('1'.toNat + i) }

def menuDef (mode : GameMode) : MenuId → Def
  | .main => { items := #[
      { status := 1, patch := "M_NGAME", hotkey := 'n' },
      { status := 1, patch := "M_OPTION", hotkey := 'o' },
      { status := 1, patch := "M_LOADG", hotkey := 'l' },
      { status := 1, patch := "M_SAVEG", hotkey := 's' },
      { status := 1, patch := "M_RDTHIS", hotkey := 'r' },
      { status := 1, patch := "M_QUITG", hotkey := 'q' }], prev := none, x := 97, y := 64 }
  | .episode =>
    let n := if mode == .retail then 4 else 3
    { items := episodeItems.extract 0 n, prev := some .gameType, x := 48, y := 63 }
  | .skill => { items := #[
      { status := 1, patch := "M_JKILL", hotkey := 'i' },
      { status := 1, patch := "M_ROUGH", hotkey := 'h' },
      { status := 1, patch := "M_HURT", hotkey := 'h' },
      { status := 1, patch := "M_ULTRA", hotkey := 'u' },
      { status := 1, patch := "M_NMARE", hotkey := 'n' }], prev := some (if mode == .commercial then .gameType else .episode), x := 48, y := 63, defaultOn := 2 }
  | .options => { items := #[
      { status := 1, patch := "M_ENDGAM", hotkey := 'e' },
      { status := 1, patch := "M_MESSG", hotkey := 'm' },
      { status := 2, patch := "M_SCRNSZ", hotkey := 's' },
      { status := -1, patch := "" },
      { status := 1, patch := "M_SVOL", hotkey := 's' }], prev := some .main, x := 60, y := 37 }
  | .sound => { items := #[
      { status := 2, patch := "M_SFXVOL", hotkey := 's' },
      { status := -1, patch := "" },
      { status := 2, patch := "M_MUSVOL", hotkey := 'm' },
      { status := -1, patch := "" }], prev := some .options, x := 80, y := 64 }
  | .gameType => { items := #[
      { status := 1, patch := "", text := "single player", hotkey := 's' },
      { status := 1, patch := "", text := "host co-op lan game", hotkey := 'c' },
      { status := 1, patch := "", text := "host deathmatch lan game", hotkey := 'd' },
      { status := 1, patch := "", text := "join lan game", hotkey := 'j' }], prev := some .main, x := 72, y := 72 }
  | .load => { items := slotItems, prev := some .main, x := 80, y := 54 }
  | .save => { items := slotItems, prev := some .main, x := 80, y := 54 }
  | .readThis1 => { items := #[{ status := 1, patch := "" }], prev := some .main, x := 280, y := 185 }
  | .readThis2 => { items := #[{ status := 1, patch := "" }], prev := some .readThis1, x := 330, y := 175 }

/-- Player-adjustable settings the menu owns (`m_menu.c` / `m_config.c`). -/
structure Settings where
  sfxVolume : Nat := 8
  musicVolume : Nat := 8
  showMessages : Bool := true
  /-- `screenblocks` 3..11 (11 = full screen, no status bar). -/
  screenblocks : Nat := 9
  deriving Repr, BEq

/-- What a prompt does when answered. -/
inductive Prompt | info | quit | endGame | nightmare
  deriving BEq, Repr, Inhabited

/-- Effects for the game loop to perform. -/
inductive Effect
  | sound (sfx : Nat)
  | newGame (skill episode : Nat)
  /-- Open a LAN lobby as host: 1 co-op, 2 deathmatch. -/
  | hostGame (mode skill episode : Nat)
  /-- Save / load the game in slot 0..5 (kept in memory for this session). -/
  | saveGame (slot : Nat)
  | loadGame (slot : Nat)
  /-- Look for a LAN game to join. -/
  | joinGame
  | endGame
  | quit
  | message (text : String)
  deriving Repr, BEq

structure State where
  active : Bool := false
  current : MenuId := .main
  itemOn : Nat := 0
  lastOn : Array Nat := #[0, 0, 2, 0, 0, 0, 0, 0, 0, 0]
  skullCounter : Nat := 10
  whichSkull : Nat := 0
  /-- Message box text, its handler, and whether it waits for an answer. -/
  message : Option (String × Prompt × Bool) := none
  messageReturnsToMenu : Bool := false
  episode : Nat := 0
  /-- Chosen game type: 0 single player, 1 host co-op, 2 host deathmatch. -/
  netMode : Nat := 0
  /-- `savegamestrings`: description per slot, "" when empty. -/
  slotNames : Array String := Array.replicate 6 ""
  settings : Settings := {}
  deriving Inhabited

/-- What the menu needs to know about the game. -/
structure Context where
  mode : GameMode
  /-- A level is being played (not the title or a demo). -/
  usergame : Bool
  netgame : Bool
  gametic : Nat

/-- `sounds.h` ids used by the menu. -/
def sfx_pistol : Nat := 1
def sfx_pstop : Nat := 19
def sfx_swtchn : Nat := 23
def sfx_swtchx : Nat := 24
def sfx_stnmov : Nat := 22
def sfx_oof : Nat := 34

def pressYN : String := "press ok or back."
def pressKey : String := "press a key."

def doom1EndMsg : Array String := #[
  "are you sure you want to\nquit this great game?",
  "please don't leave, there's more\ndemons to toast!",
  "let's beat it -- this is turning\ninto a bloodbath!",
  "i wouldn't leave if i were you.\nyour tv is much worse.",
  "you're trying to say you like tv\nbetter than me, right?",
  "don't leave yet -- there's a\ndemon around that corner!",
  "ya know, next time you come in here\ni'm gonna toast ya.",
  "go ahead and leave. see if i care."]

def doom2EndMsg : Array String := #[
  "are you sure you want to\nquit this great game?",
  "you want to quit?\nthen, thou hast lost an eighth!",
  "don't go now, there's a \ndimensional shambler waiting\nat the tv!",
  "get outta here and go back\nto your boring programs.",
  "if i were your boss, i'd \n deathmatch ya in a minute!",
  "look, bud. you leave now\nand you forfeit your body count!",
  "just leave. when you come\nback, i'll be waiting with a bat.",
  "you're lucky i don't smack\nyou for thinking about leaving."]

private def startMessage (st : State) (text : String) (p : Prompt) (input : Bool) : State :=
  { st with message := some (text, p, input), messageReturnsToMenu := st.active, active := true }

private def setupNext (st : State) (mode : GameMode) (m : MenuId) : State :=
  let _ := mode
  { st with current := m, itemOn := st.lastOn[m.index]?.getD 0 }

/-- `M_StartControlPanel`. -/
def open_ (st : State) : State :=
  if st.active then st else { st with active := true, current := .main, itemOn := st.lastOn[0]?.getD 0 }

/-- `M_ClearMenus`. -/
def close (st : State) : State := { st with active := false }

private def rememberOn (st : State) : State :=
  { st with lastOn := st.lastOn.set! st.current.index st.itemOn }

/-- Run a menu item's routine (`menuitems[itemOn].routine(choice)`). -/
private def activate (st : State) (ctx : Context) (choice : Nat) : State × Array Effect :=
  let s := st.settings
  match st.current, st.itemOn with
  | .main, 0 =>
    if ctx.netgame then
      (startMessage st s!"you can't start a new game\nwhile in a network game.\n\n{pressKey}" .info false, #[])
    else (setupNext st ctx.mode .gameType, #[])
  | .gameType, 3 => (close st, #[.joinGame])
  | .gameType, k =>
    (setupNext { st with netMode := k } ctx.mode (if ctx.mode == .commercial then .skill else .episode), #[])
  | .main, 1 => (setupNext st ctx.mode .options, #[])
  | .main, 2 =>
    if ctx.netgame then
      (startMessage st s!"you can't do load while in a net game!\n\n{pressKey}" .info false, #[])
    else (setupNext st ctx.mode .load, #[])
  | .main, 3 =>
    if !ctx.usergame || ctx.netgame then
      (startMessage st s!"you can't save if you aren't playing!\n\n{pressKey}" .info false, #[])
    else (setupNext st ctx.mode .save, #[])
  | .main, 4 => (setupNext st ctx.mode .readThis1, #[])
  | .load, k => if (st.slotNames[k]?.getD "") == "" then (st, #[]) else (close st, #[.loadGame k])
  | .save, k => (close st, #[.saveGame k])
  | .main, _ =>
    let msgs := if ctx.mode == .commercial then doom2EndMsg else doom1EndMsg
    (startMessage st s!"{msgs[ctx.gametic % msgs.size]!}\n\n(press ok to quit.)" .quit true, #[])
  | .episode, e =>
    if ctx.mode == .shareware && e != 0 then
      (startMessage (setupNext st ctx.mode .readThis1)
        s!"this is the shareware version of doom.\n\nyou need to order the entire trilogy.\n\n{pressKey}" .info false, #[])
    else (setupNext { st with episode := e } ctx.mode .skill, #[])
  | .skill, 4 =>
    (startMessage st s!"are you sure? this skill level\nisn't even remotely fair.\n\n{pressYN}" .nightmare true, #[])
  | .skill, k =>
    if st.netMode == 0 then (close st, #[.newGame k (st.episode + 1)])
    else (close st, #[.hostGame st.netMode k (st.episode + 1)])
  | .options, 0 =>
    if !ctx.usergame then (st, #[.sound sfx_oof])
    else if ctx.netgame then (startMessage st s!"you can't end a netgame!\n\n{pressKey}" .info false, #[])
    else (startMessage st s!"are you sure you want to end the game?\n\n{pressYN}" .endGame true, #[])
  | .options, 1 =>
    let on := !s.showMessages
    ({ st with settings := { s with showMessages := on } }, #[.message (if on then "Messages ON" else "Messages OFF")])
  | .options, 2 =>
    let b := if choice == 0 then (if s.screenblocks > 3 then s.screenblocks - 1 else 3)
      else (if s.screenblocks < 11 then s.screenblocks + 1 else 11)
    ({ st with settings := { s with screenblocks := b } }, #[])
  | .options, _ => (setupNext st ctx.mode .sound, #[])
  | .sound, i =>
    let adj (v : Nat) := if choice == 0 then v - 1 else min 15 (v + 1)
    if i == 0 then ({ st with settings := { s with sfxVolume := adj s.sfxVolume } }, #[])
    else ({ st with settings := { s with musicVolume := adj s.musicVolume } }, #[])
  | .readThis1, _ =>
    (setupNext st ctx.mode (if ctx.mode == .commercial then .main else .readThis2), #[])
  | .readThis2, _ => (setupNext st ctx.mode .main, #[])

/-- Answer to a message box (`messageRoutine(key)`). -/
private def answer (st : State) (p : Prompt) (yes : Bool) (ctx : Context) : State × Array Effect :=
  match p, yes with
  | .quit, true => (st, #[.quit])
  | .endGame, true => (close st, #[.endGame])
  | .nightmare, true =>
    (close st, #[if st.netMode == 0 then .newGame 4 (st.episode + 1) else .hostGame st.netMode 4 (st.episode + 1)])
  | _, _ => let _ := ctx; (st, #[])

/-- `M_Responder` for one key press (or repeat). Returns whether the menu ate
the key; unhandled keys go on to the game. -/
def respond (st : State) (ctx : Context) (key : Nat) : State × Array Effect × Bool :=
  match st.message with
  | some (_, p, needsInput) =>
    let isYes := key == key_menu_confirm || key == KEY_ENTER
    let isNo := key == key_menu_abort || key == KEY_ESCAPE || key == KEY_BACKSPACE
    if needsInput && !(isYes || isNo || key == ' '.toNat) then (st, #[], false)
    else
      let st' := { st with message := none, active := st.messageReturnsToMenu }
      let (st'', fx) := answer st' p (needsInput && isYes) ctx
      (st'', fx.push (.sound sfx_swtchx), true)
  | none =>
    if !st.active then
      if key == KEY_ESCAPE then (open_ st, #[.sound sfx_swtchn], true) else (st, #[], false)
    else
      let d := menuDef ctx.mode st.current
      let n := d.items.size
      let status (i : Nat) := (d.items[i]?.map (·.status)).getD (-1)
      let rec step (i : Nat) (down : Bool) (fuel : Nat) : Nat :=
        let j := if down then (if i + 1 ≥ n then 0 else i + 1) else (if i == 0 then n - 1 else i - 1)
        match fuel with
        | 0 => j
        | f + 1 => if status j == -1 then step j down f else j
      if key == KEY_DOWNARROW then ({ st with itemOn := step st.itemOn true n }, #[.sound sfx_pstop], true)
      else if key == KEY_UPARROW then ({ st with itemOn := step st.itemOn false n }, #[.sound sfx_pstop], true)
      else if key == KEY_LEFTARROW || key == KEY_RIGHTARROW then
        if status st.itemOn == 2 then
          let (s', fx) := activate st ctx (if key == KEY_RIGHTARROW then 1 else 0)
          (s', #[.sound sfx_stnmov] ++ fx, true)
        else (st, #[], true)
      else if key == KEY_ENTER then
        if status st.itemOn == 2 then
          let (s', fx) := activate (rememberOn st) ctx 1
          (s', #[.sound sfx_stnmov] ++ fx, true)
        else if status st.itemOn == 1 then
          let (s', fx) := activate (rememberOn st) ctx st.itemOn
          (s', #[.sound sfx_pistol] ++ fx, true)
        else (st, #[], true)
      else if key == KEY_ESCAPE || key == KEY_BACKSPACE then
        -- Remote Back: one level up; closes from the main menu.
        let st := rememberOn st
        match d.prev with
        | some p => (setupNext st ctx.mode p, #[.sound sfx_swtchn], true)
        | none => (close st, #[.sound sfx_swtchx], true)
      else
        -- Hotkeys: first item after the cursor, wrapping (`alphaKey`).
        let idx := (List.range n).map (fun k => (st.itemOn + 1 + k) % n)
        match idx.find? (fun i => (d.items[i]?.map (·.hotkey.toNat)) == some key) with
        | some i => ({ st with itemOn := i }, #[.sound sfx_pstop], true)
        | none => (st, #[], true)

/-- `M_Ticker`: blink the skull. -/
def tick (st : State) : State :=
  if st.skullCounter ≤ 1 then { st with skullCounter := 8, whichSkull := 1 - st.whichSkull }
  else { st with skullCounter := st.skullCounter - 1 }

private def patch (wad : WadDirectory) (fb : Framebuffer) (name : String) (x y : Int32) :
    Except String Framebuffer :=
  match checkNumForName wad name with
  | some i => drawPatch wad fb i x y
  | none => pure fb

/-- `M_DrawThermo`. -/
private def thermo (wad : WadDirectory) (fb : Framebuffer) (x y : Int32) (width dot : Nat) :
    Except String Framebuffer := do
  let mut fb ← patch wad fb "M_THERML" x y
  let mut xx := x + 8
  for _ in [0:width] do
    fb ← patch wad fb "M_THERMM" xx y
    xx := xx + 8
  fb ← patch wad fb "M_THERMR" xx y
  patch wad fb "M_THERMO" (x + 8 + Int32.ofNat (dot * 8)) y

private def charWidth (wad : WadDirectory) (c : Char) : Int32 :=
  let n := c.toUpper.toNat
  if n < Doom.Render.Hud.HU_FONTSTART || n > Doom.Render.Hud.HU_FONTEND then 4
  else match checkNumForName wad (Doom.Render.Hud.stcfnName n) with
    | some i => match wad.entries[i]? with
      | some e => match wad.data[e.filepos.toNat]?, wad.data[e.filepos.toNat + 1]? with
        | some lo, some hi => (lo.toUInt32 ||| (hi.toUInt32 <<< 8)).toInt32
        | _, _ => 4
      | none => 4
    | none => 4

/-- `M_StringWidth`. -/
def stringWidth (wad : WadDirectory) (s : String) : Int32 :=
  s.foldl (fun w c => w + charWidth wad c) 0

/-- The centered message box (`M_Drawer` with `messageToPrint`). -/
private def drawMessage (wad : WadDirectory) (fb : Framebuffer) (text : String) :
    Except String Framebuffer := do
  let lines := text.splitOn "\n"
  let mut y : Int32 := Int32.ofNat screenHeight / 2 - Int32.ofNat (lines.length * 8) / 2
  let mut fb := fb
  for l in lines do
    let x := Int32.ofNat screenWidth / 2 - stringWidth wad l / 2
    fb ← Doom.Render.Hud.drawTextLine wad fb x y l
    y := y + 8
  pure fb

/-- `M_Drawer`: the active menu or message box over `fb`. -/
def draw (st : State) (mode : GameMode) (wad : WadDirectory) (fb : Framebuffer) :
    Except String Framebuffer := do
  if let some (text, _, _) := st.message then
    return (← drawMessage wad fb text)
  if !st.active then return fb
  let d := menuDef mode st.current
  let s := st.settings
  let mut fb := fb
  match st.current with
  | .main => fb ← patch wad fb "M_DOOM" 94 2
  | .episode => fb ← patch wad fb "M_EPISOD" 54 38
  | .skill =>
    fb ← patch wad fb "M_NEWG" 96 14
    fb ← patch wad fb "M_SKILL" 54 38
  | .options =>
    fb ← patch wad fb "M_OPTTTL" 108 15
    fb ← patch wad fb (if s.showMessages then "M_MSGON" else "M_MSGOFF") (d.x + 120) (d.y + lineHeight * Int32.ofNat optMessages)
    fb ← thermo wad fb d.x (d.y + lineHeight * Int32.ofNat (optScreenSize + 1)) 9 (s.screenblocks - 3)
  | .sound =>
    fb ← patch wad fb "M_SVOL" 60 38
    fb ← thermo wad fb d.x (d.y + lineHeight) 16 s.sfxVolume
    fb ← thermo wad fb d.x (d.y + lineHeight * 3) 16 s.musicVolume
  | .load | .save =>
    fb ← patch wad fb (if st.current == .load then "M_LOADG" else "M_SAVEG") 72 28
    for i in [0:6] do
      let y := d.y + lineHeight * Int32.ofNat i
      -- `M_DrawSaveLoadBorder`
      fb ← patch wad fb "M_LSLEFT" (d.x - 8) (y + 7)
      for k in [0:24] do fb ← patch wad fb "M_LSCNTR" (d.x + 8 * Int32.ofNat k) (y + 7)
      fb ← patch wad fb "M_LSRGHT" (d.x + 8 * 24) (y + 7)
      let name := st.slotNames[i]?.getD ""
      fb ← Doom.Render.Hud.drawTextLine wad fb d.x y (if name == "" then "empty slot" else name)
  | .gameType =>
    fb ← patch wad fb "M_NEWG" 96 14
    fb ← Doom.Render.Hud.drawTextLine wad fb 72 48 "choose game type:"
  | .readThis1 => fb ← patch wad fb (if mode == .commercial then "HELP" else "HELP1") 0 0
  | .readThis2 => fb ← patch wad fb (if mode == .retail then "CREDIT" else "HELP2") 0 0
  let mut y := d.y
  for item in d.items do
    if item.patch != "" then fb ← patch wad fb item.patch d.x y
    else if item.text != "" then fb ← Doom.Render.Hud.drawTextLine wad fb d.x (y + 4) item.text
    y := y + lineHeight
  if st.current == .readThis1 || st.current == .readThis2 then return fb
  patch wad fb (if st.whichSkull == 0 then "M_SKULL1" else "M_SKULL2")
    (d.x + skullXOff) (d.y - 5 + Int32.ofNat st.itemOn * lineHeight)

end Doom.Front.Menu
