// Translated from FFmpeg 9.0.1 libavcodec/h264dec.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
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
    static ff_h264_golomb_to_pict_type: [uint8_t; 5];
    fn memset(
        _: *mut ::core::ffi::c_void,
        _: ::core::ffi::c_int,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        _: *const ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> ::core::ffi::c_int;
    static ff_golomb_vlc_len: [uint8_t; 512];
    static ff_ue_golomb_vlc_code: [uint8_t; 512];
    fn ff_thread_report_progress(
        f: *mut ThreadFrame,
        progress: ::core::ffi::c_int,
        field: ::core::ffi::c_int,
    );
    fn ff_thread_finish_setup(avctx: *mut AVCodecContext);
    fn abort() -> !;
    static ff_h264_profiles: [AVProfile; 0];
    fn ff_er_init(s: *mut ERContext) -> ::core::ffi::c_int;
    fn pthread_once(
        _: *mut pthread_once_t,
        _: Option<unsafe extern "C" fn() -> ()>,
    ) -> ::core::ffi::c_int;
    fn ff_er_frame_end(s: *mut ERContext, decode_error_flags: *mut ::core::ffi::c_int);
    fn ff_print_debug_info2(
        avctx: *mut AVCodecContext,
        pict: *mut AVFrame,
        mbtype_table: *const uint32_t,
        qscale_table: *const int8_t,
        motion_val: *const *mut [int16_t; 2],
        mb_width: ::core::ffi::c_int,
        mb_height: ::core::ffi::c_int,
        mb_stride: ::core::ffi::c_int,
        quarter_sample: ::core::ffi::c_int,
    );
    fn avpriv_request_sample(
        avc: *mut ::core::ffi::c_void,
        msg: *const ::core::ffi::c_char,
        ...
    );
    fn ff_h2645_packet_split(
        pkt: *mut H2645Packet,
        buf: *const uint8_t,
        length: ::core::ffi::c_int,
        logctx: *mut ::core::ffi::c_void,
        nal_length_size: ::core::ffi::c_int,
        codec_id: AVCodecID,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_refstruct_unref(objp: *mut ::core::ffi::c_void);
    fn ff_h264_decode_extradata(
        data: *const uint8_t,
        size: ::core::ffi::c_int,
        ps: *mut H264ParamSets,
        is_avc: *mut ::core::ffi::c_int,
        nal_length_size: *mut ::core::ffi::c_int,
        err_recognition: ::core::ffi::c_int,
        logctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn ff_h2645_packet_uninit(pkt: *mut H2645Packet);
    fn ff_h264_sei_decode(
        h: *mut H264SEIContext,
        gb: *mut GetBitContext,
        ps: *const H264ParamSets,
        logctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn ff_h264_sei_uninit(h: *mut H264SEIContext);
    fn ff_h264_sei_stereo_mode(
        h: *const H2645SEIFramePacking,
    ) -> *const ::core::ffi::c_char;
    fn av_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn ff_h264_decode_seq_parameter_set(
        gb: *mut GetBitContext,
        avctx: *mut AVCodecContext,
        ps: *mut H264ParamSets,
        ignore_truncation: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_dict_set(
        pm: *mut *mut AVDictionary,
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_video_enc_params_create_side_data(
        frame: *mut AVFrame,
        type_0: AVVideoEncParamsType,
        nb_blocks: ::core::ffi::c_uint,
    ) -> *mut AVVideoEncParams;
    fn ff_h264_decode_picture_parameter_set(
        gb: *mut GetBitContext,
        avctx: *mut AVCodecContext,
        ps: *mut H264ParamSets,
        bit_length: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_image_copy(
        dst_data: *const *mut uint8_t,
        dst_linesizes: *const ::core::ffi::c_int,
        src_data: *const *const uint8_t,
        src_linesizes: *const ::core::ffi::c_int,
        pix_fmt: AVPixelFormat,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    );
    fn ff_h264_ps_uninit(ps: *mut H264ParamSets);
    fn av_pix_fmt_desc_get(pix_fmt: AVPixelFormat) -> *const AVPixFmtDescriptor;
    fn av_refstruct_pool_alloc(
        size: size_t,
        flags: ::core::ffi::c_uint,
    ) -> *mut AVRefStructPool;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_default_item_name(ctx: *mut ::core::ffi::c_void) -> *const ::core::ffi::c_char;
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn ff_h264_remove_all_refs(h: *mut H264Context);
    fn ff_h264_hl_decode_mb(h: *const H264Context, sl: *mut H264SliceContext);
    fn ff_h264_decode_init_vlc();
    fn ff_h264_field_end(
        h: *mut H264Context,
        sl: *mut H264SliceContext,
        in_setup: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ff_h264_unref_picture(pic: *mut H264Picture);
    fn ff_h264_queue_decode_slice(
        h: *mut H264Context,
        nal: *const H2645NAL,
    ) -> ::core::ffi::c_int;
    fn ff_h264_execute_decode_slices(h: *mut H264Context) -> ::core::ffi::c_int;
    fn ff_h264_update_thread_context(
        dst: *mut AVCodecContext,
        src: *const AVCodecContext,
    ) -> ::core::ffi::c_int;
    fn ff_h264_update_thread_context_for_user(
        dst: *mut AVCodecContext,
        src: *const AVCodecContext,
    ) -> ::core::ffi::c_int;
    fn ff_h264_set_erpic(dst: *mut ERPicture, src: *const H264Picture);
    fn av_packet_get_side_data(
        pkt: *const AVPacket,
        type_0: AVPacketSideDataType,
        size: *mut size_t,
    ) -> *mut uint8_t;
    fn av_frame_alloc() -> *mut AVFrame;
    fn av_frame_free(frame: *mut *mut AVFrame);
    fn av_frame_ref(dst: *mut AVFrame, src: *const AVFrame) -> ::core::ffi::c_int;
    fn av_frame_unref(frame: *mut AVFrame);
    fn av_frame_copy_props(dst: *mut AVFrame, src: *const AVFrame) -> ::core::ffi::c_int;
    fn av_frame_remove_side_data(frame: *mut AVFrame, type_0: AVFrameSideDataType);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
}
pub type int64_t = ::core::ffi::c_long;
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
pub type int8_t = ::core::ffi::c_schar;
pub type int16_t = ::core::ffi::c_short;
pub type int32_t = ::core::ffi::c_int;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint16_t = ::core::ffi::c_ushort;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
pub type ptrdiff_t = ::core::ffi::c_long;
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
pub type pthread_once_t = ::core::ffi::c_int;
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
pub type AVVideoEncParamsType = ::core::ffi::c_int;
pub const AV_VIDEO_ENC_PARAMS_MPEG2: AVVideoEncParamsType = 2;
pub const AV_VIDEO_ENC_PARAMS_H264: AVVideoEncParamsType = 1;
pub const AV_VIDEO_ENC_PARAMS_VP9: AVVideoEncParamsType = 0;
pub const AV_VIDEO_ENC_PARAMS_NONE: AVVideoEncParamsType = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVVideoEncParams {
    pub nb_blocks: ::core::ffi::c_uint,
    pub blocks_offset: size_t,
    pub block_size: size_t,
    pub type_0: AVVideoEncParamsType,
    pub qp: int32_t,
    pub delta_qp: [[int32_t; 2]; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVVideoBlockParams {
    pub src_x: ::core::ffi::c_int,
    pub src_y: ::core::ffi::c_int,
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub delta_qp: int32_t,
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
pub struct AVCodecHWConfig {
    pub pix_fmt: AVPixelFormat,
    pub methods: ::core::ffi::c_int,
    pub device_type: AVHWDeviceType,
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
pub struct FFCodecDefault {
    pub key: *const ::core::ffi::c_char,
    pub value: *const ::core::ffi::c_char,
}
pub type FFCodecType = ::core::ffi::c_uint;
pub const FF_CODEC_CB_TYPE_RECEIVE_PACKET: FFCodecType = 5;
pub const FF_CODEC_CB_TYPE_ENCODE_SUB: FFCodecType = 4;
pub const FF_CODEC_CB_TYPE_ENCODE: FFCodecType = 3;
pub const FF_CODEC_CB_TYPE_RECEIVE_FRAME: FFCodecType = 2;
pub const FF_CODEC_CB_TYPE_DECODE_SUB: FFCodecType = 1;
pub const FF_CODEC_CB_TYPE_DECODE: FFCodecType = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVCodecHWConfigInternal {
    pub public: AVCodecHWConfig,
    pub hwaccel: *const FFHWAccel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFHWAccel {
    pub p: AVHWAccel,
    pub alloc_frame: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVFrame) -> ::core::ffi::c_int,
    >,
    pub start_frame: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVBufferRef,
            *const uint8_t,
            uint32_t,
        ) -> ::core::ffi::c_int,
    >,
    pub decode_params: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            ::core::ffi::c_int,
            *const uint8_t,
            uint32_t,
        ) -> ::core::ffi::c_int,
    >,
    pub decode_slice: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const uint8_t,
            uint32_t,
        ) -> ::core::ffi::c_int,
    >,
    pub end_frame: Option<
        unsafe extern "C" fn(*mut AVCodecContext) -> ::core::ffi::c_int,
    >,
    pub frame_priv_data_size: ::core::ffi::c_int,
    pub priv_data_size: ::core::ffi::c_int,
    pub caps_internal: ::core::ffi::c_int,
    pub init: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ::core::ffi::c_int>,
    pub uninit: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ::core::ffi::c_int>,
    pub frame_params: Option<
        unsafe extern "C" fn(*mut AVCodecContext, *mut AVBufferRef) -> ::core::ffi::c_int,
    >,
    pub update_thread_context: Option<
        unsafe extern "C" fn(
            *mut AVCodecContext,
            *const AVCodecContext,
        ) -> ::core::ffi::c_int,
    >,
    pub free_frame_priv: Option<
        unsafe extern "C" fn(AVRefStructOpaque, *mut ::core::ffi::c_void) -> (),
    >,
    pub flush: Option<unsafe extern "C" fn(*mut AVCodecContext) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union AVRefStructOpaque {
    pub nc: *mut ::core::ffi::c_void,
    pub c: *const ::core::ffi::c_void,
}
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
pub struct H264ParamSets {
    pub sps_list: [*const SPS; 32],
    pub pps_list: [*const PPS; 256],
    pub pps: *const PPS,
    pub sps: *const SPS,
    pub overread_warning_printed: [::core::ffi::c_int; 2],
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
pub struct H264SEIContext {
    pub common: H2645SEI,
    pub picture_timing: H264SEIPictureTiming,
    pub recovery_point: H264SEIRecoveryPoint,
    pub buffering_period: H264SEIBufferingPeriod,
    pub green_metadata: H264SEIGreenMetaData,
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
pub struct H264SEIBufferingPeriod {
    pub present: ::core::ffi::c_int,
    pub initial_cpb_removal_delay: [::core::ffi::c_int; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264SEIRecoveryPoint {
    pub recovery_frame_cnt: ::core::ffi::c_int,
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
pub struct H264SEITimeCode {
    pub full: ::core::ffi::c_int,
    pub frame: ::core::ffi::c_int,
    pub seconds: ::core::ffi::c_int,
    pub minutes: ::core::ffi::c_int,
    pub hours: ::core::ffi::c_int,
    pub dropframe: ::core::ffi::c_int,
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
pub struct H2645SEIContentLight {
    pub present: ::core::ffi::c_int,
    pub max_content_light_level: uint16_t,
    pub max_pic_average_light_level: uint16_t,
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
pub struct H2645SEIAmbientViewingEnvironment {
    pub present: ::core::ffi::c_int,
    pub ambient_illuminance: uint32_t,
    pub ambient_light_x: uint16_t,
    pub ambient_light_y: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645SEIAlternativeTransfer {
    pub present: ::core::ffi::c_int,
    pub preferred_transfer_characteristics: ::core::ffi::c_int,
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
pub struct AVFilmGrainAFGS1Params {
    pub enable: ::core::ffi::c_int,
    pub sets: [*mut AVBufferRef; 8],
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
pub type atomic_int = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MMCO {
    pub opcode: MMCOOpcode,
    pub short_pic_num: ::core::ffi::c_int,
    pub long_arg: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H2645Packet {
    pub nals: *mut H2645NAL,
    pub rbsp: H2645RBSP,
    pub nb_nals: ::core::ffi::c_int,
    pub nals_allocated: ::core::ffi::c_int,
    pub nal_buffer_size: ::core::ffi::c_uint,
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
pub struct GetBitContext {
    pub buffer: *const uint8_t,
    pub index: ::core::ffi::c_int,
    pub size_in_bits: ::core::ffi::c_int,
    pub size_in_bits_plus8: ::core::ffi::c_int,
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
    pub ref_modifications: [[C2RustUnnamed_5; 32]; 2],
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
pub struct CABACContext {
    pub low: ::core::ffi::c_int,
    pub range: ::core::ffi::c_int,
    pub bytestream_start: *const uint8_t,
    pub bytestream: *const uint8_t,
    pub bytestream_end: *const uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_5 {
    pub op: uint8_t,
    pub val: uint32_t,
}
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
pub struct H264QpelContext {
    pub put_h264_qpel_pixels_tab: [[qpel_mc_func; 16]; 3],
    pub avg_h264_qpel_pixels_tab: [[qpel_mc_func; 16]; 3],
}
pub type qpel_mc_func = Option<
    unsafe extern "C" fn(*mut uint8_t, *const uint8_t, ptrdiff_t) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct H264ChromaContext {
    pub put_h264_chroma_pixels_tab: [h264_chroma_mc_func; 4],
    pub avg_h264_chroma_pixels_tab: [h264_chroma_mc_func; 4],
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
pub type memory_order = ::core::ffi::c_uint;
pub const memory_order_seq_cst: memory_order = 5;
pub const memory_order_acq_rel: memory_order = 4;
pub const memory_order_release: memory_order = 3;
pub const memory_order_acquire: memory_order = 2;
pub const memory_order_consume: memory_order = 1;
pub const memory_order_relaxed: memory_order = 0;
pub type C2RustUnnamed_6 = ::core::ffi::c_uint;
pub const H264_NAL_UNSPECIFIED31: C2RustUnnamed_6 = 31;
pub const H264_NAL_UNSPECIFIED30: C2RustUnnamed_6 = 30;
pub const H264_NAL_UNSPECIFIED29: C2RustUnnamed_6 = 29;
pub const H264_NAL_UNSPECIFIED28: C2RustUnnamed_6 = 28;
pub const H264_NAL_UNSPECIFIED27: C2RustUnnamed_6 = 27;
pub const H264_NAL_UNSPECIFIED26: C2RustUnnamed_6 = 26;
pub const H264_NAL_UNSPECIFIED25: C2RustUnnamed_6 = 25;
pub const H264_NAL_UNSPECIFIED24: C2RustUnnamed_6 = 24;
pub const H264_NAL_RESERVED23: C2RustUnnamed_6 = 23;
pub const H264_NAL_RESERVED22: C2RustUnnamed_6 = 22;
pub const H264_NAL_DEPTH_EXTEN_SLICE: C2RustUnnamed_6 = 21;
pub const H264_NAL_EXTEN_SLICE: C2RustUnnamed_6 = 20;
pub const H264_NAL_AUXILIARY_SLICE: C2RustUnnamed_6 = 19;
pub const H264_NAL_RESERVED18: C2RustUnnamed_6 = 18;
pub const H264_NAL_RESERVED17: C2RustUnnamed_6 = 17;
pub const H264_NAL_DPS: C2RustUnnamed_6 = 16;
pub const H264_NAL_SUB_SPS: C2RustUnnamed_6 = 15;
pub const H264_NAL_PREFIX: C2RustUnnamed_6 = 14;
pub const H264_NAL_SPS_EXT: C2RustUnnamed_6 = 13;
pub const H264_NAL_FILLER_DATA: C2RustUnnamed_6 = 12;
pub const H264_NAL_END_STREAM: C2RustUnnamed_6 = 11;
pub const H264_NAL_END_SEQUENCE: C2RustUnnamed_6 = 10;
pub const H264_NAL_AUD: C2RustUnnamed_6 = 9;
pub const H264_NAL_PPS: C2RustUnnamed_6 = 8;
pub const H264_NAL_SPS: C2RustUnnamed_6 = 7;
pub const H264_NAL_SEI: C2RustUnnamed_6 = 6;
pub const H264_NAL_IDR_SLICE: C2RustUnnamed_6 = 5;
pub const H264_NAL_DPC: C2RustUnnamed_6 = 4;
pub const H264_NAL_DPB: C2RustUnnamed_6 = 3;
pub const H264_NAL_DPA: C2RustUnnamed_6 = 2;
pub const H264_NAL_SLICE: C2RustUnnamed_6 = 1;
pub const H264_NAL_UNSPECIFIED: C2RustUnnamed_6 = 0;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union unaligned_64 {
    pub l: uint64_t,
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
pub type C2RustUnnamed_7 = ::core::ffi::c_uint;
pub const H2645_FLAG_USE_REF: C2RustUnnamed_7 = 4;
pub const H2645_FLAG_SMALL_PADDING: C2RustUnnamed_7 = 2;
pub const H2645_FLAG_IS_NALFF: C2RustUnnamed_7 = 1;
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
unsafe extern "C" fn av_bswap16(mut x: uint16_t) -> uint16_t {
    x = (x as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
        | (x as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint16_t;
    return x;
}
#[no_mangle]
pub static mut ff_h264_mb_sizes: [uint16_t; 4] = [
    256 as ::core::ffi::c_int as uint16_t,
    384 as ::core::ffi::c_int as uint16_t,
    512 as ::core::ffi::c_int as uint16_t,
    768 as ::core::ffi::c_int as uint16_t,
];
#[no_mangle]
pub unsafe extern "C" fn avpriv_h264_has_num_reorder_frames(
    mut avctx: *mut AVCodecContext,
) -> ::core::ffi::c_int {
    let mut h: *mut H264Context = (*avctx).priv_data as *mut H264Context;
    return if !h.is_null() && !(*h).ps.sps.is_null() {
        (*(*h).ps.sps).num_reorder_frames
    } else {
        0 as ::core::ffi::c_int
    };
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
unsafe extern "C" fn h264_er_decode_mb(
    mut opaque: *mut ::core::ffi::c_void,
    mut ref_0: ::core::ffi::c_int,
    mut mv_dir: ::core::ffi::c_int,
    mut mv_type: ::core::ffi::c_int,
    mut mv: *mut [[[::core::ffi::c_int; 2]; 4]; 2],
    mut mb_x: ::core::ffi::c_int,
    mut mb_y: ::core::ffi::c_int,
    mut mb_intra: ::core::ffi::c_int,
    mut mb_skipped: ::core::ffi::c_int,
) {
    let mut h: *const H264Context = opaque as *const H264Context;
    let mut sl: *mut H264SliceContext = (*h)
        .slice_ctx
        .offset(0 as ::core::ffi::c_int as isize) as *mut H264SliceContext;
    (*sl).mb_x = mb_x;
    (*sl).mb_y = mb_y;
    (*sl).mb_xy = mb_x + mb_y * (*h).mb_stride;
    memset(
        &raw mut (*sl).non_zero_count_cache as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 120]>() as size_t,
    );
    if ref_0 as ::core::ffi::c_uint >= (*sl).ref_count[0 as ::core::ffi::c_int as usize]
    {
        ref_0 = 0 as ::core::ffi::c_int;
    }
    if (*sl)
        .ref_list[0 as ::core::ffi::c_int as usize][ref_0 as usize]
        .data[0 as ::core::ffi::c_int as usize]
        .is_null()
    {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            48 as ::core::ffi::c_int,
            b"Reference not available for error concealing\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        ref_0 = 0 as ::core::ffi::c_int;
    }
    if (*sl).ref_list[0 as ::core::ffi::c_int as usize][ref_0 as usize].reference
        & 3 as ::core::ffi::c_int != 3 as ::core::ffi::c_int
    {
        av_log(
            (*h).avctx as *mut ::core::ffi::c_void,
            48 as ::core::ffi::c_int,
            b"Reference invalid\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return;
    }
    fill_rectangle(
        (*(&raw const (*h).cur_pic.ref_index as *const *mut int8_t)
            .offset(0 as ::core::ffi::c_int as isize))
            .offset((4 as ::core::ffi::c_int * (*sl).mb_xy) as isize) as *mut int8_t
            as *mut ::core::ffi::c_void,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        ref_0 as uint32_t,
        1 as ::core::ffi::c_int,
    );
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
        ref_0 as uint32_t,
        1 as ::core::ffi::c_int,
    );
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
        pack16to32(
            (*mv)[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
            (*mv)[0 as ::core::ffi::c_int
                as usize][0 as ::core::ffi::c_int
                as usize][1 as ::core::ffi::c_int as usize] as ::core::ffi::c_uint,
        ),
        4 as ::core::ffi::c_int,
    );
    (*sl).mb_field_decoding_flag = 0 as ::core::ffi::c_int;
    (*sl).mb_mbaff = (*sl).mb_field_decoding_flag;
    ff_h264_hl_decode_mb(
        h,
        (*h).slice_ctx.offset(0 as ::core::ffi::c_int as isize) as *mut H264SliceContext,
    );
}
#[inline]
unsafe extern "C" fn av_bswap64(mut x: uint64_t) -> uint64_t {
    return (av_bswap32(x as uint32_t) as uint64_t) << 32 as ::core::ffi::c_int
        | av_bswap32((x >> 32 as ::core::ffi::c_int) as uint32_t) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_draw_horiz_band(
    mut h: *const H264Context,
    mut sl: *mut H264SliceContext,
    mut y: ::core::ffi::c_int,
    mut height: ::core::ffi::c_int,
) {
    let mut avctx: *mut AVCodecContext = (*h).avctx;
    let mut src: *const AVFrame = (*h).cur_pic.f;
    let mut desc: *const AVPixFmtDescriptor = ::core::ptr::null::<AVPixFmtDescriptor>();
    let mut offset: [::core::ffi::c_int; 8] = [0; 8];
    let mut vshift: ::core::ffi::c_int = 0;
    let field_pic: ::core::ffi::c_int = ((*h).picture_structure
        != 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if (*avctx).draw_horiz_band.is_none() {
        return;
    }
    if field_pic != 0 && (*h).first_field != 0
        && (*avctx).slice_flags & 0x2 as ::core::ffi::c_int == 0
    {
        return;
    }
    if field_pic != 0 {
        height <<= 1 as ::core::ffi::c_int;
        y <<= 1 as ::core::ffi::c_int;
    }
    height = if height > (*avctx).height - y { (*avctx).height - y } else { height };
    desc = av_pix_fmt_desc_get((*avctx).pix_fmt);
    vshift = (*desc).log2_chroma_h as ::core::ffi::c_int;
    offset[0 as ::core::ffi::c_int as usize] = y
        * (*src).linesize[0 as ::core::ffi::c_int as usize];
    offset[2 as ::core::ffi::c_int as usize] = (y >> vshift)
        * (*src).linesize[1 as ::core::ffi::c_int as usize];
    offset[1 as ::core::ffi::c_int as usize] = offset[2 as ::core::ffi::c_int as usize];
    let mut i: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
    while i < 8 as ::core::ffi::c_int {
        offset[i as usize] = 0 as ::core::ffi::c_int;
        i += 1;
    }
    (*avctx)
        .draw_horiz_band
        .expect(
            "non-null function pointer",
        )(
        avctx as *mut AVCodecContext,
        src,
        &raw mut offset as *mut ::core::ffi::c_int,
        y,
        (*h).picture_structure,
        height,
    );
}
#[inline]
unsafe extern "C" fn get_ue_golomb_long(
    mut gb: *mut GetBitContext,
) -> ::core::ffi::c_uint {
    let mut buf: ::core::ffi::c_uint = 0;
    let mut log: ::core::ffi::c_uint = 0;
    buf = show_bits_long(gb, 32 as ::core::ffi::c_int);
    log = (31 as ::core::ffi::c_int
        - (31 as ::core::ffi::c_int
            - (buf | 1 as ::core::ffi::c_uint).leading_zeros() as i32))
        as ::core::ffi::c_uint;
    skip_bits_long(gb, log as ::core::ffi::c_int);
    return get_bits_long(
            gb,
            log.wrapping_add(1 as ::core::ffi::c_uint) as ::core::ffi::c_int,
        )
        .wrapping_sub(1 as ::core::ffi::c_uint);
}
#[inline]
unsafe extern "C" fn get_ue_golomb_31(mut gb: *mut GetBitContext) -> ::core::ffi::c_int {
    let mut buf: ::core::ffi::c_uint = 0;
    let mut re_index: ::core::ffi::c_uint = (*gb).index as ::core::ffi::c_uint;
    let mut re_cache: ::core::ffi::c_uint = 0;
    re_cache = (av_bswap32(
        (*((*gb).buffer.offset((re_index >> 3 as ::core::ffi::c_int) as isize)
            as *const unaligned_32))
            .l,
    ) << (re_index & 7 as ::core::ffi::c_uint)
        >> 32 as ::core::ffi::c_int - 32 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    buf = re_cache as ::core::ffi::c_uint;
    buf >>= 32 as ::core::ffi::c_int - 9 as ::core::ffi::c_int;
    re_index = re_index
        .wrapping_add(ff_golomb_vlc_len[buf as usize] as ::core::ffi::c_uint);
    (*gb).index = re_index as ::core::ffi::c_int;
    return ff_ue_golomb_vlc_code[buf as usize] as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn pack16to32(
    mut a: ::core::ffi::c_uint,
    mut b: ::core::ffi::c_uint,
) -> uint32_t {
    return (a as uint32_t & 0xffff as uint32_t)
        .wrapping_add((b as uint32_t) << 16 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_free_tables(mut h: *mut H264Context) {
    let mut i: ::core::ffi::c_int = 0;
    av_freep(&raw mut (*h).intra4x4_pred_mode as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*h).chroma_pred_mode_table as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*h).cbp_table as *mut ::core::ffi::c_void);
    av_freep(
        (&raw mut (*h).mvd_table as *mut *mut [uint8_t; 2])
            .offset(0 as ::core::ffi::c_int as isize) as *mut *mut [uint8_t; 2]
            as *mut ::core::ffi::c_void,
    );
    av_freep(
        (&raw mut (*h).mvd_table as *mut *mut [uint8_t; 2])
            .offset(1 as ::core::ffi::c_int as isize) as *mut *mut [uint8_t; 2]
            as *mut ::core::ffi::c_void,
    );
    av_freep(&raw mut (*h).direct_table as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*h).non_zero_count as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*h).slice_table_base as *mut ::core::ffi::c_void);
    (*h).slice_table = ::core::ptr::null_mut::<uint16_t>();
    av_freep(&raw mut (*h).list_counts as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*h).mb2b_xy as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*h).mb2br_xy as *mut ::core::ffi::c_void);
    av_refstruct_pool_uninit(&raw mut (*h).qscale_table_pool);
    av_refstruct_pool_uninit(&raw mut (*h).mb_type_pool);
    av_refstruct_pool_uninit(&raw mut (*h).motion_val_pool);
    av_refstruct_pool_uninit(&raw mut (*h).ref_index_pool);
    av_freep(&raw mut (*h).er.mb_index2xy as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*h).er.error_status_table as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*h).er.er_temp_buffer as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*h).dc_val_base as *mut ::core::ffi::c_void);
    i = 0 as ::core::ffi::c_int;
    while i < (*h).nb_slice_ctx {
        let mut sl: *mut H264SliceContext = (*h).slice_ctx.offset(i as isize)
            as *mut H264SliceContext;
        av_freep(&raw mut (*sl).bipred_scratchpad as *mut ::core::ffi::c_void);
        av_freep(&raw mut (*sl).edge_emu_buffer as *mut ::core::ffi::c_void);
        av_freep(
            (&raw mut (*sl).top_borders as *mut *mut [uint8_t; 96])
                .offset(0 as ::core::ffi::c_int as isize) as *mut *mut [uint8_t; 96]
                as *mut ::core::ffi::c_void,
        );
        av_freep(
            (&raw mut (*sl).top_borders as *mut *mut [uint8_t; 96])
                .offset(1 as ::core::ffi::c_int as isize) as *mut *mut [uint8_t; 96]
                as *mut ::core::ffi::c_void,
        );
        (*sl).bipred_scratchpad_allocated = 0 as ::core::ffi::c_int;
        (*sl).edge_emu_buffer_allocated = 0 as ::core::ffi::c_int;
        (*sl).top_borders_allocated[0 as ::core::ffi::c_int as usize] = 0
            as ::core::ffi::c_int;
        (*sl).top_borders_allocated[1 as ::core::ffi::c_int as usize] = 0
            as ::core::ffi::c_int;
        i += 1;
    }
}
#[inline(always)]
unsafe extern "C" fn av_video_enc_params_block(
    mut par: *mut AVVideoEncParams,
    mut idx: ::core::ffi::c_uint,
) -> *mut AVVideoBlockParams {
    if !(idx < (*par).nb_blocks) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"idx < par->nb_blocks\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/video_enc_params.h\0" as *const u8
                as *const ::core::ffi::c_char,
            145 as ::core::ffi::c_int,
        );
        abort();
    }
    return (par as *mut uint8_t)
        .offset((*par).blocks_offset as isize)
        .offset((idx as size_t).wrapping_mul((*par).block_size) as isize)
        as *mut AVVideoBlockParams;
}
#[inline]
unsafe extern "C" fn ffhwaccel(mut codec: *const AVHWAccel) -> *const FFHWAccel {
    return codec as *const FFHWAccel;
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_alloc_tables(
    mut h: *mut H264Context,
) -> ::core::ffi::c_int {
    let er: *mut ERContext = &raw mut (*h).er;
    let big_mb_num: ::core::ffi::c_int = (*h).mb_stride
        * ((*h).mb_height + 1 as ::core::ffi::c_int);
    let row_mb_num: ::core::ffi::c_int = 2 as ::core::ffi::c_int * (*h).mb_stride
        * (if (*h).nb_slice_ctx > 1 as ::core::ffi::c_int {
            (*h).nb_slice_ctx
        } else {
            1 as ::core::ffi::c_int
        });
    let st_size: ::core::ffi::c_int = big_mb_num + (*h).mb_stride;
    let mut x: ::core::ffi::c_int = 0;
    let mut y: ::core::ffi::c_int = 0;
    (*h).intra4x4_pred_mode = av_calloc(
        (row_mb_num * 8 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<int8_t>() as size_t,
    ) as *mut int8_t;
    if (*h).intra4x4_pred_mode.is_null()
        || {
            (*h).non_zero_count = av_calloc(
                big_mb_num as size_t,
                ::core::mem::size_of::<[uint8_t; 48]>() as size_t,
            ) as *mut [uint8_t; 48];
            (*h).non_zero_count.is_null()
        }
        || {
            (*h).slice_table_base = av_calloc(
                st_size as size_t,
                ::core::mem::size_of::<uint16_t>() as size_t,
            ) as *mut uint16_t;
            (*h).slice_table_base.is_null()
        }
        || {
            (*h).cbp_table = av_calloc(
                big_mb_num as size_t,
                ::core::mem::size_of::<uint16_t>() as size_t,
            ) as *mut uint16_t;
            (*h).cbp_table.is_null()
        }
        || {
            (*h).chroma_pred_mode_table = av_calloc(
                big_mb_num as size_t,
                ::core::mem::size_of::<uint8_t>() as size_t,
            ) as *mut uint8_t;
            (*h).chroma_pred_mode_table.is_null()
        }
        || {
            (*h).mvd_table[0 as ::core::ffi::c_int as usize] = av_calloc(
                (row_mb_num * 8 as ::core::ffi::c_int) as size_t,
                ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
            ) as *mut [uint8_t; 2];
            (*h).mvd_table[0 as ::core::ffi::c_int as usize].is_null()
        }
        || {
            (*h).mvd_table[1 as ::core::ffi::c_int as usize] = av_calloc(
                (row_mb_num * 8 as ::core::ffi::c_int) as size_t,
                ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
            ) as *mut [uint8_t; 2];
            (*h).mvd_table[1 as ::core::ffi::c_int as usize].is_null()
        }
        || {
            (*h).direct_table = av_calloc(
                (big_mb_num * 4 as ::core::ffi::c_int) as size_t,
                ::core::mem::size_of::<uint8_t>() as size_t,
            ) as *mut uint8_t;
            (*h).direct_table.is_null()
        }
        || {
            (*h).list_counts = av_calloc(
                big_mb_num as size_t,
                ::core::mem::size_of::<uint8_t>() as size_t,
            ) as *mut uint8_t;
            (*h).list_counts.is_null()
        }
        || {
            (*h).mb2b_xy = av_calloc(
                big_mb_num as size_t,
                ::core::mem::size_of::<uint32_t>() as size_t,
            ) as *mut uint32_t;
            (*h).mb2b_xy.is_null()
        }
        || {
            (*h).mb2br_xy = av_calloc(
                big_mb_num as size_t,
                ::core::mem::size_of::<uint32_t>() as size_t,
            ) as *mut uint32_t;
            (*h).mb2br_xy.is_null()
        }
    {
        return -(12 as ::core::ffi::c_int);
    }
    let ref mut fresh0 = (*(*h).slice_ctx.offset(0 as ::core::ffi::c_int as isize))
        .intra4x4_pred_mode;
    *fresh0 = (*h).intra4x4_pred_mode;
    let ref mut fresh1 = (*(*h).slice_ctx.offset(0 as ::core::ffi::c_int as isize))
        .mvd_table[0 as ::core::ffi::c_int as usize];
    *fresh1 = (*h).mvd_table[0 as ::core::ffi::c_int as usize];
    let ref mut fresh2 = (*(*h).slice_ctx.offset(0 as ::core::ffi::c_int as isize))
        .mvd_table[1 as ::core::ffi::c_int as usize];
    *fresh2 = (*h).mvd_table[1 as ::core::ffi::c_int as usize];
    memset(
        (*h).slice_table_base as *mut ::core::ffi::c_void,
        -(1 as ::core::ffi::c_int),
        (st_size as size_t).wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
    );
    (*h).slice_table = (*h)
        .slice_table_base
        .offset(((*h).mb_stride * 2 as ::core::ffi::c_int) as isize)
        .offset(1 as ::core::ffi::c_int as isize);
    y = 0 as ::core::ffi::c_int;
    while y < (*h).mb_height {
        x = 0 as ::core::ffi::c_int;
        while x < (*h).mb_width {
            let mb_xy: ::core::ffi::c_int = x + y * (*h).mb_stride;
            let b_xy: ::core::ffi::c_int = 4 as ::core::ffi::c_int * x
                + 4 as ::core::ffi::c_int * y * (*h).b_stride;
            *(*h).mb2b_xy.offset(mb_xy as isize) = b_xy as uint32_t;
            *(*h).mb2br_xy.offset(mb_xy as isize) = (8 as ::core::ffi::c_int
                * (if 0 as ::core::ffi::c_int != 0 {
                    mb_xy
                } else {
                    mb_xy % (2 as ::core::ffi::c_int * (*h).mb_stride)
                })) as uint32_t;
            x += 1;
        }
        y += 1;
    }
    let mut y_size: ::core::ffi::c_int = (2 as ::core::ffi::c_int * (*h).mb_width
        + 1 as ::core::ffi::c_int)
        * (2 as ::core::ffi::c_int * (*h).mb_height + 1 as ::core::ffi::c_int);
    let mut yc_size: ::core::ffi::c_int = y_size + 2 as ::core::ffi::c_int * big_mb_num;
    (*er).avctx = (*h).avctx;
    (*er).decode_mb = Some(
        h264_er_decode_mb
            as unsafe extern "C" fn(
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
    )
        as Option<
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
        >;
    (*er).opaque = h as *mut ::core::ffi::c_void;
    (*er).quarter_sample = 1 as ::core::ffi::c_int;
    (*er).mb_num = (*h).mb_num;
    (*er).mb_width = (*h).mb_width;
    (*er).mb_height = (*h).mb_height;
    (*er).mb_stride = (*h).mb_stride as ptrdiff_t;
    (*er).b8_stride = ((*h).mb_width * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
        as ptrdiff_t;
    (*er).mb_index2xy = av_calloc(
        ((*h).mb_num + 1 as ::core::ffi::c_int) as size_t,
        ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
    ) as *mut ::core::ffi::c_int;
    if (*er).mb_index2xy.is_null()
        || {
            (*h).dc_val_base = av_calloc(
                yc_size as size_t,
                ::core::mem::size_of::<int16_t>() as size_t,
            ) as *mut int16_t;
            (*h).dc_val_base.is_null()
        }
    {
        return -(12 as ::core::ffi::c_int);
    }
    y = 0 as ::core::ffi::c_int;
    while y < (*h).mb_height {
        x = 0 as ::core::ffi::c_int;
        while x < (*h).mb_width {
            *(*er).mb_index2xy.offset((x + y * (*h).mb_width) as isize) = x
                + y * (*h).mb_stride;
            x += 1;
        }
        y += 1;
    }
    *(*er).mb_index2xy.offset(((*h).mb_height * (*h).mb_width) as isize) = ((*h)
        .mb_height - 1 as ::core::ffi::c_int) * (*h).mb_stride + (*h).mb_width;
    (*er).dc_val[0 as ::core::ffi::c_int as usize] = (*h)
        .dc_val_base
        .offset(((*h).mb_width * 2 as ::core::ffi::c_int) as isize)
        .offset(2 as ::core::ffi::c_int as isize);
    (*er).dc_val[1 as ::core::ffi::c_int as usize] = (*h)
        .dc_val_base
        .offset(y_size as isize)
        .offset((*h).mb_stride as isize)
        .offset(1 as ::core::ffi::c_int as isize);
    (*er).dc_val[2 as ::core::ffi::c_int as usize] = (*er)
        .dc_val[1 as ::core::ffi::c_int as usize]
        .offset(big_mb_num as isize);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < yc_size {
        *(*h).dc_val_base.offset(i as isize) = 1024 as int16_t;
        i += 1;
    }
    return ff_er_init(er);
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_slice_context_init(
    mut h: *mut H264Context,
    mut sl: *mut H264SliceContext,
) {
    (*sl)
        .ref_cache[1 as ::core::ffi::c_int
        as usize][(scan8[13 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as usize] = -(2 as ::core::ffi::c_int) as int8_t;
    (*sl)
        .ref_cache[1 as ::core::ffi::c_int
        as usize][(scan8[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as usize] = (*sl)
        .ref_cache[1 as ::core::ffi::c_int
        as usize][(scan8[13 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as usize];
    (*sl)
        .ref_cache[1 as ::core::ffi::c_int
        as usize][(scan8[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as usize] = (*sl)
        .ref_cache[1 as ::core::ffi::c_int
        as usize][(scan8[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as usize];
    (*sl)
        .ref_cache[0 as ::core::ffi::c_int
        as usize][(scan8[13 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as usize] = (*sl)
        .ref_cache[1 as ::core::ffi::c_int
        as usize][(scan8[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as usize];
    (*sl)
        .ref_cache[0 as ::core::ffi::c_int
        as usize][(scan8[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as usize] = (*sl)
        .ref_cache[0 as ::core::ffi::c_int
        as usize][(scan8[13 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as usize];
    (*sl)
        .ref_cache[0 as ::core::ffi::c_int
        as usize][(scan8[5 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as usize] = (*sl)
        .ref_cache[0 as ::core::ffi::c_int
        as usize][(scan8[7 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as usize];
    (*sl).er = &raw mut (*h).er;
}
unsafe extern "C" fn h264_init_pic(mut pic: *mut H264Picture) -> ::core::ffi::c_int {
    (*pic).f = av_frame_alloc();
    if (*pic).f.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*pic).f_grain = av_frame_alloc();
    if (*pic).f_grain.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn skip_bits_long(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) {
    (*s).index += n;
}
unsafe extern "C" fn h264_init_context(
    mut avctx: *mut AVCodecContext,
    mut h: *mut H264Context,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    (*h).avctx = avctx;
    (*h).cur_chroma_format_idc = -(1 as ::core::ffi::c_int);
    (*h).width_from_caller = (*avctx).width;
    (*h).height_from_caller = (*avctx).height;
    (*h).workaround_bugs = (*avctx).workaround_bugs;
    (*h).flags = (*avctx).flags;
    (*h).poc.prev_poc_msb = (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int;
    (*h).recovery_frame = -(1 as ::core::ffi::c_int);
    (*h).frame_recovered = 0 as ::core::ffi::c_int;
    (*h).poc.prev_frame_num = -(1 as ::core::ffi::c_int);
    (*h).sei.common.frame_packing.arrangement_cancel_flag = -(1 as ::core::ffi::c_int);
    (*h).sei.common.unregistered.x264_build = -(1 as ::core::ffi::c_int);
    (*h).next_outputed_poc = -(2147483647 as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[::core::ffi::c_int; 16]>() as usize)
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
    {
        (*h).last_pocs[i as usize] = -(2147483647 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int;
        i += 1;
    }
    ff_h264_sei_uninit(&raw mut (*h).sei);
    if (*avctx).active_thread_type & 1 as ::core::ffi::c_int != 0 {
        (*h).decode_error_flags_pool = av_refstruct_pool_alloc(
            ::core::mem::size_of::<atomic_int>() as size_t,
            0 as ::core::ffi::c_uint,
        ) as *mut AVRefStructPool;
        if (*h).decode_error_flags_pool.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
    }
    (*h).nb_slice_ctx = if (*avctx).active_thread_type & 2 as ::core::ffi::c_int != 0 {
        (*avctx).thread_count
    } else {
        1 as ::core::ffi::c_int
    };
    (*h).slice_ctx = av_calloc(
        (*h).nb_slice_ctx as size_t,
        ::core::mem::size_of::<H264SliceContext>() as size_t,
    ) as *mut H264SliceContext;
    if (*h).slice_ctx.is_null() {
        (*h).nb_slice_ctx = 0 as ::core::ffi::c_int;
        return -(12 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while i < 36 as ::core::ffi::c_int {
        ret = h264_init_pic(
            (&raw mut (*h).DPB as *mut H264Picture).offset(i as isize)
                as *mut H264Picture,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        i += 1;
    }
    ret = h264_init_pic(&raw mut (*h).cur_pic);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    ret = h264_init_pic(&raw mut (*h).last_pic_for_ec);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*h).nb_slice_ctx {
        let ref mut fresh4 = (*(*h).slice_ctx.offset(i as isize)).h264;
        *fresh4 = h;
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn av_refstruct_pool_uninit(mut poolp: *mut *mut AVRefStructPool) {
    av_refstruct_unref(poolp as *mut ::core::ffi::c_void);
}
#[inline]
unsafe extern "C" fn get_bits(
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
    re_index = re_index.wrapping_add(n as ::core::ffi::c_uint);
    (*s).index = re_index as ::core::ffi::c_int;
    return tmp;
}
unsafe extern "C" fn h264_free_pic(mut h: *mut H264Context, mut pic: *mut H264Picture) {
    ff_h264_unref_picture(pic);
    av_frame_free(&raw mut (*pic).f);
    av_frame_free(&raw mut (*pic).f_grain);
}
#[cold]
unsafe extern "C" fn h264_decode_end(
    mut avctx: *mut AVCodecContext,
) -> ::core::ffi::c_int {
    let mut h: *mut H264Context = (*avctx).priv_data as *mut H264Context;
    let mut i: ::core::ffi::c_int = 0;
    ff_h264_remove_all_refs(h);
    ff_h264_free_tables(h);
    i = 0 as ::core::ffi::c_int;
    while i < 36 as ::core::ffi::c_int {
        h264_free_pic(
            h,
            (&raw mut (*h).DPB as *mut H264Picture).offset(i as isize)
                as *mut H264Picture,
        );
        i += 1;
    }
    memset(
        &raw mut (*h).delayed_pic as *mut *mut H264Picture as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[*mut H264Picture; 18]>() as size_t,
    );
    (*h).cur_pic_ptr = ::core::ptr::null_mut::<H264Picture>();
    av_refstruct_pool_uninit(&raw mut (*h).decode_error_flags_pool);
    av_freep(&raw mut (*h).slice_ctx as *mut ::core::ffi::c_void);
    (*h).nb_slice_ctx = 0 as ::core::ffi::c_int;
    ff_h264_sei_uninit(&raw mut (*h).sei);
    ff_h264_ps_uninit(&raw mut (*h).ps);
    ff_h2645_packet_uninit(&raw mut (*h).pkt);
    h264_free_pic(h, &raw mut (*h).cur_pic);
    h264_free_pic(h, &raw mut (*h).last_pic_for_ec);
    return 0 as ::core::ffi::c_int;
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
static mut h264_vlc_init: pthread_once_t = 0 as pthread_once_t;
#[cold]
unsafe extern "C" fn h264_decode_init(
    mut avctx: *mut AVCodecContext,
) -> ::core::ffi::c_int {
    let mut h: *mut H264Context = (*avctx).priv_data as *mut H264Context;
    let mut ret: ::core::ffi::c_int = 0;
    ret = h264_init_context(avctx, h);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    ret = pthread_once(
        &raw mut h264_vlc_init,
        Some(ff_h264_decode_init_vlc as unsafe extern "C" fn() -> ()),
    );
    if ret != 0 as ::core::ffi::c_int {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"pthread_once has failed.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -((('U' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('K' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('N' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*(*avctx).internal).is_copy == 0 {
        if (*avctx).extradata_size > 0 as ::core::ffi::c_int
            && !(*avctx).extradata.is_null()
        {
            ret = ff_h264_decode_extradata(
                (*avctx).extradata,
                (*avctx).extradata_size,
                &raw mut (*h).ps,
                &raw mut (*h).is_avc,
                &raw mut (*h).nal_length_size,
                (*avctx).err_recognition,
                avctx as *mut ::core::ffi::c_void,
            );
            if ret < 0 as ::core::ffi::c_int {
                let mut explode: ::core::ffi::c_int = (*avctx).err_recognition
                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
                av_log(
                    avctx as *mut ::core::ffi::c_void,
                    if explode != 0 {
                        16 as ::core::ffi::c_int
                    } else {
                        24 as ::core::ffi::c_int
                    },
                    b"Error decoding the extradata\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                if explode != 0 {
                    return ret;
                }
                ret = 0 as ::core::ffi::c_int;
            }
        }
    }
    if !(*h).ps.sps.is_null() && (*(*h).ps.sps).bitstream_restriction_flag != 0
        && (*(*h).avctx).has_b_frames < (*(*h).ps.sps).num_reorder_frames
    {
        (*(*h).avctx).has_b_frames = (*(*h).ps.sps).num_reorder_frames;
    }
    ff_h264_flush_change(h);
    if (*h).enable_er < 0 as ::core::ffi::c_int
        && (*avctx).active_thread_type & 2 as ::core::ffi::c_int != 0
    {
        (*h).enable_er = 0 as ::core::ffi::c_int;
    }
    if (*h).enable_er != 0 && (*avctx).active_thread_type & 2 as ::core::ffi::c_int != 0
    {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            24 as ::core::ffi::c_int,
            b"Error resilience with slice threads is enabled. It is unsafe and unsupported and may crash. Use it at your own risk\n\0"
                as *const u8 as *const ::core::ffi::c_char,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn get_bits_long(
    mut s: *mut GetBitContext,
    mut n: ::core::ffi::c_int,
) -> ::core::ffi::c_uint {
    if n == 0 {
        return 0 as ::core::ffi::c_uint
    } else if (1 as ::core::ffi::c_int == 0 || 0 != 0) && n <= 25 as ::core::ffi::c_int {
        return get_bits(s, n)
    } else {
        let mut tmp: ::core::ffi::c_uint = 0;
        let mut re_index: ::core::ffi::c_uint = (*s).index as ::core::ffi::c_uint;
        let mut re_cache: ::core::ffi::c_uint = 0;
        re_cache = (av_bswap64(
            (*((*s).buffer.offset((re_index >> 3 as ::core::ffi::c_int) as isize)
                as *const unaligned_64))
                .l,
        ) << (re_index & 7 as ::core::ffi::c_uint)
            >> 64 as ::core::ffi::c_int - 32 as ::core::ffi::c_int)
            as ::core::ffi::c_uint;
        tmp = (re_cache >> 32 as ::core::ffi::c_int - n) as ::core::ffi::c_uint;
        re_index = re_index.wrapping_add(n as ::core::ffi::c_uint);
        (*s).index = re_index as ::core::ffi::c_int;
        return tmp;
    };
}
unsafe extern "C" fn idr(mut h: *mut H264Context) {
    let mut i: ::core::ffi::c_int = 0;
    ff_h264_remove_all_refs(h);
    (*h).poc.prev_frame_num_offset = 0 as ::core::ffi::c_int;
    (*h).poc.prev_frame_num = (*h).poc.prev_frame_num_offset;
    (*h).poc.prev_poc_msb = (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int;
    (*h).poc.prev_poc_lsb = -(1 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[::core::ffi::c_int; 16]>() as usize)
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as usize)
    {
        (*h).last_pocs[i as usize] = -(2147483647 as ::core::ffi::c_int)
            - 1 as ::core::ffi::c_int;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ff_h264_flush_change(mut h: *mut H264Context) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    (*h).next_outputed_poc = -(2147483647 as ::core::ffi::c_int)
        - 1 as ::core::ffi::c_int;
    (*h).prev_interlaced_frame = 1 as ::core::ffi::c_int;
    idr(h);
    (*h).poc.prev_frame_num = -(1 as ::core::ffi::c_int);
    if !(*h).cur_pic_ptr.is_null() {
        (*(*h).cur_pic_ptr).reference = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        j = i;
        while !(*h).delayed_pic[i as usize].is_null() {
            if (*h).delayed_pic[i as usize] != (*h).cur_pic_ptr {
                let fresh3 = j;
                j = j + 1;
                (*h).delayed_pic[fresh3 as usize] = (*h).delayed_pic[i as usize];
            }
            i += 1;
        }
        (*h).delayed_pic[j as usize] = ::core::ptr::null_mut::<H264Picture>();
    }
    ff_h264_unref_picture(&raw mut (*h).last_pic_for_ec);
    (*h).first_field = 0 as ::core::ffi::c_int;
    (*h).recovery_frame = -(1 as ::core::ffi::c_int);
    (*h).frame_recovered = 0 as ::core::ffi::c_int;
    (*h).current_slice = 0 as ::core::ffi::c_int;
    (*h).mmco_reset = 1 as ::core::ffi::c_int;
}
#[cold]
unsafe extern "C" fn h264_decode_flush(mut avctx: *mut AVCodecContext) {
    let mut h: *mut H264Context = (*avctx).priv_data as *mut H264Context;
    let mut i: ::core::ffi::c_int = 0;
    memset(
        &raw mut (*h).delayed_pic as *mut *mut H264Picture as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[*mut H264Picture; 18]>() as size_t,
    );
    ff_h264_flush_change(h);
    ff_h264_sei_uninit(&raw mut (*h).sei);
    i = 0 as ::core::ffi::c_int;
    while i < 36 as ::core::ffi::c_int {
        ff_h264_unref_picture(
            (&raw mut (*h).DPB as *mut H264Picture).offset(i as isize)
                as *mut H264Picture,
        );
        i += 1;
    }
    (*h).cur_pic_ptr = ::core::ptr::null_mut::<H264Picture>();
    ff_h264_unref_picture(&raw mut (*h).cur_pic);
    (*h).mb_y = 0 as ::core::ffi::c_int;
    (*h).non_gray = 0 as ::core::ffi::c_int;
    ff_h264_free_tables(h);
    (*h).context_initialized = 0 as ::core::ffi::c_int;
    if !(*avctx).hwaccel.is_null()
        && (*ffhwaccel((*avctx).hwaccel as *const AVHWAccel)).flush.is_some()
    {
        (*ffhwaccel((*avctx).hwaccel as *const AVHWAccel))
            .flush
            .expect("non-null function pointer")(avctx);
    }
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
unsafe extern "C" fn get_last_needed_nal(mut h: *mut H264Context) -> ::core::ffi::c_int {
    let mut nals_needed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut slice_type: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut picture_intra_only: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut first_slice: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*h).pkt.nb_nals {
        let mut nal: *mut H2645NAL = (*h).pkt.nals.offset(i as isize) as *mut H2645NAL;
        let mut gb: GetBitContext = GetBitContext {
            buffer: ::core::ptr::null::<uint8_t>(),
            index: 0,
            size_in_bits: 0,
            size_in_bits_plus8: 0,
        };
        match (*nal).type_0 {
            7 | 8 => {
                nals_needed = i;
            }
            2 | 5 | 1 => {
                ret = init_get_bits8(
                    &raw mut gb,
                    (*nal).data.offset(1 as ::core::ffi::c_int as isize),
                    (*nal).size - 1 as ::core::ffi::c_int,
                );
                if ret < 0 as ::core::ffi::c_int {
                    av_log(
                        (*h).avctx as *mut ::core::ffi::c_void,
                        16 as ::core::ffi::c_int,
                        b"Invalid zero-sized VCL NAL unit\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                    );
                    if (*(*h).avctx).err_recognition
                        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                    {
                        return ret;
                    }
                } else {
                    if get_ue_golomb_long(&raw mut gb) == 0 || first_slice == 0
                        || first_slice != (*nal).type_0
                    {
                        nals_needed = i;
                    }
                    slice_type = get_ue_golomb_31(&raw mut gb);
                    if slice_type > 9 as ::core::ffi::c_int {
                        slice_type = 0 as ::core::ffi::c_int;
                    }
                    if slice_type > 4 as ::core::ffi::c_int {
                        slice_type -= 5 as ::core::ffi::c_int;
                    }
                    slice_type = ff_h264_golomb_to_pict_type[slice_type as usize]
                        as ::core::ffi::c_int;
                    picture_intra_only
                        &= (slice_type & 3 as ::core::ffi::c_int
                            == AV_PICTURE_TYPE_I as ::core::ffi::c_int)
                            as ::core::ffi::c_int;
                    if first_slice == 0 {
                        first_slice = (*nal).type_0;
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    (*h).picture_intra_only = picture_intra_only;
    return nals_needed;
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
#[inline]
unsafe extern "C" fn init_get_bits8(
    mut s: *mut GetBitContext,
    mut buffer: *const uint8_t,
    mut byte_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if byte_size > 2147483647 as ::core::ffi::c_int / 8 as ::core::ffi::c_int
        || byte_size < 0 as ::core::ffi::c_int
    {
        byte_size = -(1 as ::core::ffi::c_int);
    }
    return init_get_bits(s, buffer, byte_size * 8 as ::core::ffi::c_int);
}
unsafe extern "C" fn debug_green_metadata(
    mut gm: *const H264SEIGreenMetaData,
    mut logctx: *mut ::core::ffi::c_void,
) {
    av_log(
        logctx,
        48 as ::core::ffi::c_int,
        b"Green Metadata Info SEI message\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    av_log(
        logctx,
        48 as ::core::ffi::c_int,
        b"  green_metadata_type: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        (*gm).green_metadata_type as ::core::ffi::c_int,
    );
    if (*gm).green_metadata_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        av_log(
            logctx,
            48 as ::core::ffi::c_int,
            b"  green_metadata_period_type: %d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*gm).period_type as ::core::ffi::c_int,
        );
        if (*gm).period_type as ::core::ffi::c_int == 2 as ::core::ffi::c_int {
            av_log(
                logctx,
                48 as ::core::ffi::c_int,
                b"  green_metadata_num_seconds: %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*gm).num_seconds as ::core::ffi::c_int,
            );
        } else if (*gm).period_type as ::core::ffi::c_int == 3 as ::core::ffi::c_int {
            av_log(
                logctx,
                48 as ::core::ffi::c_int,
                b"  green_metadata_num_pictures: %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*gm).num_pictures as ::core::ffi::c_int,
            );
        }
        av_log(
            logctx,
            48 as ::core::ffi::c_int,
            b"  SEI GREEN Complexity Metrics: %f %f %f %f\n\0" as *const u8
                as *const ::core::ffi::c_char,
            ((*gm).percent_non_zero_macroblocks as ::core::ffi::c_float
                / 255 as ::core::ffi::c_int as ::core::ffi::c_float)
                as ::core::ffi::c_double,
            ((*gm).percent_intra_coded_macroblocks as ::core::ffi::c_float
                / 255 as ::core::ffi::c_int as ::core::ffi::c_float)
                as ::core::ffi::c_double,
            ((*gm).percent_six_tap_filtering as ::core::ffi::c_float
                / 255 as ::core::ffi::c_int as ::core::ffi::c_float)
                as ::core::ffi::c_double,
            ((*gm).percent_alpha_point_deblocking_instance as ::core::ffi::c_float
                / 255 as ::core::ffi::c_int as ::core::ffi::c_float)
                as ::core::ffi::c_double,
        );
    } else if (*gm).green_metadata_type as ::core::ffi::c_int == 1 as ::core::ffi::c_int
    {
        av_log(
            logctx,
            48 as ::core::ffi::c_int,
            b"  xsd_metric_type: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
            (*gm).xsd_metric_type as ::core::ffi::c_int,
        );
        if (*gm).xsd_metric_type as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            av_log(
                logctx,
                48 as ::core::ffi::c_int,
                b"  xsd_metric_value: %f\n\0" as *const u8 as *const ::core::ffi::c_char,
                ((*gm).xsd_metric_value as ::core::ffi::c_float
                    / 100 as ::core::ffi::c_int as ::core::ffi::c_float)
                    as ::core::ffi::c_double,
            );
        }
    }
}
unsafe extern "C" fn decode_nal_units(
    mut h: *mut H264Context,
    mut buf_ref: *mut AVBufferRef,
    mut buf: *const uint8_t,
    mut buf_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let avctx: *mut AVCodecContext = (*h).avctx;
    let mut nals_needed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut idr_cleared: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*h).has_slice = 0 as ::core::ffi::c_int;
    (*h).nal_unit_type = 0 as ::core::ffi::c_int;
    if (*avctx).flags2 & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int == 0 {
        (*h).current_slice = 0 as ::core::ffi::c_int;
        if (*h).first_field == 0 {
            (*h).cur_pic_ptr = ::core::ptr::null_mut::<H264Picture>();
            ff_h264_sei_uninit(&raw mut (*h).sei);
        }
    }
    if (*h).nal_length_size == 4 as ::core::ffi::c_int {
        if buf_size > 8 as ::core::ffi::c_int
            && av_bswap32((*(buf as *const unaligned_32)).l) == 1 as uint32_t
            && av_bswap32(
                (*(buf.offset(5 as ::core::ffi::c_int as isize) as *const unaligned_32))
                    .l,
            ) > buf_size as ::core::ffi::c_uint
        {
            (*h).is_avc = 0 as ::core::ffi::c_int;
        } else if buf_size > 3 as ::core::ffi::c_int
            && av_bswap32((*(buf as *const unaligned_32)).l) > 1 as uint32_t
            && av_bswap32((*(buf as *const unaligned_32)).l)
                <= buf_size as ::core::ffi::c_uint
        {
            (*h).is_avc = 1 as ::core::ffi::c_int;
        }
    }
    ret = ff_h2645_packet_split(
        &raw mut (*h).pkt,
        buf,
        buf_size,
        avctx as *mut ::core::ffi::c_void,
        (*h).nal_length_size,
        (*avctx).codec_id,
        ((*h).is_avc != 0) as ::core::ffi::c_int
            * H2645_FLAG_IS_NALFF as ::core::ffi::c_int,
    );
    if ret < 0 as ::core::ffi::c_int {
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"Error splitting the input into NAL units.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return ret;
    }
    if (*avctx).active_thread_type & 1 as ::core::ffi::c_int != 0 {
        nals_needed = get_last_needed_nal(h);
    }
    if nals_needed < 0 as ::core::ffi::c_int {
        return nals_needed;
    }
    i = 0 as ::core::ffi::c_int;
    loop {
        if !(i < (*h).pkt.nb_nals) {
            current_block = 726525485109251713;
            break;
        }
        let mut nal: *mut H2645NAL = (*h).pkt.nals.offset(i as isize) as *mut H2645NAL;
        let mut max_slice_ctx: ::core::ffi::c_int = 0;
        let mut err: ::core::ffi::c_int = 0;
        if !((*avctx).skip_frame as ::core::ffi::c_int
            >= AVDISCARD_NONREF as ::core::ffi::c_int
            && (*nal).ref_idc == 0 as ::core::ffi::c_int
            && (*nal).type_0 != H264_NAL_SEI as ::core::ffi::c_int)
        {
            (*h).nal_ref_idc = (*nal).ref_idc;
            (*h).nal_unit_type = (*nal).type_0;
            err = 0 as ::core::ffi::c_int;
            match (*nal).type_0 {
                5 => {
                    if *(*nal).data.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int & 0xfc as ::core::ffi::c_int
                        == 0x98 as ::core::ffi::c_int
                    {
                        av_log(
                            (*h).avctx as *mut ::core::ffi::c_void,
                            16 as ::core::ffi::c_int,
                            b"Invalid inter IDR frame\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                        );
                        (*h).next_outputed_poc = -(2147483647 as ::core::ffi::c_int)
                            - 1 as ::core::ffi::c_int;
                        ret = -(1 as ::core::ffi::c_int);
                        current_block = 10068863416881715265;
                        break;
                    } else {
                        if idr_cleared == 0 {
                            idr(h);
                        }
                        idr_cleared = 1 as ::core::ffi::c_int;
                        (*h).has_recovery_point = 1 as ::core::ffi::c_int;
                    }
                    current_block = 17564531375414694319;
                }
                1 => {
                    current_block = 17564531375414694319;
                }
                2 | 3 | 4 => {
                    avpriv_request_sample(
                        avctx as *mut ::core::ffi::c_void,
                        b"data partitioning\0" as *const u8 as *const ::core::ffi::c_char,
                    );
                    current_block = 919954187481050311;
                }
                6 => {
                    if (*h).setup_finished != 0 {
                        avpriv_request_sample(
                            avctx as *mut ::core::ffi::c_void,
                            b"Late SEI\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                    } else {
                        ret = ff_h264_sei_decode(
                            &raw mut (*h).sei,
                            &raw mut (*nal).gb,
                            &raw mut (*h).ps,
                            avctx as *mut ::core::ffi::c_void,
                        );
                        (*h).has_recovery_point = ((*h).has_recovery_point != 0
                            || (*h).sei.recovery_point.recovery_frame_cnt
                                != -(1 as ::core::ffi::c_int)) as ::core::ffi::c_int;
                        if (*avctx).debug & 0x800000 as ::core::ffi::c_int != 0 {
                            debug_green_metadata(
                                &raw mut (*h).sei.green_metadata,
                                (*h).avctx as *mut ::core::ffi::c_void,
                            );
                        }
                        if ret < 0 as ::core::ffi::c_int
                            && (*(*h).avctx).err_recognition
                                & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                        {
                            current_block = 10068863416881715265;
                            break;
                        }
                    }
                    current_block = 919954187481050311;
                }
                7 => {
                    let mut tmp_gb: GetBitContext = (*nal).gb;
                    if !(*avctx).hwaccel.is_null()
                        && (*ffhwaccel((*avctx).hwaccel as *const AVHWAccel))
                            .decode_params
                            .is_some()
                    {
                        ret = (*ffhwaccel((*avctx).hwaccel as *const AVHWAccel))
                            .decode_params
                            .expect(
                                "non-null function pointer",
                            )(
                            avctx,
                            (*nal).type_0,
                            (*nal).raw_data,
                            (*nal).raw_size as uint32_t,
                        );
                        if ret < 0 as ::core::ffi::c_int {
                            current_block = 10068863416881715265;
                            break;
                        }
                    }
                    if ff_h264_decode_seq_parameter_set(
                        &raw mut tmp_gb,
                        avctx,
                        &raw mut (*h).ps,
                        0 as ::core::ffi::c_int,
                    ) >= 0 as ::core::ffi::c_int
                    {
                        current_block = 919954187481050311;
                    } else {
                        av_log(
                            (*h).avctx as *mut ::core::ffi::c_void,
                            48 as ::core::ffi::c_int,
                            b"SPS decoding failure, trying again with the complete NAL\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                        init_get_bits8(
                            &raw mut tmp_gb,
                            (*nal).raw_data.offset(1 as ::core::ffi::c_int as isize),
                            (*nal).raw_size - 1 as ::core::ffi::c_int,
                        );
                        if ff_h264_decode_seq_parameter_set(
                            &raw mut tmp_gb,
                            avctx,
                            &raw mut (*h).ps,
                            0 as ::core::ffi::c_int,
                        ) >= 0 as ::core::ffi::c_int
                        {
                            current_block = 919954187481050311;
                        } else {
                            ff_h264_decode_seq_parameter_set(
                                &raw mut (*nal).gb,
                                avctx,
                                &raw mut (*h).ps,
                                1 as ::core::ffi::c_int,
                            );
                            current_block = 919954187481050311;
                        }
                    }
                }
                8 => {
                    if !(*avctx).hwaccel.is_null()
                        && (*ffhwaccel((*avctx).hwaccel as *const AVHWAccel))
                            .decode_params
                            .is_some()
                    {
                        ret = (*ffhwaccel((*avctx).hwaccel as *const AVHWAccel))
                            .decode_params
                            .expect(
                                "non-null function pointer",
                            )(
                            avctx,
                            (*nal).type_0,
                            (*nal).raw_data,
                            (*nal).raw_size as uint32_t,
                        );
                        if ret < 0 as ::core::ffi::c_int {
                            current_block = 10068863416881715265;
                            break;
                        }
                    }
                    ret = ff_h264_decode_picture_parameter_set(
                        &raw mut (*nal).gb,
                        avctx,
                        &raw mut (*h).ps,
                        (*nal).size_bits,
                    );
                    if ret < 0 as ::core::ffi::c_int
                        && (*(*h).avctx).err_recognition
                            & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                    {
                        current_block = 10068863416881715265;
                        break;
                    }
                    current_block = 919954187481050311;
                }
                9 | 10 | 11 | 12 | 13 | 19 => {
                    current_block = 919954187481050311;
                }
                _ => {
                    av_log(
                        avctx as *mut ::core::ffi::c_void,
                        48 as ::core::ffi::c_int,
                        b"Unknown NAL code: %d (%d bits)\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        (*nal).type_0,
                        (*nal).size_bits,
                    );
                    current_block = 919954187481050311;
                }
            }
            match current_block {
                17564531375414694319 => {
                    (*h).has_slice = 1 as ::core::ffi::c_int;
                    err = ff_h264_queue_decode_slice(h, nal);
                    if err != 0 {
                        let mut sl: *mut H264SliceContext = (*h)
                            .slice_ctx
                            .offset((*h).nb_slice_ctx_queued as isize);
                        (*sl).ref_count[1 as ::core::ffi::c_int as usize] = 0
                            as ::core::ffi::c_uint;
                        (*sl).ref_count[0 as ::core::ffi::c_int as usize] = (*sl)
                            .ref_count[1 as ::core::ffi::c_int as usize];
                    } else {
                        if (*h).current_slice == 1 as ::core::ffi::c_int {
                            if (*avctx).active_thread_type & 1 as ::core::ffi::c_int != 0
                                && i >= nals_needed && (*h).setup_finished == 0
                                && !(*h).cur_pic_ptr.is_null()
                            {
                                ff_thread_finish_setup(avctx);
                                (*h).setup_finished = 1 as ::core::ffi::c_int;
                            }
                            if !(*(*h).avctx).hwaccel.is_null()
                                && {
                                    ret = (*ffhwaccel(
                                        (*(*h).avctx).hwaccel as *const AVHWAccel,
                                    ))
                                        .start_frame
                                        .expect(
                                            "non-null function pointer",
                                        )((*h).avctx, buf_ref, buf, buf_size as uint32_t);
                                    ret < 0 as ::core::ffi::c_int
                                }
                            {
                                current_block = 10068863416881715265;
                                break;
                            }
                        }
                        max_slice_ctx = if !(*avctx).hwaccel.is_null() {
                            1 as ::core::ffi::c_int
                        } else {
                            (*h).nb_slice_ctx
                        };
                        if (*h).nb_slice_ctx_queued == max_slice_ctx {
                            if !(*(*h).avctx).hwaccel.is_null() {
                                ret = (*ffhwaccel((*avctx).hwaccel as *const AVHWAccel))
                                    .decode_slice
                                    .expect(
                                        "non-null function pointer",
                                    )(avctx, (*nal).raw_data, (*nal).raw_size as uint32_t);
                                (*h).nb_slice_ctx_queued = 0 as ::core::ffi::c_int;
                            } else {
                                ret = ff_h264_execute_decode_slices(h);
                            }
                            if ret < 0 as ::core::ffi::c_int
                                && (*(*h).avctx).err_recognition
                                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                            {
                                current_block = 10068863416881715265;
                                break;
                            }
                        }
                    }
                }
                _ => {}
            }
            if err < 0 as ::core::ffi::c_int
                && (*(*h).avctx).err_recognition
                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
            {
                av_log(
                    (*h).avctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"decode_slice_header error\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                );
                ret = err;
                current_block = 10068863416881715265;
                break;
            }
        }
        i += 1;
    }
    match current_block {
        726525485109251713 => {
            ret = ff_h264_execute_decode_slices(h);
            if !(ret < 0 as ::core::ffi::c_int
                && (*(*h).avctx).err_recognition
                    & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0)
            {
                if (ret < 0 as ::core::ffi::c_int || (*h).er.error_occurred != 0)
                    && !(*h).cur_pic_ptr.is_null()
                {
                    if !(*(*h).cur_pic_ptr).decode_error_flags.is_null() {
                        let mut decode_error: *mut atomic_int = (*(*h).cur_pic_ptr)
                            .decode_error_flags;
                        crate::rt::atomic_or_relaxed(
                            decode_error,
                            8 as ::core::ffi::c_int,
                        );
                    } else {
                        (*(*(*h).cur_pic_ptr).f).decode_error_flags
                            |= 8 as ::core::ffi::c_int;
                    }
                }
                ret = 0 as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    if !((*h).picture_structure != 3 as ::core::ffi::c_int) && (*h).current_slice != 0
        && (*h).enable_er != 0
    {
        let mut sl_0: *mut H264SliceContext = (*h).slice_ctx;
        let mut use_last_pic: ::core::ffi::c_int = (!(*(*h).last_pic_for_ec.f)
            .buf[0 as ::core::ffi::c_int as usize]
            .is_null() && (*sl_0).ref_count[0 as ::core::ffi::c_int as usize] == 0)
            as ::core::ffi::c_int;
        let mut decode_error_flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        ff_h264_set_erpic(&raw mut (*h).er.cur_pic, (*h).cur_pic_ptr);
        if use_last_pic != 0 {
            ff_h264_set_erpic(&raw mut (*h).er.last_pic, &raw mut (*h).last_pic_for_ec);
            (*sl_0)
                .ref_list[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize]
                .parent = &raw mut (*h).last_pic_for_ec;
            memcpy(
                &raw mut (*(&raw mut *(&raw mut (*sl_0).ref_list as *mut [H264Ref; 48])
                    .offset(0 as ::core::ffi::c_int as isize) as *mut H264Ref)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .data as *mut *mut uint8_t as *mut ::core::ffi::c_void,
                &raw mut (*(*h).last_pic_for_ec.f).data as *mut *mut uint8_t
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[*mut uint8_t; 3]>() as size_t,
            );
            memcpy(
                &raw mut (*(&raw mut *(&raw mut (*sl_0).ref_list as *mut [H264Ref; 48])
                    .offset(0 as ::core::ffi::c_int as isize) as *mut H264Ref)
                    .offset(0 as ::core::ffi::c_int as isize))
                    .linesize as *mut ::core::ffi::c_int as *mut ::core::ffi::c_void,
                &raw mut (*(*h).last_pic_for_ec.f).linesize as *mut ::core::ffi::c_int
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_int; 3]>() as size_t,
            );
            (*sl_0)
                .ref_list[0 as ::core::ffi::c_int
                    as usize][0 as ::core::ffi::c_int as usize]
                .reference = (*h).last_pic_for_ec.reference;
        } else if (*sl_0).ref_count[0 as ::core::ffi::c_int as usize] != 0 {
            ff_h264_set_erpic(
                &raw mut (*h).er.last_pic,
                (*sl_0)
                    .ref_list[0 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize]
                    .parent,
            );
        } else {
            ff_h264_set_erpic(
                &raw mut (*h).er.last_pic,
                ::core::ptr::null::<H264Picture>(),
            );
        }
        if (*sl_0).ref_count[1 as ::core::ffi::c_int as usize] != 0 {
            ff_h264_set_erpic(
                &raw mut (*h).er.next_pic,
                (*sl_0)
                    .ref_list[1 as ::core::ffi::c_int
                        as usize][0 as ::core::ffi::c_int as usize]
                    .parent,
            );
        }
        ff_er_frame_end(&raw mut (*h).er, &raw mut decode_error_flags);
        if decode_error_flags != 0 {
            if !(*(*h).cur_pic_ptr).decode_error_flags.is_null() {
                let mut decode_error_0: *mut atomic_int = (*(*h).cur_pic_ptr)
                    .decode_error_flags;
                crate::rt::atomic_or_relaxed(
                    decode_error_0,
                    decode_error_flags,
                );
            } else {
                (*(*(*h).cur_pic_ptr).f).decode_error_flags |= decode_error_flags;
            }
        }
        if use_last_pic != 0 {
            memset(
                (&raw mut *(&raw mut (*sl_0).ref_list as *mut [H264Ref; 48])
                    .offset(0 as ::core::ffi::c_int as isize) as *mut H264Ref)
                    .offset(0 as ::core::ffi::c_int as isize) as *mut H264Ref
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<H264Ref>() as size_t,
            );
        }
    }
    if !(*h).cur_pic_ptr.is_null() && (*h).droppable == 0 && (*h).has_slice != 0 {
        ff_thread_report_progress(
            &raw mut (*(*h).cur_pic_ptr).tf,
            2147483647 as ::core::ffi::c_int,
            ((*h).picture_structure == 2 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
    }
    return if ret < 0 as ::core::ffi::c_int { ret } else { buf_size };
}
unsafe extern "C" fn h264_export_enc_params(
    mut f: *mut AVFrame,
    mut p: *const H264Picture,
) -> ::core::ffi::c_int {
    let mut par: *mut AVVideoEncParams = ::core::ptr::null_mut::<AVVideoEncParams>();
    let mut nb_mb: ::core::ffi::c_uint = ((*p).mb_height * (*p).mb_width)
        as ::core::ffi::c_uint;
    let mut x: ::core::ffi::c_uint = 0;
    let mut y: ::core::ffi::c_uint = 0;
    par = av_video_enc_params_create_side_data(f, AV_VIDEO_ENC_PARAMS_H264, nb_mb);
    if par.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    (*par).qp = (*(*p).pps).init_qp as int32_t;
    (*par)
        .delta_qp[1 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] = (*(*p)
        .pps)
        .chroma_qp_index_offset[0 as ::core::ffi::c_int as usize] as int32_t;
    (*par)
        .delta_qp[1 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize] = (*(*p)
        .pps)
        .chroma_qp_index_offset[0 as ::core::ffi::c_int as usize] as int32_t;
    (*par)
        .delta_qp[2 as ::core::ffi::c_int as usize][0 as ::core::ffi::c_int as usize] = (*(*p)
        .pps)
        .chroma_qp_index_offset[1 as ::core::ffi::c_int as usize] as int32_t;
    (*par)
        .delta_qp[2 as ::core::ffi::c_int as usize][1 as ::core::ffi::c_int as usize] = (*(*p)
        .pps)
        .chroma_qp_index_offset[1 as ::core::ffi::c_int as usize] as int32_t;
    y = 0 as ::core::ffi::c_uint;
    while y < (*p).mb_height as ::core::ffi::c_uint {
        x = 0 as ::core::ffi::c_uint;
        while x < (*p).mb_width as ::core::ffi::c_uint {
            let block_idx: ::core::ffi::c_uint = y
                .wrapping_mul((*p).mb_width as ::core::ffi::c_uint)
                .wrapping_add(x);
            let mb_xy: ::core::ffi::c_uint = y
                .wrapping_mul((*p).mb_stride as ::core::ffi::c_uint)
                .wrapping_add(x);
            let mut b: *mut AVVideoBlockParams = av_video_enc_params_block(
                par,
                block_idx,
            );
            (*b).src_x = x.wrapping_mul(16 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            (*b).src_y = y.wrapping_mul(16 as ::core::ffi::c_uint) as ::core::ffi::c_int;
            (*b).w = 16 as ::core::ffi::c_int;
            (*b).h = 16 as ::core::ffi::c_int;
            (*b).delta_qp = *(*p).qscale_table.offset(mb_xy as isize) as int32_t
                - (*par).qp;
            x = x.wrapping_add(1);
        }
        y = y.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn output_frame(
    mut h: *mut H264Context,
    mut dst: *mut AVFrame,
    mut srcp: *mut H264Picture,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    ret = av_frame_ref(
        dst,
        if (*srcp).needs_fg != 0 { (*srcp).f_grain } else { (*srcp).f },
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*srcp).needs_fg != 0
        && {
            ret = av_frame_copy_props(dst, (*srcp).f);
            ret < 0 as ::core::ffi::c_int
        }
    {
        return ret;
    }
    if !(*srcp).decode_error_flags.is_null() {
        let mut decode_error: *mut atomic_int = (*srcp).decode_error_flags;
        (*dst).decode_error_flags
            |= crate::rt::atomic_load_relaxed(decode_error);
    }
    av_dict_set(
        &raw mut (*dst).metadata,
        b"stereo_mode\0" as *const u8 as *const ::core::ffi::c_char,
        ff_h264_sei_stereo_mode(&raw mut (*h).sei.common.frame_packing),
        0 as ::core::ffi::c_int,
    );
    if (*srcp).sei_recovery_frame_cnt == 0 as ::core::ffi::c_int {
        (*dst).flags |= (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
    }
    if (*(*h).avctx).export_side_data
        & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int != 0
    {
        ret = h264_export_enc_params(dst, srcp);
        if ret < 0 as ::core::ffi::c_int {
            av_frame_unref(dst);
            return ret;
        }
    }
    if (*(*h).avctx).export_side_data
        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int == 0
    {
        av_frame_remove_side_data(dst, AV_FRAME_DATA_FILM_GRAIN_PARAMS);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn is_avcc_extradata(
    mut buf: *const uint8_t,
    mut buf_size: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut cnt: ::core::ffi::c_int = *buf.offset(5 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int;
    let mut p: *const uint8_t = buf.offset(6 as ::core::ffi::c_int as isize);
    if cnt == 0 {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        let fresh5 = cnt;
        cnt = cnt - 1;
        if !(fresh5 != 0) {
            break;
        }
        let mut nalsize: ::core::ffi::c_int = av_bswap16((*(p as *const unaligned_16)).l)
            as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
        if nalsize as ::core::ffi::c_long
            > buf_size as ::core::ffi::c_long - p.offset_from(buf) as ::core::ffi::c_long
            || *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x9f as ::core::ffi::c_int != 7 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        p = p.offset(nalsize as isize);
    }
    let fresh6 = p;
    p = p.offset(1);
    cnt = *fresh6 as ::core::ffi::c_int;
    if cnt == 0 {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        let fresh7 = cnt;
        cnt = cnt - 1;
        if !(fresh7 != 0) {
            break;
        }
        let mut nalsize_0: ::core::ffi::c_int = av_bswap16(
            (*(p as *const unaligned_16)).l,
        ) as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
        if nalsize_0 as ::core::ffi::c_long
            > buf_size as ::core::ffi::c_long - p.offset_from(buf) as ::core::ffi::c_long
            || *p.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x9f as ::core::ffi::c_int != 8 as ::core::ffi::c_int
        {
            return 0 as ::core::ffi::c_int;
        }
        p = p.offset(nalsize_0 as isize);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn finalize_frame(
    mut h: *mut H264Context,
    mut dst: *mut AVFrame,
    mut out: *mut H264Picture,
    mut got_frame: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if (*(*h).avctx).flags & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
        || (*(*h).avctx).flags2 & (1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int
            != 0 || (*out).recovered != 0
    {
        if (*h).skip_gray > 0 as ::core::ffi::c_int && (*h).non_gray != 0
            && (*out).gray != 0
            && (*(*h).avctx).flags2
                & (1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if (*(*h).avctx).hwaccel.is_null()
            && ((*out).field_poc[0 as ::core::ffi::c_int as usize]
                == 2147483647 as ::core::ffi::c_int
                || (*out).field_poc[1 as ::core::ffi::c_int as usize]
                    == 2147483647 as ::core::ffi::c_int)
        {
            let mut p: ::core::ffi::c_int = 0;
            let mut f: *mut AVFrame = (*out).f;
            let mut field: ::core::ffi::c_int = ((*out)
                .field_poc[0 as ::core::ffi::c_int as usize]
                == 2147483647 as ::core::ffi::c_int) as ::core::ffi::c_int;
            let mut dst_data: [*mut uint8_t; 4] = [::core::ptr::null_mut::<
                uint8_t,
            >(); 4];
            let mut linesizes: [::core::ffi::c_int; 4] = [0; 4];
            let mut src_data: [*const uint8_t; 4] = [::core::ptr::null::<uint8_t>(); 4];
            av_log(
                (*h).avctx as *mut ::core::ffi::c_void,
                48 as ::core::ffi::c_int,
                b"Duplicating field %d to fill missing\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                field,
            );
            p = 0 as ::core::ffi::c_int;
            while p < 4 as ::core::ffi::c_int {
                dst_data[p as usize] = (*f)
                    .data[p as usize]
                    .offset(
                        ((field ^ 1 as ::core::ffi::c_int) * (*f).linesize[p as usize])
                            as isize,
                    );
                src_data[p as usize] = (*f)
                    .data[p as usize]
                    .offset((field * (*f).linesize[p as usize]) as isize);
                linesizes[p as usize] = 2 as ::core::ffi::c_int
                    * (*f).linesize[p as usize];
                p += 1;
            }
            av_image_copy(
                &raw mut dst_data as *mut *mut uint8_t as *const *mut uint8_t,
                &raw mut linesizes as *mut ::core::ffi::c_int
                    as *const ::core::ffi::c_int,
                &raw mut src_data as *mut *const uint8_t as *const *const uint8_t,
                &raw mut linesizes as *mut ::core::ffi::c_int
                    as *const ::core::ffi::c_int,
                (*f).format as AVPixelFormat,
                (*f).width,
                (*f).height >> 1 as ::core::ffi::c_int,
            );
        }
        ret = output_frame(h, dst, out);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        *got_frame = 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn send_next_delayed_frame(
    mut h: *mut H264Context,
    mut dst_frame: *mut AVFrame,
    mut got_frame: *mut ::core::ffi::c_int,
    mut buf_index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut out_idx: ::core::ffi::c_int = 0;
    let mut out: *mut H264Picture = ::core::ptr::null_mut::<H264Picture>();
    (*h).cur_pic_ptr = ::core::ptr::null_mut::<H264Picture>();
    (*h).first_field = 0 as ::core::ffi::c_int;
    while !(*h).delayed_pic[0 as ::core::ffi::c_int as usize].is_null() {
        out = (*h).delayed_pic[0 as ::core::ffi::c_int as usize];
        out_idx = 0 as ::core::ffi::c_int;
        i = 1 as ::core::ffi::c_int;
        while !(*h).delayed_pic[i as usize].is_null()
            && (*(*(*h).delayed_pic[i as usize]).f).flags
                & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int == 0
            && (*(*h).delayed_pic[i as usize]).mmco_reset == 0
        {
            if (*(*h).delayed_pic[i as usize]).poc < (*out).poc {
                out = (*h).delayed_pic[i as usize];
                out_idx = i;
            }
            i += 1;
        }
        i = out_idx;
        while !(*h).delayed_pic[i as usize].is_null() {
            (*h).delayed_pic[i as usize] = (*h)
                .delayed_pic[(i + 1 as ::core::ffi::c_int) as usize];
            i += 1;
        }
        if out.is_null() {
            continue;
        }
        (*h).frame_recovered |= (*out).recovered;
        (*out).recovered
            |= (*h).frame_recovered
                & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
        (*out).reference &= !((1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int);
        ret = finalize_frame(h, dst_frame, out, got_frame);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        if *got_frame != 0 {
            break;
        }
    }
    return buf_index;
}
unsafe extern "C" fn h264_decode_frame(
    mut avctx: *mut AVCodecContext,
    mut pict: *mut AVFrame,
    mut got_frame: *mut ::core::ffi::c_int,
    mut avpkt: *mut AVPacket,
) -> ::core::ffi::c_int {
    let mut buf: *const uint8_t = (*avpkt).data;
    let mut buf_size: ::core::ffi::c_int = (*avpkt).size;
    let mut h: *mut H264Context = (*avctx).priv_data as *mut H264Context;
    let mut buf_index: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    (*h).flags = (*avctx).flags;
    (*h).setup_finished = 0 as ::core::ffi::c_int;
    (*h).nb_slice_ctx_queued = 0 as ::core::ffi::c_int;
    ff_h264_unref_picture(&raw mut (*h).last_pic_for_ec);
    if buf_size == 0 as ::core::ffi::c_int {
        return send_next_delayed_frame(h, pict, got_frame, 0 as ::core::ffi::c_int);
    }
    if !av_packet_get_side_data(
            avpkt,
            AV_PKT_DATA_NEW_EXTRADATA,
            ::core::ptr::null_mut::<size_t>(),
        )
        .is_null()
    {
        let mut side_size: size_t = 0;
        let mut side: *mut uint8_t = av_packet_get_side_data(
            avpkt,
            AV_PKT_DATA_NEW_EXTRADATA,
            &raw mut side_size,
        );
        ff_h264_decode_extradata(
            side,
            side_size as ::core::ffi::c_int,
            &raw mut (*h).ps,
            &raw mut (*h).is_avc,
            &raw mut (*h).nal_length_size,
            (*avctx).err_recognition,
            avctx as *mut ::core::ffi::c_void,
        );
    }
    if (*h).is_avc != 0 && buf_size >= 9 as ::core::ffi::c_int
        && *buf.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int
        && *buf.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        && *buf.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xfc as ::core::ffi::c_int == 0xfc as ::core::ffi::c_int
    {
        if is_avcc_extradata(buf, buf_size) != 0 {
            return ff_h264_decode_extradata(
                buf,
                buf_size,
                &raw mut (*h).ps,
                &raw mut (*h).is_avc,
                &raw mut (*h).nal_length_size,
                (*avctx).err_recognition,
                avctx as *mut ::core::ffi::c_void,
            );
        }
    }
    buf_index = decode_nal_units(h, (*avpkt).buf, buf, buf_size);
    if buf_index < 0 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*h).cur_pic_ptr.is_null()
        && (*h).nal_unit_type == H264_NAL_END_SEQUENCE as ::core::ffi::c_int
    {
        if !(buf_index <= buf_size) {
            av_log(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                b"Assertion %s failed at %s:%d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                b"buf_index <= buf_size\0" as *const u8 as *const ::core::ffi::c_char,
                b"src/libavcodec/h264dec.c\0" as *const u8 as *const ::core::ffi::c_char,
                1057 as ::core::ffi::c_int,
            );
            abort();
        }
        return send_next_delayed_frame(h, pict, got_frame, buf_index);
    }
    if (*avctx).flags2 & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int == 0
        && ((*h).cur_pic_ptr.is_null() || (*h).has_slice == 0)
    {
        if (*avctx).skip_frame as ::core::ffi::c_int
            >= AVDISCARD_NONREF as ::core::ffi::c_int
            || buf_size >= 4 as ::core::ffi::c_int
                && memcmp(
                    b"Q264\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    buf as *const ::core::ffi::c_void,
                    4 as size_t,
                ) == 0
        {
            return buf_size;
        }
        av_log(
            avctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"no frame!\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*avctx).flags2 & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int == 0
        || (*h).mb_y >= (*h).mb_height && (*h).mb_height != 0
    {
        ret = ff_h264_field_end(
            h,
            (*h).slice_ctx.offset(0 as ::core::ffi::c_int as isize)
                as *mut H264SliceContext,
            0 as ::core::ffi::c_int,
        );
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        if !(*h).next_output_pic.is_null() {
            ret = finalize_frame(h, pict, (*h).next_output_pic, got_frame);
            if ret < 0 as ::core::ffi::c_int {
                return ret;
            }
        }
    }
    if !(!(*pict).buf[0 as ::core::ffi::c_int as usize].is_null() || *got_frame == 0) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"pict->buf[0] || !*got_frame\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/h264dec.c\0" as *const u8 as *const ::core::ffi::c_char,
            1082 as ::core::ffi::c_int,
        );
        abort();
    }
    ff_h264_unref_picture(&raw mut (*h).last_pic_for_ec);
    return buf_size;
}
static mut h264_options: [AVOption; 7] = [
    AVOption {
        name: b"is_avc\0" as *const u8 as *const ::core::ffi::c_char,
        help: b"is avc\0" as *const u8 as *const ::core::ffi::c_char,
        offset: 31752 as ::core::ffi::c_ulong as ::core::ffi::c_int,
        type_0: AV_OPT_TYPE_BOOL,
        default_val: C2RustUnnamed {
            i64_0: 0 as ::core::ffi::c_int as int64_t,
        },
        min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        max: 1 as ::core::ffi::c_int as ::core::ffi::c_double,
        flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int,
        unit: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVOption {
        name: b"nal_length_size\0" as *const u8 as *const ::core::ffi::c_char,
        help: b"nal_length_size\0" as *const u8 as *const ::core::ffi::c_char,
        offset: 31756 as ::core::ffi::c_ulong as ::core::ffi::c_int,
        type_0: AV_OPT_TYPE_INT,
        default_val: C2RustUnnamed {
            i64_0: 0 as ::core::ffi::c_int as int64_t,
        },
        min: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
        max: 4 as ::core::ffi::c_int as ::core::ffi::c_double,
        flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int,
        unit: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVOption {
        name: b"enable_er\0" as *const u8 as *const ::core::ffi::c_char,
        help: b"Enable error resilience on damaged frames (unsafe)\0" as *const u8
            as *const ::core::ffi::c_char,
        offset: 35936 as ::core::ffi::c_ulong as ::core::ffi::c_int,
        type_0: AV_OPT_TYPE_BOOL,
        default_val: C2RustUnnamed {
            i64_0: -(1 as ::core::ffi::c_int) as int64_t,
        },
        min: -(1 as ::core::ffi::c_int) as ::core::ffi::c_double,
        max: 1 as ::core::ffi::c_int as ::core::ffi::c_double,
        flags: (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
        unit: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    AVOption {
        name: b"x264_build\0" as *const u8 as *const ::core::ffi::c_char,
        help: b"Assume this x264 version if no x264 version found in any SEI\0"
            as *const u8 as *const ::core::ffi::c_char,
        offset: 30124 as ::core::ffi::c_ulong as ::core::ffi::c_int,
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
        name: b"skip_gray\0" as *const u8 as *const ::core::ffi::c_char,
        help: b"Do not return gray gap frames\0" as *const u8
            as *const ::core::ffi::c_char,
        offset: 53400 as ::core::ffi::c_ulong as ::core::ffi::c_int,
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
        name: b"noref_gray\0" as *const u8 as *const ::core::ffi::c_char,
        help: b"Avoid using gray gap frames as references\0" as *const u8
            as *const ::core::ffi::c_char,
        offset: 53396 as ::core::ffi::c_ulong as ::core::ffi::c_int,
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
static mut h264_class: AVClass = unsafe {
    AVClass {
        class_name: b"H264 Decoder\0" as *const u8 as *const ::core::ffi::c_char,
        item_name: Some(
            av_default_item_name
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                ) -> *const ::core::ffi::c_char,
        ),
        option: &raw const h264_options as *const AVOption,
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
pub static mut ff_h264_decoder: FFCodec = FFCodec {
    p: AVCodec {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        long_name: ::core::ptr::null::<::core::ffi::c_char>(),
        type_0: AVMEDIA_TYPE_VIDEO,
        id: AV_CODEC_ID_NONE,
        capabilities: 0,
        max_lowres: 0,
        priv_class: ::core::ptr::null::<AVClass>(),
        profiles: ::core::ptr::null::<AVProfile>(),
        wrapper_name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    caps_internal_is_decoder_color_ranges_alpha_modes_cb_type: [0; 4],
    priv_data_size: 0,
    update_thread_context: None,
    update_thread_context_for_user: None,
    defaults: ::core::ptr::null::<FFCodecDefault>(),
    init: None,
    cb: C2RustUnnamed_4 { decode: None },
    close: None,
    flush: None,
    bsfs: ::core::ptr::null::<::core::ffi::c_char>(),
    hw_configs: ::core::ptr::null::<*const AVCodecHWConfigInternal>(),
    codec_tags: ::core::ptr::null::<uint32_t>(),
    get_supported_config: None,
    c2rust_unnamed: C2RustUnnamed_1 {
        c2rust_unnamed: C2RustUnnamed_3 {
            supported_framerates: ::core::ptr::null::<AVRational>(),
            pix_fmts: ::core::ptr::null::<AVPixelFormat>(),
        },
    },
};
unsafe extern "C" fn run_static_initializers() {
    ff_h264_decoder = {
        let fresh8: [*const AVCodecHWConfigInternal; 1] = [
            ::core::ptr::null::<AVCodecHWConfigInternal>(),
        ];
        {
            let mut init = FFCodec {
                caps_internal_is_decoder_color_ranges_alpha_modes_cb_type: [0; 4],
                p: AVCodec {
                    name: b"h264\0" as *const u8 as *const ::core::ffi::c_char,
                    long_name: b"H.264 / AVC / MPEG-4 AVC / MPEG-4 part 10\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    type_0: AVMEDIA_TYPE_VIDEO,
                    id: AV_CODEC_ID_H264,
                    capabilities: (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 13 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int,
                    max_lowres: 0,
                    priv_class: &raw const h264_class,
                    profiles: &raw const ff_h264_profiles as *const AVProfile,
                    wrapper_name: ::core::ptr::null::<::core::ffi::c_char>(),
                },
                priv_data_size: ::core::mem::size_of::<H264Context>()
                    as ::core::ffi::c_int,
                update_thread_context: Some(
                    ff_h264_update_thread_context
                        as unsafe extern "C" fn(
                            *mut AVCodecContext,
                            *const AVCodecContext,
                        ) -> ::core::ffi::c_int,
                ),
                update_thread_context_for_user: Some(
                    ff_h264_update_thread_context_for_user
                        as unsafe extern "C" fn(
                            *mut AVCodecContext,
                            *const AVCodecContext,
                        ) -> ::core::ffi::c_int,
                ),
                defaults: ::core::ptr::null::<FFCodecDefault>(),
                init: Some(
                    h264_decode_init
                        as unsafe extern "C" fn(
                            *mut AVCodecContext,
                        ) -> ::core::ffi::c_int,
                ),
                cb: C2RustUnnamed_4 {
                    decode: Some(
                        h264_decode_frame
                            as unsafe extern "C" fn(
                                *mut AVCodecContext,
                                *mut AVFrame,
                                *mut ::core::ffi::c_int,
                                *mut AVPacket,
                            ) -> ::core::ffi::c_int,
                    ),
                },
                close: Some(
                    h264_decode_end
                        as unsafe extern "C" fn(
                            *mut AVCodecContext,
                        ) -> ::core::ffi::c_int,
                ),
                flush: Some(
                    h264_decode_flush as unsafe extern "C" fn(*mut AVCodecContext) -> (),
                ),
                bsfs: ::core::ptr::null::<::core::ffi::c_char>(),
                hw_configs: &raw const fresh8 as *const *const AVCodecHWConfigInternal,
                codec_tags: ::core::ptr::null::<uint32_t>(),
                get_supported_config: None,
                c2rust_unnamed: C2RustUnnamed_1 {
                    c2rust_unnamed: C2RustUnnamed_3 {
                        supported_framerates: ::core::ptr::null::<AVRational>(),
                        pix_fmts: ::core::ptr::null::<AVPixelFormat>(),
                    },
                },
            };
            init.set_caps_internal(
                ((1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint,
            );
            init.set_is_decoder(1 as ::core::ffi::c_uint);
            init.set_color_ranges(0);
            init.set_alpha_modes(0);
            init.set_cb_type(
                FF_CODEC_CB_TYPE_DECODE as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
            init
        }
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
#[repr(C)]
pub struct AVDictionary {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVBuffer {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVCodecDescriptor {
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
pub struct ThreadProgress {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct ThreadFrameProgress {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct MPVEncContext {
    _opaque: [u8; 0],
}
