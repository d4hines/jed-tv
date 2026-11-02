/*
 * Host reference harness for Doom.Audio.Music / OplMusic.
 *
 * Builds Chocolate Doom's own i_oplmusic.c, midifile.c, mus2mid.c, memio.c,
 * opl_queue.c and opl3.c (Nuked-OPL3-fast) with a single-threaded stand-in
 * for the parts of opl.c / opl_sdl.c that need SDL. The driver functions
 * below are line-for-line copies of opl_sdl.c's timing logic
 * (OPL_Mix_Callback / AdvanceTime / WriteRegister) and opl.c's
 * OPL_InitRegisters, minus the mutexes.
 *
 * usage: music_ref GENMIDI SONG RATE FRAMES BUFFER DRIVER OPL3 VOLUME LOOP OUT.raw [WRITELOG]
 *   DRIVER: 0 = Doom 1 v1.666, 1 = Doom 2 v1.666, 2 = Doom v1.9
 *   output: raw s16le stereo (the OPL postmix added to silence)
 *   WRITELOG: "frame reg value" per register write reaching the chip
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdarg.h>

#include "doomtype.h"
#include "i_sound.h"
#include "m_misc.h"
#include "w_wad.h"
#include "opl.h"
#include "opl_queue.h"
#include "opl3.h"

/* ---- globals normally provided by other Chocolate Doom modules ---- */
int snd_samplerate = 44100;
static unsigned char *genmidi_data;
static char tmpfile_path[1024];

void *W_CacheLumpName(const char *name, int tag) { (void) name; (void) tag; return genmidi_data; }
void W_ReleaseLumpName(const char *name) { (void) name; }
const char *DEH_String(const char *s) { return s; }
void *Z_Malloc(int size, int tag, void *user) { (void) tag; (void) user; void *p = malloc(size); if (!p) abort(); return p; }
void Z_Free(void *ptr) { free(ptr); }
void *I_Realloc(void *ptr, size_t size) { void *p = realloc(ptr, size); if (!p && size) abort(); return p; }
FILE *M_fopen(const char *filename, const char *mode) { return fopen(filename, mode); }
int M_remove(const char *path) { return remove(path); }
char *M_TempFile(const char *s) { (void) s; return strdup(tmpfile_path); }
boolean M_WriteFile(const char *name, const void *source, int length)
{
    FILE *f = fopen(name, "wb");
    if (!f) return false;
    int ok = fwrite(source, 1, length, f) == (size_t) length;
    fclose(f);
    return ok;
}
int M_snprintf(char *buf, size_t buf_len, const char *s, ...)
{
    va_list args; va_start(args, s);
    int r = vsnprintf(buf, buf_len, s, args);
    va_end(args); return r;
}
boolean M_StringConcat(char *dest, const char *src, size_t dest_size)
{
    size_t l = strlen(dest);
    if (l >= dest_size) return false;
    snprintf(dest + l, dest_size - l, "%s", src);
    return true;
}

/* ---- opl.c / opl_sdl.c stand-in ---- */
static opl3_chip opl_chip;
static opl_callback_queue_t *callback_queue;
static uint64_t current_time;
static int opl_sdl_paused;
static uint64_t pause_offset;
static int mixing_freq;
static unsigned int opl_sample_rate = 22050;
static long frames_out;
static FILE *write_log;

void OPL_SetSampleRate(unsigned int rate) { opl_sample_rate = rate; }

opl_init_result_t OPL_Init(unsigned int port_base)
{
    (void) port_base;
    mixing_freq = (int) opl_sample_rate;
    callback_queue = OPL_Queue_Create();
    current_time = 0;
    opl_sdl_paused = 0;
    pause_offset = 0;
    OPL3_Reset(&opl_chip, mixing_freq);
    return OPL_INIT_OPL3;
}

void OPL_Shutdown(void) {}

/* opl_sdl.c WriteRegister (timers are emulated by the driver, not the chip) */
void OPL_WriteRegister(int reg_num, int value)
{
    switch (reg_num)
    {
        case OPL_REG_TIMER1:
        case OPL_REG_TIMER2:
        case OPL_REG_TIMER_CTRL:
            break;
        default:
            if (write_log) fprintf(write_log, "%ld %d %d\n", frames_out, reg_num, value);
            OPL3_WriteRegBuffered(&opl_chip, reg_num, value);
            break;
    }
}

/* opl.c OPL_InitRegisters, verbatim apart from the function name prefix */
void OPL_InitRegisters(int opl3)
{
    int r;
    for (r=OPL_REGS_LEVEL; r <= OPL_REGS_LEVEL + OPL_NUM_OPERATORS; ++r)
        OPL_WriteRegister(r, 0x3f);
    for (r=OPL_REGS_ATTACK; r <= OPL_REGS_WAVEFORM + OPL_NUM_OPERATORS; ++r)
        OPL_WriteRegister(r, 0x00);
    for (r=1; r < OPL_REGS_LEVEL; ++r)
        OPL_WriteRegister(r, 0x00);
    OPL_WriteRegister(OPL_REG_TIMER_CTRL,      0x60);
    OPL_WriteRegister(OPL_REG_TIMER_CTRL,      0x80);
    OPL_WriteRegister(OPL_REG_WAVEFORM_ENABLE, 0x20);
    if (opl3)
    {
        OPL_WriteRegister(OPL_REG_NEW, 0x01);
        for (r=OPL_REGS_LEVEL; r <= OPL_REGS_LEVEL + OPL_NUM_OPERATORS; ++r)
            OPL_WriteRegister(r | 0x100, 0x3f);
        for (r=OPL_REGS_ATTACK; r <= OPL_REGS_WAVEFORM + OPL_NUM_OPERATORS; ++r)
            OPL_WriteRegister(r | 0x100, 0x00);
        for (r=1; r < OPL_REGS_LEVEL; ++r)
            OPL_WriteRegister(r | 0x100, 0x00);
    }
    OPL_WriteRegister(OPL_REG_FM_MODE,         0x40);
    if (opl3)
        OPL_WriteRegister(OPL_REG_NEW, 0x01);
}

void OPL_SetCallback(uint64_t us, opl_callback_t callback, void *data)
{
    OPL_Queue_Push(callback_queue, callback, data, current_time - pause_offset + us);
}
void OPL_ClearCallbacks(void) { OPL_Queue_Clear(callback_queue); }
void OPL_Lock(void) {}
void OPL_Unlock(void) {}
void OPL_SetPaused(int paused) { opl_sdl_paused = paused; }
void OPL_AdjustCallbacks(float factor) { OPL_Queue_AdjustCallbacks(callback_queue, current_time, factor); }

static void AdvanceTime(unsigned int nsamples)
{
    opl_callback_t callback;
    void *callback_data;
    uint64_t us;
    us = ((uint64_t) nsamples * OPL_SECOND) / mixing_freq;
    current_time += us;
    if (opl_sdl_paused)
        pause_offset += us;
    while (!OPL_Queue_IsEmpty(callback_queue)
        && current_time >= OPL_Queue_Peek(callback_queue) + pause_offset)
    {
        if (!OPL_Queue_Pop(callback_queue, &callback, &callback_data))
            break;
        callback(callback_data);
    }
}

static void FillBuffer(int16_t *buffer, unsigned int nsamples)
{
    OPL3_GenerateStream(&opl_chip, buffer, nsamples);
    frames_out += nsamples;
}

static void OPL_Mix_Callback(int16_t *buffer, unsigned int buffer_samples)
{
    unsigned int filled = 0;
    while (filled < buffer_samples)
    {
        uint64_t next_callback_time;
        uint64_t nsamples;
        if (opl_sdl_paused || OPL_Queue_IsEmpty(callback_queue))
        {
            nsamples = buffer_samples - filled;
        }
        else
        {
            next_callback_time = OPL_Queue_Peek(callback_queue) + pause_offset;
            nsamples = (next_callback_time - current_time) * mixing_freq;
            nsamples = (nsamples + OPL_SECOND - 1) / OPL_SECOND;
            if (nsamples > buffer_samples - filled)
                nsamples = buffer_samples - filled;
        }
        FillBuffer(buffer + filled * 2, nsamples);
        filled += nsamples;
        AdvanceTime(nsamples);
    }
}

static unsigned char *read_all(const char *path, int *len)
{
    FILE *f = fopen(path, "rb");
    if (!f) { perror(path); exit(1); }
    fseek(f, 0, SEEK_END); long n = ftell(f); fseek(f, 0, SEEK_SET);
    unsigned char *b = malloc(n + 1);
    if (fread(b, 1, n, f) != (size_t) n) { perror("read"); exit(1); }
    fclose(f); *len = (int) n; return b;
}

extern char *snd_dmxoption;

int main(int argc, char **argv)
{
    if (argc != 11 && argc != 12)
    {
        fprintf(stderr, "usage: %s GENMIDI SONG RATE FRAMES BUFFER DRIVER OPL3 VOLUME LOOP OUT [WRITELOG]\n", argv[0]);
        return 2;
    }
    int glen, slen;
    genmidi_data = read_all(argv[1], &glen);
    unsigned char *song = read_all(argv[2], &slen);
    snd_samplerate = atoi(argv[3]);
    long frames = atol(argv[4]);
    unsigned int buffer = (unsigned int) atoi(argv[5]);
    int driver = atoi(argv[6]);
    int opl3 = atoi(argv[7]);
    int volume = atoi(argv[8]);
    int looping = atoi(argv[9]);
    snprintf(tmpfile_path, sizeof(tmpfile_path), "%s.tmp.mid", argv[10]);
    if (argc == 12) write_log = fopen(argv[11], "w");
    unsetenv("DMXOPTION");
    snd_dmxoption = opl3 ? "-opl3" : "";

    I_SetOPLDriverVer(driver == 0 ? opl_doom1_1_666 : driver == 1 ? opl_doom2_1_666 : opl_doom_1_9);
    if (!music_opl_module.Init()) { fprintf(stderr, "init failed\n"); return 1; }
    music_opl_module.SetMusicVolume(volume);
    void *handle = music_opl_module.RegisterSong(song, slen);
    if (handle == NULL) fprintf(stderr, "music_ref: song failed to load\n");
    music_opl_module.PlaySong(handle, looping);

    FILE *out = fopen(argv[10], "wb");
    int16_t *buf = malloc(sizeof(int16_t) * 2 * buffer);
    long done = 0;
    while (done < frames)
    {
        unsigned int n = buffer;
        if ((long) n > frames - done) n = (unsigned int) (frames - done);
        OPL_Mix_Callback(buf, n);
        fwrite(buf, sizeof(int16_t), 2 * n, out);
        done += n;
    }
    fclose(out);
    if (write_log) fclose(write_log);
    return 0;
}
