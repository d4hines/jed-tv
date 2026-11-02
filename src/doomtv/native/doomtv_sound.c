/* doomtv_sound.c: Doom sound effects for doomtv (doomgeneric's DG_sound_module).
 *
 * The engine starts/stops/pans sounds on up to NUM_CHANNELS channels (engine
 * thread); doomtv's audio thread pulls mixed audio with doomtv_sound_mix() and
 * writes it to ALSA (platform::alsa). `lock` guards the channel table only;
 * sample data is cached PU_STATIC by the engine thread before use and never
 * freed.
 * Effects are DMX lumps (DS*): 8-bit unsigned mono, usually 11025 Hz; they are
 * resampled linearly to the output rate. Music (MUS -> OPL/MIDI) is not
 * implemented: DG_music_module is silent. */
#include <pthread.h>
#include <stdint.h>
#include <string.h>
#include "doomtype.h"
#include "i_sound.h"
#include "m_misc.h"
#include "w_wad.h"
#include "z_zone.h"

#define NUM_CHANNELS 16

/* Config variables that i_sound.c binds when FEATURE_SOUND is set (they belong
 * to the SDL backend we do not build). */
int use_libsamplerate = 0;
float libsamplerate_scale = 0.65f;

void I_InitTimidityConfig(void)
{
}

typedef struct {
    const uint8_t *samples; /* unsigned 8-bit, mono */
    uint32_t len;
    uint32_t rate;
} sfx_data_t;

typedef struct {
    const sfx_data_t *sfx; /* NULL = idle */
    uint64_t pos;          /* 32.32 fixed point, in source samples */
    int left, right;       /* 0..254, as Doom's (vol, sep) give them */
} channel_t;

static channel_t channels[NUM_CHANNELS];
static pthread_mutex_t lock = PTHREAD_MUTEX_INITIALIZER;
static boolean use_prefix;

static boolean Init(boolean use_sfx_prefix)
{
    use_prefix = use_sfx_prefix;
    pthread_mutex_lock(&lock);
    memset(channels, 0, sizeof(channels));
    pthread_mutex_unlock(&lock);
    return true;
}

static void Shutdown(void)
{
    pthread_mutex_lock(&lock);
    memset(channels, 0, sizeof(channels));
    pthread_mutex_unlock(&lock);
}

static int GetSfxLumpNum(sfxinfo_t *sfx)
{
    char name[9];
    if (sfx->link != NULL) {
        sfx = sfx->link;
    }
    if (use_prefix) {
        M_snprintf(name, sizeof(name), "ds%s", sfx->name);
    } else {
        M_StringCopy(name, sfx->name, sizeof(name));
    }
    return W_CheckNumForName(name);
}

/* Parse (once) the DMX header: u16 format 3, u16 rate, u32 length, then the
 * samples. Like DMX (and Chocolate Doom), skip 16 padding bytes at each end. */
static const sfx_data_t *Load(sfxinfo_t *sfx)
{
    sfx_data_t *d;
    const uint8_t *raw;
    int lump = sfx->lumpnum, lumplen;
    uint32_t rate, len;

    if (sfx->driver_data != NULL) {
        return sfx->driver_data;
    }
    if (lump < 0) {
        return NULL;
    }
    raw = W_CacheLumpNum(lump, PU_STATIC);
    lumplen = W_LumpLength(lump);
    if (lumplen < 8 || raw[0] != 3 || raw[1] != 0) {
        return NULL;
    }
    rate = raw[2] | (raw[3] << 8);
    len = raw[4] | (raw[5] << 8) | (raw[6] << 16) | ((uint32_t)raw[7] << 24);
    if (len > (uint32_t)lumplen - 8 || len <= 48 || rate == 0) {
        return NULL;
    }
    d = Z_Malloc(sizeof(*d), PU_STATIC, NULL);
    d->samples = raw + 8 + 16;
    d->len = len - 32;
    d->rate = rate;
    sfx->driver_data = d;
    return d;
}

static void SetParams(channel_t *ch, int vol, int sep)
{
    if (vol < 0) vol = 0;
    if (vol > 127) vol = 127;
    if (sep < 0) sep = 0;
    if (sep > 254) sep = 254;
    ch->left = ((254 - sep) * vol) / 127;
    ch->right = (sep * vol) / 127;
}

static void UpdateSoundParams(int channel, int vol, int sep)
{
    if (channel < 0 || channel >= NUM_CHANNELS) {
        return;
    }
    pthread_mutex_lock(&lock);
    SetParams(&channels[channel], vol, sep);
    pthread_mutex_unlock(&lock);
}

static int StartSound(sfxinfo_t *sfx, int channel, int vol, int sep)
{
    const sfx_data_t *d;
    if (channel < 0 || channel >= NUM_CHANNELS) {
        return -1;
    }
    d = Load(sfx);
    if (d == NULL) {
        return -1;
    }
    pthread_mutex_lock(&lock);
    channels[channel].sfx = d;
    channels[channel].pos = 0;
    SetParams(&channels[channel], vol, sep);
    pthread_mutex_unlock(&lock);
    return channel;
}

static void StopSound(int channel)
{
    if (channel >= 0 && channel < NUM_CHANNELS) {
        pthread_mutex_lock(&lock);
        channels[channel].sfx = NULL;
        pthread_mutex_unlock(&lock);
    }
}

static boolean SoundIsPlaying(int channel)
{
    boolean playing;
    if (channel < 0 || channel >= NUM_CHANNELS) {
        return false;
    }
    pthread_mutex_lock(&lock);
    playing = channels[channel].sfx != NULL;
    pthread_mutex_unlock(&lock);
    return playing;
}

static void Update(void)
{
}

static void CacheSounds(sfxinfo_t *sounds, int num_sounds)
{
    (void)sounds;
    (void)num_sounds;
}

static snddevice_t sound_devices[] = {
    SNDDEVICE_SB, SNDDEVICE_PAS, SNDDEVICE_GUS, SNDDEVICE_WAVEBLASTER,
    SNDDEVICE_SOUNDCANVAS, SNDDEVICE_AWE32,
};

sound_module_t DG_sound_module = {
    sound_devices, sizeof(sound_devices) / sizeof(*sound_devices),
    Init, Shutdown, GetSfxLumpNum, Update, UpdateSoundParams,
    StartSound, StopSound, SoundIsPlaying, CacheSounds,
};

/* Mix `frames` stereo S16 frames at `rate` Hz into `out` (overwritten).
 * Called from the audio thread. */
void doomtv_sound_mix(int16_t *out, int frames, int rate)
{
    int i, c;
    pthread_mutex_lock(&lock);
    for (i = 0; i < frames; i++) {
        int32_t l = 0, r = 0;
        for (c = 0; c < NUM_CHANNELS; c++) {
            channel_t *ch = &channels[c];
            const sfx_data_t *d = ch->sfx;
            uint32_t at, frac;
            int32_t a, b, s;
            if (d == NULL) {
                continue;
            }
            at = (uint32_t)(ch->pos >> 32);
            if (at >= d->len) {
                ch->sfx = NULL;
                continue;
            }
            frac = (uint32_t)ch->pos >> 16; /* 16-bit fraction */
            a = (int32_t)d->samples[at] - 128;
            b = at + 1 < d->len ? (int32_t)d->samples[at + 1] - 128 : a;
            s = (a << 8) + (((b - a) * (int32_t)frac) >> 8); /* ~16-bit */
            l += (s * ch->left) >> 8;
            r += (s * ch->right) >> 8;
            ch->pos += ((uint64_t)d->rate << 32) / (uint32_t)rate;
        }
        out[2 * i] = l > 32767 ? 32767 : l < -32768 ? -32768 : l;
        out[2 * i + 1] = r > 32767 ? 32767 : r < -32768 ? -32768 : r;
    }
    pthread_mutex_unlock(&lock);
}

/* ---- music: not implemented (silent) ---- */

static boolean MusicInit(void) { return true; }
static void MusicShutdown(void) {}
static void SetMusicVolume(int volume) { (void)volume; }
static void PauseMusic(void) {}
static void ResumeMusic(void) {}
static void *RegisterSong(void *data, int len) { (void)data; (void)len; return NULL; }
static void UnRegisterSong(void *handle) { (void)handle; }
static void PlaySong(void *handle, boolean looping) { (void)handle; (void)looping; }
static void StopSong(void) {}
static boolean MusicIsPlaying(void) { return false; }
static void Poll(void) {}

music_module_t DG_music_module = {
    sound_devices, sizeof(sound_devices) / sizeof(*sound_devices),
    MusicInit, MusicShutdown, SetMusicVolume, PauseMusic, ResumeMusic,
    RegisterSong, UnRegisterSong, PlaySong, StopSong, MusicIsPlaying, Poll,
};
