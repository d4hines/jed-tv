import Types

/- Descriptor decoding is data validation, not a hardware operation. The
   adapter supplies stable words after its DMA barrier and owns the buffers. -/
namespace LeanBoot.Ethernet.Descriptor

@[inline] def ready (word : UInt32) : Bool := word &&& 1 != 0

@[inline] def expected (index : UInt32) : UInt32 :=
  0x06401000 + index * 2048 + (if index == 3 then 2 else 0)

@[inline] def validAddress (index word : UInt32) : Bool :=
  index < 4 && word &&& 0xfffffffe == expected index

/-- Drop the four FCS bytes only after validating the complete frame length.
    Invalid descriptors cannot call the packet parser. -/
@[inline] def withPayload {α : Type} (word : UInt32) (discard : α)
    (accept : ByteCount → α) : α :=
  let length := word &&& 2047
  if length >= 4 && length <= 1518 then accept ⟨length - 4⟩ else discard

end LeanBoot.Ethernet.Descriptor
