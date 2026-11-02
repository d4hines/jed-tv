import Memory

/- Deterministic Ethernet responder. The caller supplies stable, disjoint RX/TX
   memory, after DMA has relinquished RX. No device or clock capability here. -/
namespace LeanBoot.Ethernet

variable {m : Type → Type} [Monad m] [Memory.Reader m] [Memory.Writer m]

@[specialize] private def be16 (base size off : UInt32) : m UInt32 := do
  return ((← Memory.read8 base size off) <<< 8) ||| (← Memory.read8 base size (off + 1))

@[specialize] private def put16 (base size off value : UInt32) : m UInt32 := do
  let _ ← Memory.write8 base size off (value >>> 8)
  Memory.write8 base size (off + 1) value

@[inline] private def fold (sum : UInt32) : UInt32 :=
  let s : UInt32 := (sum &&& (65535 : UInt32)) + (sum >>> (16 : UInt32))
  (s &&& (65535 : UInt32)) + (s >>> (16 : UInt32))

@[specialize] private partial def sumBytes (base size off count sum : UInt32) : m UInt32 := do
  if count == 0 then return fold sum
  if count == 1 then return fold (sum + ((← Memory.read8 base size off) <<< 8))
  sumBytes base size (off + 2) (count - 2) (sum + (← be16 base size off))

@[specialize] private def mac (base size off : UInt32) : m UInt64 := do
  let a ← Memory.readBE32 base size off
  let b ← be16 base size (off + 4)
  return (a <<< 16) ||| b.toUInt64

@[specialize] private def putMac (base size off : UInt32) (value : UInt64) : m UInt32 := do
  let _ ← Memory.writeBE32 base size off (value >>> 16).toUInt32
  put16 base size (off + 4) value.toUInt32

/-- 0: ignore; otherwise the exact response length. Only ARP, unfragmented
    IPv4 ICMP echo and UDP echo port 7533 are accepted. Input is never changed.
    Rejections perform no output writes. Hardware FCS validation is required. -/
@[specialize] def respond (rx length tx capacity ip : UInt32) (own : UInt64) : m UInt32 := do
  if length < 42 || length > 1514 || capacity < 1514 ||
      !Memory.addressable rx length || !Memory.addressable tx capacity ||
      !(tx + capacity <= rx || rx + length <= tx) then return 0
  let dest ← mac rx length 0
  let source ← mac rx length 6
  if source == 0 || source == 0xffffffffffff || (source >>> 40) &&& 1 != 0 then return 0
  let kind ← be16 rx length 12
  if kind == 0x0806 then
    if dest != own && dest != 0xffffffffffff then return 0
    if (← Memory.readBE32 rx length 14) != 0x00010800 ||
        (← Memory.readBE32 rx length 18) != 0x06040001 ||
        (← Memory.readBE32 rx length 38) != ip.toUInt64 ||
        (← mac rx length 22) != source then return 0
    let _ ← Memory.fill tx capacity 0 60 0
    let _ ← putMac tx capacity 0 source
    let _ ← putMac tx capacity 6 own
    let _ ← put16 tx capacity 12 0x0806
    let _ ← Memory.writeBE32 tx capacity 14 0x00010800
    let _ ← Memory.writeBE32 tx capacity 18 0x06040002
    let _ ← putMac tx capacity 22 own
    let _ ← Memory.writeBE32 tx capacity 28 ip
    let _ ← putMac tx capacity 32 source
    let _ ← Memory.copy tx capacity 38 rx length 28 4
    return 60
  if kind != 0x0800 || dest != own || (← Memory.read8 rx length 14) != 0x45 then return 0
  let total ← be16 rx length 16
  if total < 28 || total > 1500 || total + 14 > length ||
      (← be16 rx length 20) &&& 0xbfff != 0 ||
      (← Memory.read8 rx length 22) == 0 ||
      (← Memory.readBE32 rx length 30) != ip.toUInt64 ||
      (← sumBytes rx length 14 20 0) != 65535 then return 0
  let remote := (← Memory.readBE32 rx length 26).toUInt32
  if remote == 0 || remote == ip || remote >>> 24 >= 224 then return 0
  let protocol ← Memory.read8 rx length 23
  let payload := total - 20
  if protocol == 1 then
    if (← be16 rx length 34) != 0x0800 ||
        (← sumBytes rx length 34 payload 0) != 65535 then return 0
  else if protocol == 17 then
    if (← be16 rx length 36) != 7533 || (← be16 rx length 38) != payload ||
        (← be16 rx length 34) == 0 then return 0
    let pseudo := fold remote + fold ip + 17 + payload
    if (← be16 rx length 40) != 0 && (← sumBytes rx length 34 payload pseudo) != 65535 then return 0
  else return 0
  let count := max 60 (total + 14)
  let _ ← Memory.fill tx capacity 0 count 0
  let _ ← Memory.copy tx capacity 0 rx length 0 (total + 14)
  let _ ← putMac tx capacity 0 source
  let _ ← putMac tx capacity 6 own
  let _ ← Memory.writeBE32 tx capacity 26 ip
  let _ ← Memory.writeBE32 tx capacity 30 remote
  let _ ← Memory.write8 tx capacity 22 64
  let _ ← put16 tx capacity 24 0
  let _ ← put16 tx capacity 24 (65535 - (← sumBytes tx capacity 14 20 0))
  if protocol == 1 then
    let _ ← put16 tx capacity 34 0
    let _ ← put16 tx capacity 36 0
    let _ ← put16 tx capacity 36 (65535 - (← sumBytes tx capacity 34 payload 0))
    return count
  let port ← be16 rx length 34
  let _ ← put16 tx capacity 34 7533
  let _ ← put16 tx capacity 36 port
  let _ ← put16 tx capacity 40 0
  let checksum := 65535 - (← sumBytes tx capacity 34 payload (fold ip + fold remote + 17 + payload))
  let _ ← put16 tx capacity 40 (if checksum == 0 then 65535 else checksum)
  return count

/-- Finish a replacement UDP payload in an already validated/swapped response.
    The caller owns tx and has written exactly payload bytes starting at 42. -/
@[specialize] def finishUdp (tx capacity payload : UInt32) : m UInt32 := do
  if capacity < 1514 || payload > 1472 || !Memory.addressable tx capacity then return 0
  let total := payload + 28
  let count := max 60 (total + 14)
  let _ ← Memory.fill tx capacity (total + 14) (count - total - 14) 0
  let _ ← put16 tx capacity 16 total
  let _ ← put16 tx capacity 24 0
  let _ ← put16 tx capacity 24 (65535 - (← sumBytes tx capacity 14 20 0))
  let _ ← put16 tx capacity 38 (payload + 8)
  let _ ← put16 tx capacity 40 0
  let source := (← Memory.readBE32 tx capacity 26).toUInt32
  let dest := (← Memory.readBE32 tx capacity 30).toUInt32
  let checksum := 65535 - (← sumBytes tx capacity 34 (payload + 8)
    (fold source + fold dest + 17 + payload + 8))
  let _ ← put16 tx capacity 40 (if checksum == 0 then 65535 else checksum)
  return count

end LeanBoot.Ethernet
