import Init

/- First RAM-only bootloader experiment. All output and timeout decisions live
   here. Main.lean supplies normalized UART status and performs the requested
   byte store through Unsafe. No pointers, allocation, storage or OS services.

   State: byte index in bits 0..7, unsuccessful polls in bits 8..31.
   Status: bit 0 = TX can accept a byte, bit 1 = transmitter completely idle.
   Result: next state in the high word, action in the low word:
     0 = poll again, 0x100|byte = transmit, 0x200 = return success,
     0x201 = return timeout, 0x202 = return invalid-state error.
   Return actions are terminal: the adapter must return to its caller.
   The poll budget is an instruction-count bound, not a wall-clock timeout. -/
namespace LeanBoot

def markerLength : UInt32 := 14
def pollLimit : UInt32 := 1048576

-- ASCII "LEANBOOT RAM\r\n"; scalar literals avoid a runtime String object.
@[inline] def markerByte (index : UInt32) : UInt32 :=
  if index == 0 then 76 else if index == 1 then 69
  else if index == 2 then 65 else if index == 3 then 78
  else if index == 4 then 66 else if index == 5 then 79
  else if index == 6 then 79 else if index == 7 then 84
  else if index == 8 then 32 else if index == 9 then 82
  else if index == 10 then 65 else if index == 11 then 77
  else if index == 12 then 13 else 10

@[inline] def result (state action : UInt32) : UInt64 :=
  (state.toUInt64 <<< 32) ||| action.toUInt64

@[export leanboot_step]
def step (state status : UInt32) : UInt64 :=
  let index := state &&& 255
  let polls := state >>> 8
  if index > markerLength || polls > pollLimit then result state 0x202
  else if index == markerLength && status &&& 2 != 0 then result state 0x200
  else if index < markerLength && status &&& 1 != 0 then
    result (index + 1) (0x100 ||| markerByte index)
  else if polls == pollLimit then result state 0x201
  else result (state + 256) 0

end LeanBoot
