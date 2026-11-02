import Yt.Crypto.Util
/-! AES-128/256 encryption (FIPS 197), the forward direction only (GCM
needs nothing else). Table-driven: one 256-entry T-table (the other three
are its byte rotations) and the S-box, both computed at module
initialisation. Table lookups are indexed by secret bytes, so this is not
constant-time against cache-timing observers on the same machine. -/
namespace Yt.Crypto.Aes

@[inline] def xtime (x : UInt8) : UInt8 := (x <<< 1) ^^^ (if x &&& 0x80 != 0 then 0x1b else 0)
@[inline] def rotl8 (x : UInt8) (n : UInt8) : UInt8 := (x <<< n) ||| (x >>> (8 - n))

/-- The S-box, generated from the field inverse and the affine map. -/
def sbox : ByteArray := Id.run do
  let mut s := zeros 256
  let mut p : UInt8 := 1
  let mut q : UInt8 := 1
  for _ in [0:255] do
    p := p ^^^ xtime p
    q := q ^^^ (q <<< 1)
    q := q ^^^ (q <<< 2)
    q := q ^^^ (q <<< 4)
    if q &&& 0x80 != 0 then q := q ^^^ 0x09
    s := s.set! p.toNat (q ^^^ rotl8 q 1 ^^^ rotl8 q 2 ^^^ rotl8 q 3 ^^^ rotl8 q 4 ^^^ 0x63)
  s.set! 0 0x63

/-- `te[x]` = (2·S[x], S[x], S[x], 3·S[x]) as a big-endian word. -/
def te : Array UInt32 := Id.run do
  let mut t : Array UInt32 := Array.mkEmpty 256
  for x in [0:256] do
    let s := sbox[x]!
    let s2 := xtime s
    t := t.push ((s2.toUInt32 <<< 24) ||| (s.toUInt32 <<< 16) ||| (s.toUInt32 <<< 8) ||| (s2 ^^^ s).toUInt32)
  t

@[inline] def rotr (x : UInt32) (n : UInt32) : UInt32 := (x >>> n) ||| (x <<< (32 - n))
@[inline] def T0 (x : UInt32) : UInt32 := te[(x >>> 24).toNat]!
@[inline] def T1 (x : UInt32) : UInt32 := rotr te[((x >>> 16) &&& 0xff).toNat]! 8
@[inline] def T2 (x : UInt32) : UInt32 := rotr te[((x >>> 8) &&& 0xff).toNat]! 16
@[inline] def T3 (x : UInt32) : UInt32 := rotr te[(x &&& 0xff).toNat]! 24
@[inline] def S (x : UInt32) : UInt32 := (sbox[(x &&& 0xff).toNat]!).toUInt32

/-- An expanded key: `4 * (rounds + 1)` round-key words. -/
structure Key where
  rk : Array UInt32
  rounds : Nat
deriving Inhabited

@[inline] def subWord (x : UInt32) : UInt32 :=
  (S (x >>> 24) <<< 24) ||| (S (x >>> 16) <<< 16) ||| (S (x >>> 8) <<< 8) ||| S x

/-- Key expansion for a 16- or 32-byte key (other lengths: treated as 16). -/
def expand (key : ByteArray) : Key := Id.run do
  let nk := if key.size == 32 then 8 else 4
  let nr := nk + 6
  let mut w : Array UInt32 := Array.mkEmpty (4 * (nr + 1))
  for i in [0:nk] do w := w.push (getU32BE key (4 * i))
  let mut rcon : UInt32 := 1
  for i in [nk:4 * (nr + 1)] do
    let mut t := w[i-1]!
    if i % nk == 0 then
      t := subWord (rotr t 24) ^^^ (rcon <<< 24)
      rcon := (xtime rcon.toUInt8).toUInt32
    else if nk == 8 && i % nk == 4 then
      t := subWord t
    w := w.push (w[i-nk]! ^^^ t)
  ⟨w, nr⟩

/-- One block: four big-endian words. -/
structure Block where
  w0 : UInt32
  w1 : UInt32
  w2 : UInt32
  w3 : UInt32
deriving Inhabited

partial def roundsLoop (rk : Array UInt32) (last : Nat) (r : Nat) (s0 s1 s2 s3 : UInt32) : Block :=
  if r < last then
    let k := 4 * r
    let t0 := T0 s0 ^^^ T1 s1 ^^^ T2 s2 ^^^ T3 s3 ^^^ rk[k]!
    let t1 := T0 s1 ^^^ T1 s2 ^^^ T2 s3 ^^^ T3 s0 ^^^ rk[k+1]!
    let t2 := T0 s2 ^^^ T1 s3 ^^^ T2 s0 ^^^ T3 s1 ^^^ rk[k+2]!
    let t3 := T0 s3 ^^^ T1 s0 ^^^ T2 s1 ^^^ T3 s2 ^^^ rk[k+3]!
    roundsLoop rk last (r + 1) t0 t1 t2 t3
  else
    let k := 4 * last
    let f (a b c d : UInt32) : UInt32 :=
      (S (a >>> 24) <<< 24) ||| (S (b >>> 16) <<< 16) ||| (S (c >>> 8) <<< 8) ||| S d
    ⟨f s0 s1 s2 s3 ^^^ rk[k]!, f s1 s2 s3 s0 ^^^ rk[k+1]!,
     f s2 s3 s0 s1 ^^^ rk[k+2]!, f s3 s0 s1 s2 ^^^ rk[k+3]!⟩

/-- Encrypt one block given as four big-endian words. -/
@[inline] def encryptWords (k : Key) (a b c d : UInt32) : Block :=
  roundsLoop k.rk k.rounds 1 (a ^^^ k.rk[0]!) (b ^^^ k.rk[1]!) (c ^^^ k.rk[2]!) (d ^^^ k.rk[3]!)

/-- Encrypt the 16-byte block of `b` at `o`. -/
def encryptBlock (k : Key) (b : ByteArray) (o : Nat := 0) : ByteArray :=
  let r := encryptWords k (getU32BE b o) (getU32BE b (o+4)) (getU32BE b (o+8)) (getU32BE b (o+12))
  pushU32BE (pushU32BE (pushU32BE (pushU32BE (ByteArray.emptyWithCapacity 16) r.w0) r.w1) r.w2) r.w3

end Yt.Crypto.Aes
