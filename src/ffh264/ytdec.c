/* Thin C shim over FFmpeg's H.264 decoder, so the Rust side needs no FFmpeg
 * struct layouts: open with the avcC record, send MP4 (length-prefixed)
 * samples, receive YUV 4:2:0 frames. */
#include <libavcodec/avcodec.h>
#include <libavutil/frame.h>
#include <stdint.h>

typedef struct {
    const uint8_t *y, *u, *v;
    int y_stride, u_stride, v_stride;
    int width, height;
    int64_t pts;
    int keyframe;
    int pix_fmt; /* 0 = YUV420P */
} ytdec_frame;

typedef struct {
    AVCodecContext *ctx;
    AVPacket *pkt;
    AVFrame *frame;
} ytdec;

ytdec *ytdec_open(const uint8_t *avcc, int avcc_len, int threads) {
    const AVCodec *codec = avcodec_find_decoder(AV_CODEC_ID_H264);
    if (!codec) return NULL;
    ytdec *d = av_mallocz(sizeof *d);
    if (!d) return NULL;
    d->ctx = avcodec_alloc_context3(codec);
    d->pkt = av_packet_alloc();
    d->frame = av_frame_alloc();
    if (!d->ctx || !d->pkt || !d->frame) goto fail;
    d->ctx->extradata = av_mallocz(avcc_len + AV_INPUT_BUFFER_PADDING_SIZE);
    if (!d->ctx->extradata) goto fail;
    memcpy(d->ctx->extradata, avcc, avcc_len);
    d->ctx->extradata_size = avcc_len;
    d->ctx->thread_count = threads; /* 0 = auto */
    d->ctx->thread_type = FF_THREAD_FRAME | FF_THREAD_SLICE;
    if (avcodec_open2(d->ctx, codec, NULL) < 0) goto fail;
    return d;
fail:
    avcodec_free_context(&d->ctx);
    av_packet_free(&d->pkt);
    av_frame_free(&d->frame);
    av_free(d);
    return NULL;
}

/* data == NULL flushes. Returns 0, or a negative AVERROR (AVERROR(EAGAIN) = must receive first). */
int ytdec_send(ytdec *d, const uint8_t *data, int len, int64_t pts, int64_t dts) {
    if (!data) return avcodec_send_packet(d->ctx, NULL);
    int r = av_new_packet(d->pkt, len);
    if (r < 0) return r;
    memcpy(d->pkt->data, data, len);
    d->pkt->pts = pts;
    d->pkt->dts = dts;
    r = avcodec_send_packet(d->ctx, d->pkt);
    av_packet_unref(d->pkt);
    return r;
}

/* 0 = frame filled (valid until the next call), 1 = needs input, 2 = end of stream, <0 = error. */
int ytdec_receive(ytdec *d, ytdec_frame *f) {
    av_frame_unref(d->frame);
    int r = avcodec_receive_frame(d->ctx, d->frame);
    if (r == AVERROR(EAGAIN)) return 1;
    if (r == AVERROR_EOF) return 2;
    if (r < 0) return r;
    AVFrame *fr = d->frame;
    f->y = fr->data[0]; f->u = fr->data[1]; f->v = fr->data[2];
    f->y_stride = fr->linesize[0]; f->u_stride = fr->linesize[1]; f->v_stride = fr->linesize[2];
    f->width = fr->width; f->height = fr->height;
    f->pts = fr->best_effort_timestamp;
    f->keyframe = (fr->flags & AV_FRAME_FLAG_KEY) != 0;
    f->pix_fmt = fr->format;
    return 0;
}

void ytdec_close(ytdec *d) {
    if (!d) return;
    avcodec_free_context(&d->ctx);
    av_packet_free(&d->pkt);
    av_frame_free(&d->frame);
    av_free(d);
}
