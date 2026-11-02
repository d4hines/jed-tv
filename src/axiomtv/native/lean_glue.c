/* Lean FFI for Axiom on the TV: unpacks Lean strings and byte arrays and
 * wraps IO results around the agfx_* functions of ../gfx (Rust). Nothing else
 * lives here; the game is Lean and the drawing is Rust. */
#include <lean/lean.h>
#include <stdint.h>
#include <stddef.h>

int32_t agfx_open(int32_t remote_fd, uint32_t rate);
uint32_t agfx_poll_key(void);
void agfx_begin_frame(void);
void agfx_rect(int32_t x, int32_t y, int32_t w, int32_t h, uint32_t rgba);
void agfx_gradient_v(int32_t x, int32_t y, int32_t w, int32_t h, uint32_t top, uint32_t bottom);
void agfx_gradient_h(int32_t x, int32_t y, int32_t w, int32_t h, uint32_t left, uint32_t right);
void agfx_circle(int32_t x, int32_t y, float r, uint32_t rgba);
void agfx_line(int32_t x0, int32_t y0, int32_t x1, int32_t y1, uint32_t rgba);
void agfx_text(int32_t x, int32_t y, uint32_t scale, uint32_t rgba, const uint8_t *s, size_t len);
void agfx_begin_3d(float px, float py, float pz, float tx, float ty, float tz, float fovy);
void agfx_section(uint32_t ox, uint32_t oy, uint32_t oz, uint32_t size, const uint8_t *faces, size_t len);
void agfx_cube(float cx, float cy, float cz, float sx, float sy, float sz, uint32_t rgba);
void agfx_cube_wires(float cx, float cy, float cz, float size, uint32_t rgba);
void agfx_plane(float cx, float cy, float cz, float w, float d, uint32_t rgba);
int32_t agfx_present(void);
int32_t agfx_audio_write(const int16_t *samples, size_t count);
uint32_t agfx_now_ms(void);

static lean_obj_res unit(void) { return lean_io_result_mk_ok(lean_box(0)); }
static lean_obj_res u32(uint32_t v) { return lean_io_result_mk_ok(lean_box_uint32(v)); }

lean_obj_res atv_open(uint32_t remote_fd, uint32_t rate, lean_obj_arg w) {
    return u32((uint32_t)agfx_open((int32_t)remote_fd, rate));
}
lean_obj_res atv_poll_key(lean_obj_arg w) { return u32(agfx_poll_key()); }
lean_obj_res atv_now_ms(lean_obj_arg w) { return u32(agfx_now_ms()); }
lean_obj_res atv_begin_frame(lean_obj_arg w) { agfx_begin_frame(); return unit(); }
lean_obj_res atv_rect(uint32_t x, uint32_t y, uint32_t wd, uint32_t h, uint32_t rgba, lean_obj_arg w) {
    agfx_rect((int32_t)x, (int32_t)y, (int32_t)wd, (int32_t)h, rgba); return unit();
}
lean_obj_res atv_gradient_v(uint32_t x, uint32_t y, uint32_t wd, uint32_t h, uint32_t top, uint32_t bottom, lean_obj_arg w) {
    agfx_gradient_v((int32_t)x, (int32_t)y, (int32_t)wd, (int32_t)h, top, bottom); return unit();
}
lean_obj_res atv_gradient_h(uint32_t x, uint32_t y, uint32_t wd, uint32_t h, uint32_t left, uint32_t right, lean_obj_arg w) {
    agfx_gradient_h((int32_t)x, (int32_t)y, (int32_t)wd, (int32_t)h, left, right); return unit();
}
lean_obj_res atv_circle(uint32_t x, uint32_t y, float r, uint32_t rgba, lean_obj_arg w) {
    agfx_circle((int32_t)x, (int32_t)y, r, rgba); return unit();
}
lean_obj_res atv_line(uint32_t x0, uint32_t y0, uint32_t x1, uint32_t y1, uint32_t rgba, lean_obj_arg w) {
    agfx_line((int32_t)x0, (int32_t)y0, (int32_t)x1, (int32_t)y1, rgba); return unit();
}
lean_obj_res atv_text(uint32_t x, uint32_t y, uint32_t scale, uint32_t rgba, b_lean_obj_arg s, lean_obj_arg w) {
    agfx_text((int32_t)x, (int32_t)y, scale, rgba, (const uint8_t *)lean_string_cstr(s), lean_string_size(s) - 1);
    return unit();
}
lean_obj_res atv_begin_3d(float px, float py, float pz, float tx, float ty, float tz, float fovy, lean_obj_arg w) {
    agfx_begin_3d(px, py, pz, tx, ty, tz, fovy); return unit();
}
lean_obj_res atv_section(uint32_t ox, uint32_t oy, uint32_t oz, uint32_t size, b_lean_obj_arg faces, lean_obj_arg w) {
    agfx_section(ox, oy, oz, size, lean_sarray_cptr(faces), lean_sarray_size(faces)); return unit();
}
lean_obj_res atv_cube(float cx, float cy, float cz, float sx, float sy, float sz, uint32_t rgba, lean_obj_arg w) {
    agfx_cube(cx, cy, cz, sx, sy, sz, rgba); return unit();
}
lean_obj_res atv_cube_wires(float cx, float cy, float cz, float size, uint32_t rgba, lean_obj_arg w) {
    agfx_cube_wires(cx, cy, cz, size, rgba); return unit();
}
lean_obj_res atv_plane(float cx, float cy, float cz, float wd, float d, uint32_t rgba, lean_obj_arg w) {
    agfx_plane(cx, cy, cz, wd, d, rgba); return unit();
}
lean_obj_res atv_present(lean_obj_arg w) { return u32((uint32_t)agfx_present()); }
/* Little-endian signed 16-bit mono samples. */
lean_obj_res atv_audio_write(b_lean_obj_arg pcm, lean_obj_arg w) {
    return u32((uint32_t)agfx_audio_write((const int16_t *)lean_sarray_cptr(pcm), lean_sarray_size(pcm) / 2));
}
