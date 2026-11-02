// Translated from FFmpeg 9.0.1 libavcodec/utils.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// utils for libavcodec
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
    fn ff_timecode_set_smpte(
        drop_0: *mut ::core::ffi::c_uint,
        hh: *mut ::core::ffi::c_uint,
        mm: *mut ::core::ffi::c_uint,
        ss: *mut ::core::ffi::c_uint,
        ff: *mut ::core::ffi::c_uint,
        rate: AVRational,
        tcsmpte: uint32_t,
        prevent_df: ::core::ffi::c_int,
        skip_field: ::core::ffi::c_int,
    );
    fn avcodec_descriptor_get(id: AVCodecID) -> *const AVCodecDescriptor;
    fn av_refstruct_ref(obj: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_sample_fmt_is_planar(sample_fmt: AVSampleFormat) -> ::core::ffi::c_int;
    fn av_refstruct_replace(
        dstp: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
    );
    fn av_samples_get_buffer_size(
        linesize: *mut ::core::ffi::c_int,
        nb_channels: ::core::ffi::c_int,
        nb_samples: ::core::ffi::c_int,
        sample_fmt: AVSampleFormat,
        align: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn av_get_bits_per_pixel(pixdesc: *const AVPixFmtDescriptor) -> ::core::ffi::c_int;
    fn av_pix_fmt_desc_get(pix_fmt: AVPixelFormat) -> *const AVPixFmtDescriptor;
    fn av_samples_fill_arrays(
        audio_data: *mut *mut uint8_t,
        linesize: *mut ::core::ffi::c_int,
        buf: *const uint8_t,
        nb_channels: ::core::ffi::c_int,
        nb_samples: ::core::ffi::c_int,
        sample_fmt: AVSampleFormat,
        align: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_image_check_size2(
        w: ::core::ffi::c_uint,
        h: ::core::ffi::c_uint,
        max_pixels: int64_t,
        pix_fmt: AVPixelFormat,
        log_offset: ::core::ffi::c_int,
        log_ctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn av_image_check_sar(
        w: ::core::ffi::c_uint,
        h: ::core::ffi::c_uint,
        sar: AVRational,
    ) -> ::core::ffi::c_int;
    fn av_fast_malloc(
        ptr: *mut ::core::ffi::c_void,
        size: *mut ::core::ffi::c_uint,
        min_size: size_t,
    );
    fn av_fast_mallocz(
        ptr: *mut ::core::ffi::c_void,
        size: *mut ::core::ffi::c_uint,
        min_size: size_t,
    );
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_frame_ref(dst: *mut AVFrame, src: *const AVFrame) -> ::core::ffi::c_int;
    fn av_frame_replace(dst: *mut AVFrame, src: *const AVFrame) -> ::core::ffi::c_int;
    fn av_frame_new_side_data(
        frame: *mut AVFrame,
        type_0: AVFrameSideDataType,
        size: size_t,
    ) -> *mut AVFrameSideData;
    fn av_frame_get_side_data(
        frame: *const AVFrame,
        type_0: AVFrameSideDataType,
    ) -> *mut AVFrameSideData;
}
pub type size_t = ::core::ffi::c_uint;
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
pub type intptr_t = ::core::ffi::c_int;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulonglong;
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
pub type AVMatrixEncoding = ::core::ffi::c_uint;
pub const AV_MATRIX_ENCODING_NB: AVMatrixEncoding = 7;
pub const AV_MATRIX_ENCODING_DOLBYHEADPHONE: AVMatrixEncoding = 6;
pub const AV_MATRIX_ENCODING_DOLBYEX: AVMatrixEncoding = 5;
pub const AV_MATRIX_ENCODING_DPLIIZ: AVMatrixEncoding = 4;
pub const AV_MATRIX_ENCODING_DPLIIX: AVMatrixEncoding = 3;
pub const AV_MATRIX_ENCODING_DPLII: AVMatrixEncoding = 2;
pub const AV_MATRIX_ENCODING_DOLBY: AVMatrixEncoding = 1;
pub const AV_MATRIX_ENCODING_NONE: AVMatrixEncoding = 0;
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
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
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
pub struct AVComponentDescriptor {
    pub plane: ::core::ffi::c_int,
    pub step: ::core::ffi::c_int,
    pub offset: ::core::ffi::c_int,
    pub shift: ::core::ffi::c_int,
    pub depth: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPixFmtDescriptor {
    pub name: *const ::core::ffi::c_char,
    pub nb_components: uint8_t,
    pub log2_chroma_w: uint8_t,
    pub log2_chroma_h: uint8_t,
    pub flags: uint64_t,
    pub comp: [AVComponentDescriptor; 4],
    pub alias: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
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
pub struct AVCodecDescriptor {
    pub id: AVCodecID,
    pub type_0: AVMediaType,
    pub name: *const ::core::ffi::c_char,
    pub long_name: *const ::core::ffi::c_char,
    pub props: ::core::ffi::c_int,
    pub mime_types: *const *const ::core::ffi::c_char,
    pub profiles: *const AVProfile,
}
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
    pub cb: C2RustUnnamed_3,
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
    pub c2rust_unnamed: C2RustUnnamed_0,
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
pub union C2RustUnnamed_0 {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub c2rust_unnamed_0: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub ch_layouts: *const AVChannelLayout,
    pub supported_samplerates: *const ::core::ffi::c_int,
    pub sample_fmts: *const AVSampleFormat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
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
pub struct AVPacketSideData {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub type_0: AVPacketSideDataType,
}
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
pub type AVDiscard = ::core::ffi::c_int;
pub const AVDISCARD_ALL: AVDiscard = 48;
pub const AVDISCARD_NONKEY: AVDiscard = 32;
pub const AVDISCARD_NONINTRA: AVDiscard = 24;
pub const AVDISCARD_BIDIR: AVDiscard = 16;
pub const AVDISCARD_NONREF: AVDiscard = 8;
pub const AVDISCARD_DEFAULT: AVDiscard = 0;
pub const AVDISCARD_NONE: AVDiscard = -16;
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
pub struct RcOverride {
    pub start_frame: ::core::ffi::c_int,
    pub end_frame: ::core::ffi::c_int,
    pub qscale: ::core::ffi::c_int,
    pub quality_factor: ::core::ffi::c_float,
}
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
pub type AVFieldOrder = ::core::ffi::c_uint;
pub const AV_FIELD_BT: AVFieldOrder = 5;
pub const AV_FIELD_TB: AVFieldOrder = 4;
pub const AV_FIELD_BB: AVFieldOrder = 3;
pub const AV_FIELD_TT: AVFieldOrder = 2;
pub const AV_FIELD_PROGRESSIVE: AVFieldOrder = 1;
pub const AV_FIELD_UNKNOWN: AVFieldOrder = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecInternal {
    pub is_copy: ::core::ffi::c_int,
    pub is_frame_mt: ::core::ffi::c_int,
    pub pad_samples: ::core::ffi::c_int,
    pub pool: *mut FramePool,
    pub progress_frame_pool: *mut AVRefStructPool,
    pub thread_ctx: *mut ::core::ffi::c_void,
    pub in_pkt: *mut AVPacket,
    pub bsf: *mut AVBSFContext,
    pub last_pkt_props: *mut AVPacket,
    pub byte_buffer: *mut uint8_t,
    pub byte_buffer_size: ::core::ffi::c_uint,
    pub frame_thread_encoder: *mut ::core::ffi::c_void,
    pub in_frame: *mut AVFrame,
    pub recon_frame: *mut AVFrame,
    pub needs_close: ::core::ffi::c_int,
    pub skip_samples: ::core::ffi::c_int,
    pub hwaccel_priv_data: *mut ::core::ffi::c_void,
    pub draining: ::core::ffi::c_int,
    pub buffer_pkt: *mut AVPacket,
    pub buffer_frame: *mut AVFrame,
    pub draining_done: ::core::ffi::c_int,
    pub warned_on_failed_allocation_from_fixed_pool: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecHWConfigInternal {
    pub public: AVCodecHWConfig,
    pub hwaccel: *const FFHWAccel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecHWConfig {
    pub pix_fmt: AVPixelFormat,
    pub methods: ::core::ffi::c_int,
    pub device_type: AVHWDeviceType,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_3 {
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
pub type AVSubtitleType = ::core::ffi::c_uint;
pub const SUBTITLE_ASS: AVSubtitleType = 3;
pub const SUBTITLE_TEXT: AVSubtitleType = 2;
pub const SUBTITLE_BITMAP: AVSubtitleType = 1;
pub const SUBTITLE_NONE: AVSubtitleType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFCodecDefault {
    pub key: *const ::core::ffi::c_char,
    pub value: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCPBProperties {
    pub max_bitrate: int64_t,
    pub min_bitrate: int64_t,
    pub avg_bitrate: int64_t,
    pub buffer_size: int64_t,
    pub vbv_delay: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecParameters {
    pub codec_type: AVMediaType,
    pub codec_id: AVCodecID,
    pub codec_tag: uint32_t,
    pub extradata: *mut uint8_t,
    pub extradata_size: ::core::ffi::c_int,
    pub coded_side_data: *mut AVPacketSideData,
    pub nb_coded_side_data: ::core::ffi::c_int,
    pub format: ::core::ffi::c_int,
    pub bit_rate: int64_t,
    pub bits_per_coded_sample: ::core::ffi::c_int,
    pub bits_per_raw_sample: ::core::ffi::c_int,
    pub profile: ::core::ffi::c_int,
    pub level: ::core::ffi::c_int,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub sample_aspect_ratio: AVRational,
    pub framerate: AVRational,
    pub field_order: AVFieldOrder,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub chroma_location: AVChromaLocation,
    pub video_delay: ::core::ffi::c_int,
    pub ch_layout: AVChannelLayout,
    pub sample_rate: ::core::ffi::c_int,
    pub block_align: ::core::ffi::c_int,
    pub frame_size: ::core::ffi::c_int,
    pub initial_padding: ::core::ffi::c_int,
    pub trailing_padding: ::core::ffi::c_int,
    pub seek_preroll: ::core::ffi::c_int,
    pub alpha_mode: AVAlphaMode,
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
pub struct PutBitContext {
    pub bit_buf: BitBuf,
    pub bit_left: ::core::ffi::c_int,
    pub buf: *mut uint8_t,
    pub buf_ptr: *mut uint8_t,
    pub buf_end: *mut uint8_t,
}
pub type BitBuf = uint32_t;
#[no_mangle]
pub unsafe extern "C" fn av_fast_padded_malloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: *mut ::core::ffi::c_uint,
    mut min_size: size_t,
) {
    let mut p: *mut *mut uint8_t = ptr as *mut *mut uint8_t;
    if min_size
        > (0xffffffff as ::core::ffi::c_uint).wrapping_sub(64 as ::core::ffi::c_uint)
    {
        av_freep(p as *mut ::core::ffi::c_void);
        *size = 0 as ::core::ffi::c_uint;
        return;
    }
    av_fast_mallocz(
        p as *mut ::core::ffi::c_void,
        size,
        min_size.wrapping_add(64 as size_t),
    );
    if !(*p).is_null() {
        memset(
            (*p).offset(min_size as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            64 as size_t,
        );
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
#[inline]
unsafe extern "C" fn init_put_bits(
    mut s: *mut PutBitContext,
    mut buffer: *mut uint8_t,
    mut buffer_size: ::core::ffi::c_int,
) {
    if buffer_size < 0 as ::core::ffi::c_int {
        buffer_size = 0 as ::core::ffi::c_int;
        buffer = ::core::ptr::null_mut::<uint8_t>();
    }
    (*s).buf = buffer;
    (*s).buf_end = (*s).buf.offset(buffer_size as isize);
    (*s).buf_ptr = (*s).buf;
    (*s).bit_left = 32 as ::core::ffi::c_int;
    (*s).bit_buf = 0 as BitBuf;
}
#[no_mangle]
pub unsafe extern "C" fn av_fast_padded_mallocz(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: *mut ::core::ffi::c_uint,
    mut min_size: size_t,
) {
    let mut p: *mut *mut uint8_t = ptr as *mut *mut uint8_t;
    if min_size
        > (0xffffffff as ::core::ffi::c_uint).wrapping_sub(64 as ::core::ffi::c_uint)
    {
        av_freep(p as *mut ::core::ffi::c_void);
        *size = 0 as ::core::ffi::c_uint;
        return;
    }
    av_fast_malloc(
        p as *mut ::core::ffi::c_void,
        size,
        min_size.wrapping_add(64 as size_t),
    );
    if !(*p).is_null() {
        memset(
            *p as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            min_size.wrapping_add(64 as size_t),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_codec_is_encoder(
    mut avcodec: *const AVCodec,
) -> ::core::ffi::c_int {
    let codec: *const FFCodec = ffcodec(avcodec) as *const FFCodec;
    return (!codec.is_null() && (*codec).is_decoder() == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_codec_is_decoder(
    mut avcodec: *const AVCodec,
) -> ::core::ffi::c_int {
    let codec: *const FFCodec = ffcodec(avcodec) as *const FFCodec;
    return (!codec.is_null() && (*codec).is_decoder() as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_set_dimensions(
    mut s: *mut AVCodecContext,
    mut width: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = av_image_check_size2(
        width as ::core::ffi::c_uint,
        height as ::core::ffi::c_uint,
        (*s).max_pixels,
        AV_PIX_FMT_NONE,
        0 as ::core::ffi::c_int,
        s as *mut ::core::ffi::c_void,
    );
    if ret < 0 as ::core::ffi::c_int {
        height = 0 as ::core::ffi::c_int;
        width = height;
    }
    (*s).coded_width = width;
    (*s).coded_height = height;
    (*s).width = if 0 == 0 {
        -(-width >> (*s).lowres)
    } else {
        width + ((1 as ::core::ffi::c_int) << (*s).lowres) - 1 as ::core::ffi::c_int
            >> (*s).lowres
    };
    (*s).height = if 0 == 0 {
        -(-height >> (*s).lowres)
    } else {
        height + ((1 as ::core::ffi::c_int) << (*s).lowres) - 1 as ::core::ffi::c_int
            >> (*s).lowres
    };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ff_set_sar(
    mut avctx: *mut AVCodecContext,
    mut sar: AVRational,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = av_image_check_sar(
        (*avctx).width as ::core::ffi::c_uint,
        (*avctx).height as ::core::ffi::c_uint,
        sar,
    );
    if ret < 0 as ::core::ffi::c_int {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            24 as ::core::ffi::c_int,
            b"ignoring invalid SAR: %d/%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            sar.num,
            sar.den,
        );
        (*avctx).sample_aspect_ratio = AVRational {
            num: 0 as ::core::ffi::c_int,
            den: 1 as ::core::ffi::c_int,
        };
        return ret;
    } else {
        (*avctx).sample_aspect_ratio = sar;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_side_data_update_matrix_encoding(
    mut frame: *mut AVFrame,
    mut matrix_encoding: AVMatrixEncoding,
) -> ::core::ffi::c_int {
    let mut side_data: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    let mut data: *mut AVMatrixEncoding = ::core::ptr::null_mut::<AVMatrixEncoding>();
    side_data = av_frame_get_side_data(frame, AV_FRAME_DATA_MATRIXENCODING);
    if side_data.is_null() {
        side_data = av_frame_new_side_data(
            frame,
            AV_FRAME_DATA_MATRIXENCODING,
            ::core::mem::size_of::<AVMatrixEncoding>() as size_t,
        );
    }
    if side_data.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    data = (*side_data).data as *mut AVMatrixEncoding;
    *data = matrix_encoding;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_align_dimensions2(
    mut s: *mut AVCodecContext,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
    mut linesize_align: *mut ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut w_align: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut h_align: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get((*s).pix_fmt);
    if !desc.is_null() {
        w_align = (1 as ::core::ffi::c_int)
            << (*desc).log2_chroma_w as ::core::ffi::c_int;
        h_align = (1 as ::core::ffi::c_int)
            << (*desc).log2_chroma_h as ::core::ffi::c_int;
    }
    match (*s).pix_fmt as ::core::ffi::c_int {
        0 | 1 | 108 | 15 | 4 | 31 | 5 | 71 | 111 | 8 | 29 | 30 | 12 | 13 | 32 | 14 | 33
        | 78 | 79 | 60 | 59 | 62 | 61 | 123 | 122 | 125 | 124 | 45 | 46 | 81 | 80 | 87
        | 86 | 93 | 92 | 70 | 69 | 64 | 63 | 127 | 126 | 129 | 128 | 47 | 48 | 83 | 82
        | 89 | 88 | 185 | 184 | 95 | 94 | 151 | 152 | 153 | 154 | 66 | 65 | 68 | 67 | 131
        | 130 | 133 | 132 | 49 | 50 | 85 | 84 | 91 | 90 | 187 | 186 | 97 | 96 | 73 | 72
        | 75 | 74 | 135 | 134 | 137 | 136 | 77 | 76 | 161 | 160 | 113 | 112 => {
            w_align = 16 as ::core::ffi::c_int;
            h_align = 16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
            if (*s).codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_BINKVIDEO as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                w_align = 16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
            }
        }
        7 | 138 | 16 => {
            w_align = 32 as ::core::ffi::c_int;
            h_align = 16 as ::core::ffi::c_int * 2 as ::core::ffi::c_int;
        }
        6 => {
            if (*s).codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_SVQ1 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                w_align = 64 as ::core::ffi::c_int;
                h_align = 64 as ::core::ffi::c_int;
            } else if (*s).codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_SNOW as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                w_align = 16 as ::core::ffi::c_int;
                h_align = 16 as ::core::ffi::c_int;
            }
        }
        39 => {
            if (*s).codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_RPZA as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                w_align = 4 as ::core::ffi::c_int;
                h_align = 4 as ::core::ffi::c_int;
            }
            if (*s).codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_INTERPLAY_VIDEO as ::core::ffi::c_int
                    as ::core::ffi::c_uint
            {
                w_align = 8 as ::core::ffi::c_int;
                h_align = 8 as ::core::ffi::c_int;
            }
        }
        11 | 17 | 20 => {
            if (*s).codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_SMC as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*s).codec_id as ::core::ffi::c_uint
                    == AV_CODEC_ID_CINEPAK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                w_align = 4 as ::core::ffi::c_int;
                h_align = 4 as ::core::ffi::c_int;
            }
            if (*s).codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_JV as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*s).codec_id as ::core::ffi::c_uint
                    == AV_CODEC_ID_ARGO as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*s).codec_id as ::core::ffi::c_uint
                    == AV_CODEC_ID_INTERPLAY_VIDEO as ::core::ffi::c_int
                        as ::core::ffi::c_uint
            {
                w_align = 8 as ::core::ffi::c_int;
                h_align = 8 as ::core::ffi::c_int;
            }
            if (*s).codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_MJPEG as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*s).codec_id as ::core::ffi::c_uint
                    == AV_CODEC_ID_MJPEGB as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*s).codec_id as ::core::ffi::c_uint
                    == AV_CODEC_ID_LJPEG as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*s).codec_id as ::core::ffi::c_uint
                    == AV_CODEC_ID_SMVJPEG as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*s).codec_id as ::core::ffi::c_uint
                    == AV_CODEC_ID_AMV as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*s).codec_id as ::core::ffi::c_uint
                    == AV_CODEC_ID_SP5X as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*s).codec_id as ::core::ffi::c_uint
                    == AV_CODEC_ID_JPEGLS as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                w_align = 8 as ::core::ffi::c_int;
                h_align = 2 as ::core::ffi::c_int * 8 as ::core::ffi::c_int;
            }
        }
        3 => {
            if (*s).codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_MSZH as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*s).codec_id as ::core::ffi::c_uint
                    == AV_CODEC_ID_ZLIB as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                w_align = 4 as ::core::ffi::c_int;
                h_align = 4 as ::core::ffi::c_int;
            }
        }
        2 => {
            if (*s).codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_CINEPAK as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                w_align = 4 as ::core::ffi::c_int;
                h_align = 4 as ::core::ffi::c_int;
            }
        }
        121 => {
            if (*s).codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_ARGO as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                w_align = 8 as ::core::ffi::c_int;
                h_align = 8 as ::core::ffi::c_int;
            }
        }
        139 | 140 | 141 | 142 | 143 | 144 | 145 | 146 | 147 | 148 | 149 | 150 => {
            w_align = if w_align > 2 as ::core::ffi::c_int {
                w_align
            } else {
                2 as ::core::ffi::c_int
            };
            h_align = if h_align > 2 as ::core::ffi::c_int {
                h_align
            } else {
                2 as ::core::ffi::c_int
            };
        }
        _ => {}
    }
    if (*s).codec_id as ::core::ffi::c_uint
        == AV_CODEC_ID_IFF_ILBM as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        w_align = if w_align > 16 as ::core::ffi::c_int {
            w_align
        } else {
            16 as ::core::ffi::c_int
        };
    }
    *width = *width + w_align - 1 as ::core::ffi::c_int
        & !(w_align - 1 as ::core::ffi::c_int);
    *height = *height + h_align - 1 as ::core::ffi::c_int
        & !(h_align - 1 as ::core::ffi::c_int);
    if (*s).codec_id as ::core::ffi::c_uint
        == AV_CODEC_ID_H264 as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*s).lowres != 0
        || (*s).codec_id as ::core::ffi::c_uint
            == AV_CODEC_ID_VC1 as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*s).codec_id as ::core::ffi::c_uint
            == AV_CODEC_ID_WMV3 as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*s).codec_id as ::core::ffi::c_uint
            == AV_CODEC_ID_VP5 as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*s).codec_id as ::core::ffi::c_uint
            == AV_CODEC_ID_VP6 as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*s).codec_id as ::core::ffi::c_uint
            == AV_CODEC_ID_VP6F as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*s).codec_id as ::core::ffi::c_uint
            == AV_CODEC_ID_VP6A as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *height += 2 as ::core::ffi::c_int;
        *width = if *width > 32 as ::core::ffi::c_int {
            *width
        } else {
            32 as ::core::ffi::c_int
        };
    }
    if (*s).codec_id as ::core::ffi::c_uint
        == AV_CODEC_ID_SVQ3 as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *width = if *width > 32 as ::core::ffi::c_int {
            *width
        } else {
            32 as ::core::ffi::c_int
        };
    }
    i = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        *linesize_align.offset(i as isize) = 16 as ::core::ffi::c_int;
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn flush_put_bits(mut s: *mut PutBitContext) {
    if (*s).bit_left < 32 as ::core::ffi::c_int {
        (*s).bit_buf <<= (*s).bit_left;
    }
    while (*s).bit_left < 32 as ::core::ffi::c_int {
        if !((*s).buf_ptr < (*s).buf_end) {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"s->buf_ptr < s->buf_end\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavcodec/put_bits.h\0" as *const u8
                    as *const ::core::ffi::c_char,
                160 as ::core::ffi::c_int,
            );
            abort();
        }
        let fresh1 = (*s).buf_ptr;
        (*s).buf_ptr = (*s).buf_ptr.offset(1);
        *fresh1 = ((*s).bit_buf >> 32 as ::core::ffi::c_int - 8 as ::core::ffi::c_int)
            as uint8_t;
        (*s).bit_buf <<= 8 as ::core::ffi::c_int;
        (*s).bit_left += 8 as ::core::ffi::c_int;
    }
    (*s).bit_left = 32 as ::core::ffi::c_int;
    (*s).bit_buf = 0 as BitBuf;
}
#[inline(always)]
unsafe extern "C" fn av_inv_q(mut q: AVRational) -> AVRational {
    let mut r: AVRational = AVRational {
        num: q.den,
        den: q.num,
    };
    return r;
}
#[inline]
unsafe extern "C" fn put_bits_no_assert(
    mut s: *mut PutBitContext,
    mut n: ::core::ffi::c_int,
    mut value: BitBuf,
) {
    let mut bit_buf: BitBuf = 0;
    let mut bit_left: ::core::ffi::c_int = 0;
    bit_buf = (*s).bit_buf;
    bit_left = (*s).bit_left;
    if n < bit_left {
        bit_buf = bit_buf << n | value;
        bit_left -= n;
    } else {
        bit_buf <<= bit_left;
        bit_buf |= value >> n - bit_left;
        if (*s).buf_end.offset_from((*s).buf_ptr) as ::core::ffi::c_int as usize
            >= ::core::mem::size_of::<BitBuf>() as usize
        {
            (*((*s).buf_ptr as *mut unaligned_32)).l = av_bswap32(bit_buf as uint32_t);
            (*s).buf_ptr = (*s)
                .buf_ptr
                .offset(::core::mem::size_of::<BitBuf>() as usize as isize);
        } else {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                16 as ::core::ffi::c_int,
                b"Internal error, put_bits buffer too small\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
        }
        bit_left += 32 as ::core::ffi::c_int - n;
        bit_buf = value;
    }
    (*s).bit_buf = bit_buf;
    (*s).bit_left = bit_left;
}
#[inline]
unsafe extern "C" fn put_bits(
    mut s: *mut PutBitContext,
    mut n: ::core::ffi::c_int,
    mut value: BitBuf,
) {
    put_bits_no_assert(s, n, value);
}
#[inline(always)]
unsafe extern "C" fn ffcodec(mut codec: *const AVCodec) -> *const FFCodec {
    return codec as *const FFCodec;
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_align_dimensions(
    mut s: *mut AVCodecContext,
    mut width: *mut ::core::ffi::c_int,
    mut height: *mut ::core::ffi::c_int,
) {
    let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get((*s).pix_fmt);
    let mut chroma_shift: ::core::ffi::c_int = (*desc).log2_chroma_w
        as ::core::ffi::c_int;
    let mut linesize_align: [::core::ffi::c_int; 8] = [0; 8];
    let mut align: ::core::ffi::c_int = 0;
    avcodec_align_dimensions2(
        s,
        width,
        height,
        &raw mut linesize_align as *mut ::core::ffi::c_int,
    );
    align = if linesize_align[0 as ::core::ffi::c_int as usize]
        > linesize_align[3 as ::core::ffi::c_int as usize]
    {
        linesize_align[0 as ::core::ffi::c_int as usize]
    } else {
        linesize_align[3 as ::core::ffi::c_int as usize]
    };
    linesize_align[1 as ::core::ffi::c_int as usize] <<= chroma_shift;
    linesize_align[2 as ::core::ffi::c_int as usize] <<= chroma_shift;
    align = if (if align > linesize_align[1 as ::core::ffi::c_int as usize] {
        align
    } else {
        linesize_align[1 as ::core::ffi::c_int as usize]
    }) > linesize_align[2 as ::core::ffi::c_int as usize]
    {
        if align > linesize_align[1 as ::core::ffi::c_int as usize] {
            align
        } else {
            linesize_align[1 as ::core::ffi::c_int as usize]
        }
    } else {
        linesize_align[2 as ::core::ffi::c_int as usize]
    };
    *width = *width + align - 1 as ::core::ffi::c_int
        & !(align - 1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_fill_audio_frame(
    mut frame: *mut AVFrame,
    mut nb_channels: ::core::ffi::c_int,
    mut sample_fmt: AVSampleFormat,
    mut buf: *const uint8_t,
    mut buf_size: ::core::ffi::c_int,
    mut align: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ch: ::core::ffi::c_int = 0;
    let mut planar: ::core::ffi::c_int = 0;
    let mut needed_size: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    needed_size = av_samples_get_buffer_size(
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        nb_channels,
        (*frame).nb_samples,
        sample_fmt,
        align,
    );
    if buf_size < needed_size {
        return -(22 as ::core::ffi::c_int);
    }
    planar = av_sample_fmt_is_planar(sample_fmt);
    if planar != 0 && nb_channels > 8 as ::core::ffi::c_int {
        (*frame).extended_data = av_calloc(
            nb_channels as size_t,
            ::core::mem::size_of::<*mut uint8_t>() as size_t,
        ) as *mut *mut uint8_t;
        if (*frame).extended_data.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
    } else {
        (*frame).extended_data = &raw mut (*frame).data as *mut *mut uint8_t;
    }
    ret = av_samples_fill_arrays(
        (*frame).extended_data,
        (&raw mut (*frame).linesize as *mut ::core::ffi::c_int)
            .offset(0 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int,
        buf as intptr_t as *mut uint8_t,
        nb_channels,
        (*frame).nb_samples,
        sample_fmt,
        align,
    );
    if ret < 0 as ::core::ffi::c_int {
        if (*frame).extended_data != &raw mut (*frame).data as *mut *mut uint8_t {
            av_freep(&raw mut (*frame).extended_data as *mut ::core::ffi::c_void);
        }
        return ret;
    }
    if (*frame).extended_data != &raw mut (*frame).data as *mut *mut uint8_t {
        ch = 0 as ::core::ffi::c_int;
        while ch < 8 as ::core::ffi::c_int {
            (*frame).data[ch as usize] = *(*frame).extended_data.offset(ch as isize);
            ch += 1;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn avpriv_codec_get_cap_skip_frame_fill_param(
    mut codec: *const AVCodec,
) -> ::core::ffi::c_int {
    return ((*ffcodec(codec)).caps_internal() as ::core::ffi::c_int
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_get_name(
    mut id: AVCodecID,
) -> *const ::core::ffi::c_char {
    let mut cd: *const AVCodecDescriptor = ::core::ptr::null::<AVCodecDescriptor>();
    if id as ::core::ffi::c_uint
        == AV_CODEC_ID_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return b"none\0" as *const u8 as *const ::core::ffi::c_char;
    }
    cd = avcodec_descriptor_get(id);
    if !cd.is_null() {
        return (*cd).name;
    }
    return b"unknown_codec\0" as *const u8 as *const ::core::ffi::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_profile_name(
    mut codec: *const AVCodec,
    mut profile: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut p: *const AVProfile = ::core::ptr::null::<AVProfile>();
    if profile == -(99 as ::core::ffi::c_int) || (*codec).profiles.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    p = (*codec).profiles;
    while (*p).profile != -(99 as ::core::ffi::c_int) {
        if (*p).profile == profile {
            return (*p).name;
        }
        p = p.offset(1);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_profile_name(
    mut codec_id: AVCodecID,
    mut profile: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    let mut desc: *const AVCodecDescriptor = avcodec_descriptor_get(codec_id);
    let mut p: *const AVProfile = ::core::ptr::null::<AVProfile>();
    if profile == -(99 as ::core::ffi::c_int) || desc.is_null()
        || (*desc).profiles.is_null()
    {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    p = (*desc).profiles as *const AVProfile;
    while (*p).profile != -(99 as ::core::ffi::c_int) {
        if (*p).profile == profile {
            return (*p).name;
        }
        p = p.offset(1);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn av_get_exact_bits_per_sample(
    mut codec_id: AVCodecID,
) -> ::core::ffi::c_int {
    match codec_id as ::core::ffi::c_uint {
        86112 => return 1 as ::core::ffi::c_int,
        86070 | 86071 | 69674 | 69644 | 69678 | 69651 | 69661 | 69677 | 69655 | 69690
        | 69664 | 69636 | 69675 | 69660 | 69646 | 69670 => return 4 as ::core::ffi::c_int,
        86089 | 86090 | 86091 | 86092 | 65543 | 65542 | 65571 | 65540 | 65563 | 65572
        | 65541 | 81924 | 81928 | 81926 | 81927 | 69692 => return 8 as ::core::ffi::c_int,
        65537 | 65566 | 65536 | 65554 | 65539 | 65538 => return 16 as ::core::ffi::c_int,
        65552 | 65549 | 65548 | 65564 | 65551 | 65550 => return 24 as ::core::ffi::c_int,
        65545 | 65544 | 65565 | 65547 | 65546 | 65556 | 65557 | 65570 | 65569 => {
            return 32 as ::core::ffi::c_int;
        }
        65558 | 65559 | 65568 | 65567 => return 64 as ::core::ffi::c_int,
        _ => return 0 as ::core::ffi::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_get_pcm_codec(
    mut fmt: AVSampleFormat,
    mut be: ::core::ffi::c_int,
) -> AVCodecID {
    static mut map: [[AVCodecID; 2]; 12] = [
        [AV_CODEC_ID_PCM_U8, AV_CODEC_ID_PCM_U8],
        [AV_CODEC_ID_PCM_S16LE, AV_CODEC_ID_PCM_S16BE],
        [AV_CODEC_ID_PCM_S32LE, AV_CODEC_ID_PCM_S32BE],
        [AV_CODEC_ID_PCM_F32LE, AV_CODEC_ID_PCM_F32BE],
        [AV_CODEC_ID_PCM_F64LE, AV_CODEC_ID_PCM_F64BE],
        [AV_CODEC_ID_PCM_U8, AV_CODEC_ID_PCM_U8],
        [AV_CODEC_ID_PCM_S16LE, AV_CODEC_ID_PCM_S16BE],
        [AV_CODEC_ID_PCM_S32LE, AV_CODEC_ID_PCM_S32BE],
        [AV_CODEC_ID_PCM_F32LE, AV_CODEC_ID_PCM_F32BE],
        [AV_CODEC_ID_PCM_F64LE, AV_CODEC_ID_PCM_F64BE],
        [AV_CODEC_ID_NONE; 2],
        [AV_CODEC_ID_PCM_S64LE, AV_CODEC_ID_PCM_S64BE],
    ];
    if (fmt as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
        || fmt as usize
            >= (::core::mem::size_of::<[[AVCodecID; 2]; 12]>() as usize)
                .wrapping_div(::core::mem::size_of::<[AVCodecID; 2]>() as usize)
    {
        return AV_CODEC_ID_NONE;
    }
    if be < 0 as ::core::ffi::c_int || be > 1 as ::core::ffi::c_int {
        be = 0 as ::core::ffi::c_int;
    }
    return map[fmt as usize][be as usize];
}
#[no_mangle]
pub unsafe extern "C" fn av_get_bits_per_sample(
    mut codec_id: AVCodecID,
) -> ::core::ffi::c_int {
    match codec_id as ::core::ffi::c_uint {
        86112 => return 1 as ::core::ffi::c_int,
        69649 | 86123 => return 2 as ::core::ffi::c_int,
        69648 => return 3 as ::core::ffi::c_int,
        69647 | 69633 | 69684 | 69632 | 69645 | 69638 => return 4 as ::core::ffi::c_int,
        _ => return av_get_exact_bits_per_sample(codec_id),
    };
}
unsafe extern "C" fn get_audio_frame_duration(
    mut id: AVCodecID,
    mut sr: ::core::ffi::c_int,
    mut ch: ::core::ffi::c_int,
    mut ba: ::core::ffi::c_int,
    mut tag: uint32_t,
    mut bits_per_coded_sample: ::core::ffi::c_int,
    mut bitrate: int64_t,
    mut extradata: *mut uint8_t,
    mut frame_size: ::core::ffi::c_int,
    mut frame_bytes: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bps: ::core::ffi::c_int = av_get_exact_bits_per_sample(id);
    let mut framecount: ::core::ffi::c_int = if ba > 0 as ::core::ffi::c_int
        && frame_bytes / ba > 0 as ::core::ffi::c_int
    {
        frame_bytes / ba
    } else {
        1 as ::core::ffi::c_int
    };
    if bps > 0 as ::core::ffi::c_int && ch > 0 as ::core::ffi::c_int
        && frame_bytes > 0 as ::core::ffi::c_int && ch < 32768 as ::core::ffi::c_int
        && bps < 32768 as ::core::ffi::c_int
    {
        return (frame_bytes as ::core::ffi::c_longlong * 8 as ::core::ffi::c_longlong
            / (bps * ch) as ::core::ffi::c_longlong) as ::core::ffi::c_int;
    }
    bps = bits_per_coded_sample;
    match id as ::core::ffi::c_uint {
        69641 => return 32 as ::core::ffi::c_int,
        69632 => return 64 as ::core::ffi::c_int,
        69657 => return 128 as ::core::ffi::c_int,
        73728 | 86087 | 86034 | 86040 | 77825 => return 160 as ::core::ffi::c_int,
        73729 | 86046 => return 320 as ::core::ffi::c_int,
        86058 => return 384 as ::core::ffi::c_int,
        86062 => return 512 as ::core::ffi::c_int,
        86104 | 86047 => {
            if framecount > 2147483647 as ::core::ffi::c_int / 1024 as ::core::ffi::c_int
            {
                return 0 as ::core::ffi::c_int;
            }
            return 1024 as ::core::ffi::c_int * framecount;
        }
        86055 => return 2048 as ::core::ffi::c_int,
        86016 | 86044 => return 1152 as ::core::ffi::c_int,
        86019 => return 1536 as ::core::ffi::c_int,
        86116 => return 1024 as ::core::ffi::c_int,
        _ => {}
    }
    if sr > 0 as ::core::ffi::c_int {
        if id as ::core::ffi::c_uint
            == AV_CODEC_ID_TTA as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return (256 as ::core::ffi::c_longlong * sr as ::core::ffi::c_longlong
                / 245 as ::core::ffi::c_longlong) as ::core::ffi::c_int
        } else if id as ::core::ffi::c_uint
            == AV_CODEC_ID_DST as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return (588 as ::core::ffi::c_longlong * sr as ::core::ffi::c_longlong
                / 44100 as ::core::ffi::c_longlong) as ::core::ffi::c_int
        } else if id as ::core::ffi::c_uint
            == AV_CODEC_ID_BINKAUDIO_DCT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if sr / 22050 as ::core::ffi::c_int > 22 as ::core::ffi::c_int {
                return 0 as ::core::ffi::c_int;
            }
            return (480 as ::core::ffi::c_int) << sr / 22050 as ::core::ffi::c_int;
        }
        if id as ::core::ffi::c_uint
            == AV_CODEC_ID_MP3 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return if sr <= 24000 as ::core::ffi::c_int {
                576 as ::core::ffi::c_int
            } else {
                1152 as ::core::ffi::c_int
            };
        }
    }
    if ba > 0 as ::core::ffi::c_int {
        if id as ::core::ffi::c_uint
            == AV_CODEC_ID_SIPR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            match ba {
                20 => return 160 as ::core::ffi::c_int,
                19 => return 144 as ::core::ffi::c_int,
                29 => return 288 as ::core::ffi::c_int,
                37 => return 480 as ::core::ffi::c_int,
                _ => {}
            }
        } else if id as ::core::ffi::c_uint
            == AV_CODEC_ID_ILBC as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            match ba {
                38 => return 160 as ::core::ffi::c_int,
                50 => return 240 as ::core::ffi::c_int,
                _ => {}
            }
        }
    }
    if frame_bytes > 0 as ::core::ffi::c_int {
        let mut d: int64_t = -(1 as ::core::ffi::c_int) as int64_t
            - 0x7fffffffffffffff as int64_t;
        match id as ::core::ffi::c_uint {
            86037 => {
                d = (240 as ::core::ffi::c_longlong
                    * (frame_bytes / 32 as ::core::ffi::c_int)
                        as ::core::ffi::c_longlong) as int64_t;
            }
            86049 => {
                d = (256 as ::core::ffi::c_longlong
                    * (frame_bytes / 64 as ::core::ffi::c_int)
                        as ::core::ffi::c_longlong) as int64_t;
            }
            77824 => {
                d = (160 as ::core::ffi::c_longlong
                    * (frame_bytes / 20 as ::core::ffi::c_int)
                        as ::core::ffi::c_longlong) as int64_t;
            }
            86101 => {
                d = (4 as ::core::ffi::c_longlong
                    * (frame_bytes / 4 as ::core::ffi::c_int) as ::core::ffi::c_longlong)
                    as int64_t;
            }
            86102 => {
                d = (4 as ::core::ffi::c_longlong
                    * (frame_bytes / 6 as ::core::ffi::c_int) as ::core::ffi::c_longlong)
                    as int64_t;
            }
            _ => {}
        }
        if d
            > -(1 as ::core::ffi::c_int) as ::core::ffi::c_longlong
                - 0x7fffffffffffffff as ::core::ffi::c_longlong
        {
            return (if d as ::core::ffi::c_int as int64_t == d && d > 0 as int64_t {
                d
            } else {
                0 as int64_t
            }) as ::core::ffi::c_int;
        }
        if bps > 0 as ::core::ffi::c_int {
            if id as ::core::ffi::c_uint
                == AV_CODEC_ID_ADPCM_G726 as ::core::ffi::c_int as ::core::ffi::c_uint
                || id as ::core::ffi::c_uint
                    == AV_CODEC_ID_ADPCM_G726LE as ::core::ffi::c_int
                        as ::core::ffi::c_uint
            {
                return frame_bytes * 8 as ::core::ffi::c_int / bps;
            }
        }
        if ch > 0 as ::core::ffi::c_int
            && ch < 2147483647 as ::core::ffi::c_int / 16 as ::core::ffi::c_int
        {
            match id as ::core::ffi::c_uint {
                86110 => {
                    return frame_bytes / (40 as ::core::ffi::c_int * ch)
                        * 256 as ::core::ffi::c_int;
                }
                69681 => {
                    return (frame_bytes - 4 as ::core::ffi::c_int * ch)
                        / (128 as ::core::ffi::c_int * ch) * 256 as ::core::ffi::c_int;
                }
                69663 => {
                    return frame_bytes / (9 as ::core::ffi::c_int * ch)
                        * 16 as ::core::ffi::c_int;
                }
                69688 => {
                    frame_bytes /= 9 as ::core::ffi::c_int * ch;
                    if frame_bytes
                        > 2147483647 as ::core::ffi::c_int / 16 as ::core::ffi::c_int
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    return frame_bytes * 16 as ::core::ffi::c_int;
                }
                69669 | 69665 => {
                    frame_bytes /= 16 as ::core::ffi::c_int * ch;
                    if frame_bytes
                        > 2147483647 as ::core::ffi::c_int / 28 as ::core::ffi::c_int
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    return frame_bytes * 28 as ::core::ffi::c_int;
                }
                69691 => {
                    frame_bytes = (frame_bytes - 1 as ::core::ffi::c_int) / ch;
                    if frame_bytes
                        > 2147483647 as ::core::ffi::c_int / 2 as ::core::ffi::c_int
                    {
                        return 0 as ::core::ffi::c_int;
                    }
                    return frame_bytes * 2 as ::core::ffi::c_int;
                }
                69639 | 69682 | 69671 | 69659 | 69687 => {
                    return (frame_bytes - 4 as ::core::ffi::c_int * ch)
                        * 2 as ::core::ffi::c_int / ch;
                }
                69637 => {
                    return (frame_bytes - 4 as ::core::ffi::c_int)
                        * 2 as ::core::ffi::c_int / ch;
                }
                69651 => {
                    return (frame_bytes - 8 as ::core::ffi::c_int)
                        * 2 as ::core::ffi::c_int;
                }
                69650 | 69668 => {
                    if !extradata.is_null() {
                        return (frame_bytes as ::core::ffi::c_longlong
                            * 14 as ::core::ffi::c_longlong
                            / (8 as ::core::ffi::c_int * ch) as ::core::ffi::c_longlong)
                            as ::core::ffi::c_int;
                    }
                }
                69640 => {
                    return frame_bytes / 128 as ::core::ffi::c_int
                        * 224 as ::core::ffi::c_int / ch;
                }
                81921 => return (frame_bytes - 6 as ::core::ffi::c_int - ch) / ch,
                81920 => return (frame_bytes - 8 as ::core::ffi::c_int) / ch,
                81922 => return (frame_bytes - 2 as ::core::ffi::c_int * ch) / ch,
                86025 => return 3 as ::core::ffi::c_int * frame_bytes / ch,
                86026 => return 6 as ::core::ffi::c_int * frame_bytes / ch,
                65561 => {
                    return 2 as ::core::ffi::c_int
                        * (frame_bytes / (5 as ::core::ffi::c_int * ch));
                }
                86074 | 86043 => return 4 as ::core::ffi::c_int * frame_bytes / ch,
                _ => {}
            }
            if tag != 0 {
                if id as ::core::ffi::c_uint
                    == AV_CODEC_ID_SOL_DPCM as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    if tag == 3 as uint32_t {
                        return frame_bytes / ch
                    } else {
                        return frame_bytes * 2 as ::core::ffi::c_int / ch
                    }
                }
            }
            if ba > 0 as ::core::ffi::c_int {
                let mut blocks: ::core::ffi::c_int = frame_bytes / ba;
                let mut tmp: int64_t = 0 as int64_t;
                match id as ::core::ffi::c_uint {
                    69684 => {
                        if bps != 4 as ::core::ffi::c_int {
                            return 0 as ::core::ffi::c_int;
                        }
                        tmp = (blocks
                            * ((ba - 4 as ::core::ffi::c_int * ch) / (bps * ch)
                                * 8 as ::core::ffi::c_int)) as int64_t;
                    }
                    69633 => {
                        if bps < 2 as ::core::ffi::c_int || bps > 5 as ::core::ffi::c_int
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        tmp = (blocks as ::core::ffi::c_longlong
                            * (1 as ::core::ffi::c_longlong
                                + ((ba - 4 as ::core::ffi::c_int * ch) / (bps * ch))
                                    as ::core::ffi::c_longlong * 8 as ::core::ffi::c_longlong))
                            as int64_t;
                    }
                    69634 => {
                        tmp = (blocks as ::core::ffi::c_longlong
                            * ((ba as ::core::ffi::c_longlong
                                - 16 as ::core::ffi::c_longlong)
                                * 2 as ::core::ffi::c_longlong
                                / 3 as ::core::ffi::c_longlong
                                * 4 as ::core::ffi::c_longlong
                                / ch as ::core::ffi::c_longlong)) as int64_t;
                    }
                    69635 => {
                        tmp = (blocks as ::core::ffi::c_longlong
                            * (1 as ::core::ffi::c_longlong
                                + (ba as ::core::ffi::c_longlong
                                    - 4 as ::core::ffi::c_longlong
                                        * ch as ::core::ffi::c_longlong)
                                    * 2 as ::core::ffi::c_longlong
                                    / ch as ::core::ffi::c_longlong)) as int64_t;
                    }
                    69666 => {
                        tmp = (blocks as ::core::ffi::c_longlong
                            * ((ba as ::core::ffi::c_longlong
                                - 4 as ::core::ffi::c_longlong
                                    * ch as ::core::ffi::c_longlong)
                                * 2 as ::core::ffi::c_longlong
                                / ch as ::core::ffi::c_longlong)) as int64_t;
                    }
                    69638 => {
                        tmp = (blocks as ::core::ffi::c_longlong
                            * (2 as ::core::ffi::c_longlong
                                + (ba as ::core::ffi::c_longlong
                                    - 7 as ::core::ffi::c_longlong
                                        * ch as ::core::ffi::c_longlong)
                                    * 2 as ::core::ffi::c_longlong
                                    / ch as ::core::ffi::c_longlong)) as int64_t;
                    }
                    69672 => {
                        tmp = (blocks as ::core::ffi::c_longlong
                            * (ba as ::core::ffi::c_longlong
                                - 16 as ::core::ffi::c_longlong)
                            * 2 as ::core::ffi::c_longlong
                            / ch as ::core::ffi::c_longlong) as int64_t;
                    }
                    69683 => {
                        tmp = (blocks * 32 as ::core::ffi::c_int) as int64_t;
                    }
                    _ => {}
                }
                if tmp != 0 {
                    if tmp != tmp as ::core::ffi::c_int as int64_t {
                        return 0 as ::core::ffi::c_int;
                    }
                    return tmp as ::core::ffi::c_int;
                }
            }
            if bps > 0 as ::core::ffi::c_int {
                match id as ::core::ffi::c_uint {
                    65555 => {
                        if bps < 4 as ::core::ffi::c_int
                            || frame_bytes < 3 as ::core::ffi::c_int
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        return 2 as ::core::ffi::c_int
                            * ((frame_bytes - 3 as ::core::ffi::c_int)
                                / (bps * 2 as ::core::ffi::c_int / 8 as ::core::ffi::c_int
                                    * ch));
                    }
                    65560 => {
                        if bps < 4 as ::core::ffi::c_int
                            || frame_bytes < 4 as ::core::ffi::c_int
                        {
                            return 0 as ::core::ffi::c_int;
                        }
                        return (frame_bytes - 4 as ::core::ffi::c_int)
                            / ((ch + 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
                                & !(2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                                * bps / 8 as ::core::ffi::c_int);
                    }
                    65562 => {
                        return 2 as ::core::ffi::c_int
                            * (frame_bytes
                                / ((bps + 4 as ::core::ffi::c_int)
                                    / 4 as ::core::ffi::c_int)) / ch;
                    }
                    _ => {}
                }
            }
        }
    }
    if frame_size > 1 as ::core::ffi::c_int && frame_bytes != 0 {
        return frame_size;
    }
    if bitrate > 0 as int64_t && frame_bytes > 0 as ::core::ffi::c_int
        && sr > 0 as ::core::ffi::c_int && ba > 1 as ::core::ffi::c_int
    {
        if id as ::core::ffi::c_uint
            == AV_CODEC_ID_WMAV1 as ::core::ffi::c_int as ::core::ffi::c_uint
            || id as ::core::ffi::c_uint
                == AV_CODEC_ID_WMAV2 as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            return (frame_bytes as int64_t * 8 as int64_t * sr as int64_t / bitrate)
                as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_get_audio_frame_duration(
    mut avctx: *mut AVCodecContext,
    mut frame_bytes: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut channels: ::core::ffi::c_int = (*avctx).ch_layout.nb_channels;
    let mut duration: ::core::ffi::c_int = 0;
    duration = get_audio_frame_duration(
        (*avctx).codec_id,
        (*avctx).sample_rate,
        channels,
        (*avctx).block_align,
        (*avctx).codec_tag as uint32_t,
        (*avctx).bits_per_coded_sample,
        (*avctx).bit_rate,
        (*avctx).extradata,
        (*avctx).frame_size,
        frame_bytes,
    );
    return if 0 as ::core::ffi::c_int > duration {
        0 as ::core::ffi::c_int
    } else {
        duration
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_get_audio_frame_duration2(
    mut par: *mut AVCodecParameters,
    mut frame_bytes: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut channels: ::core::ffi::c_int = (*par).ch_layout.nb_channels;
    let mut duration: ::core::ffi::c_int = 0;
    duration = get_audio_frame_duration(
        (*par).codec_id,
        (*par).sample_rate,
        channels,
        (*par).block_align,
        (*par).codec_tag,
        (*par).bits_per_coded_sample,
        (*par).bit_rate,
        (*par).extradata,
        (*par).frame_size,
        frame_bytes,
    );
    return if 0 as ::core::ffi::c_int > duration {
        0 as ::core::ffi::c_int
    } else {
        duration
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_xiphlacing(
    mut s: *mut ::core::ffi::c_uchar,
    mut v: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut n: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while v >= 0xff as ::core::ffi::c_uint {
        let fresh0 = s;
        s = s.offset(1);
        *fresh0 = 0xff as ::core::ffi::c_uchar;
        v = v.wrapping_sub(0xff as ::core::ffi::c_uint);
        n = n.wrapping_add(1);
    }
    *s = v as ::core::ffi::c_uchar;
    n = n.wrapping_add(1);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn ff_match_2uint16(
    mut tab: *const [uint16_t; 2],
    mut size: ::core::ffi::c_int,
    mut a: ::core::ffi::c_int,
    mut b: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < size
        && !((*tab.offset(i as isize))[0 as ::core::ffi::c_int as usize]
            as ::core::ffi::c_int == a
            && (*tab.offset(i as isize))[1 as ::core::ffi::c_int as usize]
                as ::core::ffi::c_int == b)
    {
        i += 1;
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn avcodec_get_hw_config(
    mut avcodec: *const AVCodec,
    mut index: ::core::ffi::c_int,
) -> *const AVCodecHWConfig {
    let codec: *const FFCodec = ffcodec(avcodec) as *const FFCodec;
    let mut i: ::core::ffi::c_int = 0;
    if (*codec).hw_configs.is_null() || index < 0 as ::core::ffi::c_int {
        return ::core::ptr::null::<AVCodecHWConfig>();
    }
    i = 0 as ::core::ffi::c_int;
    while i <= index {
        if (*(*codec).hw_configs.offset(i as isize)).is_null() {
            return ::core::ptr::null::<AVCodecHWConfig>();
        }
        i += 1;
    }
    return &raw const (**(*codec).hw_configs.offset(index as isize)).public;
}
#[no_mangle]
pub unsafe extern "C" fn ff_thread_ref_frame(
    mut dst: *mut ThreadFrame,
    mut src: *const ThreadFrame,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    (*dst).owner[0 as ::core::ffi::c_int as usize] = (*src)
        .owner[0 as ::core::ffi::c_int as usize];
    (*dst).owner[1 as ::core::ffi::c_int as usize] = (*src)
        .owner[1 as ::core::ffi::c_int as usize];
    ret = av_frame_ref((*dst).f, (*src).f);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if !(*dst).progress.is_null() {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"!dst->progress\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/utils.c\0" as *const u8 as *const ::core::ffi::c_char,
            880 as ::core::ffi::c_int,
        );
        abort();
    }
    if !(*src).progress.is_null() {
        (*dst).progress = av_refstruct_ref((*src).progress as *mut ::core::ffi::c_void)
            as *mut ThreadFrameProgress;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_thread_replace_frame(
    mut dst: *mut ThreadFrame,
    mut src: *const ThreadFrame,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    (*dst).owner[0 as ::core::ffi::c_int as usize] = (*src)
        .owner[0 as ::core::ffi::c_int as usize];
    (*dst).owner[1 as ::core::ffi::c_int as usize] = (*src)
        .owner[1 as ::core::ffi::c_int as usize];
    ret = av_frame_replace((*dst).f, (*src).f);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    av_refstruct_replace(
        &raw mut (*dst).progress as *mut ::core::ffi::c_void,
        (*src).progress as *const ::core::ffi::c_void,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avpriv_find_start_code(
    mut p: *const uint8_t,
    mut end: *const uint8_t,
    mut state: *mut uint32_t,
) -> *const uint8_t {
    let mut i: ::core::ffi::c_int = 0;
    if !(p <= end) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"p <= end\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/utils.c\0" as *const u8 as *const ::core::ffi::c_char,
            948 as ::core::ffi::c_int,
        );
        abort();
    }
    if p >= end {
        return end;
    }
    i = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        let mut tmp: uint32_t = *state << 8 as ::core::ffi::c_int;
        let fresh2 = p;
        p = p.offset(1);
        *state = tmp.wrapping_add(*fresh2 as uint32_t);
        if tmp == 0x100 as uint32_t || p == end {
            return p;
        }
        i += 1;
    }
    while p < end {
        if *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            > 1 as ::core::ffi::c_int
        {
            p = p.offset(3 as ::core::ffi::c_int as isize);
        } else if *p.offset(-(2 as ::core::ffi::c_int) as isize) != 0 {
            p = p.offset(2 as ::core::ffi::c_int as isize);
        } else if *p.offset(-(3 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            | *p.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int != 0
        {
            p = p.offset(1);
        } else {
            p = p.offset(1);
            break;
        }
    }
    p = (if p > end { end } else { p }).offset(-(4 as ::core::ffi::c_int as isize));
    *state = av_bswap32((*(p as *const unaligned_32)).l);
    return p.offset(4 as ::core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn av_cpb_properties_alloc(
    mut size: *mut size_t,
) -> *mut AVCPBProperties {
    let mut props: *mut AVCPBProperties = av_mallocz(
        ::core::mem::size_of::<AVCPBProperties>() as size_t,
    ) as *mut AVCPBProperties;
    if props.is_null() {
        return ::core::ptr::null_mut::<AVCPBProperties>();
    }
    if !size.is_null() {
        *size = ::core::mem::size_of::<AVCPBProperties>() as usize as size_t;
    }
    (*props).vbv_delay = 0xffffffffffffffff as ::core::ffi::c_ulonglong as uint64_t;
    return props;
}
unsafe extern "C" fn put_timecode_fields(
    mut pb: *mut PutBitContext,
    mut rate: AVRational,
    mut tc: uint32_t,
) {
    let mut hours: ::core::ffi::c_uint = 0;
    let mut minutes: ::core::ffi::c_uint = 0;
    let mut seconds: ::core::ffi::c_uint = 0;
    let mut frames: ::core::ffi::c_uint = 0;
    let mut drop_0: ::core::ffi::c_uint = 0;
    ff_timecode_set_smpte(
        &raw mut drop_0,
        &raw mut hours,
        &raw mut minutes,
        &raw mut seconds,
        &raw mut frames,
        rate,
        tc,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    put_bits(pb, 5 as ::core::ffi::c_int, 0 as BitBuf);
    put_bits(pb, 1 as ::core::ffi::c_int, 1 as BitBuf);
    put_bits(pb, 1 as ::core::ffi::c_int, 0 as BitBuf);
    put_bits(pb, 1 as ::core::ffi::c_int, drop_0 as BitBuf);
    put_bits(pb, 9 as ::core::ffi::c_int, frames as BitBuf);
    put_bits(pb, 6 as ::core::ffi::c_int, seconds as BitBuf);
    put_bits(pb, 6 as ::core::ffi::c_int, minutes as BitBuf);
    put_bits(pb, 5 as ::core::ffi::c_int, hours as BitBuf);
}
#[no_mangle]
pub unsafe extern "C" fn ff_alloc_timecode_sei(
    mut frame: *const AVFrame,
    mut rate: AVRational,
    mut prefix_len: size_t,
    mut data: *mut *mut ::core::ffi::c_void,
    mut sei_size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut sd: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    let mut sei_data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut pb: PutBitContext = PutBitContext {
        bit_buf: 0,
        bit_left: 0,
        buf: ::core::ptr::null_mut::<uint8_t>(),
        buf_ptr: ::core::ptr::null_mut::<uint8_t>(),
        buf_end: ::core::ptr::null_mut::<uint8_t>(),
    };
    let mut tc: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    let mut m: ::core::ffi::c_int = 0;
    if !frame.is_null() {
        sd = av_frame_get_side_data(frame, AV_FRAME_DATA_S12M_TIMECODE);
    }
    if sd.is_null() {
        *data = ::core::ptr::null_mut::<::core::ffi::c_void>();
        return 0 as ::core::ffi::c_int;
    }
    tc = (*sd).data as *mut uint32_t;
    m = (*tc.offset(0 as ::core::ffi::c_int as isize) & 3 as uint32_t)
        as ::core::ffi::c_int;
    *sei_size = (::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(4 as usize)
        as size_t;
    *data = av_mallocz((*sei_size).wrapping_add(prefix_len));
    if (*data).is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    sei_data = (*data as *mut uint8_t).offset(prefix_len as isize);
    init_put_bits(&raw mut pb, sei_data, *sei_size as ::core::ffi::c_int);
    put_bits(&raw mut pb, 2 as ::core::ffi::c_int, m as BitBuf);
    let mut j: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while j <= m {
        put_bits(&raw mut pb, 1 as ::core::ffi::c_int, 1 as BitBuf);
        put_bits(&raw mut pb, 1 as ::core::ffi::c_int, 1 as BitBuf);
        put_timecode_fields(&raw mut pb, rate, *tc.offset(j as isize));
        put_bits(&raw mut pb, 5 as ::core::ffi::c_int, 0 as BitBuf);
        j += 1;
    }
    flush_put_bits(&raw mut pb);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_alloc_timecode_metadata_av1(
    mut frame: *const AVFrame,
    mut rate: AVRational,
    mut data: *mut *mut ::core::ffi::c_void,
    mut size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut sd: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    let mut pb: PutBitContext = PutBitContext {
        bit_buf: 0,
        bit_left: 0,
        buf: ::core::ptr::null_mut::<uint8_t>(),
        buf_ptr: ::core::ptr::null_mut::<uint8_t>(),
        buf_end: ::core::ptr::null_mut::<uint8_t>(),
    };
    let mut tc: *mut uint32_t = ::core::ptr::null_mut::<uint32_t>();
    if !frame.is_null() {
        sd = av_frame_get_side_data(frame, AV_FRAME_DATA_S12M_TIMECODE);
    }
    *data = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if sd.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    tc = (*sd).data as *mut uint32_t;
    if *tc.offset(0 as ::core::ffi::c_int as isize) & 3 as uint32_t == 0 {
        return 0 as ::core::ffi::c_int;
    }
    *size = 5 as size_t;
    *data = av_mallocz(*size);
    if (*data).is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    init_put_bits(&raw mut pb, *data as *mut uint8_t, *size as ::core::ffi::c_int);
    put_timecode_fields(&raw mut pb, rate, *tc.offset(1 as ::core::ffi::c_int as isize));
    put_bits(&raw mut pb, 5 as ::core::ffi::c_int, 1 as BitBuf);
    put_bits(&raw mut pb, 1 as ::core::ffi::c_int, 0 as BitBuf);
    flush_put_bits(&raw mut pb);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_guess_coded_bitrate(
    mut avctx: *mut AVCodecContext,
) -> int64_t {
    let mut framerate: AVRational = (*avctx).framerate;
    let mut bits_per_coded_sample: ::core::ffi::c_int = (*avctx).bits_per_coded_sample;
    let mut bitrate: int64_t = 0;
    if !(framerate.num != 0 && framerate.den != 0) {
        framerate = av_inv_q((*avctx).time_base);
    }
    if !(framerate.num != 0 && framerate.den != 0) {
        return 0 as int64_t;
    }
    if bits_per_coded_sample == 0 {
        let mut desc: *const AVPixFmtDescriptor = av_pix_fmt_desc_get((*avctx).pix_fmt);
        bits_per_coded_sample = av_get_bits_per_pixel(desc);
    }
    bitrate = bits_per_coded_sample as int64_t * (*avctx).width as int64_t
        * (*avctx).height as int64_t * framerate.num as int64_t
        / framerate.den as int64_t;
    return bitrate;
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
pub struct AVBSFContext {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVRefStructPool {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct FramePool {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct FFHWAccel {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct ThreadFrameProgress {
    _opaque: [u8; 0],
}
