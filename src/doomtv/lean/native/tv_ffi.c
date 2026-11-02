/* Thin Lean FFI over the TV app ABI (src/platform/include/tv_app_v1.h) plus
 * private-LAN UDP for multiplayer. The engine stays in Lean; this file only
 * moves bytes: one indexed 320x200 frame + palette per present, PCM blocks,
 * key events and datagrams. Each handle has a single owner (this process),
 * and no frame pointer is kept after present. */
#include <lean/lean.h>
#include "tv_app_v1.h"
#include <errno.h>
#include <fcntl.h>
#include <net/if.h>
#include <netinet/in.h>
#include <sys/ioctl.h>
#include <string.h>
#include <sys/socket.h>
#include <unistd.h>

#define SRC_W 320
#define SRC_H 200

static TvDisplay *g_display;
static TvAudio *g_audio;
static TvInput *g_input;
static int g_udp = -1;

static lean_obj_res ok_u32(uint32_t v) { return lean_io_result_mk_ok(lean_box_uint32(v)); }

lean_obj_res dtv_display_open(void) {
    if (g_display) return ok_u32(0);
    return ok_u32((uint32_t)tv_display_open(&g_display));
}

/* Scale the indexed frame to the largest centered 4:3 rectangle (Doom's
 * 320x200 is displayed at 4:3, so pixels are 1.2x taller than wide), with
 * black bars, converting through the palette into the frame's pixel format.
 * The whole frame is written: buffer contents are not preserved. */
lean_obj_res dtv_display_present(b_lean_obj_arg fb, b_lean_obj_arg pal) {
    if (!g_display) return ok_u32((uint32_t)-ENODEV);
    if (lean_sarray_size(fb) < SRC_W * SRC_H || lean_sarray_size(pal) < 768)
        return ok_u32((uint32_t)-EINVAL);
    TvFrame f;
    int32_t r = tv_display_acquire(g_display, &f);
    if (r != 0) return ok_u32((uint32_t)r);
    const uint8_t *src = lean_sarray_cptr(fb), *p = lean_sarray_cptr(pal);
    uint32_t lut[256];
    for (int i = 0; i < 256; i++)
        lut[i] = f.opaque_alpha | ((uint32_t)p[3 * i] << f.red_shift) |
                 ((uint32_t)p[3 * i + 1] << f.green_shift) | ((uint32_t)p[3 * i + 2] << f.blue_shift);
    uint32_t w = f.height * 4 / 3, h = f.height;
    if (w > f.width) { w = f.width; h = w * 3 / 4; }
    uint32_t x0 = (f.width - w) / 2, y0 = (f.height - h) / 2;
    static uint16_t xmap[4096];
    for (uint32_t x = 0; x < w && x < 4096; x++) xmap[x] = (uint16_t)(x * SRC_W / w);
    uint32_t black = f.opaque_alpha;
    uint32_t *prev = NULL;
    uint32_t prevSy = UINT32_MAX;
    for (uint32_t y = 0; y < f.height; y++) {
        uint32_t *row = (uint32_t *)((uint8_t *)f.pixels + (size_t)y * f.stride_bytes);
        if (y < y0 || y >= y0 + h) {
            for (uint32_t x = 0; x < f.width; x++) row[x] = black;
            continue;
        }
        uint32_t sy = (y - y0) * SRC_H / h;
        if (sy == prevSy) {  /* each Doom row covers 3-4 output rows: copy, don't reconvert */
            memcpy(row, prev, (size_t)f.width * 4);
            continue;
        }
        const uint8_t *s = src + (size_t)sy * SRC_W;
        for (uint32_t x = 0; x < x0; x++) row[x] = black;
        for (uint32_t x = 0; x < w; x++) row[x0 + x] = lut[s[xmap[x]]];
        for (uint32_t x = x0 + w; x < f.width; x++) row[x] = black;
        prev = row;
        prevSy = sy;
    }
    return ok_u32((uint32_t)tv_display_present(g_display));
}

/* Benchmark only: the same conversion as dtv_display_present into a private
 * 1280x720 XRGB buffer, so frame cost can be measured without a display. */
lean_obj_res dtv_bench_blit(b_lean_obj_arg fb, b_lean_obj_arg pal) {
    static uint32_t scratch[1280 * 720];
    if (lean_sarray_size(fb) < SRC_W * SRC_H || lean_sarray_size(pal) < 768) return ok_u32((uint32_t)-EINVAL);
    const uint8_t *src = lean_sarray_cptr(fb), *p = lean_sarray_cptr(pal);
    uint32_t lut[256];
    for (int i = 0; i < 256; i++)
        lut[i] = 0xff000000u | ((uint32_t)p[3 * i] << 16) | ((uint32_t)p[3 * i + 1] << 8) | p[3 * i + 2];
    static uint16_t xmap[960];
    for (uint32_t x = 0; x < 960; x++) xmap[x] = (uint16_t)(x * SRC_W / 960);
    uint32_t prevSy = UINT32_MAX;
    for (uint32_t y = 0; y < 720; y++) {
        uint32_t *row = scratch + (size_t)y * 1280;
        uint32_t sy = y * SRC_H / 720;
        if (sy == prevSy) { memcpy(row, row - 1280, 1280 * 4); continue; }
        const uint8_t *s = src + (size_t)sy * SRC_W;
        for (uint32_t x = 0; x < 160; x++) row[x] = 0xff000000u;
        for (uint32_t x = 0; x < 960; x++) row[160 + x] = lut[s[xmap[x]]];
        for (uint32_t x = 1120; x < 1280; x++) row[x] = 0xff000000u;
        prevSy = sy;
    }
    return ok_u32(scratch[(720 / 2) * 1280 + 640]);
}

lean_obj_res dtv_input_open(void) {
    if (g_input) return ok_u32(0);
    return ok_u32((uint32_t)tv_input_open(&g_input));
}

/* 0 = no event; otherwise 0x80000000 | value << 16 | keycode (value 0 release,
 * 1 press, 2 repeat; keycodes above 0xffff are dropped). */
lean_obj_res dtv_input_poll(void) {
    if (!g_input) return ok_u32(0);
    for (;;) {
        uint32_t code; int32_t value;
        int32_t r = tv_input_poll(g_input, &code, &value);
        if (r != 1) return ok_u32(0);
        if (code <= 0xffff && value >= 0 && value <= 2)
            return ok_u32(0x80000000u | ((uint32_t)value << 16) | code);
    }
}

lean_obj_res dtv_audio_open(uint32_t rate, uint32_t channels) {
    if (g_audio) return ok_u32(0);
    return ok_u32((uint32_t)tv_audio_open(rate, channels, &g_audio));
}

/* Interleaved S16LE stereo bytes; returns 0 or negative errno. */
lean_obj_res dtv_audio_write(b_lean_obj_arg pcm) {
    if (!g_audio) return ok_u32((uint32_t)-ENODEV);
    size_t n = lean_sarray_size(pcm) / 4 * 2;
    if (n == 0) return ok_u32(0);
    return ok_u32((uint32_t)tv_audio_write(g_audio, (const int16_t *)lean_sarray_cptr(pcm), n));
}

/* Frames queued for playback, or 0xffffffff when unknown/no audio. */
lean_obj_res dtv_audio_queued(void) {
    int64_t frames;
    if (!g_audio || tv_audio_queued(g_audio, &frames) != 0 || frames < 0) return ok_u32(0xffffffffu);
    return ok_u32(frames > 0x7fffffff ? 0x7fffffffu : (uint32_t)frames);
}

/* UDP on 0.0.0.0:port, nonblocking, close-on-exec. */
lean_obj_res dtv_udp_open(uint16_t port) {
    if (g_udp >= 0) return ok_u32(0);
    int s = socket(AF_INET, SOCK_DGRAM | SOCK_NONBLOCK | SOCK_CLOEXEC, 0);
    if (s < 0) return ok_u32((uint32_t)-errno);
    int one = 1;  /* LAN game discovery broadcasts to 255.255.255.255 */
    setsockopt(s, SOL_SOCKET, SO_BROADCAST, &one, sizeof one);
    struct sockaddr_in a = { .sin_family = AF_INET, .sin_port = htons(port), .sin_addr = { htonl(INADDR_ANY) } };
    if (bind(s, (struct sockaddr *)&a, sizeof a) != 0) { int e = errno; close(s); return ok_u32((uint32_t)-e); }
    g_udp = s;
    return ok_u32(0);
}

lean_obj_res dtv_udp_send(uint32_t ip, uint16_t port, b_lean_obj_arg bytes) {
    if (g_udp < 0) return ok_u32((uint32_t)-EBADF);
    struct sockaddr_in a = { .sin_family = AF_INET, .sin_port = htons(port), .sin_addr = { htonl(ip) } };
    ssize_t n = sendto(g_udp, lean_sarray_cptr(bytes), lean_sarray_size(bytes), 0, (struct sockaddr *)&a, sizeof a);
    return ok_u32(n < 0 ? (uint32_t)-errno : 0);
}

/* LAN discovery: send to every IPv4 interface's broadcast address (a limited
 * broadcast to 255.255.255.255 needs a route the TV may not have), then to
 * 255.255.255.255. Returns how many sends succeeded, or negative errno. */
lean_obj_res dtv_udp_broadcast(uint16_t port, b_lean_obj_arg bytes) {
    if (g_udp < 0) return ok_u32((uint32_t)-EBADF);
    struct ifreq ifs[16];
    struct ifconf ifc = { .ifc_len = sizeof ifs, .ifc_req = ifs };
    uint32_t sent = 0;
    int err = 0;
    if (ioctl(g_udp, SIOCGIFCONF, &ifc) == 0) {
        for (size_t i = 0; i < ifc.ifc_len / sizeof(struct ifreq); i++) {
            struct ifreq r = ifs[i];
            if (ioctl(g_udp, SIOCGIFFLAGS, &r) != 0 || !(r.ifr_flags & IFF_BROADCAST) || (r.ifr_flags & IFF_LOOPBACK)) continue;
            r = ifs[i];
            if (ioctl(g_udp, SIOCGIFBRDADDR, &r) != 0) continue;
            struct sockaddr_in a = *(struct sockaddr_in *)&r.ifr_broadaddr;
            a.sin_port = htons(port);
            if (sendto(g_udp, lean_sarray_cptr(bytes), lean_sarray_size(bytes), 0, (struct sockaddr *)&a, sizeof a) >= 0) sent++;
            else err = errno;
        }
    } else err = errno;
    struct sockaddr_in all = { .sin_family = AF_INET, .sin_port = htons(port), .sin_addr = { htonl(INADDR_BROADCAST) } };
    if (sendto(g_udp, lean_sarray_cptr(bytes), lean_sarray_size(bytes), 0, (struct sockaddr *)&all, sizeof all) >= 0) sent++;
    else if (!err) err = errno;
    return ok_u32(sent ? sent : (uint32_t)-err);
}

/* One datagram as 6 bytes of sender (IPv4 big-endian, port big-endian)
 * followed by the payload, or an empty array when none is waiting. */
lean_obj_res dtv_udp_recv(void) {
    uint8_t buf[6 + 1500];
    struct sockaddr_in a;
    socklen_t al = sizeof a;
    ssize_t n = g_udp < 0 ? -1 : recvfrom(g_udp, buf + 6, sizeof buf - 6, 0, (struct sockaddr *)&a, &al);
    if (n < 0 || al < sizeof a || a.sin_family != AF_INET) n = -6;
    else { memcpy(buf, &a.sin_addr.s_addr, 4); memcpy(buf + 4, &a.sin_port, 2); }
    size_t len = (size_t)(n + 6);
    lean_object *r = lean_alloc_sarray(1, len, len);
    memcpy(lean_sarray_cptr(r), buf, len);
    return lean_io_result_mk_ok(r);
}
