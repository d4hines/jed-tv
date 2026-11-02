// Translated from FFmpeg 9.0.1 libavcodec/h2645_sei.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// Common H.264 and HEVC Supplementary Enhancement Information messages
//
// Copyright (c) 2003 Michael Niedermayer <michaelni@gmx.at>
// Copyright (C) 2012 - 2013 Guillaume Martres
// Copyright (C) 2012 - 2013 Gildas Cocherel
// Copyright (C) 2013 Vittorio Giovara
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
    fn strncmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: size_t,
    ) -> ::core::ffi::c_int;
    static ff_log2_tab: [uint8_t; 256];
    fn av_ambient_viewing_environment_alloc(
        size: *mut size_t,
    ) -> *mut AVAmbientViewingEnvironment;
    fn av_refstruct_alloc_ext_c(
        size: size_t,
        flags: ::core::ffi::c_uint,
        opaque: AVRefStructOpaque,
        free_cb: Option<
            unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
        >,
    ) -> *mut ::core::ffi::c_void;
    fn av_display_rotation_set(matrix: *mut int32_t, angle: ::core::ffi::c_double);
    fn ff_itut_t35_parse_buffer(
        itut_t35: *mut FFITUTT35,
        buf: *const uint8_t,
        size: size_t,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_buffer_alloc(size: size_t) -> *mut AVBufferRef;
    fn av_display_matrix_flip(
        matrix: *mut int32_t,
        hflip: ::core::ffi::c_int,
        vflip: ::core::ffi::c_int,
    );
    fn ff_itut_t35_parse_payload_to_struct(
        itut_t35: *mut FFITUTT35,
        aux: *mut FFITUTT35Aux,
        metadata: *mut FFITUTT35Meta,
        err_recognition: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_refstruct_unref(objp: *mut ::core::ffi::c_void);
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn av_buffer_create(
        data: *mut uint8_t,
        size: size_t,
        free: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>,
        opaque: *mut ::core::ffi::c_void,
        flags: ::core::ffi::c_int,
    ) -> *mut AVBufferRef;
    fn ff_itut_t35_unref(metadata: *mut FFITUTT35Meta);
    fn av_refstruct_replace(
        dstp: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
    );
    fn av_buffer_ref(buf: *const AVBufferRef) -> *mut AVBufferRef;
    fn av_buffer_unref(buf: *mut *mut AVBufferRef);
    fn ff_frame_new_side_data_from_buf(
        avctx: *const AVCodecContext,
        frame: *mut AVFrame,
        type_0: AVFrameSideDataType,
        buf: *mut *mut AVBufferRef,
    ) -> ::core::ffi::c_int;
    fn ff_frame_new_side_data_from_buf_ext(
        avctx: *const AVCodecContext,
        sd: *mut *mut *mut AVFrameSideData,
        nb_sd: *mut ::core::ffi::c_int,
        type_0: AVFrameSideDataType,
        buf: *mut *mut AVBufferRef,
    ) -> ::core::ffi::c_int;
    fn ff_decode_mastering_display_new_ext(
        avctx: *const AVCodecContext,
        sd: *mut *mut *mut AVFrameSideData,
        nb_sd: *mut ::core::ffi::c_int,
        mdm: *mut *mut AVMasteringDisplayMetadata,
    ) -> ::core::ffi::c_int;
    fn av_buffer_replace(
        dst: *mut *mut AVBufferRef,
        src: *const AVBufferRef,
    ) -> ::core::ffi::c_int;
    fn ff_decode_content_light_new_ext(
        avctx: *const AVCodecContext,
        sd: *mut *mut *mut AVFrameSideData,
        nb_sd: *mut ::core::ffi::c_int,
        clm: *mut *mut AVContentLightMetadata,
    ) -> ::core::ffi::c_int;
    fn av_realloc_array(
        ptr: *mut ::core::ffi::c_void,
        nmemb: size_t,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_reallocp_array(
        ptr: *mut ::core::ffi::c_void,
        nmemb: size_t,
        size: size_t,
    ) -> ::core::ffi::c_int;
    fn av_stereo3d_create_side_data(frame: *mut AVFrame) -> *mut AVStereo3D;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_film_grain_params_create_side_data(
        frame: *mut AVFrame,
    ) -> *mut AVFilmGrainParams;
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_frame_new_side_data(
        frame: *mut AVFrame,
        type_0: AVFrameSideDataType,
        size: size_t,
    ) -> *mut AVFrameSideData;
    fn av_frame_new_side_data_from_buf(
        frame: *mut AVFrame,
        type_0: AVFrameSideDataType,
        buf: *mut AVBufferRef,
    ) -> *mut AVFrameSideData;
    fn av_frame_side_data_add(
        sd: *mut *mut *mut AVFrameSideData,
        nb_sd: *mut ::core::ffi::c_int,
        type_0: AVFrameSideDataType,
        buf: *mut *mut AVBufferRef,
        flags: ::core::ffi::c_uint,
    ) -> *mut AVFrameSideData;
}
pub type size_t = ::core::ffi::c_uint;
pub type int8_t = ::core::ffi::c_schar;
pub type int16_t = ::core::ffi::c_short;
pub type int32_t = ::core::ffi::c_int;
pub type int64_t = ::core::ffi::c_longlong;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulonglong;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVAmbientViewingEnvironment {
    pub ambient_illuminance: AVRational,
    pub ambient_light_x: AVRational,
    pub ambient_light_y: AVRational,
}
pub type AVFilmGrainParamsType = ::core::ffi::c_uint;
pub const AV_FILM_GRAIN_PARAMS_H274: AVFilmGrainParamsType = 2;
pub const AV_FILM_GRAIN_PARAMS_AV1: AVFilmGrainParamsType = 1;
pub const AV_FILM_GRAIN_PARAMS_NONE: AVFilmGrainParamsType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilmGrainAOMParams {
    pub num_y_points: ::core::ffi::c_int,
    pub y_points: [[uint8_t; 2]; 14],
    pub chroma_scaling_from_luma: ::core::ffi::c_int,
    pub num_uv_points: [::core::ffi::c_int; 2],
    pub uv_points: [[[uint8_t; 2]; 10]; 2],
    pub scaling_shift: ::core::ffi::c_int,
    pub ar_coeff_lag: ::core::ffi::c_int,
    pub ar_coeffs_y: [int8_t; 24],
    pub ar_coeffs_uv: [[int8_t; 25]; 2],
    pub ar_coeff_shift: ::core::ffi::c_int,
    pub grain_scale_shift: ::core::ffi::c_int,
    pub uv_mult: [::core::ffi::c_int; 2],
    pub uv_mult_luma: [::core::ffi::c_int; 2],
    pub uv_offset: [::core::ffi::c_int; 2],
    pub overlap_flag: ::core::ffi::c_int,
    pub limit_output_range: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilmGrainH274Params {
    pub model_id: ::core::ffi::c_int,
    pub blending_mode_id: ::core::ffi::c_int,
    pub log2_scale_factor: ::core::ffi::c_int,
    pub component_model_present: [::core::ffi::c_int; 3],
    pub num_intensity_intervals: [uint16_t; 3],
    pub num_model_values: [uint8_t; 3],
    pub intensity_interval_lower_bound: [[uint8_t; 256]; 3],
    pub intensity_interval_upper_bound: [[uint8_t; 256]; 3],
    pub comp_model_value: [[[int16_t; 6]; 256]; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVFilmGrainParams {
    pub type_0: AVFilmGrainParamsType,
    pub seed: uint64_t,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub subsampling_x: ::core::ffi::c_int,
    pub subsampling_y: ::core::ffi::c_int,
    pub color_range: AVColorRange,
    pub color_primaries: AVColorPrimaries,
    pub color_trc: AVColorTransferCharacteristic,
    pub color_space: AVColorSpace,
    pub bit_depth_luma: ::core::ffi::c_int,
    pub bit_depth_chroma: ::core::ffi::c_int,
    pub codec: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub aom: AVFilmGrainAOMParams,
    pub h274: AVFilmGrainH274Params,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVMasteringDisplayMetadata {
    pub display_primaries: [[AVRational; 2]; 3],
    pub white_point: [AVRational; 2],
    pub min_luminance: AVRational,
    pub max_luminance: AVRational,
    pub has_primaries: ::core::ffi::c_int,
    pub has_luminance: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVContentLightMetadata {
    pub MaxCLL: ::core::ffi::c_uint,
    pub MaxFALL: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union AVRefStructOpaque {
    pub nc: *mut ::core::ffi::c_void,
    pub c: *const ::core::ffi::c_void,
}
pub type AVStereo3DType = ::core::ffi::c_uint;
pub const AV_STEREO3D_UNSPEC: AVStereo3DType = 8;
pub const AV_STEREO3D_COLUMNS: AVStereo3DType = 7;
pub const AV_STEREO3D_LINES: AVStereo3DType = 6;
pub const AV_STEREO3D_SIDEBYSIDE_QUINCUNX: AVStereo3DType = 5;
pub const AV_STEREO3D_CHECKERBOARD: AVStereo3DType = 4;
pub const AV_STEREO3D_FRAMESEQUENCE: AVStereo3DType = 3;
pub const AV_STEREO3D_TOPBOTTOM: AVStereo3DType = 2;
pub const AV_STEREO3D_SIDEBYSIDE: AVStereo3DType = 1;
pub const AV_STEREO3D_2D: AVStereo3DType = 0;
pub type AVStereo3DView = ::core::ffi::c_uint;
pub const AV_STEREO3D_VIEW_UNSPEC: AVStereo3DView = 3;
pub const AV_STEREO3D_VIEW_RIGHT: AVStereo3DView = 2;
pub const AV_STEREO3D_VIEW_LEFT: AVStereo3DView = 1;
pub const AV_STEREO3D_VIEW_PACKED: AVStereo3DView = 0;
pub type AVStereo3DPrimaryEye = ::core::ffi::c_uint;
pub const AV_PRIMARY_EYE_RIGHT: AVStereo3DPrimaryEye = 2;
pub const AV_PRIMARY_EYE_LEFT: AVStereo3DPrimaryEye = 1;
pub const AV_PRIMARY_EYE_NONE: AVStereo3DPrimaryEye = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVStereo3D {
    pub type_0: AVStereo3DType,
    pub flags: ::core::ffi::c_int,
    pub view: AVStereo3DView,
    pub primary_eye: AVStereo3DPrimaryEye,
    pub baseline: uint32_t,
    pub horizontal_disparity_adjustment: AVRational,
    pub horizontal_field_of_view: AVRational,
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
#[repr(C, packed)]
pub union unaligned_32 {
    pub l: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_16 {
    pub l: uint16_t,
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
pub struct GetByteContext {
    pub buffer: *const uint8_t,
    pub buffer_end: *const uint8_t,
    pub buffer_start: *const uint8_t,
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
pub struct AVFilmGrainAFGS1Params {
    pub enable: ::core::ffi::c_int,
    pub sets: [*mut AVBufferRef; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCIExy {
    pub x: AVRational,
    pub y: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVPrimaryCoefficients {
    pub r: AVCIExy,
    pub g: AVCIExy,
    pub b: AVCIExy,
}
pub type AVWhitepointCoefficients = AVCIExy;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVColorPrimariesDesc {
    pub wp: AVWhitepointCoefficients,
    pub prim: AVPrimaryCoefficients,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDecoderConfigurationRecord {
    pub dv_version_major: uint8_t,
    pub dv_version_minor: uint8_t,
    pub dv_profile: uint8_t,
    pub dv_level: uint8_t,
    pub rpu_present_flag: uint8_t,
    pub el_present_flag: uint8_t,
    pub bl_present_flag: uint8_t,
    pub dv_bl_signal_compatibility_id: uint8_t,
    pub dv_md_compression: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIRpuDataHeader {
    pub rpu_type: uint8_t,
    pub rpu_format: uint16_t,
    pub vdr_rpu_profile: uint8_t,
    pub vdr_rpu_level: uint8_t,
    pub chroma_resampling_explicit_filter_flag: uint8_t,
    pub coef_data_type: uint8_t,
    pub coef_log2_denom: uint8_t,
    pub vdr_rpu_normalized_idc: uint8_t,
    pub bl_video_full_range_flag: uint8_t,
    pub bl_bit_depth: uint8_t,
    pub el_bit_depth: uint8_t,
    pub vdr_bit_depth: uint8_t,
    pub spatial_resampling_filter_flag: uint8_t,
    pub el_spatial_resampling_filter_flag: uint8_t,
    pub disable_residual_flag: uint8_t,
    pub ext_mapping_idc_0_4: uint8_t,
    pub ext_mapping_idc_5_7: uint8_t,
}
pub type AVDOVIMappingMethod = ::core::ffi::c_uint;
pub const AV_DOVI_MAPPING_MMR: AVDOVIMappingMethod = 1;
pub const AV_DOVI_MAPPING_POLYNOMIAL: AVDOVIMappingMethod = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIReshapingCurve {
    pub num_pivots: uint8_t,
    pub pivots: [uint16_t; 9],
    pub mapping_idc: [AVDOVIMappingMethod; 8],
    pub poly_order: [uint8_t; 8],
    pub poly_coef: [[int64_t; 3]; 8],
    pub mmr_order: [uint8_t; 8],
    pub mmr_constant: [int64_t; 8],
    pub mmr_coef: [[[int64_t; 7]; 3]; 8],
}
pub type AVDOVINLQMethod = ::core::ffi::c_int;
pub const AV_DOVI_NLQ_LINEAR_DZ: AVDOVINLQMethod = 0;
pub const AV_DOVI_NLQ_NONE: AVDOVINLQMethod = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVINLQParams {
    pub nlq_offset: uint16_t,
    pub vdr_in_max: uint64_t,
    pub linear_deadzone_slope: uint64_t,
    pub linear_deadzone_threshold: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDataMapping {
    pub vdr_rpu_id: uint8_t,
    pub mapping_color_space: uint8_t,
    pub mapping_chroma_format_idc: uint8_t,
    pub curves: [AVDOVIReshapingCurve; 3],
    pub nlq_method_idc: AVDOVINLQMethod,
    pub num_x_partitions: uint32_t,
    pub num_y_partitions: uint32_t,
    pub nlq: [AVDOVINLQParams; 3],
    pub nlq_pivots: [uint16_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIColorMetadata {
    pub dm_metadata_id: uint8_t,
    pub scene_refresh_flag: uint8_t,
    pub ycc_to_rgb_matrix: [AVRational; 9],
    pub ycc_to_rgb_offset: [AVRational; 3],
    pub rgb_to_lms_matrix: [AVRational; 9],
    pub signal_eotf: uint16_t,
    pub signal_eotf_param0: uint16_t,
    pub signal_eotf_param1: uint16_t,
    pub signal_eotf_param2: uint32_t,
    pub signal_bit_depth: uint8_t,
    pub signal_color_space: uint8_t,
    pub signal_chroma_format: uint8_t,
    pub signal_full_range_flag: uint8_t,
    pub source_min_pq: uint16_t,
    pub source_max_pq: uint16_t,
    pub source_diagonal: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel1 {
    pub min_pq: uint16_t,
    pub max_pq: uint16_t,
    pub avg_pq: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel2 {
    pub target_max_pq: uint16_t,
    pub trim_slope: uint16_t,
    pub trim_offset: uint16_t,
    pub trim_power: uint16_t,
    pub trim_chroma_weight: uint16_t,
    pub trim_saturation_gain: uint16_t,
    pub ms_weight: int16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel3 {
    pub min_pq_offset: uint16_t,
    pub max_pq_offset: uint16_t,
    pub avg_pq_offset: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel4 {
    pub anchor_pq: uint16_t,
    pub anchor_power: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel5 {
    pub left_offset: uint16_t,
    pub right_offset: uint16_t,
    pub top_offset: uint16_t,
    pub bottom_offset: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel6 {
    pub max_luminance: uint16_t,
    pub min_luminance: uint16_t,
    pub max_cll: uint16_t,
    pub max_fall: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel8 {
    pub target_display_index: uint8_t,
    pub trim_slope: uint16_t,
    pub trim_offset: uint16_t,
    pub trim_power: uint16_t,
    pub trim_chroma_weight: uint16_t,
    pub trim_saturation_gain: uint16_t,
    pub ms_weight: uint16_t,
    pub target_mid_contrast: uint16_t,
    pub clip_trim: uint16_t,
    pub saturation_vector_field: [uint8_t; 6],
    pub hue_vector_field: [uint8_t; 6],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel9 {
    pub source_primary_index: uint8_t,
    pub source_display_primaries: AVColorPrimariesDesc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel10 {
    pub target_display_index: uint8_t,
    pub target_max_pq: uint16_t,
    pub target_min_pq: uint16_t,
    pub target_primary_index: uint8_t,
    pub target_display_primaries: AVColorPrimariesDesc,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel11 {
    pub content_type: uint8_t,
    pub whitepoint: uint8_t,
    pub reference_mode_flag: uint8_t,
    pub sharpness: uint8_t,
    pub noise_reduction: uint8_t,
    pub mpeg_noise_reduction: uint8_t,
    pub frame_rate_conversion: uint8_t,
    pub brightness: uint8_t,
    pub color: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel254 {
    pub dm_mode: uint8_t,
    pub dm_version_index: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmLevel255 {
    pub dm_run_mode: uint8_t,
    pub dm_run_version: uint8_t,
    pub dm_debug: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDOVIDmData {
    pub level: uint8_t,
    pub c2rust_unnamed: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub l1: AVDOVIDmLevel1,
    pub l2: AVDOVIDmLevel2,
    pub l3: AVDOVIDmLevel3,
    pub l4: AVDOVIDmLevel4,
    pub l5: AVDOVIDmLevel5,
    pub l6: AVDOVIDmLevel6,
    pub l8: AVDOVIDmLevel8,
    pub l9: AVDOVIDmLevel9,
    pub l10: AVDOVIDmLevel10,
    pub l11: AVDOVIDmLevel11,
    pub l254: AVDOVIDmLevel254,
    pub l255: AVDOVIDmLevel255,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOVIExt {
    pub dm_static: [AVDOVIDmData; 7],
    pub dm_dynamic: [AVDOVIDmData; 25],
    pub num_static: ::core::ffi::c_int,
    pub num_dynamic: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DOVIContext {
    pub logctx: *mut ::core::ffi::c_void,
    pub enable: ::core::ffi::c_int,
    pub cfg: AVDOVIDecoderConfigurationRecord,
    pub header: AVDOVIRpuDataHeader,
    pub mapping: *const AVDOVIDataMapping,
    pub color: *const AVDOVIColorMetadata,
    pub ext_blocks: *mut DOVIExt,
    pub dm: *mut AVDOVIColorMetadata,
    pub vdr: [*mut AVDOVIDataMapping; 16],
    pub rpu_buf: *mut uint8_t,
    pub rpu_buf_sz: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFITUTT35 {
    pub country_code: ::core::ffi::c_int,
    pub provider_code: ::core::ffi::c_int,
    pub provider_oriented_code: ::core::ffi::c_uint,
    pub payload: *const uint8_t,
    pub payload_size: size_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFITUTT35Aux {
    pub dovi: *mut DOVIContext,
}
pub type SEIType = ::core::ffi::c_uint;
pub const SEI_TYPE_SAMPLE_ASPECT_RATIO_INFO: SEIType = 204;
pub const SEI_TYPE_SUBPIC_LEVEL_INFO: SEIType = 203;
pub const SEI_TYPE_ANNOTATED_REGIONS: SEIType = 202;
pub const SEI_TYPE_SEI_PREFIX_INDICATION: SEIType = 201;
pub const SEI_TYPE_SEI_MANIFEST: SEIType = 200;
pub const SEI_TYPE_ALTERNATIVE_DEPTH_INFO: SEIType = 181;
pub const SEI_TYPE_MULTIVIEW_VIEW_POSITION_5: SEIType = 180;
pub const SEI_TYPE_MULTIVIEW_ACQUISITION_INFO_5: SEIType = 179;
pub const SEI_TYPE_MULTIVIEW_SCENE_INFO_5: SEIType = 178;
pub const SEI_TYPE_DEPTH_REPRESENTATION_INFO_5: SEIType = 177;
pub const SEI_TYPE_THREE_DIMENSIONAL_REFERENCE_DISPLAYS_INFO: SEIType = 176;
pub const SEI_TYPE_FRAME_FIELD_INFO: SEIType = 168;
pub const SEI_TYPE_TEMPORAL_MV_PREDICTION_CONSTRAINTS: SEIType = 167;
pub const SEI_TYPE_OVERLAY_INFO: SEIType = 166;
pub const SEI_TYPE_ALPHA_CHANNEL_INFO: SEIType = 165;
pub const SEI_TYPE_SUB_BITSTREAM_PROPERTY: SEIType = 164;
pub const SEI_TYPE_BSP_INITIAL_ARRIVAL_TIME: SEIType = 163;
pub const SEI_TYPE_BSP_NESTING: SEIType = 162;
pub const SEI_TYPE_INTER_LAYER_CONSTRAINED_TILE_SETS: SEIType = 161;
pub const SEI_TYPE_LAYERS_NOT_PRESENT_5: SEIType = 160;
pub const SEI_TYPE_MCTS_EXTRACTION_INFO_NESTING: SEIType = 159;
pub const SEI_TYPE_MCTS_EXTRACTION_INFO_SETS: SEIType = 158;
pub const SEI_TYPE_REGIONAL_NESTING: SEIType = 157;
pub const SEI_TYPE_OMNI_VIEWPORT: SEIType = 156;
pub const SEI_TYPE_REGIONWISE_PACKING: SEIType = 155;
pub const SEI_TYPE_SPHERE_ROTATION: SEIType = 154;
pub const SEI_TYPE_FISHEYE_VIDEO_INFO: SEIType = 152;
pub const SEI_TYPE_CUBEMAP_PROJECTION: SEIType = 151;
pub const SEI_TYPE_EQUIRECTANGULAR_PROJECTION: SEIType = 150;
pub const SEI_TYPE_CONTENT_COLOUR_VOLUME: SEIType = 149;
pub const SEI_TYPE_AMBIENT_VIEWING_ENVIRONMENT: SEIType = 148;
pub const SEI_TYPE_ALTERNATIVE_TRANSFER_CHARACTERISTICS: SEIType = 147;
pub const SEI_TYPE_CODED_REGION_COMPLETION: SEIType = 146;
pub const SEI_TYPE_DEPENDENT_RAP_INDICATION: SEIType = 145;
pub const SEI_TYPE_CONTENT_LIGHT_LEVEL_INFO: SEIType = 144;
pub const SEI_TYPE_DEINTERLACED_FIELD_IDENTIFICATION: SEIType = 143;
pub const SEI_TYPE_COLOUR_REMAPPING_INFO: SEIType = 142;
pub const SEI_TYPE_KNEE_FUNCTION_INFO: SEIType = 141;
pub const SEI_TYPE_CHROMA_RESAMPLING_FILTER_HINT: SEIType = 140;
pub const SEI_TYPE_TEMPORAL_MOTION_CONSTRAINED_TILE_SETS: SEIType = 139;
pub const SEI_TYPE_SEGMENTED_RECT_FRAME_PACKING_ARRANGEMENT: SEIType = 138;
pub const SEI_TYPE_MASTERING_DISPLAY_COLOUR_VOLUME: SEIType = 137;
pub const SEI_TYPE_TIME_CODE: SEIType = 136;
pub const SEI_TYPE_NO_DISPLAY: SEIType = 135;
pub const SEI_TYPE_REGION_REFRESH_INFO: SEIType = 134;
pub const SEI_TYPE_SCALABLE_NESTING_5: SEIType = 133;
pub const SEI_TYPE_DECODED_PICTURE_HASH: SEIType = 132;
pub const SEI_TYPE_TEMPORAL_SUB_LAYER_ZERO_IDX: SEIType = 131;
pub const SEI_TYPE_DECODING_UNIT_INFO: SEIType = 130;
pub const SEI_TYPE_PARAMETER_SETS_INCLUSION_INDICATION: SEIType = 129;
pub const SEI_TYPE_ACTIVE_PARAMETER_SETS: SEIType = 129;
pub const SEI_TYPE_STRUCTURE_OF_PICTURES_INFO: SEIType = 128;
pub const SEI_TYPE_GREEN_METADATA: SEIType = 56;
pub const SEI_TYPE_CONSTRAINED_DEPTH_PARAMETER_SET_IDENTIFIER: SEIType = 54;
pub const SEI_TYPE_DEPTH_SAMPLING_INFO: SEIType = 53;
pub const SEI_TYPE_DEPTH_TIMING: SEIType = 52;
pub const SEI_TYPE_THREE_DIMENSIONAL_REFERENCE_DISPLAYS_INFO_4: SEIType = 51;
pub const SEI_TYPE_DEPTH_REPRESENTATION_INFO_4: SEIType = 50;
pub const SEI_TYPE_MVCD_VIEW_SCALABILITY_INFO: SEIType = 49;
pub const SEI_TYPE_MVCD_SCALABLE_NESTING: SEIType = 48;
pub const SEI_TYPE_DISPLAY_ORIENTATION: SEIType = 47;
pub const SEI_TYPE_MULTIVIEW_VIEW_POSITION_4: SEIType = 46;
pub const SEI_TYPE_FRAME_PACKING_ARRANGEMENT: SEIType = 45;
pub const SEI_TYPE_BASE_VIEW_TEMPORAL_HRD: SEIType = 44;
pub const SEI_TYPE_OPERATION_POINTS_NOT_PRESENT: SEIType = 43;
pub const SEI_TYPE_VIEW_DEPENDENCY_CHANGE: SEIType = 42;
pub const SEI_TYPE_NON_REQUIRED_VIEW_COMPONENT: SEIType = 41;
pub const SEI_TYPE_MULTIVIEW_ACQUISITION_INFO_4: SEIType = 40;
pub const SEI_TYPE_MULTIVIEW_SCENE_INFO_4: SEIType = 39;
pub const SEI_TYPE_VIEW_SCALABILITY_INFO: SEIType = 38;
pub const SEI_TYPE_MVC_SCALABLE_NESTING: SEIType = 37;
pub const SEI_TYPE_PARALLEL_DECODING_INFO: SEIType = 36;
pub const SEI_TYPE_TL_SWITCHING_POINT: SEIType = 35;
pub const SEI_TYPE_TL0_DEP_REP_INDEX: SEIType = 34;
pub const SEI_TYPE_REDUNDANT_PIC_PROPERTY: SEIType = 33;
pub const SEI_TYPE_QUALITY_LAYER_INTEGRITY_CHECK: SEIType = 32;
pub const SEI_TYPE_BASE_LAYER_TEMPORAL_HRD: SEIType = 31;
pub const SEI_TYPE_SCALABLE_NESTING_4: SEIType = 30;
pub const SEI_TYPE_LAYER_DEPENDENCY_CHANGE: SEIType = 29;
pub const SEI_TYPE_LAYERS_NOT_PRESENT_4: SEIType = 28;
pub const SEI_TYPE_PRIORITY_LAYER_INFO: SEIType = 27;
pub const SEI_TYPE_NON_REQUIRED_LAYER_REP: SEIType = 26;
pub const SEI_TYPE_SUB_PIC_SCALABLE_LAYER: SEIType = 25;
pub const SEI_TYPE_SCALABILITY_INFO: SEIType = 24;
pub const SEI_TYPE_TONE_MAPPING_INFO: SEIType = 23;
pub const SEI_TYPE_POST_FILTER_HINT: SEIType = 22;
pub const SEI_TYPE_STEREO_VIDEO_INFO: SEIType = 21;
pub const SEI_TYPE_DEBLOCKING_FILTER_DISPLAY_PREFERENCE: SEIType = 20;
pub const SEI_TYPE_FILM_GRAIN_CHARACTERISTICS: SEIType = 19;
pub const SEI_TYPE_MOTION_CONSTRAINED_SLICE_GROUP_SET: SEIType = 18;
pub const SEI_TYPE_PROGRESSIVE_REFINEMENT_SEGMENT_END: SEIType = 17;
pub const SEI_TYPE_PROGRESSIVE_REFINEMENT_SEGMENT_START: SEIType = 16;
pub const SEI_TYPE_FULL_FRAME_SNAPSHOT: SEIType = 15;
pub const SEI_TYPE_FULL_FRAME_FREEZE_RELEASE: SEIType = 14;
pub const SEI_TYPE_FULL_FRAME_FREEZE: SEIType = 13;
pub const SEI_TYPE_SUB_SEQ_CHARACTERISTICS: SEIType = 12;
pub const SEI_TYPE_SUB_SEQ_LAYER_CHARACTERISTICS: SEIType = 11;
pub const SEI_TYPE_SUB_SEQ_INFO: SEIType = 10;
pub const SEI_TYPE_SCENE_INFO: SEIType = 9;
pub const SEI_TYPE_SPARE_PIC: SEIType = 8;
pub const SEI_TYPE_DEC_REF_PIC_MARKING_REPETITION: SEIType = 7;
pub const SEI_TYPE_RECOVERY_POINT: SEIType = 6;
pub const SEI_TYPE_USER_DATA_UNREGISTERED: SEIType = 5;
pub const SEI_TYPE_USER_DATA_REGISTERED_ITU_T_T35: SEIType = 4;
pub const SEI_TYPE_FILLER_PAYLOAD: SEIType = 3;
pub const SEI_TYPE_PAN_SCAN_RECT: SEIType = 2;
pub const SEI_TYPE_PIC_TIMING: SEIType = 1;
pub const SEI_TYPE_BUFFERING_PERIOD: SEIType = 0;
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
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
pub const FF_H2645_SEI_MESSAGE_UNHANDLED: C2RustUnnamed_2 = 1;
pub const FF_H2645_SEI_MESSAGE_HANDLED: C2RustUnnamed_2 = 0;
unsafe extern "C" fn decode_registered_user_data(
    mut h: *mut H2645SEI,
    mut gb: *mut GetByteContext,
    mut codec_id: AVCodecID,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut itut_t35: FFITUTT35 = FFITUTT35 {
        country_code: 0 as ::core::ffi::c_int,
        provider_code: 0,
        provider_oriented_code: 0,
        payload: ::core::ptr::null::<uint8_t>(),
        payload_size: 0,
    };
    let mut aux: FFITUTT35Aux = FFITUTT35Aux {
        dovi: ::core::ptr::null_mut::<DOVIContext>(),
    };
    let mut ret: ::core::ffi::c_int = 0;
    ret = ff_itut_t35_parse_buffer(
        &raw mut itut_t35,
        (*gb).buffer,
        bytestream2_get_bytes_left(gb) as size_t,
        0 as ::core::ffi::c_int,
    );
    if ret == 0 {
        av_log(
            logctx,
            40 as ::core::ffi::c_int,
            b"Unsupported User Data Registered ITU-T T35 SEI message (country_code = %d, provider_code = %d)\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            itut_t35.country_code,
            itut_t35.provider_code,
        );
    }
    if ret <= 0 as ::core::ffi::c_int {
        return ret;
    }
    return ff_itut_t35_parse_payload_to_struct(
        &raw mut itut_t35,
        &raw mut aux,
        &raw mut (*h).itut_t35,
        0 as ::core::ffi::c_int,
    );
}
#[inline(always)]
unsafe extern "C" fn ff_log2_c(mut v: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if v & 0xffff0000 as ::core::ffi::c_uint != 0 {
        v >>= 16 as ::core::ffi::c_int;
        n += 16 as ::core::ffi::c_int;
    }
    if v & 0xff00 as ::core::ffi::c_uint != 0 {
        v >>= 8 as ::core::ffi::c_int;
        n += 8 as ::core::ffi::c_int;
    }
    n += ff_log2_tab[v as usize] as ::core::ffi::c_int;
    return n;
}
#[inline(always)]
unsafe extern "C" fn av_bswap16(mut x: uint16_t) -> uint16_t {
    x = (x as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
        | (x as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint16_t;
    return x;
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
unsafe extern "C" fn decode_unregistered_user_data(
    mut h: *mut H2645SEIUnregistered,
    mut gb: *mut GetByteContext,
    mut codec_id: AVCodecID,
) -> ::core::ffi::c_int {
    let mut user_data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut size: ::core::ffi::c_int = bytestream2_get_bytes_left(gb);
    let mut buf_ref: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut tmp: *mut *mut AVBufferRef = ::core::ptr::null_mut::<*mut AVBufferRef>();
    if size < 16 as ::core::ffi::c_int
        || size >= 2147483647 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    tmp = av_realloc_array(
        (*h).buf_ref as *mut ::core::ffi::c_void,
        ((*h).nb_buf_ref as size_t).wrapping_add(1 as size_t),
        ::core::mem::size_of::<*mut AVBufferRef>() as size_t,
    ) as *mut *mut AVBufferRef;
    if tmp.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*h).buf_ref = tmp;
    buf_ref = av_buffer_alloc((size + 1 as ::core::ffi::c_int) as size_t);
    if buf_ref.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    user_data = (*buf_ref).data;
    bytestream2_get_bufferu(gb, user_data, size as ::core::ffi::c_uint);
    *user_data.offset(size as isize) = 0 as uint8_t;
    (*buf_ref).size = size as size_t;
    let fresh0 = (*h).nb_buf_ref;
    (*h).nb_buf_ref = (*h).nb_buf_ref.wrapping_add(1);
    let ref mut fresh1 = *(*h).buf_ref.offset(fresh0 as isize);
    *fresh1 = buf_ref;
    if if 1 as ::core::ffi::c_int != 0
        && (0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0)
    {
        (codec_id as ::core::ffi::c_uint
            == AV_CODEC_ID_H264 as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    } != 0
    {
        let mut e: ::core::ffi::c_int = 0;
        let mut build: ::core::ffi::c_int = 0;
        e = sscanf(
            user_data.offset(16 as ::core::ffi::c_int as isize)
                as *const ::core::ffi::c_char,
            b"x264 - core %d\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut build,
        );
        if e == 1 as ::core::ffi::c_int && build > 0 as ::core::ffi::c_int {
            (*h).x264_build = build;
        }
        if e == 1 as ::core::ffi::c_int && build == 1 as ::core::ffi::c_int
            && strncmp(
                user_data.offset(16 as ::core::ffi::c_int as isize)
                    as *const ::core::ffi::c_char,
                b"x264 - core 0000\0" as *const u8 as *const ::core::ffi::c_char,
                16 as size_t,
            ) == 0
        {
            (*h).x264_build = 67 as ::core::ffi::c_int;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn av_make_q(
    mut num: ::core::ffi::c_int,
    mut den: ::core::ffi::c_int,
) -> AVRational {
    let mut r: AVRational = AVRational { num: num, den: den };
    return r;
}
#[inline]
unsafe extern "C" fn av_refstruct_alloc_ext(
    mut size: size_t,
    mut flags: ::core::ffi::c_uint,
    mut opaque: *mut ::core::ffi::c_void,
    mut free_cb: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
) -> *mut ::core::ffi::c_void {
    return av_refstruct_alloc_ext_c(
        size,
        flags,
        AVRefStructOpaque { nc: opaque },
        free_cb,
    );
}
#[inline(always)]
unsafe extern "C" fn bytestream_get_be32(
    mut b: *mut *const uint8_t,
) -> ::core::ffi::c_uint {
    *b = (*b).offset(4 as ::core::ffi::c_int as isize);
    return av_bswap32(
        (*((*b).offset(-(4 as ::core::ffi::c_int as isize)) as *const unaligned_32)).l,
    ) as ::core::ffi::c_uint;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_be32u(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    return bytestream_get_be32(&raw mut (*g).buffer);
}
#[inline(always)]
unsafe extern "C" fn bytestream_get_be16(
    mut b: *mut *const uint8_t,
) -> ::core::ffi::c_uint {
    *b = (*b).offset(2 as ::core::ffi::c_int as isize);
    return av_bswap16(
        (*((*b).offset(-(2 as ::core::ffi::c_int as isize)) as *const unaligned_16)).l,
    ) as ::core::ffi::c_uint;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_be16u(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    return bytestream_get_be16(&raw mut (*g).buffer);
}
#[inline(always)]
unsafe extern "C" fn bytestream_get_byte(
    mut b: *mut *const uint8_t,
) -> ::core::ffi::c_uint {
    *b = (*b).offset(1 as ::core::ffi::c_int as isize);
    return *(*b)
        .offset(-(1 as ::core::ffi::c_int as isize))
        .offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_uint;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_byteu(
    mut g: *mut GetByteContext,
) -> ::core::ffi::c_uint {
    return bytestream_get_byte(&raw mut (*g).buffer);
}
#[inline]
unsafe extern "C" fn get_ue_golomb_long(
    mut gb: *mut GetBitContext,
) -> ::core::ffi::c_uint {
    let mut buf: ::core::ffi::c_uint = 0;
    let mut log: ::core::ffi::c_uint = 0;
    buf = show_bits_long(gb, 32 as ::core::ffi::c_int);
    log = (31 as ::core::ffi::c_int - ff_log2_c(buf)) as ::core::ffi::c_uint;
    skip_bits_long(gb, log as ::core::ffi::c_int);
    return get_bits_long(
            gb,
            log.wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        )
        .wrapping_sub(1 as ::core::ffi::c_uint);
}
#[inline]
unsafe extern "C" fn av_refstruct_allocz(mut size: size_t) -> *mut ::core::ffi::c_void {
    return av_refstruct_alloc_ext(
        size,
        0 as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
        None,
    );
}
#[inline]
unsafe extern "C" fn av_q2d(mut a: AVRational) -> ::core::ffi::c_double {
    return a.num as ::core::ffi::c_double / a.den as ::core::ffi::c_double;
}
unsafe extern "C" fn decode_display_orientation(
    mut h: *mut H2645SEIDisplayOrientation,
    mut gb: *mut GetBitContext,
) -> ::core::ffi::c_int {
    (*h).present = (get_bits1(gb) == 0) as ::core::ffi::c_int;
    if (*h).present != 0 {
        (*h).hflip = get_bits1(gb) as ::core::ffi::c_int;
        (*h).vflip = get_bits1(gb) as ::core::ffi::c_int;
        (*h).anticlockwise_rotation = get_bits(gb, 16 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn decode_frame_packing_arrangement(
    mut h: *mut H2645SEIFramePacking,
    mut gb: *mut GetBitContext,
    mut codec_id: AVCodecID,
) -> ::core::ffi::c_int {
    (*h).arrangement_id = get_ue_golomb_long(gb) as ::core::ffi::c_int;
    (*h).arrangement_cancel_flag = get_bits1(gb) as ::core::ffi::c_int;
    (*h).present = ((*h).arrangement_cancel_flag == 0) as ::core::ffi::c_int;
    if (*h).present != 0 {
        (*h).arrangement_type = get_bits(gb, 7 as ::core::ffi::c_int) as SEIFpaType;
        (*h).quincunx_sampling_flag = get_bits1(gb) as ::core::ffi::c_int;
        (*h).content_interpretation_type = get_bits(gb, 6 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        skip_bits(gb, 3 as ::core::ffi::c_int);
        (*h).current_frame_is_frame0_flag = get_bits1(gb) as ::core::ffi::c_int;
        skip_bits(gb, 2 as ::core::ffi::c_int);
        if (*h).quincunx_sampling_flag == 0
            && (*h).arrangement_type as ::core::ffi::c_uint != 5 as ::core::ffi::c_uint
        {
            skip_bits(gb, 16 as ::core::ffi::c_int);
        }
        skip_bits(gb, 8 as ::core::ffi::c_int);
        if if 1 as ::core::ffi::c_int != 0
            && (0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0)
        {
            (codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_H264 as ::core::ffi::c_int as ::core::ffi::c_uint)
                as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        } != 0
        {
            (*h).arrangement_repetition_period = get_ue_golomb_long(gb)
                as ::core::ffi::c_int;
        } else {
            skip_bits1(gb);
        }
    }
    skip_bits1(gb);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn decode_alternative_transfer(
    mut s: *mut H2645SEIAlternativeTransfer,
    mut gb: *mut GetByteContext,
) -> ::core::ffi::c_int {
    if bytestream2_get_bytes_left(gb) < 1 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*s).present = 1 as ::core::ffi::c_int;
    (*s).preferred_transfer_characteristics = bytestream2_get_byteu(gb)
        as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_bytes_left(
    mut g: *const GetByteContext,
) -> ::core::ffi::c_int {
    return (*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_int;
}
unsafe extern "C" fn decode_ambient_viewing_environment(
    mut s: *mut H2645SEIAmbientViewingEnvironment,
    mut gb: *mut GetByteContext,
) -> ::core::ffi::c_int {
    static mut max_ambient_light_value: uint16_t = 50000 as uint16_t;
    if bytestream2_get_bytes_left(gb) < 8 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*s).ambient_illuminance = bytestream2_get_be32u(gb) as uint32_t;
    if (*s).ambient_illuminance == 0 {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*s).ambient_light_x = bytestream2_get_be16u(gb) as uint16_t;
    if (*s).ambient_light_x as ::core::ffi::c_int
        > max_ambient_light_value as ::core::ffi::c_int
    {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*s).ambient_light_y = bytestream2_get_be16u(gb) as uint16_t;
    if (*s).ambient_light_y as ::core::ffi::c_int
        > max_ambient_light_value as ::core::ffi::c_int
    {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*s).present = 1 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: ::core::ffi::c_int,
    mut amin: ::core::ffi::c_int,
    mut amax: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if a < amin { return amin } else if a > amax { return amax } else { return a };
}
unsafe extern "C" fn decode_film_grain_characteristics(
    mut h: *mut H2645SEIFilmGrainCharacteristics,
    mut codec_id: AVCodecID,
    mut gb: *mut GetBitContext,
) -> ::core::ffi::c_int {
    (*h).present = (get_bits1(gb) == 0) as ::core::ffi::c_int;
    if (*h).present != 0 {
        memset(
            h as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<H2645SEIFilmGrainCharacteristics>() as size_t,
        );
        (*h).model_id = get_bits(gb, 2 as ::core::ffi::c_int) as ::core::ffi::c_int;
        (*h).separate_colour_description_present_flag = get_bits1(gb)
            as ::core::ffi::c_int;
        if (*h).separate_colour_description_present_flag != 0 {
            (*h).bit_depth_luma = get_bits(gb, 3 as ::core::ffi::c_int)
                .wrapping_add(8 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            (*h).bit_depth_chroma = get_bits(gb, 3 as ::core::ffi::c_int)
                .wrapping_add(8 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            (*h).full_range = get_bits1(gb) as ::core::ffi::c_int;
            (*h).color_primaries = get_bits(gb, 8 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            (*h).transfer_characteristics = get_bits(gb, 8 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            (*h).matrix_coeffs = get_bits(gb, 8 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
        }
        (*h).blending_mode_id = get_bits(gb, 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        (*h).log2_scale_factor = get_bits(gb, 4 as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while c < 3 as ::core::ffi::c_int {
            (*h).comp_model_present_flag[c as usize] = get_bits1(gb)
                as ::core::ffi::c_int;
            c += 1;
        }
        let mut c_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while c_0 < 3 as ::core::ffi::c_int {
            if (*h).comp_model_present_flag[c_0 as usize] != 0 {
                (*h).num_intensity_intervals[c_0 as usize] = get_bits(
                        gb,
                        8 as ::core::ffi::c_int,
                    )
                    .wrapping_add(1 as ::core::ffi::c_uint) as uint16_t;
                (*h).num_model_values[c_0 as usize] = get_bits(
                        gb,
                        3 as ::core::ffi::c_int,
                    )
                    .wrapping_add(1 as ::core::ffi::c_uint) as uint8_t;
                if (*h).num_model_values[c_0 as usize] as ::core::ffi::c_int
                    > 6 as ::core::ffi::c_int
                {
                    return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                        | ('D' as i32) << 16 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                        | ('A' as i32 as ::core::ffi::c_uint)
                            << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                }
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i
                    < (*h).num_intensity_intervals[c_0 as usize] as ::core::ffi::c_int
                {
                    (*h).intensity_interval_lower_bound[c_0 as usize][i as usize] = get_bits(
                        gb,
                        8 as ::core::ffi::c_int,
                    ) as uint8_t;
                    (*h).intensity_interval_upper_bound[c_0 as usize][i as usize] = get_bits(
                        gb,
                        8 as ::core::ffi::c_int,
                    ) as uint8_t;
                    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while j < (*h).num_model_values[c_0 as usize] as ::core::ffi::c_int {
                        (*h).comp_model_value[c_0 as usize][i as usize][j as usize] = get_se_golomb_long(
                            gb,
                        ) as int16_t;
                        j += 1;
                    }
                    i += 1;
                }
            }
            c_0 += 1;
        }
        if if 1 as ::core::ffi::c_int != 0
            && (0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0)
        {
            (codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_H264 as ::core::ffi::c_int as ::core::ffi::c_uint)
                as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        } == 0
        {
            (*h).persistence_flag = get_bits1(gb) as ::core::ffi::c_int;
        } else {
            (*h).repetition_period = get_ue_golomb_long(gb) as ::core::ffi::c_int;
        }
        (*h).present = 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn decode_nal_sei_mastering_display_info(
    mut s: *mut H2645SEIMasteringDisplay,
    mut gb: *mut GetByteContext,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    if bytestream2_get_bytes_left(gb) < 24 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < 3 as ::core::ffi::c_int {
        (*s).display_primaries[i as usize][0 as ::core::ffi::c_int as usize] = bytestream2_get_be16u(
            gb,
        ) as uint16_t;
        (*s).display_primaries[i as usize][1 as ::core::ffi::c_int as usize] = bytestream2_get_be16u(
            gb,
        ) as uint16_t;
        i += 1;
    }
    (*s).white_point[0 as ::core::ffi::c_int as usize] = bytestream2_get_be16u(gb)
        as uint16_t;
    (*s).white_point[1 as ::core::ffi::c_int as usize] = bytestream2_get_be16u(gb)
        as uint16_t;
    (*s).max_luminance = bytestream2_get_be32u(gb) as uint32_t;
    (*s).min_luminance = bytestream2_get_be32u(gb) as uint32_t;
    (*s).present = 2 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn decode_nal_sei_content_light_info(
    mut s: *mut H2645SEIContentLight,
    mut gb: *mut GetByteContext,
) -> ::core::ffi::c_int {
    if bytestream2_get_bytes_left(gb) < 4 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    (*s).max_content_light_level = bytestream2_get_be16u(gb) as uint16_t;
    (*s).max_pic_average_light_level = bytestream2_get_be16u(gb) as uint16_t;
    (*s).present = 2 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_bufferu(
    mut g: *mut GetByteContext,
    mut dst: *mut uint8_t,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    memcpy(
        dst as *mut ::core::ffi::c_void,
        (*g).buffer as *const ::core::ffi::c_void,
        size as size_t,
    );
    (*g).buffer = (*g).buffer.offset(size as isize);
    return size;
}
#[inline]
unsafe extern "C" fn skip_bits_long(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) {
    (*s).index += av_clip_c(n, -(*s).index, (*s).size_in_bits_plus8 - (*s).index);
}
#[no_mangle]
pub unsafe extern "C" fn ff_h2645_sei_message_decode(
    mut h: *mut H2645SEI,
    mut type_0: SEIType,
    mut codec_id: AVCodecID,
    mut gb: *mut GetBitContext,
    mut gbyte: *mut GetByteContext,
    mut logctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    match type_0 as ::core::ffi::c_uint {
        4 => return decode_registered_user_data(h, gbyte, codec_id, logctx),
        5 => {
            return decode_unregistered_user_data(
                &raw mut (*h).unregistered,
                gbyte,
                codec_id,
            );
        }
        47 => return decode_display_orientation(&raw mut (*h).display_orientation, gb),
        19 => {
            av_refstruct_unref(
                &raw mut (*h).film_grain_characteristics as *mut ::core::ffi::c_void,
            );
            (*h).film_grain_characteristics = av_refstruct_allocz(
                ::core::mem::size_of::<H2645SEIFilmGrainCharacteristics>() as size_t,
            ) as *mut H2645SEIFilmGrainCharacteristics;
            if (*h).film_grain_characteristics.is_null() {
                return -(12 as ::core::ffi::c_int);
            }
            return decode_film_grain_characteristics(
                (*h).film_grain_characteristics,
                codec_id,
                gb,
            );
        }
        45 => {
            return decode_frame_packing_arrangement(
                &raw mut (*h).frame_packing,
                gb,
                codec_id,
            );
        }
        147 => {
            return decode_alternative_transfer(&raw mut (*h).alternative_transfer, gbyte);
        }
        148 => {
            return decode_ambient_viewing_environment(
                &raw mut (*h).ambient_viewing_environment,
                gbyte,
            );
        }
        137 => {
            return decode_nal_sei_mastering_display_info(
                &raw mut (*h).mastering_display,
                gbyte,
            );
        }
        144 => {
            return decode_nal_sei_content_light_info(&raw mut (*h).content_light, gbyte);
        }
        _ => return FF_H2645_SEI_MESSAGE_UNHANDLED as ::core::ffi::c_int,
    };
}
#[inline]
unsafe extern "C" fn get_se_golomb_long(
    mut gb: *mut GetBitContext,
) -> ::core::ffi::c_int {
    let mut buf: ::core::ffi::c_uint = get_ue_golomb_long(gb);
    let mut sign: ::core::ffi::c_int = (buf & 1 as ::core::ffi::c_uint)
        .wrapping_sub(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
    return (buf >> 1 as ::core::ffi::c_int ^ sign as ::core::ffi::c_uint)
        .wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_h2645_sei_ctx_replace(
    mut dst: *mut H2645SEI,
    mut src: *const H2645SEI,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = av_buffer_replace(
        &raw mut (*dst).itut_t35.a53_cc,
        (*src).itut_t35.a53_cc,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < (*dst).unregistered.nb_buf_ref {
        av_buffer_unref(
            (*dst).unregistered.buf_ref.offset(i as isize) as *mut *mut AVBufferRef,
        );
        i = i.wrapping_add(1);
    }
    (*dst).unregistered.nb_buf_ref = 0 as ::core::ffi::c_uint;
    ret = av_buffer_replace(&raw mut (*dst).itut_t35.lcevc, (*src).itut_t35.lcevc);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*src).unregistered.nb_buf_ref != 0 {
        ret = av_reallocp_array(
            &raw mut (*dst).unregistered.buf_ref as *mut ::core::ffi::c_void,
            (*src).unregistered.nb_buf_ref as size_t,
            ::core::mem::size_of::<*mut AVBufferRef>() as size_t,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        let mut i_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while i_0 < (*src).unregistered.nb_buf_ref {
            let ref mut fresh2 = *(*dst).unregistered.buf_ref.offset(i_0 as isize);
            *fresh2 = av_buffer_ref(*(*src).unregistered.buf_ref.offset(i_0 as isize));
            if (*(*dst).unregistered.buf_ref.offset(i_0 as isize)).is_null() {
                return -(12 as ::core::ffi::c_int);
            }
            (*dst).unregistered.nb_buf_ref = (*dst)
                .unregistered
                .nb_buf_ref
                .wrapping_add(1);
            i_0 = i_0.wrapping_add(1);
        }
    }
    let mut i_1: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while (i_1 as usize)
        < (::core::mem::size_of::<[*mut AVBufferRef; 8]>() as usize)
            .wrapping_div(::core::mem::size_of::<*mut AVBufferRef>() as usize)
    {
        ret = av_buffer_replace(
            (&raw mut (*dst).itut_t35.aom_film_grain.sets as *mut *mut AVBufferRef)
                .offset(i_1 as isize) as *mut *mut AVBufferRef,
            (*src).itut_t35.aom_film_grain.sets[i_1 as usize],
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        i_1 = i_1.wrapping_add(1);
    }
    (*dst).itut_t35.aom_film_grain.enable = (*src).itut_t35.aom_film_grain.enable;
    (*dst).ambient_viewing_environment = (*src).ambient_viewing_environment;
    (*dst).mastering_display = (*src).mastering_display;
    (*dst).content_light = (*src).content_light;
    av_refstruct_replace(
        &raw mut (*dst).film_grain_characteristics as *mut ::core::ffi::c_void,
        (*src).film_grain_characteristics as *const ::core::ffi::c_void,
    );
    return 0 as ::core::ffi::c_int;
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
unsafe extern "C" fn is_frame_packing_type_valid(
    mut type_0: SEIFpaType,
    mut codec_id: AVCodecID,
) -> ::core::ffi::c_int {
    if if 1 as ::core::ffi::c_int != 0
        && (0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0)
    {
        (codec_id as ::core::ffi::c_uint
            == AV_CODEC_ID_H264 as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    } != 0
    {
        return (type_0 as ::core::ffi::c_uint
            <= SEI_FPA_H264_TYPE_2D as ::core::ffi::c_int as ::core::ffi::c_uint
            && type_0 as ::core::ffi::c_uint
                >= SEI_FPA_H264_TYPE_CHECKERBOARD as ::core::ffi::c_int
                    as ::core::ffi::c_uint) as ::core::ffi::c_int
    } else {
        return (type_0 as ::core::ffi::c_uint
            <= SEI_FPA_TYPE_INTERLEAVE_TEMPORAL as ::core::ffi::c_int
                as ::core::ffi::c_uint
            && type_0 as ::core::ffi::c_uint
                >= SEI_FPA_TYPE_SIDE_BY_SIDE as ::core::ffi::c_int
                    as ::core::ffi::c_uint) as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn show_bits(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    let mut tmp: ::core::ffi::c_uint = 0;
    let mut re_index: ::core::ffi::c_uint = (*s).index as ::core::ffi::c_uint;
    let mut re_cache: ::core::ffi::c_uint = 0;
    re_cache = (av_bswap32(
        (*((*s).buffer.offset((re_index >> 3 as ::core::ffi::c_int) as isize)
            as *const unaligned_32))
            .l,
    ) << (re_index & 7 as ::core::ffi::c_uint)
        >> 32 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    tmp = (re_cache >> 32 as ::core::ffi::c_int - n) as ::core::ffi::c_uint;
    return tmp;
}
unsafe extern "C" fn h2645_sei_to_side_data(
    mut avctx: *mut AVCodecContext,
    mut sei: *mut H2645SEI,
    mut sd: *mut *mut *mut AVFrameSideData,
    mut nb_sd: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < (*sei).unregistered.nb_buf_ref {
        let mut unreg: *mut H2645SEIUnregistered = &raw mut (*sei).unregistered;
        if !(*(*unreg).buf_ref.offset(i as isize)).is_null() {
            let mut entry: *mut AVFrameSideData = av_frame_side_data_add(
                sd,
                nb_sd,
                AV_FRAME_DATA_SEI_UNREGISTERED,
                (*unreg).buf_ref.offset(i as isize) as *mut *mut AVBufferRef,
                0 as ::core::ffi::c_uint,
            );
            if entry.is_null() {
                av_buffer_unref(
                    (*unreg).buf_ref.offset(i as isize) as *mut *mut AVBufferRef,
                );
            }
        }
        i = i.wrapping_add(1);
    }
    (*sei).unregistered.nb_buf_ref = 0 as ::core::ffi::c_uint;
    if (*sei).ambient_viewing_environment.present != 0 {
        let mut env: *mut H2645SEIAmbientViewingEnvironment = &raw mut (*sei)
            .ambient_viewing_environment;
        let mut buf: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
        let mut size: size_t = 0;
        let mut dst_env: *mut AVAmbientViewingEnvironment = av_ambient_viewing_environment_alloc(
            &raw mut size,
        );
        if dst_env.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
        buf = av_buffer_create(
            dst_env as *mut uint8_t,
            size,
            None,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
        );
        if buf.is_null() {
            av_free(dst_env as *mut ::core::ffi::c_void);
            return -(12 as ::core::ffi::c_int);
        }
        (*dst_env).ambient_illuminance = av_make_q(
            (*env).ambient_illuminance as ::core::ffi::c_int,
            10000 as ::core::ffi::c_int,
        );
        (*dst_env).ambient_light_x = av_make_q(
            (*env).ambient_light_x as ::core::ffi::c_int,
            50000 as ::core::ffi::c_int,
        );
        (*dst_env).ambient_light_y = av_make_q(
            (*env).ambient_light_y as ::core::ffi::c_int,
            50000 as ::core::ffi::c_int,
        );
        ret = ff_frame_new_side_data_from_buf_ext(
            avctx,
            sd,
            nb_sd,
            AV_FRAME_DATA_AMBIENT_VIEWING_ENVIRONMENT,
            &raw mut buf,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    if (*sei).mastering_display.present != 0 {
        let mapping: [::core::ffi::c_int; 3] = [
            2 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
        ];
        let chroma_den: ::core::ffi::c_int = 50000 as ::core::ffi::c_int;
        let luma_den: ::core::ffi::c_int = 10000 as ::core::ffi::c_int;
        let mut i_0: ::core::ffi::c_int = 0;
        let mut metadata: *mut AVMasteringDisplayMetadata = ::core::ptr::null_mut::<
            AVMasteringDisplayMetadata,
        >();
        ret = ff_decode_mastering_display_new_ext(avctx, sd, nb_sd, &raw mut metadata);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        if !metadata.is_null() {
            (*metadata).has_luminance = 1 as ::core::ffi::c_int;
            (*metadata).has_primaries = 1 as ::core::ffi::c_int;
            i_0 = 0 as ::core::ffi::c_int;
            while i_0 < 3 as ::core::ffi::c_int {
                let j: ::core::ffi::c_int = mapping[i_0 as usize];
                (*metadata)
                    .display_primaries[i_0 as usize][0 as ::core::ffi::c_int as usize]
                    .num = (*sei)
                    .mastering_display
                    .display_primaries[j as usize][0 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                (*metadata)
                    .display_primaries[i_0 as usize][0 as ::core::ffi::c_int as usize]
                    .den = chroma_den;
                (*metadata).has_primaries
                    &= ((*sei)
                        .mastering_display
                        .display_primaries[j as usize][0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int >= 5 as ::core::ffi::c_int
                        && (*sei)
                            .mastering_display
                            .display_primaries[j
                            as usize][0 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int <= 37000 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                (*metadata)
                    .display_primaries[i_0 as usize][1 as ::core::ffi::c_int as usize]
                    .num = (*sei)
                    .mastering_display
                    .display_primaries[j as usize][1 as ::core::ffi::c_int as usize]
                    as ::core::ffi::c_int;
                (*metadata)
                    .display_primaries[i_0 as usize][1 as ::core::ffi::c_int as usize]
                    .den = chroma_den;
                (*metadata).has_primaries
                    &= ((*sei)
                        .mastering_display
                        .display_primaries[j as usize][1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int >= 5 as ::core::ffi::c_int
                        && (*sei)
                            .mastering_display
                            .display_primaries[j
                            as usize][1 as ::core::ffi::c_int as usize]
                            as ::core::ffi::c_int <= 42000 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                i_0 += 1;
            }
            (*metadata).white_point[0 as ::core::ffi::c_int as usize].num = (*sei)
                .mastering_display
                .white_point[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
            (*metadata).white_point[0 as ::core::ffi::c_int as usize].den = chroma_den;
            (*metadata).has_primaries
                &= ((*sei)
                    .mastering_display
                    .white_point[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    >= 5 as ::core::ffi::c_int
                    && (*sei)
                        .mastering_display
                        .white_point[0 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int <= 37000 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            (*metadata).white_point[1 as ::core::ffi::c_int as usize].num = (*sei)
                .mastering_display
                .white_point[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int;
            (*metadata).white_point[1 as ::core::ffi::c_int as usize].den = chroma_den;
            (*metadata).has_primaries
                &= ((*sei)
                    .mastering_display
                    .white_point[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    >= 5 as ::core::ffi::c_int
                    && (*sei)
                        .mastering_display
                        .white_point[1 as ::core::ffi::c_int as usize]
                        as ::core::ffi::c_int <= 42000 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
            (*metadata).max_luminance.num = (*sei).mastering_display.max_luminance
                as ::core::ffi::c_int;
            (*metadata).max_luminance.den = luma_den;
            (*metadata).has_luminance
                &= ((*sei).mastering_display.max_luminance >= 50000 as uint32_t
                    && (*sei).mastering_display.max_luminance
                        <= 100000000 as ::core::ffi::c_int as uint32_t)
                    as ::core::ffi::c_int;
            (*metadata).min_luminance.num = (*sei).mastering_display.min_luminance
                as ::core::ffi::c_int;
            (*metadata).min_luminance.den = luma_den;
            (*metadata).has_luminance
                &= ((*sei).mastering_display.min_luminance <= 50000 as uint32_t
                    && (*sei).mastering_display.min_luminance
                        < (*sei).mastering_display.max_luminance) as ::core::ffi::c_int;
            if (*avctx).strict_std_compliance >= 1 as ::core::ffi::c_int {
                (*metadata).has_luminance
                    &= ((*sei).mastering_display.min_luminance >= 1 as uint32_t)
                        as ::core::ffi::c_int;
            }
            if (*metadata).has_luminance != 0 || (*metadata).has_primaries != 0 {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    48 as ::core::ffi::c_int,
                    b"Mastering Display Metadata:\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
            }
            if (*metadata).has_primaries != 0 {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    48 as ::core::ffi::c_int,
                    b"r(%5.4f,%5.4f) g(%5.4f,%5.4f) b(%5.4f %5.4f) wp(%5.4f, %5.4f)\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    av_q2d(
                        (*metadata)
                            .display_primaries[0 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int as usize],
                    ),
                    av_q2d(
                        (*metadata)
                            .display_primaries[0 as ::core::ffi::c_int
                            as usize][1 as ::core::ffi::c_int as usize],
                    ),
                    av_q2d(
                        (*metadata)
                            .display_primaries[1 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int as usize],
                    ),
                    av_q2d(
                        (*metadata)
                            .display_primaries[1 as ::core::ffi::c_int
                            as usize][1 as ::core::ffi::c_int as usize],
                    ),
                    av_q2d(
                        (*metadata)
                            .display_primaries[2 as ::core::ffi::c_int
                            as usize][0 as ::core::ffi::c_int as usize],
                    ),
                    av_q2d(
                        (*metadata)
                            .display_primaries[2 as ::core::ffi::c_int
                            as usize][1 as ::core::ffi::c_int as usize],
                    ),
                    av_q2d((*metadata).white_point[0 as ::core::ffi::c_int as usize]),
                    av_q2d((*metadata).white_point[1 as ::core::ffi::c_int as usize]),
                );
            }
            if (*metadata).has_luminance != 0 {
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    48 as ::core::ffi::c_int,
                    b"min_luminance=%f, max_luminance=%f\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    av_q2d((*metadata).min_luminance),
                    av_q2d((*metadata).max_luminance),
                );
            }
        }
    }
    if (*sei).content_light.present != 0 {
        let mut metadata_0: *mut AVContentLightMetadata = ::core::ptr::null_mut::<
            AVContentLightMetadata,
        >();
        ret = ff_decode_content_light_new_ext(avctx, sd, nb_sd, &raw mut metadata_0);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        if !metadata_0.is_null() {
            (*metadata_0).MaxCLL = (*sei).content_light.max_content_light_level
                as ::core::ffi::c_uint;
            (*metadata_0).MaxFALL = (*sei).content_light.max_pic_average_light_level
                as ::core::ffi::c_uint;
            av_log(
                avctx as *mut ::core::ffi::c_void,
                48 as ::core::ffi::c_int,
                b"Content Light Level Metadata:\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            av_log(
                avctx as *mut ::core::ffi::c_void,
                48 as ::core::ffi::c_int,
                b"MaxCLL=%d, MaxFALL=%d\n\0" as *const u8 as *const ::core::ffi::c_char,
                (*metadata_0).MaxCLL,
                (*metadata_0).MaxFALL,
            );
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn skip_bits(mut s: *mut GetBitContext, mut n: ::core::ffi::c_int) {
    let mut re_index: ::core::ffi::c_uint = (*s).index as ::core::ffi::c_uint;
    let mut re_size_plus8: ::core::ffi::c_uint = (*s).size_in_bits_plus8
        as ::core::ffi::c_uint;
    re_index = if re_size_plus8 > re_index.wrapping_add(n as ::core::ffi::c_uint) {
        re_index.wrapping_add(n as ::core::ffi::c_uint)
    } else {
        re_size_plus8
    };
    (*s).index = re_index as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn get_bits1(mut s: *mut GetBitContext) -> ::core::ffi::c_uint {
    let mut index: ::core::ffi::c_uint = (*s).index as ::core::ffi::c_uint;
    let mut result: uint8_t = *(*s)
        .buffer
        .offset((index >> 3 as ::core::ffi::c_int) as isize);
    result = ((result as ::core::ffi::c_int) << (index & 7 as ::core::ffi::c_uint))
        as uint8_t;
    result = (result as ::core::ffi::c_int
        >> 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as uint8_t;
    if (*s).index < (*s).size_in_bits_plus8 {
        index = index.wrapping_add(1);
    }
    (*s).index = index as ::core::ffi::c_int;
    return result as ::core::ffi::c_uint;
}
#[inline]
unsafe extern "C" fn skip_bits1(mut s: *mut GetBitContext) {
    skip_bits(s, 1 as ::core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn get_bits_long(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    if n == 0 {
        return 0 as ::core::ffi::c_uint
    } else if (0 as ::core::ffi::c_int == 0 || 0 != 0) && n <= 25 as ::core::ffi::c_int {
        return get_bits(s, n)
    } else {
        let mut ret: ::core::ffi::c_uint = get_bits(s, 16 as ::core::ffi::c_int)
            << n - 16 as ::core::ffi::c_int;
        return ret | get_bits(s, n - 16 as ::core::ffi::c_int);
    };
}
#[inline]
unsafe extern "C" fn show_bits_long(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    if n <= 25 as ::core::ffi::c_int {
        return show_bits(s, n)
    } else {
        let mut gb: GetBitContext = *s;
        return get_bits_long(&raw mut gb, n);
    };
}
#[no_mangle]
pub unsafe extern "C" fn ff_h2645_sei_to_frame(
    mut frame: *mut AVFrame,
    mut sei: *mut H2645SEI,
    mut codec_id: AVCodecID,
    mut avctx: *mut AVCodecContext,
    mut vui: *const H2645VUI,
    mut bit_depth_luma: ::core::ffi::c_uint,
    mut bit_depth_chroma: ::core::ffi::c_uint,
    mut seed: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fp: *mut H2645SEIFramePacking = &raw mut (*sei).frame_packing;
    let mut itut_t35: *mut FFITUTT35Meta = &raw mut (*sei).itut_t35;
    let mut ret: ::core::ffi::c_int = 0;
    if (*fp).present != 0
        && is_frame_packing_type_valid((*fp).arrangement_type, codec_id) != 0
        && (*fp).content_interpretation_type > 0 as ::core::ffi::c_int
        && (*fp).content_interpretation_type < 3 as ::core::ffi::c_int
    {
        let mut stereo: *mut AVStereo3D = av_stereo3d_create_side_data(frame);
        if stereo.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
        match (*fp).arrangement_type as ::core::ffi::c_uint {
            0 => {
                (*stereo).type_0 = AV_STEREO3D_CHECKERBOARD;
            }
            1 => {
                (*stereo).type_0 = AV_STEREO3D_COLUMNS;
            }
            2 => {
                (*stereo).type_0 = AV_STEREO3D_LINES;
            }
            3 => {
                if (*fp).quincunx_sampling_flag != 0 {
                    (*stereo).type_0 = AV_STEREO3D_SIDEBYSIDE_QUINCUNX;
                } else {
                    (*stereo).type_0 = AV_STEREO3D_SIDEBYSIDE;
                }
            }
            4 => {
                (*stereo).type_0 = AV_STEREO3D_TOPBOTTOM;
            }
            5 => {
                (*stereo).type_0 = AV_STEREO3D_FRAMESEQUENCE;
            }
            6 => {
                (*stereo).type_0 = AV_STEREO3D_2D;
            }
            _ => {}
        }
        if (*fp).content_interpretation_type == 2 as ::core::ffi::c_int {
            (*stereo).flags = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
        }
        if (*fp).arrangement_type as ::core::ffi::c_uint
            == SEI_FPA_TYPE_INTERLEAVE_TEMPORAL as ::core::ffi::c_int
                as ::core::ffi::c_uint
        {
            if (*fp).current_frame_is_frame0_flag != 0 {
                (*stereo).view = AV_STEREO3D_VIEW_LEFT;
            } else {
                (*stereo).view = AV_STEREO3D_VIEW_RIGHT;
            }
        }
    }
    if (*sei).display_orientation.present != 0
        && ((*sei).display_orientation.anticlockwise_rotation != 0
            || (*sei).display_orientation.hflip != 0
            || (*sei).display_orientation.vflip != 0)
    {
        let mut o: *mut H2645SEIDisplayOrientation = &raw mut (*sei).display_orientation;
        let mut angle: ::core::ffi::c_double = ((*o).anticlockwise_rotation
            * 360 as ::core::ffi::c_int) as ::core::ffi::c_double
            / ((1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                as ::core::ffi::c_double;
        let mut rotation: *mut AVFrameSideData = av_frame_new_side_data(
            frame,
            AV_FRAME_DATA_DISPLAYMATRIX,
            (::core::mem::size_of::<int32_t>() as size_t).wrapping_mul(9 as size_t),
        );
        if rotation.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
        angle = -angle
            * (1 as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int * ((*o).hflip != 0) as ::core::ffi::c_int)
                as ::core::ffi::c_double
            * (1 as ::core::ffi::c_int
                - 2 as ::core::ffi::c_int * ((*o).vflip != 0) as ::core::ffi::c_int)
                as ::core::ffi::c_double;
        av_display_rotation_set((*rotation).data as *mut int32_t, angle);
        av_display_matrix_flip((*rotation).data as *mut int32_t, (*o).hflip, (*o).vflip);
    }
    if !(*itut_t35).a53_cc.is_null() {
        let mut sd: *mut AVFrameSideData = av_frame_new_side_data_from_buf(
            frame,
            AV_FRAME_DATA_A53_CC,
            (*itut_t35).a53_cc,
        );
        if sd.is_null() {
            av_buffer_unref(&raw mut (*itut_t35).a53_cc);
        }
        (*itut_t35).a53_cc = ::core::ptr::null_mut::<AVBufferRef>();
    }
    ret = h2645_sei_to_side_data(
        avctx,
        sei,
        &raw mut (*frame).side_data,
        &raw mut (*frame).nb_side_data,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if !(*itut_t35).afd.is_null() {
        if av_frame_new_side_data_from_buf(frame, AV_FRAME_DATA_AFD, (*itut_t35).afd)
            .is_null()
        {
            av_buffer_unref(&raw mut (*itut_t35).afd);
        }
        (*itut_t35).afd = ::core::ptr::null_mut::<AVBufferRef>();
    }
    if !(*itut_t35).lcevc.is_null() {
        ret = ff_frame_new_side_data_from_buf(
            avctx,
            frame,
            AV_FRAME_DATA_LCEVC,
            &raw mut (*itut_t35).lcevc,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    if !(*sei).film_grain_characteristics.is_null()
        && (*(*sei).film_grain_characteristics).present != 0
    {
        let mut fgc: *mut H2645SEIFilmGrainCharacteristics = (*sei)
            .film_grain_characteristics;
        let mut fgp: *mut AVFilmGrainParams = av_film_grain_params_create_side_data(
            frame,
        );
        let mut h274: *mut AVFilmGrainH274Params = ::core::ptr::null_mut::<
            AVFilmGrainH274Params,
        >();
        if fgp.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
        (*fgp).type_0 = AV_FILM_GRAIN_PARAMS_H274;
        h274 = &raw mut (*fgp).codec.h274;
        (*fgp).seed = seed as uint64_t;
        (*fgp).width = (*frame).width;
        (*fgp).height = (*frame).height;
        (*fgp).subsampling_y = 0 as ::core::ffi::c_int;
        (*fgp).subsampling_x = (*fgp).subsampling_y;
        (*h274).model_id = (*fgc).model_id;
        if (if 0 as ::core::ffi::c_int != 0
            && (1 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0)
        {
            (codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_VVC as ::core::ffi::c_int as ::core::ffi::c_uint)
                as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) != 0 || (*fgc).separate_colour_description_present_flag != 0
        {
            (*fgp).bit_depth_luma = (*fgc).bit_depth_luma;
            (*fgp).bit_depth_chroma = (*fgc).bit_depth_chroma;
            (*fgp).color_range = ((*fgc).full_range + 1 as ::core::ffi::c_int)
                as AVColorRange;
            (*fgp).color_primaries = (*fgc).color_primaries as AVColorPrimaries;
            (*fgp).color_trc = (*fgc).transfer_characteristics
                as AVColorTransferCharacteristic;
            (*fgp).color_space = (*fgc).matrix_coeffs as AVColorSpace;
        } else {
            (*fgp).bit_depth_luma = bit_depth_luma as ::core::ffi::c_int;
            (*fgp).bit_depth_chroma = bit_depth_chroma as ::core::ffi::c_int;
            if (*vui).video_signal_type_present_flag != 0 {
                (*fgp).color_range = ((*vui).video_full_range_flag
                    + 1 as ::core::ffi::c_int) as AVColorRange;
            }
            if (*vui).colour_description_present_flag != 0 {
                (*fgp).color_primaries = (*vui).colour_primaries;
                (*fgp).color_trc = (*vui).transfer_characteristics;
                (*fgp).color_space = (*vui).matrix_coeffs;
            }
        }
        (*h274).blending_mode_id = (*fgc).blending_mode_id;
        (*h274).log2_scale_factor = (*fgc).log2_scale_factor;
        memcpy(
            &raw mut (*h274).component_model_present as *mut ::core::ffi::c_void,
            &raw mut (*fgc).comp_model_present_flag as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_int; 3]>() as size_t,
        );
        memcpy(
            &raw mut (*h274).num_intensity_intervals as *mut ::core::ffi::c_void,
            &raw mut (*fgc).num_intensity_intervals as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint16_t; 3]>() as size_t,
        );
        memcpy(
            &raw mut (*h274).num_model_values as *mut ::core::ffi::c_void,
            &raw mut (*fgc).num_model_values as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 3]>() as size_t,
        );
        memcpy(
            &raw mut (*h274).intensity_interval_lower_bound as *mut ::core::ffi::c_void,
            &raw mut (*fgc).intensity_interval_lower_bound as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[[uint8_t; 256]; 3]>() as size_t,
        );
        memcpy(
            &raw mut (*h274).intensity_interval_upper_bound as *mut ::core::ffi::c_void,
            &raw mut (*fgc).intensity_interval_upper_bound as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[[uint8_t; 256]; 3]>() as size_t,
        );
        memcpy(
            &raw mut (*h274).comp_model_value as *mut ::core::ffi::c_void,
            &raw mut (*fgc).comp_model_value as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[[[int16_t; 6]; 256]; 3]>() as size_t,
        );
        if if 1 as ::core::ffi::c_int != 0
            && (0 as ::core::ffi::c_int != 0 || 0 as ::core::ffi::c_int != 0)
        {
            (codec_id as ::core::ffi::c_uint
                == AV_CODEC_ID_H264 as ::core::ffi::c_int as ::core::ffi::c_uint)
                as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        } != 0
        {
            (*fgc).present = ((*fgc).repetition_period != 0) as ::core::ffi::c_int;
        } else {
            (*fgc).present = (*fgc).persistence_flag;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_h2645_sei_to_context(
    mut avctx: *mut AVCodecContext,
    mut sei: *mut H2645SEI,
) -> ::core::ffi::c_int {
    return h2645_sei_to_side_data(
        avctx,
        sei,
        &raw mut (*avctx).decoded_side_data,
        &raw mut (*avctx).nb_decoded_side_data,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ff_h2645_sei_reset(mut s: *mut H2645SEI) {
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < (*s).unregistered.nb_buf_ref {
        av_buffer_unref(
            (*s).unregistered.buf_ref.offset(i as isize) as *mut *mut AVBufferRef,
        );
        i = i.wrapping_add(1);
    }
    (*s).unregistered.nb_buf_ref = 0 as ::core::ffi::c_uint;
    av_freep(&raw mut (*s).unregistered.buf_ref as *mut ::core::ffi::c_void);
    ff_itut_t35_unref(&raw mut (*s).itut_t35);
    (*s).ambient_viewing_environment.present = 0 as ::core::ffi::c_int;
    (*s).mastering_display.present = 0 as ::core::ffi::c_int;
    (*s).content_light.present = 0 as ::core::ffi::c_int;
    av_refstruct_unref(
        &raw mut (*s).film_grain_characteristics as *mut ::core::ffi::c_void,
    );
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
