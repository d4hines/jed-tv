// Translated from FFmpeg 9.0.1 libavcodec/options.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Copyright (c) 2001 Fabrice Bellard
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
    fn abort() -> !;
    fn ff_codec_close(avctx: *mut AVCodecContext);
    fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_codec_iterate(opaque: *mut *mut ::core::ffi::c_void) -> *const AVCodec;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_opt_set_defaults(s: *mut ::core::ffi::c_void);
    fn av_opt_set_defaults2(
        s: *mut ::core::ffi::c_void,
        mask: ::core::ffi::c_int,
        flags: ::core::ffi::c_int,
    );
    fn av_channel_layout_uninit(channel_layout: *mut AVChannelLayout);
    fn av_opt_set(
        obj: *mut ::core::ffi::c_void,
        name: *const ::core::ffi::c_char,
        val: *const ::core::ffi::c_char,
        search_flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn avcodec_default_get_buffer2(
        s: *mut AVCodecContext,
        frame: *mut AVFrame,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn avcodec_default_get_encode_buffer(
        s: *mut AVCodecContext,
        pkt: *mut AVPacket,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn avcodec_default_get_format(
        s: *mut AVCodecContext,
        fmt: *const AVPixelFormat,
    ) -> AVPixelFormat;
    fn avcodec_default_execute(
        c: *mut AVCodecContext,
        func: Option<
            unsafe extern "C" fn(
                *mut AVCodecContext,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        arg: *mut ::core::ffi::c_void,
        ret: *mut ::core::ffi::c_int,
        count: ::core::ffi::c_int,
        size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn avcodec_default_execute2(
        c: *mut AVCodecContext,
        func: Option<
            unsafe extern "C" fn(
                *mut AVCodecContext,
                *mut ::core::ffi::c_void,
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >,
        arg: *mut ::core::ffi::c_void,
        ret: *mut ::core::ffi::c_int,
        count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
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
pub type size_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
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
pub struct AVOptionRanges {
    pub range: *mut *mut AVOptionRange,
    pub nb_ranges: ::core::ffi::c_int,
    pub nb_components: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionRange {
    pub str_0: *const ::core::ffi::c_char,
    pub value_min: ::core::ffi::c_double,
    pub value_max: ::core::ffi::c_double,
    pub component_min: ::core::ffi::c_double,
    pub component_max: ::core::ffi::c_double,
    pub is_range: ::core::ffi::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOption {
    pub name: *const ::core::ffi::c_char,
    pub help: *const ::core::ffi::c_char,
    pub offset: ::core::ffi::c_int,
    pub type_0: AVOptionType,
    pub default_val: C2RustUnnamed,
    pub min: ::core::ffi::c_double,
    pub max: ::core::ffi::c_double,
    pub flags: ::core::ffi::c_int,
    pub unit: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub i64_0: int64_t,
    pub dbl: ::core::ffi::c_double,
    pub str_0: *const ::core::ffi::c_char,
    pub q: AVRational,
    pub arr: *const AVOptionArrayDef,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVOptionArrayDef {
    pub def: *const ::core::ffi::c_char,
    pub size_min: ::core::ffi::c_uint,
    pub size_max: ::core::ffi::c_uint,
    pub sep: ::core::ffi::c_char,
}
pub type AVOptionType = ::core::ffi::c_uint;
pub const AV_OPT_TYPE_FLAG_ARRAY: AVOptionType = 65536;
pub const AV_OPT_TYPE_UINT: AVOptionType = 20;
pub const AV_OPT_TYPE_CHLAYOUT: AVOptionType = 19;
pub const AV_OPT_TYPE_BOOL: AVOptionType = 18;
pub const AV_OPT_TYPE_COLOR: AVOptionType = 17;
pub const AV_OPT_TYPE_DURATION: AVOptionType = 16;
pub const AV_OPT_TYPE_VIDEO_RATE: AVOptionType = 15;
pub const AV_OPT_TYPE_SAMPLE_FMT: AVOptionType = 14;
pub const AV_OPT_TYPE_PIXEL_FMT: AVOptionType = 13;
pub const AV_OPT_TYPE_IMAGE_SIZE: AVOptionType = 12;
pub const AV_OPT_TYPE_CONST: AVOptionType = 11;
pub const AV_OPT_TYPE_UINT64: AVOptionType = 10;
pub const AV_OPT_TYPE_DICT: AVOptionType = 9;
pub const AV_OPT_TYPE_BINARY: AVOptionType = 8;
pub const AV_OPT_TYPE_RATIONAL: AVOptionType = 7;
pub const AV_OPT_TYPE_STRING: AVOptionType = 6;
pub const AV_OPT_TYPE_FLOAT: AVOptionType = 5;
pub const AV_OPT_TYPE_DOUBLE: AVOptionType = 4;
pub const AV_OPT_TYPE_INT64: AVOptionType = 3;
pub const AV_OPT_TYPE_INT: AVOptionType = 2;
pub const AV_OPT_TYPE_FLAGS: AVOptionType = 1;
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
    pub u: C2RustUnnamed_0,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type AVSubtitleType = ::core::ffi::c_uint;
pub const SUBTITLE_ASS: AVSubtitleType = 3;
pub const SUBTITLE_TEXT: AVSubtitleType = 2;
pub const SUBTITLE_BITMAP: AVSubtitleType = 1;
pub const SUBTITLE_NONE: AVSubtitleType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitleRect {
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub nb_colors: ::core::ffi::c_int,
    pub data: [*mut uint8_t; 4],
    pub linesize: [::core::ffi::c_int; 4],
    pub flags: ::core::ffi::c_int,
    pub type_0: AVSubtitleType,
    pub text: *mut ::core::ffi::c_char,
    pub ass: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVSubtitle {
    pub format: uint16_t,
    pub start_display_time: uint32_t,
    pub end_display_time: uint32_t,
    pub num_rects: ::core::ffi::c_uint,
    pub rects: *mut *mut AVSubtitleRect,
    pub pts: int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFCodecDefault {
    pub key: *const ::core::ffi::c_char,
    pub value: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFCodec {
    pub p: AVCodec,
    pub caps_internal_is_decoder_color_ranges_alpha_modes_cb_type: [u8; 4],
    pub priv_data_size: ::core::ffi::c_int,
    pub update_thread_context: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVCodecContext,
        ) -> ::core::ffi::c_int,
    >,
    pub update_thread_context_for_user: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVCodecContext,
        ) -> ::core::ffi::c_int,
    >,
    pub defaults: *const FFCodecDefault,
    pub init: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ::core::ffi::c_int>,
    pub cb: C2RustUnnamed_4,
    pub close: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ::core::ffi::c_int>,
    pub flush: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ()>,
    pub bsfs: *const ::core::ffi::c_char,
    pub hw_configs: *const *const AVCodecHWConfigInternal,
    pub codec_tags: *const uint32_t,
    pub get_supported_config: Option<
        unsafe extern "C" fn(
            *const AVCodecContext,
            *const AVCodec,
            AVCodecConfig,
            ::core::ffi::c_uint,
            *mut *const ::core::ffi::c_void,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub c2rust_unnamed: C2RustUnnamed_1,
}
impl FFCodec {
    pub fn caps_internal(&self) -> ::core::ffi::c_uint {
        crate::rt::bf_get(&self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 0, 23) as ::core::ffi::c_uint
    }
    pub fn set_caps_internal(&mut self, v: ::core::ffi::c_uint) {
        crate::rt::bf_set(&mut self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 0, 23, v as u64)
    }
    pub fn is_decoder(&self) -> ::core::ffi::c_uint {
        crate::rt::bf_get(&self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 24, 24) as ::core::ffi::c_uint
    }
    pub fn set_is_decoder(&mut self, v: ::core::ffi::c_uint) {
        crate::rt::bf_set(&mut self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 24, 24, v as u64)
    }
    pub fn color_ranges(&self) -> ::core::ffi::c_uint {
        crate::rt::bf_get(&self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 25, 26) as ::core::ffi::c_uint
    }
    pub fn set_color_ranges(&mut self, v: ::core::ffi::c_uint) {
        crate::rt::bf_set(&mut self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 25, 26, v as u64)
    }
    pub fn alpha_modes(&self) -> ::core::ffi::c_uint {
        crate::rt::bf_get(&self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 27, 28) as ::core::ffi::c_uint
    }
    pub fn set_alpha_modes(&mut self, v: ::core::ffi::c_uint) {
        crate::rt::bf_set(&mut self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 27, 28, v as u64)
    }
    pub fn cb_type(&self) -> ::core::ffi::c_uint {
        crate::rt::bf_get(&self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 29, 31) as ::core::ffi::c_uint
    }
    pub fn set_cb_type(&mut self, v: ::core::ffi::c_uint) {
        crate::rt::bf_set(&mut self.caps_internal_is_decoder_color_ranges_alpha_modes_cb_type, 29, 31, v as u64)
    }
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub c2rust_unnamed: C2RustUnnamed_3,
    pub c2rust_unnamed_0: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub ch_layouts: *const AVChannelLayout,
    pub supported_samplerates: *const ::core::ffi::c_int,
    pub sample_fmts: *const AVSampleFormat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub supported_framerates: *const AVRational,
    pub pix_fmts: *const AVPixelFormat,
}
pub type AVCodecConfig = ::core::ffi::c_uint;
pub const AV_CODEC_CONFIG_ALPHA_MODE: AVCodecConfig = 7;
pub const AV_CODEC_CONFIG_COLOR_SPACE: AVCodecConfig = 6;
pub const AV_CODEC_CONFIG_COLOR_RANGE: AVCodecConfig = 5;
pub const AV_CODEC_CONFIG_CHANNEL_LAYOUT: AVCodecConfig = 4;
pub const AV_CODEC_CONFIG_SAMPLE_FORMAT: AVCodecConfig = 3;
pub const AV_CODEC_CONFIG_SAMPLE_RATE: AVCodecConfig = 2;
pub const AV_CODEC_CONFIG_FRAME_RATE: AVCodecConfig = 1;
pub const AV_CODEC_CONFIG_PIX_FORMAT: AVCodecConfig = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub decode: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVFrame,
            *mut ::core::ffi::c_int,
            *mut AVPacket,
        ) -> ::core::ffi::c_int,
    >,
    pub decode_sub: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVSubtitle,
            *mut ::core::ffi::c_int,
            *const AVPacket,
        ) -> ::core::ffi::c_int,
    >,
    pub receive_frame: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> ::core::ffi::c_int,
    >,
    pub encode: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut AVPacket,
            *const AVFrame,
            *mut ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
    pub encode_sub: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *mut uint8_t,
            ::core::ffi::c_int,
            *const AVSubtitle,
        ) -> ::core::ffi::c_int,
    >,
    pub receive_packet: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVPacket) -> ::core::ffi::c_int,
    >,
}
unsafe extern "C" fn context_to_name(
    mut ptr: *mut ::core::ffi::c_void,
) -> *const ::core::ffi::c_char {
    let mut avc: *mut AVCodecContext = ptr as *mut AVCodecContext;
    if !avc.is_null() && !(*avc).codec.is_null() {
        return (*(*avc).codec).name
    } else {
        return b"NULL\0" as *const u8 as *const ::core::ffi::c_char
    };
}
static mut avcodec_options: [AVOption; 358] = [AVOption {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    help: ::core::ptr::null::<::core::ffi::c_char>(),
    offset: 0,
    type_0: 0 as AVOptionType,
    default_val: C2RustUnnamed { i64_0: 0 },
    min: 0.,
    max: 0.,
    flags: 0,
    unit: ::core::ptr::null::<::core::ffi::c_char>(),
}; 358];
unsafe extern "C" fn codec_child_next(
    mut obj: *mut ::core::ffi::c_void,
    mut prev: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut s: *mut AVCodecContext = obj as *mut AVCodecContext;
    if prev.is_null() && !(*s).codec.is_null() && !(*(*s).codec).priv_class.is_null()
        && !(*s).priv_data.is_null()
    {
        return (*s).priv_data;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
unsafe extern "C" fn codec_child_class_iterate(
    mut iter: *mut *mut ::core::ffi::c_void,
) -> *const AVClass {
    let mut c: *const AVCodec = ::core::ptr::null::<AVCodec>();
    loop {
        c = av_codec_iterate(iter);
        if c.is_null() {
            break;
        }
        if !(*c).priv_class.is_null() {
            return (*c).priv_class;
        }
    }
    return ::core::ptr::null::<AVClass>();
}
unsafe extern "C" fn get_category(mut ptr: *mut ::core::ffi::c_void) -> AVClassCategory {
    let mut avctx: *mut AVCodecContext = ptr as *mut AVCodecContext;
    if !(*avctx).codec.is_null()
        && ff_codec_is_decoder((*avctx).codec as *const AVCodec) != 0
    {
        return AV_CLASS_CATEGORY_DECODER
    } else {
        return AV_CLASS_CATEGORY_ENCODER
    };
}
static mut av_codec_context_class: AVClass = unsafe {
    AVClass {
        class_name: b"AVCodecContext\0" as *const u8 as *const ::core::ffi::c_char,
        item_name: Some(
            context_to_name
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *const ::core::ffi::c_char,
        ),
        option: &raw const avcodec_options as *const AVOption,
        version: (61 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            | 101 as ::core::ffi::c_int,
        log_level_offset_offset: 8 as ::core::ffi::c_ulong as ::core::ffi::c_int,
        parent_log_context_offset: 0,
        category: AV_CLASS_CATEGORY_ENCODER,
        get_category: Some(
            get_category
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> AVClassCategory,
        ),
        query_ranges: None,
        child_next: Some(
            codec_child_next
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_void,
        ),
        child_class_iterate: Some(
            codec_child_class_iterate
                as unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> *const AVClass,
        ),
        state_flags_offset: 0,
    }
};
unsafe extern "C" fn init_context_defaults(
    mut s: *mut AVCodecContext,
    mut codec: *const AVCodec,
) -> ::core::ffi::c_int {
    let codec2: *const FFCodec = ffcodec(codec) as *const FFCodec;
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    memset(
        s as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AVCodecContext>() as size_t,
    );
    (*s).av_class = &raw const av_codec_context_class;
    (*s).codec_type = (if !codec.is_null() {
        (*codec).type_0 as ::core::ffi::c_int
    } else {
        AVMEDIA_TYPE_UNKNOWN as ::core::ffi::c_int
    }) as AVMediaType;
    if !codec.is_null() {
        (*s).codec = codec as *const AVCodec;
        (*s).codec_id = (*codec).id;
    }
    if (*s).codec_type as ::core::ffi::c_int == AVMEDIA_TYPE_AUDIO as ::core::ffi::c_int
    {
        flags = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
    } else if (*s).codec_type as ::core::ffi::c_int
        == AVMEDIA_TYPE_VIDEO as ::core::ffi::c_int
    {
        flags = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
    } else if (*s).codec_type as ::core::ffi::c_int
        == AVMEDIA_TYPE_SUBTITLE as ::core::ffi::c_int
    {
        flags = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
    }
    av_opt_set_defaults2(s as *mut ::core::ffi::c_void, flags, flags);
    av_channel_layout_uninit(&raw mut (*s).ch_layout);
    (*s).time_base = AVRational {
        num: 0 as ::core::ffi::c_int,
        den: 1 as ::core::ffi::c_int,
    };
    (*s).framerate = AVRational {
        num: 0 as ::core::ffi::c_int,
        den: 1 as ::core::ffi::c_int,
    };
    (*s).pkt_timebase = AVRational {
        num: 0 as ::core::ffi::c_int,
        den: 1 as ::core::ffi::c_int,
    };
    (*s).get_buffer2 = Some(
        avcodec_default_get_buffer2
            as unsafe extern "C" fn(
                *mut AVCodecContext,
                *mut AVFrame,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut AVCodecContext,
                *mut AVFrame,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    (*s).get_format = Some(
        avcodec_default_get_format
            as unsafe extern "C" fn(
                *mut AVCodecContext,
                *const AVPixelFormat,
            ) -> AVPixelFormat,
    )
        as Option<
            unsafe extern "C" fn(
                *mut AVCodecContext,
                *const AVPixelFormat,
            ) -> AVPixelFormat,
        >;
    (*s).get_encode_buffer = Some(
        avcodec_default_get_encode_buffer
            as unsafe extern "C" fn(
                *mut AVCodecContext,
                *mut AVPacket,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut AVCodecContext,
                *mut AVPacket,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    (*s).execute = Some(
        avcodec_default_execute
            as unsafe extern "C" fn(
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
    )
        as Option<
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
        >;
    (*s).execute2 = Some(
        avcodec_default_execute2
            as unsafe extern "C" fn(
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
    )
        as Option<
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
        >;
    (*s).sample_aspect_ratio = AVRational {
        num: 0 as ::core::ffi::c_int,
        den: 1 as ::core::ffi::c_int,
    };
    (*s).ch_layout.order = AV_CHANNEL_ORDER_UNSPEC;
    (*s).pix_fmt = AV_PIX_FMT_NONE;
    (*s).sw_pix_fmt = AV_PIX_FMT_NONE;
    (*s).sample_fmt = AV_SAMPLE_FMT_NONE;
    if !codec.is_null() && (*codec2).priv_data_size != 0 {
        (*s).priv_data = av_mallocz((*codec2).priv_data_size as size_t);
        if (*s).priv_data.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
        if !(*codec).priv_class.is_null() {
            let ref mut fresh0 = *((*s).priv_data as *mut *const AVClass);
            *fresh0 = (*codec).priv_class;
            av_opt_set_defaults((*s).priv_data);
        }
    }
    if !codec.is_null() && !(*codec2).defaults.is_null() {
        let mut ret: ::core::ffi::c_int = 0;
        let mut d: *const FFCodecDefault = (*codec2).defaults;
        while !(*d).key.is_null() {
            ret = av_opt_set(
                s as *mut ::core::ffi::c_void,
                (*d).key,
                (*d).value,
                0 as ::core::ffi::c_int,
            );
            if !(ret >= 0 as ::core::ffi::c_int) {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    0 as ::core::ffi::c_int,
                    b"Assertion %s failed at %s:%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"ret >= 0\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/libavcodec/options.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    142 as ::core::ffi::c_int,
                );
                abort();
            }
            d = d.offset(1);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_alloc_context3(
    mut codec: *const AVCodec,
) -> *mut AVCodecContext {
    let mut avctx: *mut AVCodecContext = av_malloc(
        ::core::mem::size_of::<AVCodecContext>() as size_t,
    ) as *mut AVCodecContext;
    if avctx.is_null() {
        return ::core::ptr::null_mut::<AVCodecContext>();
    }
    if init_context_defaults(avctx, codec) < 0 as ::core::ffi::c_int {
        av_free(avctx as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<AVCodecContext>();
    }
    return avctx;
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_free_context(mut pavctx: *mut *mut AVCodecContext) {
    let mut avctx: *mut AVCodecContext = *pavctx;
    if avctx.is_null() {
        return;
    }
    ff_codec_close(avctx as *mut AVCodecContext);
    av_freep(&raw mut (*avctx).extradata as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*avctx).subtitle_header as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*avctx).intra_matrix as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*avctx).chroma_intra_matrix as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*avctx).inter_matrix as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*avctx).rc_override as *mut ::core::ffi::c_void);
    av_channel_layout_uninit(&raw mut (*avctx).ch_layout);
    av_freep(pavctx as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_get_class() -> *const AVClass {
    return &raw const av_codec_context_class;
}
static mut subtitle_rect_options: [AVOption; 8] = [
    AVOption {
        name: b"x\0" as *const u8 as *const ::core::ffi::c_char,
        help: b"\0" as *const u8 as *const ::core::ffi::c_char,
        offset: 0 as ::core::ffi::c_ulong as ::core::ffi::c_int,
        type_0: AV_OPT_TYPE_INT,
        default_val: C2RustUnnamed {
            i64_0: 0 as ::core::ffi::c_int as int64_t,
        },
        min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
        flags: 0 as ::core::ffi::c_int,
        unit: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVOption {
        name: b"y\0" as *const u8 as *const ::core::ffi::c_char,
        help: b"\0" as *const u8 as *const ::core::ffi::c_char,
        offset: 4 as ::core::ffi::c_ulong as ::core::ffi::c_int,
        type_0: AV_OPT_TYPE_INT,
        default_val: C2RustUnnamed {
            i64_0: 0 as ::core::ffi::c_int as int64_t,
        },
        min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
        flags: 0 as ::core::ffi::c_int,
        unit: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVOption {
        name: b"w\0" as *const u8 as *const ::core::ffi::c_char,
        help: b"\0" as *const u8 as *const ::core::ffi::c_char,
        offset: 8 as ::core::ffi::c_ulong as ::core::ffi::c_int,
        type_0: AV_OPT_TYPE_INT,
        default_val: C2RustUnnamed {
            i64_0: 0 as ::core::ffi::c_int as int64_t,
        },
        min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
        flags: 0 as ::core::ffi::c_int,
        unit: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVOption {
        name: b"h\0" as *const u8 as *const ::core::ffi::c_char,
        help: b"\0" as *const u8 as *const ::core::ffi::c_char,
        offset: 12 as ::core::ffi::c_ulong as ::core::ffi::c_int,
        type_0: AV_OPT_TYPE_INT,
        default_val: C2RustUnnamed {
            i64_0: 0 as ::core::ffi::c_int as int64_t,
        },
        min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
        flags: 0 as ::core::ffi::c_int,
        unit: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVOption {
        name: b"type\0" as *const u8 as *const ::core::ffi::c_char,
        help: b"\0" as *const u8 as *const ::core::ffi::c_char,
        offset: 76 as ::core::ffi::c_ulong as ::core::ffi::c_int,
        type_0: AV_OPT_TYPE_INT,
        default_val: C2RustUnnamed {
            i64_0: 0 as ::core::ffi::c_int as int64_t,
        },
        min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
        flags: 0 as ::core::ffi::c_int,
        unit: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVOption {
        name: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        help: b"\0" as *const u8 as *const ::core::ffi::c_char,
        offset: 72 as ::core::ffi::c_ulong as ::core::ffi::c_int,
        type_0: AV_OPT_TYPE_FLAGS,
        default_val: C2RustUnnamed {
            i64_0: 0 as ::core::ffi::c_int as int64_t,
        },
        min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        max: 1 as ::core::ffi::c_int as ::core::ffi::c_double,
        flags: 0 as ::core::ffi::c_int,
        unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
    },
    AVOption {
        name: b"forced\0" as *const u8 as *const ::core::ffi::c_char,
        help: b"\0" as *const u8 as *const ::core::ffi::c_char,
        offset: 72 as ::core::ffi::c_ulong as ::core::ffi::c_int,
        type_0: AV_OPT_TYPE_FLAGS,
        default_val: C2RustUnnamed {
            i64_0: 0 as ::core::ffi::c_int as int64_t,
        },
        min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        max: 1 as ::core::ffi::c_int as ::core::ffi::c_double,
        flags: 0 as ::core::ffi::c_int,
        unit: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVOption {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        help: ::core::ptr::null::<::core::ffi::c_char>(),
        offset: 0,
        type_0: 0 as AVOptionType,
        default_val: C2RustUnnamed { i64_0: 0 },
        min: 0.,
        max: 0.,
        flags: 0,
        unit: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
static mut av_subtitle_rect_class: AVClass = unsafe {
    AVClass {
        class_name: b"AVSubtitleRect\0" as *const u8 as *const ::core::ffi::c_char,
        item_name: None,
        option: &raw const subtitle_rect_options as *const AVOption,
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
#[no_mangle]
pub unsafe extern "C" fn avcodec_get_subtitle_rect_class() -> *const AVClass {
    return &raw const av_subtitle_rect_class;
}
#[inline(always)]
unsafe extern "C" fn ffcodec(mut codec: *const AVCodec) -> *const FFCodec {
    return codec as *const FFCodec;
}
#[inline]
unsafe extern "C" fn ff_codec_is_decoder(
    mut avcodec: *const AVCodec,
) -> ::core::ffi::c_int {
    let codec: *const FFCodec = ffcodec(avcodec) as *const FFCodec;
    return (*codec).is_decoder() as ::core::ffi::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    avcodec_options = [
        AVOption {
            name: b"b\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set bitrate (in bits/s)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 56 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT64,
            default_val: C2RustUnnamed {
                i64_0: (200 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int)
                    as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0x7fffffffffffffff as ::core::ffi::c_long as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"ab\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set bitrate (in bits/s)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 56 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT64,
            default_val: C2RustUnnamed {
                i64_0: (128 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int)
                    as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"bt\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Set video bitrate tolerance (in bits/s). In 1-pass mode, bitrate tolerance specifies how far ratecontrol is willing to deviate from the target average bitrate value. This is not related to minimum/maximum bitrate. Lowering tolerance too much has an adverse effect on quality.\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 416 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: (200 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int
                    * 20 as ::core::ffi::c_int) as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 64 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLAGS,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unaligned\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"allow decoders to produce unaligned output\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"mv4\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"use four motion vectors per macroblock (MPEG-4)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"qpel\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"use 1/4-pel motion compensation\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"loop\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"use loop filter\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"qscale\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"use fixed qscale\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"recon_frame\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"export reconstructed frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"copy_opaque\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"propagate opaque values\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"frame_duration\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"use frame durations\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"pass1\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"use internal 2-pass ratecontrol in first  pass mode\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"pass2\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"use internal 2-pass ratecontrol in second pass mode\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"gray\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"only decode/encode grayscale\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"psnr\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"error[?] variables will be set during encoding\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ildct\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"use interlaced DCT\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"low_delay\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"force low delay\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 19 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"global_header\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"place global headers in extradata instead of every keyframe\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bitexact\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"use only bitexact functions (except (I)DCT)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 23 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"aic\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"H.263 advanced intra coding / MPEG-4 AC prediction\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ilme\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"interlaced motion estimation\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 29 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"cgop\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"closed GOP\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int)
                    as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"output_corrupt\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Output even potentially corrupted frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"flags2\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 68 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLAGS,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int,
            unit: b"flags2\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"fast\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"allow non-spec-compliant speedup tricks\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags2\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"noout\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"skip bitstream encoding\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags2\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ignorecrop\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"ignore cropping information from sps\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"flags2\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"local_header\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"place global headers at every keyframe instead of in extradata\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags2\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"chunks\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Frame data might be split into multiple chunks\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"flags2\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"showall\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Show all frames before the first keyframe\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"flags2\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"export_mvs\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"export motion vectors through frame side data\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"flags2\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"skip_manual\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"do not skip samples and export skip information as frame side data\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 29 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"flags2\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ass_ro_flush_noop\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"do not reset ASS ReadOrder field on flush\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"flags2\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"icc_profiles\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"generate/parse embedded ICC profiles from/to colorimetry tags\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_uint) << 31 as ::core::ffi::c_int)
                    as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"flags2\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"fixed_frame_size\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Force a fixed frame size\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"flags2\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"export_side_data\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Export metadata as side data\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 788 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLAGS,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"export_side_data\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"mvs\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"export motion vectors through frame side data\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"export_side_data\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"prft\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"export Producer Reference Time through packet side data\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"export_side_data\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"venc_params\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"export video encoding parameters through frame side data\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"export_side_data\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"film_grain\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"export film grain parameters through frame side data\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"export_side_data\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"enhancements\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"export picture enhancement metadata through frame side data\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"export_side_data\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"time_base\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 84 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_RATIONAL,
            default_val: C2RustUnnamed {
                dbl: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"g\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set the group of picture (GOP) size\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 332 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 12 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"ar\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set audio sampling rate (in Hz)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 344 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"cutoff\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set cutoff bandwidth\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 384 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"frame_size\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 376 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"frame_number\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 824 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT64,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"delay\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 108 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"qcomp\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"video quantizer scale compression (VBR). Constant of ratecontrol equation. Recommended range for default rc_eq: 0.0-1.0\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 428 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLOAT,
            default_val: C2RustUnnamed { dbl: 0.5f64 },
            min: -3.40282347e+38f32 as ::core::ffi::c_double,
            max: 3.40282347e+38f64,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"qblur\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"video quantizer scale blur (VBR)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 432 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLOAT,
            default_val: C2RustUnnamed { dbl: 0.5f64 },
            min: -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            max: 3.40282347e+38f64,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"qmin\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"minimum video quantizer scale (VBR)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 436 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 2 as ::core::ffi::c_int as int64_t,
            },
            min: -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            max: 69 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"qmax\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"maximum video quantizer scale (VBR)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 440 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 31 as ::core::ffi::c_int as int64_t,
            },
            min: -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            max: 1024 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"qdiff\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"maximum difference between the quantizer scales (VBR)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 444 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 3 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"bf\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set maximum number of B-frames between non-B-frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 200 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"b_qfactor\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"QP factor between P- and B-frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 204 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLOAT,
            default_val: C2RustUnnamed { dbl: 1.25f64 },
            min: -3.40282347e+38f32 as ::core::ffi::c_double,
            max: 3.40282347e+38f64,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"codec_tag\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 28 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"work around not autodetected encoder bugs\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 512 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLAGS,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"autodetect\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"xvid_ilace\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Xvid interlacing bug (autodetected if FOURCC == XVIX)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 4 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ump4\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"(autodetected if FOURCC == UMP4)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 8 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"no_padding\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"padding bug (autodetected)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 16 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"amv\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 32 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"qpel_chroma\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 64 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"std_qpel\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"old standard qpel (autodetected per FOURCC/version)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 128 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"qpel_chroma2\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 256 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"direct_blocksize\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"direct-qpel-blocksize bug (autodetected per FOURCC/version)\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 512 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"edge\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"edge padding bug (autodetected per FOURCC/version)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 1024 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"hpel_chroma\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 2048 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"dc_clip\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 4096 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ms\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"work around various bugs in Microsoft's broken decoders\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 8192 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"trunc\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"truncated frames\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 16384 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"iedge\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 32768 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"bug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"strict\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"how strictly to follow the standards\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 516 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"strict\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"very\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"strictly conform to a older more strict version of the spec or reference software\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 2 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"strict\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"strict\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"strictly conform to all the things in the spec no matter what the consequences\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"strict\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"normal\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"strict\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unofficial\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"allow unofficial extensions\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: -(1 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"strict\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"experimental\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"allow non-standardized experimental things\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: -(2 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"strict\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"b_qoffset\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"QP offset between P- and B-frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 208 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLOAT,
            default_val: C2RustUnnamed { dbl: 1.25f64 },
            min: -3.40282347e+38f32 as ::core::ffi::c_double,
            max: 3.40282347e+38f64,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"err_detect\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set error detection flags\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 528 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLAGS,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"err_detect\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"crccheck\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"verify embedded CRCs\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"err_detect\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bitstream\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"detect bitstream specification deviations\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"err_detect\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"buffer\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"detect improper bitstream length\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"err_detect\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"explode\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"abort decoding on minor error detection\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"err_detect\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ignore_err\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"ignore errors\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"err_detect\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"careful\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"consider things that violate the spec, are fast to check and have not been seen in the wild as errors\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"err_detect\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"compliant\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"consider all spec non compliancies as errors\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"err_detect\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"aggressive\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"consider things that a sane encoder should not do as an error\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"err_detect\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"has_b_frames\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 172 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"block_align\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 380 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"rc_override_count\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 452 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"maxrate\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"maximum bitrate (in bits/s). Used for VBV together with bufsize.\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 464 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT64,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"minrate\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"minimum bitrate (in bits/s). Most useful in setting up a CBR encode. It is of little use otherwise.\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 472 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT64,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"bufsize\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set ratecontrol buffer size (in bits)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 448 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"i_qfactor\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"QP factor between P- and I-frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 212 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLOAT,
            default_val: C2RustUnnamed { dbl: -0.8f64 },
            min: -3.40282347e+38f32 as ::core::ffi::c_double,
            max: 3.40282347e+38f64,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"i_qoffset\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"QP offset between P- and I-frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 216 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLOAT,
            default_val: C2RustUnnamed { dbl: 0.0f64 },
            min: -3.40282347e+38f32 as ::core::ffi::c_double,
            max: 3.40282347e+38f64,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"dct\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"DCT algorithm\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 640 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"dct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"auto\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"autoselect a good one\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"dct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"fastint\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"fast integer\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"dct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"int\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"accurate integer\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 2 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"dct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"mmx\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 3 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"dct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"altivec\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 5 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"dct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"faan\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"floating point AAN DCT\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 6 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"dct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"neon\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 7 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"dct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"lumi_mask\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"compresses bright areas stronger than medium ones\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 220 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLOAT,
            default_val: C2RustUnnamed {
                dbl: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            },
            min: -3.40282347e+38f32 as ::core::ffi::c_double,
            max: 3.40282347e+38f64,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"tcplx_mask\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"temporal complexity masking\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 224 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLOAT,
            default_val: C2RustUnnamed {
                dbl: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            },
            min: -3.40282347e+38f32 as ::core::ffi::c_double,
            max: 3.40282347e+38f64,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"scplx_mask\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"spatial complexity masking\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 228 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLOAT,
            default_val: C2RustUnnamed {
                dbl: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            },
            min: -3.40282347e+38f32 as ::core::ffi::c_double,
            max: 3.40282347e+38f64,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"p_mask\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"inter masking\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 232 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLOAT,
            default_val: C2RustUnnamed {
                dbl: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            },
            min: -3.40282347e+38f32 as ::core::ffi::c_double,
            max: 3.40282347e+38f64,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"dark_mask\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"compresses dark areas stronger than medium ones\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 236 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLOAT,
            default_val: C2RustUnnamed {
                dbl: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            },
            min: -3.40282347e+38f32 as ::core::ffi::c_double,
            max: 3.40282347e+38f64,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"select IDCT implementation\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 644 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"auto\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"int\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"simple\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 2 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"simplemmx\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 3 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"arm\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 7 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"altivec\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 8 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"simplearm\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 10 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"simplearmv5te\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 16 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"simplearmv6\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 17 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"simpleneon\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 22 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"xvid\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 14 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"xvidmmx\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"deprecated, for compatibility only\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 14 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"faani\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"floating point AAN IDCT\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 20 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"simpleauto\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 128 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"idct\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ec\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set error concealment strategy\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 520 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLAGS,
            default_val: C2RustUnnamed {
                i64_0: 3 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"ec\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"guess_mvs\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"iterative motion vector (MV) search (slow)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"ec\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"deblock\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"use strong deblock filter for damaged MBs\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 2 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"ec\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"favor_inter\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"favor predicting from the previous frame\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 256 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"ec\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bits_per_coded_sample\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 648 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"aspect\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"sample aspect ratio\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 128 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_RATIONAL,
            default_val: C2RustUnnamed {
                dbl: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 10 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"sar\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"sample aspect ratio\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 128 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_RATIONAL,
            default_val: C2RustUnnamed {
                dbl: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 10 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"print specific debug info\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 524 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLAGS,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"pict\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"picture info\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"rc\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"rate control\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 2 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bitstream\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 4 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"mb_type\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"macroblock (MB) type\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 8 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"qp\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"per-block quantization parameter (QP)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 16 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"dct_coeff\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0x40 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"green_metadata\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0x800000 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"skip\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0x80 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"startcode\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0x100 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"er\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"error recognition\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0x400 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"mmco\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"memory management control operations (H.264)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0x800 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bugs\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0x1000 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"buffers\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"picture buffer allocations\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0x8000 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"thread_ops\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"threading operations\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0x10000 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"nomc\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"skip motion compensation\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0x1000000 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"debug\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"dia_size\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"diamond type & size for motion estimation\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 260 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"last_pred\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"amount of motion predictors from the previous frame\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 264 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"pre_dia_size\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"diamond type & size for motion estimation pre-pass\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 272 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"subq\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"sub-pel motion estimation quality\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 276 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 8 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"me_range\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"limit motion vectors range (1023 for DivX player)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 280 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"global_quality\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 420 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"slice_flags\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 176 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"mbd\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"macroblock decision algorithm (high quality mode)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 284 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"mbd\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"simple\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"use mbcmp\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"mbd\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bits\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"use fewest bits\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"mbd\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"rd\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"use best rate distortion\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 2 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"mbd\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"rc_init_occupancy\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"number of bits which should be loaded into the rc buffer before decoding starts\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 488 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"threads\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set the number of threads\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 656 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"threads\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"auto\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"autodetect a suitable number of threads to use\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"threads\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"dc\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"deprecated; use intra_dc_precision for MPEG-2 instead\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 312 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: -(8 as ::core::ffi::c_int) as ::core::ffi::c_double,
            max: 16 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"nssew\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"nsse weight\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 240 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 8 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"skip_top\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"number of macroblock rows at the top which are skipped\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 712 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"skip_bottom\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"number of macroblock rows at the bottom which are skipped\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 716 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"profile\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 688 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: -(99 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int,
            unit: b"avctx.profile\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: -(99 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"avctx.profile\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"main10\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 2 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"avctx.profile\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"level\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"encoding level, usually corresponding to the profile level, codec-specific\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 692 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: -(99 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int,
            unit: b"avctx.level\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: -(99 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"avctx.level\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"lowres\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"decode at 1= 1/2, 2=1/4, 3=1/8 resolutions\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 720 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"cmp\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"full-pel ME compare function\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 244 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"subcmp\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"sub-pel ME compare function\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 248 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"mbcmp\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"macroblock compare function\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 252 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ildctcmp\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"interlaced DCT compare function\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 256 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 8 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"precmp\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"pre motion estimation compare function\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 268 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"sad\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"sum of absolute differences, fast\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"sse\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"sum of squared errors\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"satd\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"sum of absolute Hadamard transformed differences\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 2 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"dct\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"sum of absolute DCT transformed differences\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 3 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"psnr\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"sum of squared quantization errors (avoid, low quality)\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 4 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bit\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"number of bits needed for the block\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 5 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"rd\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"rate distortion optimal, slow\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 6 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"zero\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"0\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 7 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"vsad\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"sum of absolute vertical differences\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 8 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"vsse\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"sum of squared vertical differences\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 9 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"nsse\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"noise preserving sum of squared differences\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 10 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"dctmax\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 13 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"chroma\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 256 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"msad\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"sum of absolute differences, median predicted\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 15 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"cmp_func\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"mblmin\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"minimum macroblock Lagrange factor (VBR)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 316 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: (118 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int64_t,
            },
            min: 1 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: (256 as ::core::ffi::c_int * 128 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"mblmax\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"maximum macroblock Lagrange factor (VBR)\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 320 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: (118 as ::core::ffi::c_int * 31 as ::core::ffi::c_int) as int64_t,
            },
            min: 1 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: (256 as ::core::ffi::c_int * 128 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"skip_loop_filter\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"skip loop filtering process for the selected frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 696 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: AVDISCARD_DEFAULT as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"avdiscard\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"skip_idct\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"skip IDCT/dequantization for the selected frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 700 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: AVDISCARD_DEFAULT as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"avdiscard\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"skip_frame\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"skip decoding for the selected frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 704 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: AVDISCARD_DEFAULT as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"avdiscard\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"none\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"discard no frame\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVDISCARD_NONE as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"avdiscard\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"default\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"discard useless frames\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVDISCARD_DEFAULT as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"avdiscard\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"noref\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"discard all non-reference frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVDISCARD_NONREF as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"avdiscard\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bidir\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"discard all bidirectional frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVDISCARD_BIDIR as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"avdiscard\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"nointra\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"discard all frames except I frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVDISCARD_NONINTRA as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"avdiscard\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"nokey\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"discard all frames except keyframes\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVDISCARD_NONKEY as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"avdiscard\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"all\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"discard all frames\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVDISCARD_ALL as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"avdiscard\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bidir_refine\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"refine the two motion vectors used in bidirectional macroblocks\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 324 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 4 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"keyint_min\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"minimum interval between IDR-frames\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 328 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 25 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"refs\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"reference frames to consider for motion compensation\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 168 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"trellis\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"rate-distortion optimal quantization\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 492 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"mv0_threshold\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 336 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 256 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"compression_level\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 424 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: -(1 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"bits_per_raw_sample\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 652 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"ch_layout\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 352 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CHLAYOUT,
            default_val: C2RustUnnamed {
                str_0: ::core::ptr::null::<::core::ffi::c_char>(),
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"ch_layout\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"rc_max_vbv_use\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 480 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLOAT,
            default_val: C2RustUnnamed {
                dbl: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            },
            min: 0.0f64,
            max: 3.40282347e+38f64,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"rc_min_vbv_use\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 484 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLOAT,
            default_val: C2RustUnnamed {
                dbl: 3 as ::core::ffi::c_int as ::core::ffi::c_double,
            },
            min: 0.0f64,
            max: 3.40282347e+38f64,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"color_primaries\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"color primaries\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 144 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: 1 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt709\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.709\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_BT709 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt470m\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.470 M\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_BT470M as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt470bg\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.470 BG\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_BT470BG as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte170m\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 170 M\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_SMPTE170M as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte240m\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 240 M\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_SMPTE240M as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"film\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Film\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_FILM as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt2020\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.2020\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_BT2020 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte428\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 428-1\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_SMPTE428 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte428_1\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 428-1\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_SMPTE428 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte431\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 431-2\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_SMPTE431 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte432\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 422-1\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_SMPTE432 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"jedec-p22\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"JEDEC P22\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_JEDEC_P22 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ebu3213\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"EBU 3213-E\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_EBU3213 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_PRI_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_primaries_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"color_trc\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"color transfer characteristics\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 148 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: 1 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt709\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.709\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_BT709 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"gamma22\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.470 M\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_GAMMA22 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt470m\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.470 M\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_GAMMA22 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"gamma28\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.470 BG\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_GAMMA28 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt470bg\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.470 BG\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_GAMMA28 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte170m\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 170 M\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_SMPTE170M as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte240m\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 240 M\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_SMPTE240M as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"linear\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Linear\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_LINEAR as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"log100\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Log\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_LOG as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"log316\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Log square root\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_LOG_SQRT as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"iec61966-2-4\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"IEC 61966-2-4\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_IEC61966_2_4 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt1361e\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.1361\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_BT1361_ECG as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"iec61966-2-1\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"IEC 61966-2-1\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_IEC61966_2_1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt2020-10\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.2020 - 10 bit\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_BT2020_10 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt2020-12\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.2020 - 12 bit\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_BT2020_12 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte2084\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 2084\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_SMPTE2084 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte428\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 428-1\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_SMPTE428 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"arib-std-b67\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"ARIB STD-B67\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_ARIB_STD_B67 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"log\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Log\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_LOG as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"log_sqrt\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Log square root\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_LOG_SQRT as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"iec61966_2_4\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"IEC 61966-2-4\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_IEC61966_2_4 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt1361\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.1361\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_BT1361_ECG as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"iec61966_2_1\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"IEC 61966-2-1\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_IEC61966_2_1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt2020_10bit\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.2020 - 10 bit\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_BT2020_10 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt2020_12bit\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.2020 - 12 bit\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_BT2020_12 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte428_1\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 428-1\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_TRC_SMPTE428 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_trc_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"colorspace\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"color space\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 152 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"rgb\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"RGB\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_RGB as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt709\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.709\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_BT709 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"fcc\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"FCC\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_FCC as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt470bg\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.470 BG\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_BT470BG as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte170m\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 170 M\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_SMPTE170M as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte240m\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 240 M\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_SMPTE240M as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ycgco\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"YCGCO\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_YCGCO as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt2020nc\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.2020 NCL\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_BT2020_NCL as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt2020c\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.2020 CL\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_BT2020_CL as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"smpte2085\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"SMPTE 2085\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_SMPTE2085 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"chroma-derived-nc\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Chroma-derived NCL\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_CHROMA_DERIVED_NCL as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"chroma-derived-c\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Chroma-derived CL\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_CHROMA_DERIVED_CL as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ictcp\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"ICtCp\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_ICTCP as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ipt-c2\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"IPT-C2\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_IPT_C2 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ycocg\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"YCGCO\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_YCGCO as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ycgco-re\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"YCgCo-R, even add.\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_YCGCO_RE as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ycgco-ro\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"YCgCo-R, odd add.\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_YCGCO_RO as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt2020_ncl\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.2020 NCL\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_BT2020_NCL as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt2020_cl\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"BT.2020 CL\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_SPC_BT2020_CL as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"colorspace_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"color_range\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"color range\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 156 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_RANGE_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_range_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_RANGE_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_range_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"tv\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"MPEG (219*2^(n-8))\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_RANGE_MPEG as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_range_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"pc\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"JPEG (2^n-1)\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_RANGE_JPEG as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_range_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_RANGE_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_range_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"mpeg\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"MPEG (219*2^(n-8))\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_RANGE_MPEG as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_range_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"jpeg\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"JPEG (2^n-1)\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_RANGE_JPEG as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_range_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"limited\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"MPEG (219*2^(n-8))\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_RANGE_MPEG as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_range_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"full\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"JPEG (2^n-1)\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCOL_RANGE_JPEG as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"color_range_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"chroma_sample_location\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"chroma sample location\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 160 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: AVCHROMA_LOC_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"chroma_sample_location_type\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCHROMA_LOC_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"chroma_sample_location_type\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"left\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Left\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCHROMA_LOC_LEFT as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"chroma_sample_location_type\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"center\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Center\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCHROMA_LOC_CENTER as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"chroma_sample_location_type\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"topleft\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Top-left\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCHROMA_LOC_TOPLEFT as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"chroma_sample_location_type\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"top\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Top\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCHROMA_LOC_TOP as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"chroma_sample_location_type\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bottomleft\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Bottom-left\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCHROMA_LOC_BOTTOMLEFT as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"chroma_sample_location_type\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bottom\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Bottom\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCHROMA_LOC_BOTTOM as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"chroma_sample_location_type\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVCHROMA_LOC_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"chroma_sample_location_type\0" as *const u8
                as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"alpha_mode\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"alpha mode\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 860 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: AVALPHA_MODE_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"alpha_mode_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unknown\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVALPHA_MODE_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"alpha_mode_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Unspecified\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVALPHA_MODE_UNSPECIFIED as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"alpha_mode_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"premultiplied\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Premultiplied\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVALPHA_MODE_PREMULTIPLIED as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"alpha_mode_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"straight\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Straight\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AVALPHA_MODE_STRAIGHT as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"alpha_mode_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"log_level_offset\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set the log level offset\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 8 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"slices\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set the number of slices, used in parallelized encoding\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 340 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"thread_type\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"select multithreading type\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 660 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLAGS,
            default_val: C2RustUnnamed {
                i64_0: (2 as ::core::ffi::c_int | 1 as ::core::ffi::c_int) as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"thread_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"slice\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 2 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"thread_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"frame\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"thread_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"audio_service_type\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"audio service type\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 388 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: AV_AUDIO_SERVICE_TYPE_MAIN as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: (AV_AUDIO_SERVICE_TYPE_NB as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"audio_service_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ma\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Main Audio Service\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_AUDIO_SERVICE_TYPE_MAIN as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"audio_service_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ef\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Effects\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_AUDIO_SERVICE_TYPE_EFFECTS as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"audio_service_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"vi\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Visually Impaired\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_AUDIO_SERVICE_TYPE_VISUALLY_IMPAIRED as ::core::ffi::c_int
                    as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"audio_service_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"hi\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Hearing Impaired\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_AUDIO_SERVICE_TYPE_HEARING_IMPAIRED as ::core::ffi::c_int
                    as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"audio_service_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"di\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Dialogue\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_AUDIO_SERVICE_TYPE_DIALOGUE as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"audio_service_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"co\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Commentary\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_AUDIO_SERVICE_TYPE_COMMENTARY as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"audio_service_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"em\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Emergency\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_AUDIO_SERVICE_TYPE_EMERGENCY as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"audio_service_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"vo\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Voice Over\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_AUDIO_SERVICE_TYPE_VOICE_OVER as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"audio_service_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ka\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Karaoke\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_AUDIO_SERVICE_TYPE_KARAOKE as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"audio_service_type\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"request_sample_fmt\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"sample format audio decoders should prefer\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 392 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_SAMPLE_FMT,
            default_val: C2RustUnnamed {
                i64_0: AV_SAMPLE_FMT_NONE as ::core::ffi::c_int as int64_t,
            },
            min: -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"request_sample_fmt\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"pkt_timebase\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 92 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_RATIONAL,
            default_val: C2RustUnnamed {
                dbl: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"sub_charenc\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set input text subtitles character encoding\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 736 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_STRING,
            default_val: C2RustUnnamed {
                str_0: ::core::ptr::null::<::core::ffi::c_char>(),
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"sub_charenc_mode\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set input text subtitles character encoding mode\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 744 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLAGS,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"sub_charenc_mode\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"do_nothing\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: -(1 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"sub_charenc_mode\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"auto\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"sub_charenc_mode\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"pre_decoder\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"sub_charenc_mode\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ignore\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: 2 as ::core::ffi::c_int as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"sub_charenc_mode\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"apply_cropping\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 800 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_BOOL,
            default_val: C2RustUnnamed {
                i64_0: 1 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 1 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"skip_alpha\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Skip processing alpha\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 708 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_BOOL,
            default_val: C2RustUnnamed {
                i64_0: 0 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 1 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"field_order\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Field order\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 164 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: AV_FIELD_UNKNOWN as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 5 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"field_order\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"progressive\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_FIELD_PROGRESSIVE as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"field_order\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"tt\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_FIELD_TT as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"field_order\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bb\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_FIELD_BB as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"field_order\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"tb\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_FIELD_TB as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"field_order\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"bt\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_FIELD_BT as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: b"field_order\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"dump_separator\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set information dump field separator\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 760 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_STRING,
            default_val: C2RustUnnamed {
                str_0: ::core::ptr::null::<::core::ffi::c_char>(),
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"codec_whitelist\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"List of decoders that are allowed to be used\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 768 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_STRING,
            default_val: C2RustUnnamed {
                str_0: ::core::ptr::null::<::core::ffi::c_char>(),
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"pixel_format\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set pixel format\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 136 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_PIXEL_FMT,
            default_val: C2RustUnnamed {
                i64_0: AV_PIX_FMT_NONE as ::core::ffi::c_int as int64_t,
            },
            min: -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"video_size\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"set video size\0" as *const u8 as *const ::core::ffi::c_char,
            offset: 112 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_IMAGE_SIZE,
            default_val: C2RustUnnamed {
                str_0: ::core::ptr::null::<::core::ffi::c_char>(),
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"max_pixels\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Maximum number of pixels\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 792 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT64,
            default_val: C2RustUnnamed {
                i64_0: 2147483647 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"max_samples\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Maximum number of samples\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 808 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT64,
            default_val: C2RustUnnamed {
                i64_0: 2147483647 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"hwaccel_flags\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 568 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_FLAGS,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"hwaccel_flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"ignore_level\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"ignore level even if the codec level used is unknown or higher than the maximum supported level reported by the hardware driver\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"hwaccel_flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"allow_high_depth\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"allow to output YUV pixel formats with a different chroma sampling than 4:2:0 and/or other than 8 bits per component\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"hwaccel_flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"allow_profile_mismatch\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"attempt to decode anyway if HW accelerated decoder's supported profiles do not exactly match the stream\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"hwaccel_flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"unsafe_output\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"allow potentially unsafe hwaccel frame output that might require special care to process successfully\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 0 as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int) as int64_t,
            },
            min: (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
                as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"hwaccel_flags\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"extra_hw_frames\0" as *const u8 as *const ::core::ffi::c_char,
            help: b"Number of extra hardware frames to allocate for the user\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 572 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: -(1 as ::core::ffi::c_int) as int64_t,
            },
            min: -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"discard_damaged_percentage\0" as *const u8
                as *const ::core::ffi::c_char,
            help: b"Percentage of damaged samples to discard a frame\0" as *const u8
                as *const ::core::ffi::c_char,
            offset: 804 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: AV_OPT_TYPE_INT,
            default_val: C2RustUnnamed {
                i64_0: 95 as ::core::ffi::c_int as int64_t,
            },
            min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            max: 100 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
        AVOption {
            name: b"side_data_prefer_packet\0" as *const u8
                as *const ::core::ffi::c_char,
            help: b"Comma-separated list of side data types for which user-supplied (container) data is preferred over coded bytestream\0"
                as *const u8 as *const ::core::ffi::c_char,
            offset: 832 as ::core::ffi::c_ulong as ::core::ffi::c_int,
            type_0: (AV_OPT_TYPE_INT as ::core::ffi::c_int
                | AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as AVOptionType,
            default_val: C2RustUnnamed { i64_0: 0 },
            min: -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
            max: 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double,
            flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"side_data_pkt\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"replaygain\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_PKT_DATA_REPLAYGAIN as ::core::ffi::c_int as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"side_data_pkt\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"displaymatrix\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_PKT_DATA_DISPLAYMATRIX as ::core::ffi::c_int as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"side_data_pkt\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"spherical\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_PKT_DATA_SPHERICAL as ::core::ffi::c_int as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"side_data_pkt\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"stereo3d\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_PKT_DATA_STEREO3D as ::core::ffi::c_int as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"side_data_pkt\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"audio_service_type\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_PKT_DATA_AUDIO_SERVICE_TYPE as ::core::ffi::c_int as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"side_data_pkt\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"mastering_display_metadata\0" as *const u8
                as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_PKT_DATA_MASTERING_DISPLAY_METADATA as ::core::ffi::c_int
                    as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"side_data_pkt\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"content_light_level\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_PKT_DATA_CONTENT_LIGHT_LEVEL as ::core::ffi::c_int as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"side_data_pkt\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"icc_profile\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_PKT_DATA_ICC_PROFILE as ::core::ffi::c_int as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"side_data_pkt\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: b"exif\0" as *const u8 as *const ::core::ffi::c_char,
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0,
            type_0: AV_OPT_TYPE_CONST,
            default_val: C2RustUnnamed {
                i64_0: AV_PKT_DATA_EXIF as ::core::ffi::c_int as int64_t,
            },
            min: 0.,
            max: 0.,
            flags: (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
            unit: b"side_data_pkt\0" as *const u8 as *const ::core::ffi::c_char,
        },
        AVOption {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            help: ::core::ptr::null::<::core::ffi::c_char>(),
            offset: 0,
            type_0: 0 as AVOptionType,
            default_val: C2RustUnnamed { i64_0: 0 },
            min: 0.,
            max: 0.,
            flags: 0,
            unit: ::core::ptr::null::<::core::ffi::c_char>(),
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
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
pub struct AVCodecHWConfigInternal {
    _opaque: [u8; 0],
}
