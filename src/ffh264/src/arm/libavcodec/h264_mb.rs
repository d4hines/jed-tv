// Translated from FFmpeg 9.0.1 libavcodec/h264_mb.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// H.26L/H.264/AVC/JVT/14496-10/... decoder
// Copyright (c) 2003 Michael Niedermayer <michaelni@gmx.at>
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
    fn memcpy(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn ff_thread_await_progress(
        f: *const ThreadFrame,
        progress: ::core::ffi::c_int,
        field: ::core::ffi::c_int,
    );
    static ff_h264_mb_sizes: [uint16_t; 4];
}
pub type int8_t = ::core::ffi::c_schar;
pub type int16_t = ::core::ffi::c_short;
pub type int32_t = ::core::ffi::c_int;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulonglong;
pub type size_t = ::core::ffi::c_uint;
pub type ptrdiff_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union av_alias64 {
    pub u64_0: uint64_t,
    pub u32_0: [uint32_t; 2],
    pub u16_0: [uint16_t; 4],
    pub u8_0: [uint8_t; 8],
    pub f64_0: ::core::ffi::c_double,
    pub f32_0: [::core::ffi::c_float; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union av_alias32 {
    pub u32_0: uint32_t,
    pub u16_0: [uint16_t; 2],
    pub u8_0: [uint8_t; 4],
    pub f32_0: ::core::ffi::c_float,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union av_alias16 {
    pub u16_0: uint16_t,
    pub u8_0: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CABACContext {
    pub low: ::core::ffi::c_int,
    pub range: ::core::ffi::c_int,
    pub bytestream_start: *const uint8_t,
    pub bytestream: *const uint8_t,
    pub bytestream_end: *const uint8_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GetBitContext {
    pub buffer: *const uint8_t,
    pub index: ::core::ffi::c_int,
    pub size_in_bits: ::core::ffi::c_int,
    pub size_in_bits_plus8: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645VUI {
    pub sar: AVRational,
    pub aspect_ratio_idc: ::core::ffi::c_int,
    pub aspect_ratio_info_present_flag: ::core::ffi::c_int,
    pub overscan_info_present_flag: ::core::ffi::c_int,
    pub overscan_appropriate_flag: ::core::ffi::c_int,
    pub video_signal_type_present_flag: ::core::ffi::c_int,
    pub video_format: ::core::ffi::c_int,
    pub video_full_range_flag: ::core::ffi::c_int,
    pub colour_description_present_flag: ::core::ffi::c_int,
    pub colour_primaries: AVColorPrimaries,
    pub transfer_characteristics: AVColorTransferCharacteristic,
    pub matrix_coeffs: AVColorSpace,
    pub chroma_loc_info_present_flag: ::core::ffi::c_int,
    pub chroma_sample_loc_type_top_field: ::core::ffi::c_int,
    pub chroma_sample_loc_type_bottom_field: ::core::ffi::c_int,
    pub chroma_location: AVChromaLocation,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SPS {
    pub sps_id: ::core::ffi::c_uint,
    pub profile_idc: ::core::ffi::c_int,
    pub level_idc: ::core::ffi::c_int,
    pub chroma_format_idc: ::core::ffi::c_int,
    pub transform_bypass: ::core::ffi::c_int,
    pub log2_max_frame_num: ::core::ffi::c_int,
    pub poc_type: ::core::ffi::c_int,
    pub log2_max_poc_lsb: ::core::ffi::c_int,
    pub delta_pic_order_always_zero_flag: ::core::ffi::c_int,
    pub offset_for_non_ref_pic: ::core::ffi::c_int,
    pub offset_for_top_to_bottom_field: ::core::ffi::c_int,
    pub poc_cycle_length: ::core::ffi::c_int,
    pub ref_frame_count: ::core::ffi::c_int,
    pub gaps_in_frame_num_allowed_flag: ::core::ffi::c_int,
    pub mb_width: ::core::ffi::c_int,
    pub mb_height: ::core::ffi::c_int,
    pub frame_mbs_only_flag: ::core::ffi::c_int,
    pub mb_aff: ::core::ffi::c_int,
    pub direct_8x8_inference_flag: ::core::ffi::c_int,
    pub crop: ::core::ffi::c_int,
    pub crop_left: ::core::ffi::c_uint,
    pub crop_right: ::core::ffi::c_uint,
    pub crop_top: ::core::ffi::c_uint,
    pub crop_bottom: ::core::ffi::c_uint,
    pub vui_parameters_present_flag: ::core::ffi::c_int,
    pub vui: H2645VUI,
    pub timing_info_present_flag: ::core::ffi::c_int,
    pub num_units_in_tick: uint32_t,
    pub time_scale: uint32_t,
    pub fixed_frame_rate_flag: ::core::ffi::c_int,
    pub offset_for_ref_frame: [int32_t; 256],
    pub bitstream_restriction_flag: ::core::ffi::c_int,
    pub num_reorder_frames: ::core::ffi::c_int,
    pub max_dec_frame_buffering: ::core::ffi::c_int,
    pub scaling_matrix_present: ::core::ffi::c_int,
    pub scaling_matrix_present_mask: uint16_t,
    pub scaling_matrix4: [[uint8_t; 16]; 6],
    pub scaling_matrix8: [[uint8_t; 64]; 6],
    pub nal_hrd_parameters_present_flag: ::core::ffi::c_int,
    pub vcl_hrd_parameters_present_flag: ::core::ffi::c_int,
    pub pic_struct_present_flag: ::core::ffi::c_int,
    pub time_offset_length: ::core::ffi::c_int,
    pub cpb_cnt: ::core::ffi::c_int,
    pub bit_rate_scale: ::core::ffi::c_int,
    pub bit_rate_value: [uint32_t; 32],
    pub cpb_size_value: [uint32_t; 32],
    pub cpr_flag: uint32_t,
    pub initial_cpb_removal_delay_length: ::core::ffi::c_int,
    pub cpb_removal_delay_length: ::core::ffi::c_int,
    pub dpb_output_delay_length: ::core::ffi::c_int,
    pub bit_depth_luma: ::core::ffi::c_int,
    pub bit_depth_chroma: ::core::ffi::c_int,
    pub residual_color_transform_flag: ::core::ffi::c_int,
    pub constraint_set_flags: ::core::ffi::c_int,
    pub data: [uint8_t; 4096],
    pub data_size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PPS {
    pub pps_id: ::core::ffi::c_uint,
    pub sps_id: ::core::ffi::c_uint,
    pub cabac: ::core::ffi::c_int,
    pub pic_order_present: ::core::ffi::c_int,
    pub slice_group_count: ::core::ffi::c_int,
    pub mb_slice_group_map_type: ::core::ffi::c_int,
    pub ref_count: [::core::ffi::c_uint; 2],
    pub weighted_pred: ::core::ffi::c_int,
    pub weighted_bipred_idc: ::core::ffi::c_int,
    pub init_qp: ::core::ffi::c_int,
    pub init_qs: ::core::ffi::c_int,
    pub chroma_qp_index_offset: [::core::ffi::c_int; 2],
    pub deblocking_filter_parameters_present: ::core::ffi::c_int,
    pub constrained_intra_pred: ::core::ffi::c_int,
    pub redundant_pic_cnt_present: ::core::ffi::c_int,
    pub transform_8x8_mode: ::core::ffi::c_int,
    pub pic_scaling_matrix_present_flag: ::core::ffi::c_int,
    pub pic_scaling_matrix_present_mask: uint16_t,
    pub scaling_matrix4: [[uint8_t; 16]; 6],
    pub scaling_matrix8: [[uint8_t; 64]; 6],
    pub chroma_qp_table: [[uint8_t; 88]; 2],
    pub chroma_qp_diff: ::core::ffi::c_int,
    pub data: [uint8_t; 4096],
    pub data_size: size_t,
    pub dequant4_buffer: [[[uint32_t; 16]; 88]; 6],
    pub dequant8_buffer: [[[uint32_t; 64]; 88]; 6],
    pub dequant4_coeff: [*mut [uint32_t; 16]; 6],
    pub dequant8_coeff: [*mut [uint32_t; 64]; 6],
    pub sps: *const SPS,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264ParamSets {
    pub sps_list: [*const SPS; 32],
    pub pps_list: [*const PPS; 256],
    pub pps: *const PPS,
    pub sps: *const SPS,
    pub overread_warning_printed: [::core::ffi::c_int; 2],
}
pub type MMCOOpcode = ::core::ffi::c_uint;
pub const MMCO_LONG: MMCOOpcode = 6;
pub const MMCO_RESET: MMCOOpcode = 5;
pub const MMCO_SET_MAX_LONG: MMCOOpcode = 4;
pub const MMCO_SHORT2LONG: MMCOOpcode = 3;
pub const MMCO_LONG2UNUSED: MMCOOpcode = 2;
pub const MMCO_SHORT2UNUSED: MMCOOpcode = 1;
pub const MMCO_END: MMCOOpcode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264PredWeightTable {
    pub use_weight: ::core::ffi::c_int,
    pub use_weight_chroma: ::core::ffi::c_int,
    pub luma_log2_weight_denom: ::core::ffi::c_int,
    pub chroma_log2_weight_denom: ::core::ffi::c_int,
    pub luma_weight_flag: [::core::ffi::c_int; 2],
    pub chroma_weight_flag: [::core::ffi::c_int; 2],
    pub luma_weight: [[[::core::ffi::c_int; 2]; 2]; 48],
    pub chroma_weight: [[[[::core::ffi::c_int; 2]; 2]; 2]; 48],
    pub implicit_weight: [[[::core::ffi::c_int; 2]; 48]; 48],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264POCContext {
    pub poc_lsb: ::core::ffi::c_int,
    pub poc_msb: ::core::ffi::c_int,
    pub delta_poc_bottom: ::core::ffi::c_int,
    pub delta_poc: [::core::ffi::c_int; 2],
    pub frame_num: ::core::ffi::c_int,
    pub prev_poc_msb: ::core::ffi::c_int,
    pub prev_poc_lsb: ::core::ffi::c_int,
    pub frame_num_offset: ::core::ffi::c_int,
    pub prev_frame_num_offset: ::core::ffi::c_int,
    pub prev_frame_num: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilmGrainAFGS1Params {
    pub enable: ::core::ffi::c_int,
    pub sets: [*mut AVBufferRef; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFITUTT35Meta {
    pub afd: *mut AVBufferRef,
    pub a53_cc: *mut AVBufferRef,
    pub lcevc: *mut AVBufferRef,
    pub hdr_plus: *mut AVBufferRef,
    pub hdr_smpte2094_app5: *mut AVBufferRef,
    pub hdr_vivid: *mut AVBufferRef,
    pub dovi: *mut AVBufferRef,
    pub aom_film_grain: AVFilmGrainAFGS1Params,
}
pub type SEIFpaType = ::core::ffi::c_uint;
pub const SEI_FPA_H264_TYPE_2D: SEIFpaType = 6;
pub const SEI_FPA_TYPE_INTERLEAVE_TEMPORAL: SEIFpaType = 5;
pub const SEI_FPA_TYPE_TOP_BOTTOM: SEIFpaType = 4;
pub const SEI_FPA_TYPE_SIDE_BY_SIDE: SEIFpaType = 3;
pub const SEI_FPA_H264_TYPE_INTERLEAVE_ROW: SEIFpaType = 2;
pub const SEI_FPA_H264_TYPE_INTERLEAVE_COLUMN: SEIFpaType = 1;
pub const SEI_FPA_H264_TYPE_CHECKERBOARD: SEIFpaType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIUnregistered {
    pub buf_ref: *mut *mut AVBufferRef,
    pub nb_buf_ref: ::core::ffi::c_uint,
    pub x264_build: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIFramePacking {
    pub present: ::core::ffi::c_int,
    pub arrangement_id: ::core::ffi::c_int,
    pub arrangement_cancel_flag: ::core::ffi::c_int,
    pub arrangement_type: SEIFpaType,
    pub arrangement_repetition_period: ::core::ffi::c_int,
    pub content_interpretation_type: ::core::ffi::c_int,
    pub quincunx_sampling_flag: ::core::ffi::c_int,
    pub current_frame_is_frame0_flag: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIDisplayOrientation {
    pub present: ::core::ffi::c_int,
    pub anticlockwise_rotation: ::core::ffi::c_int,
    pub hflip: ::core::ffi::c_int,
    pub vflip: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIAlternativeTransfer {
    pub present: ::core::ffi::c_int,
    pub preferred_transfer_characteristics: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIAmbientViewingEnvironment {
    pub present: ::core::ffi::c_int,
    pub ambient_illuminance: uint32_t,
    pub ambient_light_x: uint16_t,
    pub ambient_light_y: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIFilmGrainCharacteristics {
    pub present: ::core::ffi::c_int,
    pub model_id: ::core::ffi::c_int,
    pub separate_colour_description_present_flag: ::core::ffi::c_int,
    pub bit_depth_luma: ::core::ffi::c_int,
    pub bit_depth_chroma: ::core::ffi::c_int,
    pub full_range: ::core::ffi::c_int,
    pub color_primaries: ::core::ffi::c_int,
    pub transfer_characteristics: ::core::ffi::c_int,
    pub matrix_coeffs: ::core::ffi::c_int,
    pub blending_mode_id: ::core::ffi::c_int,
    pub log2_scale_factor: ::core::ffi::c_int,
    pub comp_model_present_flag: [::core::ffi::c_int; 3],
    pub num_intensity_intervals: [uint16_t; 3],
    pub num_model_values: [uint8_t; 3],
    pub intensity_interval_lower_bound: [[uint8_t; 256]; 3],
    pub intensity_interval_upper_bound: [[uint8_t; 256]; 3],
    pub comp_model_value: [[[int16_t; 6]; 256]; 3],
    pub repetition_period: ::core::ffi::c_int,
    pub persistence_flag: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIMasteringDisplay {
    pub present: ::core::ffi::c_int,
    pub display_primaries: [[uint16_t; 2]; 3],
    pub white_point: [uint16_t; 2],
    pub max_luminance: uint32_t,
    pub min_luminance: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIContentLight {
    pub present: ::core::ffi::c_int,
    pub max_content_light_level: uint16_t,
    pub max_pic_average_light_level: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEI {
    pub itut_t35: FFITUTT35Meta,
    pub unregistered: H2645SEIUnregistered,
    pub frame_packing: H2645SEIFramePacking,
    pub display_orientation: H2645SEIDisplayOrientation,
    pub alternative_transfer: H2645SEIAlternativeTransfer,
    pub ambient_viewing_environment: H2645SEIAmbientViewingEnvironment,
    pub mastering_display: H2645SEIMasteringDisplay,
    pub content_light: H2645SEIContentLight,
    pub film_grain_characteristics: *mut H2645SEIFilmGrainCharacteristics,
}
pub type H264_SEI_PicStructType = ::core::ffi::c_uint;
pub const H264_SEI_PIC_STRUCT_FRAME_TRIPLING: H264_SEI_PicStructType = 8;
pub const H264_SEI_PIC_STRUCT_FRAME_DOUBLING: H264_SEI_PicStructType = 7;
pub const H264_SEI_PIC_STRUCT_BOTTOM_TOP_BOTTOM: H264_SEI_PicStructType = 6;
pub const H264_SEI_PIC_STRUCT_TOP_BOTTOM_TOP: H264_SEI_PicStructType = 5;
pub const H264_SEI_PIC_STRUCT_BOTTOM_TOP: H264_SEI_PicStructType = 4;
pub const H264_SEI_PIC_STRUCT_TOP_BOTTOM: H264_SEI_PicStructType = 3;
pub const H264_SEI_PIC_STRUCT_BOTTOM_FIELD: H264_SEI_PicStructType = 2;
pub const H264_SEI_PIC_STRUCT_TOP_FIELD: H264_SEI_PicStructType = 1;
pub const H264_SEI_PIC_STRUCT_FRAME: H264_SEI_PicStructType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264SEITimeCode {
    pub full: ::core::ffi::c_int,
    pub frame: ::core::ffi::c_int,
    pub seconds: ::core::ffi::c_int,
    pub minutes: ::core::ffi::c_int,
    pub hours: ::core::ffi::c_int,
    pub dropframe: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264SEIPictureTiming {
    pub payload: [uint8_t; 40],
    pub payload_size_bytes: ::core::ffi::c_int,
    pub present: ::core::ffi::c_int,
    pub pic_struct: H264_SEI_PicStructType,
    pub ct_type: ::core::ffi::c_int,
    pub dpb_output_delay: ::core::ffi::c_int,
    pub cpb_removal_delay: ::core::ffi::c_int,
    pub timecode: [H264SEITimeCode; 3],
    pub timecode_cnt: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264SEIRecoveryPoint {
    pub recovery_frame_cnt: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264SEIBufferingPeriod {
    pub present: ::core::ffi::c_int,
    pub initial_cpb_removal_delay: [::core::ffi::c_int; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264SEIGreenMetaData {
    pub green_metadata_type: uint8_t,
    pub period_type: uint8_t,
    pub num_seconds: uint16_t,
    pub num_pictures: uint16_t,
    pub percent_non_zero_macroblocks: uint8_t,
    pub percent_intra_coded_macroblocks: uint8_t,
    pub percent_six_tap_filtering: uint8_t,
    pub percent_alpha_point_deblocking_instance: uint8_t,
    pub xsd_metric_type: uint8_t,
    pub xsd_metric_value: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264SEIContext {
    pub common: H2645SEI,
    pub picture_timing: H264SEIPictureTiming,
    pub recovery_point: H264SEIRecoveryPoint,
    pub buffering_period: H264SEIBufferingPeriod,
    pub green_metadata: H264SEIGreenMetaData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645NAL {
    pub data: *const uint8_t,
    pub size: ::core::ffi::c_int,
    pub size_bits: ::core::ffi::c_int,
    pub raw_size: ::core::ffi::c_int,
    pub raw_data: *const uint8_t,
    pub gb: GetBitContext,
    pub type_0: ::core::ffi::c_int,
    pub ref_idc: ::core::ffi::c_int,
    pub temporal_id: ::core::ffi::c_int,
    pub nuh_layer_id: ::core::ffi::c_int,
    pub skipped_bytes: ::core::ffi::c_int,
    pub skipped_bytes_pos_size: ::core::ffi::c_int,
    pub skipped_bytes_pos: *mut ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645RBSP {
    pub rbsp_buffer: *mut uint8_t,
    pub rbsp_buffer_ref: *mut AVBufferRef,
    pub rbsp_buffer_alloc_size: ::core::ffi::c_int,
    pub rbsp_buffer_size: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645Packet {
    pub nals: *mut H2645NAL,
    pub rbsp: H2645RBSP,
    pub nb_nals: ::core::ffi::c_int,
    pub nals_allocated: ::core::ffi::c_int,
    pub nal_buffer_size: ::core::ffi::c_uint,
}
pub type h264_chroma_mc_func = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        *const uint8_t,
        ptrdiff_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264ChromaContext {
    pub put_h264_chroma_pixels_tab: [h264_chroma_mc_func; 4],
    pub avg_h264_chroma_pixels_tab: [h264_chroma_mc_func; 4],
}
pub type h264_weight_func = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        ptrdiff_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
pub type h264_biweight_func = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        *mut uint8_t,
        ptrdiff_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264DSPContext {
    pub weight_pixels_tab: [h264_weight_func; 4],
    pub biweight_pixels_tab: [h264_biweight_func; 4],
    pub v_loop_filter_luma: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub h_loop_filter_luma: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub h_loop_filter_luma_mbaff: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub v_loop_filter_luma_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub h_loop_filter_luma_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub h_loop_filter_luma_mbaff_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub v_loop_filter_chroma: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub h_loop_filter_chroma: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub h_loop_filter_chroma_mbaff: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut int8_t,
        ) -> (),
    >,
    pub v_loop_filter_chroma_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub h_loop_filter_chroma_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub h_loop_filter_chroma_mbaff_intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub loop_filter_strength: Option<
        unsafe extern "C" fn(
            *mut [[int16_t; 4]; 4],
            *mut uint8_t,
            *mut [int8_t; 40],
            *mut [[int16_t; 2]; 40],
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub idct_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub idct8_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub idct_dc_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub idct8_dc_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub idct_add16: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ::core::ffi::c_int,
            *const uint8_t,
        ) -> (),
    >,
    pub idct8_add4: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ::core::ffi::c_int,
            *const uint8_t,
        ) -> (),
    >,
    pub idct_add8: Option<
        unsafe extern "C" fn(
            *mut *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ::core::ffi::c_int,
            *const uint8_t,
        ) -> (),
    >,
    pub idct_add16intra: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ::core::ffi::c_int,
            *const uint8_t,
        ) -> (),
    >,
    pub luma_dc_dequant_idct: Option<
        unsafe extern "C" fn(*mut int16_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub chroma_dc_dequant_idct: Option<
        unsafe extern "C" fn(*mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub add_pixels8_clear: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub add_pixels4_clear: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    >,
    pub startcode_find_candidate: Option<
        unsafe extern "C" fn(*const uint8_t, ::core::ffi::c_int) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264PredContext {
    pub pred4x4: [Option<
        unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
    >; 15],
    pub pred8x8l: [Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ptrdiff_t,
        ) -> (),
    >; 12],
    pub pred8x8: [Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>; 11],
    pub pred16x16: [Option<unsafe extern "C" fn(*mut uint8_t, ptrdiff_t) -> ()>; 9],
    pub pred4x4_add: [Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ptrdiff_t) -> (),
    >; 2],
    pub pred8x8l_add: [Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ptrdiff_t) -> (),
    >; 2],
    pub pred8x8l_filter_add: [Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *mut int16_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ptrdiff_t,
        ) -> (),
    >; 2],
    pub pred8x8_add: [Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ptrdiff_t,
        ) -> (),
    >; 3],
    pub pred16x16_add: [Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const ::core::ffi::c_int,
            *mut int16_t,
            ptrdiff_t,
        ) -> (),
    >; 3],
}
pub type qpel_mc_func = Option<
    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264QpelContext {
    pub put_h264_qpel_pixels_tab: [[qpel_mc_func; 16]; 3],
    pub avg_h264_qpel_pixels_tab: [[qpel_mc_func; 16]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VideoDSPContext {
    pub emulated_edge_mc: Option<
        unsafe extern "C" fn(
            *mut uint8_t,
            *const uint8_t,
            ptrdiff_t,
            ptrdiff_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub prefetch: Option<
        unsafe extern "C" fn(*const uint8_t, ptrdiff_t, ::core::ffi::c_int) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MMCO {
    pub opcode: MMCOOpcode,
    pub short_pic_num: ::core::ffi::c_int,
    pub long_arg: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264Picture {
    pub f: *mut AVFrame,
    pub tf: ThreadFrame,
    pub f_grain: *mut AVFrame,
    pub qscale_table_base: *mut int8_t,
    pub qscale_table: *mut int8_t,
    pub motion_val_base: [*mut [int16_t; 2]; 2],
    pub motion_val: [*mut [int16_t; 2]; 2],
    pub mb_type_base: *mut uint32_t,
    pub mb_type: *mut uint32_t,
    pub hwaccel_picture_private: *mut ::core::ffi::c_void,
    pub ref_index: [*mut int8_t; 2],
    pub field_poc: [::core::ffi::c_int; 2],
    pub poc: ::core::ffi::c_int,
    pub frame_num: ::core::ffi::c_int,
    pub mmco_reset: ::core::ffi::c_int,
    pub pic_id: ::core::ffi::c_int,
    pub long_ref: ::core::ffi::c_int,
    pub ref_poc: [[[::core::ffi::c_int; 32]; 2]; 2],
    pub ref_count: [[::core::ffi::c_int; 2]; 2],
    pub mbaff: ::core::ffi::c_int,
    pub field_picture: ::core::ffi::c_int,
    pub reference: ::core::ffi::c_int,
    pub recovered: ::core::ffi::c_int,
    pub invalid_gap: ::core::ffi::c_int,
    pub sei_recovery_frame_cnt: ::core::ffi::c_int,
    pub needs_fg: ::core::ffi::c_int,
    pub pps: *const PPS,
    pub mb_width: ::core::ffi::c_int,
    pub mb_height: ::core::ffi::c_int,
    pub mb_stride: ::core::ffi::c_int,
    pub decode_error_flags: *mut atomic_int,
    pub gray: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264Ref {
    pub data: [*mut uint8_t; 3],
    pub linesize: [::core::ffi::c_int; 3],
    pub reference: ::core::ffi::c_int,
    pub poc: ::core::ffi::c_int,
    pub pic_id: ::core::ffi::c_int,
    pub parent: *const H264Picture,
}
#[derive(Copy, Clone)]
#[repr(C, align(16))]
pub struct H264SliceContext {
    pub h264: *const H264Context,
    pub gb: GetBitContext,
    pub er: *mut ERContext,
    pub slice_num: ::core::ffi::c_int,
    pub slice_type: ::core::ffi::c_int,
    pub slice_type_nos: ::core::ffi::c_int,
    pub slice_type_fixed: ::core::ffi::c_int,
    pub qscale: ::core::ffi::c_int,
    pub chroma_qp: [::core::ffi::c_int; 2],
    pub qp_thresh: ::core::ffi::c_int,
    pub last_qscale_diff: ::core::ffi::c_int,
    pub deblocking_filter: ::core::ffi::c_int,
    pub slice_alpha_c0_offset: ::core::ffi::c_int,
    pub slice_beta_offset: ::core::ffi::c_int,
    pub pwt: H264PredWeightTable,
    pub prev_mb_skipped: ::core::ffi::c_int,
    pub next_mb_skipped: ::core::ffi::c_int,
    pub chroma_pred_mode: ::core::ffi::c_int,
    pub intra16x16_pred_mode: ::core::ffi::c_int,
    pub intra4x4_pred_mode_cache: [int8_t; 40],
    pub intra4x4_pred_mode: *mut int8_t,
    pub topleft_mb_xy: ::core::ffi::c_int,
    pub top_mb_xy: ::core::ffi::c_int,
    pub topright_mb_xy: ::core::ffi::c_int,
    pub left_mb_xy: [::core::ffi::c_int; 2],
    pub topleft_type: ::core::ffi::c_int,
    pub top_type: ::core::ffi::c_int,
    pub topright_type: ::core::ffi::c_int,
    pub left_type: [::core::ffi::c_int; 2],
    pub left_block: *const uint8_t,
    pub topleft_partition: ::core::ffi::c_int,
    pub topleft_samples_available: ::core::ffi::c_uint,
    pub top_samples_available: ::core::ffi::c_uint,
    pub topright_samples_available: ::core::ffi::c_uint,
    pub left_samples_available: ::core::ffi::c_uint,
    pub linesize: ptrdiff_t,
    pub uvlinesize: ptrdiff_t,
    pub mb_linesize: ptrdiff_t,
    pub mb_uvlinesize: ptrdiff_t,
    pub mb_x: ::core::ffi::c_int,
    pub mb_y: ::core::ffi::c_int,
    pub mb_xy: ::core::ffi::c_int,
    pub resync_mb_x: ::core::ffi::c_int,
    pub resync_mb_y: ::core::ffi::c_int,
    pub first_mb_addr: ::core::ffi::c_uint,
    pub next_slice_idx: ::core::ffi::c_int,
    pub mb_skip_run: ::core::ffi::c_int,
    pub is_complex: ::core::ffi::c_int,
    pub picture_structure: ::core::ffi::c_int,
    pub mb_field_decoding_flag: ::core::ffi::c_int,
    pub mb_mbaff: ::core::ffi::c_int,
    pub redundant_pic_count: ::core::ffi::c_int,
    pub neighbor_transform_size: ::core::ffi::c_int,
    pub direct_spatial_mv_pred: ::core::ffi::c_int,
    pub col_parity: ::core::ffi::c_int,
    pub col_fieldoff: ::core::ffi::c_int,
    pub cbp: ::core::ffi::c_int,
    pub top_cbp: ::core::ffi::c_int,
    pub left_cbp: ::core::ffi::c_int,
    pub dist_scale_factor: [::core::ffi::c_int; 32],
    pub dist_scale_factor_field: [[::core::ffi::c_int; 32]; 2],
    pub map_col_to_list0: [[::core::ffi::c_int; 48]; 2],
    pub map_col_to_list0_field: [[[::core::ffi::c_int; 48]; 2]; 2],
    pub ref_count: [::core::ffi::c_uint; 2],
    pub list_count: ::core::ffi::c_uint,
    pub ref_list: [[H264Ref; 48]; 2],
    pub ref_modifications: [[C2RustUnnamed_0; 32]; 2],
    pub nb_ref_modifications: [::core::ffi::c_int; 2],
    pub pps_id: ::core::ffi::c_uint,
    pub intra_pcm_ptr: *const uint8_t,
    pub bipred_scratchpad: *mut uint8_t,
    pub edge_emu_buffer: *mut uint8_t,
    pub top_borders: [*mut [uint8_t; 96]; 2],
    pub bipred_scratchpad_allocated: ::core::ffi::c_int,
    pub edge_emu_buffer_allocated: ::core::ffi::c_int,
    pub top_borders_allocated: [::core::ffi::c_int; 2],
    pub non_zero_count_cache: [uint8_t; 120],
    pub c2rust_alignpad_0: [u8; 8],
    pub mv_cache: [[[int16_t; 2]; 40]; 2],
    pub ref_cache: [[int8_t; 40]; 2],
    pub mvd_cache: [[[uint8_t; 2]; 40]; 2],
    pub direct_cache: [uint8_t; 40],
    pub sub_mb_type: [uint16_t; 4],
    pub mb: [int16_t; 1536],
    pub mb_luma_dc: [[int16_t; 32]; 3],
    pub mb_padding: [int16_t; 512],
    pub mvd_table: [*mut [uint8_t; 2]; 2],
    pub cabac: CABACContext,
    pub cabac_state: [uint8_t; 1024],
    pub cabac_init_idc: ::core::ffi::c_int,
    pub mmco: [MMCO; 67],
    pub nb_mmco: ::core::ffi::c_int,
    pub explicit_ref_marking: ::core::ffi::c_int,
    pub frame_num: ::core::ffi::c_int,
    pub idr_pic_id: ::core::ffi::c_int,
    pub poc_lsb: ::core::ffi::c_int,
    pub delta_poc_bottom: ::core::ffi::c_int,
    pub delta_poc: [::core::ffi::c_int; 2],
    pub curr_pic_num: ::core::ffi::c_int,
    pub max_pic_num: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub op: uint8_t,
    pub val: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264Context {
    pub class: *const AVClass,
    pub avctx: *mut AVCodecContext,
    pub vdsp: VideoDSPContext,
    pub h264dsp: H264DSPContext,
    pub h264chroma: H264ChromaContext,
    pub h264qpel: H264QpelContext,
    pub DPB: [H264Picture; 36],
    pub cur_pic_ptr: *mut H264Picture,
    pub cur_pic: H264Picture,
    pub last_pic_for_ec: H264Picture,
    pub slice_ctx: *mut H264SliceContext,
    pub nb_slice_ctx: ::core::ffi::c_int,
    pub nb_slice_ctx_queued: ::core::ffi::c_int,
    pub pkt: H2645Packet,
    pub pixel_shift: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub chroma_x_shift: ::core::ffi::c_int,
    pub chroma_y_shift: ::core::ffi::c_int,
    pub droppable: ::core::ffi::c_int,
    pub context_initialized: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_int,
    pub workaround_bugs: ::core::ffi::c_int,
    pub x264_build: ::core::ffi::c_int,
    pub postpone_filter: ::core::ffi::c_int,
    pub picture_idr: ::core::ffi::c_int,
    pub picture_intra_only: ::core::ffi::c_int,
    pub crop_left: ::core::ffi::c_int,
    pub crop_right: ::core::ffi::c_int,
    pub crop_top: ::core::ffi::c_int,
    pub crop_bottom: ::core::ffi::c_int,
    pub intra4x4_pred_mode: *mut int8_t,
    pub hpc: H264PredContext,
    pub non_zero_count: *mut [uint8_t; 48],
    pub block_offset: [::core::ffi::c_int; 96],
    pub mb2b_xy: *mut uint32_t,
    pub mb2br_xy: *mut uint32_t,
    pub b_stride: ::core::ffi::c_int,
    pub slice_table: *mut uint16_t,
    pub mb_aff_frame: ::core::ffi::c_int,
    pub picture_structure: ::core::ffi::c_int,
    pub first_field: ::core::ffi::c_int,
    pub list_counts: *mut uint8_t,
    pub cbp_table: *mut uint16_t,
    pub chroma_pred_mode_table: *mut uint8_t,
    pub mvd_table: [*mut [uint8_t; 2]; 2],
    pub direct_table: *mut uint8_t,
    pub scan_padding: [uint8_t; 16],
    pub zigzag_scan: [uint8_t; 16],
    pub zigzag_scan8x8: [uint8_t; 64],
    pub zigzag_scan8x8_cavlc: [uint8_t; 64],
    pub field_scan: [uint8_t; 16],
    pub field_scan8x8: [uint8_t; 64],
    pub field_scan8x8_cavlc: [uint8_t; 64],
    pub zigzag_scan_q0: [uint8_t; 16],
    pub zigzag_scan8x8_q0: [uint8_t; 64],
    pub zigzag_scan8x8_cavlc_q0: [uint8_t; 64],
    pub field_scan_q0: [uint8_t; 16],
    pub field_scan8x8_q0: [uint8_t; 64],
    pub field_scan8x8_cavlc_q0: [uint8_t; 64],
    pub mb_y: ::core::ffi::c_int,
    pub mb_height: ::core::ffi::c_int,
    pub mb_width: ::core::ffi::c_int,
    pub mb_stride: ::core::ffi::c_int,
    pub mb_num: ::core::ffi::c_int,
    pub nal_ref_idc: ::core::ffi::c_int,
    pub nal_unit_type: ::core::ffi::c_int,
    pub has_slice: ::core::ffi::c_int,
    pub is_avc: ::core::ffi::c_int,
    pub nal_length_size: ::core::ffi::c_int,
    pub bit_depth_luma: ::core::ffi::c_int,
    pub chroma_format_idc: ::core::ffi::c_int,
    pub ps: H264ParamSets,
    pub slice_table_base: *mut uint16_t,
    pub poc: H264POCContext,
    pub default_ref: [H264Ref; 2],
    pub short_ref: [*mut H264Picture; 32],
    pub long_ref: [*mut H264Picture; 32],
    pub delayed_pic: [*mut H264Picture; 18],
    pub last_pocs: [::core::ffi::c_int; 16],
    pub next_output_pic: *mut H264Picture,
    pub next_outputed_poc: ::core::ffi::c_int,
    pub poc_offset: ::core::ffi::c_int,
    pub mmco: [MMCO; 67],
    pub nb_mmco: ::core::ffi::c_int,
    pub mmco_reset: ::core::ffi::c_int,
    pub explicit_ref_marking: ::core::ffi::c_int,
    pub long_ref_count: ::core::ffi::c_int,
    pub short_ref_count: ::core::ffi::c_int,
    pub current_slice: ::core::ffi::c_int,
    pub prev_interlaced_frame: ::core::ffi::c_int,
    pub valid_recovery_point: ::core::ffi::c_int,
    pub recovery_frame: ::core::ffi::c_int,
    pub frame_recovered: ::core::ffi::c_int,
    pub has_recovery_point: ::core::ffi::c_int,
    pub missing_fields: ::core::ffi::c_int,
    pub setup_finished: ::core::ffi::c_int,
    pub cur_chroma_format_idc: ::core::ffi::c_int,
    pub cur_bit_depth_luma: ::core::ffi::c_int,
    pub slice_row: [int16_t; 32],
    pub width_from_caller: ::core::ffi::c_int,
    pub height_from_caller: ::core::ffi::c_int,
    pub enable_er: ::core::ffi::c_int,
    pub er: ERContext,
    pub dc_val_base: *mut int16_t,
    pub sei: H264SEIContext,
    pub qscale_table_pool: *mut AVRefStructPool,
    pub mb_type_pool: *mut AVRefStructPool,
    pub motion_val_pool: *mut AVRefStructPool,
    pub ref_index_pool: *mut AVRefStructPool,
    pub decode_error_flags_pool: *mut AVRefStructPool,
    pub ref2frm: [[[::core::ffi::c_int; 64]; 2]; 32],
    pub non_gray: ::core::ffi::c_int,
    pub noref_gray: ::core::ffi::c_int,
    pub skip_gray: ::core::ffi::c_int,
}
unsafe extern "C" fn mc_part_444_simple_8(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut square: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut x_offset: ::core::ffi::c_int,
    mut y_offset: ::core::ffi::c_int,
    mut qpix_put: *const qpel_mc_func,
    mut chroma_put: h264_chroma_mc_func,
    mut qpix_avg: *const qpel_mc_func,
    mut chroma_avg: h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
    mut list0: ::core::ffi::c_int,
    mut list1: ::core::ffi::c_int,
) {
    if (*sl).pwt.use_weight == 2 as ::core::ffi::c_int && list0 != 0 && list1 != 0
        && (*sl)
            .pwt
            .implicit_weight[(*sl)
            .ref_cache[0 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][(*sl)
            .ref_cache[1 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][((*sl).mb_y & 1 as ::core::ffi::c_int) as usize]
            != 32 as ::core::ffi::c_int
        || (*sl).pwt.use_weight == 1 as ::core::ffi::c_int
    {
        mc_part_weighted(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            *weight_op.offset(0 as ::core::ffi::c_int as isize),
            *weight_op.offset(1 as ::core::ffi::c_int as isize),
            *weight_avg.offset(0 as ::core::ffi::c_int as isize),
            *weight_avg.offset(1 as ::core::ffi::c_int as isize),
            list0,
            list1,
            8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
        );
    } else {
        mc_part_std(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            qpix_avg,
            chroma_avg,
            list0,
            list1,
            8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn mc_part_420_simple_8(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut square: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut x_offset: ::core::ffi::c_int,
    mut y_offset: ::core::ffi::c_int,
    mut qpix_put: *const qpel_mc_func,
    mut chroma_put: h264_chroma_mc_func,
    mut qpix_avg: *const qpel_mc_func,
    mut chroma_avg: h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
    mut list0: ::core::ffi::c_int,
    mut list1: ::core::ffi::c_int,
) {
    if (*sl).pwt.use_weight == 2 as ::core::ffi::c_int && list0 != 0 && list1 != 0
        && (*sl)
            .pwt
            .implicit_weight[(*sl)
            .ref_cache[0 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][(*sl)
            .ref_cache[1 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][((*sl).mb_y & 1 as ::core::ffi::c_int) as usize]
            != 32 as ::core::ffi::c_int
        || (*sl).pwt.use_weight == 1 as ::core::ffi::c_int
    {
        mc_part_weighted(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            *weight_op.offset(0 as ::core::ffi::c_int as isize),
            *weight_op.offset(1 as ::core::ffi::c_int as isize),
            *weight_avg.offset(0 as ::core::ffi::c_int as isize),
            *weight_avg.offset(1 as ::core::ffi::c_int as isize),
            list0,
            list1,
            8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    } else {
        mc_part_std(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            qpix_avg,
            chroma_avg,
            list0,
            list1,
            8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn mc_part_420_simple_16(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut square: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut x_offset: ::core::ffi::c_int,
    mut y_offset: ::core::ffi::c_int,
    mut qpix_put: *const qpel_mc_func,
    mut chroma_put: h264_chroma_mc_func,
    mut qpix_avg: *const qpel_mc_func,
    mut chroma_avg: h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
    mut list0: ::core::ffi::c_int,
    mut list1: ::core::ffi::c_int,
) {
    if (*sl).pwt.use_weight == 2 as ::core::ffi::c_int && list0 != 0 && list1 != 0
        && (*sl)
            .pwt
            .implicit_weight[(*sl)
            .ref_cache[0 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][(*sl)
            .ref_cache[1 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][((*sl).mb_y & 1 as ::core::ffi::c_int) as usize]
            != 32 as ::core::ffi::c_int
        || (*sl).pwt.use_weight == 1 as ::core::ffi::c_int
    {
        mc_part_weighted(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            *weight_op.offset(0 as ::core::ffi::c_int as isize),
            *weight_op.offset(1 as ::core::ffi::c_int as isize),
            *weight_avg.offset(0 as ::core::ffi::c_int as isize),
            *weight_avg.offset(1 as ::core::ffi::c_int as isize),
            list0,
            list1,
            16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    } else {
        mc_part_std(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            qpix_avg,
            chroma_avg,
            list0,
            list1,
            16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn mc_part_422_simple_8(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut square: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut x_offset: ::core::ffi::c_int,
    mut y_offset: ::core::ffi::c_int,
    mut qpix_put: *const qpel_mc_func,
    mut chroma_put: h264_chroma_mc_func,
    mut qpix_avg: *const qpel_mc_func,
    mut chroma_avg: h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
    mut list0: ::core::ffi::c_int,
    mut list1: ::core::ffi::c_int,
) {
    if (*sl).pwt.use_weight == 2 as ::core::ffi::c_int && list0 != 0 && list1 != 0
        && (*sl)
            .pwt
            .implicit_weight[(*sl)
            .ref_cache[0 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][(*sl)
            .ref_cache[1 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][((*sl).mb_y & 1 as ::core::ffi::c_int) as usize]
            != 32 as ::core::ffi::c_int
        || (*sl).pwt.use_weight == 1 as ::core::ffi::c_int
    {
        mc_part_weighted(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            *weight_op.offset(0 as ::core::ffi::c_int as isize),
            *weight_op.offset(1 as ::core::ffi::c_int as isize),
            *weight_avg.offset(0 as ::core::ffi::c_int as isize),
            *weight_avg.offset(1 as ::core::ffi::c_int as isize),
            list0,
            list1,
            8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
    } else {
        mc_part_std(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            qpix_avg,
            chroma_avg,
            list0,
            list1,
            8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn mc_part_444_complex(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut square: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut x_offset: ::core::ffi::c_int,
    mut y_offset: ::core::ffi::c_int,
    mut qpix_put: *const qpel_mc_func,
    mut chroma_put: h264_chroma_mc_func,
    mut qpix_avg: *const qpel_mc_func,
    mut chroma_avg: h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
    mut list0: ::core::ffi::c_int,
    mut list1: ::core::ffi::c_int,
) {
    if (*sl).pwt.use_weight == 2 as ::core::ffi::c_int && list0 != 0 && list1 != 0
        && (*sl)
            .pwt
            .implicit_weight[(*sl)
            .ref_cache[0 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][(*sl)
            .ref_cache[1 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][((*sl).mb_y & 1 as ::core::ffi::c_int) as usize]
            != 32 as ::core::ffi::c_int
        || (*sl).pwt.use_weight == 1 as ::core::ffi::c_int
    {
        mc_part_weighted(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            *weight_op.offset(0 as ::core::ffi::c_int as isize),
            *weight_op.offset(1 as ::core::ffi::c_int as isize),
            *weight_avg.offset(0 as ::core::ffi::c_int as isize),
            *weight_avg.offset(1 as ::core::ffi::c_int as isize),
            list0,
            list1,
            (*h).pixel_shift,
            3 as ::core::ffi::c_int,
        );
    } else {
        mc_part_std(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            qpix_avg,
            chroma_avg,
            list0,
            list1,
            (*h).pixel_shift,
            3 as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn mc_part_422_simple_16(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut square: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut x_offset: ::core::ffi::c_int,
    mut y_offset: ::core::ffi::c_int,
    mut qpix_put: *const qpel_mc_func,
    mut chroma_put: h264_chroma_mc_func,
    mut qpix_avg: *const qpel_mc_func,
    mut chroma_avg: h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
    mut list0: ::core::ffi::c_int,
    mut list1: ::core::ffi::c_int,
) {
    if (*sl).pwt.use_weight == 2 as ::core::ffi::c_int && list0 != 0 && list1 != 0
        && (*sl)
            .pwt
            .implicit_weight[(*sl)
            .ref_cache[0 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][(*sl)
            .ref_cache[1 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][((*sl).mb_y & 1 as ::core::ffi::c_int) as usize]
            != 32 as ::core::ffi::c_int
        || (*sl).pwt.use_weight == 1 as ::core::ffi::c_int
    {
        mc_part_weighted(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            *weight_op.offset(0 as ::core::ffi::c_int as isize),
            *weight_op.offset(1 as ::core::ffi::c_int as isize),
            *weight_avg.offset(0 as ::core::ffi::c_int as isize),
            *weight_avg.offset(1 as ::core::ffi::c_int as isize),
            list0,
            list1,
            16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
    } else {
        mc_part_std(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            qpix_avg,
            chroma_avg,
            list0,
            list1,
            16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn mc_part_422_complex(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut square: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut x_offset: ::core::ffi::c_int,
    mut y_offset: ::core::ffi::c_int,
    mut qpix_put: *const qpel_mc_func,
    mut chroma_put: h264_chroma_mc_func,
    mut qpix_avg: *const qpel_mc_func,
    mut chroma_avg: h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
    mut list0: ::core::ffi::c_int,
    mut list1: ::core::ffi::c_int,
) {
    if (*sl).pwt.use_weight == 2 as ::core::ffi::c_int && list0 != 0 && list1 != 0
        && (*sl)
            .pwt
            .implicit_weight[(*sl)
            .ref_cache[0 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][(*sl)
            .ref_cache[1 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][((*sl).mb_y & 1 as ::core::ffi::c_int) as usize]
            != 32 as ::core::ffi::c_int
        || (*sl).pwt.use_weight == 1 as ::core::ffi::c_int
    {
        mc_part_weighted(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            *weight_op.offset(0 as ::core::ffi::c_int as isize),
            *weight_op.offset(1 as ::core::ffi::c_int as isize),
            *weight_avg.offset(0 as ::core::ffi::c_int as isize),
            *weight_avg.offset(1 as ::core::ffi::c_int as isize),
            list0,
            list1,
            (*h).pixel_shift,
            2 as ::core::ffi::c_int,
        );
    } else {
        mc_part_std(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            qpix_avg,
            chroma_avg,
            list0,
            list1,
            (*h).pixel_shift,
            2 as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn mc_part_420_complex(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut square: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut x_offset: ::core::ffi::c_int,
    mut y_offset: ::core::ffi::c_int,
    mut qpix_put: *const qpel_mc_func,
    mut chroma_put: h264_chroma_mc_func,
    mut qpix_avg: *const qpel_mc_func,
    mut chroma_avg: h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
    mut list0: ::core::ffi::c_int,
    mut list1: ::core::ffi::c_int,
) {
    if (*sl).pwt.use_weight == 2 as ::core::ffi::c_int && list0 != 0 && list1 != 0
        && (*sl)
            .pwt
            .implicit_weight[(*sl)
            .ref_cache[0 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][(*sl)
            .ref_cache[1 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as usize][((*sl).mb_y & 1 as ::core::ffi::c_int) as usize]
            != 32 as ::core::ffi::c_int
        || (*sl).pwt.use_weight == 1 as ::core::ffi::c_int
    {
        mc_part_weighted(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            *weight_op.offset(0 as ::core::ffi::c_int as isize),
            *weight_op.offset(1 as ::core::ffi::c_int as isize),
            *weight_avg.offset(0 as ::core::ffi::c_int as isize),
            *weight_avg.offset(1 as ::core::ffi::c_int as isize),
            list0,
            list1,
            (*h).pixel_shift,
            1 as ::core::ffi::c_int,
        );
    } else {
        mc_part_std(
            h,
            sl,
            n,
            square,
            height,
            delta,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            qpix_avg,
            chroma_avg,
            list0,
            list1,
            (*h).pixel_shift,
            1 as ::core::ffi::c_int,
        );
    };
}
static mut scan8: [uint8_t; 51] = [
    (4 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 6 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 7 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 11 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 12 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (4 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (5 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 13 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (6 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (7 as ::core::ffi::c_int + 14 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
    (0 as ::core::ffi::c_int + 10 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
        as uint8_t,
];
#[inline]
unsafe extern "C" fn get_lowest_part_list_y(
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut y_offset: ::core::ffi::c_int,
    mut list: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut raw_my: ::core::ffi::c_int = (*sl)
        .mv_cache[list
        as usize][scan8[n as usize] as usize][1 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int;
    let mut filter_height_down: ::core::ffi::c_int = if raw_my & 3 as ::core::ffi::c_int
        != 0
    {
        3 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
    let mut full_my: ::core::ffi::c_int = (raw_my >> 2 as ::core::ffi::c_int) + y_offset;
    let mut bottom: ::core::ffi::c_int = full_my + filter_height_down + height;
    return if 0 as ::core::ffi::c_int > bottom {
        0 as ::core::ffi::c_int
    } else {
        bottom
    };
}
#[inline(never)]
unsafe extern "C" fn hl_decode_mb_simple_16(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mb_x: ::core::ffi::c_int = (*sl).mb_x;
    let mb_y: ::core::ffi::c_int = (*sl).mb_y;
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    let mut dest_y: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut dest_cb: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut dest_cr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut linesize: ::core::ffi::c_int = 0;
    let mut uvlinesize: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut block_offset: *const ::core::ffi::c_int = (&raw const (*h).block_offset
        as *const ::core::ffi::c_int)
        .offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_int;
    let transform_bypass: ::core::ffi::c_int = (1 as ::core::ffi::c_int == 0
        && ((*sl).qscale == 0 as ::core::ffi::c_int
            && (*(*h).ps.sps).transform_bypass != 0)) as ::core::ffi::c_int;
    let mut idct_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    > = None;
    let block_h: ::core::ffi::c_int = 16 as ::core::ffi::c_int >> (*h).chroma_y_shift;
    let chroma422: ::core::ffi::c_int = ((*(*h).ps.sps).chroma_format_idc
        == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    dest_y = (*(*h).cur_pic.f)
        .data[0 as ::core::ffi::c_int as usize]
        .offset(
            (((mb_x << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                + mb_y * (*sl).linesize as ::core::ffi::c_int)
                * 16 as ::core::ffi::c_int) as isize,
        );
    dest_cb = (*(*h).cur_pic.f)
        .data[1 as ::core::ffi::c_int as usize]
        .offset(
            ((mb_x << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                * 8 as ::core::ffi::c_int) as isize,
        )
        .offset((mb_y * (*sl).uvlinesize as ::core::ffi::c_int * block_h) as isize);
    dest_cr = (*(*h).cur_pic.f)
        .data[2 as ::core::ffi::c_int as usize]
        .offset(
            ((mb_x << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                * 8 as ::core::ffi::c_int) as isize,
        )
        .offset((mb_y * (*sl).uvlinesize as ::core::ffi::c_int * block_h) as isize);
    (*h)
        .vdsp
        .prefetch
        .expect(
            "non-null function pointer",
        )(
        dest_y
            .offset(
                (((*sl).mb_x as ptrdiff_t & 3 as ptrdiff_t) * 4 as ptrdiff_t
                    * (*sl).linesize) as isize,
            )
            .offset(
                ((64 as ::core::ffi::c_int)
                    << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)) as isize,
            ),
        (*sl).linesize,
        4 as ::core::ffi::c_int,
    );
    (*h)
        .vdsp
        .prefetch
        .expect(
            "non-null function pointer",
        )(
        dest_cb
            .offset(
                (((*sl).mb_x as ptrdiff_t & 7 as ptrdiff_t) * (*sl).uvlinesize) as isize,
            )
            .offset(
                ((64 as ::core::ffi::c_int)
                    << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)) as isize,
            ),
        dest_cr.offset_from(dest_cb) as ptrdiff_t,
        2 as ::core::ffi::c_int,
    );
    *(*h).list_counts.offset(mb_xy as isize) = (*sl).list_count as uint8_t;
    if 1 as ::core::ffi::c_int == 0 && (*sl).mb_field_decoding_flag != 0 {
        (*sl).mb_linesize = ((*sl).linesize as ::core::ffi::c_int
            * 2 as ::core::ffi::c_int) as ptrdiff_t;
        linesize = (*sl).mb_linesize as ::core::ffi::c_int;
        (*sl).mb_uvlinesize = ((*sl).uvlinesize as ::core::ffi::c_int
            * 2 as ::core::ffi::c_int) as ptrdiff_t;
        uvlinesize = (*sl).mb_uvlinesize as ::core::ffi::c_int;
        block_offset = (&raw const (*h).block_offset as *const ::core::ffi::c_int)
            .offset(48 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_int;
        if mb_y & 1 as ::core::ffi::c_int != 0 {
            dest_y = dest_y
                .offset(
                    -(((*sl).linesize as ::core::ffi::c_int * 15 as ::core::ffi::c_int)
                        as isize),
                );
            dest_cb = dest_cb
                .offset(
                    -(((*sl).uvlinesize as ::core::ffi::c_int
                        * (block_h - 1 as ::core::ffi::c_int)) as isize),
                );
            dest_cr = dest_cr
                .offset(
                    -(((*sl).uvlinesize as ::core::ffi::c_int
                        * (block_h - 1 as ::core::ffi::c_int)) as isize),
                );
        }
        if (*h).mb_aff_frame != 0 {
            let mut list: ::core::ffi::c_int = 0;
            list = 0 as ::core::ffi::c_int;
            while (list as ::core::ffi::c_uint) < (*sl).list_count {
                if !(mb_type
                    & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int * list == 0)
                {
                    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        != 0
                    {
                        let mut ref_0: *mut int8_t = (&raw mut *(&raw mut (*sl).ref_cache
                            as *mut [int8_t; 40])
                            .offset(list as isize) as *mut int8_t)
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize) as isize,
                            ) as *mut int8_t;
                        fill_rectangle(
                            ref_0 as *mut ::core::ffi::c_void,
                            4 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            (16 as ::core::ffi::c_int + *ref_0 as ::core::ffi::c_int
                                ^ (*sl).mb_y & 1 as ::core::ffi::c_int) as uint32_t,
                            1 as ::core::ffi::c_int,
                        );
                    } else {
                        i = 0 as ::core::ffi::c_int;
                        while i < 16 as ::core::ffi::c_int {
                            let mut ref_1: ::core::ffi::c_int = (*sl)
                                .ref_cache[list as usize][scan8[i as usize] as usize]
                                as ::core::ffi::c_int;
                            if ref_1 >= 0 as ::core::ffi::c_int {
                                fill_rectangle(
                                    (&raw mut *(&raw mut (*sl).ref_cache as *mut [int8_t; 40])
                                        .offset(list as isize) as *mut int8_t)
                                        .offset(
                                            *(&raw const scan8 as *const uint8_t).offset(i as isize)
                                                as isize,
                                        ) as *mut int8_t as *mut ::core::ffi::c_void,
                                    2 as ::core::ffi::c_int,
                                    2 as ::core::ffi::c_int,
                                    8 as ::core::ffi::c_int,
                                    (16 as ::core::ffi::c_int + ref_1
                                        ^ (*sl).mb_y & 1 as ::core::ffi::c_int) as uint32_t,
                                    1 as ::core::ffi::c_int,
                                );
                            }
                            i += 4 as ::core::ffi::c_int;
                        }
                    }
                }
                list += 1;
            }
        }
    } else {
        (*sl).mb_linesize = (*sl).linesize;
        linesize = (*sl).mb_linesize as ::core::ffi::c_int;
        (*sl).mb_uvlinesize = (*sl).uvlinesize;
        uvlinesize = (*sl).mb_uvlinesize as ::core::ffi::c_int;
    }
    if 1 as ::core::ffi::c_int == 0
        && mb_type & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        let bit_depth: ::core::ffi::c_int = (*(*h).ps.sps).bit_depth_luma;
        if 16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int != 0 {
            let mut j_0: ::core::ffi::c_int = 0;
            let mut gb: GetBitContext = GetBitContext {
                buffer: ::core::ptr::null::<uint8_t>(),
                index: 0,
                size_in_bits: 0,
                size_in_bits_plus8: 0,
            };
            init_get_bits(
                &raw mut gb,
                (*sl).intra_pcm_ptr,
                ff_h264_mb_sizes[(*(*h).ps.sps).chroma_format_idc as usize]
                    as ::core::ffi::c_int * bit_depth,
            );
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                let mut tmp_y: *mut uint16_t = dest_y.offset((i * linesize) as isize)
                    as *mut uint16_t;
                j_0 = 0 as ::core::ffi::c_int;
                while j_0 < 16 as ::core::ffi::c_int {
                    *tmp_y.offset(j_0 as isize) = get_bits(&raw mut gb, bit_depth)
                        as uint16_t;
                    j_0 += 1;
                }
                i += 1;
            }
            if 1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                    == 0
            {
                if (*(*h).ps.sps).chroma_format_idc == 0 {
                    i = 0 as ::core::ffi::c_int;
                    while i < block_h {
                        let mut tmp_cb: *mut uint16_t = dest_cb
                            .offset((i * uvlinesize) as isize) as *mut uint16_t;
                        let mut tmp_cr: *mut uint16_t = dest_cr
                            .offset((i * uvlinesize) as isize) as *mut uint16_t;
                        j_0 = 0 as ::core::ffi::c_int;
                        while j_0 < 8 as ::core::ffi::c_int {
                            let ref mut fresh1 = *tmp_cr.offset(j_0 as isize);
                            *fresh1 = ((1 as ::core::ffi::c_int)
                                << bit_depth - 1 as ::core::ffi::c_int) as uint16_t;
                            *tmp_cb.offset(j_0 as isize) = *fresh1;
                            j_0 += 1;
                        }
                        i += 1;
                    }
                } else {
                    i = 0 as ::core::ffi::c_int;
                    while i < block_h {
                        let mut tmp_cb_0: *mut uint16_t = dest_cb
                            .offset((i * uvlinesize) as isize) as *mut uint16_t;
                        j_0 = 0 as ::core::ffi::c_int;
                        while j_0 < 8 as ::core::ffi::c_int {
                            *tmp_cb_0.offset(j_0 as isize) = get_bits(
                                &raw mut gb,
                                bit_depth,
                            ) as uint16_t;
                            j_0 += 1;
                        }
                        i += 1;
                    }
                    i = 0 as ::core::ffi::c_int;
                    while i < block_h {
                        let mut tmp_cr_0: *mut uint16_t = dest_cr
                            .offset((i * uvlinesize) as isize) as *mut uint16_t;
                        j_0 = 0 as ::core::ffi::c_int;
                        while j_0 < 8 as ::core::ffi::c_int {
                            *tmp_cr_0.offset(j_0 as isize) = get_bits(
                                &raw mut gb,
                                bit_depth,
                            ) as uint16_t;
                            j_0 += 1;
                        }
                        i += 1;
                    }
                }
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                memcpy(
                    dest_y.offset((i * linesize) as isize) as *mut ::core::ffi::c_void,
                    (*sl).intra_pcm_ptr.offset((i * 16 as ::core::ffi::c_int) as isize)
                        as *const ::core::ffi::c_void,
                    16 as size_t,
                );
                i += 1;
            }
            if 1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                    == 0
            {
                if (*(*h).ps.sps).chroma_format_idc == 0 {
                    i = 0 as ::core::ffi::c_int;
                    while i < 8 as ::core::ffi::c_int {
                        memset(
                            dest_cb.offset((i * uvlinesize) as isize)
                                as *mut ::core::ffi::c_void,
                            (1 as ::core::ffi::c_int)
                                << bit_depth - 1 as ::core::ffi::c_int,
                            8 as size_t,
                        );
                        memset(
                            dest_cr.offset((i * uvlinesize) as isize)
                                as *mut ::core::ffi::c_void,
                            (1 as ::core::ffi::c_int)
                                << bit_depth - 1 as ::core::ffi::c_int,
                            8 as size_t,
                        );
                        i += 1;
                    }
                } else {
                    let mut src_cb: *const uint8_t = (*sl)
                        .intra_pcm_ptr
                        .offset(256 as ::core::ffi::c_int as isize);
                    let mut src_cr: *const uint8_t = (*sl)
                        .intra_pcm_ptr
                        .offset(256 as ::core::ffi::c_int as isize)
                        .offset((block_h * 8 as ::core::ffi::c_int) as isize);
                    i = 0 as ::core::ffi::c_int;
                    while i < block_h {
                        memcpy(
                            dest_cb.offset((i * uvlinesize) as isize)
                                as *mut ::core::ffi::c_void,
                            src_cb.offset((i * 8 as ::core::ffi::c_int) as isize)
                                as *const ::core::ffi::c_void,
                            8 as size_t,
                        );
                        memcpy(
                            dest_cr.offset((i * uvlinesize) as isize)
                                as *mut ::core::ffi::c_void,
                            src_cr.offset((i * 8 as ::core::ffi::c_int) as isize)
                                as *const ::core::ffi::c_void,
                            8 as size_t,
                        );
                        i += 1;
                    }
                }
            }
        }
    } else {
        if mb_type & 7 as ::core::ffi::c_int != 0 {
            if (*sl).deblocking_filter != 0 {
                xchg_mb_border(
                    h,
                    sl,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    linesize,
                    uvlinesize,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                );
            }
            if 1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                    == 0
            {
                (*h)
                    .hpc
                    .pred8x8[(*sl).chroma_pred_mode as usize]
                    .expect(
                        "non-null function pointer",
                    )(dest_cb, uvlinesize as ptrdiff_t);
                (*h)
                    .hpc
                    .pred8x8[(*sl).chroma_pred_mode as usize]
                    .expect(
                        "non-null function pointer",
                    )(dest_cr, uvlinesize as ptrdiff_t);
            }
            hl_decode_mb_predict_luma(
                h,
                sl,
                mb_type,
                1 as ::core::ffi::c_int,
                transform_bypass,
                16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                block_offset,
                linesize,
                dest_y,
                0 as ::core::ffi::c_int,
            );
            if (*sl).deblocking_filter != 0 {
                xchg_mb_border(
                    h,
                    sl,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    linesize,
                    uvlinesize,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                );
            }
        } else if chroma422 != 0 {
            hl_motion_422_simple_16(
                h,
                sl,
                dest_y,
                dest_cb,
                dest_cr,
                &raw const (*h).h264qpel.put_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.put_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264qpel.avg_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.avg_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264dsp.weight_pixels_tab as *const h264_weight_func,
                &raw const (*h).h264dsp.biweight_pixels_tab as *const h264_biweight_func,
            );
        } else {
            hl_motion_420_simple_16(
                h,
                sl,
                dest_y,
                dest_cb,
                dest_cr,
                &raw const (*h).h264qpel.put_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.put_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264qpel.avg_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.avg_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264dsp.weight_pixels_tab as *const h264_weight_func,
                &raw const (*h).h264dsp.biweight_pixels_tab as *const h264_biweight_func,
            );
        }
        hl_decode_mb_idct_luma(
            h,
            sl,
            mb_type,
            1 as ::core::ffi::c_int,
            transform_bypass,
            16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            block_offset,
            linesize,
            dest_y,
            0 as ::core::ffi::c_int,
        );
        if (1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
            || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int == 0)
            && (*sl).cbp & 0x30 as ::core::ffi::c_int != 0
        {
            let mut dest: [*mut uint8_t; 2] = [dest_cb, dest_cr];
            if transform_bypass != 0 {
                if mb_type & 7 as ::core::ffi::c_int != 0
                    && (*(*h).ps.sps).profile_idc == 244 as ::core::ffi::c_int
                    && ((*sl).chroma_pred_mode == 2 as ::core::ffi::c_int
                        || (*sl).chroma_pred_mode == 1 as ::core::ffi::c_int)
                {
                    (*h)
                        .hpc
                        .pred8x8_add[(*sl).chroma_pred_mode as usize]
                        .expect(
                            "non-null function pointer",
                        )(
                        dest[0 as ::core::ffi::c_int as usize],
                        block_offset.offset(16 as ::core::ffi::c_int as isize),
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                    * 1 as ::core::ffi::c_int)
                                    << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                    as isize,
                            ),
                        uvlinesize as ptrdiff_t,
                    );
                    (*h)
                        .hpc
                        .pred8x8_add[(*sl).chroma_pred_mode as usize]
                        .expect(
                            "non-null function pointer",
                        )(
                        dest[1 as ::core::ffi::c_int as usize],
                        block_offset.offset(32 as ::core::ffi::c_int as isize),
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int)
                                    << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                    as isize,
                            ),
                        uvlinesize as ptrdiff_t,
                    );
                } else {
                    idct_add = (*h).h264dsp.add_pixels4_clear;
                    j = 1 as ::core::ffi::c_int;
                    while j < 3 as ::core::ffi::c_int {
                        i = j * 16 as ::core::ffi::c_int;
                        while i < j * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                        {
                            if (*sl).non_zero_count_cache[scan8[i as usize] as usize]
                                as ::core::ffi::c_int != 0
                                || dctcoef_get(
                                    &raw mut (*sl).mb as *mut int16_t,
                                    16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                                    i * 16 as ::core::ffi::c_int,
                                ) != 0
                            {
                                idct_add
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dest[(j - 1 as ::core::ffi::c_int) as usize]
                                        .offset(*block_offset.offset(i as isize) as isize),
                                    (&raw mut (*sl).mb as *mut int16_t)
                                        .offset(
                                            ((i * 16 as ::core::ffi::c_int)
                                                << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                                as isize,
                                        ),
                                    uvlinesize,
                                );
                            }
                            i += 1;
                        }
                        if chroma422 != 0 {
                            i = j * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int;
                            while i
                                < j * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                            {
                                if (*sl)
                                    .non_zero_count_cache[scan8[(i + 4 as ::core::ffi::c_int)
                                    as usize] as usize] as ::core::ffi::c_int != 0
                                    || dctcoef_get(
                                        &raw mut (*sl).mb as *mut int16_t,
                                        16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                                        i * 16 as ::core::ffi::c_int,
                                    ) != 0
                                {
                                    idct_add
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        dest[(j - 1 as ::core::ffi::c_int) as usize]
                                            .offset(
                                                *block_offset.offset((i + 4 as ::core::ffi::c_int) as isize)
                                                    as isize,
                                            ),
                                        (&raw mut (*sl).mb as *mut int16_t)
                                            .offset(
                                                ((i * 16 as ::core::ffi::c_int)
                                                    << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                                    as isize,
                                            ),
                                        uvlinesize,
                                    );
                                }
                                i += 1;
                            }
                        }
                        j += 1;
                    }
                }
            } else {
                let mut qp: [::core::ffi::c_int; 2] = [0; 2];
                if chroma422 != 0 {
                    qp[0 as ::core::ffi::c_int as usize] = (*sl)
                        .chroma_qp[0 as ::core::ffi::c_int as usize]
                        + 3 as ::core::ffi::c_int;
                    qp[1 as ::core::ffi::c_int as usize] = (*sl)
                        .chroma_qp[1 as ::core::ffi::c_int as usize]
                        + 3 as ::core::ffi::c_int;
                } else {
                    qp[0 as ::core::ffi::c_int as usize] = (*sl)
                        .chroma_qp[0 as ::core::ffi::c_int as usize];
                    qp[1 as ::core::ffi::c_int as usize] = (*sl)
                        .chroma_qp[1 as ::core::ffi::c_int as usize];
                }
                if (*sl)
                    .non_zero_count_cache[scan8[(49 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int) as usize] as usize] != 0
                {
                    (*h)
                        .h264dsp
                        .chroma_dc_dequant_idct
                        .expect(
                            "non-null function pointer",
                        )(
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                    * 1 as ::core::ffi::c_int)
                                    << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                    as isize,
                            ),
                        (*(*(*h).ps.pps)
                            .dequant4_coeff[(if mb_type & 7 as ::core::ffi::c_int != 0 {
                                1 as ::core::ffi::c_int
                            } else {
                                4 as ::core::ffi::c_int
                            }) as usize]
                            .offset(
                                qp[0 as ::core::ffi::c_int as usize] as isize,
                            ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    );
                }
                if (*sl)
                    .non_zero_count_cache[scan8[(49 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize] as usize] != 0
                {
                    (*h)
                        .h264dsp
                        .chroma_dc_dequant_idct
                        .expect(
                            "non-null function pointer",
                        )(
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int)
                                    << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                    as isize,
                            ),
                        (*(*(*h).ps.pps)
                            .dequant4_coeff[(if mb_type & 7 as ::core::ffi::c_int != 0 {
                                2 as ::core::ffi::c_int
                            } else {
                                5 as ::core::ffi::c_int
                            }) as usize]
                            .offset(
                                qp[1 as ::core::ffi::c_int as usize] as isize,
                            ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    );
                }
                (*h)
                    .h264dsp
                    .idct_add8
                    .expect(
                        "non-null function pointer",
                    )(
                    &raw mut dest as *mut *mut uint8_t,
                    block_offset,
                    &raw mut (*sl).mb as *mut int16_t,
                    uvlinesize,
                    &raw mut (*sl).non_zero_count_cache as *mut uint8_t as *const uint8_t,
                );
            }
        }
    };
}
#[inline(never)]
unsafe extern "C" fn hl_decode_mb_complex(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mb_x: ::core::ffi::c_int = (*sl).mb_x;
    let mb_y: ::core::ffi::c_int = (*sl).mb_y;
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    let mut dest_y: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut dest_cb: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut dest_cr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut linesize: ::core::ffi::c_int = 0;
    let mut uvlinesize: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut block_offset: *const ::core::ffi::c_int = (&raw const (*h).block_offset
        as *const ::core::ffi::c_int)
        .offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_int;
    let transform_bypass: ::core::ffi::c_int = (0 as ::core::ffi::c_int == 0
        && ((*sl).qscale == 0 as ::core::ffi::c_int
            && (*(*h).ps.sps).transform_bypass != 0)) as ::core::ffi::c_int;
    let mut idct_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    > = None;
    let block_h: ::core::ffi::c_int = 16 as ::core::ffi::c_int >> (*h).chroma_y_shift;
    let chroma422: ::core::ffi::c_int = ((*(*h).ps.sps).chroma_format_idc
        == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    dest_y = (*(*h).cur_pic.f)
        .data[0 as ::core::ffi::c_int as usize]
        .offset(
            (((mb_x << (*h).pixel_shift) + mb_y * (*sl).linesize as ::core::ffi::c_int)
                * 16 as ::core::ffi::c_int) as isize,
        );
    dest_cb = (*(*h).cur_pic.f)
        .data[1 as ::core::ffi::c_int as usize]
        .offset(((mb_x << (*h).pixel_shift) * 8 as ::core::ffi::c_int) as isize)
        .offset((mb_y * (*sl).uvlinesize as ::core::ffi::c_int * block_h) as isize);
    dest_cr = (*(*h).cur_pic.f)
        .data[2 as ::core::ffi::c_int as usize]
        .offset(((mb_x << (*h).pixel_shift) * 8 as ::core::ffi::c_int) as isize)
        .offset((mb_y * (*sl).uvlinesize as ::core::ffi::c_int * block_h) as isize);
    (*h)
        .vdsp
        .prefetch
        .expect(
            "non-null function pointer",
        )(
        dest_y
            .offset(
                (((*sl).mb_x as ptrdiff_t & 3 as ptrdiff_t) * 4 as ptrdiff_t
                    * (*sl).linesize) as isize,
            )
            .offset(((64 as ::core::ffi::c_int) << (*h).pixel_shift) as isize),
        (*sl).linesize,
        4 as ::core::ffi::c_int,
    );
    (*h)
        .vdsp
        .prefetch
        .expect(
            "non-null function pointer",
        )(
        dest_cb
            .offset(
                (((*sl).mb_x as ptrdiff_t & 7 as ptrdiff_t) * (*sl).uvlinesize) as isize,
            )
            .offset(((64 as ::core::ffi::c_int) << (*h).pixel_shift) as isize),
        dest_cr.offset_from(dest_cb) as ptrdiff_t,
        2 as ::core::ffi::c_int,
    );
    *(*h).list_counts.offset(mb_xy as isize) = (*sl).list_count as uint8_t;
    if 0 as ::core::ffi::c_int == 0 && (*sl).mb_field_decoding_flag != 0 {
        (*sl).mb_linesize = ((*sl).linesize as ::core::ffi::c_int
            * 2 as ::core::ffi::c_int) as ptrdiff_t;
        linesize = (*sl).mb_linesize as ::core::ffi::c_int;
        (*sl).mb_uvlinesize = ((*sl).uvlinesize as ::core::ffi::c_int
            * 2 as ::core::ffi::c_int) as ptrdiff_t;
        uvlinesize = (*sl).mb_uvlinesize as ::core::ffi::c_int;
        block_offset = (&raw const (*h).block_offset as *const ::core::ffi::c_int)
            .offset(48 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_int;
        if mb_y & 1 as ::core::ffi::c_int != 0 {
            dest_y = dest_y
                .offset(
                    -(((*sl).linesize as ::core::ffi::c_int * 15 as ::core::ffi::c_int)
                        as isize),
                );
            dest_cb = dest_cb
                .offset(
                    -(((*sl).uvlinesize as ::core::ffi::c_int
                        * (block_h - 1 as ::core::ffi::c_int)) as isize),
                );
            dest_cr = dest_cr
                .offset(
                    -(((*sl).uvlinesize as ::core::ffi::c_int
                        * (block_h - 1 as ::core::ffi::c_int)) as isize),
                );
        }
        if (*h).mb_aff_frame != 0 {
            let mut list: ::core::ffi::c_int = 0;
            list = 0 as ::core::ffi::c_int;
            while (list as ::core::ffi::c_uint) < (*sl).list_count {
                if !(mb_type
                    & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int * list == 0)
                {
                    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        != 0
                    {
                        let mut ref_0: *mut int8_t = (&raw mut *(&raw mut (*sl).ref_cache
                            as *mut [int8_t; 40])
                            .offset(list as isize) as *mut int8_t)
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize) as isize,
                            ) as *mut int8_t;
                        fill_rectangle(
                            ref_0 as *mut ::core::ffi::c_void,
                            4 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            (16 as ::core::ffi::c_int + *ref_0 as ::core::ffi::c_int
                                ^ (*sl).mb_y & 1 as ::core::ffi::c_int) as uint32_t,
                            1 as ::core::ffi::c_int,
                        );
                    } else {
                        i = 0 as ::core::ffi::c_int;
                        while i < 16 as ::core::ffi::c_int {
                            let mut ref_1: ::core::ffi::c_int = (*sl)
                                .ref_cache[list as usize][scan8[i as usize] as usize]
                                as ::core::ffi::c_int;
                            if ref_1 >= 0 as ::core::ffi::c_int {
                                fill_rectangle(
                                    (&raw mut *(&raw mut (*sl).ref_cache as *mut [int8_t; 40])
                                        .offset(list as isize) as *mut int8_t)
                                        .offset(
                                            *(&raw const scan8 as *const uint8_t).offset(i as isize)
                                                as isize,
                                        ) as *mut int8_t as *mut ::core::ffi::c_void,
                                    2 as ::core::ffi::c_int,
                                    2 as ::core::ffi::c_int,
                                    8 as ::core::ffi::c_int,
                                    (16 as ::core::ffi::c_int + ref_1
                                        ^ (*sl).mb_y & 1 as ::core::ffi::c_int) as uint32_t,
                                    1 as ::core::ffi::c_int,
                                );
                            }
                            i += 4 as ::core::ffi::c_int;
                        }
                    }
                }
                list += 1;
            }
        }
    } else {
        (*sl).mb_linesize = (*sl).linesize;
        linesize = (*sl).mb_linesize as ::core::ffi::c_int;
        (*sl).mb_uvlinesize = (*sl).uvlinesize;
        uvlinesize = (*sl).mb_uvlinesize as ::core::ffi::c_int;
    }
    if 0 as ::core::ffi::c_int == 0
        && mb_type & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        let bit_depth: ::core::ffi::c_int = (*(*h).ps.sps).bit_depth_luma;
        if (*h).pixel_shift != 0 {
            let mut j_0: ::core::ffi::c_int = 0;
            let mut gb: GetBitContext = GetBitContext {
                buffer: ::core::ptr::null::<uint8_t>(),
                index: 0,
                size_in_bits: 0,
                size_in_bits_plus8: 0,
            };
            init_get_bits(
                &raw mut gb,
                (*sl).intra_pcm_ptr,
                ff_h264_mb_sizes[(*(*h).ps.sps).chroma_format_idc as usize]
                    as ::core::ffi::c_int * bit_depth,
            );
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                let mut tmp_y: *mut uint16_t = dest_y.offset((i * linesize) as isize)
                    as *mut uint16_t;
                j_0 = 0 as ::core::ffi::c_int;
                while j_0 < 16 as ::core::ffi::c_int {
                    *tmp_y.offset(j_0 as isize) = get_bits(&raw mut gb, bit_depth)
                        as uint16_t;
                    j_0 += 1;
                }
                i += 1;
            }
            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                    == 0
            {
                if (*(*h).ps.sps).chroma_format_idc == 0 {
                    i = 0 as ::core::ffi::c_int;
                    while i < block_h {
                        let mut tmp_cb: *mut uint16_t = dest_cb
                            .offset((i * uvlinesize) as isize) as *mut uint16_t;
                        let mut tmp_cr: *mut uint16_t = dest_cr
                            .offset((i * uvlinesize) as isize) as *mut uint16_t;
                        j_0 = 0 as ::core::ffi::c_int;
                        while j_0 < 8 as ::core::ffi::c_int {
                            let ref mut fresh2 = *tmp_cr.offset(j_0 as isize);
                            *fresh2 = ((1 as ::core::ffi::c_int)
                                << bit_depth - 1 as ::core::ffi::c_int) as uint16_t;
                            *tmp_cb.offset(j_0 as isize) = *fresh2;
                            j_0 += 1;
                        }
                        i += 1;
                    }
                } else {
                    i = 0 as ::core::ffi::c_int;
                    while i < block_h {
                        let mut tmp_cb_0: *mut uint16_t = dest_cb
                            .offset((i * uvlinesize) as isize) as *mut uint16_t;
                        j_0 = 0 as ::core::ffi::c_int;
                        while j_0 < 8 as ::core::ffi::c_int {
                            *tmp_cb_0.offset(j_0 as isize) = get_bits(
                                &raw mut gb,
                                bit_depth,
                            ) as uint16_t;
                            j_0 += 1;
                        }
                        i += 1;
                    }
                    i = 0 as ::core::ffi::c_int;
                    while i < block_h {
                        let mut tmp_cr_0: *mut uint16_t = dest_cr
                            .offset((i * uvlinesize) as isize) as *mut uint16_t;
                        j_0 = 0 as ::core::ffi::c_int;
                        while j_0 < 8 as ::core::ffi::c_int {
                            *tmp_cr_0.offset(j_0 as isize) = get_bits(
                                &raw mut gb,
                                bit_depth,
                            ) as uint16_t;
                            j_0 += 1;
                        }
                        i += 1;
                    }
                }
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                memcpy(
                    dest_y.offset((i * linesize) as isize) as *mut ::core::ffi::c_void,
                    (*sl).intra_pcm_ptr.offset((i * 16 as ::core::ffi::c_int) as isize)
                        as *const ::core::ffi::c_void,
                    16 as size_t,
                );
                i += 1;
            }
            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                    == 0
            {
                if (*(*h).ps.sps).chroma_format_idc == 0 {
                    i = 0 as ::core::ffi::c_int;
                    while i < 8 as ::core::ffi::c_int {
                        memset(
                            dest_cb.offset((i * uvlinesize) as isize)
                                as *mut ::core::ffi::c_void,
                            (1 as ::core::ffi::c_int)
                                << bit_depth - 1 as ::core::ffi::c_int,
                            8 as size_t,
                        );
                        memset(
                            dest_cr.offset((i * uvlinesize) as isize)
                                as *mut ::core::ffi::c_void,
                            (1 as ::core::ffi::c_int)
                                << bit_depth - 1 as ::core::ffi::c_int,
                            8 as size_t,
                        );
                        i += 1;
                    }
                } else {
                    let mut src_cb: *const uint8_t = (*sl)
                        .intra_pcm_ptr
                        .offset(256 as ::core::ffi::c_int as isize);
                    let mut src_cr: *const uint8_t = (*sl)
                        .intra_pcm_ptr
                        .offset(256 as ::core::ffi::c_int as isize)
                        .offset((block_h * 8 as ::core::ffi::c_int) as isize);
                    i = 0 as ::core::ffi::c_int;
                    while i < block_h {
                        memcpy(
                            dest_cb.offset((i * uvlinesize) as isize)
                                as *mut ::core::ffi::c_void,
                            src_cb.offset((i * 8 as ::core::ffi::c_int) as isize)
                                as *const ::core::ffi::c_void,
                            8 as size_t,
                        );
                        memcpy(
                            dest_cr.offset((i * uvlinesize) as isize)
                                as *mut ::core::ffi::c_void,
                            src_cr.offset((i * 8 as ::core::ffi::c_int) as isize)
                                as *const ::core::ffi::c_void,
                            8 as size_t,
                        );
                        i += 1;
                    }
                }
            }
        }
    } else {
        if mb_type & 7 as ::core::ffi::c_int != 0 {
            if (*sl).deblocking_filter != 0 {
                xchg_mb_border(
                    h,
                    sl,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    linesize,
                    uvlinesize,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    (*h).pixel_shift,
                );
            }
            if 0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                    == 0
            {
                (*h)
                    .hpc
                    .pred8x8[(*sl).chroma_pred_mode as usize]
                    .expect(
                        "non-null function pointer",
                    )(dest_cb, uvlinesize as ptrdiff_t);
                (*h)
                    .hpc
                    .pred8x8[(*sl).chroma_pred_mode as usize]
                    .expect(
                        "non-null function pointer",
                    )(dest_cr, uvlinesize as ptrdiff_t);
            }
            hl_decode_mb_predict_luma(
                h,
                sl,
                mb_type,
                0 as ::core::ffi::c_int,
                transform_bypass,
                (*h).pixel_shift,
                block_offset,
                linesize,
                dest_y,
                0 as ::core::ffi::c_int,
            );
            if (*sl).deblocking_filter != 0 {
                xchg_mb_border(
                    h,
                    sl,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    linesize,
                    uvlinesize,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    (*h).pixel_shift,
                );
            }
        } else if chroma422 != 0 {
            hl_motion_422_complex(
                h,
                sl,
                dest_y,
                dest_cb,
                dest_cr,
                &raw const (*h).h264qpel.put_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.put_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264qpel.avg_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.avg_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264dsp.weight_pixels_tab as *const h264_weight_func,
                &raw const (*h).h264dsp.biweight_pixels_tab as *const h264_biweight_func,
            );
        } else {
            hl_motion_420_complex(
                h,
                sl,
                dest_y,
                dest_cb,
                dest_cr,
                &raw const (*h).h264qpel.put_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.put_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264qpel.avg_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.avg_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264dsp.weight_pixels_tab as *const h264_weight_func,
                &raw const (*h).h264dsp.biweight_pixels_tab as *const h264_biweight_func,
            );
        }
        hl_decode_mb_idct_luma(
            h,
            sl,
            mb_type,
            0 as ::core::ffi::c_int,
            transform_bypass,
            (*h).pixel_shift,
            block_offset,
            linesize,
            dest_y,
            0 as ::core::ffi::c_int,
        );
        if (0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
            || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int == 0)
            && (*sl).cbp & 0x30 as ::core::ffi::c_int != 0
        {
            let mut dest: [*mut uint8_t; 2] = [dest_cb, dest_cr];
            if transform_bypass != 0 {
                if mb_type & 7 as ::core::ffi::c_int != 0
                    && (*(*h).ps.sps).profile_idc == 244 as ::core::ffi::c_int
                    && ((*sl).chroma_pred_mode == 2 as ::core::ffi::c_int
                        || (*sl).chroma_pred_mode == 1 as ::core::ffi::c_int)
                {
                    (*h)
                        .hpc
                        .pred8x8_add[(*sl).chroma_pred_mode as usize]
                        .expect(
                            "non-null function pointer",
                        )(
                        dest[0 as ::core::ffi::c_int as usize],
                        block_offset.offset(16 as ::core::ffi::c_int as isize),
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                    * 1 as ::core::ffi::c_int) << (*h).pixel_shift) as isize,
                            ),
                        uvlinesize as ptrdiff_t,
                    );
                    (*h)
                        .hpc
                        .pred8x8_add[(*sl).chroma_pred_mode as usize]
                        .expect(
                            "non-null function pointer",
                        )(
                        dest[1 as ::core::ffi::c_int as usize],
                        block_offset.offset(32 as ::core::ffi::c_int as isize),
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int) << (*h).pixel_shift) as isize,
                            ),
                        uvlinesize as ptrdiff_t,
                    );
                } else {
                    idct_add = (*h).h264dsp.add_pixels4_clear;
                    j = 1 as ::core::ffi::c_int;
                    while j < 3 as ::core::ffi::c_int {
                        i = j * 16 as ::core::ffi::c_int;
                        while i < j * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                        {
                            if (*sl).non_zero_count_cache[scan8[i as usize] as usize]
                                as ::core::ffi::c_int != 0
                                || dctcoef_get(
                                    &raw mut (*sl).mb as *mut int16_t,
                                    (*h).pixel_shift,
                                    i * 16 as ::core::ffi::c_int,
                                ) != 0
                            {
                                idct_add
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dest[(j - 1 as ::core::ffi::c_int) as usize]
                                        .offset(*block_offset.offset(i as isize) as isize),
                                    (&raw mut (*sl).mb as *mut int16_t)
                                        .offset(
                                            ((i * 16 as ::core::ffi::c_int) << (*h).pixel_shift)
                                                as isize,
                                        ),
                                    uvlinesize,
                                );
                            }
                            i += 1;
                        }
                        if chroma422 != 0 {
                            i = j * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int;
                            while i
                                < j * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                            {
                                if (*sl)
                                    .non_zero_count_cache[scan8[(i + 4 as ::core::ffi::c_int)
                                    as usize] as usize] as ::core::ffi::c_int != 0
                                    || dctcoef_get(
                                        &raw mut (*sl).mb as *mut int16_t,
                                        (*h).pixel_shift,
                                        i * 16 as ::core::ffi::c_int,
                                    ) != 0
                                {
                                    idct_add
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        dest[(j - 1 as ::core::ffi::c_int) as usize]
                                            .offset(
                                                *block_offset.offset((i + 4 as ::core::ffi::c_int) as isize)
                                                    as isize,
                                            ),
                                        (&raw mut (*sl).mb as *mut int16_t)
                                            .offset(
                                                ((i * 16 as ::core::ffi::c_int) << (*h).pixel_shift)
                                                    as isize,
                                            ),
                                        uvlinesize,
                                    );
                                }
                                i += 1;
                            }
                        }
                        j += 1;
                    }
                }
            } else {
                let mut qp: [::core::ffi::c_int; 2] = [0; 2];
                if chroma422 != 0 {
                    qp[0 as ::core::ffi::c_int as usize] = (*sl)
                        .chroma_qp[0 as ::core::ffi::c_int as usize]
                        + 3 as ::core::ffi::c_int;
                    qp[1 as ::core::ffi::c_int as usize] = (*sl)
                        .chroma_qp[1 as ::core::ffi::c_int as usize]
                        + 3 as ::core::ffi::c_int;
                } else {
                    qp[0 as ::core::ffi::c_int as usize] = (*sl)
                        .chroma_qp[0 as ::core::ffi::c_int as usize];
                    qp[1 as ::core::ffi::c_int as usize] = (*sl)
                        .chroma_qp[1 as ::core::ffi::c_int as usize];
                }
                if (*sl)
                    .non_zero_count_cache[scan8[(49 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int) as usize] as usize] != 0
                {
                    (*h)
                        .h264dsp
                        .chroma_dc_dequant_idct
                        .expect(
                            "non-null function pointer",
                        )(
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                    * 1 as ::core::ffi::c_int) << (*h).pixel_shift) as isize,
                            ),
                        (*(*(*h).ps.pps)
                            .dequant4_coeff[(if mb_type & 7 as ::core::ffi::c_int != 0 {
                                1 as ::core::ffi::c_int
                            } else {
                                4 as ::core::ffi::c_int
                            }) as usize]
                            .offset(
                                qp[0 as ::core::ffi::c_int as usize] as isize,
                            ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    );
                }
                if (*sl)
                    .non_zero_count_cache[scan8[(49 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize] as usize] != 0
                {
                    (*h)
                        .h264dsp
                        .chroma_dc_dequant_idct
                        .expect(
                            "non-null function pointer",
                        )(
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int) << (*h).pixel_shift) as isize,
                            ),
                        (*(*(*h).ps.pps)
                            .dequant4_coeff[(if mb_type & 7 as ::core::ffi::c_int != 0 {
                                2 as ::core::ffi::c_int
                            } else {
                                5 as ::core::ffi::c_int
                            }) as usize]
                            .offset(
                                qp[1 as ::core::ffi::c_int as usize] as isize,
                            ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    );
                }
                (*h)
                    .h264dsp
                    .idct_add8
                    .expect(
                        "non-null function pointer",
                    )(
                    &raw mut dest as *mut *mut uint8_t,
                    block_offset,
                    &raw mut (*sl).mb as *mut int16_t,
                    uvlinesize,
                    &raw mut (*sl).non_zero_count_cache as *mut uint8_t as *const uint8_t,
                );
            }
        }
    };
}
#[inline(always)]
unsafe extern "C" fn fill_rectangle(
    mut vp: *mut ::core::ffi::c_void,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut stride: ::core::ffi::c_int,
    mut val: uint32_t,
    mut size: ::core::ffi::c_int,
) {
    let mut p: *mut uint8_t = vp as *mut uint8_t;
    w *= size;
    stride *= size;
    if w == 2 as ::core::ffi::c_int {
        let v: uint16_t = (if size == 4 as ::core::ffi::c_int {
            val
        } else {
            val.wrapping_mul(0x101 as uint32_t)
        }) as uint16_t;
        *(p.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint16_t) = v;
        if h == 1 as ::core::ffi::c_int {
            return;
        }
        *(p.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint16_t) = v;
        if h == 2 as ::core::ffi::c_int {
            return;
        }
        *(p.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint16_t) = v;
        *(p.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint16_t) = v;
    } else if w == 4 as ::core::ffi::c_int {
        let v_0: uint32_t = if size == 4 as ::core::ffi::c_int {
            val
        } else if size == 2 as ::core::ffi::c_int {
            val.wrapping_mul(0x10001 as ::core::ffi::c_int as uint32_t)
        } else {
            val.wrapping_mul(0x1010101 as ::core::ffi::c_int as uint32_t)
        };
        *(p.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_0;
        if h == 1 as ::core::ffi::c_int {
            return;
        }
        *(p.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_0;
        if h == 2 as ::core::ffi::c_int {
            return;
        }
        *(p.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_0;
        *(p.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_0;
    } else if w == 8 as ::core::ffi::c_int {
        let v_1: uint32_t = if size == 2 as ::core::ffi::c_int {
            val.wrapping_mul(0x10001 as ::core::ffi::c_int as uint32_t)
        } else {
            val
        };
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_1;
        *(p
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_1;
        if h == 1 as ::core::ffi::c_int {
            return;
        }
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_1;
        *(p
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_1;
        if h == 2 as ::core::ffi::c_int {
            return;
        }
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_1;
        *(p
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_1;
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_1;
        *(p
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = v_1;
    } else if w == 16 as ::core::ffi::c_int {
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(8 as ::core::ffi::c_int as isize)
            .offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(12 as ::core::ffi::c_int as isize)
            .offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(8 as ::core::ffi::c_int as isize)
            .offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(12 as ::core::ffi::c_int as isize)
            .offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        if h == 2 as ::core::ffi::c_int {
            return;
        }
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(8 as ::core::ffi::c_int as isize)
            .offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(12 as ::core::ffi::c_int as isize)
            .offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(4 as ::core::ffi::c_int as isize)
            .offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(8 as ::core::ffi::c_int as isize)
            .offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
        *(p
            .offset(12 as ::core::ffi::c_int as isize)
            .offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint32_t) = val;
    }
}
#[inline(never)]
unsafe extern "C" fn hl_decode_mb_simple_8(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mb_x: ::core::ffi::c_int = (*sl).mb_x;
    let mb_y: ::core::ffi::c_int = (*sl).mb_y;
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    let mut dest_y: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut dest_cb: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut dest_cr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut linesize: ::core::ffi::c_int = 0;
    let mut uvlinesize: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut block_offset: *const ::core::ffi::c_int = (&raw const (*h).block_offset
        as *const ::core::ffi::c_int)
        .offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_int;
    let transform_bypass: ::core::ffi::c_int = (1 as ::core::ffi::c_int == 0
        && ((*sl).qscale == 0 as ::core::ffi::c_int
            && (*(*h).ps.sps).transform_bypass != 0)) as ::core::ffi::c_int;
    let mut idct_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    > = None;
    let block_h: ::core::ffi::c_int = 16 as ::core::ffi::c_int >> (*h).chroma_y_shift;
    let chroma422: ::core::ffi::c_int = ((*(*h).ps.sps).chroma_format_idc
        == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    dest_y = (*(*h).cur_pic.f)
        .data[0 as ::core::ffi::c_int as usize]
        .offset(
            (((mb_x << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                + mb_y * (*sl).linesize as ::core::ffi::c_int)
                * 16 as ::core::ffi::c_int) as isize,
        );
    dest_cb = (*(*h).cur_pic.f)
        .data[1 as ::core::ffi::c_int as usize]
        .offset(
            ((mb_x << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                * 8 as ::core::ffi::c_int) as isize,
        )
        .offset((mb_y * (*sl).uvlinesize as ::core::ffi::c_int * block_h) as isize);
    dest_cr = (*(*h).cur_pic.f)
        .data[2 as ::core::ffi::c_int as usize]
        .offset(
            ((mb_x << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                * 8 as ::core::ffi::c_int) as isize,
        )
        .offset((mb_y * (*sl).uvlinesize as ::core::ffi::c_int * block_h) as isize);
    (*h)
        .vdsp
        .prefetch
        .expect(
            "non-null function pointer",
        )(
        dest_y
            .offset(
                (((*sl).mb_x as ptrdiff_t & 3 as ptrdiff_t) * 4 as ptrdiff_t
                    * (*sl).linesize) as isize,
            )
            .offset(
                ((64 as ::core::ffi::c_int)
                    << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)) as isize,
            ),
        (*sl).linesize,
        4 as ::core::ffi::c_int,
    );
    (*h)
        .vdsp
        .prefetch
        .expect(
            "non-null function pointer",
        )(
        dest_cb
            .offset(
                (((*sl).mb_x as ptrdiff_t & 7 as ptrdiff_t) * (*sl).uvlinesize) as isize,
            )
            .offset(
                ((64 as ::core::ffi::c_int)
                    << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)) as isize,
            ),
        dest_cr.offset_from(dest_cb) as ptrdiff_t,
        2 as ::core::ffi::c_int,
    );
    *(*h).list_counts.offset(mb_xy as isize) = (*sl).list_count as uint8_t;
    if 1 as ::core::ffi::c_int == 0 && (*sl).mb_field_decoding_flag != 0 {
        (*sl).mb_linesize = ((*sl).linesize as ::core::ffi::c_int
            * 2 as ::core::ffi::c_int) as ptrdiff_t;
        linesize = (*sl).mb_linesize as ::core::ffi::c_int;
        (*sl).mb_uvlinesize = ((*sl).uvlinesize as ::core::ffi::c_int
            * 2 as ::core::ffi::c_int) as ptrdiff_t;
        uvlinesize = (*sl).mb_uvlinesize as ::core::ffi::c_int;
        block_offset = (&raw const (*h).block_offset as *const ::core::ffi::c_int)
            .offset(48 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_int;
        if mb_y & 1 as ::core::ffi::c_int != 0 {
            dest_y = dest_y
                .offset(
                    -(((*sl).linesize as ::core::ffi::c_int * 15 as ::core::ffi::c_int)
                        as isize),
                );
            dest_cb = dest_cb
                .offset(
                    -(((*sl).uvlinesize as ::core::ffi::c_int
                        * (block_h - 1 as ::core::ffi::c_int)) as isize),
                );
            dest_cr = dest_cr
                .offset(
                    -(((*sl).uvlinesize as ::core::ffi::c_int
                        * (block_h - 1 as ::core::ffi::c_int)) as isize),
                );
        }
        if (*h).mb_aff_frame != 0 {
            let mut list: ::core::ffi::c_int = 0;
            list = 0 as ::core::ffi::c_int;
            while (list as ::core::ffi::c_uint) < (*sl).list_count {
                if !(mb_type
                    & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int * list == 0)
                {
                    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        != 0
                    {
                        let mut ref_0: *mut int8_t = (&raw mut *(&raw mut (*sl).ref_cache
                            as *mut [int8_t; 40])
                            .offset(list as isize) as *mut int8_t)
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize) as isize,
                            ) as *mut int8_t;
                        fill_rectangle(
                            ref_0 as *mut ::core::ffi::c_void,
                            4 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            (16 as ::core::ffi::c_int + *ref_0 as ::core::ffi::c_int
                                ^ (*sl).mb_y & 1 as ::core::ffi::c_int) as uint32_t,
                            1 as ::core::ffi::c_int,
                        );
                    } else {
                        i = 0 as ::core::ffi::c_int;
                        while i < 16 as ::core::ffi::c_int {
                            let mut ref_1: ::core::ffi::c_int = (*sl)
                                .ref_cache[list as usize][scan8[i as usize] as usize]
                                as ::core::ffi::c_int;
                            if ref_1 >= 0 as ::core::ffi::c_int {
                                fill_rectangle(
                                    (&raw mut *(&raw mut (*sl).ref_cache as *mut [int8_t; 40])
                                        .offset(list as isize) as *mut int8_t)
                                        .offset(
                                            *(&raw const scan8 as *const uint8_t).offset(i as isize)
                                                as isize,
                                        ) as *mut int8_t as *mut ::core::ffi::c_void,
                                    2 as ::core::ffi::c_int,
                                    2 as ::core::ffi::c_int,
                                    8 as ::core::ffi::c_int,
                                    (16 as ::core::ffi::c_int + ref_1
                                        ^ (*sl).mb_y & 1 as ::core::ffi::c_int) as uint32_t,
                                    1 as ::core::ffi::c_int,
                                );
                            }
                            i += 4 as ::core::ffi::c_int;
                        }
                    }
                }
                list += 1;
            }
        }
    } else {
        (*sl).mb_linesize = (*sl).linesize;
        linesize = (*sl).mb_linesize as ::core::ffi::c_int;
        (*sl).mb_uvlinesize = (*sl).uvlinesize;
        uvlinesize = (*sl).mb_uvlinesize as ::core::ffi::c_int;
    }
    if 1 as ::core::ffi::c_int == 0
        && mb_type & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        let bit_depth: ::core::ffi::c_int = (*(*h).ps.sps).bit_depth_luma;
        if 8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int != 0 {
            let mut j_0: ::core::ffi::c_int = 0;
            let mut gb: GetBitContext = GetBitContext {
                buffer: ::core::ptr::null::<uint8_t>(),
                index: 0,
                size_in_bits: 0,
                size_in_bits_plus8: 0,
            };
            init_get_bits(
                &raw mut gb,
                (*sl).intra_pcm_ptr,
                ff_h264_mb_sizes[(*(*h).ps.sps).chroma_format_idc as usize]
                    as ::core::ffi::c_int * bit_depth,
            );
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                let mut tmp_y: *mut uint16_t = dest_y.offset((i * linesize) as isize)
                    as *mut uint16_t;
                j_0 = 0 as ::core::ffi::c_int;
                while j_0 < 16 as ::core::ffi::c_int {
                    *tmp_y.offset(j_0 as isize) = get_bits(&raw mut gb, bit_depth)
                        as uint16_t;
                    j_0 += 1;
                }
                i += 1;
            }
            if 1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                    == 0
            {
                if (*(*h).ps.sps).chroma_format_idc == 0 {
                    i = 0 as ::core::ffi::c_int;
                    while i < block_h {
                        let mut tmp_cb: *mut uint16_t = dest_cb
                            .offset((i * uvlinesize) as isize) as *mut uint16_t;
                        let mut tmp_cr: *mut uint16_t = dest_cr
                            .offset((i * uvlinesize) as isize) as *mut uint16_t;
                        j_0 = 0 as ::core::ffi::c_int;
                        while j_0 < 8 as ::core::ffi::c_int {
                            let ref mut fresh0 = *tmp_cr.offset(j_0 as isize);
                            *fresh0 = ((1 as ::core::ffi::c_int)
                                << bit_depth - 1 as ::core::ffi::c_int) as uint16_t;
                            *tmp_cb.offset(j_0 as isize) = *fresh0;
                            j_0 += 1;
                        }
                        i += 1;
                    }
                } else {
                    i = 0 as ::core::ffi::c_int;
                    while i < block_h {
                        let mut tmp_cb_0: *mut uint16_t = dest_cb
                            .offset((i * uvlinesize) as isize) as *mut uint16_t;
                        j_0 = 0 as ::core::ffi::c_int;
                        while j_0 < 8 as ::core::ffi::c_int {
                            *tmp_cb_0.offset(j_0 as isize) = get_bits(
                                &raw mut gb,
                                bit_depth,
                            ) as uint16_t;
                            j_0 += 1;
                        }
                        i += 1;
                    }
                    i = 0 as ::core::ffi::c_int;
                    while i < block_h {
                        let mut tmp_cr_0: *mut uint16_t = dest_cr
                            .offset((i * uvlinesize) as isize) as *mut uint16_t;
                        j_0 = 0 as ::core::ffi::c_int;
                        while j_0 < 8 as ::core::ffi::c_int {
                            *tmp_cr_0.offset(j_0 as isize) = get_bits(
                                &raw mut gb,
                                bit_depth,
                            ) as uint16_t;
                            j_0 += 1;
                        }
                        i += 1;
                    }
                }
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                memcpy(
                    dest_y.offset((i * linesize) as isize) as *mut ::core::ffi::c_void,
                    (*sl).intra_pcm_ptr.offset((i * 16 as ::core::ffi::c_int) as isize)
                        as *const ::core::ffi::c_void,
                    16 as size_t,
                );
                i += 1;
            }
            if 1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                    == 0
            {
                if (*(*h).ps.sps).chroma_format_idc == 0 {
                    i = 0 as ::core::ffi::c_int;
                    while i < 8 as ::core::ffi::c_int {
                        memset(
                            dest_cb.offset((i * uvlinesize) as isize)
                                as *mut ::core::ffi::c_void,
                            (1 as ::core::ffi::c_int)
                                << bit_depth - 1 as ::core::ffi::c_int,
                            8 as size_t,
                        );
                        memset(
                            dest_cr.offset((i * uvlinesize) as isize)
                                as *mut ::core::ffi::c_void,
                            (1 as ::core::ffi::c_int)
                                << bit_depth - 1 as ::core::ffi::c_int,
                            8 as size_t,
                        );
                        i += 1;
                    }
                } else {
                    let mut src_cb: *const uint8_t = (*sl)
                        .intra_pcm_ptr
                        .offset(256 as ::core::ffi::c_int as isize);
                    let mut src_cr: *const uint8_t = (*sl)
                        .intra_pcm_ptr
                        .offset(256 as ::core::ffi::c_int as isize)
                        .offset((block_h * 8 as ::core::ffi::c_int) as isize);
                    i = 0 as ::core::ffi::c_int;
                    while i < block_h {
                        memcpy(
                            dest_cb.offset((i * uvlinesize) as isize)
                                as *mut ::core::ffi::c_void,
                            src_cb.offset((i * 8 as ::core::ffi::c_int) as isize)
                                as *const ::core::ffi::c_void,
                            8 as size_t,
                        );
                        memcpy(
                            dest_cr.offset((i * uvlinesize) as isize)
                                as *mut ::core::ffi::c_void,
                            src_cr.offset((i * 8 as ::core::ffi::c_int) as isize)
                                as *const ::core::ffi::c_void,
                            8 as size_t,
                        );
                        i += 1;
                    }
                }
            }
        }
    } else {
        if mb_type & 7 as ::core::ffi::c_int != 0 {
            if (*sl).deblocking_filter != 0 {
                xchg_mb_border(
                    h,
                    sl,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    linesize,
                    uvlinesize,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                );
            }
            if 1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
                || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                    == 0
            {
                (*h)
                    .hpc
                    .pred8x8[(*sl).chroma_pred_mode as usize]
                    .expect(
                        "non-null function pointer",
                    )(dest_cb, uvlinesize as ptrdiff_t);
                (*h)
                    .hpc
                    .pred8x8[(*sl).chroma_pred_mode as usize]
                    .expect(
                        "non-null function pointer",
                    )(dest_cr, uvlinesize as ptrdiff_t);
            }
            hl_decode_mb_predict_luma(
                h,
                sl,
                mb_type,
                1 as ::core::ffi::c_int,
                transform_bypass,
                8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                block_offset,
                linesize,
                dest_y,
                0 as ::core::ffi::c_int,
            );
            if (*sl).deblocking_filter != 0 {
                xchg_mb_border(
                    h,
                    sl,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    linesize,
                    uvlinesize,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                );
            }
        } else if chroma422 != 0 {
            hl_motion_422_simple_8(
                h,
                sl,
                dest_y,
                dest_cb,
                dest_cr,
                &raw const (*h).h264qpel.put_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.put_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264qpel.avg_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.avg_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264dsp.weight_pixels_tab as *const h264_weight_func,
                &raw const (*h).h264dsp.biweight_pixels_tab as *const h264_biweight_func,
            );
        } else {
            hl_motion_420_simple_8(
                h,
                sl,
                dest_y,
                dest_cb,
                dest_cr,
                &raw const (*h).h264qpel.put_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.put_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264qpel.avg_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.avg_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264dsp.weight_pixels_tab as *const h264_weight_func,
                &raw const (*h).h264dsp.biweight_pixels_tab as *const h264_biweight_func,
            );
        }
        hl_decode_mb_idct_luma(
            h,
            sl,
            mb_type,
            1 as ::core::ffi::c_int,
            transform_bypass,
            8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            block_offset,
            linesize,
            dest_y,
            0 as ::core::ffi::c_int,
        );
        if (1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int == 0
            || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int == 0)
            && (*sl).cbp & 0x30 as ::core::ffi::c_int != 0
        {
            let mut dest: [*mut uint8_t; 2] = [dest_cb, dest_cr];
            if transform_bypass != 0 {
                if mb_type & 7 as ::core::ffi::c_int != 0
                    && (*(*h).ps.sps).profile_idc == 244 as ::core::ffi::c_int
                    && ((*sl).chroma_pred_mode == 2 as ::core::ffi::c_int
                        || (*sl).chroma_pred_mode == 1 as ::core::ffi::c_int)
                {
                    (*h)
                        .hpc
                        .pred8x8_add[(*sl).chroma_pred_mode as usize]
                        .expect(
                            "non-null function pointer",
                        )(
                        dest[0 as ::core::ffi::c_int as usize],
                        block_offset.offset(16 as ::core::ffi::c_int as isize),
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                    * 1 as ::core::ffi::c_int)
                                    << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                    as isize,
                            ),
                        uvlinesize as ptrdiff_t,
                    );
                    (*h)
                        .hpc
                        .pred8x8_add[(*sl).chroma_pred_mode as usize]
                        .expect(
                            "non-null function pointer",
                        )(
                        dest[1 as ::core::ffi::c_int as usize],
                        block_offset.offset(32 as ::core::ffi::c_int as isize),
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int)
                                    << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                    as isize,
                            ),
                        uvlinesize as ptrdiff_t,
                    );
                } else {
                    idct_add = (*h).h264dsp.add_pixels4_clear;
                    j = 1 as ::core::ffi::c_int;
                    while j < 3 as ::core::ffi::c_int {
                        i = j * 16 as ::core::ffi::c_int;
                        while i < j * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                        {
                            if (*sl).non_zero_count_cache[scan8[i as usize] as usize]
                                as ::core::ffi::c_int != 0
                                || dctcoef_get(
                                    &raw mut (*sl).mb as *mut int16_t,
                                    8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                                    i * 16 as ::core::ffi::c_int,
                                ) != 0
                            {
                                idct_add
                                    .expect(
                                        "non-null function pointer",
                                    )(
                                    dest[(j - 1 as ::core::ffi::c_int) as usize]
                                        .offset(*block_offset.offset(i as isize) as isize),
                                    (&raw mut (*sl).mb as *mut int16_t)
                                        .offset(
                                            ((i * 16 as ::core::ffi::c_int)
                                                << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                                as isize,
                                        ),
                                    uvlinesize,
                                );
                            }
                            i += 1;
                        }
                        if chroma422 != 0 {
                            i = j * 16 as ::core::ffi::c_int + 4 as ::core::ffi::c_int;
                            while i
                                < j * 16 as ::core::ffi::c_int + 8 as ::core::ffi::c_int
                            {
                                if (*sl)
                                    .non_zero_count_cache[scan8[(i + 4 as ::core::ffi::c_int)
                                    as usize] as usize] as ::core::ffi::c_int != 0
                                    || dctcoef_get(
                                        &raw mut (*sl).mb as *mut int16_t,
                                        8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                                        i * 16 as ::core::ffi::c_int,
                                    ) != 0
                                {
                                    idct_add
                                        .expect(
                                            "non-null function pointer",
                                        )(
                                        dest[(j - 1 as ::core::ffi::c_int) as usize]
                                            .offset(
                                                *block_offset.offset((i + 4 as ::core::ffi::c_int) as isize)
                                                    as isize,
                                            ),
                                        (&raw mut (*sl).mb as *mut int16_t)
                                            .offset(
                                                ((i * 16 as ::core::ffi::c_int)
                                                    << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                                    as isize,
                                            ),
                                        uvlinesize,
                                    );
                                }
                                i += 1;
                            }
                        }
                        j += 1;
                    }
                }
            } else {
                let mut qp: [::core::ffi::c_int; 2] = [0; 2];
                if chroma422 != 0 {
                    qp[0 as ::core::ffi::c_int as usize] = (*sl)
                        .chroma_qp[0 as ::core::ffi::c_int as usize]
                        + 3 as ::core::ffi::c_int;
                    qp[1 as ::core::ffi::c_int as usize] = (*sl)
                        .chroma_qp[1 as ::core::ffi::c_int as usize]
                        + 3 as ::core::ffi::c_int;
                } else {
                    qp[0 as ::core::ffi::c_int as usize] = (*sl)
                        .chroma_qp[0 as ::core::ffi::c_int as usize];
                    qp[1 as ::core::ffi::c_int as usize] = (*sl)
                        .chroma_qp[1 as ::core::ffi::c_int as usize];
                }
                if (*sl)
                    .non_zero_count_cache[scan8[(49 as ::core::ffi::c_int
                    + 0 as ::core::ffi::c_int) as usize] as usize] != 0
                {
                    (*h)
                        .h264dsp
                        .chroma_dc_dequant_idct
                        .expect(
                            "non-null function pointer",
                        )(
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                    * 1 as ::core::ffi::c_int)
                                    << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                    as isize,
                            ),
                        (*(*(*h).ps.pps)
                            .dequant4_coeff[(if mb_type & 7 as ::core::ffi::c_int != 0 {
                                1 as ::core::ffi::c_int
                            } else {
                                4 as ::core::ffi::c_int
                            }) as usize]
                            .offset(
                                qp[0 as ::core::ffi::c_int as usize] as isize,
                            ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    );
                }
                if (*sl)
                    .non_zero_count_cache[scan8[(49 as ::core::ffi::c_int
                    + 1 as ::core::ffi::c_int) as usize] as usize] != 0
                {
                    (*h)
                        .h264dsp
                        .chroma_dc_dequant_idct
                        .expect(
                            "non-null function pointer",
                        )(
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int
                                    * 2 as ::core::ffi::c_int)
                                    << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                                    as isize,
                            ),
                        (*(*(*h).ps.pps)
                            .dequant4_coeff[(if mb_type & 7 as ::core::ffi::c_int != 0 {
                                2 as ::core::ffi::c_int
                            } else {
                                5 as ::core::ffi::c_int
                            }) as usize]
                            .offset(
                                qp[1 as ::core::ffi::c_int as usize] as isize,
                            ))[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
                    );
                }
                (*h)
                    .h264dsp
                    .idct_add8
                    .expect(
                        "non-null function pointer",
                    )(
                    &raw mut dest as *mut *mut uint8_t,
                    block_offset,
                    &raw mut (*sl).mb as *mut int16_t,
                    uvlinesize,
                    &raw mut (*sl).non_zero_count_cache as *mut uint8_t as *const uint8_t,
                );
            }
        }
    };
}
#[inline]
unsafe extern "C" fn get_lowest_part_y(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut refs: *mut [int16_t; 48],
    mut n: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut y_offset: ::core::ffi::c_int,
    mut list0: ::core::ffi::c_int,
    mut list1: ::core::ffi::c_int,
    mut nrefs: *mut ::core::ffi::c_int,
) {
    let mut my: ::core::ffi::c_int = 0;
    y_offset += 16 as ::core::ffi::c_int * ((*sl).mb_y >> (*sl).mb_field_decoding_flag);
    if list0 != 0 {
        let mut ref_n: ::core::ffi::c_int = (*sl)
            .ref_cache[0 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as ::core::ffi::c_int;
        let mut ref_0: *mut H264Ref = (&raw mut *(&raw mut (*sl).ref_list
            as *mut [H264Ref; 48])
            .offset(0 as ::core::ffi::c_int as isize) as *mut H264Ref)
            .offset(ref_n as isize) as *mut H264Ref;
        if (*(*ref_0).parent).tf.progress != (*h).cur_pic.tf.progress
            || (*ref_0).reference & 3 as ::core::ffi::c_int != (*h).picture_structure
        {
            my = get_lowest_part_list_y(
                sl,
                n,
                height,
                y_offset,
                0 as ::core::ffi::c_int,
            );
            if ((*refs.offset(0 as ::core::ffi::c_int as isize))[ref_n as usize]
                as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            {
                *nrefs.offset(0 as ::core::ffi::c_int as isize)
                    += 1 as ::core::ffi::c_int;
            }
            (*refs.offset(0 as ::core::ffi::c_int as isize))[ref_n as usize] = (if (*refs
                .offset(0 as ::core::ffi::c_int as isize))[ref_n as usize]
                as ::core::ffi::c_int > my
            {
                (*refs.offset(0 as ::core::ffi::c_int as isize))[ref_n as usize]
                    as ::core::ffi::c_int
            } else {
                my
            }) as int16_t;
        }
    }
    if list1 != 0 {
        let mut ref_n_0: ::core::ffi::c_int = (*sl)
            .ref_cache[1 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as ::core::ffi::c_int;
        let mut ref_1: *mut H264Ref = (&raw mut *(&raw mut (*sl).ref_list
            as *mut [H264Ref; 48])
            .offset(1 as ::core::ffi::c_int as isize) as *mut H264Ref)
            .offset(ref_n_0 as isize) as *mut H264Ref;
        if (*(*ref_1).parent).tf.progress != (*h).cur_pic.tf.progress
            || (*ref_1).reference & 3 as ::core::ffi::c_int != (*h).picture_structure
        {
            my = get_lowest_part_list_y(
                sl,
                n,
                height,
                y_offset,
                1 as ::core::ffi::c_int,
            );
            if ((*refs.offset(1 as ::core::ffi::c_int as isize))[ref_n_0 as usize]
                as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
            {
                *nrefs.offset(1 as ::core::ffi::c_int as isize)
                    += 1 as ::core::ffi::c_int;
            }
            (*refs.offset(1 as ::core::ffi::c_int as isize))[ref_n_0 as usize] = (if (*refs
                .offset(1 as ::core::ffi::c_int as isize))[ref_n_0 as usize]
                as ::core::ffi::c_int > my
            {
                (*refs.offset(1 as ::core::ffi::c_int as isize))[ref_n_0 as usize]
                    as ::core::ffi::c_int
            } else {
                my
            }) as int16_t;
        }
    }
}
#[inline(always)]
unsafe extern "C" fn av_bswap32(mut x: uint32_t) -> uint32_t {
    return (x << 8 as ::core::ffi::c_int & 0xff00 as uint32_t
        | x >> 8 as ::core::ffi::c_int & 0xff as uint32_t) << 16 as ::core::ffi::c_int
        | ((x >> 16 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
            & 0xff00 as uint32_t
            | x >> 16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
                & 0xff as uint32_t);
}
unsafe extern "C" fn hl_motion_422_complex(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut qpix_put: *const [qpel_mc_func; 16],
    mut chroma_put: *const h264_chroma_mc_func,
    mut qpix_avg: *const [qpel_mc_func; 16],
    mut chroma_avg: *const h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    if 1 as ::core::ffi::c_int != 0
        && (*(*h).avctx).active_thread_type & 1 as ::core::ffi::c_int != 0
    {
        await_references(h, sl);
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            0 as ::core::ffi::c_int,
            (*h).pixel_shift,
            2 as ::core::ffi::c_int,
        );
    }
    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        mc_part_422_complex(
            h,
            sl,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
        mc_part_422_complex(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int) << (*h).pixel_shift,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_422_complex(
            h,
            sl,
            8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int) << (*h).pixel_shift,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
        mc_part_422_complex(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_422_complex(
            h,
            sl,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let sub_mb_type: ::core::ffi::c_int = (*sl).sub_mb_type[i as usize]
                as ::core::ffi::c_int;
            let n: ::core::ffi::c_int = 4 as ::core::ffi::c_int * i;
            let mut x_offset: ::core::ffi::c_int = (i & 1 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int;
            let mut y_offset: ::core::ffi::c_int = (i & 2 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int;
            if sub_mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
                mc_part_422_complex(
                    h,
                    sl,
                    n,
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                != 0
            {
                mc_part_422_complex(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int) << (*h).pixel_shift,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_422_complex(
                    h,
                    sl,
                    n + 2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int) << (*h).pixel_shift,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset + 2 as ::core::ffi::c_int,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                != 0
            {
                mc_part_422_complex(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_422_complex(
                    h,
                    sl,
                    n + 1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset + 2 as ::core::ffi::c_int,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else {
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
                while j < 4 as ::core::ffi::c_int {
                    let mut sub_x_offset: ::core::ffi::c_int = x_offset
                        + 2 as ::core::ffi::c_int * (j & 1 as ::core::ffi::c_int);
                    let mut sub_y_offset: ::core::ffi::c_int = y_offset
                        + (j & 2 as ::core::ffi::c_int);
                    mc_part_422_complex(
                        h,
                        sl,
                        n + j,
                        1 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dest_y,
                        dest_cb,
                        dest_cr,
                        sub_x_offset,
                        sub_y_offset,
                        &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        weight_op.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_weight_func,
                        weight_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_biweight_func,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    );
                    j += 1;
                }
            }
            i += 1;
        }
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            1 as ::core::ffi::c_int,
            (*h).pixel_shift,
            2 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn hl_motion_420_simple_8(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut qpix_put: *const [qpel_mc_func; 16],
    mut chroma_put: *const h264_chroma_mc_func,
    mut qpix_avg: *const [qpel_mc_func; 16],
    mut chroma_avg: *const h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    if 1 as ::core::ffi::c_int != 0
        && (*(*h).avctx).active_thread_type & 1 as ::core::ffi::c_int != 0
    {
        await_references(h, sl);
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    }
    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        mc_part_420_simple_8(
            h,
            sl,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
        mc_part_420_simple_8(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int)
                << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_420_simple_8(
            h,
            sl,
            8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int)
                << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
        mc_part_420_simple_8(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_420_simple_8(
            h,
            sl,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let sub_mb_type: ::core::ffi::c_int = (*sl).sub_mb_type[i as usize]
                as ::core::ffi::c_int;
            let n: ::core::ffi::c_int = 4 as ::core::ffi::c_int * i;
            let mut x_offset: ::core::ffi::c_int = (i & 1 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int;
            let mut y_offset: ::core::ffi::c_int = (i & 2 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int;
            if sub_mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
                mc_part_420_simple_8(
                    h,
                    sl,
                    n,
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                != 0
            {
                mc_part_420_simple_8(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int)
                        << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_420_simple_8(
                    h,
                    sl,
                    n + 2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int)
                        << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset + 2 as ::core::ffi::c_int,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                != 0
            {
                mc_part_420_simple_8(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_420_simple_8(
                    h,
                    sl,
                    n + 1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset + 2 as ::core::ffi::c_int,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else {
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
                while j < 4 as ::core::ffi::c_int {
                    let mut sub_x_offset: ::core::ffi::c_int = x_offset
                        + 2 as ::core::ffi::c_int * (j & 1 as ::core::ffi::c_int);
                    let mut sub_y_offset: ::core::ffi::c_int = y_offset
                        + (j & 2 as ::core::ffi::c_int);
                    mc_part_420_simple_8(
                        h,
                        sl,
                        n + j,
                        1 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dest_y,
                        dest_cb,
                        dest_cr,
                        sub_x_offset,
                        sub_y_offset,
                        &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        weight_op.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_weight_func,
                        weight_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_biweight_func,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    );
                    j += 1;
                }
            }
            i += 1;
        }
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            1 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn hl_motion_422_simple_8(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut qpix_put: *const [qpel_mc_func; 16],
    mut chroma_put: *const h264_chroma_mc_func,
    mut qpix_avg: *const [qpel_mc_func; 16],
    mut chroma_avg: *const h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    if 1 as ::core::ffi::c_int != 0
        && (*(*h).avctx).active_thread_type & 1 as ::core::ffi::c_int != 0
    {
        await_references(h, sl);
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
    }
    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        mc_part_422_simple_8(
            h,
            sl,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
        mc_part_422_simple_8(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int)
                << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_422_simple_8(
            h,
            sl,
            8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int)
                << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
        mc_part_422_simple_8(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_422_simple_8(
            h,
            sl,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let sub_mb_type: ::core::ffi::c_int = (*sl).sub_mb_type[i as usize]
                as ::core::ffi::c_int;
            let n: ::core::ffi::c_int = 4 as ::core::ffi::c_int * i;
            let mut x_offset: ::core::ffi::c_int = (i & 1 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int;
            let mut y_offset: ::core::ffi::c_int = (i & 2 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int;
            if sub_mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
                mc_part_422_simple_8(
                    h,
                    sl,
                    n,
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                != 0
            {
                mc_part_422_simple_8(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int)
                        << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_422_simple_8(
                    h,
                    sl,
                    n + 2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int)
                        << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset + 2 as ::core::ffi::c_int,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                != 0
            {
                mc_part_422_simple_8(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_422_simple_8(
                    h,
                    sl,
                    n + 1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset + 2 as ::core::ffi::c_int,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else {
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
                while j < 4 as ::core::ffi::c_int {
                    let mut sub_x_offset: ::core::ffi::c_int = x_offset
                        + 2 as ::core::ffi::c_int * (j & 1 as ::core::ffi::c_int);
                    let mut sub_y_offset: ::core::ffi::c_int = y_offset
                        + (j & 2 as ::core::ffi::c_int);
                    mc_part_422_simple_8(
                        h,
                        sl,
                        n + j,
                        1 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dest_y,
                        dest_cb,
                        dest_cr,
                        sub_x_offset,
                        sub_y_offset,
                        &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        weight_op.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_weight_func,
                        weight_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_biweight_func,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    );
                    j += 1;
                }
            }
            i += 1;
        }
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            1 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn hl_motion_444_simple_8(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut qpix_put: *const [qpel_mc_func; 16],
    mut chroma_put: *const h264_chroma_mc_func,
    mut qpix_avg: *const [qpel_mc_func; 16],
    mut chroma_avg: *const h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    if 1 as ::core::ffi::c_int != 0
        && (*(*h).avctx).active_thread_type & 1 as ::core::ffi::c_int != 0
    {
        await_references(h, sl);
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
        );
    }
    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        mc_part_444_simple_8(
            h,
            sl,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
        mc_part_444_simple_8(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int)
                << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_444_simple_8(
            h,
            sl,
            8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int)
                << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
        mc_part_444_simple_8(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_444_simple_8(
            h,
            sl,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let sub_mb_type: ::core::ffi::c_int = (*sl).sub_mb_type[i as usize]
                as ::core::ffi::c_int;
            let n: ::core::ffi::c_int = 4 as ::core::ffi::c_int * i;
            let mut x_offset: ::core::ffi::c_int = (i & 1 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int;
            let mut y_offset: ::core::ffi::c_int = (i & 2 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int;
            if sub_mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
                mc_part_444_simple_8(
                    h,
                    sl,
                    n,
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                != 0
            {
                mc_part_444_simple_8(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int)
                        << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_444_simple_8(
                    h,
                    sl,
                    n + 2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int)
                        << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset + 2 as ::core::ffi::c_int,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                != 0
            {
                mc_part_444_simple_8(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_444_simple_8(
                    h,
                    sl,
                    n + 1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset + 2 as ::core::ffi::c_int,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else {
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
                while j < 4 as ::core::ffi::c_int {
                    let mut sub_x_offset: ::core::ffi::c_int = x_offset
                        + 2 as ::core::ffi::c_int * (j & 1 as ::core::ffi::c_int);
                    let mut sub_y_offset: ::core::ffi::c_int = y_offset
                        + (j & 2 as ::core::ffi::c_int);
                    mc_part_444_simple_8(
                        h,
                        sl,
                        n + j,
                        1 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dest_y,
                        dest_cb,
                        dest_cr,
                        sub_x_offset,
                        sub_y_offset,
                        &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        weight_op.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_weight_func,
                        weight_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_biweight_func,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    );
                    j += 1;
                }
            }
            i += 1;
        }
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            1 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            3 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn hl_motion_420_complex(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut qpix_put: *const [qpel_mc_func; 16],
    mut chroma_put: *const h264_chroma_mc_func,
    mut qpix_avg: *const [qpel_mc_func; 16],
    mut chroma_avg: *const h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    if 1 as ::core::ffi::c_int != 0
        && (*(*h).avctx).active_thread_type & 1 as ::core::ffi::c_int != 0
    {
        await_references(h, sl);
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            0 as ::core::ffi::c_int,
            (*h).pixel_shift,
            1 as ::core::ffi::c_int,
        );
    }
    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        mc_part_420_complex(
            h,
            sl,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
        mc_part_420_complex(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int) << (*h).pixel_shift,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_420_complex(
            h,
            sl,
            8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int) << (*h).pixel_shift,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
        mc_part_420_complex(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_420_complex(
            h,
            sl,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let sub_mb_type: ::core::ffi::c_int = (*sl).sub_mb_type[i as usize]
                as ::core::ffi::c_int;
            let n: ::core::ffi::c_int = 4 as ::core::ffi::c_int * i;
            let mut x_offset: ::core::ffi::c_int = (i & 1 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int;
            let mut y_offset: ::core::ffi::c_int = (i & 2 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int;
            if sub_mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
                mc_part_420_complex(
                    h,
                    sl,
                    n,
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                != 0
            {
                mc_part_420_complex(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int) << (*h).pixel_shift,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_420_complex(
                    h,
                    sl,
                    n + 2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int) << (*h).pixel_shift,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset + 2 as ::core::ffi::c_int,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                != 0
            {
                mc_part_420_complex(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_420_complex(
                    h,
                    sl,
                    n + 1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset + 2 as ::core::ffi::c_int,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else {
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
                while j < 4 as ::core::ffi::c_int {
                    let mut sub_x_offset: ::core::ffi::c_int = x_offset
                        + 2 as ::core::ffi::c_int * (j & 1 as ::core::ffi::c_int);
                    let mut sub_y_offset: ::core::ffi::c_int = y_offset
                        + (j & 2 as ::core::ffi::c_int);
                    mc_part_420_complex(
                        h,
                        sl,
                        n + j,
                        1 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dest_y,
                        dest_cb,
                        dest_cr,
                        sub_x_offset,
                        sub_y_offset,
                        &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        weight_op.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_weight_func,
                        weight_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_biweight_func,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    );
                    j += 1;
                }
            }
            i += 1;
        }
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            1 as ::core::ffi::c_int,
            (*h).pixel_shift,
            1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn hl_motion_420_simple_16(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut qpix_put: *const [qpel_mc_func; 16],
    mut chroma_put: *const h264_chroma_mc_func,
    mut qpix_avg: *const [qpel_mc_func; 16],
    mut chroma_avg: *const h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    if 1 as ::core::ffi::c_int != 0
        && (*(*h).avctx).active_thread_type & 1 as ::core::ffi::c_int != 0
    {
        await_references(h, sl);
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    }
    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        mc_part_420_simple_16(
            h,
            sl,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
        mc_part_420_simple_16(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int)
                << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_420_simple_16(
            h,
            sl,
            8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int)
                << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
        mc_part_420_simple_16(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_420_simple_16(
            h,
            sl,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let sub_mb_type: ::core::ffi::c_int = (*sl).sub_mb_type[i as usize]
                as ::core::ffi::c_int;
            let n: ::core::ffi::c_int = 4 as ::core::ffi::c_int * i;
            let mut x_offset: ::core::ffi::c_int = (i & 1 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int;
            let mut y_offset: ::core::ffi::c_int = (i & 2 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int;
            if sub_mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
                mc_part_420_simple_16(
                    h,
                    sl,
                    n,
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                != 0
            {
                mc_part_420_simple_16(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int)
                        << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_420_simple_16(
                    h,
                    sl,
                    n + 2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int)
                        << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset + 2 as ::core::ffi::c_int,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                != 0
            {
                mc_part_420_simple_16(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_420_simple_16(
                    h,
                    sl,
                    n + 1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset + 2 as ::core::ffi::c_int,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else {
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
                while j < 4 as ::core::ffi::c_int {
                    let mut sub_x_offset: ::core::ffi::c_int = x_offset
                        + 2 as ::core::ffi::c_int * (j & 1 as ::core::ffi::c_int);
                    let mut sub_y_offset: ::core::ffi::c_int = y_offset
                        + (j & 2 as ::core::ffi::c_int);
                    mc_part_420_simple_16(
                        h,
                        sl,
                        n + j,
                        1 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dest_y,
                        dest_cb,
                        dest_cr,
                        sub_x_offset,
                        sub_y_offset,
                        &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        weight_op.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_weight_func,
                        weight_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_biweight_func,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    );
                    j += 1;
                }
            }
            i += 1;
        }
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            1 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn hl_motion_422_simple_16(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut qpix_put: *const [qpel_mc_func; 16],
    mut chroma_put: *const h264_chroma_mc_func,
    mut qpix_avg: *const [qpel_mc_func; 16],
    mut chroma_avg: *const h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    if 1 as ::core::ffi::c_int != 0
        && (*(*h).avctx).active_thread_type & 1 as ::core::ffi::c_int != 0
    {
        await_references(h, sl);
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
    }
    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        mc_part_422_simple_16(
            h,
            sl,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
        mc_part_422_simple_16(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int)
                << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_422_simple_16(
            h,
            sl,
            8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int)
                << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
        mc_part_422_simple_16(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_422_simple_16(
            h,
            sl,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let sub_mb_type: ::core::ffi::c_int = (*sl).sub_mb_type[i as usize]
                as ::core::ffi::c_int;
            let n: ::core::ffi::c_int = 4 as ::core::ffi::c_int * i;
            let mut x_offset: ::core::ffi::c_int = (i & 1 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int;
            let mut y_offset: ::core::ffi::c_int = (i & 2 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int;
            if sub_mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
                mc_part_422_simple_16(
                    h,
                    sl,
                    n,
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                != 0
            {
                mc_part_422_simple_16(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int)
                        << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_422_simple_16(
                    h,
                    sl,
                    n + 2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int)
                        << (16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int),
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset + 2 as ::core::ffi::c_int,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                != 0
            {
                mc_part_422_simple_16(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_422_simple_16(
                    h,
                    sl,
                    n + 1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset + 2 as ::core::ffi::c_int,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else {
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
                while j < 4 as ::core::ffi::c_int {
                    let mut sub_x_offset: ::core::ffi::c_int = x_offset
                        + 2 as ::core::ffi::c_int * (j & 1 as ::core::ffi::c_int);
                    let mut sub_y_offset: ::core::ffi::c_int = y_offset
                        + (j & 2 as ::core::ffi::c_int);
                    mc_part_422_simple_16(
                        h,
                        sl,
                        n + j,
                        1 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dest_y,
                        dest_cb,
                        dest_cr,
                        sub_x_offset,
                        sub_y_offset,
                        &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        weight_op.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_weight_func,
                        weight_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_biweight_func,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    );
                    j += 1;
                }
            }
            i += 1;
        }
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            1 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn hl_motion_444_complex(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut qpix_put: *const [qpel_mc_func; 16],
    mut chroma_put: *const h264_chroma_mc_func,
    mut qpix_avg: *const [qpel_mc_func; 16],
    mut chroma_avg: *const h264_chroma_mc_func,
    mut weight_op: *const h264_weight_func,
    mut weight_avg: *const h264_biweight_func,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    if 1 as ::core::ffi::c_int != 0
        && (*(*h).avctx).active_thread_type & 1 as ::core::ffi::c_int != 0
    {
        await_references(h, sl);
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            0 as ::core::ffi::c_int,
            (*h).pixel_shift,
            3 as ::core::ffi::c_int,
        );
    }
    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        mc_part_444_complex(
            h,
            sl,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(0 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
        mc_part_444_complex(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int) << (*h).pixel_shift,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_444_complex(
            h,
            sl,
            8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            (8 as ::core::ffi::c_int) << (*h).pixel_shift,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(0 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op,
            weight_avg,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
        mc_part_444_complex(
            h,
            sl,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
        mc_part_444_complex(
            h,
            sl,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            4 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_put.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const qpel_mc_func,
            *chroma_avg.offset(1 as ::core::ffi::c_int as isize) as h264_chroma_mc_func,
            weight_op.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_weight_func,
            weight_avg.offset(1 as ::core::ffi::c_int as isize)
                as *const h264_biweight_func,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
        );
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let sub_mb_type: ::core::ffi::c_int = (*sl).sub_mb_type[i as usize]
                as ::core::ffi::c_int;
            let n: ::core::ffi::c_int = 4 as ::core::ffi::c_int * i;
            let mut x_offset: ::core::ffi::c_int = (i & 1 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int;
            let mut y_offset: ::core::ffi::c_int = (i & 2 as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int;
            if sub_mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
                mc_part_444_complex(
                    h,
                    sl,
                    n,
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                != 0
            {
                mc_part_444_complex(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int) << (*h).pixel_shift,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_444_complex(
                    h,
                    sl,
                    n + 2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    (4 as ::core::ffi::c_int) << (*h).pixel_shift,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset + 2 as ::core::ffi::c_int,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(1 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(1 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                != 0
            {
                mc_part_444_complex(
                    h,
                    sl,
                    n,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
                mc_part_444_complex(
                    h,
                    sl,
                    n + 1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int * (*sl).mb_linesize as ::core::ffi::c_int,
                    dest_y,
                    dest_cb,
                    dest_cr,
                    x_offset + 2 as ::core::ffi::c_int,
                    y_offset,
                    &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const qpel_mc_func,
                    *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                        as h264_chroma_mc_func,
                    weight_op.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_weight_func,
                    weight_avg.offset(2 as ::core::ffi::c_int as isize)
                        as *const h264_biweight_func,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                );
            } else {
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
                while j < 4 as ::core::ffi::c_int {
                    let mut sub_x_offset: ::core::ffi::c_int = x_offset
                        + 2 as ::core::ffi::c_int * (j & 1 as ::core::ffi::c_int);
                    let mut sub_y_offset: ::core::ffi::c_int = y_offset
                        + (j & 2 as ::core::ffi::c_int);
                    mc_part_444_complex(
                        h,
                        sl,
                        n + j,
                        1 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dest_y,
                        dest_cb,
                        dest_cr,
                        sub_x_offset,
                        sub_y_offset,
                        &raw const *qpix_put.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_put.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        &raw const *qpix_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const qpel_mc_func,
                        *chroma_avg.offset(2 as ::core::ffi::c_int as isize)
                            as h264_chroma_mc_func,
                        weight_op.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_weight_func,
                        weight_avg.offset(2 as ::core::ffi::c_int as isize)
                            as *const h264_biweight_func,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    );
                    j += 1;
                }
            }
            i += 1;
        }
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int != 0
    {
        prefetch_motion(
            h,
            sl,
            1 as ::core::ffi::c_int,
            (*h).pixel_shift,
            3 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn await_references(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    let mut refs: [[int16_t; 48]; 2] = [[0; 48]; 2];
    let mut nrefs: [::core::ffi::c_int; 2] = [0 as ::core::ffi::c_int; 2];
    let mut ref_0: ::core::ffi::c_int = 0;
    let mut list: ::core::ffi::c_int = 0;
    memset(
        &raw mut refs as *mut [int16_t; 48] as *mut ::core::ffi::c_void,
        -(1 as ::core::ffi::c_int),
        ::core::mem::size_of::<[[int16_t; 48]; 2]>() as size_t,
    );
    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        get_lowest_part_y(
            h,
            sl,
            &raw mut refs as *mut [int16_t; 48],
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
            &raw mut nrefs as *mut ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
        get_lowest_part_y(
            h,
            sl,
            &raw mut refs as *mut [int16_t; 48],
            0 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
            &raw mut nrefs as *mut ::core::ffi::c_int,
        );
        get_lowest_part_y(
            h,
            sl,
            &raw mut refs as *mut [int16_t; 48],
            8 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
            &raw mut nrefs as *mut ::core::ffi::c_int,
        );
    } else if mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
        get_lowest_part_y(
            h,
            sl,
            &raw mut refs as *mut [int16_t; 48],
            0 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 0 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
            &raw mut nrefs as *mut ::core::ffi::c_int,
        );
        get_lowest_part_y(
            h,
            sl,
            &raw mut refs as *mut [int16_t; 48],
            4 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
            mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int
                        + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
            &raw mut nrefs as *mut ::core::ffi::c_int,
        );
    } else {
        let mut i: ::core::ffi::c_int = 0;
        i = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int {
            let sub_mb_type: ::core::ffi::c_int = (*sl).sub_mb_type[i as usize]
                as ::core::ffi::c_int;
            let n: ::core::ffi::c_int = 4 as ::core::ffi::c_int * i;
            let mut y_offset: ::core::ffi::c_int = (i & 2 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int;
            if sub_mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
                get_lowest_part_y(
                    h,
                    sl,
                    &raw mut refs as *mut [int16_t; 48],
                    n,
                    8 as ::core::ffi::c_int,
                    y_offset,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    &raw mut nrefs as *mut ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                != 0
            {
                get_lowest_part_y(
                    h,
                    sl,
                    &raw mut refs as *mut [int16_t; 48],
                    n,
                    4 as ::core::ffi::c_int,
                    y_offset,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    &raw mut nrefs as *mut ::core::ffi::c_int,
                );
                get_lowest_part_y(
                    h,
                    sl,
                    &raw mut refs as *mut [int16_t; 48],
                    n + 2 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    y_offset + 4 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    &raw mut nrefs as *mut ::core::ffi::c_int,
                );
            } else if sub_mb_type & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                != 0
            {
                get_lowest_part_y(
                    h,
                    sl,
                    &raw mut refs as *mut [int16_t; 48],
                    n,
                    8 as ::core::ffi::c_int,
                    y_offset,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    &raw mut nrefs as *mut ::core::ffi::c_int,
                );
                get_lowest_part_y(
                    h,
                    sl,
                    &raw mut refs as *mut [int16_t; 48],
                    n + 1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    y_offset,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                    sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                    &raw mut nrefs as *mut ::core::ffi::c_int,
                );
            } else {
                let mut j: ::core::ffi::c_int = 0;
                j = 0 as ::core::ffi::c_int;
                while j < 4 as ::core::ffi::c_int {
                    let mut sub_y_offset: ::core::ffi::c_int = y_offset
                        + 2 as ::core::ffi::c_int * (j & 2 as ::core::ffi::c_int);
                    get_lowest_part_y(
                        h,
                        sl,
                        &raw mut refs as *mut [int16_t; 48],
                        n + j,
                        4 as ::core::ffi::c_int,
                        sub_y_offset,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int,
                        sub_mb_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                                << 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int,
                        &raw mut nrefs as *mut ::core::ffi::c_int,
                    );
                    j += 1;
                }
            }
            i += 1;
        }
    }
    list = (*sl).list_count.wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    while list >= 0 as ::core::ffi::c_int {
        ref_0 = 0 as ::core::ffi::c_int;
        while ref_0 < 48 as ::core::ffi::c_int && nrefs[list as usize] != 0 {
            let mut row: ::core::ffi::c_int = refs[list as usize][ref_0 as usize]
                as ::core::ffi::c_int;
            if row >= 0 as ::core::ffi::c_int {
                let mut ref_pic: *mut H264Ref = (&raw mut *(&raw mut (*sl).ref_list
                    as *mut [H264Ref; 48])
                    .offset(list as isize) as *mut H264Ref)
                    .offset(ref_0 as isize) as *mut H264Ref;
                let mut ref_field: ::core::ffi::c_int = (*ref_pic).reference
                    - 1 as ::core::ffi::c_int;
                let mut ref_field_picture: ::core::ffi::c_int = (*(*ref_pic).parent)
                    .field_picture;
                let mut pic_height: ::core::ffi::c_int = 16 as ::core::ffi::c_int
                    * (*h).mb_height >> ref_field_picture;
                row <<= (*sl).mb_mbaff;
                nrefs[list as usize] -= 1;
                if !((*h).picture_structure != 3 as ::core::ffi::c_int)
                    && ref_field_picture != 0
                {
                    ff_thread_await_progress(
                        &raw const (*(*ref_pic).parent).tf,
                        if (row >> 1 as ::core::ffi::c_int)
                            - (row & 1 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                            > pic_height - 1 as ::core::ffi::c_int
                        {
                            pic_height - 1 as ::core::ffi::c_int
                        } else {
                            (row >> 1 as ::core::ffi::c_int)
                                - (row & 1 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
                        },
                        1 as ::core::ffi::c_int,
                    );
                    ff_thread_await_progress(
                        &raw const (*(*ref_pic).parent).tf,
                        if row >> 1 as ::core::ffi::c_int
                            > pic_height - 1 as ::core::ffi::c_int
                        {
                            pic_height - 1 as ::core::ffi::c_int
                        } else {
                            row >> 1 as ::core::ffi::c_int
                        },
                        0 as ::core::ffi::c_int,
                    );
                } else if (*h).picture_structure != 3 as ::core::ffi::c_int
                    && ref_field_picture == 0
                {
                    ff_thread_await_progress(
                        &raw const (*(*ref_pic).parent).tf,
                        if row * 2 as ::core::ffi::c_int + ref_field
                            > pic_height - 1 as ::core::ffi::c_int
                        {
                            pic_height - 1 as ::core::ffi::c_int
                        } else {
                            row * 2 as ::core::ffi::c_int + ref_field
                        },
                        0 as ::core::ffi::c_int,
                    );
                } else if (*h).picture_structure != 3 as ::core::ffi::c_int {
                    ff_thread_await_progress(
                        &raw const (*(*ref_pic).parent).tf,
                        if row > pic_height - 1 as ::core::ffi::c_int {
                            pic_height - 1 as ::core::ffi::c_int
                        } else {
                            row
                        },
                        ref_field,
                    );
                } else {
                    ff_thread_await_progress(
                        &raw const (*(*ref_pic).parent).tf,
                        if row > pic_height - 1 as ::core::ffi::c_int {
                            pic_height - 1 as ::core::ffi::c_int
                        } else {
                            row
                        },
                        0 as ::core::ffi::c_int,
                    );
                }
            }
            ref_0 += 1;
        }
        list -= 1;
    }
}
#[inline(always)]
unsafe extern "C" fn mc_dir_part(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut pic: *mut H264Ref,
    mut n: ::core::ffi::c_int,
    mut square: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut list: ::core::ffi::c_int,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut src_x_offset: ::core::ffi::c_int,
    mut src_y_offset: ::core::ffi::c_int,
    mut qpix_op: *const qpel_mc_func,
    mut chroma_op: h264_chroma_mc_func,
    mut pixel_shift: ::core::ffi::c_int,
    mut chroma_idc: ::core::ffi::c_int,
) {
    let mx: ::core::ffi::c_int = (*sl)
        .mv_cache[list
        as usize][scan8[n as usize] as usize][0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int + src_x_offset * 8 as ::core::ffi::c_int;
    let mut my: ::core::ffi::c_int = (*sl)
        .mv_cache[list
        as usize][scan8[n as usize] as usize][1 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int + src_y_offset * 8 as ::core::ffi::c_int;
    let luma_xy: ::core::ffi::c_int = (mx & 3 as ::core::ffi::c_int)
        + ((my & 3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int);
    let mut offset: ptrdiff_t = (mx as ptrdiff_t >> 2 as ::core::ffi::c_int)
        * ((1 as ptrdiff_t) << pixel_shift)
        + (my as ptrdiff_t >> 2 as ::core::ffi::c_int) * (*sl).mb_linesize;
    let mut src_y: *mut uint8_t = (*pic)
        .data[0 as ::core::ffi::c_int as usize]
        .offset(offset as isize);
    let mut src_cb: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut src_cr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut extra_width: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut extra_height: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut emu: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let full_mx: ::core::ffi::c_int = mx >> 2 as ::core::ffi::c_int;
    let full_my: ::core::ffi::c_int = my >> 2 as ::core::ffi::c_int;
    let pic_width: ::core::ffi::c_int = 16 as ::core::ffi::c_int * (*h).mb_width;
    let pic_height: ::core::ffi::c_int = 16 as ::core::ffi::c_int * (*h).mb_height
        >> (*sl).mb_field_decoding_flag;
    let mut ysh: ::core::ffi::c_int = 0;
    if mx & 7 as ::core::ffi::c_int != 0 {
        extra_width -= 3 as ::core::ffi::c_int;
    }
    if my & 7 as ::core::ffi::c_int != 0 {
        extra_height -= 3 as ::core::ffi::c_int;
    }
    if full_mx < 0 as ::core::ffi::c_int - extra_width
        || full_my < 0 as ::core::ffi::c_int - extra_height
        || full_mx + 16 as ::core::ffi::c_int > pic_width + extra_width
        || full_my + 16 as ::core::ffi::c_int > pic_height + extra_height
    {
        (*h)
            .vdsp
            .emulated_edge_mc
            .expect(
                "non-null function pointer",
            )(
            (*sl).edge_emu_buffer,
            src_y
                .offset(-(((2 as ::core::ffi::c_int) << pixel_shift) as isize))
                .offset(-((2 as ptrdiff_t * (*sl).mb_linesize) as isize)),
            (*sl).mb_linesize,
            (*sl).mb_linesize,
            16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
            16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
            full_mx - 2 as ::core::ffi::c_int,
            full_my - 2 as ::core::ffi::c_int,
            pic_width,
            pic_height,
        );
        src_y = (*sl)
            .edge_emu_buffer
            .offset(((2 as ::core::ffi::c_int) << pixel_shift) as isize)
            .offset((2 as ptrdiff_t * (*sl).mb_linesize) as isize);
        emu = 1 as ::core::ffi::c_int;
    }
    (*qpix_op.offset(luma_xy as isize))
        .expect("non-null function pointer")(dest_y, src_y, (*sl).mb_linesize);
    if square == 0 {
        (*qpix_op.offset(luma_xy as isize))
            .expect(
                "non-null function pointer",
            )(
            dest_y.offset(delta as isize),
            src_y.offset(delta as isize),
            (*sl).mb_linesize,
        );
    }
    if 0 as ::core::ffi::c_int != 0
        && (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int != 0
    {
        return;
    }
    if chroma_idc == 3 as ::core::ffi::c_int {
        src_cb = (*pic).data[1 as ::core::ffi::c_int as usize].offset(offset as isize);
        if emu != 0 {
            (*h)
                .vdsp
                .emulated_edge_mc
                .expect(
                    "non-null function pointer",
                )(
                (*sl).edge_emu_buffer,
                src_cb
                    .offset(-(((2 as ::core::ffi::c_int) << pixel_shift) as isize))
                    .offset(-((2 as ptrdiff_t * (*sl).mb_linesize) as isize)),
                (*sl).mb_linesize,
                (*sl).mb_linesize,
                16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                full_mx - 2 as ::core::ffi::c_int,
                full_my - 2 as ::core::ffi::c_int,
                pic_width,
                pic_height,
            );
            src_cb = (*sl)
                .edge_emu_buffer
                .offset(((2 as ::core::ffi::c_int) << pixel_shift) as isize)
                .offset((2 as ptrdiff_t * (*sl).mb_linesize) as isize);
        }
        (*qpix_op.offset(luma_xy as isize))
            .expect("non-null function pointer")(dest_cb, src_cb, (*sl).mb_linesize);
        if square == 0 {
            (*qpix_op.offset(luma_xy as isize))
                .expect(
                    "non-null function pointer",
                )(
                dest_cb.offset(delta as isize),
                src_cb.offset(delta as isize),
                (*sl).mb_linesize,
            );
        }
        src_cr = (*pic).data[2 as ::core::ffi::c_int as usize].offset(offset as isize);
        if emu != 0 {
            (*h)
                .vdsp
                .emulated_edge_mc
                .expect(
                    "non-null function pointer",
                )(
                (*sl).edge_emu_buffer,
                src_cr
                    .offset(-(((2 as ::core::ffi::c_int) << pixel_shift) as isize))
                    .offset(-((2 as ptrdiff_t * (*sl).mb_linesize) as isize)),
                (*sl).mb_linesize,
                (*sl).mb_linesize,
                16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                16 as ::core::ffi::c_int + 5 as ::core::ffi::c_int,
                full_mx - 2 as ::core::ffi::c_int,
                full_my - 2 as ::core::ffi::c_int,
                pic_width,
                pic_height,
            );
            src_cr = (*sl)
                .edge_emu_buffer
                .offset(((2 as ::core::ffi::c_int) << pixel_shift) as isize)
                .offset((2 as ptrdiff_t * (*sl).mb_linesize) as isize);
        }
        (*qpix_op.offset(luma_xy as isize))
            .expect("non-null function pointer")(dest_cr, src_cr, (*sl).mb_linesize);
        if square == 0 {
            (*qpix_op.offset(luma_xy as isize))
                .expect(
                    "non-null function pointer",
                )(
                dest_cr.offset(delta as isize),
                src_cr.offset(delta as isize),
                (*sl).mb_linesize,
            );
        }
        return;
    }
    ysh = 3 as ::core::ffi::c_int
        - (chroma_idc == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if chroma_idc == 1 as ::core::ffi::c_int && (*sl).mb_field_decoding_flag != 0 {
        my
            += 2 as ::core::ffi::c_int
                * (((*sl).mb_y & 1 as ::core::ffi::c_int)
                    - ((*pic).reference - 1 as ::core::ffi::c_int));
        emu
            |= ((my >> 3 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                || (my >> 3 as ::core::ffi::c_int) + 8 as ::core::ffi::c_int
                    >= pic_height >> 1 as ::core::ffi::c_int) as ::core::ffi::c_int;
    }
    src_cb = (*pic)
        .data[1 as ::core::ffi::c_int as usize]
        .offset(
            ((mx >> 3 as ::core::ffi::c_int)
                * ((1 as ::core::ffi::c_int) << pixel_shift)) as isize,
        )
        .offset(((my as ptrdiff_t >> ysh) * (*sl).mb_uvlinesize) as isize);
    src_cr = (*pic)
        .data[2 as ::core::ffi::c_int as usize]
        .offset(
            ((mx >> 3 as ::core::ffi::c_int)
                * ((1 as ::core::ffi::c_int) << pixel_shift)) as isize,
        )
        .offset(((my as ptrdiff_t >> ysh) * (*sl).mb_uvlinesize) as isize);
    if emu != 0 {
        (*h)
            .vdsp
            .emulated_edge_mc
            .expect(
                "non-null function pointer",
            )(
            (*sl).edge_emu_buffer,
            src_cb,
            (*sl).mb_uvlinesize,
            (*sl).mb_uvlinesize,
            9 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * chroma_idc + 1 as ::core::ffi::c_int,
            mx >> 3 as ::core::ffi::c_int,
            my >> ysh,
            pic_width >> 1 as ::core::ffi::c_int,
            pic_height >> (chroma_idc == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
        src_cb = (*sl).edge_emu_buffer;
    }
    chroma_op
        .expect(
            "non-null function pointer",
        )(
        dest_cb,
        src_cb,
        (*sl).mb_uvlinesize,
        height >> (chroma_idc == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
        mx & 7 as ::core::ffi::c_int,
        ((my as ::core::ffi::c_uint)
            << (chroma_idc == 2 as ::core::ffi::c_int) as ::core::ffi::c_int
            & 7 as ::core::ffi::c_uint) as ::core::ffi::c_int,
    );
    if emu != 0 {
        (*h)
            .vdsp
            .emulated_edge_mc
            .expect(
                "non-null function pointer",
            )(
            (*sl).edge_emu_buffer,
            src_cr,
            (*sl).mb_uvlinesize,
            (*sl).mb_uvlinesize,
            9 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int * chroma_idc + 1 as ::core::ffi::c_int,
            mx >> 3 as ::core::ffi::c_int,
            my >> ysh,
            pic_width >> 1 as ::core::ffi::c_int,
            pic_height >> (chroma_idc == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
        src_cr = (*sl).edge_emu_buffer;
    }
    chroma_op
        .expect(
            "non-null function pointer",
        )(
        dest_cr,
        src_cr,
        (*sl).mb_uvlinesize,
        height >> (chroma_idc == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
        mx & 7 as ::core::ffi::c_int,
        ((my as ::core::ffi::c_uint)
            << (chroma_idc == 2 as ::core::ffi::c_int) as ::core::ffi::c_int
            & 7 as ::core::ffi::c_uint) as ::core::ffi::c_int,
    );
}
#[inline(never)]
unsafe extern "C" fn hl_decode_mb_444_simple_8(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mb_x: ::core::ffi::c_int = (*sl).mb_x;
    let mb_y: ::core::ffi::c_int = (*sl).mb_y;
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    let mut dest: [*mut uint8_t; 3] = [::core::ptr::null_mut::<uint8_t>(); 3];
    let mut linesize: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut block_offset: *const ::core::ffi::c_int = (&raw const (*h).block_offset
        as *const ::core::ffi::c_int)
        .offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_int;
    let transform_bypass: ::core::ffi::c_int = (1 as ::core::ffi::c_int == 0
        && ((*sl).qscale == 0 as ::core::ffi::c_int
            && (*(*h).ps.sps).transform_bypass != 0)) as ::core::ffi::c_int;
    let plane_count: ::core::ffi::c_int = if 1 as ::core::ffi::c_int != 0
        || 0 as ::core::ffi::c_int == 0
        || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int == 0
    {
        3 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    p = 0 as ::core::ffi::c_int;
    while p < plane_count {
        dest[p as usize] = (*(*h).cur_pic.f)
            .data[p as usize]
            .offset(
                (((mb_x << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int))
                    + mb_y * (*sl).linesize as ::core::ffi::c_int)
                    * 16 as ::core::ffi::c_int) as isize,
            );
        (*h)
            .vdsp
            .prefetch
            .expect(
                "non-null function pointer",
            )(
            dest[p as usize]
                .offset(
                    (((*sl).mb_x as ptrdiff_t & 3 as ptrdiff_t) * 4 as ptrdiff_t
                        * (*sl).linesize) as isize,
                )
                .offset(
                    ((64 as ::core::ffi::c_int)
                        << (8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int)) as isize,
                ),
            (*sl).linesize,
            4 as ::core::ffi::c_int,
        );
        p += 1;
    }
    *(*h).list_counts.offset(mb_xy as isize) = (*sl).list_count as uint8_t;
    if 1 as ::core::ffi::c_int == 0 && (*sl).mb_field_decoding_flag != 0 {
        (*sl).mb_uvlinesize = ((*sl).linesize as ::core::ffi::c_int
            * 2 as ::core::ffi::c_int) as ptrdiff_t;
        (*sl).mb_linesize = (*sl).mb_uvlinesize;
        linesize = (*sl).mb_linesize as ::core::ffi::c_int;
        block_offset = (&raw const (*h).block_offset as *const ::core::ffi::c_int)
            .offset(48 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_int;
        if mb_y & 1 as ::core::ffi::c_int != 0 {
            p = 0 as ::core::ffi::c_int;
            while p < 3 as ::core::ffi::c_int {
                dest[p as usize] = dest[p as usize]
                    .offset(
                        -(((*sl).linesize as ::core::ffi::c_int
                            * 15 as ::core::ffi::c_int) as isize),
                    );
                p += 1;
            }
        }
        if (*h).mb_aff_frame != 0 {
            let mut list: ::core::ffi::c_int = 0;
            list = 0 as ::core::ffi::c_int;
            while (list as ::core::ffi::c_uint) < (*sl).list_count {
                if !(mb_type
                    & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int * list == 0)
                {
                    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        != 0
                    {
                        let mut ref_0: *mut int8_t = (&raw mut *(&raw mut (*sl).ref_cache
                            as *mut [int8_t; 40])
                            .offset(list as isize) as *mut int8_t)
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize) as isize,
                            ) as *mut int8_t;
                        fill_rectangle(
                            ref_0 as *mut ::core::ffi::c_void,
                            4 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            (16 as ::core::ffi::c_int + *ref_0 as ::core::ffi::c_int
                                ^ (*sl).mb_y & 1 as ::core::ffi::c_int) as uint32_t,
                            1 as ::core::ffi::c_int,
                        );
                    } else {
                        i = 0 as ::core::ffi::c_int;
                        while i < 16 as ::core::ffi::c_int {
                            let mut ref_1: ::core::ffi::c_int = (*sl)
                                .ref_cache[list as usize][scan8[i as usize] as usize]
                                as ::core::ffi::c_int;
                            if ref_1 >= 0 as ::core::ffi::c_int {
                                fill_rectangle(
                                    (&raw mut *(&raw mut (*sl).ref_cache as *mut [int8_t; 40])
                                        .offset(list as isize) as *mut int8_t)
                                        .offset(
                                            *(&raw const scan8 as *const uint8_t).offset(i as isize)
                                                as isize,
                                        ) as *mut int8_t as *mut ::core::ffi::c_void,
                                    2 as ::core::ffi::c_int,
                                    2 as ::core::ffi::c_int,
                                    8 as ::core::ffi::c_int,
                                    (16 as ::core::ffi::c_int + ref_1
                                        ^ (*sl).mb_y & 1 as ::core::ffi::c_int) as uint32_t,
                                    1 as ::core::ffi::c_int,
                                );
                            }
                            i += 4 as ::core::ffi::c_int;
                        }
                    }
                }
                list += 1;
            }
        }
    } else {
        (*sl).mb_uvlinesize = (*sl).linesize;
        (*sl).mb_linesize = (*sl).mb_uvlinesize;
        linesize = (*sl).mb_linesize as ::core::ffi::c_int;
    }
    if 1 as ::core::ffi::c_int == 0
        && mb_type & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        if 8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int != 0 {
            let bit_depth: ::core::ffi::c_int = (*(*h).ps.sps).bit_depth_luma;
            let mut gb: GetBitContext = GetBitContext {
                buffer: ::core::ptr::null::<uint8_t>(),
                index: 0,
                size_in_bits: 0,
                size_in_bits_plus8: 0,
            };
            init_get_bits(
                &raw mut gb,
                (*sl).intra_pcm_ptr,
                768 as ::core::ffi::c_int * bit_depth,
            );
            p = 0 as ::core::ffi::c_int;
            while p < plane_count {
                i = 0 as ::core::ffi::c_int;
                while i < 16 as ::core::ffi::c_int {
                    let mut tmp: *mut uint16_t = dest[p as usize]
                        .offset((i * linesize) as isize) as *mut uint16_t;
                    j = 0 as ::core::ffi::c_int;
                    while j < 16 as ::core::ffi::c_int {
                        *tmp.offset(j as isize) = get_bits(&raw mut gb, bit_depth)
                            as uint16_t;
                        j += 1;
                    }
                    i += 1;
                }
                p += 1;
            }
        } else {
            p = 0 as ::core::ffi::c_int;
            while p < plane_count {
                i = 0 as ::core::ffi::c_int;
                while i < 16 as ::core::ffi::c_int {
                    memcpy(
                        dest[p as usize].offset((i * linesize) as isize)
                            as *mut ::core::ffi::c_void,
                        (*sl)
                            .intra_pcm_ptr
                            .offset((p * 256 as ::core::ffi::c_int) as isize)
                            .offset((i * 16 as ::core::ffi::c_int) as isize)
                            as *const ::core::ffi::c_void,
                        16 as size_t,
                    );
                    i += 1;
                }
                p += 1;
            }
        }
    } else {
        if mb_type & 7 as ::core::ffi::c_int != 0 {
            if (*sl).deblocking_filter != 0 {
                xchg_mb_border(
                    h,
                    sl,
                    dest[0 as ::core::ffi::c_int as usize],
                    dest[1 as ::core::ffi::c_int as usize],
                    dest[2 as ::core::ffi::c_int as usize],
                    linesize,
                    linesize,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                );
            }
            p = 0 as ::core::ffi::c_int;
            while p < plane_count {
                hl_decode_mb_predict_luma(
                    h,
                    sl,
                    mb_type,
                    1 as ::core::ffi::c_int,
                    transform_bypass,
                    8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                    block_offset,
                    linesize,
                    dest[p as usize],
                    p,
                );
                p += 1;
            }
            if (*sl).deblocking_filter != 0 {
                xchg_mb_border(
                    h,
                    sl,
                    dest[0 as ::core::ffi::c_int as usize],
                    dest[1 as ::core::ffi::c_int as usize],
                    dest[2 as ::core::ffi::c_int as usize],
                    linesize,
                    linesize,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                );
            }
        } else {
            hl_motion_444_simple_8(
                h,
                sl,
                dest[0 as ::core::ffi::c_int as usize],
                dest[1 as ::core::ffi::c_int as usize],
                dest[2 as ::core::ffi::c_int as usize],
                &raw const (*h).h264qpel.put_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.put_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264qpel.avg_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.avg_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264dsp.weight_pixels_tab as *const h264_weight_func,
                &raw const (*h).h264dsp.biweight_pixels_tab as *const h264_biweight_func,
            );
        }
        p = 0 as ::core::ffi::c_int;
        while p < plane_count {
            hl_decode_mb_idct_luma(
                h,
                sl,
                mb_type,
                1 as ::core::ffi::c_int,
                transform_bypass,
                8 as ::core::ffi::c_int >> 4 as ::core::ffi::c_int,
                block_offset,
                linesize,
                dest[p as usize],
                p,
            );
            p += 1;
        }
    };
}
#[inline(never)]
unsafe extern "C" fn hl_decode_mb_444_complex(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mb_x: ::core::ffi::c_int = (*sl).mb_x;
    let mb_y: ::core::ffi::c_int = (*sl).mb_y;
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    let mut dest: [*mut uint8_t; 3] = [::core::ptr::null_mut::<uint8_t>(); 3];
    let mut linesize: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut block_offset: *const ::core::ffi::c_int = (&raw const (*h).block_offset
        as *const ::core::ffi::c_int)
        .offset(0 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_int;
    let transform_bypass: ::core::ffi::c_int = (0 as ::core::ffi::c_int == 0
        && ((*sl).qscale == 0 as ::core::ffi::c_int
            && (*(*h).ps.sps).transform_bypass != 0)) as ::core::ffi::c_int;
    let plane_count: ::core::ffi::c_int = if 0 as ::core::ffi::c_int != 0
        || 0 as ::core::ffi::c_int == 0
        || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int == 0
    {
        3 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    p = 0 as ::core::ffi::c_int;
    while p < plane_count {
        dest[p as usize] = (*(*h).cur_pic.f)
            .data[p as usize]
            .offset(
                (((mb_x << (*h).pixel_shift)
                    + mb_y * (*sl).linesize as ::core::ffi::c_int)
                    * 16 as ::core::ffi::c_int) as isize,
            );
        (*h)
            .vdsp
            .prefetch
            .expect(
                "non-null function pointer",
            )(
            dest[p as usize]
                .offset(
                    (((*sl).mb_x as ptrdiff_t & 3 as ptrdiff_t) * 4 as ptrdiff_t
                        * (*sl).linesize) as isize,
                )
                .offset(((64 as ::core::ffi::c_int) << (*h).pixel_shift) as isize),
            (*sl).linesize,
            4 as ::core::ffi::c_int,
        );
        p += 1;
    }
    *(*h).list_counts.offset(mb_xy as isize) = (*sl).list_count as uint8_t;
    if 0 as ::core::ffi::c_int == 0 && (*sl).mb_field_decoding_flag != 0 {
        (*sl).mb_uvlinesize = ((*sl).linesize as ::core::ffi::c_int
            * 2 as ::core::ffi::c_int) as ptrdiff_t;
        (*sl).mb_linesize = (*sl).mb_uvlinesize;
        linesize = (*sl).mb_linesize as ::core::ffi::c_int;
        block_offset = (&raw const (*h).block_offset as *const ::core::ffi::c_int)
            .offset(48 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_int;
        if mb_y & 1 as ::core::ffi::c_int != 0 {
            p = 0 as ::core::ffi::c_int;
            while p < 3 as ::core::ffi::c_int {
                dest[p as usize] = dest[p as usize]
                    .offset(
                        -(((*sl).linesize as ::core::ffi::c_int
                            * 15 as ::core::ffi::c_int) as isize),
                    );
                p += 1;
            }
        }
        if (*h).mb_aff_frame != 0 {
            let mut list: ::core::ffi::c_int = 0;
            list = 0 as ::core::ffi::c_int;
            while (list as ::core::ffi::c_uint) < (*sl).list_count {
                if !(mb_type
                    & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int * list == 0)
                {
                    if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                        != 0
                    {
                        let mut ref_0: *mut int8_t = (&raw mut *(&raw mut (*sl).ref_cache
                            as *mut [int8_t; 40])
                            .offset(list as isize) as *mut int8_t)
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize) as isize,
                            ) as *mut int8_t;
                        fill_rectangle(
                            ref_0 as *mut ::core::ffi::c_void,
                            4 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            (16 as ::core::ffi::c_int + *ref_0 as ::core::ffi::c_int
                                ^ (*sl).mb_y & 1 as ::core::ffi::c_int) as uint32_t,
                            1 as ::core::ffi::c_int,
                        );
                    } else {
                        i = 0 as ::core::ffi::c_int;
                        while i < 16 as ::core::ffi::c_int {
                            let mut ref_1: ::core::ffi::c_int = (*sl)
                                .ref_cache[list as usize][scan8[i as usize] as usize]
                                as ::core::ffi::c_int;
                            if ref_1 >= 0 as ::core::ffi::c_int {
                                fill_rectangle(
                                    (&raw mut *(&raw mut (*sl).ref_cache as *mut [int8_t; 40])
                                        .offset(list as isize) as *mut int8_t)
                                        .offset(
                                            *(&raw const scan8 as *const uint8_t).offset(i as isize)
                                                as isize,
                                        ) as *mut int8_t as *mut ::core::ffi::c_void,
                                    2 as ::core::ffi::c_int,
                                    2 as ::core::ffi::c_int,
                                    8 as ::core::ffi::c_int,
                                    (16 as ::core::ffi::c_int + ref_1
                                        ^ (*sl).mb_y & 1 as ::core::ffi::c_int) as uint32_t,
                                    1 as ::core::ffi::c_int,
                                );
                            }
                            i += 4 as ::core::ffi::c_int;
                        }
                    }
                }
                list += 1;
            }
        }
    } else {
        (*sl).mb_uvlinesize = (*sl).linesize;
        (*sl).mb_linesize = (*sl).mb_uvlinesize;
        linesize = (*sl).mb_linesize as ::core::ffi::c_int;
    }
    if 0 as ::core::ffi::c_int == 0
        && mb_type & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        if (*h).pixel_shift != 0 {
            let bit_depth: ::core::ffi::c_int = (*(*h).ps.sps).bit_depth_luma;
            let mut gb: GetBitContext = GetBitContext {
                buffer: ::core::ptr::null::<uint8_t>(),
                index: 0,
                size_in_bits: 0,
                size_in_bits_plus8: 0,
            };
            init_get_bits(
                &raw mut gb,
                (*sl).intra_pcm_ptr,
                768 as ::core::ffi::c_int * bit_depth,
            );
            p = 0 as ::core::ffi::c_int;
            while p < plane_count {
                i = 0 as ::core::ffi::c_int;
                while i < 16 as ::core::ffi::c_int {
                    let mut tmp: *mut uint16_t = dest[p as usize]
                        .offset((i * linesize) as isize) as *mut uint16_t;
                    j = 0 as ::core::ffi::c_int;
                    while j < 16 as ::core::ffi::c_int {
                        *tmp.offset(j as isize) = get_bits(&raw mut gb, bit_depth)
                            as uint16_t;
                        j += 1;
                    }
                    i += 1;
                }
                p += 1;
            }
        } else {
            p = 0 as ::core::ffi::c_int;
            while p < plane_count {
                i = 0 as ::core::ffi::c_int;
                while i < 16 as ::core::ffi::c_int {
                    memcpy(
                        dest[p as usize].offset((i * linesize) as isize)
                            as *mut ::core::ffi::c_void,
                        (*sl)
                            .intra_pcm_ptr
                            .offset((p * 256 as ::core::ffi::c_int) as isize)
                            .offset((i * 16 as ::core::ffi::c_int) as isize)
                            as *const ::core::ffi::c_void,
                        16 as size_t,
                    );
                    i += 1;
                }
                p += 1;
            }
        }
    } else {
        if mb_type & 7 as ::core::ffi::c_int != 0 {
            if (*sl).deblocking_filter != 0 {
                xchg_mb_border(
                    h,
                    sl,
                    dest[0 as ::core::ffi::c_int as usize],
                    dest[1 as ::core::ffi::c_int as usize],
                    dest[2 as ::core::ffi::c_int as usize],
                    linesize,
                    linesize,
                    1 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    (*h).pixel_shift,
                );
            }
            p = 0 as ::core::ffi::c_int;
            while p < plane_count {
                hl_decode_mb_predict_luma(
                    h,
                    sl,
                    mb_type,
                    0 as ::core::ffi::c_int,
                    transform_bypass,
                    (*h).pixel_shift,
                    block_offset,
                    linesize,
                    dest[p as usize],
                    p,
                );
                p += 1;
            }
            if (*sl).deblocking_filter != 0 {
                xchg_mb_border(
                    h,
                    sl,
                    dest[0 as ::core::ffi::c_int as usize],
                    dest[1 as ::core::ffi::c_int as usize],
                    dest[2 as ::core::ffi::c_int as usize],
                    linesize,
                    linesize,
                    0 as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    (*h).pixel_shift,
                );
            }
        } else {
            hl_motion_444_complex(
                h,
                sl,
                dest[0 as ::core::ffi::c_int as usize],
                dest[1 as ::core::ffi::c_int as usize],
                dest[2 as ::core::ffi::c_int as usize],
                &raw const (*h).h264qpel.put_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.put_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264qpel.avg_h264_qpel_pixels_tab
                    as *const [qpel_mc_func; 16],
                &raw const (*h).h264chroma.avg_h264_chroma_pixels_tab
                    as *const h264_chroma_mc_func,
                &raw const (*h).h264dsp.weight_pixels_tab as *const h264_weight_func,
                &raw const (*h).h264dsp.biweight_pixels_tab as *const h264_biweight_func,
            );
        }
        p = 0 as ::core::ffi::c_int;
        while p < plane_count {
            hl_decode_mb_idct_luma(
                h,
                sl,
                mb_type,
                0 as ::core::ffi::c_int,
                transform_bypass,
                (*h).pixel_shift,
                block_offset,
                linesize,
                dest[p as usize],
                p,
            );
            p += 1;
        }
    };
}
#[inline(always)]
unsafe extern "C" fn mc_part_std(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut square: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut x_offset: ::core::ffi::c_int,
    mut y_offset: ::core::ffi::c_int,
    mut qpix_put: *const qpel_mc_func,
    mut chroma_put: h264_chroma_mc_func,
    mut qpix_avg: *const qpel_mc_func,
    mut chroma_avg: h264_chroma_mc_func,
    mut list0: ::core::ffi::c_int,
    mut list1: ::core::ffi::c_int,
    mut pixel_shift: ::core::ffi::c_int,
    mut chroma_idc: ::core::ffi::c_int,
) {
    let mut qpix_op: *const qpel_mc_func = qpix_put;
    let mut chroma_op: h264_chroma_mc_func = chroma_put;
    dest_y = dest_y
        .offset(
            (((2 as ptrdiff_t * x_offset as ptrdiff_t) << pixel_shift)
                + 2 as ptrdiff_t * y_offset as ptrdiff_t * (*sl).mb_linesize) as isize,
        );
    if chroma_idc == 3 as ::core::ffi::c_int {
        dest_cb = dest_cb
            .offset(
                (((2 as ptrdiff_t * x_offset as ptrdiff_t) << pixel_shift)
                    + 2 as ptrdiff_t * y_offset as ptrdiff_t * (*sl).mb_linesize)
                    as isize,
            );
        dest_cr = dest_cr
            .offset(
                (((2 as ptrdiff_t * x_offset as ptrdiff_t) << pixel_shift)
                    + 2 as ptrdiff_t * y_offset as ptrdiff_t * (*sl).mb_linesize)
                    as isize,
            );
    } else if chroma_idc == 2 as ::core::ffi::c_int {
        dest_cb = dest_cb
            .offset(
                (((x_offset as ptrdiff_t) << pixel_shift)
                    + 2 as ptrdiff_t * y_offset as ptrdiff_t * (*sl).mb_uvlinesize)
                    as isize,
            );
        dest_cr = dest_cr
            .offset(
                (((x_offset as ptrdiff_t) << pixel_shift)
                    + 2 as ptrdiff_t * y_offset as ptrdiff_t * (*sl).mb_uvlinesize)
                    as isize,
            );
    } else {
        dest_cb = dest_cb
            .offset(
                (((x_offset as ptrdiff_t) << pixel_shift)
                    + y_offset as ptrdiff_t * (*sl).mb_uvlinesize) as isize,
            );
        dest_cr = dest_cr
            .offset(
                (((x_offset as ptrdiff_t) << pixel_shift)
                    + y_offset as ptrdiff_t * (*sl).mb_uvlinesize) as isize,
            );
    }
    x_offset += 8 as ::core::ffi::c_int * (*sl).mb_x;
    y_offset += 8 as ::core::ffi::c_int * ((*sl).mb_y >> (*sl).mb_field_decoding_flag);
    if list0 != 0 {
        let mut ref_0: *mut H264Ref = (&raw mut *(&raw mut (*sl).ref_list
            as *mut [H264Ref; 48])
            .offset(0 as ::core::ffi::c_int as isize) as *mut H264Ref)
            .offset(
                *(&raw mut *(&raw mut (*sl).ref_cache as *mut [int8_t; 40])
                    .offset(0 as ::core::ffi::c_int as isize) as *mut int8_t)
                    .offset(
                        *(&raw const scan8 as *const uint8_t).offset(n as isize) as isize,
                    ) as isize,
            ) as *mut H264Ref;
        mc_dir_part(
            h,
            sl,
            ref_0,
            n,
            square,
            height,
            delta,
            0 as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_op,
            chroma_op,
            pixel_shift,
            chroma_idc,
        );
        qpix_op = qpix_avg;
        chroma_op = chroma_avg;
    }
    if list1 != 0 {
        let mut ref_1: *mut H264Ref = (&raw mut *(&raw mut (*sl).ref_list
            as *mut [H264Ref; 48])
            .offset(1 as ::core::ffi::c_int as isize) as *mut H264Ref)
            .offset(
                *(&raw mut *(&raw mut (*sl).ref_cache as *mut [int8_t; 40])
                    .offset(1 as ::core::ffi::c_int as isize) as *mut int8_t)
                    .offset(
                        *(&raw const scan8 as *const uint8_t).offset(n as isize) as isize,
                    ) as isize,
            ) as *mut H264Ref;
        mc_dir_part(
            h,
            sl,
            ref_1,
            n,
            square,
            height,
            delta,
            1 as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_op,
            chroma_op,
            pixel_shift,
            chroma_idc,
        );
    }
}
#[inline]
unsafe extern "C" fn get_bits(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut tmp: ::core::ffi::c_uint = 0;
    let mut re_index: ::core::ffi::c_uint = (*s).index as ::core::ffi::c_uint;
    let mut re_cache: ::core::ffi::c_uint = 0;
    let mut re_size_plus8: ::core::ffi::c_uint = (*s).size_in_bits_plus8
        as ::core::ffi::c_uint;
    re_cache = (av_bswap32(
        (*((*s).buffer.offset((re_index >> 3 as ::core::ffi::c_int) as isize)
            as *const unaligned_32))
            .l,
    ) << (re_index & 7 as ::core::ffi::c_uint)
        >> 32 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    tmp = (re_cache >> 32 as ::core::ffi::c_int - n) as ::core::ffi::c_uint;
    re_index = if re_size_plus8 > re_index.wrapping_add(n as ::core::ffi::c_uint) {
        re_index.wrapping_add(n as ::core::ffi::c_uint)
    } else {
        re_size_plus8
    };
    (*s).index = re_index as ::core::ffi::c_int;
    return tmp;
}
#[inline(always)]
unsafe extern "C" fn mc_part_weighted(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut square: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
    mut delta: ::core::ffi::c_int,
    mut dest_y: *mut uint8_t,
    mut dest_cb: *mut uint8_t,
    mut dest_cr: *mut uint8_t,
    mut x_offset: ::core::ffi::c_int,
    mut y_offset: ::core::ffi::c_int,
    mut qpix_put: *const qpel_mc_func,
    mut chroma_put: h264_chroma_mc_func,
    mut luma_weight_op: h264_weight_func,
    mut chroma_weight_op: h264_weight_func,
    mut luma_weight_avg: h264_biweight_func,
    mut chroma_weight_avg: h264_biweight_func,
    mut list0: ::core::ffi::c_int,
    mut list1: ::core::ffi::c_int,
    mut pixel_shift: ::core::ffi::c_int,
    mut chroma_idc: ::core::ffi::c_int,
) {
    let mut chroma_height: ::core::ffi::c_int = 0;
    dest_y = dest_y
        .offset(
            (((2 as ptrdiff_t * x_offset as ptrdiff_t) << pixel_shift)
                + 2 as ptrdiff_t * y_offset as ptrdiff_t * (*sl).mb_linesize) as isize,
        );
    if chroma_idc == 3 as ::core::ffi::c_int {
        chroma_height = height;
        chroma_weight_avg = luma_weight_avg;
        chroma_weight_op = luma_weight_op;
        dest_cb = dest_cb
            .offset(
                (((2 as ptrdiff_t * x_offset as ptrdiff_t) << pixel_shift)
                    + 2 as ptrdiff_t * y_offset as ptrdiff_t * (*sl).mb_linesize)
                    as isize,
            );
        dest_cr = dest_cr
            .offset(
                (((2 as ptrdiff_t * x_offset as ptrdiff_t) << pixel_shift)
                    + 2 as ptrdiff_t * y_offset as ptrdiff_t * (*sl).mb_linesize)
                    as isize,
            );
    } else if chroma_idc == 2 as ::core::ffi::c_int {
        chroma_height = height;
        dest_cb = dest_cb
            .offset(
                (((x_offset as ptrdiff_t) << pixel_shift)
                    + 2 as ptrdiff_t * y_offset as ptrdiff_t * (*sl).mb_uvlinesize)
                    as isize,
            );
        dest_cr = dest_cr
            .offset(
                (((x_offset as ptrdiff_t) << pixel_shift)
                    + 2 as ptrdiff_t * y_offset as ptrdiff_t * (*sl).mb_uvlinesize)
                    as isize,
            );
    } else {
        chroma_height = height >> 1 as ::core::ffi::c_int;
        dest_cb = dest_cb
            .offset(
                (((x_offset as ptrdiff_t) << pixel_shift)
                    + y_offset as ptrdiff_t * (*sl).mb_uvlinesize) as isize,
            );
        dest_cr = dest_cr
            .offset(
                (((x_offset as ptrdiff_t) << pixel_shift)
                    + y_offset as ptrdiff_t * (*sl).mb_uvlinesize) as isize,
            );
    }
    x_offset += 8 as ::core::ffi::c_int * (*sl).mb_x;
    y_offset += 8 as ::core::ffi::c_int * ((*sl).mb_y >> (*sl).mb_field_decoding_flag);
    if list0 != 0 && list1 != 0 {
        let mut tmp_cb: *mut uint8_t = (*sl).bipred_scratchpad;
        let mut tmp_cr: *mut uint8_t = (*sl)
            .bipred_scratchpad
            .offset(
                ((8 as ::core::ffi::c_int)
                    << pixel_shift
                        + (chroma_idc == 3 as ::core::ffi::c_int) as ::core::ffi::c_int)
                    as isize,
            );
        let mut tmp_y: *mut uint8_t = (*sl)
            .bipred_scratchpad
            .offset((16 as ptrdiff_t * (*sl).mb_uvlinesize) as isize);
        let mut refn0: ::core::ffi::c_int = (*sl)
            .ref_cache[0 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as ::core::ffi::c_int;
        let mut refn1: ::core::ffi::c_int = (*sl)
            .ref_cache[1 as ::core::ffi::c_int as usize][scan8[n as usize] as usize]
            as ::core::ffi::c_int;
        mc_dir_part(
            h,
            sl,
            (&raw mut *(&raw mut (*sl).ref_list as *mut [H264Ref; 48])
                .offset(0 as ::core::ffi::c_int as isize) as *mut H264Ref)
                .offset(refn0 as isize) as *mut H264Ref,
            n,
            square,
            height,
            delta,
            0 as ::core::ffi::c_int,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            pixel_shift,
            chroma_idc,
        );
        mc_dir_part(
            h,
            sl,
            (&raw mut *(&raw mut (*sl).ref_list as *mut [H264Ref; 48])
                .offset(1 as ::core::ffi::c_int as isize) as *mut H264Ref)
                .offset(refn1 as isize) as *mut H264Ref,
            n,
            square,
            height,
            delta,
            1 as ::core::ffi::c_int,
            tmp_y,
            tmp_cb,
            tmp_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            pixel_shift,
            chroma_idc,
        );
        if (*sl).pwt.use_weight == 2 as ::core::ffi::c_int {
            let mut weight0: ::core::ffi::c_int = (*sl)
                .pwt
                .implicit_weight[refn0
                as usize][refn1
                as usize][((*sl).mb_y & 1 as ::core::ffi::c_int) as usize];
            let mut weight1: ::core::ffi::c_int = 64 as ::core::ffi::c_int - weight0;
            luma_weight_avg
                .expect(
                    "non-null function pointer",
                )(
                dest_y,
                tmp_y,
                (*sl).mb_linesize,
                height,
                5 as ::core::ffi::c_int,
                weight0,
                weight1,
                0 as ::core::ffi::c_int,
            );
            if 0 as ::core::ffi::c_int == 0
                || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                    == 0
            {
                chroma_weight_avg
                    .expect(
                        "non-null function pointer",
                    )(
                    dest_cb,
                    tmp_cb,
                    (*sl).mb_uvlinesize,
                    chroma_height,
                    5 as ::core::ffi::c_int,
                    weight0,
                    weight1,
                    0 as ::core::ffi::c_int,
                );
                chroma_weight_avg
                    .expect(
                        "non-null function pointer",
                    )(
                    dest_cr,
                    tmp_cr,
                    (*sl).mb_uvlinesize,
                    chroma_height,
                    5 as ::core::ffi::c_int,
                    weight0,
                    weight1,
                    0 as ::core::ffi::c_int,
                );
            }
        } else {
            luma_weight_avg
                .expect(
                    "non-null function pointer",
                )(
                dest_y,
                tmp_y,
                (*sl).mb_linesize,
                height,
                (*sl).pwt.luma_log2_weight_denom,
                (*sl)
                    .pwt
                    .luma_weight[refn0
                    as usize][0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize],
                (*sl)
                    .pwt
                    .luma_weight[refn1
                    as usize][1 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize],
                (*sl)
                    .pwt
                    .luma_weight[refn0
                    as usize][0 as ::core::ffi::c_int
                    as usize][1 as ::core::ffi::c_int as usize]
                    + (*sl)
                        .pwt
                        .luma_weight[refn1
                        as usize][1 as ::core::ffi::c_int
                        as usize][1 as ::core::ffi::c_int as usize],
            );
            if 0 as ::core::ffi::c_int == 0
                || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                    == 0
            {
                chroma_weight_avg
                    .expect(
                        "non-null function pointer",
                    )(
                    dest_cb,
                    tmp_cb,
                    (*sl).mb_uvlinesize,
                    chroma_height,
                    (*sl).pwt.chroma_log2_weight_denom,
                    (*sl)
                        .pwt
                        .chroma_weight[refn0
                        as usize][0 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize],
                    (*sl)
                        .pwt
                        .chroma_weight[refn1
                        as usize][1 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize],
                    (*sl)
                        .pwt
                        .chroma_weight[refn0
                        as usize][0 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int
                        as usize][1 as ::core::ffi::c_int as usize]
                        + (*sl)
                            .pwt
                            .chroma_weight[refn1
                            as usize][1 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int
                            as usize][1 as ::core::ffi::c_int as usize],
                );
                chroma_weight_avg
                    .expect(
                        "non-null function pointer",
                    )(
                    dest_cr,
                    tmp_cr,
                    (*sl).mb_uvlinesize,
                    chroma_height,
                    (*sl).pwt.chroma_log2_weight_denom,
                    (*sl)
                        .pwt
                        .chroma_weight[refn0
                        as usize][0 as ::core::ffi::c_int
                        as usize][1 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize],
                    (*sl)
                        .pwt
                        .chroma_weight[refn1
                        as usize][1 as ::core::ffi::c_int
                        as usize][1 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize],
                    (*sl)
                        .pwt
                        .chroma_weight[refn0
                        as usize][0 as ::core::ffi::c_int
                        as usize][1 as ::core::ffi::c_int
                        as usize][1 as ::core::ffi::c_int as usize]
                        + (*sl)
                            .pwt
                            .chroma_weight[refn1
                            as usize][1 as ::core::ffi::c_int
                            as usize][1 as ::core::ffi::c_int
                            as usize][1 as ::core::ffi::c_int as usize],
                );
            }
        }
    } else {
        let mut list: ::core::ffi::c_int = if list1 != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let mut refn: ::core::ffi::c_int = (*sl)
            .ref_cache[list as usize][scan8[n as usize] as usize] as ::core::ffi::c_int;
        let mut ref_0: *mut H264Ref = (&raw mut *(&raw mut (*sl).ref_list
            as *mut [H264Ref; 48])
            .offset(list as isize) as *mut H264Ref)
            .offset(refn as isize) as *mut H264Ref;
        mc_dir_part(
            h,
            sl,
            ref_0,
            n,
            square,
            height,
            delta,
            list,
            dest_y,
            dest_cb,
            dest_cr,
            x_offset,
            y_offset,
            qpix_put,
            chroma_put,
            pixel_shift,
            chroma_idc,
        );
        luma_weight_op
            .expect(
                "non-null function pointer",
            )(
            dest_y,
            (*sl).mb_linesize,
            height,
            (*sl).pwt.luma_log2_weight_denom,
            (*sl)
                .pwt
                .luma_weight[refn
                as usize][list as usize][0 as ::core::ffi::c_int as usize],
            (*sl)
                .pwt
                .luma_weight[refn
                as usize][list as usize][1 as ::core::ffi::c_int as usize],
        );
        if 0 as ::core::ffi::c_int == 0
            || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int == 0
        {
            if (*sl).pwt.use_weight_chroma != 0 {
                chroma_weight_op
                    .expect(
                        "non-null function pointer",
                    )(
                    dest_cb,
                    (*sl).mb_uvlinesize,
                    chroma_height,
                    (*sl).pwt.chroma_log2_weight_denom,
                    (*sl)
                        .pwt
                        .chroma_weight[refn
                        as usize][list
                        as usize][0 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize],
                    (*sl)
                        .pwt
                        .chroma_weight[refn
                        as usize][list
                        as usize][0 as ::core::ffi::c_int
                        as usize][1 as ::core::ffi::c_int as usize],
                );
                chroma_weight_op
                    .expect(
                        "non-null function pointer",
                    )(
                    dest_cr,
                    (*sl).mb_uvlinesize,
                    chroma_height,
                    (*sl).pwt.chroma_log2_weight_denom,
                    (*sl)
                        .pwt
                        .chroma_weight[refn
                        as usize][list
                        as usize][1 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize],
                    (*sl)
                        .pwt
                        .chroma_weight[refn
                        as usize][list
                        as usize][1 as ::core::ffi::c_int
                        as usize][1 as ::core::ffi::c_int as usize],
                );
            }
        }
    };
}
#[inline(always)]
unsafe extern "C" fn prefetch_motion(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut list: ::core::ffi::c_int,
    mut pixel_shift: ::core::ffi::c_int,
    mut chroma_idc: ::core::ffi::c_int,
) {
    let refn: ::core::ffi::c_int = (*sl)
        .ref_cache[list as usize][scan8[0 as ::core::ffi::c_int as usize] as usize]
        as ::core::ffi::c_int;
    if refn >= 0 as ::core::ffi::c_int {
        let mx: ::core::ffi::c_int = ((*sl)
            .mv_cache[list
            as usize][scan8[0 as ::core::ffi::c_int as usize]
            as usize][0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) + 16 as ::core::ffi::c_int * (*sl).mb_x
            + 8 as ::core::ffi::c_int;
        let my: ::core::ffi::c_int = ((*sl)
            .mv_cache[list
            as usize][scan8[0 as ::core::ffi::c_int as usize]
            as usize][1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            >> 2 as ::core::ffi::c_int) + 16 as ::core::ffi::c_int * (*sl).mb_y;
        let mut src: *mut *mut uint8_t = &raw mut (*(&raw mut *(&raw mut (*sl).ref_list
            as *mut [H264Ref; 48])
            .offset(list as isize) as *mut H264Ref)
            .offset(refn as isize))
            .data as *mut *mut uint8_t;
        let mut off: ::core::ffi::c_int = mx * ((1 as ::core::ffi::c_int) << pixel_shift)
            + (my + ((*sl).mb_x & 3 as ::core::ffi::c_int) * 4 as ::core::ffi::c_int)
                * (*sl).mb_linesize as ::core::ffi::c_int
            + ((64 as ::core::ffi::c_int) << pixel_shift);
        (*h)
            .vdsp
            .prefetch
            .expect(
                "non-null function pointer",
            )(
            (*src.offset(0 as ::core::ffi::c_int as isize)).offset(off as isize),
            (*sl).linesize,
            4 as ::core::ffi::c_int,
        );
        if chroma_idc == 3 as ::core::ffi::c_int {
            (*h)
                .vdsp
                .prefetch
                .expect(
                    "non-null function pointer",
                )(
                (*src.offset(1 as ::core::ffi::c_int as isize)).offset(off as isize),
                (*sl).linesize,
                4 as ::core::ffi::c_int,
            );
            (*h)
                .vdsp
                .prefetch
                .expect(
                    "non-null function pointer",
                )(
                (*src.offset(2 as ::core::ffi::c_int as isize)).offset(off as isize),
                (*sl).linesize,
                4 as ::core::ffi::c_int,
            );
        } else {
            off = (((mx as ptrdiff_t >> 1 as ::core::ffi::c_int) + 64 as ptrdiff_t)
                * ((1 as ptrdiff_t) << pixel_shift)
                + ((my as ptrdiff_t >> 1 as ::core::ffi::c_int)
                    + ((*sl).mb_x as ptrdiff_t & 7 as ptrdiff_t)) * (*sl).uvlinesize)
                as ::core::ffi::c_int;
            (*h)
                .vdsp
                .prefetch
                .expect(
                    "non-null function pointer",
                )(
                (*src.offset(1 as ::core::ffi::c_int as isize)).offset(off as isize),
                (*src.offset(2 as ::core::ffi::c_int as isize))
                    .offset_from(*src.offset(1 as ::core::ffi::c_int as isize))
                    as ptrdiff_t,
                2 as ::core::ffi::c_int,
            );
        }
    }
}
#[inline(always)]
unsafe extern "C" fn xchg_mb_border(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut src_y: *mut uint8_t,
    mut src_cb: *mut uint8_t,
    mut src_cr: *mut uint8_t,
    mut linesize: ::core::ffi::c_int,
    mut uvlinesize: ::core::ffi::c_int,
    mut xchg: ::core::ffi::c_int,
    mut chroma444: ::core::ffi::c_int,
    mut simple: ::core::ffi::c_int,
    mut pixel_shift: ::core::ffi::c_int,
) {
    let mut deblock_topleft: ::core::ffi::c_int = 0;
    let mut deblock_top: ::core::ffi::c_int = 0;
    let mut top_idx: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut top_border_m1: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut top_border: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if simple == 0 && (*h).mb_aff_frame != 0 {
        if (*sl).mb_y & 1 as ::core::ffi::c_int != 0 {
            if (*sl).mb_mbaff == 0 {
                return;
            }
        } else {
            top_idx = if (*sl).mb_mbaff != 0 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
        }
    }
    if (*sl).deblocking_filter == 2 as ::core::ffi::c_int {
        deblock_topleft = (*(*h)
            .slice_table
            .offset(
                ((*sl).mb_xy - 1 as ::core::ffi::c_int
                    - ((*h).mb_stride << (*sl).mb_field_decoding_flag)) as isize,
            ) as ::core::ffi::c_int == (*sl).slice_num) as ::core::ffi::c_int;
        deblock_top = (*sl).top_type;
    } else {
        deblock_topleft = ((*sl).mb_x > 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        deblock_top = ((*sl).mb_y
            > ((*sl).mb_field_decoding_flag != 0) as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
    src_y = src_y.offset(-((linesize + 1 as ::core::ffi::c_int + pixel_shift) as isize));
    src_cb = src_cb
        .offset(-((uvlinesize + 1 as ::core::ffi::c_int + pixel_shift) as isize));
    src_cr = src_cr
        .offset(-((uvlinesize + 1 as ::core::ffi::c_int + pixel_shift) as isize));
    top_border_m1 = &raw mut *(*(&raw mut (*sl).top_borders as *mut *mut [uint8_t; 96])
        .offset(top_idx as isize))
        .offset(((*sl).mb_x - 1 as ::core::ffi::c_int) as isize) as *mut uint8_t;
    top_border = &raw mut *(*(&raw mut (*sl).top_borders as *mut *mut [uint8_t; 96])
        .offset(top_idx as isize))
        .offset((*sl).mb_x as isize) as *mut uint8_t;
    if deblock_top != 0 {
        if deblock_topleft != 0 {
            if pixel_shift != 0 {
                let mut SWAP_tmp: av_alias64 = *(top_border_m1
                    .offset(((8 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                *(top_border_m1
                    .offset(((8 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_y
                    .offset(-(((7 as ::core::ffi::c_int) << pixel_shift) as isize))
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                *(src_y
                    .offset(-(((7 as ::core::ffi::c_int) << pixel_shift) as isize))
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp;
                let mut SWAP_tmp_0: av_alias64 = *(top_border_m1
                    .offset(((8 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                *(top_border_m1
                    .offset(((8 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_y
                    .offset(-(((7 as ::core::ffi::c_int) << pixel_shift) as isize))
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                *(src_y
                    .offset(-(((7 as ::core::ffi::c_int) << pixel_shift) as isize))
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_0;
            } else {
                let mut SWAP_tmp_1: av_alias64 = *(top_border_m1
                    .offset(((8 as ::core::ffi::c_int) << pixel_shift) as isize)
                    as *mut av_alias64);
                *(top_border_m1
                    .offset(((8 as ::core::ffi::c_int) << pixel_shift) as isize)
                    as *mut av_alias64) = *(src_y
                    .offset(-(((7 as ::core::ffi::c_int) << pixel_shift) as isize))
                    as *mut av_alias64);
                *(src_y.offset(-(((7 as ::core::ffi::c_int) << pixel_shift) as isize))
                    as *mut av_alias64) = SWAP_tmp_1;
            }
        }
        if pixel_shift != 0 {
            if xchg != 0 {
                let mut SWAP_tmp_2: av_alias64 = *(top_border
                    .offset(((0 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                *(top_border
                    .offset(((0 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_y
                    .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                *(src_y
                    .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_2;
                let mut SWAP_tmp_3: av_alias64 = *(top_border
                    .offset(((0 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                *(top_border
                    .offset(((0 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_y
                    .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                *(src_y
                    .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_3;
            } else {
                (*(src_y.offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                    as *mut av_alias64))
                    .u64_0 = (*(top_border
                    .offset(((0 as ::core::ffi::c_int) << pixel_shift) as isize)
                    as *const av_alias64))
                    .u64_0;
                (*((src_y.offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                    .u64_0 = (*((top_border
                    .offset(((0 as ::core::ffi::c_int) << pixel_shift) as isize)
                    as *mut ::core::ffi::c_char)
                    .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                    .u64_0;
            }
        } else if xchg != 0 {
            let mut SWAP_tmp_4: av_alias64 = *(top_border
                .offset(((0 as ::core::ffi::c_int) << pixel_shift) as isize)
                as *mut av_alias64);
            *(top_border.offset(((0 as ::core::ffi::c_int) << pixel_shift) as isize)
                as *mut av_alias64) = *(src_y
                .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                as *mut av_alias64);
            *(src_y.offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                as *mut av_alias64) = SWAP_tmp_4;
        } else {
            (*(src_y.offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                as *mut av_alias64))
                .u64_0 = (*(top_border
                .offset(((0 as ::core::ffi::c_int) << pixel_shift) as isize)
                as *const av_alias64))
                .u64_0;
        }
        if pixel_shift != 0 {
            let mut SWAP_tmp_5: av_alias64 = *(top_border
                .offset(((8 as ::core::ffi::c_int) << pixel_shift) as isize)
                .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
            *(top_border
                .offset(((8 as ::core::ffi::c_int) << pixel_shift) as isize)
                .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_y
                .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
            *(src_y
                .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_5;
            let mut SWAP_tmp_6: av_alias64 = *(top_border
                .offset(((8 as ::core::ffi::c_int) << pixel_shift) as isize)
                .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
            *(top_border
                .offset(((8 as ::core::ffi::c_int) << pixel_shift) as isize)
                .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_y
                .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
            *(src_y
                .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_6;
        } else {
            let mut SWAP_tmp_7: av_alias64 = *(top_border
                .offset(((8 as ::core::ffi::c_int) << pixel_shift) as isize)
                as *mut av_alias64);
            *(top_border.offset(((8 as ::core::ffi::c_int) << pixel_shift) as isize)
                as *mut av_alias64) = *(src_y
                .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                as *mut av_alias64);
            *(src_y.offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                as *mut av_alias64) = SWAP_tmp_7;
        }
        if ((*sl).mb_x + 1 as ::core::ffi::c_int) < (*h).mb_width {
            if pixel_shift != 0 {
                let mut SWAP_tmp_8: av_alias64 = *((&raw mut *(*(&raw mut (*sl)
                    .top_borders as *mut *mut [uint8_t; 96])
                    .offset(top_idx as isize))
                    .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                    as *mut uint8_t)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                *((&raw mut *(*(&raw mut (*sl).top_borders as *mut *mut [uint8_t; 96])
                    .offset(top_idx as isize))
                    .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                    as *mut uint8_t)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_y
                    .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                *(src_y
                    .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_8;
                let mut SWAP_tmp_9: av_alias64 = *((&raw mut *(*(&raw mut (*sl)
                    .top_borders as *mut *mut [uint8_t; 96])
                    .offset(top_idx as isize))
                    .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                    as *mut uint8_t)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                *((&raw mut *(*(&raw mut (*sl).top_borders as *mut *mut [uint8_t; 96])
                    .offset(top_idx as isize))
                    .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                    as *mut uint8_t)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_y
                    .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                *(src_y
                    .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                    .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_9;
            } else {
                let mut SWAP_tmp_10: av_alias64 = *(&raw mut *(*(&raw mut (*sl)
                    .top_borders as *mut *mut [uint8_t; 96])
                    .offset(top_idx as isize))
                    .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                    as *mut uint8_t as *mut av_alias64);
                *(&raw mut *(*(&raw mut (*sl).top_borders as *mut *mut [uint8_t; 96])
                    .offset(top_idx as isize))
                    .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                    as *mut uint8_t as *mut av_alias64) = *(src_y
                    .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                    as *mut av_alias64);
                *(src_y.offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                    as *mut av_alias64) = SWAP_tmp_10;
            }
        }
        if simple != 0 || 0 as ::core::ffi::c_int == 0
            || (*h).flags & (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int == 0
        {
            if chroma444 != 0 {
                if deblock_topleft != 0 {
                    if pixel_shift != 0 {
                        let mut SWAP_tmp_11: av_alias64 = *(top_border_m1
                            .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(top_border_m1
                            .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cb
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cb
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_11;
                        let mut SWAP_tmp_12: av_alias64 = *(top_border_m1
                            .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(top_border_m1
                            .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cb
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cb
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_12;
                    } else {
                        let mut SWAP_tmp_13: av_alias64 = *(top_border_m1
                            .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64);
                        *(top_border_m1
                            .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64) = *(src_cb
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            ) as *mut av_alias64);
                        *(src_cb
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            ) as *mut av_alias64) = SWAP_tmp_13;
                    }
                    if pixel_shift != 0 {
                        let mut SWAP_tmp_14: av_alias64 = *(top_border_m1
                            .offset(((40 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(top_border_m1
                            .offset(((40 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cr
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cr
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_14;
                        let mut SWAP_tmp_15: av_alias64 = *(top_border_m1
                            .offset(((40 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(top_border_m1
                            .offset(((40 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cr
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cr
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_15;
                    } else {
                        let mut SWAP_tmp_16: av_alias64 = *(top_border_m1
                            .offset(((40 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64);
                        *(top_border_m1
                            .offset(((40 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64) = *(src_cr
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            ) as *mut av_alias64);
                        *(src_cr
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            ) as *mut av_alias64) = SWAP_tmp_16;
                    }
                }
                if pixel_shift != 0 {
                    if xchg != 0 {
                        let mut SWAP_tmp_17: av_alias64 = *(top_border
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(top_border
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cb
                            .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cb
                            .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_17;
                        let mut SWAP_tmp_18: av_alias64 = *(top_border
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(top_border
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cb
                            .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cb
                            .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_18;
                    } else {
                        (*(src_cb
                            .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64))
                            .u64_0 = (*(top_border
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *const av_alias64))
                            .u64_0;
                        (*((src_cb
                            .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut ::core::ffi::c_char)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64))
                            .u64_0 = (*((top_border
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut ::core::ffi::c_char)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *const av_alias64))
                            .u64_0;
                    }
                } else if xchg != 0 {
                    let mut SWAP_tmp_19: av_alias64 = *(top_border
                        .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64);
                    *(top_border
                        .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64) = *(src_cb
                        .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64);
                    *(src_cb.offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64) = SWAP_tmp_19;
                } else {
                    (*(src_cb.offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64))
                        .u64_0 = (*(top_border
                        .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *const av_alias64))
                        .u64_0;
                }
                if pixel_shift != 0 {
                    let mut SWAP_tmp_20: av_alias64 = *(top_border
                        .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(top_border
                        .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_cb
                        .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(src_cb
                        .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_20;
                    let mut SWAP_tmp_21: av_alias64 = *(top_border
                        .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(top_border
                        .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_cb
                        .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(src_cb
                        .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_21;
                } else {
                    let mut SWAP_tmp_22: av_alias64 = *(top_border
                        .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64);
                    *(top_border
                        .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64) = *(src_cb
                        .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64);
                    *(src_cb.offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64) = SWAP_tmp_22;
                }
                if pixel_shift != 0 {
                    if xchg != 0 {
                        let mut SWAP_tmp_23: av_alias64 = *(top_border
                            .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(top_border
                            .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cr
                            .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cr
                            .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_23;
                        let mut SWAP_tmp_24: av_alias64 = *(top_border
                            .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(top_border
                            .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cr
                            .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cr
                            .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_24;
                    } else {
                        (*(src_cr
                            .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64))
                            .u64_0 = (*(top_border
                            .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *const av_alias64))
                            .u64_0;
                        (*((src_cr
                            .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut ::core::ffi::c_char)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64))
                            .u64_0 = (*((top_border
                            .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut ::core::ffi::c_char)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *const av_alias64))
                            .u64_0;
                    }
                } else if xchg != 0 {
                    let mut SWAP_tmp_25: av_alias64 = *(top_border
                        .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64);
                    *(top_border
                        .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64) = *(src_cr
                        .offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64);
                    *(src_cr.offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64) = SWAP_tmp_25;
                } else {
                    (*(src_cr.offset(((1 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64))
                        .u64_0 = (*(top_border
                        .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *const av_alias64))
                        .u64_0;
                }
                if pixel_shift != 0 {
                    let mut SWAP_tmp_26: av_alias64 = *(top_border
                        .offset(((40 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(top_border
                        .offset(((40 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_cr
                        .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(src_cr
                        .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_26;
                    let mut SWAP_tmp_27: av_alias64 = *(top_border
                        .offset(((40 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(top_border
                        .offset(((40 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_cr
                        .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(src_cr
                        .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_27;
                } else {
                    let mut SWAP_tmp_28: av_alias64 = *(top_border
                        .offset(((40 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64);
                    *(top_border
                        .offset(((40 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64) = *(src_cr
                        .offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64);
                    *(src_cr.offset(((9 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64) = SWAP_tmp_28;
                }
                if ((*sl).mb_x + 1 as ::core::ffi::c_int) < (*h).mb_width {
                    if pixel_shift != 0 {
                        let mut SWAP_tmp_29: av_alias64 = *((&raw mut *(*(&raw mut (*sl)
                            .top_borders as *mut *mut [uint8_t; 96])
                            .offset(top_idx as isize))
                            .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                            as *mut uint8_t)
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *((&raw mut *(*(&raw mut (*sl).top_borders
                            as *mut *mut [uint8_t; 96])
                            .offset(top_idx as isize))
                            .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                            as *mut uint8_t)
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cb
                            .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cb
                            .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_29;
                        let mut SWAP_tmp_30: av_alias64 = *((&raw mut *(*(&raw mut (*sl)
                            .top_borders as *mut *mut [uint8_t; 96])
                            .offset(top_idx as isize))
                            .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                            as *mut uint8_t)
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *((&raw mut *(*(&raw mut (*sl).top_borders
                            as *mut *mut [uint8_t; 96])
                            .offset(top_idx as isize))
                            .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                            as *mut uint8_t)
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cb
                            .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cb
                            .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_30;
                    } else {
                        let mut SWAP_tmp_31: av_alias64 = *((&raw mut *(*(&raw mut (*sl)
                            .top_borders as *mut *mut [uint8_t; 96])
                            .offset(top_idx as isize))
                            .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                            as *mut uint8_t)
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64);
                        *((&raw mut *(*(&raw mut (*sl).top_borders
                            as *mut *mut [uint8_t; 96])
                            .offset(top_idx as isize))
                            .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                            as *mut uint8_t)
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64) = *(src_cb
                            .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64);
                        *(src_cb
                            .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64) = SWAP_tmp_31;
                    }
                    if pixel_shift != 0 {
                        let mut SWAP_tmp_32: av_alias64 = *((&raw mut *(*(&raw mut (*sl)
                            .top_borders as *mut *mut [uint8_t; 96])
                            .offset(top_idx as isize))
                            .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                            as *mut uint8_t)
                            .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *((&raw mut *(*(&raw mut (*sl).top_borders
                            as *mut *mut [uint8_t; 96])
                            .offset(top_idx as isize))
                            .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                            as *mut uint8_t)
                            .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cr
                            .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cr
                            .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_32;
                        let mut SWAP_tmp_33: av_alias64 = *((&raw mut *(*(&raw mut (*sl)
                            .top_borders as *mut *mut [uint8_t; 96])
                            .offset(top_idx as isize))
                            .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                            as *mut uint8_t)
                            .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *((&raw mut *(*(&raw mut (*sl).top_borders
                            as *mut *mut [uint8_t; 96])
                            .offset(top_idx as isize))
                            .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                            as *mut uint8_t)
                            .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cr
                            .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cr
                            .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_33;
                    } else {
                        let mut SWAP_tmp_34: av_alias64 = *((&raw mut *(*(&raw mut (*sl)
                            .top_borders as *mut *mut [uint8_t; 96])
                            .offset(top_idx as isize))
                            .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                            as *mut uint8_t)
                            .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64);
                        *((&raw mut *(*(&raw mut (*sl).top_borders
                            as *mut *mut [uint8_t; 96])
                            .offset(top_idx as isize))
                            .offset(((*sl).mb_x + 1 as ::core::ffi::c_int) as isize)
                            as *mut uint8_t)
                            .offset(((32 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64) = *(src_cr
                            .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64);
                        *(src_cr
                            .offset(((17 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64) = SWAP_tmp_34;
                    }
                }
            } else {
                if deblock_topleft != 0 {
                    if pixel_shift != 0 {
                        let mut SWAP_tmp_35: av_alias64 = *(top_border_m1
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(top_border_m1
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cb
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cb
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_35;
                        let mut SWAP_tmp_36: av_alias64 = *(top_border_m1
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(top_border_m1
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cb
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cb
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_36;
                    } else {
                        let mut SWAP_tmp_37: av_alias64 = *(top_border_m1
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64);
                        *(top_border_m1
                            .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64) = *(src_cb
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            ) as *mut av_alias64);
                        *(src_cb
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            ) as *mut av_alias64) = SWAP_tmp_37;
                    }
                    if pixel_shift != 0 {
                        let mut SWAP_tmp_38: av_alias64 = *(top_border_m1
                            .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(top_border_m1
                            .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cr
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cr
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_38;
                        let mut SWAP_tmp_39: av_alias64 = *(top_border_m1
                            .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(top_border_m1
                            .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = *(src_cr
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64);
                        *(src_cr
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            )
                            .offset(8 as ::core::ffi::c_int as isize)
                            as *mut av_alias64) = SWAP_tmp_39;
                    } else {
                        let mut SWAP_tmp_40: av_alias64 = *(top_border_m1
                            .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64);
                        *(top_border_m1
                            .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                            as *mut av_alias64) = *(src_cr
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            ) as *mut av_alias64);
                        *(src_cr
                            .offset(
                                -(((7 as ::core::ffi::c_int) << pixel_shift) as isize),
                            ) as *mut av_alias64) = SWAP_tmp_40;
                    }
                }
                if pixel_shift != 0 {
                    let mut SWAP_tmp_41: av_alias64 = *(top_border
                        .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(top_border
                        .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_cb
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset(pixel_shift as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(src_cb
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset(pixel_shift as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_41;
                    let mut SWAP_tmp_42: av_alias64 = *(top_border
                        .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(top_border
                        .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_cb
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset(pixel_shift as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(src_cb
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset(pixel_shift as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_42;
                } else {
                    let mut SWAP_tmp_43: av_alias64 = *(top_border
                        .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64);
                    *(top_border
                        .offset(((16 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64) = *(src_cb
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset(pixel_shift as isize) as *mut av_alias64);
                    *(src_cb
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset(pixel_shift as isize) as *mut av_alias64) = SWAP_tmp_43;
                }
                if pixel_shift != 0 {
                    let mut SWAP_tmp_44: av_alias64 = *(top_border
                        .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(top_border
                        .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_cr
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset(pixel_shift as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(src_cr
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset(pixel_shift as isize)
                        .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_44;
                    let mut SWAP_tmp_45: av_alias64 = *(top_border
                        .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(top_border
                        .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = *(src_cr
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset(pixel_shift as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64);
                    *(src_cr
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset(pixel_shift as isize)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64) = SWAP_tmp_45;
                } else {
                    let mut SWAP_tmp_46: av_alias64 = *(top_border
                        .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64);
                    *(top_border
                        .offset(((24 as ::core::ffi::c_int) << pixel_shift) as isize)
                        as *mut av_alias64) = *(src_cr
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset(pixel_shift as isize) as *mut av_alias64);
                    *(src_cr
                        .offset(1 as ::core::ffi::c_int as isize)
                        .offset(pixel_shift as isize) as *mut av_alias64) = SWAP_tmp_46;
                }
            }
        }
    }
}
#[inline]
unsafe extern "C" fn init_get_bits(
    mut s: *mut GetBitContext,
    mut buffer: *const uint8_t,
    mut bit_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if bit_size
        >= 2147483647 as ::core::ffi::c_int
            - (if 7 as ::core::ffi::c_int
                > 64 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            {
                7 as ::core::ffi::c_int
            } else {
                64 as ::core::ffi::c_int * 8 as ::core::ffi::c_int
            }) || bit_size < 0 as ::core::ffi::c_int || buffer.is_null()
    {
        bit_size = 0 as ::core::ffi::c_int;
        buffer = ::core::ptr::null::<uint8_t>();
        ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*s).buffer = buffer;
    (*s).size_in_bits = bit_size;
    (*s).size_in_bits_plus8 = bit_size + 8 as ::core::ffi::c_int;
    (*s).index = 0 as ::core::ffi::c_int;
    return ret;
}
#[inline(always)]
unsafe extern "C" fn dctcoef_get(
    mut mb: *mut int16_t,
    mut high_bit_depth: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if high_bit_depth != 0 {
        return (*((mb as *mut int32_t).offset(index as isize) as *const av_alias32))
            .u32_0 as ::core::ffi::c_int
    } else {
        return (*(mb.offset(index as isize) as *const av_alias16)).u16_0
            as ::core::ffi::c_int
    };
}
#[inline(always)]
unsafe extern "C" fn dctcoef_set(
    mut mb: *mut int16_t,
    mut high_bit_depth: ::core::ffi::c_int,
    mut index: ::core::ffi::c_int,
    mut value: ::core::ffi::c_int,
) {
    if high_bit_depth != 0 {
        (*((mb as *mut int32_t).offset(index as isize) as *mut av_alias32)).u32_0 = value
            as uint32_t;
    } else {
        (*(mb.offset(index as isize) as *mut av_alias16)).u16_0 = value as uint16_t;
    };
}
#[inline(always)]
unsafe extern "C" fn hl_decode_mb_predict_luma(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut mb_type: ::core::ffi::c_int,
    mut simple: ::core::ffi::c_int,
    mut transform_bypass: ::core::ffi::c_int,
    mut pixel_shift: ::core::ffi::c_int,
    mut block_offset: *const ::core::ffi::c_int,
    mut linesize: ::core::ffi::c_int,
    mut dest_y: *mut uint8_t,
    mut p: ::core::ffi::c_int,
) {
    let mut idct_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    > = None;
    let mut idct_dc_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    > = None;
    let mut i: ::core::ffi::c_int = 0;
    let mut qscale: ::core::ffi::c_int = if p == 0 as ::core::ffi::c_int {
        (*sl).qscale
    } else {
        (*sl).chroma_qp[(p - 1 as ::core::ffi::c_int) as usize]
    };
    block_offset = block_offset.offset((16 as ::core::ffi::c_int * p) as isize);
    if mb_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        if mb_type & 0x1000000 as ::core::ffi::c_int != 0 {
            if transform_bypass != 0 {
                idct_add = (*h).h264dsp.add_pixels8_clear;
                idct_dc_add = idct_add;
            } else {
                idct_dc_add = (*h).h264dsp.idct8_dc_add;
                idct_add = (*h).h264dsp.idct8_add;
            }
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                let ptr: *mut uint8_t = dest_y
                    .offset(*block_offset.offset(i as isize) as isize);
                let dir: ::core::ffi::c_int = (*sl)
                    .intra4x4_pred_mode_cache[scan8[i as usize] as usize]
                    as ::core::ffi::c_int;
                if transform_bypass != 0
                    && (*(*h).ps.sps).profile_idc == 244 as ::core::ffi::c_int
                    && dir <= 1 as ::core::ffi::c_int
                {
                    if ((*h).x264_build as ::core::ffi::c_uint)
                        < 151 as ::core::ffi::c_uint
                    {
                        (*h)
                            .hpc
                            .pred8x8l_add[dir as usize]
                            .expect(
                                "non-null function pointer",
                            )(
                            ptr,
                            (&raw mut (*sl).mb as *mut int16_t)
                                .offset(
                                    ((i * 16 as ::core::ffi::c_int
                                        + p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                                ),
                            linesize as ptrdiff_t,
                        );
                    } else {
                        (*h)
                            .hpc
                            .pred8x8l_filter_add[dir as usize]
                            .expect(
                                "non-null function pointer",
                            )(
                            ptr,
                            (&raw mut (*sl).mb as *mut int16_t)
                                .offset(
                                    ((i * 16 as ::core::ffi::c_int
                                        + p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                                ),
                            ((*sl).topleft_samples_available << i
                                & 0x8000 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                            ((*sl).topright_samples_available << i
                                & 0x4000 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                            linesize as ptrdiff_t,
                        );
                    }
                } else {
                    let nnz: ::core::ffi::c_int = (*sl)
                        .non_zero_count_cache[scan8[(i + p * 16 as ::core::ffi::c_int)
                        as usize] as usize] as ::core::ffi::c_int;
                    (*h)
                        .hpc
                        .pred8x8l[dir as usize]
                        .expect(
                            "non-null function pointer",
                        )(
                        ptr,
                        ((*sl).topleft_samples_available << i
                            & 0x8000 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                        ((*sl).topright_samples_available << i
                            & 0x4000 as ::core::ffi::c_uint) as ::core::ffi::c_int,
                        linesize as ptrdiff_t,
                    );
                    if nnz != 0 {
                        if nnz == 1 as ::core::ffi::c_int
                            && dctcoef_get(
                                &raw mut (*sl).mb as *mut int16_t,
                                pixel_shift,
                                i * 16 as ::core::ffi::c_int + p * 256 as ::core::ffi::c_int,
                            ) != 0
                        {
                            idct_dc_add
                                .expect(
                                    "non-null function pointer",
                                )(
                                ptr,
                                (&raw mut (*sl).mb as *mut int16_t)
                                    .offset(
                                        ((i * 16 as ::core::ffi::c_int
                                            + p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                                    ),
                                linesize,
                            );
                        } else {
                            idct_add
                                .expect(
                                    "non-null function pointer",
                                )(
                                ptr,
                                (&raw mut (*sl).mb as *mut int16_t)
                                    .offset(
                                        ((i * 16 as ::core::ffi::c_int
                                            + p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                                    ),
                                linesize,
                            );
                        }
                    }
                }
                i += 4 as ::core::ffi::c_int;
            }
        } else {
            if transform_bypass != 0 {
                idct_add = (*h).h264dsp.add_pixels4_clear;
                idct_dc_add = idct_add;
            } else {
                idct_dc_add = (*h).h264dsp.idct_dc_add;
                idct_add = (*h).h264dsp.idct_add;
            }
            i = 0 as ::core::ffi::c_int;
            while i < 16 as ::core::ffi::c_int {
                let ptr_0: *mut uint8_t = dest_y
                    .offset(*block_offset.offset(i as isize) as isize);
                let dir_0: ::core::ffi::c_int = (*sl)
                    .intra4x4_pred_mode_cache[scan8[i as usize] as usize]
                    as ::core::ffi::c_int;
                if transform_bypass != 0
                    && (*(*h).ps.sps).profile_idc == 244 as ::core::ffi::c_int
                    && dir_0 <= 1 as ::core::ffi::c_int
                {
                    (*h)
                        .hpc
                        .pred4x4_add[dir_0 as usize]
                        .expect(
                            "non-null function pointer",
                        )(
                        ptr_0,
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((i * 16 as ::core::ffi::c_int
                                    + p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                            ),
                        linesize as ptrdiff_t,
                    );
                } else {
                    let mut topright: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                    let mut nnz_0: ::core::ffi::c_int = 0;
                    let mut tr: ::core::ffi::c_int = 0;
                    let mut tr_high: uint64_t = 0;
                    if dir_0 == 3 as ::core::ffi::c_int
                        || dir_0 == 7 as ::core::ffi::c_int
                    {
                        let topright_avail: ::core::ffi::c_int = ((*sl)
                            .topright_samples_available << i
                            & 0x8000 as ::core::ffi::c_uint) as ::core::ffi::c_int;
                        if topright_avail == 0 {
                            if pixel_shift != 0 {
                                tr_high = (*(ptr_0 as *mut uint16_t)
                                    .offset(
                                        (3 as ::core::ffi::c_int
                                            - linesize / 2 as ::core::ffi::c_int) as isize,
                                    ) as ::core::ffi::c_ulonglong)
                                    .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong)
                                    as uint64_t;
                                topright = &raw mut tr_high as *mut uint8_t;
                            } else {
                                tr = (*ptr_0
                                    .offset((3 as ::core::ffi::c_int - linesize) as isize)
                                    as ::core::ffi::c_uint)
                                    .wrapping_mul(0x1010101 as ::core::ffi::c_uint)
                                    as ::core::ffi::c_int;
                                topright = &raw mut tr as *mut uint8_t;
                            }
                        } else {
                            topright = ptr_0
                                .offset(((4 as ::core::ffi::c_int) << pixel_shift) as isize)
                                .offset(-(linesize as isize));
                        }
                    } else {
                        topright = ::core::ptr::null_mut::<uint8_t>();
                    }
                    (*h)
                        .hpc
                        .pred4x4[dir_0 as usize]
                        .expect(
                            "non-null function pointer",
                        )(ptr_0, topright, linesize as ptrdiff_t);
                    nnz_0 = (*sl)
                        .non_zero_count_cache[scan8[(i + p * 16 as ::core::ffi::c_int)
                        as usize] as usize] as ::core::ffi::c_int;
                    if nnz_0 != 0 {
                        if nnz_0 == 1 as ::core::ffi::c_int
                            && dctcoef_get(
                                &raw mut (*sl).mb as *mut int16_t,
                                pixel_shift,
                                i * 16 as ::core::ffi::c_int + p * 256 as ::core::ffi::c_int,
                            ) != 0
                        {
                            idct_dc_add
                                .expect(
                                    "non-null function pointer",
                                )(
                                ptr_0,
                                (&raw mut (*sl).mb as *mut int16_t)
                                    .offset(
                                        ((i * 16 as ::core::ffi::c_int
                                            + p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                                    ),
                                linesize,
                            );
                        } else {
                            idct_add
                                .expect(
                                    "non-null function pointer",
                                )(
                                ptr_0,
                                (&raw mut (*sl).mb as *mut int16_t)
                                    .offset(
                                        ((i * 16 as ::core::ffi::c_int
                                            + p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                                    ),
                                linesize,
                            );
                        }
                    }
                }
                i += 1;
            }
        }
    } else {
        (*h)
            .hpc
            .pred16x16[(*sl).intra16x16_pred_mode as usize]
            .expect("non-null function pointer")(dest_y, linesize as ptrdiff_t);
        if (*sl)
            .non_zero_count_cache[scan8[(48 as ::core::ffi::c_int + p) as usize]
            as usize] != 0
        {
            if transform_bypass == 0 {
                (*h)
                    .h264dsp
                    .luma_dc_dequant_idct
                    .expect(
                        "non-null function pointer",
                    )(
                    (&raw mut (*sl).mb as *mut int16_t)
                        .offset(
                            ((p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                        ),
                    &raw mut *(&raw mut (*sl).mb_luma_dc as *mut [int16_t; 32])
                        .offset(p as isize) as *mut int16_t,
                    (*(*(*h).ps.pps)
                        .dequant4_coeff[p as usize]
                        .offset(qscale as isize))[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int,
                );
            } else {
                static mut dc_mapping: [uint8_t; 16] = [
                    (0 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (1 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (4 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (5 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (2 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (3 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (6 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (7 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (8 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (9 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (12 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (13 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (10 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (11 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (14 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                    (15 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as uint8_t,
                ];
                i = 0 as ::core::ffi::c_int;
                while i < 16 as ::core::ffi::c_int {
                    dctcoef_set(
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                            ),
                        pixel_shift,
                        dc_mapping[i as usize] as ::core::ffi::c_int,
                        dctcoef_get(
                            &raw mut *(&raw mut (*sl).mb_luma_dc as *mut [int16_t; 32])
                                .offset(p as isize) as *mut int16_t,
                            pixel_shift,
                            i,
                        ),
                    );
                    i += 1;
                }
            }
        }
    };
}
#[inline(always)]
unsafe extern "C" fn hl_decode_mb_idct_luma(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut mb_type: ::core::ffi::c_int,
    mut simple: ::core::ffi::c_int,
    mut transform_bypass: ::core::ffi::c_int,
    mut pixel_shift: ::core::ffi::c_int,
    mut block_offset: *const ::core::ffi::c_int,
    mut linesize: ::core::ffi::c_int,
    mut dest_y: *mut uint8_t,
    mut p: ::core::ffi::c_int,
) {
    let mut idct_add: Option<
        unsafe extern "C" fn(*mut uint8_t, *mut int16_t, ::core::ffi::c_int) -> (),
    > = None;
    let mut i: ::core::ffi::c_int = 0;
    block_offset = block_offset.offset((16 as ::core::ffi::c_int * p) as isize);
    if mb_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int == 0 {
        if mb_type & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
            if transform_bypass != 0 {
                if (*(*h).ps.sps).profile_idc == 244 as ::core::ffi::c_int
                    && ((*sl).intra16x16_pred_mode == 2 as ::core::ffi::c_int
                        || (*sl).intra16x16_pred_mode == 1 as ::core::ffi::c_int)
                {
                    (*h)
                        .hpc
                        .pred16x16_add[(*sl).intra16x16_pred_mode as usize]
                        .expect(
                            "non-null function pointer",
                        )(
                        dest_y,
                        block_offset,
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                ((p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                            ),
                        linesize as ptrdiff_t,
                    );
                } else {
                    i = 0 as ::core::ffi::c_int;
                    while i < 16 as ::core::ffi::c_int {
                        if (*sl)
                            .non_zero_count_cache[scan8[(i
                            + p * 16 as ::core::ffi::c_int) as usize] as usize]
                            as ::core::ffi::c_int != 0
                            || dctcoef_get(
                                &raw mut (*sl).mb as *mut int16_t,
                                pixel_shift,
                                i * 16 as ::core::ffi::c_int + p * 256 as ::core::ffi::c_int,
                            ) != 0
                        {
                            (*h)
                                .h264dsp
                                .add_pixels4_clear
                                .expect(
                                    "non-null function pointer",
                                )(
                                dest_y.offset(*block_offset.offset(i as isize) as isize),
                                (&raw mut (*sl).mb as *mut int16_t)
                                    .offset(
                                        ((i * 16 as ::core::ffi::c_int
                                            + p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                                    ),
                                linesize,
                            );
                        }
                        i += 1;
                    }
                }
            } else {
                (*h)
                    .h264dsp
                    .idct_add16intra
                    .expect(
                        "non-null function pointer",
                    )(
                    dest_y,
                    block_offset,
                    (&raw mut (*sl).mb as *mut int16_t)
                        .offset(
                            ((p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                        ),
                    linesize,
                    (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                        .offset(
                            (p * 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *const uint8_t,
                );
            }
        } else if (*sl).cbp & 15 as ::core::ffi::c_int != 0 {
            if transform_bypass != 0 {
                let di: ::core::ffi::c_int = if mb_type & 0x1000000 as ::core::ffi::c_int
                    != 0
                {
                    4 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                };
                idct_add = if mb_type & 0x1000000 as ::core::ffi::c_int != 0 {
                    (*h).h264dsp.add_pixels8_clear
                } else {
                    (*h).h264dsp.add_pixels4_clear
                };
                i = 0 as ::core::ffi::c_int;
                while i < 16 as ::core::ffi::c_int {
                    if (*sl)
                        .non_zero_count_cache[scan8[(i + p * 16 as ::core::ffi::c_int)
                        as usize] as usize] != 0
                    {
                        idct_add
                            .expect(
                                "non-null function pointer",
                            )(
                            dest_y.offset(*block_offset.offset(i as isize) as isize),
                            (&raw mut (*sl).mb as *mut int16_t)
                                .offset(
                                    ((i * 16 as ::core::ffi::c_int
                                        + p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                                ),
                            linesize,
                        );
                    }
                    i += di;
                }
            } else if mb_type & 0x1000000 as ::core::ffi::c_int != 0 {
                (*h)
                    .h264dsp
                    .idct8_add4
                    .expect(
                        "non-null function pointer",
                    )(
                    dest_y,
                    block_offset,
                    (&raw mut (*sl).mb as *mut int16_t)
                        .offset(
                            ((p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                        ),
                    linesize,
                    (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                        .offset(
                            (p * 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *const uint8_t,
                );
            } else {
                (*h)
                    .h264dsp
                    .idct_add16
                    .expect(
                        "non-null function pointer",
                    )(
                    dest_y,
                    block_offset,
                    (&raw mut (*sl).mb as *mut int16_t)
                        .offset(
                            ((p * 256 as ::core::ffi::c_int) << pixel_shift) as isize,
                        ),
                    linesize,
                    (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                        .offset(
                            (p * 5 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *const uint8_t,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_hl_decode_mb(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mb_type: ::core::ffi::c_int = *(*h).cur_pic.mb_type.offset(mb_xy as isize)
        as ::core::ffi::c_int;
    let mut is_complex: ::core::ffi::c_int = (0 as ::core::ffi::c_int != 0
        || (*sl).is_complex != 0
        || mb_type & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
        || (*sl).qscale == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if (*(*h).ps.sps).chroma_format_idc == 3 as ::core::ffi::c_int {
        if is_complex != 0 || (*h).pixel_shift != 0 {
            hl_decode_mb_444_complex(h, sl);
        } else {
            hl_decode_mb_444_simple_8(h, sl);
        }
    } else if is_complex != 0 {
        hl_decode_mb_complex(h, sl);
    } else if (*h).pixel_shift != 0 {
        hl_decode_mb_simple_16(h, sl);
    } else {
        hl_decode_mb_simple_8(h, sl);
    };
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
pub struct ThreadProgress {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct ThreadFrameProgress {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVRefStructPool {
    _opaque: [u8; 0],
}
