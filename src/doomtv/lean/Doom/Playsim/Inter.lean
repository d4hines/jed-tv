import Doom.Playsim.Defs
import Doom.Playsim.Fixed
import Doom.Playsim.Flags
import Doom.Playsim.GameState
import Doom.Playsim.MobjBase
import Doom.Playsim.Player
import Doom.Playsim.Sound
import Doom.Playsim.Weapons

/-!
# Doom.Playsim.Inter

`p_inter.c` item pickup: `P_GiveAmmo` / `P_GiveWeapon` / `P_GiveBody` /
`P_GiveArmor` / `P_GiveCard` / `P_GivePower` and `P_TouchSpecialThing`.
Damage and death live in `Core` (they run state actions).
-/

namespace Doom.Playsim.Inter

open Doom.Playsim.Defs
open Doom.Playsim.Fixed
open Doom.Playsim.Flags
open Doom.Playsim.GameState
open Doom.Playsim.MobjBase
open Doom.Playsim.Player
open Doom.Playsim.Sound
open Doom.Playsim.Weapons (weaponinfo am_clip am_shell am_cell am_misl am_noammo)

def BONUSADD : Int32 := 6
def clipammo : Array Int32 := #[10, 4, 20, 1]
def maxammoDefault : Array Int32 := #[200, 50, 300, 50]

def INVULNTICS : Int32 := 30 * 35
def INVISTICS : Int32 := 60 * 35
def INFRATICS : Int32 := 120 * 35
def IRONTICS : Int32 := 60 * 35

@[inline] def aget (a : Array Int32) (i : Int32) : Int32 := a[i.toNatClampNeg]!
@[inline] def aset (a : Array Int32) (i : Int32) (v : Int32) : Array Int32 := a.set! i.toNatClampNeg v

/-- `P_GiveAmmo` (pure on the player). -/
def giveAmmo (p : Player) (ammo num0 gameskill : Int32) : Player × Bool := Id.run do
  if ammo == am_noammo then return (p, false)
  if ammo < 0 || ammo >= 4 then return (p, false)
  if aget p.ammo ammo == aget p.maxammo ammo then return (p, false)
  let mut num := if num0 != 0 then num0 * aget clipammo ammo else aget clipammo ammo / 2
  if gameskill == sk_baby || gameskill == sk_nightmare then num := num <<< 1
  let oldammo := aget p.ammo ammo
  let mut newammo := oldammo + num
  if newammo > aget p.maxammo ammo then newammo := aget p.maxammo ammo
  let mut p := { p with ammo := aset p.ammo ammo newammo }
  if oldammo != 0 then return (p, true)
  let owned (w : Int32) := aget p.weaponowned w != 0
  if ammo == am_clip then
    if p.readyweapon == wp_fist then
      p := { p with pendingweapon := if owned wp_chaingun then wp_chaingun else wp_pistol }
  else if ammo == am_shell then
    if p.readyweapon == wp_fist || p.readyweapon == wp_pistol then
      if owned wp_shotgun then p := { p with pendingweapon := wp_shotgun }
  else if ammo == am_cell then
    if p.readyweapon == wp_fist || p.readyweapon == wp_pistol then
      if owned wp_plasma then p := { p with pendingweapon := wp_plasma }
  else if ammo == am_misl then
    if p.readyweapon == wp_fist then
      if owned wp_missile then p := { p with pendingweapon := wp_missile }
  return (p, true)

/-- `P_GiveWeapon`. -/
def giveWeapon (pi : Nat) (weapon : Int32) (dropped : Bool) : M Bool := do
  let gs ← get
  let p := gs.players[pi]!
  let wi := weaponinfo[weapon.toNatClampNeg]!
  if gs.netgame && gs.deathmatchMode != 2 && !dropped then
    if aget p.weaponowned weapon != 0 then return false
    let p := { p with bonuscount := p.bonuscount + BONUSADD
                      weaponowned := aset p.weaponowned weapon 1 }
    let (p, _) := giveAmmo p wi.ammo (if gs.deathmatchMode != 0 then 5 else 2) gs.gameskill
    modifyPl pi fun _ => { p with pendingweapon := weapon }
    if pi == gs.consoleplayer then startSoundNull sfx_wpnup
    return false
  let mut p := p
  let mut gaveammo := false
  if wi.ammo != am_noammo then
    let (p', g) := giveAmmo p wi.ammo (if dropped then 1 else 2) gs.gameskill
    p := p'
    gaveammo := g
  let mut gaveweapon := false
  if aget p.weaponowned weapon == 0 then
    gaveweapon := true
    p := { p with weaponowned := aset p.weaponowned weapon 1, pendingweapon := weapon }
  modifyPl pi fun _ => p
  return gaveweapon || gaveammo

/-- `P_GiveBody`. -/
def giveBody (pi : Nat) (num : Int32) : M Bool := do
  let p ← getPl pi
  if p.health >= 100 then return false
  let h := if p.health + num > 100 then 100 else p.health + num
  modifyPl pi fun p => { p with health := h }
  modifyMo p.mo.toNatClampNeg fun m => { m with health := h }
  return true

/-- `P_GiveArmor`. -/
def giveArmor (pi : Nat) (armortype : Int32) : M Bool := do
  let p ← getPl pi
  let hits := armortype * 100
  if p.armorpoints >= hits then return false
  modifyPl pi fun p => { p with armortype, armorpoints := hits }
  return true

/-- `P_GiveCard`. -/
def giveCard (pi : Nat) (card : Nat) : M Unit := modifyPl pi fun p =>
  if p.cards[card]! then p
  else { p with bonuscount := BONUSADD, cards := p.cards.set! card true }

/-- `P_GivePower`. -/
def givePower (pi : Nat) (power : Nat) : M Bool := do
  let p ← getPl pi
  let setPow (v : Int32) : M Unit := modifyPl pi fun p => { p with powers := p.powers.set! power v }
  if power == pw_invulnerability then setPow INVULNTICS; return true
  if power == pw_invisibility then
    setPow INVISTICS
    modifyMo p.mo.toNatClampNeg fun m => { m with flags := m.flags ||| MF_SHADOW }
    return true
  if power == pw_infrared then setPow INFRATICS; return true
  if power == pw_ironfeet then setPow IRONTICS; return true
  if power == pw_strength then
    let _ ← giveBody pi 100
    setPow 1
    return true
  if p.powers[power]! != 0 then return false
  setPow 1
  return true

private def setMsg (pi : Nat) (s : String) : M Unit := do
  let gs ← get
  let m := dehString gs s
  modifyPl pi fun p => { p with message := some m }

/-- `P_TouchSpecialThing(special, toucher)`. -/
def touchSpecialThing (si ti : Nat) : M Unit := do
  let gs ← get
  let special := gs.mobjs[si]!
  let toucher := gs.mobjs[ti]!
  let delta := special.z - toucher.z
  if delta > toucher.height || delta < -8 * FRACUNIT then return
  if toucher.health <= 0 then return
  let pi := toucher.player.toNatClampNeg
  let ultimateish := gs.res.gameversion > exe_doom_1_2
  let netgame := gs.netgame
  let commercial := gs.res.gamemode == gm_commercial
  let mut sound := sfx_itemup
  let spr := special.sprite
  let dropped := (special.flags &&& MF_DROPPED) != 0
  let ammoPick (ammo num : Int32) (msg : String) : M Bool := do
    let gs ← get
    let (p, ok) := giveAmmo gs.players[pi]! ammo num gs.gameskill
    if !ok then return false
    modifyPl pi fun _ => p
    setMsg pi msg
    return true
  if spr == SPR_ARM1 then
    if !(← giveArmor pi 1) then return
    setMsg pi "Picked up the armor."
  else if spr == SPR_ARM2 then
    if !(← giveArmor pi 2) then return
    setMsg pi "Picked up the MegaArmor!"
  else if spr == SPR_BON1 then
    let p ← getPl pi
    let h := if p.health + 1 > 200 then 200 else p.health + 1
    modifyPl pi fun p => { p with health := h }
    modifyMo p.mo.toNatClampNeg fun m => { m with health := h }
    setMsg pi "Picked up a health bonus."
  else if spr == SPR_BON2 then
    modifyPl pi fun p =>
      let a := p.armorpoints + 1
      let a := if a > 200 && ultimateish then 200 else a
      { p with armorpoints := a, armortype := if p.armortype == 0 then 1 else p.armortype }
    setMsg pi "Picked up an armor bonus."
  else if spr == SPR_SOUL then
    let p ← getPl pi
    let h := if p.health + 100 > 200 then 200 else p.health + 100
    modifyPl pi fun p => { p with health := h }
    modifyMo p.mo.toNatClampNeg fun m => { m with health := h }
    setMsg pi "Supercharge!"
    if ultimateish then sound := sfx_getpow
  else if spr == SPR_MEGA then
    if !commercial then return
    let p ← getPl pi
    modifyPl pi fun p => { p with health := 200 }
    modifyMo p.mo.toNatClampNeg fun m => { m with health := 200 }
    let _ ← giveArmor pi 2
    setMsg pi "MegaSphere!"
    if ultimateish then sound := sfx_getpow
  else if spr == SPR_BKEY || spr == SPR_YKEY || spr == SPR_RKEY ||
      spr == SPR_BSKU || spr == SPR_YSKU || spr == SPR_RSKU then
    let (card, msg) :=
      if spr == SPR_BKEY then (it_bluecard, "Picked up a blue keycard.")
      else if spr == SPR_YKEY then (it_yellowcard, "Picked up a yellow keycard.")
      else if spr == SPR_RKEY then (it_redcard, "Picked up a red keycard.")
      else if spr == SPR_BSKU then (it_blueskull, "Picked up a blue skull key.")
      else if spr == SPR_YSKU then (it_yellowskull, "Picked up a yellow skull key.")
      else (it_redskull, "Picked up a red skull key.")
    if !(← getPl pi).cards[card]! then setMsg pi msg
    giveCard pi card
    if netgame then return
  else if spr == SPR_STIM then
    if !(← giveBody pi 10) then return
    setMsg pi "Picked up a stimpack."
  else if spr == SPR_MEDI then
    if !(← giveBody pi 25) then return
    if (← getPl pi).health < 25 then setMsg pi "Picked up a medikit that you REALLY need!"
    else setMsg pi "Picked up a medikit."
  else if spr == SPR_PINV then
    if !(← givePower pi pw_invulnerability) then return
    setMsg pi "Invulnerability!"
    if ultimateish then sound := sfx_getpow
  else if spr == SPR_PSTR then
    if !(← givePower pi pw_strength) then return
    setMsg pi "Berserk!"
    modifyPl pi fun p => if p.readyweapon != wp_fist then { p with pendingweapon := wp_fist } else p
    if ultimateish then sound := sfx_getpow
  else if spr == SPR_PINS then
    if !(← givePower pi pw_invisibility) then return
    setMsg pi "Partial Invisibility"
    if ultimateish then sound := sfx_getpow
  else if spr == SPR_SUIT then
    if !(← givePower pi pw_ironfeet) then return
    setMsg pi "Radiation Shielding Suit"
    if ultimateish then sound := sfx_getpow
  else if spr == SPR_PMAP then
    if !(← givePower pi pw_allmap) then return
    setMsg pi "Computer Area Map"
    if ultimateish then sound := sfx_getpow
  else if spr == SPR_PVIS then
    if !(← givePower pi pw_infrared) then return
    setMsg pi "Light Amplification Visor"
    if ultimateish then sound := sfx_getpow
  else if spr == SPR_CLIP then
    if !(← ammoPick am_clip (if dropped then 0 else 1) "Picked up a clip.") then return
  else if spr == SPR_AMMO then
    if !(← ammoPick am_clip 5 "Picked up a box of bullets.") then return
  else if spr == SPR_ROCK then
    if !(← ammoPick am_misl 1 "Picked up a rocket.") then return
  else if spr == SPR_BROK then
    if !(← ammoPick am_misl 5 "Picked up a box of rockets.") then return
  else if spr == SPR_CELL then
    if !(← ammoPick am_cell 1 "Picked up an energy cell.") then return
  else if spr == SPR_CELP then
    if !(← ammoPick am_cell 5 "Picked up an energy cell pack.") then return
  else if spr == SPR_SHEL then
    if !(← ammoPick am_shell 1 "Picked up 4 shotgun shells.") then return
  else if spr == SPR_SBOX then
    if !(← ammoPick am_shell 5 "Picked up a box of shotgun shells.") then return
  else if spr == SPR_BPAK then
    let p ← getPl pi
    if !p.backpack then
      modifyPl pi fun p => { p with maxammo := p.maxammo.map (· * 2), backpack := true }
    for i in [0:4] do
      let gs ← get
      let (p, _) := giveAmmo gs.players[pi]! i.toInt32 1 gs.gameskill
      modifyPl pi fun _ => p
    setMsg pi "Picked up a backpack full of ammo!"
  else if spr == SPR_BFUG || spr == SPR_MGUN || spr == SPR_CSAW || spr == SPR_LAUN ||
      spr == SPR_PLAS || spr == SPR_SHOT || spr == SPR_SGN2 then
    let (w, dr, msg) :=
      if spr == SPR_BFUG then (wp_bfg, false, "You got the BFG9000!  Oh, yes.")
      else if spr == SPR_MGUN then (wp_chaingun, dropped, "You got the chaingun!")
      else if spr == SPR_CSAW then (wp_chainsaw, false, "A chainsaw!  Find some meat!")
      else if spr == SPR_LAUN then (wp_missile, false, "You got the rocket launcher!")
      else if spr == SPR_PLAS then (wp_plasma, false, "You got the plasma gun!")
      else if spr == SPR_SHOT then (wp_shotgun, dropped, "You got the shotgun!")
      else (wp_supershotgun, dropped, "You got the super shotgun!")
    if !(← giveWeapon pi w dr) then return
    setMsg pi msg
    sound := sfx_wpnup
  else
    iError "P_SpecialThing: Unknown gettable thing"
    return
  if (special.flags &&& MF_COUNTITEM) != 0 then
    modifyPl pi fun p => { p with itemcount := p.itemcount + 1 }
  removeMobj si
  modifyPl pi fun p => { p with bonuscount := p.bonuscount + BONUSADD }
  if pi == (← get).consoleplayer then startSoundNull sound

end Doom.Playsim.Inter
