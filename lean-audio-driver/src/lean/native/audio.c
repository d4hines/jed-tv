// ByteArray and String marshalling for the Lean audio driver's mechanisms.
// The I/O itself is Rust (src/pid1/src/audio_service.rs); this file only
// moves bytes between Lean objects and plain pointers, using lean.h's own
// accessors so that no object layout is duplicated anywhere else. Lean 4.35
// erases the IO world token from these signatures.
#include <lean/lean.h>
#include <stdint.h>

// Rust: 0 on success, otherwise a result word (TV.Audio.outcome).
uint64_t tv_audio_recv_bytes(uint8_t *buffer, uint32_t len);
uint64_t tv_audio_send_bytes(const uint8_t *buffer, uint32_t len);
void tv_audio_log_bytes(const uint8_t *text, uint32_t len);
uint64_t tv_audio_pcm_write_bytes(uint32_t handle, const uint8_t *buffer, uint32_t len);
uint64_t tv_audio_aout_connected_into(uint32_t output, uint32_t count, uint32_t *handles);
uint64_t tv_audio_aout_volume_into(uint32_t output, int32_t *pair);

static void put_u64(uint8_t *data, uint64_t word) {
    for (int i = 0; i < 8; i++) data[i] = (uint8_t)(word >> (8 * i));
}

static void put_u32(uint8_t *data, uint32_t value) {
    for (int i = 0; i < 4; i++) data[i] = (uint8_t)(value >> (8 * i));
}

/* Exactly len bytes, or an empty array when the socket closed, timed out or failed. */
lean_obj_res tv_audio_recv_exact(uint32_t len) {
    lean_obj_res bytes = lean_alloc_sarray(1, len, len);
    if (len != 0 && tv_audio_recv_bytes(lean_sarray_cptr(bytes), len) != 0) {
        lean_dec(bytes);
        bytes = lean_alloc_sarray(1, 0, 0);
    }
    return lean_io_result_mk_ok(bytes);
}

lean_obj_res tv_audio_send(b_lean_obj_arg bytes) {
    uint64_t status = tv_audio_send_bytes(lean_sarray_cptr(bytes), (uint32_t)lean_sarray_size(bytes));
    return lean_io_result_mk_ok(lean_box_uint64(status));
}

lean_obj_res tv_audio_log(b_lean_obj_arg message) {
    tv_audio_log_bytes((const uint8_t *)lean_string_cstr(message), (uint32_t)(lean_string_size(message) - 1));
    return lean_io_result_mk_ok(lean_box(0));
}

lean_obj_res tv_audio_pcm_write(uint32_t handle, b_lean_obj_arg bytes, uint32_t offset, uint32_t len) {
    size_t size = lean_sarray_size(bytes);
    uint64_t word;
    if ((size_t)offset > size || (size_t)len > size - offset) {
        word = 2 | ((uint64_t)22 << 32); /* EINVAL: a Lean bug, not the hardware */
    } else {
        word = tv_audio_pcm_write_bytes(handle, lean_sarray_cptr(bytes) + offset, len);
    }
    return lean_io_result_mk_ok(lean_box_uint64(word));
}

/* An 8-byte result word, then count handles in native (little-endian) order. */
lean_obj_res tv_audio_aout_connected(uint32_t output, uint32_t count) {
    size_t size = 8 + 4 * (size_t)count;
    lean_obj_res bytes = lean_alloc_sarray(1, size, size);
    uint8_t *data = lean_sarray_cptr(bytes);
    uint64_t word = tv_audio_aout_connected_into(output, count, (uint32_t *)(data + 8));
    put_u64(data, word);
    return lean_io_result_mk_ok(bytes);
}

/* An 8-byte result word, then the integer and thousandths dB as 32-bit values. */
lean_obj_res tv_audio_aout_volume(uint32_t output) {
    int32_t pair[2] = {0, 0};
    uint64_t word = tv_audio_aout_volume_into(output, pair);
    lean_obj_res bytes = lean_alloc_sarray(1, 16, 16);
    uint8_t *data = lean_sarray_cptr(bytes);
    put_u64(data, word);
    put_u32(data + 8, (uint32_t)pair[0]);
    put_u32(data + 12, (uint32_t)pair[1]);
    return lean_io_result_mk_ok(bytes);
}
