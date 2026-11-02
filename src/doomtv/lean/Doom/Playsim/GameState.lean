import Doom.Playsim.NameMap
import Doom.Playsim.Level
import Doom.Playsim.Random
import Doom.Playsim.Mobj
import Doom.Playsim.Player
import Doom.Playsim.Thinker
import Doom.Playsim.Info
import Doom.Playsim.Weapons
import Doom.Wad

/-!
# Doom.Playsim.GameState

The whole game: level geometry, runtime sectors, players, mobjs, special
thinkers, the C file-scope globals of the playsim (`tm*`, `spechit`,
`linetarget`, `opentop`, intercepts, ...) and the `g_game.c` / `wi_stuff.c` /
`f_finale.c` state. Keeping the C globals as fields makes nested calls
clobber them exactly as they do in Chocolate Doom.

All updates go through the state monad `M`, so the state stays uniquely
referenced and array writes are in place.
-/

namespace Doom.Playsim.GameState

open Doom.Playsim.Level
open Doom.Playsim.Random
open Doom.Playsim.Mobj
open Doom.Playsim.Player
open Doom.Playsim.Thinker
open Doom.Playsim.Info
open Doom.Wad

instance : Inhabited Doom.Playsim.Weapons.WeaponInfo := ⟨{
  ammo := 0, upstate := 0, downstate := 0,
  readystate := 0, atkstate := 0, flashstate := 0 }⟩
instance : Inhabited Psprite := ⟨Psprite.inactive⟩
instance : Inhabited State := ⟨{
  sprite := 0, frame := 0, tics := 0, action := 0, nextstate := 0,
  misc1 := 0, misc2 := 0 }⟩
instance : Inhabited MobjInfo := ⟨{
  doomednum := 0, spawnstate := 0, spawnhealth := 0, seestate := 0,
  seesound := 0, reactiontime := 0, attacksound := 0, painstate := 0, painchance := 0, painsound := 0,
  meleestate := 0, missilestate := 0, deathstate := 0, xdeathstate := 0, deathsound := 0, speed := 0,
  radius := 0, height := 0, mass := 1, damage := 0, activesound := 0, flags := 0, raisestate := 0 }⟩

/-- Skill ordinals (`skill_t`). -/
def sk_baby : Int32 := 0
def sk_easy : Int32 := 1
def sk_medium : Int32 := 2
def sk_hard : Int32 := 3
def sk_nightmare : Int32 := 4

/-- `GameMode_t` (`d_mode.h`). -/
def gm_shareware : Nat := 0
def gm_registered : Nat := 1
def gm_commercial : Nat := 2
def gm_retail : Nat := 3

/-- `GameVersion_t` ordinals (`d_mode.h`). -/
def exe_doom_1_2 : Nat := 0
def exe_doom_1_5 : Nat := 1
def exe_doom_1_666 : Nat := 2
def exe_doom_1_7 : Nat := 3
def exe_doom_1_8 : Nat := 4
def exe_doom_1_9 : Nat := 5
def exe_hacx : Nat := 6
def exe_ultimate : Nat := 7
def exe_final : Nat := 8
def exe_final2 : Nat := 9
def exe_chex : Nat := 10

/-- Sector runtime overlay (mutable light / special / thinglist / specialdata). -/
structure SectorRuntime where
  lightlevel : Int32
  special : Int32
  tag : Int32
  floorheight : Int32
  ceilingheight : Int32
  /-- Head of sector thinglist (`mobj` index), or `-1`. -/
  thinglist : Int32
  /-- `specialdata`: thinker reference `kind * 2^26 + idx`, or `-1` (NULL). -/
  specialdata : Int32
  /-- Sector `soundtarget` mobj index, or `-1`. -/
  soundtarget : Int32
  soundtraversed : Int32
  validcount : Nat
  /-- Line indices (copied from geometry). -/
  lines : Array UInt32
  /-- Mutable floor flat name (renderer view of `floorpicNum`). -/
  floorpic : ByteArray
  /-- `floorpic` flat number. -/
  floorpicNum : Int32 := 0
  /-- `ceilingpic` flat number. -/
  ceilingpicNum : Int32 := 0

structure LightFlash where
  traceId : UInt32
  sector : UInt32
  count : Int32
  maxlight : Int32
  minlight : Int32
  maxtime : Int32
  mintime : Int32

structure StrobeFlash where
  traceId : UInt32
  sector : UInt32
  count : Int32
  minlight : Int32
  maxlight : Int32
  darktime : Int32
  brighttime : Int32

structure Glow where
  traceId : UInt32
  sector : UInt32
  minlight : Int32
  maxlight : Int32
  direction : Int32

structure FireFlicker where
  traceId : UInt32
  sector : UInt32
  count : Int32
  maxlight : Int32
  minlight : Int32

/-- Static per-line geometry with scalar fields (hot-path copy of `Line`
plus its vertices). -/
structure LineGeo where
  v1x : Int32
  v1y : Int32
  v2x : Int32
  v2y : Int32
  dx : Int32
  dy : Int32
  bbTop : Int32
  bbBottom : Int32
  bbLeft : Int32
  bbRight : Int32
  flags : Int32
  slopetype : UInt32
  front : Int32
  back : Int32
  sidenum1 : Int32
  deriving Inhabited

/-- `vldoor_t`. -/
structure VerticalDoor where
  traceId : UInt32
  func : UInt32 := THF_VERTICALDOOR
  type_ : Int32
  sector : UInt32
  topheight : Int32
  speed : Int32
  direction : Int32
  topwait : Int32
  topcountdown : Int32

/-- `ceiling_t`. -/
structure Ceiling where
  traceId : UInt32
  func : UInt32 := THF_MOVECEILING
  type_ : Int32
  sector : UInt32
  bottomheight : Int32
  topheight : Int32
  speed : Int32
  crush : Int32
  direction : Int32
  tag : Int32
  olddirection : Int32

/-- `floormove_t`. `texture` is a C `short`; `texPad` models the two padding
bytes that follow it on the 64-bit oracle (see `EV_VerticalDoor`). -/
structure FloorMove where
  traceId : UInt32
  func : UInt32 := THF_MOVEFLOOR
  type_ : Int32
  crush : Int32
  sector : UInt32
  direction : Int32
  newspecial : Int32
  texture : Int32
  texPad : Int32 := 0
  floordestheight : Int32
  speed : Int32

/-- `plat_t`. -/
structure Plat where
  traceId : UInt32
  func : UInt32 := THF_PLATRAISE
  sector : UInt32
  speed : Int32
  low : Int32
  high : Int32
  wait : Int32
  count : Int32
  status : Int32
  oldstatus : Int32
  crush : Int32
  tag : Int32
  type_ : Int32

/-- `button_t`. `soundorg` is the sector whose `soundorg` the C pointer
targets, or `-1` for NULL. -/
structure Button where
  line : Int32 := -1
  where_ : Int32 := 0
  btexture : Int32 := 0
  btimer : Int32 := 0
  soundorg : Int32 := -1
  deriving Inhabited

/-- `mapthing_t` (map units, C `short` fields). -/
structure MapThing where
  x : Int32 := 0
  y : Int32 := 0
  angle : Int32 := 0
  type_ : Int32 := 0
  options : Int32 := 0
  deriving Inhabited, Repr

/-- Untraced `ST_Ticker` face widget state (`st_stuff.c` statics). -/
structure StFaceState where
  faceindex : Int32
  facecount : Int32
  oldhealth : Int32
  oldweaponsowned : Array Int32
  lastattackdown : Int32
  priority : Int32
  painOffset : Int32
  painOldhealth : Int32
  spawnWeaponsOwned : Array Int32

/-- Untraced `anim_t` (`p_spec.c`). -/
structure PicAnim where
  istexture : Bool
  picnum : Int32
  basepic : Int32
  numpics : Int32
  speed : Int32
  deriving Repr

def StFaceState.init : StFaceState := {
  faceindex := 0
  facecount := 0
  oldhealth := -1
  oldweaponsowned := Array.replicate NUMWEAPONS 0
  lastattackdown := -1
  priority := 0
  painOffset := 0
  painOldhealth := -1
  spawnWeaponsOwned := Array.replicate NUMWEAPONS 0
}

/-- Untraced `HU_Ticker` message widget (`hu_stuff.c` statics). -/
structure HuState where
  messageOn : Bool
  messageCounter : Int32
  messageText : String

def HuState.init : HuState := {
  messageOn := false
  messageCounter := 0
  messageText := ""
}

/-- One WI background anim (`wi_stuff.c` `anim_t`, logic fields). -/
structure WiAnim where
  type_ : Int32
  period : Int32
  nanims : Int32
  data1 : Int32 := 0
  data2 : Int32 := 0
  ctr : Int32
  nexttic : Int32
  deriving Repr, Inhabited

/-- `wbplayerstruct_t`. -/
structure WbPlayer where
  in_ : Bool := false
  skills : Int32 := 0
  sitems : Int32 := 0
  ssecret : Int32 := 0
  stime : Int32 := 0
  frags : Array Int32 := Array.replicate 4 0
  deriving Inhabited, Repr

/-- `wbstartstruct_t`. -/
structure WbStart where
  epsd : Int32 := 0
  didsecret : Bool := false
  last : Int32 := 0
  next : Int32 := 0
  maxkills : Int32 := 0
  maxitems : Int32 := 0
  maxsecret : Int32 := 0
  maxfrags : Int32 := 0
  partime : Int32 := 0
  pnum : Int32 := 0
  plyr : Array WbPlayer := Array.replicate 4 {}
  deriving Inhabited, Repr

/-- `wi_stuff.c` statics. The SP counters `cntKills`.. mirror `cnt_*[me]`
for the renderer. -/
structure WiState where
  state : Int32
  acceleratestage : Int32
  bcnt : Int32
  cnt : Int32
  spState : Int32
  cntPause : Int32
  cntKills : Int32
  cntItems : Int32
  cntSecret : Int32
  cntTime : Int32
  cntPar : Int32
  maxkills : Int32
  maxitems : Int32
  maxsecret : Int32
  partime : Int32
  next : Int32
  last : Int32
  epsd : Int32
  skills : Int32
  sitems : Int32
  ssecret : Int32
  stime : Int32
  anims : Array WiAnim
  me : Int32 := 0
  ngState : Int32 := 0
  dmState : Int32 := 0
  dofrags : Int32 := 0
  snlPointeron : Bool := false
  cntKillsA : Array Int32 := Array.replicate 4 0
  cntItemsA : Array Int32 := Array.replicate 4 0
  cntSecretA : Array Int32 := Array.replicate 4 0
  cntFrags : Array Int32 := Array.replicate 4 0
  dmFrags : Array (Array Int32) := Array.replicate 4 (Array.replicate 4 0)
  dmTotals : Array Int32 := Array.replicate 4 0
  deriving Repr

def WiState.empty : WiState := {
  state := 0, acceleratestage := 0, bcnt := 0, cnt := 0, spState := 0
  cntPause := 0, cntKills := -1, cntItems := -1, cntSecret := -1
  cntTime := -1, cntPar := -1, maxkills := 1, maxitems := 1, maxsecret := 1
  partime := 0, next := 0, last := 0, epsd := 0, skills := 0, sitems := 0
  ssecret := 0, stime := 0, anims := #[]
}

/-- `f_finale.c` statics (logic only; text and pictures are the front end's). -/
structure FinaleState where
  finalestage : Int32 := 0
  finalecount : Int32 := 0
  /-- Finale text id (`F_StartFinale` screen table index; front end resolves). -/
  textIndex : Int32 := 0
  castnum : Int32 := 0
  casttics : Int32 := 0
  caststate : UInt32 := 0
  castdeath : Bool := false
  castframes : Int32 := 0
  castonmelee : Int32 := 0
  castattacking : Bool := false
  deriving Inhabited, Repr

/-- Origin of a sound event. -/
inductive SoundOrigin where
  /-- `S_StartSound(NULL, ...)`: full volume, no position. -/
  | none
  /-- A map object (index into `mobjs`) at its position when the sound starts. -/
  | mobj (index : UInt32) (x y : Int32)
  /-- A sector's `soundorg` (doors, lifts, switches). -/
  | sector (x y : Int32)
  deriving Repr, BEq, Inhabited

/-- Per-tic audio event, appended at every `S_StartSound` / `S_StopSound` /
`S_ChangeMusic` / `S_StartMusic` call site. -/
inductive SoundEvent where
  /-- `S_StartSound`; `audible` is whether the oracle would play it for the
  console player (it then also drew the pitch `M_Random`). -/
  | start (sfx : Nat) (origin : SoundOrigin) (audible : Bool)
  /-- `S_StopSound(mobj)`. -/
  | stop (origin : SoundOrigin)
  /-- `S_ChangeMusic` / `S_StartMusic` (`mus_*` ordinal). -/
  | music (mus : Nat) (looping : Bool)
  deriving Repr, BEq, Inhabited

/-- Static per-game resources (WAD-derived tables, game mode). Never
modified during play. -/
structure Res where
  wad : WadDirectory
  gamemode : Nat
  gameversion : Nat
  /-- Upper-cased 8-byte texture names in `TEXTURE1`+`TEXTURE2` order. -/
  textureNames : Array ByteArray
  /-- `textureheight[]` (fixed_t). -/
  textureHeights : Array Int32
  /-- Flat names in `F_START..F_END` lump order (index = flat number). -/
  flatNames : Array ByteArray
  firstflat : Nat
  skyflatnum : Int32
  /-- `switchlist[]` texture numbers, pairs (off, on). -/
  switchlist : Array Int32
  picAnims : Array PicAnim
  /-- `map31` exists (`G_SecretExitLevel` commercial check). -/
  hasMap31 : Bool
  /-- DEHACKED / BEX string replacements (original, replacement). -/
  dehStrings : Array (String × String) := #[]
  /-- BEX `[STRINGS]` as given (mnemonic, value), for the front end. -/
  dehBex : Array (String × String) := #[]
  /-- Upper-cased texture name to first texture number. -/
  texIdx : Doom.Playsim.NameMap := {}
  /-- Upper-cased lump name to last lump index (`W_CheckNumForName`). -/
  lumpIdx : Doom.Playsim.NameMap := {}

instance : Inhabited Res := ⟨{
  wad := { identification := ByteArray.empty, numlumps := 0, infotableofs := 0,
           entries := #[], data := ByteArray.empty }
  gamemode := gm_shareware, gameversion := exe_doom_1_9
  textureNames := #[], textureHeights := #[], flatNames := #[], firstflat := 0
  skyflatnum := 0, switchlist := #[], picAnims := #[], hasMap31 := false }⟩

structure GameState where
  res : Res
  /-- Mutable `states[]` (DEHACKED frames, fast-monster tic halving). -/
  states : Array State
  /-- Mutable `mobjinfo[]` (fast-missile speeds). -/
  mobjinfo : Array MobjInfo
  level : LevelData
  sectors : Array SectorRuntime
  /-- Scalar line geometry (`lines[]` without the mutable special). -/
  lineGeo : Array LineGeo := #[]
  /-- Side texture numbers; names live in `level.sides` for the renderer. -/
  sideTop : Array Int32 := #[]
  sideMid : Array Int32 := #[]
  sideBottom : Array Int32 := #[]
  players : Array Player
  playeringame : Array Bool
  mobjs : Array Mobj
  /-- Thinker list in C order (`Thinker.thRef` encoding). -/
  thinkers : Array Nat
  lightFlashes : Array LightFlash := #[]
  strobes : Array StrobeFlash := #[]
  glows : Array Glow := #[]
  fireFlickers : Array FireFlicker := #[]
  verticalDoors : Array VerticalDoor := #[]
  ceilings : Array Ceiling := #[]
  floors : Array FloorMove := #[]
  plats : Array Plat := #[]
  /-- `activeplats[MAXPLATS]`: `plats` index + 1, 0 = NULL. -/
  activePlats : Array Nat := Array.replicate 30 0
  /-- `activeceilings[MAXCEILINGS]`: `ceilings` index + 1, 0 = NULL. -/
  activeCeilings : Array Nat := Array.replicate 30 0
  buttons : Array Button := Array.replicate 16 {}
  rng : RandomState
  traceIdCounter : UInt32
  leveltime : UInt32
  gametic : UInt32
  -- Demo playback.
  demoBytes : ByteArray := ByteArray.empty
  demoCursor : Nat := 0
  demoplayback : Bool := false
  /-- Set when the demo marker was read: the oracle quits at this point. -/
  demoEnded : Bool := false
  -- Game options (`g_game.c` / `doomstat.c`).
  gameskill : Int32
  consoleplayer : Nat
  displayplayer : Nat := 0
  /-- Renderer view: `deathmatchMode != 0`. -/
  deathmatch : Bool
  deathmatchMode : Int32 := 0
  nomonsters : Bool
  netgame : Bool
  respawnparm : Bool := false
  fastparm : Bool := false
  respawnmonsters : Bool := false
  /-- Renderer view: `res.gamemode == commercial`. -/
  commercial : Bool
  paused : Bool := false
  menuactive : Bool := false
  automapactive : Bool := false
  viewactive : Bool := true
  usergame : Bool := false
  timelimit : Int32 := 0
  totalkills : Int32
  totalitems : Int32
  totalsecret : Int32
  levelstarttic : UInt32 := 0
  turbodetected : Array Bool := Array.replicate 4 false
  -- Global `validcount` and per-line stamps.
  validcount : Nat
  lineValidcount : Array Nat
  /-- `blocklinks`: mobj index + 1, 0 = empty. -/
  blocklinks : Array Nat
  /-- Blockmap lump (signed shorts, little endian). -/
  bmapLump : ByteArray := ByteArray.empty
  bmaporgx : Int32 := 0
  bmaporgy : Int32 := 0
  bmapwidth : Int32 := 0
  bmapheight : Int32 := 0
  /-- `rejectmatrix`, padded as Chocolate Doom pads short lumps. -/
  reject : ByteArray := ByteArray.empty
  stFace : StFaceState
  hu : HuState
  picAnims : Array PicAnim := #[]
  flattranslation : Array Int32 := #[]
  texturetranslation : Array Int32 := #[]
  gameaction : Int32 := 0
  secretexit : Bool := false
  /-- `linespeciallist` line indices (special 48). -/
  lineSpecials : Array Nat := #[]
  levelTimer : Bool := false
  levelTimeCount : Int32 := 0
  gamestate : Int32 := 0
  oldgamestate : Int32 := -1
  /-- `d_main.c` `wipegamestate` (display wipe emulation). -/
  wipegamestate : Int32 := 3
  gameepisode : Int32 := 1
  gamemap : Int32 := 1
  wminfo : WbStart := {}
  wi : WiState := WiState.empty
  finale : FinaleState := {}
  -- p_setup.c
  deathmatchstarts : Array MapThing := #[]
  playerstarts : Array MapThing := Array.replicate 4 {}
  playerstartsingame : Array Bool := Array.replicate 4 false
  -- p_mobj.c item respawn queue
  itemrespawnque : Array MapThing := Array.replicate 128 {}
  itemrespawntime : Array Int32 := Array.replicate 128 0
  iquehead : Nat := 0
  iquetail : Nat := 0
  -- g_game.c body queue: mobj index + 1, 0 = NULL
  bodyque : Array Nat := Array.replicate 32 0
  bodyqueslot : Nat := 0
  -- p_enemy.c
  soundtarget : Int32 := -1
  corpsehit : Int32 := -1
  vileobj : Int32 := -1
  viletryx : Int32 := 0
  viletryy : Int32 := 0
  braintargets : Array Nat := #[]
  braintargeton : Nat := 0
  /-- `static int easy` in `A_BrainSpit` (never reset). -/
  brainEasy : Int32 := 0
  -- p_map.c
  tmbboxTop : Int32 := 0
  tmbboxBottom : Int32 := 0
  tmbboxLeft : Int32 := 0
  tmbboxRight : Int32 := 0
  tmthing : Int32 := -1
  tmflags : UInt32 := 0
  tmx : Int32 := 0
  tmy : Int32 := 0
  floatok : Bool := false
  tmfloorz : Int32 := 0
  tmceilingz : Int32 := 0
  tmdropoffz : Int32 := 0
  ceilingline : Int32 := -1
  spechit : Array Nat := Array.replicate 20 0
  numspechit : Int32 := 0
  bestslidefrac : Int32 := 0
  secondslidefrac : Int32 := 0
  bestslideline : Int32 := -1
  secondslideline : Int32 := -1
  slidemo : Int32 := -1
  tmxmove : Int32 := 0
  tmymove : Int32 := 0
  linetarget : Int32 := -1
  shootthing : Int32 := -1
  shootz : Int32 := 0
  laDamage : Int32 := 0
  attackrange : Int32 := 0
  aimslope : Int32 := 0
  topslope : Int32 := 0
  bottomslope : Int32 := 0
  usething : Int32 := -1
  bombsource : Int32 := -1
  bombspot : Int32 := -1
  bombdamage : Int32 := 0
  crushchange : Int32 := 0
  nofit : Int32 := 0
  -- p_maputl.c
  opentop : Int32 := 0
  openbottom : Int32 := 0
  openrange : Int32 := 0
  lowfloor : Int32 := 0
  /-- Intercepts: 8 bytes each (frac, then index with bit 31 = isaline). -/
  intercepts : ByteArray := ByteArray.empty
  numIntercepts : Nat := 0
  traceX : Int32 := 0
  traceY : Int32 := 0
  traceDx : Int32 := 0
  traceDy : Int32 := 0
  earlyout : Bool := false
  -- p_pspr.c
  bulletslope : Int32 := 0
  -- p_sight.c
  sightzstart : Int32 := 0
  -- p_user.c
  onground : Bool := false
  -- Sound / music.
  soundEvents : Array SoundEvent := #[]
  musPlaying : Nat := 0
  /-- First engine error (C `I_Error`); the tic loop stops on it. -/
  error : Option String := none

instance : Inhabited LevelData := ⟨{
  vertexes := #[], sectors := #[], sides := #[],
  lines := #[], segs := #[], subsectors := #[], nodes := #[], things := #[],
  blockmap := { originX := 0, originY := 0, width := 0, height := 0, lump := #[] },
  reject := ByteArray.empty }⟩

instance : Inhabited GameState := ⟨{
  res := default, states := #[], mobjinfo := #[], level := default
  sectors := #[], players := #[], playeringame := #[], mobjs := #[], thinkers := #[]
  rng := clearRandom, traceIdCounter := 1, leveltime := 0, gametic := 0
  gameskill := 0, consoleplayer := 0, deathmatch := false, nomonsters := false
  netgame := false, commercial := false, totalkills := 0, totalitems := 0
  totalsecret := 0, validcount := 0, lineValidcount := #[], blocklinks := #[]
  stFace := StFaceState.init, hu := HuState.init }⟩

/-- `gameaction_t`. -/
def ga_nothing : Int32 := 0
def ga_loadlevel : Int32 := 1
def ga_newgame : Int32 := 2
def ga_loadgame : Int32 := 3
def ga_savegame : Int32 := 4
def ga_playdemo : Int32 := 5
def ga_completed : Int32 := 6
def ga_victory : Int32 := 7
def ga_worlddone : Int32 := 8
def ga_screenshot : Int32 := 9

/-- `gamestate_t`. -/
def gamestateLevel : Int32 := 0
def gamestateIntermission : Int32 := 1
def gamestateFinale : Int32 := 2
def gamestateDemoScreen : Int32 := 3

/-- Proof-free array set. -/
@[inline] def arrSet {α : Type} (arr : Array α) (i : Nat) (v : α) : Array α :=
  if h : i < arr.size then arr.set i v else arr

instance : Inhabited Line := ⟨{
  v1 := 0, v2 := 0, flags := 0, special := 0, tag := 0,
  sidenum0 := -1, sidenum1 := -1, dx := 0, dy := 0, slopetype := 0, bbox := #[0,0,0,0],
  frontsector := -1, backsector := -1 }⟩
instance : Inhabited Vertex := ⟨{ x := 0, y := 0 }⟩
instance : Inhabited Side := ⟨{
  textureoffset := 0, rowoffset := 0, toptexture := ByteArray.empty,
  bottomtexture := ByteArray.empty, midtexture := ByteArray.empty, sector := 0 }⟩
instance : Inhabited SectorRuntime := ⟨{
  lightlevel := 0, special := 0, tag := 0,
  floorheight := 0, ceilingheight := 0, thinglist := -1, specialdata := -1,
  soundtarget := -1, soundtraversed := 0, validcount := 0, lines := #[],
  floorpic := ByteArray.empty }⟩
instance : Inhabited Subsector := ⟨{ numsegs := 0, firstseg := 0, sector := 0 }⟩
instance : Inhabited Seg := ⟨{
  v1 := 0, v2 := 0, angle := 0, offset := 0, linedef := 0,
  side := 0, frontsector := -1, backsector := -1 }⟩
instance : Inhabited Node := ⟨{
  x := 0, y := 0, dx := 0, dy := 0, bbox0 := #[], bbox1 := #[],
  child0 := 0, child1 := 0 }⟩
instance : Inhabited Sector := ⟨{
  floorheight := 0, ceilingheight := 0, floorpic := ByteArray.empty,
  ceilingpic := ByteArray.empty, lightlevel := 0, special := 0, tag := 0, lines := #[],
  blockbox := #[0,0,0,0], soundorgX := 0, soundorgY := 0 }⟩
instance : Inhabited VerticalDoor := ⟨{
  traceId := 0, type_ := 0, sector := 0, topheight := 0,
  speed := 0, direction := 0, topwait := 0, topcountdown := 0 }⟩
instance : Inhabited Ceiling := ⟨{
  traceId := 0, type_ := 0, sector := 0, bottomheight := 0,
  topheight := 0, speed := 0, crush := 0, direction := 0, tag := 0, olddirection := 0 }⟩
instance : Inhabited FloorMove := ⟨{
  traceId := 0, type_ := 0, crush := 0, sector := 0,
  direction := 0, newspecial := 0, texture := 0, floordestheight := 0, speed := 0 }⟩
instance : Inhabited Plat := ⟨{
  traceId := 0, sector := 0, speed := 0, low := 0, high := 0,
  wait := 0, count := 0, status := 0, oldstatus := 0, crush := 0, tag := 0, type_ := 0 }⟩
instance : Inhabited LightFlash := ⟨{
  traceId := 0, sector := 0, count := 0, maxlight := 0,
  minlight := 0, maxtime := 0, mintime := 0 }⟩
instance : Inhabited StrobeFlash := ⟨{
  traceId := 0, sector := 0, count := 0, minlight := 0,
  maxlight := 0, darktime := 0, brighttime := 0 }⟩
instance : Inhabited Glow := ⟨{
  traceId := 0, sector := 0, minlight := 0, maxlight := 0,
  direction := 0 }⟩
instance : Inhabited FireFlicker := ⟨{
  traceId := 0, sector := 0, count := 0, maxlight := 0,
  minlight := 0 }⟩

/-! ## The state monad -/

abbrev M := StateM GameState

@[inline] def getMo (i : Nat) : M Mobj := return (← get).mobjs[i]!
@[inline] def getMoI (i : Int32) : M Mobj := return (← get).mobjs[i.toNatClampNeg]!
@[inline] def setMo (i : Nat) (mo : Mobj) : M Unit :=
  modify fun gs => { gs with mobjs := gs.mobjs.set! i mo }
@[inline] def modifyMo (i : Nat) (f : Mobj → Mobj) : M Unit :=
  modify fun gs => { gs with mobjs := gs.mobjs.modify i f }
@[inline] def getPl (i : Nat) : M Player := return (← get).players[i]!
@[inline] def modifyPl (i : Nat) (f : Player → Player) : M Unit :=
  modify fun gs => { gs with players := gs.players.modify i f }
@[inline] def getSec (i : Nat) : M SectorRuntime := return (← get).sectors[i]!
@[inline] def modifySec (i : Nat) (f : SectorRuntime → SectorRuntime) : M Unit :=
  modify fun gs => { gs with sectors := gs.sectors.modify i f }
@[inline] def getLine (i : Nat) : M Line := return (← get).level.lines[i]!
@[inline] def getState (s : UInt32) : M State := return (← get).states[s.toNat]!
@[inline] def getInfo (t : Int32) : M MobjInfo := return (← get).mobjinfo[t.toNatClampNeg]!

/-- C `I_Error`: record the first error; the tic loop stops on it. -/
def iError (msg : String) : M Unit :=
  modify fun gs => if gs.error.isNone then { gs with error := some msg } else gs

/-- `P_Random`. -/
@[inline] def pRandomM : M Int32 := do
  let gs ← get
  let (r, rng) := pRandom gs.rng
  set { gs with rng }
  return r

/-- `P_SubRandom`. -/
@[inline] def pSubRandomM : M Int32 := do
  let r1 ← pRandomM
  let r2 ← pRandomM
  return r1 - r2

/-- `M_Random`. -/
@[inline] def mRandomM : M Int32 := do
  let gs ← get
  let (r, rng) := mRandom gs.rng
  set { gs with rng }
  return r

/-- `P_AddThinker`: append, assign `trace_id`. Returns the id. -/
@[inline] def addThinkerRef (kind idx : Nat) : M UInt32 := do
  let gs ← get
  let tid := gs.traceIdCounter
  set { gs with thinkers := gs.thinkers.push (thRef kind idx), traceIdCounter := tid + 1 }
  return tid

/-- Sector index of a mobj (`mobj->subsector->sector`). -/
@[inline] def moSector (gs : GameState) (mo : Mobj) : Nat :=
  gs.level.subsectors[mo.subsector.toNat]!.sector.toNat

/-- `validcount++` (wraps at 2^30 by clearing all stamps; never in practice). -/
def bumpValidcount : M Unit := modify fun gs =>
  if gs.validcount + 1 < 1073741824 then { gs with validcount := gs.validcount + 1 }
  else
    { gs with validcount := 1
              lineValidcount := Array.replicate gs.lineValidcount.size 0
              sectors := gs.sectors.map fun s => { s with validcount := 0 } }

/-- `DEH_String`. -/
def dehString (gs : GameState) (s : String) : String :=
  match gs.res.dehStrings.find? (·.1 == s) with
  | some (_, r) => r
  | none => s

/-- Integer `Int32` helpers. -/
@[inline] def b2i (b : Bool) : Int32 := if b then 1 else 0

end Doom.Playsim.GameState
