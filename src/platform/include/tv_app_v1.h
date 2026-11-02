#ifndef TV_APP_V1_H
#define TV_APP_V1_H
#include <stdint.h>
#include <stddef.h>
#ifdef __cplusplus
extern "C" {
#endif
/* Opaque, single-owner handles. Calls on one handle must be serialized. */
typedef struct TvDisplay TvDisplay;
typedef struct TvAudio TvAudio;
typedef struct TvInput TvInput;
typedef struct {
    uint32_t *pixels;
    uint32_t width, height, stride_bytes;
    uint32_t red_shift, green_shift, blue_shift, opaque_alpha;
} TvFrame;
/* Unless documented otherwise: 0 = success, negative errno = failure. */
uint32_t tv_app_api_version(void); /* 0x00010000 */
int32_t tv_display_open(TvDisplay **out);
int32_t tv_display_acquire(TvDisplay *, TvFrame *out);
/* Consume the lease. Publication does not promise physical scanout. */
int32_t tv_display_present(TvDisplay *);
void tv_display_cancel(TvDisplay *);
void tv_display_close(TvDisplay *);
int32_t tv_audio_open(uint32_t rate, uint32_t channels, TvAudio **out);
/* Signed 16-bit interleaved PCM; count is samples, not bytes or frames. */
int32_t tv_audio_write(TvAudio *, const int16_t *samples, size_t count);
int32_t tv_audio_queued(TvAudio *, int64_t *frames);
int32_t tv_audio_reset(TvAudio *);
int32_t tv_audio_drain(TvAudio *);
void tv_audio_close(TvAudio *);
int32_t tv_input_open(TvInput **out);
/* Nonblocking: 1 = event, 0 = none; value 0 release, 1 press, 2 repeat. */
int32_t tv_input_poll(TvInput *, uint32_t *linux_keycode, int32_t *value);
void tv_input_close(TvInput *);
#ifdef __cplusplus
}
#endif
#endif
