#ifndef TV_CORE_EFFECTS_H
#define TV_CORE_EFFECTS_H
#include <stdint.h>
#include <lean/lean.h>
/* TV.Core is the sole controller. Calls execute in a scoped context on its
 * registered Lean thread. See TV/Core.lean for scalar wire encodings.
 * IO world tokens are erased by the pinned Lean compiler. */
lean_obj_res tv_core_next(void);
lean_obj_res tv_core_clock(void);
lean_obj_res tv_core_load(void);
lean_obj_res tv_core_save(uint32_t pin, uint32_t enabled);
lean_obj_res tv_core_show_home(void);
lean_obj_res tv_core_show_app(uint32_t id);
lean_obj_res tv_core_menu_key(uint8_t kind, uint32_t code, uint8_t held);
lean_obj_res tv_core_menu_apps(uint32_t enabled);
lean_obj_res tv_core_tell(uint8_t kind, uint32_t id);
lean_obj_res tv_core_start(uint32_t id);
lean_obj_res tv_core_signal(uint32_t id, uint8_t kill);
lean_obj_res tv_core_forward(uint32_t id, uint32_t code, uint8_t value);
lean_obj_res tv_core_power_off(void);
lean_obj_res tv_core_request_byte(uint32_t index);
lean_obj_res tv_core_reply(uint32_t id, uint8_t granted);
/* Lean-constructed IO results: native code does not inspect object layout. */
lean_obj_res tv_core_io_word(uint64_t word);
lean_obj_res tv_core_io_unit(uint32_t ignored);
lean_obj_res tv_core_io_error(uint32_t error);
lean_obj_res tv_core_main(void);
#endif
