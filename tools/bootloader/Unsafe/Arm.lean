import ZrAsm.Dsl

/- All authored assembly lives here. Policy and all loops are compiled Lean.
   No routine calls MBOOT. Terminal transitions do not return. -/
namespace LeanBoot.Unsafe.Arm
open ZrAsm ZrAsm.Reg ZrAsm.Asm

def load8 : Routine := routine "leanboot_load8" do
  ins (.ldr true r0 { rn := r0, off := .imm true 0, idx := .offset })
  bx lr

def store8 : Routine := routine "leanboot_store8" do
  ins (.str true r1 { rn := r0, off := .imm true 0, idx := .offset })
  mov r0 (imm 0)
  bx lr

def load32 : Routine := routine "leanboot_load32" do
  ldr r0 r0 0
  bx lr

def store32 : Routine := routine "leanboot_store32" do
  str r1 r0 0
  mov r0 (imm 0)
  bx lr

def load16 : Routine := routine "leanboot_load16" do
  ins (.ldrh r0 { rn := r0, off := .imm true 0, idx := .offset })
  bx lr

def store16 : Routine := routine "leanboot_store16" do
  ins (.strh r1 { rn := r0, off := .imm true 0, idx := .offset })
  mov r0 (imm 0)
  bx lr

def readSctlr : Routine := routine "leanboot_read_sctlr" do
  mrc Cp15.sctlr r0
  bx lr

def writeSctlr : Routine := routine "leanboot_write_sctlr" do
  mcr Cp15.sctlr r0
  isb
  mov r0 (imm 0)
  bx lr

def readCpsr : Routine := routine "leanboot_read_cpsr" do
  ins (.mrsCpsr r0)
  bx lr

def maskInterrupts : Routine := routine "leanboot_mask_interrupts" do
  ins (.mrsCpsr r0)
  ins (.cps false true true)
  bx lr

def restoreInterrupts : Routine := routine "leanboot_restore_interrupts" do
  ins (.msrCpsr false true (.reg r0))
  mov r0 (imm 0)
  bx lr

def readSp : Routine := routine "leanboot_read_sp" do
  mov r0 sp
  bx lr

def osStart : Routine := routine "leanboot_os_start" do
  push [r4, lr]
  mov r4 sp
  loadImm sp 0x26340000
  bl "leanboot_os_main"
  mov sp r4
  pop [r4, pc]

def ethernetStart : Routine := routine "leanboot_ethernet_start" do
  push [r4, lr]
  mov r4 sp
  loadImm sp 0x26340000
  bl "leanboot_ethernet_main"
  mov sp r4
  pop [r4, pc]

-- Authenticated network code runs on a separate stack and may return.
def callNetwork : Routine := routine "leanboot_call_network" do
  push [r4, lr]
  mov r4 sp
  mov r3 r0
  mov r0 (imm 0)
  mcr Cp15.iciallu r0
  mcr Cp15.bpiall r0
  dsb
  isb
  mov r0 r1
  loadImm sp 0x26360000
  ins (.blx r3)
  mov sp r4
  pop [r4, pc]

def kernelProbeStart : Routine := routine "leanboot_kernel_probe_start" do
  loadImm sp 0x26340000
  ins (.b (.label "leanboot_kernel_probe_main"))

-- One-word RAM-only override of the final MBOOT kernel target load. The host
-- verifies the original instruction first; it never writes this to firmware.
def kernelProbeRedirect : Routine := routine "leanboot_kernel_probe_redirect" do
  mov r3 (imm 0x26000000)

def imageProbeRedirect : Routine := routine "leanboot_image_probe_redirect" do
  mov r3 (imm 0x40000000)

def bundleStart : Routine := routine "leanboot_bundle_start" do
  loadImm sp 0x26340000
  ins (.b (.label "leanboot_bundle_copy"))

def gateRun : Routine := routine "leanboot_gate_run" do
  ins (.b (.label "leanboot_gate_main"))

def readClidr : Routine := routine "leanboot_read_clidr" do
  mrc ⟨15, 1, 0, 0, 1⟩ r0
  bx lr

def readCcsidr : Routine := routine "leanboot_read_ccsidr" do
  mrc ⟨15, 1, 0, 0, 0⟩ r0
  bx lr

def selectCache : Routine := routine "leanboot_select_cache" do
  mcr Cp15.csselr r0
  isb
  mov r0 (imm 0)
  bx lr

def cleanSetWay : Routine := routine "leanboot_clean_setway" do
  mcr Cp15.dccisw r0
  mov r0 (imm 0)
  bx lr

def barrier : Routine := routine "leanboot_barrier" do
  dsb
  isb
  mov r0 (imm 0)
  bx lr

def beginTransition : Routine := routine "leanboot_begin_transition" do
  ins (.cps false true true)
  dsb
  mcr Cp15.sctlr r0
  isb
  loadImm sp 0x26310000
  mov r0 r1
  mov r1 r2
  ins (.b (.label "leanboot_after_cache_off"))

def sendVerifiedBoot : Routine := routine "leanboot_send_verified_boot" do
  push [r4, r5, r6, r7, r8, r9, r10, r11, r12, lr]
  mov r1 r0
  loadImm r0 0xb200585e
  ins (.smc 0)
  pop [r4, r5, r6, r7, r8, r9, r10, r11, r12, pc]

def jumpLinux : Routine := routine "leanboot_jump_linux" do
  push [r0, r1]
  mov r0 (imm 0)
  mcr Cp15.iciallu r0
  mcr Cp15.bpiall r0
  dsb
  isb
  loadImm r0 0x8200000f
  ins (.smc 0)
  ins (.cps false true true)
  pop [r3, r4]
  mov r0 (imm 0)
  loadImm r1 0xffffffff
  mov r2 r4
  bx r3

def qemuStart : Routine := routine "leanboot_qemu_start" do
  loadImm sp 0x46410000
  ins (.b (.label "leanboot_qemu_main"))

def qemuTransition : Routine := routine "leanboot_qemu_transition" do
  ins (.cps false true true)
  dsb
  mcr Cp15.sctlr r0
  isb
  loadImm sp 0x46310000
  mov r0 r1
  mov r1 r2
  ins (.b (.label "leanboot_qemu_after_cache_off"))

def qemuJump : Routine := routine "leanboot_qemu_jump" do
  mov r3 r0
  mov r2 r1
  mov r0 (imm 0)
  mcr Cp15.iciallu r0
  mcr Cp15.bpiall r0
  dsb
  isb
  loadImm r1 0xffffffff
  bx r3

end LeanBoot.Unsafe.Arm

def main : IO Unit := do
  IO.print "/* Generated from Unsafe/Arm.lean using vendored ZrAsm. */\n.syntax unified\n.arch_extension sec\n"
  for r in [LeanBoot.Unsafe.Arm.load8, LeanBoot.Unsafe.Arm.store8,
            LeanBoot.Unsafe.Arm.load32, LeanBoot.Unsafe.Arm.store32,
            LeanBoot.Unsafe.Arm.load16, LeanBoot.Unsafe.Arm.store16,
            LeanBoot.Unsafe.Arm.readSctlr, LeanBoot.Unsafe.Arm.writeSctlr, LeanBoot.Unsafe.Arm.readClidr,
            LeanBoot.Unsafe.Arm.readCpsr, LeanBoot.Unsafe.Arm.readSp,
            LeanBoot.Unsafe.Arm.maskInterrupts, LeanBoot.Unsafe.Arm.restoreInterrupts,
            LeanBoot.Unsafe.Arm.osStart,
            LeanBoot.Unsafe.Arm.ethernetStart,
            LeanBoot.Unsafe.Arm.callNetwork,
            LeanBoot.Unsafe.Arm.kernelProbeStart,
            LeanBoot.Unsafe.Arm.kernelProbeRedirect,
            LeanBoot.Unsafe.Arm.imageProbeRedirect,
            LeanBoot.Unsafe.Arm.bundleStart, LeanBoot.Unsafe.Arm.gateRun,
            LeanBoot.Unsafe.Arm.readCcsidr, LeanBoot.Unsafe.Arm.selectCache,
            LeanBoot.Unsafe.Arm.cleanSetWay, LeanBoot.Unsafe.Arm.barrier,
            LeanBoot.Unsafe.Arm.sendVerifiedBoot,
            LeanBoot.Unsafe.Arm.beginTransition, LeanBoot.Unsafe.Arm.jumpLinux,
            LeanBoot.Unsafe.Arm.qemuStart, LeanBoot.Unsafe.Arm.qemuTransition,
            LeanBoot.Unsafe.Arm.qemuJump] do
    IO.print (ZrAsm.printItems r.items)
