import ZrAsm.Stubs

/-!
# The Lean runtime's entry points, bound to ZrLow

`lean_alloc_small(sz, slot)` and `lean_alloc_object(sz)` (src/c/zr_runtime.c)
are tail branches to the ZrLow allocator's exports
(src/lean/ZrLean/Low/Alloc.lean): `sz` is the first argument of both, the
result the same pointer, and the unused slot index is simply left in r1. Linked
by the firmware and the FSBL (`runtime.S`) except in the builds that count
allocations (`ZR_RT_COUNT_COPIES`, the test features), which keep the C
functions that count and then call the same exports.

The object entry points have matching pointer/word ABIs too, including the
byte-sized flags. Returning an ignored byte from a void function is harmless
under AAPCS; the branch keeps the original caller's return address.
-/

namespace ZrAsm.Runtime
open Reg Asm

/-- Lean represents Int8/Int16 as unsigned bits. AAPCS signed returns must
be extended to a full register at the C ABI boundary. -/
def signedReturn (name target : String) (shift : Nat) : Routine := routine name do
  push [r4, lr]
  bl target
  mov r0 (.reg r0 (.lsl shift))
  mov r0 (.reg r0 (.asr shift))
  pop [r4, pc]


def routines : List Routine :=
  stubs [("lean_alloc_small", "zr_low_alloc_small"), ("lean_alloc_object", "zr_low_alloc_object"),
    ("lean_dec_ref_cold", "zr_low_dec_ref_cold"),
    ("lean_free_object", "zr_low_free_object"),
    ("lean_mark_persistent", "zr_low_mark_persistent"),
    ("lean_mk_array", "zr_low_mk_array"),
    ("lean_copy_expand_array_nonlinear", "zr_low_copy_expand_array_nonlinear"),
    ("lean_array_push", "zr_low_array_push"),
    ("lean_copy_sarray_nonlinear", "zr_low_copy_sarray_nonlinear"),
    ("lean_byte_array_push", "zr_low_byte_array_push"),
    ("lean_byte_array_mk", "zr_low_byte_array_mk"),
    ("lean_byte_array_copy_slice", "zr_low_byte_array_copy_slice"),
    ("lean_string_to_utf8", "zr_low_string_to_utf8"),
    ("lean_uint8_once_cold", "zr_low_uint8_once_cold"),
    ("lean_uint16_once_cold", "zr_low_uint16_once_cold"),
    ("lean_uint32_once_cold", "zr_low_uint32_once_cold"),
    ("lean_uint64_once_cold", "zr_low_uint64_once_cold"),
    ("lean_usize_once_cold", "zr_low_usize_once_cold"),
    ("lean_float_once_cold", "zr_low_float_once_cold"),
    ("lean_float32_once_cold", "zr_low_float32_once_cold"),
    ("lean_obj_once_cold", "zr_low_obj_once_cold"),
    ("zr_rt_nat_of", "zr_low_nat_of"),
    ("zr_rt_nat_get", "zr_low_nat_get"),
    ("lean_big_usize_to_nat", "zr_low_big_usize_to_nat"),
    ("lean_big_uint64_to_nat", "zr_low_big_uint64_to_nat"),
    ("zr_lean_big_unsigned_to_nat", "zr_low_big_unsigned_to_nat"),
    ("lean_cstr_to_nat", "zr_low_cstr_to_nat"),
    ("lean_nat_big_add", "zr_low_nat_big_add"),
    ("lean_nat_big_sub", "zr_low_nat_big_sub"),
    ("lean_nat_big_mul", "zr_low_nat_big_mul"),
    ("lean_nat_big_div", "zr_low_nat_big_div"),
    ("lean_nat_big_mod", "zr_low_nat_big_mod"),
    ("lean_nat_big_land", "zr_low_nat_big_land"),
    ("lean_nat_big_lor", "zr_low_nat_big_lor"),
    ("lean_nat_big_xor", "zr_low_nat_big_xor"),
    ("lean_nat_big_eq", "zr_low_nat_big_eq"),
    ("lean_nat_big_lt", "zr_low_nat_big_lt"),
    ("lean_nat_big_le", "zr_low_nat_big_le"),
    ("lean_nat_big_shiftr", "zr_low_nat_big_shiftr"),
    ("lean_nat_gcd", "zr_low_nat_gcd"),
    ("lean_nat_pow", "zr_low_nat_pow"),
    ("lean_nat_shiftl", "zr_low_nat_shiftl"),
    ("lean_nat_overflow_mul", "zr_low_nat_overflow_mul"),
    ("lean_nat_big_div_exact", "zr_low_nat_big_div"),
    ("lean_uint8_of_big_nat", "zr_low_uint8_of_big_nat"),
    ("lean_uint16_of_big_nat", "zr_low_uint16_of_big_nat"),
    ("lean_uint32_of_big_nat", "zr_low_uint32_of_big_nat"),
    ("lean_int32_of_big_int", "zr_low_int32_of_big_int"),
    ("lean_uint64_of_big_nat", "zr_low_uint64_of_big_nat"),
    ("lean_int64_of_big_int", "zr_low_int64_of_big_int"),
    ("lean_usize_of_big_nat", "zr_low_usize_of_big_nat"),
    ("zr_rt_apply", "zr_low_apply"),
    ("lean_apply_1", "zr_low_apply_1"),
    ("lean_apply_2", "zr_low_apply_2"),
    ("lean_apply_3", "zr_low_apply_3"),
    ("lean_apply_4", "zr_low_apply_4"),
    ("lean_apply_5", "zr_low_apply_5"),
    ("lean_apply_6", "zr_low_apply_6"),
    ("lean_apply_7", "zr_low_apply_7"),
    ("lean_apply_8", "zr_low_apply_8"),
    ("lean_apply_9", "zr_low_apply_9"),
    ("lean_apply_10", "zr_low_apply_10"),
    ("lean_apply_11", "zr_low_apply_11"),
    ("lean_apply_12", "zr_low_apply_12"),
    ("lean_apply_13", "zr_low_apply_13"),
    ("lean_apply_14", "zr_low_apply_14"),
    ("lean_apply_15", "zr_low_apply_15"),
    ("lean_apply_16", "zr_low_apply_16"),
    ("zr_low_fatal", "zr_low_runtime_fatal"),
    ("zr_low_oom", "zr_low_runtime_oom"),
    ("lean_internal_panic", "zr_low_internal_panic"),
    ("lean_internal_panic_out_of_memory", "zr_low_runtime_oom"),
    ("lean_internal_panic_overflow", "zr_low_runtime_overflow"),
    ("zr_rt_lean_panic", "zr_low_lean_panic"),
    ("zr_rt_panic_count", "zr_low_panic_count"),
    ("lean_array_get_panic", "zr_low_array_get_panic"),
    ("lean_array_set_panic", "zr_low_array_set_panic"),
    ("lean_initialize_runtime_module", "zr_low_runtime_module_init"),
    ("lean_io_mark_end_initialization", "zr_low_runtime_module_init"),
    ("initialize_Init", "zr_low_initialize_Init"),
    -- These modules contribute only types/proofs to our native programs.
    -- Their executable dependencies must remain satisfied explicitly below;
    -- never generalize this audited list to arbitrary imported initializers.
    ("initialize_Init_System_IO", "zr_low_initialize_Init"),
    ("initialize_Init_Data_Int_OfNat", "zr_low_initialize_Init"),
    ("initialize_Init_Data_List_Lemmas", "zr_low_initialize_Init"),
    ("initialize_Init_Omega", "zr_low_initialize_Init"),
    ("initialize_Init_Data_UInt", "zr_low_initialize_Init"),
    ("initialize_Init_Data_String_Basic", "zr_low_initialize_Init"),
    ("initialize_Init_Data_List_Basic", "zr_low_initialize_Init"),
    ("initialize_Init_Control_Basic", "zr_low_initialize_Init"),
    ("initialize_Init_Control_Except", "zr_low_initialize_Init"),
    ("initialize_Init_Data_ByteArray_Basic", "zr_low_initialize_Init"),
    ("initialize_Init_Data_SInt_Float32", "zr_low_initialize_Init"),
    ("initialize_Init_Data_Range_Basic", "zr_low_initialize_Init"),
    ("initialize_Init_Data_Int_DivMod_Basic", "zr_low_initialize_Init"),
    ("initialize_tv_TV_CoreProofs", "zr_low_initialize_Init"),
    ("lean_st_mk_ref", "zr_low_st_mk_ref"),
    ("lean_st_ref_get", "zr_low_st_ref_get"),
    ("lean_st_ref_take", "zr_low_st_ref_take"),
    ("lean_st_ref_put", "zr_low_st_ref_put"),
    ("lean_st_ref_swap", "zr_low_st_ref_swap"),
    -- ZrLean.Low.Str (dande-tv): what apps need beyond the router's firmware
    ("lean_apply_n", "zr_low_apply"),
    ("lean_apply_m", "zr_low_apply"),
    ("lean_array_mk", "zr_low_array_mk"),
    ("lean_array_to_list", "zr_low_array_to_list"),
    ("lean_sarray_eq_cold", "zr_low_sarray_eq_cold"),
    ("lean_byte_array_data", "zr_low_byte_array_data"),
    ("lean_nat_log2", "zr_low_nat_log2"),
    ("lean_panic_fn", "zr_low_panic_fn"),
    ("lean_panic_fn_borrowed", "zr_low_panic_fn_borrowed"),
    ("lean_copy_string_nonlinear", "zr_low_copy_string_nonlinear"),
    ("lean_string_push", "zr_low_string_push"),
    ("lean_string_data", "zr_low_string_data"),
    ("lean_string_mk", "zr_low_string_mk"),
    ("lean_string_append", "zr_low_string_append"),
    ("lean_string_eq_cold", "zr_low_string_eq_cold"),
    ("lean_string_lt", "zr_low_string_lt"),
    ("lean_string_compare", "zr_low_string_compare"),
    ("lean_string_from_utf8_unchecked", "zr_low_string_from_utf8_unchecked"),
    ("lean_string_validate_utf8", "zr_low_string_validate_utf8"),
    -- ZrLean.Low.InitShim: upstream Init names (signatures: ZrInitCheck.lean)
    ("l_outOfBounds___redArg", "zr_low_shim_out_of_bounds"),
    ("l_Char_ofNat", "zr_low_shim_char_of_nat"),
    ("l_Nat_reprFast", "zr_low_shim_nat_repr"),
    ("l_ByteArray_extract", "zr_low_shim_byte_array_extract"),
    ("l_Array_reverse___redArg", "zr_low_shim_array_reverse"),
    ("l_Array_extract___redArg", "zr_low_shim_array_extract"),
    ("l_Array_append___redArg", "zr_low_shim_array_append"),
    ("l_Array_instInhabited___redArg", "zr_low_shim_array_inhabited"),
    ("l_Int_toNat", "zr_low_shim_int_to_nat"),
    ("l_Int_pow", "zr_low_shim_int_pow"),
    ("l_Int_shiftRight", "zr_low_shim_int_shift_right"),
    ("l_Int_shiftLeft", "zr_low_shim_int_shift_left"),
    ("l_List_beq___at___00Lean_Syntax_instBEqPreresolved_beq_spec__0", "zr_low_shim_list_string_eq"),
    ("l_List_reverse___redArg", "zr_low_shim_list_reverse"),
    ("l_List_appendTR___redArg", "zr_low_shim_list_append"),
    ("l_List_range", "zr_low_shim_list_range"),
    ("l_List_replicateTR___redArg", "zr_low_shim_list_replicate"),
    ("l_List_zipIdxTR___redArg", "zr_low_shim_list_zip_idx"),
    ("l_String_Slice_Pos_nextn", "zr_low_shim_string_slice_nextn"),
    ("l_String_Slice_toString", "zr_low_shim_string_slice_to_string"),
    ("lean_string_memcmp", "zr_low_shim_string_memcmp"),
    ("lean_mk_string_unchecked", "zr_low_shim_mk_string_unchecked"),
    ("lean_mk_string_from_bytes", "zr_low_shim_mk_string_from_bytes"),
    ("l_ByteArray_isEmpty", "zr_low_shim_byte_array_is_empty"),
    ("l_String_intercalate", "zr_low_string_intercalate"),
    ("l_Int_repr", "zr_low_int_repr"),
    ("lean_string_utf8_get_fast", "zr_low_string_utf8_get_fast"),
    ("lean_string_utf8_next_fast", "zr_low_string_utf8_next_fast"),
    ("l_List_get_x3fInternal___redArg", "zr_low_shim_list_get_opt"),
    ("l_List_getD___redArg", "zr_low_shim_list_get_d"),
    ("l_List_lengthTR___redArg", "zr_low_shim_list_length"),
    ("l_List_drop___redArg", "zr_low_shim_list_drop"),
    ("l_List_isEmpty___redArg", "zr_low_shim_list_is_empty"),
    ("l_List_foldl___at___00Array_appendList_spec__0___redArg", "zr_low_shim_array_append_list"),
    ("l_instDecidableEqNat___boxed", "zr_low_shim_nat_dec_eq_boxed"),
    ("l_Option_instDecidableEq___redArg", "zr_low_shim_option_dec_eq"),
    ("l_List_modifyTR___redArg", "zr_low_shim_list_modify"),
    ("l_List_MergeSort_Internal_mergeSortTR_u2082___redArg", "zr_low_shim_list_merge_sort")] ++
  [signedReturn "lean_int8_of_big_int" "zr_low_int8_of_big_int" 24,
   signedReturn "lean_int16_of_big_int" "zr_low_int16_of_big_int" 16] ++
  [storage "zr_the_heap" (198 * 4), storage "zr_mark_stack" (64 * 4),
   storage "zr_nat_literals" (3 * 4), storage "zr_runtime_state" (2 * 4),
   storage "l_ByteArray_empty" 4,
   -- `Int.instInhabited`, the closed term `(0 : Int)`: lean_box(0), never written
   roWords "l_Int_instInhabited" [1]]

end ZrAsm.Runtime
