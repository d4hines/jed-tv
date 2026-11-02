/* Byte transport only. Parsing, layout, glyph rasterization, compositing and
 * navigation are Lean. Handles and framebuffer leases belong to this process. */
#include <lean/lean.h>
#include "tv_app_v1.h"
#include <errno.h>
#include <stdint.h>
/* Slides is a single-threaded event loop; honor its one-thread sandbox. */
void __wrap_lean_init_task_manager(void) { lean_init_task_manager_using(0); }
lean_object *__wrap_lean_run_main(lean_object *(*main_fn)(int, char **), int argc, char **argv) {
    return main_fn(argc, argv);
}
static TvDisplay *display;
static TvInput *input;
static lean_obj_res result(int32_t n) { return lean_io_result_mk_ok(lean_box_uint32((uint32_t)n)); }
lean_obj_res slides_open(void) {
    int32_t r = tv_display_open(&display);
    if (!r) r = tv_input_open(&input);
    if (r && display) { tv_display_close(display); display = NULL; }
    return result(r);
}
lean_obj_res slides_present(b_lean_obj_arg bytes) {
    if (!display || lean_sarray_size(bytes) != 1280u * 720u * 4u) return result(-EINVAL);
    TvFrame frame;
    int32_t r = tv_display_acquire(display, &frame);
    if (r) return result(r);
    const uint8_t *src = lean_sarray_cptr(bytes);
    for (uint32_t y = 0; y < frame.height; y++) {
        uint32_t *row = (uint32_t *)((uint8_t *)frame.pixels + (size_t)y * frame.stride_bytes);
        for (uint32_t x = 0; x < frame.width; x++) {
            const uint8_t *p = src + ((size_t)(y * 720u / frame.height) * 1280u + x * 1280u / frame.width) * 4u;
            row[x] = frame.opaque_alpha | (uint32_t)p[2] << frame.red_shift |
                     (uint32_t)p[1] << frame.green_shift | (uint32_t)p[0] << frame.blue_shift;
        }
    }
    return result(tv_display_present(display));
}
lean_obj_res slides_poll(void) {
    uint32_t key = 0; int32_t value = 0;
    int32_t r = tv_input_poll(input, &key, &value);
    if (r < 0) return result(0);
    return result(r == 1 ? (int32_t)(0x80000000u | ((uint32_t)value << 16) | key) : 0);
}
lean_obj_res slides_close(void) {
    if (input) { tv_input_close(input); input = NULL; }
    if (display) { tv_display_close(display); display = NULL; }
    return lean_io_result_mk_ok(lean_box(0));
}
