namespace ShimTest
def line (o : ByteArray) (s : String) : ByteArray := o ++ (s ++ "\n").toUTF8
def bytes (b : ByteArray) : String := Id.run do
  let mut s := "["
  for x in b do s := s ++ toString x.toNat ++ ","
  s ++ "]"
def nats (a : Array Nat) : String := Id.run do
  let mut s := "["
  for x in a do s := s ++ toString x ++ ","
  s ++ "]"
@[export zr_test_transcript] def transcript (k : Nat) : ByteArray := Id.run do
  let mut o := ByteArray.empty
  for n in #[0, 1, 9, 10, 99, 127, 128, 12345, 2147483647, 2147483648, 4294967295, 4294967296,
            18446744073709551615, 18446744073709551616, 340282366920938463463374607431768211455] do
    o := line o ("repr " ++ toString (n + k))
  for n in #[0, 65, 0xd7ff, 0xd800, 0xdfff, 0xe000, 0x10ffff, 0x110000, 4294967296] do
    o := line o ("char " ++ toString (Char.ofNat (n + k)).toNat)
  let b : ByteArray := ⟨#[10, 20, 30, 40, 50]⟩
  o := line o ("get! " ++ toString b[2 + k]!.toNat ++ " " ++ toString b[7 + k]!.toNat)
  for (i, j) in #[(0, 5), (1, 3), (3, 1), (4, 9), (6, 8), (0, 0)] do
    o := line o ("extract " ++ bytes (b.extract (i + k) (j + k)))
  let a : Array Nat := #[1, 2, 3, 4, 5, 6, 7].map (· + k)
  o := line o ("reverse " ++ nats a.reverse ++ nats (#[] : Array Nat).reverse ++ nats #[k].reverse)
  for (i, j) in #[(0, 7), (2, 5), (5, 2), (3, 100), (9, 12), (0, 0)] do
    o := line o ("array-extract " ++ nats (a.extract (i + k) (j + k)))
  let mut grow : Array Nat := Array.mkEmpty 8
  grow := grow.push (1 + k)
  o := line o ("append " ++ nats (a ++ a) ++ nats (a ++ #[]) ++ nats ((#[] : Array Nat) ++ a) ++ nats (grow ++ #[2, 3]) ++ nats grow)
  let shared := a
  o := line o ("reverse-shared " ++ nats shared.reverse ++ nats a)
  o
end ShimTest
