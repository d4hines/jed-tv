import ZrAbi.C

namespace ZrAbi.Platform
open C

def header : Header := [
  .pragmaOnce, .include "stddef.h", .include "stdint.h", .include "lean/lean.h",
  .prototype (.named "uint32_t") "zr_hw_read32" [⟨"addr", .named "uint32_t"⟩],
  .prototype (.named "lean_obj_res") "zr_hw_write32" [⟨"addr", .named "uint32_t"⟩, ⟨"value", .named "uint32_t"⟩],
  .prototype (.named "lean_obj_res") "zr_hw_dmb" [],
  .prototype (.named "lean_obj_res") "zr_hw_dsb" [],
  .prototype (.named "lean_obj_res") "zr_hw_isb" [],
  .prototype (.named "lean_obj_res") "zr_hw_dc_clean" [⟨"mva", .named "uint32_t"⟩],
  .prototype (.named "lean_obj_res") "zr_hw_dc_inval" [⟨"mva", .named "uint32_t"⟩],
  .prototype (.named "lean_obj_res") "zr_hw_dc_clean_inval" [⟨"mva", .named "uint32_t"⟩],
  .prototype (.named "lean_obj_res") "zr_hw_copy_in" [⟨"addr", .named "uint32_t"⟩, ⟨"len", .named "uint32_t"⟩],
  .prototype (.named "lean_obj_res") "zr_hw_copy_out" [⟨"addr", .named "uint32_t"⟩, ⟨"bytes", .named "b_lean_obj_arg"⟩, ⟨"off", .named "uint32_t"⟩, ⟨"len", .named "uint32_t"⟩],
  .prototype (.named "uint64_t") "zr_hw_timer" [],
  .prototype (.named "void") "zr_start_l1_invalidate" [],
  .prototype (.named "void") "zr_start_vfp_scu" [],
  .prototype (.named "void") "zr_start_clean_for_walks" [⟨"p", .ptr (.const (.named "void"))⟩, ⟨"len", .named "uint32_t"⟩],
  .prototype (.named "void") "zr_start_tlb_flush" [],
  .prototype (.named "void") "zr_start_mmu_on" [⟨"l1", .ptr (.const (.named "uint32_t"))⟩],
  .prototype (.named "void") "zr_start_scu_on" [],
  .prototype (.named "lean_obj_res") "zr_hw_dcci_all" [],
  .prototype (.named "lean_obj_res") "zr_hw_copy" [⟨"dst", .named "uint32_t"⟩, ⟨"src", .named "uint32_t"⟩, ⟨"len", .named "uint32_t"⟩],
  .prototype (.named "lean_obj_res") "zr_hw_zero" [⟨"dst", .named "uint32_t"⟩, ⟨"len", .named "uint32_t"⟩],
  .prototype (.named "uint8_t") "zr_fw_heap_init" [⟨"lo", .named "size_t"⟩, ⟨"hi", .named "size_t"⟩],
  .prototype (.named "size_t") "zr_fw_heap_alloc" [⟨"size", .named "size_t"⟩, ⟨"align", .named "size_t"⟩],
  .prototype (.named "uint8_t") "zr_fw_heap_free" [⟨"ptr", .named "size_t"⟩, ⟨"size", .named "size_t"⟩, ⟨"align", .named "size_t"⟩]
]

end ZrAbi.Platform
