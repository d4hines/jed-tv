import Doom.Audio.OplTables

/-!
# Doom.Audio.Opl

A bit-exact port of the Nuked OPL3 emulator (YMF262), version 1.8, as used by
Chocolate Doom's software OPL backend.

Reference: https://github.com/nukeykt/Nuked-OPL3 commit
cfedb09efc03f1d7b5fc1f04dd449d77d8c49d50 (the base of the Nuked-OPL3-fast
fork that Chocolate Doom vendors as `opl/opl3.c`). Both C versions are
compared sample-for-sample against this module by the host harness
(`AudioCheck/c/opl_ref.c`, see `check-audio.sh`).

Nuked OPL3 — Copyright (C) 2013-2020 Nuke.YKT. Some write-time caches follow
Nuked-OPL3-fast (Copyright (C) 2026 Tony Gies). Nuked OPL3 is free software:
you can redistribute it and/or modify it under the terms of the GNU Lesser
General Public License as published by the Free Software Foundation, either
version 2.1 of the License, or (at your option) any later version. It is
distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A
PARTICULAR PURPOSE. See the GNU Lesser General Public License for details.
This Lean translation is distributed under the same terms (and, as part of
the Doom engine, GPL-2.0-or-later, which LGPL-2.1+ permits).

Implementation notes (32-bit ARM performance):
* All per-slot and per-channel state lives in structures whose fields are
  all scalars, held in `Array`s that are kept uniquely referenced and updated
  in place (`Chip.modSlot` takes the array out of the chip before
  `Array.modify`, so the element is unshared and its constructor is reused).
* `C` pointers become indices: a slot's modulation input is `modKind`
  (0 zero, 1 own feedback, 2 another slot's output `modSlot`), and channel
  outputs are slot indices (`noOut` = the chip's zero sample).
* Channel values read live by upstream (`f_num`, `block`, `ksv`, `fb`) are
  cached in each slot and refreshed by every register write that changes
  them (as Nuked-OPL3-fast does for `pg_inc`/`eg_tl_ksl`).
* While no register in the second (OPL3) bank has been written, slots 18-35
  provably stay in their reset state and output 0; they are skipped (the
  noise LFSR is still clocked once per skipped slot), which halves the cost
  of Doom's default OPL2 mode without changing a single output bit.
* The OPL3 "DAC2" outputs (`mixbuff[2]`, `mixbuff[3]`) are not computed:
  `OPL3_GenerateResampled`, which Chocolate uses, discards them.
-/

namespace Doom.Audio.Opl

/-! ## Small tables -/

def mtTab : ByteArray := ⟨#[1, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 20, 24, 24, 30, 30]⟩
def kslrom : ByteArray := ⟨#[0, 32, 40, 45, 48, 51, 53, 55, 56, 58, 59, 60, 61, 62, 63, 64]⟩
def kslshift : ByteArray := ⟨#[8, 1, 2, 0]⟩
def egIncstep : ByteArray := ⟨#[0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 1, 0, 1, 1, 1, 0]⟩
/-- `ad_slot[0x20]`, with 255 for -1. -/
def adSlot : ByteArray := ⟨#[0, 1, 2, 3, 4, 5, 255, 255, 6, 7, 8, 9, 10, 11, 255, 255,
  12, 13, 14, 15, 16, 17, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255]⟩
def chSlot : ByteArray := ⟨#[0, 1, 2, 6, 7, 8, 12, 13, 14, 18, 19, 20, 24, 25, 26, 30, 31, 32]⟩

@[inline] def tb (t : ByteArray) (i : UInt32) : UInt32 := (t.get! i.toNat).toUInt32

/-- Sign-extend the low 16 bits (UInt32 arithmetic only: Int32 literals are not folded). -/
@[inline] def sext16 (x : UInt32) : Int32 := (((x &&& 0xffff) ^^^ 0x8000) - 0x8000).toInt32

/-- An `Int32` constant written as its `UInt32` bit pattern (a plain cast in C). -/
@[inline] def i32 (u : UInt32) : Int32 := u.toInt32

/--
Waveform entry `(out | neg << 15)` for waveform `wf` at 10-bit `phase`, from
upstream `OPL3_EnvelopeCalcSin0..7` (the table layout is Nuked-OPL3-fast's
`logsin_wf`).
-/
def wfEntry (wf : Nat) (phase0 : UInt32) : UInt16 :=
  let phase := phase0 &&& 0x3ff
  let ls (i : UInt32) : UInt32 := (logsinrom[(i &&& 0xff).toNat]!).toUInt32
  let (out, neg) : UInt32 × Bool :=
    match wf with
    | 0 =>
      ((if (phase &&& 0x100) != 0 then ls ((phase &&& 0xff) ^^^ 0xff) else ls phase),
       (phase &&& 0x200) != 0)
    | 1 =>
      ((if (phase &&& 0x200) != 0 then 0x1000
        else if (phase &&& 0x100) != 0 then ls ((phase &&& 0xff) ^^^ 0xff) else ls phase), false)
    | 2 =>
      ((if (phase &&& 0x100) != 0 then ls ((phase &&& 0xff) ^^^ 0xff) else ls phase), false)
    | 3 =>
      ((if (phase &&& 0x100) != 0 then 0x1000 else ls phase), false)
    | 4 =>
      ((if (phase &&& 0x200) != 0 then 0x1000
        else if (phase &&& 0x80) != 0 then ls (((phase ^^^ 0xff) <<< 1) &&& 0xff)
        else ls ((phase <<< 1) &&& 0xff)),
       (phase &&& 0x300) == 0x100)
    | 5 =>
      ((if (phase &&& 0x200) != 0 then 0x1000
        else if (phase &&& 0x80) != 0 then ls (((phase ^^^ 0xff) <<< 1) &&& 0xff)
        else ls ((phase <<< 1) &&& 0xff)), false)
    | 6 => (0, (phase &&& 0x200) != 0)
    | _ =>
      let neg := (phase &&& 0x200) != 0
      let p := if neg then (phase &&& 0x1ff) ^^^ 0x1ff else phase
      (p <<< 3, neg)
  (out ||| (if neg then 0x8000 else 0)).toUInt16

/-- 8 x 1024 waveform table indexed by `(wf << 10) | phase`. -/
def logsinWf : Array UInt16 := Id.run do
  let mut a : Array UInt16 := Array.mkEmpty 8192
  for wf in [0:8] do
    for p in [0:1024] do
      a := a.push (wfEntry wf p.toUInt32)
  return a

/-- `exprom[i] << 1`. -/
def expromX2 : Array UInt16 := exprom.map (fun x => x <<< (1 : UInt16))

/-! ## State -/

def ch2op : UInt32 := 0
def ch4op : UInt32 := 1
def ch4op2 : UInt32 := 2
def chDrum : UInt32 := 3
def egkNorm : UInt32 := 1
def egkDrum : UInt32 := 2
/-- Modulation sources. -/
def modZero : UInt32 := 0
def modFb : UInt32 := 1
def modOut : UInt32 := 2
/-- "Points at `zeromod`" for channel outputs / missing pair. -/
def noOut : UInt32 := 255

structure Slot where
  pgPhase : UInt32 := 0
  /-- Cached non-vibrato phase increment. -/
  pgInc : UInt32 := 0
  pgPhaseOut : UInt32 := 0
  pgReset : UInt32 := 0
  out : Int32 := 0
  fbmod : Int32 := 0
  prout : Int32 := 0
  egRout : UInt32 := 0x1ff
  egOut : UInt32 := 0x1ff
  egGen : UInt32 := 3
  egKsl : UInt32 := 0
  /-- Cached `(reg_tl << 2) + (eg_ksl >> kslshift[reg_ksl])`. -/
  egTlKsl : UInt32 := 0
  /-- Cached `channel->ksv >> ((reg_ksr ^ 1) << 1)`. -/
  egKs : UInt32 := 0
  /--
  Cached envelope rates per generator state (attack, decay, sustain,
  release), one byte each: bit 7 = rate register nonzero, bits 2-5 =
  `rate_hi`, bits 0-1 = `rate_lo` (Nuked-OPL3-fast's `eg_rate_hi/lo`).
  -/
  egRates : UInt32 := 0
  key : UInt32 := 0
  regVib : UInt32 := 0
  regType : UInt32 := 0
  regKsr : UInt32 := 0
  regMult : UInt32 := 0
  regKsl : UInt32 := 0
  regTl : UInt32 := 0
  regAr : UInt32 := 0
  regDr : UInt32 := 0
  regSl : UInt32 := 0
  regRr : UInt32 := 0
  regWf : UInt32 := 0
  tremOn : UInt32 := 0
  modKind : UInt32 := 0
  modSlot : UInt32 := 0
  /-- Copies of the owning channel's `fb`, `f_num`, `block`. -/
  fb : UInt32 := 0
  fNum : UInt32 := 0
  block : UInt32 := 0
  chan : UInt32 := 0
  slotNum : UInt32 := 0
deriving Inhabited

structure Chan where
  slot0 : UInt32 := 0
  slot1 : UInt32 := 0
  pair : UInt32 := noOut
  chtype : UInt32 := 0
  fNum : UInt32 := 0
  block : UInt32 := 0
  fb : UInt32 := 0
  con : UInt32 := 0
  alg : UInt32 := 0
  ksv : UInt32 := 0
  cha : UInt32 := 0xffff
  chb : UInt32 := 0xffff
  chc : UInt32 := 0
  chd : UInt32 := 0
  out0 : UInt32 := noOut
  out1 : UInt32 := noOut
  out2 : UInt32 := noOut
  out3 : UInt32 := noOut
  outCnt : UInt32 := 0
  chNum : UInt32 := 0
deriving Inhabited

def writebufSize : UInt32 := 1024
/-- Bytes per write-buffer entry: time (8, LE), reg (2, LE), data (1), pad (1). -/
def wbEntry : Nat := 12

structure Chip where
  slots : Array Slot
  chans : Array Chan
  timer : UInt32 := 0
  egTimer : UInt64 := 0
  egTimerRem : UInt32 := 0
  egState : UInt32 := 0
  egAdd : UInt32 := 0
  egTimerLo : UInt32 := 0
  newm : UInt32 := 0
  nts : UInt32 := 0
  rhy : UInt32 := 0
  vibpos : UInt32 := 0
  vibshift : UInt32 := 1
  tremolo : UInt32 := 0
  tremolopos : UInt32 := 0
  tremoloshift : UInt32 := 4
  noise : UInt32 := 1
  mix0 : Int32 := 0
  mix1 : Int32 := 0
  rmHh2 : UInt32 := 0
  rmHh3 : UInt32 := 0
  rmHh7 : UInt32 := 0
  rmHh8 : UInt32 := 0
  rmTc3 : UInt32 := 0
  rmTc5 : UInt32 := 0
  rateratio : Int32 := 0
  samplecnt : Int32 := 0
  old0 : Int32 := 0
  old1 : Int32 := 0
  cur0 : Int32 := 0
  cur1 : Int32 := 0
  wbuf : ByteArray
  wbufSamplecnt : UInt64 := 0
  wbufCur : UInt32 := 0
  wbufLast : UInt32 := 0
  wbufLasttime : UInt64 := 0
  /-- Some register of the second bank (`reg & 0x100`) has been written. -/
  hiTouched : Bool := false
  /--
  Cached mix lists for the left (`cha`) and right (`chb`) passes: for each
  contributing channel, its output count followed by that many slot indices.
  Rebuilt when `mixDirty` (set by every change of channel outputs, output
  masks or `hiTouched`).
  -/
  mixL : ByteArray := .empty
  mixR : ByteArray := .empty
  /-- `mixL` split into entries for slots below 15 and slots 15..17 (OPL2-only fast path). -/
  mixLow : ByteArray := .empty
  mixHigh : ByteArray := .empty
  mixDirty : Bool := true

instance : Inhabited Chip := ⟨{ slots := #[], chans := #[], wbuf := .empty }⟩

def emptySlots : Array Slot := #[]
def emptyChans : Array Chan := #[]
def emptyBytes : ByteArray := .empty

@[inline] def Chip.slot (c : Chip) (i : UInt32) : Slot := c.slots[i.toNat]!
@[inline] def Chip.chan (c : Chip) (i : UInt32) : Chan := c.chans[i.toNat]!

/-- In-place update of one slot (the array is detached first so it stays unshared). -/
@[inline] def Chip.modSlot (c : Chip) (i : Nat) (f : Slot → Slot) : Chip :=
  let slots := c.slots
  let c := { c with slots := emptySlots }
  { c with slots := slots.modify i f }

@[inline] def Chip.modChan (c : Chip) (i : Nat) (f : Chan → Chan) : Chip :=
  let chans := c.chans
  let c := { c with chans := emptyChans }
  { c with chans := chans.modify i f }

/-! ## Envelope / phase helpers used by register writes -/

/-- `OPL3_EnvelopeUpdateKSL`'s `eg_ksl` value for a channel `f_num`/`block`. -/
@[inline] def kslValue (fNum block : UInt32) : UInt32 :=
  let a : Int32 := (tb kslrom (fNum >>> 6)).toInt32 <<< 2
  let b : Int32 := ((8 : Int32) - block.toInt32) <<< 5
  let k := a - b
  if k < 0 then 0 else k.toUInt32

@[inline] def phaseIncFor (fNum block mult : UInt32) : UInt32 :=
  (((fNum <<< block) >>> 1) * tb mtTab mult) >>> 1

/-- Packed rate byte for one envelope state (see `Slot.egRates`). -/
@[inline] def rateByte (ks regRate : UInt32) : UInt32 :=
  let rate := ks + (regRate <<< 2)
  let hi0 := rate >>> 2
  let hi : UInt32 := if (hi0 &&& 0x10) != 0 then 0x0f else hi0
  (if regRate != 0 then (0x80 : UInt32) else 0) ||| (hi <<< 2) ||| (rate &&& 3)

/-- Recompute `egRates` from `egKs` and the AR/DR/RR/EG-type registers. -/
@[inline] def Slot.refreshRates (s : Slot) : Slot :=
  let ks := s.egKs
  { s with egRates := rateByte ks s.regAr ||| (rateByte ks s.regDr <<< 8) |||
      (rateByte ks (if s.regType == 0 then s.regRr else 0) <<< 16) ||| (rateByte ks s.regRr <<< 24) }

/-- Refresh every cached copy of channel state in a slot. -/
@[inline] def Slot.syncChan (s : Slot) (fNum block ksv : UInt32) : Slot :=
  let ksl := kslValue fNum block
  { s with fNum := fNum, block := block, egKsl := ksl,
           egTlKsl := (s.regTl <<< 2) + (ksl >>> tb kslshift s.regKsl),
           egKs := ksv >>> ((s.regKsr ^^^ 1) <<< 1),
           pgInc := phaseIncFor fNum block s.regMult }.refreshRates

/-- Push a channel's `f_num`/`block`/`ksv` into both of its slots. -/
def syncChanSlots (c : Chip) (ci : UInt32) : Chip :=
  let ch := c.chan ci
  let c := c.modSlot ch.slot0.toNat (·.syncChan ch.fNum ch.block ch.ksv)
  c.modSlot ch.slot1.toNat (·.syncChan ch.fNum ch.block ch.ksv)

/-! ## Slot register writes -/

def slotWrite20 (c : Chip) (si : Nat) (data : UInt32) : Chip :=
  let s := c.slots[si]!
  let ksv := (c.chan s.chan).ksv
  c.modSlot si fun s =>
    let ksr := (data >>> 4) &&& 1
    let mult := data &&& 0x0f
    { s with tremOn := (data >>> 7) &&& 1, regVib := (data >>> 6) &&& 1,
             regType := (data >>> 5) &&& 1, regKsr := ksr, regMult := mult,
             egKs := ksv >>> ((ksr ^^^ 1) <<< 1),
             pgInc := phaseIncFor s.fNum s.block mult }.refreshRates

def slotWrite40 (c : Chip) (si : Nat) (data : UInt32) : Chip :=
  c.modSlot si fun s =>
    let regKsl := (data >>> 6) &&& 3
    let regTl := data &&& 0x3f
    let ksl := kslValue s.fNum s.block
    { s with regKsl := regKsl, regTl := regTl, egKsl := ksl,
             egTlKsl := (regTl <<< 2) + (ksl >>> tb kslshift regKsl) }

def slotWrite60 (c : Chip) (si : Nat) (data : UInt32) : Chip :=
  c.modSlot si fun s => { s with regAr := (data >>> 4) &&& 0x0f, regDr := data &&& 0x0f }.refreshRates

def slotWrite80 (c : Chip) (si : Nat) (data : UInt32) : Chip :=
  c.modSlot si fun s =>
    let sl := (data >>> 4) &&& 0x0f
    { s with regSl := (if sl == 0x0f then 0x1f else sl), regRr := data &&& 0x0f }.refreshRates

def slotWriteE0 (c : Chip) (si : Nat) (data : UInt32) : Chip :=
  let newm := c.newm
  c.modSlot si fun s =>
    let wf := data &&& 7
    { s with regWf := (if newm == 0 then wf &&& 3 else wf) }

/-! ## Channels -/

@[inline] def setMod (c : Chip) (si : UInt32) (kind src : UInt32) : Chip :=
  c.modSlot si.toNat fun s => { s with modKind := kind, modSlot := src }

@[inline] def setOuts (c : Chip) (ci : UInt32) (o0 o1 o2 o3 cnt : UInt32) : Chip :=
  let c := c.modChan ci.toNat fun ch => { ch with out0 := o0, out1 := o1, out2 := o2, out3 := o3, outCnt := cnt }
  { c with mixDirty := true }

/-- `OPL3_ChannelSetupAlg`. -/
def setupAlg (c : Chip) (ci : UInt32) : Chip :=
  let ch := c.chan ci
  let s0 := ch.slot0
  let s1 := ch.slot1
  if ch.chtype == chDrum then
    if ch.chNum == 7 || ch.chNum == 8 then
      setMod (setMod c s0 modZero 0) s1 modZero 0
    else if (ch.alg &&& 1) == 0 then
      setMod (setMod c s0 modFb 0) s1 modOut s0
    else
      setMod (setMod c s0 modFb 0) s1 modZero 0
  else if (ch.alg &&& 8) != 0 then
    c
  else if (ch.alg &&& 4) != 0 then
    let p := c.chan ch.pair
    let ps0 := p.slot0
    let ps1 := p.slot1
    let c := setOuts c ch.pair noOut noOut noOut noOut 0
    match ch.alg &&& 3 with
    | 0 =>
      let c := setMod c ps0 modFb 0
      let c := setMod c ps1 modOut ps0
      let c := setMod c s0 modOut ps1
      let c := setMod c s1 modOut s0
      setOuts c ci s1 noOut noOut noOut 1
    | 1 =>
      let c := setMod c ps0 modFb 0
      let c := setMod c ps1 modOut ps0
      let c := setMod c s0 modZero 0
      let c := setMod c s1 modOut s0
      setOuts c ci ps1 s1 noOut noOut 2
    | 2 =>
      let c := setMod c ps0 modFb 0
      let c := setMod c ps1 modZero 0
      let c := setMod c s0 modOut ps1
      let c := setMod c s1 modOut s0
      setOuts c ci ps0 s1 noOut noOut 2
    | _ =>
      let c := setMod c ps0 modFb 0
      let c := setMod c ps1 modZero 0
      let c := setMod c s0 modOut ps1
      let c := setMod c s1 modZero 0
      setOuts c ci ps0 s0 s1 noOut 3
  else if (ch.alg &&& 1) == 0 then
    let c := setMod c s0 modFb 0
    let c := setMod c s1 modOut s0
    setOuts c ci s1 noOut noOut noOut 1
  else
    let c := setMod c s0 modFb 0
    let c := setMod c s1 modZero 0
    setOuts c ci s0 s1 noOut noOut 2

/-- `OPL3_ChannelUpdateAlg`. -/
def updateAlg (c : Chip) (ci : UInt32) : Chip :=
  let ch := c.chan ci
  let c := c.modChan ci.toNat fun x => { x with alg := x.con }
  if c.newm != 0 then
    if ch.chtype == ch4op then
      let p := c.chan ch.pair
      let c := c.modChan ch.pair.toNat fun x => { x with alg := (4 : UInt32) ||| (ch.con <<< (1 : UInt32)) ||| p.con }
      let c := c.modChan ci.toNat fun x => { x with alg := 8 }
      setupAlg c ch.pair
    else if ch.chtype == ch4op2 then
      let p := c.chan ch.pair
      let c := c.modChan ci.toNat fun x => { x with alg := (4 : UInt32) ||| (p.con <<< (1 : UInt32)) ||| ch.con }
      let c := c.modChan ch.pair.toNat fun x => { x with alg := 8 }
      setupAlg c ci
    else
      setupAlg c ci
  else
    setupAlg c ci

@[inline] def keyOnSlot (c : Chip) (si : UInt32) (ty : UInt32) : Chip :=
  c.modSlot si.toNat fun s => { s with key := s.key ||| ty }

@[inline] def keyOffSlot (c : Chip) (si : UInt32) (ty : UInt32) : Chip :=
  c.modSlot si.toNat fun s => { s with key := s.key &&& ~~~ty }

def chanKey (c : Chip) (ci : UInt32) (on : Bool) : Chip :=
  let ch := c.chan ci
  let k (c : Chip) (si : UInt32) : Chip := if on then keyOnSlot c si egkNorm else keyOffSlot c si egkNorm
  if c.newm != 0 then
    if ch.chtype == ch4op then
      let p := c.chan ch.pair
      k (k (k (k c ch.slot0) ch.slot1) p.slot0) p.slot1
    else if ch.chtype == ch2op || ch.chtype == chDrum then
      k (k c ch.slot0) ch.slot1
    else
      c
  else
    k (k c ch.slot0) ch.slot1

/-- `OPL3_ChannelUpdateRhythm`. -/
def updateRhythm (c : Chip) (data : UInt32) : Chip :=
  let rhy := data &&& 0x3f
  let c := { c with rhy := rhy }
  let ch6 := c.chan 6
  let ch7 := c.chan 7
  let ch8 := c.chan 8
  if (rhy &&& 0x20) != 0 then
    let c := setOuts c 6 ch6.slot1 ch6.slot1 noOut noOut 2
    let c := setOuts c 7 ch7.slot0 ch7.slot0 ch7.slot1 ch7.slot1 4
    let c := setOuts c 8 ch8.slot0 ch8.slot0 ch8.slot1 ch8.slot1 4
    let c := c.modChan 6 fun x => { x with chtype := chDrum }
    let c := c.modChan 7 fun x => { x with chtype := chDrum }
    let c := c.modChan 8 fun x => { x with chtype := chDrum }
    let c := setupAlg c 6
    let c := setupAlg c 7
    let c := setupAlg c 8
    let kd (c : Chip) (bit : UInt32) (si : UInt32) : Chip :=
      if (rhy &&& bit) != 0 then keyOnSlot c si egkDrum else keyOffSlot c si egkDrum
    let c := kd c 0x01 ch7.slot0   -- hh
    let c := kd c 0x02 ch8.slot1   -- tc
    let c := kd c 0x04 ch8.slot0   -- tom
    let c := kd c 0x08 ch7.slot1   -- sd
    let c := kd c 0x10 ch6.slot0   -- bd
    kd c 0x10 ch6.slot1
  else
    let off (c : Chip) (ci : UInt32) (ch : Chan) : Chip :=
      let c := c.modChan ci.toNat fun x => { x with chtype := ch2op }
      let c := setupAlg c ci
      keyOffSlot (keyOffSlot c ch.slot0 egkDrum) ch.slot1 egkDrum
    off (off (off c 6 ch6) 7 ch7) 8 ch8

def writeA0 (c : Chip) (ci : UInt32) (data : UInt32) : Chip :=
  let ch := c.chan ci
  if c.newm != 0 && ch.chtype == ch4op2 then c else
  let fNum : UInt32 := (ch.fNum &&& (0x300 : UInt32)) ||| data
  let ksv := (ch.block <<< 1) ||| ((fNum >>> (9 - c.nts)) &&& 1)
  let c := c.modChan ci.toNat fun x => { x with fNum := fNum, ksv := ksv }
  let c := syncChanSlots c ci
  if c.newm != 0 && ch.chtype == ch4op then
    let c := c.modChan ch.pair.toNat fun x => { x with fNum := fNum, ksv := ksv }
    syncChanSlots c ch.pair
  else c

def writeB0 (c : Chip) (ci : UInt32) (data : UInt32) : Chip :=
  let ch := c.chan ci
  if c.newm != 0 && ch.chtype == ch4op2 then c else
  let fNum : UInt32 := (ch.fNum &&& (0xff : UInt32)) ||| ((data &&& (3 : UInt32)) <<< (8 : UInt32))
  let block := (data >>> 2) &&& 7
  let ksv := (block <<< 1) ||| ((fNum >>> (9 - c.nts)) &&& 1)
  let c := c.modChan ci.toNat fun x => { x with fNum := fNum, block := block, ksv := ksv }
  let c := syncChanSlots c ci
  if c.newm != 0 && ch.chtype == ch4op then
    let c := c.modChan ch.pair.toNat fun x => { x with fNum := fNum, block := block, ksv := ksv }
    syncChanSlots c ch.pair
  else c

def writeC0 (c : Chip) (ci : UInt32) (data : UInt32) : Chip :=
  let fb := (data &&& 0x0e) >>> 1
  let c := c.modChan ci.toNat fun x => { x with fb := fb, con := data &&& 1 }
  let c := updateAlg c ci
  let newm := c.newm
  let m (b : UInt32) : UInt32 := if ((data >>> b) &&& 1) != 0 then 0xffff else 0
  let c := c.modChan ci.toNat fun x =>
    if newm != 0 then { x with cha := m 4, chb := m 5, chc := m 6, chd := m 7 }
    else { x with cha := 0xffff, chb := 0xffff, chc := 0, chd := 0 }
  let ch := c.chan ci
  let c := c.modSlot ch.slot0.toNat fun s => { s with fb := fb }
  let c := c.modSlot ch.slot1.toNat fun s => { s with fb := fb }
  { c with mixDirty := true }

/-- `OPL3_ChannelSet4Op`. -/
def set4Op (c : Chip) (data : UInt32) : Chip := Id.run do
  let mut c := c
  for bit in [0:6] do
    let chnum : UInt32 := if bit >= 3 then bit.toUInt32 + 6 else bit.toUInt32
    if ((data >>> bit.toUInt32) &&& 1) != 0 then
      c := c.modChan chnum.toNat fun x => { x with chtype := ch4op }
      c := c.modChan (chnum + 3).toNat fun x => { x with chtype := ch4op2 }
      c := updateAlg c chnum
    else
      c := c.modChan chnum.toNat fun x => { x with chtype := ch2op }
      c := c.modChan (chnum + 3).toNat fun x => { x with chtype := ch2op }
      c := updateAlg c chnum
      c := updateAlg c (chnum + 3)
  return c

/-- `OPL3_WriteReg`. -/
def writeReg (c : Chip) (reg : UInt32) (v0 : UInt32) : Chip :=
  let v := v0 &&& 0xff
  let high := (reg >>> 8) &&& 1
  let regm := reg &&& 0xff
  let c := if high != 0 then { c with hiTouched := true, mixDirty := true } else c
  let slotIdx := tb adSlot (regm &&& 0x1f)
  let si := (18 * high + slotIdx).toNat
  let chIdx := 9 * high + (regm &&& 0x0f)
  match regm &&& 0xf0 with
  | 0x00 =>
    if high != 0 then
      match regm &&& 0x0f with
      | 0x04 => set4Op c v
      | 0x05 => { c with newm := v &&& 1 }
      | _ => c
    else if (regm &&& 0x0f) == 0x08 then { c with nts := (v >>> 6) &&& 1 } else c
  | 0x20 | 0x30 => if slotIdx != 255 then slotWrite20 c si v else c
  | 0x40 | 0x50 => if slotIdx != 255 then slotWrite40 c si v else c
  | 0x60 | 0x70 => if slotIdx != 255 then slotWrite60 c si v else c
  | 0x80 | 0x90 => if slotIdx != 255 then slotWrite80 c si v else c
  | 0xe0 | 0xf0 => if slotIdx != 255 then slotWriteE0 c si v else c
  | 0xa0 => if (regm &&& 0x0f) < 9 then writeA0 c chIdx v else c
  | 0xb0 =>
    if regm == 0xbd && high == 0 then
      let c := { c with tremoloshift := (((v >>> 7) ^^^ 1) <<< 1) + 2,
                        vibshift := ((v >>> 6) &&& 1) ^^^ 1 }
      updateRhythm c v
    else if (regm &&& 0x0f) < 9 then
      let c := writeB0 c chIdx v
      chanKey c chIdx ((v &&& 0x20) != 0)
    else c
  | 0xc0 => if (regm &&& 0x0f) < 9 then writeC0 c chIdx v else c
  | _ => c

/-! ## Reset -/

/-- `(samplerate << RSM_FRAC) / 49716`, forced to at least 1 (rates below ~49 Hz are meaningless). -/
def rateRatioFor (samplerate : Nat) : Int32 :=
  let r := (samplerate.toUInt32 <<< 10) / 49716
  (if r == 0 then 1 else r).toInt32

/-- `OPL3_Reset(chip, samplerate)`. -/
def Chip.reset (samplerate : Nat) : Chip := Id.run do
  let mut slots : Array Slot := Array.mkEmpty 36
  for i in [0:36] do
    slots := slots.push { slotNum := i.toUInt32 }
  let mut chans : Array Chan := Array.mkEmpty 18
  for ci in [0:18] do
    let lslot := tb chSlot ci.toUInt32
    let pair : UInt32 :=
      if ci % 9 < 3 then (ci + 3).toUInt32 else if ci % 9 < 6 then (ci - 3).toUInt32 else noOut
    chans := chans.push { slot0 := lslot, slot1 := lslot + 3, pair := pair, chNum := ci.toUInt32 }
    slots := slots.modify lslot.toNat fun s => { s with chan := ci.toUInt32 }
    slots := slots.modify (lslot + 3).toNat fun s => { s with chan := ci.toUInt32 }
  let mut wbuf := ByteArray.emptyWithCapacity (writebufSize.toNat * wbEntry)
  for _ in [0:writebufSize.toNat * wbEntry] do
    wbuf := wbuf.push 0
  let mut c : Chip := { slots := slots, chans := chans, wbuf := wbuf,
                        rateratio := rateRatioFor samplerate }
  for ci in [0:18] do
    c := setupAlg c ci.toUInt32
  return c

/-- Change the output rate of the built-in resampler (keeps chip state). -/
def Chip.setRate (c : Chip) (samplerate : Nat) : Chip :=
  { c with rateratio := rateRatioFor samplerate }

/-! ## Sample generation -/

/--
`OPL3_SlotCalcFB` + `OPL3_EnvelopeCalc` + `OPL3_PhaseGenerate` +
`OPL3_SlotGenerate` for one slot. `modOther` is the current value of the
other slot's output when `modKind == modOut` (else 0); `override` is the
rhythm-mode phase output (or `0xffffffff`).
-/
def stepSlot (s : Slot) (modOther : Int32) (trem egAdd egState egTimerLo vibpos vibshift
    override : UInt32) : Slot :=
  -- feedback
  let fbmod : Int32 := if s.fb != 0 then (s.prout + s.out) >>> (9 - s.fb).toInt32 else i32 0
  let prout := s.out
  -- envelope
  let egOut := s.egRout + s.egTlKsl + trem
  let gen := s.egGen
  let key := s.key
  let resetF : UInt32 := if key != 0 then (if gen == 3 then 1 else 0) else 0
  let rb := (s.egRates >>> ((if resetF != 0 then 0 else gen) <<< 3)) &&& 0xff
  let nonzero := (rb &&& 0x80) != 0
  let rateHi := (rb >>> 2) &&& 0x0f
  let rateLo := rb &&& 3
  let egShift := rateHi + egAdd
  let shift : UInt32 :=
    if !nonzero then 0
    else if rateHi < 12 then
      if egState != 0 then
        if egShift == 12 then 1
        else if egShift == 13 then (rateLo >>> 1) &&& 1
        else if egShift == 14 then rateLo &&& 1
        else 0
      else 0
    else
      let sh : UInt32 := (rateHi &&& (3 : UInt32)) + tb egIncstep ((rateLo <<< (2 : UInt32)) + egTimerLo)
      let sh := if (sh &&& 4) != 0 then 3 else sh
      if sh == 0 then egState else sh
  let rout0 := s.egRout
  let offF : UInt32 := if (rout0 &&& 0x1f8) == 0x1f8 then 1 else 0
  let r1 : UInt32 := if resetF != 0 then (if rateHi == 0x0f then 0 else rout0) else rout0
  let r2 : UInt32 :=
    if gen != 0 then (if resetF == 0 then (if offF != 0 then 0x1ff else r1) else r1) else r1
  let lin : UInt32 :=
    if (offF ||| resetF) == 0 then (if shift > 0 then (1 : UInt32) <<< (shift - 1) else 0) else 0
  let inc : UInt32 :=
    if gen == 0 then
      if rout0 == 0 then 0
      else if key != 0 then
        if shift > 0 then
          if rateHi != 0x0f then ((~~~rout0).toInt32 >>> (4 - shift).toInt32).toUInt32 else 0
        else 0
      else 0
    else if gen == 1 then (if (rout0 >>> 4) == s.regSl then 0 else lin)
    else lin
  let gen1 : UInt32 :=
    if gen == 0 then (if rout0 == 0 then 1 else 0)
    else if gen == 1 then (if (rout0 >>> 4) == s.regSl then 2 else 1)
    else gen
  let egRout : UInt32 := (r2 + inc) &&& 0x1ff
  let gen2 : UInt32 := if resetF != 0 then 0 else gen1
  let gen3 : UInt32 := if key == 0 then 3 else gen2
  -- phase
  let phaseInc : UInt32 :=
    if s.regVib != 0 then
      let fNum := s.fNum
      let range0 := (fNum >>> 7) &&& 7
      let range1 : UInt32 := if (vibpos &&& 3) == 0 then 0 else if (vibpos &&& 1) != 0 then range0 >>> 1 else range0
      let range2 := range1 >>> vibshift
      let fNum' : UInt32 := (if (vibpos &&& 4) != 0 then fNum - range2 else fNum + range2) &&& 0xffff
      phaseIncFor fNum' s.block s.regMult
    else s.pgInc
  let phase := (s.pgPhase >>> 9) &&& 0xffff
  let pgPhase := (if resetF != 0 then 0 else s.pgPhase) + phaseInc
  let phaseOut := if override == 0xffffffff then phase else override
  -- output
  let modv : Int32 := if s.modKind == modFb then fbmod else modOther
  let ph := (phaseOut + modv.toUInt32) &&& 0x3ff
  let wf := (logsinWf[((s.regWf <<< 10) ||| ph).toNat]!).toUInt32
  let level0 : UInt32 := (wf &&& (0x7fff : UInt32)) + (egOut <<< (3 : UInt32))
  let level : UInt32 := if level0 > 0x1fff then 0x1fff else level0
  let v : UInt32 := (expromX2[(level &&& 0xff).toNat]!).toUInt32 >>> (level >>> (8 : UInt32))
  let neg : UInt32 := if (wf &&& 0x8000) != 0 then 0xffff else 0
  { s with fbmod := fbmod, prout := prout, egOut := egOut, pgReset := resetF,
           egRout := egRout, egGen := gen3, pgPhase := pgPhase, pgPhaseOut := phaseOut,
           out := sext16 (v ^^^ neg) }

/--
Clock the 23-bit noise LFSR `k <= 9` times at once. New bits enter at bit 22
and are first read back (as tap 14) after 8 more clocks, so up to 9 clocks
only depend on the current state:
`n' = (n >> k) | (((n ^ (n >> 14)) & (2^k - 1)) << (23 - k))`.
-/
@[inline] def noiseAdvK (n : UInt32) (k : UInt32) : UInt32 :=
  (n >>> k) ||| (((n ^^^ (n >>> 14)) &&& ((1 <<< k) - 1)) <<< (23 - k))

/-- Clock the noise LFSR `k` times (any `k`). -/
def noiseAdv (n : UInt32) : Nat → UInt32
  | 0 => n
  | k + 1 =>
    let k := k + 1
    if k >= 8 then noiseAdv (noiseAdvK n 8) (k - 8) else noiseAdvK n k.toUInt32
termination_by k => k
decreasing_by omega

/-- 36 clocks (one chip sample: one per slot). -/
@[inline] def noiseAdv36 (n : UInt32) : UInt32 :=
  noiseAdvK (noiseAdvK (noiseAdvK (noiseAdvK (noiseAdvK n 8) 8) 8) 8) 4

/-- 13 clocks (noise seen by slot 13). -/
@[inline] def noiseAdv13 (n : UInt32) : UInt32 := noiseAdvK (noiseAdvK n 8) 5

@[inline] def outOf (slots : Array Slot) (si : UInt32) : Int32 := (slots[si.toNat]!).out

/--
Fast path of `stepSlot` for a keyed-off, fully attenuated slot
(`key == 0 && eg_rout == 0x1ff`, no rhythm phase override), as in
Nuked-OPL3-fast: the envelope cannot move and the output is just the sign of
the waveform (0 or -1), but feedback, phase and `eg_out` still advance.
-/
def stepSilent (s : Slot) (modOther : Int32) (trem vibpos vibshift : UInt32) : Slot :=
  let fbmod : Int32 := if s.fb != 0 then (s.prout + s.out) >>> (9 - s.fb).toInt32 else i32 0
  let phaseInc : UInt32 :=
    if s.regVib != 0 then
      let fNum := s.fNum
      let range0 := (fNum >>> 7) &&& 7
      let range1 : UInt32 := if (vibpos &&& 3) == 0 then 0 else if (vibpos &&& 1) != 0 then range0 >>> 1 else range0
      let range2 := range1 >>> vibshift
      let fNum' : UInt32 := (if (vibpos &&& 4) != 0 then fNum - range2 else fNum + range2) &&& 0xffff
      phaseIncFor fNum' s.block s.regMult
    else s.pgInc
  let phase := (s.pgPhase >>> 9) &&& 0xffff
  let modv : Int32 := if s.modKind == modFb then fbmod else modOther
  let ph := (phase + modv.toUInt32) &&& 0x3ff
  let wf := (logsinWf[((s.regWf <<< 10) ||| ph).toNat]!).toUInt32
  { s with fbmod := fbmod, prout := s.out, egOut := 0x1ff + s.egTlKsl + trem, pgReset := 0,
           egGen := 3, pgPhase := s.pgPhase + phaseInc, pgPhaseOut := phase,
           out := if (wf &&& 0x8000) != 0 then i32 0xffffffff else i32 0 }

/--
Fast path of `stepSlot` for a keyed-on slot sustaining with a zero sustain
rate (EG type 1, or release rate 0), as in Nuked-OPL3-fast: the envelope
generator cannot move `eg_rout` except for the "envelope off" clamp.
-/
def stepSustain (s : Slot) (modOther : Int32) (trem vibpos vibshift override : UInt32) : Slot :=
  let fbmod : Int32 := if s.fb != 0 then (s.prout + s.out) >>> (9 - s.fb).toInt32 else i32 0
  let egOut := s.egRout + s.egTlKsl + trem
  let egRout : UInt32 := if (s.egRout &&& 0x1f8) == 0x1f8 then 0x1ff else s.egRout
  let phaseInc : UInt32 :=
    if s.regVib != 0 then
      let fNum := s.fNum
      let range0 := (fNum >>> 7) &&& 7
      let range1 : UInt32 := if (vibpos &&& 3) == 0 then 0 else if (vibpos &&& 1) != 0 then range0 >>> 1 else range0
      let range2 := range1 >>> vibshift
      let fNum' : UInt32 := (if (vibpos &&& 4) != 0 then fNum - range2 else fNum + range2) &&& 0xffff
      phaseIncFor fNum' s.block s.regMult
    else s.pgInc
  let phase := (s.pgPhase >>> 9) &&& 0xffff
  let phaseOut := if override == 0xffffffff then phase else override
  let modv : Int32 := if s.modKind == modFb then fbmod else modOther
  let ph := (phaseOut + modv.toUInt32) &&& 0x3ff
  let wf := (logsinWf[((s.regWf <<< 10) ||| ph).toNat]!).toUInt32
  let level0 : UInt32 := (wf &&& (0x7fff : UInt32)) + (egOut <<< (3 : UInt32))
  let level : UInt32 := if level0 > 0x1fff then 0x1fff else level0
  let v : UInt32 := (expromX2[(level &&& 0xff).toNat]!).toUInt32 >>> (level >>> (8 : UInt32))
  let neg : UInt32 := if (wf &&& 0x8000) != 0 then 0xffff else 0
  { s with fbmod := fbmod, prout := s.out, egOut := egOut, pgReset := 0, egRout := egRout,
           pgPhase := s.pgPhase + phaseInc, pgPhaseOut := phaseOut, out := sext16 (v ^^^ neg) }

/--
`OPL3_ProcessSlot` for slot `i`. The slot array is an owned parameter so the
`modify` is in place. `ovr13/16/17` are the rhythm-mode phase outputs of the
hi-hat, snare and top-cymbal slots (`0xffffffff` = none). Inlined with
literal indices by `procBlock0/1`.
-/
@[inline] def procOne (slots : Array Slot) (i : Nat) (egAdd egState egTimerLo vibpos vibshift
    tremolo ovr13 ovr16 ovr17 : UInt32) : Array Slot :=
  let s := slots[i]!
  let modOther : Int32 := if s.modKind == modOut then outOf slots s.modSlot else i32 0
  let trem := if s.tremOn != 0 then tremolo else 0
  let override : UInt32 :=
    if i == 13 then ovr13
    else if i == 16 then ovr16
    else if i == 17 then ovr17
    else 0xffffffff
  if s.key == 0 then
    if s.egRout == 0x1ff && override == 0xffffffff then
      slots.modify i fun s => stepSilent s modOther trem vibpos vibshift
    else
      slots.modify i fun s => stepSlot s modOther trem egAdd egState egTimerLo vibpos vibshift override
  else if s.egGen == 2 && (s.regType != 0 || s.regRr == 0) then
    slots.modify i fun s => stepSustain s modOther trem vibpos vibshift override
  else
    slots.modify i fun s => stepSlot s modOther trem egAdd egState egTimerLo vibpos vibshift override

/-- Slots `[i, stop)`, in order. -/
def procSlots (slots : Array Slot) (i stop : Nat) (egAdd egState egTimerLo vibpos vibshift
    tremolo ovr13 ovr16 ovr17 : UInt32) : Array Slot :=
  if i < stop then
    procSlots (procOne slots i egAdd egState egTimerLo vibpos vibshift tremolo ovr13 ovr16 ovr17)
      (i + 1) stop egAdd egState egTimerLo vibpos vibshift tremolo ovr13 ovr16 ovr17
  else slots
termination_by stop - i

/-- Slots 0..14 with literal indices (the first-bank prefix processed before the left mix). -/
def procBlock0 (slots : Array Slot) (a b c d e f g h k : UInt32) : Array Slot :=
  let slots := procOne slots 0 a b c d e f g h k
  let slots := procOne slots 1 a b c d e f g h k
  let slots := procOne slots 2 a b c d e f g h k
  let slots := procOne slots 3 a b c d e f g h k
  let slots := procOne slots 4 a b c d e f g h k
  let slots := procOne slots 5 a b c d e f g h k
  let slots := procOne slots 6 a b c d e f g h k
  let slots := procOne slots 7 a b c d e f g h k
  let slots := procOne slots 8 a b c d e f g h k
  let slots := procOne slots 9 a b c d e f g h k
  let slots := procOne slots 10 a b c d e f g h k
  let slots := procOne slots 11 a b c d e f g h k
  let slots := procOne slots 12 a b c d e f g h k
  let slots := procOne slots 13 a b c d e f g h k
  procOne slots 14 a b c d e f g h k

/-- Slots 15..17. -/
def procBlock1 (slots : Array Slot) (a b c d e f g h k : UInt32) : Array Slot :=
  let slots := procOne slots 15 a b c d e f g h k
  let slots := procOne slots 16 a b c d e f g h k
  procOne slots 17 a b c d e f g h k

/-- One stereo mix pass over channels `[i, stop)`; `left` selects `cha`/`chb`. -/
def mixPass (slots : Array Slot) (chans : Array Chan) (left : Bool) (i stop : Nat) (acc : Int32) : Int32 :=
  if i < stop then
    let ch := chans[i]!
    let mask := if left then ch.cha else ch.chb
    let cnt := ch.outCnt
    if cnt == 0 then mixPass slots chans left (i + 1) stop acc
    else if mask == 0 then mixPass slots chans left (i + 1) stop acc
    else
      let a := outOf slots ch.out0
      let a := if cnt > 1 then a + outOf slots ch.out1 else a
      let a := if cnt > 2 then a + outOf slots ch.out2 else a
      let a := if cnt > 3 then a + outOf slots ch.out3 else a
      mixPass slots chans left (i + 1) stop (acc + sext16 a.toUInt32)
  else acc
termination_by stop - i

/--
Build the left/right mix lists (see `Chip.mixL`): the slot index of every
channel output that the pass adds (a slot appears twice where a channel lists
it twice, as rhythm channels do). Upstream adds each channel's outputs as an
`int16_t accm`; a slot output is at most 4085 in magnitude, so a 4-output sum
never wraps and the per-channel truncation can be dropped.
-/
def buildMixLists (chans : Array Chan) (nch : Nat) : ByteArray × ByteArray := Id.run do
  let mut l := ByteArray.empty
  let mut r := ByteArray.empty
  for i in [0:nch] do
    let ch := chans[i]!
    let cnt := ch.outCnt
    if cnt != 0 then
      let outs := #[ch.out0, ch.out1, ch.out2, ch.out3]
      for k in [0:cnt.toNat] do
        if ch.cha != 0 then l := l.push (outs[k]!).toUInt8
        if ch.chb != 0 then r := r.push (outs[k]!).toUInt8
  return (l, r)

/-- One mix pass: the sum of the listed slot outputs. -/
def mixList (slots : Array Slot) (list : ByteArray) (i : Nat) (acc : Int32) : Int32 :=
  if h : i < list.size then
    mixList slots list (i + 1) (acc + outOf slots list[i].toUInt32)
  else acc
termination_by list.size - i

@[inline] def clip16 (x : Int32) : Int32 :=
  if x > i32 32767 then i32 32767 else if x < i32 0xffff8000 then i32 0xffff8000 else x

def deBruijnCtz : ByteArray := ⟨#[0, 1, 28, 2, 29, 14, 24, 3, 30, 22, 20, 15, 25, 17, 4, 8,
  31, 27, 13, 23, 21, 19, 16, 7, 26, 12, 18, 6, 11, 5, 10, 9]⟩

/-- Trailing zeros of a nonzero value (de Bruijn multiply); the EG timer's `shift` loop. -/
@[inline] def ctz32 (x : UInt32) : UInt32 :=
  tb deBruijnCtz (((x &&& (0 - x)) * 0x077CB531) >>> 27)

@[inline] def rd8 (b : ByteArray) (i : Nat) : UInt32 := (b.get! i).toUInt32

def rd64 (b : ByteArray) (off : Nat) : UInt64 :=
  let lo := rd8 b off ||| (rd8 b (off + 1) <<< 8) ||| (rd8 b (off + 2) <<< 16) ||| (rd8 b (off + 3) <<< 24)
  let hi := rd8 b (off + 4) ||| (rd8 b (off + 5) <<< 8) ||| (rd8 b (off + 6) <<< 16) ||| (rd8 b (off + 7) <<< 24)
  lo.toUInt64 ||| (hi.toUInt64 <<< 32)

def wr64 (b : ByteArray) (off : Nat) (v : UInt64) : ByteArray :=
  let lo := v.toUInt32
  let hi := (v >>> 32).toUInt32
  b.set! off lo.toUInt8 |>.set! (off + 1) (lo >>> 8).toUInt8 |>.set! (off + 2) (lo >>> 16).toUInt8
   |>.set! (off + 3) (lo >>> 24).toUInt8 |>.set! (off + 4) hi.toUInt8
   |>.set! (off + 5) (hi >>> 8).toUInt8 |>.set! (off + 6) (hi >>> 16).toUInt8
   |>.set! (off + 7) (hi >>> 24).toUInt8

/-- The entry at `writebuf_cur` is due now (flag `0x200` set and `time <= samplecnt`). -/
@[inline] def writeDue (c : Chip) : Bool :=
  let off := c.wbufCur.toNat * wbEntry
  if (rd8 c.wbuf (off + 9) &&& 0x02) == 0 then false
  else rd64 c.wbuf off <= c.wbufSamplecnt

/-- Apply all due buffered writes (the `while` loop at the end of `OPL3_Generate4Ch`). -/
partial def drainWritebuf (c : Chip) : Chip :=
  if writeDue c then
    let off := c.wbufCur.toNat * wbEntry
    let reg := rd8 c.wbuf (off + 8) ||| ((rd8 c.wbuf (off + 9) &&& 1) <<< 8)
    let data := rd8 c.wbuf (off + 10)
    let wb := c.wbuf
    let c := { c with wbuf := emptyBytes }
    let wb := wb.set! (off + 9) ((reg >>> 8) &&& 1).toUInt8
    let c := { c with wbuf := wb }
    let c := writeReg c reg data
    drainWritebuf { c with wbufCur := (c.wbufCur + 1) % writebufSize }
  else c

/--
The single per-sample update of the chip scalars, kept out of line with a
straight-line body so the constructor is reused in place (inlined into the
branchy `genSample` it would be re-allocated every sample).
-/
@[noinline] def Chip.afterSample (st : Chip) (m0 m1 : Int32) (tp tremolo vibpos timer egAdd egTimerLo : UInt32)
    (egTimer : UInt64) (egTimerRem egState noise hh2 hh3 hh7 hh8 tc3 tc5 : UInt32)
    (outL outR decr : Int32) (wbInc : UInt64) : Chip :=
  { st with mix0 := m0, mix1 := m1, tremolopos := tp, tremolo := tremolo, vibpos := vibpos,
            timer := timer, egAdd := egAdd, egTimerLo := egTimerLo, egTimer := egTimer,
            egTimerRem := egTimerRem, egState := egState, noise := noise,
            rmHh2 := hh2, rmHh3 := hh3, rmHh7 := hh7, rmHh8 := hh8, rmTc3 := tc3, rmTc5 := tc5,
            old0 := st.cur0, old1 := st.cur1, cur0 := outL, cur1 := outR,
            samplecnt := st.samplecnt - decr, wbufSamplecnt := st.wbufSamplecnt + wbInc }

/--
`OPL3_Generate4Ch` for the two used outputs, with the slot array carried
separately from the rest of the chip (`st.slots` is ignored); `samplecnt`
is decreased by `decr` (the resampler's bookkeeping). Afterwards
`old0/old1` hold the previous outputs and `cur0/cur1` the new ones
(left = `buf4[0]`, right = `buf4[1]`).
-/
@[inline] def genSample (slots : Array Slot) (st : Chip) (decr : Int32) : Array Slot × Chip :=
  let rhyOn := (st.rhy &&& 0x20) != 0
  -- rhythm bits: hh from slot 13 (always), tc from slot 17 (rhythm mode only)
  let ph13 := ((slots[13]!).pgPhase >>> 9) &&& 0xffff
  let hh2 := (ph13 >>> 2) &&& 1
  let hh3 := (ph13 >>> 3) &&& 1
  let hh7 := (ph13 >>> 7) &&& 1
  let hh8 := (ph13 >>> 8) &&& 1
  let ph17 := ((slots[17]!).pgPhase >>> 9) &&& 0xffff
  let tc3 := if rhyOn then (ph17 >>> 3) &&& 1 else st.rmTc3
  let tc5 := if rhyOn then (ph17 >>> 5) &&& 1 else st.rmTc5
  let n13 := if rhyOn then noiseAdv13 st.noise else 0
  let n16 := noiseAdvK n13 3
  let ovr13 : UInt32 :=
    if rhyOn then
      let x := (hh2 ^^^ hh7) ||| (hh3 ^^^ st.rmTc5) ||| (st.rmTc3 ^^^ st.rmTc5)
      (x <<< 9) ||| (if (x ^^^ (n13 &&& 1)) != 0 then 0xd0 else 0x34)
    else 0xffffffff
  let ovr16 : UInt32 := if rhyOn then (hh8 <<< 9) ||| ((hh8 ^^^ (n16 &&& 1)) <<< 8) else 0xffffffff
  let ovr17 : UInt32 :=
    if rhyOn then
      let x := (hh2 ^^^ hh7) ||| (hh3 ^^^ tc5) ||| (tc3 ^^^ tc5)
      (x <<< 9) ||| 0x80
    else 0xffffffff
  let egAdd := st.egAdd
  let egState := st.egState
  let egTimerLo := st.egTimerLo
  let vibpos := st.vibpos
  let vibshift := st.vibshift
  let tremolo := st.tremolo
  let hi := st.hiTouched
  let st := if st.mixDirty then
      let (l, r) := buildMixLists st.chans (if hi then 18 else 9)
      { st with mixL := l, mixR := r, mixDirty := false,
                mixLow := ⟨l.data.filter (· < 15)⟩, mixHigh := ⟨l.data.filter (· >= 15)⟩ }
    else st
  let outR := clip16 st.mix1
  let slots := procBlock0 slots egAdd egState egTimerLo vibpos vibshift tremolo ovr13 ovr16 ovr17
  -- Without second-bank writes `newm` is 0, so every channel has cha = chb and
  -- the left and right lists are equal; slots below 15 do not change between
  -- the two mix points, so only the entries of slots 15..17 are summed twice.
  let mLow := if hi then i32 0 else mixList slots st.mixLow 0 (i32 0)
  let m0 := if hi then mixList slots st.mixL 0 (i32 0) else mixList slots st.mixHigh 0 mLow
  let slots := procBlock1 slots egAdd egState egTimerLo vibpos vibshift tremolo ovr13 ovr16 ovr17
  let outL := clip16 m0
  let slots := if hi then
      procSlots slots 18 33 egAdd egState egTimerLo vibpos vibshift tremolo ovr13 ovr16 ovr17
    else slots
  let m1 := if hi then mixList slots st.mixR 0 (i32 0) else mixList slots st.mixHigh 0 mLow
  let slots := if hi then
      procSlots slots 33 36 egAdd egState egTimerLo vibpos vibshift tremolo ovr13 ovr16 ovr17
    else slots
  -- LFOs and envelope clock
  let timer := st.timer
  let tp := st.tremolopos
  let tp := if (timer &&& 0x3f) == 0x3f then (if tp + 1 == 210 then 0 else tp + 1) else tp
  let tremolo := if tp < 105 then tp >>> st.tremoloshift else (210 - tp) >>> st.tremoloshift
  let vibpos := if (timer &&& 0x3ff) == 0x3ff then (vibpos + 1) &&& 7 else vibpos
  let egTimer := st.egTimer
  let low := egTimer.toUInt32 &&& 0x1fff
  let egAdd := if egState != 0 then (if low == 0 then 0 else ctz32 low + 1) else egAdd
  let egTimerLo := if egState != 0 then egTimer.toUInt32 &&& 3 else egTimerLo
  let advance := (st.egTimerRem ||| egState) != 0
  let wrap := egTimer == 0xfffffffff
  let egTimer' : UInt64 := if advance then (if wrap then 0 else egTimer + 1) else egTimer
  let egTimerRem : UInt32 := if advance then (if wrap then 1 else 0) else st.egTimerRem
  -- buffered writes due now (decided on the unchanged write-buffer fields)
  let due := writeDue st
  let st := st.afterSample m0 m1 tp tremolo vibpos ((timer + 1) &&& 0xffff) egAdd egTimerLo egTimer'
    egTimerRem (egState ^^^ 1) (noiseAdv36 st.noise) hh2 hh3 hh7 hh8 tc3 tc5 outL outR decr
    (if due then 0 else 1)
  if due then
    let c := drainWritebuf { st with slots := slots }
    let slots := c.slots
    let c := { c with slots := emptySlots, wbufSamplecnt := c.wbufSamplecnt + 1 }
    (slots, c)
  else
    (slots, st)

/-- `OPL3_Generate`: one sample at the native 49716 Hz rate in `cur0`/`cur1`. -/
def Chip.generate (c : Chip) : Chip :=
  let slots := c.slots
  let st := { c with slots := emptySlots }
  let (slots, st) := genSample slots st (i32 0)
  { st with slots := slots }

@[inline] def pushS16 (b : ByteArray) (v : Int32) : ByteArray :=
  let u := v.toUInt32
  (b.push u.toUInt8).push (u >>> 8).toUInt8

/--
`OPL3_GenerateStream` loop: emit `n` resampled frames, generating chip
samples whenever `samplecnt >= rateratio` (`OPL3_Generate4ChResampled`).
-/
@[inline] def setS16 (b : ByteArray) (o : Nat) (v : Int32) : ByteArray :=
  let u := v.toUInt32
  (b.set! o u.toUInt8).set! (o + 1) (u >>> 8).toUInt8

partial def renderLoop (slots : Array Slot) (st : Chip) (out : ByteArray) (o : Nat) (n : Nat) :
    Array Slot × Chip × ByteArray :=
  if n == 0 then (slots, st, out)
  else if st.samplecnt >= st.rateratio then
    let (slots, st) := genSample slots st st.rateratio
    renderLoop slots st out o n
  else
    let r := st.rateratio
    let cnt := st.samplecnt
    let l := (st.old0 * (r - cnt) + st.cur0 * cnt) / r
    let rr := (st.old1 * (r - cnt) + st.cur1 * cnt) / r
    renderLoop slots { st with samplecnt := cnt + i32 1024 } (setS16 (setS16 out o l) (o + 2) rr) (o + 4) (n - 1)

/-- 64 KiB of zeros (output buffers are grown with `copySlice`, i.e. `memcpy`). -/
def zeroBlock : ByteArray := Id.run do
  let mut b := ByteArray.emptyWithCapacity 65536
  for _ in [0:65536] do b := b.push 0
  return b

/-- `out` grown by `n` zero bytes. -/
partial def growZero (out : ByteArray) (n : Nat) : ByteArray :=
  if n == 0 then out
  else
    let k := min n 65536
    growZero (zeroBlock.copySlice 0 out out.size k false) (n - k)

/-- Append `frames` resampled stereo frames (s16le, interleaved) to `out`. -/
def Chip.renderStream (c : Chip) (out : ByteArray) (frames : Nat) : Chip × ByteArray :=
  let slots := c.slots
  let st := { c with slots := emptySlots }
  let o := out.size
  let (slots, st, out) := renderLoop slots st (growZero out (frames * 4)) o frames
  ({ st with slots := slots }, out)

/-- `OPL3_WriteRegBuffered`. -/
def Chip.writeRegBuffered (c : Chip) (reg v : UInt32) : Chip :=
  let last := c.wbufLast
  let off := last.toNat * wbEntry
  let oreg := rd8 c.wbuf (off + 8) ||| (rd8 c.wbuf (off + 9) <<< 8)
  let c :=
    if (oreg &&& 0x200) != 0 then
      let odata := rd8 c.wbuf (off + 10)
      let otime := rd64 c.wbuf off
      let c := writeReg c (oreg &&& 0x1ff) odata
      { c with wbufCur := (last + 1) % writebufSize, wbufSamplecnt := otime }
    else c
  let reg' := (reg &&& 0x1ff) ||| 0x200
  let t1 := c.wbufLasttime + 2
  let t2 := c.wbufSamplecnt
  let time := if t1 < t2 then t2 else t1
  let wb := c.wbuf
  let c := { c with wbuf := emptyBytes }
  let wb := wr64 wb off time
  let wb := wb.set! (off + 8) reg'.toUInt8 |>.set! (off + 9) (reg' >>> 8).toUInt8
    |>.set! (off + 10) v.toUInt8
  { c with wbuf := wb, wbufLasttime := time, wbufLast := (last + 1) % writebufSize }

/-- Unbuffered register write (`OPL3_WriteReg`). -/
def Chip.writeRegNow (c : Chip) (reg v : UInt32) : Chip := writeReg c reg v

end Doom.Audio.Opl
