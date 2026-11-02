import Unsafe.UpdateCrypto
import Update.Install
import Update.Receiver
import Update.Server

/- Model-only foreign storage operations. There is deliberately no physical
   eMMC implementation or bank address map in this target. -/
namespace LeanBoot.Unsafe.UpdateModel
@[extern "leanboot_model_read"]
private opaque rawRead (bank sector target : UInt32) : BaseIO Bool
@[extern "leanboot_model_write"]
private opaque rawWrite (bank sector source : UInt32) : BaseIO Bool
@[extern "leanboot_model_flush"]
private opaque rawFlush : BaseIO Bool
@[always_inline] instance : Update.Storage HardwareM where
  read := fun b s p _ => rawRead b s p
  write := fun b s p _ => rawWrite b s p
  flush := fun _ => rawFlush

@[inline] def policy : Update.Policy := {
  board := 0x00130001, component := 1, minimumVersion := 1
  capacity := Update.maxPayload }

@[export leanboot_update_install]
def install (source size scratch capacity : UInt32) : BaseIO UInt32 :=
  runHardware <| Update.installNext policy source size scratch capacity

@[export leanboot_update_bank]
def bank (which scratch capacity : UInt32) : BaseIO UInt64 :=
  runHardware <| Update.withBank policy which scratch capacity (pure 0)
    (fun version _ entry => pure ((version.toUInt64 <<< 32) ||| entry.toUInt64))

@[export leanboot_update_receive]
def receive (state buffer capacity packet size : UInt32) : BaseIO UInt64 :=
  runMemory <| Update.Receiver.receive state buffer capacity packet size

@[export leanboot_update_serve]
def serve (state buffer packet size : UInt32) : BaseIO UInt32 :=
  runHardware <| Update.Server.respond policy state buffer (Update.maxPayload + 160)
    packet size 0x23002000 0x21000000 (Update.maxPayload + 4096)

end LeanBoot.Unsafe.UpdateModel
