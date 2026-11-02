import Os

/- Host-only pure interpreters. Neither imports nor instantiates the physical
   RAM adapter. The exact same generic core runs here and in the ARM image. -/
open LeanBoot

structure Region where
  base : UInt32
  capacity : Nat
  bytes : Array UInt8
  writable : Bool
  deriving BEq, Inhabited

structure Arena where
  regions : Array Region
  fault : Bool := false
  writes : Nat := 0
  callbacks : Nat := 0
  config : Array UInt32 := #[]
  deriving BEq

def contains (r : Region) (a : UInt32) : Bool :=
  a >= r.base && (a - r.base).toNat < r.capacity

def peek (s : Arena) (a : UInt32) : UInt8 := Id.run do
  for r in s.regions do
    if contains r a then return r.bytes[(a - r.base).toNat]?.getD 0
  return 0

def valid (s : Arena) (a : UInt32) : Bool := s.regions.any (contains · a)

def word (s : Arena) (a : UInt32) : UInt32 :=
  (peek s a).toUInt32 ||| ((peek s (a+1)).toUInt32 <<< 8) |||
  ((peek s (a+2)).toUInt32 <<< 16) ||| ((peek s (a+3)).toUInt32 <<< 24)

def poke (s : Arena) (a : UInt32) (v : UInt8) (fixture := false) : Arena := Id.run do
  for i in [:s.regions.size] do
    let r := s.regions[i]!
    if contains r a then
      if !r.writable && !fixture then return {s with fault := true}
      let offset := (a-r.base).toNat
      let bytes := if offset < r.bytes.size then r.bytes
        else r.bytes ++ Array.replicate (offset+1-r.bytes.size) 0
      let r := {r with bytes := bytes.set! offset v}
      return {s with regions := s.regions.set! i r, writes := s.writes + (if fixture then 0 else 1)}
  return {s with fault := true}

abbrev ReadModel := ReaderT Arena Id
abbrev WriteModel := StateM Arena

@[always_inline] instance : Memory.Reader ReadModel where
  byte := fun a s => peek s a
  word := fun a s => word s a

@[always_inline] instance : Memory.Reader WriteModel where
  byte := fun a s => (peek s a, {s with fault := s.fault || !valid s a})
  word := fun a s => (word s a, {s with fault := s.fault || !valid s a || !valid s (a+3)})

@[always_inline] instance : Memory.Writer WriteModel where
  byte := fun a v s => (0, poke s a v)
  word := fun a v s =>
    (0, poke (poke (poke (poke s a v.toUInt8) (a+1) (v >>> 8).toUInt8)
      (a+2) (v >>> 16).toUInt8) (a+3) (v >>> 24).toUInt8)

-- This definition type-checks with Reader alone: no Writer instance exists.
def inspectOnly (s : Arena) : UInt32 :=
  (Os.inspect true fun _ p => pure p.kernel.entry : ReadModel UInt32) s

def runCore (s : Arena) : UInt32 × Arena :=
  (Os.withPrepared true fun c => do
    modify fun a => {a with
      callbacks := a.callbacks+1
      config := #[c.entry,c.fdt,c.initrd,c.initrdSize,c.secureMessage]}
    pure 77 : WriteModel UInt32) s

def setWord (s : Arena) (a v : UInt32) : Arena :=
  poke (poke (poke (poke s a v.toUInt8 true) (a+1) (v >>> 8).toUInt8 true)
    (a+2) (v >>> 16).toUInt8 true) (a+3) (v >>> 24).toUInt8 true

def require (b : Bool) (message : String) : IO Unit :=
  unless b do throw (IO.userError message)

def main (args : List String) : IO Unit := do
  let dir : System.FilePath := args.head!
  let mut regions := #[]
  for (name,base,capacity,writable) in #[
      ("boot",0x40207000,0x2000000,false), ("vendor",0x44000000,0x1400000,true),
      ("init",0x45400000,0x800000,false), ("control",0x46200000,65536,false)] do
    let bytes ← IO.FS.readBinFile (dir / (name ++ ".bin"))
    regions := regions.push {base := base, capacity := capacity, bytes := bytes.data, writable := writable}
  let initial : Arena := {regions := regions.push {base := 0x43000000, capacity := 65536, bytes := #[], writable := true}}
  require (inspectOnly initial == 0x40208000) "reader-only inspection failed"
  let (result, final) := runCore initial
  require (result == 77 && !final.fault && final.callbacks == 1) "pure preparation/callback failed"
  require (runCore initial == (result,final)) "pure replay differs"
  require (final.config == #[0x40208000,0x43000000,0x44000000,word initial 0x4620002c,0]) "wrong kernel configuration"
  for (address,value,expected) in #[(0x40207000,0,4), (0x46200010,0xffffffff,2),
      (0x46200024,0xfffffff0,3), (0x4620002c,0,13), (0x46200020,32769,3)] do
    let bad := setWord initial address value
    let (rc,after) := runCore bad
    require (rc == expected && !after.fault && after.writes == 0 && after.callbacks == 0)
      s!"invalid input caused effects: {address}, rc {rc}"
  for r in final.regions do
    if r.base == 0x43000000 then IO.FS.writeBinFile (dir / "model.dtb") ⟨r.bytes⟩
    if r.base == 0x44000000 then
      IO.FS.writeBinFile (dir / "model-initrd.bin") ⟨r.bytes.extract 0 final.config[3]!.toNat⟩
  IO.println "PASS: reader-only inspection, deterministic pure-state preparation, one callback, five side-effect-free rejections"
