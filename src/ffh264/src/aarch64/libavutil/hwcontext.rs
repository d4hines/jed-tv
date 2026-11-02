// Translated from FFmpeg 9.0.1 libavutil/hwcontext.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// This file is part of FFmpeg.
//
// FFmpeg is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// FFmpeg is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with FFmpeg; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA
//

extern "C" {
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn abort() -> !;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_buffer_create(
        data: *mut uint8_t,
        size: size_t,
        free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>,
        opaque: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> *mut AVBufferRef;
    fn av_buffer_ref(buf: *const AVBufferRef) -> *mut AVBufferRef;
    fn av_buffer_unref(buf: *mut *mut AVBufferRef);
    fn av_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_image_check_size(
        w: ::core::ffi::c_uint,
        h: ::core::ffi::c_uint,
        log_offset: ::core::ffi::c_int,
        log_ctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn av_buffer_pool_uninit(pool: *mut *mut AVBufferPool);
    fn av_get_pix_fmt_name(pix_fmt: AVPixelFormat) -> *const ::core::ffi::c_char;
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_default_item_name(ctx: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_frame_alloc() -> *mut AVFrame;
    fn av_frame_free(frame: *mut *mut AVFrame);
    fn av_frame_ref(dst: *mut AVFrame, src: *const AVFrame) -> ::core::ffi::c_int;
    fn av_frame_replace(dst: *mut AVFrame, src: *const AVFrame) -> ::core::ffi::c_int;
    fn av_frame_unref(frame: *mut AVFrame);
    fn av_frame_move_ref(dst: *mut AVFrame, src: *mut AVFrame);
    fn av_frame_get_buffer(
        frame: *mut AVFrame,
        align: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type size_t = ::core::ffi::c_ulong;
pub type AVClassCategory = ::core::ffi::c_uint;
pub const AV_CLASS_CATEGORY_NB: AVClassCategory = 46;
pub const AV_CLASS_CATEGORY_DEVICE_INPUT: AVClassCategory = 45;
pub const AV_CLASS_CATEGORY_DEVICE_OUTPUT: AVClassCategory = 44;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_INPUT: AVClassCategory = 43;
pub const AV_CLASS_CATEGORY_DEVICE_AUDIO_OUTPUT: AVClassCategory = 42;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_INPUT: AVClassCategory = 41;
pub const AV_CLASS_CATEGORY_DEVICE_VIDEO_OUTPUT: AVClassCategory = 40;
pub const AV_CLASS_CATEGORY_HWDEVICE: AVClassCategory = 11;
pub const AV_CLASS_CATEGORY_SWRESAMPLER: AVClassCategory = 10;
pub const AV_CLASS_CATEGORY_SWSCALER: AVClassCategory = 9;
pub const AV_CLASS_CATEGORY_BITSTREAM_FILTER: AVClassCategory = 8;
pub const AV_CLASS_CATEGORY_FILTER: AVClassCategory = 7;
pub const AV_CLASS_CATEGORY_DECODER: AVClassCategory = 6;
pub const AV_CLASS_CATEGORY_ENCODER: AVClassCategory = 5;
pub const AV_CLASS_CATEGORY_DEMUXER: AVClassCategory = 4;
pub const AV_CLASS_CATEGORY_MUXER: AVClassCategory = 3;
pub const AV_CLASS_CATEGORY_OUTPUT: AVClassCategory = 2;
pub const AV_CLASS_CATEGORY_INPUT: AVClassCategory = 1;
pub const AV_CLASS_CATEGORY_NA: AVClassCategory = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVClass {
    pub class_name: *const ::core::ffi::c_char,
    pub item_name: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *const ::core::ffi::c_char,
    >,
    pub option: *const AVOption,
    pub version: ::core::ffi::c_int,
    pub log_level_offset_offset: ::core::ffi::c_int,
    pub parent_log_context_offset: ::core::ffi::c_int,
    pub category: AVClassCategory,
    pub get_category: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void) -> AVClassCategory,
    >,
    pub query_ranges: Option<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub child_next: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_void,
    >,
    pub child_class_iterate: Option<
        unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> *const AVClass,
    >,
    pub state_flags_offset: ::core::ffi::c_int,
}
pub type int64_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint64_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
pub type AVPictureType = ::core::ffi::c_uint;
pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;
pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
pub type AVPixelFormat = ::core::ffi::c_int;
pub const AV_PIX_FMT_NB: AVPixelFormat = 267;
pub const AV_PIX_FMT_OHCODEC: AVPixelFormat = 266;
pub const AV_PIX_FMT_GBRP12MSBLE: AVPixelFormat = 265;
pub const AV_PIX_FMT_GBRP12MSBBE: AVPixelFormat = 264;
pub const AV_PIX_FMT_GBRP10MSBLE: AVPixelFormat = 263;
pub const AV_PIX_FMT_GBRP10MSBBE: AVPixelFormat = 262;
pub const AV_PIX_FMT_YUV444P12MSBLE: AVPixelFormat = 261;
pub const AV_PIX_FMT_YUV444P12MSBBE: AVPixelFormat = 260;
pub const AV_PIX_FMT_YUV444P10MSBLE: AVPixelFormat = 259;
pub const AV_PIX_FMT_YUV444P10MSBBE: AVPixelFormat = 258;
pub const AV_PIX_FMT_GBRAP32LE: AVPixelFormat = 257;
pub const AV_PIX_FMT_GBRAP32BE: AVPixelFormat = 256;
pub const AV_PIX_FMT_YAF16LE: AVPixelFormat = 255;
pub const AV_PIX_FMT_YAF16BE: AVPixelFormat = 254;
pub const AV_PIX_FMT_YAF32LE: AVPixelFormat = 253;
pub const AV_PIX_FMT_YAF32BE: AVPixelFormat = 252;
pub const AV_PIX_FMT_GRAY32LE: AVPixelFormat = 251;
pub const AV_PIX_FMT_GRAY32BE: AVPixelFormat = 250;
pub const AV_PIX_FMT_AMF_SURFACE: AVPixelFormat = 249;
pub const AV_PIX_FMT_GRAYF16LE: AVPixelFormat = 248;
pub const AV_PIX_FMT_GRAYF16BE: AVPixelFormat = 247;
pub const AV_PIX_FMT_GBRAPF16LE: AVPixelFormat = 246;
pub const AV_PIX_FMT_GBRAPF16BE: AVPixelFormat = 245;
pub const AV_PIX_FMT_GBRPF16LE: AVPixelFormat = 244;
pub const AV_PIX_FMT_GBRPF16BE: AVPixelFormat = 243;
pub const AV_PIX_FMT_XV48LE: AVPixelFormat = 242;
pub const AV_PIX_FMT_XV48BE: AVPixelFormat = 241;
pub const AV_PIX_FMT_Y216LE: AVPixelFormat = 240;
pub const AV_PIX_FMT_Y216BE: AVPixelFormat = 239;
pub const AV_PIX_FMT_RGB96LE: AVPixelFormat = 238;
pub const AV_PIX_FMT_RGB96BE: AVPixelFormat = 237;
pub const AV_PIX_FMT_RGBA128LE: AVPixelFormat = 236;
pub const AV_PIX_FMT_RGBA128BE: AVPixelFormat = 235;
pub const AV_PIX_FMT_RGBF16LE: AVPixelFormat = 234;
pub const AV_PIX_FMT_RGBF16BE: AVPixelFormat = 233;
pub const AV_PIX_FMT_V30XLE: AVPixelFormat = 232;
pub const AV_PIX_FMT_V30XBE: AVPixelFormat = 231;
pub const AV_PIX_FMT_VYU444: AVPixelFormat = 230;
pub const AV_PIX_FMT_UYVA: AVPixelFormat = 229;
pub const AV_PIX_FMT_AYUV: AVPixelFormat = 228;
pub const AV_PIX_FMT_D3D12: AVPixelFormat = 227;
pub const AV_PIX_FMT_GBRAP14LE: AVPixelFormat = 226;
pub const AV_PIX_FMT_GBRAP14BE: AVPixelFormat = 225;
pub const AV_PIX_FMT_P412LE: AVPixelFormat = 224;
pub const AV_PIX_FMT_P412BE: AVPixelFormat = 223;
pub const AV_PIX_FMT_P212LE: AVPixelFormat = 222;
pub const AV_PIX_FMT_P212BE: AVPixelFormat = 221;
pub const AV_PIX_FMT_RGBAF32LE: AVPixelFormat = 220;
pub const AV_PIX_FMT_RGBAF32BE: AVPixelFormat = 219;
pub const AV_PIX_FMT_RGBF32LE: AVPixelFormat = 218;
pub const AV_PIX_FMT_RGBF32BE: AVPixelFormat = 217;
pub const AV_PIX_FMT_XV36LE: AVPixelFormat = 216;
pub const AV_PIX_FMT_XV36BE: AVPixelFormat = 215;
pub const AV_PIX_FMT_XV30LE: AVPixelFormat = 214;
pub const AV_PIX_FMT_XV30BE: AVPixelFormat = 213;
pub const AV_PIX_FMT_Y212LE: AVPixelFormat = 212;
pub const AV_PIX_FMT_Y212BE: AVPixelFormat = 211;
pub const AV_PIX_FMT_P012BE: AVPixelFormat = 210;
pub const AV_PIX_FMT_P012LE: AVPixelFormat = 209;
pub const AV_PIX_FMT_VUYX: AVPixelFormat = 208;
pub const AV_PIX_FMT_RGBAF16LE: AVPixelFormat = 207;
pub const AV_PIX_FMT_RGBAF16BE: AVPixelFormat = 206;
pub const AV_PIX_FMT_VUYA: AVPixelFormat = 205;
pub const AV_PIX_FMT_P416LE: AVPixelFormat = 204;
pub const AV_PIX_FMT_P416BE: AVPixelFormat = 203;
pub const AV_PIX_FMT_P216LE: AVPixelFormat = 202;
pub const AV_PIX_FMT_P216BE: AVPixelFormat = 201;
pub const AV_PIX_FMT_P410LE: AVPixelFormat = 200;
pub const AV_PIX_FMT_P410BE: AVPixelFormat = 199;
pub const AV_PIX_FMT_P210LE: AVPixelFormat = 198;
pub const AV_PIX_FMT_P210BE: AVPixelFormat = 197;
pub const AV_PIX_FMT_X2BGR10BE: AVPixelFormat = 196;
pub const AV_PIX_FMT_X2BGR10LE: AVPixelFormat = 195;
pub const AV_PIX_FMT_X2RGB10BE: AVPixelFormat = 194;
pub const AV_PIX_FMT_X2RGB10LE: AVPixelFormat = 193;
pub const AV_PIX_FMT_Y210LE: AVPixelFormat = 192;
pub const AV_PIX_FMT_Y210BE: AVPixelFormat = 191;
pub const AV_PIX_FMT_VULKAN: AVPixelFormat = 190;
pub const AV_PIX_FMT_NV42: AVPixelFormat = 189;
pub const AV_PIX_FMT_NV24: AVPixelFormat = 188;
pub const AV_PIX_FMT_YUVA444P12LE: AVPixelFormat = 187;
pub const AV_PIX_FMT_YUVA444P12BE: AVPixelFormat = 186;
pub const AV_PIX_FMT_YUVA422P12LE: AVPixelFormat = 185;
pub const AV_PIX_FMT_YUVA422P12BE: AVPixelFormat = 184;
pub const AV_PIX_FMT_GRAYF32LE: AVPixelFormat = 183;
pub const AV_PIX_FMT_GRAYF32BE: AVPixelFormat = 182;
pub const AV_PIX_FMT_GRAY14LE: AVPixelFormat = 181;
pub const AV_PIX_FMT_GRAY14BE: AVPixelFormat = 180;
pub const AV_PIX_FMT_OPENCL: AVPixelFormat = 179;
pub const AV_PIX_FMT_DRM_PRIME: AVPixelFormat = 178;
pub const AV_PIX_FMT_GBRAPF32LE: AVPixelFormat = 177;
pub const AV_PIX_FMT_GBRAPF32BE: AVPixelFormat = 176;
pub const AV_PIX_FMT_GBRPF32LE: AVPixelFormat = 175;
pub const AV_PIX_FMT_GBRPF32BE: AVPixelFormat = 174;
pub const AV_PIX_FMT_GRAY9LE: AVPixelFormat = 173;
pub const AV_PIX_FMT_GRAY9BE: AVPixelFormat = 172;
pub const AV_PIX_FMT_D3D11: AVPixelFormat = 171;
pub const AV_PIX_FMT_P016BE: AVPixelFormat = 170;
pub const AV_PIX_FMT_P016LE: AVPixelFormat = 169;
pub const AV_PIX_FMT_GRAY10LE: AVPixelFormat = 168;
pub const AV_PIX_FMT_GRAY10BE: AVPixelFormat = 167;
pub const AV_PIX_FMT_GRAY12LE: AVPixelFormat = 166;
pub const AV_PIX_FMT_GRAY12BE: AVPixelFormat = 165;
pub const AV_PIX_FMT_MEDIACODEC: AVPixelFormat = 164;
pub const AV_PIX_FMT_GBRAP10LE: AVPixelFormat = 163;
pub const AV_PIX_FMT_GBRAP10BE: AVPixelFormat = 162;
pub const AV_PIX_FMT_GBRAP12LE: AVPixelFormat = 161;
pub const AV_PIX_FMT_GBRAP12BE: AVPixelFormat = 160;
pub const AV_PIX_FMT_P010BE: AVPixelFormat = 159;
pub const AV_PIX_FMT_P010LE: AVPixelFormat = 158;
pub const AV_PIX_FMT_VIDEOTOOLBOX: AVPixelFormat = 157;
pub const AV_PIX_FMT_AYUV64BE: AVPixelFormat = 156;
pub const AV_PIX_FMT_AYUV64LE: AVPixelFormat = 155;
pub const AV_PIX_FMT_YUV440P12BE: AVPixelFormat = 154;
pub const AV_PIX_FMT_YUV440P12LE: AVPixelFormat = 153;
pub const AV_PIX_FMT_YUV440P10BE: AVPixelFormat = 152;
pub const AV_PIX_FMT_YUV440P10LE: AVPixelFormat = 151;
pub const AV_PIX_FMT_BAYER_GRBG16BE: AVPixelFormat = 150;
pub const AV_PIX_FMT_BAYER_GRBG16LE: AVPixelFormat = 149;
pub const AV_PIX_FMT_BAYER_GBRG16BE: AVPixelFormat = 148;
pub const AV_PIX_FMT_BAYER_GBRG16LE: AVPixelFormat = 147;
pub const AV_PIX_FMT_BAYER_RGGB16BE: AVPixelFormat = 146;
pub const AV_PIX_FMT_BAYER_RGGB16LE: AVPixelFormat = 145;
pub const AV_PIX_FMT_BAYER_BGGR16BE: AVPixelFormat = 144;
pub const AV_PIX_FMT_BAYER_BGGR16LE: AVPixelFormat = 143;
pub const AV_PIX_FMT_BAYER_GRBG8: AVPixelFormat = 142;
pub const AV_PIX_FMT_BAYER_GBRG8: AVPixelFormat = 141;
pub const AV_PIX_FMT_BAYER_RGGB8: AVPixelFormat = 140;
pub const AV_PIX_FMT_BAYER_BGGR8: AVPixelFormat = 139;
pub const AV_PIX_FMT_YUVJ411P: AVPixelFormat = 138;
pub const AV_PIX_FMT_GBRP14LE: AVPixelFormat = 137;
pub const AV_PIX_FMT_GBRP14BE: AVPixelFormat = 136;
pub const AV_PIX_FMT_GBRP12LE: AVPixelFormat = 135;
pub const AV_PIX_FMT_GBRP12BE: AVPixelFormat = 134;
pub const AV_PIX_FMT_YUV444P14LE: AVPixelFormat = 133;
pub const AV_PIX_FMT_YUV444P14BE: AVPixelFormat = 132;
pub const AV_PIX_FMT_YUV444P12LE: AVPixelFormat = 131;
pub const AV_PIX_FMT_YUV444P12BE: AVPixelFormat = 130;
pub const AV_PIX_FMT_YUV422P14LE: AVPixelFormat = 129;
pub const AV_PIX_FMT_YUV422P14BE: AVPixelFormat = 128;
pub const AV_PIX_FMT_YUV422P12LE: AVPixelFormat = 127;
pub const AV_PIX_FMT_YUV422P12BE: AVPixelFormat = 126;
pub const AV_PIX_FMT_YUV420P14LE: AVPixelFormat = 125;
pub const AV_PIX_FMT_YUV420P14BE: AVPixelFormat = 124;
pub const AV_PIX_FMT_YUV420P12LE: AVPixelFormat = 123;
pub const AV_PIX_FMT_YUV420P12BE: AVPixelFormat = 122;
pub const AV_PIX_FMT_BGR0: AVPixelFormat = 121;
pub const AV_PIX_FMT_0BGR: AVPixelFormat = 120;
pub const AV_PIX_FMT_RGB0: AVPixelFormat = 119;
pub const AV_PIX_FMT_0RGB: AVPixelFormat = 118;
pub const AV_PIX_FMT_CUDA: AVPixelFormat = 117;
pub const AV_PIX_FMT_D3D11VA_VLD: AVPixelFormat = 116;
pub const AV_PIX_FMT_MMAL: AVPixelFormat = 115;
pub const AV_PIX_FMT_QSV: AVPixelFormat = 114;
pub const AV_PIX_FMT_GBRAP16LE: AVPixelFormat = 113;
pub const AV_PIX_FMT_GBRAP16BE: AVPixelFormat = 112;
pub const AV_PIX_FMT_GBRAP: AVPixelFormat = 111;
pub const AV_PIX_FMT_YA16LE: AVPixelFormat = 110;
pub const AV_PIX_FMT_YA16BE: AVPixelFormat = 109;
pub const AV_PIX_FMT_YVYU422: AVPixelFormat = 108;
pub const AV_PIX_FMT_BGRA64LE: AVPixelFormat = 107;
pub const AV_PIX_FMT_BGRA64BE: AVPixelFormat = 106;
pub const AV_PIX_FMT_RGBA64LE: AVPixelFormat = 105;
pub const AV_PIX_FMT_RGBA64BE: AVPixelFormat = 104;
pub const AV_PIX_FMT_NV20BE: AVPixelFormat = 103;
pub const AV_PIX_FMT_NV20LE: AVPixelFormat = 102;
pub const AV_PIX_FMT_NV16: AVPixelFormat = 101;
pub const AV_PIX_FMT_XYZ12BE: AVPixelFormat = 100;
pub const AV_PIX_FMT_XYZ12LE: AVPixelFormat = 99;
pub const AV_PIX_FMT_VDPAU: AVPixelFormat = 98;
pub const AV_PIX_FMT_YUVA444P16LE: AVPixelFormat = 97;
pub const AV_PIX_FMT_YUVA444P16BE: AVPixelFormat = 96;
pub const AV_PIX_FMT_YUVA422P16LE: AVPixelFormat = 95;
pub const AV_PIX_FMT_YUVA422P16BE: AVPixelFormat = 94;
pub const AV_PIX_FMT_YUVA420P16LE: AVPixelFormat = 93;
pub const AV_PIX_FMT_YUVA420P16BE: AVPixelFormat = 92;
pub const AV_PIX_FMT_YUVA444P10LE: AVPixelFormat = 91;
pub const AV_PIX_FMT_YUVA444P10BE: AVPixelFormat = 90;
pub const AV_PIX_FMT_YUVA422P10LE: AVPixelFormat = 89;
pub const AV_PIX_FMT_YUVA422P10BE: AVPixelFormat = 88;
pub const AV_PIX_FMT_YUVA420P10LE: AVPixelFormat = 87;
pub const AV_PIX_FMT_YUVA420P10BE: AVPixelFormat = 86;
pub const AV_PIX_FMT_YUVA444P9LE: AVPixelFormat = 85;
pub const AV_PIX_FMT_YUVA444P9BE: AVPixelFormat = 84;
pub const AV_PIX_FMT_YUVA422P9LE: AVPixelFormat = 83;
pub const AV_PIX_FMT_YUVA422P9BE: AVPixelFormat = 82;
pub const AV_PIX_FMT_YUVA420P9LE: AVPixelFormat = 81;
pub const AV_PIX_FMT_YUVA420P9BE: AVPixelFormat = 80;
pub const AV_PIX_FMT_YUVA444P: AVPixelFormat = 79;
pub const AV_PIX_FMT_YUVA422P: AVPixelFormat = 78;
pub const AV_PIX_FMT_GBRP16LE: AVPixelFormat = 77;
pub const AV_PIX_FMT_GBRP16BE: AVPixelFormat = 76;
pub const AV_PIX_FMT_GBRP10LE: AVPixelFormat = 75;
pub const AV_PIX_FMT_GBRP10BE: AVPixelFormat = 74;
pub const AV_PIX_FMT_GBRP9LE: AVPixelFormat = 73;
pub const AV_PIX_FMT_GBRP9BE: AVPixelFormat = 72;
pub const AV_PIX_FMT_GBR24P: AVPixelFormat = 71;
pub const AV_PIX_FMT_GBRP: AVPixelFormat = 71;
pub const AV_PIX_FMT_YUV422P9LE: AVPixelFormat = 70;
pub const AV_PIX_FMT_YUV422P9BE: AVPixelFormat = 69;
pub const AV_PIX_FMT_YUV444P10LE: AVPixelFormat = 68;
pub const AV_PIX_FMT_YUV444P10BE: AVPixelFormat = 67;
pub const AV_PIX_FMT_YUV444P9LE: AVPixelFormat = 66;
pub const AV_PIX_FMT_YUV444P9BE: AVPixelFormat = 65;
pub const AV_PIX_FMT_YUV422P10LE: AVPixelFormat = 64;
pub const AV_PIX_FMT_YUV422P10BE: AVPixelFormat = 63;
pub const AV_PIX_FMT_YUV420P10LE: AVPixelFormat = 62;
pub const AV_PIX_FMT_YUV420P10BE: AVPixelFormat = 61;
pub const AV_PIX_FMT_YUV420P9LE: AVPixelFormat = 60;
pub const AV_PIX_FMT_YUV420P9BE: AVPixelFormat = 59;
pub const AV_PIX_FMT_BGR48LE: AVPixelFormat = 58;
pub const AV_PIX_FMT_BGR48BE: AVPixelFormat = 57;
pub const AV_PIX_FMT_GRAY8A: AVPixelFormat = 56;
pub const AV_PIX_FMT_Y400A: AVPixelFormat = 56;
pub const AV_PIX_FMT_YA8: AVPixelFormat = 56;
pub const AV_PIX_FMT_BGR444BE: AVPixelFormat = 55;
pub const AV_PIX_FMT_BGR444LE: AVPixelFormat = 54;
pub const AV_PIX_FMT_RGB444BE: AVPixelFormat = 53;
pub const AV_PIX_FMT_RGB444LE: AVPixelFormat = 52;
pub const AV_PIX_FMT_DXVA2_VLD: AVPixelFormat = 51;
pub const AV_PIX_FMT_YUV444P16BE: AVPixelFormat = 50;
pub const AV_PIX_FMT_YUV444P16LE: AVPixelFormat = 49;
pub const AV_PIX_FMT_YUV422P16BE: AVPixelFormat = 48;
pub const AV_PIX_FMT_YUV422P16LE: AVPixelFormat = 47;
pub const AV_PIX_FMT_YUV420P16BE: AVPixelFormat = 46;
pub const AV_PIX_FMT_YUV420P16LE: AVPixelFormat = 45;
pub const AV_PIX_FMT_VAAPI: AVPixelFormat = 44;
pub const AV_PIX_FMT_BGR555LE: AVPixelFormat = 43;
pub const AV_PIX_FMT_BGR555BE: AVPixelFormat = 42;
pub const AV_PIX_FMT_BGR565LE: AVPixelFormat = 41;
pub const AV_PIX_FMT_BGR565BE: AVPixelFormat = 40;
pub const AV_PIX_FMT_RGB555LE: AVPixelFormat = 39;
pub const AV_PIX_FMT_RGB555BE: AVPixelFormat = 38;
pub const AV_PIX_FMT_RGB565LE: AVPixelFormat = 37;
pub const AV_PIX_FMT_RGB565BE: AVPixelFormat = 36;
pub const AV_PIX_FMT_RGB48LE: AVPixelFormat = 35;
pub const AV_PIX_FMT_RGB48BE: AVPixelFormat = 34;
pub const AV_PIX_FMT_YUVA420P: AVPixelFormat = 33;
pub const AV_PIX_FMT_YUVJ440P: AVPixelFormat = 32;
pub const AV_PIX_FMT_YUV440P: AVPixelFormat = 31;
pub const AV_PIX_FMT_GRAY16LE: AVPixelFormat = 30;
pub const AV_PIX_FMT_GRAY16BE: AVPixelFormat = 29;
pub const AV_PIX_FMT_BGRA: AVPixelFormat = 28;
pub const AV_PIX_FMT_ABGR: AVPixelFormat = 27;
pub const AV_PIX_FMT_RGBA: AVPixelFormat = 26;
pub const AV_PIX_FMT_ARGB: AVPixelFormat = 25;
pub const AV_PIX_FMT_NV21: AVPixelFormat = 24;
pub const AV_PIX_FMT_NV12: AVPixelFormat = 23;
pub const AV_PIX_FMT_RGB4_BYTE: AVPixelFormat = 22;
pub const AV_PIX_FMT_RGB4: AVPixelFormat = 21;
pub const AV_PIX_FMT_RGB8: AVPixelFormat = 20;
pub const AV_PIX_FMT_BGR4_BYTE: AVPixelFormat = 19;
pub const AV_PIX_FMT_BGR4: AVPixelFormat = 18;
pub const AV_PIX_FMT_BGR8: AVPixelFormat = 17;
pub const AV_PIX_FMT_UYYVYY411: AVPixelFormat = 16;
pub const AV_PIX_FMT_UYVY422: AVPixelFormat = 15;
pub const AV_PIX_FMT_YUVJ444P: AVPixelFormat = 14;
pub const AV_PIX_FMT_YUVJ422P: AVPixelFormat = 13;
pub const AV_PIX_FMT_YUVJ420P: AVPixelFormat = 12;
pub const AV_PIX_FMT_PAL8: AVPixelFormat = 11;
pub const AV_PIX_FMT_MONOBLACK: AVPixelFormat = 10;
pub const AV_PIX_FMT_MONOWHITE: AVPixelFormat = 9;
pub const AV_PIX_FMT_GRAY8: AVPixelFormat = 8;
pub const AV_PIX_FMT_YUV411P: AVPixelFormat = 7;
pub const AV_PIX_FMT_YUV410P: AVPixelFormat = 6;
pub const AV_PIX_FMT_YUV444P: AVPixelFormat = 5;
pub const AV_PIX_FMT_YUV422P: AVPixelFormat = 4;
pub const AV_PIX_FMT_BGR24: AVPixelFormat = 3;
pub const AV_PIX_FMT_RGB24: AVPixelFormat = 2;
pub const AV_PIX_FMT_YUYV422: AVPixelFormat = 1;
pub const AV_PIX_FMT_YUV420P: AVPixelFormat = 0;
pub const AV_PIX_FMT_NONE: AVPixelFormat = -1;
pub type AVColorPrimaries = ::core::ffi::c_uint;
pub const AVCOL_PRI_EXT_NB: AVColorPrimaries = 257;
pub const AVCOL_PRI_V_GAMUT: AVColorPrimaries = 256;
pub const AVCOL_PRI_EXT_BASE: AVColorPrimaries = 256;
pub const AVCOL_PRI_NB: AVColorPrimaries = 23;
pub const AVCOL_PRI_JEDEC_P22: AVColorPrimaries = 22;
pub const AVCOL_PRI_EBU3213: AVColorPrimaries = 22;
pub const AVCOL_PRI_SMPTE432: AVColorPrimaries = 12;
pub const AVCOL_PRI_SMPTE431: AVColorPrimaries = 11;
pub const AVCOL_PRI_SMPTEST428_1: AVColorPrimaries = 10;
pub const AVCOL_PRI_SMPTE428: AVColorPrimaries = 10;
pub const AVCOL_PRI_BT2020: AVColorPrimaries = 9;
pub const AVCOL_PRI_FILM: AVColorPrimaries = 8;
pub const AVCOL_PRI_SMPTE240M: AVColorPrimaries = 7;
pub const AVCOL_PRI_SMPTE170M: AVColorPrimaries = 6;
pub const AVCOL_PRI_BT470BG: AVColorPrimaries = 5;
pub const AVCOL_PRI_BT470M: AVColorPrimaries = 4;
pub const AVCOL_PRI_RESERVED: AVColorPrimaries = 3;
pub const AVCOL_PRI_UNSPECIFIED: AVColorPrimaries = 2;
pub const AVCOL_PRI_BT709: AVColorPrimaries = 1;
pub const AVCOL_PRI_RESERVED0: AVColorPrimaries = 0;
pub type AVColorTransferCharacteristic = ::core::ffi::c_uint;
pub const AVCOL_TRC_EXT_NB: AVColorTransferCharacteristic = 257;
pub const AVCOL_TRC_V_LOG: AVColorTransferCharacteristic = 256;
pub const AVCOL_TRC_EXT_BASE: AVColorTransferCharacteristic = 256;
pub const AVCOL_TRC_NB: AVColorTransferCharacteristic = 19;
pub const AVCOL_TRC_ARIB_STD_B67: AVColorTransferCharacteristic = 18;
pub const AVCOL_TRC_SMPTEST428_1: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTE428: AVColorTransferCharacteristic = 17;
pub const AVCOL_TRC_SMPTEST2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_SMPTE2084: AVColorTransferCharacteristic = 16;
pub const AVCOL_TRC_BT2020_12: AVColorTransferCharacteristic = 15;
pub const AVCOL_TRC_BT2020_10: AVColorTransferCharacteristic = 14;
pub const AVCOL_TRC_IEC61966_2_1: AVColorTransferCharacteristic = 13;
pub const AVCOL_TRC_BT1361_ECG: AVColorTransferCharacteristic = 12;
pub const AVCOL_TRC_IEC61966_2_4: AVColorTransferCharacteristic = 11;
pub const AVCOL_TRC_LOG_SQRT: AVColorTransferCharacteristic = 10;
pub const AVCOL_TRC_LOG: AVColorTransferCharacteristic = 9;
pub const AVCOL_TRC_LINEAR: AVColorTransferCharacteristic = 8;
pub const AVCOL_TRC_SMPTE240M: AVColorTransferCharacteristic = 7;
pub const AVCOL_TRC_SMPTE170M: AVColorTransferCharacteristic = 6;
pub const AVCOL_TRC_GAMMA28: AVColorTransferCharacteristic = 5;
pub const AVCOL_TRC_GAMMA22: AVColorTransferCharacteristic = 4;
pub const AVCOL_TRC_RESERVED: AVColorTransferCharacteristic = 3;
pub const AVCOL_TRC_UNSPECIFIED: AVColorTransferCharacteristic = 2;
pub const AVCOL_TRC_BT709: AVColorTransferCharacteristic = 1;
pub const AVCOL_TRC_RESERVED0: AVColorTransferCharacteristic = 0;
pub type AVColorSpace = ::core::ffi::c_uint;
pub const AVCOL_SPC_NB: AVColorSpace = 18;
pub const AVCOL_SPC_YCGCO_RO: AVColorSpace = 17;
pub const AVCOL_SPC_YCGCO_RE: AVColorSpace = 16;
pub const AVCOL_SPC_IPT_C2: AVColorSpace = 15;
pub const AVCOL_SPC_ICTCP: AVColorSpace = 14;
pub const AVCOL_SPC_CHROMA_DERIVED_CL: AVColorSpace = 13;
pub const AVCOL_SPC_CHROMA_DERIVED_NCL: AVColorSpace = 12;
pub const AVCOL_SPC_SMPTE2085: AVColorSpace = 11;
pub const AVCOL_SPC_BT2020_CL: AVColorSpace = 10;
pub const AVCOL_SPC_BT2020_NCL: AVColorSpace = 9;
pub const AVCOL_SPC_YCOCG: AVColorSpace = 8;
pub const AVCOL_SPC_YCGCO: AVColorSpace = 8;
pub const AVCOL_SPC_SMPTE240M: AVColorSpace = 7;
pub const AVCOL_SPC_SMPTE170M: AVColorSpace = 6;
pub const AVCOL_SPC_BT470BG: AVColorSpace = 5;
pub const AVCOL_SPC_FCC: AVColorSpace = 4;
pub const AVCOL_SPC_RESERVED: AVColorSpace = 3;
pub const AVCOL_SPC_UNSPECIFIED: AVColorSpace = 2;
pub const AVCOL_SPC_BT709: AVColorSpace = 1;
pub const AVCOL_SPC_RGB: AVColorSpace = 0;
pub type AVColorRange = ::core::ffi::c_uint;
pub const AVCOL_RANGE_NB: AVColorRange = 3;
pub const AVCOL_RANGE_JPEG: AVColorRange = 2;
pub const AVCOL_RANGE_MPEG: AVColorRange = 1;
pub const AVCOL_RANGE_UNSPECIFIED: AVColorRange = 0;
pub type AVChromaLocation = ::core::ffi::c_uint;
pub const AVCHROMA_LOC_NB: AVChromaLocation = 7;
pub const AVCHROMA_LOC_BOTTOM: AVChromaLocation = 6;
pub const AVCHROMA_LOC_BOTTOMLEFT: AVChromaLocation = 5;
pub const AVCHROMA_LOC_TOP: AVChromaLocation = 4;
pub const AVCHROMA_LOC_TOPLEFT: AVChromaLocation = 3;
pub const AVCHROMA_LOC_CENTER: AVChromaLocation = 2;
pub const AVCHROMA_LOC_LEFT: AVChromaLocation = 1;
pub const AVCHROMA_LOC_UNSPECIFIED: AVChromaLocation = 0;
pub type AVAlphaMode = ::core::ffi::c_uint;
pub const AVALPHA_MODE_NB: AVAlphaMode = 3;
pub const AVALPHA_MODE_STRAIGHT: AVAlphaMode = 2;
pub const AVALPHA_MODE_PREMULTIPLIED: AVAlphaMode = 1;
pub const AVALPHA_MODE_UNSPECIFIED: AVAlphaMode = 0;
pub type AVChannel = ::core::ffi::c_int;
pub const AV_CHAN_AMBISONIC_END: AVChannel = 2047;
pub const AV_CHAN_AMBISONIC_BASE: AVChannel = 1024;
pub const AV_CHAN_UNKNOWN: AVChannel = 768;
pub const AV_CHAN_UNUSED: AVChannel = 512;
pub const AV_CHAN_BINAURAL_RIGHT: AVChannel = 62;
pub const AV_CHAN_BINAURAL_LEFT: AVChannel = 61;
pub const AV_CHAN_TOP_SURROUND_RIGHT: AVChannel = 44;
pub const AV_CHAN_TOP_SURROUND_LEFT: AVChannel = 43;
pub const AV_CHAN_SIDE_SURROUND_RIGHT: AVChannel = 42;
pub const AV_CHAN_SIDE_SURROUND_LEFT: AVChannel = 41;
pub const AV_CHAN_BOTTOM_FRONT_RIGHT: AVChannel = 40;
pub const AV_CHAN_BOTTOM_FRONT_LEFT: AVChannel = 39;
pub const AV_CHAN_BOTTOM_FRONT_CENTER: AVChannel = 38;
pub const AV_CHAN_TOP_SIDE_RIGHT: AVChannel = 37;
pub const AV_CHAN_TOP_SIDE_LEFT: AVChannel = 36;
pub const AV_CHAN_LOW_FREQUENCY_2: AVChannel = 35;
pub const AV_CHAN_SURROUND_DIRECT_RIGHT: AVChannel = 34;
pub const AV_CHAN_SURROUND_DIRECT_LEFT: AVChannel = 33;
pub const AV_CHAN_WIDE_RIGHT: AVChannel = 32;
pub const AV_CHAN_WIDE_LEFT: AVChannel = 31;
pub const AV_CHAN_STEREO_RIGHT: AVChannel = 30;
pub const AV_CHAN_STEREO_LEFT: AVChannel = 29;
pub const AV_CHAN_TOP_BACK_RIGHT: AVChannel = 17;
pub const AV_CHAN_TOP_BACK_CENTER: AVChannel = 16;
pub const AV_CHAN_TOP_BACK_LEFT: AVChannel = 15;
pub const AV_CHAN_TOP_FRONT_RIGHT: AVChannel = 14;
pub const AV_CHAN_TOP_FRONT_CENTER: AVChannel = 13;
pub const AV_CHAN_TOP_FRONT_LEFT: AVChannel = 12;
pub const AV_CHAN_TOP_CENTER: AVChannel = 11;
pub const AV_CHAN_SIDE_RIGHT: AVChannel = 10;
pub const AV_CHAN_SIDE_LEFT: AVChannel = 9;
pub const AV_CHAN_BACK_CENTER: AVChannel = 8;
pub const AV_CHAN_FRONT_RIGHT_OF_CENTER: AVChannel = 7;
pub const AV_CHAN_FRONT_LEFT_OF_CENTER: AVChannel = 6;
pub const AV_CHAN_BACK_RIGHT: AVChannel = 5;
pub const AV_CHAN_BACK_LEFT: AVChannel = 4;
pub const AV_CHAN_LOW_FREQUENCY: AVChannel = 3;
pub const AV_CHAN_FRONT_CENTER: AVChannel = 2;
pub const AV_CHAN_FRONT_RIGHT: AVChannel = 1;
pub const AV_CHAN_FRONT_LEFT: AVChannel = 0;
pub const AV_CHAN_NONE: AVChannel = -1;
pub type AVChannelOrder = ::core::ffi::c_uint;
pub const FF_CHANNEL_ORDER_NB: AVChannelOrder = 4;
pub const AV_CHANNEL_ORDER_AMBISONIC: AVChannelOrder = 3;
pub const AV_CHANNEL_ORDER_CUSTOM: AVChannelOrder = 2;
pub const AV_CHANNEL_ORDER_NATIVE: AVChannelOrder = 1;
pub const AV_CHANNEL_ORDER_UNSPEC: AVChannelOrder = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChannelCustom {
    pub id: AVChannel,
    pub name: [::core::ffi::c_char; 16],
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVChannelLayout {
    pub order: AVChannelOrder,
    pub nb_channels: ::core::ffi::c_int,
    pub u: C2RustUnnamed,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub mask: uint64_t,
    pub map: *mut AVChannelCustom,
}
pub type AVFrameSideDataType = ::core::ffi::c_uint;
pub const AV_FRAME_DATA_RAW_COLOR_PARAMS: AVFrameSideDataType = 36;
pub const AV_FRAME_DATA_IAMF_RECON_GAIN_INFO_PARAM: AVFrameSideDataType = 35;
pub const AV_FRAME_DATA_IAMF_DEMIXING_INFO_PARAM: AVFrameSideDataType = 34;
pub const AV_FRAME_DATA_IAMF_MIX_GAIN_PARAM: AVFrameSideDataType = 33;
pub const AV_FRAME_DATA_DYNAMIC_HDR_SMPTE_2094_APP5: AVFrameSideDataType = 32;
pub const AV_FRAME_DATA_EXIF: AVFrameSideDataType = 31;
pub const AV_FRAME_DATA_3D_REFERENCE_DISPLAYS: AVFrameSideDataType = 30;
pub const AV_FRAME_DATA_VIEW_ID: AVFrameSideDataType = 29;
pub const AV_FRAME_DATA_LCEVC: AVFrameSideDataType = 28;
pub const AV_FRAME_DATA_VIDEO_HINT: AVFrameSideDataType = 27;
pub const AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT: AVFrameSideDataType = 26;
pub const AV_FRAME_DATA_DYNAMIC_HDR_VIVID: AVFrameSideDataType = 25;
pub const AV_FRAME_DATA_DOVI_METADATA: AVFrameSideDataType = 24;
pub const AV_FRAME_DATA_DOVI_RPU_BUFFER: AVFrameSideDataType = 23;
pub const AV_FRAME_DATA_DETECTION_BBOXES: AVFrameSideDataType = 22;
pub const AV_FRAME_DATA_FILM_GRAIN_PARAMS: AVFrameSideDataType = 21;
pub const AV_FRAME_DATA_SEI_UNREGISTERED: AVFrameSideDataType = 20;
pub const AV_FRAME_DATA_VIDEO_ENC_PARAMS: AVFrameSideDataType = 19;
pub const AV_FRAME_DATA_REGIONS_OF_INTEREST: AVFrameSideDataType = 18;
pub const AV_FRAME_DATA_DYNAMIC_HDR_PLUS: AVFrameSideDataType = 17;
pub const AV_FRAME_DATA_S12M_TIMECODE: AVFrameSideDataType = 16;
pub const AV_FRAME_DATA_ICC_PROFILE: AVFrameSideDataType = 15;
pub const AV_FRAME_DATA_CONTENT_LIGHT_LEVEL: AVFrameSideDataType = 14;
pub const AV_FRAME_DATA_SPHERICAL: AVFrameSideDataType = 13;
pub const AV_FRAME_DATA_GOP_TIMECODE: AVFrameSideDataType = 12;
pub const AV_FRAME_DATA_MASTERING_DISPLAY_METADATA: AVFrameSideDataType = 11;
pub const AV_FRAME_DATA_AUDIO_SERVICE_TYPE: AVFrameSideDataType = 10;
pub const AV_FRAME_DATA_SKIP_SAMPLES: AVFrameSideDataType = 9;
pub const AV_FRAME_DATA_MOTION_VECTORS: AVFrameSideDataType = 8;
pub const AV_FRAME_DATA_AFD: AVFrameSideDataType = 7;
pub const AV_FRAME_DATA_DISPLAYMATRIX: AVFrameSideDataType = 6;
pub const AV_FRAME_DATA_REPLAYGAIN: AVFrameSideDataType = 5;
pub const AV_FRAME_DATA_DOWNMIX_INFO: AVFrameSideDataType = 4;
pub const AV_FRAME_DATA_MATRIXENCODING: AVFrameSideDataType = 3;
pub const AV_FRAME_DATA_STEREO3D: AVFrameSideDataType = 2;
pub const AV_FRAME_DATA_A53_CC: AVFrameSideDataType = 1;
pub const AV_FRAME_DATA_PANSCAN: AVFrameSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrameSideData {
    pub type_0: AVFrameSideDataType,
    pub data: *mut uint8_t,
    pub size: size_t,
    pub metadata: *mut AVDictionary,
    pub buf: *mut AVBufferRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFrame {
    pub data: [*mut uint8_t; 8],
    pub linesize: [::core::ffi::c_int; 8],
    pub extended_data: *mut *mut uint8_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub nb_samples: ::core::ffi::c_int,
    pub format: ::core::ffi::c_int,
    pub pict_type: AVPictureType,
    pub sample_aspect_ratio: AVRational,
    pub pts: int64_t,
    pub pkt_dts: int64_t,
    pub time_base: AVRational,
    pub quality: ::core::ffi::c_int,
    pub opaque: *mut ::core::ffi::c_void,
    pub repeat_pict: ::core::ffi::c_int,
    pub sample_rate: ::core::ffi::c_int,
    pub buf: [*mut AVBufferRef; 8],
    pub extended_buf: *mut *mut AVBufferRef,
    pub nb_extended_buf: ::core::ffi::c_int,
    pub side_data: *mut *mut AVFrameSideData,
    pub nb_side_data: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub best_effort_timestamp: int64_t,
    pub metadata: *mut AVDictionary,
    pub decode_error_flags: ::core::ffi::c_int,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub opaque_ref: *mut AVBufferRef,
    pub crop_top: size_t,
    pub crop_bottom: size_t,
    pub crop_left: size_t,
    pub crop_right: size_t,
    pub private_ref: *mut ::core::ffi::c_void,
    pub ch_layout: AVChannelLayout,
    pub duration: int64_t,
    pub alpha_mode: AVAlphaMode,
}
pub type AVHWDeviceType = ::core::ffi::c_uint;
pub const AV_HWDEVICE_TYPE_OHCODEC: AVHWDeviceType = 14;
pub const AV_HWDEVICE_TYPE_AMF: AVHWDeviceType = 13;
pub const AV_HWDEVICE_TYPE_D3D12VA: AVHWDeviceType = 12;
pub const AV_HWDEVICE_TYPE_VULKAN: AVHWDeviceType = 11;
pub const AV_HWDEVICE_TYPE_MEDIACODEC: AVHWDeviceType = 10;
pub const AV_HWDEVICE_TYPE_OPENCL: AVHWDeviceType = 9;
pub const AV_HWDEVICE_TYPE_DRM: AVHWDeviceType = 8;
pub const AV_HWDEVICE_TYPE_D3D11VA: AVHWDeviceType = 7;
pub const AV_HWDEVICE_TYPE_VIDEOTOOLBOX: AVHWDeviceType = 6;
pub const AV_HWDEVICE_TYPE_QSV: AVHWDeviceType = 5;
pub const AV_HWDEVICE_TYPE_DXVA2: AVHWDeviceType = 4;
pub const AV_HWDEVICE_TYPE_VAAPI: AVHWDeviceType = 3;
pub const AV_HWDEVICE_TYPE_CUDA: AVHWDeviceType = 2;
pub const AV_HWDEVICE_TYPE_VDPAU: AVHWDeviceType = 1;
pub const AV_HWDEVICE_TYPE_NONE: AVHWDeviceType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWDeviceContext {
    pub av_class: *const AVClass,
    pub type_0: AVHWDeviceType,
    pub hwctx: *mut ::core::ffi::c_void,
    pub free: Option<unsafe extern "C" fn(*mut AVHWDeviceContext) -> ()>,
    pub user_opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWFramesContext {
    pub av_class: *const AVClass,
    pub device_ref: *mut AVBufferRef,
    pub device_ctx: *mut AVHWDeviceContext,
    pub hwctx: *mut ::core::ffi::c_void,
    pub free: Option<unsafe extern "C" fn(*mut AVHWFramesContext) -> ()>,
    pub user_opaque: *mut ::core::ffi::c_void,
    pub pool: *mut AVBufferPool,
    pub initial_pool_size: ::core::ffi::c_int,
    pub format: AVPixelFormat,
    pub sw_format: AVPixelFormat,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HWContextType {
    pub type_0: AVHWDeviceType,
    pub name: *const ::core::ffi::c_char,
    pub pix_fmts: *const AVPixelFormat,
    pub device_hwctx_size: size_t,
    pub device_hwconfig_size: size_t,
    pub frames_hwctx_size: size_t,
    pub device_create: Option<
        unsafe extern "C" fn(
            *mut AVHWDeviceContext,
            *const ::core::ffi::c_char,
            *mut AVDictionary,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub device_derive: Option<
        unsafe extern "C" fn(
            *mut AVHWDeviceContext,
            *mut AVHWDeviceContext,
            *mut AVDictionary,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub device_init: Option<
        unsafe extern "C" fn(*mut AVHWDeviceContext) -> ::core::ffi::c_int,
    >,
    pub device_uninit: Option<unsafe extern "C" fn(*mut AVHWDeviceContext) -> ()>,
    pub frames_get_constraints: Option<
        unsafe extern "C" fn(
            *mut AVHWDeviceContext,
            *const ::core::ffi::c_void,
            *mut AVHWFramesConstraints,
        ) -> ::core::ffi::c_int,
    >,
    pub frames_init: Option<
        unsafe extern "C" fn(*mut AVHWFramesContext) -> ::core::ffi::c_int,
    >,
    pub frames_uninit: Option<unsafe extern "C" fn(*mut AVHWFramesContext) -> ()>,
    pub frames_get_buffer: Option<
        unsafe extern "C" fn(*mut AVHWFramesContext, *mut AVFrame) -> ::core::ffi::c_int,
    >,
    pub transfer_get_formats: Option<
        unsafe extern "C" fn(
            *mut AVHWFramesContext,
            AVHWFrameTransferDirection,
            *mut *mut AVPixelFormat,
        ) -> ::core::ffi::c_int,
    >,
    pub transfer_data_to: Option<
        unsafe extern "C" fn(
            *mut AVHWFramesContext,
            *mut AVFrame,
            *const AVFrame,
        ) -> ::core::ffi::c_int,
    >,
    pub transfer_data_from: Option<
        unsafe extern "C" fn(
            *mut AVHWFramesContext,
            *mut AVFrame,
            *const AVFrame,
        ) -> ::core::ffi::c_int,
    >,
    pub map_to: Option<
        unsafe extern "C" fn(
            *mut AVHWFramesContext,
            *mut AVFrame,
            *const AVFrame,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub map_from: Option<
        unsafe extern "C" fn(
            *mut AVHWFramesContext,
            *mut AVFrame,
            *const AVFrame,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub frames_derive_to: Option<
        unsafe extern "C" fn(
            *mut AVHWFramesContext,
            *mut AVHWFramesContext,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub frames_derive_from: Option<
        unsafe extern "C" fn(
            *mut AVHWFramesContext,
            *mut AVHWFramesContext,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
}
pub type AVHWFrameTransferDirection = ::core::ffi::c_uint;
pub const AV_HWFRAME_TRANSFER_DIRECTION_TO: AVHWFrameTransferDirection = 1;
pub const AV_HWFRAME_TRANSFER_DIRECTION_FROM: AVHWFrameTransferDirection = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWFramesConstraints {
    pub valid_hw_formats: *mut AVPixelFormat,
    pub valid_sw_formats: *mut AVPixelFormat,
    pub min_width: ::core::ffi::c_int,
    pub min_height: ::core::ffi::c_int,
    pub max_width: ::core::ffi::c_int,
    pub max_height: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFHWDeviceContext {
    pub p: AVHWDeviceContext,
    pub hw_type: *const HWContextType,
    pub source_device: *mut AVBufferRef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFHWFramesContext {
    pub p: AVHWFramesContext,
    pub hw_type: *const HWContextType,
    pub pool_internal: *mut AVBufferPool,
    pub source_frames: *mut AVBufferRef,
    pub source_allocation_map_flags: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct HWMapDescriptor {
    pub source: *mut AVFrame,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub unmap: Option<
        unsafe extern "C" fn(*mut AVHWFramesContext, *mut HWMapDescriptor) -> (),
    >,
    pub priv_0: *mut ::core::ffi::c_void,
}
pub type C2RustUnnamed_0 = ::core::ffi::c_uint;
pub const AV_HWFRAME_MAP_DIRECT: C2RustUnnamed_0 = 8;
pub const AV_HWFRAME_MAP_OVERWRITE: C2RustUnnamed_0 = 4;
pub const AV_HWFRAME_MAP_WRITE: C2RustUnnamed_0 = 2;
pub const AV_HWFRAME_MAP_READ: C2RustUnnamed_0 = 1;
static mut hw_table: [*const HWContextType; 1] = [::core::ptr::null::<HWContextType>()];
static mut hw_type_names: [*const ::core::ffi::c_char; 15] = [
    ::core::ptr::null::<::core::ffi::c_char>(),
    b"vdpau\0" as *const u8 as *const ::core::ffi::c_char,
    b"cuda\0" as *const u8 as *const ::core::ffi::c_char,
    b"vaapi\0" as *const u8 as *const ::core::ffi::c_char,
    b"dxva2\0" as *const u8 as *const ::core::ffi::c_char,
    b"qsv\0" as *const u8 as *const ::core::ffi::c_char,
    b"videotoolbox\0" as *const u8 as *const ::core::ffi::c_char,
    b"d3d11va\0" as *const u8 as *const ::core::ffi::c_char,
    b"drm\0" as *const u8 as *const ::core::ffi::c_char,
    b"opencl\0" as *const u8 as *const ::core::ffi::c_char,
    b"mediacodec\0" as *const u8 as *const ::core::ffi::c_char,
    b"vulkan\0" as *const u8 as *const ::core::ffi::c_char,
    b"d3d12va\0" as *const u8 as *const ::core::ffi::c_char,
    b"amf\0" as *const u8 as *const ::core::ffi::c_char,
    b"ohcodec\0" as *const u8 as *const ::core::ffi::c_char,
];
#[no_mangle]
pub unsafe extern "C" fn av_hwdevice_find_type_by_name(
    mut name: *const ::core::ffi::c_char,
) -> AVHWDeviceType {
    let mut type_0: ::core::ffi::c_int = 0;
    type_0 = 0 as ::core::ffi::c_int;
    while (type_0 as usize)
        < (::core::mem::size_of::<[*const ::core::ffi::c_char; 15]>() as usize)
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as usize)
    {
        if !hw_type_names[type_0 as usize].is_null()
            && strcmp(hw_type_names[type_0 as usize], name) == 0
        {
            return type_0 as AVHWDeviceType;
        }
        type_0 += 1;
    }
    return AV_HWDEVICE_TYPE_NONE;
}
#[no_mangle]
pub unsafe extern "C" fn av_hwdevice_get_type_name(
    mut type_0: AVHWDeviceType,
) -> *const ::core::ffi::c_char {
    if type_0 as ::core::ffi::c_uint
        > AV_HWDEVICE_TYPE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        && (type_0 as usize)
            < (::core::mem::size_of::<[*const ::core::ffi::c_char; 15]>() as usize)
                .wrapping_div(
                    ::core::mem::size_of::<*const ::core::ffi::c_char>() as usize,
                )
    {
        return hw_type_names[type_0 as usize]
    } else {
        return ::core::ptr::null::<::core::ffi::c_char>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_hwdevice_iterate_types(
    mut prev: AVHWDeviceType,
) -> AVHWDeviceType {
    let mut next: AVHWDeviceType = AV_HWDEVICE_TYPE_NONE;
    let mut i: ::core::ffi::c_int = 0;
    let mut set: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while !hw_table[i as usize].is_null() {
        if !(prev as ::core::ffi::c_uint
            != AV_HWDEVICE_TYPE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*hw_table[i as usize]).type_0 as ::core::ffi::c_uint
                <= prev as ::core::ffi::c_uint)
        {
            if set == 0
                || ((*hw_table[i as usize]).type_0 as ::core::ffi::c_uint)
                    < next as ::core::ffi::c_uint
            {
                next = (*hw_table[i as usize]).type_0;
                set = 1 as ::core::ffi::c_int;
            }
        }
        i += 1;
    }
    return (if set != 0 {
        next as ::core::ffi::c_uint
    } else {
        AV_HWDEVICE_TYPE_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    }) as AVHWDeviceType;
}
unsafe extern "C" fn hwdevice_ctx_get_name(
    mut ptr: *mut ::core::ffi::c_void,
) -> *const ::core::ffi::c_char {
    let mut ctx: *mut FFHWDeviceContext = ptr as *mut FFHWDeviceContext;
    return (*(*ctx).hw_type).name;
}
static mut hwdevice_ctx_class: AVClass = unsafe {
    AVClass {
        class_name: b"AVHWDeviceContext\0" as *const u8 as *const ::core::ffi::c_char,
        item_name: Some(
            hwdevice_ctx_get_name
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *const ::core::ffi::c_char,
        ),
        option: ::core::ptr::null::<AVOption>(),
        version: (61 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | 101 as ::core::ffi::c_int,
        log_level_offset_offset: 0,
        parent_log_context_offset: 0,
        category: AV_CLASS_CATEGORY_HWDEVICE,
        get_category: None,
        query_ranges: None,
        child_next: None,
        child_class_iterate: None,
        state_flags_offset: 0,
    }
};
unsafe extern "C" fn hwdevice_ctx_free(
    mut opaque: *mut ::core::ffi::c_void,
    mut data: *mut uint8_t,
) {
    let mut ctxi: *mut FFHWDeviceContext = data as *mut FFHWDeviceContext;
    let mut ctx: *mut AVHWDeviceContext = &raw mut (*ctxi).p;
    if (*(*ctxi).hw_type).device_uninit.is_some() {
        (*(*ctxi).hw_type).device_uninit.expect("non-null function pointer")(ctx);
    }
    if (*ctx).free.is_some() {
        (*ctx).free.expect("non-null function pointer")(ctx as *mut AVHWDeviceContext);
    }
    av_buffer_unref(&raw mut (*ctxi).source_device);
    av_freep(&raw mut (*ctx).hwctx as *mut ::core::ffi::c_void);
    av_freep(&raw mut ctx as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_hwdevice_ctx_alloc(
    mut type_0: AVHWDeviceType,
) -> *mut AVBufferRef {
    let mut current_block: u64;
    let mut ctxi: *mut FFHWDeviceContext = ::core::ptr::null_mut::<FFHWDeviceContext>();
    let mut ctx: *mut AVHWDeviceContext = ::core::ptr::null_mut::<AVHWDeviceContext>();
    let mut buf: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut hw_type: *const HWContextType = ::core::ptr::null::<HWContextType>();
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while !hw_table[i as usize].is_null() {
        if (*hw_table[i as usize]).type_0 as ::core::ffi::c_uint
            == type_0 as ::core::ffi::c_uint
        {
            hw_type = hw_table[i as usize];
            break;
        } else {
            i += 1;
        }
    }
    if hw_type.is_null() {
        return ::core::ptr::null_mut::<AVBufferRef>();
    }
    ctxi = av_mallocz(::core::mem::size_of::<FFHWDeviceContext>() as size_t)
        as *mut FFHWDeviceContext;
    if ctxi.is_null() {
        return ::core::ptr::null_mut::<AVBufferRef>();
    }
    ctx = &raw mut (*ctxi).p;
    if (*hw_type).device_hwctx_size != 0 {
        (*ctx).hwctx = av_mallocz((*hw_type).device_hwctx_size);
        if (*ctx).hwctx.is_null() {
            current_block = 9139567895286454764;
        } else {
            current_block = 10599921512955367680;
        }
    } else {
        current_block = 10599921512955367680;
    }
    match current_block {
        10599921512955367680 => {
            buf = av_buffer_create(
                ctx as *mut uint8_t,
                ::core::mem::size_of::<AVHWDeviceContext>() as size_t,
                Some(
                    hwdevice_ctx_free
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut uint8_t,
                        ) -> (),
                ),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            );
            if !buf.is_null() {
                (*ctx).type_0 = type_0;
                (*ctx).av_class = &raw const hwdevice_ctx_class;
                (*ctxi).hw_type = hw_type;
                return buf;
            }
        }
        _ => {}
    }
    av_freep(&raw mut (*ctx).hwctx as *mut ::core::ffi::c_void);
    av_freep(&raw mut ctx as *mut ::core::ffi::c_void);
    return ::core::ptr::null_mut::<AVBufferRef>();
}
#[no_mangle]
pub unsafe extern "C" fn av_hwdevice_ctx_init(
    mut ref_0: *mut AVBufferRef,
) -> ::core::ffi::c_int {
    let mut ctxi: *mut FFHWDeviceContext = (*ref_0).data as *mut FFHWDeviceContext;
    let mut ctx: *mut AVHWDeviceContext = &raw mut (*ctxi).p;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*(*ctxi).hw_type).device_init.is_some() {
        ret = (*(*ctxi).hw_type).device_init.expect("non-null function pointer")(ctx);
    }
    return ret;
}
static mut hwframe_ctx_class: AVClass = unsafe {
    AVClass {
        class_name: b"AVHWFramesContext\0" as *const u8 as *const ::core::ffi::c_char,
        item_name: Some(
            av_default_item_name
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *const ::core::ffi::c_char,
        ),
        option: ::core::ptr::null::<AVOption>(),
        version: (61 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | 101 as ::core::ffi::c_int,
        log_level_offset_offset: 0,
        parent_log_context_offset: 0,
        category: AV_CLASS_CATEGORY_NA,
        get_category: None,
        query_ranges: None,
        child_next: None,
        child_class_iterate: None,
        state_flags_offset: 0,
    }
};
unsafe extern "C" fn hwframe_ctx_free(
    mut opaque: *mut ::core::ffi::c_void,
    mut data: *mut uint8_t,
) {
    let mut ctxi: *mut FFHWFramesContext = data as *mut FFHWFramesContext;
    let mut ctx: *mut AVHWFramesContext = &raw mut (*ctxi).p;
    if !(*ctxi).pool_internal.is_null() {
        av_buffer_pool_uninit(&raw mut (*ctxi).pool_internal);
    }
    if (*(*ctxi).hw_type).frames_uninit.is_some() {
        (*(*ctxi).hw_type).frames_uninit.expect("non-null function pointer")(ctx);
    }
    if (*ctx).free.is_some() {
        (*ctx).free.expect("non-null function pointer")(ctx as *mut AVHWFramesContext);
    }
    av_buffer_unref(&raw mut (*ctxi).source_frames);
    av_buffer_unref(&raw mut (*ctx).device_ref);
    av_freep(&raw mut (*ctx).hwctx as *mut ::core::ffi::c_void);
    av_freep(&raw mut ctx as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_hwframe_ctx_alloc(
    mut device_ref_in: *mut AVBufferRef,
) -> *mut AVBufferRef {
    let mut current_block: u64;
    let mut device_ctx: *mut FFHWDeviceContext = (*device_ref_in).data
        as *mut FFHWDeviceContext;
    let mut hw_type: *const HWContextType = (*device_ctx).hw_type;
    let mut ctxi: *mut FFHWFramesContext = ::core::ptr::null_mut::<FFHWFramesContext>();
    let mut ctx: *mut AVHWFramesContext = ::core::ptr::null_mut::<AVHWFramesContext>();
    let mut buf: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut device_ref: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    ctxi = av_mallocz(::core::mem::size_of::<FFHWFramesContext>() as size_t)
        as *mut FFHWFramesContext;
    if ctxi.is_null() {
        return ::core::ptr::null_mut::<AVBufferRef>();
    }
    ctx = &raw mut (*ctxi).p;
    if (*hw_type).frames_hwctx_size != 0 {
        (*ctx).hwctx = av_mallocz((*hw_type).frames_hwctx_size);
        if (*ctx).hwctx.is_null() {
            current_block = 10334003491957544446;
        } else {
            current_block = 7351195479953500246;
        }
    } else {
        current_block = 7351195479953500246;
    }
    match current_block {
        7351195479953500246 => {
            device_ref = av_buffer_ref(device_ref_in);
            if !device_ref.is_null() {
                buf = av_buffer_create(
                    ctx as *mut uint8_t,
                    ::core::mem::size_of::<AVHWFramesContext>() as size_t,
                    Some(
                        hwframe_ctx_free
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut uint8_t,
                            ) -> (),
                    ),
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
                );
                if !buf.is_null() {
                    (*ctx).av_class = &raw const hwframe_ctx_class;
                    (*ctx).device_ref = device_ref;
                    (*ctx).device_ctx = &raw mut (*device_ctx).p;
                    (*ctx).format = AV_PIX_FMT_NONE;
                    (*ctx).sw_format = AV_PIX_FMT_NONE;
                    (*ctxi).hw_type = hw_type;
                    return buf;
                }
            }
        }
        _ => {}
    }
    av_buffer_unref(&raw mut device_ref);
    av_freep(&raw mut (*ctx).hwctx as *mut ::core::ffi::c_void);
    av_freep(&raw mut ctx as *mut ::core::ffi::c_void);
    return ::core::ptr::null_mut::<AVBufferRef>();
}
unsafe extern "C" fn hwframe_pool_prealloc(
    mut ref_0: *mut AVBufferRef,
) -> ::core::ffi::c_int {
    let mut ctx: *mut AVHWFramesContext = (*ref_0).data as *mut AVHWFramesContext;
    let mut frames: *mut *mut AVFrame = ::core::ptr::null_mut::<*mut AVFrame>();
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    frames = av_calloc(
        (*ctx).initial_pool_size as size_t,
        ::core::mem::size_of::<*mut AVFrame>() as size_t,
    ) as *mut *mut AVFrame;
    if frames.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*ctx).initial_pool_size {
        let ref mut fresh0 = *frames.offset(i as isize);
        *fresh0 = av_frame_alloc();
        if (*frames.offset(i as isize)).is_null() {
            break;
        }
        ret = av_hwframe_get_buffer(
            ref_0,
            *frames.offset(i as isize),
            0 as ::core::ffi::c_int,
        );
        if ret < 0 as ::core::ffi::c_int {
            break;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*ctx).initial_pool_size {
        av_frame_free(frames.offset(i as isize) as *mut *mut AVFrame);
        i += 1;
    }
    av_freep(&raw mut frames as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_hwframe_ctx_init(
    mut ref_0: *mut AVBufferRef,
) -> ::core::ffi::c_int {
    let mut ctxi: *mut FFHWFramesContext = (*ref_0).data as *mut FFHWFramesContext;
    let mut ctx: *mut AVHWFramesContext = &raw mut (*ctxi).p;
    let mut pix_fmt: *const AVPixelFormat = ::core::ptr::null::<AVPixelFormat>();
    let mut ret: ::core::ffi::c_int = 0;
    if !(*ctxi).source_frames.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    pix_fmt = (*(*ctxi).hw_type).pix_fmts;
    while *pix_fmt as ::core::ffi::c_int != AV_PIX_FMT_NONE as ::core::ffi::c_int {
        if *pix_fmt as ::core::ffi::c_int == (*ctx).format as ::core::ffi::c_int {
            break;
        }
        pix_fmt = pix_fmt.offset(1);
    }
    if *pix_fmt as ::core::ffi::c_int == AV_PIX_FMT_NONE as ::core::ffi::c_int {
        av_log(
            ctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"The hardware pixel format '%s' is not supported by the device type '%s'\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            av_get_pix_fmt_name((*ctx).format),
            (*(*ctxi).hw_type).name,
        );
        return -(38 as ::core::ffi::c_int);
    }
    ret = av_image_check_size(
        (*ctx).width as ::core::ffi::c_uint,
        (*ctx).height as ::core::ffi::c_uint,
        0 as ::core::ffi::c_int,
        ctx as *mut ::core::ffi::c_void,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*(*ctxi).hw_type).frames_init.is_some() {
        ret = (*(*ctxi).hw_type).frames_init.expect("non-null function pointer")(ctx);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    if !(*ctxi).pool_internal.is_null() && (*ctx).pool.is_null() {
        (*ctx).pool = (*ctxi).pool_internal;
    }
    if (*ctx).initial_pool_size > 0 as ::core::ffi::c_int {
        ret = hwframe_pool_prealloc(ref_0);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_hwframe_transfer_get_formats(
    mut hwframe_ref: *mut AVBufferRef,
    mut dir: AVHWFrameTransferDirection,
    mut formats: *mut *mut AVPixelFormat,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ctxi: *mut FFHWFramesContext = (*hwframe_ref).data as *mut FFHWFramesContext;
    if (*(*ctxi).hw_type).transfer_get_formats.is_none() {
        return -(38 as ::core::ffi::c_int);
    }
    return (*(*ctxi).hw_type)
        .transfer_get_formats
        .expect("non-null function pointer")(&raw mut (*ctxi).p, dir, formats);
}
unsafe extern "C" fn transfer_data_alloc(
    mut dst: *mut AVFrame,
    mut src: *const AVFrame,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ctx: *mut AVHWFramesContext = ::core::ptr::null_mut::<AVHWFramesContext>();
    let mut frame_tmp: *mut AVFrame = ::core::ptr::null_mut::<AVFrame>();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*src).hw_frames_ctx.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    ctx = (*(*src).hw_frames_ctx).data as *mut AVHWFramesContext;
    frame_tmp = av_frame_alloc();
    if frame_tmp.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    if (*dst).format >= 0 as ::core::ffi::c_int {
        (*frame_tmp).format = (*dst).format;
        current_block = 17965632435239708295;
    } else {
        let mut formats: *mut AVPixelFormat = ::core::ptr::null_mut::<AVPixelFormat>();
        ret = av_hwframe_transfer_get_formats(
            (*src).hw_frames_ctx,
            AV_HWFRAME_TRANSFER_DIRECTION_FROM,
            &raw mut formats,
            0 as ::core::ffi::c_int,
        );
        if ret < 0 as ::core::ffi::c_int {
            current_block = 12321389370689218654;
        } else {
            (*frame_tmp).format = *formats.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int;
            av_freep(&raw mut formats as *mut ::core::ffi::c_void);
            current_block = 17965632435239708295;
        }
    }
    match current_block {
        17965632435239708295 => {
            (*frame_tmp).width = (*ctx).width;
            (*frame_tmp).height = (*ctx).height;
            ret = av_frame_get_buffer(frame_tmp, 0 as ::core::ffi::c_int);
            if !(ret < 0 as ::core::ffi::c_int) {
                ret = av_hwframe_transfer_data(frame_tmp, src, flags);
                if !(ret < 0 as ::core::ffi::c_int) {
                    (*frame_tmp).width = (*src).width;
                    (*frame_tmp).height = (*src).height;
                    av_frame_move_ref(dst, frame_tmp);
                }
            }
        }
        _ => {}
    }
    av_frame_free(&raw mut frame_tmp);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_hwframe_transfer_data(
    mut dst: *mut AVFrame,
    mut src: *const AVFrame,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if (*dst).buf[0 as ::core::ffi::c_int as usize].is_null() {
        return transfer_data_alloc(dst, src, flags);
    }
    if !(*src).hw_frames_ctx.is_null() && !(*dst).hw_frames_ctx.is_null() {
        let mut src_ctx: *mut FFHWFramesContext = (*(*src).hw_frames_ctx).data
            as *mut FFHWFramesContext;
        let mut dst_ctx: *mut FFHWFramesContext = (*(*dst).hw_frames_ctx).data
            as *mut FFHWFramesContext;
        if !(*src_ctx).source_frames.is_null() {
            av_log(
                src_ctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"A device with a derived frame context cannot be used as the source of a HW -> HW transfer.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return -(38 as ::core::ffi::c_int);
        }
        if !(*dst_ctx).source_frames.is_null() {
            av_log(
                src_ctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"A device with a derived frame context cannot be used as the destination of a HW -> HW transfer.\0"
                    as *const u8 as *const ::core::ffi::c_char,
            );
            return -(38 as ::core::ffi::c_int);
        }
        ret = (*(*src_ctx).hw_type)
            .transfer_data_from
            .expect("non-null function pointer")(&raw mut (*src_ctx).p, dst, src);
        if ret == -(38 as ::core::ffi::c_int) {
            ret = (*(*dst_ctx).hw_type)
                .transfer_data_to
                .expect("non-null function pointer")(&raw mut (*dst_ctx).p, dst, src);
        }
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    } else if !(*src).hw_frames_ctx.is_null() {
        let mut ctx: *mut FFHWFramesContext = (*(*src).hw_frames_ctx).data
            as *mut FFHWFramesContext;
        ret = (*(*ctx).hw_type)
            .transfer_data_from
            .expect("non-null function pointer")(&raw mut (*ctx).p, dst, src);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    } else if !(*dst).hw_frames_ctx.is_null() {
        let mut ctx_0: *mut FFHWFramesContext = (*(*dst).hw_frames_ctx).data
            as *mut FFHWFramesContext;
        ret = (*(*ctx_0).hw_type)
            .transfer_data_to
            .expect("non-null function pointer")(&raw mut (*ctx_0).p, dst, src);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    } else {
        return -(38 as ::core::ffi::c_int)
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_hwframe_get_buffer(
    mut hwframe_ref: *mut AVBufferRef,
    mut frame: *mut AVFrame,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ctxi: *mut FFHWFramesContext = (*hwframe_ref).data as *mut FFHWFramesContext;
    let mut ctx: *mut AVHWFramesContext = &raw mut (*ctxi).p;
    let mut ret: ::core::ffi::c_int = 0;
    if !(*ctxi).source_frames.is_null() {
        let mut src_frame: *mut AVFrame = ::core::ptr::null_mut::<AVFrame>();
        (*frame).format = (*ctx).format as ::core::ffi::c_int;
        (*frame).hw_frames_ctx = av_buffer_ref(hwframe_ref);
        if (*frame).hw_frames_ctx.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
        src_frame = av_frame_alloc();
        if src_frame.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
        ret = av_hwframe_get_buffer(
            (*ctxi).source_frames,
            src_frame,
            0 as ::core::ffi::c_int,
        );
        if ret < 0 as ::core::ffi::c_int {
            av_frame_free(&raw mut src_frame);
            return ret;
        }
        ret = av_hwframe_map(frame, src_frame, (*ctxi).source_allocation_map_flags);
        if ret != 0 {
            av_log(
                ctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Failed to map frame into derived frame context: %d.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                ret,
            );
            av_frame_free(&raw mut src_frame);
            return ret;
        }
        av_frame_free(&raw mut src_frame);
        return 0 as ::core::ffi::c_int;
    }
    if (*(*ctxi).hw_type).frames_get_buffer.is_none() {
        return -(38 as ::core::ffi::c_int);
    }
    if (*ctx).pool.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    (*frame).hw_frames_ctx = av_buffer_ref(hwframe_ref);
    if (*frame).hw_frames_ctx.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    ret = (*(*ctxi).hw_type)
        .frames_get_buffer
        .expect("non-null function pointer")(ctx, frame);
    if ret < 0 as ::core::ffi::c_int {
        av_buffer_unref(&raw mut (*frame).hw_frames_ctx);
        return ret;
    }
    (*frame).extended_data = &raw mut (*frame).data as *mut *mut uint8_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_hwdevice_hwconfig_alloc(
    mut ref_0: *mut AVBufferRef,
) -> *mut ::core::ffi::c_void {
    let mut ctx: *mut FFHWDeviceContext = (*ref_0).data as *mut FFHWDeviceContext;
    let mut hw_type: *const HWContextType = (*ctx).hw_type;
    if (*hw_type).device_hwconfig_size == 0 as size_t {
        return ::core::ptr::null_mut::<::core::ffi::c_void>();
    }
    return av_mallocz((*hw_type).device_hwconfig_size);
}
#[no_mangle]
pub unsafe extern "C" fn av_hwdevice_get_hwframe_constraints(
    mut ref_0: *mut AVBufferRef,
    mut hwconfig: *const ::core::ffi::c_void,
) -> *mut AVHWFramesConstraints {
    let mut ctx: *mut FFHWDeviceContext = (*ref_0).data as *mut FFHWDeviceContext;
    let mut hw_type: *const HWContextType = (*ctx).hw_type;
    let mut constraints: *mut AVHWFramesConstraints = ::core::ptr::null_mut::<
        AVHWFramesConstraints,
    >();
    if (*hw_type).frames_get_constraints.is_none() {
        return ::core::ptr::null_mut::<AVHWFramesConstraints>();
    }
    constraints = av_mallocz(::core::mem::size_of::<AVHWFramesConstraints>() as size_t)
        as *mut AVHWFramesConstraints;
    if constraints.is_null() {
        return ::core::ptr::null_mut::<AVHWFramesConstraints>();
    }
    (*constraints).min_height = 0 as ::core::ffi::c_int;
    (*constraints).min_width = (*constraints).min_height;
    (*constraints).max_height = 2147483647 as ::core::ffi::c_int;
    (*constraints).max_width = (*constraints).max_height;
    if (*hw_type)
        .frames_get_constraints
        .expect("non-null function pointer")(&raw mut (*ctx).p, hwconfig, constraints)
        >= 0 as ::core::ffi::c_int
    {
        return constraints
    } else {
        av_hwframe_constraints_free(&raw mut constraints);
        return ::core::ptr::null_mut::<AVHWFramesConstraints>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_hwframe_constraints_free(
    mut constraints: *mut *mut AVHWFramesConstraints,
) {
    if !(*constraints).is_null() {
        av_freep(&raw mut (**constraints).valid_hw_formats as *mut ::core::ffi::c_void);
        av_freep(&raw mut (**constraints).valid_sw_formats as *mut ::core::ffi::c_void);
    }
    av_freep(constraints as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_hwdevice_ctx_create(
    mut pdevice_ref: *mut *mut AVBufferRef,
    mut type_0: AVHWDeviceType,
    mut device: *const ::core::ffi::c_char,
    mut opts: *mut AVDictionary,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut device_ref: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut device_ctx: *mut FFHWDeviceContext = ::core::ptr::null_mut::<
        FFHWDeviceContext,
    >();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    device_ref = av_hwdevice_ctx_alloc(type_0);
    if device_ref.is_null() {
        ret = -(12 as ::core::ffi::c_int);
    } else {
        device_ctx = (*device_ref).data as *mut FFHWDeviceContext;
        if (*(*device_ctx).hw_type).device_create.is_none() {
            ret = -(38 as ::core::ffi::c_int);
        } else {
            ret = (*(*device_ctx).hw_type)
                .device_create
                .expect(
                    "non-null function pointer",
                )(&raw mut (*device_ctx).p, device, opts, flags);
            if !(ret < 0 as ::core::ffi::c_int) {
                ret = av_hwdevice_ctx_init(device_ref);
                if !(ret < 0 as ::core::ffi::c_int) {
                    *pdevice_ref = device_ref;
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    av_buffer_unref(&raw mut device_ref);
    *pdevice_ref = ::core::ptr::null_mut::<AVBufferRef>();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_hwdevice_ctx_create_derived_opts(
    mut dst_ref_ptr: *mut *mut AVBufferRef,
    mut type_0: AVHWDeviceType,
    mut src_ref: *mut AVBufferRef,
    mut options: *mut AVDictionary,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut dst_ref: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut tmp_ref: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut dst_ctx: *mut FFHWDeviceContext = ::core::ptr::null_mut::<
        FFHWDeviceContext,
    >();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    tmp_ref = src_ref;
    loop {
        if tmp_ref.is_null() {
            current_block = 14523784380283086299;
            break;
        }
        let mut tmp_ctx: *mut FFHWDeviceContext = (*tmp_ref).data
            as *mut FFHWDeviceContext;
        if (*tmp_ctx).p.type_0 as ::core::ffi::c_uint == type_0 as ::core::ffi::c_uint {
            dst_ref = av_buffer_ref(tmp_ref);
            if !dst_ref.is_null() {
                current_block = 14693468631578731366;
                break;
            }
            ret = -(12 as ::core::ffi::c_int);
            current_block = 236649877317029051;
            break;
        } else {
            tmp_ref = (*tmp_ctx).source_device;
        }
    }
    match current_block {
        14523784380283086299 => {
            dst_ref = av_hwdevice_ctx_alloc(type_0);
            if dst_ref.is_null() {
                ret = -(12 as ::core::ffi::c_int);
                current_block = 236649877317029051;
            } else {
                dst_ctx = (*dst_ref).data as *mut FFHWDeviceContext;
                tmp_ref = src_ref;
                loop {
                    if tmp_ref.is_null() {
                        current_block = 5634871135123216486;
                        break;
                    }
                    let mut tmp_ctx_0: *mut FFHWDeviceContext = (*tmp_ref).data
                        as *mut FFHWDeviceContext;
                    if (*(*dst_ctx).hw_type).device_derive.is_some() {
                        ret = (*(*dst_ctx).hw_type)
                            .device_derive
                            .expect(
                                "non-null function pointer",
                            )(
                            &raw mut (*dst_ctx).p,
                            &raw mut (*tmp_ctx_0).p,
                            options,
                            flags,
                        );
                        if ret == 0 as ::core::ffi::c_int {
                            (*dst_ctx).source_device = av_buffer_ref(src_ref);
                            if (*dst_ctx).source_device.is_null() {
                                ret = -(12 as ::core::ffi::c_int);
                                current_block = 236649877317029051;
                                break;
                            } else {
                                ret = av_hwdevice_ctx_init(dst_ref);
                                if ret < 0 as ::core::ffi::c_int {
                                    current_block = 236649877317029051;
                                    break;
                                } else {
                                    current_block = 14693468631578731366;
                                    break;
                                }
                            }
                        } else if ret != -(38 as ::core::ffi::c_int) {
                            current_block = 236649877317029051;
                            break;
                        }
                    }
                    tmp_ref = (*tmp_ctx_0).source_device;
                }
                match current_block {
                    236649877317029051 => {}
                    14693468631578731366 => {}
                    _ => {
                        ret = -(38 as ::core::ffi::c_int);
                        current_block = 236649877317029051;
                    }
                }
            }
        }
        _ => {}
    }
    match current_block {
        236649877317029051 => {
            av_buffer_unref(&raw mut dst_ref);
            *dst_ref_ptr = ::core::ptr::null_mut::<AVBufferRef>();
            return ret;
        }
        _ => {
            *dst_ref_ptr = dst_ref;
            return 0 as ::core::ffi::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_hwdevice_ctx_create_derived(
    mut dst_ref_ptr: *mut *mut AVBufferRef,
    mut type_0: AVHWDeviceType,
    mut src_ref: *mut AVBufferRef,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return av_hwdevice_ctx_create_derived_opts(
        dst_ref_ptr,
        type_0,
        src_ref,
        ::core::ptr::null_mut::<AVDictionary>(),
        flags,
    );
}
unsafe extern "C" fn ff_hwframe_unmap(
    mut opaque: *mut ::core::ffi::c_void,
    mut data: *mut uint8_t,
) {
    let mut hwmap: *mut HWMapDescriptor = data as *mut HWMapDescriptor;
    let mut ctx: *mut AVHWFramesContext = opaque as *mut AVHWFramesContext;
    if (*hwmap).unmap.is_some() {
        (*hwmap)
            .unmap
            .expect("non-null function pointer")(ctx, hwmap as *mut HWMapDescriptor);
    }
    av_frame_free(&raw mut (*hwmap).source);
    av_buffer_unref(&raw mut (*hwmap).hw_frames_ctx);
    av_free(hwmap as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn ff_hwframe_map_create(
    mut hwframe_ref: *mut AVBufferRef,
    mut dst: *mut AVFrame,
    mut src: *const AVFrame,
    mut unmap: Option<
        unsafe extern "C" fn(*mut AVHWFramesContext, *mut HWMapDescriptor) -> (),
    >,
    mut priv_0: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut AVHWFramesContext = (*hwframe_ref).data as *mut AVHWFramesContext;
    let mut hwmap: *mut HWMapDescriptor = ::core::ptr::null_mut::<HWMapDescriptor>();
    let mut ret: ::core::ffi::c_int = 0;
    hwmap = av_mallocz(::core::mem::size_of::<HWMapDescriptor>() as size_t)
        as *mut HWMapDescriptor;
    if hwmap.is_null() {
        ret = -(12 as ::core::ffi::c_int);
    } else {
        (*hwmap).source = av_frame_alloc();
        if (*hwmap).source.is_null() {
            ret = -(12 as ::core::ffi::c_int);
        } else {
            ret = av_frame_ref((*hwmap).source, src);
            if !(ret < 0 as ::core::ffi::c_int) {
                (*hwmap).hw_frames_ctx = av_buffer_ref(hwframe_ref);
                if (*hwmap).hw_frames_ctx.is_null() {
                    ret = -(12 as ::core::ffi::c_int);
                } else {
                    (*hwmap).unmap = unmap
                        as Option<
                            unsafe extern "C" fn(
                                *mut AVHWFramesContext,
                                *mut HWMapDescriptor,
                            ) -> (),
                        >;
                    (*hwmap).priv_0 = priv_0;
                    (*dst).buf[0 as ::core::ffi::c_int as usize] = av_buffer_create(
                        hwmap as *mut uint8_t,
                        ::core::mem::size_of::<HWMapDescriptor>() as size_t,
                        Some(
                            ff_hwframe_unmap
                                as unsafe extern "C" fn(
                                    *mut ::core::ffi::c_void,
                                    *mut uint8_t,
                                ) -> (),
                        ),
                        ctx as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                    );
                    if (*dst).buf[0 as ::core::ffi::c_int as usize].is_null() {
                        ret = -(12 as ::core::ffi::c_int);
                    } else {
                        return 0 as ::core::ffi::c_int
                    }
                }
            }
        }
    }
    if !hwmap.is_null() {
        av_buffer_unref(&raw mut (*hwmap).hw_frames_ctx);
        av_frame_free(&raw mut (*hwmap).source);
    }
    av_free(hwmap as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_hwframe_map(
    mut dst: *mut AVFrame,
    mut src: *const AVFrame,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut orig_dst_frames: *mut AVBufferRef = (*dst).hw_frames_ctx;
    let mut orig_dst_fmt: AVPixelFormat = (*dst).format as AVPixelFormat;
    let mut hwmap: *mut HWMapDescriptor = ::core::ptr::null_mut::<HWMapDescriptor>();
    let mut ret: ::core::ffi::c_int = 0;
    if !(*src).hw_frames_ctx.is_null() && !(*dst).hw_frames_ctx.is_null() {
        let mut src_frames: *mut FFHWFramesContext = (*(*src).hw_frames_ctx).data
            as *mut FFHWFramesContext;
        let mut dst_frames: *mut FFHWFramesContext = (*(*dst).hw_frames_ctx).data
            as *mut FFHWFramesContext;
        if src_frames == dst_frames
            && (*src).format == (*dst_frames).p.sw_format as ::core::ffi::c_int
            && (*dst).format == (*dst_frames).p.format as ::core::ffi::c_int
            || !(*src_frames).source_frames.is_null()
                && (*(*src_frames).source_frames).data == dst_frames as *mut uint8_t
        {
            if (*src).buf[0 as ::core::ffi::c_int as usize].is_null() {
                av_log(
                    src_frames as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"Invalid mapping found when attempting unmap.\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(22 as ::core::ffi::c_int);
            }
            hwmap = (*(*src).buf[0 as ::core::ffi::c_int as usize]).data
                as *mut HWMapDescriptor;
            return av_frame_replace(dst, (*hwmap).source);
        }
    }
    if !(*src).hw_frames_ctx.is_null() {
        let mut src_frames_0: *mut FFHWFramesContext = (*(*src).hw_frames_ctx).data
            as *mut FFHWFramesContext;
        if (*src_frames_0).p.format as ::core::ffi::c_int == (*src).format
            && (*(*src_frames_0).hw_type).map_from.is_some()
        {
            ret = (*(*src_frames_0).hw_type)
                .map_from
                .expect(
                    "non-null function pointer",
                )(&raw mut (*src_frames_0).p, dst, src, flags);
            if ret >= 0 as ::core::ffi::c_int {
                return ret
            } else if ret != -(38 as ::core::ffi::c_int) {
                current_block = 8114046332099938093;
            } else {
                current_block = 9606288038608642794;
            }
        } else {
            current_block = 9606288038608642794;
        }
    } else {
        current_block = 9606288038608642794;
    }
    match current_block {
        9606288038608642794 => {
            if !(*dst).hw_frames_ctx.is_null() {
                let mut dst_frames_0: *mut FFHWFramesContext = (*(*dst).hw_frames_ctx)
                    .data as *mut FFHWFramesContext;
                if (*dst_frames_0).p.format as ::core::ffi::c_int == (*dst).format
                    && (*(*dst_frames_0).hw_type).map_to.is_some()
                {
                    ret = (*(*dst_frames_0).hw_type)
                        .map_to
                        .expect(
                            "non-null function pointer",
                        )(&raw mut (*dst_frames_0).p, dst, src, flags);
                    if ret >= 0 as ::core::ffi::c_int {
                        return ret
                    } else if ret != -(38 as ::core::ffi::c_int) {
                        current_block = 8114046332099938093;
                    } else {
                        current_block = 15089075282327824602;
                    }
                } else {
                    current_block = 15089075282327824602;
                }
            } else {
                current_block = 15089075282327824602;
            }
            match current_block {
                8114046332099938093 => {}
                _ => return -(38 as ::core::ffi::c_int),
            }
        }
        _ => {}
    }
    if !(orig_dst_frames.is_null() || orig_dst_frames == (*dst).hw_frames_ctx) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"orig_dst_frames == ((void*)0) || orig_dst_frames == dst->hw_frames_ctx\0"
                as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/hwcontext.c\0" as *const u8 as *const ::core::ffi::c_char,
            858 as ::core::ffi::c_int,
        );
        abort();
    }
    (*dst).hw_frames_ctx = ::core::ptr::null_mut::<AVBufferRef>();
    av_frame_unref(dst);
    (*dst).hw_frames_ctx = orig_dst_frames;
    (*dst).format = orig_dst_fmt as ::core::ffi::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_hwframe_ctx_create_derived(
    mut derived_frame_ctx: *mut *mut AVBufferRef,
    mut format: AVPixelFormat,
    mut derived_device_ctx: *mut AVBufferRef,
    mut source_frame_ctx: *mut AVBufferRef,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut dst_ref: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut dsti: *mut FFHWFramesContext = ::core::ptr::null_mut::<FFHWFramesContext>();
    let mut srci: *mut FFHWFramesContext = (*source_frame_ctx).data
        as *mut FFHWFramesContext;
    let mut dst: *mut AVHWFramesContext = ::core::ptr::null_mut::<AVHWFramesContext>();
    let mut src: *mut AVHWFramesContext = &raw mut (*srci).p;
    let mut ret: ::core::ffi::c_int = 0;
    if !(*srci).source_frames.is_null() {
        let mut src_src: *mut AVHWFramesContext = (*(*srci).source_frames).data
            as *mut AVHWFramesContext;
        let mut dst_dev: *mut AVHWDeviceContext = (*derived_device_ctx).data
            as *mut AVHWDeviceContext;
        if (*src_src).device_ctx == dst_dev {
            *derived_frame_ctx = av_buffer_ref((*srci).source_frames);
            if (*derived_frame_ctx).is_null() {
                ret = -(12 as ::core::ffi::c_int);
            } else {
                return 0 as ::core::ffi::c_int
            }
            current_block = 11938402873348412865;
        } else {
            current_block = 14523784380283086299;
        }
    } else {
        current_block = 14523784380283086299;
    }
    match current_block {
        14523784380283086299 => {
            dst_ref = av_hwframe_ctx_alloc(derived_device_ctx);
            if dst_ref.is_null() {
                ret = -(12 as ::core::ffi::c_int);
            } else {
                dsti = (*dst_ref).data as *mut FFHWFramesContext;
                dst = &raw mut (*dsti).p;
                (*dst).format = format;
                (*dst).sw_format = (*src).sw_format;
                (*dst).width = (*src).width;
                (*dst).height = (*src).height;
                (*dsti).source_frames = av_buffer_ref(source_frame_ctx);
                if (*dsti).source_frames.is_null() {
                    ret = -(12 as ::core::ffi::c_int);
                } else {
                    (*dsti).source_allocation_map_flags = flags
                        & (AV_HWFRAME_MAP_READ as ::core::ffi::c_int
                            | AV_HWFRAME_MAP_WRITE as ::core::ffi::c_int
                            | AV_HWFRAME_MAP_OVERWRITE as ::core::ffi::c_int
                            | AV_HWFRAME_MAP_DIRECT as ::core::ffi::c_int);
                    ret = -(38 as ::core::ffi::c_int);
                    if (*(*srci).hw_type).frames_derive_from.is_some() {
                        ret = (*(*srci).hw_type)
                            .frames_derive_from
                            .expect("non-null function pointer")(dst, src, flags);
                    }
                    if ret == -(38 as ::core::ffi::c_int)
                        && (*(*dsti).hw_type).frames_derive_to.is_some()
                    {
                        ret = (*(*dsti).hw_type)
                            .frames_derive_to
                            .expect("non-null function pointer")(dst, src, flags);
                    }
                    if ret == -(38 as ::core::ffi::c_int) {
                        ret = 0 as ::core::ffi::c_int;
                    }
                    if !(ret != 0) {
                        *derived_frame_ctx = dst_ref;
                        return 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
        _ => {}
    }
    if !dsti.is_null() {
        av_buffer_unref(&raw mut (*dsti).source_frames);
    }
    av_buffer_unref(&raw mut dst_ref);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ff_hwframe_map_replace(
    mut dst: *mut AVFrame,
    mut src: *const AVFrame,
) -> ::core::ffi::c_int {
    let mut hwmap: *mut HWMapDescriptor = (*(*dst).buf[0 as ::core::ffi::c_int as usize])
        .data as *mut HWMapDescriptor;
    return av_frame_replace((*hwmap).source, src);
}
#[repr(C)]
pub struct AVOptionRanges {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVOption {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVBuffer {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVBufferPool {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVDictionary {
    _opaque: [u8; 0],
}
