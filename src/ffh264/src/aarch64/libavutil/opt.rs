// Translated from FFmpeg 9.0.1 libavutil/opt.c
// (preprocessed for aarch64-unknown-linux-musl) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// AVOptions
// Copyright (c) 2005 Michael Niedermayer <michaelni@gmx.at>
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
    fn memmove(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_void,
        _: size_t,
    ) -> *mut ::core::ffi::c_void;
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
    fn strtol(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn abort() -> !;
    fn strspn(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn av_expr_parse_and_eval(
        res: *mut ::core::ffi::c_double,
        s: *const ::core::ffi::c_char,
        const_names: *const *const ::core::ffi::c_char,
        const_values: *const ::core::ffi::c_double,
        func1_names: *const *const ::core::ffi::c_char,
        funcs1: *const Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_double,
        >,
        func2_names: *const *const ::core::ffi::c_char,
        funcs2: *const Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                ::core::ffi::c_double,
                ::core::ffi::c_double,
            ) -> ::core::ffi::c_double,
        >,
        opaque: *mut ::core::ffi::c_void,
        log_offset: ::core::ffi::c_int,
        log_ctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn strlen(_: *const ::core::ffi::c_char) -> size_t;
    fn av_parse_video_size(
        width_ptr: *mut ::core::ffi::c_int,
        height_ptr: *mut ::core::ffi::c_int,
        str: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn av_parse_video_rate(
        rate: *mut AVRational,
        str: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn av_get_sample_fmt_name(sample_fmt: AVSampleFormat) -> *const ::core::ffi::c_char;
    fn av_get_sample_fmt(name: *const ::core::ffi::c_char) -> AVSampleFormat;
    fn av_strerror(
        errnum: ::core::ffi::c_int,
        errbuf: *mut ::core::ffi::c_char,
        errbuf_size: size_t,
    ) -> ::core::ffi::c_int;
    fn av_parse_color(
        rgba_color: *mut uint8_t,
        color_string: *const ::core::ffi::c_char,
        slen: ::core::ffi::c_int,
        log_ctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn snprintf(
        _: *mut ::core::ffi::c_char,
        _: size_t,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn sscanf(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_strlcatf(
        dst: *mut ::core::ffi::c_char,
        size: size_t,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> size_t;
    fn av_dict_iterate(
        m: *const AVDictionary,
        prev: *const AVDictionaryEntry,
    ) -> *const AVDictionaryEntry;
    fn av_bprint_init(
        buf: *mut AVBPrint,
        size_init: ::core::ffi::c_uint,
        size_max: ::core::ffi::c_uint,
    );
    fn av_parse_time(
        timeval: *mut int64_t,
        timestr: *const ::core::ffi::c_char,
        duration: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn av_dict_set(
        pm: *mut *mut AVDictionary,
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_get_token(
        buf: *mut *const ::core::ffi::c_char,
        term: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn av_bprint_append_data(
        buf: *mut AVBPrint,
        data: *const ::core::ffi::c_char,
        size: ::core::ffi::c_uint,
    );
    fn av_d2q(d: ::core::ffi::c_double, max: ::core::ffi::c_int) -> AVRational;
    fn av_dict_parse_string(
        pm: *mut *mut AVDictionary,
        str: *const ::core::ffi::c_char,
        key_val_sep: *const ::core::ffi::c_char,
        pairs_sep: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_reallocp(ptr: *mut ::core::ffi::c_void, size: size_t) -> ::core::ffi::c_int;
    fn av_dict_copy(
        dst: *mut *mut AVDictionary,
        src: *const AVDictionary,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_dict_free(m: *mut *mut AVDictionary);
    fn av_bprint_finalize(
        buf: *mut AVBPrint,
        ret_str: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn av_dict_get_string(
        m: *const AVDictionary,
        buffer: *mut *mut ::core::ffi::c_char,
        key_val_sep: ::core::ffi::c_char,
        pairs_sep: ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn av_realloc_array(
        ptr: *mut ::core::ffi::c_void,
        nmemb: size_t,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_bprint_escape(
        dstbuf: *mut AVBPrint,
        src: *const ::core::ffi::c_char,
        special_chars: *const ::core::ffi::c_char,
        mode: AVEscapeMode,
        flags: ::core::ffi::c_int,
    );
    fn llrint(_: ::core::ffi::c_double) -> ::core::ffi::c_longlong;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn av_match_name(
        name: *const ::core::ffi::c_char,
        names: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn av_get_pix_fmt(name: *const ::core::ffi::c_char) -> AVPixelFormat;
    fn av_get_pix_fmt_name(pix_fmt: AVPixelFormat) -> *const ::core::ffi::c_char;
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_strdup(s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn av_memdup(
        p: *const ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_channel_layout_from_string(
        channel_layout: *mut AVChannelLayout,
        str: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn av_channel_layout_uninit(channel_layout: *mut AVChannelLayout);
    fn av_channel_layout_copy(
        dst: *mut AVChannelLayout,
        src: *const AVChannelLayout,
    ) -> ::core::ffi::c_int;
    fn av_channel_layout_describe(
        channel_layout: *const AVChannelLayout,
        buf: *mut ::core::ffi::c_char,
        buf_size: size_t,
    ) -> ::core::ffi::c_int;
    fn av_channel_layout_compare(
        chl: *const AVChannelLayout,
        chl1: *const AVChannelLayout,
    ) -> ::core::ffi::c_int;
}
pub type intptr_t = ::core::ffi::c_long;
pub type int64_t = ::core::ffi::c_long;
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint64_t = ::core::ffi::c_ulong;
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
pub type AVClassStateFlags = ::core::ffi::c_uint;
pub const AV_CLASS_STATE_INITIALIZED: AVClassStateFlags = 1;
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
pub type AVEscapeMode = ::core::ffi::c_uint;
pub const AV_ESCAPE_MODE_XML: AVEscapeMode = 3;
pub const AV_ESCAPE_MODE_QUOTE: AVEscapeMode = 2;
pub const AV_ESCAPE_MODE_BACKSLASH: AVEscapeMode = 1;
pub const AV_ESCAPE_MODE_AUTO: AVEscapeMode = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBPrint {
    pub str_0: *mut ::core::ffi::c_char,
    pub len: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
    pub size_max: ::core::ffi::c_uint,
    pub reserved_internal_buffer: [::core::ffi::c_char; 1],
    pub reserved_padding: [::core::ffi::c_char; 1000],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVDictionaryEntry {
    pub key: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub size: size_t,
    pub name: *const ::core::ffi::c_char,
}
pub const AV_OPT_FLAG_IMPLICIT_KEY: C2RustUnnamed_2 = 1;
pub type C2RustUnnamed_2 = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub data: *mut uint8_t,
    pub size: ::core::ffi::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_next(
    mut obj: *const ::core::ffi::c_void,
    mut last: *const AVOption,
) -> *const AVOption {
    let mut class: *const AVClass = ::core::ptr::null::<AVClass>();
    if obj.is_null() {
        return ::core::ptr::null::<AVOption>();
    }
    class = *(obj as *mut *const AVClass);
    if last.is_null() && !class.is_null() && !(*class).option.is_null()
        && !(*(*class).option.offset(0 as ::core::ffi::c_int as isize)).name.is_null()
    {
        return (*class).option as *const AVOption;
    }
    if !last.is_null()
        && !(*last.offset(1 as ::core::ffi::c_int as isize)).name.is_null()
    {
        last = last.offset(1);
        return last;
    }
    return ::core::ptr::null::<AVOption>();
}
static mut opt_type_desc: [C2RustUnnamed_1; 21] = [
    C2RustUnnamed_1 {
        size: 0,
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<::core::ffi::c_uint>() as size_t,
        name: b"<flags>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        name: b"<int>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<int64_t>() as size_t,
        name: b"<int64>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
        name: b"<double>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<::core::ffi::c_float>() as size_t,
        name: b"<float>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t,
        name: b"<string>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<AVRational>() as size_t,
        name: b"<rational>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<*mut uint8_t>() as size_t,
        name: b"<binary>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<*mut AVDictionary>() as size_t,
        name: b"<dictionary>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<uint64_t>() as size_t,
        name: b"<uint64>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: 0,
        name: ::core::ptr::null::<::core::ffi::c_char>(),
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<[::core::ffi::c_int; 2]>() as size_t,
        name: b"<image_size>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        name: b"<pix_fmt>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        name: b"<sample_fmt>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<AVRational>() as size_t,
        name: b"<video_rate>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<int64_t>() as size_t,
        name: b"<duration>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
        name: b"<color>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        name: b"<boolean>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<AVChannelLayout>() as size_t,
        name: b"<channel_layout>\0" as *const u8 as *const ::core::ffi::c_char,
    },
    C2RustUnnamed_1 {
        size: ::core::mem::size_of::<::core::ffi::c_uint>() as size_t,
        name: b"<unsigned>\0" as *const u8 as *const ::core::ffi::c_char,
    },
];
unsafe extern "C" fn opt_is_pod(mut type_0: AVOptionType) -> ::core::ffi::c_int {
    match type_0 as ::core::ffi::c_uint {
        1 | 2 | 3 | 4 | 5 | 7 | 10 | 12 | 13 | 14 | 15 | 16 | 17 | 18 | 20 => {
            return 1 as ::core::ffi::c_int;
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn av_cmp_q(
    mut a: AVRational,
    mut b: AVRational,
) -> ::core::ffi::c_int {
    let tmp: int64_t = a.num as int64_t * b.den as int64_t
        - b.num as int64_t * a.den as int64_t;
    if tmp != 0 {
        return ((tmp ^ a.den as int64_t ^ b.den as int64_t) >> 63 as ::core::ffi::c_int)
            as ::core::ffi::c_int | 1 as ::core::ffi::c_int
    } else if b.den != 0 && a.den != 0 {
        return 0 as ::core::ffi::c_int
    } else if a.num != 0 && b.num != 0 {
        return (a.num >> 31 as ::core::ffi::c_int) - (b.num >> 31 as ::core::ffi::c_int)
    } else {
        return -(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn opt_array_sep(mut o: *const AVOption) -> uint8_t {
    let mut d: *const AVOptionArrayDef = (*o).default_val.arr;
    return (if !d.is_null() && (*d).sep as ::core::ffi::c_int != 0 {
        (*d).sep as ::core::ffi::c_int
    } else {
        ',' as i32
    }) as uint8_t;
}
#[inline]
unsafe extern "C" fn av_make_error_string(
    mut errbuf: *mut ::core::ffi::c_char,
    mut errbuf_size: size_t,
    mut errnum: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    av_strerror(errnum, errbuf, errbuf_size);
    return errbuf;
}
unsafe extern "C" fn opt_array_pelem(
    mut o: *const AVOption,
    mut array: *mut ::core::ffi::c_void,
    mut idx: ::core::ffi::c_uint,
) -> *mut ::core::ffi::c_void {
    return (array as *mut uint8_t)
        .offset(
            (idx as size_t)
                .wrapping_mul(
                    opt_type_desc[((*o).type_0 as ::core::ffi::c_uint
                            & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int)
                                as ::core::ffi::c_uint) as usize]
                        .size,
                ) as isize,
        ) as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn opt_array_pcount(
    mut parray: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_uint {
    return (parray as *const *const ::core::ffi::c_void)
        .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_uint;
}
unsafe extern "C" fn opt_free_elem(
    mut type_0: AVOptionType,
    mut ptr: *mut ::core::ffi::c_void,
) {
    match type_0 as ::core::ffi::c_uint
        & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        6 | 8 => {
            av_freep(ptr);
        }
        9 => {
            av_dict_free(ptr as *mut *mut AVDictionary);
        }
        19 => {
            av_channel_layout_uninit(ptr as *mut AVChannelLayout);
        }
        _ => {}
    };
}
unsafe extern "C" fn opt_free_array(
    mut o: *const AVOption,
    mut parray: *mut ::core::ffi::c_void,
    mut count: *mut ::core::ffi::c_uint,
) {
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < *count {
        opt_free_elem(
            (*o).type_0,
            opt_array_pelem(o, *(parray as *mut *mut ::core::ffi::c_void), i),
        );
        i = i.wrapping_add(1);
    }
    av_freep(parray);
    *count = 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn opt_set_init(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut require_type: ::core::ffi::c_int,
    mut ptgt: *mut *mut ::core::ffi::c_void,
    mut po: *mut *const AVOption,
    mut pdst: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut o: *const AVOption = ::core::ptr::null::<AVOption>();
    let mut tgt: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    o = av_opt_find2(
        obj,
        name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        search_flags,
        &raw mut tgt,
    );
    if o.is_null() || tgt.is_null() {
        return -(((0xf8 as ::core::ffi::c_int | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*o).flags & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0 {
        return -(22 as ::core::ffi::c_int);
    }
    if require_type != 0
        && (*o).type_0 as ::core::ffi::c_uint != require_type as ::core::ffi::c_uint
    {
        av_log(
            obj,
            16 as ::core::ffi::c_int,
            b"Tried to set option '%s' of type %s from value of type %s, this is not supported\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*o).name,
            opt_type_desc[(*o).type_0 as usize].name,
            opt_type_desc[require_type as usize].name,
        );
        return -(22 as ::core::ffi::c_int);
    }
    if (*o).flags & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int == 0 {
        let mut state_flags: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<
            ::core::ffi::c_uint,
        >();
        let mut class: *const AVClass = ::core::ptr::null::<AVClass>();
        class = *(tgt as *mut *const AVClass);
        if (*class).state_flags_offset != 0 {
            state_flags = (tgt as *mut uint8_t)
                .offset((*class).state_flags_offset as isize)
                as *mut ::core::ffi::c_uint;
        }
        if state_flags.is_null() && obj != tgt {
            class = *(obj as *mut *const AVClass);
            if (*class).state_flags_offset != 0 {
                state_flags = (obj as *mut uint8_t)
                    .offset((*class).state_flags_offset as isize)
                    as *mut ::core::ffi::c_uint;
            }
        }
        if !state_flags.is_null()
            && *state_flags
                & AV_CLASS_STATE_INITIALIZED as ::core::ffi::c_int as ::core::ffi::c_uint
                != 0
        {
            av_log(
                obj,
                16 as ::core::ffi::c_int,
                b"Option '%s' is not a runtime option and so cannot be set after the object has been initialized\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*o).name,
            );
            return -(22 as ::core::ffi::c_int);
        }
    }
    if (*o).flags & (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int != 0 {
        av_log(
            obj,
            24 as ::core::ffi::c_int,
            b"The \"%s\" option is deprecated: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
            (*o).help,
        );
    }
    if !po.is_null() {
        *po = o;
    }
    if !ptgt.is_null() {
        *ptgt = tgt;
    }
    if !pdst.is_null() {
        *pdst = (tgt as *mut uint8_t).offset((*o).offset as isize)
            as *mut ::core::ffi::c_void;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn double_to_rational(mut d: ::core::ffi::c_double) -> AVRational {
    let mut r: AVRational = av_d2q(
        d,
        (1 as ::core::ffi::c_int) << 24 as ::core::ffi::c_int,
    );
    if (r.num == 0 || r.den == 0) && d != 0. {
        r = av_d2q(d, 2147483647 as ::core::ffi::c_int);
    }
    return r;
}
unsafe extern "C" fn read_number(
    mut o: *const AVOption,
    mut dst: *const ::core::ffi::c_void,
    mut num: *mut ::core::ffi::c_double,
    mut den: *mut ::core::ffi::c_int,
    mut intnum: *mut int64_t,
) -> ::core::ffi::c_int {
    match (*o).type_0 as ::core::ffi::c_uint
        & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        1 => {
            *intnum = *(dst as *mut ::core::ffi::c_uint) as int64_t;
            return 0 as ::core::ffi::c_int;
        }
        13 => {
            *intnum = *(dst as *mut AVPixelFormat) as int64_t;
            return 0 as ::core::ffi::c_int;
        }
        14 => {
            *intnum = *(dst as *mut AVSampleFormat) as int64_t;
            return 0 as ::core::ffi::c_int;
        }
        18 | 2 => {
            *intnum = *(dst as *mut ::core::ffi::c_int) as int64_t;
            return 0 as ::core::ffi::c_int;
        }
        20 => {
            *intnum = *(dst as *mut ::core::ffi::c_uint) as int64_t;
            return 0 as ::core::ffi::c_int;
        }
        16 | 3 | 10 => {
            *intnum = *(dst as *mut int64_t);
            return 0 as ::core::ffi::c_int;
        }
        5 => {
            *num = *(dst as *mut ::core::ffi::c_float) as ::core::ffi::c_double;
            return 0 as ::core::ffi::c_int;
        }
        4 => {
            *num = *(dst as *mut ::core::ffi::c_double);
            return 0 as ::core::ffi::c_int;
        }
        7 => {
            *intnum = (*(dst as *mut AVRational)).num as int64_t;
            *den = (*(dst as *mut AVRational)).den;
            return 0 as ::core::ffi::c_int;
        }
        11 => {
            *intnum = (*o).default_val.i64_0;
            return 0 as ::core::ffi::c_int;
        }
        _ => {}
    }
    return -(22 as ::core::ffi::c_int);
}
unsafe extern "C" fn write_number(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut dst: *mut ::core::ffi::c_void,
    mut num: ::core::ffi::c_double,
    mut den: ::core::ffi::c_int,
    mut intnum: int64_t,
) -> ::core::ffi::c_int {
    let type_0: AVOptionType = ((*o).type_0 as ::core::ffi::c_uint
        & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint)
        as AVOptionType;
    if type_0 as ::core::ffi::c_uint
        != AV_OPT_TYPE_FLAGS as ::core::ffi::c_int as ::core::ffi::c_uint
        && (den == 0
            || ((*o).max * den as ::core::ffi::c_double)
                < num * intnum as ::core::ffi::c_double
            || (*o).min * den as ::core::ffi::c_double
                > num * intnum as ::core::ffi::c_double)
    {
        num = if den != 0 {
            num * intnum as ::core::ffi::c_double / den as ::core::ffi::c_double
        } else {
            (if num != 0. && intnum != 0 {
                ::core::f32::INFINITY
            } else {
                ::core::f32::NAN
            }) as ::core::ffi::c_double
        };
        av_log(
            obj,
            16 as ::core::ffi::c_int,
            b"Value %f for parameter '%s' out of range [%g - %g]\n\0" as *const u8
                as *const ::core::ffi::c_char,
            num,
            (*o).name,
            (*o).min,
            (*o).max,
        );
        return -(34 as ::core::ffi::c_int);
    }
    if type_0 as ::core::ffi::c_uint
        == AV_OPT_TYPE_FLAGS as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut d: ::core::ffi::c_double = num * intnum as ::core::ffi::c_double
            / den as ::core::ffi::c_double;
        if d < -1.5f64
            || d > 0xffffffff as ::core::ffi::c_uint as ::core::ffi::c_double + 0.5f64
            || llrint(d * 256 as ::core::ffi::c_int as ::core::ffi::c_double)
                & 255 as ::core::ffi::c_longlong != 0
        {
            av_log(
                obj,
                16 as ::core::ffi::c_int,
                b"Value %f for parameter '%s' is not a valid set of 32bit integer flags\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                num * intnum as ::core::ffi::c_double / den as ::core::ffi::c_double,
                (*o).name,
            );
            return -(34 as ::core::ffi::c_int);
        }
    }
    match type_0 as ::core::ffi::c_uint {
        13 => {
            *(dst as *mut AVPixelFormat) = (llrint(num / den as ::core::ffi::c_double)
                * intnum as ::core::ffi::c_longlong) as AVPixelFormat;
        }
        14 => {
            *(dst as *mut AVSampleFormat) = (llrint(num / den as ::core::ffi::c_double)
                * intnum as ::core::ffi::c_longlong) as AVSampleFormat;
        }
        18 | 1 | 2 | 20 => {
            *(dst as *mut ::core::ffi::c_int) = (llrint(
                num / den as ::core::ffi::c_double,
            ) * intnum as ::core::ffi::c_longlong) as ::core::ffi::c_int;
        }
        16 | 3 => {
            let mut d_0: ::core::ffi::c_double = num / den as ::core::ffi::c_double;
            if intnum == 1 as int64_t
                && d_0
                    == 0x7fffffffffffffff as ::core::ffi::c_long as ::core::ffi::c_double
            {
                *(dst as *mut int64_t) = 0x7fffffffffffffff as ::core::ffi::c_long
                    as int64_t;
            } else {
                *(dst as *mut int64_t) = (llrint(d_0)
                    * intnum as ::core::ffi::c_longlong) as int64_t;
            }
        }
        10 => {
            let mut d_1: ::core::ffi::c_double = num / den as ::core::ffi::c_double;
            if intnum == 1 as int64_t
                && d_1
                    == 0xffffffffffffffff as ::core::ffi::c_ulong
                        as ::core::ffi::c_double
            {
                *(dst as *mut uint64_t) = 0xffffffffffffffff as ::core::ffi::c_ulong
                    as uint64_t;
            } else if d_1
                > (0x7fffffffffffffff as ::core::ffi::c_ulonglong)
                    .wrapping_add(1 as ::core::ffi::c_ulonglong) as ::core::ffi::c_double
            {
                *(dst as *mut uint64_t) = (llrint(
                    d_1
                        - (0x7fffffffffffffff as ::core::ffi::c_ulonglong)
                            .wrapping_add(1 as ::core::ffi::c_ulonglong)
                            as ::core::ffi::c_double,
                ) as ::core::ffi::c_ulonglong)
                    .wrapping_add(
                        (0x7fffffffffffffff as ::core::ffi::c_ulonglong)
                            .wrapping_add(1 as ::core::ffi::c_ulonglong),
                    )
                    .wrapping_mul(intnum as ::core::ffi::c_ulonglong) as uint64_t;
            } else {
                *(dst as *mut uint64_t) = (llrint(d_1)
                    * intnum as ::core::ffi::c_longlong) as uint64_t;
            }
        }
        5 => {
            *(dst as *mut ::core::ffi::c_float) = (num * intnum as ::core::ffi::c_double
                / den as ::core::ffi::c_double) as ::core::ffi::c_float;
        }
        4 => {
            *(dst as *mut ::core::ffi::c_double) = num * intnum as ::core::ffi::c_double
                / den as ::core::ffi::c_double;
        }
        7 | 15 => {
            if num as ::core::ffi::c_int as ::core::ffi::c_double == num {
                *(dst as *mut AVRational) = AVRational {
                    num: (num * intnum as ::core::ffi::c_double) as ::core::ffi::c_int,
                    den: den,
                };
            } else {
                *(dst as *mut AVRational) = double_to_rational(
                    num * intnum as ::core::ffi::c_double / den as ::core::ffi::c_double,
                );
            }
        }
        _ => return -(22 as ::core::ffi::c_int),
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn av_x_if_null(
    mut p: *const ::core::ffi::c_void,
    mut x: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    return (if !p.is_null() { p } else { x }) as intptr_t as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn hexchar2int(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    if c as ::core::ffi::c_int >= '0' as i32 && c as ::core::ffi::c_int <= '9' as i32 {
        return c as ::core::ffi::c_int - '0' as i32;
    }
    if c as ::core::ffi::c_int >= 'a' as i32 && c as ::core::ffi::c_int <= 'f' as i32 {
        return c as ::core::ffi::c_int - 'a' as i32 + 10 as ::core::ffi::c_int;
    }
    if c as ::core::ffi::c_int >= 'A' as i32 && c as ::core::ffi::c_int <= 'F' as i32 {
        return c as ::core::ffi::c_int - 'A' as i32 + 10 as ::core::ffi::c_int;
    }
    return -(1 as ::core::ffi::c_int);
}
unsafe extern "C" fn set_string_binary(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut lendst: *mut ::core::ffi::c_int = dst
        .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    let mut bin: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut len: ::core::ffi::c_int = 0;
    av_freep(dst as *mut ::core::ffi::c_void);
    *lendst = 0 as ::core::ffi::c_int;
    if val.is_null()
        || {
            len = strlen(val) as ::core::ffi::c_int;
            len == 0
        }
    {
        return 0 as ::core::ffi::c_int;
    }
    if len & 1 as ::core::ffi::c_int != 0 {
        return -(22 as ::core::ffi::c_int);
    }
    len /= 2 as ::core::ffi::c_int;
    bin = av_malloc(len as size_t) as *mut uint8_t;
    ptr = bin;
    if ptr.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    while *val != 0 {
        let fresh0 = val;
        val = val.offset(1);
        let mut a: ::core::ffi::c_int = hexchar2int(*fresh0);
        let fresh1 = val;
        val = val.offset(1);
        let mut b: ::core::ffi::c_int = hexchar2int(*fresh1);
        if a < 0 as ::core::ffi::c_int || b < 0 as ::core::ffi::c_int {
            av_free(bin as *mut ::core::ffi::c_void);
            return -(22 as ::core::ffi::c_int);
        }
        let fresh2 = ptr;
        ptr = ptr.offset(1);
        *fresh2 = (a << 4 as ::core::ffi::c_int | b) as uint8_t;
    }
    *dst = bin;
    *lendst = len;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn set_string(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut *mut uint8_t,
) -> ::core::ffi::c_int {
    av_freep(dst as *mut ::core::ffi::c_void);
    if val.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    *dst = av_strdup(val) as *mut uint8_t;
    return if !(*dst).is_null() {
        0 as ::core::ffi::c_int
    } else {
        -(12 as ::core::ffi::c_int)
    };
}
unsafe extern "C" fn set_string_number(
    mut obj: *mut ::core::ffi::c_void,
    mut target_obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let type_0: AVOptionType = ((*o).type_0 as ::core::ffi::c_uint
        & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint)
        as AVOptionType;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if type_0 as ::core::ffi::c_uint
        == AV_OPT_TYPE_RATIONAL as ::core::ffi::c_int as ::core::ffi::c_uint
        || type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_VIDEO_RATE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut num: ::core::ffi::c_int = 0;
        let mut den: ::core::ffi::c_int = 0;
        let mut c: ::core::ffi::c_char = 0;
        if sscanf(
            val,
            b"%d%*1[:/]%d%c\0" as *const u8 as *const ::core::ffi::c_char,
            &raw mut num,
            &raw mut den,
            &raw mut c,
        ) == 2 as ::core::ffi::c_int
        {
            ret = write_number(
                obj,
                o,
                dst,
                1 as ::core::ffi::c_int as ::core::ffi::c_double,
                den,
                num as int64_t,
            );
            if ret >= 0 as ::core::ffi::c_int {
                return ret;
            }
            ret = 0 as ::core::ffi::c_int;
        }
    }
    loop {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut buf: [::core::ffi::c_char; 256] = [0; 256];
        let mut cmd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut d: ::core::ffi::c_double = 0.;
        let mut intnum: int64_t = 1 as int64_t;
        if type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_FLAGS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if *val as ::core::ffi::c_int == '+' as i32
                || *val as ::core::ffi::c_int == '-' as i32
            {
                let fresh5 = val;
                val = val.offset(1);
                cmd = *fresh5 as ::core::ffi::c_int;
            }
            while (i as usize)
                < (::core::mem::size_of::<[::core::ffi::c_char; 256]>() as usize)
                    .wrapping_sub(1 as usize)
                && *val.offset(i as isize) as ::core::ffi::c_int != 0
                && *val.offset(i as isize) as ::core::ffi::c_int != '+' as i32
                && *val.offset(i as isize) as ::core::ffi::c_int != '-' as i32
            {
                buf[i as usize] = *val.offset(i as isize);
                i += 1;
            }
            buf[i as usize] = 0 as ::core::ffi::c_char;
        }
        let mut res: ::core::ffi::c_int = 0;
        let mut ci: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut const_values: [::core::ffi::c_double; 64] = [0.; 64];
        let mut const_names: [*const ::core::ffi::c_char; 64] = [::core::ptr::null::<
            ::core::ffi::c_char,
        >(); 64];
        let mut search_flags: ::core::ffi::c_int = if (*o).flags
            & (1 as ::core::ffi::c_int) << 18 as ::core::ffi::c_int != 0
        {
            (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
        let mut o_named: *const AVOption = av_opt_find(
            target_obj,
            if i != 0 {
                &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_char
            } else {
                val
            },
            (*o).unit,
            0 as ::core::ffi::c_int,
            search_flags,
        );
        if !o_named.is_null()
            && (*o_named).type_0 as ::core::ffi::c_uint
                == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            d = if (*o_named).type_0 as ::core::ffi::c_uint
                == AV_OPT_TYPE_INT64 as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*o_named).type_0 as ::core::ffi::c_uint
                    == AV_OPT_TYPE_UINT64 as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*o_named).type_0 as ::core::ffi::c_uint
                    == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*o_named).type_0 as ::core::ffi::c_uint
                    == AV_OPT_TYPE_FLAGS as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*o_named).type_0 as ::core::ffi::c_uint
                    == AV_OPT_TYPE_UINT as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*o_named).type_0 as ::core::ffi::c_uint
                    == AV_OPT_TYPE_INT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*o_named).default_val.i64_0 as ::core::ffi::c_double
            } else {
                (*o_named).default_val.dbl
            };
            if (*o_named).flags & (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int
                != 0
            {
                av_log(
                    obj,
                    24 as ::core::ffi::c_int,
                    b"The \"%s\" option is deprecated: %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*o_named).name,
                    (*o_named).help,
                );
            }
        } else {
            if !(*o).unit.is_null() {
                o_named = ::core::ptr::null::<AVOption>();
                loop {
                    o_named = av_opt_next(target_obj, o_named);
                    if o_named.is_null() {
                        break;
                    }
                    if (*o_named).type_0 as ::core::ffi::c_uint
                        == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
                        && !(*o_named).unit.is_null()
                        && strcmp((*o_named).unit, (*o).unit) == 0
                    {
                        if (ci + 6 as ::core::ffi::c_int) as usize
                            >= (::core::mem::size_of::<[::core::ffi::c_double; 64]>()
                                as usize)
                                .wrapping_div(
                                    ::core::mem::size_of::<::core::ffi::c_double>() as usize,
                                )
                        {
                            av_log(
                                obj,
                                16 as ::core::ffi::c_int,
                                b"const_values array too small for %s\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                (*o).unit,
                            );
                            return -((('P' as i32
                                | ('A' as i32) << 8 as ::core::ffi::c_int
                                | ('W' as i32) << 16 as ::core::ffi::c_int)
                                as ::core::ffi::c_uint
                                | ('E' as i32 as ::core::ffi::c_uint)
                                    << 24 as ::core::ffi::c_int) as ::core::ffi::c_int);
                        }
                        const_names[ci as usize] = (*o_named).name;
                        let fresh6 = ci;
                        ci = ci + 1;
                        const_values[fresh6 as usize] = if (*o_named).type_0
                            as ::core::ffi::c_uint
                            == AV_OPT_TYPE_INT64 as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            || (*o_named).type_0 as ::core::ffi::c_uint
                                == AV_OPT_TYPE_UINT64 as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            || (*o_named).type_0 as ::core::ffi::c_uint
                                == AV_OPT_TYPE_CONST as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            || (*o_named).type_0 as ::core::ffi::c_uint
                                == AV_OPT_TYPE_FLAGS as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            || (*o_named).type_0 as ::core::ffi::c_uint
                                == AV_OPT_TYPE_UINT as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            || (*o_named).type_0 as ::core::ffi::c_uint
                                == AV_OPT_TYPE_INT as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                        {
                            (*o_named).default_val.i64_0 as ::core::ffi::c_double
                        } else {
                            (*o_named).default_val.dbl
                        };
                    }
                }
            }
            const_names[ci as usize] = b"default\0" as *const u8
                as *const ::core::ffi::c_char;
            let fresh7 = ci;
            ci = ci + 1;
            const_values[fresh7 as usize] = if (*o).type_0 as ::core::ffi::c_uint
                == AV_OPT_TYPE_INT64 as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*o).type_0 as ::core::ffi::c_uint
                    == AV_OPT_TYPE_UINT64 as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*o).type_0 as ::core::ffi::c_uint
                    == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*o).type_0 as ::core::ffi::c_uint
                    == AV_OPT_TYPE_FLAGS as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*o).type_0 as ::core::ffi::c_uint
                    == AV_OPT_TYPE_UINT as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*o).type_0 as ::core::ffi::c_uint
                    == AV_OPT_TYPE_INT as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*o).default_val.i64_0 as ::core::ffi::c_double
            } else {
                (*o).default_val.dbl
            };
            const_names[ci as usize] = b"max\0" as *const u8
                as *const ::core::ffi::c_char;
            let fresh8 = ci;
            ci = ci + 1;
            const_values[fresh8 as usize] = (*o).max;
            const_names[ci as usize] = b"min\0" as *const u8
                as *const ::core::ffi::c_char;
            let fresh9 = ci;
            ci = ci + 1;
            const_values[fresh9 as usize] = (*o).min;
            const_names[ci as usize] = b"none\0" as *const u8
                as *const ::core::ffi::c_char;
            let fresh10 = ci;
            ci = ci + 1;
            const_values[fresh10 as usize] = 0 as ::core::ffi::c_int
                as ::core::ffi::c_double;
            const_names[ci as usize] = b"all\0" as *const u8
                as *const ::core::ffi::c_char;
            let fresh11 = ci;
            ci = ci + 1;
            const_values[fresh11 as usize] = !(0 as ::core::ffi::c_int)
                as ::core::ffi::c_double;
            const_names[ci as usize] = ::core::ptr::null::<::core::ffi::c_char>();
            const_values[ci as usize] = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            res = av_expr_parse_and_eval(
                &raw mut d,
                if i != 0 {
                    &raw mut buf as *mut ::core::ffi::c_char
                        as *const ::core::ffi::c_char
                } else {
                    val
                },
                &raw mut const_names as *mut *const ::core::ffi::c_char,
                &raw mut const_values as *mut ::core::ffi::c_double,
                ::core::ptr::null::<*const ::core::ffi::c_char>(),
                ::core::ptr::null::<
                    Option<
                        unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_double,
                        ) -> ::core::ffi::c_double,
                    >,
                >(),
                ::core::ptr::null::<*const ::core::ffi::c_char>(),
                ::core::ptr::null::<
                    Option<
                        unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            ::core::ffi::c_double,
                            ::core::ffi::c_double,
                        ) -> ::core::ffi::c_double,
                    >,
                >(),
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                0 as ::core::ffi::c_int,
                obj,
            );
            if res < 0 as ::core::ffi::c_int {
                av_log(
                    obj,
                    16 as ::core::ffi::c_int,
                    b"Unable to parse \"%s\" option value \"%s\"\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*o).name,
                    val,
                );
                return res;
            }
        }
        if type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_FLAGS as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            intnum = *(dst as *mut ::core::ffi::c_uint) as int64_t;
            if cmd == '+' as i32 {
                d = (intnum | d as int64_t) as ::core::ffi::c_double;
            } else if cmd == '-' as i32 {
                d = (intnum & !(d as int64_t)) as ::core::ffi::c_double;
            }
        }
        ret = write_number(obj, o, dst, d, 1 as ::core::ffi::c_int, 1 as int64_t);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        val = val.offset(i as isize);
        if i == 0 || *val == 0 {
            return 0 as ::core::ffi::c_int;
        }
    };
}
unsafe extern "C" fn set_string_image_size(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if val.is_null()
        || strcmp(val, b"none\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        let ref mut fresh3 = *dst.offset(1 as ::core::ffi::c_int as isize);
        *fresh3 = 0 as ::core::ffi::c_int;
        *dst.offset(0 as ::core::ffi::c_int as isize) = *fresh3;
        return 0 as ::core::ffi::c_int;
    }
    ret = av_parse_video_size(dst, dst.offset(1 as ::core::ffi::c_int as isize), val);
    if ret < 0 as ::core::ffi::c_int {
        av_log(
            obj,
            16 as ::core::ffi::c_int,
            b"Unable to parse \"%s\" option value \"%s\" as image size\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*o).name,
            val,
        );
    }
    return ret;
}
unsafe extern "C" fn set_string_video_rate(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut AVRational,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = av_parse_video_rate(dst, val);
    if ret < 0 as ::core::ffi::c_int {
        av_log(
            obj,
            16 as ::core::ffi::c_int,
            b"Unable to parse \"%s\" option value \"%s\" as video rate\n\0" as *const u8
                as *const ::core::ffi::c_char,
            (*o).name,
            val,
        );
    }
    return ret;
}
unsafe extern "C" fn set_string_color(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if val.is_null() {
        return 0 as ::core::ffi::c_int
    } else {
        ret = av_parse_color(dst, val, -(1 as ::core::ffi::c_int), obj);
        if ret < 0 as ::core::ffi::c_int {
            av_log(
                obj,
                16 as ::core::ffi::c_int,
                b"Unable to parse \"%s\" option value \"%s\" as color\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*o).name,
                val,
            );
        }
        return ret;
    };
}
unsafe extern "C" fn get_bool_name(
    mut val: ::core::ffi::c_int,
) -> *const ::core::ffi::c_char {
    if val < 0 as ::core::ffi::c_int {
        return b"auto\0" as *const u8 as *const ::core::ffi::c_char;
    }
    return if val != 0 {
        b"true\0" as *const u8 as *const ::core::ffi::c_char
    } else {
        b"false\0" as *const u8 as *const ::core::ffi::c_char
    };
}
unsafe extern "C" fn set_string_bool(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut n: ::core::ffi::c_int = 0;
    if val.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if strcmp(val, b"auto\0" as *const u8 as *const ::core::ffi::c_char) == 0 {
        n = -(1 as ::core::ffi::c_int);
        current_block = 7746791466490516765;
    } else if av_match_name(
        val,
        b"true,y,yes,enable,enabled,on\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        n = 1 as ::core::ffi::c_int;
        current_block = 7746791466490516765;
    } else if av_match_name(
        val,
        b"false,n,no,disable,disabled,off\0" as *const u8 as *const ::core::ffi::c_char,
    ) != 0
    {
        n = 0 as ::core::ffi::c_int;
        current_block = 7746791466490516765;
    } else {
        let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        n = strtol(val, &raw mut end, 10 as ::core::ffi::c_int) as ::core::ffi::c_int;
        if val.offset(strlen(val) as isize) != end as *const ::core::ffi::c_char {
            current_block = 10433985508896939146;
        } else {
            current_block = 7746791466490516765;
        }
    }
    match current_block {
        7746791466490516765 => {
            if !((n as ::core::ffi::c_double) < (*o).min
                || n as ::core::ffi::c_double > (*o).max)
            {
                *dst = n;
                return 0 as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    av_log(
        obj,
        16 as ::core::ffi::c_int,
        b"Unable to parse \"%s\" option value \"%s\" as boolean\n\0" as *const u8
            as *const ::core::ffi::c_char,
        (*o).name,
        val,
    );
    return -(22 as ::core::ffi::c_int);
}
unsafe extern "C" fn set_string_fmt(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut uint8_t,
    mut fmt_nb: ::core::ffi::c_int,
    mut get_fmt: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
    >,
    mut desc: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fmt: ::core::ffi::c_int = 0;
    let mut min: ::core::ffi::c_int = 0;
    let mut max: ::core::ffi::c_int = 0;
    if val.is_null()
        || strcmp(val, b"none\0" as *const u8 as *const ::core::ffi::c_char) == 0
    {
        fmt = -(1 as ::core::ffi::c_int);
    } else {
        fmt = get_fmt.expect("non-null function pointer")(val) as ::core::ffi::c_int;
        if fmt == -(1 as ::core::ffi::c_int) {
            let mut tail: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
                ::core::ffi::c_char,
            >();
            fmt = strtol(val, &raw mut tail, 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            if *tail as ::core::ffi::c_int != 0
                || fmt as ::core::ffi::c_uint >= fmt_nb as ::core::ffi::c_uint
            {
                av_log(
                    obj,
                    16 as ::core::ffi::c_int,
                    b"Unable to parse \"%s\" option value \"%s\" as %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    (*o).name,
                    val,
                    desc,
                );
                return -(22 as ::core::ffi::c_int);
            }
        }
    }
    min = (if (*o).min > -(1 as ::core::ffi::c_int) as ::core::ffi::c_double {
        (*o).min
    } else {
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_double
    }) as ::core::ffi::c_int;
    max = (if (*o).max > (fmt_nb - 1 as ::core::ffi::c_int) as ::core::ffi::c_double {
        (fmt_nb - 1 as ::core::ffi::c_int) as ::core::ffi::c_double
    } else {
        (*o).max
    }) as ::core::ffi::c_int;
    if min == 0 as ::core::ffi::c_int && max == 0 as ::core::ffi::c_int {
        min = -(1 as ::core::ffi::c_int);
        max = fmt_nb - 1 as ::core::ffi::c_int;
    }
    if fmt < min || fmt > max {
        av_log(
            obj,
            16 as ::core::ffi::c_int,
            b"Value %d for parameter '%s' out of %s format range [%d - %d]\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            fmt,
            (*o).name,
            desc,
            min,
            max,
        );
        return -(34 as ::core::ffi::c_int);
    }
    *(dst as *mut ::core::ffi::c_int) = fmt;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn get_pix_fmt(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return av_get_pix_fmt(name) as ::core::ffi::c_int;
}
unsafe extern "C" fn set_string_pixel_fmt(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut uint8_t,
) -> ::core::ffi::c_int {
    return set_string_fmt(
        obj,
        o,
        val,
        dst,
        AV_PIX_FMT_NB as ::core::ffi::c_int,
        Some(
            get_pix_fmt
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
        b"pixel format\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn get_sample_fmt(
    mut name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return av_get_sample_fmt(name) as ::core::ffi::c_int;
}
unsafe extern "C" fn set_string_sample_fmt(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut uint8_t,
) -> ::core::ffi::c_int {
    return set_string_fmt(
        obj,
        o,
        val,
        dst,
        AV_SAMPLE_FMT_NB as ::core::ffi::c_int,
        Some(
            get_sample_fmt
                as unsafe extern "C" fn(*const ::core::ffi::c_char) -> ::core::ffi::c_int,
        ),
        b"sample format\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn set_string_dict(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut options: *mut AVDictionary = ::core::ptr::null_mut::<AVDictionary>();
    if !val.is_null() {
        let mut ret: ::core::ffi::c_int = av_dict_parse_string(
            &raw mut options,
            val,
            b"=\0" as *const u8 as *const ::core::ffi::c_char,
            b":\0" as *const u8 as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
        );
        if ret < 0 as ::core::ffi::c_int {
            av_dict_free(&raw mut options);
            return ret;
        }
    }
    av_dict_free(dst as *mut *mut AVDictionary);
    *dst = options as *mut uint8_t;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn set_string_channel_layout(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut channel_layout: *mut AVChannelLayout = dst as *mut AVChannelLayout;
    av_channel_layout_uninit(channel_layout);
    if val.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    return av_channel_layout_from_string(channel_layout, val);
}
unsafe extern "C" fn opt_set_elem(
    mut obj: *mut ::core::ffi::c_void,
    mut target_obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let type_0: AVOptionType = ((*o).type_0 as ::core::ffi::c_uint
        & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint)
        as AVOptionType;
    let mut ret: ::core::ffi::c_int = 0;
    if val.is_null()
        && (type_0 as ::core::ffi::c_uint
            != AV_OPT_TYPE_STRING as ::core::ffi::c_int as ::core::ffi::c_uint
            && type_0 as ::core::ffi::c_uint
                != AV_OPT_TYPE_PIXEL_FMT as ::core::ffi::c_int as ::core::ffi::c_uint
            && type_0 as ::core::ffi::c_uint
                != AV_OPT_TYPE_SAMPLE_FMT as ::core::ffi::c_int as ::core::ffi::c_uint
            && type_0 as ::core::ffi::c_uint
                != AV_OPT_TYPE_IMAGE_SIZE as ::core::ffi::c_int as ::core::ffi::c_uint
            && type_0 as ::core::ffi::c_uint
                != AV_OPT_TYPE_DURATION as ::core::ffi::c_int as ::core::ffi::c_uint
            && type_0 as ::core::ffi::c_uint
                != AV_OPT_TYPE_COLOR as ::core::ffi::c_int as ::core::ffi::c_uint
            && type_0 as ::core::ffi::c_uint
                != AV_OPT_TYPE_BOOL as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        return -(22 as ::core::ffi::c_int);
    }
    match type_0 as ::core::ffi::c_uint {
        18 => return set_string_bool(obj, o, val, dst as *mut ::core::ffi::c_int),
        6 => return set_string(obj, o, val, dst as *mut *mut uint8_t),
        8 => return set_string_binary(obj, o, val, dst as *mut *mut uint8_t),
        1 | 2 | 20 | 3 | 10 | 5 | 4 | 7 => {
            return set_string_number(obj, target_obj, o, val, dst);
        }
        12 => return set_string_image_size(obj, o, val, dst as *mut ::core::ffi::c_int),
        15 => {
            let mut tmp: AVRational = AVRational { num: 0, den: 0 };
            ret = set_string_video_rate(obj, o, val, &raw mut tmp);
            if ret < 0 as ::core::ffi::c_int {
                return ret;
            }
            return write_number(
                obj,
                o,
                dst,
                1 as ::core::ffi::c_int as ::core::ffi::c_double,
                tmp.den,
                tmp.num as int64_t,
            );
        }
        13 => return set_string_pixel_fmt(obj, o, val, dst as *mut uint8_t),
        14 => return set_string_sample_fmt(obj, o, val, dst as *mut uint8_t),
        16 => {
            let mut usecs: int64_t = 0 as int64_t;
            if !val.is_null() {
                ret = av_parse_time(&raw mut usecs, val, 1 as ::core::ffi::c_int);
                if ret < 0 as ::core::ffi::c_int {
                    av_log(
                        obj,
                        16 as ::core::ffi::c_int,
                        b"Unable to parse \"%s\" option value \"%s\" as duration\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        (*o).name,
                        val,
                    );
                    return ret;
                }
            }
            if (usecs as ::core::ffi::c_double) < (*o).min
                || usecs as ::core::ffi::c_double > (*o).max
            {
                av_log(
                    obj,
                    16 as ::core::ffi::c_int,
                    b"Value %f for parameter '%s' out of range [%g - %g]\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    usecs as ::core::ffi::c_double / 1000000.0f64,
                    (*o).name,
                    (*o).min / 1000000.0f64,
                    (*o).max / 1000000.0f64,
                );
                return -(34 as ::core::ffi::c_int);
            }
            *(dst as *mut int64_t) = usecs;
            return 0 as ::core::ffi::c_int;
        }
        17 => return set_string_color(obj, o, val, dst as *mut uint8_t),
        19 => {
            ret = set_string_channel_layout(obj, o, val, dst);
            if ret < 0 as ::core::ffi::c_int {
                av_log(
                    obj,
                    16 as ::core::ffi::c_int,
                    b"Unable to parse \"%s\" option value \"%s\" as channel layout\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    (*o).name,
                    val,
                );
                ret = -(22 as ::core::ffi::c_int);
            }
            return ret;
        }
        9 => return set_string_dict(obj, o, val, dst as *mut *mut uint8_t),
        _ => {}
    }
    av_log(
        obj,
        16 as ::core::ffi::c_int,
        b"Invalid option type.\n\0" as *const u8 as *const ::core::ffi::c_char,
    );
    return -(22 as ::core::ffi::c_int);
}
unsafe extern "C" fn opt_set_array(
    mut obj: *mut ::core::ffi::c_void,
    mut target_obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut dst: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut arr: *const AVOptionArrayDef = (*o).default_val.arr;
    let elem_size: size_t = opt_type_desc[((*o).type_0 as ::core::ffi::c_uint
            & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as usize]
        .size;
    let sep: uint8_t = opt_array_sep(o) as uint8_t;
    let mut str: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut elems: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut nb_elems: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut ret: ::core::ffi::c_int = 0;
    if !val.is_null() && *val as ::core::ffi::c_int != 0 {
        str = av_malloc((strlen(val) as size_t).wrapping_add(1 as size_t))
            as *mut uint8_t;
        if str.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
    }
    loop {
        if !(!val.is_null() && *val as ::core::ffi::c_int != 0) {
            current_block = 15768484401365413375;
            break;
        }
        let mut p: *mut uint8_t = str;
        let mut tmp: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
            ::core::ffi::c_void,
        >();
        if !arr.is_null() && (*arr).size_max != 0 && nb_elems >= (*arr).size_max {
            av_log(
                obj,
                16 as ::core::ffi::c_int,
                b"Cannot assign more than %u elements to array option %s\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                (*arr).size_max,
                (*o).name,
            );
            ret = -(22 as ::core::ffi::c_int);
            current_block = 10358277024149305092;
            break;
        } else {
            while *val != 0 {
                if *val as ::core::ffi::c_int == '\\' as i32
                    && *val.offset(1 as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int != 0
                {
                    val = val.offset(1);
                } else if *val as ::core::ffi::c_int == sep as ::core::ffi::c_int {
                    val = val.offset(1);
                    break;
                }
                *p = *val as uint8_t;
                val = val.offset(1);
                p = p.offset(1);
            }
            *p = 0 as uint8_t;
            tmp = av_realloc_array(
                elems,
                nb_elems.wrapping_add(1 as ::core::ffi::c_uint) as size_t,
                elem_size,
            );
            if tmp.is_null() {
                ret = -(12 as ::core::ffi::c_int);
                current_block = 10358277024149305092;
                break;
            } else {
                elems = tmp;
                tmp = opt_array_pelem(o, elems, nb_elems);
                memset(tmp, 0 as ::core::ffi::c_int, elem_size as size_t);
                ret = opt_set_elem(
                    obj,
                    target_obj,
                    o,
                    str as *const ::core::ffi::c_char,
                    tmp,
                );
                if ret < 0 as ::core::ffi::c_int {
                    current_block = 10358277024149305092;
                    break;
                }
                nb_elems = nb_elems.wrapping_add(1);
            }
        }
    }
    match current_block {
        15768484401365413375 => {
            av_freep(&raw mut str as *mut ::core::ffi::c_void);
            opt_free_array(o, dst, opt_array_pcount(dst));
            if !arr.is_null() && nb_elems < (*arr).size_min {
                av_log(
                    obj,
                    16 as ::core::ffi::c_int,
                    b"Cannot assign fewer than %u elements to array option %s\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    (*arr).size_min,
                    (*o).name,
                );
                ret = -(22 as ::core::ffi::c_int);
            } else {
                let ref mut fresh4 = *(dst as *mut *mut ::core::ffi::c_void);
                *fresh4 = elems;
                *opt_array_pcount(dst) = nb_elems;
                return 0 as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    av_freep(&raw mut str as *mut ::core::ffi::c_void);
    opt_free_array(o, &raw mut elems as *mut ::core::ffi::c_void, &raw mut nb_elems);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut val: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dst: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut target_obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut o: *const AVOption = ::core::ptr::null::<AVOption>();
    let mut ret: ::core::ffi::c_int = 0;
    ret = opt_set_init(
        obj,
        name,
        search_flags,
        0 as ::core::ffi::c_int,
        &raw mut target_obj,
        &raw mut o,
        &raw mut dst,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    return if (*o).type_0 as ::core::ffi::c_uint
        & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        Some(
            opt_set_array
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    *const AVOption,
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
    } else {
        Some(
            opt_set_elem
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                    *const AVOption,
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
    }
        .expect("non-null function pointer")(obj, target_obj, o, val, dst);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_eval_flags(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut flags_out: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if o.is_null()
        || (*o).type_0 as ::core::ffi::c_uint
            != AV_OPT_TYPE_FLAGS as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*o).flags & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    return set_string_number(obj, obj, o, val, flags_out as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_eval_int(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut int_out: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if o.is_null()
        || (*o).type_0 as ::core::ffi::c_uint
            != AV_OPT_TYPE_INT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*o).flags & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    return set_string_number(obj, obj, o, val, int_out as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_eval_uint(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut uint_out: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if o.is_null()
        || (*o).type_0 as ::core::ffi::c_uint
            != AV_OPT_TYPE_UINT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*o).flags & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    return set_string_number(obj, obj, o, val, uint_out as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_eval_int64(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut int64_out: *mut int64_t,
) -> ::core::ffi::c_int {
    if o.is_null()
        || (*o).type_0 as ::core::ffi::c_uint
            != AV_OPT_TYPE_INT64 as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*o).flags & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    return set_string_number(obj, obj, o, val, int64_out as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_eval_float(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut float_out: *mut ::core::ffi::c_float,
) -> ::core::ffi::c_int {
    if o.is_null()
        || (*o).type_0 as ::core::ffi::c_uint
            != AV_OPT_TYPE_FLOAT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*o).flags & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    return set_string_number(obj, obj, o, val, float_out as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_eval_double(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut double_out: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    if o.is_null()
        || (*o).type_0 as ::core::ffi::c_uint
            != AV_OPT_TYPE_DOUBLE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*o).flags & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    return set_string_number(obj, obj, o, val, double_out as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_eval_q(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut val: *const ::core::ffi::c_char,
    mut q_out: *mut AVRational,
) -> ::core::ffi::c_int {
    if o.is_null()
        || (*o).type_0 as ::core::ffi::c_uint
            != AV_OPT_TYPE_RATIONAL as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*o).flags & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    return set_string_number(obj, obj, o, val, q_out as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn set_number(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut num: ::core::ffi::c_double,
    mut den: ::core::ffi::c_int,
    mut intnum: int64_t,
    mut search_flags: ::core::ffi::c_int,
    mut require_type: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dst: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut o: *const AVOption = ::core::ptr::null::<AVOption>();
    let mut ret: ::core::ffi::c_int = 0;
    ret = opt_set_init(
        obj,
        name,
        search_flags,
        require_type,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        &raw mut o,
        &raw mut dst,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    return write_number(obj, o, dst, num, den, intnum);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_int(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut val: int64_t,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return set_number(
        obj,
        name,
        1 as ::core::ffi::c_int as ::core::ffi::c_double,
        1 as ::core::ffi::c_int,
        val,
        search_flags,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_double(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut val: ::core::ffi::c_double,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return set_number(
        obj,
        name,
        val,
        1 as ::core::ffi::c_int,
        1 as int64_t,
        search_flags,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_q(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut val: AVRational,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return set_number(
        obj,
        name,
        val.num as ::core::ffi::c_double,
        val.den,
        1 as int64_t,
        search_flags,
        0 as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_bin(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut val: *const uint8_t,
    mut len: ::core::ffi::c_int,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut dst: *mut *mut uint8_t = ::core::ptr::null_mut::<*mut uint8_t>();
    let mut lendst: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<
        ::core::ffi::c_int,
    >();
    let mut ret: ::core::ffi::c_int = 0;
    ret = opt_set_init(
        obj,
        name,
        search_flags,
        AV_OPT_TYPE_BINARY as ::core::ffi::c_int,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        ::core::ptr::null_mut::<*const AVOption>(),
        &raw mut dst as *mut *mut ::core::ffi::c_void,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    ptr = (if len != 0 {
        av_malloc(len as size_t)
    } else {
        ::core::ptr::null_mut::<::core::ffi::c_void>()
    }) as *mut uint8_t;
    if len != 0 && ptr.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    lendst = dst.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int;
    av_free(*dst as *mut ::core::ffi::c_void);
    *dst = ptr;
    *lendst = len;
    if len != 0 {
        memcpy(
            ptr as *mut ::core::ffi::c_void,
            val as *const ::core::ffi::c_void,
            len as size_t,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_image_size(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut w: ::core::ffi::c_int,
    mut h: ::core::ffi::c_int,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut o: *const AVOption = ::core::ptr::null::<AVOption>();
    let mut dst: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut ret: ::core::ffi::c_int = 0;
    ret = opt_set_init(
        obj,
        name,
        search_flags,
        AV_OPT_TYPE_IMAGE_SIZE as ::core::ffi::c_int,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        &raw mut o,
        &raw mut dst as *mut *mut ::core::ffi::c_void,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if w < 0 as ::core::ffi::c_int || h < 0 as ::core::ffi::c_int {
        av_log(
            obj,
            16 as ::core::ffi::c_int,
            b"Invalid negative size value %dx%d for size '%s'\n\0" as *const u8
                as *const ::core::ffi::c_char,
            w,
            h,
            (*o).name,
        );
        return -(22 as ::core::ffi::c_int);
    }
    *dst.offset(0 as ::core::ffi::c_int as isize) = w;
    *dst.offset(1 as ::core::ffi::c_int as isize) = h;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_video_rate(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut val: AVRational,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return set_number(
        obj,
        name,
        val.num as ::core::ffi::c_double,
        val.den,
        1 as int64_t,
        search_flags,
        AV_OPT_TYPE_VIDEO_RATE as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn set_format(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut fmt: ::core::ffi::c_int,
    mut search_flags: ::core::ffi::c_int,
    mut type_0: AVOptionType,
    mut desc: *const ::core::ffi::c_char,
    mut nb_fmts: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut o: *const AVOption = ::core::ptr::null::<AVOption>();
    let mut dst: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
    let mut min: ::core::ffi::c_int = 0;
    let mut max: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    ret = opt_set_init(
        obj,
        name,
        search_flags,
        type_0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        &raw mut o,
        &raw mut dst as *mut *mut ::core::ffi::c_void,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    min = (if (*o).min > -(1 as ::core::ffi::c_int) as ::core::ffi::c_double {
        (*o).min
    } else {
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_double
    }) as ::core::ffi::c_int;
    max = (if (*o).max > (nb_fmts - 1 as ::core::ffi::c_int) as ::core::ffi::c_double {
        (nb_fmts - 1 as ::core::ffi::c_int) as ::core::ffi::c_double
    } else {
        (*o).max
    }) as ::core::ffi::c_int;
    if fmt < min || fmt > max {
        av_log(
            obj,
            16 as ::core::ffi::c_int,
            b"Value %d for parameter '%s' out of %s format range [%d - %d]\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            fmt,
            name,
            desc,
            min,
            max,
        );
        return -(34 as ::core::ffi::c_int);
    }
    *dst = fmt;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_pixel_fmt(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut fmt: AVPixelFormat,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return set_format(
        obj,
        name,
        fmt as ::core::ffi::c_int,
        search_flags,
        AV_OPT_TYPE_PIXEL_FMT,
        b"pixel\0" as *const u8 as *const ::core::ffi::c_char,
        AV_PIX_FMT_NB as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_sample_fmt(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut fmt: AVSampleFormat,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return set_format(
        obj,
        name,
        fmt as ::core::ffi::c_int,
        search_flags,
        AV_OPT_TYPE_SAMPLE_FMT,
        b"sample\0" as *const u8 as *const ::core::ffi::c_char,
        AV_SAMPLE_FMT_NB as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_dict_val(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut val: *const AVDictionary,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dst: *mut *mut AVDictionary = ::core::ptr::null_mut::<*mut AVDictionary>();
    let mut ret: ::core::ffi::c_int = 0;
    ret = opt_set_init(
        obj,
        name,
        search_flags,
        AV_OPT_TYPE_DICT as ::core::ffi::c_int,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        ::core::ptr::null_mut::<*const AVOption>(),
        &raw mut dst as *mut *mut ::core::ffi::c_void,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    av_dict_free(dst);
    return av_dict_copy(dst, val, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_chlayout(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut channel_layout: *const AVChannelLayout,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dst: *mut AVChannelLayout = ::core::ptr::null_mut::<AVChannelLayout>();
    let mut ret: ::core::ffi::c_int = 0;
    ret = opt_set_init(
        obj,
        name,
        search_flags,
        AV_OPT_TYPE_CHLAYOUT as ::core::ffi::c_int,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
        ::core::ptr::null_mut::<*const AVOption>(),
        &raw mut dst as *mut *mut ::core::ffi::c_void,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    return av_channel_layout_copy(dst, channel_layout);
}
unsafe extern "C" fn format_duration(
    mut buf: *mut ::core::ffi::c_char,
    mut size: size_t,
    mut d: int64_t,
) {
    let mut e: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(size >= 25 as size_t) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"size >= 25\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavutil/opt.c\0" as *const u8 as *const ::core::ffi::c_char,
            1008 as ::core::ffi::c_int,
        );
        abort();
    }
    if d < 0 as int64_t
        && d
            != -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
                - 0x7fffffffffffffff as ::core::ffi::c_long
    {
        let fresh12 = buf;
        buf = buf.offset(1);
        *fresh12 = '-' as i32 as ::core::ffi::c_char;
        size = size.wrapping_sub(1);
        d = -d;
    }
    if d == 0x7fffffffffffffff as ::core::ffi::c_long {
        snprintf(
            buf,
            size as size_t,
            b"INT64_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d
        == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
            - 0x7fffffffffffffff as ::core::ffi::c_long
    {
        snprintf(
            buf,
            size as size_t,
            b"INT64_MIN\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d > 3600 as ::core::ffi::c_int as int64_t * 1000000 as int64_t {
        snprintf(
            buf,
            size as size_t,
            b"%ld:%02d:%02d.%06d\0" as *const u8 as *const ::core::ffi::c_char,
            d as ::core::ffi::c_long / 3600000000 as ::core::ffi::c_long,
            (d / 60000000 as int64_t % 60 as int64_t) as ::core::ffi::c_int,
            (d / 1000000 as int64_t % 60 as int64_t) as ::core::ffi::c_int,
            (d % 1000000 as int64_t) as ::core::ffi::c_int,
        );
    } else if d > (60 as ::core::ffi::c_int * 1000000 as ::core::ffi::c_int) as int64_t {
        snprintf(
            buf,
            size as size_t,
            b"%d:%02d.%06d\0" as *const u8 as *const ::core::ffi::c_char,
            (d / 60000000 as int64_t) as ::core::ffi::c_int,
            (d / 1000000 as int64_t % 60 as int64_t) as ::core::ffi::c_int,
            (d % 1000000 as int64_t) as ::core::ffi::c_int,
        );
    } else {
        snprintf(
            buf,
            size as size_t,
            b"%d.%06d\0" as *const u8 as *const ::core::ffi::c_char,
            (d / 1000000 as int64_t) as ::core::ffi::c_int,
            (d % 1000000 as int64_t) as ::core::ffi::c_int,
        );
    }
    e = buf.offset(strlen(buf) as isize);
    while e > buf
        && *e.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == '0' as i32
    {
        e = e.offset(-1);
        *e = 0 as ::core::ffi::c_char;
    }
    if e > buf
        && *e.offset(-(1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == '.' as i32
    {
        e = e.offset(-1);
        *e = 0 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn opt_get_elem(
    mut o: *const AVOption,
    mut pbuf: *mut *mut uint8_t,
    mut buf_len: size_t,
    mut dst: *const ::core::ffi::c_void,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    match (*o).type_0 as ::core::ffi::c_uint
        & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint
    {
        18 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                get_bool_name(*(dst as *mut ::core::ffi::c_int)),
            );
        }
        1 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"0x%08X\0" as *const u8 as *const ::core::ffi::c_char,
                *(dst as *mut ::core::ffi::c_int),
            );
        }
        2 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"%d\0" as *const u8 as *const ::core::ffi::c_char,
                *(dst as *mut ::core::ffi::c_int),
            );
        }
        20 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"%u\0" as *const u8 as *const ::core::ffi::c_char,
                *(dst as *mut ::core::ffi::c_uint),
            );
        }
        3 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
                *(dst as *mut int64_t),
            );
        }
        10 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"%lu\0" as *const u8 as *const ::core::ffi::c_char,
                *(dst as *mut uint64_t),
            );
        }
        5 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"%f\0" as *const u8 as *const ::core::ffi::c_char,
                *(dst as *mut ::core::ffi::c_float) as ::core::ffi::c_double,
            );
        }
        4 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"%f\0" as *const u8 as *const ::core::ffi::c_char,
                *(dst as *mut ::core::ffi::c_double),
            );
        }
        15 | 7 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"%d/%d\0" as *const u8 as *const ::core::ffi::c_char,
                (*(dst as *mut AVRational)).num,
                (*(dst as *mut AVRational)).den,
            );
        }
        11 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
                (*o).default_val.i64_0,
            );
        }
        6 => {
            if !(*(dst as *mut *mut uint8_t)).is_null() {
                *pbuf = av_strdup(
                    *(dst as *mut *mut uint8_t) as *const ::core::ffi::c_char,
                ) as *mut uint8_t;
            } else if search_flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                != 0
            {
                *pbuf = ::core::ptr::null_mut::<uint8_t>();
                return 0 as ::core::ffi::c_int;
            } else {
                *pbuf = av_strdup(b"\0" as *const u8 as *const ::core::ffi::c_char)
                    as *mut uint8_t;
            }
            return if !(*pbuf).is_null() {
                0 as ::core::ffi::c_int
            } else {
                -(12 as ::core::ffi::c_int)
            };
        }
        8 => {
            let mut bin: *const uint8_t = ::core::ptr::null::<uint8_t>();
            let mut len: ::core::ffi::c_int = 0;
            if (*(dst as *mut *mut uint8_t)).is_null()
                && search_flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    != 0
            {
                *pbuf = ::core::ptr::null_mut::<uint8_t>();
                return 0 as ::core::ffi::c_int;
            }
            len = *((dst as *mut uint8_t)
                .offset(::core::mem::size_of::<*mut uint8_t>() as usize as isize)
                as *mut ::core::ffi::c_int);
            if (len as uint64_t).wrapping_mul(2 as uint64_t).wrapping_add(1 as uint64_t)
                > 2147483647 as uint64_t
            {
                return -(22 as ::core::ffi::c_int);
            }
            *pbuf = av_malloc(
                (len * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as size_t,
            ) as *mut uint8_t;
            if (*pbuf).is_null() {
                return -(12 as ::core::ffi::c_int);
            }
            if len == 0 {
                **pbuf.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as uint8_t;
                return 0 as ::core::ffi::c_int;
            }
            bin = *(dst as *mut *mut uint8_t);
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < len {
                snprintf(
                    (*pbuf).offset((i * 2 as ::core::ffi::c_int) as isize)
                        as *mut ::core::ffi::c_char,
                    3 as size_t,
                    b"%02X\0" as *const u8 as *const ::core::ffi::c_char,
                    *bin.offset(i as isize) as ::core::ffi::c_int,
                );
                i += 1;
            }
            return 0 as ::core::ffi::c_int;
        }
        12 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"%dx%d\0" as *const u8 as *const ::core::ffi::c_char,
                *(dst as *mut ::core::ffi::c_int)
                    .offset(0 as ::core::ffi::c_int as isize),
                *(dst as *mut ::core::ffi::c_int)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
        }
        13 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                av_x_if_null(
                    av_get_pix_fmt_name(*(dst as *mut AVPixelFormat))
                        as *const ::core::ffi::c_void,
                    b"none\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                ) as *mut ::core::ffi::c_char,
            );
        }
        14 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                av_x_if_null(
                    av_get_sample_fmt_name(*(dst as *mut AVSampleFormat))
                        as *const ::core::ffi::c_void,
                    b"none\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                ) as *mut ::core::ffi::c_char,
            );
        }
        16 => {
            let mut i64: int64_t = *(dst as *mut int64_t);
            format_duration(*pbuf as *mut ::core::ffi::c_char, buf_len, i64);
            ret = strlen(*pbuf as *const ::core::ffi::c_char) as ::core::ffi::c_int;
        }
        17 => {
            ret = snprintf(
                *pbuf as *mut ::core::ffi::c_char,
                buf_len as size_t,
                b"0x%02x%02x%02x%02x\0" as *const u8 as *const ::core::ffi::c_char,
                *(dst as *mut uint8_t).offset(0 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int,
                *(dst as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int,
                *(dst as *mut uint8_t).offset(2 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int,
                *(dst as *mut uint8_t).offset(3 as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int,
            );
        }
        19 => {
            ret = av_channel_layout_describe(
                dst as *const AVChannelLayout,
                *pbuf as *mut ::core::ffi::c_char,
                buf_len,
            );
        }
        9 => {
            if (*(dst as *mut *mut AVDictionary)).is_null()
                && search_flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    != 0
            {
                *pbuf = ::core::ptr::null_mut::<uint8_t>();
                return 0 as ::core::ffi::c_int;
            }
            return av_dict_get_string(
                *(dst as *mut *mut AVDictionary),
                pbuf as *mut *mut ::core::ffi::c_char,
                '=' as i32 as ::core::ffi::c_char,
                ':' as i32 as ::core::ffi::c_char,
            );
        }
        _ => return -(22 as ::core::ffi::c_int),
    }
    return ret;
}
unsafe extern "C" fn opt_get_array(
    mut o: *const AVOption,
    mut dst: *mut ::core::ffi::c_void,
    mut out_val: *mut *mut uint8_t,
) -> ::core::ffi::c_int {
    let count: ::core::ffi::c_uint = *opt_array_pcount(dst);
    let sep: uint8_t = opt_array_sep(o) as uint8_t;
    let mut str: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut str_len: size_t = 0 as size_t;
    let mut ret: ::core::ffi::c_int = 0;
    *out_val = ::core::ptr::null_mut::<uint8_t>();
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < count {
        let mut buf: [uint8_t; 128] = [0; 128];
        let mut out: *mut uint8_t = &raw mut buf as *mut uint8_t;
        let mut out_len: size_t = 0;
        ret = opt_get_elem(
            o,
            &raw mut out,
            ::core::mem::size_of::<[uint8_t; 128]>() as size_t,
            opt_array_pelem(o, *(dst as *mut *mut ::core::ffi::c_void), i),
            0 as ::core::ffi::c_int,
        );
        if !(ret < 0 as ::core::ffi::c_int) {
            out_len = strlen(out as *const ::core::ffi::c_char) as size_t;
            if out_len
                > (0xffffffffffffffff as ::core::ffi::c_ulong)
                    .wrapping_div(2 as ::core::ffi::c_ulong)
                    .wrapping_sub((i != 0) as ::core::ffi::c_int as ::core::ffi::c_ulong)
                || ((i != 0) as ::core::ffi::c_int as size_t)
                    .wrapping_add(out_len.wrapping_mul(2 as size_t))
                    > (0xffffffffffffffff as ::core::ffi::c_ulong)
                        .wrapping_sub(str_len as ::core::ffi::c_ulong)
                        .wrapping_sub(1 as ::core::ffi::c_ulong)
            {
                ret = -(34 as ::core::ffi::c_int);
            } else {
                ret = av_reallocp(
                    &raw mut str as *mut ::core::ffi::c_void,
                    str_len
                        .wrapping_add(1 as size_t)
                        .wrapping_add(out_len.wrapping_mul(2 as size_t))
                        .wrapping_add((i != 0) as ::core::ffi::c_int as size_t),
                );
                if !(ret < 0 as ::core::ffi::c_int) {
                    if i != 0 {
                        let fresh19 = str_len;
                        str_len = str_len.wrapping_add(1);
                        *str.offset(fresh19 as isize) = sep;
                    }
                    let mut j: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
                    while (j as size_t) < out_len {
                        let mut val: uint8_t = *out.offset(j as isize);
                        if val as ::core::ffi::c_int == sep as ::core::ffi::c_int
                            || val as ::core::ffi::c_int == '\\' as i32
                        {
                            let fresh20 = str_len;
                            str_len = str_len.wrapping_add(1);
                            *str.offset(fresh20 as isize) = '\\' as i32 as uint8_t;
                        }
                        let fresh21 = str_len;
                        str_len = str_len.wrapping_add(1);
                        *str.offset(fresh21 as isize) = val;
                        j = j.wrapping_add(1);
                    }
                    *str.offset(str_len as isize) = 0 as uint8_t;
                }
            }
        }
        if out != &raw mut buf as *mut uint8_t {
            av_freep(&raw mut out as *mut ::core::ffi::c_void);
        }
        if ret < 0 as ::core::ffi::c_int {
            av_freep(&raw mut str as *mut ::core::ffi::c_void);
            return ret;
        }
        i = i.wrapping_add(1);
    }
    *out_val = str;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_get(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut out_val: *mut *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut dst: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut target_obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut o: *const AVOption = av_opt_find2(
        obj,
        name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        search_flags,
        &raw mut target_obj,
    );
    let mut out: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut buf: [uint8_t; 128] = [0; 128];
    let mut ret: ::core::ffi::c_int = 0;
    if o.is_null() || target_obj.is_null()
        || (*o).offset <= 0 as ::core::ffi::c_int
            && (*o).type_0 as ::core::ffi::c_uint
                != AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return -(((0xf8 as ::core::ffi::c_int | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*o).flags & (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int != 0 {
        av_log(
            obj,
            24 as ::core::ffi::c_int,
            b"The \"%s\" option is deprecated: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
            (*o).help,
        );
    }
    dst = (target_obj as *mut uint8_t).offset((*o).offset as isize)
        as *mut ::core::ffi::c_void;
    if (*o).type_0 as ::core::ffi::c_uint
        & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        ret = opt_get_array(o, dst, out_val);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        if (*out_val).is_null()
            && search_flags & (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int == 0
        {
            *out_val = av_strdup(b"\0" as *const u8 as *const ::core::ffi::c_char)
                as *mut uint8_t;
            if (*out_val).is_null() {
                return -(12 as ::core::ffi::c_int);
            }
        }
        return 0 as ::core::ffi::c_int;
    }
    buf[0 as ::core::ffi::c_int as usize] = 0 as uint8_t;
    out = &raw mut buf as *mut uint8_t;
    ret = opt_get_elem(
        o,
        &raw mut out,
        ::core::mem::size_of::<[uint8_t; 128]>() as size_t,
        dst,
        search_flags,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if out != &raw mut buf as *mut uint8_t {
        *out_val = out;
        return 0 as ::core::ffi::c_int;
    }
    if ret as usize >= ::core::mem::size_of::<[uint8_t; 128]>() as usize {
        return -(22 as ::core::ffi::c_int);
    }
    *out_val = av_strdup(out as *const ::core::ffi::c_char) as *mut uint8_t;
    return if !(*out_val).is_null() {
        0 as ::core::ffi::c_int
    } else {
        -(12 as ::core::ffi::c_int)
    };
}
unsafe extern "C" fn get_number(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut num: *mut ::core::ffi::c_double,
    mut den: *mut ::core::ffi::c_int,
    mut intnum: *mut int64_t,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dst: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut target_obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut o: *const AVOption = av_opt_find2(
        obj,
        name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        search_flags,
        &raw mut target_obj,
    );
    if o.is_null() || target_obj.is_null() {
        return -(((0xf8 as ::core::ffi::c_int | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*o).type_0 as ::core::ffi::c_uint
        & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    dst = (target_obj as *mut uint8_t).offset((*o).offset as isize)
        as *mut ::core::ffi::c_void;
    return read_number(o, dst, num, den, intnum);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_get_int(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut out_val: *mut int64_t,
) -> ::core::ffi::c_int {
    let mut intnum: int64_t = 1 as int64_t;
    let mut num: ::core::ffi::c_double = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    let mut ret: ::core::ffi::c_int = 0;
    let mut den: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    ret = get_number(
        obj,
        name,
        &raw mut num,
        &raw mut den,
        &raw mut intnum,
        search_flags,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if num == den as ::core::ffi::c_double {
        *out_val = intnum;
    } else {
        *out_val = (num * intnum as ::core::ffi::c_double / den as ::core::ffi::c_double)
            as int64_t;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_get_double(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut out_val: *mut ::core::ffi::c_double,
) -> ::core::ffi::c_int {
    let mut intnum: int64_t = 1 as int64_t;
    let mut num: ::core::ffi::c_double = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    let mut ret: ::core::ffi::c_int = 0;
    let mut den: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    ret = get_number(
        obj,
        name,
        &raw mut num,
        &raw mut den,
        &raw mut intnum,
        search_flags,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    *out_val = num * intnum as ::core::ffi::c_double / den as ::core::ffi::c_double;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_get_q(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut out_val: *mut AVRational,
) -> ::core::ffi::c_int {
    let mut intnum: int64_t = 1 as int64_t;
    let mut num: ::core::ffi::c_double = 1 as ::core::ffi::c_int
        as ::core::ffi::c_double;
    let mut ret: ::core::ffi::c_int = 0;
    let mut den: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    ret = get_number(
        obj,
        name,
        &raw mut num,
        &raw mut den,
        &raw mut intnum,
        search_flags,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if num == 1.0f64 && intnum as ::core::ffi::c_int as int64_t == intnum {
        *out_val = AVRational {
            num: intnum as ::core::ffi::c_int,
            den: den,
        };
    } else {
        *out_val = double_to_rational(
            num * intnum as ::core::ffi::c_double / den as ::core::ffi::c_double,
        );
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_get_image_size(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut w_out: *mut ::core::ffi::c_int,
    mut h_out: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut dst: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut target_obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut o: *const AVOption = av_opt_find2(
        obj,
        name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        search_flags,
        &raw mut target_obj,
    );
    if o.is_null() || target_obj.is_null() {
        return -(((0xf8 as ::core::ffi::c_int | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*o).type_0 as ::core::ffi::c_uint
        != AV_OPT_TYPE_IMAGE_SIZE as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        av_log(
            obj,
            16 as ::core::ffi::c_int,
            b"The value for option '%s' is not a image size.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
        );
        return -(22 as ::core::ffi::c_int);
    }
    dst = (target_obj as *mut uint8_t).offset((*o).offset as isize)
        as *mut ::core::ffi::c_void;
    if !w_out.is_null() {
        *w_out = *(dst as *mut ::core::ffi::c_int);
    }
    if !h_out.is_null() {
        *h_out = *(dst as *mut ::core::ffi::c_int)
            .offset(1 as ::core::ffi::c_int as isize);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_get_video_rate(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut out_val: *mut AVRational,
) -> ::core::ffi::c_int {
    return av_opt_get_q(obj, name, search_flags, out_val);
}
unsafe extern "C" fn get_format(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut out_fmt: *mut ::core::ffi::c_void,
    mut type_0: AVOptionType,
    mut desc: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut dst: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut target_obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut o: *const AVOption = av_opt_find2(
        obj,
        name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        search_flags,
        &raw mut target_obj,
    );
    if o.is_null() || target_obj.is_null() {
        return -(((0xf8 as ::core::ffi::c_int | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*o).type_0 as ::core::ffi::c_uint != type_0 as ::core::ffi::c_uint {
        av_log(
            obj,
            16 as ::core::ffi::c_int,
            b"The value for option '%s' is not a %s format.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            desc,
            name,
        );
        return -(22 as ::core::ffi::c_int);
    }
    dst = (target_obj as *mut uint8_t).offset((*o).offset as isize)
        as *mut ::core::ffi::c_void;
    if type_0 as ::core::ffi::c_uint
        == AV_OPT_TYPE_PIXEL_FMT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *(out_fmt as *mut AVPixelFormat) = *(dst as *mut AVPixelFormat);
    } else {
        *(out_fmt as *mut AVSampleFormat) = *(dst as *mut AVSampleFormat);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_get_pixel_fmt(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut out_fmt: *mut AVPixelFormat,
) -> ::core::ffi::c_int {
    return get_format(
        obj,
        name,
        search_flags,
        out_fmt as *mut ::core::ffi::c_void,
        AV_OPT_TYPE_PIXEL_FMT,
        b"pixel\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_get_sample_fmt(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut out_fmt: *mut AVSampleFormat,
) -> ::core::ffi::c_int {
    return get_format(
        obj,
        name,
        search_flags,
        out_fmt as *mut ::core::ffi::c_void,
        AV_OPT_TYPE_SAMPLE_FMT,
        b"sample\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_get_chlayout(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut cl: *mut AVChannelLayout,
) -> ::core::ffi::c_int {
    let mut dst: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut target_obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut o: *const AVOption = av_opt_find2(
        obj,
        name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        search_flags,
        &raw mut target_obj,
    );
    if o.is_null() || target_obj.is_null() {
        return -(((0xf8 as ::core::ffi::c_int | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*o).type_0 as ::core::ffi::c_uint
        != AV_OPT_TYPE_CHLAYOUT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        av_log(
            obj,
            16 as ::core::ffi::c_int,
            b"The value for option '%s' is not a channel layout.\n\0" as *const u8
                as *const ::core::ffi::c_char,
            name,
        );
        return -(22 as ::core::ffi::c_int);
    }
    dst = (target_obj as *mut uint8_t).offset((*o).offset as isize)
        as *mut ::core::ffi::c_void;
    return av_channel_layout_copy(cl, dst as *const AVChannelLayout);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_get_dict_val(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut out_val: *mut *mut AVDictionary,
) -> ::core::ffi::c_int {
    let mut target_obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut src: *mut AVDictionary = ::core::ptr::null_mut::<AVDictionary>();
    let mut o: *const AVOption = av_opt_find2(
        obj,
        name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        search_flags,
        &raw mut target_obj,
    );
    if o.is_null() || target_obj.is_null() {
        return -(((0xf8 as ::core::ffi::c_int | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*o).type_0 as ::core::ffi::c_uint
        != AV_OPT_TYPE_DICT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return -(22 as ::core::ffi::c_int);
    }
    src = *((target_obj as *mut uint8_t).offset((*o).offset as isize)
        as *mut *mut AVDictionary);
    return av_dict_copy(out_val, src, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_flag_is_set(
    mut obj: *mut ::core::ffi::c_void,
    mut field_name: *const ::core::ffi::c_char,
    mut flag_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut field: *const AVOption = av_opt_find(
        obj,
        field_name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    let mut flag: *const AVOption = av_opt_find(
        obj,
        flag_name,
        if !field.is_null() {
            (*field).unit
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        },
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    let mut res: int64_t = 0;
    if field.is_null() || flag.is_null()
        || (*flag).type_0 as ::core::ffi::c_uint
            != AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
        || av_opt_get_int(obj, field_name, 0 as ::core::ffi::c_int, &raw mut res)
            < 0 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    return (res & (*flag).default_val.i64_0) as ::core::ffi::c_int;
}
unsafe extern "C" fn log_int_value(
    mut av_log_obj: *mut ::core::ffi::c_void,
    mut level: ::core::ffi::c_int,
    mut i: int64_t,
) {
    if i == 2147483647 as int64_t {
        av_log(
            av_log_obj,
            level,
            b"INT_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if i
        == (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int) as int64_t
    {
        av_log(
            av_log_obj,
            level,
            b"INT_MIN\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if i == 0xffffffff as ::core::ffi::c_uint as int64_t {
        av_log(
            av_log_obj,
            level,
            b"UINT32_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if i == 0x7fffffffffffffff as ::core::ffi::c_long {
        av_log(
            av_log_obj,
            level,
            b"I64_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if i
        == -(1 as ::core::ffi::c_int) as ::core::ffi::c_long
            - 0x7fffffffffffffff as ::core::ffi::c_long
    {
        av_log(
            av_log_obj,
            level,
            b"I64_MIN\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        av_log(
            av_log_obj,
            level,
            b"%ld\0" as *const u8 as *const ::core::ffi::c_char,
            i,
        );
    };
}
unsafe extern "C" fn log_value(
    mut av_log_obj: *mut ::core::ffi::c_void,
    mut level: ::core::ffi::c_int,
    mut d: ::core::ffi::c_double,
) {
    if d == 2147483647 as ::core::ffi::c_int as ::core::ffi::c_double {
        av_log(
            av_log_obj,
            level,
            b"INT_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d
        == (-(2147483647 as ::core::ffi::c_int) - 1 as ::core::ffi::c_int)
            as ::core::ffi::c_double
    {
        av_log(
            av_log_obj,
            level,
            b"INT_MIN\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d == 0xffffffff as ::core::ffi::c_uint as ::core::ffi::c_double {
        av_log(
            av_log_obj,
            level,
            b"UINT32_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d == 0x7fffffffffffffff as ::core::ffi::c_long as ::core::ffi::c_double {
        av_log(
            av_log_obj,
            level,
            b"I64_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d
        == (-(1 as ::core::ffi::c_int) as ::core::ffi::c_long
            - 0x7fffffffffffffff as ::core::ffi::c_long) as ::core::ffi::c_double
    {
        av_log(
            av_log_obj,
            level,
            b"I64_MIN\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d == 3.40282347e+38f64 {
        av_log(
            av_log_obj,
            level,
            b"FLT_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d == 1.17549435e-38f64 {
        av_log(
            av_log_obj,
            level,
            b"FLT_MIN\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d == -3.40282347e+38f32 as ::core::ffi::c_double {
        av_log(
            av_log_obj,
            level,
            b"-FLT_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d == -1.17549435e-38f32 as ::core::ffi::c_double {
        av_log(
            av_log_obj,
            level,
            b"-FLT_MIN\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d == 1.7976931348623157e+308f64 {
        av_log(
            av_log_obj,
            level,
            b"DBL_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d == 2.2250738585072014e-308f64 {
        av_log(
            av_log_obj,
            level,
            b"DBL_MIN\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d == -1.7976931348623157e+308f64 {
        av_log(
            av_log_obj,
            level,
            b"-DBL_MAX\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else if d == -2.2250738585072014e-308f64 {
        av_log(
            av_log_obj,
            level,
            b"-DBL_MIN\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        av_log(av_log_obj, level, b"%g\0" as *const u8 as *const ::core::ffi::c_char, d);
    };
}
unsafe extern "C" fn get_opt_const_name(
    mut obj: *mut ::core::ffi::c_void,
    mut unit: *const ::core::ffi::c_char,
    mut value: int64_t,
) -> *const ::core::ffi::c_char {
    let mut opt: *const AVOption = ::core::ptr::null::<AVOption>();
    if unit.is_null() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    loop {
        opt = av_opt_next(obj, opt);
        if opt.is_null() {
            break;
        }
        if (*opt).type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
            && strcmp((*opt).unit, unit) == 0 && (*opt).default_val.i64_0 == value
        {
            return (*opt).name;
        }
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
unsafe extern "C" fn get_opt_flags_string(
    mut obj: *mut ::core::ffi::c_void,
    mut unit: *const ::core::ffi::c_char,
    mut value: int64_t,
) -> *mut ::core::ffi::c_char {
    let mut opt: *const AVOption = ::core::ptr::null::<AVOption>();
    let mut flags: [::core::ffi::c_char; 512] = [0; 512];
    flags[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    if unit.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    loop {
        opt = av_opt_next(obj, opt);
        if opt.is_null() {
            break;
        }
        if (*opt).type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
            && strcmp((*opt).unit, unit) == 0 && (*opt).default_val.i64_0 & value != 0
        {
            if flags[0 as ::core::ffi::c_int as usize] != 0 {
                av_strlcatf(
                    &raw mut flags as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
                    b"+\0" as *const u8 as *const ::core::ffi::c_char,
                );
            }
            av_strlcatf(
                &raw mut flags as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                (*opt).name,
            );
        }
    }
    if flags[0 as ::core::ffi::c_int as usize] != 0 {
        return av_strdup(&raw mut flags as *mut ::core::ffi::c_char);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn log_type(
    mut av_log_obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut parent_type: AVOptionType,
) {
    let type_0: AVOptionType = ((*o).type_0 as ::core::ffi::c_uint
        & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint)
        as AVOptionType;
    if (*o).type_0 as ::core::ffi::c_uint
        == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
        && (parent_type as ::core::ffi::c_uint
            & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint
            == AV_OPT_TYPE_INT as ::core::ffi::c_int as ::core::ffi::c_uint
            || parent_type as ::core::ffi::c_uint
                & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint
                == AV_OPT_TYPE_INT64 as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        av_log(
            av_log_obj,
            32 as ::core::ffi::c_int,
            b"%-12ld \0" as *const u8 as *const ::core::ffi::c_char,
            (*o).default_val.i64_0,
        );
    } else if (type_0 as usize)
        < (::core::mem::size_of::<[C2RustUnnamed_1; 21]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_1>() as usize)
        && !opt_type_desc[type_0 as usize].name.is_null()
    {
        if (*o).type_0 as ::core::ffi::c_uint
            & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        {
            av_log(
                av_log_obj,
                32 as ::core::ffi::c_int,
                b"[%-10s]\0" as *const u8 as *const ::core::ffi::c_char,
                opt_type_desc[type_0 as usize].name,
            );
        } else {
            av_log(
                av_log_obj,
                32 as ::core::ffi::c_int,
                b"%-12s \0" as *const u8 as *const ::core::ffi::c_char,
                opt_type_desc[type_0 as usize].name,
            );
        }
    } else {
        av_log(
            av_log_obj,
            32 as ::core::ffi::c_int,
            b"%-12s \0" as *const u8 as *const ::core::ffi::c_char,
            b"\0" as *const u8 as *const ::core::ffi::c_char,
        );
    };
}
unsafe extern "C" fn log_default(
    mut obj: *mut ::core::ffi::c_void,
    mut av_log_obj: *mut ::core::ffi::c_void,
    mut opt: *const AVOption,
) {
    if (*opt).type_0 as ::core::ffi::c_uint
        == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*opt).type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_BINARY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if ((*opt).type_0 as ::core::ffi::c_uint
        == AV_OPT_TYPE_COLOR as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*opt).type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_IMAGE_SIZE as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*opt).type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_STRING as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*opt).type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_DICT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*opt).type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_CHLAYOUT as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*opt).type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_VIDEO_RATE as ::core::ffi::c_int as ::core::ffi::c_uint)
        && (*opt).default_val.str_0.is_null()
    {
        return;
    }
    if (*opt).type_0 as ::core::ffi::c_uint
        & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        let mut arr: *const AVOptionArrayDef = (*opt).default_val.arr;
        if !arr.is_null() && !(*arr).def.is_null() {
            av_log(
                av_log_obj,
                32 as ::core::ffi::c_int,
                b" (default %s)\0" as *const u8 as *const ::core::ffi::c_char,
                (*arr).def,
            );
        }
        return;
    }
    av_log(
        av_log_obj,
        32 as ::core::ffi::c_int,
        b" (default \0" as *const u8 as *const ::core::ffi::c_char,
    );
    match (*opt).type_0 as ::core::ffi::c_uint {
        18 => {
            av_log(
                av_log_obj,
                32 as ::core::ffi::c_int,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                get_bool_name((*opt).default_val.i64_0 as ::core::ffi::c_int),
            );
        }
        1 => {
            let mut def_flags: *mut ::core::ffi::c_char = get_opt_flags_string(
                obj,
                (*opt).unit,
                (*opt).default_val.i64_0,
            );
            if !def_flags.is_null() {
                av_log(
                    av_log_obj,
                    32 as ::core::ffi::c_int,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    def_flags,
                );
                av_freep(&raw mut def_flags as *mut ::core::ffi::c_void);
            } else {
                av_log(
                    av_log_obj,
                    32 as ::core::ffi::c_int,
                    b"%lX\0" as *const u8 as *const ::core::ffi::c_char,
                    (*opt).default_val.i64_0,
                );
            }
        }
        16 => {
            let mut buf: [::core::ffi::c_char; 25] = [0; 25];
            format_duration(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 25]>() as size_t,
                (*opt).default_val.i64_0,
            );
            av_log(
                av_log_obj,
                32 as ::core::ffi::c_int,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
        }
        20 | 2 | 10 | 3 => {
            let mut def_const: *const ::core::ffi::c_char = get_opt_const_name(
                obj,
                (*opt).unit,
                (*opt).default_val.i64_0,
            );
            if !def_const.is_null() {
                av_log(
                    av_log_obj,
                    32 as ::core::ffi::c_int,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    def_const,
                );
            } else {
                log_int_value(
                    av_log_obj,
                    32 as ::core::ffi::c_int,
                    (*opt).default_val.i64_0,
                );
            }
        }
        4 | 5 => {
            log_value(av_log_obj, 32 as ::core::ffi::c_int, (*opt).default_val.dbl);
        }
        7 => {
            let mut q: AVRational = av_d2q(
                (*opt).default_val.dbl,
                2147483647 as ::core::ffi::c_int,
            );
            av_log(
                av_log_obj,
                32 as ::core::ffi::c_int,
                b"%d/%d\0" as *const u8 as *const ::core::ffi::c_char,
                q.num,
                q.den,
            );
        }
        13 => {
            av_log(
                av_log_obj,
                32 as ::core::ffi::c_int,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                av_x_if_null(
                    av_get_pix_fmt_name((*opt).default_val.i64_0 as AVPixelFormat)
                        as *const ::core::ffi::c_void,
                    b"none\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                ) as *mut ::core::ffi::c_char,
            );
        }
        14 => {
            av_log(
                av_log_obj,
                32 as ::core::ffi::c_int,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                av_x_if_null(
                    av_get_sample_fmt_name((*opt).default_val.i64_0 as AVSampleFormat)
                        as *const ::core::ffi::c_void,
                    b"none\0" as *const u8 as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                ) as *mut ::core::ffi::c_char,
            );
        }
        17 | 12 | 6 | 9 | 15 | 19 => {
            av_log(
                av_log_obj,
                32 as ::core::ffi::c_int,
                b"\"%s\"\0" as *const u8 as *const ::core::ffi::c_char,
                (*opt).default_val.str_0,
            );
        }
        _ => {}
    }
    av_log(
        av_log_obj,
        32 as ::core::ffi::c_int,
        b")\0" as *const u8 as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn opt_list(
    mut obj: *mut ::core::ffi::c_void,
    mut av_log_obj: *mut ::core::ffi::c_void,
    mut unit: *const ::core::ffi::c_char,
    mut req_flags: ::core::ffi::c_int,
    mut rej_flags: ::core::ffi::c_int,
    mut parent_type: AVOptionType,
) {
    let mut opt: *const AVOption = ::core::ptr::null::<AVOption>();
    let mut r: *mut AVOptionRanges = ::core::ptr::null_mut::<AVOptionRanges>();
    let mut i: ::core::ffi::c_int = 0;
    loop {
        opt = av_opt_next(obj, opt);
        if opt.is_null() {
            break;
        }
        if (*opt).flags & req_flags == 0 || (*opt).flags & rej_flags != 0 {
            continue;
        }
        if unit.is_null()
            && (*opt).type_0 as ::core::ffi::c_uint
                == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            continue;
        }
        if !unit.is_null()
            && (*opt).type_0 as ::core::ffi::c_uint
                != AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            continue;
        }
        if !unit.is_null()
            && (*opt).type_0 as ::core::ffi::c_uint
                == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
            && strcmp(unit, (*opt).unit) != 0
        {
            continue;
        }
        if !unit.is_null()
            && (*opt).type_0 as ::core::ffi::c_uint
                == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            av_log(
                av_log_obj,
                32 as ::core::ffi::c_int,
                b"     %-15s \0" as *const u8 as *const ::core::ffi::c_char,
                (*opt).name,
            );
        } else {
            av_log(
                av_log_obj,
                32 as ::core::ffi::c_int,
                b"  %s%-17s \0" as *const u8 as *const ::core::ffi::c_char,
                if (*opt).flags & (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int
                    != 0
                {
                    b" \0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"-\0" as *const u8 as *const ::core::ffi::c_char
                },
                (*opt).name,
            );
        }
        log_type(av_log_obj, opt, parent_type);
        av_log(
            av_log_obj,
            32 as ::core::ffi::c_int,
            b"%c%c%c%c%c%c%c%c%c%c%c\0" as *const u8 as *const ::core::ffi::c_char,
            if (*opt).flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
                'E' as i32
            } else {
                '.' as i32
            },
            if (*opt).flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
                'D' as i32
            } else {
                '.' as i32
            },
            if (*opt).flags & (1 as ::core::ffi::c_int) << 16 as ::core::ffi::c_int != 0
            {
                'F' as i32
            } else {
                '.' as i32
            },
            if (*opt).flags & (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int != 0 {
                'V' as i32
            } else {
                '.' as i32
            },
            if (*opt).flags & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
                'A' as i32
            } else {
                '.' as i32
            },
            if (*opt).flags & (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int != 0 {
                'S' as i32
            } else {
                '.' as i32
            },
            if (*opt).flags & (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int != 0 {
                'X' as i32
            } else {
                '.' as i32
            },
            if (*opt).flags & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0 {
                'R' as i32
            } else {
                '.' as i32
            },
            if (*opt).flags & (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int != 0 {
                'B' as i32
            } else {
                '.' as i32
            },
            if (*opt).flags & (1 as ::core::ffi::c_int) << 15 as ::core::ffi::c_int != 0
            {
                'T' as i32
            } else {
                '.' as i32
            },
            if (*opt).flags & (1 as ::core::ffi::c_int) << 17 as ::core::ffi::c_int != 0
            {
                'P' as i32
            } else {
                '.' as i32
            },
        );
        if !(*opt).help.is_null() {
            av_log(
                av_log_obj,
                32 as ::core::ffi::c_int,
                b" %s\0" as *const u8 as *const ::core::ffi::c_char,
                (*opt).help,
            );
        }
        if av_opt_query_ranges(
            &raw mut r,
            obj,
            (*opt).name,
            (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int,
        ) >= 0 as ::core::ffi::c_int
        {
            match (*opt).type_0 as ::core::ffi::c_uint {
                2 | 20 | 3 | 10 | 4 | 5 | 7 => {
                    i = 0 as ::core::ffi::c_int;
                    while i < (*r).nb_ranges {
                        av_log(
                            av_log_obj,
                            32 as ::core::ffi::c_int,
                            b" (from \0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        log_value(
                            av_log_obj,
                            32 as ::core::ffi::c_int,
                            (**(*r).range.offset(i as isize)).value_min,
                        );
                        av_log(
                            av_log_obj,
                            32 as ::core::ffi::c_int,
                            b" to \0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        log_value(
                            av_log_obj,
                            32 as ::core::ffi::c_int,
                            (**(*r).range.offset(i as isize)).value_max,
                        );
                        av_log(
                            av_log_obj,
                            32 as ::core::ffi::c_int,
                            b")\0" as *const u8 as *const ::core::ffi::c_char,
                        );
                        i += 1;
                    }
                }
                _ => {}
            }
            av_opt_freep_ranges(&raw mut r);
        }
        log_default(obj, av_log_obj, opt);
        av_log(
            av_log_obj,
            32 as ::core::ffi::c_int,
            b"\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if !(*opt).unit.is_null()
            && (*opt).type_0 as ::core::ffi::c_uint
                != AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            opt_list(obj, av_log_obj, (*opt).unit, req_flags, rej_flags, (*opt).type_0);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_show2(
    mut obj: *mut ::core::ffi::c_void,
    mut av_log_obj: *mut ::core::ffi::c_void,
    mut req_flags: ::core::ffi::c_int,
    mut rej_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if obj.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    av_log(
        av_log_obj,
        32 as ::core::ffi::c_int,
        b"%s AVOptions:\n\0" as *const u8 as *const ::core::ffi::c_char,
        (**(obj as *mut *mut AVClass)).class_name,
    );
    opt_list(
        obj,
        av_log_obj,
        ::core::ptr::null::<::core::ffi::c_char>(),
        req_flags,
        rej_flags,
        4294967295 as AVOptionType,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_defaults(mut s: *mut ::core::ffi::c_void) {
    av_opt_set_defaults2(s, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_defaults2(
    mut s: *mut ::core::ffi::c_void,
    mut mask: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
) {
    let mut opt: *const AVOption = ::core::ptr::null::<AVOption>();
    loop {
        opt = av_opt_next(s, opt);
        if opt.is_null() {
            break;
        }
        let mut dst: *mut ::core::ffi::c_void = (s as *mut uint8_t)
            .offset((*opt).offset as isize) as *mut ::core::ffi::c_void;
        if (*opt).flags & mask != flags {
            continue;
        }
        if (*opt).flags & (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int != 0 {
            continue;
        }
        if (*opt).type_0 as ::core::ffi::c_uint
            & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        {
            let mut arr: *const AVOptionArrayDef = (*opt).default_val.arr;
            let sep: ::core::ffi::c_char = opt_array_sep(opt) as ::core::ffi::c_char;
            if !(sep as ::core::ffi::c_int != 0
                && sep as ::core::ffi::c_int != '\\' as i32
                && ((sep as ::core::ffi::c_int) < 'a' as i32
                    || sep as ::core::ffi::c_int > 'z' as i32)
                && ((sep as ::core::ffi::c_int) < 'A' as i32
                    || sep as ::core::ffi::c_int > 'Z' as i32)
                && ((sep as ::core::ffi::c_int) < '0' as i32
                    || sep as ::core::ffi::c_int > '9' as i32))
            {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    0 as ::core::ffi::c_int,
                    b"Assertion %s failed at %s:%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"sep && sep != '\\\\' && (sep < 'a' || sep > 'z') && (sep < 'A' || sep > 'Z') && (sep < '0' || sep > '9')\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    b"src/libavutil/opt.c\0" as *const u8 as *const ::core::ffi::c_char,
                    1695 as ::core::ffi::c_int,
                );
                abort();
            }
            if !arr.is_null() && !(*arr).def.is_null() {
                opt_set_array(s, s, opt, (*arr).def, dst);
            }
        } else {
            match (*opt).type_0 as ::core::ffi::c_uint {
                11 => {}
                18 | 1 | 2 | 20 | 3 | 10 | 16 | 13 | 14 => {
                    write_number(
                        s,
                        opt,
                        dst,
                        1 as ::core::ffi::c_int as ::core::ffi::c_double,
                        1 as ::core::ffi::c_int,
                        (*opt).default_val.i64_0,
                    );
                }
                4 | 5 => {
                    let mut val: ::core::ffi::c_double = 0.;
                    val = (*opt).default_val.dbl;
                    write_number(
                        s,
                        opt,
                        dst,
                        val,
                        1 as ::core::ffi::c_int,
                        1 as int64_t,
                    );
                }
                7 => {
                    let mut val_0: AVRational = AVRational { num: 0, den: 0 };
                    val_0 = av_d2q(
                        (*opt).default_val.dbl,
                        2147483647 as ::core::ffi::c_int,
                    );
                    write_number(
                        s,
                        opt,
                        dst,
                        1 as ::core::ffi::c_int as ::core::ffi::c_double,
                        val_0.den,
                        val_0.num as int64_t,
                    );
                }
                17 => {
                    set_string_color(
                        s,
                        opt,
                        (*opt).default_val.str_0,
                        dst as *mut uint8_t,
                    );
                }
                6 => {
                    set_string(
                        s,
                        opt,
                        (*opt).default_val.str_0,
                        dst as *mut *mut uint8_t,
                    );
                }
                12 => {
                    set_string_image_size(
                        s,
                        opt,
                        (*opt).default_val.str_0,
                        dst as *mut ::core::ffi::c_int,
                    );
                }
                15 => {
                    set_string_video_rate(
                        s,
                        opt,
                        (*opt).default_val.str_0,
                        dst as *mut AVRational,
                    );
                }
                8 => {
                    set_string_binary(
                        s,
                        opt,
                        (*opt).default_val.str_0,
                        dst as *mut *mut uint8_t,
                    );
                }
                19 => {
                    set_string_channel_layout(s, opt, (*opt).default_val.str_0, dst);
                }
                9 => {
                    set_string_dict(
                        s,
                        opt,
                        (*opt).default_val.str_0,
                        dst as *mut *mut uint8_t,
                    );
                }
                _ => {
                    av_log(
                        s,
                        48 as ::core::ffi::c_int,
                        b"AVOption type %d of option %s not implemented yet\n\0"
                            as *const u8 as *const ::core::ffi::c_char,
                        (*opt).type_0 as ::core::ffi::c_uint,
                        (*opt).name,
                    );
                }
            }
        }
    };
}
unsafe extern "C" fn parse_key_value_pair(
    mut ctx: *mut ::core::ffi::c_void,
    mut buf: *mut *const ::core::ffi::c_char,
    mut key_val_sep: *const ::core::ffi::c_char,
    mut pairs_sep: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut key: *mut ::core::ffi::c_char = av_get_token(buf, key_val_sep);
    let mut val: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut ret: ::core::ffi::c_int = 0;
    if key.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    if *key as ::core::ffi::c_int != 0 && strspn(*buf, key_val_sep) != 0 {
        *buf = (*buf).offset(1);
        val = av_get_token(buf, pairs_sep);
        if val.is_null() {
            av_freep(&raw mut key as *mut ::core::ffi::c_void);
            return -(12 as ::core::ffi::c_int);
        }
    } else {
        av_log(
            ctx,
            16 as ::core::ffi::c_int,
            b"Missing key or no key/value separator found after key '%s'\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            key,
        );
        av_free(key as *mut ::core::ffi::c_void);
        return -(22 as ::core::ffi::c_int);
    }
    av_log(
        ctx,
        48 as ::core::ffi::c_int,
        b"Setting entry with key '%s' to value '%s'\n\0" as *const u8
            as *const ::core::ffi::c_char,
        key,
        val,
    );
    ret = av_opt_set(
        ctx,
        key,
        val,
        (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
    );
    if ret
        == -(((0xf8 as ::core::ffi::c_int | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int)
    {
        av_log(
            ctx,
            16 as ::core::ffi::c_int,
            b"Key '%s' not found.\n\0" as *const u8 as *const ::core::ffi::c_char,
            key,
        );
    }
    av_free(key as *mut ::core::ffi::c_void);
    av_free(val as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_set_options_string(
    mut ctx: *mut ::core::ffi::c_void,
    mut opts: *const ::core::ffi::c_char,
    mut key_val_sep: *const ::core::ffi::c_char,
    mut pairs_sep: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if opts.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    while *opts != 0 {
        ret = parse_key_value_pair(ctx, &raw mut opts, key_val_sep, pairs_sep);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        count += 1;
        if *opts != 0 {
            opts = opts.offset(1);
        }
    }
    return count;
}
unsafe extern "C" fn is_key_char(mut c: ::core::ffi::c_char) -> ::core::ffi::c_int {
    return ((((c as ::core::ffi::c_int | 32 as ::core::ffi::c_int) - 'a' as i32)
        as ::core::ffi::c_uint) < 26 as ::core::ffi::c_uint
        || ((c as ::core::ffi::c_int - '0' as i32) as ::core::ffi::c_uint)
            < 10 as ::core::ffi::c_uint || c as ::core::ffi::c_int == '-' as i32
        || c as ::core::ffi::c_int == '_' as i32 || c as ::core::ffi::c_int == '/' as i32
        || c as ::core::ffi::c_int == '.' as i32) as ::core::ffi::c_int;
}
unsafe extern "C" fn get_key(
    mut ropts: *mut *const ::core::ffi::c_char,
    mut delim: *const ::core::ffi::c_char,
    mut rkey: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut opts: *const ::core::ffi::c_char = *ropts;
    let mut key_start: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut key_end: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    opts = opts
        .offset(
            strspn(opts, b" \n\t\r\0" as *const u8 as *const ::core::ffi::c_char)
                as isize,
        );
    key_start = opts;
    while is_key_char(*opts) != 0 {
        opts = opts.offset(1);
    }
    key_end = opts;
    opts = opts
        .offset(
            strspn(opts, b" \n\t\r\0" as *const u8 as *const ::core::ffi::c_char)
                as isize,
        );
    if *opts == 0 || strchr(delim, *opts as ::core::ffi::c_int).is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    opts = opts.offset(1);
    *rkey = av_malloc(
        (key_end.offset_from(key_start) as ::core::ffi::c_long
            + 1 as ::core::ffi::c_long) as size_t,
    ) as *mut ::core::ffi::c_char;
    if (*rkey).is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    memcpy(
        *rkey as *mut ::core::ffi::c_void,
        key_start as *const ::core::ffi::c_void,
        key_end.offset_from(key_start) as ::core::ffi::c_long as size_t,
    );
    *(*rkey).offset(key_end.offset_from(key_start) as ::core::ffi::c_long as isize) = 0
        as ::core::ffi::c_char;
    *ropts = opts;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_get_key_value(
    mut ropts: *mut *const ::core::ffi::c_char,
    mut key_val_sep: *const ::core::ffi::c_char,
    mut pairs_sep: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_uint,
    mut rkey: *mut *mut ::core::ffi::c_char,
    mut rval: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut val: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut opts: *const ::core::ffi::c_char = *ropts;
    ret = get_key(&raw mut opts, key_val_sep, &raw mut key);
    if ret < 0 as ::core::ffi::c_int
        && flags & AV_OPT_FLAG_IMPLICIT_KEY as ::core::ffi::c_int as ::core::ffi::c_uint
            == 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    val = av_get_token(&raw mut opts, pairs_sep);
    if val.is_null() {
        av_free(key as *mut ::core::ffi::c_void);
        return -(12 as ::core::ffi::c_int);
    }
    *ropts = opts;
    *rkey = key;
    *rval = val;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_from_string(
    mut ctx: *mut ::core::ffi::c_void,
    mut opts: *const ::core::ffi::c_char,
    mut shorthand: *const *const ::core::ffi::c_char,
    mut key_val_sep: *const ::core::ffi::c_char,
    mut pairs_sep: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dummy_shorthand: *const ::core::ffi::c_char = ::core::ptr::null::<
        ::core::ffi::c_char,
    >();
    let mut key: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if opts.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    if shorthand.is_null() {
        shorthand = &raw mut dummy_shorthand;
    }
    while *opts != 0 {
        let mut parsed_key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
            ::core::ffi::c_char,
        >();
        ret = av_opt_get_key_value(
            &raw mut opts,
            key_val_sep,
            pairs_sep,
            (if !(*shorthand).is_null() {
                AV_OPT_FLAG_IMPLICIT_KEY as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint,
            &raw mut parsed_key,
            &raw mut value,
        );
        if ret < 0 as ::core::ffi::c_int {
            if ret == -(22 as ::core::ffi::c_int) {
                av_log(
                    ctx,
                    16 as ::core::ffi::c_int,
                    b"No option name near '%s'\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    opts,
                );
            } else {
                let mut fresh14: [::core::ffi::c_char; 64] = [
                    0 as ::core::ffi::c_int as ::core::ffi::c_char,
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
                ];
                av_log(
                    ctx,
                    16 as ::core::ffi::c_int,
                    b"Unable to parse '%s': %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    opts,
                    av_make_error_string(
                        &raw mut fresh14 as *mut ::core::ffi::c_char,
                        64 as size_t,
                        ret,
                    ),
                );
            }
            return ret;
        }
        if *opts != 0 {
            opts = opts.offset(1);
        }
        if !parsed_key.is_null() {
            key = parsed_key;
            while !(*shorthand).is_null() {
                shorthand = shorthand.offset(1);
            }
        } else {
            let fresh15 = shorthand;
            shorthand = shorthand.offset(1);
            key = *fresh15;
        }
        av_log(
            ctx,
            48 as ::core::ffi::c_int,
            b"Setting '%s' to value '%s'\n\0" as *const u8 as *const ::core::ffi::c_char,
            key,
            value,
        );
        ret = av_opt_set(ctx, key, value, 0 as ::core::ffi::c_int);
        if ret < 0 as ::core::ffi::c_int {
            if ret
                == -(((0xf8 as ::core::ffi::c_int
                    | ('O' as i32) << 8 as ::core::ffi::c_int
                    | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int)
            {
                av_log(
                    ctx,
                    16 as ::core::ffi::c_int,
                    b"Option '%s' not found\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    key,
                );
            }
            av_free(value as *mut ::core::ffi::c_void);
            av_free(parsed_key as *mut ::core::ffi::c_void);
            return ret;
        }
        av_free(value as *mut ::core::ffi::c_void);
        av_free(parsed_key as *mut ::core::ffi::c_void);
        count += 1;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_free(mut obj: *mut ::core::ffi::c_void) {
    let mut o: *const AVOption = ::core::ptr::null::<AVOption>();
    loop {
        o = av_opt_next(obj, o);
        if o.is_null() {
            break;
        }
        let mut pitem: *mut ::core::ffi::c_void = (obj as *mut uint8_t)
            .offset((*o).offset as isize) as *mut ::core::ffi::c_void;
        if (*o).type_0 as ::core::ffi::c_uint
            & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        {
            opt_free_array(o, pitem, opt_array_pcount(pitem));
        } else {
            opt_free_elem((*o).type_0, pitem);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_dict2(
    mut obj: *mut ::core::ffi::c_void,
    mut options: *mut *mut AVDictionary,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut t: *const AVDictionaryEntry = ::core::ptr::null::<AVDictionaryEntry>();
    let mut tmp: *mut AVDictionary = ::core::ptr::null_mut::<AVDictionary>();
    let mut ret: ::core::ffi::c_int = 0;
    if options.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    loop {
        t = av_dict_iterate(*options, t);
        if t.is_null() {
            break;
        }
        ret = av_opt_set(obj, (*t).key, (*t).value, search_flags);
        if ret
            == -(((0xf8 as ::core::ffi::c_int | ('O' as i32) << 8 as ::core::ffi::c_int
                | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int)
        {
            ret = av_dict_set(
                &raw mut tmp,
                (*t).key,
                (*t).value,
                64 as ::core::ffi::c_int,
            );
        }
        if ret < 0 as ::core::ffi::c_int {
            av_log(
                obj,
                16 as ::core::ffi::c_int,
                b"Error setting option %s to value %s.\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*t).key,
                (*t).value,
            );
            av_dict_free(&raw mut tmp);
            return ret;
        }
    }
    av_dict_free(options);
    *options = tmp;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_dict(
    mut obj: *mut ::core::ffi::c_void,
    mut options: *mut *mut AVDictionary,
) -> ::core::ffi::c_int {
    return av_opt_set_dict2(obj, options, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_find(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut unit: *const ::core::ffi::c_char,
    mut opt_flags: ::core::ffi::c_int,
    mut search_flags: ::core::ffi::c_int,
) -> *const AVOption {
    return av_opt_find2(
        obj,
        name,
        unit,
        opt_flags,
        search_flags,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_find2(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut unit: *const ::core::ffi::c_char,
    mut opt_flags: ::core::ffi::c_int,
    mut search_flags: ::core::ffi::c_int,
    mut target_obj: *mut *mut ::core::ffi::c_void,
) -> *const AVOption {
    let mut c: *const AVClass = ::core::ptr::null::<AVClass>();
    let mut o: *const AVOption = ::core::ptr::null::<AVOption>();
    if obj.is_null() {
        return ::core::ptr::null::<AVOption>();
    }
    c = *(obj as *mut *mut AVClass);
    if c.is_null() {
        return ::core::ptr::null::<AVOption>();
    }
    if search_flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
        if search_flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int != 0 {
            let mut iter: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            let mut child: *const AVClass = ::core::ptr::null::<AVClass>();
            loop {
                child = av_opt_child_class_iterate(c, &raw mut iter);
                if child.is_null() {
                    break;
                }
                o = av_opt_find2(
                    &raw mut child as *mut ::core::ffi::c_void,
                    name,
                    unit,
                    opt_flags,
                    search_flags,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_void>(),
                );
                if !o.is_null() {
                    return o;
                }
            }
        } else {
            let mut child_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
                ::core::ffi::c_void,
            >();
            loop {
                child_0 = av_opt_child_next(obj, child_0);
                if child_0.is_null() {
                    break;
                }
                o = av_opt_find2(
                    child_0,
                    name,
                    unit,
                    opt_flags,
                    search_flags,
                    target_obj,
                );
                if !o.is_null() {
                    return o;
                }
            }
        }
    }
    loop {
        o = av_opt_next(obj, o);
        if o.is_null() {
            break;
        }
        if strcmp((*o).name, name) == 0 && (*o).flags & opt_flags == opt_flags
            && (unit.is_null()
                && (*o).type_0 as ::core::ffi::c_uint
                    != AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
                || !unit.is_null()
                    && (*o).type_0 as ::core::ffi::c_uint
                        == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
                    && !(*o).unit.is_null() && strcmp((*o).unit, unit) == 0)
        {
            if !target_obj.is_null() {
                if search_flags & (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
                    == 0
                {
                    *target_obj = obj;
                } else {
                    *target_obj = ::core::ptr::null_mut::<::core::ffi::c_void>();
                }
            }
            return o;
        }
    }
    return ::core::ptr::null::<AVOption>();
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_child_next(
    mut obj: *mut ::core::ffi::c_void,
    mut prev: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut c: *const AVClass = *(obj as *mut *mut AVClass);
    if (*c).child_next.is_some() {
        return (*c).child_next.expect("non-null function pointer")(obj, prev);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_child_class_iterate(
    mut parent: *const AVClass,
    mut iter: *mut *mut ::core::ffi::c_void,
) -> *const AVClass {
    if (*parent).child_class_iterate.is_some() {
        return (*parent).child_class_iterate.expect("non-null function pointer")(iter)
            as *const AVClass;
    }
    return ::core::ptr::null::<AVClass>();
}
unsafe extern "C" fn opt_copy_elem(
    mut logctx: *mut ::core::ffi::c_void,
    mut type_0: AVOptionType,
    mut dst: *mut ::core::ffi::c_void,
    mut src: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    if type_0 as ::core::ffi::c_uint
        == AV_OPT_TYPE_STRING as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut src_str: *const ::core::ffi::c_char = *(src
            as *const *const ::core::ffi::c_char);
        let mut dstp: *mut *mut ::core::ffi::c_char = dst
            as *mut *mut ::core::ffi::c_char;
        if *dstp != src_str as *mut ::core::ffi::c_char {
            av_freep(dstp as *mut ::core::ffi::c_void);
        }
        if !src_str.is_null() {
            *dstp = av_strdup(src_str);
            if (*dstp).is_null() {
                return -(12 as ::core::ffi::c_int);
            }
        }
    } else if type_0 as ::core::ffi::c_uint
        == AV_OPT_TYPE_BINARY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut src8: *const *const uint8_t = src as *const *const uint8_t;
        let mut dst8: *mut *mut uint8_t = dst as *mut *mut uint8_t;
        let mut len: ::core::ffi::c_int = *(src8.offset(1 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_int);
        if *dst8 != *src8 as *mut uint8_t {
            av_freep(dst8 as *mut ::core::ffi::c_void);
        }
        *dst8 = av_memdup(*src8 as *const ::core::ffi::c_void, len as size_t)
            as *mut uint8_t;
        if len != 0 && (*dst8).is_null() {
            *(dst8.offset(1 as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_int) = 0 as ::core::ffi::c_int;
            return -(12 as ::core::ffi::c_int);
        }
        *(dst8.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int) = len;
    } else if !(type_0 as ::core::ffi::c_uint
        == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        if type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_DICT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut sdict: *const AVDictionary = *(src as *const *const AVDictionary);
            let mut ddictp: *mut *mut AVDictionary = dst as *mut *mut AVDictionary;
            if sdict != *ddictp as *const AVDictionary {
                av_dict_free(ddictp);
            }
            *ddictp = ::core::ptr::null_mut::<AVDictionary>();
            return av_dict_copy(ddictp, sdict, 0 as ::core::ffi::c_int);
        } else if type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_CHLAYOUT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if dst != src as *mut ::core::ffi::c_void {
                return av_channel_layout_copy(
                    dst as *mut AVChannelLayout,
                    src as *const AVChannelLayout,
                );
            }
        } else if opt_is_pod(type_0) != 0 {
            let mut size: size_t = opt_type_desc[type_0 as usize].size;
            memcpy(dst, src, size as size_t);
        } else {
            av_log(
                logctx,
                16 as ::core::ffi::c_int,
                b"Unhandled option type: %d\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                type_0 as ::core::ffi::c_uint,
            );
            return -(22 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn opt_copy_array(
    mut logctx: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
    mut pdst: *mut *mut ::core::ffi::c_void,
    mut psrc: *const *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut nb_elems: ::core::ffi::c_uint = *opt_array_pcount(
        psrc as *const ::core::ffi::c_void,
    );
    let mut dst: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut ret: ::core::ffi::c_int = 0;
    if *pdst == *psrc as *mut ::core::ffi::c_void {
        *pdst = ::core::ptr::null_mut::<::core::ffi::c_void>();
        *opt_array_pcount(pdst as *const ::core::ffi::c_void) = 0 as ::core::ffi::c_uint;
    }
    opt_free_array(
        o,
        pdst as *mut ::core::ffi::c_void,
        opt_array_pcount(pdst as *const ::core::ffi::c_void),
    );
    dst = av_calloc(
        nb_elems as size_t,
        opt_type_desc[((*o).type_0 as ::core::ffi::c_uint
                & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint)
                as usize]
            .size,
    );
    if dst.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < nb_elems {
        ret = opt_copy_elem(
            logctx,
            ((*o).type_0 as ::core::ffi::c_uint
                & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint)
                as AVOptionType,
            opt_array_pelem(o, dst, i),
            opt_array_pelem(o, *(psrc as *mut *mut ::core::ffi::c_void), i),
        );
        if ret < 0 as ::core::ffi::c_int {
            opt_free_array(
                o,
                &raw mut dst as *mut ::core::ffi::c_void,
                &raw mut nb_elems,
            );
            return ret;
        }
        i = i.wrapping_add(1);
    }
    *pdst = dst;
    *opt_array_pcount(pdst as *const ::core::ffi::c_void) = nb_elems;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_copy(
    mut dst: *mut ::core::ffi::c_void,
    mut src: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut o: *const AVOption = ::core::ptr::null::<AVOption>();
    let mut c: *const AVClass = ::core::ptr::null::<AVClass>();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if src.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    c = *(src as *mut *mut AVClass);
    if c.is_null() || c != *(dst as *mut *mut AVClass) as *const AVClass {
        return -(22 as ::core::ffi::c_int);
    }
    loop {
        o = av_opt_next(src, o);
        if o.is_null() {
            break;
        }
        let mut field_dst: *mut ::core::ffi::c_void = (dst as *mut uint8_t)
            .offset((*o).offset as isize) as *mut ::core::ffi::c_void;
        let mut field_src: *mut ::core::ffi::c_void = (src as *mut uint8_t)
            .offset((*o).offset as isize) as *mut ::core::ffi::c_void;
        let mut err: ::core::ffi::c_int = if (*o).type_0 as ::core::ffi::c_uint
            & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint != 0
        {
            opt_copy_array(
                dst,
                o,
                field_dst as *mut *mut ::core::ffi::c_void,
                field_src as *const *const ::core::ffi::c_void,
            )
        } else {
            opt_copy_elem(dst, (*o).type_0, field_dst, field_src)
        };
        if err < 0 as ::core::ffi::c_int {
            ret = err;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_get_array_size(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut out_val: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut target_obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut parray: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut o: *const AVOption = ::core::ptr::null::<AVOption>();
    o = av_opt_find2(
        obj,
        name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        search_flags,
        &raw mut target_obj,
    );
    if o.is_null() || target_obj.is_null() {
        return -(((0xf8 as ::core::ffi::c_int | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*o).type_0 as ::core::ffi::c_uint
        & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint == 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    parray = (target_obj as *mut uint8_t).offset((*o).offset as isize)
        as *mut ::core::ffi::c_void;
    *out_val = *opt_array_pcount(parray);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_get_array(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut start_elem: ::core::ffi::c_uint,
    mut nb_elems: ::core::ffi::c_uint,
    mut out_type: AVOptionType,
    mut out_val: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let elem_size_out: size_t = opt_type_desc[(out_type as ::core::ffi::c_uint
            & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as usize]
        .size;
    let mut o: *const AVOption = ::core::ptr::null::<AVOption>();
    let mut target_obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut parray: *const ::core::ffi::c_void = ::core::ptr::null::<
        ::core::ffi::c_void,
    >();
    let mut array_size: ::core::ffi::c_uint = 0;
    let mut ret: ::core::ffi::c_int = 0;
    o = av_opt_find2(
        obj,
        name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        search_flags,
        &raw mut target_obj,
    );
    if o.is_null() || target_obj.is_null() {
        return -(((0xf8 as ::core::ffi::c_int | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*o).type_0 as ::core::ffi::c_uint
        & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint == 0
        || out_type as ::core::ffi::c_uint
            & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    parray = (target_obj as *mut uint8_t).offset((*o).offset as isize)
        as *const ::core::ffi::c_void;
    array_size = *opt_array_pcount(parray);
    if start_elem >= array_size || array_size.wrapping_sub(start_elem) < nb_elems {
        return -(22 as ::core::ffi::c_int);
    }
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    loop {
        if !(i < nb_elems) {
            current_block = 7659304154607701039;
            break;
        }
        let mut src: *const ::core::ffi::c_void = opt_array_pelem(
            o,
            *(parray as *mut *mut ::core::ffi::c_void),
            start_elem.wrapping_add(i),
        );
        let mut dst: *mut ::core::ffi::c_void = (out_val as *mut uint8_t)
            .offset((i as size_t).wrapping_mul(elem_size_out) as isize)
            as *mut ::core::ffi::c_void;
        if out_type as ::core::ffi::c_uint
            == (*o).type_0 as ::core::ffi::c_uint
                & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint
        {
            ret = opt_copy_elem(obj, out_type, dst, src);
            if ret < 0 as ::core::ffi::c_int {
                current_block = 6012044430873431455;
                break;
            }
        } else if out_type as ::core::ffi::c_uint
            == AV_OPT_TYPE_STRING as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut buf: [uint8_t; 128] = [0; 128];
            let mut out: *mut uint8_t = &raw mut buf as *mut uint8_t;
            ret = opt_get_elem(
                o,
                &raw mut out,
                ::core::mem::size_of::<[uint8_t; 128]>() as size_t,
                src,
                search_flags,
            );
            if ret < 0 as ::core::ffi::c_int {
                current_block = 6012044430873431455;
                break;
            }
            if out == &raw mut buf as *mut uint8_t {
                out = av_strdup(
                    &raw mut buf as *mut uint8_t as *const ::core::ffi::c_char,
                ) as *mut uint8_t;
                if out.is_null() {
                    ret = -(12 as ::core::ffi::c_int);
                    current_block = 6012044430873431455;
                    break;
                }
            }
            let ref mut fresh22 = *(dst as *mut *mut uint8_t);
            *fresh22 = out;
        } else if out_type as ::core::ffi::c_uint
            == AV_OPT_TYPE_INT64 as ::core::ffi::c_int as ::core::ffi::c_uint
            || out_type as ::core::ffi::c_uint
                == AV_OPT_TYPE_DOUBLE as ::core::ffi::c_int as ::core::ffi::c_uint
            || out_type as ::core::ffi::c_uint
                == AV_OPT_TYPE_RATIONAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut num: ::core::ffi::c_double = 1.0f64;
            let mut den: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut intnum: int64_t = 1 as int64_t;
            ret = read_number(o, src, &raw mut num, &raw mut den, &raw mut intnum);
            if ret < 0 as ::core::ffi::c_int {
                current_block = 6012044430873431455;
                break;
            }
            match out_type as ::core::ffi::c_uint {
                3 => {
                    *(dst as *mut int64_t) = (if num == den as ::core::ffi::c_double {
                        intnum as ::core::ffi::c_double
                    } else {
                        num * intnum as ::core::ffi::c_double
                            / den as ::core::ffi::c_double
                    }) as int64_t;
                }
                4 => {
                    *(dst as *mut ::core::ffi::c_double) = num
                        * intnum as ::core::ffi::c_double / den as ::core::ffi::c_double;
                }
                7 => {
                    *(dst as *mut AVRational) = if num == 1.0f64
                        && intnum as ::core::ffi::c_int as int64_t == intnum
                    {
                        AVRational {
                            num: intnum as ::core::ffi::c_int,
                            den: den,
                        }
                    } else {
                        double_to_rational(
                            num * intnum as ::core::ffi::c_double
                                / den as ::core::ffi::c_double,
                        )
                    };
                }
                _ => {
                    if 0 as ::core::ffi::c_int == 0 {
                        av_log(
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            0 as ::core::ffi::c_int,
                            b"Assertion %s failed at %s:%d\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"0\0" as *const u8 as *const ::core::ffi::c_char,
                            b"src/libavutil/opt.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            2251 as ::core::ffi::c_int,
                        );
                        abort();
                    }
                }
            }
        } else {
            return -(38 as ::core::ffi::c_int)
        }
        i = i.wrapping_add(1);
    }
    match current_block {
        7659304154607701039 => return 0 as ::core::ffi::c_int,
        _ => {
            let mut i_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
            while i_0 < nb_elems {
                opt_free_elem(
                    out_type,
                    (out_val as *mut uint8_t)
                        .offset((i_0 as size_t).wrapping_mul(elem_size_out) as isize)
                        as *mut ::core::ffi::c_void,
                );
                i_0 = i_0.wrapping_add(1);
            }
            return ret;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_set_array(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
    mut start_elem: ::core::ffi::c_uint,
    mut nb_elems: ::core::ffi::c_uint,
    mut val_type: AVOptionType,
    mut val: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let elem_size_val: size_t = opt_type_desc[(val_type as ::core::ffi::c_uint
            & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as usize]
        .size;
    let mut o: *const AVOption = ::core::ptr::null::<AVOption>();
    let mut arr: *const AVOptionArrayDef = ::core::ptr::null::<AVOptionArrayDef>();
    let mut target_obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut parray: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut new_elems: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut array_size: *mut ::core::ffi::c_uint = ::core::ptr::null_mut::<
        ::core::ffi::c_uint,
    >();
    let mut new_size: ::core::ffi::c_uint = 0;
    let mut elem_size: size_t = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ret = opt_set_init(
        obj,
        name,
        search_flags,
        0 as ::core::ffi::c_int,
        &raw mut target_obj,
        &raw mut o,
        &raw mut parray,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if (*o).type_0 as ::core::ffi::c_uint
        & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint == 0
        || val_type as ::core::ffi::c_uint
            & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        return -(22 as ::core::ffi::c_int);
    }
    arr = (*o).default_val.arr;
    array_size = opt_array_pcount(parray);
    elem_size = opt_type_desc[((*o).type_0 as ::core::ffi::c_uint
            & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint)
            as usize]
        .size;
    if start_elem > *array_size {
        return -(22 as ::core::ffi::c_int);
    }
    if val.is_null() {
        if (*array_size).wrapping_sub(start_elem) < nb_elems {
            return -(22 as ::core::ffi::c_int);
        }
        new_size = (*array_size).wrapping_sub(nb_elems);
    } else if search_flags & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0 {
        if start_elem
            >= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_sub(nb_elems)
        {
            return -(22 as ::core::ffi::c_int);
        }
        new_size = if *array_size > start_elem.wrapping_add(nb_elems) {
            *array_size
        } else {
            start_elem.wrapping_add(nb_elems)
        };
    } else {
        if nb_elems
            >= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                .wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint)
                .wrapping_sub(*array_size)
        {
            return -(22 as ::core::ffi::c_int);
        }
        new_size = (*array_size).wrapping_add(nb_elems);
    }
    if !arr.is_null()
        && ((*arr).size_max != 0 && new_size > (*arr).size_max
            || (*arr).size_min != 0 && new_size < (*arr).size_min)
    {
        return -(22 as ::core::ffi::c_int);
    }
    if val.is_null() {
        let mut array: *mut ::core::ffi::c_void = *(parray
            as *mut *mut ::core::ffi::c_void);
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while i < nb_elems {
            opt_free_elem(
                (*o).type_0,
                opt_array_pelem(o, array, start_elem.wrapping_add(i)),
            );
            i = i.wrapping_add(1);
        }
        if new_size > 0 as ::core::ffi::c_uint {
            memmove(
                opt_array_pelem(o, array, start_elem),
                opt_array_pelem(o, array, start_elem.wrapping_add(nb_elems)),
                (elem_size as size_t)
                    .wrapping_mul(
                        (*array_size).wrapping_sub(start_elem).wrapping_sub(nb_elems)
                            as size_t,
                    ),
            );
            array = av_realloc_array(array, new_size as size_t, elem_size);
            if array.is_null() {
                return -(12 as ::core::ffi::c_int);
            }
            let ref mut fresh16 = *(parray as *mut *mut ::core::ffi::c_void);
            *fresh16 = array;
        } else {
            av_freep(parray);
        }
        *array_size = new_size;
        return 0 as ::core::ffi::c_int;
    }
    new_elems = av_calloc(nb_elems as size_t, elem_size);
    if new_elems.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    let mut i_0: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    loop {
        if !(i_0 < nb_elems) {
            current_block = 129780949503461575;
            break;
        }
        let mut dst: *mut ::core::ffi::c_void = opt_array_pelem(o, new_elems, i_0);
        let mut src: *const ::core::ffi::c_void = (val as *mut uint8_t)
            .offset((i_0 as size_t).wrapping_mul(elem_size_val) as isize)
            as *const ::core::ffi::c_void;
        let mut num: ::core::ffi::c_double = 1.0f64;
        let mut den: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut intnum: int64_t = 1 as int64_t;
        if val_type as ::core::ffi::c_uint
            == (*o).type_0 as ::core::ffi::c_uint
                & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int) as ::core::ffi::c_uint
        {
            let mut err: ::core::ffi::c_int = 0;
            ret = opt_copy_elem(obj, val_type, dst, src);
            if ret < 0 as ::core::ffi::c_int {
                current_block = 9240864384605639628;
                break;
            }
            err = read_number(o, dst, &raw mut num, &raw mut den, &raw mut intnum);
            if err >= 0 as ::core::ffi::c_int
                && (*o).type_0 as ::core::ffi::c_uint
                    & !(AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                    != AV_OPT_TYPE_FLAGS as ::core::ffi::c_int as ::core::ffi::c_uint
                && (den == 0
                    || ((*o).max * den as ::core::ffi::c_double)
                        < num * intnum as ::core::ffi::c_double
                    || (*o).min * den as ::core::ffi::c_double
                        > num * intnum as ::core::ffi::c_double)
            {
                num = if den != 0 {
                    num * intnum as ::core::ffi::c_double / den as ::core::ffi::c_double
                } else {
                    (if num != 0. && intnum != 0 {
                        ::core::f32::INFINITY
                    } else {
                        ::core::f32::NAN
                    }) as ::core::ffi::c_double
                };
                av_log(
                    obj,
                    16 as ::core::ffi::c_int,
                    b"Cannot set array element %u for parameter '%s': value %f out of range [%g - %g]\n\0"
                        as *const u8 as *const ::core::ffi::c_char,
                    start_elem.wrapping_add(i_0),
                    (*o).name,
                    num,
                    (*o).min,
                    (*o).max,
                );
                ret = -(34 as ::core::ffi::c_int);
                current_block = 9240864384605639628;
                break;
            }
        } else if val_type as ::core::ffi::c_uint
            == AV_OPT_TYPE_STRING as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            ret = opt_set_elem(
                obj,
                target_obj,
                o,
                *(src as *mut *const ::core::ffi::c_char),
                dst,
            );
            if ret < 0 as ::core::ffi::c_int {
                current_block = 9240864384605639628;
                break;
            }
        } else if val_type as ::core::ffi::c_uint
            == AV_OPT_TYPE_INT as ::core::ffi::c_int as ::core::ffi::c_uint
            || val_type as ::core::ffi::c_uint
                == AV_OPT_TYPE_INT64 as ::core::ffi::c_int as ::core::ffi::c_uint
            || val_type as ::core::ffi::c_uint
                == AV_OPT_TYPE_FLOAT as ::core::ffi::c_int as ::core::ffi::c_uint
            || val_type as ::core::ffi::c_uint
                == AV_OPT_TYPE_DOUBLE as ::core::ffi::c_int as ::core::ffi::c_uint
            || val_type as ::core::ffi::c_uint
                == AV_OPT_TYPE_RATIONAL as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            match val_type as ::core::ffi::c_uint {
                2 => {
                    intnum = *(src as *mut ::core::ffi::c_int) as int64_t;
                }
                3 => {
                    intnum = *(src as *mut int64_t);
                }
                5 => {
                    num = *(src as *mut ::core::ffi::c_float) as ::core::ffi::c_double;
                }
                4 => {
                    num = *(src as *mut ::core::ffi::c_double);
                }
                7 => {
                    intnum = (*(src as *mut AVRational)).num as int64_t;
                    den = (*(src as *mut AVRational)).den;
                }
                _ => {
                    if 0 as ::core::ffi::c_int == 0 {
                        av_log(
                            ::core::ptr::null_mut::<::core::ffi::c_void>(),
                            0 as ::core::ffi::c_int,
                            b"Assertion %s failed at %s:%d\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            b"0\0" as *const u8 as *const ::core::ffi::c_char,
                            b"src/libavutil/opt.c\0" as *const u8
                                as *const ::core::ffi::c_char,
                            2397 as ::core::ffi::c_int,
                        );
                        abort();
                    }
                }
            }
            ret = write_number(obj, o, dst, num, den, intnum);
            if ret < 0 as ::core::ffi::c_int {
                current_block = 9240864384605639628;
                break;
            }
        } else {
            ret = -(38 as ::core::ffi::c_int);
            current_block = 9240864384605639628;
            break;
        }
        i_0 = i_0.wrapping_add(1);
    }
    match current_block {
        129780949503461575 => {
            if start_elem == 0 as ::core::ffi::c_uint && nb_elems == new_size {
                opt_free_array(o, parray, array_size);
                let ref mut fresh17 = *(parray as *mut *mut ::core::ffi::c_void);
                *fresh17 = new_elems;
                *array_size = nb_elems;
                new_elems = ::core::ptr::null_mut::<::core::ffi::c_void>();
                nb_elems = 0 as ::core::ffi::c_uint;
            } else {
                let mut array_0: *mut ::core::ffi::c_void = av_realloc_array(
                    *(parray as *mut *mut ::core::ffi::c_void),
                    new_size as size_t,
                    elem_size,
                );
                if array_0.is_null() {
                    ret = -(12 as ::core::ffi::c_int);
                } else {
                    if search_flags
                        & (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int != 0
                    {
                        let mut i_1: ::core::ffi::c_uint = start_elem;
                        while i_1
                            < (if start_elem.wrapping_add(nb_elems) > *array_size {
                                *array_size
                            } else {
                                start_elem.wrapping_add(nb_elems)
                            })
                        {
                            opt_free_elem((*o).type_0, opt_array_pelem(o, array_0, i_1));
                            i_1 = i_1.wrapping_add(1);
                        }
                    } else {
                        memmove(
                            opt_array_pelem(
                                o,
                                array_0,
                                start_elem.wrapping_add(nb_elems),
                            ),
                            opt_array_pelem(o, array_0, start_elem),
                            (elem_size as size_t)
                                .wrapping_mul(
                                    (*array_size).wrapping_sub(start_elem) as size_t,
                                ),
                        );
                    }
                    memcpy(
                        (array_0 as *mut uint8_t)
                            .offset(
                                elem_size.wrapping_mul(start_elem as size_t) as isize,
                            ) as *mut ::core::ffi::c_void,
                        new_elems,
                        (elem_size as size_t).wrapping_mul(nb_elems as size_t),
                    );
                    av_freep(&raw mut new_elems as *mut ::core::ffi::c_void);
                    nb_elems = 0 as ::core::ffi::c_uint;
                    let ref mut fresh18 = *(parray as *mut *mut ::core::ffi::c_void);
                    *fresh18 = array_0;
                    *array_size = new_size;
                }
            }
        }
        _ => {}
    }
    opt_free_array(o, &raw mut new_elems as *mut ::core::ffi::c_void, &raw mut nb_elems);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_query_ranges(
    mut ranges_arg: *mut *mut AVOptionRanges,
    mut obj: *mut ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut c: *const AVClass = *(obj as *mut *mut AVClass);
    let mut callback: Option<
        unsafe extern "C" fn(
            *mut *mut AVOptionRanges,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_char,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    > = (*c).query_ranges
        as Option<
            unsafe extern "C" fn(
                *mut *mut AVOptionRanges,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        >;
    if callback.is_none() {
        callback = Some(
            av_opt_query_ranges_default
                as unsafe extern "C" fn(
                    *mut *mut AVOptionRanges,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut *mut AVOptionRanges,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_char,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
            >;
    }
    ret = callback.expect("non-null function pointer")(ranges_arg, obj, key, flags);
    if ret >= 0 as ::core::ffi::c_int {
        if flags & (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int == 0 {
            ret = 1 as ::core::ffi::c_int;
        }
        (**ranges_arg).nb_components = ret;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_query_ranges_default(
    mut ranges_arg: *mut *mut AVOptionRanges,
    mut obj: *mut ::core::ffi::c_void,
    mut key: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ranges: *mut AVOptionRanges = av_mallocz(
        ::core::mem::size_of::<AVOptionRanges>() as size_t,
    ) as *mut AVOptionRanges;
    let mut range_array: *mut *mut AVOptionRange = av_mallocz(
        ::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t,
    ) as *mut *mut AVOptionRange;
    let mut range: *mut AVOptionRange = av_mallocz(
        ::core::mem::size_of::<AVOptionRange>() as size_t,
    ) as *mut AVOptionRange;
    let mut field: *const AVOption = av_opt_find(
        obj,
        key,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        flags,
    );
    let mut ret: ::core::ffi::c_int = 0;
    *ranges_arg = ::core::ptr::null_mut::<AVOptionRanges>();
    if ranges.is_null() || range.is_null() || range_array.is_null() || field.is_null() {
        ret = -(12 as ::core::ffi::c_int);
    } else {
        (*ranges).range = range_array;
        let ref mut fresh13 = *(*ranges).range.offset(0 as ::core::ffi::c_int as isize);
        *fresh13 = range;
        (*ranges).nb_ranges = 1 as ::core::ffi::c_int;
        (*ranges).nb_components = 1 as ::core::ffi::c_int;
        (*range).is_range = 1 as ::core::ffi::c_int;
        (*range).value_min = (*field).min;
        (*range).value_max = (*field).max;
        match (*field).type_0 as ::core::ffi::c_uint {
            18 | 2 | 20 | 3 | 10 | 13 | 14 | 5 | 4 | 16 | 17 => {
                current_block = 9828876828309294594;
            }
            6 => {
                (*range).component_min = 0 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                (*range).component_max = 0x10ffff as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                (*range).value_min = -(1 as ::core::ffi::c_int) as ::core::ffi::c_double;
                (*range).value_max = 2147483647 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                current_block = 9828876828309294594;
            }
            7 => {
                (*range).component_min = (-(2147483647 as ::core::ffi::c_int)
                    - 1 as ::core::ffi::c_int) as ::core::ffi::c_double;
                (*range).component_max = 2147483647 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                current_block = 9828876828309294594;
            }
            12 => {
                (*range).component_min = 0 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                (*range).component_max = (2147483647 as ::core::ffi::c_int
                    / 128 as ::core::ffi::c_int / 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_double;
                (*range).value_min = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
                (*range).value_max = (2147483647 as ::core::ffi::c_int
                    / 8 as ::core::ffi::c_int) as ::core::ffi::c_double;
                current_block = 9828876828309294594;
            }
            15 => {
                (*range).component_min = 1 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                (*range).component_max = 2147483647 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                (*range).value_min = 1 as ::core::ffi::c_int as ::core::ffi::c_double;
                (*range).value_max = 2147483647 as ::core::ffi::c_int
                    as ::core::ffi::c_double;
                current_block = 9828876828309294594;
            }
            _ => {
                ret = -(38 as ::core::ffi::c_int);
                current_block = 1927419857694670955;
            }
        }
        match current_block {
            1927419857694670955 => {}
            _ => {
                *ranges_arg = ranges;
                return 1 as ::core::ffi::c_int;
            }
        }
    }
    av_free(ranges as *mut ::core::ffi::c_void);
    av_free(range as *mut ::core::ffi::c_void);
    av_free(range_array as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_freep_ranges(mut rangesp: *mut *mut AVOptionRanges) {
    let mut i: ::core::ffi::c_int = 0;
    let mut ranges: *mut AVOptionRanges = *rangesp;
    if ranges.is_null() {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*ranges).nb_ranges * (*ranges).nb_components {
        let mut range: *mut AVOptionRange = *(*ranges).range.offset(i as isize);
        if !range.is_null() {
            av_freep(&raw mut (*range).str_0 as *mut ::core::ffi::c_void);
            av_freep(
                (*ranges).range.offset(i as isize) as *mut *mut AVOptionRange
                    as *mut ::core::ffi::c_void,
            );
        }
        i += 1;
    }
    av_freep(&raw mut (*ranges).range as *mut ::core::ffi::c_void);
    av_freep(rangesp as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_is_set_to_default(
    mut obj: *mut ::core::ffi::c_void,
    mut o: *const AVOption,
) -> ::core::ffi::c_int {
    let mut i64: int64_t = 0;
    let mut d: ::core::ffi::c_double = 0.;
    let mut q: AVRational = AVRational { num: 0, den: 0 };
    let mut ret: ::core::ffi::c_int = 0;
    let mut w: ::core::ffi::c_int = 0;
    let mut h: ::core::ffi::c_int = 0;
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut dst: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    if o.is_null() || obj.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    dst = (obj as *mut uint8_t).offset((*o).offset as isize) as *mut ::core::ffi::c_void;
    if (*o).type_0 as ::core::ffi::c_uint
        & AV_OPT_TYPE_FLAG_ARRAY as ::core::ffi::c_int as ::core::ffi::c_uint != 0
    {
        let mut def: *const ::core::ffi::c_char = if !(*o).default_val.arr.is_null() {
            (*(*o).default_val.arr).def
        } else {
            ::core::ptr::null::<::core::ffi::c_char>()
        };
        let mut val: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        ret = opt_get_array(o, dst, &raw mut val);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        if !val.is_null() as ::core::ffi::c_int != !def.is_null() as ::core::ffi::c_int {
            ret = 0 as ::core::ffi::c_int;
        } else if !val.is_null() {
            ret = (strcmp(val as *const ::core::ffi::c_char, def) == 0)
                as ::core::ffi::c_int;
        } else {
            ret = 1 as ::core::ffi::c_int;
        }
        av_freep(&raw mut val as *mut ::core::ffi::c_void);
        return ret;
    }
    match (*o).type_0 as ::core::ffi::c_uint {
        11 => return 1 as ::core::ffi::c_int,
        18 | 1 | 13 | 14 | 2 | 20 | 16 | 3 | 10 => {
            read_number(
                o,
                dst,
                ::core::ptr::null_mut::<::core::ffi::c_double>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                &raw mut i64,
            );
            return ((*o).default_val.i64_0 == i64) as ::core::ffi::c_int;
        }
        19 => {
            let mut ch_layout: AVChannelLayout = AVChannelLayout {
                order: AV_CHANNEL_ORDER_UNSPEC,
                nb_channels: 0,
                u: C2RustUnnamed_0 { mask: 0 },
                opaque: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            };
            if !(*o).default_val.str_0.is_null() {
                ret = av_channel_layout_from_string(
                    &raw mut ch_layout,
                    (*o).default_val.str_0,
                );
                if ret < 0 as ::core::ffi::c_int {
                    return ret;
                }
            }
            ret = (av_channel_layout_compare(
                dst as *mut AVChannelLayout,
                &raw mut ch_layout,
            ) == 0) as ::core::ffi::c_int;
            av_channel_layout_uninit(&raw mut ch_layout);
            return ret;
        }
        6 => {
            str = *(dst as *mut *mut ::core::ffi::c_char);
            if str == (*o).default_val.str_0 as *mut ::core::ffi::c_char {
                return 1 as ::core::ffi::c_int;
            }
            if str.is_null() || (*o).default_val.str_0.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            return (strcmp(str, (*o).default_val.str_0) == 0) as ::core::ffi::c_int;
        }
        4 => {
            d = *(dst as *mut ::core::ffi::c_double);
            return ((*o).default_val.dbl == d) as ::core::ffi::c_int;
        }
        5 => {
            d = *(dst as *mut ::core::ffi::c_float) as ::core::ffi::c_double;
            return ((*o).default_val.dbl as ::core::ffi::c_float as ::core::ffi::c_double
                == d) as ::core::ffi::c_int;
        }
        7 => {
            q = av_d2q((*o).default_val.dbl, 2147483647 as ::core::ffi::c_int);
            return (av_cmp_q(*(dst as *mut AVRational), q) == 0) as ::core::ffi::c_int;
        }
        8 => {
            let mut tmp: C2RustUnnamed_3 = C2RustUnnamed_3 {
                data: ::core::ptr::null_mut::<uint8_t>(),
                size: 0,
            };
            let mut opt_size: ::core::ffi::c_int = *((dst
                as *mut *mut ::core::ffi::c_void)
                .offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_int);
            let mut opt_ptr: *mut ::core::ffi::c_void = *(dst
                as *mut *mut ::core::ffi::c_void);
            if opt_size == 0
                && ((*o).default_val.str_0.is_null()
                    || strlen((*o).default_val.str_0) == 0)
            {
                return 1 as ::core::ffi::c_int;
            }
            if opt_size == 0 || (*o).default_val.str_0.is_null()
                || strlen((*o).default_val.str_0) == 0
            {
                return 0 as ::core::ffi::c_int;
            }
            if opt_size as size_t
                != strlen((*o).default_val.str_0).wrapping_div(2 as size_t)
            {
                return 0 as ::core::ffi::c_int;
            }
            ret = set_string_binary(
                ::core::ptr::null_mut::<::core::ffi::c_void>(),
                ::core::ptr::null::<AVOption>(),
                (*o).default_val.str_0,
                &raw mut tmp.data,
            );
            if ret == 0 {
                ret = (memcmp(
                    opt_ptr,
                    tmp.data as *const ::core::ffi::c_void,
                    tmp.size as size_t,
                ) == 0) as ::core::ffi::c_int;
            }
            av_free(tmp.data as *mut ::core::ffi::c_void);
            return ret;
        }
        9 => {
            let mut dict1: *mut AVDictionary = ::core::ptr::null_mut::<AVDictionary>();
            let mut dict2: *mut AVDictionary = *(dst as *mut *mut AVDictionary);
            let mut en1: *const AVDictionaryEntry = ::core::ptr::null::<
                AVDictionaryEntry,
            >();
            let mut en2: *const AVDictionaryEntry = ::core::ptr::null::<
                AVDictionaryEntry,
            >();
            ret = av_dict_parse_string(
                &raw mut dict1,
                (*o).default_val.str_0,
                b"=\0" as *const u8 as *const ::core::ffi::c_char,
                b":\0" as *const u8 as *const ::core::ffi::c_char,
                0 as ::core::ffi::c_int,
            );
            if ret < 0 as ::core::ffi::c_int {
                av_dict_free(&raw mut dict1);
                return ret;
            }
            loop {
                en1 = av_dict_iterate(dict1, en1);
                en2 = av_dict_iterate(dict2, en2);
                if !(!en1.is_null() && !en2.is_null()
                    && strcmp((*en1).key, (*en2).key) == 0
                    && strcmp((*en1).value, (*en2).value) == 0)
                {
                    break;
                }
            }
            av_dict_free(&raw mut dict1);
            return (en1.is_null() && en2.is_null()) as ::core::ffi::c_int;
        }
        12 => {
            if (*o).default_val.str_0.is_null()
                || strcmp(
                    (*o).default_val.str_0,
                    b"none\0" as *const u8 as *const ::core::ffi::c_char,
                ) == 0
            {
                h = 0 as ::core::ffi::c_int;
                w = h;
            } else {
                ret = av_parse_video_size(
                    &raw mut w,
                    &raw mut h,
                    (*o).default_val.str_0,
                );
                if ret < 0 as ::core::ffi::c_int {
                    return ret;
                }
            }
            return (w == *(dst as *mut ::core::ffi::c_int)
                && h
                    == *(dst as *mut ::core::ffi::c_int)
                        .offset(1 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int;
        }
        15 => {
            q = AVRational {
                num: 0 as ::core::ffi::c_int,
                den: 0 as ::core::ffi::c_int,
            };
            if !(*o).default_val.str_0.is_null() {
                ret = av_parse_video_rate(&raw mut q, (*o).default_val.str_0);
                if ret < 0 as ::core::ffi::c_int {
                    return ret;
                }
            }
            return (av_cmp_q(*(dst as *mut AVRational), q) == 0) as ::core::ffi::c_int;
        }
        17 => {
            let mut color: [uint8_t; 4] = [
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
                0 as ::core::ffi::c_int as uint8_t,
            ];
            if !(*o).default_val.str_0.is_null() {
                ret = av_parse_color(
                    &raw mut color as *mut uint8_t,
                    (*o).default_val.str_0,
                    -(1 as ::core::ffi::c_int),
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
                if ret < 0 as ::core::ffi::c_int {
                    return ret;
                }
            }
            return (memcmp(
                &raw mut color as *mut uint8_t as *const ::core::ffi::c_void,
                dst,
                ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
            ) == 0) as ::core::ffi::c_int;
        }
        _ => {
            av_log(
                obj,
                24 as ::core::ffi::c_int,
                b"Not supported option type: %d, option name: %s\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*o).type_0 as ::core::ffi::c_uint,
                (*o).name,
            );
        }
    }
    return -((('P' as i32 | ('A' as i32) << 8 as ::core::ffi::c_int
        | ('W' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
        | ('E' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
        as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_is_set_to_default_by_name(
    mut obj: *mut ::core::ffi::c_void,
    mut name: *const ::core::ffi::c_char,
    mut search_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut o: *const AVOption = ::core::ptr::null::<AVOption>();
    let mut target: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    if obj.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    o = av_opt_find2(
        obj,
        name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        search_flags,
        &raw mut target,
    );
    if o.is_null() {
        return -(((0xf8 as ::core::ffi::c_int | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('P' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('T' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    return av_opt_is_set_to_default(target, o);
}
unsafe extern "C" fn opt_serialize(
    mut obj: *mut ::core::ffi::c_void,
    mut opt_flags: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut cnt: *mut ::core::ffi::c_int,
    mut bprint: *mut AVBPrint,
    key_val_sep: ::core::ffi::c_char,
    pairs_sep: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut o: *const AVOption = ::core::ptr::null::<AVOption>();
    let mut child: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut buf: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ret: ::core::ffi::c_int = 0;
    let special_chars: [::core::ffi::c_char; 3] = [
        pairs_sep,
        key_val_sep,
        '\0' as i32 as ::core::ffi::c_char,
    ];
    if flags & 0x4 as ::core::ffi::c_int != 0 {
        loop {
            child = av_opt_child_next(obj, child);
            if child.is_null() {
                break;
            }
            ret = opt_serialize(
                child,
                opt_flags,
                flags,
                cnt,
                bprint,
                key_val_sep,
                pairs_sep,
            );
            if ret < 0 as ::core::ffi::c_int {
                return ret;
            }
        }
    }
    loop {
        o = av_opt_next(obj, o);
        if o.is_null() {
            break;
        }
        if (*o).type_0 as ::core::ffi::c_uint
            == AV_OPT_TYPE_CONST as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            continue;
        }
        if flags & 0x2 as ::core::ffi::c_int != 0 && (*o).flags != opt_flags {
            continue;
        }
        if (*o).flags & opt_flags != opt_flags {
            continue;
        }
        if flags & 0x1 as ::core::ffi::c_int != 0
            && av_opt_is_set_to_default(obj, o) > 0 as ::core::ffi::c_int
        {
            continue;
        }
        ret = av_opt_get(obj, (*o).name, 0 as ::core::ffi::c_int, &raw mut buf);
        if ret < 0 as ::core::ffi::c_int {
            av_bprint_finalize(
                bprint,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            return ret;
        }
        if !buf.is_null() {
            let fresh23 = *cnt;
            *cnt = *cnt + 1;
            if fresh23 != 0 {
                av_bprint_append_data(
                    bprint,
                    &raw const pairs_sep,
                    1 as ::core::ffi::c_uint,
                );
            }
            av_bprint_escape(
                bprint,
                (*o).name,
                &raw const special_chars as *const ::core::ffi::c_char,
                AV_ESCAPE_MODE_BACKSLASH,
                0 as ::core::ffi::c_int,
            );
            av_bprint_append_data(
                bprint,
                &raw const key_val_sep,
                1 as ::core::ffi::c_uint,
            );
            av_bprint_escape(
                bprint,
                buf as *const ::core::ffi::c_char,
                &raw const special_chars as *const ::core::ffi::c_char,
                AV_ESCAPE_MODE_BACKSLASH,
                0 as ::core::ffi::c_int,
            );
            av_freep(&raw mut buf as *mut ::core::ffi::c_void);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_opt_serialize(
    mut obj: *mut ::core::ffi::c_void,
    mut opt_flags: ::core::ffi::c_int,
    mut flags: ::core::ffi::c_int,
    mut buffer: *mut *mut ::core::ffi::c_char,
    key_val_sep: ::core::ffi::c_char,
    pairs_sep: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut bprint: AVBPrint = AVBPrint {
        str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1000],
    };
    let mut ret: ::core::ffi::c_int = 0;
    let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if pairs_sep as ::core::ffi::c_int == '\0' as i32
        || key_val_sep as ::core::ffi::c_int == '\0' as i32
        || pairs_sep as ::core::ffi::c_int == key_val_sep as ::core::ffi::c_int
        || pairs_sep as ::core::ffi::c_int == '\\' as i32
        || key_val_sep as ::core::ffi::c_int == '\\' as i32
    {
        av_log(
            obj,
            16 as ::core::ffi::c_int,
            b"Invalid separator(s) found.\0" as *const u8 as *const ::core::ffi::c_char,
        );
        return -(22 as ::core::ffi::c_int);
    }
    if obj.is_null() || buffer.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    *buffer = ::core::ptr::null_mut::<::core::ffi::c_char>();
    av_bprint_init(
        &raw mut bprint,
        64 as ::core::ffi::c_uint,
        -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    ret = opt_serialize(
        obj,
        opt_flags,
        flags,
        &raw mut cnt,
        &raw mut bprint,
        key_val_sep,
        pairs_sep,
    );
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    ret = av_bprint_finalize(&raw mut bprint, buffer);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    return 0 as ::core::ffi::c_int;
}
#[repr(C)]
pub struct AVDictionary {
    _opaque: [u8; 0],
}
