// Translated from FFmpeg 9.0.1 libavcodec/h264_cabac.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// H.26L/H.264/AVC/JVT/14496-10/... cabac decoding
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
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    static ff_h264_chroma_dc_scan: [uint8_t; 4];
    static ff_h264_cabac_tables: [uint8_t; 1343];
    static ff_h264_chroma422_dc_scan: [uint8_t; 8];
    static ff_h264_i_mb_type_info: [IMbInfo; 26];
    static ff_h264_p_mb_type_info: [PMbInfo; 5];
    static ff_h264_p_sub_mb_type_info: [PMbInfo; 4];
    fn ff_init_cabac_decoder(
        c: *mut CABACContext,
        buf: *const uint8_t,
        buf_size: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static ff_h264_b_mb_type_info: [PMbInfo; 23];
    static ff_h264_b_sub_mb_type_info: [PMbInfo; 13];
    fn ff_h264_check_intra4x4_pred_mode(
        pred_mode_cache: *mut int8_t,
        logctx: *mut ::core::ffi::c_void,
        top_samples_available: ::core::ffi::c_int,
        left_samples_available: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ff_h264_check_intra_pred_mode(
        logctx: *mut ::core::ffi::c_void,
        top_samples_available: ::core::ffi::c_int,
        left_samples_available: ::core::ffi::c_int,
        mode: ::core::ffi::c_int,
        is_chroma: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    static ff_h264_mb_sizes: [uint16_t; 4];
    fn ff_h264_pred_direct_motion(
        h: *const H264Context,
        sl: *mut H264SliceContext,
        mb_type: *mut ::core::ffi::c_int,
    );
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
pub type int8_t = ::core::ffi::c_schar;
pub type int16_t = ::core::ffi::c_short;
pub type int32_t = ::core::ffi::c_int;
pub type int64_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CABACContext {
    pub low: ::core::ffi::c_int,
    pub range: ::core::ffi::c_int,
    pub bytestream_start: *const uint8_t,
    pub bytestream: *const uint8_t,
    pub bytestream_end: *const uint8_t,
}
pub type ptrdiff_t = ::core::ffi::c_long;
pub type atomic_int = ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct PMbInfo {
    pub type_0: uint16_t,
    pub partition_count: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct IMbInfo {
    pub type_0: uint16_t,
    pub pred_mode: uint8_t,
    pub cbp: uint8_t,
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
        let v_1: uint64_t = (if size == 2 as ::core::ffi::c_int {
            (val as ::core::ffi::c_ulonglong)
                .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong)
        } else {
            (val as ::core::ffi::c_ulonglong)
                .wrapping_mul(0x100000001 as ::core::ffi::c_ulonglong)
        }) as uint64_t;
        *(p.offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_1;
        if h == 1 as ::core::ffi::c_int {
            return;
        }
        *(p.offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_1;
        if h == 2 as ::core::ffi::c_int {
            return;
        }
        *(p.offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_1;
        *(p.offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_1;
    } else if w == 16 as ::core::ffi::c_int {
        let v_2: uint64_t = (val as ::core::ffi::c_ulonglong)
            .wrapping_mul(0x100000001 as ::core::ffi::c_ulonglong) as uint64_t;
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        *(p
            .offset(8 as ::core::ffi::c_int as isize)
            .offset((0 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        *(p
            .offset(8 as ::core::ffi::c_int as isize)
            .offset((1 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        if h == 2 as ::core::ffi::c_int {
            return;
        }
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        *(p
            .offset(8 as ::core::ffi::c_int as isize)
            .offset((2 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        *(p
            .offset(0 as ::core::ffi::c_int as isize)
            .offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
        *(p
            .offset(8 as ::core::ffi::c_int as isize)
            .offset((3 as ::core::ffi::c_int * stride) as isize) as *mut uint64_t) = v_2;
    }
}
#[inline(always)]
unsafe extern "C" fn pred_intra_mode(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let index8: ::core::ffi::c_int = scan8[n as usize] as ::core::ffi::c_int;
    let left: ::core::ffi::c_int = (*sl)
        .intra4x4_pred_mode_cache[(index8 - 1 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let top: ::core::ffi::c_int = (*sl)
        .intra4x4_pred_mode_cache[(index8 - 8 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let min: ::core::ffi::c_int = if left > top { top } else { left };
    if min < 0 as ::core::ffi::c_int {
        return 2 as ::core::ffi::c_int
    } else {
        return min
    };
}
static mut cabac_context_init_I: [[int8_t; 2]; 1024] = [
    [20 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [2 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
    [3 as ::core::ffi::c_int as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [20 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [2 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
    [3 as ::core::ffi::c_int as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [-(28 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    [-(23 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 54 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
    [4 as ::core::ffi::c_int as int8_t, 86 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 97 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
    [3 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
    [-(17 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 82 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [-(21 as ::core::ffi::c_int) as int8_t, 107 as ::core::ffi::c_int as int8_t],
    [-(27 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    [-(31 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    [-(24 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    [-(18 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
    [-(27 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    [-(21 as ::core::ffi::c_int) as int8_t, 114 as ::core::ffi::c_int as int8_t],
    [-(30 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    [-(17 as ::core::ffi::c_int) as int8_t, 123 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 115 as ::core::ffi::c_int as int8_t],
    [-(16 as ::core::ffi::c_int) as int8_t, 122 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 115 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
    [-(30 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
    [-(20 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 56 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [-(22 as ::core::ffi::c_int) as int8_t, 125 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
    [8 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
    [5 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [11 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
    [5 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [31 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [28 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
    [16 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 108 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 101 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [-(16 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
    [-(19 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 72 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [18 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 95 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
    [2 as ::core::ffi::c_int as int8_t, 72 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
    [21 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
    [9 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
    [16 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 72 as ::core::ffi::c_int as int8_t],
    [24 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
    [8 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 18 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 18 as ::core::ffi::c_int as int8_t],
    [6 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
    [20 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
    [4 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
    [11 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
    [11 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [16 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
    [30 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
    [18 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 89 as ::core::ffi::c_int as int8_t],
    [26 as ::core::ffi::c_int as int8_t, -(19 as ::core::ffi::c_int) as int8_t],
    [22 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
    [26 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
    [30 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
    [28 as ::core::ffi::c_int as int8_t, -(20 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, -(23 as ::core::ffi::c_int) as int8_t],
    [37 as ::core::ffi::c_int as int8_t, -(27 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, -(23 as ::core::ffi::c_int) as int8_t],
    [40 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
    [38 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, -(11 as ::core::ffi::c_int) as int8_t],
    [40 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [41 as ::core::ffi::c_int as int8_t, -(6 as ::core::ffi::c_int) as int8_t],
    [38 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
    [41 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
    [30 as ::core::ffi::c_int as int8_t, -(6 as ::core::ffi::c_int) as int8_t],
    [27 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
    [26 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
    [37 as ::core::ffi::c_int as int8_t, -(16 as ::core::ffi::c_int) as int8_t],
    [35 as ::core::ffi::c_int as int8_t, -(4 as ::core::ffi::c_int) as int8_t],
    [38 as ::core::ffi::c_int as int8_t, -(8 as ::core::ffi::c_int) as int8_t],
    [38 as ::core::ffi::c_int as int8_t, -(3 as ::core::ffi::c_int) as int8_t],
    [37 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
    [38 as ::core::ffi::c_int as int8_t, 5 as ::core::ffi::c_int as int8_t],
    [42 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [35 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
    [39 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
    [27 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
    [21 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [2 as ::core::ffi::c_int as int8_t, 97 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 42 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 54 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 27 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 39 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
    [-(16 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [-(17 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
    [-(20 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
    [-(19 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 33 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 48 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [3 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
    [16 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [20 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
    [18 as ::core::ffi::c_int as int8_t, 32 as ::core::ffi::c_int as int8_t],
    [5 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
    [9 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
    [-(16 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [4 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 99 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
    [2 as ::core::ffi::c_int as int8_t, 95 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
    [3 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [4 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
    [11 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
    [5 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
    [6 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 14 as ::core::ffi::c_int as int8_t],
    [18 as ::core::ffi::c_int as int8_t, 13 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 26 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
    [24 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
    [21 as ::core::ffi::c_int as int8_t, 21 as ::core::ffi::c_int as int8_t],
    [25 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
    [31 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
    [22 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
    [19 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
    [9 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 109 as ::core::ffi::c_int as int8_t],
    [36 as ::core::ffi::c_int as int8_t, -(35 as ::core::ffi::c_int) as int8_t],
    [36 as ::core::ffi::c_int as int8_t, -(34 as ::core::ffi::c_int) as int8_t],
    [32 as ::core::ffi::c_int as int8_t, -(26 as ::core::ffi::c_int) as int8_t],
    [37 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
    [44 as ::core::ffi::c_int as int8_t, -(32 as ::core::ffi::c_int) as int8_t],
    [34 as ::core::ffi::c_int as int8_t, -(18 as ::core::ffi::c_int) as int8_t],
    [34 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [40 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, -(7 as ::core::ffi::c_int) as int8_t],
    [35 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [38 as ::core::ffi::c_int as int8_t, 2 as ::core::ffi::c_int as int8_t],
    [33 as ::core::ffi::c_int as int8_t, 13 as ::core::ffi::c_int as int8_t],
    [23 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [29 as ::core::ffi::c_int as int8_t, -(3 as ::core::ffi::c_int) as int8_t],
    [26 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [22 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
    [31 as ::core::ffi::c_int as int8_t, -(7 as ::core::ffi::c_int) as int8_t],
    [35 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [34 as ::core::ffi::c_int as int8_t, -(3 as ::core::ffi::c_int) as int8_t],
    [34 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
    [36 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
    [34 as ::core::ffi::c_int as int8_t, 5 as ::core::ffi::c_int as int8_t],
    [32 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
    [35 as ::core::ffi::c_int as int8_t, 5 as ::core::ffi::c_int as int8_t],
    [34 as ::core::ffi::c_int as int8_t, 12 as ::core::ffi::c_int as int8_t],
    [39 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
    [30 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
    [34 as ::core::ffi::c_int as int8_t, 26 as ::core::ffi::c_int as int8_t],
    [29 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
    [19 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [31 as ::core::ffi::c_int as int8_t, 21 as ::core::ffi::c_int as int8_t],
    [31 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
    [25 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [-(17 as ::core::ffi::c_int) as int8_t, 120 as ::core::ffi::c_int as int8_t],
    [-(20 as ::core::ffi::c_int) as int8_t, 112 as ::core::ffi::c_int as int8_t],
    [-(18 as ::core::ffi::c_int) as int8_t, 114 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
    [-(26 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [-(24 as ::core::ffi::c_int) as int8_t, 56 as ::core::ffi::c_int as int8_t],
    [-(23 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(24 as ::core::ffi::c_int) as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [23 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
    [26 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
    [40 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [49 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
    [44 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
    [45 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
    [44 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
    [33 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
    [19 as ::core::ffi::c_int as int8_t, 82 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 23 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
    [-(18 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [2 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
    [21 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
    [24 as ::core::ffi::c_int as int8_t, -(11 as ::core::ffi::c_int) as int8_t],
    [28 as ::core::ffi::c_int as int8_t, -(8 as ::core::ffi::c_int) as int8_t],
    [28 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
    [29 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
    [29 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
    [35 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
    [29 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
    [-(17 as ::core::ffi::c_int) as int8_t, 123 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 115 as ::core::ffi::c_int as int8_t],
    [-(16 as ::core::ffi::c_int) as int8_t, 122 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 115 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
    [-(30 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    [-(17 as ::core::ffi::c_int) as int8_t, 123 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 115 as ::core::ffi::c_int as int8_t],
    [-(16 as ::core::ffi::c_int) as int8_t, 122 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 115 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
    [-(30 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
    [8 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
    [5 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [11 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
    [5 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [31 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [28 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
    [16 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 108 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 101 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [-(16 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
    [-(19 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 72 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [18 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
    [8 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
    [5 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [11 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
    [5 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [31 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [28 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
    [16 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 108 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 101 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [-(16 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
    [-(19 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 72 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [18 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
    [24 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
    [8 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 18 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 18 as ::core::ffi::c_int as int8_t],
    [6 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
    [20 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
    [4 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
    [11 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
    [11 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [16 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
    [30 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
    [18 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 89 as ::core::ffi::c_int as int8_t],
    [26 as ::core::ffi::c_int as int8_t, -(19 as ::core::ffi::c_int) as int8_t],
    [22 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
    [26 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
    [30 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
    [28 as ::core::ffi::c_int as int8_t, -(20 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, -(23 as ::core::ffi::c_int) as int8_t],
    [37 as ::core::ffi::c_int as int8_t, -(27 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, -(23 as ::core::ffi::c_int) as int8_t],
    [40 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
    [38 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, -(11 as ::core::ffi::c_int) as int8_t],
    [40 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [41 as ::core::ffi::c_int as int8_t, -(6 as ::core::ffi::c_int) as int8_t],
    [38 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
    [41 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
    [24 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
    [8 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 18 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 18 as ::core::ffi::c_int as int8_t],
    [6 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
    [20 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
    [4 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
    [11 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
    [11 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [16 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
    [30 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
    [18 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 89 as ::core::ffi::c_int as int8_t],
    [26 as ::core::ffi::c_int as int8_t, -(19 as ::core::ffi::c_int) as int8_t],
    [22 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
    [26 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
    [30 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
    [28 as ::core::ffi::c_int as int8_t, -(20 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, -(23 as ::core::ffi::c_int) as int8_t],
    [37 as ::core::ffi::c_int as int8_t, -(27 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, -(23 as ::core::ffi::c_int) as int8_t],
    [40 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
    [38 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, -(11 as ::core::ffi::c_int) as int8_t],
    [40 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [41 as ::core::ffi::c_int as int8_t, -(6 as ::core::ffi::c_int) as int8_t],
    [38 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
    [41 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
    [-(17 as ::core::ffi::c_int) as int8_t, 120 as ::core::ffi::c_int as int8_t],
    [-(20 as ::core::ffi::c_int) as int8_t, 112 as ::core::ffi::c_int as int8_t],
    [-(18 as ::core::ffi::c_int) as int8_t, 114 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
    [-(26 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [-(24 as ::core::ffi::c_int) as int8_t, 56 as ::core::ffi::c_int as int8_t],
    [-(23 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(24 as ::core::ffi::c_int) as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
    [-(18 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [2 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
    [23 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
    [26 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
    [40 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [49 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
    [44 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
    [45 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
    [44 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
    [33 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
    [19 as ::core::ffi::c_int as int8_t, 82 as ::core::ffi::c_int as int8_t],
    [21 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
    [24 as ::core::ffi::c_int as int8_t, -(11 as ::core::ffi::c_int) as int8_t],
    [28 as ::core::ffi::c_int as int8_t, -(8 as ::core::ffi::c_int) as int8_t],
    [28 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
    [29 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
    [29 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
    [35 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
    [29 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 23 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
    [-(17 as ::core::ffi::c_int) as int8_t, 120 as ::core::ffi::c_int as int8_t],
    [-(20 as ::core::ffi::c_int) as int8_t, 112 as ::core::ffi::c_int as int8_t],
    [-(18 as ::core::ffi::c_int) as int8_t, 114 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
    [-(26 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [-(24 as ::core::ffi::c_int) as int8_t, 56 as ::core::ffi::c_int as int8_t],
    [-(23 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(24 as ::core::ffi::c_int) as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
    [-(18 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
    [-(14 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [2 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
    [23 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
    [26 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
    [40 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [49 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
    [44 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
    [45 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
    [44 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
    [33 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
    [19 as ::core::ffi::c_int as int8_t, 82 as ::core::ffi::c_int as int8_t],
    [21 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
    [24 as ::core::ffi::c_int as int8_t, -(11 as ::core::ffi::c_int) as int8_t],
    [28 as ::core::ffi::c_int as int8_t, -(8 as ::core::ffi::c_int) as int8_t],
    [28 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
    [29 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
    [29 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
    [35 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
    [29 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 23 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [3 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
    [16 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [20 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
    [18 as ::core::ffi::c_int as int8_t, 32 as ::core::ffi::c_int as int8_t],
    [5 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
    [9 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
    [-(16 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [4 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [3 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
    [16 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [20 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
    [18 as ::core::ffi::c_int as int8_t, 32 as ::core::ffi::c_int as int8_t],
    [5 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
    [9 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
    [-(11 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
    [-(16 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
    [-(13 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [4 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
    [6 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 14 as ::core::ffi::c_int as int8_t],
    [18 as ::core::ffi::c_int as int8_t, 13 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 26 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
    [24 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
    [21 as ::core::ffi::c_int as int8_t, 21 as ::core::ffi::c_int as int8_t],
    [25 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
    [31 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
    [22 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
    [19 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
    [9 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 109 as ::core::ffi::c_int as int8_t],
    [36 as ::core::ffi::c_int as int8_t, -(35 as ::core::ffi::c_int) as int8_t],
    [36 as ::core::ffi::c_int as int8_t, -(34 as ::core::ffi::c_int) as int8_t],
    [32 as ::core::ffi::c_int as int8_t, -(26 as ::core::ffi::c_int) as int8_t],
    [37 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
    [44 as ::core::ffi::c_int as int8_t, -(32 as ::core::ffi::c_int) as int8_t],
    [34 as ::core::ffi::c_int as int8_t, -(18 as ::core::ffi::c_int) as int8_t],
    [34 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [40 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, -(7 as ::core::ffi::c_int) as int8_t],
    [35 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [38 as ::core::ffi::c_int as int8_t, 2 as ::core::ffi::c_int as int8_t],
    [33 as ::core::ffi::c_int as int8_t, 13 as ::core::ffi::c_int as int8_t],
    [23 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
    [6 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 14 as ::core::ffi::c_int as int8_t],
    [18 as ::core::ffi::c_int as int8_t, 13 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
    [12 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
    [15 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 26 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
    [17 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
    [24 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
    [21 as ::core::ffi::c_int as int8_t, 21 as ::core::ffi::c_int as int8_t],
    [25 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
    [31 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
    [22 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
    [19 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
    [14 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [10 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
    [7 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
    [9 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 109 as ::core::ffi::c_int as int8_t],
    [36 as ::core::ffi::c_int as int8_t, -(35 as ::core::ffi::c_int) as int8_t],
    [36 as ::core::ffi::c_int as int8_t, -(34 as ::core::ffi::c_int) as int8_t],
    [32 as ::core::ffi::c_int as int8_t, -(26 as ::core::ffi::c_int) as int8_t],
    [37 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
    [44 as ::core::ffi::c_int as int8_t, -(32 as ::core::ffi::c_int) as int8_t],
    [34 as ::core::ffi::c_int as int8_t, -(18 as ::core::ffi::c_int) as int8_t],
    [34 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [40 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, -(7 as ::core::ffi::c_int) as int8_t],
    [35 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
    [33 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
    [38 as ::core::ffi::c_int as int8_t, 2 as ::core::ffi::c_int as int8_t],
    [33 as ::core::ffi::c_int as int8_t, 13 as ::core::ffi::c_int as int8_t],
    [23 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
    [13 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 42 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 54 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 27 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 39 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
    [-(16 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [-(17 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 42 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 50 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 54 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
    [1 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
    [-(1 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
    [-(5 as ::core::ffi::c_int) as int8_t, 27 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 39 as ::core::ffi::c_int as int8_t],
    [-(2 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
    [0 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
    [-(16 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
    [-(15 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
    [-(6 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
    [-(4 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
    [-(12 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
    [-(7 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
    [-(9 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
    [-(17 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
    [-(30 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
    [-(30 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    [-(3 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
    [-(8 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
    [-(10 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
    [-(30 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
];
static mut ff_h264_norm_shift: *const uint8_t = ::core::ptr::null::<uint8_t>();
#[inline(always)]
unsafe extern "C" fn write_back_intra_pred_mode(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mut i4x4: *mut int8_t = (*sl)
        .intra4x4_pred_mode
        .offset(*(*h).mb2br_xy.offset((*sl).mb_xy as isize) as isize);
    let mut i4x4_cache: *mut int8_t = &raw mut (*sl).intra4x4_pred_mode_cache
        as *mut int8_t;
    (*(i4x4 as *mut av_alias32)).u32_0 = (*(i4x4_cache
        .offset(4 as ::core::ffi::c_int as isize)
        .offset((8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as isize)
        as *const av_alias32))
        .u32_0;
    *i4x4.offset(4 as ::core::ffi::c_int as isize) = *i4x4_cache
        .offset(
            (7 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as isize,
        );
    *i4x4.offset(5 as ::core::ffi::c_int as isize) = *i4x4_cache
        .offset(
            (7 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                as isize,
        );
    *i4x4.offset(6 as ::core::ffi::c_int as isize) = *i4x4_cache
        .offset(
            (7 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                as isize,
        );
}
static mut ff_h264_lps_range: *const uint8_t = ::core::ptr::null::<uint8_t>();
static mut ff_h264_mlps_state: *const uint8_t = ::core::ptr::null::<uint8_t>();
static mut ff_h264_last_coeff_flag_offset_8x8: *const uint8_t = ::core::ptr::null::<
    uint8_t,
>();
unsafe extern "C" fn refill(mut c: *mut CABACContext) {
    (*c).low
        += ((*(*c).bytestream.offset(0 as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
            + ((*(*c).bytestream.offset(1 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int) << 1 as ::core::ffi::c_int);
    (*c).low
        -= ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int;
    (*c).bytestream = (*c)
        .bytestream
        .offset((16 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
}
#[inline(always)]
unsafe extern "C" fn write_back_non_zero_count(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mut nnz: *mut uint8_t = &raw mut *(*h).non_zero_count.offset(mb_xy as isize)
        as *mut uint8_t;
    let mut nnz_cache: *mut uint8_t = &raw mut (*sl).non_zero_count_cache
        as *mut uint8_t;
    (*(nnz.offset(0 as ::core::ffi::c_int as isize) as *mut uint8_t as *mut av_alias32))
        .u32_0 = (*(nnz_cache
        .offset(
            (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int)
                as isize,
        ) as *mut uint8_t as *const av_alias32))
        .u32_0;
    (*(nnz.offset(4 as ::core::ffi::c_int as isize) as *mut uint8_t as *mut av_alias32))
        .u32_0 = (*(nnz_cache
        .offset(
            (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                as isize,
        ) as *mut uint8_t as *const av_alias32))
        .u32_0;
    (*(nnz.offset(8 as ::core::ffi::c_int as isize) as *mut uint8_t as *mut av_alias32))
        .u32_0 = (*(nnz_cache
        .offset(
            (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int)
                as isize,
        ) as *mut uint8_t as *const av_alias32))
        .u32_0;
    (*(nnz.offset(12 as ::core::ffi::c_int as isize) as *mut uint8_t as *mut av_alias32))
        .u32_0 = (*(nnz_cache
        .offset(
            (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as isize,
        ) as *mut uint8_t as *const av_alias32))
        .u32_0;
    (*(nnz.offset(16 as ::core::ffi::c_int as isize) as *mut uint8_t as *mut av_alias32))
        .u32_0 = (*(nnz_cache
        .offset(
            (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int)
                as isize,
        ) as *mut uint8_t as *const av_alias32))
        .u32_0;
    (*(nnz.offset(20 as ::core::ffi::c_int as isize) as *mut uint8_t as *mut av_alias32))
        .u32_0 = (*(nnz_cache
        .offset(
            (4 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int)
                as isize,
        ) as *mut uint8_t as *const av_alias32))
        .u32_0;
    (*(nnz.offset(32 as ::core::ffi::c_int as isize) as *mut uint8_t as *mut av_alias32))
        .u32_0 = (*(nnz_cache
        .offset(
            (4 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int * 11 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *const av_alias32))
        .u32_0;
    (*(nnz.offset(36 as ::core::ffi::c_int as isize) as *mut uint8_t as *mut av_alias32))
        .u32_0 = (*(nnz_cache
        .offset(
            (4 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int * 12 as ::core::ffi::c_int) as isize,
        ) as *mut uint8_t as *const av_alias32))
        .u32_0;
    if (*h).chroma_y_shift == 0 {
        (*(nnz.offset(24 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *mut av_alias32))
            .u32_0 = (*(nnz_cache
            .offset(
                (4 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *const av_alias32))
            .u32_0;
        (*(nnz.offset(28 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *mut av_alias32))
            .u32_0 = (*(nnz_cache
            .offset(
                (4 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 9 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *const av_alias32))
            .u32_0;
        (*(nnz.offset(40 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *mut av_alias32))
            .u32_0 = (*(nnz_cache
            .offset(
                (4 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 13 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *const av_alias32))
            .u32_0;
        (*(nnz.offset(44 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *mut av_alias32))
            .u32_0 = (*(nnz_cache
            .offset(
                (4 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int * 14 as ::core::ffi::c_int) as isize,
            ) as *mut uint8_t as *const av_alias32))
            .u32_0;
    }
}
#[inline]
unsafe extern "C" fn renorm_cabac_decoder_once(mut c: *mut CABACContext) {
    let mut shift: ::core::ffi::c_int = (((*c).range - 0x100 as ::core::ffi::c_int)
        as uint32_t >> 31 as ::core::ffi::c_int) as ::core::ffi::c_int;
    (*c).range <<= shift;
    (*c).low <<= shift;
    if (*c).low
        & ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int == 0
    {
        refill(c);
    }
}
unsafe extern "C" fn refill2(mut c: *mut CABACContext) {
    let mut i: ::core::ffi::c_int = 0;
    let mut x: ::core::ffi::c_uint = 0;
    i = ((*c).low as ::core::ffi::c_uint).trailing_zeros() as i32
        - 16 as ::core::ffi::c_int;
    x = -(((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    x = x
        .wrapping_add(
            (((*(*c).bytestream.offset(0 as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int) << 9 as ::core::ffi::c_int)
                + ((*(*c).bytestream.offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int) << 1 as ::core::ffi::c_int))
                as ::core::ffi::c_uint,
        );
    (*c).low = ((*c).low as ::core::ffi::c_uint).wrapping_add(x << i)
        as ::core::ffi::c_int as ::core::ffi::c_int;
    (*c).bytestream = (*c)
        .bytestream
        .offset((16 as ::core::ffi::c_int / 8 as ::core::ffi::c_int) as isize);
}
#[inline(always)]
unsafe extern "C" fn write_back_motion_list(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut b_stride: ::core::ffi::c_int,
    mut b_xy: ::core::ffi::c_int,
    mut b8_xy: ::core::ffi::c_int,
    mut mb_type: ::core::ffi::c_int,
    mut list: ::core::ffi::c_int,
) {
    let mut mv_dst: *mut [int16_t; 2] = (*(&raw const (*h).cur_pic.motion_val
        as *const *mut [int16_t; 2])
        .offset(list as isize))
        .offset(b_xy as isize) as *mut [int16_t; 2];
    let mut mv_src: *mut [int16_t; 2] = (&raw mut *(&raw mut (*sl).mv_cache
        as *mut [[int16_t; 2]; 40])
        .offset(list as isize) as *mut [int16_t; 2])
        .offset(
            *(&raw const scan8 as *const uint8_t)
                .offset(0 as ::core::ffi::c_int as isize) as isize,
        ) as *mut [int16_t; 2];
    (*(mv_dst.offset((0 as ::core::ffi::c_int * b_stride) as isize) as *mut av_alias64))
        .u64_0 = (*(mv_src
        .offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
        as *const av_alias64))
        .u64_0;
    (*((mv_dst.offset((0 as ::core::ffi::c_int * b_stride) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
        .u64_0 = (*((mv_src
        .offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
        .u64_0;
    (*(mv_dst.offset((1 as ::core::ffi::c_int * b_stride) as isize) as *mut av_alias64))
        .u64_0 = (*(mv_src
        .offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
        as *const av_alias64))
        .u64_0;
    (*((mv_dst.offset((1 as ::core::ffi::c_int * b_stride) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
        .u64_0 = (*((mv_src
        .offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
        .u64_0;
    (*(mv_dst.offset((2 as ::core::ffi::c_int * b_stride) as isize) as *mut av_alias64))
        .u64_0 = (*(mv_src
        .offset((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
        as *const av_alias64))
        .u64_0;
    (*((mv_dst.offset((2 as ::core::ffi::c_int * b_stride) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
        .u64_0 = (*((mv_src
        .offset((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
        .u64_0;
    (*(mv_dst.offset((3 as ::core::ffi::c_int * b_stride) as isize) as *mut av_alias64))
        .u64_0 = (*(mv_src
        .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
        as *const av_alias64))
        .u64_0;
    (*((mv_dst.offset((3 as ::core::ffi::c_int * b_stride) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
        .u64_0 = (*((mv_src
        .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
        as *mut ::core::ffi::c_char)
        .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
        .u64_0;
    let mut mvd_dst: *mut [uint8_t; 2] = (*(&raw mut (*sl).mvd_table
        as *mut *mut [uint8_t; 2])
        .offset(list as isize))
        .offset(
            (if 0 as ::core::ffi::c_int != 0 {
                (8 as ::core::ffi::c_int * (*sl).mb_xy) as uint32_t
            } else {
                *(*h).mb2br_xy.offset((*sl).mb_xy as isize)
            }) as isize,
        ) as *mut [uint8_t; 2];
    let mut mvd_src: *mut [uint8_t; 2] = (&raw mut *(&raw mut (*sl).mvd_cache
        as *mut [[uint8_t; 2]; 40])
        .offset(list as isize) as *mut [uint8_t; 2])
        .offset(
            *(&raw const scan8 as *const uint8_t)
                .offset(0 as ::core::ffi::c_int as isize) as isize,
        ) as *mut [uint8_t; 2];
    if mb_type & (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int != 0 {
        (*(mvd_dst as *mut av_alias64)).u64_0 = 0 as uint64_t;
        (*((mvd_dst as *mut ::core::ffi::c_char).offset(8 as ::core::ffi::c_int as isize)
            as *mut av_alias64))
            .u64_0 = 0 as uint64_t;
    } else {
        (*(mvd_dst as *mut av_alias64)).u64_0 = (*(mvd_src
            .offset((8 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
            as *const av_alias64))
            .u64_0;
        (*(mvd_dst
            .offset(3 as ::core::ffi::c_int as isize)
            .offset(3 as ::core::ffi::c_int as isize) as *mut av_alias16))
            .u16_0 = (*(mvd_src
            .offset(3 as ::core::ffi::c_int as isize)
            .offset((8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize)
            as *const av_alias16))
            .u16_0;
        (*(mvd_dst
            .offset(3 as ::core::ffi::c_int as isize)
            .offset(2 as ::core::ffi::c_int as isize) as *mut av_alias16))
            .u16_0 = (*(mvd_src
            .offset(3 as ::core::ffi::c_int as isize)
            .offset((8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int) as isize)
            as *const av_alias16))
            .u16_0;
        (*(mvd_dst
            .offset(3 as ::core::ffi::c_int as isize)
            .offset(1 as ::core::ffi::c_int as isize) as *mut av_alias16))
            .u16_0 = (*(mvd_src
            .offset(3 as ::core::ffi::c_int as isize)
            .offset((8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize)
            as *const av_alias16))
            .u16_0;
    }
    let mut ref_index: *mut int8_t = (*(&raw const (*h).cur_pic.ref_index
        as *const *mut int8_t)
        .offset(list as isize))
        .offset(b8_xy as isize) as *mut int8_t;
    let mut ref_cache: *mut int8_t = &raw mut *(&raw mut (*sl).ref_cache
        as *mut [int8_t; 40])
        .offset(list as isize) as *mut int8_t;
    *ref_index
        .offset(
            (0 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                as isize,
        ) = *ref_cache.offset(scan8[0 as ::core::ffi::c_int as usize] as isize);
    *ref_index
        .offset(
            (1 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                as isize,
        ) = *ref_cache.offset(scan8[4 as ::core::ffi::c_int as usize] as isize);
    *ref_index
        .offset(
            (0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                as isize,
        ) = *ref_cache.offset(scan8[8 as ::core::ffi::c_int as usize] as isize);
    *ref_index
        .offset(
            (1 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                as isize,
        ) = *ref_cache.offset(scan8[12 as ::core::ffi::c_int as usize] as isize);
}
#[inline]
unsafe extern "C" fn median3_c(
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
    mut c: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut max2: ::core::ffi::c_int = 0;
    let mut min2: ::core::ffi::c_int = 0;
    let mut m: ::core::ffi::c_int = 0;
    if a >= b {
        max2 = a;
        min2 = b;
    } else {
        max2 = b;
        min2 = a;
    }
    m = if c >= max2 { max2 } else { c };
    return if m >= min2 { m } else { min2 };
}
#[inline(always)]
unsafe extern "C" fn get_cabac_inline(
    mut c: *mut CABACContext,
    state: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut s: ::core::ffi::c_int = *state as ::core::ffi::c_int;
    let mut RangeLPS: ::core::ffi::c_int = *ff_h264_lps_range
        .offset(
            (2 as ::core::ffi::c_int * ((*c).range & 0xc0 as ::core::ffi::c_int) + s)
                as isize,
        ) as ::core::ffi::c_int;
    let mut bit: ::core::ffi::c_int = 0;
    let mut lps_mask: ::core::ffi::c_int = 0;
    (*c).range -= RangeLPS;
    lps_mask = ((*c).range << 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        - (*c).low >> 31 as ::core::ffi::c_int;
    (*c).low
        -= (*c).range << 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int & lps_mask;
    (*c).range += RangeLPS - (*c).range & lps_mask;
    s ^= lps_mask;
    *state = *ff_h264_mlps_state
        .offset(128 as ::core::ffi::c_int as isize)
        .offset(s as isize);
    bit = s & 1 as ::core::ffi::c_int;
    lps_mask = *ff_h264_norm_shift.offset((*c).range as isize) as ::core::ffi::c_int;
    (*c).range <<= lps_mask;
    (*c).low <<= lps_mask;
    if (*c).low
        & ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int == 0
    {
        refill2(c);
    }
    return bit;
}
#[inline(always)]
unsafe extern "C" fn pack16to32(
    mut a: ::core::ffi::c_uint,
    mut b: ::core::ffi::c_uint,
) -> uint32_t {
    return (a as uint32_t & 0xffff as uint32_t)
        .wrapping_add((b as uint32_t) << 16 as ::core::ffi::c_int);
}
#[inline(always)]
unsafe extern "C" fn write_back_motion(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut mb_type: ::core::ffi::c_int,
) {
    let b_stride: ::core::ffi::c_int = (*h).b_stride;
    let b_xy: ::core::ffi::c_int = 4 as ::core::ffi::c_int * (*sl).mb_x
        + 4 as ::core::ffi::c_int * (*sl).mb_y * (*h).b_stride;
    let b8_xy: ::core::ffi::c_int = 4 as ::core::ffi::c_int * (*sl).mb_xy;
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int != 0
    {
        write_back_motion_list(
            h,
            sl,
            b_stride,
            b_xy,
            b8_xy,
            mb_type,
            0 as ::core::ffi::c_int,
        );
    } else {
        fill_rectangle(
            (*(&raw const (*h).cur_pic.ref_index as *const *mut int8_t)
                .offset(0 as ::core::ffi::c_int as isize))
                .offset(b8_xy as isize) as *mut int8_t as *mut ::core::ffi::c_void,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            -(1 as ::core::ffi::c_int) as uint8_t as uint32_t,
            1 as ::core::ffi::c_int,
        );
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 1 as ::core::ffi::c_int != 0
    {
        write_back_motion_list(
            h,
            sl,
            b_stride,
            b_xy,
            b8_xy,
            mb_type,
            1 as ::core::ffi::c_int,
        );
    }
    if (*sl).slice_type_nos == AV_PICTURE_TYPE_B as ::core::ffi::c_int
        && 1 as ::core::ffi::c_int != 0
    {
        if mb_type & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0 {
            let mut direct_table: *mut uint8_t = (*h)
                .direct_table
                .offset((4 as ::core::ffi::c_int * (*sl).mb_xy) as isize)
                as *mut uint8_t;
            *direct_table.offset(1 as ::core::ffi::c_int as isize) = ((*sl)
                .sub_mb_type[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *direct_table.offset(2 as ::core::ffi::c_int as isize) = ((*sl)
                .sub_mb_type[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
            *direct_table.offset(3 as ::core::ffi::c_int as isize) = ((*sl)
                .sub_mb_type[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int) as uint8_t;
        }
    }
}
#[inline(never)]
unsafe extern "C" fn get_cabac_noinline(
    mut c: *mut CABACContext,
    state: *mut uint8_t,
) -> ::core::ffi::c_int {
    return get_cabac_inline(c, state);
}
unsafe extern "C" fn get_cabac(
    mut c: *mut CABACContext,
    state: *mut uint8_t,
) -> ::core::ffi::c_int {
    return get_cabac_inline(c, state);
}
unsafe extern "C" fn get_cabac_bypass(mut c: *mut CABACContext) -> ::core::ffi::c_int {
    let mut range: ::core::ffi::c_int = 0;
    (*c).low += (*c).low;
    if (*c).low
        & ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int == 0
    {
        refill(c);
    }
    range = (*c).range << 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    if (*c).low < range {
        return 0 as ::core::ffi::c_int
    } else {
        (*c).low -= range;
        return 1 as ::core::ffi::c_int;
    };
}
#[inline(always)]
unsafe extern "C" fn get_dct8x8_allowed(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) -> ::core::ffi::c_int {
    if (*(*h).ps.sps).direct_8x8_inference_flag != 0 {
        return ((*(&raw mut (*sl).sub_mb_type as *mut uint16_t as *const av_alias64))
            .u64_0 as ::core::ffi::c_ulonglong
            & (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int)
                as ::core::ffi::c_ulonglong)
                .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) == 0)
            as ::core::ffi::c_int
    } else {
        return ((*(&raw mut (*sl).sub_mb_type as *mut uint16_t as *const av_alias64))
            .u64_0 as ::core::ffi::c_ulonglong
            & (((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as ::core::ffi::c_ulonglong)
                .wrapping_mul(0x1000100010001 as ::core::ffi::c_ulonglong) == 0)
            as ::core::ffi::c_int
    };
}
#[inline(always)]
unsafe extern "C" fn get_cabac_bypass_sign(
    mut c: *mut CABACContext,
    mut val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut range: ::core::ffi::c_int = 0;
    let mut mask: ::core::ffi::c_int = 0;
    (*c).low += (*c).low;
    if (*c).low
        & ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int == 0
    {
        refill(c);
    }
    range = (*c).range << 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
    (*c).low -= range;
    mask = (*c).low >> 31 as ::core::ffi::c_int;
    range &= mask;
    (*c).low += range;
    return (val ^ mask) - mask;
}
#[inline(always)]
unsafe extern "C" fn fetch_diagonal_mv(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut C: *mut *const int16_t,
    mut i: ::core::ffi::c_int,
    mut list: ::core::ffi::c_int,
    mut part_width: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let topright_ref: ::core::ffi::c_int = (*sl)
        .ref_cache[list as usize][(i - 8 as ::core::ffi::c_int + part_width) as usize]
        as ::core::ffi::c_int;
    if (*h).mb_aff_frame != 0 {
        if topright_ref == -(2 as ::core::ffi::c_int)
            && i
                >= scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
            && i & 7 as ::core::ffi::c_int == 4 as ::core::ffi::c_int
            && (*sl)
                .ref_cache[list
                as usize][(scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                != -(2 as ::core::ffi::c_int)
        {
            let mut mb_types: *const uint32_t = (*(*h).cur_pic_ptr).mb_type;
            let mut mv: *const int16_t = ::core::ptr::null::<int16_t>();
            (*(&raw mut *(&raw mut *(&raw mut (*sl).mv_cache as *mut [[int16_t; 2]; 40])
                .offset(list as isize) as *mut [int16_t; 2])
                .offset(
                    (*(&raw const scan8 as *const uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - 2 as ::core::ffi::c_int) as isize,
                ) as *mut int16_t as *mut av_alias32))
                .u32_0 = 0 as uint32_t;
            *C = &raw mut *(&raw mut *(&raw mut (*sl).mv_cache
                as *mut [[int16_t; 2]; 40])
                .offset(list as isize) as *mut [int16_t; 2])
                .offset(
                    (*(&raw const scan8 as *const uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        - 2 as ::core::ffi::c_int) as isize,
                ) as *mut int16_t;
            if (*sl).mb_field_decoding_flag == 0
                && (*sl).left_type[0 as ::core::ffi::c_int as usize]
                    & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
            {
                let xy: ::core::ffi::c_int = (*sl)
                    .left_mb_xy[0 as ::core::ffi::c_int as usize] + (*h).mb_stride;
                let y4: ::core::ffi::c_int = ((*sl).mb_y & 1 as ::core::ffi::c_int)
                    * 2 as ::core::ffi::c_int + (i >> 5 as ::core::ffi::c_int);
                let mb_type: ::core::ffi::c_int = *mb_types
                    .offset(
                        (xy + (y4 >> 2 as ::core::ffi::c_int) * (*h).mb_stride) as isize,
                    ) as ::core::ffi::c_int;
                if mb_type
                    & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int * list == 0
                {
                    return -(1 as ::core::ffi::c_int);
                }
                mv = &raw mut *(*(&raw mut (*(*h).cur_pic_ptr).motion_val
                    as *mut *mut [int16_t; 2])
                    .offset(list as isize))
                    .offset(
                        (*(*h).mb2b_xy.offset(xy as isize))
                            .wrapping_add(3 as uint32_t)
                            .wrapping_add((y4 * (*h).b_stride) as uint32_t) as isize,
                    ) as *mut int16_t;
                (*sl)
                    .mv_cache[list
                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as usize][0 as ::core::ffi::c_int as usize] = *mv
                    .offset(0 as ::core::ffi::c_int as isize);
                (*sl)
                    .mv_cache[list
                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as usize][1 as ::core::ffi::c_int as usize] = (*mv
                    .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    * 2 as ::core::ffi::c_int) as int16_t;
                return *(*(*h).cur_pic_ptr)
                    .ref_index[list as usize]
                    .offset(
                        (4 as ::core::ffi::c_int * xy + 1 as ::core::ffi::c_int
                            + (y4 & !(1 as ::core::ffi::c_int))) as isize,
                    ) as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
            }
            if (*sl).mb_field_decoding_flag != 0
                && (*sl).left_type[0 as ::core::ffi::c_int as usize]
                    & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
            {
                let xy_0: ::core::ffi::c_int = (*sl)
                    .left_mb_xy[(i >= 36 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as usize];
                let y4_0: ::core::ffi::c_int = i >> 2 as ::core::ffi::c_int
                    & 3 as ::core::ffi::c_int;
                let mb_type_0: ::core::ffi::c_int = *mb_types
                    .offset(
                        (xy_0 + (y4_0 >> 2 as ::core::ffi::c_int) * (*h).mb_stride)
                            as isize,
                    ) as ::core::ffi::c_int;
                if mb_type_0
                    & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int * list == 0
                {
                    return -(1 as ::core::ffi::c_int);
                }
                mv = &raw mut *(*(&raw mut (*(*h).cur_pic_ptr).motion_val
                    as *mut *mut [int16_t; 2])
                    .offset(list as isize))
                    .offset(
                        (*(*h).mb2b_xy.offset(xy_0 as isize))
                            .wrapping_add(3 as uint32_t)
                            .wrapping_add((y4_0 * (*h).b_stride) as uint32_t) as isize,
                    ) as *mut int16_t;
                (*sl)
                    .mv_cache[list
                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as usize][0 as ::core::ffi::c_int as usize] = *mv
                    .offset(0 as ::core::ffi::c_int as isize);
                (*sl)
                    .mv_cache[list
                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int - 2 as ::core::ffi::c_int)
                    as usize][1 as ::core::ffi::c_int as usize] = (*mv
                    .offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    / 2 as ::core::ffi::c_int) as int16_t;
                return *(*(*h).cur_pic_ptr)
                    .ref_index[list as usize]
                    .offset(
                        (4 as ::core::ffi::c_int * xy_0 + 1 as ::core::ffi::c_int
                            + (y4_0 & !(1 as ::core::ffi::c_int))) as isize,
                    ) as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
            }
        }
    }
    if topright_ref != -(2 as ::core::ffi::c_int) {
        *C = &raw mut *(&raw mut *(&raw mut (*sl).mv_cache as *mut [[int16_t; 2]; 40])
            .offset(list as isize) as *mut [int16_t; 2])
            .offset((i - 8 as ::core::ffi::c_int + part_width) as isize) as *mut int16_t;
        return topright_ref;
    } else {
        *C = &raw mut *(&raw mut *(&raw mut (*sl).mv_cache as *mut [[int16_t; 2]; 40])
            .offset(list as isize) as *mut [int16_t; 2])
            .offset((i - 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize)
            as *mut int16_t;
        return (*sl)
            .ref_cache[list
            as usize][(i - 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize]
            as ::core::ffi::c_int;
    };
}
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: ::core::ffi::c_int,
    mut amin: ::core::ffi::c_int,
    mut amax: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if a < amin { return amin } else if a > amax { return amax } else { return a };
}
unsafe extern "C" fn get_cabac_terminate(
    mut c: *mut CABACContext,
) -> ::core::ffi::c_int {
    (*c).range -= 2 as ::core::ffi::c_int;
    if (*c).low < (*c).range << 16 as ::core::ffi::c_int + 1 as ::core::ffi::c_int {
        renorm_cabac_decoder_once(c);
        return 0 as ::core::ffi::c_int;
    } else {
        return (*c).bytestream.offset_from((*c).bytestream_start) as ::core::ffi::c_long
            as ::core::ffi::c_int
    };
}
#[inline(always)]
unsafe extern "C" fn pred_motion(
    h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut part_width: ::core::ffi::c_int,
    mut list: ::core::ffi::c_int,
    mut ref_0: ::core::ffi::c_int,
    mx: *mut ::core::ffi::c_int,
    my: *mut ::core::ffi::c_int,
) {
    let index8: ::core::ffi::c_int = scan8[n as usize] as ::core::ffi::c_int;
    let top_ref: ::core::ffi::c_int = (*sl)
        .ref_cache[list as usize][(index8 - 8 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let left_ref: ::core::ffi::c_int = (*sl)
        .ref_cache[list as usize][(index8 - 1 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_int;
    let A: *const int16_t = &raw mut *(&raw mut *(&raw mut (*sl).mv_cache
        as *mut [[int16_t; 2]; 40])
        .offset(list as isize) as *mut [int16_t; 2])
        .offset((index8 - 1 as ::core::ffi::c_int) as isize) as *mut int16_t;
    let B: *const int16_t = &raw mut *(&raw mut *(&raw mut (*sl).mv_cache
        as *mut [[int16_t; 2]; 40])
        .offset(list as isize) as *mut [int16_t; 2])
        .offset((index8 - 8 as ::core::ffi::c_int) as isize) as *mut int16_t;
    let mut C: *const int16_t = ::core::ptr::null::<int16_t>();
    let mut diagonal_ref: ::core::ffi::c_int = 0;
    let mut match_count: ::core::ffi::c_int = 0;
    diagonal_ref = fetch_diagonal_mv(h, sl, &raw mut C, index8, list, part_width);
    match_count = (diagonal_ref == ref_0) as ::core::ffi::c_int
        + (top_ref == ref_0) as ::core::ffi::c_int
        + (left_ref == ref_0) as ::core::ffi::c_int;
    if match_count > 1 as ::core::ffi::c_int {
        *mx = median3_c(
            *A.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *B.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *C.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        );
        *my = median3_c(
            *A.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *B.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *C.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        );
    } else if match_count == 1 as ::core::ffi::c_int {
        if left_ref == ref_0 {
            *mx = *A.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            *my = *A.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        } else if top_ref == ref_0 {
            *mx = *B.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            *my = *B.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        } else {
            *mx = *C.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            *my = *C.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        }
    } else if top_ref == -(2 as ::core::ffi::c_int)
        && diagonal_ref == -(2 as ::core::ffi::c_int)
        && left_ref != -(2 as ::core::ffi::c_int)
    {
        *mx = *A.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
        *my = *A.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    } else {
        *mx = median3_c(
            *A.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *B.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *C.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        );
        *my = median3_c(
            *A.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *B.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *C.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
        );
    };
}
#[inline(always)]
unsafe extern "C" fn pred_16x8_motion(
    h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut list: ::core::ffi::c_int,
    mut ref_0: ::core::ffi::c_int,
    mx: *mut ::core::ffi::c_int,
    my: *mut ::core::ffi::c_int,
) {
    if n == 0 as ::core::ffi::c_int {
        let top_ref: ::core::ffi::c_int = (*sl)
            .ref_cache[list
            as usize][(scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            - 8 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
        let B: *const int16_t = &raw mut *(&raw mut *(&raw mut (*sl).mv_cache
            as *mut [[int16_t; 2]; 40])
            .offset(list as isize) as *mut [int16_t; 2])
            .offset(
                (*(&raw const scan8 as *const uint8_t)
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - 8 as ::core::ffi::c_int) as isize,
            ) as *mut int16_t;
        if top_ref == ref_0 {
            *mx = *B.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            *my = *B.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            return;
        }
    } else {
        let left_ref: ::core::ffi::c_int = (*sl)
            .ref_cache[list
            as usize][(scan8[8 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
        let A: *const int16_t = &raw mut *(&raw mut *(&raw mut (*sl).mv_cache
            as *mut [[int16_t; 2]; 40])
            .offset(list as isize) as *mut [int16_t; 2])
            .offset(
                (*(&raw const scan8 as *const uint8_t)
                    .offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as isize,
            ) as *mut int16_t;
        if left_ref == ref_0 {
            *mx = *A.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            *my = *A.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            return;
        }
    }
    pred_motion(h, sl, n, 4 as ::core::ffi::c_int, list, ref_0, mx, my);
}
#[inline(always)]
unsafe extern "C" fn pred_8x16_motion(
    h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut n: ::core::ffi::c_int,
    mut list: ::core::ffi::c_int,
    mut ref_0: ::core::ffi::c_int,
    mx: *mut ::core::ffi::c_int,
    my: *mut ::core::ffi::c_int,
) {
    if n == 0 as ::core::ffi::c_int {
        let left_ref: ::core::ffi::c_int = (*sl)
            .ref_cache[list
            as usize][(scan8[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
        let A: *const int16_t = &raw mut *(&raw mut *(&raw mut (*sl).mv_cache
            as *mut [[int16_t; 2]; 40])
            .offset(list as isize) as *mut [int16_t; 2])
            .offset(
                (*(&raw const scan8 as *const uint8_t)
                    .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as isize,
            ) as *mut int16_t;
        if left_ref == ref_0 {
            *mx = *A.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            *my = *A.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            return;
        }
    } else {
        let mut C: *const int16_t = ::core::ptr::null::<int16_t>();
        let mut diagonal_ref: ::core::ffi::c_int = 0;
        diagonal_ref = fetch_diagonal_mv(
            h,
            sl,
            &raw mut C,
            scan8[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            list,
            2 as ::core::ffi::c_int,
        );
        if diagonal_ref == ref_0 {
            *mx = *C.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            *my = *C.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
            return;
        }
    }
    pred_motion(h, sl, n, 2 as ::core::ffi::c_int, list, ref_0, mx, my);
}
static mut cabac_context_init_PB: [[[int8_t; 2]; 1024]; 3] = [
    [
        [20 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, 2 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [-(37 as ::core::ffi::c_int) as int8_t, 118 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [16 as ::core::ffi::c_int as int8_t, 90 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(46 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 88 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 88 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 97 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 126 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 98 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 101 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
        [-(29 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 108 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 126 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 24 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [16 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [25 as ::core::ffi::c_int as int8_t, 7 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, -(7 as ::core::ffi::c_int) as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 4 as ::core::ffi::c_int as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [16 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 32 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 108 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 24 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(30 as ::core::ffi::c_int) as int8_t, 119 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 5 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 14 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 18 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 114 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 98 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 103 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 107 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 108 as ::core::ffi::c_int as int8_t],
        [-(26 as ::core::ffi::c_int) as int8_t, 112 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 32 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 14 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(21 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(23 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(33 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(31 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [45 as ::core::ffi::c_int as int8_t, -(24 as ::core::ffi::c_int) as int8_t],
        [53 as ::core::ffi::c_int as int8_t, -(45 as ::core::ffi::c_int) as int8_t],
        [48 as ::core::ffi::c_int as int8_t, -(26 as ::core::ffi::c_int) as int8_t],
        [65 as ::core::ffi::c_int as int8_t, -(43 as ::core::ffi::c_int) as int8_t],
        [43 as ::core::ffi::c_int as int8_t, -(19 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 26 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(19 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
        [-(35 as ::core::ffi::c_int) as int8_t, 125 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, -(4 as ::core::ffi::c_int) as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 8 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
        [33 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 32 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [16 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 24 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 32 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 24 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 32 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(19 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(19 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [26 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(7 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [45 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
        [49 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [45 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [36 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(7 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [51 as ::core::ffi::c_int as int8_t, 2 as ::core::ffi::c_int as int8_t],
        [60 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
        [61 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
        [55 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [42 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 108 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 126 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 108 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 126 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 24 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 24 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [25 as ::core::ffi::c_int as int8_t, 7 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, -(7 as ::core::ffi::c_int) as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 4 as ::core::ffi::c_int as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [25 as ::core::ffi::c_int as int8_t, 7 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, -(7 as ::core::ffi::c_int) as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 4 as ::core::ffi::c_int as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(19 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(19 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [26 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(7 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [45 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
        [49 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [45 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [36 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(7 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [51 as ::core::ffi::c_int as int8_t, 2 as ::core::ffi::c_int as int8_t],
        [60 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
        [61 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
        [55 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [42 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(19 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(19 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [26 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(7 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [45 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
        [49 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [45 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [36 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(7 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [51 as ::core::ffi::c_int as int8_t, 2 as ::core::ffi::c_int as int8_t],
        [60 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
        [61 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
        [55 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [42 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 114 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 98 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 103 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 107 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 108 as ::core::ffi::c_int as int8_t],
        [-(26 as ::core::ffi::c_int) as int8_t, 112 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 114 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 98 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 103 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 107 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 108 as ::core::ffi::c_int as int8_t],
        [-(26 as ::core::ffi::c_int) as int8_t, 112 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 14 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(21 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(23 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(33 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(31 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [45 as ::core::ffi::c_int as int8_t, -(24 as ::core::ffi::c_int) as int8_t],
        [53 as ::core::ffi::c_int as int8_t, -(45 as ::core::ffi::c_int) as int8_t],
        [48 as ::core::ffi::c_int as int8_t, -(26 as ::core::ffi::c_int) as int8_t],
        [65 as ::core::ffi::c_int as int8_t, -(43 as ::core::ffi::c_int) as int8_t],
        [43 as ::core::ffi::c_int as int8_t, -(19 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 26 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(19 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
        [-(35 as ::core::ffi::c_int) as int8_t, 125 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, -(4 as ::core::ffi::c_int) as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 8 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
        [33 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 32 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [16 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 14 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(21 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(23 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(33 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(31 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [45 as ::core::ffi::c_int as int8_t, -(24 as ::core::ffi::c_int) as int8_t],
        [53 as ::core::ffi::c_int as int8_t, -(45 as ::core::ffi::c_int) as int8_t],
        [48 as ::core::ffi::c_int as int8_t, -(26 as ::core::ffi::c_int) as int8_t],
        [65 as ::core::ffi::c_int as int8_t, -(43 as ::core::ffi::c_int) as int8_t],
        [43 as ::core::ffi::c_int as int8_t, -(19 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 26 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(19 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
        [-(35 as ::core::ffi::c_int) as int8_t, 125 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, -(4 as ::core::ffi::c_int) as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 6 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 8 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
        [33 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 32 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [16 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 24 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(30 as ::core::ffi::c_int) as int8_t, 119 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 24 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(30 as ::core::ffi::c_int) as int8_t, 119 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 126 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 126 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 126 as ::core::ffi::c_int as int8_t],
    ],
    [
        [20 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [16 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [-(29 as ::core::ffi::c_int) as int8_t, 118 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
        [40 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [57 as ::core::ffi::c_int as int8_t, 2 as ::core::ffi::c_int as int8_t],
        [41 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(45 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 101 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 97 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(39 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 96 as ::core::ffi::c_int as int8_t],
        [-(26 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(35 as ::core::ffi::c_int) as int8_t, 98 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 119 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 99 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [-(36 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [-(35 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(31 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 103 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 105 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 112 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 99 as ::core::ffi::c_int as int8_t],
        [-(78 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(70 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(50 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(46 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(34 as ::core::ffi::c_int) as int8_t, 119 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
        [-(44 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 32 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 46 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(11 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(43 as ::core::ffi::c_int) as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [35 as ::core::ffi::c_int as int8_t, -(22 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(18 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [102 as ::core::ffi::c_int as int8_t, -(94 as ::core::ffi::c_int) as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [56 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(4 as ::core::ffi::c_int) as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [51 as ::core::ffi::c_int as int8_t, -(29 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(34 as ::core::ffi::c_int) as int8_t],
        [69 as ::core::ffi::c_int as int8_t, -(58 as ::core::ffi::c_int) as int8_t],
        [67 as ::core::ffi::c_int as int8_t, -(63 as ::core::ffi::c_int) as int8_t],
        [44 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 7 as ::core::ffi::c_int as int8_t],
        [55 as ::core::ffi::c_int as int8_t, -(29 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [36 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(27 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(18 as ::core::ffi::c_int) as int8_t],
        [35 as ::core::ffi::c_int as int8_t, -(16 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(8 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(6 as ::core::ffi::c_int) as int8_t],
        [35 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 18 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 26 as ::core::ffi::c_int as int8_t],
        [16 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [16 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 116 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 112 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(34 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 101 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 107 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 111 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 122 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 32 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 4 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 8 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 8 as ::core::ffi::c_int as int8_t],
        [16 as ::core::ffi::c_int as int8_t, 19 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 126 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 124 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(26 as ::core::ffi::c_int) as int8_t, 126 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 124 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 105 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 121 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [-(26 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 116 as ::core::ffi::c_int as int8_t],
        [-(33 as ::core::ffi::c_int) as int8_t, 122 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 111 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 114 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [16 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, -(6 as ::core::ffi::c_int) as int8_t],
        [18 as ::core::ffi::c_int as int8_t, -(6 as ::core::ffi::c_int) as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [31 as ::core::ffi::c_int as int8_t, -(16 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(22 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [47 as ::core::ffi::c_int as int8_t, -(42 as ::core::ffi::c_int) as int8_t],
        [45 as ::core::ffi::c_int as int8_t, -(36 as ::core::ffi::c_int) as int8_t],
        [49 as ::core::ffi::c_int as int8_t, -(34 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [69 as ::core::ffi::c_int as int8_t, -(71 as ::core::ffi::c_int) as int8_t],
        [63 as ::core::ffi::c_int as int8_t, -(63 as ::core::ffi::c_int) as int8_t],
        [66 as ::core::ffi::c_int as int8_t, -(64 as ::core::ffi::c_int) as int8_t],
        [77 as ::core::ffi::c_int as int8_t, -(74 as ::core::ffi::c_int) as int8_t],
        [54 as ::core::ffi::c_int as int8_t, -(39 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(35 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [36 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [40 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 14 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 26 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(33 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(36 as ::core::ffi::c_int) as int8_t],
        [40 as ::core::ffi::c_int as int8_t, -(37 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(33 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [40 as ::core::ffi::c_int as int8_t, -(24 as ::core::ffi::c_int) as int8_t],
        [49 as ::core::ffi::c_int as int8_t, -(29 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [40 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(3 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
        [25 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 21 as ::core::ffi::c_int as int8_t],
        [33 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 24 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [25 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [25 as ::core::ffi::c_int as int8_t, 32 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [49 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [53 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [64 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
        [68 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [66 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [47 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 24 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [49 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [53 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [64 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
        [68 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [66 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [47 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [-(35 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(31 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [-(35 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(31 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 103 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 105 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 112 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 99 as ::core::ffi::c_int as int8_t],
        [-(78 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(70 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(50 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(46 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(34 as ::core::ffi::c_int) as int8_t, 119 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
        [-(44 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 103 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 105 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 112 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 99 as ::core::ffi::c_int as int8_t],
        [-(78 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(70 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(50 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(46 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(34 as ::core::ffi::c_int) as int8_t, 119 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
        [-(44 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(11 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(43 as ::core::ffi::c_int) as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [35 as ::core::ffi::c_int as int8_t, -(22 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(18 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [102 as ::core::ffi::c_int as int8_t, -(94 as ::core::ffi::c_int) as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [56 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(4 as ::core::ffi::c_int) as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [51 as ::core::ffi::c_int as int8_t, -(29 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(34 as ::core::ffi::c_int) as int8_t],
        [69 as ::core::ffi::c_int as int8_t, -(58 as ::core::ffi::c_int) as int8_t],
        [67 as ::core::ffi::c_int as int8_t, -(63 as ::core::ffi::c_int) as int8_t],
        [44 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 7 as ::core::ffi::c_int as int8_t],
        [55 as ::core::ffi::c_int as int8_t, -(29 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [36 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(27 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(18 as ::core::ffi::c_int) as int8_t],
        [35 as ::core::ffi::c_int as int8_t, -(16 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(8 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(6 as ::core::ffi::c_int) as int8_t],
        [35 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 18 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(11 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(43 as ::core::ffi::c_int) as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [35 as ::core::ffi::c_int as int8_t, -(22 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(18 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [102 as ::core::ffi::c_int as int8_t, -(94 as ::core::ffi::c_int) as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [56 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(4 as ::core::ffi::c_int) as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [51 as ::core::ffi::c_int as int8_t, -(29 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(34 as ::core::ffi::c_int) as int8_t],
        [69 as ::core::ffi::c_int as int8_t, -(58 as ::core::ffi::c_int) as int8_t],
        [67 as ::core::ffi::c_int as int8_t, -(63 as ::core::ffi::c_int) as int8_t],
        [44 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 7 as ::core::ffi::c_int as int8_t],
        [55 as ::core::ffi::c_int as int8_t, -(29 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [36 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(27 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(18 as ::core::ffi::c_int) as int8_t],
        [35 as ::core::ffi::c_int as int8_t, -(16 as ::core::ffi::c_int) as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(8 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(6 as ::core::ffi::c_int) as int8_t],
        [35 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 18 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [49 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [53 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [64 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
        [68 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [66 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [47 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [49 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [53 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [64 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
        [68 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [66 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [47 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 24 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [49 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [53 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [64 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
        [68 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [66 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [47 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [49 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [53 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [64 as ::core::ffi::c_int as int8_t, 3 as ::core::ffi::c_int as int8_t],
        [68 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [66 as ::core::ffi::c_int as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [47 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 24 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 126 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 124 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(26 as ::core::ffi::c_int) as int8_t, 126 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 124 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 105 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 121 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [-(26 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 116 as ::core::ffi::c_int as int8_t],
        [-(33 as ::core::ffi::c_int) as int8_t, 122 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 111 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 114 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 126 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 124 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 110 as ::core::ffi::c_int as int8_t],
        [-(26 as ::core::ffi::c_int) as int8_t, 126 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 124 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 105 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 121 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [-(26 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 116 as ::core::ffi::c_int as int8_t],
        [-(33 as ::core::ffi::c_int) as int8_t, 122 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 111 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 114 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, -(6 as ::core::ffi::c_int) as int8_t],
        [18 as ::core::ffi::c_int as int8_t, -(6 as ::core::ffi::c_int) as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [31 as ::core::ffi::c_int as int8_t, -(16 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(22 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [47 as ::core::ffi::c_int as int8_t, -(42 as ::core::ffi::c_int) as int8_t],
        [45 as ::core::ffi::c_int as int8_t, -(36 as ::core::ffi::c_int) as int8_t],
        [49 as ::core::ffi::c_int as int8_t, -(34 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [69 as ::core::ffi::c_int as int8_t, -(71 as ::core::ffi::c_int) as int8_t],
        [63 as ::core::ffi::c_int as int8_t, -(63 as ::core::ffi::c_int) as int8_t],
        [66 as ::core::ffi::c_int as int8_t, -(64 as ::core::ffi::c_int) as int8_t],
        [77 as ::core::ffi::c_int as int8_t, -(74 as ::core::ffi::c_int) as int8_t],
        [54 as ::core::ffi::c_int as int8_t, -(39 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(35 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [36 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [40 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 14 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 26 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(33 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(36 as ::core::ffi::c_int) as int8_t],
        [40 as ::core::ffi::c_int as int8_t, -(37 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(33 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [40 as ::core::ffi::c_int as int8_t, -(24 as ::core::ffi::c_int) as int8_t],
        [49 as ::core::ffi::c_int as int8_t, -(29 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [40 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(3 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
        [25 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, -(6 as ::core::ffi::c_int) as int8_t],
        [18 as ::core::ffi::c_int as int8_t, -(6 as ::core::ffi::c_int) as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [31 as ::core::ffi::c_int as int8_t, -(16 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(22 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(28 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [47 as ::core::ffi::c_int as int8_t, -(42 as ::core::ffi::c_int) as int8_t],
        [45 as ::core::ffi::c_int as int8_t, -(36 as ::core::ffi::c_int) as int8_t],
        [49 as ::core::ffi::c_int as int8_t, -(34 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 9 as ::core::ffi::c_int as int8_t],
        [69 as ::core::ffi::c_int as int8_t, -(71 as ::core::ffi::c_int) as int8_t],
        [63 as ::core::ffi::c_int as int8_t, -(63 as ::core::ffi::c_int) as int8_t],
        [66 as ::core::ffi::c_int as int8_t, -(64 as ::core::ffi::c_int) as int8_t],
        [77 as ::core::ffi::c_int as int8_t, -(74 as ::core::ffi::c_int) as int8_t],
        [54 as ::core::ffi::c_int as int8_t, -(39 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(35 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [36 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [40 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 14 as ::core::ffi::c_int as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 26 as ::core::ffi::c_int as int8_t],
        [23 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(33 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(36 as ::core::ffi::c_int) as int8_t],
        [40 as ::core::ffi::c_int as int8_t, -(37 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(33 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [40 as ::core::ffi::c_int as int8_t, -(24 as ::core::ffi::c_int) as int8_t],
        [49 as ::core::ffi::c_int as int8_t, -(29 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [40 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [38 as ::core::ffi::c_int as int8_t, -(3 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(5 as ::core::ffi::c_int) as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
        [25 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 112 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(34 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 101 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 107 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 111 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 122 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 112 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [-(34 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 101 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 107 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 111 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 122 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(31 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(31 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(31 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    ],
    [
        [20 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, -(15 as ::core::ffi::c_int) as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
        [25 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 99 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [54 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 97 as ::core::ffi::c_int as int8_t],
        [-(32 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 93 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 103 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 116 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 96 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 101 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 97 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 97 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(36 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 95 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(31 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 103 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
        [-(37 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(37 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(30 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 98 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(29 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(20 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 7 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 23 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [16 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 124 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [44 as ::core::ffi::c_int as int8_t, -(18 as ::core::ffi::c_int) as int8_t],
        [50 as ::core::ffi::c_int as int8_t, -(34 as ::core::ffi::c_int) as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [35 as ::core::ffi::c_int as int8_t, -(18 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
        [28 as ::core::ffi::c_int as int8_t, -(3 as ::core::ffi::c_int) as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(44 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(24 as ::core::ffi::c_int) as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [36 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [25 as ::core::ffi::c_int as int8_t, 21 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 12 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 93 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [26 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 115 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(29 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 98 as ::core::ffi::c_int as int8_t],
        [-(37 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 108 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 99 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 87 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 120 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(19 as ::core::ffi::c_int) as int8_t, 114 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 118 as ::core::ffi::c_int as int8_t],
        [-(26 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 113 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 118 as ::core::ffi::c_int as int8_t],
        [-(31 as ::core::ffi::c_int) as int8_t, 120 as ::core::ffi::c_int as int8_t],
        [-(37 as ::core::ffi::c_int) as int8_t, 124 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 99 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
        [-(50 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [25 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
        [16 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [17 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [27 as ::core::ffi::c_int as int8_t, -(21 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(40 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(41 as ::core::ffi::c_int) as int8_t],
        [48 as ::core::ffi::c_int as int8_t, -(47 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(32 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(40 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(51 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(41 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(39 as ::core::ffi::c_int) as int8_t],
        [43 as ::core::ffi::c_int as int8_t, -(19 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
        [61 as ::core::ffi::c_int as int8_t, -(55 as ::core::ffi::c_int) as int8_t],
        [56 as ::core::ffi::c_int as int8_t, -(46 as ::core::ffi::c_int) as int8_t],
        [62 as ::core::ffi::c_int as int8_t, -(50 as ::core::ffi::c_int) as int8_t],
        [81 as ::core::ffi::c_int as int8_t, -(67 as ::core::ffi::c_int) as int8_t],
        [45 as ::core::ffi::c_int as int8_t, -(20 as ::core::ffi::c_int) as int8_t],
        [35 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
        [39 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [36 as ::core::ffi::c_int as int8_t, -(16 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
        [35 as ::core::ffi::c_int as int8_t, 18 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [33 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [36 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 26 as ::core::ffi::c_int as int8_t],
        [36 as ::core::ffi::c_int as int8_t, 24 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 30 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [16 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 123 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [25 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 50 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [31 as ::core::ffi::c_int as int8_t, -(4 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, 7 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 12 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [31 as ::core::ffi::c_int as int8_t, -(4 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, 7 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 12 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(37 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(30 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 84 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(37 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(30 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(29 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 51 as ::core::ffi::c_int as int8_t],
        [-(4 as ::core::ffi::c_int) as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 76 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 90 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 80 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(29 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 35 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 27 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [35 as ::core::ffi::c_int as int8_t, -(18 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
        [28 as ::core::ffi::c_int as int8_t, -(3 as ::core::ffi::c_int) as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(44 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(24 as ::core::ffi::c_int) as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [36 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 39 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 34 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 31 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 40 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 43 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 36 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [3 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [6 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [11 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 42 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [13 as ::core::ffi::c_int as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 58 as ::core::ffi::c_int as int8_t],
        [10 as ::core::ffi::c_int as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [12 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 69 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
        [8 as ::core::ffi::c_int as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [35 as ::core::ffi::c_int as int8_t, -(18 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(25 as ::core::ffi::c_int) as int8_t],
        [28 as ::core::ffi::c_int as int8_t, -(3 as ::core::ffi::c_int) as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 10 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 0 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(44 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(24 as ::core::ffi::c_int) as int8_t],
        [19 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [36 as ::core::ffi::c_int as int8_t, 29 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 33 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 20 as ::core::ffi::c_int as int8_t],
        [22 as ::core::ffi::c_int as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [31 as ::core::ffi::c_int as int8_t, -(4 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, 7 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 12 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [31 as ::core::ffi::c_int as int8_t, -(4 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, 7 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 12 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(3 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 74 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(17 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 72 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 52 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [31 as ::core::ffi::c_int as int8_t, -(4 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, 7 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 12 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [9 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [30 as ::core::ffi::c_int as int8_t, -(10 as ::core::ffi::c_int) as int8_t],
        [31 as ::core::ffi::c_int as int8_t, -(4 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, -(1 as ::core::ffi::c_int) as int8_t],
        [33 as ::core::ffi::c_int as int8_t, 7 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 12 as ::core::ffi::c_int as int8_t],
        [37 as ::core::ffi::c_int as int8_t, 23 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 37 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 49 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 56 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 63 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(6 as ::core::ffi::c_int) as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 120 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(19 as ::core::ffi::c_int) as int8_t, 114 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 118 as ::core::ffi::c_int as int8_t],
        [-(26 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 113 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 118 as ::core::ffi::c_int as int8_t],
        [-(31 as ::core::ffi::c_int) as int8_t, 120 as ::core::ffi::c_int as int8_t],
        [-(37 as ::core::ffi::c_int) as int8_t, 124 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 99 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
        [-(50 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 120 as ::core::ffi::c_int as int8_t],
        [-(27 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(19 as ::core::ffi::c_int) as int8_t, 114 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
        [-(25 as ::core::ffi::c_int) as int8_t, 118 as ::core::ffi::c_int as int8_t],
        [-(26 as ::core::ffi::c_int) as int8_t, 117 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 113 as ::core::ffi::c_int as int8_t],
        [-(28 as ::core::ffi::c_int) as int8_t, 118 as ::core::ffi::c_int as int8_t],
        [-(31 as ::core::ffi::c_int) as int8_t, 120 as ::core::ffi::c_int as int8_t],
        [-(37 as ::core::ffi::c_int) as int8_t, 124 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 102 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 99 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 106 as ::core::ffi::c_int as int8_t],
        [-(50 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 86 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(2 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(1 as ::core::ffi::c_int) as int8_t, 73 as ::core::ffi::c_int as int8_t],
        [4 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 81 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [1 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 64 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 68 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 78 as ::core::ffi::c_int as int8_t],
        [7 as ::core::ffi::c_int as int8_t, 55 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 65 as ::core::ffi::c_int as int8_t],
        [14 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 44 as ::core::ffi::c_int as int8_t],
        [5 as ::core::ffi::c_int as int8_t, 60 as ::core::ffi::c_int as int8_t],
        [2 as ::core::ffi::c_int as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
        [16 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [17 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [27 as ::core::ffi::c_int as int8_t, -(21 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(40 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(41 as ::core::ffi::c_int) as int8_t],
        [48 as ::core::ffi::c_int as int8_t, -(47 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(32 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(40 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(51 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(41 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(39 as ::core::ffi::c_int) as int8_t],
        [43 as ::core::ffi::c_int as int8_t, -(19 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
        [61 as ::core::ffi::c_int as int8_t, -(55 as ::core::ffi::c_int) as int8_t],
        [56 as ::core::ffi::c_int as int8_t, -(46 as ::core::ffi::c_int) as int8_t],
        [62 as ::core::ffi::c_int as int8_t, -(50 as ::core::ffi::c_int) as int8_t],
        [81 as ::core::ffi::c_int as int8_t, -(67 as ::core::ffi::c_int) as int8_t],
        [45 as ::core::ffi::c_int as int8_t, -(20 as ::core::ffi::c_int) as int8_t],
        [35 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
        [39 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [36 as ::core::ffi::c_int as int8_t, -(16 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
        [35 as ::core::ffi::c_int as int8_t, 18 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [33 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [36 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [17 as ::core::ffi::c_int as int8_t, -(13 as ::core::ffi::c_int) as int8_t],
        [16 as ::core::ffi::c_int as int8_t, -(9 as ::core::ffi::c_int) as int8_t],
        [17 as ::core::ffi::c_int as int8_t, -(12 as ::core::ffi::c_int) as int8_t],
        [27 as ::core::ffi::c_int as int8_t, -(21 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(30 as ::core::ffi::c_int) as int8_t],
        [41 as ::core::ffi::c_int as int8_t, -(40 as ::core::ffi::c_int) as int8_t],
        [42 as ::core::ffi::c_int as int8_t, -(41 as ::core::ffi::c_int) as int8_t],
        [48 as ::core::ffi::c_int as int8_t, -(47 as ::core::ffi::c_int) as int8_t],
        [39 as ::core::ffi::c_int as int8_t, -(32 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(40 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(51 as ::core::ffi::c_int) as int8_t],
        [46 as ::core::ffi::c_int as int8_t, -(41 as ::core::ffi::c_int) as int8_t],
        [52 as ::core::ffi::c_int as int8_t, -(39 as ::core::ffi::c_int) as int8_t],
        [43 as ::core::ffi::c_int as int8_t, -(19 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 11 as ::core::ffi::c_int as int8_t],
        [61 as ::core::ffi::c_int as int8_t, -(55 as ::core::ffi::c_int) as int8_t],
        [56 as ::core::ffi::c_int as int8_t, -(46 as ::core::ffi::c_int) as int8_t],
        [62 as ::core::ffi::c_int as int8_t, -(50 as ::core::ffi::c_int) as int8_t],
        [81 as ::core::ffi::c_int as int8_t, -(67 as ::core::ffi::c_int) as int8_t],
        [45 as ::core::ffi::c_int as int8_t, -(20 as ::core::ffi::c_int) as int8_t],
        [35 as ::core::ffi::c_int as int8_t, -(2 as ::core::ffi::c_int) as int8_t],
        [28 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
        [39 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
        [30 as ::core::ffi::c_int as int8_t, 17 as ::core::ffi::c_int as int8_t],
        [20 as ::core::ffi::c_int as int8_t, 38 as ::core::ffi::c_int as int8_t],
        [18 as ::core::ffi::c_int as int8_t, 45 as ::core::ffi::c_int as int8_t],
        [15 as ::core::ffi::c_int as int8_t, 54 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [36 as ::core::ffi::c_int as int8_t, -(16 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(14 as ::core::ffi::c_int) as int8_t],
        [37 as ::core::ffi::c_int as int8_t, -(17 as ::core::ffi::c_int) as int8_t],
        [32 as ::core::ffi::c_int as int8_t, 1 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [29 as ::core::ffi::c_int as int8_t, 15 as ::core::ffi::c_int as int8_t],
        [24 as ::core::ffi::c_int as int8_t, 25 as ::core::ffi::c_int as int8_t],
        [34 as ::core::ffi::c_int as int8_t, 22 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 16 as ::core::ffi::c_int as int8_t],
        [35 as ::core::ffi::c_int as int8_t, 18 as ::core::ffi::c_int as int8_t],
        [31 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [33 as ::core::ffi::c_int as int8_t, 41 as ::core::ffi::c_int as int8_t],
        [36 as ::core::ffi::c_int as int8_t, 28 as ::core::ffi::c_int as int8_t],
        [27 as ::core::ffi::c_int as int8_t, 47 as ::core::ffi::c_int as int8_t],
        [21 as ::core::ffi::c_int as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 115 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(29 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 98 as ::core::ffi::c_int as int8_t],
        [-(37 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 108 as ::core::ffi::c_int as int8_t],
        [-(24 as ::core::ffi::c_int) as int8_t, 115 as ::core::ffi::c_int as int8_t],
        [-(22 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 62 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 53 as ::core::ffi::c_int as int8_t],
        [0 as ::core::ffi::c_int as int8_t, 59 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 89 as ::core::ffi::c_int as int8_t],
        [-(13 as ::core::ffi::c_int) as int8_t, 94 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(29 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 100 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 57 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 67 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 71 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 77 as ::core::ffi::c_int as int8_t],
        [-(21 as ::core::ffi::c_int) as int8_t, 85 as ::core::ffi::c_int as int8_t],
        [-(16 as ::core::ffi::c_int) as int8_t, 88 as ::core::ffi::c_int as int8_t],
        [-(23 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(15 as ::core::ffi::c_int) as int8_t, 98 as ::core::ffi::c_int as int8_t],
        [-(37 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 82 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 48 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 61 as ::core::ffi::c_int as int8_t],
        [-(8 as ::core::ffi::c_int) as int8_t, 66 as ::core::ffi::c_int as int8_t],
        [-(7 as ::core::ffi::c_int) as int8_t, 70 as ::core::ffi::c_int as int8_t],
        [-(14 as ::core::ffi::c_int) as int8_t, 75 as ::core::ffi::c_int as int8_t],
        [-(10 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(9 as ::core::ffi::c_int) as int8_t, 83 as ::core::ffi::c_int as int8_t],
        [-(12 as ::core::ffi::c_int) as int8_t, 92 as ::core::ffi::c_int as int8_t],
        [-(18 as ::core::ffi::c_int) as int8_t, 108 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(30 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(30 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
        [-(5 as ::core::ffi::c_int) as int8_t, 79 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 104 as ::core::ffi::c_int as int8_t],
        [-(11 as ::core::ffi::c_int) as int8_t, 91 as ::core::ffi::c_int as int8_t],
        [-(30 as ::core::ffi::c_int) as int8_t, 127 as ::core::ffi::c_int as int8_t],
    ],
];
#[inline(always)]
unsafe extern "C" fn pred_pskip_motion(
    h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mut current_block: u64;
    static mut zeromv: crate::rt::A4<[int16_t; 2]> = crate::rt::A4([0 as ::core::ffi::c_int as int16_t, 0]);
    let mut mvbuf: crate::rt::A4<[[int16_t; 2]; 3]> = crate::rt::A4([[0; 2]; 3]);
    let mut ref_0: *mut int8_t = (*h)
        .cur_pic
        .ref_index[0 as ::core::ffi::c_int as usize];
    let mut mv: *mut [int16_t; 2] = (*h)
        .cur_pic
        .motion_val[0 as ::core::ffi::c_int as usize];
    let mut top_ref: ::core::ffi::c_int = 0;
    let mut left_ref: ::core::ffi::c_int = 0;
    let mut diagonal_ref: ::core::ffi::c_int = 0;
    let mut match_count: ::core::ffi::c_int = 0;
    let mut mx: ::core::ffi::c_int = 0;
    let mut my: ::core::ffi::c_int = 0;
    let mut A: *const int16_t = ::core::ptr::null::<int16_t>();
    let mut B: *const int16_t = ::core::ptr::null::<int16_t>();
    let mut C: *const int16_t = ::core::ptr::null::<int16_t>();
    let mut b_stride: ::core::ffi::c_int = (*h).b_stride;
    fill_rectangle(
        (&raw mut *(&raw mut (*sl).ref_cache as *mut [int8_t; 40])
            .offset(0 as ::core::ffi::c_int as isize) as *mut int8_t)
            .offset(
                *(&raw const scan8 as *const uint8_t)
                    .offset(0 as ::core::ffi::c_int as isize) as isize,
            ) as *mut int8_t as *mut ::core::ffi::c_void,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        0 as uint32_t,
        1 as ::core::ffi::c_int,
    );
    if (*sl).left_type[0 as ::core::ffi::c_int as usize]
        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
            << 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int != 0
    {
        left_ref = *ref_0
            .offset(
                (4 as ::core::ffi::c_int
                    * (*sl).left_mb_xy[0 as ::core::ffi::c_int as usize]
                    + 1 as ::core::ffi::c_int
                    + (*(*sl).left_block.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int & !(1 as ::core::ffi::c_int))) as isize,
            ) as ::core::ffi::c_int;
        A = &raw mut *mv
            .offset(
                (*(*h)
                    .mb2b_xy
                    .offset(
                        *(&raw mut (*sl).left_mb_xy as *mut ::core::ffi::c_int)
                            .offset(0 as ::core::ffi::c_int as isize) as isize,
                    ))
                    .wrapping_add(3 as uint32_t)
                    .wrapping_add(
                        (b_stride
                            * *(*sl).left_block.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int) as uint32_t,
                    ) as isize,
            ) as *mut int16_t;
        if (*h).mb_aff_frame != 0 {
            if (*sl).mb_field_decoding_flag != 0 {
                if (*sl).left_type[0 as ::core::ffi::c_int as usize]
                    & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                {
                    left_ref <<= 1 as ::core::ffi::c_int;
                    (*(&raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                        .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t
                        as *mut av_alias32))
                        .u32_0 = (*(A as *const av_alias32)).u32_0;
                    mvbuf[0 as ::core::ffi::c_int
                        as usize][1 as ::core::ffi::c_int as usize] = (mvbuf[0
                        as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                    A = &raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                        .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t;
                }
            } else if (*sl).left_type[0 as ::core::ffi::c_int as usize]
                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
            {
                left_ref >>= 1 as ::core::ffi::c_int;
                (*(&raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                    .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t
                    as *mut av_alias32))
                    .u32_0 = (*(A as *const av_alias32)).u32_0;
                mvbuf[0 as ::core::ffi::c_int
                    as usize][1 as ::core::ffi::c_int as usize] = (mvbuf[0
                    as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                A = &raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                    .offset(0 as ::core::ffi::c_int as isize) as *mut int16_t;
            }
        }
        if left_ref as uint32_t | (*(A as *const av_alias32)).u32_0 == 0 {
            current_block = 9113962527849842921;
        } else {
            current_block = 10043043949733653460;
        }
    } else if (*sl).left_type[0 as ::core::ffi::c_int as usize] != 0 {
        left_ref = -(1 as ::core::ffi::c_int);
        A = &raw const zeromv as *const int16_t;
        current_block = 10043043949733653460;
    } else {
        current_block = 9113962527849842921;
    }
    match current_block {
        10043043949733653460 => {
            if (*sl).top_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int != 0
            {
                top_ref = *ref_0
                    .offset(
                        (4 as ::core::ffi::c_int * (*sl).top_mb_xy
                            + 2 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int;
                B = &raw mut *mv
                    .offset(
                        (*(*h).mb2b_xy.offset((*sl).top_mb_xy as isize))
                            .wrapping_add(
                                (3 as ::core::ffi::c_int * b_stride) as uint32_t,
                            ) as isize,
                    ) as *mut int16_t;
                if (*h).mb_aff_frame != 0 {
                    if (*sl).mb_field_decoding_flag != 0 {
                        if (*sl).top_type
                            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                        {
                            top_ref <<= 1 as ::core::ffi::c_int;
                            (*(&raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                                .offset(1 as ::core::ffi::c_int as isize) as *mut int16_t
                                as *mut av_alias32))
                                .u32_0 = (*(B as *const av_alias32)).u32_0;
                            mvbuf[1 as ::core::ffi::c_int
                                as usize][1 as ::core::ffi::c_int as usize] = (mvbuf[1
                                as ::core::ffi::c_int
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                            B = &raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                                .offset(1 as ::core::ffi::c_int as isize) as *mut int16_t;
                        }
                    } else if (*sl).top_type
                        & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                    {
                        top_ref >>= 1 as ::core::ffi::c_int;
                        (*(&raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                            .offset(1 as ::core::ffi::c_int as isize) as *mut int16_t
                            as *mut av_alias32))
                            .u32_0 = (*(B as *const av_alias32)).u32_0;
                        mvbuf[1 as ::core::ffi::c_int
                            as usize][1 as ::core::ffi::c_int as usize] = (mvbuf[1
                            as ::core::ffi::c_int
                            as usize][1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                        B = &raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                            .offset(1 as ::core::ffi::c_int as isize) as *mut int16_t;
                    }
                }
                if top_ref as uint32_t | (*(B as *const av_alias32)).u32_0 == 0 {
                    current_block = 9113962527849842921;
                } else {
                    current_block = 10399321362245223758;
                }
            } else if (*sl).top_type != 0 {
                top_ref = -(1 as ::core::ffi::c_int);
                B = &raw const zeromv as *const int16_t;
                current_block = 10399321362245223758;
            } else {
                current_block = 9113962527849842921;
            }
            match current_block {
                9113962527849842921 => {}
                _ => {
                    if (*sl).topright_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int != 0
                    {
                        diagonal_ref = *ref_0
                            .offset(
                                (4 as ::core::ffi::c_int * (*sl).topright_mb_xy
                                    + 2 as ::core::ffi::c_int) as isize,
                            ) as ::core::ffi::c_int;
                        C = &raw mut *mv
                            .offset(
                                (*(*h).mb2b_xy.offset((*sl).topright_mb_xy as isize))
                                    .wrapping_add(
                                        (3 as ::core::ffi::c_int * b_stride) as uint32_t,
                                    ) as isize,
                            ) as *mut int16_t;
                        if (*h).mb_aff_frame != 0 {
                            if (*sl).mb_field_decoding_flag != 0 {
                                if (*sl).topright_type
                                    & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                                {
                                    diagonal_ref <<= 1 as ::core::ffi::c_int;
                                    (*(&raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                                        .offset(2 as ::core::ffi::c_int as isize) as *mut int16_t
                                        as *mut av_alias32))
                                        .u32_0 = (*(C as *const av_alias32)).u32_0;
                                    mvbuf[2 as ::core::ffi::c_int
                                        as usize][1 as ::core::ffi::c_int as usize] = (mvbuf[2
                                        as ::core::ffi::c_int
                                        as usize][1 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                                    C = &raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                                        .offset(2 as ::core::ffi::c_int as isize) as *mut int16_t;
                                }
                            } else if (*sl).topright_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                            {
                                diagonal_ref >>= 1 as ::core::ffi::c_int;
                                (*(&raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                                    .offset(2 as ::core::ffi::c_int as isize) as *mut int16_t
                                    as *mut av_alias32))
                                    .u32_0 = (*(C as *const av_alias32)).u32_0;
                                mvbuf[2 as ::core::ffi::c_int
                                    as usize][1 as ::core::ffi::c_int as usize] = (mvbuf[2
                                    as ::core::ffi::c_int
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                                C = &raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                                    .offset(2 as ::core::ffi::c_int as isize) as *mut int16_t;
                            }
                        }
                    } else if (*sl).topright_type != 0 {
                        diagonal_ref = -(1 as ::core::ffi::c_int);
                        C = &raw const zeromv as *const int16_t;
                    } else if (*sl).topleft_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int * 0 as ::core::ffi::c_int != 0
                    {
                        diagonal_ref = *ref_0
                            .offset(
                                (4 as ::core::ffi::c_int * (*sl).topleft_mb_xy
                                    + 1 as ::core::ffi::c_int
                                    + ((*sl).topleft_partition & 2 as ::core::ffi::c_int))
                                    as isize,
                            ) as ::core::ffi::c_int;
                        C = &raw mut *mv
                            .offset(
                                (*(*h).mb2b_xy.offset((*sl).topleft_mb_xy as isize))
                                    .wrapping_add(3 as uint32_t)
                                    .wrapping_add(b_stride as uint32_t)
                                    .wrapping_add(
                                        ((*sl).topleft_partition
                                            & 2 as ::core::ffi::c_int * b_stride) as uint32_t,
                                    ) as isize,
                            ) as *mut int16_t;
                        if (*h).mb_aff_frame != 0 {
                            if (*sl).mb_field_decoding_flag != 0 {
                                if (*sl).topleft_type
                                    & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                                {
                                    diagonal_ref <<= 1 as ::core::ffi::c_int;
                                    (*(&raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                                        .offset(2 as ::core::ffi::c_int as isize) as *mut int16_t
                                        as *mut av_alias32))
                                        .u32_0 = (*(C as *const av_alias32)).u32_0;
                                    mvbuf[2 as ::core::ffi::c_int
                                        as usize][1 as ::core::ffi::c_int as usize] = (mvbuf[2
                                        as ::core::ffi::c_int
                                        as usize][1 as ::core::ffi::c_int as usize]
                                        as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                                    C = &raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                                        .offset(2 as ::core::ffi::c_int as isize) as *mut int16_t;
                                }
                            } else if (*sl).topleft_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                            {
                                diagonal_ref >>= 1 as ::core::ffi::c_int;
                                (*(&raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                                    .offset(2 as ::core::ffi::c_int as isize) as *mut int16_t
                                    as *mut av_alias32))
                                    .u32_0 = (*(C as *const av_alias32)).u32_0;
                                mvbuf[2 as ::core::ffi::c_int
                                    as usize][1 as ::core::ffi::c_int as usize] = (mvbuf[2
                                    as ::core::ffi::c_int
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                                C = &raw mut *(&raw mut mvbuf as *mut [int16_t; 2])
                                    .offset(2 as ::core::ffi::c_int as isize) as *mut int16_t;
                            }
                        }
                    } else if (*sl).topleft_type != 0 {
                        diagonal_ref = -(1 as ::core::ffi::c_int);
                        C = &raw const zeromv as *const int16_t;
                    } else {
                        diagonal_ref = -(2 as ::core::ffi::c_int);
                        C = &raw const zeromv as *const int16_t;
                    }
                    match_count = (diagonal_ref == 0) as ::core::ffi::c_int
                        + (top_ref == 0) as ::core::ffi::c_int
                        + (left_ref == 0) as ::core::ffi::c_int;
                    if match_count > 1 as ::core::ffi::c_int {
                        mx = median3_c(
                            *A.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int,
                            *B.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int,
                            *C.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int,
                        );
                        my = median3_c(
                            *A.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int,
                            *B.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int,
                            *C.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int,
                        );
                    } else if match_count == 1 as ::core::ffi::c_int {
                        if left_ref == 0 {
                            mx = *A.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int;
                            my = *A.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int;
                        } else if top_ref == 0 {
                            mx = *B.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int;
                            my = *B.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int;
                        } else {
                            mx = *C.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int;
                            my = *C.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int;
                        }
                    } else {
                        mx = median3_c(
                            *A.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int,
                            *B.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int,
                            *C.offset(0 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int,
                        );
                        my = median3_c(
                            *A.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int,
                            *B.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int,
                            *C.offset(1 as ::core::ffi::c_int as isize)
                                as ::core::ffi::c_int,
                        );
                    }
                    fill_rectangle(
                        &raw mut *(&raw mut *(&raw mut (*sl).mv_cache
                            as *mut [[int16_t; 2]; 40])
                            .offset(0 as ::core::ffi::c_int as isize)
                            as *mut [int16_t; 2])
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize) as isize,
                            ) as *mut int16_t as *mut ::core::ffi::c_void,
                        4 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        8 as ::core::ffi::c_int,
                        pack16to32(mx as ::core::ffi::c_uint, my as ::core::ffi::c_uint),
                        4 as ::core::ffi::c_int,
                    );
                    return;
                }
            }
        }
        _ => {}
    }
    fill_rectangle(
        &raw mut *(&raw mut *(&raw mut (*sl).mv_cache as *mut [[int16_t; 2]; 40])
            .offset(0 as ::core::ffi::c_int as isize) as *mut [int16_t; 2])
            .offset(
                *(&raw const scan8 as *const uint8_t)
                    .offset(0 as ::core::ffi::c_int as isize) as isize,
            ) as *mut int16_t as *mut ::core::ffi::c_void,
        4 as ::core::ffi::c_int,
        4 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        0 as uint32_t,
        4 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn fill_decode_neighbors(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut mb_type: ::core::ffi::c_int,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mut topleft_xy: ::core::ffi::c_int = 0;
    let mut top_xy: ::core::ffi::c_int = 0;
    let mut topright_xy: ::core::ffi::c_int = 0;
    let mut left_xy: [::core::ffi::c_int; 2] = [0; 2];
    static mut left_block_options: [[uint8_t; 32]; 4] = [
        [
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (3 as ::core::ffi::c_int + 3 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 5 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 9 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (3 as ::core::ffi::c_int + 1 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        [
            0 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (3 as ::core::ffi::c_int + 0 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            (1 as ::core::ffi::c_int + 8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    ];
    (*sl).topleft_partition = -(1 as ::core::ffi::c_int);
    top_xy = mb_xy - ((*h).mb_stride << (*sl).mb_field_decoding_flag);
    topleft_xy = top_xy - 1 as ::core::ffi::c_int;
    topright_xy = top_xy + 1 as ::core::ffi::c_int;
    left_xy[0 as ::core::ffi::c_int as usize] = mb_xy - 1 as ::core::ffi::c_int;
    left_xy[1 as ::core::ffi::c_int as usize] = left_xy[0 as ::core::ffi::c_int
        as usize];
    (*sl).left_block = &raw const *(&raw const left_block_options
        as *const [uint8_t; 32])
        .offset(0 as ::core::ffi::c_int as isize) as *const uint8_t;
    if (*h).mb_aff_frame != 0 {
        let left_mb_field_flag: ::core::ffi::c_int = (*(*h)
            .cur_pic
            .mb_type
            .offset((mb_xy - 1 as ::core::ffi::c_int) as isize)
            & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint32_t)
            as ::core::ffi::c_int;
        let curr_mb_field_flag: ::core::ffi::c_int = mb_type
            & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
        if (*sl).mb_y & 1 as ::core::ffi::c_int != 0 {
            if left_mb_field_flag != curr_mb_field_flag {
                left_xy[0 as ::core::ffi::c_int as usize] = mb_xy - (*h).mb_stride
                    - 1 as ::core::ffi::c_int;
                left_xy[1 as ::core::ffi::c_int as usize] = left_xy[0
                    as ::core::ffi::c_int as usize];
                if curr_mb_field_flag != 0 {
                    left_xy[1 as ::core::ffi::c_int as usize] += (*h).mb_stride;
                    (*sl).left_block = &raw const *(&raw const left_block_options
                        as *const [uint8_t; 32])
                        .offset(3 as ::core::ffi::c_int as isize) as *const uint8_t;
                } else {
                    topleft_xy += (*h).mb_stride;
                    (*sl).topleft_partition = 0 as ::core::ffi::c_int;
                    (*sl).left_block = &raw const *(&raw const left_block_options
                        as *const [uint8_t; 32])
                        .offset(1 as ::core::ffi::c_int as isize) as *const uint8_t;
                }
            }
        } else {
            if curr_mb_field_flag != 0 {
                topleft_xy = (topleft_xy as uint32_t)
                    .wrapping_add(
                        (*h).mb_stride as uint32_t
                            & (*(*h)
                                .cur_pic
                                .mb_type
                                .offset((top_xy - 1 as ::core::ffi::c_int) as isize)
                                >> 7 as ::core::ffi::c_int & 1 as uint32_t)
                                .wrapping_sub(1 as uint32_t),
                    ) as ::core::ffi::c_int as ::core::ffi::c_int;
                topright_xy = (topright_xy as uint32_t)
                    .wrapping_add(
                        (*h).mb_stride as uint32_t
                            & (*(*h)
                                .cur_pic
                                .mb_type
                                .offset((top_xy + 1 as ::core::ffi::c_int) as isize)
                                >> 7 as ::core::ffi::c_int & 1 as uint32_t)
                                .wrapping_sub(1 as uint32_t),
                    ) as ::core::ffi::c_int as ::core::ffi::c_int;
                top_xy = (top_xy as uint32_t)
                    .wrapping_add(
                        (*h).mb_stride as uint32_t
                            & (*(*h).cur_pic.mb_type.offset(top_xy as isize)
                                >> 7 as ::core::ffi::c_int & 1 as uint32_t)
                                .wrapping_sub(1 as uint32_t),
                    ) as ::core::ffi::c_int as ::core::ffi::c_int;
            }
            if left_mb_field_flag != curr_mb_field_flag {
                if curr_mb_field_flag != 0 {
                    left_xy[1 as ::core::ffi::c_int as usize] += (*h).mb_stride;
                    (*sl).left_block = &raw const *(&raw const left_block_options
                        as *const [uint8_t; 32])
                        .offset(3 as ::core::ffi::c_int as isize) as *const uint8_t;
                } else {
                    (*sl).left_block = &raw const *(&raw const left_block_options
                        as *const [uint8_t; 32])
                        .offset(2 as ::core::ffi::c_int as isize) as *const uint8_t;
                }
            }
        }
    }
    (*sl).topleft_mb_xy = topleft_xy;
    (*sl).top_mb_xy = top_xy;
    (*sl).topright_mb_xy = topright_xy;
    (*sl).left_mb_xy[0 as ::core::ffi::c_int as usize] = left_xy[0 as ::core::ffi::c_int
        as usize];
    (*sl).left_mb_xy[1 as ::core::ffi::c_int as usize] = left_xy[1 as ::core::ffi::c_int
        as usize];
    (*sl).topleft_type = *(*h).cur_pic.mb_type.offset(topleft_xy as isize)
        as ::core::ffi::c_int;
    (*sl).top_type = *(*h).cur_pic.mb_type.offset(top_xy as isize) as ::core::ffi::c_int;
    (*sl).topright_type = *(*h).cur_pic.mb_type.offset(topright_xy as isize)
        as ::core::ffi::c_int;
    (*sl).left_type[0 as ::core::ffi::c_int as usize] = *(*h)
        .cur_pic
        .mb_type
        .offset(left_xy[0 as ::core::ffi::c_int as usize] as isize)
        as ::core::ffi::c_int;
    (*sl).left_type[1 as ::core::ffi::c_int as usize] = *(*h)
        .cur_pic
        .mb_type
        .offset(left_xy[1 as ::core::ffi::c_int as usize] as isize)
        as ::core::ffi::c_int;
    if *(*h).slice_table.offset(topleft_xy as isize) as ::core::ffi::c_int
        != (*sl).slice_num
    {
        (*sl).topleft_type = 0 as ::core::ffi::c_int;
        if *(*h).slice_table.offset(top_xy as isize) as ::core::ffi::c_int
            != (*sl).slice_num
        {
            (*sl).top_type = 0 as ::core::ffi::c_int;
        }
        if *(*h).slice_table.offset(left_xy[0 as ::core::ffi::c_int as usize] as isize)
            as ::core::ffi::c_int != (*sl).slice_num
        {
            (*sl).left_type[1 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_int;
            (*sl).left_type[0 as ::core::ffi::c_int as usize] = (*sl)
                .left_type[1 as ::core::ffi::c_int as usize];
        }
    }
    if *(*h).slice_table.offset(topright_xy as isize) as ::core::ffi::c_int
        != (*sl).slice_num
    {
        (*sl).topright_type = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn fill_decode_caches(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut mb_type: ::core::ffi::c_int,
) {
    let mut topleft_xy: ::core::ffi::c_int = 0;
    let mut top_xy: ::core::ffi::c_int = 0;
    let mut topright_xy: ::core::ffi::c_int = 0;
    let mut left_xy: [::core::ffi::c_int; 2] = [0; 2];
    let mut topleft_type: ::core::ffi::c_int = 0;
    let mut top_type: ::core::ffi::c_int = 0;
    let mut topright_type: ::core::ffi::c_int = 0;
    let mut left_type: [::core::ffi::c_int; 2] = [0; 2];
    let mut left_block: *const uint8_t = (*sl).left_block;
    let mut i: ::core::ffi::c_int = 0;
    let mut nnz: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut nnz_cache: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    topleft_xy = (*sl).topleft_mb_xy;
    top_xy = (*sl).top_mb_xy;
    topright_xy = (*sl).topright_mb_xy;
    left_xy[0 as ::core::ffi::c_int as usize] = (*sl)
        .left_mb_xy[0 as ::core::ffi::c_int as usize];
    left_xy[1 as ::core::ffi::c_int as usize] = (*sl)
        .left_mb_xy[1 as ::core::ffi::c_int as usize];
    topleft_type = (*sl).topleft_type;
    top_type = (*sl).top_type;
    topright_type = (*sl).topright_type;
    left_type[0 as ::core::ffi::c_int as usize] = (*sl)
        .left_type[0 as ::core::ffi::c_int as usize];
    left_type[1 as ::core::ffi::c_int as usize] = (*sl)
        .left_type[1 as ::core::ffi::c_int as usize];
    if mb_type & (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int == 0 {
        if mb_type & 7 as ::core::ffi::c_int != 0 {
            let mut type_mask: ::core::ffi::c_int = if (*(*h).ps.pps)
                .constrained_intra_pred != 0
            {
                -(1 as ::core::ffi::c_int) & 7 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            };
            (*sl).left_samples_available = 0xffff as ::core::ffi::c_uint;
            (*sl).top_samples_available = (*sl).left_samples_available;
            (*sl).topleft_samples_available = (*sl).top_samples_available;
            (*sl).topright_samples_available = 0xeeea as ::core::ffi::c_uint;
            if top_type & type_mask == 0 {
                (*sl).topleft_samples_available = 0xb3ff as ::core::ffi::c_uint;
                (*sl).top_samples_available = 0x33ff as ::core::ffi::c_uint;
                (*sl).topright_samples_available = 0x26ea as ::core::ffi::c_uint;
            }
            if mb_type & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                != left_type[0 as ::core::ffi::c_int as usize]
                    & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
            {
                if mb_type & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0 {
                    if left_type[0 as ::core::ffi::c_int as usize] & type_mask == 0 {
                        (*sl).topleft_samples_available &= 0xdfff as ::core::ffi::c_uint;
                        (*sl).left_samples_available &= 0x5fff as ::core::ffi::c_uint;
                    }
                    if left_type[1 as ::core::ffi::c_int as usize] & type_mask == 0 {
                        (*sl).topleft_samples_available &= 0xff5f as ::core::ffi::c_uint;
                        (*sl).left_samples_available &= 0xff5f as ::core::ffi::c_uint;
                    }
                } else {
                    let mut left_typei: ::core::ffi::c_int = *(*h)
                        .cur_pic
                        .mb_type
                        .offset(
                            (left_xy[0 as ::core::ffi::c_int as usize] + (*h).mb_stride)
                                as isize,
                        ) as ::core::ffi::c_int;
                    if !(left_typei & type_mask != 0
                        && left_type[0 as ::core::ffi::c_int as usize] & type_mask != 0)
                    {
                        (*sl).topleft_samples_available &= 0xdf5f as ::core::ffi::c_uint;
                        (*sl).left_samples_available &= 0x5f5f as ::core::ffi::c_uint;
                    }
                }
            } else if left_type[0 as ::core::ffi::c_int as usize] & type_mask == 0 {
                (*sl).topleft_samples_available &= 0xdf5f as ::core::ffi::c_uint;
                (*sl).left_samples_available &= 0x5f5f as ::core::ffi::c_uint;
            }
            if topleft_type & type_mask == 0 {
                (*sl).topleft_samples_available &= 0x7fff as ::core::ffi::c_uint;
            }
            if topright_type & type_mask == 0 {
                (*sl).topright_samples_available &= 0xfbff as ::core::ffi::c_uint;
            }
            if mb_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
                if top_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
                    (*((&raw mut (*sl).intra4x4_pred_mode_cache as *mut int8_t)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(
                            (8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
                        ) as *mut av_alias32))
                        .u32_0 = (*((*sl)
                        .intra4x4_pred_mode
                        .offset(*(*h).mb2br_xy.offset(top_xy as isize) as isize)
                        as *const av_alias32))
                        .u32_0;
                } else {
                    (*sl)
                        .intra4x4_pred_mode_cache[(7 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as usize] = (2
                        as ::core::ffi::c_int
                        - 3 as ::core::ffi::c_int
                            * (top_type & type_mask == 0) as ::core::ffi::c_int)
                        as int8_t;
                    (*sl)
                        .intra4x4_pred_mode_cache[(6 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as usize] = (*sl)
                        .intra4x4_pred_mode_cache[(7 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as usize];
                    (*sl)
                        .intra4x4_pred_mode_cache[(5 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as usize] = (*sl)
                        .intra4x4_pred_mode_cache[(6 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as usize];
                    (*sl)
                        .intra4x4_pred_mode_cache[(4 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as usize] = (*sl)
                        .intra4x4_pred_mode_cache[(5 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as usize];
                }
                i = 0 as ::core::ffi::c_int;
                while i < 2 as ::core::ffi::c_int {
                    if left_type[i as usize]
                        & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0
                    {
                        let mut mode: *mut int8_t = (*sl)
                            .intra4x4_pred_mode
                            .offset(
                                *(*h).mb2br_xy.offset(left_xy[i as usize] as isize) as isize,
                            );
                        (*sl)
                            .intra4x4_pred_mode_cache[(3 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                            as usize] = *mode
                            .offset(
                                (6 as ::core::ffi::c_int
                                    - *left_block
                                        .offset(
                                            (0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i)
                                                as isize,
                                        ) as ::core::ffi::c_int) as isize,
                            );
                        (*sl)
                            .intra4x4_pred_mode_cache[(3 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                            as usize] = *mode
                            .offset(
                                (6 as ::core::ffi::c_int
                                    - *left_block
                                        .offset(
                                            (1 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i)
                                                as isize,
                                        ) as ::core::ffi::c_int) as isize,
                            );
                    } else {
                        (*sl)
                            .intra4x4_pred_mode_cache[(3 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                            as usize] = (2 as ::core::ffi::c_int
                            - 3 as ::core::ffi::c_int
                                * (left_type[i as usize] & type_mask == 0)
                                    as ::core::ffi::c_int) as int8_t;
                        (*sl)
                            .intra4x4_pred_mode_cache[(3 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                            as usize] = (*sl)
                            .intra4x4_pred_mode_cache[(3 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                            as usize];
                    }
                    i += 1;
                }
            }
        }
        nnz_cache = &raw mut (*sl).non_zero_count_cache as *mut uint8_t;
        if top_type != 0 {
            nnz = &raw mut *(*h).non_zero_count.offset(top_xy as isize) as *mut uint8_t;
            (*(nnz_cache
                .offset(
                    (4 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
                ) as *mut uint8_t as *mut av_alias32))
                .u32_0 = (*(nnz
                .offset((4 as ::core::ffi::c_int * 3 as ::core::ffi::c_int) as isize)
                as *mut uint8_t as *const av_alias32))
                .u32_0;
            if (*h).chroma_y_shift == 0 {
                (*(nnz_cache
                    .offset(
                        (4 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut av_alias32))
                    .u32_0 = (*(nnz
                    .offset((4 as ::core::ffi::c_int * 7 as ::core::ffi::c_int) as isize)
                    as *mut uint8_t as *const av_alias32))
                    .u32_0;
                (*(nnz_cache
                    .offset(
                        (4 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut uint8_t as *mut av_alias32))
                    .u32_0 = (*(nnz
                    .offset(
                        (4 as ::core::ffi::c_int * 11 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *const av_alias32))
                    .u32_0;
            } else {
                (*(nnz_cache
                    .offset(
                        (4 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut av_alias32))
                    .u32_0 = (*(nnz
                    .offset((4 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize)
                    as *mut uint8_t as *const av_alias32))
                    .u32_0;
                (*(nnz_cache
                    .offset(
                        (4 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut uint8_t as *mut av_alias32))
                    .u32_0 = (*(nnz
                    .offset((4 as ::core::ffi::c_int * 9 as ::core::ffi::c_int) as isize)
                    as *mut uint8_t as *const av_alias32))
                    .u32_0;
            }
        } else {
            let mut top_empty: uint32_t = (if 1 as ::core::ffi::c_int != 0
                && mb_type & 7 as ::core::ffi::c_int == 0
            {
                0 as ::core::ffi::c_int
            } else {
                0x40404040 as ::core::ffi::c_int
            }) as uint32_t;
            (*(nnz_cache
                .offset(
                    (4 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
                ) as *mut uint8_t as *mut av_alias32))
                .u32_0 = top_empty;
            (*(nnz_cache
                .offset(
                    (4 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                ) as *mut uint8_t as *mut av_alias32))
                .u32_0 = top_empty;
            (*(nnz_cache
                .offset(
                    (4 as ::core::ffi::c_int
                        + 8 as ::core::ffi::c_int * 10 as ::core::ffi::c_int) as isize,
                ) as *mut uint8_t as *mut av_alias32))
                .u32_0 = top_empty;
        }
        i = 0 as ::core::ffi::c_int;
        while i < 2 as ::core::ffi::c_int {
            if left_type[i as usize] != 0 {
                nnz = &raw mut *(*h)
                    .non_zero_count
                    .offset(
                        *(&raw mut left_xy as *mut ::core::ffi::c_int).offset(i as isize)
                            as isize,
                    ) as *mut uint8_t;
                *nnz_cache
                    .offset(
                        (3 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                            as isize,
                    ) = *nnz
                    .offset(
                        *left_block
                            .offset(
                                (8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * i) as isize,
                            ) as isize,
                    );
                *nnz_cache
                    .offset(
                        (3 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                            as isize,
                    ) = *nnz
                    .offset(
                        *left_block
                            .offset(
                                (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * i) as isize,
                            ) as isize,
                    );
                if (*(*h).ps.sps).chroma_format_idc == 3 as ::core::ffi::c_int {
                    *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                                as isize,
                        ) = *nnz
                        .offset(
                            (*left_block
                                .offset(
                                    (8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int * i) as isize,
                                ) as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as isize,
                        );
                    *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                                as isize,
                        ) = *nnz
                        .offset(
                            (*left_block
                                .offset(
                                    (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int * i) as isize,
                                ) as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as isize,
                        );
                    *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 11 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                                as isize,
                        ) = *nnz
                        .offset(
                            (*left_block
                                .offset(
                                    (8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int * i) as isize,
                                ) as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as isize,
                        );
                    *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                                as isize,
                        ) = *nnz
                        .offset(
                            (*left_block
                                .offset(
                                    (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int * i) as isize,
                                ) as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as isize,
                        );
                } else if (*(*h).ps.sps).chroma_format_idc == 2 as ::core::ffi::c_int {
                    *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                                as isize,
                        ) = *nnz
                        .offset(
                            (*left_block
                                .offset(
                                    (8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int * i) as isize,
                                ) as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as isize,
                        );
                    *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                                as isize,
                        ) = *nnz
                        .offset(
                            (*left_block
                                .offset(
                                    (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int * i) as isize,
                                ) as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                + 4 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as isize,
                        );
                    *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 11 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                                as isize,
                        ) = *nnz
                        .offset(
                            (*left_block
                                .offset(
                                    (8 as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int * i) as isize,
                                ) as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as isize,
                        );
                    *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                                as isize,
                        ) = *nnz
                        .offset(
                            (*left_block
                                .offset(
                                    (8 as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int * i) as isize,
                                ) as ::core::ffi::c_int - 2 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 4 as ::core::ffi::c_int)
                                as isize,
                        );
                } else {
                    *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * i) as isize,
                        ) = *nnz
                        .offset(
                            *left_block
                                .offset(
                                    (8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int * i) as isize,
                                ) as isize,
                        );
                    *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 11 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * i) as isize,
                        ) = *nnz
                        .offset(
                            *left_block
                                .offset(
                                    (8 as ::core::ffi::c_int + 5 as ::core::ffi::c_int
                                        + 2 as ::core::ffi::c_int * i) as isize,
                                ) as isize,
                        );
                }
            } else {
                let ref mut fresh29 = *nnz_cache
                    .offset(
                        (3 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                            as isize,
                    );
                *fresh29 = (if 1 as ::core::ffi::c_int != 0
                    && mb_type & 7 as ::core::ffi::c_int == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    64 as ::core::ffi::c_int
                }) as uint8_t;
                let ref mut fresh30 = *nnz_cache
                    .offset(
                        (3 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 11 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                            as isize,
                    );
                *fresh30 = *fresh29;
                let ref mut fresh31 = *nnz_cache
                    .offset(
                        (3 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                            as isize,
                    );
                *fresh31 = *fresh30;
                let ref mut fresh32 = *nnz_cache
                    .offset(
                        (3 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                            as isize,
                    );
                *fresh32 = *fresh31;
                let ref mut fresh33 = *nnz_cache
                    .offset(
                        (3 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                            as isize,
                    );
                *fresh33 = *fresh32;
                *nnz_cache
                    .offset(
                        (3 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i)
                            as isize,
                    ) = *fresh33;
            }
            i += 1;
        }
        if top_type != 0 {
            (*sl).top_cbp = *(*h).cbp_table.offset(top_xy as isize)
                as ::core::ffi::c_int;
        } else {
            (*sl).top_cbp = if mb_type & 7 as ::core::ffi::c_int != 0 {
                0x7cf as ::core::ffi::c_int
            } else {
                0xf as ::core::ffi::c_int
            };
        }
        if left_type[0 as ::core::ffi::c_int as usize] != 0 {
            (*sl).left_cbp = *(*h)
                .cbp_table
                .offset(left_xy[0 as ::core::ffi::c_int as usize] as isize)
                as ::core::ffi::c_int & 0x7f0 as ::core::ffi::c_int
                | *(*h)
                    .cbp_table
                    .offset(left_xy[0 as ::core::ffi::c_int as usize] as isize)
                    as ::core::ffi::c_int
                    >> (*left_block.offset(0 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int & !(1 as ::core::ffi::c_int))
                    & 2 as ::core::ffi::c_int
                | (*(*h)
                    .cbp_table
                    .offset(left_xy[1 as ::core::ffi::c_int as usize] as isize)
                    as ::core::ffi::c_int
                    >> (*left_block.offset(2 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int & !(1 as ::core::ffi::c_int))
                    & 2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
        } else {
            (*sl).left_cbp = if mb_type & 7 as ::core::ffi::c_int != 0 {
                0x7cf as ::core::ffi::c_int
            } else {
                0xf as ::core::ffi::c_int
            };
        }
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) != 0
        || mb_type & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0
            && (*sl).direct_spatial_mv_pred != 0
    {
        let mut list: ::core::ffi::c_int = 0;
        let mut b_stride: ::core::ffi::c_int = (*h).b_stride;
        list = 0 as ::core::ffi::c_int;
        while (list as ::core::ffi::c_uint) < (*sl).list_count {
            let mut ref_cache: *mut int8_t = (&raw mut *(&raw mut (*sl).ref_cache
                as *mut [int8_t; 40])
                .offset(list as isize) as *mut int8_t)
                .offset(
                    *(&raw const scan8 as *const uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize) as isize,
                ) as *mut int8_t;
            let mut ref_0: *mut int8_t = (*h).cur_pic.ref_index[list as usize];
            let mut mv_cache: *mut [int16_t; 2] = (&raw mut *(&raw mut (*sl).mv_cache
                as *mut [[int16_t; 2]; 40])
                .offset(list as isize) as *mut [int16_t; 2])
                .offset(
                    *(&raw const scan8 as *const uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize) as isize,
                ) as *mut [int16_t; 2];
            let mut mv: *mut [int16_t; 2] = (*h).cur_pic.motion_val[list as usize];
            if !(mb_type
                & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int * list == 0)
            {
                if top_type
                    & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int * list != 0
                {
                    let b_xy: ::core::ffi::c_int = (*(*h)
                        .mb2b_xy
                        .offset(top_xy as isize))
                        .wrapping_add((3 as ::core::ffi::c_int * b_stride) as uint32_t)
                        as ::core::ffi::c_int;
                    (*(&raw mut *mv_cache
                        .offset(
                            (0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut int16_t as *mut av_alias64))
                        .u64_0 = (*(&raw mut *mv
                        .offset((b_xy + 0 as ::core::ffi::c_int) as isize)
                        as *mut int16_t as *const av_alias64))
                        .u64_0;
                    (*((&raw mut *mv_cache
                        .offset(
                            (0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut int16_t as *mut ::core::ffi::c_char)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                        .u64_0 = (*((&raw mut *mv
                        .offset((b_xy + 0 as ::core::ffi::c_int) as isize)
                        as *mut int16_t as *mut ::core::ffi::c_char)
                        .offset(8 as ::core::ffi::c_int as isize) as *const av_alias64))
                        .u64_0;
                    let ref mut fresh34 = *ref_cache
                        .offset(
                            (1 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        );
                    *fresh34 = *ref_0
                        .offset(
                            (4 as ::core::ffi::c_int * top_xy + 2 as ::core::ffi::c_int)
                                as isize,
                        );
                    *ref_cache
                        .offset(
                            (0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) = *fresh34;
                    let ref mut fresh35 = *ref_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        );
                    *fresh35 = *ref_0
                        .offset(
                            (4 as ::core::ffi::c_int * top_xy + 3 as ::core::ffi::c_int)
                                as isize,
                        );
                    *ref_cache
                        .offset(
                            (2 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) = *fresh35;
                } else {
                    (*(&raw mut *mv_cache
                        .offset(
                            (0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut int16_t as *mut av_alias64))
                        .u64_0 = 0 as uint64_t;
                    (*((&raw mut *mv_cache
                        .offset(
                            (0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut int16_t as *mut ::core::ffi::c_char)
                        .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
                        .u64_0 = 0 as uint64_t;
                    (*(ref_cache
                        .offset(
                            (0 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut int8_t as *mut av_alias32))
                        .u32_0 = (((if top_type != 0 {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        -(2 as ::core::ffi::c_int)
                    }) & 0xff as ::core::ffi::c_int) as ::core::ffi::c_uint)
                        .wrapping_mul(0x1010101 as ::core::ffi::c_uint) as uint32_t;
                }
                if mb_type
                    & ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) != 0
                {
                    i = 0 as ::core::ffi::c_int;
                    while i < 2 as ::core::ffi::c_int {
                        let mut cache_idx: ::core::ffi::c_int = -(1
                            as ::core::ffi::c_int)
                            + i * 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
                        if left_type[i as usize]
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int * list != 0
                        {
                            let b_xy_0: ::core::ffi::c_int = (*(*h)
                                .mb2b_xy
                                .offset(left_xy[i as usize] as isize))
                                .wrapping_add(3 as uint32_t) as ::core::ffi::c_int;
                            let b8_xy: ::core::ffi::c_int = 4 as ::core::ffi::c_int
                                * left_xy[i as usize] + 1 as ::core::ffi::c_int;
                            (*(&raw mut *mv_cache.offset(cache_idx as isize)
                                as *mut int16_t as *mut av_alias32))
                                .u32_0 = (*(&raw mut *mv
                                .offset(
                                    (b_xy_0
                                        + b_stride
                                            * *left_block
                                                .offset(
                                                    (0 as ::core::ffi::c_int + i * 2 as ::core::ffi::c_int)
                                                        as isize,
                                                ) as ::core::ffi::c_int) as isize,
                                ) as *mut int16_t as *const av_alias32))
                                .u32_0;
                            (*(&raw mut *mv_cache
                                .offset((cache_idx + 8 as ::core::ffi::c_int) as isize)
                                as *mut int16_t as *mut av_alias32))
                                .u32_0 = (*(&raw mut *mv
                                .offset(
                                    (b_xy_0
                                        + b_stride
                                            * *left_block
                                                .offset(
                                                    (1 as ::core::ffi::c_int + i * 2 as ::core::ffi::c_int)
                                                        as isize,
                                                ) as ::core::ffi::c_int) as isize,
                                ) as *mut int16_t as *const av_alias32))
                                .u32_0;
                            *ref_cache.offset(cache_idx as isize) = *ref_0
                                .offset(
                                    (b8_xy
                                        + (*left_block
                                            .offset(
                                                (0 as ::core::ffi::c_int + i * 2 as ::core::ffi::c_int)
                                                    as isize,
                                            ) as ::core::ffi::c_int & !(1 as ::core::ffi::c_int)))
                                        as isize,
                                );
                            *ref_cache
                                .offset((cache_idx + 8 as ::core::ffi::c_int) as isize) = *ref_0
                                .offset(
                                    (b8_xy
                                        + (*left_block
                                            .offset(
                                                (1 as ::core::ffi::c_int + i * 2 as ::core::ffi::c_int)
                                                    as isize,
                                            ) as ::core::ffi::c_int & !(1 as ::core::ffi::c_int)))
                                        as isize,
                                );
                        } else {
                            (*(&raw mut *mv_cache.offset(cache_idx as isize)
                                as *mut int16_t as *mut av_alias32))
                                .u32_0 = 0 as uint32_t;
                            (*(&raw mut *mv_cache
                                .offset((cache_idx + 8 as ::core::ffi::c_int) as isize)
                                as *mut int16_t as *mut av_alias32))
                                .u32_0 = 0 as uint32_t;
                            let ref mut fresh36 = *ref_cache
                                .offset((cache_idx + 8 as ::core::ffi::c_int) as isize);
                            *fresh36 = (if left_type[i as usize] != 0 {
                                -(1 as ::core::ffi::c_int)
                            } else {
                                -(2 as ::core::ffi::c_int)
                            }) as int8_t;
                            *ref_cache.offset(cache_idx as isize) = *fresh36;
                        }
                        i += 1;
                    }
                } else if left_type[0 as ::core::ffi::c_int as usize]
                    & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int * list != 0
                {
                    let b_xy_1: ::core::ffi::c_int = (*(*h)
                        .mb2b_xy
                        .offset(left_xy[0 as ::core::ffi::c_int as usize] as isize))
                        .wrapping_add(3 as uint32_t) as ::core::ffi::c_int;
                    let b8_xy_0: ::core::ffi::c_int = 4 as ::core::ffi::c_int
                        * left_xy[0 as ::core::ffi::c_int as usize]
                        + 1 as ::core::ffi::c_int;
                    (*(&raw mut *mv_cache.offset(-(1 as ::core::ffi::c_int) as isize)
                        as *mut int16_t as *mut av_alias32))
                        .u32_0 = (*(&raw mut *mv
                        .offset(
                            (b_xy_1
                                + b_stride
                                    * *left_block.offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int) as isize,
                        ) as *mut int16_t as *const av_alias32))
                        .u32_0;
                    *ref_cache.offset(-(1 as ::core::ffi::c_int) as isize) = *ref_0
                        .offset(
                            (b8_xy_0
                                + (*left_block.offset(0 as ::core::ffi::c_int as isize)
                                    as ::core::ffi::c_int & !(1 as ::core::ffi::c_int)))
                                as isize,
                        );
                } else {
                    (*(&raw mut *mv_cache.offset(-(1 as ::core::ffi::c_int) as isize)
                        as *mut int16_t as *mut av_alias32))
                        .u32_0 = 0 as uint32_t;
                    *ref_cache.offset(-(1 as ::core::ffi::c_int) as isize) = (if left_type[0
                        as ::core::ffi::c_int as usize] != 0
                    {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        -(2 as ::core::ffi::c_int)
                    }) as int8_t;
                }
                if topright_type
                    & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int * list != 0
                {
                    let b_xy_2: ::core::ffi::c_int = (*(*h)
                        .mb2b_xy
                        .offset(topright_xy as isize))
                        .wrapping_add((3 as ::core::ffi::c_int * b_stride) as uint32_t)
                        as ::core::ffi::c_int;
                    (*(&raw mut *mv_cache
                        .offset(
                            (4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut int16_t as *mut av_alias32))
                        .u32_0 = (*(&raw mut *mv.offset(b_xy_2 as isize) as *mut int16_t
                        as *const av_alias32))
                        .u32_0;
                    *ref_cache
                        .offset(
                            (4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) = *ref_0
                        .offset(
                            (4 as ::core::ffi::c_int * topright_xy
                                + 2 as ::core::ffi::c_int) as isize,
                        );
                } else {
                    (*(&raw mut *mv_cache
                        .offset(
                            (4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as *mut int16_t as *mut av_alias32))
                        .u32_0 = 0 as uint32_t;
                    *ref_cache
                        .offset(
                            (4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) = (if topright_type != 0 {
                        -(1 as ::core::ffi::c_int)
                    } else {
                        -(2 as ::core::ffi::c_int)
                    }) as int8_t;
                }
                if (*ref_cache
                    .offset(
                        (2 as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int) as isize,
                    ) as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                    || (*ref_cache
                        .offset(
                            (4 as ::core::ffi::c_int
                                - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                as isize,
                        ) as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
                {
                    if topleft_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int * list != 0
                    {
                        let b_xy_3: ::core::ffi::c_int = (*(*h)
                            .mb2b_xy
                            .offset(topleft_xy as isize))
                            .wrapping_add(3 as uint32_t)
                            .wrapping_add(b_stride as uint32_t)
                            .wrapping_add(
                                ((*sl).topleft_partition
                                    & 2 as ::core::ffi::c_int * b_stride) as uint32_t,
                            ) as ::core::ffi::c_int;
                        let b8_xy_1: ::core::ffi::c_int = 4 as ::core::ffi::c_int
                            * topleft_xy + 1 as ::core::ffi::c_int
                            + ((*sl).topleft_partition & 2 as ::core::ffi::c_int);
                        (*(&raw mut *mv_cache
                            .offset(
                                (-(1 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut int16_t as *mut av_alias32))
                            .u32_0 = (*(&raw mut *mv.offset(b_xy_3 as isize)
                            as *mut int16_t as *const av_alias32))
                            .u32_0;
                        *ref_cache
                            .offset(
                                (-(1 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as isize,
                            ) = *ref_0.offset(b8_xy_1 as isize);
                    } else {
                        (*(&raw mut *mv_cache
                            .offset(
                                (-(1 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut int16_t as *mut av_alias32))
                            .u32_0 = 0 as uint32_t;
                        *ref_cache
                            .offset(
                                (-(1 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as isize,
                            ) = (if topleft_type != 0 {
                            -(1 as ::core::ffi::c_int)
                        } else {
                            -(2 as ::core::ffi::c_int)
                        }) as int8_t;
                    }
                }
                if !(mb_type
                    & ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) != 0
                    && (*h).mb_aff_frame == 0)
                {
                    if mb_type
                        & ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) == 0
                    {
                        let mut mvd_cache: *mut [uint8_t; 2] = (&raw mut *(&raw mut (*sl)
                            .mvd_cache as *mut [[uint8_t; 2]; 40])
                            .offset(list as isize) as *mut [uint8_t; 2])
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize) as isize,
                            ) as *mut [uint8_t; 2];
                        let mut mvd: *mut [uint8_t; 2] = (*sl).mvd_table[list as usize];
                        let ref mut fresh37 = *ref_cache
                            .offset(
                                (2 as ::core::ffi::c_int
                                    + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as isize,
                            );
                        *fresh37 = -(2 as ::core::ffi::c_int) as int8_t;
                        *ref_cache
                            .offset(
                                (2 as ::core::ffi::c_int
                                    + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int)
                                    as isize,
                            ) = *fresh37;
                        (*(&raw mut *mv_cache
                            .offset(
                                (2 as ::core::ffi::c_int
                                    + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut int16_t as *mut av_alias32))
                            .u32_0 = 0 as uint32_t;
                        (*(&raw mut *mv_cache
                            .offset(
                                (2 as ::core::ffi::c_int
                                    + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut int16_t as *mut av_alias32))
                            .u32_0 = 0 as uint32_t;
                        if top_type
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int * list != 0
                        {
                            let b_xy_4: ::core::ffi::c_int = *(*h)
                                .mb2br_xy
                                .offset(top_xy as isize) as ::core::ffi::c_int;
                            (*(&raw mut *mvd_cache
                                .offset(
                                    (0 as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut uint8_t as *mut av_alias64))
                                .u64_0 = (*(&raw mut *mvd
                                .offset((b_xy_4 + 0 as ::core::ffi::c_int) as isize)
                                as *mut uint8_t as *const av_alias64))
                                .u64_0;
                        } else {
                            (*(&raw mut *mvd_cache
                                .offset(
                                    (0 as ::core::ffi::c_int
                                        - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut uint8_t as *mut av_alias64))
                                .u64_0 = 0 as uint64_t;
                        }
                        if left_type[0 as ::core::ffi::c_int as usize]
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int * list != 0
                        {
                            let b_xy_5: ::core::ffi::c_int = (*(*h)
                                .mb2br_xy
                                .offset(left_xy[0 as ::core::ffi::c_int as usize] as isize))
                                .wrapping_add(6 as uint32_t) as ::core::ffi::c_int;
                            (*(&raw mut *mvd_cache
                                .offset(
                                    (-(1 as ::core::ffi::c_int)
                                        + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut uint8_t as *mut av_alias16))
                                .u16_0 = (*(&raw mut *mvd
                                .offset(
                                    (b_xy_5
                                        - *left_block.offset(0 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int) as isize,
                                ) as *mut uint8_t as *const av_alias16))
                                .u16_0;
                            (*(&raw mut *mvd_cache
                                .offset(
                                    (-(1 as ::core::ffi::c_int)
                                        + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut uint8_t as *mut av_alias16))
                                .u16_0 = (*(&raw mut *mvd
                                .offset(
                                    (b_xy_5
                                        - *left_block.offset(1 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int) as isize,
                                ) as *mut uint8_t as *const av_alias16))
                                .u16_0;
                        } else {
                            (*(&raw mut *mvd_cache
                                .offset(
                                    (-(1 as ::core::ffi::c_int)
                                        + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut uint8_t as *mut av_alias16))
                                .u16_0 = 0 as uint16_t;
                            (*(&raw mut *mvd_cache
                                .offset(
                                    (-(1 as ::core::ffi::c_int)
                                        + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut uint8_t as *mut av_alias16))
                                .u16_0 = 0 as uint16_t;
                        }
                        if left_type[1 as ::core::ffi::c_int as usize]
                            & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                                | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int)
                                << 2 as ::core::ffi::c_int * list != 0
                        {
                            let b_xy_6: ::core::ffi::c_int = (*(*h)
                                .mb2br_xy
                                .offset(left_xy[1 as ::core::ffi::c_int as usize] as isize))
                                .wrapping_add(6 as uint32_t) as ::core::ffi::c_int;
                            (*(&raw mut *mvd_cache
                                .offset(
                                    (-(1 as ::core::ffi::c_int)
                                        + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut uint8_t as *mut av_alias16))
                                .u16_0 = (*(&raw mut *mvd
                                .offset(
                                    (b_xy_6
                                        - *left_block.offset(2 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int) as isize,
                                ) as *mut uint8_t as *const av_alias16))
                                .u16_0;
                            (*(&raw mut *mvd_cache
                                .offset(
                                    (-(1 as ::core::ffi::c_int)
                                        + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut uint8_t as *mut av_alias16))
                                .u16_0 = (*(&raw mut *mvd
                                .offset(
                                    (b_xy_6
                                        - *left_block.offset(3 as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int) as isize,
                                ) as *mut uint8_t as *const av_alias16))
                                .u16_0;
                        } else {
                            (*(&raw mut *mvd_cache
                                .offset(
                                    (-(1 as ::core::ffi::c_int)
                                        + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut uint8_t as *mut av_alias16))
                                .u16_0 = 0 as uint16_t;
                            (*(&raw mut *mvd_cache
                                .offset(
                                    (-(1 as ::core::ffi::c_int)
                                        + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut uint8_t as *mut av_alias16))
                                .u16_0 = 0 as uint16_t;
                        }
                        (*(&raw mut *mvd_cache
                            .offset(
                                (2 as ::core::ffi::c_int
                                    + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut uint8_t as *mut av_alias16))
                            .u16_0 = 0 as uint16_t;
                        (*(&raw mut *mvd_cache
                            .offset(
                                (2 as ::core::ffi::c_int
                                    + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as isize,
                            ) as *mut uint8_t as *mut av_alias16))
                            .u16_0 = 0 as uint16_t;
                        if (*sl).slice_type_nos
                            == AV_PICTURE_TYPE_B as ::core::ffi::c_int
                        {
                            let mut direct_cache: *mut uint8_t = (&raw mut (*sl)
                                .direct_cache as *mut uint8_t)
                                .offset(
                                    *(&raw const scan8 as *const uint8_t)
                                        .offset(0 as ::core::ffi::c_int as isize) as isize,
                                ) as *mut uint8_t;
                            let mut direct_table: *mut uint8_t = (*h).direct_table;
                            fill_rectangle(
                                direct_cache as *mut ::core::ffi::c_void,
                                4 as ::core::ffi::c_int,
                                4 as ::core::ffi::c_int,
                                8 as ::core::ffi::c_int,
                                ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    >> 1 as ::core::ffi::c_int) as uint32_t,
                                1 as ::core::ffi::c_int,
                            );
                            if top_type
                                & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0
                            {
                                (*(direct_cache
                                    .offset(
                                        (-(1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *mut uint8_t as *mut av_alias32))
                                    .u32_0 = (0x1010101 as ::core::ffi::c_uint)
                                    .wrapping_mul(
                                        ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
                                    ) as uint32_t;
                            } else if top_type
                                & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
                            {
                                let mut b8_xy_2: ::core::ffi::c_int = 4
                                    as ::core::ffi::c_int * top_xy;
                                *direct_cache
                                    .offset(
                                        (0 as ::core::ffi::c_int
                                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                            as isize,
                                    ) = *direct_table
                                    .offset((b8_xy_2 + 2 as ::core::ffi::c_int) as isize);
                                *direct_cache
                                    .offset(
                                        (2 as ::core::ffi::c_int
                                            - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                            as isize,
                                    ) = *direct_table
                                    .offset((b8_xy_2 + 3 as ::core::ffi::c_int) as isize);
                            } else {
                                (*(direct_cache
                                    .offset(
                                        (-(1 as ::core::ffi::c_int) * 8 as ::core::ffi::c_int)
                                            as isize,
                                    ) as *mut uint8_t as *mut av_alias32))
                                    .u32_0 = (0x1010101 as ::core::ffi::c_int
                                    * ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                        >> 1 as ::core::ffi::c_int)) as uint32_t;
                            }
                            if left_type[0 as ::core::ffi::c_int as usize]
                                & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0
                            {
                                *direct_cache
                                    .offset(
                                        (-(1 as ::core::ffi::c_int)
                                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                            as isize,
                                    ) = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                    >> 1 as ::core::ffi::c_int) as uint8_t;
                            } else if left_type[0 as ::core::ffi::c_int as usize]
                                & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
                            {
                                *direct_cache
                                    .offset(
                                        (-(1 as ::core::ffi::c_int)
                                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                            as isize,
                                    ) = *direct_table
                                    .offset(
                                        (4 as ::core::ffi::c_int
                                            * left_xy[0 as ::core::ffi::c_int as usize]
                                            + 1 as ::core::ffi::c_int
                                            + (*left_block.offset(0 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int & !(1 as ::core::ffi::c_int)))
                                            as isize,
                                    );
                            } else {
                                *direct_cache
                                    .offset(
                                        (-(1 as ::core::ffi::c_int)
                                            + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                            as isize,
                                    ) = ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    >> 1 as ::core::ffi::c_int) as uint8_t;
                            }
                            if left_type[1 as ::core::ffi::c_int as usize]
                                & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0
                            {
                                *direct_cache
                                    .offset(
                                        (-(1 as ::core::ffi::c_int)
                                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                            as isize,
                                    ) = ((1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                                    >> 1 as ::core::ffi::c_int) as uint8_t;
                            } else if left_type[1 as ::core::ffi::c_int as usize]
                                & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0
                            {
                                *direct_cache
                                    .offset(
                                        (-(1 as ::core::ffi::c_int)
                                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                            as isize,
                                    ) = *direct_table
                                    .offset(
                                        (4 as ::core::ffi::c_int
                                            * left_xy[1 as ::core::ffi::c_int as usize]
                                            + 1 as ::core::ffi::c_int
                                            + (*left_block.offset(2 as ::core::ffi::c_int as isize)
                                                as ::core::ffi::c_int & !(1 as ::core::ffi::c_int)))
                                            as isize,
                                    );
                            } else {
                                *direct_cache
                                    .offset(
                                        (-(1 as ::core::ffi::c_int)
                                            + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                            as isize,
                                    ) = ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                                    >> 1 as ::core::ffi::c_int) as uint8_t;
                            }
                        }
                    }
                    if (*h).mb_aff_frame != 0 {
                        if (*sl).mb_field_decoding_flag != 0 {
                            if topleft_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if top_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if top_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if top_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if top_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if topright_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if left_type[0 as ::core::ffi::c_int as usize]
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if left_type[0 as ::core::ffi::c_int as usize]
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if left_type[1 as ::core::ffi::c_int as usize]
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if left_type[1 as ::core::ffi::c_int as usize]
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int == 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int * 2 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                        } else {
                            if topleft_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = (((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if top_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = (((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 0 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if top_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = (((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if top_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = (((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 2 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if top_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = (((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 3 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if topright_type
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = (((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int + 4 as ::core::ffi::c_int
                                    - 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if left_type[0 as ::core::ffi::c_int as usize]
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = (((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 0 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if left_type[0 as ::core::ffi::c_int as usize]
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = (((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 1 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if left_type[1 as ::core::ffi::c_int as usize]
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = (((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                            if left_type[1 as ::core::ffi::c_int as usize]
                                & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
                                && (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
                            {
                                (*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] = ((*sl)
                                    .ref_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize] as ::core::ffi::c_int >> 1 as ::core::ffi::c_int)
                                    as int8_t;
                                (*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = ((*sl)
                                    .mv_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as int16_t;
                                (*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize] = (((*sl)
                                    .mvd_cache[list
                                    as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                    + 3 as ::core::ffi::c_int * 8 as ::core::ffi::c_int)
                                    as usize][1 as ::core::ffi::c_int as usize]
                                    as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                                    as uint8_t;
                            }
                        }
                    }
                }
            }
            list += 1;
        }
    }
    (*sl).neighbor_transform_size = (top_type & 0x1000000 as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int
        + (left_type[0 as ::core::ffi::c_int as usize] & 0x1000000 as ::core::ffi::c_int
            != 0) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn get_chroma_qp(
    mut pps: *const PPS,
    mut t: ::core::ffi::c_int,
    mut qscale: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return (*pps).chroma_qp_table[t as usize][qscale as usize] as ::core::ffi::c_int;
}
unsafe extern "C" fn decode_mb_skip(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mb_xy: ::core::ffi::c_int = (*sl).mb_xy;
    let mut mb_type: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    memset(
        &raw mut *(*h).non_zero_count.offset(mb_xy as isize) as *mut uint8_t
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        48 as size_t,
    );
    if (*sl).mb_field_decoding_flag != 0 {
        mb_type |= (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
    }
    if (*sl).slice_type_nos == AV_PICTURE_TYPE_B as ::core::ffi::c_int {
        mb_type
            |= (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                | ((1 as ::core::ffi::c_int) << 14 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int)
                | (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int;
        if (*sl).direct_spatial_mv_pred != 0 {
            fill_decode_neighbors(h, sl, mb_type);
            fill_decode_caches(h, sl, mb_type);
        }
        ff_h264_pred_direct_motion(h, sl, &raw mut mb_type);
        mb_type |= (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int;
    } else {
        mb_type
            |= (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int;
        fill_decode_neighbors(h, sl, mb_type);
        pred_pskip_motion(h, sl);
    }
    write_back_motion(h, sl, mb_type);
    *(*h).cur_pic.mb_type.offset(mb_xy as isize) = mb_type as uint32_t;
    *(*h).cur_pic.qscale_table.offset(mb_xy as isize) = (*sl).qscale as int8_t;
    *(*h).slice_table.offset(mb_xy as isize) = (*sl).slice_num as uint16_t;
    (*sl).prev_mb_skipped = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_init_cabac_states(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut tab: *const [int8_t; 2] = ::core::ptr::null::<[int8_t; 2]>();
    let slice_qp: ::core::ffi::c_int = av_clip_c(
        (*sl).qscale
            - 6 as ::core::ffi::c_int
                * ((*(*h).ps.sps).bit_depth_luma - 8 as ::core::ffi::c_int),
        0 as ::core::ffi::c_int,
        51 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
    if (*sl).slice_type_nos == AV_PICTURE_TYPE_I as ::core::ffi::c_int {
        tab = &raw const cabac_context_init_I as *const [int8_t; 2]
            as *const [int8_t; 2];
    } else {
        tab = &raw const *(&raw const cabac_context_init_PB
            as *const [[int8_t; 2]; 1024])
            .offset((*sl).cabac_init_idc as isize) as *const [int8_t; 2]
            as *const [int8_t; 2];
    }
    i = 0 as ::core::ffi::c_int;
    while i < 1024 as ::core::ffi::c_int {
        let mut pre: ::core::ffi::c_int = 2 as ::core::ffi::c_int
            * (((*tab.offset(i as isize))[0 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int * slice_qp >> 4 as ::core::ffi::c_int)
                + (*tab.offset(i as isize))[1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int) - 127 as ::core::ffi::c_int;
        pre ^= pre >> 31 as ::core::ffi::c_int;
        if pre > 124 as ::core::ffi::c_int {
            pre = 124 as ::core::ffi::c_int + (pre & 1 as ::core::ffi::c_int);
        }
        (*sl).cabac_state[i as usize] = pre as uint8_t;
        i += 1;
    }
}
#[inline(always)]
unsafe extern "C" fn pack8to16(
    mut a: ::core::ffi::c_uint,
    mut b: ::core::ffi::c_uint,
) -> uint16_t {
    return (a & 0xff as ::core::ffi::c_uint).wrapping_add(b << 8 as ::core::ffi::c_int)
        as uint16_t;
}
unsafe extern "C" fn decode_cabac_field_decoding_flag(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) -> ::core::ffi::c_int {
    let mbb_xy: ::core::ffi::c_int = (*sl).mb_xy
        - 2 as ::core::ffi::c_int * (*h).mb_stride;
    let mut ctx: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    ctx = ctx
        .wrapping_add(
            ((*sl).mb_field_decoding_flag & ((*sl).mb_x != 0) as ::core::ffi::c_int)
                as ::core::ffi::c_ulong,
        );
    ctx = ctx
        .wrapping_add(
            (*(*h).cur_pic.mb_type.offset(mbb_xy as isize) >> 7 as ::core::ffi::c_int
                & (*(*h).slice_table.offset(mbb_xy as isize) as ::core::ffi::c_int
                    == (*sl).slice_num) as ::core::ffi::c_int as uint32_t)
                as ::core::ffi::c_ulong,
        );
    return get_cabac_noinline(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset(70 as ::core::ffi::c_int as isize)
            .offset(ctx as isize) as *mut uint8_t,
    );
}
unsafe extern "C" fn decode_cabac_intra_mb_type(
    mut sl: *mut H264SliceContext,
    mut ctx_base: ::core::ffi::c_int,
    mut intra_slice: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut state: *mut uint8_t = (&raw mut (*sl).cabac_state as *mut uint8_t)
        .offset(ctx_base as isize) as *mut uint8_t;
    let mut mb_type: ::core::ffi::c_int = 0;
    if intra_slice != 0 {
        let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*sl).left_type[0 as ::core::ffi::c_int as usize]
            & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) != 0
        {
            ctx += 1;
        }
        if (*sl).top_type
            & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) != 0
        {
            ctx += 1;
        }
        if get_cabac_noinline(
            &raw mut (*sl).cabac,
            state.offset(ctx as isize) as *mut uint8_t,
        ) == 0 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        state = state.offset(2 as ::core::ffi::c_int as isize);
    } else if get_cabac_noinline(&raw mut (*sl).cabac, state) == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int
    }
    if get_cabac_terminate(&raw mut (*sl).cabac) != 0 {
        return 25 as ::core::ffi::c_int;
    }
    mb_type = 1 as ::core::ffi::c_int;
    mb_type
        += 12 as ::core::ffi::c_int
            * get_cabac_noinline(
                &raw mut (*sl).cabac,
                state.offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t,
            );
    if get_cabac_noinline(
        &raw mut (*sl).cabac,
        state.offset(2 as ::core::ffi::c_int as isize) as *mut uint8_t,
    ) != 0
    {
        mb_type
            += 4 as ::core::ffi::c_int
                + 4 as ::core::ffi::c_int
                    * get_cabac_noinline(
                        &raw mut (*sl).cabac,
                        state.offset((2 as ::core::ffi::c_int + intra_slice) as isize)
                            as *mut uint8_t,
                    );
    }
    mb_type
        += 2 as ::core::ffi::c_int
            * get_cabac_noinline(
                &raw mut (*sl).cabac,
                state.offset((3 as ::core::ffi::c_int + intra_slice) as isize)
                    as *mut uint8_t,
            );
    mb_type
        += 1 as ::core::ffi::c_int
            * get_cabac_noinline(
                &raw mut (*sl).cabac,
                state
                    .offset(
                        (3 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * intra_slice)
                            as isize,
                    ) as *mut uint8_t,
            );
    return mb_type;
}
unsafe extern "C" fn decode_cabac_mb_skip(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut mba_xy: ::core::ffi::c_int = 0;
    let mut mbb_xy: ::core::ffi::c_int = 0;
    let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*h).mb_aff_frame != 0 {
        let mut mb_xy: ::core::ffi::c_int = mb_x
            + (mb_y & !(1 as ::core::ffi::c_int)) * (*h).mb_stride;
        mba_xy = mb_xy - 1 as ::core::ffi::c_int;
        if mb_y & 1 as ::core::ffi::c_int != 0
            && *(*h).slice_table.offset(mba_xy as isize) as ::core::ffi::c_int
                == (*sl).slice_num
            && (*sl).mb_field_decoding_flag
                == (*(*h).cur_pic.mb_type.offset(mba_xy as isize)
                    & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint32_t
                    != 0) as ::core::ffi::c_int
        {
            mba_xy += (*h).mb_stride;
        }
        if (*sl).mb_field_decoding_flag != 0 {
            mbb_xy = mb_xy - (*h).mb_stride;
            if mb_y & 1 as ::core::ffi::c_int == 0
                && *(*h).slice_table.offset(mbb_xy as isize) as ::core::ffi::c_int
                    == (*sl).slice_num
                && *(*h).cur_pic.mb_type.offset(mbb_xy as isize)
                    & ((1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int) as uint32_t
                    != 0
            {
                mbb_xy -= (*h).mb_stride;
            }
        } else {
            mbb_xy = mb_x + (mb_y - 1 as ::core::ffi::c_int) * (*h).mb_stride;
        }
    } else {
        let mut mb_xy_0: ::core::ffi::c_int = (*sl).mb_xy;
        mba_xy = mb_xy_0 - 1 as ::core::ffi::c_int;
        mbb_xy = mb_xy_0
            - ((*h).mb_stride
                << ((*h).picture_structure != 3 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
    }
    if *(*h).slice_table.offset(mba_xy as isize) as ::core::ffi::c_int == (*sl).slice_num
        && *(*h).cur_pic.mb_type.offset(mba_xy as isize)
            & ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as uint32_t == 0
    {
        ctx += 1;
    }
    if *(*h).slice_table.offset(mbb_xy as isize) as ::core::ffi::c_int == (*sl).slice_num
        && *(*h).cur_pic.mb_type.offset(mbb_xy as isize)
            & ((1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int) as uint32_t == 0
    {
        ctx += 1;
    }
    if (*sl).slice_type_nos == AV_PICTURE_TYPE_B as ::core::ffi::c_int {
        ctx += 13 as ::core::ffi::c_int;
    }
    return get_cabac_noinline(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset((11 as ::core::ffi::c_int + ctx) as isize) as *mut uint8_t,
    );
}
unsafe extern "C" fn decode_cabac_mb_intra4x4_pred_mode(
    mut sl: *mut H264SliceContext,
    mut pred_mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut mode: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if get_cabac(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset(68 as ::core::ffi::c_int as isize) as *mut uint8_t,
    ) != 0
    {
        return pred_mode;
    }
    mode
        += 1 as ::core::ffi::c_int
            * get_cabac(
                &raw mut (*sl).cabac,
                (&raw mut (*sl).cabac_state as *mut uint8_t)
                    .offset(69 as ::core::ffi::c_int as isize) as *mut uint8_t,
            );
    mode
        += 2 as ::core::ffi::c_int
            * get_cabac(
                &raw mut (*sl).cabac,
                (&raw mut (*sl).cabac_state as *mut uint8_t)
                    .offset(69 as ::core::ffi::c_int as isize) as *mut uint8_t,
            );
    mode
        += 4 as ::core::ffi::c_int
            * get_cabac(
                &raw mut (*sl).cabac,
                (&raw mut (*sl).cabac_state as *mut uint8_t)
                    .offset(69 as ::core::ffi::c_int as isize) as *mut uint8_t,
            );
    return mode + (mode >= pred_mode) as ::core::ffi::c_int;
}
unsafe extern "C" fn decode_cabac_mb_chroma_pre_mode(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) -> ::core::ffi::c_int {
    let mba_xy: ::core::ffi::c_int = (*sl).left_mb_xy[0 as ::core::ffi::c_int as usize];
    let mbb_xy: ::core::ffi::c_int = (*sl).top_mb_xy;
    let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*sl).left_type[0 as ::core::ffi::c_int as usize] != 0
        && *(*h).chroma_pred_mode_table.offset(mba_xy as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        ctx += 1;
    }
    if (*sl).top_type != 0
        && *(*h).chroma_pred_mode_table.offset(mbb_xy as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        ctx += 1;
    }
    if get_cabac_noinline(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset((64 as ::core::ffi::c_int + ctx) as isize) as *mut uint8_t,
    ) == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if get_cabac_noinline(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset((64 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
            as *mut uint8_t,
    ) == 0 as ::core::ffi::c_int
    {
        return 1 as ::core::ffi::c_int;
    }
    if get_cabac_noinline(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset((64 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
            as *mut uint8_t,
    ) == 0 as ::core::ffi::c_int
    {
        return 2 as ::core::ffi::c_int
    } else {
        return 3 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn decode_cabac_mb_cbp_luma(
    mut sl: *mut H264SliceContext,
) -> ::core::ffi::c_int {
    let mut cbp_b: ::core::ffi::c_int = 0;
    let mut cbp_a: ::core::ffi::c_int = 0;
    let mut ctx: ::core::ffi::c_int = 0;
    let mut cbp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    cbp_a = (*sl).left_cbp;
    cbp_b = (*sl).top_cbp;
    ctx = (cbp_a & 0x2 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * (cbp_b & 0x4 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    cbp
        += get_cabac_noinline(
            &raw mut (*sl).cabac,
            (&raw mut (*sl).cabac_state as *mut uint8_t)
                .offset((73 as ::core::ffi::c_int + ctx) as isize) as *mut uint8_t,
        );
    ctx = (cbp & 0x1 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * (cbp_b & 0x8 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    cbp
        += get_cabac_noinline(
            &raw mut (*sl).cabac,
            (&raw mut (*sl).cabac_state as *mut uint8_t)
                .offset((73 as ::core::ffi::c_int + ctx) as isize) as *mut uint8_t,
        ) << 1 as ::core::ffi::c_int;
    ctx = (cbp_a & 0x8 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * (cbp & 0x1 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    cbp
        += get_cabac_noinline(
            &raw mut (*sl).cabac,
            (&raw mut (*sl).cabac_state as *mut uint8_t)
                .offset((73 as ::core::ffi::c_int + ctx) as isize) as *mut uint8_t,
        ) << 2 as ::core::ffi::c_int;
    ctx = (cbp & 0x4 as ::core::ffi::c_int == 0) as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
            * (cbp & 0x2 as ::core::ffi::c_int == 0) as ::core::ffi::c_int;
    cbp
        += get_cabac_noinline(
            &raw mut (*sl).cabac,
            (&raw mut (*sl).cabac_state as *mut uint8_t)
                .offset((73 as ::core::ffi::c_int + ctx) as isize) as *mut uint8_t,
        ) << 3 as ::core::ffi::c_int;
    return cbp;
}
unsafe extern "C" fn decode_cabac_mb_cbp_chroma(
    mut sl: *mut H264SliceContext,
) -> ::core::ffi::c_int {
    let mut ctx: ::core::ffi::c_int = 0;
    let mut cbp_a: ::core::ffi::c_int = 0;
    let mut cbp_b: ::core::ffi::c_int = 0;
    cbp_a = (*sl).left_cbp >> 4 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int;
    cbp_b = (*sl).top_cbp >> 4 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int;
    ctx = 0 as ::core::ffi::c_int;
    if cbp_a > 0 as ::core::ffi::c_int {
        ctx += 1;
    }
    if cbp_b > 0 as ::core::ffi::c_int {
        ctx += 2 as ::core::ffi::c_int;
    }
    if get_cabac_noinline(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset((77 as ::core::ffi::c_int + ctx) as isize) as *mut uint8_t,
    ) == 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    ctx = 4 as ::core::ffi::c_int;
    if cbp_a == 2 as ::core::ffi::c_int {
        ctx += 1;
    }
    if cbp_b == 2 as ::core::ffi::c_int {
        ctx += 2 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int
        + get_cabac_noinline(
            &raw mut (*sl).cabac,
            (&raw mut (*sl).cabac_state as *mut uint8_t)
                .offset((77 as ::core::ffi::c_int + ctx) as isize) as *mut uint8_t,
        );
}
unsafe extern "C" fn decode_cabac_p_mb_sub_type(
    mut sl: *mut H264SliceContext,
) -> ::core::ffi::c_int {
    if get_cabac(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset(21 as ::core::ffi::c_int as isize) as *mut uint8_t,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if get_cabac(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset(22 as ::core::ffi::c_int as isize) as *mut uint8_t,
    ) == 0
    {
        return 1 as ::core::ffi::c_int;
    }
    if get_cabac(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset(23 as ::core::ffi::c_int as isize) as *mut uint8_t,
    ) != 0
    {
        return 2 as ::core::ffi::c_int;
    }
    return 3 as ::core::ffi::c_int;
}
unsafe extern "C" fn decode_cabac_b_mb_sub_type(
    mut sl: *mut H264SliceContext,
) -> ::core::ffi::c_int {
    let mut type_0: ::core::ffi::c_int = 0;
    if get_cabac(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset(36 as ::core::ffi::c_int as isize) as *mut uint8_t,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if get_cabac(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset(37 as ::core::ffi::c_int as isize) as *mut uint8_t,
    ) == 0
    {
        return 1 as ::core::ffi::c_int
            + get_cabac(
                &raw mut (*sl).cabac,
                (&raw mut (*sl).cabac_state as *mut uint8_t)
                    .offset(39 as ::core::ffi::c_int as isize) as *mut uint8_t,
            );
    }
    type_0 = 3 as ::core::ffi::c_int;
    if get_cabac(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset(38 as ::core::ffi::c_int as isize) as *mut uint8_t,
    ) != 0
    {
        if get_cabac(
            &raw mut (*sl).cabac,
            (&raw mut (*sl).cabac_state as *mut uint8_t)
                .offset(39 as ::core::ffi::c_int as isize) as *mut uint8_t,
        ) != 0
        {
            return 11 as ::core::ffi::c_int
                + get_cabac(
                    &raw mut (*sl).cabac,
                    (&raw mut (*sl).cabac_state as *mut uint8_t)
                        .offset(39 as ::core::ffi::c_int as isize) as *mut uint8_t,
                );
        }
        type_0 += 4 as ::core::ffi::c_int;
    }
    type_0
        += 2 as ::core::ffi::c_int
            * get_cabac(
                &raw mut (*sl).cabac,
                (&raw mut (*sl).cabac_state as *mut uint8_t)
                    .offset(39 as ::core::ffi::c_int as isize) as *mut uint8_t,
            );
    type_0
        += get_cabac(
            &raw mut (*sl).cabac,
            (&raw mut (*sl).cabac_state as *mut uint8_t)
                .offset(39 as ::core::ffi::c_int as isize) as *mut uint8_t,
        );
    return type_0;
}
unsafe extern "C" fn decode_cabac_mb_ref(
    mut sl: *mut H264SliceContext,
    mut list: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut refa: ::core::ffi::c_int = (*sl)
        .ref_cache[list
        as usize][(scan8[n as usize] as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
        as usize] as ::core::ffi::c_int;
    let mut refb: ::core::ffi::c_int = (*sl)
        .ref_cache[list
        as usize][(scan8[n as usize] as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
        as usize] as ::core::ffi::c_int;
    let mut ref_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*sl).slice_type_nos == AV_PICTURE_TYPE_B as ::core::ffi::c_int {
        if refa > 0 as ::core::ffi::c_int
            && (*sl)
                .direct_cache[(scan8[n as usize] as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int == 0
        {
            ctx += 1;
        }
        if refb > 0 as ::core::ffi::c_int
            && (*sl)
                .direct_cache[(scan8[n as usize] as ::core::ffi::c_int
                - 8 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int
                    >> 1 as ::core::ffi::c_int == 0
        {
            ctx += 2 as ::core::ffi::c_int;
        }
    } else {
        if refa > 0 as ::core::ffi::c_int {
            ctx += 1;
        }
        if refb > 0 as ::core::ffi::c_int {
            ctx += 2 as ::core::ffi::c_int;
        }
    }
    while get_cabac(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset((54 as ::core::ffi::c_int + ctx) as isize) as *mut uint8_t,
    ) != 0
    {
        ref_0 += 1;
        ctx = (ctx >> 2 as ::core::ffi::c_int) + 4 as ::core::ffi::c_int;
        if ref_0 >= 32 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    return ref_0;
}
unsafe extern "C" fn decode_cabac_mb_mvd(
    mut sl: *mut H264SliceContext,
    mut ctxbase: ::core::ffi::c_int,
    mut amvd: ::core::ffi::c_int,
    mut mvda: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut mvd: ::core::ffi::c_int = 0;
    if get_cabac(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset(
                (ctxbase
                    + (amvd - 3 as ::core::ffi::c_int
                        >> (8 as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            )
                            .wrapping_sub(1 as usize))
                    + (amvd - 33 as ::core::ffi::c_int
                        >> (8 as usize)
                            .wrapping_mul(
                                ::core::mem::size_of::<::core::ffi::c_int>() as usize,
                            )
                            .wrapping_sub(1 as usize)) + 2 as ::core::ffi::c_int)
                    as isize,
            ) as *mut uint8_t,
    ) == 0
    {
        *mvda = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    mvd = 1 as ::core::ffi::c_int;
    ctxbase += 3 as ::core::ffi::c_int;
    while mvd < 9 as ::core::ffi::c_int
        && get_cabac(
            &raw mut (*sl).cabac,
            (&raw mut (*sl).cabac_state as *mut uint8_t).offset(ctxbase as isize)
                as *mut uint8_t,
        ) != 0
    {
        if mvd < 4 as ::core::ffi::c_int {
            ctxbase += 1;
        }
        mvd += 1;
    }
    if mvd >= 9 as ::core::ffi::c_int {
        let mut k: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
        while get_cabac_bypass(&raw mut (*sl).cabac) != 0 {
            mvd += (1 as ::core::ffi::c_int) << k;
            k += 1;
            if k > 24 as ::core::ffi::c_int {
                av_log(
                    (*(*sl).h264).avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"overflow in decode_cabac_mb_mvd\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                return -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int;
            }
        }
        loop {
            let fresh28 = k;
            k = k - 1;
            if !(fresh28 != 0) {
                break;
            }
            mvd += get_cabac_bypass(&raw mut (*sl).cabac) << k;
        }
        *mvda = if mvd < 70 as ::core::ffi::c_int {
            mvd
        } else {
            70 as ::core::ffi::c_int
        };
    } else {
        *mvda = mvd;
    }
    return get_cabac_bypass_sign(&raw mut (*sl).cabac, -mvd);
}
#[inline(always)]
unsafe extern "C" fn get_cabac_cbf_ctx(
    mut sl: *mut H264SliceContext,
    mut cat: ::core::ffi::c_int,
    mut idx: ::core::ffi::c_int,
    mut max_coeff: ::core::ffi::c_int,
    mut is_dc: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nza: ::core::ffi::c_int = 0;
    let mut nzb: ::core::ffi::c_int = 0;
    let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut base_ctx: [uint16_t; 14] = [
        85 as ::core::ffi::c_int as uint16_t,
        89 as ::core::ffi::c_int as uint16_t,
        93 as ::core::ffi::c_int as uint16_t,
        97 as ::core::ffi::c_int as uint16_t,
        101 as ::core::ffi::c_int as uint16_t,
        1012 as ::core::ffi::c_int as uint16_t,
        460 as ::core::ffi::c_int as uint16_t,
        464 as ::core::ffi::c_int as uint16_t,
        468 as ::core::ffi::c_int as uint16_t,
        1016 as ::core::ffi::c_int as uint16_t,
        472 as ::core::ffi::c_int as uint16_t,
        476 as ::core::ffi::c_int as uint16_t,
        480 as ::core::ffi::c_int as uint16_t,
        1020 as ::core::ffi::c_int as uint16_t,
    ];
    if is_dc != 0 {
        if cat == 3 as ::core::ffi::c_int {
            idx -= 49 as ::core::ffi::c_int;
            nza = (*sl).left_cbp >> 6 as ::core::ffi::c_int + idx
                & 0x1 as ::core::ffi::c_int;
            nzb = (*sl).top_cbp >> 6 as ::core::ffi::c_int + idx
                & 0x1 as ::core::ffi::c_int;
        } else {
            idx -= 48 as ::core::ffi::c_int;
            nza = (*sl).left_cbp & (0x100 as ::core::ffi::c_int) << idx;
            nzb = (*sl).top_cbp & (0x100 as ::core::ffi::c_int) << idx;
        }
    } else {
        nza = (*sl)
            .non_zero_count_cache[(scan8[idx as usize] as ::core::ffi::c_int
            - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
        nzb = (*sl)
            .non_zero_count_cache[(scan8[idx as usize] as ::core::ffi::c_int
            - 8 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
    }
    if nza > 0 as ::core::ffi::c_int {
        ctx += 1;
    }
    if nzb > 0 as ::core::ffi::c_int {
        ctx += 2 as ::core::ffi::c_int;
    }
    return base_ctx[cat as usize] as ::core::ffi::c_int + ctx;
}
#[inline(always)]
unsafe extern "C" fn decode_cabac_residual_internal(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut block: *mut int16_t,
    mut cat: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut scantable: *const uint8_t,
    mut qmul: *const uint32_t,
    mut max_coeff: ::core::ffi::c_int,
    mut is_dc: ::core::ffi::c_int,
    mut chroma422: ::core::ffi::c_int,
) {
    static mut significant_coeff_flag_offset: [[::core::ffi::c_int; 14]; 2] = [
        [
            105 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            105 as ::core::ffi::c_int + 15 as ::core::ffi::c_int,
            105 as ::core::ffi::c_int + 29 as ::core::ffi::c_int,
            105 as ::core::ffi::c_int + 44 as ::core::ffi::c_int,
            105 as ::core::ffi::c_int + 47 as ::core::ffi::c_int,
            402 as ::core::ffi::c_int,
            484 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            484 as ::core::ffi::c_int + 15 as ::core::ffi::c_int,
            484 as ::core::ffi::c_int + 29 as ::core::ffi::c_int,
            660 as ::core::ffi::c_int,
            528 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            528 as ::core::ffi::c_int + 15 as ::core::ffi::c_int,
            528 as ::core::ffi::c_int + 29 as ::core::ffi::c_int,
            718 as ::core::ffi::c_int,
        ],
        [
            277 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            277 as ::core::ffi::c_int + 15 as ::core::ffi::c_int,
            277 as ::core::ffi::c_int + 29 as ::core::ffi::c_int,
            277 as ::core::ffi::c_int + 44 as ::core::ffi::c_int,
            277 as ::core::ffi::c_int + 47 as ::core::ffi::c_int,
            436 as ::core::ffi::c_int,
            776 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            776 as ::core::ffi::c_int + 15 as ::core::ffi::c_int,
            776 as ::core::ffi::c_int + 29 as ::core::ffi::c_int,
            675 as ::core::ffi::c_int,
            820 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            820 as ::core::ffi::c_int + 15 as ::core::ffi::c_int,
            820 as ::core::ffi::c_int + 29 as ::core::ffi::c_int,
            733 as ::core::ffi::c_int,
        ],
    ];
    static mut last_coeff_flag_offset: [[::core::ffi::c_int; 14]; 2] = [
        [
            166 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            166 as ::core::ffi::c_int + 15 as ::core::ffi::c_int,
            166 as ::core::ffi::c_int + 29 as ::core::ffi::c_int,
            166 as ::core::ffi::c_int + 44 as ::core::ffi::c_int,
            166 as ::core::ffi::c_int + 47 as ::core::ffi::c_int,
            417 as ::core::ffi::c_int,
            572 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            572 as ::core::ffi::c_int + 15 as ::core::ffi::c_int,
            572 as ::core::ffi::c_int + 29 as ::core::ffi::c_int,
            690 as ::core::ffi::c_int,
            616 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            616 as ::core::ffi::c_int + 15 as ::core::ffi::c_int,
            616 as ::core::ffi::c_int + 29 as ::core::ffi::c_int,
            748 as ::core::ffi::c_int,
        ],
        [
            338 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            338 as ::core::ffi::c_int + 15 as ::core::ffi::c_int,
            338 as ::core::ffi::c_int + 29 as ::core::ffi::c_int,
            338 as ::core::ffi::c_int + 44 as ::core::ffi::c_int,
            338 as ::core::ffi::c_int + 47 as ::core::ffi::c_int,
            451 as ::core::ffi::c_int,
            864 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            864 as ::core::ffi::c_int + 15 as ::core::ffi::c_int,
            864 as ::core::ffi::c_int + 29 as ::core::ffi::c_int,
            699 as ::core::ffi::c_int,
            908 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
            908 as ::core::ffi::c_int + 15 as ::core::ffi::c_int,
            908 as ::core::ffi::c_int + 29 as ::core::ffi::c_int,
            757 as ::core::ffi::c_int,
        ],
    ];
    static mut coeff_abs_level_m1_offset: [::core::ffi::c_int; 14] = [
        227 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
        227 as ::core::ffi::c_int + 10 as ::core::ffi::c_int,
        227 as ::core::ffi::c_int + 20 as ::core::ffi::c_int,
        227 as ::core::ffi::c_int + 30 as ::core::ffi::c_int,
        227 as ::core::ffi::c_int + 39 as ::core::ffi::c_int,
        426 as ::core::ffi::c_int,
        952 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
        952 as ::core::ffi::c_int + 10 as ::core::ffi::c_int,
        952 as ::core::ffi::c_int + 20 as ::core::ffi::c_int,
        708 as ::core::ffi::c_int,
        982 as ::core::ffi::c_int + 0 as ::core::ffi::c_int,
        982 as ::core::ffi::c_int + 10 as ::core::ffi::c_int,
        982 as ::core::ffi::c_int + 20 as ::core::ffi::c_int,
        766 as ::core::ffi::c_int,
    ];
    static mut significant_coeff_flag_offset_8x8: [[uint8_t; 63]; 2] = [
        [
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            13 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            13 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            13 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
        ],
        [
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            13 as ::core::ffi::c_int as uint8_t,
            13 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            13 as ::core::ffi::c_int as uint8_t,
            13 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
            14 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    static mut sig_coeff_offset_dc: [uint8_t; 7] = [
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
    ];
    static mut coeff_abs_level1_ctx: [uint8_t; 8] = [
        1 as ::core::ffi::c_int as uint8_t,
        2 as ::core::ffi::c_int as uint8_t,
        3 as ::core::ffi::c_int as uint8_t,
        4 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
        0 as ::core::ffi::c_int as uint8_t,
    ];
    static mut coeff_abs_levelgt1_ctx: [[uint8_t; 8]; 2] = [
        [
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
        ],
        [
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    static mut coeff_abs_level_transition: [[uint8_t; 8]; 2] = [
        [
            1 as ::core::ffi::c_int as uint8_t,
            2 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
        ],
        [
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            5 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    let mut index: [::core::ffi::c_int; 64] = [0; 64];
    let mut last: ::core::ffi::c_int = 0;
    let mut coeff_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut node_ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut significant_coeff_ctx_base: *mut uint8_t = ::core::ptr::null_mut::<
        uint8_t,
    >();
    let mut last_coeff_ctx_base: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut abs_level_m1_ctx_base: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut cc: CABACContext = CABACContext {
        low: 0,
        range: 0,
        bytestream_start: ::core::ptr::null::<uint8_t>(),
        bytestream: ::core::ptr::null::<uint8_t>(),
        bytestream_end: ::core::ptr::null::<uint8_t>(),
    };
    cc.range = (*sl).cabac.range;
    cc.low = (*sl).cabac.low;
    cc.bytestream = (*sl).cabac.bytestream;
    cc.bytestream_end = (*sl).cabac.bytestream_end;
    significant_coeff_ctx_base = (&raw mut (*sl).cabac_state as *mut uint8_t)
        .offset(
            significant_coeff_flag_offset[(*sl).mb_field_decoding_flag
                as usize][cat as usize] as isize,
        );
    last_coeff_ctx_base = (&raw mut (*sl).cabac_state as *mut uint8_t)
        .offset(
            last_coeff_flag_offset[(*sl).mb_field_decoding_flag as usize][cat as usize]
                as isize,
        );
    abs_level_m1_ctx_base = (&raw mut (*sl).cabac_state as *mut uint8_t)
        .offset(coeff_abs_level_m1_offset[cat as usize] as isize);
    if is_dc == 0 && max_coeff == 64 as ::core::ffi::c_int {
        let mut sig_off: *const uint8_t = &raw const *(&raw const significant_coeff_flag_offset_8x8
            as *const [uint8_t; 63])
            .offset((*sl).mb_field_decoding_flag as isize) as *const uint8_t;
        last = 0 as ::core::ffi::c_int;
        while last < 63 as ::core::ffi::c_int {
            let mut sig_ctx: *mut uint8_t = significant_coeff_ctx_base
                .offset(*sig_off.offset(last as isize) as ::core::ffi::c_int as isize);
            if get_cabac(&raw mut cc, sig_ctx) != 0 {
                let mut last_ctx: *mut uint8_t = last_coeff_ctx_base
                    .offset(
                        *ff_h264_last_coeff_flag_offset_8x8.offset(last as isize)
                            as ::core::ffi::c_int as isize,
                    );
                let fresh18 = coeff_count;
                coeff_count = coeff_count + 1;
                index[fresh18 as usize] = last;
                if get_cabac(&raw mut cc, last_ctx) != 0 {
                    last = max_coeff;
                    break;
                }
            }
            last += 1;
        }
        if last == max_coeff - 1 as ::core::ffi::c_int {
            let fresh19 = coeff_count;
            coeff_count = coeff_count + 1;
            index[fresh19 as usize] = last;
        }
    } else if is_dc != 0 && chroma422 != 0 {
        last = 0 as ::core::ffi::c_int;
        while last < 7 as ::core::ffi::c_int {
            let mut sig_ctx_0: *mut uint8_t = significant_coeff_ctx_base
                .offset(
                    sig_coeff_offset_dc[last as usize] as ::core::ffi::c_int as isize,
                );
            if get_cabac(&raw mut cc, sig_ctx_0) != 0 {
                let mut last_ctx_0: *mut uint8_t = last_coeff_ctx_base
                    .offset(
                        sig_coeff_offset_dc[last as usize] as ::core::ffi::c_int as isize,
                    );
                let fresh20 = coeff_count;
                coeff_count = coeff_count + 1;
                index[fresh20 as usize] = last;
                if get_cabac(&raw mut cc, last_ctx_0) != 0 {
                    last = max_coeff;
                    break;
                }
            }
            last += 1;
        }
        if last == max_coeff - 1 as ::core::ffi::c_int {
            let fresh21 = coeff_count;
            coeff_count = coeff_count + 1;
            index[fresh21 as usize] = last;
        }
    } else {
        last = 0 as ::core::ffi::c_int;
        while last < max_coeff - 1 as ::core::ffi::c_int {
            let mut sig_ctx_1: *mut uint8_t = significant_coeff_ctx_base
                .offset(last as isize);
            if get_cabac(&raw mut cc, sig_ctx_1) != 0 {
                let mut last_ctx_1: *mut uint8_t = last_coeff_ctx_base
                    .offset(last as isize);
                let fresh22 = coeff_count;
                coeff_count = coeff_count + 1;
                index[fresh22 as usize] = last;
                if get_cabac(&raw mut cc, last_ctx_1) != 0 {
                    last = max_coeff;
                    break;
                }
            }
            last += 1;
        }
        if last == max_coeff - 1 as ::core::ffi::c_int {
            let fresh23 = coeff_count;
            coeff_count = coeff_count + 1;
            index[fresh23 as usize] = last;
        }
    }
    if is_dc != 0 {
        if cat == 3 as ::core::ffi::c_int {
            let ref mut fresh24 = *(*h).cbp_table.offset((*sl).mb_xy as isize);
            *fresh24 = (*fresh24 as ::core::ffi::c_int
                | (0x40 as ::core::ffi::c_int) << n - 49 as ::core::ffi::c_int)
                as uint16_t;
        } else {
            let ref mut fresh25 = *(*h).cbp_table.offset((*sl).mb_xy as isize);
            *fresh25 = (*fresh25 as ::core::ffi::c_int
                | (0x100 as ::core::ffi::c_int) << n - 48 as ::core::ffi::c_int)
                as uint16_t;
        }
        (*sl).non_zero_count_cache[scan8[n as usize] as usize] = coeff_count as uint8_t;
    } else if max_coeff == 64 as ::core::ffi::c_int {
        fill_rectangle(
            (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                .offset(
                    *(&raw const scan8 as *const uint8_t).offset(n as isize) as isize,
                ) as *mut uint8_t as *mut ::core::ffi::c_void,
            2 as ::core::ffi::c_int,
            2 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            coeff_count as uint32_t,
            1 as ::core::ffi::c_int,
        );
    } else {
        (*sl).non_zero_count_cache[scan8[n as usize] as usize] = coeff_count as uint8_t;
    }
    if (*h).pixel_shift != 0 {
        loop {
            let mut ctx: *mut uint8_t = abs_level_m1_ctx_base
                .offset(
                    coeff_abs_level1_ctx[node_ctx as usize] as ::core::ffi::c_int
                        as isize,
                );
            coeff_count -= 1;
            let mut j: ::core::ffi::c_int = *scantable
                .offset(index[coeff_count as usize] as isize) as ::core::ffi::c_int;
            if get_cabac(&raw mut cc, ctx) == 0 as ::core::ffi::c_int {
                node_ctx = coeff_abs_level_transition[0 as ::core::ffi::c_int
                    as usize][node_ctx as usize] as ::core::ffi::c_int;
                if is_dc != 0 {
                    *(block as *mut int32_t).offset(j as isize) = get_cabac_bypass_sign(
                        &raw mut cc,
                        -(1 as ::core::ffi::c_int),
                    ) as int32_t;
                } else {
                    *(block as *mut int32_t).offset(j as isize) = (get_cabac_bypass_sign(
                        &raw mut cc,
                        (*qmul.offset(j as isize)).wrapping_neg() as ::core::ffi::c_int,
                    ) + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as int32_t;
                }
            } else {
                let mut coeff_abs: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
                ctx = abs_level_m1_ctx_base
                    .offset(
                        coeff_abs_levelgt1_ctx[(is_dc != 0 && chroma422 != 0)
                            as ::core::ffi::c_int as usize][node_ctx as usize]
                            as ::core::ffi::c_int as isize,
                    );
                node_ctx = coeff_abs_level_transition[1 as ::core::ffi::c_int
                    as usize][node_ctx as usize] as ::core::ffi::c_int;
                while coeff_abs < 15 as ::core::ffi::c_uint
                    && get_cabac(&raw mut cc, ctx) != 0
                {
                    coeff_abs = coeff_abs.wrapping_add(1);
                }
                if coeff_abs >= 15 as ::core::ffi::c_uint {
                    let mut j_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while get_cabac_bypass(&raw mut cc) != 0
                        && j_0 < 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int
                    {
                        j_0 += 1;
                    }
                    coeff_abs = 1 as ::core::ffi::c_uint;
                    loop {
                        let fresh26 = j_0;
                        j_0 = j_0 - 1;
                        if !(fresh26 != 0) {
                            break;
                        }
                        coeff_abs = coeff_abs
                            .wrapping_add(
                                coeff_abs
                                    .wrapping_add(
                                        get_cabac_bypass(&raw mut cc) as ::core::ffi::c_uint,
                                    ),
                            );
                    }
                    coeff_abs = coeff_abs.wrapping_add(14 as ::core::ffi::c_uint);
                }
                if is_dc != 0 {
                    *(block as *mut int32_t).offset(j as isize) = get_cabac_bypass_sign(
                        &raw mut cc,
                        coeff_abs.wrapping_neg() as ::core::ffi::c_int,
                    ) as int32_t;
                } else {
                    *(block as *mut int32_t).offset(j as isize) = ((get_cabac_bypass_sign(
                        &raw mut cc,
                        coeff_abs.wrapping_neg() as ::core::ffi::c_int,
                    ) as uint32_t)
                        .wrapping_mul(*qmul.offset(j as isize))
                        .wrapping_add(32 as uint32_t) as ::core::ffi::c_int
                        >> 6 as ::core::ffi::c_int) as int32_t;
                }
            }
            if !(coeff_count != 0) {
                break;
            }
        }
    } else {
        loop {
            let mut ctx_0: *mut uint8_t = abs_level_m1_ctx_base
                .offset(
                    coeff_abs_level1_ctx[node_ctx as usize] as ::core::ffi::c_int
                        as isize,
                );
            coeff_count -= 1;
            let mut j_1: ::core::ffi::c_int = *scantable
                .offset(index[coeff_count as usize] as isize) as ::core::ffi::c_int;
            if get_cabac(&raw mut cc, ctx_0) == 0 as ::core::ffi::c_int {
                node_ctx = coeff_abs_level_transition[0 as ::core::ffi::c_int
                    as usize][node_ctx as usize] as ::core::ffi::c_int;
                if is_dc != 0 {
                    *block.offset(j_1 as isize) = get_cabac_bypass_sign(
                        &raw mut cc,
                        -(1 as ::core::ffi::c_int),
                    ) as int16_t;
                } else {
                    *block.offset(j_1 as isize) = (get_cabac_bypass_sign(
                        &raw mut cc,
                        (*qmul.offset(j_1 as isize)).wrapping_neg() as ::core::ffi::c_int,
                    ) + 32 as ::core::ffi::c_int >> 6 as ::core::ffi::c_int) as int16_t;
                }
            } else {
                let mut coeff_abs_0: ::core::ffi::c_uint = 2 as ::core::ffi::c_uint;
                ctx_0 = abs_level_m1_ctx_base
                    .offset(
                        coeff_abs_levelgt1_ctx[(is_dc != 0 && chroma422 != 0)
                            as ::core::ffi::c_int as usize][node_ctx as usize]
                            as ::core::ffi::c_int as isize,
                    );
                node_ctx = coeff_abs_level_transition[1 as ::core::ffi::c_int
                    as usize][node_ctx as usize] as ::core::ffi::c_int;
                while coeff_abs_0 < 15 as ::core::ffi::c_uint
                    && get_cabac(&raw mut cc, ctx_0) != 0
                {
                    coeff_abs_0 = coeff_abs_0.wrapping_add(1);
                }
                if coeff_abs_0 >= 15 as ::core::ffi::c_uint {
                    let mut j_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while get_cabac_bypass(&raw mut cc) != 0
                        && j_2 < 16 as ::core::ffi::c_int + 7 as ::core::ffi::c_int
                    {
                        j_2 += 1;
                    }
                    coeff_abs_0 = 1 as ::core::ffi::c_uint;
                    loop {
                        let fresh27 = j_2;
                        j_2 = j_2 - 1;
                        if !(fresh27 != 0) {
                            break;
                        }
                        coeff_abs_0 = coeff_abs_0
                            .wrapping_add(
                                coeff_abs_0
                                    .wrapping_add(
                                        get_cabac_bypass(&raw mut cc) as ::core::ffi::c_uint,
                                    ),
                            );
                    }
                    coeff_abs_0 = coeff_abs_0.wrapping_add(14 as ::core::ffi::c_uint);
                }
                if is_dc != 0 {
                    *block.offset(j_1 as isize) = get_cabac_bypass_sign(
                        &raw mut cc,
                        coeff_abs_0.wrapping_neg() as ::core::ffi::c_int,
                    ) as int16_t;
                } else {
                    *block.offset(j_1 as isize) = ((get_cabac_bypass_sign(
                        &raw mut cc,
                        coeff_abs_0.wrapping_neg() as ::core::ffi::c_int,
                    ) as uint32_t)
                        .wrapping_mul(*qmul.offset(j_1 as isize))
                        .wrapping_add(32 as uint32_t) as ::core::ffi::c_int
                        >> 6 as ::core::ffi::c_int) as int16_t;
                }
            }
            if !(coeff_count != 0) {
                break;
            }
        }
    }
    (*sl).cabac.range = cc.range;
    (*sl).cabac.low = cc.low;
    (*sl).cabac.bytestream = cc.bytestream;
}
#[inline(never)]
unsafe extern "C" fn decode_cabac_residual_dc_internal(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut block: *mut int16_t,
    mut cat: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut scantable: *const uint8_t,
    mut max_coeff: ::core::ffi::c_int,
) {
    decode_cabac_residual_internal(
        h,
        sl,
        block,
        cat,
        n,
        scantable,
        ::core::ptr::null::<uint32_t>(),
        max_coeff,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
#[inline(never)]
unsafe extern "C" fn decode_cabac_residual_dc_internal_422(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut block: *mut int16_t,
    mut cat: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut scantable: *const uint8_t,
    mut max_coeff: ::core::ffi::c_int,
) {
    decode_cabac_residual_internal(
        h,
        sl,
        block,
        cat,
        n,
        scantable,
        ::core::ptr::null::<uint32_t>(),
        max_coeff,
        1 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    );
}
#[inline(never)]
unsafe extern "C" fn decode_cabac_residual_nondc_internal(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut block: *mut int16_t,
    mut cat: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut scantable: *const uint8_t,
    mut qmul: *const uint32_t,
    mut max_coeff: ::core::ffi::c_int,
) {
    decode_cabac_residual_internal(
        h,
        sl,
        block,
        cat,
        n,
        scantable,
        qmul,
        max_coeff,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn decode_cabac_residual_dc(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut block: *mut int16_t,
    mut cat: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut scantable: *const uint8_t,
    mut max_coeff: ::core::ffi::c_int,
) {
    if get_cabac(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset(
                (get_cabac_cbf_ctx
                    as unsafe extern "C" fn(
                        *mut H264SliceContext,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int)(
                    sl,
                    cat,
                    n,
                    max_coeff,
                    1 as ::core::ffi::c_int,
                ) as isize,
            ) as *mut uint8_t,
    ) == 0 as ::core::ffi::c_int
    {
        (*sl).non_zero_count_cache[scan8[n as usize] as usize] = 0 as uint8_t;
        return;
    }
    decode_cabac_residual_dc_internal(h, sl, block, cat, n, scantable, max_coeff);
}
#[inline(always)]
unsafe extern "C" fn decode_cabac_residual_dc_422(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut block: *mut int16_t,
    mut cat: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut scantable: *const uint8_t,
    mut max_coeff: ::core::ffi::c_int,
) {
    if get_cabac(
        &raw mut (*sl).cabac,
        (&raw mut (*sl).cabac_state as *mut uint8_t)
            .offset(
                (get_cabac_cbf_ctx
                    as unsafe extern "C" fn(
                        *mut H264SliceContext,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                        ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int)(
                    sl,
                    cat,
                    n,
                    max_coeff,
                    1 as ::core::ffi::c_int,
                ) as isize,
            ) as *mut uint8_t,
    ) == 0 as ::core::ffi::c_int
    {
        (*sl).non_zero_count_cache[scan8[n as usize] as usize] = 0 as uint8_t;
        return;
    }
    decode_cabac_residual_dc_internal_422(h, sl, block, cat, n, scantable, max_coeff);
}
#[inline(always)]
unsafe extern "C" fn decode_cabac_residual_nondc(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut block: *mut int16_t,
    mut cat: ::core::ffi::c_int,
    mut n: ::core::ffi::c_int,
    mut scantable: *const uint8_t,
    mut qmul: *const uint32_t,
    mut max_coeff: ::core::ffi::c_int,
) {
    if (cat != 5 as ::core::ffi::c_int
        || (*(*h).ps.sps).chroma_format_idc == 3 as ::core::ffi::c_int)
        && get_cabac(
            &raw mut (*sl).cabac,
            (&raw mut (*sl).cabac_state as *mut uint8_t)
                .offset(
                    (get_cabac_cbf_ctx
                        as unsafe extern "C" fn(
                            *mut H264SliceContext,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                            ::core::ffi::c_int,
                        ) -> ::core::ffi::c_int)(
                        sl,
                        cat,
                        n,
                        max_coeff,
                        0 as ::core::ffi::c_int,
                    ) as isize,
                ) as *mut uint8_t,
        ) == 0 as ::core::ffi::c_int
    {
        if max_coeff == 64 as ::core::ffi::c_int {
            fill_rectangle(
                (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                    .offset(
                        *(&raw const scan8 as *const uint8_t).offset(n as isize) as isize,
                    ) as *mut uint8_t as *mut ::core::ffi::c_void,
                2 as ::core::ffi::c_int,
                2 as ::core::ffi::c_int,
                8 as ::core::ffi::c_int,
                0 as uint32_t,
                1 as ::core::ffi::c_int,
            );
        } else {
            (*sl).non_zero_count_cache[scan8[n as usize] as usize] = 0 as uint8_t;
        }
        return;
    }
    decode_cabac_residual_nondc_internal(
        h,
        sl,
        block,
        cat,
        n,
        scantable,
        qmul,
        max_coeff,
    );
}
#[inline(always)]
unsafe extern "C" fn decode_cabac_luma_residual(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut scan: *const uint8_t,
    mut scan8x8: *const uint8_t,
    mut pixel_shift: ::core::ffi::c_int,
    mut mb_type: ::core::ffi::c_int,
    mut cbp: ::core::ffi::c_int,
    mut p: ::core::ffi::c_int,
) {
    static mut ctx_cat: [[uint8_t; 3]; 4] = [
        [
            0 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            10 as ::core::ffi::c_int as uint8_t,
        ],
        [
            1 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            11 as ::core::ffi::c_int as uint8_t,
        ],
        [
            2 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            12 as ::core::ffi::c_int as uint8_t,
        ],
        [
            5 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            13 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    let mut qmul: *const uint32_t = ::core::ptr::null::<uint32_t>();
    let mut i8x8: ::core::ffi::c_int = 0;
    let mut i4x4: ::core::ffi::c_int = 0;
    let mut qscale: ::core::ffi::c_int = if p == 0 as ::core::ffi::c_int {
        (*sl).qscale
    } else {
        (*sl).chroma_qp[(p - 1 as ::core::ffi::c_int) as usize]
    };
    if mb_type & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
        (*((&raw mut *(&raw mut (*sl).mb_luma_dc as *mut [int16_t; 32])
            .offset(p as isize) as *mut int16_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut av_alias64))
            .u64_0 = 0 as uint64_t;
        (*(((&raw mut *(&raw mut (*sl).mb_luma_dc as *mut [int16_t; 32])
            .offset(p as isize) as *mut int16_t)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char)
            .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
            .u64_0 = 0 as uint64_t;
        (*((&raw mut *(&raw mut (*sl).mb_luma_dc as *mut [int16_t; 32])
            .offset(p as isize) as *mut int16_t)
            .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
            .u64_0 = 0 as uint64_t;
        (*(((&raw mut *(&raw mut (*sl).mb_luma_dc as *mut [int16_t; 32])
            .offset(p as isize) as *mut int16_t)
            .offset(8 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char)
            .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
            .u64_0 = 0 as uint64_t;
        (*((&raw mut *(&raw mut (*sl).mb_luma_dc as *mut [int16_t; 32])
            .offset(p as isize) as *mut int16_t)
            .offset(16 as ::core::ffi::c_int as isize) as *mut av_alias64))
            .u64_0 = 0 as uint64_t;
        (*(((&raw mut *(&raw mut (*sl).mb_luma_dc as *mut [int16_t; 32])
            .offset(p as isize) as *mut int16_t)
            .offset(16 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char)
            .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
            .u64_0 = 0 as uint64_t;
        (*((&raw mut *(&raw mut (*sl).mb_luma_dc as *mut [int16_t; 32])
            .offset(p as isize) as *mut int16_t)
            .offset(24 as ::core::ffi::c_int as isize) as *mut av_alias64))
            .u64_0 = 0 as uint64_t;
        (*(((&raw mut *(&raw mut (*sl).mb_luma_dc as *mut [int16_t; 32])
            .offset(p as isize) as *mut int16_t)
            .offset(24 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_char)
            .offset(8 as ::core::ffi::c_int as isize) as *mut av_alias64))
            .u64_0 = 0 as uint64_t;
        decode_cabac_residual_dc(
            h,
            sl,
            &raw mut *(&raw mut (*sl).mb_luma_dc as *mut [int16_t; 32])
                .offset(p as isize) as *mut int16_t,
            ctx_cat[0 as ::core::ffi::c_int as usize][p as usize] as ::core::ffi::c_int,
            48 as ::core::ffi::c_int + p,
            scan,
            16 as ::core::ffi::c_int,
        );
        if cbp & 15 as ::core::ffi::c_int != 0 {
            qmul = &raw mut *(*(&raw const (*(*h).ps.pps).dequant4_coeff
                as *const *mut [uint32_t; 16])
                .offset(p as isize))
                .offset(qscale as isize) as *mut uint32_t;
            i4x4 = 0 as ::core::ffi::c_int;
            while i4x4 < 16 as ::core::ffi::c_int {
                let index: ::core::ffi::c_int = 16 as ::core::ffi::c_int * p + i4x4;
                decode_cabac_residual_nondc(
                    h,
                    sl,
                    (&raw mut (*sl).mb as *mut int16_t)
                        .offset(
                            (16 as ::core::ffi::c_int * index << pixel_shift) as isize,
                        ),
                    ctx_cat[1 as ::core::ffi::c_int as usize][p as usize]
                        as ::core::ffi::c_int,
                    index,
                    scan.offset(1 as ::core::ffi::c_int as isize),
                    qmul,
                    15 as ::core::ffi::c_int,
                );
                i4x4 += 1;
            }
        } else {
            fill_rectangle(
                (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                    .offset(
                        *(&raw const scan8 as *const uint8_t)
                            .offset((16 as ::core::ffi::c_int * p) as isize) as isize,
                    ) as *mut uint8_t as *mut ::core::ffi::c_void,
                4 as ::core::ffi::c_int,
                4 as ::core::ffi::c_int,
                8 as ::core::ffi::c_int,
                0 as uint32_t,
                1 as ::core::ffi::c_int,
            );
        }
    } else {
        let mut cqm: ::core::ffi::c_int = (if mb_type & 7 as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            3 as ::core::ffi::c_int
        }) + p;
        i8x8 = 0 as ::core::ffi::c_int;
        while i8x8 < 4 as ::core::ffi::c_int {
            if cbp & (1 as ::core::ffi::c_int) << i8x8 != 0 {
                if mb_type & 0x1000000 as ::core::ffi::c_int != 0 {
                    let index_0: ::core::ffi::c_int = 16 as ::core::ffi::c_int * p
                        + 4 as ::core::ffi::c_int * i8x8;
                    decode_cabac_residual_nondc(
                        h,
                        sl,
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                (16 as ::core::ffi::c_int * index_0 << pixel_shift) as isize,
                            ),
                        ctx_cat[3 as ::core::ffi::c_int as usize][p as usize]
                            as ::core::ffi::c_int,
                        index_0,
                        scan8x8,
                        &raw mut *(*(&raw const (*(*h).ps.pps).dequant8_coeff
                            as *const *mut [uint32_t; 64])
                            .offset(cqm as isize))
                            .offset(qscale as isize) as *mut uint32_t,
                        64 as ::core::ffi::c_int,
                    );
                } else {
                    qmul = &raw mut *(*(&raw const (*(*h).ps.pps).dequant4_coeff
                        as *const *mut [uint32_t; 16])
                        .offset(cqm as isize))
                        .offset(qscale as isize) as *mut uint32_t;
                    i4x4 = 0 as ::core::ffi::c_int;
                    while i4x4 < 4 as ::core::ffi::c_int {
                        let index_1: ::core::ffi::c_int = 16 as ::core::ffi::c_int * p
                            + 4 as ::core::ffi::c_int * i8x8 + i4x4;
                        decode_cabac_residual_nondc(
                            h,
                            sl,
                            (&raw mut (*sl).mb as *mut int16_t)
                                .offset(
                                    (16 as ::core::ffi::c_int * index_1 << pixel_shift) as isize,
                                ),
                            ctx_cat[2 as ::core::ffi::c_int as usize][p as usize]
                                as ::core::ffi::c_int,
                            index_1,
                            scan,
                            qmul,
                            16 as ::core::ffi::c_int,
                        );
                        i4x4 += 1;
                    }
                }
            } else {
                fill_rectangle(
                    (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                        .offset(
                            *(&raw const scan8 as *const uint8_t)
                                .offset(
                                    (4 as ::core::ffi::c_int * i8x8
                                        + 16 as ::core::ffi::c_int * p) as isize,
                                ) as isize,
                        ) as *mut uint8_t as *mut ::core::ffi::c_void,
                    2 as ::core::ffi::c_int,
                    2 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    0 as uint32_t,
                    1 as ::core::ffi::c_int,
                );
            }
            i8x8 += 1;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_decode_mb_cabac(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut sps: *const SPS = (*h).ps.sps;
    let mut mb_xy: ::core::ffi::c_int = 0;
    let mut mb_type: ::core::ffi::c_int = 0;
    let mut partition_count: ::core::ffi::c_int = 0;
    let mut cbp: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dct8x8_allowed: ::core::ffi::c_int = (*(*h).ps.pps).transform_8x8_mode;
    let decode_chroma: ::core::ffi::c_int = ((*sps).chroma_format_idc
        == 1 as ::core::ffi::c_int
        || (*sps).chroma_format_idc == 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
    let pixel_shift: ::core::ffi::c_int = (*h).pixel_shift;
    (*sl).mb_xy = (*sl).mb_x + (*sl).mb_y * (*h).mb_stride;
    mb_xy = (*sl).mb_xy;
    if (*sl).slice_type_nos != AV_PICTURE_TYPE_I as ::core::ffi::c_int {
        let mut skip: ::core::ffi::c_int = 0;
        if (*h).mb_aff_frame != 0
            && (*sl).mb_y & 1 as ::core::ffi::c_int == 1 as ::core::ffi::c_int
            && (*sl).prev_mb_skipped != 0
        {
            skip = (*sl).next_mb_skipped;
        } else {
            skip = decode_cabac_mb_skip(h, sl, (*sl).mb_x, (*sl).mb_y);
        }
        if skip != 0 {
            if (*h).mb_aff_frame != 0
                && (*sl).mb_y & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            {
                *(*h).cur_pic.mb_type.offset(mb_xy as isize) = ((1 as ::core::ffi::c_int)
                    << 17 as ::core::ffi::c_int) as uint32_t;
                (*sl).next_mb_skipped = decode_cabac_mb_skip(
                    h,
                    sl,
                    (*sl).mb_x,
                    (*sl).mb_y + 1 as ::core::ffi::c_int,
                );
                if (*sl).next_mb_skipped == 0 {
                    (*sl).mb_field_decoding_flag = decode_cabac_field_decoding_flag(
                        h,
                        sl,
                    );
                    (*sl).mb_mbaff = (*sl).mb_field_decoding_flag;
                }
            }
            decode_mb_skip(h, sl);
            *(*h).cbp_table.offset(mb_xy as isize) = 0 as uint16_t;
            *(*h).chroma_pred_mode_table.offset(mb_xy as isize) = 0 as uint8_t;
            (*sl).last_qscale_diff = 0 as ::core::ffi::c_int;
            return 0 as ::core::ffi::c_int;
        }
    }
    if (*h).mb_aff_frame != 0 {
        if (*sl).mb_y & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*sl).mb_field_decoding_flag = decode_cabac_field_decoding_flag(h, sl);
            (*sl).mb_mbaff = (*sl).mb_field_decoding_flag;
        }
    }
    (*sl).prev_mb_skipped = 0 as ::core::ffi::c_int;
    fill_decode_neighbors(h, sl, -(*sl).mb_field_decoding_flag);
    if (*sl).slice_type_nos == AV_PICTURE_TYPE_B as ::core::ffi::c_int {
        let mut ctx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*sl).left_type[0 as ::core::ffi::c_int as usize] - 1 as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
        {
            ctx += 1;
        }
        if (*sl).top_type - 1 as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
        {
            ctx += 1;
        }
        if get_cabac_noinline(
            &raw mut (*sl).cabac,
            (&raw mut (*sl).cabac_state as *mut uint8_t)
                .offset((27 as ::core::ffi::c_int + ctx) as isize) as *mut uint8_t,
        ) == 0
        {
            mb_type = 0 as ::core::ffi::c_int;
            current_block = 12497913735442871383;
        } else if get_cabac_noinline(
            &raw mut (*sl).cabac,
            (&raw mut (*sl).cabac_state as *mut uint8_t)
                .offset((27 as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as isize)
                as *mut uint8_t,
        ) == 0
        {
            mb_type = 1 as ::core::ffi::c_int
                + get_cabac_noinline(
                    &raw mut (*sl).cabac,
                    (&raw mut (*sl).cabac_state as *mut uint8_t)
                        .offset(
                            (27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                        ) as *mut uint8_t,
                );
            current_block = 12497913735442871383;
        } else {
            let mut bits: ::core::ffi::c_int = 0;
            bits = get_cabac_noinline(
                &raw mut (*sl).cabac,
                (&raw mut (*sl).cabac_state as *mut uint8_t)
                    .offset(
                        (27 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t,
            ) << 3 as ::core::ffi::c_int;
            bits
                += get_cabac_noinline(
                    &raw mut (*sl).cabac,
                    (&raw mut (*sl).cabac_state as *mut uint8_t)
                        .offset(
                            (27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                        ) as *mut uint8_t,
                ) << 2 as ::core::ffi::c_int;
            bits
                += get_cabac_noinline(
                    &raw mut (*sl).cabac,
                    (&raw mut (*sl).cabac_state as *mut uint8_t)
                        .offset(
                            (27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                        ) as *mut uint8_t,
                ) << 1 as ::core::ffi::c_int;
            bits
                += get_cabac_noinline(
                    &raw mut (*sl).cabac,
                    (&raw mut (*sl).cabac_state as *mut uint8_t)
                        .offset(
                            (27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int) as isize,
                        ) as *mut uint8_t,
                );
            if bits < 8 as ::core::ffi::c_int {
                mb_type = bits + 3 as ::core::ffi::c_int;
                current_block = 12497913735442871383;
            } else if bits == 13 as ::core::ffi::c_int {
                mb_type = decode_cabac_intra_mb_type(
                    sl,
                    32 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                current_block = 17325532449512129659;
            } else {
                if bits == 14 as ::core::ffi::c_int {
                    mb_type = 11 as ::core::ffi::c_int;
                } else if bits == 15 as ::core::ffi::c_int {
                    mb_type = 22 as ::core::ffi::c_int;
                } else {
                    bits = (bits << 1 as ::core::ffi::c_int)
                        + get_cabac_noinline(
                            &raw mut (*sl).cabac,
                            (&raw mut (*sl).cabac_state as *mut uint8_t)
                                .offset(
                                    (27 as ::core::ffi::c_int + 5 as ::core::ffi::c_int)
                                        as isize,
                                ) as *mut uint8_t,
                        );
                    mb_type = bits - 4 as ::core::ffi::c_int;
                }
                current_block = 12497913735442871383;
            }
        }
        match current_block {
            17325532449512129659 => {}
            _ => {
                partition_count = ff_h264_b_mb_type_info[mb_type as usize]
                    .partition_count as ::core::ffi::c_int;
                mb_type = ff_h264_b_mb_type_info[mb_type as usize].type_0
                    as ::core::ffi::c_int;
                current_block = 7494008139977416618;
            }
        }
    } else if (*sl).slice_type_nos == AV_PICTURE_TYPE_P as ::core::ffi::c_int {
        if get_cabac_noinline(
            &raw mut (*sl).cabac,
            (&raw mut (*sl).cabac_state as *mut uint8_t)
                .offset(14 as ::core::ffi::c_int as isize) as *mut uint8_t,
        ) == 0 as ::core::ffi::c_int
        {
            if get_cabac_noinline(
                &raw mut (*sl).cabac,
                (&raw mut (*sl).cabac_state as *mut uint8_t)
                    .offset(15 as ::core::ffi::c_int as isize) as *mut uint8_t,
            ) == 0 as ::core::ffi::c_int
            {
                mb_type = 3 as ::core::ffi::c_int
                    * get_cabac_noinline(
                        &raw mut (*sl).cabac,
                        (&raw mut (*sl).cabac_state as *mut uint8_t)
                            .offset(16 as ::core::ffi::c_int as isize) as *mut uint8_t,
                    );
            } else {
                mb_type = 2 as ::core::ffi::c_int
                    - get_cabac_noinline(
                        &raw mut (*sl).cabac,
                        (&raw mut (*sl).cabac_state as *mut uint8_t)
                            .offset(17 as ::core::ffi::c_int as isize) as *mut uint8_t,
                    );
            }
            partition_count = ff_h264_p_mb_type_info[mb_type as usize].partition_count
                as ::core::ffi::c_int;
            mb_type = ff_h264_p_mb_type_info[mb_type as usize].type_0
                as ::core::ffi::c_int;
            current_block = 7494008139977416618;
        } else {
            mb_type = decode_cabac_intra_mb_type(
                sl,
                17 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
            current_block = 17325532449512129659;
        }
    } else {
        mb_type = decode_cabac_intra_mb_type(
            sl,
            3 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        );
        if (*sl).slice_type == AV_PICTURE_TYPE_SI as ::core::ffi::c_int && mb_type != 0 {
            mb_type -= 1;
        }
        current_block = 17325532449512129659;
    }
    match current_block {
        17325532449512129659 => {
            partition_count = 0 as ::core::ffi::c_int;
            cbp = ff_h264_i_mb_type_info[mb_type as usize].cbp as ::core::ffi::c_int;
            (*sl).intra16x16_pred_mode = ff_h264_i_mb_type_info[mb_type as usize]
                .pred_mode as ::core::ffi::c_int;
            mb_type = ff_h264_i_mb_type_info[mb_type as usize].type_0
                as ::core::ffi::c_int;
        }
        _ => {}
    }
    if (*sl).mb_field_decoding_flag != 0 {
        mb_type |= (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
    }
    *(*h).slice_table.offset(mb_xy as isize) = (*sl).slice_num as uint16_t;
    if mb_type & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0 {
        let mb_size: ::core::ffi::c_int = ff_h264_mb_sizes[(*sps).chroma_format_idc
            as usize] as ::core::ffi::c_int * (*sps).bit_depth_luma
            >> 3 as ::core::ffi::c_int;
        let mut ptr: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut ret: ::core::ffi::c_int = 0;
        ptr = (*sl).cabac.bytestream;
        if (*sl).cabac.low & 0x1 as ::core::ffi::c_int != 0 {
            ptr = ptr.offset(-1);
        }
        if 16 as ::core::ffi::c_int == 16 as ::core::ffi::c_int {
            if (*sl).cabac.low & 0x1ff as ::core::ffi::c_int != 0 {
                ptr = ptr.offset(-1);
            }
        }
        if ((*sl).cabac.bytestream_end.offset_from(ptr) as ::core::ffi::c_long
            as ::core::ffi::c_int) < mb_size
        {
            return -(1 as ::core::ffi::c_int);
        }
        (*sl).intra_pcm_ptr = ptr;
        ptr = ptr.offset(mb_size as isize);
        ret = ff_init_cabac_decoder(
            &raw mut (*sl).cabac,
            ptr,
            (*sl).cabac.bytestream_end.offset_from(ptr) as ::core::ffi::c_long
                as ::core::ffi::c_int,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        *(*h).cbp_table.offset(mb_xy as isize) = 0xf7ef as uint16_t;
        *(*h).chroma_pred_mode_table.offset(mb_xy as isize) = 0 as uint8_t;
        *(*h).cur_pic.qscale_table.offset(mb_xy as isize) = 0 as int8_t;
        memset(
            &raw mut *(*h).non_zero_count.offset(mb_xy as isize) as *mut uint8_t
                as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            48 as size_t,
        );
        *(*h).cur_pic.mb_type.offset(mb_xy as isize) = mb_type as uint32_t;
        (*sl).last_qscale_diff = 0 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    fill_decode_caches(h, sl, mb_type);
    if mb_type & 7 as ::core::ffi::c_int != 0 {
        let mut i: ::core::ffi::c_int = 0;
        let mut pred_mode: ::core::ffi::c_int = 0;
        if mb_type & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
            if dct8x8_allowed != 0
                && get_cabac_noinline(
                    &raw mut (*sl).cabac,
                    (&raw mut (*sl).cabac_state as *mut uint8_t)
                        .offset(
                            (399 as ::core::ffi::c_int + (*sl).neighbor_transform_size)
                                as isize,
                        ) as *mut uint8_t,
                ) != 0
            {
                mb_type |= 0x1000000 as ::core::ffi::c_int;
                i = 0 as ::core::ffi::c_int;
                while i < 16 as ::core::ffi::c_int {
                    let mut pred: ::core::ffi::c_int = pred_intra_mode(h, sl, i);
                    let mut mode: ::core::ffi::c_int = decode_cabac_mb_intra4x4_pred_mode(
                        sl,
                        pred,
                    );
                    fill_rectangle(
                        (&raw mut (*sl).intra4x4_pred_mode_cache as *mut int8_t)
                            .offset(
                                *(&raw const scan8 as *const uint8_t).offset(i as isize)
                                    as isize,
                            ) as *mut int8_t as *mut ::core::ffi::c_void,
                        2 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        8 as ::core::ffi::c_int,
                        mode as uint32_t,
                        1 as ::core::ffi::c_int,
                    );
                    i += 4 as ::core::ffi::c_int;
                }
            } else {
                i = 0 as ::core::ffi::c_int;
                while i < 16 as ::core::ffi::c_int {
                    let mut pred_0: ::core::ffi::c_int = pred_intra_mode(h, sl, i);
                    (*sl).intra4x4_pred_mode_cache[scan8[i as usize] as usize] = decode_cabac_mb_intra4x4_pred_mode(
                        sl,
                        pred_0,
                    ) as int8_t;
                    i += 1;
                }
            }
            write_back_intra_pred_mode(h, sl);
            if ff_h264_check_intra4x4_pred_mode(
                &raw mut (*sl).intra4x4_pred_mode_cache as *mut int8_t,
                (*h).avctx as *mut ::core::ffi::c_void,
                (*sl).top_samples_available as ::core::ffi::c_int,
                (*sl).left_samples_available as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                return -(1 as ::core::ffi::c_int);
            }
        } else {
            (*sl).intra16x16_pred_mode = ff_h264_check_intra_pred_mode(
                (*h).avctx as *mut ::core::ffi::c_void,
                (*sl).top_samples_available as ::core::ffi::c_int,
                (*sl).left_samples_available as ::core::ffi::c_int,
                (*sl).intra16x16_pred_mode,
                0 as ::core::ffi::c_int,
            );
            if (*sl).intra16x16_pred_mode < 0 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
        }
        if decode_chroma != 0 {
            pred_mode = decode_cabac_mb_chroma_pre_mode(h, sl);
            *(*h).chroma_pred_mode_table.offset(mb_xy as isize) = pred_mode as uint8_t;
            pred_mode = ff_h264_check_intra_pred_mode(
                (*h).avctx as *mut ::core::ffi::c_void,
                (*sl).top_samples_available as ::core::ffi::c_int,
                (*sl).left_samples_available as ::core::ffi::c_int,
                pred_mode,
                1 as ::core::ffi::c_int,
            );
            if pred_mode < 0 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int);
            }
            (*sl).chroma_pred_mode = pred_mode;
        } else {
            (*sl).chroma_pred_mode = 6 as ::core::ffi::c_int;
        }
    } else if partition_count == 4 as ::core::ffi::c_int {
        let mut i_0: ::core::ffi::c_int = 0;
        let mut j: ::core::ffi::c_int = 0;
        let mut sub_partition_count: [::core::ffi::c_int; 4] = [0; 4];
        let mut list: ::core::ffi::c_int = 0;
        let mut ref_0: [[::core::ffi::c_int; 4]; 2] = [[0; 4]; 2];
        if (*sl).slice_type_nos == AV_PICTURE_TYPE_B as ::core::ffi::c_int {
            i_0 = 0 as ::core::ffi::c_int;
            while i_0 < 4 as ::core::ffi::c_int {
                (*sl).sub_mb_type[i_0 as usize] = decode_cabac_b_mb_sub_type(sl)
                    as uint16_t;
                sub_partition_count[i_0 as usize] = ff_h264_b_sub_mb_type_info[(*sl)
                        .sub_mb_type[i_0 as usize] as usize]
                    .partition_count as ::core::ffi::c_int;
                (*sl).sub_mb_type[i_0 as usize] = ff_h264_b_sub_mb_type_info[(*sl)
                        .sub_mb_type[i_0 as usize] as usize]
                    .type_0;
                i_0 += 1;
            }
            if ((*sl).sub_mb_type[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                | (*sl).sub_mb_type[1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                | (*sl).sub_mb_type[2 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int
                | (*sl).sub_mb_type[3 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int)
                & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0
            {
                ff_h264_pred_direct_motion(h, sl, &raw mut mb_type);
                (*sl)
                    .ref_cache[1 as ::core::ffi::c_int
                    as usize][scan8[12 as ::core::ffi::c_int as usize] as usize] = -(2
                    as ::core::ffi::c_int) as int8_t;
                (*sl)
                    .ref_cache[0 as ::core::ffi::c_int
                    as usize][scan8[12 as ::core::ffi::c_int as usize] as usize] = (*sl)
                    .ref_cache[1 as ::core::ffi::c_int
                    as usize][scan8[12 as ::core::ffi::c_int as usize] as usize];
                (*sl)
                    .ref_cache[1 as ::core::ffi::c_int
                    as usize][scan8[4 as ::core::ffi::c_int as usize] as usize] = (*sl)
                    .ref_cache[0 as ::core::ffi::c_int
                    as usize][scan8[12 as ::core::ffi::c_int as usize] as usize];
                (*sl)
                    .ref_cache[0 as ::core::ffi::c_int
                    as usize][scan8[4 as ::core::ffi::c_int as usize] as usize] = (*sl)
                    .ref_cache[1 as ::core::ffi::c_int
                    as usize][scan8[4 as ::core::ffi::c_int as usize] as usize];
                i_0 = 0 as ::core::ffi::c_int;
                while i_0 < 4 as ::core::ffi::c_int {
                    fill_rectangle(
                        (&raw mut (*sl).direct_cache as *mut uint8_t)
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset((4 as ::core::ffi::c_int * i_0) as isize) as isize,
                            ) as *mut uint8_t as *mut ::core::ffi::c_void,
                        2 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        8 as ::core::ffi::c_int,
                        ((*sl).sub_mb_type[i_0 as usize] as ::core::ffi::c_int
                            >> 1 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
                            as uint32_t,
                        1 as ::core::ffi::c_int,
                    );
                    i_0 += 1;
                }
            }
        } else {
            i_0 = 0 as ::core::ffi::c_int;
            while i_0 < 4 as ::core::ffi::c_int {
                (*sl).sub_mb_type[i_0 as usize] = decode_cabac_p_mb_sub_type(sl)
                    as uint16_t;
                sub_partition_count[i_0 as usize] = ff_h264_p_sub_mb_type_info[(*sl)
                        .sub_mb_type[i_0 as usize] as usize]
                    .partition_count as ::core::ffi::c_int;
                (*sl).sub_mb_type[i_0 as usize] = ff_h264_p_sub_mb_type_info[(*sl)
                        .sub_mb_type[i_0 as usize] as usize]
                    .type_0;
                i_0 += 1;
            }
        }
        list = 0 as ::core::ffi::c_int;
        while (list as ::core::ffi::c_uint) < (*sl).list_count {
            i_0 = 0 as ::core::ffi::c_int;
            while i_0 < 4 as ::core::ffi::c_int {
                if !((*sl).sub_mb_type[i_0 as usize] as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0)
                {
                    if (*sl).sub_mb_type[i_0 as usize] as ::core::ffi::c_int
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << 0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * list
                        != 0
                    {
                        let mut rc: ::core::ffi::c_uint = (*sl).ref_count[list as usize]
                            << (*sl).mb_mbaff;
                        if rc > 1 as ::core::ffi::c_uint {
                            ref_0[list as usize][i_0 as usize] = decode_cabac_mb_ref(
                                sl,
                                list,
                                4 as ::core::ffi::c_int * i_0,
                            );
                            if ref_0[list as usize][i_0 as usize] as ::core::ffi::c_uint
                                >= rc
                            {
                                av_log(
                                    (*h).avctx as *mut ::core::ffi::c_void,
                                    16 as ::core::ffi::c_int,
                                    b"Reference %d >= %d\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    ref_0[list as usize][i_0 as usize],
                                    rc,
                                );
                                return -(1 as ::core::ffi::c_int);
                            }
                        } else {
                            ref_0[list as usize][i_0 as usize] = 0 as ::core::ffi::c_int;
                        }
                    } else {
                        ref_0[list as usize][i_0 as usize] = -(1 as ::core::ffi::c_int);
                    }
                    (*sl)
                        .ref_cache[list
                        as usize][(scan8[(4 as ::core::ffi::c_int * i_0) as usize]
                        as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as usize] = ref_0[list
                        as usize][i_0 as usize] as int8_t;
                    (*sl)
                        .ref_cache[list
                        as usize][(scan8[(4 as ::core::ffi::c_int * i_0) as usize]
                        as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize] = (*sl)
                        .ref_cache[list
                        as usize][(scan8[(4 as ::core::ffi::c_int * i_0) as usize]
                        as ::core::ffi::c_int + 9 as ::core::ffi::c_int) as usize];
                    (*sl)
                        .ref_cache[list
                        as usize][(scan8[(4 as ::core::ffi::c_int * i_0) as usize]
                        as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize] = (*sl)
                        .ref_cache[list
                        as usize][(scan8[(4 as ::core::ffi::c_int * i_0) as usize]
                        as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as usize];
                }
                i_0 += 1;
            }
            list += 1;
        }
        if dct8x8_allowed != 0 {
            dct8x8_allowed = get_dct8x8_allowed(h, sl);
        }
        list = 0 as ::core::ffi::c_int;
        while (list as ::core::ffi::c_uint) < (*sl).list_count {
            i_0 = 0 as ::core::ffi::c_int;
            while i_0 < 4 as ::core::ffi::c_int {
                (*sl)
                    .ref_cache[list
                    as usize][scan8[(4 as ::core::ffi::c_int * i_0) as usize]
                    as usize] = (*sl)
                    .ref_cache[list
                    as usize][(scan8[(4 as ::core::ffi::c_int * i_0) as usize]
                    as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize];
                if (*sl).sub_mb_type[i_0 as usize] as ::core::ffi::c_int
                    & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0
                {
                    fill_rectangle(
                        &raw mut *(&raw mut *(&raw mut (*sl).mvd_cache
                            as *mut [[uint8_t; 2]; 40])
                            .offset(list as isize) as *mut [uint8_t; 2])
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset((4 as ::core::ffi::c_int * i_0) as isize) as isize,
                            ) as *mut uint8_t as *mut ::core::ffi::c_void,
                        2 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        8 as ::core::ffi::c_int,
                        0 as uint32_t,
                        2 as ::core::ffi::c_int,
                    );
                } else if (*sl).sub_mb_type[i_0 as usize] as ::core::ffi::c_int
                    & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                        << 0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * list != 0
                    && (*sl).sub_mb_type[i_0 as usize] as ::core::ffi::c_int
                        & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int == 0
                {
                    let sub_mb_type: ::core::ffi::c_int = (*sl).sub_mb_type[i_0 as usize]
                        as ::core::ffi::c_int;
                    let block_width: ::core::ffi::c_int = if sub_mb_type
                        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) != 0
                    {
                        2 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    };
                    j = 0 as ::core::ffi::c_int;
                    while j < sub_partition_count[i_0 as usize] {
                        let mut mpx: ::core::ffi::c_int = 0;
                        let mut mpy: ::core::ffi::c_int = 0;
                        let mut mx: ::core::ffi::c_int = 0;
                        let mut my: ::core::ffi::c_int = 0;
                        let index: ::core::ffi::c_int = 4 as ::core::ffi::c_int * i_0
                            + block_width * j;
                        let mut mv_cache: *mut [int16_t; 2] = (&raw mut *(&raw mut (*sl)
                            .mv_cache as *mut [[int16_t; 2]; 40])
                            .offset(list as isize) as *mut [int16_t; 2])
                            .offset(
                                *(&raw const scan8 as *const uint8_t).offset(index as isize)
                                    as isize,
                            ) as *mut [int16_t; 2];
                        let mut mvd_cache: *mut [uint8_t; 2] = (&raw mut *(&raw mut (*sl)
                            .mvd_cache as *mut [[uint8_t; 2]; 40])
                            .offset(list as isize) as *mut [uint8_t; 2])
                            .offset(
                                *(&raw const scan8 as *const uint8_t).offset(index as isize)
                                    as isize,
                            ) as *mut [uint8_t; 2];
                        pred_motion(
                            h,
                            sl,
                            index,
                            block_width,
                            list,
                            (*sl)
                                .ref_cache[list as usize][scan8[index as usize] as usize]
                                as ::core::ffi::c_int,
                            &raw mut mx,
                            &raw mut my,
                        );
                        let mut amvd0: ::core::ffi::c_int = (*sl)
                            .mvd_cache[list
                            as usize][(scan8[index as usize] as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*sl)
                                .mvd_cache[list
                                as usize][(scan8[index as usize] as ::core::ffi::c_int
                                - 8 as ::core::ffi::c_int)
                                as usize][0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int;
                        let mut amvd1: ::core::ffi::c_int = (*sl)
                            .mvd_cache[list
                            as usize][(scan8[index as usize] as ::core::ffi::c_int
                            - 1 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*sl)
                                .mvd_cache[list
                                as usize][(scan8[index as usize] as ::core::ffi::c_int
                                - 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int;
                        let mut mxd: ::core::ffi::c_int = decode_cabac_mb_mvd(
                            sl,
                            40 as ::core::ffi::c_int,
                            amvd0,
                            &raw mut mpx,
                        );
                        let mut myd: ::core::ffi::c_int = decode_cabac_mb_mvd(
                            sl,
                            47 as ::core::ffi::c_int,
                            amvd1,
                            &raw mut mpy,
                        );
                        if mxd
                            == -(2147483647 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int
                            || myd
                                == -(2147483647 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int
                        {
                            return -((('I' as i32
                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint
                                | ('A' as i32 as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                        }
                        mx += mxd;
                        my += myd;
                        if sub_mb_type
                            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                        {
                            let ref mut fresh0 = (*mv_cache
                                .offset(
                                    9 as ::core::ffi::c_int as isize,
                                ))[0 as ::core::ffi::c_int as usize];
                            *fresh0 = mx as int16_t;
                            let ref mut fresh1 = (*mv_cache
                                .offset(
                                    8 as ::core::ffi::c_int as isize,
                                ))[0 as ::core::ffi::c_int as usize];
                            *fresh1 = *fresh0;
                            (*mv_cache
                                .offset(
                                    1 as ::core::ffi::c_int as isize,
                                ))[0 as ::core::ffi::c_int as usize] = *fresh1;
                            let ref mut fresh2 = (*mv_cache
                                .offset(
                                    9 as ::core::ffi::c_int as isize,
                                ))[1 as ::core::ffi::c_int as usize];
                            *fresh2 = my as int16_t;
                            let ref mut fresh3 = (*mv_cache
                                .offset(
                                    8 as ::core::ffi::c_int as isize,
                                ))[1 as ::core::ffi::c_int as usize];
                            *fresh3 = *fresh2;
                            (*mv_cache
                                .offset(
                                    1 as ::core::ffi::c_int as isize,
                                ))[1 as ::core::ffi::c_int as usize] = *fresh3;
                            let ref mut fresh4 = (*mvd_cache
                                .offset(
                                    9 as ::core::ffi::c_int as isize,
                                ))[0 as ::core::ffi::c_int as usize];
                            *fresh4 = mpx as uint8_t;
                            let ref mut fresh5 = (*mvd_cache
                                .offset(
                                    8 as ::core::ffi::c_int as isize,
                                ))[0 as ::core::ffi::c_int as usize];
                            *fresh5 = *fresh4;
                            (*mvd_cache
                                .offset(
                                    1 as ::core::ffi::c_int as isize,
                                ))[0 as ::core::ffi::c_int as usize] = *fresh5;
                            let ref mut fresh6 = (*mvd_cache
                                .offset(
                                    9 as ::core::ffi::c_int as isize,
                                ))[1 as ::core::ffi::c_int as usize];
                            *fresh6 = mpy as uint8_t;
                            let ref mut fresh7 = (*mvd_cache
                                .offset(
                                    8 as ::core::ffi::c_int as isize,
                                ))[1 as ::core::ffi::c_int as usize];
                            *fresh7 = *fresh6;
                            (*mvd_cache
                                .offset(
                                    1 as ::core::ffi::c_int as isize,
                                ))[1 as ::core::ffi::c_int as usize] = *fresh7;
                        } else if sub_mb_type
                            & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0
                        {
                            (*mv_cache
                                .offset(
                                    1 as ::core::ffi::c_int as isize,
                                ))[0 as ::core::ffi::c_int as usize] = mx as int16_t;
                            (*mv_cache
                                .offset(
                                    1 as ::core::ffi::c_int as isize,
                                ))[1 as ::core::ffi::c_int as usize] = my as int16_t;
                            (*mvd_cache
                                .offset(
                                    1 as ::core::ffi::c_int as isize,
                                ))[0 as ::core::ffi::c_int as usize] = mpx as uint8_t;
                            (*mvd_cache
                                .offset(
                                    1 as ::core::ffi::c_int as isize,
                                ))[1 as ::core::ffi::c_int as usize] = mpy as uint8_t;
                        } else if sub_mb_type
                            & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0
                        {
                            (*mv_cache
                                .offset(
                                    8 as ::core::ffi::c_int as isize,
                                ))[0 as ::core::ffi::c_int as usize] = mx as int16_t;
                            (*mv_cache
                                .offset(
                                    8 as ::core::ffi::c_int as isize,
                                ))[1 as ::core::ffi::c_int as usize] = my as int16_t;
                            (*mvd_cache
                                .offset(
                                    8 as ::core::ffi::c_int as isize,
                                ))[0 as ::core::ffi::c_int as usize] = mpx as uint8_t;
                            (*mvd_cache
                                .offset(
                                    8 as ::core::ffi::c_int as isize,
                                ))[1 as ::core::ffi::c_int as usize] = mpy as uint8_t;
                        }
                        (*mv_cache
                            .offset(
                                0 as ::core::ffi::c_int as isize,
                            ))[0 as ::core::ffi::c_int as usize] = mx as int16_t;
                        (*mv_cache
                            .offset(
                                0 as ::core::ffi::c_int as isize,
                            ))[1 as ::core::ffi::c_int as usize] = my as int16_t;
                        (*mvd_cache
                            .offset(
                                0 as ::core::ffi::c_int as isize,
                            ))[0 as ::core::ffi::c_int as usize] = mpx as uint8_t;
                        (*mvd_cache
                            .offset(
                                0 as ::core::ffi::c_int as isize,
                            ))[1 as ::core::ffi::c_int as usize] = mpy as uint8_t;
                        j += 1;
                    }
                } else {
                    fill_rectangle(
                        &raw mut *(&raw mut *(&raw mut (*sl).mv_cache
                            as *mut [[int16_t; 2]; 40])
                            .offset(list as isize) as *mut [int16_t; 2])
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset((4 as ::core::ffi::c_int * i_0) as isize) as isize,
                            ) as *mut int16_t as *mut ::core::ffi::c_void,
                        2 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        8 as ::core::ffi::c_int,
                        0 as uint32_t,
                        4 as ::core::ffi::c_int,
                    );
                    fill_rectangle(
                        &raw mut *(&raw mut *(&raw mut (*sl).mvd_cache
                            as *mut [[uint8_t; 2]; 40])
                            .offset(list as isize) as *mut [uint8_t; 2])
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset((4 as ::core::ffi::c_int * i_0) as isize) as isize,
                            ) as *mut uint8_t as *mut ::core::ffi::c_void,
                        2 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        8 as ::core::ffi::c_int,
                        0 as uint32_t,
                        2 as ::core::ffi::c_int,
                    );
                }
                i_0 += 1;
            }
            list += 1;
        }
    } else if mb_type & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0 {
        ff_h264_pred_direct_motion(h, sl, &raw mut mb_type);
        fill_rectangle(
            &raw mut *(&raw mut *(&raw mut (*sl).mvd_cache as *mut [[uint8_t; 2]; 40])
                .offset(0 as ::core::ffi::c_int as isize) as *mut [uint8_t; 2])
                .offset(
                    *(&raw const scan8 as *const uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize) as isize,
                ) as *mut uint8_t as *mut ::core::ffi::c_void,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            0 as uint32_t,
            2 as ::core::ffi::c_int,
        );
        fill_rectangle(
            &raw mut *(&raw mut *(&raw mut (*sl).mvd_cache as *mut [[uint8_t; 2]; 40])
                .offset(1 as ::core::ffi::c_int as isize) as *mut [uint8_t; 2])
                .offset(
                    *(&raw const scan8 as *const uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize) as isize,
                ) as *mut uint8_t as *mut ::core::ffi::c_void,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            0 as uint32_t,
            2 as ::core::ffi::c_int,
        );
        dct8x8_allowed &= (*sps).direct_8x8_inference_flag;
    } else {
        let mut list_0: ::core::ffi::c_int = 0;
        let mut i_1: ::core::ffi::c_int = 0;
        if mb_type & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
            list_0 = 0 as ::core::ffi::c_int;
            while (list_0 as ::core::ffi::c_uint) < (*sl).list_count {
                if mb_type
                    & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                        << 0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * list_0
                    != 0
                {
                    let mut ref_1: ::core::ffi::c_int = 0;
                    let mut rc_0: ::core::ffi::c_uint = (*sl).ref_count[list_0 as usize]
                        << (*sl).mb_mbaff;
                    if rc_0 > 1 as ::core::ffi::c_uint {
                        ref_1 = decode_cabac_mb_ref(sl, list_0, 0 as ::core::ffi::c_int);
                        if ref_1 as ::core::ffi::c_uint >= rc_0 {
                            av_log(
                                (*h).avctx as *mut ::core::ffi::c_void,
                                16 as ::core::ffi::c_int,
                                b"Reference %d >= %d\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                ref_1,
                                rc_0,
                            );
                            return -(1 as ::core::ffi::c_int);
                        }
                    } else {
                        ref_1 = 0 as ::core::ffi::c_int;
                    }
                    fill_rectangle(
                        (&raw mut *(&raw mut (*sl).ref_cache as *mut [int8_t; 40])
                            .offset(list_0 as isize) as *mut int8_t)
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize) as isize,
                            ) as *mut int8_t as *mut ::core::ffi::c_void,
                        4 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        8 as ::core::ffi::c_int,
                        ref_1 as uint32_t,
                        1 as ::core::ffi::c_int,
                    );
                }
                list_0 += 1;
            }
            list_0 = 0 as ::core::ffi::c_int;
            while (list_0 as ::core::ffi::c_uint) < (*sl).list_count {
                if mb_type
                    & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                        << 0 as ::core::ffi::c_int + 2 as ::core::ffi::c_int * list_0
                    != 0
                {
                    let mut mx_0: ::core::ffi::c_int = 0;
                    let mut my_0: ::core::ffi::c_int = 0;
                    let mut mpx_0: ::core::ffi::c_int = 0;
                    let mut mpy_0: ::core::ffi::c_int = 0;
                    pred_motion(
                        h,
                        sl,
                        0 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        list_0,
                        (*sl)
                            .ref_cache[list_0
                            as usize][scan8[0 as ::core::ffi::c_int as usize] as usize]
                            as ::core::ffi::c_int,
                        &raw mut mx_0,
                        &raw mut my_0,
                    );
                    let mut amvd0_0: ::core::ffi::c_int = (*sl)
                        .mvd_cache[list_0
                        as usize][(scan8[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as usize][0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        + (*sl)
                            .mvd_cache[list_0
                            as usize][(scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                            as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                    let mut amvd1_0: ::core::ffi::c_int = (*sl)
                        .mvd_cache[list_0
                        as usize][(scan8[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                        as usize][1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                        + (*sl)
                            .mvd_cache[list_0
                            as usize][(scan8[0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int;
                    let mut mxd_0: ::core::ffi::c_int = decode_cabac_mb_mvd(
                        sl,
                        40 as ::core::ffi::c_int,
                        amvd0_0,
                        &raw mut mpx_0,
                    );
                    let mut myd_0: ::core::ffi::c_int = decode_cabac_mb_mvd(
                        sl,
                        47 as ::core::ffi::c_int,
                        amvd1_0,
                        &raw mut mpy_0,
                    );
                    if mxd_0
                        == -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
                        || myd_0
                            == -(2147483647 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int
                    {
                        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                            | ('D' as i32) << 16 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint
                            | ('A' as i32 as ::core::ffi::c_uint)
                                << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                    }
                    mx_0 += mxd_0;
                    my_0 += myd_0;
                    fill_rectangle(
                        &raw mut *(&raw mut *(&raw mut (*sl).mvd_cache
                            as *mut [[uint8_t; 2]; 40])
                            .offset(list_0 as isize) as *mut [uint8_t; 2])
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize) as isize,
                            ) as *mut uint8_t as *mut ::core::ffi::c_void,
                        4 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        8 as ::core::ffi::c_int,
                        pack8to16(
                            mpx_0 as ::core::ffi::c_uint,
                            mpy_0 as ::core::ffi::c_uint,
                        ) as uint32_t,
                        2 as ::core::ffi::c_int,
                    );
                    fill_rectangle(
                        &raw mut *(&raw mut *(&raw mut (*sl).mv_cache
                            as *mut [[int16_t; 2]; 40])
                            .offset(list_0 as isize) as *mut [int16_t; 2])
                            .offset(
                                *(&raw const scan8 as *const uint8_t)
                                    .offset(0 as ::core::ffi::c_int as isize) as isize,
                            ) as *mut int16_t as *mut ::core::ffi::c_void,
                        4 as ::core::ffi::c_int,
                        4 as ::core::ffi::c_int,
                        8 as ::core::ffi::c_int,
                        pack16to32(
                            mx_0 as ::core::ffi::c_uint,
                            my_0 as ::core::ffi::c_uint,
                        ),
                        4 as ::core::ffi::c_int,
                    );
                }
                list_0 += 1;
            }
        } else if mb_type & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
            list_0 = 0 as ::core::ffi::c_int;
            while (list_0 as ::core::ffi::c_uint) < (*sl).list_count {
                i_1 = 0 as ::core::ffi::c_int;
                while i_1 < 2 as ::core::ffi::c_int {
                    if mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << i_1 + 2 as ::core::ffi::c_int * list_0 != 0
                    {
                        let mut ref_2: ::core::ffi::c_int = 0;
                        let mut rc_1: ::core::ffi::c_uint = (*sl)
                            .ref_count[list_0 as usize] << (*sl).mb_mbaff;
                        if rc_1 > 1 as ::core::ffi::c_uint {
                            ref_2 = decode_cabac_mb_ref(
                                sl,
                                list_0,
                                8 as ::core::ffi::c_int * i_1,
                            );
                            if ref_2 as ::core::ffi::c_uint >= rc_1 {
                                av_log(
                                    (*h).avctx as *mut ::core::ffi::c_void,
                                    16 as ::core::ffi::c_int,
                                    b"Reference %d >= %d\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    ref_2,
                                    rc_1,
                                );
                                return -(1 as ::core::ffi::c_int);
                            }
                        } else {
                            ref_2 = 0 as ::core::ffi::c_int;
                        }
                        fill_rectangle(
                            (&raw mut *(&raw mut (*sl).ref_cache as *mut [int8_t; 40])
                                .offset(list_0 as isize) as *mut int8_t)
                                .offset(
                                    (*(&raw const scan8 as *const uint8_t)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int + 16 as ::core::ffi::c_int * i_1)
                                        as isize,
                                ) as *mut int8_t as *mut ::core::ffi::c_void,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            ref_2 as uint32_t,
                            1 as ::core::ffi::c_int,
                        );
                    } else {
                        fill_rectangle(
                            (&raw mut *(&raw mut (*sl).ref_cache as *mut [int8_t; 40])
                                .offset(list_0 as isize) as *mut int8_t)
                                .offset(
                                    (*(&raw const scan8 as *const uint8_t)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int + 16 as ::core::ffi::c_int * i_1)
                                        as isize,
                                ) as *mut int8_t as *mut ::core::ffi::c_void,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            (-(1 as ::core::ffi::c_int) & 0xff as ::core::ffi::c_int)
                                as uint32_t,
                            1 as ::core::ffi::c_int,
                        );
                    }
                    i_1 += 1;
                }
                list_0 += 1;
            }
            list_0 = 0 as ::core::ffi::c_int;
            while (list_0 as ::core::ffi::c_uint) < (*sl).list_count {
                i_1 = 0 as ::core::ffi::c_int;
                while i_1 < 2 as ::core::ffi::c_int {
                    if mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << i_1 + 2 as ::core::ffi::c_int * list_0 != 0
                    {
                        let mut mx_1: ::core::ffi::c_int = 0;
                        let mut my_1: ::core::ffi::c_int = 0;
                        let mut mpx_1: ::core::ffi::c_int = 0;
                        let mut mpy_1: ::core::ffi::c_int = 0;
                        pred_16x8_motion(
                            h,
                            sl,
                            8 as ::core::ffi::c_int * i_1,
                            list_0,
                            (*sl)
                                .ref_cache[list_0
                                as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int + 16 as ::core::ffi::c_int * i_1)
                                as usize] as ::core::ffi::c_int,
                            &raw mut mx_1,
                            &raw mut my_1,
                        );
                        let mut amvd0_1: ::core::ffi::c_int = (*sl)
                            .mvd_cache[list_0
                            as usize][(scan8[(8 as ::core::ffi::c_int * i_1) as usize]
                            as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*sl)
                                .mvd_cache[list_0
                                as usize][(scan8[(8 as ::core::ffi::c_int * i_1) as usize]
                                as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                                as usize][0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int;
                        let mut amvd1_1: ::core::ffi::c_int = (*sl)
                            .mvd_cache[list_0
                            as usize][(scan8[(8 as ::core::ffi::c_int * i_1) as usize]
                            as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*sl)
                                .mvd_cache[list_0
                                as usize][(scan8[(8 as ::core::ffi::c_int * i_1) as usize]
                                as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int;
                        let mut mxd_1: ::core::ffi::c_int = decode_cabac_mb_mvd(
                            sl,
                            40 as ::core::ffi::c_int,
                            amvd0_1,
                            &raw mut mpx_1,
                        );
                        let mut myd_1: ::core::ffi::c_int = decode_cabac_mb_mvd(
                            sl,
                            47 as ::core::ffi::c_int,
                            amvd1_1,
                            &raw mut mpy_1,
                        );
                        if mxd_1
                            == -(2147483647 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int
                            || myd_1
                                == -(2147483647 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int
                        {
                            return -((('I' as i32
                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint
                                | ('A' as i32 as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                        }
                        mx_1 += mxd_1;
                        my_1 += myd_1;
                        fill_rectangle(
                            &raw mut *(&raw mut *(&raw mut (*sl).mvd_cache
                                as *mut [[uint8_t; 2]; 40])
                                .offset(list_0 as isize) as *mut [uint8_t; 2])
                                .offset(
                                    (*(&raw const scan8 as *const uint8_t)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int + 16 as ::core::ffi::c_int * i_1)
                                        as isize,
                                ) as *mut uint8_t as *mut ::core::ffi::c_void,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            pack8to16(
                                mpx_1 as ::core::ffi::c_uint,
                                mpy_1 as ::core::ffi::c_uint,
                            ) as uint32_t,
                            2 as ::core::ffi::c_int,
                        );
                        fill_rectangle(
                            &raw mut *(&raw mut *(&raw mut (*sl).mv_cache
                                as *mut [[int16_t; 2]; 40])
                                .offset(list_0 as isize) as *mut [int16_t; 2])
                                .offset(
                                    (*(&raw const scan8 as *const uint8_t)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int + 16 as ::core::ffi::c_int * i_1)
                                        as isize,
                                ) as *mut int16_t as *mut ::core::ffi::c_void,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            pack16to32(
                                mx_1 as ::core::ffi::c_uint,
                                my_1 as ::core::ffi::c_uint,
                            ),
                            4 as ::core::ffi::c_int,
                        );
                    } else {
                        fill_rectangle(
                            &raw mut *(&raw mut *(&raw mut (*sl).mvd_cache
                                as *mut [[uint8_t; 2]; 40])
                                .offset(list_0 as isize) as *mut [uint8_t; 2])
                                .offset(
                                    (*(&raw const scan8 as *const uint8_t)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int + 16 as ::core::ffi::c_int * i_1)
                                        as isize,
                                ) as *mut uint8_t as *mut ::core::ffi::c_void,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            0 as uint32_t,
                            2 as ::core::ffi::c_int,
                        );
                        fill_rectangle(
                            &raw mut *(&raw mut *(&raw mut (*sl).mv_cache
                                as *mut [[int16_t; 2]; 40])
                                .offset(list_0 as isize) as *mut [int16_t; 2])
                                .offset(
                                    (*(&raw const scan8 as *const uint8_t)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int + 16 as ::core::ffi::c_int * i_1)
                                        as isize,
                                ) as *mut int16_t as *mut ::core::ffi::c_void,
                            4 as ::core::ffi::c_int,
                            2 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            0 as uint32_t,
                            4 as ::core::ffi::c_int,
                        );
                    }
                    i_1 += 1;
                }
                list_0 += 1;
            }
        } else {
            list_0 = 0 as ::core::ffi::c_int;
            while (list_0 as ::core::ffi::c_uint) < (*sl).list_count {
                i_1 = 0 as ::core::ffi::c_int;
                while i_1 < 2 as ::core::ffi::c_int {
                    if mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << i_1 + 2 as ::core::ffi::c_int * list_0 != 0
                    {
                        let mut ref_3: ::core::ffi::c_int = 0;
                        let mut rc_2: ::core::ffi::c_uint = (*sl)
                            .ref_count[list_0 as usize] << (*sl).mb_mbaff;
                        if rc_2 > 1 as ::core::ffi::c_uint {
                            ref_3 = decode_cabac_mb_ref(
                                sl,
                                list_0,
                                4 as ::core::ffi::c_int * i_1,
                            );
                            if ref_3 as ::core::ffi::c_uint >= rc_2 {
                                av_log(
                                    (*h).avctx as *mut ::core::ffi::c_void,
                                    16 as ::core::ffi::c_int,
                                    b"Reference %d >= %d\n\0" as *const u8
                                        as *const ::core::ffi::c_char,
                                    ref_3,
                                    rc_2,
                                );
                                return -(1 as ::core::ffi::c_int);
                            }
                        } else {
                            ref_3 = 0 as ::core::ffi::c_int;
                        }
                        fill_rectangle(
                            (&raw mut *(&raw mut (*sl).ref_cache as *mut [int8_t; 40])
                                .offset(list_0 as isize) as *mut int8_t)
                                .offset(
                                    (*(&raw const scan8 as *const uint8_t)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i_1)
                                        as isize,
                                ) as *mut int8_t as *mut ::core::ffi::c_void,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            ref_3 as uint32_t,
                            1 as ::core::ffi::c_int,
                        );
                    } else {
                        fill_rectangle(
                            (&raw mut *(&raw mut (*sl).ref_cache as *mut [int8_t; 40])
                                .offset(list_0 as isize) as *mut int8_t)
                                .offset(
                                    (*(&raw const scan8 as *const uint8_t)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i_1)
                                        as isize,
                                ) as *mut int8_t as *mut ::core::ffi::c_void,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            (-(1 as ::core::ffi::c_int) & 0xff as ::core::ffi::c_int)
                                as uint32_t,
                            1 as ::core::ffi::c_int,
                        );
                    }
                    i_1 += 1;
                }
                list_0 += 1;
            }
            list_0 = 0 as ::core::ffi::c_int;
            while (list_0 as ::core::ffi::c_uint) < (*sl).list_count {
                i_1 = 0 as ::core::ffi::c_int;
                while i_1 < 2 as ::core::ffi::c_int {
                    if mb_type
                        & ((1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int)
                            << i_1 + 2 as ::core::ffi::c_int * list_0 != 0
                    {
                        let mut mx_2: ::core::ffi::c_int = 0;
                        let mut my_2: ::core::ffi::c_int = 0;
                        let mut mpx_2: ::core::ffi::c_int = 0;
                        let mut mpy_2: ::core::ffi::c_int = 0;
                        pred_8x16_motion(
                            h,
                            sl,
                            i_1 * 4 as ::core::ffi::c_int,
                            list_0,
                            (*sl)
                                .ref_cache[list_0
                                as usize][(scan8[0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i_1)
                                as usize] as ::core::ffi::c_int,
                            &raw mut mx_2,
                            &raw mut my_2,
                        );
                        let mut amvd0_2: ::core::ffi::c_int = (*sl)
                            .mvd_cache[list_0
                            as usize][(scan8[(4 as ::core::ffi::c_int * i_1) as usize]
                            as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*sl)
                                .mvd_cache[list_0
                                as usize][(scan8[(4 as ::core::ffi::c_int * i_1) as usize]
                                as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                                as usize][0 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int;
                        let mut amvd1_2: ::core::ffi::c_int = (*sl)
                            .mvd_cache[list_0
                            as usize][(scan8[(4 as ::core::ffi::c_int * i_1) as usize]
                            as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                            as usize][1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int
                            + (*sl)
                                .mvd_cache[list_0
                                as usize][(scan8[(4 as ::core::ffi::c_int * i_1) as usize]
                                as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
                                as usize][1 as ::core::ffi::c_int as usize]
                                as ::core::ffi::c_int;
                        let mut mxd_2: ::core::ffi::c_int = decode_cabac_mb_mvd(
                            sl,
                            40 as ::core::ffi::c_int,
                            amvd0_2,
                            &raw mut mpx_2,
                        );
                        let mut myd_2: ::core::ffi::c_int = decode_cabac_mb_mvd(
                            sl,
                            47 as ::core::ffi::c_int,
                            amvd1_2,
                            &raw mut mpy_2,
                        );
                        if mxd_2
                            == -(2147483647 as ::core::ffi::c_int)
                                - 1 as ::core::ffi::c_int
                            || myd_2
                                == -(2147483647 as ::core::ffi::c_int)
                                    - 1 as ::core::ffi::c_int
                        {
                            return -((('I' as i32
                                | ('N' as i32) << 8 as ::core::ffi::c_int
                                | ('D' as i32) << 16 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint
                                | ('A' as i32 as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                        }
                        mx_2 += mxd_2;
                        my_2 += myd_2;
                        fill_rectangle(
                            &raw mut *(&raw mut *(&raw mut (*sl).mvd_cache
                                as *mut [[uint8_t; 2]; 40])
                                .offset(list_0 as isize) as *mut [uint8_t; 2])
                                .offset(
                                    (*(&raw const scan8 as *const uint8_t)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i_1)
                                        as isize,
                                ) as *mut uint8_t as *mut ::core::ffi::c_void,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            pack8to16(
                                mpx_2 as ::core::ffi::c_uint,
                                mpy_2 as ::core::ffi::c_uint,
                            ) as uint32_t,
                            2 as ::core::ffi::c_int,
                        );
                        fill_rectangle(
                            &raw mut *(&raw mut *(&raw mut (*sl).mv_cache
                                as *mut [[int16_t; 2]; 40])
                                .offset(list_0 as isize) as *mut [int16_t; 2])
                                .offset(
                                    (*(&raw const scan8 as *const uint8_t)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i_1)
                                        as isize,
                                ) as *mut int16_t as *mut ::core::ffi::c_void,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            pack16to32(
                                mx_2 as ::core::ffi::c_uint,
                                my_2 as ::core::ffi::c_uint,
                            ),
                            4 as ::core::ffi::c_int,
                        );
                    } else {
                        fill_rectangle(
                            &raw mut *(&raw mut *(&raw mut (*sl).mvd_cache
                                as *mut [[uint8_t; 2]; 40])
                                .offset(list_0 as isize) as *mut [uint8_t; 2])
                                .offset(
                                    (*(&raw const scan8 as *const uint8_t)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i_1)
                                        as isize,
                                ) as *mut uint8_t as *mut ::core::ffi::c_void,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            0 as uint32_t,
                            2 as ::core::ffi::c_int,
                        );
                        fill_rectangle(
                            &raw mut *(&raw mut *(&raw mut (*sl).mv_cache
                                as *mut [[int16_t; 2]; 40])
                                .offset(list_0 as isize) as *mut [int16_t; 2])
                                .offset(
                                    (*(&raw const scan8 as *const uint8_t)
                                        .offset(0 as ::core::ffi::c_int as isize)
                                        as ::core::ffi::c_int + 2 as ::core::ffi::c_int * i_1)
                                        as isize,
                                ) as *mut int16_t as *mut ::core::ffi::c_void,
                            2 as ::core::ffi::c_int,
                            4 as ::core::ffi::c_int,
                            8 as ::core::ffi::c_int,
                            0 as uint32_t,
                            4 as ::core::ffi::c_int,
                        );
                    }
                    i_1 += 1;
                }
                list_0 += 1;
            }
        }
    }
    if mb_type
        & ((1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) != 0
    {
        *(*h).chroma_pred_mode_table.offset(mb_xy as isize) = 0 as uint8_t;
        write_back_motion(h, sl, mb_type);
    }
    if mb_type & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0 {
        cbp = decode_cabac_mb_cbp_luma(sl);
        if decode_chroma != 0 {
            cbp |= decode_cabac_mb_cbp_chroma(sl) << 4 as ::core::ffi::c_int;
        }
    } else if decode_chroma == 0 && cbp > 15 as ::core::ffi::c_int {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"gray chroma\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*sl).cbp = cbp;
    *(*h).cbp_table.offset(mb_xy as isize) = (*sl).cbp as uint16_t;
    if dct8x8_allowed != 0 && cbp & 15 as ::core::ffi::c_int != 0
        && mb_type & 7 as ::core::ffi::c_int == 0
    {
        mb_type
            |= 0x1000000 as ::core::ffi::c_int
                * get_cabac_noinline(
                    &raw mut (*sl).cabac,
                    (&raw mut (*sl).cabac_state as *mut uint8_t)
                        .offset(
                            (399 as ::core::ffi::c_int + (*sl).neighbor_transform_size)
                                as isize,
                        ) as *mut uint8_t,
                );
    }
    if (*(*h).ps.sps).chroma_format_idc == 3 as ::core::ffi::c_int
        && mb_type & 0x1000000 as ::core::ffi::c_int != 0
    {
        let mut i_2: ::core::ffi::c_int = 0;
        let mut nnz_cache: *mut uint8_t = &raw mut (*sl).non_zero_count_cache
            as *mut uint8_t;
        if ((*h).x264_build as ::core::ffi::c_uint) < 151 as ::core::ffi::c_uint {
            i_2 = 0 as ::core::ffi::c_int;
            while i_2 < 2 as ::core::ffi::c_int {
                if (*sl).left_type[i_2 as usize] != 0
                    && (*sl).left_type[i_2 as usize] & 0x1000000 as ::core::ffi::c_int
                        == 0
                {
                    let ref mut fresh8 = *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i_2)
                                as isize,
                        );
                    *fresh8 = (if mb_type & 7 as ::core::ffi::c_int != 0 {
                        64 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint8_t;
                    let ref mut fresh9 = *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 11 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i_2)
                                as isize,
                        );
                    *fresh9 = *fresh8;
                    let ref mut fresh10 = *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i_2)
                                as isize,
                        );
                    *fresh10 = *fresh9;
                    let ref mut fresh11 = *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i_2)
                                as isize,
                        );
                    *fresh11 = *fresh10;
                    let ref mut fresh12 = *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i_2)
                                as isize,
                        );
                    *fresh12 = *fresh11;
                    *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i_2)
                                as isize,
                        ) = *fresh12;
                }
                i_2 += 1;
            }
            if (*sl).top_type != 0
                && (*sl).top_type & 0x1000000 as ::core::ffi::c_int == 0
            {
                let mut top_empty: uint32_t = (if mb_type & 7 as ::core::ffi::c_int == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    0x40404040 as ::core::ffi::c_int
                }) as uint32_t;
                (*(nnz_cache
                    .offset(
                        (4 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut av_alias32))
                    .u32_0 = top_empty;
                (*(nnz_cache
                    .offset(
                        (4 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut av_alias32))
                    .u32_0 = top_empty;
                (*(nnz_cache
                    .offset(
                        (4 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut uint8_t as *mut av_alias32))
                    .u32_0 = top_empty;
            }
        } else {
            i_2 = 0 as ::core::ffi::c_int;
            while i_2 < 2 as ::core::ffi::c_int {
                if (*sl).left_type[i_2 as usize] != 0
                    && (*sl).left_type[i_2 as usize] & 0x1000000 as ::core::ffi::c_int
                        == 0
                {
                    let ref mut fresh13 = *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 12 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i_2)
                                as isize,
                        );
                    *fresh13 = (if (*sl).left_type[i_2 as usize]
                        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        64 as ::core::ffi::c_int
                    }) as uint8_t;
                    let ref mut fresh14 = *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 11 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i_2)
                                as isize,
                        );
                    *fresh14 = *fresh13;
                    let ref mut fresh15 = *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 7 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i_2)
                                as isize,
                        );
                    *fresh15 = *fresh14;
                    let ref mut fresh16 = *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 6 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i_2)
                                as isize,
                        );
                    *fresh16 = *fresh15;
                    let ref mut fresh17 = *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i_2)
                                as isize,
                        );
                    *fresh17 = *fresh16;
                    *nnz_cache
                        .offset(
                            (3 as ::core::ffi::c_int
                                + 8 as ::core::ffi::c_int * 1 as ::core::ffi::c_int
                                + 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int * i_2)
                                as isize,
                        ) = *fresh17;
                }
                i_2 += 1;
            }
            if (*sl).top_type != 0
                && (*sl).top_type & 0x1000000 as ::core::ffi::c_int == 0
            {
                let mut top_empty_0: uint32_t = (if (*sl).top_type
                    & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
                {
                    0 as ::core::ffi::c_int
                } else {
                    0x40404040 as ::core::ffi::c_int
                }) as uint32_t;
                (*(nnz_cache
                    .offset(
                        (4 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 0 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut av_alias32))
                    .u32_0 = top_empty_0;
                (*(nnz_cache
                    .offset(
                        (4 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 5 as ::core::ffi::c_int) as isize,
                    ) as *mut uint8_t as *mut av_alias32))
                    .u32_0 = top_empty_0;
                (*(nnz_cache
                    .offset(
                        (4 as ::core::ffi::c_int
                            + 8 as ::core::ffi::c_int * 10 as ::core::ffi::c_int)
                            as isize,
                    ) as *mut uint8_t as *mut av_alias32))
                    .u32_0 = top_empty_0;
            }
        }
    }
    *(*h).cur_pic.mb_type.offset(mb_xy as isize) = mb_type as uint32_t;
    if cbp != 0 || mb_type & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
        let mut scan: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut scan8x8: *const uint8_t = ::core::ptr::null::<uint8_t>();
        let mut qmul: *const uint32_t = ::core::ptr::null::<uint32_t>();
        if get_cabac_noinline(
            &raw mut (*sl).cabac,
            (&raw mut (*sl).cabac_state as *mut uint8_t)
                .offset(
                    (60 as ::core::ffi::c_int
                        + ((*sl).last_qscale_diff != 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_int) as isize,
                ) as *mut uint8_t,
        ) != 0
        {
            let mut val: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut ctx_0: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
            let max_qp: ::core::ffi::c_int = 51 as ::core::ffi::c_int
                + 6 as ::core::ffi::c_int
                    * ((*sps).bit_depth_luma - 8 as ::core::ffi::c_int);
            while get_cabac_noinline(
                &raw mut (*sl).cabac,
                (&raw mut (*sl).cabac_state as *mut uint8_t)
                    .offset((60 as ::core::ffi::c_int + ctx_0) as isize) as *mut uint8_t,
            ) != 0
            {
                ctx_0 = 3 as ::core::ffi::c_int;
                val += 1;
                if val > 2 as ::core::ffi::c_int * max_qp {
                    av_log(
                        (*h).avctx as *mut ::core::ffi::c_void,
                        16 as ::core::ffi::c_int,
                        b"cabac decode of qscale diff failed at %d %d\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*sl).mb_x,
                        (*sl).mb_y,
                    );
                    return -(1 as ::core::ffi::c_int);
                }
            }
            if val & 0x1 as ::core::ffi::c_int != 0 {
                val = val + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int;
            } else {
                val = -(val + 1 as ::core::ffi::c_int >> 1 as ::core::ffi::c_int);
            }
            (*sl).last_qscale_diff = val;
            (*sl).qscale += val;
            if (*sl).qscale as ::core::ffi::c_uint > max_qp as ::core::ffi::c_uint {
                if (*sl).qscale < 0 as ::core::ffi::c_int {
                    (*sl).qscale += max_qp + 1 as ::core::ffi::c_int;
                } else {
                    (*sl).qscale -= max_qp + 1 as ::core::ffi::c_int;
                }
            }
            (*sl).chroma_qp[0 as ::core::ffi::c_int as usize] = get_chroma_qp(
                (*h).ps.pps,
                0 as ::core::ffi::c_int,
                (*sl).qscale,
            );
            (*sl).chroma_qp[1 as ::core::ffi::c_int as usize] = get_chroma_qp(
                (*h).ps.pps,
                1 as ::core::ffi::c_int,
                (*sl).qscale,
            );
        } else {
            (*sl).last_qscale_diff = 0 as ::core::ffi::c_int;
        }
        if mb_type & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0 {
            scan8x8 = if (*sl).qscale != 0 {
                &raw const (*h).field_scan8x8 as *const uint8_t
            } else {
                &raw const (*h).field_scan8x8_q0 as *const uint8_t
            };
            scan = if (*sl).qscale != 0 {
                &raw const (*h).field_scan as *const uint8_t
            } else {
                &raw const (*h).field_scan_q0 as *const uint8_t
            };
        } else {
            scan8x8 = if (*sl).qscale != 0 {
                &raw const (*h).zigzag_scan8x8 as *const uint8_t
            } else {
                &raw const (*h).zigzag_scan8x8_q0 as *const uint8_t
            };
            scan = if (*sl).qscale != 0 {
                &raw const (*h).zigzag_scan as *const uint8_t
            } else {
                &raw const (*h).zigzag_scan_q0 as *const uint8_t
            };
        }
        decode_cabac_luma_residual(
            h,
            sl,
            scan,
            scan8x8,
            pixel_shift,
            mb_type,
            cbp,
            0 as ::core::ffi::c_int,
        );
        if (*(*h).ps.sps).chroma_format_idc == 3 as ::core::ffi::c_int {
            decode_cabac_luma_residual(
                h,
                sl,
                scan,
                scan8x8,
                pixel_shift,
                mb_type,
                cbp,
                1 as ::core::ffi::c_int,
            );
            decode_cabac_luma_residual(
                h,
                sl,
                scan,
                scan8x8,
                pixel_shift,
                mb_type,
                cbp,
                2 as ::core::ffi::c_int,
            );
        } else if (*(*h).ps.sps).chroma_format_idc == 2 as ::core::ffi::c_int {
            if cbp & 0x30 as ::core::ffi::c_int != 0 {
                let mut c: ::core::ffi::c_int = 0;
                c = 0 as ::core::ffi::c_int;
                while c < 2 as ::core::ffi::c_int {
                    decode_cabac_residual_dc_422(
                        h,
                        sl,
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                (256 as ::core::ffi::c_int
                                    + 16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * c
                                    << pixel_shift) as isize,
                            ),
                        3 as ::core::ffi::c_int,
                        49 as ::core::ffi::c_int + c,
                        &raw const ff_h264_chroma422_dc_scan as *const uint8_t,
                        8 as ::core::ffi::c_int,
                    );
                    c += 1;
                }
            }
            if cbp & 0x20 as ::core::ffi::c_int != 0 {
                let mut c_0: ::core::ffi::c_int = 0;
                let mut i_3: ::core::ffi::c_int = 0;
                let mut i8x8: ::core::ffi::c_int = 0;
                c_0 = 0 as ::core::ffi::c_int;
                while c_0 < 2 as ::core::ffi::c_int {
                    let mut mb: *mut int16_t = (&raw mut (*sl).mb as *mut int16_t)
                        .offset(
                            (16 as ::core::ffi::c_int
                                * (16 as ::core::ffi::c_int
                                    + 16 as ::core::ffi::c_int * c_0) << pixel_shift) as isize,
                        );
                    qmul = &raw mut *(*(&raw const (*(*h).ps.pps).dequant4_coeff
                        as *const *mut [uint32_t; 16])
                        .offset(
                            (c_0 + 1 as ::core::ffi::c_int
                                + (if mb_type & 7 as ::core::ffi::c_int != 0 {
                                    0 as ::core::ffi::c_int
                                } else {
                                    3 as ::core::ffi::c_int
                                })) as isize,
                        ))
                        .offset(
                            *(&raw mut (*sl).chroma_qp as *mut ::core::ffi::c_int)
                                .offset(c_0 as isize) as isize,
                        ) as *mut uint32_t;
                    i8x8 = 0 as ::core::ffi::c_int;
                    while i8x8 < 2 as ::core::ffi::c_int {
                        i_3 = 0 as ::core::ffi::c_int;
                        while i_3 < 4 as ::core::ffi::c_int {
                            let index_0: ::core::ffi::c_int = 16 as ::core::ffi::c_int
                                + 16 as ::core::ffi::c_int * c_0
                                + 8 as ::core::ffi::c_int * i8x8 + i_3;
                            decode_cabac_residual_nondc(
                                h,
                                sl,
                                mb,
                                4 as ::core::ffi::c_int,
                                index_0,
                                scan.offset(1 as ::core::ffi::c_int as isize),
                                qmul,
                                15 as ::core::ffi::c_int,
                            );
                            mb = mb
                                .offset(
                                    ((16 as ::core::ffi::c_int) << pixel_shift) as isize,
                                );
                            i_3 += 1;
                        }
                        i8x8 += 1;
                    }
                    c_0 += 1;
                }
            } else {
                fill_rectangle(
                    (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                        .offset(
                            *(&raw const scan8 as *const uint8_t)
                                .offset(16 as ::core::ffi::c_int as isize) as isize,
                        ) as *mut uint8_t as *mut ::core::ffi::c_void,
                    4 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    0 as uint32_t,
                    1 as ::core::ffi::c_int,
                );
                fill_rectangle(
                    (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                        .offset(
                            *(&raw const scan8 as *const uint8_t)
                                .offset(32 as ::core::ffi::c_int as isize) as isize,
                        ) as *mut uint8_t as *mut ::core::ffi::c_void,
                    4 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    0 as uint32_t,
                    1 as ::core::ffi::c_int,
                );
            }
        } else {
            if cbp & 0x30 as ::core::ffi::c_int != 0 {
                let mut c_1: ::core::ffi::c_int = 0;
                c_1 = 0 as ::core::ffi::c_int;
                while c_1 < 2 as ::core::ffi::c_int {
                    decode_cabac_residual_dc(
                        h,
                        sl,
                        (&raw mut (*sl).mb as *mut int16_t)
                            .offset(
                                (256 as ::core::ffi::c_int
                                    + 16 as ::core::ffi::c_int * 16 as ::core::ffi::c_int * c_1
                                    << pixel_shift) as isize,
                            ),
                        3 as ::core::ffi::c_int,
                        49 as ::core::ffi::c_int + c_1,
                        &raw const ff_h264_chroma_dc_scan as *const uint8_t,
                        4 as ::core::ffi::c_int,
                    );
                    c_1 += 1;
                }
            }
            if cbp & 0x20 as ::core::ffi::c_int != 0 {
                let mut c_2: ::core::ffi::c_int = 0;
                let mut i_4: ::core::ffi::c_int = 0;
                c_2 = 0 as ::core::ffi::c_int;
                while c_2 < 2 as ::core::ffi::c_int {
                    qmul = &raw mut *(*(&raw const (*(*h).ps.pps).dequant4_coeff
                        as *const *mut [uint32_t; 16])
                        .offset(
                            (c_2 + 1 as ::core::ffi::c_int
                                + (if mb_type & 7 as ::core::ffi::c_int != 0 {
                                    0 as ::core::ffi::c_int
                                } else {
                                    3 as ::core::ffi::c_int
                                })) as isize,
                        ))
                        .offset(
                            *(&raw mut (*sl).chroma_qp as *mut ::core::ffi::c_int)
                                .offset(c_2 as isize) as isize,
                        ) as *mut uint32_t;
                    i_4 = 0 as ::core::ffi::c_int;
                    while i_4 < 4 as ::core::ffi::c_int {
                        let index_1: ::core::ffi::c_int = 16 as ::core::ffi::c_int
                            + 16 as ::core::ffi::c_int * c_2 + i_4;
                        decode_cabac_residual_nondc(
                            h,
                            sl,
                            (&raw mut (*sl).mb as *mut int16_t)
                                .offset(
                                    (16 as ::core::ffi::c_int * index_1 << pixel_shift) as isize,
                                ),
                            4 as ::core::ffi::c_int,
                            index_1,
                            scan.offset(1 as ::core::ffi::c_int as isize),
                            qmul,
                            15 as ::core::ffi::c_int,
                        );
                        i_4 += 1;
                    }
                    c_2 += 1;
                }
            } else {
                fill_rectangle(
                    (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                        .offset(
                            *(&raw const scan8 as *const uint8_t)
                                .offset(16 as ::core::ffi::c_int as isize) as isize,
                        ) as *mut uint8_t as *mut ::core::ffi::c_void,
                    4 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    0 as uint32_t,
                    1 as ::core::ffi::c_int,
                );
                fill_rectangle(
                    (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                        .offset(
                            *(&raw const scan8 as *const uint8_t)
                                .offset(32 as ::core::ffi::c_int as isize) as isize,
                        ) as *mut uint8_t as *mut ::core::ffi::c_void,
                    4 as ::core::ffi::c_int,
                    4 as ::core::ffi::c_int,
                    8 as ::core::ffi::c_int,
                    0 as uint32_t,
                    1 as ::core::ffi::c_int,
                );
            }
        }
    } else {
        fill_rectangle(
            (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                .offset(
                    *(&raw const scan8 as *const uint8_t)
                        .offset(0 as ::core::ffi::c_int as isize) as isize,
                ) as *mut uint8_t as *mut ::core::ffi::c_void,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            0 as uint32_t,
            1 as ::core::ffi::c_int,
        );
        fill_rectangle(
            (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                .offset(
                    *(&raw const scan8 as *const uint8_t)
                        .offset(16 as ::core::ffi::c_int as isize) as isize,
                ) as *mut uint8_t as *mut ::core::ffi::c_void,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            0 as uint32_t,
            1 as ::core::ffi::c_int,
        );
        fill_rectangle(
            (&raw mut (*sl).non_zero_count_cache as *mut uint8_t)
                .offset(
                    *(&raw const scan8 as *const uint8_t)
                        .offset(32 as ::core::ffi::c_int as isize) as isize,
                ) as *mut uint8_t as *mut ::core::ffi::c_void,
            4 as ::core::ffi::c_int,
            4 as ::core::ffi::c_int,
            8 as ::core::ffi::c_int,
            0 as uint32_t,
            1 as ::core::ffi::c_int,
        );
        (*sl).last_qscale_diff = 0 as ::core::ffi::c_int;
    }
    *(*h).cur_pic.qscale_table.offset(mb_xy as isize) = (*sl).qscale as int8_t;
    write_back_non_zero_count(h, sl);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn run_static_initializers() {
    ff_h264_norm_shift = (&raw const ff_h264_cabac_tables as *const uint8_t)
        .offset(0 as ::core::ffi::c_int as isize);
    ff_h264_lps_range = (&raw const ff_h264_cabac_tables as *const uint8_t)
        .offset(512 as ::core::ffi::c_int as isize);
    ff_h264_mlps_state = (&raw const ff_h264_cabac_tables as *const uint8_t)
        .offset(1024 as ::core::ffi::c_int as isize);
    ff_h264_last_coeff_flag_offset_8x8 = (&raw const ff_h264_cabac_tables
        as *const uint8_t)
        .offset(1280 as ::core::ffi::c_int as isize);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
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
