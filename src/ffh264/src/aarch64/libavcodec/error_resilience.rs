// Translated from FFmpeg 9.0.1 libavcodec/error_resilience.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Error resilience / concealment
//
// Copyright (c) 2002-2004 Michael Niedermayer <michaelni@gmx.at>
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
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static ff_crop_tab: [uint8_t; 2304];
    fn abort() -> !;
    fn ff_thread_await_progress(
        f: *const ThreadFrame,
        progress: ::core::ffi::c_int,
        field: ::core::ffi::c_int,
    );
    fn ff_me_cmp_init(c: *mut MECmpContext, avctx: *mut AVCodecContext);
    fn ff_thread_progress_await(
        pro: *const ThreadProgress,
        progress: ::core::ffi::c_int,
    );
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_malloc_array(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn av_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_get_picture_type_char(pict_type: AVPictureType) -> ::core::ffi::c_char;
    fn av_freep(ptr: *mut ::core::ffi::c_void);
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
pub type ptrdiff_t = ::core::ffi::c_long;
pub type int8_t = ::core::ffi::c_schar;
pub type int16_t = ::core::ffi::c_short;
pub type int64_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
pub type AVSampleFormat = ::core::ffi::c_int;
pub const AV_SAMPLE_FMT_NB: AVSampleFormat = 12;
pub const AV_SAMPLE_FMT_S64P: AVSampleFormat = 11;
pub const AV_SAMPLE_FMT_S64: AVSampleFormat = 10;
pub const AV_SAMPLE_FMT_DBLP: AVSampleFormat = 9;
pub const AV_SAMPLE_FMT_FLTP: AVSampleFormat = 8;
pub const AV_SAMPLE_FMT_S32P: AVSampleFormat = 7;
pub const AV_SAMPLE_FMT_S16P: AVSampleFormat = 6;
pub const AV_SAMPLE_FMT_U8P: AVSampleFormat = 5;
pub const AV_SAMPLE_FMT_DBL: AVSampleFormat = 4;
pub const AV_SAMPLE_FMT_FLT: AVSampleFormat = 3;
pub const AV_SAMPLE_FMT_S32: AVSampleFormat = 2;
pub const AV_SAMPLE_FMT_S16: AVSampleFormat = 1;
pub const AV_SAMPLE_FMT_U8: AVSampleFormat = 0;
pub const AV_SAMPLE_FMT_NONE: AVSampleFormat = -1;
pub type AVMediaType = ::core::ffi::c_int;
pub const AVMEDIA_TYPE_NB: AVMediaType = 5;
pub const AVMEDIA_TYPE_ATTACHMENT: AVMediaType = 4;
pub const AVMEDIA_TYPE_SUBTITLE: AVMediaType = 3;
pub const AVMEDIA_TYPE_DATA: AVMediaType = 2;
pub const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
pub const AVMEDIA_TYPE_VIDEO: AVMediaType = 0;
pub const AVMEDIA_TYPE_UNKNOWN: AVMediaType = -1;
pub type AVPictureType = ::core::ffi::c_uint;
pub const AV_PICTURE_TYPE_BI: AVPictureType = 7;
pub const AV_PICTURE_TYPE_SP: AVPictureType = 6;
pub const AV_PICTURE_TYPE_SI: AVPictureType = 5;
pub const AV_PICTURE_TYPE_S: AVPictureType = 4;
pub const AV_PICTURE_TYPE_B: AVPictureType = 3;
pub const AV_PICTURE_TYPE_P: AVPictureType = 2;
pub const AV_PICTURE_TYPE_I: AVPictureType = 1;
pub const AV_PICTURE_TYPE_NONE: AVPictureType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
}
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
pub type AVCodecID = ::core::ffi::c_uint;
pub const AV_CODEC_ID_ANULL: AVCodecID = 135171;
pub const AV_CODEC_ID_VNULL: AVCodecID = 135170;
pub const AV_CODEC_ID_WRAPPED_AVFRAME: AVCodecID = 135169;
pub const AV_CODEC_ID_FFMETADATA: AVCodecID = 135168;
pub const AV_CODEC_ID_MPEG4SYSTEMS: AVCodecID = 131073;
pub const AV_CODEC_ID_MPEG2TS: AVCodecID = 131072;
pub const AV_CODEC_ID_PROBE: AVCodecID = 102400;
pub const AV_CODEC_ID_SMPTE_436M_ANC: AVCodecID = 98317;
pub const AV_CODEC_ID_LCEVC: AVCodecID = 98316;
pub const AV_CODEC_ID_SMPTE_2038: AVCodecID = 98315;
pub const AV_CODEC_ID_BIN_DATA: AVCodecID = 98314;
pub const AV_CODEC_ID_TIMED_ID3: AVCodecID = 98313;
pub const AV_CODEC_ID_DVD_NAV: AVCodecID = 98312;
pub const AV_CODEC_ID_SMPTE_KLV: AVCodecID = 98311;
pub const AV_CODEC_ID_OTF: AVCodecID = 98310;
pub const AV_CODEC_ID_IDF: AVCodecID = 98309;
pub const AV_CODEC_ID_XBIN: AVCodecID = 98308;
pub const AV_CODEC_ID_BINTEXT: AVCodecID = 98307;
pub const AV_CODEC_ID_EPG: AVCodecID = 98306;
pub const AV_CODEC_ID_SCTE_35: AVCodecID = 98305;
pub const AV_CODEC_ID_TTF: AVCodecID = 98304;
pub const AV_CODEC_ID_FIRST_UNKNOWN: AVCodecID = 98304;
pub const AV_CODEC_ID_IVTV_VBI: AVCodecID = 94234;
pub const AV_CODEC_ID_ARIB_CAPTION: AVCodecID = 94233;
pub const AV_CODEC_ID_TTML: AVCodecID = 94232;
pub const AV_CODEC_ID_HDMV_TEXT_SUBTITLE: AVCodecID = 94231;
pub const AV_CODEC_ID_ASS: AVCodecID = 94230;
pub const AV_CODEC_ID_PJS: AVCodecID = 94229;
pub const AV_CODEC_ID_VPLAYER: AVCodecID = 94228;
pub const AV_CODEC_ID_MPL2: AVCodecID = 94227;
pub const AV_CODEC_ID_WEBVTT: AVCodecID = 94226;
pub const AV_CODEC_ID_SUBRIP: AVCodecID = 94225;
pub const AV_CODEC_ID_SUBVIEWER: AVCodecID = 94224;
pub const AV_CODEC_ID_SUBVIEWER1: AVCodecID = 94223;
pub const AV_CODEC_ID_STL: AVCodecID = 94222;
pub const AV_CODEC_ID_REALTEXT: AVCodecID = 94221;
pub const AV_CODEC_ID_SAMI: AVCodecID = 94220;
pub const AV_CODEC_ID_JACOSUB: AVCodecID = 94219;
pub const AV_CODEC_ID_EIA_608: AVCodecID = 94218;
pub const AV_CODEC_ID_MICRODVD: AVCodecID = 94217;
pub const AV_CODEC_ID_SRT: AVCodecID = 94216;
pub const AV_CODEC_ID_DVB_TELETEXT: AVCodecID = 94215;
pub const AV_CODEC_ID_HDMV_PGS_SUBTITLE: AVCodecID = 94214;
pub const AV_CODEC_ID_MOV_TEXT: AVCodecID = 94213;
pub const AV_CODEC_ID_SSA: AVCodecID = 94212;
pub const AV_CODEC_ID_XSUB: AVCodecID = 94211;
pub const AV_CODEC_ID_TEXT: AVCodecID = 94210;
pub const AV_CODEC_ID_DVB_SUBTITLE: AVCodecID = 94209;
pub const AV_CODEC_ID_DVD_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_FIRST_SUBTITLE: AVCodecID = 94208;
pub const AV_CODEC_ID_APPLE_APAC: AVCodecID = 86125;
pub const AV_CODEC_ID_AHX: AVCodecID = 86124;
pub const AV_CODEC_ID_G728: AVCodecID = 86123;
pub const AV_CODEC_ID_LC3: AVCodecID = 86122;
pub const AV_CODEC_ID_QOA: AVCodecID = 86121;
pub const AV_CODEC_ID_OSQ: AVCodecID = 86120;
pub const AV_CODEC_ID_AC4: AVCodecID = 86119;
pub const AV_CODEC_ID_RKA: AVCodecID = 86118;
pub const AV_CODEC_ID_WAVARC: AVCodecID = 86117;
pub const AV_CODEC_ID_FTR: AVCodecID = 86116;
pub const AV_CODEC_ID_APAC: AVCodecID = 86115;
pub const AV_CODEC_ID_MISC4: AVCodecID = 86114;
pub const AV_CODEC_ID_BONK: AVCodecID = 86113;
pub const AV_CODEC_ID_DFPWM: AVCodecID = 86112;
pub const AV_CODEC_ID_MSNSIREN: AVCodecID = 86111;
pub const AV_CODEC_ID_FASTAUDIO: AVCodecID = 86110;
pub const AV_CODEC_ID_HCA: AVCodecID = 86109;
pub const AV_CODEC_ID_SIREN: AVCodecID = 86108;
pub const AV_CODEC_ID_MPEGH_3D_AUDIO: AVCodecID = 86107;
pub const AV_CODEC_ID_ACELP_KELVIN: AVCodecID = 86106;
pub const AV_CODEC_ID_HCOM: AVCodecID = 86105;
pub const AV_CODEC_ID_ATRAC9: AVCodecID = 86104;
pub const AV_CODEC_ID_SBC: AVCodecID = 86103;
pub const AV_CODEC_ID_APTX_HD: AVCodecID = 86102;
pub const AV_CODEC_ID_APTX: AVCodecID = 86101;
pub const AV_CODEC_ID_DOLBY_E: AVCodecID = 86100;
pub const AV_CODEC_ID_ATRAC3PAL: AVCodecID = 86099;
pub const AV_CODEC_ID_ATRAC3AL: AVCodecID = 86098;
pub const AV_CODEC_ID_DST: AVCodecID = 86097;
pub const AV_CODEC_ID_XMA2: AVCodecID = 86096;
pub const AV_CODEC_ID_XMA1: AVCodecID = 86095;
pub const AV_CODEC_ID_INTERPLAY_ACM: AVCodecID = 86094;
pub const AV_CODEC_ID_4GV: AVCodecID = 86093;
pub const AV_CODEC_ID_DSD_MSBF_PLANAR: AVCodecID = 86092;
pub const AV_CODEC_ID_DSD_LSBF_PLANAR: AVCodecID = 86091;
pub const AV_CODEC_ID_DSD_MSBF: AVCodecID = 86090;
pub const AV_CODEC_ID_DSD_LSBF: AVCodecID = 86089;
pub const AV_CODEC_ID_SMV: AVCodecID = 86088;
pub const AV_CODEC_ID_EVRC: AVCodecID = 86087;
pub const AV_CODEC_ID_SONIC_LS: AVCodecID = 86086;
pub const AV_CODEC_ID_SONIC: AVCodecID = 86085;
pub const AV_CODEC_ID_FFWAVESYNTH: AVCodecID = 86084;
pub const AV_CODEC_ID_CODEC2: AVCodecID = 86083;
pub const AV_CODEC_ID_DSS_SP: AVCodecID = 86082;
pub const AV_CODEC_ID_ON2AVC: AVCodecID = 86081;
pub const AV_CODEC_ID_PAF_AUDIO: AVCodecID = 86080;
pub const AV_CODEC_ID_METASOUND: AVCodecID = 86079;
pub const AV_CODEC_ID_TAK: AVCodecID = 86078;
pub const AV_CODEC_ID_COMFORT_NOISE: AVCodecID = 86077;
pub const AV_CODEC_ID_OPUS: AVCodecID = 86076;
pub const AV_CODEC_ID_ILBC: AVCodecID = 86075;
pub const AV_CODEC_ID_IAC: AVCodecID = 86074;
pub const AV_CODEC_ID_RALF: AVCodecID = 86073;
pub const AV_CODEC_ID_BMV_AUDIO: AVCodecID = 86072;
pub const AV_CODEC_ID_8SVX_FIB: AVCodecID = 86071;
pub const AV_CODEC_ID_8SVX_EXP: AVCodecID = 86070;
pub const AV_CODEC_ID_G729: AVCodecID = 86069;
pub const AV_CODEC_ID_G723_1: AVCodecID = 86068;
pub const AV_CODEC_ID_CELT: AVCodecID = 86067;
pub const AV_CODEC_ID_QDMC: AVCodecID = 86066;
pub const AV_CODEC_ID_AAC_LATM: AVCodecID = 86065;
pub const AV_CODEC_ID_BINKAUDIO_DCT: AVCodecID = 86064;
pub const AV_CODEC_ID_BINKAUDIO_RDFT: AVCodecID = 86063;
pub const AV_CODEC_ID_ATRAC1: AVCodecID = 86062;
pub const AV_CODEC_ID_MP4ALS: AVCodecID = 86061;
pub const AV_CODEC_ID_TRUEHD: AVCodecID = 86060;
pub const AV_CODEC_ID_TWINVQ: AVCodecID = 86059;
pub const AV_CODEC_ID_MP1: AVCodecID = 86058;
pub const AV_CODEC_ID_SIPR: AVCodecID = 86057;
pub const AV_CODEC_ID_EAC3: AVCodecID = 86056;
pub const AV_CODEC_ID_ATRAC3P: AVCodecID = 86055;
pub const AV_CODEC_ID_WMALOSSLESS: AVCodecID = 86054;
pub const AV_CODEC_ID_WMAPRO: AVCodecID = 86053;
pub const AV_CODEC_ID_WMAVOICE: AVCodecID = 86052;
pub const AV_CODEC_ID_SPEEX: AVCodecID = 86051;
pub const AV_CODEC_ID_MUSEPACK8: AVCodecID = 86050;
pub const AV_CODEC_ID_NELLYMOSER: AVCodecID = 86049;
pub const AV_CODEC_ID_APE: AVCodecID = 86048;
pub const AV_CODEC_ID_ATRAC3: AVCodecID = 86047;
pub const AV_CODEC_ID_GSM_MS: AVCodecID = 86046;
pub const AV_CODEC_ID_MLP: AVCodecID = 86045;
pub const AV_CODEC_ID_MUSEPACK7: AVCodecID = 86044;
pub const AV_CODEC_ID_IMC: AVCodecID = 86043;
pub const AV_CODEC_ID_DSICINAUDIO: AVCodecID = 86042;
pub const AV_CODEC_ID_WAVPACK: AVCodecID = 86041;
pub const AV_CODEC_ID_QCELP: AVCodecID = 86040;
pub const AV_CODEC_ID_SMACKAUDIO: AVCodecID = 86039;
pub const AV_CODEC_ID_TTA: AVCodecID = 86038;
pub const AV_CODEC_ID_TRUESPEECH: AVCodecID = 86037;
pub const AV_CODEC_ID_COOK: AVCodecID = 86036;
pub const AV_CODEC_ID_QDM2: AVCodecID = 86035;
pub const AV_CODEC_ID_GSM: AVCodecID = 86034;
pub const AV_CODEC_ID_WESTWOOD_SND1: AVCodecID = 86033;
pub const AV_CODEC_ID_ALAC: AVCodecID = 86032;
pub const AV_CODEC_ID_SHORTEN: AVCodecID = 86031;
pub const AV_CODEC_ID_MP3ON4: AVCodecID = 86030;
pub const AV_CODEC_ID_MP3ADU: AVCodecID = 86029;
pub const AV_CODEC_ID_FLAC: AVCodecID = 86028;
pub const AV_CODEC_ID_VMDAUDIO: AVCodecID = 86027;
pub const AV_CODEC_ID_MACE6: AVCodecID = 86026;
pub const AV_CODEC_ID_MACE3: AVCodecID = 86025;
pub const AV_CODEC_ID_WMAV2: AVCodecID = 86024;
pub const AV_CODEC_ID_WMAV1: AVCodecID = 86023;
pub const AV_CODEC_ID_DVAUDIO: AVCodecID = 86022;
pub const AV_CODEC_ID_VORBIS: AVCodecID = 86021;
pub const AV_CODEC_ID_DTS: AVCodecID = 86020;
pub const AV_CODEC_ID_AC3: AVCodecID = 86019;
pub const AV_CODEC_ID_AAC: AVCodecID = 86018;
pub const AV_CODEC_ID_MP3: AVCodecID = 86017;
pub const AV_CODEC_ID_MP2: AVCodecID = 86016;
pub const AV_CODEC_ID_CBD2_DPCM: AVCodecID = 81928;
pub const AV_CODEC_ID_WADY_DPCM: AVCodecID = 81927;
pub const AV_CODEC_ID_DERF_DPCM: AVCodecID = 81926;
pub const AV_CODEC_ID_GREMLIN_DPCM: AVCodecID = 81925;
pub const AV_CODEC_ID_SDX2_DPCM: AVCodecID = 81924;
pub const AV_CODEC_ID_SOL_DPCM: AVCodecID = 81923;
pub const AV_CODEC_ID_XAN_DPCM: AVCodecID = 81922;
pub const AV_CODEC_ID_INTERPLAY_DPCM: AVCodecID = 81921;
pub const AV_CODEC_ID_ROQ_DPCM: AVCodecID = 81920;
pub const AV_CODEC_ID_RA_288: AVCodecID = 77825;
pub const AV_CODEC_ID_RA_144: AVCodecID = 77824;
pub const AV_CODEC_ID_AMR_WB: AVCodecID = 73729;
pub const AV_CODEC_ID_AMR_NB: AVCodecID = 73728;
pub const AV_CODEC_ID_ADPCM_IMA_ESCAPE: AVCodecID = 69693;
pub const AV_CODEC_ID_ADPCM_CIRCUS: AVCodecID = 69692;
pub const AV_CODEC_ID_ADPCM_PSXC: AVCodecID = 69691;
pub const AV_CODEC_ID_ADPCM_IMA_MAGIX: AVCodecID = 69690;
pub const AV_CODEC_ID_ADPCM_IMA_HVQM2: AVCodecID = 69689;
pub const AV_CODEC_ID_ADPCM_N64: AVCodecID = 69688;
pub const AV_CODEC_ID_ADPCM_IMA_PDA: AVCodecID = 69687;
pub const AV_CODEC_ID_ADPCM_IMA_HVQM4: AVCodecID = 69686;
pub const AV_CODEC_ID_ADPCM_SANYO: AVCodecID = 69685;
pub const AV_CODEC_ID_ADPCM_IMA_XBOX: AVCodecID = 69684;
pub const AV_CODEC_ID_ADPCM_XMD: AVCodecID = 69683;
pub const AV_CODEC_ID_ADPCM_IMA_ACORN: AVCodecID = 69682;
pub const AV_CODEC_ID_ADPCM_IMA_MOFLEX: AVCodecID = 69681;
pub const AV_CODEC_ID_ADPCM_IMA_CUNNING: AVCodecID = 69680;
pub const AV_CODEC_ID_ADPCM_IMA_MTF: AVCodecID = 69679;
pub const AV_CODEC_ID_ADPCM_IMA_ALP: AVCodecID = 69678;
pub const AV_CODEC_ID_ADPCM_IMA_APM: AVCodecID = 69677;
pub const AV_CODEC_ID_ADPCM_ZORK: AVCodecID = 69676;
pub const AV_CODEC_ID_ADPCM_IMA_SSI: AVCodecID = 69675;
pub const AV_CODEC_ID_ADPCM_ARGO: AVCodecID = 69674;
pub const AV_CODEC_ID_ADPCM_AGM: AVCodecID = 69673;
pub const AV_CODEC_ID_ADPCM_MTAF: AVCodecID = 69672;
pub const AV_CODEC_ID_ADPCM_IMA_DAT4: AVCodecID = 69671;
pub const AV_CODEC_ID_ADPCM_AICA: AVCodecID = 69670;
pub const AV_CODEC_ID_ADPCM_PSX: AVCodecID = 69669;
pub const AV_CODEC_ID_ADPCM_THP_LE: AVCodecID = 69668;
pub const AV_CODEC_ID_ADPCM_G726LE: AVCodecID = 69667;
pub const AV_CODEC_ID_ADPCM_IMA_RAD: AVCodecID = 69666;
pub const AV_CODEC_ID_ADPCM_DTK: AVCodecID = 69665;
pub const AV_CODEC_ID_ADPCM_IMA_OKI: AVCodecID = 69664;
pub const AV_CODEC_ID_ADPCM_AFC: AVCodecID = 69663;
pub const AV_CODEC_ID_ADPCM_VIMA: AVCodecID = 69662;
pub const AV_CODEC_ID_ADPCM_IMA_APC: AVCodecID = 69661;
pub const AV_CODEC_ID_ADPCM_G722: AVCodecID = 69660;
pub const AV_CODEC_ID_ADPCM_IMA_ISS: AVCodecID = 69659;
pub const AV_CODEC_ID_ADPCM_EA_MAXIS_XA: AVCodecID = 69658;
pub const AV_CODEC_ID_ADPCM_EA_XAS: AVCodecID = 69657;
pub const AV_CODEC_ID_ADPCM_IMA_EA_EACS: AVCodecID = 69656;
pub const AV_CODEC_ID_ADPCM_IMA_EA_SEAD: AVCodecID = 69655;
pub const AV_CODEC_ID_ADPCM_EA_R2: AVCodecID = 69654;
pub const AV_CODEC_ID_ADPCM_EA_R3: AVCodecID = 69653;
pub const AV_CODEC_ID_ADPCM_EA_R1: AVCodecID = 69652;
pub const AV_CODEC_ID_ADPCM_IMA_AMV: AVCodecID = 69651;
pub const AV_CODEC_ID_ADPCM_THP: AVCodecID = 69650;
pub const AV_CODEC_ID_ADPCM_SBPRO_2: AVCodecID = 69649;
pub const AV_CODEC_ID_ADPCM_SBPRO_3: AVCodecID = 69648;
pub const AV_CODEC_ID_ADPCM_SBPRO_4: AVCodecID = 69647;
pub const AV_CODEC_ID_ADPCM_YAMAHA: AVCodecID = 69646;
pub const AV_CODEC_ID_ADPCM_SWF: AVCodecID = 69645;
pub const AV_CODEC_ID_ADPCM_CT: AVCodecID = 69644;
pub const AV_CODEC_ID_ADPCM_G726: AVCodecID = 69643;
pub const AV_CODEC_ID_ADPCM_EA: AVCodecID = 69642;
pub const AV_CODEC_ID_ADPCM_ADX: AVCodecID = 69641;
pub const AV_CODEC_ID_ADPCM_XA: AVCodecID = 69640;
pub const AV_CODEC_ID_ADPCM_4XM: AVCodecID = 69639;
pub const AV_CODEC_ID_ADPCM_MS: AVCodecID = 69638;
pub const AV_CODEC_ID_ADPCM_IMA_SMJPEG: AVCodecID = 69637;
pub const AV_CODEC_ID_ADPCM_IMA_WS: AVCodecID = 69636;
pub const AV_CODEC_ID_ADPCM_IMA_DK4: AVCodecID = 69635;
pub const AV_CODEC_ID_ADPCM_IMA_DK3: AVCodecID = 69634;
pub const AV_CODEC_ID_ADPCM_IMA_WAV: AVCodecID = 69633;
pub const AV_CODEC_ID_ADPCM_IMA_QT: AVCodecID = 69632;
pub const AV_CODEC_ID_PCM_SGA: AVCodecID = 65572;
pub const AV_CODEC_ID_PCM_VIDC: AVCodecID = 65571;
pub const AV_CODEC_ID_PCM_F24LE: AVCodecID = 65570;
pub const AV_CODEC_ID_PCM_F16LE: AVCodecID = 65569;
pub const AV_CODEC_ID_PCM_S64BE: AVCodecID = 65568;
pub const AV_CODEC_ID_PCM_S64LE: AVCodecID = 65567;
pub const AV_CODEC_ID_PCM_S16BE_PLANAR: AVCodecID = 65566;
pub const AV_CODEC_ID_PCM_S32LE_PLANAR: AVCodecID = 65565;
pub const AV_CODEC_ID_PCM_S24LE_PLANAR: AVCodecID = 65564;
pub const AV_CODEC_ID_PCM_S8_PLANAR: AVCodecID = 65563;
pub const AV_CODEC_ID_S302M: AVCodecID = 65562;
pub const AV_CODEC_ID_PCM_LXF: AVCodecID = 65561;
pub const AV_CODEC_ID_PCM_BLURAY: AVCodecID = 65560;
pub const AV_CODEC_ID_PCM_F64LE: AVCodecID = 65559;
pub const AV_CODEC_ID_PCM_F64BE: AVCodecID = 65558;
pub const AV_CODEC_ID_PCM_F32LE: AVCodecID = 65557;
pub const AV_CODEC_ID_PCM_F32BE: AVCodecID = 65556;
pub const AV_CODEC_ID_PCM_DVD: AVCodecID = 65555;
pub const AV_CODEC_ID_PCM_S16LE_PLANAR: AVCodecID = 65554;
pub const AV_CODEC_ID_PCM_ZORK: AVCodecID = 65553;
pub const AV_CODEC_ID_PCM_S24DAUD: AVCodecID = 65552;
pub const AV_CODEC_ID_PCM_U24BE: AVCodecID = 65551;
pub const AV_CODEC_ID_PCM_U24LE: AVCodecID = 65550;
pub const AV_CODEC_ID_PCM_S24BE: AVCodecID = 65549;
pub const AV_CODEC_ID_PCM_S24LE: AVCodecID = 65548;
pub const AV_CODEC_ID_PCM_U32BE: AVCodecID = 65547;
pub const AV_CODEC_ID_PCM_U32LE: AVCodecID = 65546;
pub const AV_CODEC_ID_PCM_S32BE: AVCodecID = 65545;
pub const AV_CODEC_ID_PCM_S32LE: AVCodecID = 65544;
pub const AV_CODEC_ID_PCM_ALAW: AVCodecID = 65543;
pub const AV_CODEC_ID_PCM_MULAW: AVCodecID = 65542;
pub const AV_CODEC_ID_PCM_U8: AVCodecID = 65541;
pub const AV_CODEC_ID_PCM_S8: AVCodecID = 65540;
pub const AV_CODEC_ID_PCM_U16BE: AVCodecID = 65539;
pub const AV_CODEC_ID_PCM_U16LE: AVCodecID = 65538;
pub const AV_CODEC_ID_PCM_S16BE: AVCodecID = 65537;
pub const AV_CODEC_ID_PCM_S16LE: AVCodecID = 65536;
pub const AV_CODEC_ID_FIRST_AUDIO: AVCodecID = 65536;
pub const AV_CODEC_ID_WEBP_ANIM: AVCodecID = 273;
pub const AV_CODEC_ID_JPEGXS: AVCodecID = 272;
pub const AV_CODEC_ID_PRORES_RAW: AVCodecID = 271;
pub const AV_CODEC_ID_APV: AVCodecID = 270;
pub const AV_CODEC_ID_JPEGXL_ANIM: AVCodecID = 269;
pub const AV_CODEC_ID_RV60: AVCodecID = 268;
pub const AV_CODEC_ID_DNXUC: AVCodecID = 267;
pub const AV_CODEC_ID_LEAD: AVCodecID = 266;
pub const AV_CODEC_ID_VMIX: AVCodecID = 265;
pub const AV_CODEC_ID_RTV1: AVCodecID = 264;
pub const AV_CODEC_ID_EVC: AVCodecID = 263;
pub const AV_CODEC_ID_PDV: AVCodecID = 262;
pub const AV_CODEC_ID_VQC: AVCodecID = 261;
pub const AV_CODEC_ID_MEDIA100: AVCodecID = 260;
pub const AV_CODEC_ID_WBMP: AVCodecID = 259;
pub const AV_CODEC_ID_RADIANCE_HDR: AVCodecID = 258;
pub const AV_CODEC_ID_PHM: AVCodecID = 257;
pub const AV_CODEC_ID_QOI: AVCodecID = 256;
pub const AV_CODEC_ID_JPEGXL: AVCodecID = 255;
pub const AV_CODEC_ID_VBN: AVCodecID = 254;
pub const AV_CODEC_ID_GEM: AVCodecID = 253;
pub const AV_CODEC_ID_SGA_VIDEO: AVCodecID = 252;
pub const AV_CODEC_ID_SIMBIOSIS_IMX: AVCodecID = 251;
pub const AV_CODEC_ID_CRI: AVCodecID = 250;
pub const AV_CODEC_ID_ARGO: AVCodecID = 249;
pub const AV_CODEC_ID_IPU: AVCodecID = 248;
pub const AV_CODEC_ID_PHOTOCD: AVCodecID = 247;
pub const AV_CODEC_ID_MOBICLIP: AVCodecID = 246;
pub const AV_CODEC_ID_PFM: AVCodecID = 245;
pub const AV_CODEC_ID_NOTCHLC: AVCodecID = 244;
pub const AV_CODEC_ID_MV30: AVCodecID = 243;
pub const AV_CODEC_ID_CDTOONS: AVCodecID = 242;
pub const AV_CODEC_ID_MVHA: AVCodecID = 241;
pub const AV_CODEC_ID_MVDV: AVCodecID = 240;
pub const AV_CODEC_ID_IMM5: AVCodecID = 239;
pub const AV_CODEC_ID_VP4: AVCodecID = 238;
pub const AV_CODEC_ID_LSCR: AVCodecID = 237;
pub const AV_CODEC_ID_AGM: AVCodecID = 236;
pub const AV_CODEC_ID_ARBC: AVCodecID = 235;
pub const AV_CODEC_ID_HYMT: AVCodecID = 234;
pub const AV_CODEC_ID_RASC: AVCodecID = 233;
pub const AV_CODEC_ID_WCMV: AVCodecID = 232;
pub const AV_CODEC_ID_MWSC: AVCodecID = 231;
pub const AV_CODEC_ID_PROSUMER: AVCodecID = 230;
pub const AV_CODEC_ID_IMM4: AVCodecID = 229;
pub const AV_CODEC_ID_FITS: AVCodecID = 228;
pub const AV_CODEC_ID_GDV: AVCodecID = 227;
pub const AV_CODEC_ID_SVG: AVCodecID = 226;
pub const AV_CODEC_ID_SRGC: AVCodecID = 225;
pub const AV_CODEC_ID_MSCC: AVCodecID = 224;
pub const AV_CODEC_ID_BITPACKED: AVCodecID = 223;
pub const AV_CODEC_ID_AV1: AVCodecID = 222;
pub const AV_CODEC_ID_XPM: AVCodecID = 221;
pub const AV_CODEC_ID_CLEARVIDEO: AVCodecID = 220;
pub const AV_CODEC_ID_SCPR: AVCodecID = 219;
pub const AV_CODEC_ID_FMVC: AVCodecID = 218;
pub const AV_CODEC_ID_SPEEDHQ: AVCodecID = 217;
pub const AV_CODEC_ID_PIXLET: AVCodecID = 216;
pub const AV_CODEC_ID_PSD: AVCodecID = 215;
pub const AV_CODEC_ID_YLC: AVCodecID = 214;
pub const AV_CODEC_ID_SHEERVIDEO: AVCodecID = 213;
pub const AV_CODEC_ID_MAGICYUV: AVCodecID = 212;
pub const AV_CODEC_ID_M101: AVCodecID = 211;
pub const AV_CODEC_ID_TRUEMOTION2RT: AVCodecID = 210;
pub const AV_CODEC_ID_CFHD: AVCodecID = 209;
pub const AV_CODEC_ID_DAALA: AVCodecID = 208;
pub const AV_CODEC_ID_APNG: AVCodecID = 207;
pub const AV_CODEC_ID_SMVJPEG: AVCodecID = 206;
pub const AV_CODEC_ID_SNOW: AVCodecID = 205;
pub const AV_CODEC_ID_XFACE: AVCodecID = 204;
pub const AV_CODEC_ID_CPIA: AVCodecID = 203;
pub const AV_CODEC_ID_AVRN: AVCodecID = 202;
pub const AV_CODEC_ID_YUV4: AVCodecID = 201;
pub const AV_CODEC_ID_TARGA_Y216: AVCodecID = 200;
pub const AV_CODEC_ID_AVUI: AVCodecID = 199;
pub const AV_CODEC_ID_012V: AVCodecID = 198;
pub const AV_CODEC_ID_AVRP: AVCodecID = 197;
pub const AV_CODEC_ID_Y41P: AVCodecID = 196;
pub const AV_CODEC_ID_VVC: AVCodecID = 195;
pub const AV_CODEC_ID_MSP2: AVCodecID = 194;
pub const AV_CODEC_ID_AVS3: AVCodecID = 193;
pub const AV_CODEC_ID_PGX: AVCodecID = 192;
pub const AV_CODEC_ID_AVS2: AVCodecID = 191;
pub const AV_CODEC_ID_RSCC: AVCodecID = 190;
pub const AV_CODEC_ID_SCREENPRESSO: AVCodecID = 189;
pub const AV_CODEC_ID_DXV: AVCodecID = 188;
pub const AV_CODEC_ID_DDS: AVCodecID = 187;
pub const AV_CODEC_ID_HAP: AVCodecID = 186;
pub const AV_CODEC_ID_HQ_HQA: AVCodecID = 185;
pub const AV_CODEC_ID_TDSC: AVCodecID = 184;
pub const AV_CODEC_ID_HQX: AVCodecID = 183;
pub const AV_CODEC_ID_MVC2: AVCodecID = 182;
pub const AV_CODEC_ID_MVC1: AVCodecID = 181;
pub const AV_CODEC_ID_SGIRLE: AVCodecID = 180;
pub const AV_CODEC_ID_SANM: AVCodecID = 179;
pub const AV_CODEC_ID_VP7: AVCodecID = 178;
pub const AV_CODEC_ID_EXR: AVCodecID = 177;
pub const AV_CODEC_ID_PAF_VIDEO: AVCodecID = 176;
pub const AV_CODEC_ID_BRENDER_PIX: AVCodecID = 175;
pub const AV_CODEC_ID_ALIAS_PIX: AVCodecID = 174;
pub const AV_CODEC_ID_FIC: AVCodecID = 173;
pub const AV_CODEC_ID_HEVC: AVCodecID = 172;
pub const AV_CODEC_ID_HNM4_VIDEO: AVCodecID = 171;
pub const AV_CODEC_ID_WEBP: AVCodecID = 170;
pub const AV_CODEC_ID_G2M: AVCodecID = 169;
pub const AV_CODEC_ID_ESCAPE130: AVCodecID = 168;
pub const AV_CODEC_ID_AIC: AVCodecID = 167;
pub const AV_CODEC_ID_VP9: AVCodecID = 166;
pub const AV_CODEC_ID_MSS2: AVCodecID = 165;
pub const AV_CODEC_ID_CLLC: AVCodecID = 164;
pub const AV_CODEC_ID_MTS2: AVCodecID = 163;
pub const AV_CODEC_ID_TSCC2: AVCodecID = 162;
pub const AV_CODEC_ID_MSA1: AVCodecID = 161;
pub const AV_CODEC_ID_MSS1: AVCodecID = 160;
pub const AV_CODEC_ID_ZEROCODEC: AVCodecID = 159;
pub const AV_CODEC_ID_XBM: AVCodecID = 158;
pub const AV_CODEC_ID_CDXL: AVCodecID = 157;
pub const AV_CODEC_ID_XWD: AVCodecID = 156;
pub const AV_CODEC_ID_DXTORY: AVCodecID = 155;
pub const AV_CODEC_ID_VBLE: AVCodecID = 154;
pub const AV_CODEC_ID_BMV_VIDEO: AVCodecID = 153;
pub const AV_CODEC_ID_UTVIDEO: AVCodecID = 152;
pub const AV_CODEC_ID_VC1IMAGE: AVCodecID = 151;
pub const AV_CODEC_ID_WMV3IMAGE: AVCodecID = 150;
pub const AV_CODEC_ID_DFA: AVCodecID = 149;
pub const AV_CODEC_ID_JV: AVCodecID = 148;
pub const AV_CODEC_ID_PRORES: AVCodecID = 147;
pub const AV_CODEC_ID_LAGARITH: AVCodecID = 146;
pub const AV_CODEC_ID_MXPEG: AVCodecID = 145;
pub const AV_CODEC_ID_R10K: AVCodecID = 144;
pub const AV_CODEC_ID_A64_MULTI5: AVCodecID = 143;
pub const AV_CODEC_ID_A64_MULTI: AVCodecID = 142;
pub const AV_CODEC_ID_ANSI: AVCodecID = 141;
pub const AV_CODEC_ID_PICTOR: AVCodecID = 140;
pub const AV_CODEC_ID_VP8: AVCodecID = 139;
pub const AV_CODEC_ID_YOP: AVCodecID = 138;
pub const AV_CODEC_ID_KGV1: AVCodecID = 137;
pub const AV_CODEC_ID_IFF_ILBM: AVCodecID = 136;
pub const AV_CODEC_ID_BINKVIDEO: AVCodecID = 135;
pub const AV_CODEC_ID_ANM: AVCodecID = 134;
pub const AV_CODEC_ID_R210: AVCodecID = 133;
pub const AV_CODEC_ID_CDGRAPHICS: AVCodecID = 132;
pub const AV_CODEC_ID_FLASHSV2: AVCodecID = 131;
pub const AV_CODEC_ID_FRWU: AVCodecID = 130;
pub const AV_CODEC_ID_MAD: AVCodecID = 129;
pub const AV_CODEC_ID_DPX: AVCodecID = 128;
pub const AV_CODEC_ID_V210: AVCodecID = 127;
pub const AV_CODEC_ID_TMV: AVCodecID = 126;
pub const AV_CODEC_ID_V210X: AVCodecID = 125;
pub const AV_CODEC_ID_AURA2: AVCodecID = 124;
pub const AV_CODEC_ID_AURA: AVCodecID = 123;
pub const AV_CODEC_ID_TQI: AVCodecID = 122;
pub const AV_CODEC_ID_TGQ: AVCodecID = 121;
pub const AV_CODEC_ID_TGV: AVCodecID = 120;
pub const AV_CODEC_ID_MOTIONPIXELS: AVCodecID = 119;
pub const AV_CODEC_ID_CMV: AVCodecID = 118;
pub const AV_CODEC_ID_BFI: AVCodecID = 117;
pub const AV_CODEC_ID_DIRAC: AVCodecID = 116;
pub const AV_CODEC_ID_ESCAPE124: AVCodecID = 115;
pub const AV_CODEC_ID_RL2: AVCodecID = 114;
pub const AV_CODEC_ID_MIMIC: AVCodecID = 113;
pub const AV_CODEC_ID_INDEO5: AVCodecID = 112;
pub const AV_CODEC_ID_INDEO4: AVCodecID = 111;
pub const AV_CODEC_ID_SUNRAST: AVCodecID = 110;
pub const AV_CODEC_ID_PCX: AVCodecID = 109;
pub const AV_CODEC_ID_VB: AVCodecID = 108;
pub const AV_CODEC_ID_AMV: AVCodecID = 107;
pub const AV_CODEC_ID_VP6A: AVCodecID = 106;
pub const AV_CODEC_ID_TXD: AVCodecID = 105;
pub const AV_CODEC_ID_PTX: AVCodecID = 104;
pub const AV_CODEC_ID_BETHSOFTVID: AVCodecID = 103;
pub const AV_CODEC_ID_C93: AVCodecID = 102;
pub const AV_CODEC_ID_SGI: AVCodecID = 101;
pub const AV_CODEC_ID_THP: AVCodecID = 100;
pub const AV_CODEC_ID_DNXHD: AVCodecID = 99;
pub const AV_CODEC_ID_DXA: AVCodecID = 98;
pub const AV_CODEC_ID_GIF: AVCodecID = 97;
pub const AV_CODEC_ID_TIFF: AVCodecID = 96;
pub const AV_CODEC_ID_TIERTEXSEQVIDEO: AVCodecID = 95;
pub const AV_CODEC_ID_DSICINVIDEO: AVCodecID = 94;
pub const AV_CODEC_ID_TARGA: AVCodecID = 93;
pub const AV_CODEC_ID_VP6F: AVCodecID = 92;
pub const AV_CODEC_ID_VP6: AVCodecID = 91;
pub const AV_CODEC_ID_VP5: AVCodecID = 90;
pub const AV_CODEC_ID_VMNC: AVCodecID = 89;
pub const AV_CODEC_ID_JPEG2000: AVCodecID = 88;
pub const AV_CODEC_ID_CAVS: AVCodecID = 87;
pub const AV_CODEC_ID_FLASHSV: AVCodecID = 86;
pub const AV_CODEC_ID_KMVC: AVCodecID = 85;
pub const AV_CODEC_ID_NUV: AVCodecID = 84;
pub const AV_CODEC_ID_SMACKVIDEO: AVCodecID = 83;
pub const AV_CODEC_ID_AVS: AVCodecID = 82;
pub const AV_CODEC_ID_ZMBV: AVCodecID = 81;
pub const AV_CODEC_ID_MMVIDEO: AVCodecID = 80;
pub const AV_CODEC_ID_CSCD: AVCodecID = 79;
pub const AV_CODEC_ID_BMP: AVCodecID = 78;
pub const AV_CODEC_ID_TRUEMOTION2: AVCodecID = 77;
pub const AV_CODEC_ID_FRAPS: AVCodecID = 76;
pub const AV_CODEC_ID_INDEO2: AVCodecID = 75;
pub const AV_CODEC_ID_AASC: AVCodecID = 74;
pub const AV_CODEC_ID_WNV1: AVCodecID = 73;
pub const AV_CODEC_ID_LOCO: AVCodecID = 72;
pub const AV_CODEC_ID_WMV3: AVCodecID = 71;
pub const AV_CODEC_ID_VC1: AVCodecID = 70;
pub const AV_CODEC_ID_RV40: AVCodecID = 69;
pub const AV_CODEC_ID_RV30: AVCodecID = 68;
pub const AV_CODEC_ID_FFVHUFF: AVCodecID = 67;
pub const AV_CODEC_ID_PAM: AVCodecID = 66;
pub const AV_CODEC_ID_PGMYUV: AVCodecID = 65;
pub const AV_CODEC_ID_PGM: AVCodecID = 64;
pub const AV_CODEC_ID_PBM: AVCodecID = 63;
pub const AV_CODEC_ID_PPM: AVCodecID = 62;
pub const AV_CODEC_ID_PNG: AVCodecID = 61;
pub const AV_CODEC_ID_QPEG: AVCodecID = 60;
pub const AV_CODEC_ID_VIXL: AVCodecID = 59;
pub const AV_CODEC_ID_QDRAW: AVCodecID = 58;
pub const AV_CODEC_ID_ULTI: AVCodecID = 57;
pub const AV_CODEC_ID_TSCC: AVCodecID = 56;
pub const AV_CODEC_ID_QTRLE: AVCodecID = 55;
pub const AV_CODEC_ID_ZLIB: AVCodecID = 54;
pub const AV_CODEC_ID_MSZH: AVCodecID = 53;
pub const AV_CODEC_ID_VMDVIDEO: AVCodecID = 52;
pub const AV_CODEC_ID_TRUEMOTION1: AVCodecID = 51;
pub const AV_CODEC_ID_FLIC: AVCodecID = 50;
pub const AV_CODEC_ID_SMC: AVCodecID = 49;
pub const AV_CODEC_ID_8BPS: AVCodecID = 48;
pub const AV_CODEC_ID_IDCIN: AVCodecID = 47;
pub const AV_CODEC_ID_MSVIDEO1: AVCodecID = 46;
pub const AV_CODEC_ID_MSRLE: AVCodecID = 45;
pub const AV_CODEC_ID_WS_VQA: AVCodecID = 44;
pub const AV_CODEC_ID_CINEPAK: AVCodecID = 43;
pub const AV_CODEC_ID_RPZA: AVCodecID = 42;
pub const AV_CODEC_ID_XAN_WC4: AVCodecID = 41;
pub const AV_CODEC_ID_XAN_WC3: AVCodecID = 40;
pub const AV_CODEC_ID_INTERPLAY_VIDEO: AVCodecID = 39;
pub const AV_CODEC_ID_ROQ: AVCodecID = 38;
pub const AV_CODEC_ID_MDEC: AVCodecID = 37;
pub const AV_CODEC_ID_CLJR: AVCodecID = 36;
pub const AV_CODEC_ID_VCR1: AVCodecID = 35;
pub const AV_CODEC_ID_4XM: AVCodecID = 34;
pub const AV_CODEC_ID_FFV1: AVCodecID = 33;
pub const AV_CODEC_ID_ASV2: AVCodecID = 32;
pub const AV_CODEC_ID_ASV1: AVCodecID = 31;
pub const AV_CODEC_ID_THEORA: AVCodecID = 30;
pub const AV_CODEC_ID_VP3: AVCodecID = 29;
pub const AV_CODEC_ID_INDEO3: AVCodecID = 28;
pub const AV_CODEC_ID_H264: AVCodecID = 27;
pub const AV_CODEC_ID_CYUV: AVCodecID = 26;
pub const AV_CODEC_ID_HUFFYUV: AVCodecID = 25;
pub const AV_CODEC_ID_DVVIDEO: AVCodecID = 24;
pub const AV_CODEC_ID_SVQ3: AVCodecID = 23;
pub const AV_CODEC_ID_SVQ1: AVCodecID = 22;
pub const AV_CODEC_ID_FLV1: AVCodecID = 21;
pub const AV_CODEC_ID_H263I: AVCodecID = 20;
pub const AV_CODEC_ID_H263P: AVCodecID = 19;
pub const AV_CODEC_ID_WMV2: AVCodecID = 18;
pub const AV_CODEC_ID_WMV1: AVCodecID = 17;
pub const AV_CODEC_ID_MSMPEG4V3: AVCodecID = 16;
pub const AV_CODEC_ID_MSMPEG4V2: AVCodecID = 15;
pub const AV_CODEC_ID_MSMPEG4V1: AVCodecID = 14;
pub const AV_CODEC_ID_RAWVIDEO: AVCodecID = 13;
pub const AV_CODEC_ID_MPEG4: AVCodecID = 12;
pub const AV_CODEC_ID_JPEGLS: AVCodecID = 11;
pub const AV_CODEC_ID_SP5X: AVCodecID = 10;
pub const AV_CODEC_ID_LJPEG: AVCodecID = 9;
pub const AV_CODEC_ID_MJPEGB: AVCodecID = 8;
pub const AV_CODEC_ID_MJPEG: AVCodecID = 7;
pub const AV_CODEC_ID_RV20: AVCodecID = 6;
pub const AV_CODEC_ID_RV10: AVCodecID = 5;
pub const AV_CODEC_ID_H263: AVCodecID = 4;
pub const AV_CODEC_ID_H261: AVCodecID = 3;
pub const AV_CODEC_ID_MPEG2VIDEO: AVCodecID = 2;
pub const AV_CODEC_ID_MPEG1VIDEO: AVCodecID = 1;
pub const AV_CODEC_ID_NONE: AVCodecID = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVProfile {
    pub profile: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodec {
    pub name: *const ::core::ffi::c_char,
    pub long_name: *const ::core::ffi::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub capabilities: ::core::ffi::c_int,
    pub max_lowres: uint8_t,
    pub priv_class: *const AVClass,
    pub profiles: *const AVProfile,
    pub wrapper_name: *const ::core::ffi::c_char,
}
pub type AVFieldOrder = ::core::ffi::c_uint;
pub const AV_FIELD_BT: AVFieldOrder = 5;
pub const AV_FIELD_TB: AVFieldOrder = 4;
pub const AV_FIELD_BB: AVFieldOrder = 3;
pub const AV_FIELD_TT: AVFieldOrder = 2;
pub const AV_FIELD_PROGRESSIVE: AVFieldOrder = 1;
pub const AV_FIELD_UNKNOWN: AVFieldOrder = 0;
pub type AVDiscard = ::core::ffi::c_int;
pub const AVDISCARD_ALL: AVDiscard = 48;
pub const AVDISCARD_NONKEY: AVDiscard = 32;
pub const AVDISCARD_NONINTRA: AVDiscard = 24;
pub const AVDISCARD_BIDIR: AVDiscard = 16;
pub const AVDISCARD_NONREF: AVDiscard = 8;
pub const AVDISCARD_DEFAULT: AVDiscard = 0;
pub const AVDISCARD_NONE: AVDiscard = -16;
pub type AVAudioServiceType = ::core::ffi::c_uint;
pub const AV_AUDIO_SERVICE_TYPE_NB: AVAudioServiceType = 9;
pub const AV_AUDIO_SERVICE_TYPE_KARAOKE: AVAudioServiceType = 8;
pub const AV_AUDIO_SERVICE_TYPE_VOICE_OVER: AVAudioServiceType = 7;
pub const AV_AUDIO_SERVICE_TYPE_EMERGENCY: AVAudioServiceType = 6;
pub const AV_AUDIO_SERVICE_TYPE_COMMENTARY: AVAudioServiceType = 5;
pub const AV_AUDIO_SERVICE_TYPE_DIALOGUE: AVAudioServiceType = 4;
pub const AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED: AVAudioServiceType = 3;
pub const AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED: AVAudioServiceType = 2;
pub const AV_AUDIO_SERVICE_TYPE_EFFECTS: AVAudioServiceType = 1;
pub const AV_AUDIO_SERVICE_TYPE_MAIN: AVAudioServiceType = 0;
pub type AVPacketSideDataType = ::core::ffi::c_uint;
pub const AV_PKT_DATA_NB: AVPacketSideDataType = 43;
pub const AV_PKT_DATA_HEVC_CONF: AVPacketSideDataType = 42;
pub const AV_PKT_DATA_DYNAMIC_HDR_SMPTE_2094_APP5: AVPacketSideDataType = 41;
pub const AV_PKT_DATA_EXIF: AVPacketSideDataType = 40;
pub const AV_PKT_DATA_RTCP_SR: AVPacketSideDataType = 39;
pub const AV_PKT_DATA_3D_REFERENCE_DISPLAYS: AVPacketSideDataType = 38;
pub const AV_PKT_DATA_LCEVC: AVPacketSideDataType = 37;
pub const AV_PKT_DATA_FRAME_CROPPING: AVPacketSideDataType = 36;
pub const AV_PKT_DATA_AMBIENT_VIEWING_ENVIRONMENT: AVPacketSideDataType = 35;
pub const AV_PKT_DATA_IAMF_RECON_GAIN_INFO_PARAM: AVPacketSideDataType = 34;
pub const AV_PKT_DATA_IAMF_DEMIXING_INFO_PARAM: AVPacketSideDataType = 33;
pub const AV_PKT_DATA_IAMF_MIX_GAIN_PARAM: AVPacketSideDataType = 32;
pub const AV_PKT_DATA_DYNAMIC_HDR10_PLUS: AVPacketSideDataType = 31;
pub const AV_PKT_DATA_S12M_TIMECODE: AVPacketSideDataType = 30;
pub const AV_PKT_DATA_DOVI_CONF: AVPacketSideDataType = 29;
pub const AV_PKT_DATA_ICC_PROFILE: AVPacketSideDataType = 28;
pub const AV_PKT_DATA_PRFT: AVPacketSideDataType = 27;
pub const AV_PKT_DATA_AFD: AVPacketSideDataType = 26;
pub const AV_PKT_DATA_ENCRYPTION_INFO: AVPacketSideDataType = 25;
pub const AV_PKT_DATA_ENCRYPTION_INIT_INFO: AVPacketSideDataType = 24;
pub const AV_PKT_DATA_A53_CC: AVPacketSideDataType = 23;
pub const AV_PKT_DATA_CONTENT_LIGHT_LEVEL: AVPacketSideDataType = 22;
pub const AV_PKT_DATA_SPHERICAL: AVPacketSideDataType = 21;
pub const AV_PKT_DATA_MASTERING_DISPLAY_METADATA: AVPacketSideDataType = 20;
pub const AV_PKT_DATA_MPEGTS_STREAM_ID: AVPacketSideDataType = 19;
pub const AV_PKT_DATA_METADATA_UPDATE: AVPacketSideDataType = 18;
pub const AV_PKT_DATA_WEBVTT_SETTINGS: AVPacketSideDataType = 17;
pub const AV_PKT_DATA_WEBVTT_IDENTIFIER: AVPacketSideDataType = 16;
pub const AV_PKT_DATA_MATROSKA_BLOCKADDITIONAL: AVPacketSideDataType = 15;
pub const AV_PKT_DATA_SUBTITLE_POSITION: AVPacketSideDataType = 14;
pub const AV_PKT_DATA_STRINGS_METADATA: AVPacketSideDataType = 13;
pub const AV_PKT_DATA_JP_DUALMONO: AVPacketSideDataType = 12;
pub const AV_PKT_DATA_SKIP_SAMPLES: AVPacketSideDataType = 11;
pub const AV_PKT_DATA_CPB_PROPERTIES: AVPacketSideDataType = 10;
pub const AV_PKT_DATA_FALLBACK_TRACK: AVPacketSideDataType = 9;
pub const AV_PKT_DATA_QUALITY_STATS: AVPacketSideDataType = 8;
pub const AV_PKT_DATA_AUDIO_SERVICE_TYPE: AVPacketSideDataType = 7;
pub const AV_PKT_DATA_STEREO3D: AVPacketSideDataType = 6;
pub const AV_PKT_DATA_DISPLAYMATRIX: AVPacketSideDataType = 5;
pub const AV_PKT_DATA_REPLAYGAIN: AVPacketSideDataType = 4;
pub const AV_PKT_DATA_H263_MB_INFO: AVPacketSideDataType = 3;
pub const AV_PKT_DATA_PARAM_CHANGE: AVPacketSideDataType = 2;
pub const AV_PKT_DATA_NEW_EXTRADATA: AVPacketSideDataType = 1;
pub const AV_PKT_DATA_PALETTE: AVPacketSideDataType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacketSideData {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub type_0: AVPacketSideDataType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPacket {
    pub buf: *mut AVBufferRef,
    pub pts: int64_t,
    pub dts: int64_t,
    pub data: *mut uint8_t,
    pub size: ::core::ffi::c_int,
    pub stream_index: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub side_data: *mut AVPacketSideData,
    pub side_data_elems: ::core::ffi::c_int,
    pub duration: int64_t,
    pub pos: int64_t,
    pub opaque: *mut ::core::ffi::c_void,
    pub opaque_ref: *mut AVBufferRef,
    pub time_base: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RcOverride {
    pub start_frame: ::core::ffi::c_int,
    pub end_frame: ::core::ffi::c_int,
    pub qscale: ::core::ffi::c_int,
    pub quality_factor: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecContext {
    pub av_class: *const AVClass,
    pub log_level_offset: ::core::ffi::c_int,
    pub codec_type: AVMediaType,
    pub codec: *const AVCodec,
    pub codec_id: AVCodecID,
    pub codec_tag: ::core::ffi::c_uint,
    pub priv_data: *mut ::core::ffi::c_void,
    pub internal: *mut AVCodecInternal,
    pub opaque: *mut ::core::ffi::c_void,
    pub bit_rate: int64_t,
    pub flags: ::core::ffi::c_int,
    pub flags2: ::core::ffi::c_int,
    pub extradata: *mut uint8_t,
    pub extradata_size: ::core::ffi::c_int,
    pub time_base: AVRational,
    pub pkt_timebase: AVRational,
    pub framerate: AVRational,
    pub delay: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub coded_width: ::core::ffi::c_int,
    pub coded_height: ::core::ffi::c_int,
    pub sample_aspect_ratio: AVRational,
    pub pix_fmt: AVPixelFormat,
    pub sw_pix_fmt: AVPixelFormat,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub colorspace: AVColorSpace,
    pub color_range: AVColorRange,
    pub chroma_sample_location: AVChromaLocation,
    pub field_order: AVFieldOrder,
    pub refs: ::core::ffi::c_int,
    pub has_b_frames: ::core::ffi::c_int,
    pub slice_flags: ::core::ffi::c_int,
    pub draw_horiz_band: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVFrame,
            *mut ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub get_format: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *const AVPixelFormat) -> AVPixelFormat,
    >,
    pub max_b_frames: ::core::ffi::c_int,
    pub b_quant_factor: ::core::ffi::c_float,
    pub b_quant_offset: ::core::ffi::c_float,
    pub i_quant_factor: ::core::ffi::c_float,
    pub i_quant_offset: ::core::ffi::c_float,
    pub lumi_masking: ::core::ffi::c_float,
    pub temporal_cplx_masking: ::core::ffi::c_float,
    pub spatial_cplx_masking: ::core::ffi::c_float,
    pub p_masking: ::core::ffi::c_float,
    pub dark_masking: ::core::ffi::c_float,
    pub nsse_weight: ::core::ffi::c_int,
    pub me_cmp: ::core::ffi::c_int,
    pub me_sub_cmp: ::core::ffi::c_int,
    pub mb_cmp: ::core::ffi::c_int,
    pub ildct_cmp: ::core::ffi::c_int,
    pub dia_size: ::core::ffi::c_int,
    pub last_predictor_count: ::core::ffi::c_int,
    pub me_pre_cmp: ::core::ffi::c_int,
    pub pre_dia_size: ::core::ffi::c_int,
    pub me_subpel_quality: ::core::ffi::c_int,
    pub me_range: ::core::ffi::c_int,
    pub mb_decision: ::core::ffi::c_int,
    pub intra_matrix: *mut uint16_t,
    pub inter_matrix: *mut uint16_t,
    pub chroma_intra_matrix: *mut uint16_t,
    pub intra_dc_precision: ::core::ffi::c_int,
    pub mb_lmin: ::core::ffi::c_int,
    pub mb_lmax: ::core::ffi::c_int,
    pub bidir_refine: ::core::ffi::c_int,
    pub keyint_min: ::core::ffi::c_int,
    pub gop_size: ::core::ffi::c_int,
    pub mv0_threshold: ::core::ffi::c_int,
    pub slices: ::core::ffi::c_int,
    pub sample_rate: ::core::ffi::c_int,
    pub sample_fmt: AVSampleFormat,
    pub ch_layout: AVChannelLayout,
    pub frame_size: ::core::ffi::c_int,
    pub block_align: ::core::ffi::c_int,
    pub cutoff: ::core::ffi::c_int,
    pub audio_service_type: AVAudioServiceType,
    pub request_sample_fmt: AVSampleFormat,
    pub initial_padding: ::core::ffi::c_int,
    pub trailing_padding: ::core::ffi::c_int,
    pub seek_preroll: ::core::ffi::c_int,
    pub get_buffer2: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVFrame,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub bit_rate_tolerance: ::core::ffi::c_int,
    pub global_quality: ::core::ffi::c_int,
    pub compression_level: ::core::ffi::c_int,
    pub qcompress: ::core::ffi::c_float,
    pub qblur: ::core::ffi::c_float,
    pub qmin: ::core::ffi::c_int,
    pub qmax: ::core::ffi::c_int,
    pub max_qdiff: ::core::ffi::c_int,
    pub rc_buffer_size: ::core::ffi::c_int,
    pub rc_override_count: ::core::ffi::c_int,
    pub rc_override: *mut RcOverride,
    pub rc_max_rate: int64_t,
    pub rc_min_rate: int64_t,
    pub rc_max_available_vbv_use: ::core::ffi::c_float,
    pub rc_min_vbv_overflow_use: ::core::ffi::c_float,
    pub rc_initial_buffer_occupancy: ::core::ffi::c_int,
    pub trellis: ::core::ffi::c_int,
    pub stats_out: *mut ::core::ffi::c_char,
    pub stats_in: *mut ::core::ffi::c_char,
    pub workaround_bugs: ::core::ffi::c_int,
    pub strict_std_compliance: ::core::ffi::c_int,
    pub error_concealment: ::core::ffi::c_int,
    pub debug: ::core::ffi::c_int,
    pub err_recognition: ::core::ffi::c_int,
    pub hwaccel: *const AVHWAccel,
    pub hwaccel_context: *mut ::core::ffi::c_void,
    pub hw_frames_ctx: *mut AVBufferRef,
    pub hw_device_ctx: *mut AVBufferRef,
    pub hwaccel_flags: ::core::ffi::c_int,
    pub extra_hw_frames: ::core::ffi::c_int,
    pub error: [uint64_t; 8],
    pub dct_algo: ::core::ffi::c_int,
    pub idct_algo: ::core::ffi::c_int,
    pub bits_per_coded_sample: ::core::ffi::c_int,
    pub bits_per_raw_sample: ::core::ffi::c_int,
    pub thread_count: ::core::ffi::c_int,
    pub thread_type: ::core::ffi::c_int,
    pub active_thread_type: ::core::ffi::c_int,
    pub execute: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub execute2: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            Option<
                unsafe extern "C" fn(
                    *mut AVCodecContext,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >,
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub profile: ::core::ffi::c_int,
    pub level: ::core::ffi::c_int,
    pub skip_loop_filter: AVDiscard,
    pub skip_idct: AVDiscard,
    pub skip_frame: AVDiscard,
    pub skip_alpha: ::core::ffi::c_int,
    pub skip_top: ::core::ffi::c_int,
    pub skip_bottom: ::core::ffi::c_int,
    pub lowres: ::core::ffi::c_int,
    pub codec_descriptor: *const AVCodecDescriptor,
    pub sub_charenc: *mut ::core::ffi::c_char,
    pub sub_charenc_mode: ::core::ffi::c_int,
    pub subtitle_header_size: ::core::ffi::c_int,
    pub subtitle_header: *mut uint8_t,
    pub dump_separator: *mut uint8_t,
    pub codec_whitelist: *mut ::core::ffi::c_char,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: ::core::ffi::c_int,
    pub export_side_data: ::core::ffi::c_int,
    pub max_pixels: int64_t,
    pub apply_cropping: ::core::ffi::c_int,
    pub discard_damaged_percentage: ::core::ffi::c_int,
    pub max_samples: int64_t,
    pub get_encode_buffer: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVPacket,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub frame_num: int64_t,
    pub side_data_prefer_packet: *mut ::core::ffi::c_int,
    pub nb_side_data_prefer_packet: ::core::ffi::c_uint,
    pub decoded_side_data: *mut *mut AVFrameSideData,
    pub nb_decoded_side_data: ::core::ffi::c_int,
    pub alpha_mode: AVAlphaMode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVHWAccel {
    pub name: *const ::core::ffi::c_char,
    pub type_0: AVMediaType,
    pub id: AVCodecID,
    pub pix_fmt: AVPixelFormat,
    pub capabilities: ::core::ffi::c_int,
}
pub type atomic_int = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ERPicture {
    pub f: *mut AVFrame,
    pub tf: *const ThreadFrame,
    pub progress: *const ThreadProgress,
    pub motion_val: [*mut [int16_t; 2]; 2],
    pub ref_index: [*mut int8_t; 2],
    pub mb_type: *mut uint32_t,
    pub field_picture: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadProgress {
    pub progress: atomic_int,
    pub init: ::core::ffi::c_uint,
    pub progress_mutex: pthread_mutex_t,
    pub progress_cond: pthread_cond_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_cond_t {
    pub __u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __i: [::core::ffi::c_int; 12],
    pub __vi: [::core::ffi::c_int; 12],
    pub __p: [*mut ::core::ffi::c_void; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pthread_mutex_t {
    pub __u: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub __i: [::core::ffi::c_int; 10],
    pub __vi: [::core::ffi::c_int; 10],
    pub __p: [*mut ::core::ffi::c_void; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThreadFrame {
    pub f: *mut AVFrame,
    pub owner: [*mut AVCodecContext; 2],
    pub progress: *mut ThreadFrameProgress,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ERContext {
    pub avctx: *mut AVCodecContext,
    pub sad: Option<
        unsafe extern "C" fn(
            *mut MPVEncContext,
            *const uint8_t,
            *const uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub mb_index2xy: *mut ::core::ffi::c_int,
    pub mb_num: ::core::ffi::c_int,
    pub mb_width: ::core::ffi::c_int,
    pub mb_height: ::core::ffi::c_int,
    pub mb_stride: ptrdiff_t,
    pub b8_stride: ptrdiff_t,
    pub error_count: atomic_int,
    pub error_occurred: ::core::ffi::c_int,
    pub error_status_table: *mut uint8_t,
    pub er_temp_buffer: *mut uint8_t,
    pub dc_val: [*mut int16_t; 3],
    pub mbskip_table: *mut uint8_t,
    pub mbintra_table: *mut uint8_t,
    pub mv: [[[::core::ffi::c_int; 2]; 4]; 2],
    pub cur_pic: ERPicture,
    pub last_pic: ERPicture,
    pub next_pic: ERPicture,
    pub ref_index: [*mut int8_t; 2],
    pub motion_val_base: [*mut [int16_t; 2]; 2],
    pub pp_time: uint16_t,
    pub pb_time: uint16_t,
    pub quarter_sample: ::core::ffi::c_int,
    pub partitioned_frame: ::core::ffi::c_int,
    pub decode_mb: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut [[[::core::ffi::c_int; 2]; 4]; 2],
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub opaque: *mut ::core::ffi::c_void,
}
pub type me_cmp_func = Option<
    unsafe extern "C" fn(
        *mut MPVEncContext,
        *const uint8_t,
        *const uint8_t,
        ptrdiff_t,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MECmpContext {
    pub sum_abs_dctelem: Option<
        unsafe extern "C" fn(*const int16_t) -> ::core::ffi::c_int,
    >,
    pub sad: [me_cmp_func; 6],
    pub sse: [me_cmp_func; 6],
    pub hadamard8_diff: [me_cmp_func; 6],
    pub dct_sad: [me_cmp_func; 6],
    pub quant_psnr: [me_cmp_func; 6],
    pub bit: [me_cmp_func; 6],
    pub rd: [me_cmp_func; 6],
    pub vsad: [me_cmp_func; 6],
    pub vsse: [me_cmp_func; 6],
    pub nsse: [me_cmp_func; 6],
    pub w53: [me_cmp_func; 6],
    pub w97: [me_cmp_func; 6],
    pub dct_max: [me_cmp_func; 6],
    pub dct264_sad: [me_cmp_func; 6],
    pub pix_abs: [[me_cmp_func; 4]; 2],
    pub median_sad: [me_cmp_func; 6],
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn ff_er_init(s: *mut ERContext) -> ::core::ffi::c_int {
    let mut mecc: MECmpContext = MECmpContext {
        sum_abs_dctelem: None,
        sad: [None; 6],
        sse: [None; 6],
        hadamard8_diff: [None; 6],
        dct_sad: [None; 6],
        quant_psnr: [None; 6],
        bit: [None; 6],
        rd: [None; 6],
        vsad: [None; 6],
        vsse: [None; 6],
        nsse: [None; 6],
        w53: [None; 6],
        w97: [None; 6],
        dct_max: [None; 6],
        dct264_sad: [None; 6],
        pix_abs: [[None; 4]; 2],
        median_sad: [None; 6],
    };
    let mut mb_array_size: ::core::ffi::c_uint = ((*s).mb_height as ptrdiff_t
        * (*s).mb_stride) as ::core::ffi::c_uint;
    (*s).error_status_table = av_mallocz(mb_array_size as size_t) as *mut uint8_t;
    if (*s).error_status_table.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*s).er_temp_buffer = av_malloc_array(
        mb_array_size as size_t,
        (4 as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_int>() as size_t)
            .wrapping_add(1 as size_t),
    ) as *mut uint8_t;
    if (*s).er_temp_buffer.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    ff_me_cmp_init(&raw mut mecc, (*s).avctx);
    (*s).sad = mecc.sad[0 as ::core::ffi::c_int as usize]
        as Option<
            unsafe extern "C" fn(
                *mut MPVEncContext,
                *const uint8_t,
                *const uint8_t,
                ptrdiff_t,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn set_mv_strides(
    mut s: *mut ERContext,
    mut mv_step: *mut ptrdiff_t,
    mut stride: *mut ptrdiff_t,
) {
    if (*(*s).avctx).codec_id as ::core::ffi::c_uint
        == AV_CODEC_ID_H264 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*s).quarter_sample == 0 {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"s->quarter_sample\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavcodec/error_resilience.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                67 as ::core::ffi::c_int,
            );
            abort();
        }
        *mv_step = 4 as ptrdiff_t;
        *stride = ((*s).mb_width * 4 as ::core::ffi::c_int) as ptrdiff_t;
    } else {
        *mv_step = 2 as ptrdiff_t;
        *stride = (*s).b8_stride;
    };
}
unsafe extern "C" fn put_dc(
    mut s: *mut ERContext,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) {
    let mut linesize: *mut ::core::ffi::c_int = &raw mut (*(*s).cur_pic.f).linesize
        as *mut ::core::ffi::c_int;
    let mut dc: ::core::ffi::c_int = 0;
    let mut dcu: ::core::ffi::c_int = 0;
    let mut dcv: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        dc = *(*s)
            .dc_val[0 as ::core::ffi::c_int as usize]
            .offset(
                ((mb_x * 2 as ::core::ffi::c_int + (i & 1 as ::core::ffi::c_int))
                    as ptrdiff_t
                    + (mb_y * 2 as ::core::ffi::c_int + (i >> 1 as ::core::ffi::c_int))
                        as ptrdiff_t * (*s).b8_stride) as isize,
            ) as ::core::ffi::c_int;
        if dc < 0 as ::core::ffi::c_int {
            dc = 0 as ::core::ffi::c_int;
        } else if dc > 2040 as ::core::ffi::c_int {
            dc = 2040 as ::core::ffi::c_int;
        }
        y = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            let mut x: ::core::ffi::c_int = 0;
            x = 0 as ::core::ffi::c_int;
            while x < 8 as ::core::ffi::c_int {
                *dest_y
                    .offset(
                        (x + (i & 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int
                            + (y
                                + (i >> 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                                * *linesize.offset(0 as ::core::ffi::c_int as isize))
                            as isize,
                    ) = (dc / 8 as ::core::ffi::c_int) as uint8_t;
                x += 1;
            }
            y += 1;
        }
        i += 1;
    }
    dcu = *(*s)
        .dc_val[1 as ::core::ffi::c_int as usize]
        .offset((mb_x as ptrdiff_t + mb_y as ptrdiff_t * (*s).mb_stride) as isize)
        as ::core::ffi::c_int;
    dcv = *(*s)
        .dc_val[2 as ::core::ffi::c_int as usize]
        .offset((mb_x as ptrdiff_t + mb_y as ptrdiff_t * (*s).mb_stride) as isize)
        as ::core::ffi::c_int;
    if dcu < 0 as ::core::ffi::c_int {
        dcu = 0 as ::core::ffi::c_int;
    } else if dcu > 2040 as ::core::ffi::c_int {
        dcu = 2040 as ::core::ffi::c_int;
    }
    if dcv < 0 as ::core::ffi::c_int {
        dcv = 0 as ::core::ffi::c_int;
    } else if dcv > 2040 as ::core::ffi::c_int {
        dcv = 2040 as ::core::ffi::c_int;
    }
    if !dest_cr.is_null() {
        y = 0 as ::core::ffi::c_int;
        while y < 8 as ::core::ffi::c_int {
            let mut x_0: ::core::ffi::c_int = 0;
            x_0 = 0 as ::core::ffi::c_int;
            while x_0 < 8 as ::core::ffi::c_int {
                *dest_cb
                    .offset(
                        (x_0 + y * *linesize.offset(1 as ::core::ffi::c_int as isize))
                            as isize,
                    ) = (dcu / 8 as ::core::ffi::c_int) as uint8_t;
                *dest_cr
                    .offset(
                        (x_0 + y * *linesize.offset(2 as ::core::ffi::c_int as isize))
                            as isize,
                    ) = (dcv / 8 as ::core::ffi::c_int) as uint8_t;
                x_0 += 1;
            }
            y += 1;
        }
    }
}
unsafe extern "C" fn filter181(
    mut data: *mut int16_t,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut stride: ptrdiff_t,
) {
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    y = 1 as ::core::ffi::c_int;
    while y < height - 1 as ::core::ffi::c_int {
        let mut prev_dc: ::core::ffi::c_int = *data
            .offset((0 as ptrdiff_t + y as ptrdiff_t * stride) as isize)
            as ::core::ffi::c_int;
        x = 1 as ::core::ffi::c_int;
        while x < width - 1 as ::core::ffi::c_int {
            let mut dc: ::core::ffi::c_int = 0;
            dc = -prev_dc
                + *data.offset((x as ptrdiff_t + y as ptrdiff_t * stride) as isize)
                    as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                - *data
                    .offset(
                        ((x + 1 as ::core::ffi::c_int) as ptrdiff_t
                            + y as ptrdiff_t * stride) as isize,
                    ) as ::core::ffi::c_int;
            dc = av_clip_c(
                dc,
                (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                    / 10923 as ::core::ffi::c_int,
                2147483647 as ::core::ffi::c_int / 10923 as ::core::ffi::c_int
                    - 32768 as ::core::ffi::c_int,
            ) * 10923 as ::core::ffi::c_int + 32768 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            prev_dc = *data.offset((x as ptrdiff_t + y as ptrdiff_t * stride) as isize)
                as ::core::ffi::c_int;
            *data.offset((x as ptrdiff_t + y as ptrdiff_t * stride) as isize) = dc
                as int16_t;
            x += 1;
        }
        y += 1;
    }
    x = 1 as ::core::ffi::c_int;
    while x < width - 1 as ::core::ffi::c_int {
        let mut prev_dc_0: ::core::ffi::c_int = *data.offset(x as isize)
            as ::core::ffi::c_int;
        y = 1 as ::core::ffi::c_int;
        while y < height - 1 as ::core::ffi::c_int {
            let mut dc_0: ::core::ffi::c_int = 0;
            dc_0 = -prev_dc_0
                + *data.offset((x as ptrdiff_t + y as ptrdiff_t * stride) as isize)
                    as ::core::ffi::c_int * 8 as ::core::ffi::c_int
                - *data
                    .offset(
                        (x as ptrdiff_t
                            + (y + 1 as ::core::ffi::c_int) as ptrdiff_t * stride)
                            as isize,
                    ) as ::core::ffi::c_int;
            dc_0 = av_clip_c(
                dc_0,
                (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                    / 10923 as ::core::ffi::c_int,
                2147483647 as ::core::ffi::c_int / 10923 as ::core::ffi::c_int
                    - 32768 as ::core::ffi::c_int,
            ) * 10923 as ::core::ffi::c_int + 32768 as ::core::ffi::c_int
                >> 16 as ::core::ffi::c_int;
            prev_dc_0 = *data.offset((x as ptrdiff_t + y as ptrdiff_t * stride) as isize)
                as ::core::ffi::c_int;
            *data.offset((x as ptrdiff_t + y as ptrdiff_t * stride) as isize) = dc_0
                as int16_t;
            y += 1;
        }
        x += 1;
    }
}
unsafe extern "C" fn guess_dc(
    mut s: *mut ERContext,
    mut dc: *mut int16_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut stride: ptrdiff_t,
    mut is_luma: ::core::ffi::c_int,
) {
    let mut b_x: ::core::ffi::c_int = 0;
    let mut b_y: ::core::ffi::c_int = 0;
    let mut col: *mut [int16_t; 4] = av_malloc_array(
        stride as size_t,
        (h as size_t)
            .wrapping_mul(::core::mem::size_of::<int16_t>() as size_t)
            .wrapping_mul(4 as size_t),
    ) as *mut [int16_t; 4];
    let mut dist: *mut [uint32_t; 4] = av_malloc_array(
        stride as size_t,
        (h as size_t)
            .wrapping_mul(::core::mem::size_of::<uint32_t>() as size_t)
            .wrapping_mul(4 as size_t),
    ) as *mut [uint32_t; 4];
    if col.is_null() || dist.is_null() {
        av_log(
            (*s).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"guess_dc() is out of memory\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        b_y = 0 as ::core::ffi::c_int;
        while b_y < h {
            let mut color: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
            let mut distance: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            b_x = 0 as ::core::ffi::c_int;
            while b_x < w {
                let mut mb_index_j: ::core::ffi::c_int = ((b_x >> is_luma) as ptrdiff_t
                    + (b_y >> is_luma) as ptrdiff_t * (*s).mb_stride)
                    as ::core::ffi::c_int;
                let mut error_j: ::core::ffi::c_int = *(*s)
                    .error_status_table
                    .offset(mb_index_j as isize) as ::core::ffi::c_int;
                let mut intra_j: ::core::ffi::c_int = (*(*s)
                    .cur_pic
                    .mb_type
                    .offset(mb_index_j as isize) & 7 as uint32_t) as ::core::ffi::c_int;
                if intra_j == 0 as ::core::ffi::c_int
                    || error_j & 4 as ::core::ffi::c_int == 0
                {
                    color = *dc
                        .offset((b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize)
                        as ::core::ffi::c_int;
                    distance = b_x;
                }
                (*col
                    .offset(
                        (b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize,
                    ))[1 as ::core::ffi::c_int as usize] = color as int16_t;
                (*dist
                    .offset(
                        (b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize,
                    ))[1 as ::core::ffi::c_int as usize] = (if distance
                    >= 0 as ::core::ffi::c_int
                {
                    b_x - distance
                } else {
                    9999 as ::core::ffi::c_int
                }) as uint32_t;
                b_x += 1;
            }
            color = 1024 as ::core::ffi::c_int;
            distance = -(1 as ::core::ffi::c_int);
            b_x = w - 1 as ::core::ffi::c_int;
            while b_x >= 0 as ::core::ffi::c_int {
                let mut mb_index_j_0: ::core::ffi::c_int = ((b_x >> is_luma) as ptrdiff_t
                    + (b_y >> is_luma) as ptrdiff_t * (*s).mb_stride)
                    as ::core::ffi::c_int;
                let mut error_j_0: ::core::ffi::c_int = *(*s)
                    .error_status_table
                    .offset(mb_index_j_0 as isize) as ::core::ffi::c_int;
                let mut intra_j_0: ::core::ffi::c_int = (*(*s)
                    .cur_pic
                    .mb_type
                    .offset(mb_index_j_0 as isize) & 7 as uint32_t)
                    as ::core::ffi::c_int;
                if intra_j_0 == 0 as ::core::ffi::c_int
                    || error_j_0 & 4 as ::core::ffi::c_int == 0
                {
                    color = *dc
                        .offset((b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize)
                        as ::core::ffi::c_int;
                    distance = b_x;
                }
                (*col
                    .offset(
                        (b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize,
                    ))[0 as ::core::ffi::c_int as usize] = color as int16_t;
                (*dist
                    .offset(
                        (b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize,
                    ))[0 as ::core::ffi::c_int as usize] = (if distance
                    >= 0 as ::core::ffi::c_int
                {
                    distance - b_x
                } else {
                    9999 as ::core::ffi::c_int
                }) as uint32_t;
                b_x -= 1;
            }
            b_y += 1;
        }
        b_x = 0 as ::core::ffi::c_int;
        while b_x < w {
            let mut color_0: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
            let mut distance_0: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            b_y = 0 as ::core::ffi::c_int;
            while b_y < h {
                let mut mb_index_j_1: ::core::ffi::c_int = ((b_x >> is_luma) as ptrdiff_t
                    + (b_y >> is_luma) as ptrdiff_t * (*s).mb_stride)
                    as ::core::ffi::c_int;
                let mut error_j_1: ::core::ffi::c_int = *(*s)
                    .error_status_table
                    .offset(mb_index_j_1 as isize) as ::core::ffi::c_int;
                let mut intra_j_1: ::core::ffi::c_int = (*(*s)
                    .cur_pic
                    .mb_type
                    .offset(mb_index_j_1 as isize) & 7 as uint32_t)
                    as ::core::ffi::c_int;
                if intra_j_1 == 0 as ::core::ffi::c_int
                    || error_j_1 & 4 as ::core::ffi::c_int == 0
                {
                    color_0 = *dc
                        .offset((b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize)
                        as ::core::ffi::c_int;
                    distance_0 = b_y;
                }
                (*col
                    .offset(
                        (b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize,
                    ))[3 as ::core::ffi::c_int as usize] = color_0 as int16_t;
                (*dist
                    .offset(
                        (b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize,
                    ))[3 as ::core::ffi::c_int as usize] = (if distance_0
                    >= 0 as ::core::ffi::c_int
                {
                    b_y - distance_0
                } else {
                    9999 as ::core::ffi::c_int
                }) as uint32_t;
                b_y += 1;
            }
            color_0 = 1024 as ::core::ffi::c_int;
            distance_0 = -(1 as ::core::ffi::c_int);
            b_y = h - 1 as ::core::ffi::c_int;
            while b_y >= 0 as ::core::ffi::c_int {
                let mut mb_index_j_2: ::core::ffi::c_int = ((b_x >> is_luma) as ptrdiff_t
                    + (b_y >> is_luma) as ptrdiff_t * (*s).mb_stride)
                    as ::core::ffi::c_int;
                let mut error_j_2: ::core::ffi::c_int = *(*s)
                    .error_status_table
                    .offset(mb_index_j_2 as isize) as ::core::ffi::c_int;
                let mut intra_j_2: ::core::ffi::c_int = (*(*s)
                    .cur_pic
                    .mb_type
                    .offset(mb_index_j_2 as isize) & 7 as uint32_t)
                    as ::core::ffi::c_int;
                if intra_j_2 == 0 as ::core::ffi::c_int
                    || error_j_2 & 4 as ::core::ffi::c_int == 0
                {
                    color_0 = *dc
                        .offset((b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize)
                        as ::core::ffi::c_int;
                    distance_0 = b_y;
                }
                (*col
                    .offset(
                        (b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize,
                    ))[2 as ::core::ffi::c_int as usize] = color_0 as int16_t;
                (*dist
                    .offset(
                        (b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize,
                    ))[2 as ::core::ffi::c_int as usize] = (if distance_0
                    >= 0 as ::core::ffi::c_int
                {
                    distance_0 - b_y
                } else {
                    9999 as ::core::ffi::c_int
                }) as uint32_t;
                b_y -= 1;
            }
            b_x += 1;
        }
        b_y = 0 as ::core::ffi::c_int;
        while b_y < h {
            b_x = 0 as ::core::ffi::c_int;
            while b_x < w {
                let mut mb_index: ::core::ffi::c_int = 0;
                let mut error: ::core::ffi::c_int = 0;
                let mut j: ::core::ffi::c_int = 0;
                let mut guess: int64_t = 0;
                let mut weight_sum: int64_t = 0;
                mb_index = ((b_x >> is_luma) as ptrdiff_t
                    + (b_y >> is_luma) as ptrdiff_t * (*s).mb_stride)
                    as ::core::ffi::c_int;
                error = *(*s).error_status_table.offset(mb_index as isize)
                    as ::core::ffi::c_int;
                if !(*(*s).cur_pic.mb_type.offset(mb_index as isize)
                    & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                        as uint32_t != 0)
                {
                    if !(error & 4 as ::core::ffi::c_int == 0) {
                        weight_sum = 0 as int64_t;
                        guess = 0 as int64_t;
                        j = 0 as ::core::ffi::c_int;
                        while j < 4 as ::core::ffi::c_int {
                            let mut weight: int64_t = ((256 as ::core::ffi::c_int
                                * 256 as ::core::ffi::c_int * 256 as ::core::ffi::c_int
                                * 16 as ::core::ffi::c_int) as uint32_t)
                                .wrapping_div(
                                    (if (*dist
                                        .offset(
                                            (b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize,
                                        ))[j as usize] > 1 as uint32_t
                                    {
                                        (*dist
                                            .offset(
                                                (b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize,
                                            ))[j as usize]
                                    } else {
                                        1 as uint32_t
                                    }),
                                ) as int64_t;
                            guess
                                += weight
                                    * (*col
                                        .offset(
                                            (b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize,
                                        ))[j as usize] as int64_t;
                            weight_sum += weight;
                            j += 1;
                        }
                        guess = (guess + weight_sum / 2 as int64_t) / weight_sum;
                        *dc
                            .offset(
                                (b_x as ptrdiff_t + b_y as ptrdiff_t * stride) as isize,
                            ) = guess as int16_t;
                    }
                }
                b_x += 1;
            }
            b_y += 1;
        }
    }
    av_freep(&raw mut col as *mut ::core::ffi::c_void);
    av_freep(&raw mut dist as *mut ::core::ffi::c_void);
}
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: ::core::ffi::c_int,
    mut amin: ::core::ffi::c_int,
    mut amax: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if a < amin { return amin } else if a > amax { return amax } else { return a };
}
unsafe extern "C" fn h_block_filter(
    mut s: *mut ERContext,
    mut dst: *mut uint8_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut stride: ptrdiff_t,
    mut is_luma: ::core::ffi::c_int,
) {
    let mut b_x: ::core::ffi::c_int = 0;
    let mut b_y: ::core::ffi::c_int = 0;
    let mut mvx_stride: ptrdiff_t = 0;
    let mut mvy_stride: ptrdiff_t = 0;
    let mut cm: *const uint8_t = (&raw const ff_crop_tab as *const uint8_t)
        .offset(1024 as ::core::ffi::c_int as isize);
    set_mv_strides(s, &raw mut mvx_stride, &raw mut mvy_stride);
    mvx_stride >>= is_luma;
    mvy_stride *= mvx_stride;
    b_y = 0 as ::core::ffi::c_int;
    while b_y < h {
        b_x = 0 as ::core::ffi::c_int;
        while b_x < w - 1 as ::core::ffi::c_int {
            let mut y: ::core::ffi::c_int = 0;
            let mut left_status: ::core::ffi::c_int = *(*s)
                .error_status_table
                .offset(
                    ((b_x >> is_luma) as ptrdiff_t
                        + (b_y >> is_luma) as ptrdiff_t * (*s).mb_stride) as isize,
                ) as ::core::ffi::c_int;
            let mut right_status: ::core::ffi::c_int = *(*s)
                .error_status_table
                .offset(
                    ((b_x + 1 as ::core::ffi::c_int >> is_luma) as ptrdiff_t
                        + (b_y >> is_luma) as ptrdiff_t * (*s).mb_stride) as isize,
                ) as ::core::ffi::c_int;
            let mut left_intra: ::core::ffi::c_int = (*(*s)
                .cur_pic
                .mb_type
                .offset(
                    ((b_x >> is_luma) as ptrdiff_t
                        + (b_y >> is_luma) as ptrdiff_t * (*s).mb_stride) as isize,
                ) & 7 as uint32_t) as ::core::ffi::c_int;
            let mut right_intra: ::core::ffi::c_int = (*(*s)
                .cur_pic
                .mb_type
                .offset(
                    ((b_x + 1 as ::core::ffi::c_int >> is_luma) as ptrdiff_t
                        + (b_y >> is_luma) as ptrdiff_t * (*s).mb_stride) as isize,
                ) & 7 as uint32_t) as ::core::ffi::c_int;
            let mut left_damage: ::core::ffi::c_int = left_status
                & (2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                    | 8 as ::core::ffi::c_int);
            let mut right_damage: ::core::ffi::c_int = right_status
                & (2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                    | 8 as ::core::ffi::c_int);
            let mut offset: ::core::ffi::c_int = ((b_x * 8 as ::core::ffi::c_int)
                as ptrdiff_t + b_y as ptrdiff_t * stride * 8 as ptrdiff_t)
                as ::core::ffi::c_int;
            let mut left_mv: *mut int16_t = &raw mut *(*(&raw mut (*s).cur_pic.motion_val
                as *mut *mut [int16_t; 2])
                .offset(0 as ::core::ffi::c_int as isize))
                .offset(
                    (mvy_stride * b_y as ptrdiff_t + mvx_stride * b_x as ptrdiff_t)
                        as isize,
                ) as *mut int16_t;
            let mut right_mv: *mut int16_t = &raw mut *(*(&raw mut (*s)
                .cur_pic
                .motion_val as *mut *mut [int16_t; 2])
                .offset(0 as ::core::ffi::c_int as isize))
                .offset(
                    (mvy_stride * b_y as ptrdiff_t
                        + mvx_stride * (b_x + 1 as ::core::ffi::c_int) as ptrdiff_t)
                        as isize,
                ) as *mut int16_t;
            if left_damage != 0 || right_damage != 0 {
                if !(left_intra == 0 && right_intra == 0
                    && (if *left_mv.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        - *right_mv.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                    {
                        *left_mv.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - *right_mv.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                    } else {
                        -(*left_mv.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - *right_mv.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                    })
                        + (if *left_mv.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            + *right_mv.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                        {
                            *left_mv.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                + *right_mv.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                        } else {
                            -(*left_mv.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                + *right_mv.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int)
                        }) < 2 as ::core::ffi::c_int)
                {
                    y = 0 as ::core::ffi::c_int;
                    while y < 8 as ::core::ffi::c_int {
                        let mut a: ::core::ffi::c_int = 0;
                        let mut b: ::core::ffi::c_int = 0;
                        let mut c: ::core::ffi::c_int = 0;
                        let mut d: ::core::ffi::c_int = 0;
                        a = *dst
                            .offset(
                                ((offset + 7 as ::core::ffi::c_int) as ptrdiff_t
                                    + y as ptrdiff_t * stride) as isize,
                            ) as ::core::ffi::c_int
                            - *dst
                                .offset(
                                    ((offset + 6 as ::core::ffi::c_int) as ptrdiff_t
                                        + y as ptrdiff_t * stride) as isize,
                                ) as ::core::ffi::c_int;
                        b = *dst
                            .offset(
                                ((offset + 8 as ::core::ffi::c_int) as ptrdiff_t
                                    + y as ptrdiff_t * stride) as isize,
                            ) as ::core::ffi::c_int
                            - *dst
                                .offset(
                                    ((offset + 7 as ::core::ffi::c_int) as ptrdiff_t
                                        + y as ptrdiff_t * stride) as isize,
                                ) as ::core::ffi::c_int;
                        c = *dst
                            .offset(
                                ((offset + 9 as ::core::ffi::c_int) as ptrdiff_t
                                    + y as ptrdiff_t * stride) as isize,
                            ) as ::core::ffi::c_int
                            - *dst
                                .offset(
                                    ((offset + 8 as ::core::ffi::c_int) as ptrdiff_t
                                        + y as ptrdiff_t * stride) as isize,
                                ) as ::core::ffi::c_int;
                        d = (if b >= 0 as ::core::ffi::c_int { b } else { -b })
                            - ((if a >= 0 as ::core::ffi::c_int { a } else { -a })
                                + (if c >= 0 as ::core::ffi::c_int { c } else { -c })
                                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int);
                        d = if d > 0 as ::core::ffi::c_int {
                            d
                        } else {
                            0 as ::core::ffi::c_int
                        };
                        if b < 0 as ::core::ffi::c_int {
                            d = -d;
                        }
                        if !(d == 0 as ::core::ffi::c_int) {
                            if !(left_damage != 0 && right_damage != 0) {
                                d = d * 16 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                            }
                            if left_damage != 0 {
                                *dst
                                    .offset(
                                        ((offset + 7 as ::core::ffi::c_int) as ptrdiff_t
                                            + y as ptrdiff_t * stride) as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + 7 as ::core::ffi::c_int) as ptrdiff_t
                                                    + y as ptrdiff_t * stride) as isize,
                                            ) as ::core::ffi::c_int
                                            + (d * 7 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                                *dst
                                    .offset(
                                        ((offset + 6 as ::core::ffi::c_int) as ptrdiff_t
                                            + y as ptrdiff_t * stride) as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + 6 as ::core::ffi::c_int) as ptrdiff_t
                                                    + y as ptrdiff_t * stride) as isize,
                                            ) as ::core::ffi::c_int
                                            + (d * 5 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                                *dst
                                    .offset(
                                        ((offset + 5 as ::core::ffi::c_int) as ptrdiff_t
                                            + y as ptrdiff_t * stride) as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + 5 as ::core::ffi::c_int) as ptrdiff_t
                                                    + y as ptrdiff_t * stride) as isize,
                                            ) as ::core::ffi::c_int
                                            + (d * 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                                *dst
                                    .offset(
                                        ((offset + 4 as ::core::ffi::c_int) as ptrdiff_t
                                            + y as ptrdiff_t * stride) as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + 4 as ::core::ffi::c_int) as ptrdiff_t
                                                    + y as ptrdiff_t * stride) as isize,
                                            ) as ::core::ffi::c_int
                                            + (d * 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                            }
                            if right_damage != 0 {
                                *dst
                                    .offset(
                                        ((offset + 8 as ::core::ffi::c_int) as ptrdiff_t
                                            + y as ptrdiff_t * stride) as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + 8 as ::core::ffi::c_int) as ptrdiff_t
                                                    + y as ptrdiff_t * stride) as isize,
                                            ) as ::core::ffi::c_int
                                            - (d * 7 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                                *dst
                                    .offset(
                                        ((offset + 9 as ::core::ffi::c_int) as ptrdiff_t
                                            + y as ptrdiff_t * stride) as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + 9 as ::core::ffi::c_int) as ptrdiff_t
                                                    + y as ptrdiff_t * stride) as isize,
                                            ) as ::core::ffi::c_int
                                            - (d * 5 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                                *dst
                                    .offset(
                                        ((offset + 10 as ::core::ffi::c_int) as ptrdiff_t
                                            + y as ptrdiff_t * stride) as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + 10 as ::core::ffi::c_int) as ptrdiff_t
                                                    + y as ptrdiff_t * stride) as isize,
                                            ) as ::core::ffi::c_int
                                            - (d * 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                                *dst
                                    .offset(
                                        ((offset + 11 as ::core::ffi::c_int) as ptrdiff_t
                                            + y as ptrdiff_t * stride) as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + 11 as ::core::ffi::c_int) as ptrdiff_t
                                                    + y as ptrdiff_t * stride) as isize,
                                            ) as ::core::ffi::c_int
                                            - (d * 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                            }
                        }
                        y += 1;
                    }
                }
            }
            b_x += 1;
        }
        b_y += 1;
    }
}
unsafe extern "C" fn v_block_filter(
    mut s: *mut ERContext,
    mut dst: *mut uint8_t,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut stride: ptrdiff_t,
    mut is_luma: ::core::ffi::c_int,
) {
    let mut b_x: ::core::ffi::c_int = 0;
    let mut b_y: ::core::ffi::c_int = 0;
    let mut mvx_stride: ptrdiff_t = 0;
    let mut mvy_stride: ptrdiff_t = 0;
    let mut cm: *const uint8_t = (&raw const ff_crop_tab as *const uint8_t)
        .offset(1024 as ::core::ffi::c_int as isize);
    set_mv_strides(s, &raw mut mvx_stride, &raw mut mvy_stride);
    mvx_stride >>= is_luma;
    mvy_stride *= mvx_stride;
    b_y = 0 as ::core::ffi::c_int;
    while b_y < h - 1 as ::core::ffi::c_int {
        b_x = 0 as ::core::ffi::c_int;
        while b_x < w {
            let mut x: ::core::ffi::c_int = 0;
            let mut top_status: ::core::ffi::c_int = *(*s)
                .error_status_table
                .offset(
                    ((b_x >> is_luma) as ptrdiff_t
                        + (b_y >> is_luma) as ptrdiff_t * (*s).mb_stride) as isize,
                ) as ::core::ffi::c_int;
            let mut bottom_status: ::core::ffi::c_int = *(*s)
                .error_status_table
                .offset(
                    ((b_x >> is_luma) as ptrdiff_t
                        + (b_y + 1 as ::core::ffi::c_int >> is_luma) as ptrdiff_t
                            * (*s).mb_stride) as isize,
                ) as ::core::ffi::c_int;
            let mut top_intra: ::core::ffi::c_int = (*(*s)
                .cur_pic
                .mb_type
                .offset(
                    ((b_x >> is_luma) as ptrdiff_t
                        + (b_y >> is_luma) as ptrdiff_t * (*s).mb_stride) as isize,
                ) & 7 as uint32_t) as ::core::ffi::c_int;
            let mut bottom_intra: ::core::ffi::c_int = (*(*s)
                .cur_pic
                .mb_type
                .offset(
                    ((b_x >> is_luma) as ptrdiff_t
                        + (b_y + 1 as ::core::ffi::c_int >> is_luma) as ptrdiff_t
                            * (*s).mb_stride) as isize,
                ) & 7 as uint32_t) as ::core::ffi::c_int;
            let mut top_damage: ::core::ffi::c_int = top_status
                & (2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                    | 8 as ::core::ffi::c_int);
            let mut bottom_damage: ::core::ffi::c_int = bottom_status
                & (2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                    | 8 as ::core::ffi::c_int);
            let mut offset: ::core::ffi::c_int = ((b_x * 8 as ::core::ffi::c_int)
                as ptrdiff_t + b_y as ptrdiff_t * stride * 8 as ptrdiff_t)
                as ::core::ffi::c_int;
            let mut top_mv: *mut int16_t = &raw mut *(*(&raw mut (*s).cur_pic.motion_val
                as *mut *mut [int16_t; 2])
                .offset(0 as ::core::ffi::c_int as isize))
                .offset(
                    (mvy_stride * b_y as ptrdiff_t + mvx_stride * b_x as ptrdiff_t)
                        as isize,
                ) as *mut int16_t;
            let mut bottom_mv: *mut int16_t = &raw mut *(*(&raw mut (*s)
                .cur_pic
                .motion_val as *mut *mut [int16_t; 2])
                .offset(0 as ::core::ffi::c_int as isize))
                .offset(
                    (mvy_stride * (b_y + 1 as ::core::ffi::c_int) as ptrdiff_t
                        + mvx_stride * b_x as ptrdiff_t) as isize,
                ) as *mut int16_t;
            if top_damage != 0 || bottom_damage != 0 {
                if !(top_intra == 0 && bottom_intra == 0
                    && (if *top_mv.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        - *bottom_mv.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                    {
                        *top_mv.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - *bottom_mv.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                    } else {
                        -(*top_mv.offset(0 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            - *bottom_mv.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int)
                    })
                        + (if *top_mv.offset(1 as ::core::ffi::c_int as isize)
                            as ::core::ffi::c_int
                            + *bottom_mv.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                        {
                            *top_mv.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                + *bottom_mv.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int
                        } else {
                            -(*top_mv.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int
                                + *bottom_mv.offset(1 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int)
                        }) < 2 as ::core::ffi::c_int)
                {
                    x = 0 as ::core::ffi::c_int;
                    while x < 8 as ::core::ffi::c_int {
                        let mut a: ::core::ffi::c_int = 0;
                        let mut b: ::core::ffi::c_int = 0;
                        let mut c: ::core::ffi::c_int = 0;
                        let mut d: ::core::ffi::c_int = 0;
                        a = *dst
                            .offset(
                                ((offset + x) as ptrdiff_t + 7 as ptrdiff_t * stride)
                                    as isize,
                            ) as ::core::ffi::c_int
                            - *dst
                                .offset(
                                    ((offset + x) as ptrdiff_t + 6 as ptrdiff_t * stride)
                                        as isize,
                                ) as ::core::ffi::c_int;
                        b = *dst
                            .offset(
                                ((offset + x) as ptrdiff_t + 8 as ptrdiff_t * stride)
                                    as isize,
                            ) as ::core::ffi::c_int
                            - *dst
                                .offset(
                                    ((offset + x) as ptrdiff_t + 7 as ptrdiff_t * stride)
                                        as isize,
                                ) as ::core::ffi::c_int;
                        c = *dst
                            .offset(
                                ((offset + x) as ptrdiff_t + 9 as ptrdiff_t * stride)
                                    as isize,
                            ) as ::core::ffi::c_int
                            - *dst
                                .offset(
                                    ((offset + x) as ptrdiff_t + 8 as ptrdiff_t * stride)
                                        as isize,
                                ) as ::core::ffi::c_int;
                        d = (if b >= 0 as ::core::ffi::c_int { b } else { -b })
                            - ((if a >= 0 as ::core::ffi::c_int { a } else { -a })
                                + (if c >= 0 as ::core::ffi::c_int { c } else { -c })
                                + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int);
                        d = if d > 0 as ::core::ffi::c_int {
                            d
                        } else {
                            0 as ::core::ffi::c_int
                        };
                        if b < 0 as ::core::ffi::c_int {
                            d = -d;
                        }
                        if !(d == 0 as ::core::ffi::c_int) {
                            if !(top_damage != 0 && bottom_damage != 0) {
                                d = d * 16 as ::core::ffi::c_int / 9 as ::core::ffi::c_int;
                            }
                            if top_damage != 0 {
                                *dst
                                    .offset(
                                        ((offset + x) as ptrdiff_t + 7 as ptrdiff_t * stride)
                                            as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + x) as ptrdiff_t + 7 as ptrdiff_t * stride)
                                                    as isize,
                                            ) as ::core::ffi::c_int
                                            + (d * 7 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                                *dst
                                    .offset(
                                        ((offset + x) as ptrdiff_t + 6 as ptrdiff_t * stride)
                                            as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + x) as ptrdiff_t + 6 as ptrdiff_t * stride)
                                                    as isize,
                                            ) as ::core::ffi::c_int
                                            + (d * 5 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                                *dst
                                    .offset(
                                        ((offset + x) as ptrdiff_t + 5 as ptrdiff_t * stride)
                                            as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + x) as ptrdiff_t + 5 as ptrdiff_t * stride)
                                                    as isize,
                                            ) as ::core::ffi::c_int
                                            + (d * 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                                *dst
                                    .offset(
                                        ((offset + x) as ptrdiff_t + 4 as ptrdiff_t * stride)
                                            as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + x) as ptrdiff_t + 4 as ptrdiff_t * stride)
                                                    as isize,
                                            ) as ::core::ffi::c_int
                                            + (d * 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                            }
                            if bottom_damage != 0 {
                                *dst
                                    .offset(
                                        ((offset + x) as ptrdiff_t + 8 as ptrdiff_t * stride)
                                            as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + x) as ptrdiff_t + 8 as ptrdiff_t * stride)
                                                    as isize,
                                            ) as ::core::ffi::c_int
                                            - (d * 7 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                                *dst
                                    .offset(
                                        ((offset + x) as ptrdiff_t + 9 as ptrdiff_t * stride)
                                            as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + x) as ptrdiff_t + 9 as ptrdiff_t * stride)
                                                    as isize,
                                            ) as ::core::ffi::c_int
                                            - (d * 5 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                                *dst
                                    .offset(
                                        ((offset + x) as ptrdiff_t + 10 as ptrdiff_t * stride)
                                            as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + x) as ptrdiff_t + 10 as ptrdiff_t * stride)
                                                    as isize,
                                            ) as ::core::ffi::c_int
                                            - (d * 3 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                                *dst
                                    .offset(
                                        ((offset + x) as ptrdiff_t + 11 as ptrdiff_t * stride)
                                            as isize,
                                    ) = *cm
                                    .offset(
                                        (*dst
                                            .offset(
                                                ((offset + x) as ptrdiff_t + 11 as ptrdiff_t * stride)
                                                    as isize,
                                            ) as ::core::ffi::c_int
                                            - (d * 1 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                            as isize,
                                    );
                            }
                        }
                        x += 1;
                    }
                }
            }
            b_x += 1;
        }
        b_y += 1;
    }
}
#[inline(always)]
unsafe extern "C" fn add_blocklist(
    mut blocklist: *mut [::core::ffi::c_int; 2],
    mut blocklist_length: *mut ::core::ffi::c_int,
    mut fixed: *mut uint8_t,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut mb_xy: ::core::ffi::c_int,
) {
    if *fixed.offset(mb_xy as isize) != 0 {
        return;
    }
    *fixed.offset(mb_xy as isize) = 1 as uint8_t;
    (*blocklist.offset(*blocklist_length as isize))[0 as ::core::ffi::c_int as usize] = mb_x;
    let fresh6 = *blocklist_length;
    *blocklist_length = *blocklist_length + 1;
    (*blocklist.offset(fresh6 as isize))[1 as ::core::ffi::c_int as usize] = mb_y;
}
unsafe extern "C" fn guess_mv(mut s: *mut ERContext) {
    let mut blocklist: *mut [::core::ffi::c_int; 2] = ::core::ptr::null_mut::<
        [::core::ffi::c_int; 2],
    >();
    let mut next_blocklist: *mut [::core::ffi::c_int; 2] = ::core::ptr::null_mut::<
        [::core::ffi::c_int; 2],
    >();
    let mut fixed: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mb_stride: ptrdiff_t = (*s).mb_stride;
    let mb_width: ::core::ffi::c_int = (*s).mb_width;
    let mut mb_height: ::core::ffi::c_int = (*s).mb_height;
    let mut i: ::core::ffi::c_int = 0;
    let mut num_avail: ::core::ffi::c_int = 0;
    let mut mb_x: ::core::ffi::c_int = 0;
    let mut mb_y: ::core::ffi::c_int = 0;
    let mut mot_step: ptrdiff_t = 0;
    let mut mot_stride: ptrdiff_t = 0;
    let mut blocklist_length: ::core::ffi::c_int = 0;
    let mut next_blocklist_length: ::core::ffi::c_int = 0;
    if !(*s).last_pic.f.is_null()
        && !(*(*s).last_pic.f).data[0 as ::core::ffi::c_int as usize].is_null()
    {
        mb_height = if mb_height
            > (*(*s).last_pic.f).height + 15 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int
        {
            (*(*s).last_pic.f).height + 15 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int
        } else {
            mb_height
        };
    }
    if !(*s).next_pic.f.is_null()
        && !(*(*s).next_pic.f).data[0 as ::core::ffi::c_int as usize].is_null()
    {
        mb_height = if mb_height
            > (*(*s).next_pic.f).height + 15 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int
        {
            (*(*s).next_pic.f).height + 15 as ::core::ffi::c_int
                >> 4 as ::core::ffi::c_int
        } else {
            mb_height
        };
    }
    blocklist = (*s).er_temp_buffer as *mut [::core::ffi::c_int; 2];
    next_blocklist = blocklist
        .offset(((*s).mb_stride * (*s).mb_height as ptrdiff_t) as isize);
    fixed = next_blocklist
        .offset(((*s).mb_stride * (*s).mb_height as ptrdiff_t) as isize) as *mut uint8_t;
    set_mv_strides(s, &raw mut mot_step, &raw mut mot_stride);
    num_avail = 0 as ::core::ffi::c_int;
    if !(*s).last_pic.motion_val[0 as ::core::ffi::c_int as usize].is_null() {
        if !(*s).last_pic.tf.is_null() {
            ff_thread_await_progress(
                (*s).last_pic.tf as *const ThreadFrame,
                mb_height - 1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        } else {
            ff_thread_progress_await(
                (*s).last_pic.progress as *const ThreadProgress,
                mb_height - 1 as ::core::ffi::c_int,
            );
        }
    }
    i = 0 as ::core::ffi::c_int;
    while i < mb_width * mb_height {
        let mb_xy: ::core::ffi::c_int = *(*s).mb_index2xy.offset(i as isize);
        let mut f: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut error: ::core::ffi::c_int = *(*s)
            .error_status_table
            .offset(mb_xy as isize) as ::core::ffi::c_int;
        if *(*s).cur_pic.mb_type.offset(mb_xy as isize) & 7 as uint32_t != 0 {
            f = 8 as ::core::ffi::c_int;
        }
        if error & 8 as ::core::ffi::c_int == 0 {
            f = 8 as ::core::ffi::c_int;
        }
        *fixed.offset(mb_xy as isize) = f as uint8_t;
        if f == 8 as ::core::ffi::c_int {
            num_avail += 1;
        } else if !(*(*s).last_pic.f).data[0 as ::core::ffi::c_int as usize].is_null()
            && !(*s).last_pic.motion_val[0 as ::core::ffi::c_int as usize].is_null()
        {
            let mb_y_0: ::core::ffi::c_int = (mb_xy as ptrdiff_t / (*s).mb_stride)
                as ::core::ffi::c_int;
            let mb_x_0: ::core::ffi::c_int = (mb_xy as ptrdiff_t % (*s).mb_stride)
                as ::core::ffi::c_int;
            let mot_index: ::core::ffi::c_int = ((mb_x_0 as ptrdiff_t
                + mb_y_0 as ptrdiff_t * mot_stride) * mot_step) as ::core::ffi::c_int;
            (*(*s)
                .cur_pic
                .motion_val[0 as ::core::ffi::c_int as usize]
                .offset(mot_index as isize))[0 as ::core::ffi::c_int as usize] = (*(*s)
                .last_pic
                .motion_val[0 as ::core::ffi::c_int as usize]
                .offset(mot_index as isize))[0 as ::core::ffi::c_int as usize];
            (*(*s)
                .cur_pic
                .motion_val[0 as ::core::ffi::c_int as usize]
                .offset(mot_index as isize))[1 as ::core::ffi::c_int as usize] = (*(*s)
                .last_pic
                .motion_val[0 as ::core::ffi::c_int as usize]
                .offset(mot_index as isize))[1 as ::core::ffi::c_int as usize];
            *(*s)
                .cur_pic
                .ref_index[0 as ::core::ffi::c_int as usize]
                .offset((4 as ::core::ffi::c_int * mb_xy) as isize) = *(*s)
                .last_pic
                .ref_index[0 as ::core::ffi::c_int as usize]
                .offset((4 as ::core::ffi::c_int * mb_xy) as isize);
        }
        i += 1;
    }
    if (*(*s).avctx).error_concealment & 1 as ::core::ffi::c_int == 0
        || num_avail
            <= (if mb_width > mb_height { mb_width } else { mb_height })
                / 2 as ::core::ffi::c_int
    {
        mb_y = 0 as ::core::ffi::c_int;
        while mb_y < mb_height {
            mb_x = 0 as ::core::ffi::c_int;
            while mb_x < (*s).mb_width {
                let mb_xy_0: ::core::ffi::c_int = (mb_x as ptrdiff_t
                    + mb_y as ptrdiff_t * (*s).mb_stride) as ::core::ffi::c_int;
                let mut mv_dir: ::core::ffi::c_int = if !(*s).last_pic.f.is_null()
                    && !(*(*s).last_pic.f)
                        .data[0 as ::core::ffi::c_int as usize]
                        .is_null()
                {
                    1 as ::core::ffi::c_int
                } else {
                    2 as ::core::ffi::c_int
                };
                if !(*(*s).cur_pic.mb_type.offset(mb_xy_0 as isize) & 7 as uint32_t != 0)
                {
                    if !(*(*s).error_status_table.offset(mb_xy_0 as isize)
                        as ::core::ffi::c_int & 8 as ::core::ffi::c_int == 0)
                    {
                        (*s)
                            .mv[0 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int as usize] = 0
                            as ::core::ffi::c_int;
                        (*s)
                            .mv[0 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int
                            as usize][1 as ::core::ffi::c_int as usize] = 0
                            as ::core::ffi::c_int;
                        (*s)
                            .decode_mb
                            .expect(
                                "non-null function pointer",
                            )(
                            (*s).opaque,
                            0 as ::core::ffi::c_int,
                            mv_dir,
                            0 as ::core::ffi::c_int,
                            &raw mut (*s).mv,
                            mb_x,
                            mb_y,
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                        );
                    }
                }
                mb_x += 1;
            }
            mb_y += 1;
        }
        return;
    }
    blocklist_length = 0 as ::core::ffi::c_int;
    mb_y = 0 as ::core::ffi::c_int;
    while mb_y < mb_height {
        mb_x = 0 as ::core::ffi::c_int;
        while mb_x < mb_width {
            let mb_xy_1: ::core::ffi::c_int = (mb_x as ptrdiff_t
                + mb_y as ptrdiff_t * mb_stride) as ::core::ffi::c_int;
            if *fixed.offset(mb_xy_1 as isize) as ::core::ffi::c_int
                == 8 as ::core::ffi::c_int
            {
                if mb_x != 0 {
                    add_blocklist(
                        blocklist,
                        &raw mut blocklist_length,
                        fixed,
                        mb_x - 1 as ::core::ffi::c_int,
                        mb_y,
                        mb_xy_1 - 1 as ::core::ffi::c_int,
                    );
                }
                if mb_y != 0 {
                    add_blocklist(
                        blocklist,
                        &raw mut blocklist_length,
                        fixed,
                        mb_x,
                        mb_y - 1 as ::core::ffi::c_int,
                        (mb_xy_1 as ptrdiff_t - mb_stride) as ::core::ffi::c_int,
                    );
                }
                if (mb_x + 1 as ::core::ffi::c_int) < mb_width {
                    add_blocklist(
                        blocklist,
                        &raw mut blocklist_length,
                        fixed,
                        mb_x + 1 as ::core::ffi::c_int,
                        mb_y,
                        mb_xy_1 + 1 as ::core::ffi::c_int,
                    );
                }
                if (mb_y + 1 as ::core::ffi::c_int) < mb_height {
                    add_blocklist(
                        blocklist,
                        &raw mut blocklist_length,
                        fixed,
                        mb_x,
                        mb_y + 1 as ::core::ffi::c_int,
                        (mb_xy_1 as ptrdiff_t + mb_stride) as ::core::ffi::c_int,
                    );
                }
            }
            mb_x += 1;
        }
        mb_y += 1;
    }
    loop {
        let mut changed: ::core::ffi::c_int = 0;
        let mut pass: ::core::ffi::c_int = 0;
        let mut none_left: ::core::ffi::c_int = 0;
        let mut blocklist_index: ::core::ffi::c_int = 0;
        none_left = 1 as ::core::ffi::c_int;
        changed = 1 as ::core::ffi::c_int;
        pass = 0 as ::core::ffi::c_int;
        while (changed != 0 || pass < 2 as ::core::ffi::c_int)
            && pass < 10 as ::core::ffi::c_int
        {
            changed = 0 as ::core::ffi::c_int;
            let mut current_block_187: u64;
            blocklist_index = 0 as ::core::ffi::c_int;
            while blocklist_index < blocklist_length {
                let mb_x_1: ::core::ffi::c_int = (*blocklist
                    .offset(blocklist_index as isize))[0 as ::core::ffi::c_int as usize];
                let mb_y_1: ::core::ffi::c_int = (*blocklist
                    .offset(blocklist_index as isize))[1 as ::core::ffi::c_int as usize];
                let mb_xy_2: ::core::ffi::c_int = (mb_x_1 as ptrdiff_t
                    + mb_y_1 as ptrdiff_t * mb_stride) as ::core::ffi::c_int;
                let mut mv_predictor: [[::core::ffi::c_int; 2]; 8] = [[0; 2]; 8];
                let mut ref_0: [::core::ffi::c_int; 8] = [0; 8];
                let mut pred_count: ::core::ffi::c_int = 0;
                let mut j: ::core::ffi::c_int = 0;
                let mut best_score: ::core::ffi::c_int = 0;
                let mut best_pred: ::core::ffi::c_int = 0;
                let mut mot_index_0: ::core::ffi::c_int = 0;
                let mut prev_x: ::core::ffi::c_int = 0;
                let mut prev_y: ::core::ffi::c_int = 0;
                let mut prev_ref: ::core::ffi::c_int = 0;
                if !((mb_x_1 ^ mb_y_1 ^ pass) & 1 as ::core::ffi::c_int != 0) {
                    j = 0 as ::core::ffi::c_int;
                    if mb_x_1 > 0 as ::core::ffi::c_int {
                        j
                            |= *fixed
                                .offset((mb_xy_2 - 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int;
                    }
                    if (mb_x_1 + 1 as ::core::ffi::c_int) < mb_width {
                        j
                            |= *fixed
                                .offset((mb_xy_2 + 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int;
                    }
                    if mb_y_1 > 0 as ::core::ffi::c_int {
                        j
                            |= *fixed.offset((mb_xy_2 as ptrdiff_t - mb_stride) as isize)
                                as ::core::ffi::c_int;
                    }
                    if (mb_y_1 + 1 as ::core::ffi::c_int) < mb_height {
                        j
                            |= *fixed.offset((mb_xy_2 as ptrdiff_t + mb_stride) as isize)
                                as ::core::ffi::c_int;
                    }
                    if !(j & 4 as ::core::ffi::c_int == 0
                        && pass > 1 as ::core::ffi::c_int)
                    {
                        none_left = 0 as ::core::ffi::c_int;
                        pred_count = 0 as ::core::ffi::c_int;
                        mot_index_0 = ((mb_x_1 as ptrdiff_t
                            + mb_y_1 as ptrdiff_t * mot_stride) * mot_step)
                            as ::core::ffi::c_int;
                        if mb_x_1 > 0 as ::core::ffi::c_int
                            && *fixed
                                .offset((mb_xy_2 - 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                        {
                            mv_predictor[pred_count
                                as usize][0 as ::core::ffi::c_int as usize] = (*(*s)
                                .cur_pic
                                .motion_val[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    (mot_index_0 as ptrdiff_t - mot_step) as isize,
                                ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                            mv_predictor[pred_count
                                as usize][1 as ::core::ffi::c_int as usize] = (*(*s)
                                .cur_pic
                                .motion_val[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    (mot_index_0 as ptrdiff_t - mot_step) as isize,
                                ))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                            ref_0[pred_count as usize] = *(*s)
                                .cur_pic
                                .ref_index[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    (4 as ::core::ffi::c_int
                                        * (mb_xy_2 - 1 as ::core::ffi::c_int)) as isize,
                                ) as ::core::ffi::c_int;
                            pred_count += 1;
                        }
                        if (mb_x_1 + 1 as ::core::ffi::c_int) < mb_width
                            && *fixed
                                .offset((mb_xy_2 + 1 as ::core::ffi::c_int) as isize)
                                as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                        {
                            mv_predictor[pred_count
                                as usize][0 as ::core::ffi::c_int as usize] = (*(*s)
                                .cur_pic
                                .motion_val[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    (mot_index_0 as ptrdiff_t + mot_step) as isize,
                                ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                            mv_predictor[pred_count
                                as usize][1 as ::core::ffi::c_int as usize] = (*(*s)
                                .cur_pic
                                .motion_val[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    (mot_index_0 as ptrdiff_t + mot_step) as isize,
                                ))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                            ref_0[pred_count as usize] = *(*s)
                                .cur_pic
                                .ref_index[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    (4 as ::core::ffi::c_int
                                        * (mb_xy_2 + 1 as ::core::ffi::c_int)) as isize,
                                ) as ::core::ffi::c_int;
                            pred_count += 1;
                        }
                        if mb_y_1 > 0 as ::core::ffi::c_int
                            && *fixed.offset((mb_xy_2 as ptrdiff_t - mb_stride) as isize)
                                as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                        {
                            mv_predictor[pred_count
                                as usize][0 as ::core::ffi::c_int as usize] = (*(*s)
                                .cur_pic
                                .motion_val[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    (mot_index_0 as ptrdiff_t - mot_stride * mot_step) as isize,
                                ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                            mv_predictor[pred_count
                                as usize][1 as ::core::ffi::c_int as usize] = (*(*s)
                                .cur_pic
                                .motion_val[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    (mot_index_0 as ptrdiff_t - mot_stride * mot_step) as isize,
                                ))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                            ref_0[pred_count as usize] = *(*s)
                                .cur_pic
                                .ref_index[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    (4 as ptrdiff_t * (mb_xy_2 as ptrdiff_t - (*s).mb_stride))
                                        as isize,
                                ) as ::core::ffi::c_int;
                            pred_count += 1;
                        }
                        if (mb_y_1 + 1 as ::core::ffi::c_int) < mb_height
                            && *fixed.offset((mb_xy_2 as ptrdiff_t + mb_stride) as isize)
                                as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                        {
                            mv_predictor[pred_count
                                as usize][0 as ::core::ffi::c_int as usize] = (*(*s)
                                .cur_pic
                                .motion_val[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    (mot_index_0 as ptrdiff_t + mot_stride * mot_step) as isize,
                                ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                            mv_predictor[pred_count
                                as usize][1 as ::core::ffi::c_int as usize] = (*(*s)
                                .cur_pic
                                .motion_val[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    (mot_index_0 as ptrdiff_t + mot_stride * mot_step) as isize,
                                ))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                            ref_0[pred_count as usize] = *(*s)
                                .cur_pic
                                .ref_index[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    (4 as ptrdiff_t * (mb_xy_2 as ptrdiff_t + (*s).mb_stride))
                                        as isize,
                                ) as ::core::ffi::c_int;
                            pred_count += 1;
                        }
                        if !(pred_count == 0 as ::core::ffi::c_int) {
                            if pred_count > 1 as ::core::ffi::c_int {
                                let mut sum_x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                let mut sum_y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                let mut sum_r: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                let mut max_x: ::core::ffi::c_int = 0;
                                let mut max_y: ::core::ffi::c_int = 0;
                                let mut min_x: ::core::ffi::c_int = 0;
                                let mut min_y: ::core::ffi::c_int = 0;
                                let mut max_r: ::core::ffi::c_int = 0;
                                let mut min_r: ::core::ffi::c_int = 0;
                                j = 0 as ::core::ffi::c_int;
                                loop {
                                    if !(j < pred_count) {
                                        current_block_187 = 14155412868135599428;
                                        break;
                                    }
                                    sum_x
                                        += mv_predictor[j
                                            as usize][0 as ::core::ffi::c_int as usize];
                                    sum_y
                                        += mv_predictor[j
                                            as usize][1 as ::core::ffi::c_int as usize];
                                    sum_r += ref_0[j as usize];
                                    if j != 0
                                        && ref_0[j as usize]
                                            != ref_0[(j - 1 as ::core::ffi::c_int) as usize]
                                    {
                                        current_block_187 = 2493844787405816816;
                                        break;
                                    }
                                    j += 1;
                                }
                                match current_block_187 {
                                    2493844787405816816 => {}
                                    _ => {
                                        mv_predictor[pred_count
                                            as usize][0 as ::core::ffi::c_int as usize] = sum_x / j;
                                        mv_predictor[pred_count
                                            as usize][1 as ::core::ffi::c_int as usize] = sum_y / j;
                                        ref_0[pred_count as usize] = sum_r / j;
                                        if pred_count >= 3 as ::core::ffi::c_int {
                                            min_r = 99999 as ::core::ffi::c_int;
                                            min_x = min_r;
                                            min_y = min_x;
                                            max_r = -(99999 as ::core::ffi::c_int);
                                            max_x = max_r;
                                            max_y = max_x;
                                        } else {
                                            max_r = 0 as ::core::ffi::c_int;
                                            min_r = max_r;
                                            max_y = min_r;
                                            max_x = max_y;
                                            min_y = max_x;
                                            min_x = min_y;
                                        }
                                        j = 0 as ::core::ffi::c_int;
                                        while j < pred_count {
                                            max_x = if max_x
                                                > mv_predictor[j as usize][0 as ::core::ffi::c_int as usize]
                                            {
                                                max_x
                                            } else {
                                                mv_predictor[j as usize][0 as ::core::ffi::c_int as usize]
                                            };
                                            max_y = if max_y
                                                > mv_predictor[j as usize][1 as ::core::ffi::c_int as usize]
                                            {
                                                max_y
                                            } else {
                                                mv_predictor[j as usize][1 as ::core::ffi::c_int as usize]
                                            };
                                            max_r = if max_r > ref_0[j as usize] {
                                                max_r
                                            } else {
                                                ref_0[j as usize]
                                            };
                                            min_x = if min_x
                                                > mv_predictor[j as usize][0 as ::core::ffi::c_int as usize]
                                            {
                                                mv_predictor[j as usize][0 as ::core::ffi::c_int as usize]
                                            } else {
                                                min_x
                                            };
                                            min_y = if min_y
                                                > mv_predictor[j as usize][1 as ::core::ffi::c_int as usize]
                                            {
                                                mv_predictor[j as usize][1 as ::core::ffi::c_int as usize]
                                            } else {
                                                min_y
                                            };
                                            min_r = if min_r > ref_0[j as usize] {
                                                ref_0[j as usize]
                                            } else {
                                                min_r
                                            };
                                            j += 1;
                                        }
                                        mv_predictor[(pred_count + 1 as ::core::ffi::c_int)
                                            as usize][0 as ::core::ffi::c_int as usize] = sum_x - max_x
                                            - min_x;
                                        mv_predictor[(pred_count + 1 as ::core::ffi::c_int)
                                            as usize][1 as ::core::ffi::c_int as usize] = sum_y - max_y
                                            - min_y;
                                        ref_0[(pred_count + 1 as ::core::ffi::c_int) as usize] = sum_r
                                            - max_r - min_r;
                                        if pred_count == 4 as ::core::ffi::c_int {
                                            mv_predictor[(pred_count + 1 as ::core::ffi::c_int)
                                                as usize][0 as ::core::ffi::c_int as usize]
                                                /= 2 as ::core::ffi::c_int;
                                            mv_predictor[(pred_count + 1 as ::core::ffi::c_int)
                                                as usize][1 as ::core::ffi::c_int as usize]
                                                /= 2 as ::core::ffi::c_int;
                                            ref_0[(pred_count + 1 as ::core::ffi::c_int) as usize]
                                                /= 2 as ::core::ffi::c_int;
                                        }
                                        pred_count += 2 as ::core::ffi::c_int;
                                    }
                                }
                            }
                            ref_0[pred_count as usize] = 0 as ::core::ffi::c_int;
                            mv_predictor[pred_count
                                as usize][1 as ::core::ffi::c_int as usize] = ref_0[pred_count
                                as usize];
                            mv_predictor[pred_count
                                as usize][0 as ::core::ffi::c_int as usize] = mv_predictor[pred_count
                                as usize][1 as ::core::ffi::c_int as usize];
                            pred_count += 1;
                            prev_x = (*(*s)
                                .cur_pic
                                .motion_val[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    mot_index_0 as isize,
                                ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                            prev_y = (*(*s)
                                .cur_pic
                                .motion_val[0 as ::core::ffi::c_int as usize]
                                .offset(
                                    mot_index_0 as isize,
                                ))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                            prev_ref = *(*s)
                                .cur_pic
                                .ref_index[0 as ::core::ffi::c_int as usize]
                                .offset((4 as ::core::ffi::c_int * mb_xy_2) as isize)
                                as ::core::ffi::c_int;
                            mv_predictor[pred_count
                                as usize][0 as ::core::ffi::c_int as usize] = prev_x;
                            mv_predictor[pred_count
                                as usize][1 as ::core::ffi::c_int as usize] = prev_y;
                            ref_0[pred_count as usize] = prev_ref;
                            pred_count += 1;
                            best_pred = 0 as ::core::ffi::c_int;
                            best_score = 256 as ::core::ffi::c_int
                                * 256 as ::core::ffi::c_int * 256 as ::core::ffi::c_int
                                * 64 as ::core::ffi::c_int;
                            j = 0 as ::core::ffi::c_int;
                            while j < pred_count {
                                let mut linesize: *mut ::core::ffi::c_int = &raw mut (*(*s)
                                    .cur_pic
                                    .f)
                                    .linesize as *mut ::core::ffi::c_int;
                                let mut score: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                                let mut src: *mut uint8_t = (*(*s).cur_pic.f)
                                    .data[0 as ::core::ffi::c_int as usize]
                                    .offset((mb_x_1 * 16 as ::core::ffi::c_int) as isize)
                                    .offset(
                                        (mb_y_1 * 16 as ::core::ffi::c_int
                                            * *linesize.offset(0 as ::core::ffi::c_int as isize))
                                            as isize,
                                    );
                                (*s)
                                    .mv[0 as ::core::ffi::c_int
                                    as usize][0 as ::core::ffi::c_int
                                    as usize][0 as ::core::ffi::c_int as usize] = mv_predictor[j
                                    as usize][0 as ::core::ffi::c_int as usize];
                                (*(*s)
                                    .cur_pic
                                    .motion_val[0 as ::core::ffi::c_int as usize]
                                    .offset(
                                        mot_index_0 as isize,
                                    ))[0 as ::core::ffi::c_int as usize] = (*s)
                                    .mv[0 as ::core::ffi::c_int
                                    as usize][0 as ::core::ffi::c_int
                                    as usize][0 as ::core::ffi::c_int as usize] as int16_t;
                                (*s)
                                    .mv[0 as ::core::ffi::c_int
                                    as usize][0 as ::core::ffi::c_int
                                    as usize][1 as ::core::ffi::c_int as usize] = mv_predictor[j
                                    as usize][1 as ::core::ffi::c_int as usize];
                                (*(*s)
                                    .cur_pic
                                    .motion_val[0 as ::core::ffi::c_int as usize]
                                    .offset(
                                        mot_index_0 as isize,
                                    ))[1 as ::core::ffi::c_int as usize] = (*s)
                                    .mv[0 as ::core::ffi::c_int
                                    as usize][0 as ::core::ffi::c_int
                                    as usize][1 as ::core::ffi::c_int as usize] as int16_t;
                                if !(ref_0[j as usize] < 0 as ::core::ffi::c_int) {
                                    (*s)
                                        .decode_mb
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        (*s).opaque,
                                        ref_0[j as usize],
                                        1 as ::core::ffi::c_int,
                                        0 as ::core::ffi::c_int,
                                        &raw mut (*s).mv,
                                        mb_x_1,
                                        mb_y_1,
                                        0 as ::core::ffi::c_int,
                                        0 as ::core::ffi::c_int,
                                    );
                                    if mb_x_1 > 0 as ::core::ffi::c_int
                                        && *fixed
                                            .offset((mb_xy_2 - 1 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                                    {
                                        let mut k: ::core::ffi::c_int = 0;
                                        k = 0 as ::core::ffi::c_int;
                                        while k < 16 as ::core::ffi::c_int {
                                            score
                                                += if *src
                                                    .offset(
                                                        (k * *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                            - 1 as ::core::ffi::c_int) as isize,
                                                    ) as ::core::ffi::c_int
                                                    - *src
                                                        .offset(
                                                            (k * *linesize.offset(0 as ::core::ffi::c_int as isize))
                                                                as isize,
                                                        ) as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                                                {
                                                    *src
                                                        .offset(
                                                            (k * *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                                - 1 as ::core::ffi::c_int) as isize,
                                                        ) as ::core::ffi::c_int
                                                        - *src
                                                            .offset(
                                                                (k * *linesize.offset(0 as ::core::ffi::c_int as isize))
                                                                    as isize,
                                                            ) as ::core::ffi::c_int
                                                } else {
                                                    -(*src
                                                        .offset(
                                                            (k * *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                                - 1 as ::core::ffi::c_int) as isize,
                                                        ) as ::core::ffi::c_int
                                                        - *src
                                                            .offset(
                                                                (k * *linesize.offset(0 as ::core::ffi::c_int as isize))
                                                                    as isize,
                                                            ) as ::core::ffi::c_int)
                                                };
                                            k += 1;
                                        }
                                    }
                                    if (mb_x_1 + 1 as ::core::ffi::c_int) < mb_width
                                        && *fixed
                                            .offset((mb_xy_2 + 1 as ::core::ffi::c_int) as isize)
                                            as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                                    {
                                        let mut k_0: ::core::ffi::c_int = 0;
                                        k_0 = 0 as ::core::ffi::c_int;
                                        while k_0 < 16 as ::core::ffi::c_int {
                                            score
                                                += if *src
                                                    .offset(
                                                        (k_0 * *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                            + 15 as ::core::ffi::c_int) as isize,
                                                    ) as ::core::ffi::c_int
                                                    - *src
                                                        .offset(
                                                            (k_0 * *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                                + 16 as ::core::ffi::c_int) as isize,
                                                        ) as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                                                {
                                                    *src
                                                        .offset(
                                                            (k_0 * *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                                + 15 as ::core::ffi::c_int) as isize,
                                                        ) as ::core::ffi::c_int
                                                        - *src
                                                            .offset(
                                                                (k_0 * *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                                    + 16 as ::core::ffi::c_int) as isize,
                                                            ) as ::core::ffi::c_int
                                                } else {
                                                    -(*src
                                                        .offset(
                                                            (k_0 * *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                                + 15 as ::core::ffi::c_int) as isize,
                                                        ) as ::core::ffi::c_int
                                                        - *src
                                                            .offset(
                                                                (k_0 * *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                                    + 16 as ::core::ffi::c_int) as isize,
                                                            ) as ::core::ffi::c_int)
                                                };
                                            k_0 += 1;
                                        }
                                    }
                                    if mb_y_1 > 0 as ::core::ffi::c_int
                                        && *fixed
                                            .offset((mb_xy_2 as ptrdiff_t - mb_stride) as isize)
                                            as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                                    {
                                        let mut k_1: ::core::ffi::c_int = 0;
                                        k_1 = 0 as ::core::ffi::c_int;
                                        while k_1 < 16 as ::core::ffi::c_int {
                                            score
                                                += if *src
                                                    .offset(
                                                        (k_1 - *linesize.offset(0 as ::core::ffi::c_int as isize))
                                                            as isize,
                                                    ) as ::core::ffi::c_int
                                                    - *src.offset(k_1 as isize) as ::core::ffi::c_int
                                                    >= 0 as ::core::ffi::c_int
                                                {
                                                    *src
                                                        .offset(
                                                            (k_1 - *linesize.offset(0 as ::core::ffi::c_int as isize))
                                                                as isize,
                                                        ) as ::core::ffi::c_int
                                                        - *src.offset(k_1 as isize) as ::core::ffi::c_int
                                                } else {
                                                    -(*src
                                                        .offset(
                                                            (k_1 - *linesize.offset(0 as ::core::ffi::c_int as isize))
                                                                as isize,
                                                        ) as ::core::ffi::c_int
                                                        - *src.offset(k_1 as isize) as ::core::ffi::c_int)
                                                };
                                            k_1 += 1;
                                        }
                                    }
                                    if (mb_y_1 + 1 as ::core::ffi::c_int) < mb_height
                                        && *fixed
                                            .offset((mb_xy_2 as ptrdiff_t + mb_stride) as isize)
                                            as ::core::ffi::c_int > 1 as ::core::ffi::c_int
                                    {
                                        let mut k_2: ::core::ffi::c_int = 0;
                                        k_2 = 0 as ::core::ffi::c_int;
                                        while k_2 < 16 as ::core::ffi::c_int {
                                            score
                                                += if *src
                                                    .offset(
                                                        (k_2
                                                            + *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                                * 15 as ::core::ffi::c_int) as isize,
                                                    ) as ::core::ffi::c_int
                                                    - *src
                                                        .offset(
                                                            (k_2
                                                                + *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                                    * 16 as ::core::ffi::c_int) as isize,
                                                        ) as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                                                {
                                                    *src
                                                        .offset(
                                                            (k_2
                                                                + *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                                    * 15 as ::core::ffi::c_int) as isize,
                                                        ) as ::core::ffi::c_int
                                                        - *src
                                                            .offset(
                                                                (k_2
                                                                    + *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                                        * 16 as ::core::ffi::c_int) as isize,
                                                            ) as ::core::ffi::c_int
                                                } else {
                                                    -(*src
                                                        .offset(
                                                            (k_2
                                                                + *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                                    * 15 as ::core::ffi::c_int) as isize,
                                                        ) as ::core::ffi::c_int
                                                        - *src
                                                            .offset(
                                                                (k_2
                                                                    + *linesize.offset(0 as ::core::ffi::c_int as isize)
                                                                        * 16 as ::core::ffi::c_int) as isize,
                                                            ) as ::core::ffi::c_int)
                                                };
                                            k_2 += 1;
                                        }
                                    }
                                    if score <= best_score {
                                        best_score = score;
                                        best_pred = j;
                                    }
                                }
                                j += 1;
                            }
                            (*s)
                                .mv[0 as ::core::ffi::c_int
                                as usize][0 as ::core::ffi::c_int
                                as usize][0 as ::core::ffi::c_int as usize] = mv_predictor[best_pred
                                as usize][0 as ::core::ffi::c_int as usize];
                            (*s)
                                .mv[0 as ::core::ffi::c_int
                                as usize][0 as ::core::ffi::c_int
                                as usize][1 as ::core::ffi::c_int as usize] = mv_predictor[best_pred
                                as usize][1 as ::core::ffi::c_int as usize];
                            i = 0 as ::core::ffi::c_int;
                            while (i as ptrdiff_t) < mot_step {
                                j = 0 as ::core::ffi::c_int;
                                while (j as ptrdiff_t) < mot_step {
                                    (*(*s)
                                        .cur_pic
                                        .motion_val[0 as ::core::ffi::c_int as usize]
                                        .offset(
                                            ((mot_index_0 + i) as ptrdiff_t
                                                + j as ptrdiff_t * mot_stride) as isize,
                                        ))[0 as ::core::ffi::c_int as usize] = (*s)
                                        .mv[0 as ::core::ffi::c_int
                                        as usize][0 as ::core::ffi::c_int
                                        as usize][0 as ::core::ffi::c_int as usize] as int16_t;
                                    (*(*s)
                                        .cur_pic
                                        .motion_val[0 as ::core::ffi::c_int as usize]
                                        .offset(
                                            ((mot_index_0 + i) as ptrdiff_t
                                                + j as ptrdiff_t * mot_stride) as isize,
                                        ))[1 as ::core::ffi::c_int as usize] = (*s)
                                        .mv[0 as ::core::ffi::c_int
                                        as usize][0 as ::core::ffi::c_int
                                        as usize][1 as ::core::ffi::c_int as usize] as int16_t;
                                    j += 1;
                                }
                                i += 1;
                            }
                            (*s)
                                .decode_mb
                                .expect(
                                    "non-null function pointer",
                                )(
                                (*s).opaque,
                                ref_0[best_pred as usize],
                                1 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                &raw mut (*s).mv,
                                mb_x_1,
                                mb_y_1,
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                            );
                            if (*s)
                                .mv[0 as ::core::ffi::c_int
                                as usize][0 as ::core::ffi::c_int
                                as usize][0 as ::core::ffi::c_int as usize] != prev_x
                                || (*s)
                                    .mv[0 as ::core::ffi::c_int
                                    as usize][0 as ::core::ffi::c_int
                                    as usize][1 as ::core::ffi::c_int as usize] != prev_y
                            {
                                *fixed.offset(mb_xy_2 as isize) = 4 as uint8_t;
                                changed += 1;
                            } else {
                                *fixed.offset(mb_xy_2 as isize) = 2 as uint8_t;
                            }
                        }
                    }
                }
                blocklist_index += 1;
            }
            pass += 1;
        }
        if none_left != 0 {
            return;
        }
        next_blocklist_length = 0 as ::core::ffi::c_int;
        blocklist_index = 0 as ::core::ffi::c_int;
        while blocklist_index < blocklist_length {
            let mb_x_2: ::core::ffi::c_int = (*blocklist
                .offset(blocklist_index as isize))[0 as ::core::ffi::c_int as usize];
            let mb_y_2: ::core::ffi::c_int = (*blocklist
                .offset(blocklist_index as isize))[1 as ::core::ffi::c_int as usize];
            let mb_xy_3: ::core::ffi::c_int = (mb_x_2 as ptrdiff_t
                + mb_y_2 as ptrdiff_t * mb_stride) as ::core::ffi::c_int;
            if *fixed.offset(mb_xy_3 as isize) as ::core::ffi::c_int
                & (4 as ::core::ffi::c_int | 2 as ::core::ffi::c_int
                    | 8 as ::core::ffi::c_int) != 0
            {
                *fixed.offset(mb_xy_3 as isize) = 8 as uint8_t;
                if mb_x_2 > 0 as ::core::ffi::c_int {
                    add_blocklist(
                        next_blocklist,
                        &raw mut next_blocklist_length,
                        fixed,
                        mb_x_2 - 1 as ::core::ffi::c_int,
                        mb_y_2,
                        mb_xy_3 - 1 as ::core::ffi::c_int,
                    );
                }
                if mb_y_2 > 0 as ::core::ffi::c_int {
                    add_blocklist(
                        next_blocklist,
                        &raw mut next_blocklist_length,
                        fixed,
                        mb_x_2,
                        mb_y_2 - 1 as ::core::ffi::c_int,
                        (mb_xy_3 as ptrdiff_t - mb_stride) as ::core::ffi::c_int,
                    );
                }
                if (mb_x_2 + 1 as ::core::ffi::c_int) < mb_width {
                    add_blocklist(
                        next_blocklist,
                        &raw mut next_blocklist_length,
                        fixed,
                        mb_x_2 + 1 as ::core::ffi::c_int,
                        mb_y_2,
                        mb_xy_3 + 1 as ::core::ffi::c_int,
                    );
                }
                if (mb_y_2 + 1 as ::core::ffi::c_int) < mb_height {
                    add_blocklist(
                        next_blocklist,
                        &raw mut next_blocklist_length,
                        fixed,
                        mb_x_2,
                        mb_y_2 + 1 as ::core::ffi::c_int,
                        (mb_xy_3 as ptrdiff_t + mb_stride) as ::core::ffi::c_int,
                    );
                }
            }
            blocklist_index += 1;
        }
        if !(next_blocklist_length <= mb_height * mb_width) {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"next_blocklist_length <= mb_height * mb_width\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"src/libavcodec/error_resilience.c\0" as *const u8
                    as *const ::core::ffi::c_char,
                737 as ::core::ffi::c_int,
            );
            abort();
        }
        let mut SWAP_tmp: ::core::ffi::c_int = next_blocklist_length;
        next_blocklist_length = blocklist_length;
        blocklist_length = SWAP_tmp;
        let mut SWAP_tmp_0: *mut ::core::ffi::c_void = next_blocklist
            as *mut ::core::ffi::c_void;
        next_blocklist = blocklist;
        blocklist = SWAP_tmp_0 as *mut [::core::ffi::c_int; 2];
    };
}
unsafe extern "C" fn is_intra_more_likely(mut s: *mut ERContext) -> ::core::ffi::c_int {
    let mut is_intra_likely: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut undamaged_count: ::core::ffi::c_int = 0;
    let mut skip_amount: ::core::ffi::c_int = 0;
    let mut mb_x: ::core::ffi::c_int = 0;
    let mut mb_y: ::core::ffi::c_int = 0;
    if (*s).last_pic.f.is_null()
        || (*(*s).last_pic.f).data[0 as ::core::ffi::c_int as usize].is_null()
    {
        return 1 as ::core::ffi::c_int;
    }
    if (*(*s).avctx).error_concealment & 256 as ::core::ffi::c_int != 0 {
        return 0 as ::core::ffi::c_int;
    }
    undamaged_count = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*s).mb_num {
        let mb_xy: ::core::ffi::c_int = *(*s).mb_index2xy.offset(i as isize);
        let error: ::core::ffi::c_int = *(*s).error_status_table.offset(mb_xy as isize)
            as ::core::ffi::c_int;
        if !(error & 4 as ::core::ffi::c_int != 0
            && error & 8 as ::core::ffi::c_int != 0)
        {
            undamaged_count += 1;
        }
        i += 1;
    }
    if undamaged_count < 5 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    skip_amount = if undamaged_count / 50 as ::core::ffi::c_int > 1 as ::core::ffi::c_int
    {
        undamaged_count / 50 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    is_intra_likely = 0 as ::core::ffi::c_int;
    j = 0 as ::core::ffi::c_int;
    mb_y = 0 as ::core::ffi::c_int;
    while mb_y < (*s).mb_height - 1 as ::core::ffi::c_int {
        mb_x = 0 as ::core::ffi::c_int;
        while mb_x < (*s).mb_width {
            let mut error_0: ::core::ffi::c_int = 0;
            let mb_xy_0: ::core::ffi::c_int = (mb_x as ptrdiff_t
                + mb_y as ptrdiff_t * (*s).mb_stride) as ::core::ffi::c_int;
            error_0 = *(*s).error_status_table.offset(mb_xy_0 as isize)
                as ::core::ffi::c_int;
            if !(error_0 & 4 as ::core::ffi::c_int != 0
                && error_0 & 8 as ::core::ffi::c_int != 0)
            {
                j += 1;
                if !(j % skip_amount != 0 as ::core::ffi::c_int) {
                    if (*(*s).cur_pic.f).pict_type as ::core::ffi::c_uint
                        == AV_PICTURE_TYPE_I as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        let mut linesize: *mut ::core::ffi::c_int = &raw mut (*(*s)
                            .cur_pic
                            .f)
                            .linesize as *mut ::core::ffi::c_int;
                        let mut mb_ptr: *mut uint8_t = (*(*s).cur_pic.f)
                            .data[0 as ::core::ffi::c_int as usize]
                            .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                            .offset(
                                (mb_y * 16 as ::core::ffi::c_int
                                    * *linesize.offset(0 as ::core::ffi::c_int as isize))
                                    as isize,
                            );
                        let mut last_mb_ptr: *mut uint8_t = (*(*s).last_pic.f)
                            .data[0 as ::core::ffi::c_int as usize]
                            .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                            .offset(
                                (mb_y * 16 as ::core::ffi::c_int
                                    * *linesize.offset(0 as ::core::ffi::c_int as isize))
                                    as isize,
                            );
                        if !((*(*s).avctx).codec_id as ::core::ffi::c_uint
                            == AV_CODEC_ID_H264 as ::core::ffi::c_int
                                as ::core::ffi::c_uint)
                        {
                            ff_thread_progress_await(
                                (*s).last_pic.progress as *const ThreadProgress,
                                mb_y,
                            );
                        }
                        is_intra_likely
                            += (*s)
                                .sad
                                .expect(
                                    "non-null function pointer",
                                )(
                                ::core::ptr::null_mut::<MPVEncContext>(),
                                last_mb_ptr,
                                mb_ptr,
                                *linesize.offset(0 as ::core::ffi::c_int as isize)
                                    as ptrdiff_t,
                                16 as ::core::ffi::c_int,
                            );
                        is_intra_likely
                            -= (*s)
                                .sad
                                .expect(
                                    "non-null function pointer",
                                )(
                                ::core::ptr::null_mut::<MPVEncContext>(),
                                last_mb_ptr,
                                last_mb_ptr
                                    .offset(
                                        (*linesize.offset(0 as ::core::ffi::c_int as isize)
                                            * 16 as ::core::ffi::c_int) as isize,
                                    ),
                                *linesize.offset(0 as ::core::ffi::c_int as isize)
                                    as ptrdiff_t,
                                16 as ::core::ffi::c_int,
                            );
                    } else if *(*s).cur_pic.mb_type.offset(mb_xy_0 as isize)
                        & 7 as uint32_t != 0
                    {
                        is_intra_likely += 1;
                    } else {
                        is_intra_likely -= 1;
                    }
                }
            }
            mb_x += 1;
        }
        mb_y += 1;
    }
    return (is_intra_likely > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_er_frame_start(mut s: *mut ERContext) {
    if (*(*s).avctx).error_concealment == 0 {
        return;
    }
    memset(
        (*s).error_status_table as *mut ::core::ffi::c_void,
        2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int | 8 as ::core::ffi::c_int
            | 1 as ::core::ffi::c_int
            | (16 as ::core::ffi::c_int | 32 as ::core::ffi::c_int
                | 64 as ::core::ffi::c_int),
        (((*s).mb_stride * (*s).mb_height as ptrdiff_t) as size_t)
            .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
    );
    *&raw mut (*s).error_count = 3 as ::core::ffi::c_int * (*s).mb_num;
    (*s).error_occurred = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn er_supported(mut s: *mut ERContext) -> ::core::ffi::c_int {
    if !(*(*s).avctx).hwaccel.is_null() || (*s).cur_pic.f.is_null()
        || (*s).cur_pic.field_picture != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_er_add_slice(
    mut s: *mut ERContext,
    mut startx: ::core::ffi::c_int,
    mut starty: ::core::ffi::c_int,
    mut endx: ::core::ffi::c_int,
    mut endy: ::core::ffi::c_int,
    mut status: ::core::ffi::c_int,
) {
    let start_i: ::core::ffi::c_int = av_clip_c(
        startx + starty * (*s).mb_width,
        0 as ::core::ffi::c_int,
        (*s).mb_num - 1 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    let end_i: ::core::ffi::c_int = av_clip_c(
        endx + endy * (*s).mb_width,
        0 as ::core::ffi::c_int,
        (*s).mb_num,
    ) as ::core::ffi::c_int;
    let start_xy: ::core::ffi::c_int = *(*s).mb_index2xy.offset(start_i as isize);
    let end_xy: ::core::ffi::c_int = *(*s).mb_index2xy.offset(end_i as isize);
    let mut mask: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if !(*(*s).avctx).hwaccel.is_null() {
        return;
    }
    if start_i > end_i || start_xy > end_xy {
        av_log(
            (*s).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"internal error, slice end before start\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if (*(*s).avctx).error_concealment == 0 {
        return;
    }
    mask &= !(1 as ::core::ffi::c_int);
    if status & (2 as ::core::ffi::c_int | 16 as ::core::ffi::c_int) != 0 {
        mask &= !(2 as ::core::ffi::c_int | 16 as ::core::ffi::c_int);
        crate::rt::atomic_xadd_seqcst(
            &raw mut (*s).error_count,
            start_i - end_i - 1 as ::core::ffi::c_int,
        );
    }
    if status & (4 as ::core::ffi::c_int | 32 as ::core::ffi::c_int) != 0 {
        mask &= !(4 as ::core::ffi::c_int | 32 as ::core::ffi::c_int);
        crate::rt::atomic_xadd_seqcst(
            &raw mut (*s).error_count,
            start_i - end_i - 1 as ::core::ffi::c_int,
        );
    }
    if status & (8 as ::core::ffi::c_int | 64 as ::core::ffi::c_int) != 0 {
        mask &= !(8 as ::core::ffi::c_int | 64 as ::core::ffi::c_int);
        crate::rt::atomic_xadd_seqcst(
            &raw mut (*s).error_count,
            start_i - end_i - 1 as ::core::ffi::c_int,
        );
    }
    if status
        & (2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int | 8 as ::core::ffi::c_int)
        != 0
    {
        (*s).error_occurred = 1 as ::core::ffi::c_int;
        crate::rt::atomic_store_seqcst(
            &raw mut (*s).error_count,
            2147483647 as ::core::ffi::c_int,
        );
    }
    if mask == !(0x7f as ::core::ffi::c_int) {
        memset(
            (*s).error_status_table.offset(start_xy as isize) as *mut uint8_t
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ((end_xy - start_xy) as size_t)
                .wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
        );
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = start_xy;
        while i < end_xy {
            let ref mut fresh7 = *(*s).error_status_table.offset(i as isize);
            *fresh7 = (*fresh7 as ::core::ffi::c_int & mask) as uint8_t;
            i += 1;
        }
    }
    if end_i == (*s).mb_num {
        crate::rt::atomic_store_seqcst(
            &raw mut (*s).error_count,
            2147483647 as ::core::ffi::c_int,
        );
    } else {
        let ref mut fresh8 = *(*s).error_status_table.offset(end_xy as isize);
        *fresh8 = (*fresh8 as ::core::ffi::c_int & mask) as uint8_t;
        let ref mut fresh9 = *(*s).error_status_table.offset(end_xy as isize);
        *fresh9 = (*fresh9 as ::core::ffi::c_int | status) as uint8_t;
    }
    let ref mut fresh10 = *(*s).error_status_table.offset(start_xy as isize);
    *fresh10 = (*fresh10 as ::core::ffi::c_int | 1 as ::core::ffi::c_int) as uint8_t;
    if start_xy > 0 as ::core::ffi::c_int
        && (*(*s).avctx).active_thread_type & 2 as ::core::ffi::c_int == 0
        && er_supported(s) != 0 && (*(*s).avctx).skip_top * (*s).mb_width < start_i
    {
        let mut prev_status: ::core::ffi::c_int = *(*s)
            .error_status_table
            .offset(
                *(*s).mb_index2xy.offset((start_i - 1 as ::core::ffi::c_int) as isize)
                    as isize,
            ) as ::core::ffi::c_int;
        prev_status &= !(1 as ::core::ffi::c_int);
        if prev_status
            != 64 as ::core::ffi::c_int | 32 as ::core::ffi::c_int
                | 16 as ::core::ffi::c_int
        {
            (*s).error_occurred = 1 as ::core::ffi::c_int;
            crate::rt::atomic_store_seqcst(
                &raw mut (*s).error_count,
                2147483647 as ::core::ffi::c_int,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_er_frame_end(
    mut s: *mut ERContext,
    mut decode_error_flags: *mut ::core::ffi::c_int,
) {
    let mut current_block: u64;
    let mut linesize: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut i: ::core::ffi::c_int = 0;
    let mut mb_x: ::core::ffi::c_int = 0;
    let mut mb_y: ::core::ffi::c_int = 0;
    let mut error: ::core::ffi::c_int = 0;
    let mut error_type: ::core::ffi::c_int = 0;
    let mut dc_error: ::core::ffi::c_int = 0;
    let mut mv_error: ::core::ffi::c_int = 0;
    let mut ac_error: ::core::ffi::c_int = 0;
    let mut distance: ::core::ffi::c_int = 0;
    let mut threshold_part: [::core::ffi::c_int; 4] = [
        100 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int,
        100 as ::core::ffi::c_int,
        0,
    ];
    let mut threshold: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
    let mut is_intra_likely: ::core::ffi::c_int = 0;
    let mut size: ::core::ffi::c_int = ((*s).b8_stride * 2 as ptrdiff_t
        * (*s).mb_height as ptrdiff_t) as ::core::ffi::c_int;
    let mut guessed_mb_type: ::core::ffi::c_int = 0;
    if (*(*s).avctx).error_concealment == 0
        || crate::rt::atomic_load_seqcst(&raw mut (*s).error_count) == 0
        || (*(*s).avctx).lowres != 0 || er_supported(s) == 0
        || crate::rt::atomic_load_seqcst(&raw mut (*s).error_count)
            == 3 as ::core::ffi::c_int * (*s).mb_width
                * ((*(*s).avctx).skip_top + (*(*s).avctx).skip_bottom)
    {
        return;
    }
    linesize = &raw mut (*(*s).cur_pic.f).linesize as *mut ::core::ffi::c_int;
    if (*(*s).avctx).codec_id as ::core::ffi::c_uint
        == AV_CODEC_ID_MPEG2VIDEO as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*(*s).avctx).height + 16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
            & !(16 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            & 16 as ::core::ffi::c_int != 0
        && crate::rt::atomic_load_seqcst(&raw mut (*s).error_count)
            == 3 as ::core::ffi::c_int * (*s).mb_width
                * ((*(*s).avctx).skip_top + (*(*s).avctx).skip_bottom
                    + 1 as ::core::ffi::c_int)
    {
        mb_x = 0 as ::core::ffi::c_int;
        while mb_x < (*s).mb_width {
            let mut status: ::core::ffi::c_int = *(*s)
                .error_status_table
                .offset(
                    (mb_x as ptrdiff_t
                        + ((*s).mb_height - 1 as ::core::ffi::c_int) as ptrdiff_t
                            * (*s).mb_stride) as isize,
                ) as ::core::ffi::c_int;
            if status != 0x7f as ::core::ffi::c_int {
                break;
            }
            mb_x += 1;
        }
        if mb_x == (*s).mb_width {
            av_log(
                (*s).avctx as *mut ::core::ffi::c_void,
                48 as ::core::ffi::c_int,
                b"ignoring last missing slice\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return;
        }
    }
    if !(*s).last_pic.f.is_null() {
        if (*(*s).last_pic.f).width != (*(*s).cur_pic.f).width
            || (*(*s).last_pic.f).height != (*(*s).cur_pic.f).height
            || (*(*s).last_pic.f).format != (*(*s).cur_pic.f).format
        {
            av_log(
                (*s).avctx as *mut ::core::ffi::c_void,
                24 as ::core::ffi::c_int,
                b"Cannot use previous picture in error concealment\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            memset(
                &raw mut (*s).last_pic as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<ERPicture>() as size_t,
            );
        }
    }
    if !(*s).next_pic.f.is_null() {
        if (*(*s).next_pic.f).width != (*(*s).cur_pic.f).width
            || (*(*s).next_pic.f).height != (*(*s).cur_pic.f).height
            || (*(*s).next_pic.f).format != (*(*s).cur_pic.f).format
        {
            av_log(
                (*s).avctx as *mut ::core::ffi::c_void,
                24 as ::core::ffi::c_int,
                b"Cannot use next picture in error concealment\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            memset(
                &raw mut (*s).next_pic as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<ERPicture>() as size_t,
            );
        }
    }
    if (*s).cur_pic.motion_val[0 as ::core::ffi::c_int as usize].is_null()
        || (*s).cur_pic.ref_index[0 as ::core::ffi::c_int as usize].is_null()
    {
        av_log(
            (*s).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Warning MVs not available\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        loop {
            if !(i < 2 as ::core::ffi::c_int) {
                current_block = 5494826135382683477;
                break;
            }
            (*s).ref_index[i as usize] = av_calloc(
                ((*s).mb_stride * (*s).mb_height as ptrdiff_t) as size_t,
                (4 as size_t).wrapping_mul(::core::mem::size_of::<uint8_t>() as size_t),
            ) as *mut int8_t;
            (*s).motion_val_base[i as usize] = av_calloc(
                (size + 4 as ::core::ffi::c_int) as size_t,
                (2 as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
            ) as *mut [int16_t; 2];
            if (*s).ref_index[i as usize].is_null()
                || (*s).motion_val_base[i as usize].is_null()
            {
                current_block = 4652747497698925025;
                break;
            }
            (*s).cur_pic.ref_index[i as usize] = (*s).ref_index[i as usize];
            (*s).cur_pic.motion_val[i as usize] = (*s)
                .motion_val_base[i as usize]
                .offset(4 as ::core::ffi::c_int as isize);
            i += 1;
        }
    } else {
        current_block = 5494826135382683477;
    }
    match current_block {
        5494826135382683477 => {
            if (*(*s).avctx).debug & 0x400 as ::core::ffi::c_int != 0 {
                mb_y = 0 as ::core::ffi::c_int;
                while mb_y < (*s).mb_height {
                    mb_x = 0 as ::core::ffi::c_int;
                    while mb_x < (*s).mb_width {
                        let mut status_0: ::core::ffi::c_int = *(*s)
                            .error_status_table
                            .offset(
                                (mb_x as ptrdiff_t + mb_y as ptrdiff_t * (*s).mb_stride)
                                    as isize,
                            ) as ::core::ffi::c_int;
                        av_log(
                            (*s).avctx as *mut ::core::ffi::c_void,
                            48 as ::core::ffi::c_int,
                            b"%2X \0" as *const u8 as *const ::core::ffi::c_char,
                            status_0,
                        );
                        mb_x += 1;
                    }
                    av_log(
                        (*s).avctx as *mut ::core::ffi::c_void,
                        48 as ::core::ffi::c_int,
                        b"\n\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    mb_y += 1;
                }
            }
            error_type = 1 as ::core::ffi::c_int;
            while error_type <= 3 as ::core::ffi::c_int {
                let mut end_ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                i = (*s).mb_num - 1 as ::core::ffi::c_int;
                while i >= 0 as ::core::ffi::c_int {
                    let mb_xy: ::core::ffi::c_int = *(*s).mb_index2xy.offset(i as isize);
                    let mut error_0: ::core::ffi::c_int = *(*s)
                        .error_status_table
                        .offset(mb_xy as isize) as ::core::ffi::c_int;
                    if error_0 & (1 as ::core::ffi::c_int) << error_type != 0 {
                        end_ok = 1 as ::core::ffi::c_int;
                    }
                    if error_0 & (8 as ::core::ffi::c_int) << error_type != 0 {
                        end_ok = 1 as ::core::ffi::c_int;
                    }
                    if end_ok == 0 {
                        let ref mut fresh0 = *(*s)
                            .error_status_table
                            .offset(mb_xy as isize);
                        *fresh0 = (*fresh0 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << error_type) as uint8_t;
                    }
                    if error_0 & 1 as ::core::ffi::c_int != 0 {
                        end_ok = 0 as ::core::ffi::c_int;
                    }
                    i -= 1;
                }
                error_type += 1;
            }
            if (*s).partitioned_frame != 0 {
                let mut end_ok_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                i = (*s).mb_num - 1 as ::core::ffi::c_int;
                while i >= 0 as ::core::ffi::c_int {
                    let mb_xy_0: ::core::ffi::c_int = *(*s)
                        .mb_index2xy
                        .offset(i as isize);
                    let mut error_1: ::core::ffi::c_int = *(*s)
                        .error_status_table
                        .offset(mb_xy_0 as isize) as ::core::ffi::c_int;
                    if error_1 & 16 as ::core::ffi::c_int != 0 {
                        end_ok_0 = 0 as ::core::ffi::c_int;
                    }
                    if error_1 & 64 as ::core::ffi::c_int != 0
                        || error_1 & 32 as ::core::ffi::c_int != 0
                        || error_1 & 2 as ::core::ffi::c_int != 0
                    {
                        end_ok_0 = 1 as ::core::ffi::c_int;
                    }
                    if end_ok_0 == 0 {
                        let ref mut fresh1 = *(*s)
                            .error_status_table
                            .offset(mb_xy_0 as isize);
                        *fresh1 = (*fresh1 as ::core::ffi::c_int
                            | 2 as ::core::ffi::c_int) as uint8_t;
                    }
                    if error_1 & 1 as ::core::ffi::c_int != 0 {
                        end_ok_0 = 0 as ::core::ffi::c_int;
                    }
                    i -= 1;
                }
            }
            if (*(*s).avctx).err_recognition
                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
            {
                let mut end_ok_1: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                i = (*s).mb_num - 2 as ::core::ffi::c_int;
                while i >= (*s).mb_width + 100 as ::core::ffi::c_int {
                    let mb_xy_1: ::core::ffi::c_int = *(*s)
                        .mb_index2xy
                        .offset(i as isize);
                    let mut error1: ::core::ffi::c_int = *(*s)
                        .error_status_table
                        .offset(mb_xy_1 as isize) as ::core::ffi::c_int;
                    let mut error2: ::core::ffi::c_int = *(*s)
                        .error_status_table
                        .offset(
                            *(*s)
                                .mb_index2xy
                                .offset((i + 1 as ::core::ffi::c_int) as isize) as isize,
                        ) as ::core::ffi::c_int;
                    if error1 & 1 as ::core::ffi::c_int != 0 {
                        end_ok_1 = 1 as ::core::ffi::c_int;
                    }
                    if error2
                        == 1 as ::core::ffi::c_int
                            | (2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                                | 8 as ::core::ffi::c_int)
                            | (16 as ::core::ffi::c_int | 32 as ::core::ffi::c_int
                                | 64 as ::core::ffi::c_int)
                        && error1
                            != 1 as ::core::ffi::c_int
                                | (2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                                    | 8 as ::core::ffi::c_int)
                                | (16 as ::core::ffi::c_int | 32 as ::core::ffi::c_int
                                    | 64 as ::core::ffi::c_int)
                        && (error1 & 16 as ::core::ffi::c_int != 0
                            || error1 & 32 as ::core::ffi::c_int != 0
                            || error1 & 64 as ::core::ffi::c_int != 0)
                    {
                        end_ok_1 = 0 as ::core::ffi::c_int;
                    }
                    if end_ok_1 == 0 {
                        let ref mut fresh2 = *(*s)
                            .error_status_table
                            .offset(mb_xy_1 as isize);
                        *fresh2 = (*fresh2 as ::core::ffi::c_int
                            | (2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                                | 8 as ::core::ffi::c_int)) as uint8_t;
                    }
                    i -= 1;
                }
            }
            distance = 9999999 as ::core::ffi::c_int;
            error_type = 1 as ::core::ffi::c_int;
            while error_type <= 3 as ::core::ffi::c_int {
                i = (*s).mb_num - 1 as ::core::ffi::c_int;
                while i >= 0 as ::core::ffi::c_int {
                    let mb_xy_2: ::core::ffi::c_int = *(*s)
                        .mb_index2xy
                        .offset(i as isize);
                    let mut error_2: ::core::ffi::c_int = *(*s)
                        .error_status_table
                        .offset(mb_xy_2 as isize) as ::core::ffi::c_int;
                    if (*s).mbskip_table.is_null()
                        || *(*s).mbskip_table.offset(mb_xy_2 as isize) == 0
                    {
                        distance += 1;
                    }
                    if error_2 & (1 as ::core::ffi::c_int) << error_type != 0 {
                        distance = 0 as ::core::ffi::c_int;
                    }
                    if (*s).partitioned_frame != 0 {
                        if distance
                            < threshold_part[(error_type - 1 as ::core::ffi::c_int)
                                as usize]
                        {
                            let ref mut fresh3 = *(*s)
                                .error_status_table
                                .offset(mb_xy_2 as isize);
                            *fresh3 = (*fresh3 as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << error_type) as uint8_t;
                        }
                    } else if distance < threshold {
                        let ref mut fresh4 = *(*s)
                            .error_status_table
                            .offset(mb_xy_2 as isize);
                        *fresh4 = (*fresh4 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << error_type) as uint8_t;
                    }
                    if error_2 & 1 as ::core::ffi::c_int != 0 {
                        distance = 9999999 as ::core::ffi::c_int;
                    }
                    i -= 1;
                }
                error_type += 1;
            }
            error = 0 as ::core::ffi::c_int;
            i = 0 as ::core::ffi::c_int;
            while i < (*s).mb_num {
                let mb_xy_3: ::core::ffi::c_int = *(*s).mb_index2xy.offset(i as isize);
                let mut old_error: ::core::ffi::c_int = *(*s)
                    .error_status_table
                    .offset(mb_xy_3 as isize) as ::core::ffi::c_int;
                if old_error & 1 as ::core::ffi::c_int != 0 {
                    error = old_error
                        & (2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                            | 8 as ::core::ffi::c_int);
                } else {
                    error
                        |= old_error
                            & (2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                                | 8 as ::core::ffi::c_int);
                    let ref mut fresh5 = *(*s)
                        .error_status_table
                        .offset(mb_xy_3 as isize);
                    *fresh5 = (*fresh5 as ::core::ffi::c_int | error) as uint8_t;
                }
                i += 1;
            }
            if (*s).partitioned_frame == 0 {
                i = 0 as ::core::ffi::c_int;
                while i < (*s).mb_num {
                    let mb_xy_4: ::core::ffi::c_int = *(*s)
                        .mb_index2xy
                        .offset(i as isize);
                    let mut error_3: ::core::ffi::c_int = *(*s)
                        .error_status_table
                        .offset(mb_xy_4 as isize) as ::core::ffi::c_int;
                    if error_3
                        & (2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                            | 8 as ::core::ffi::c_int) != 0
                    {
                        error_3
                            |= 2 as ::core::ffi::c_int | 4 as ::core::ffi::c_int
                                | 8 as ::core::ffi::c_int;
                    }
                    *(*s).error_status_table.offset(mb_xy_4 as isize) = error_3
                        as uint8_t;
                    i += 1;
                }
            }
            mv_error = 0 as ::core::ffi::c_int;
            ac_error = mv_error;
            dc_error = ac_error;
            i = 0 as ::core::ffi::c_int;
            while i < (*s).mb_num {
                let mb_xy_5: ::core::ffi::c_int = *(*s).mb_index2xy.offset(i as isize);
                let mut error_4: ::core::ffi::c_int = *(*s)
                    .error_status_table
                    .offset(mb_xy_5 as isize) as ::core::ffi::c_int;
                if error_4 & 4 as ::core::ffi::c_int != 0 {
                    dc_error += 1;
                }
                if error_4 & 2 as ::core::ffi::c_int != 0 {
                    ac_error += 1;
                }
                if error_4 & 8 as ::core::ffi::c_int != 0 {
                    mv_error += 1;
                }
                i += 1;
            }
            av_log(
                (*s).avctx as *mut ::core::ffi::c_void,
                32 as ::core::ffi::c_int,
                b"concealing %d DC, %d AC, %d MV errors in %c frame\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                dc_error,
                ac_error,
                mv_error,
                av_get_picture_type_char((*(*s).cur_pic.f).pict_type)
                    as ::core::ffi::c_int,
            );
            if !decode_error_flags.is_null() {
                *decode_error_flags |= 4 as ::core::ffi::c_int;
            } else {
                (*(*s).cur_pic.f).decode_error_flags |= 4 as ::core::ffi::c_int;
            }
            is_intra_likely = is_intra_more_likely(s);
            guessed_mb_type = if is_intra_likely != 0 {
                (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
            } else {
                (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                    | (if (*(*s).avctx).codec_id as ::core::ffi::c_uint
                        == AV_CODEC_ID_H264 as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                    } else {
                        (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                    })
            };
            i = 0 as ::core::ffi::c_int;
            while i < (*s).mb_num {
                let mb_xy_6: ::core::ffi::c_int = *(*s).mb_index2xy.offset(i as isize);
                let mut error_5: ::core::ffi::c_int = *(*s)
                    .error_status_table
                    .offset(mb_xy_6 as isize) as ::core::ffi::c_int;
                if error_5 & 4 as ::core::ffi::c_int != 0
                    && error_5 & 8 as ::core::ffi::c_int != 0
                {
                    *(*s).cur_pic.mb_type.offset(mb_xy_6 as isize) = guessed_mb_type
                        as uint32_t;
                }
                i += 1;
            }
            if !(!(*s).last_pic.f.is_null()
                && !(*(*s).last_pic.f).data[0 as ::core::ffi::c_int as usize].is_null())
                && !(!(*s).next_pic.f.is_null()
                    && !(*(*s).next_pic.f)
                        .data[0 as ::core::ffi::c_int as usize]
                        .is_null())
            {
                i = 0 as ::core::ffi::c_int;
                while i < (*s).mb_num {
                    let mb_xy_7: ::core::ffi::c_int = *(*s)
                        .mb_index2xy
                        .offset(i as isize);
                    if *(*s).cur_pic.mb_type.offset(mb_xy_7 as isize) & 7 as uint32_t
                        == 0
                    {
                        *(*s).cur_pic.mb_type.offset(mb_xy_7 as isize) = ((1
                            as ::core::ffi::c_int) << 0 as ::core::ffi::c_int)
                            as uint32_t;
                    }
                    i += 1;
                }
            }
            mb_y = 0 as ::core::ffi::c_int;
            while mb_y < (*s).mb_height {
                mb_x = 0 as ::core::ffi::c_int;
                while mb_x < (*s).mb_width {
                    let mb_xy_8: ::core::ffi::c_int = (mb_x as ptrdiff_t
                        + mb_y as ptrdiff_t * (*s).mb_stride) as ::core::ffi::c_int;
                    let mb_type: ::core::ffi::c_int = *(*s)
                        .cur_pic
                        .mb_type
                        .offset(mb_xy_8 as isize) as ::core::ffi::c_int;
                    let dir: ::core::ffi::c_int = !(!(*s).last_pic.f.is_null()
                        && !(*(*s).last_pic.f)
                            .data[0 as ::core::ffi::c_int as usize]
                            .is_null()) as ::core::ffi::c_int;
                    let mv_dir: ::core::ffi::c_int = if dir != 0 {
                        2 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    };
                    let mut mv_type: ::core::ffi::c_int = 0;
                    let mut error_6: ::core::ffi::c_int = *(*s)
                        .error_status_table
                        .offset(mb_xy_8 as isize) as ::core::ffi::c_int;
                    if !(mb_type & 7 as ::core::ffi::c_int != 0) {
                        if !(error_6 & 8 as ::core::ffi::c_int != 0) {
                            if !(error_6 & 2 as ::core::ffi::c_int == 0) {
                                if mb_type
                                    & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
                                {
                                    let mut mb_index: ::core::ffi::c_int = ((mb_x
                                        * 2 as ::core::ffi::c_int) as ptrdiff_t
                                        + (mb_y * 2 as ::core::ffi::c_int) as ptrdiff_t
                                            * (*s).b8_stride) as ::core::ffi::c_int;
                                    let mut j: ::core::ffi::c_int = 0;
                                    mv_type = 1 as ::core::ffi::c_int;
                                    j = 0 as ::core::ffi::c_int;
                                    while j < 4 as ::core::ffi::c_int {
                                        (*s)
                                            .mv[0 as ::core::ffi::c_int
                                            as usize][j as usize][0 as ::core::ffi::c_int as usize] = (*(*s)
                                            .cur_pic
                                            .motion_val[dir as usize]
                                            .offset(
                                                ((mb_index + (j & 1 as ::core::ffi::c_int)) as ptrdiff_t
                                                    + (j >> 1 as ::core::ffi::c_int) as ptrdiff_t
                                                        * (*s).b8_stride) as isize,
                                            ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                                        (*s)
                                            .mv[0 as ::core::ffi::c_int
                                            as usize][j as usize][1 as ::core::ffi::c_int as usize] = (*(*s)
                                            .cur_pic
                                            .motion_val[dir as usize]
                                            .offset(
                                                ((mb_index + (j & 1 as ::core::ffi::c_int)) as ptrdiff_t
                                                    + (j >> 1 as ::core::ffi::c_int) as ptrdiff_t
                                                        * (*s).b8_stride) as isize,
                                            ))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                                        j += 1;
                                    }
                                } else {
                                    mv_type = 0 as ::core::ffi::c_int;
                                    (*s)
                                        .mv[0 as ::core::ffi::c_int
                                        as usize][0 as ::core::ffi::c_int
                                        as usize][0 as ::core::ffi::c_int as usize] = (*(*s)
                                        .cur_pic
                                        .motion_val[dir as usize]
                                        .offset(
                                            ((mb_x * 2 as ::core::ffi::c_int) as ptrdiff_t
                                                + (mb_y * 2 as ::core::ffi::c_int) as ptrdiff_t
                                                    * (*s).b8_stride) as isize,
                                        ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                                    (*s)
                                        .mv[0 as ::core::ffi::c_int
                                        as usize][0 as ::core::ffi::c_int
                                        as usize][1 as ::core::ffi::c_int as usize] = (*(*s)
                                        .cur_pic
                                        .motion_val[dir as usize]
                                        .offset(
                                            ((mb_x * 2 as ::core::ffi::c_int) as ptrdiff_t
                                                + (mb_y * 2 as ::core::ffi::c_int) as ptrdiff_t
                                                    * (*s).b8_stride) as isize,
                                        ))[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
                                }
                                (*s)
                                    .decode_mb
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    (*s).opaque,
                                    0 as ::core::ffi::c_int,
                                    mv_dir,
                                    mv_type,
                                    &raw mut (*s).mv,
                                    mb_x,
                                    mb_y,
                                    0 as ::core::ffi::c_int,
                                    0 as ::core::ffi::c_int,
                                );
                            }
                        }
                    }
                    mb_x += 1;
                }
                mb_y += 1;
            }
            if (*(*s).cur_pic.f).pict_type as ::core::ffi::c_uint
                == AV_PICTURE_TYPE_B as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                mb_y = 0 as ::core::ffi::c_int;
                while mb_y < (*s).mb_height {
                    mb_x = 0 as ::core::ffi::c_int;
                    while mb_x < (*s).mb_width {
                        let mut xy: ::core::ffi::c_int = ((mb_x
                            * 2 as ::core::ffi::c_int) as ptrdiff_t
                            + (mb_y * 2 as ::core::ffi::c_int) as ptrdiff_t
                                * (*s).b8_stride) as ::core::ffi::c_int;
                        let mb_xy_9: ::core::ffi::c_int = (mb_x as ptrdiff_t
                            + mb_y as ptrdiff_t * (*s).mb_stride) as ::core::ffi::c_int;
                        let mb_type_0: ::core::ffi::c_int = *(*s)
                            .cur_pic
                            .mb_type
                            .offset(mb_xy_9 as isize) as ::core::ffi::c_int;
                        let mut mv_dir_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int
                            | 2 as ::core::ffi::c_int;
                        let mut error_7: ::core::ffi::c_int = *(*s)
                            .error_status_table
                            .offset(mb_xy_9 as isize) as ::core::ffi::c_int;
                        if !(mb_type_0 & 7 as ::core::ffi::c_int != 0) {
                            if !(error_7 & 8 as ::core::ffi::c_int == 0) {
                                if !(error_7 & 2 as ::core::ffi::c_int == 0) {
                                    if !(!(*s).last_pic.f.is_null()
                                        && !(*(*s).last_pic.f)
                                            .data[0 as ::core::ffi::c_int as usize]
                                            .is_null())
                                    {
                                        mv_dir_0 &= !(1 as ::core::ffi::c_int);
                                    }
                                    if !(!(*s).next_pic.f.is_null()
                                        && !(*(*s).next_pic.f)
                                            .data[0 as ::core::ffi::c_int as usize]
                                            .is_null())
                                    {
                                        mv_dir_0 &= !(2 as ::core::ffi::c_int);
                                    }
                                    if (*s).pp_time != 0 {
                                        let mut time_pp: ::core::ffi::c_int = (*s).pp_time
                                            as ::core::ffi::c_int;
                                        let mut time_pb: ::core::ffi::c_int = (*s).pb_time
                                            as ::core::ffi::c_int;
                                        if !((*(*s).avctx).codec_id as ::core::ffi::c_uint
                                            != AV_CODEC_ID_H264 as ::core::ffi::c_int
                                                as ::core::ffi::c_uint)
                                        {
                                            av_log(
                                                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                                                0 as ::core::ffi::c_int,
                                                b"Assertion %s failed at %s:%d\n\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                b"s->avctx->codec_id != AV_CODEC_ID_H264\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                b"src/libavcodec/error_resilience.c\0" as *const u8
                                                    as *const ::core::ffi::c_char,
                                                1219 as ::core::ffi::c_int,
                                            );
                                            abort();
                                        }
                                        ff_thread_progress_await(
                                            (*s).next_pic.progress as *const ThreadProgress,
                                            mb_y,
                                        );
                                        (*s)
                                            .mv[0 as ::core::ffi::c_int
                                            as usize][0 as ::core::ffi::c_int
                                            as usize][0 as ::core::ffi::c_int as usize] = (*(*s)
                                            .next_pic
                                            .motion_val[0 as ::core::ffi::c_int as usize]
                                            .offset(xy as isize))[0 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int * time_pb / time_pp;
                                        (*s)
                                            .mv[0 as ::core::ffi::c_int
                                            as usize][0 as ::core::ffi::c_int
                                            as usize][1 as ::core::ffi::c_int as usize] = (*(*s)
                                            .next_pic
                                            .motion_val[0 as ::core::ffi::c_int as usize]
                                            .offset(xy as isize))[1 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int * time_pb / time_pp;
                                        (*s)
                                            .mv[1 as ::core::ffi::c_int
                                            as usize][0 as ::core::ffi::c_int
                                            as usize][0 as ::core::ffi::c_int as usize] = (*(*s)
                                            .next_pic
                                            .motion_val[0 as ::core::ffi::c_int as usize]
                                            .offset(xy as isize))[0 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int * (time_pb - time_pp) / time_pp;
                                        (*s)
                                            .mv[1 as ::core::ffi::c_int
                                            as usize][0 as ::core::ffi::c_int
                                            as usize][1 as ::core::ffi::c_int as usize] = (*(*s)
                                            .next_pic
                                            .motion_val[0 as ::core::ffi::c_int as usize]
                                            .offset(xy as isize))[1 as ::core::ffi::c_int as usize]
                                            as ::core::ffi::c_int * (time_pb - time_pp) / time_pp;
                                    } else {
                                        (*s)
                                            .mv[0 as ::core::ffi::c_int
                                            as usize][0 as ::core::ffi::c_int
                                            as usize][0 as ::core::ffi::c_int as usize] = 0
                                            as ::core::ffi::c_int;
                                        (*s)
                                            .mv[0 as ::core::ffi::c_int
                                            as usize][0 as ::core::ffi::c_int
                                            as usize][1 as ::core::ffi::c_int as usize] = 0
                                            as ::core::ffi::c_int;
                                        (*s)
                                            .mv[1 as ::core::ffi::c_int
                                            as usize][0 as ::core::ffi::c_int
                                            as usize][0 as ::core::ffi::c_int as usize] = 0
                                            as ::core::ffi::c_int;
                                        (*s)
                                            .mv[1 as ::core::ffi::c_int
                                            as usize][0 as ::core::ffi::c_int
                                            as usize][1 as ::core::ffi::c_int as usize] = 0
                                            as ::core::ffi::c_int;
                                    }
                                    (*s)
                                        .decode_mb
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        (*s).opaque,
                                        0 as ::core::ffi::c_int,
                                        mv_dir_0,
                                        0 as ::core::ffi::c_int,
                                        &raw mut (*s).mv,
                                        mb_x,
                                        mb_y,
                                        0 as ::core::ffi::c_int,
                                        0 as ::core::ffi::c_int,
                                    );
                                }
                            }
                        }
                        mb_x += 1;
                    }
                    mb_y += 1;
                }
            } else {
                guess_mv(s);
            }
            mb_y = 0 as ::core::ffi::c_int;
            while mb_y < (*s).mb_height {
                mb_x = 0 as ::core::ffi::c_int;
                while mb_x < (*s).mb_width {
                    let mut dc: ::core::ffi::c_int = 0;
                    let mut dcu: ::core::ffi::c_int = 0;
                    let mut dcv: ::core::ffi::c_int = 0;
                    let mut y: ::core::ffi::c_int = 0;
                    let mut n: ::core::ffi::c_int = 0;
                    let mut dc_ptr: *mut int16_t = ::core::ptr::null_mut::<int16_t>();
                    let mut dest_y: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                    let mut dest_cb: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                    let mut dest_cr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                    let mb_xy_10: ::core::ffi::c_int = (mb_x as ptrdiff_t
                        + mb_y as ptrdiff_t * (*s).mb_stride) as ::core::ffi::c_int;
                    let mb_type_1: ::core::ffi::c_int = *(*s)
                        .cur_pic
                        .mb_type
                        .offset(mb_xy_10 as isize) as ::core::ffi::c_int;
                    if !(mb_type_1 & 7 as ::core::ffi::c_int != 0
                        && (*s).partitioned_frame != 0)
                    {
                        dest_y = (*(*s).cur_pic.f)
                            .data[0 as ::core::ffi::c_int as usize]
                            .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                            .offset(
                                (mb_y * 16 as ::core::ffi::c_int
                                    * *linesize.offset(0 as ::core::ffi::c_int as isize))
                                    as isize,
                            );
                        dest_cb = (*(*s).cur_pic.f)
                            .data[1 as ::core::ffi::c_int as usize]
                            .offset((mb_x * 8 as ::core::ffi::c_int) as isize)
                            .offset(
                                (mb_y * 8 as ::core::ffi::c_int
                                    * *linesize.offset(1 as ::core::ffi::c_int as isize))
                                    as isize,
                            );
                        dest_cr = (*(*s).cur_pic.f)
                            .data[2 as ::core::ffi::c_int as usize]
                            .offset((mb_x * 8 as ::core::ffi::c_int) as isize)
                            .offset(
                                (mb_y * 8 as ::core::ffi::c_int
                                    * *linesize.offset(2 as ::core::ffi::c_int as isize))
                                    as isize,
                            );
                        dc_ptr = (*(&raw mut (*s).dc_val as *mut *mut int16_t)
                            .offset(0 as ::core::ffi::c_int as isize))
                            .offset(
                                ((mb_x * 2 as ::core::ffi::c_int) as ptrdiff_t
                                    + (mb_y * 2 as ::core::ffi::c_int) as ptrdiff_t
                                        * (*s).b8_stride) as isize,
                            ) as *mut int16_t;
                        n = 0 as ::core::ffi::c_int;
                        while n < 4 as ::core::ffi::c_int {
                            dc = 0 as ::core::ffi::c_int;
                            y = 0 as ::core::ffi::c_int;
                            while y < 8 as ::core::ffi::c_int {
                                let mut x: ::core::ffi::c_int = 0;
                                x = 0 as ::core::ffi::c_int;
                                while x < 8 as ::core::ffi::c_int {
                                    dc
                                        += *dest_y
                                            .offset(
                                                (x + (n & 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int
                                                    + (y
                                                        + (n >> 1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                                                        * *linesize.offset(0 as ::core::ffi::c_int as isize))
                                                    as isize,
                                            ) as ::core::ffi::c_int;
                                    x += 1;
                                }
                                y += 1;
                            }
                            *dc_ptr
                                .offset(
                                    ((n & 1 as ::core::ffi::c_int) as ptrdiff_t
                                        + (n >> 1 as ::core::ffi::c_int) as ptrdiff_t
                                            * (*s).b8_stride) as isize,
                                ) = (dc + 4 as ::core::ffi::c_int
                                >> 3 as ::core::ffi::c_int) as int16_t;
                            n += 1;
                        }
                        if !(*(*s).cur_pic.f)
                            .data[2 as ::core::ffi::c_int as usize]
                            .is_null()
                        {
                            dcv = 0 as ::core::ffi::c_int;
                            dcu = dcv;
                            y = 0 as ::core::ffi::c_int;
                            while y < 8 as ::core::ffi::c_int {
                                let mut x_0: ::core::ffi::c_int = 0;
                                x_0 = 0 as ::core::ffi::c_int;
                                while x_0 < 8 as ::core::ffi::c_int {
                                    dcu
                                        += *dest_cb
                                            .offset(
                                                (x_0
                                                    + y * *linesize.offset(1 as ::core::ffi::c_int as isize))
                                                    as isize,
                                            ) as ::core::ffi::c_int;
                                    dcv
                                        += *dest_cr
                                            .offset(
                                                (x_0
                                                    + y * *linesize.offset(2 as ::core::ffi::c_int as isize))
                                                    as isize,
                                            ) as ::core::ffi::c_int;
                                    x_0 += 1;
                                }
                                y += 1;
                            }
                            *(*s)
                                .dc_val[1 as ::core::ffi::c_int as usize]
                                .offset(
                                    (mb_x as ptrdiff_t + mb_y as ptrdiff_t * (*s).mb_stride)
                                        as isize,
                                ) = (dcu + 4 as ::core::ffi::c_int
                                >> 3 as ::core::ffi::c_int) as int16_t;
                            *(*s)
                                .dc_val[2 as ::core::ffi::c_int as usize]
                                .offset(
                                    (mb_x as ptrdiff_t + mb_y as ptrdiff_t * (*s).mb_stride)
                                        as isize,
                                ) = (dcv + 4 as ::core::ffi::c_int
                                >> 3 as ::core::ffi::c_int) as int16_t;
                        }
                    }
                    mb_x += 1;
                }
                mb_y += 1;
            }
            guess_dc(
                s,
                (*s).dc_val[0 as ::core::ffi::c_int as usize],
                (*s).mb_width * 2 as ::core::ffi::c_int,
                (*s).mb_height * 2 as ::core::ffi::c_int,
                (*s).b8_stride,
                1 as ::core::ffi::c_int,
            );
            guess_dc(
                s,
                (*s).dc_val[1 as ::core::ffi::c_int as usize],
                (*s).mb_width,
                (*s).mb_height,
                (*s).mb_stride,
                0 as ::core::ffi::c_int,
            );
            guess_dc(
                s,
                (*s).dc_val[2 as ::core::ffi::c_int as usize],
                (*s).mb_width,
                (*s).mb_height,
                (*s).mb_stride,
                0 as ::core::ffi::c_int,
            );
            filter181(
                (*s).dc_val[0 as ::core::ffi::c_int as usize],
                (*s).mb_width * 2 as ::core::ffi::c_int,
                (*s).mb_height * 2 as ::core::ffi::c_int,
                (*s).b8_stride,
            );
            mb_y = 0 as ::core::ffi::c_int;
            while mb_y < (*s).mb_height {
                mb_x = 0 as ::core::ffi::c_int;
                while mb_x < (*s).mb_width {
                    let mut dest_y_0: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                    let mut dest_cb_0: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                    let mut dest_cr_0: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                    let mb_xy_11: ::core::ffi::c_int = (mb_x as ptrdiff_t
                        + mb_y as ptrdiff_t * (*s).mb_stride) as ::core::ffi::c_int;
                    let mb_type_2: ::core::ffi::c_int = *(*s)
                        .cur_pic
                        .mb_type
                        .offset(mb_xy_11 as isize) as ::core::ffi::c_int;
                    let mut error_8: ::core::ffi::c_int = *(*s)
                        .error_status_table
                        .offset(mb_xy_11 as isize) as ::core::ffi::c_int;
                    if !(mb_type_2
                        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) != 0)
                    {
                        if !(error_8 & 2 as ::core::ffi::c_int == 0) {
                            dest_y_0 = (*(*s).cur_pic.f)
                                .data[0 as ::core::ffi::c_int as usize]
                                .offset((mb_x * 16 as ::core::ffi::c_int) as isize)
                                .offset(
                                    (mb_y * 16 as ::core::ffi::c_int
                                        * *linesize.offset(0 as ::core::ffi::c_int as isize))
                                        as isize,
                                );
                            dest_cb_0 = (*(*s).cur_pic.f)
                                .data[1 as ::core::ffi::c_int as usize]
                                .offset((mb_x * 8 as ::core::ffi::c_int) as isize)
                                .offset(
                                    (mb_y * 8 as ::core::ffi::c_int
                                        * *linesize.offset(1 as ::core::ffi::c_int as isize))
                                        as isize,
                                );
                            dest_cr_0 = (*(*s).cur_pic.f)
                                .data[2 as ::core::ffi::c_int as usize]
                                .offset((mb_x * 8 as ::core::ffi::c_int) as isize)
                                .offset(
                                    (mb_y * 8 as ::core::ffi::c_int
                                        * *linesize.offset(2 as ::core::ffi::c_int as isize))
                                        as isize,
                                );
                            if (*(*s).cur_pic.f)
                                .data[2 as ::core::ffi::c_int as usize]
                                .is_null()
                            {
                                dest_cr_0 = ::core::ptr::null_mut::<uint8_t>();
                                dest_cb_0 = dest_cr_0;
                            }
                            put_dc(s, dest_y_0, dest_cb_0, dest_cr_0, mb_x, mb_y);
                        }
                    }
                    mb_x += 1;
                }
                mb_y += 1;
            }
            if (*(*s).avctx).error_concealment & 2 as ::core::ffi::c_int != 0 {
                h_block_filter(
                    s,
                    (*(*s).cur_pic.f).data[0 as ::core::ffi::c_int as usize],
                    (*s).mb_width * 2 as ::core::ffi::c_int,
                    (*s).mb_height * 2 as ::core::ffi::c_int,
                    *linesize.offset(0 as ::core::ffi::c_int as isize) as ptrdiff_t,
                    1 as ::core::ffi::c_int,
                );
                v_block_filter(
                    s,
                    (*(*s).cur_pic.f).data[0 as ::core::ffi::c_int as usize],
                    (*s).mb_width * 2 as ::core::ffi::c_int,
                    (*s).mb_height * 2 as ::core::ffi::c_int,
                    *linesize.offset(0 as ::core::ffi::c_int as isize) as ptrdiff_t,
                    1 as ::core::ffi::c_int,
                );
                if !(*(*s).cur_pic.f).data[2 as ::core::ffi::c_int as usize].is_null() {
                    h_block_filter(
                        s,
                        (*(*s).cur_pic.f).data[1 as ::core::ffi::c_int as usize],
                        (*s).mb_width,
                        (*s).mb_height,
                        *linesize.offset(1 as ::core::ffi::c_int as isize) as ptrdiff_t,
                        0 as ::core::ffi::c_int,
                    );
                    h_block_filter(
                        s,
                        (*(*s).cur_pic.f).data[2 as ::core::ffi::c_int as usize],
                        (*s).mb_width,
                        (*s).mb_height,
                        *linesize.offset(2 as ::core::ffi::c_int as isize) as ptrdiff_t,
                        0 as ::core::ffi::c_int,
                    );
                    v_block_filter(
                        s,
                        (*(*s).cur_pic.f).data[1 as ::core::ffi::c_int as usize],
                        (*s).mb_width,
                        (*s).mb_height,
                        *linesize.offset(1 as ::core::ffi::c_int as isize) as ptrdiff_t,
                        0 as ::core::ffi::c_int,
                    );
                    v_block_filter(
                        s,
                        (*(*s).cur_pic.f).data[2 as ::core::ffi::c_int as usize],
                        (*s).mb_width,
                        (*s).mb_height,
                        *linesize.offset(2 as ::core::ffi::c_int as isize) as ptrdiff_t,
                        0 as ::core::ffi::c_int,
                    );
                }
            }
            i = 0 as ::core::ffi::c_int;
            while i < (*s).mb_num {
                let mb_xy_12: ::core::ffi::c_int = *(*s).mb_index2xy.offset(i as isize);
                let mut error_9: ::core::ffi::c_int = *(*s)
                    .error_status_table
                    .offset(mb_xy_12 as isize) as ::core::ffi::c_int;
                if !(*s).mbskip_table.is_null()
                    && (*(*s).cur_pic.f).pict_type as ::core::ffi::c_uint
                        != AV_PICTURE_TYPE_B as ::core::ffi::c_int as ::core::ffi::c_uint
                    && error_9
                        & (4 as ::core::ffi::c_int | 8 as ::core::ffi::c_int
                            | 2 as ::core::ffi::c_int) != 0
                {
                    *(*s).mbskip_table.offset(mb_xy_12 as isize) = 0 as uint8_t;
                }
                if !(*s).mbintra_table.is_null() {
                    *(*s).mbintra_table.offset(mb_xy_12 as isize) = 1 as uint8_t;
                }
                i += 1;
            }
            memset(
                &raw mut (*s).cur_pic as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<ERPicture>() as size_t,
            );
            memset(
                &raw mut (*s).last_pic as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<ERPicture>() as size_t,
            );
            memset(
                &raw mut (*s).next_pic as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<ERPicture>() as size_t,
            );
        }
        _ => {}
    }
    i = 0 as ::core::ffi::c_int;
    while i < 2 as ::core::ffi::c_int {
        av_freep(
            (&raw mut (*s).ref_index as *mut *mut int8_t).offset(i as isize)
                as *mut *mut int8_t as *mut ::core::ffi::c_void,
        );
        av_freep(
            (&raw mut (*s).motion_val_base as *mut *mut [int16_t; 2]).offset(i as isize)
                as *mut *mut [int16_t; 2] as *mut ::core::ffi::c_void,
        );
        (*s).cur_pic.ref_index[i as usize] = ::core::ptr::null_mut::<int8_t>();
        (*s).cur_pic.motion_val[i as usize] = ::core::ptr::null_mut::<[int16_t; 2]>();
        i += 1;
    }
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
pub struct AVDictionary {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVCodecDescriptor {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVCodecInternal {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct MPVEncContext {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct ThreadFrameProgress {
    _opaque: [u8; 0],
}
