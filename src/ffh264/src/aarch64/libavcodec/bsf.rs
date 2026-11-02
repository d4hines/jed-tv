// Translated from FFmpeg 9.0.1 libavcodec/bsf.c
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
    fn ff_bsf_child_class_iterate(
        opaque: *mut *mut ::core::ffi::c_void,
    ) -> *const AVClass;
    fn avcodec_descriptor_get(id: AVCodecID) -> *const AVCodecDescriptor;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_bsf_get_by_name(name: *const ::core::ffi::c_char) -> *const AVBitStreamFilter;
    fn av_bprint_init(
        buf: *mut AVBPrint,
        size_init: ::core::ffi::c_uint,
        size_max: ::core::ffi::c_uint,
    );
    fn av_bprintf(buf: *mut AVBPrint, fmt: *const ::core::ffi::c_char, ...);
    fn av_get_token(
        buf: *mut *const ::core::ffi::c_char,
        term: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn av_strtok(
        s: *mut ::core::ffi::c_char,
        delim: *const ::core::ffi::c_char,
        saveptr: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn av_bprint_finalize(
        buf: *mut AVBPrint,
        ret_str: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn av_log(
        avcl: *mut ::core::ffi::c_void,
        level: ::core::ffi::c_int,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn avcodec_parameters_alloc() -> *mut AVCodecParameters;
    fn avcodec_parameters_free(par: *mut *mut AVCodecParameters);
    fn avcodec_parameters_copy(
        dst: *mut AVCodecParameters,
        src: *const AVCodecParameters,
    ) -> ::core::ffi::c_int;
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_dynarray_add_nofree(
        tab_ptr: *mut ::core::ffi::c_void,
        nb_ptr: *mut ::core::ffi::c_int,
        elem: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn av_opt_set_defaults(s: *mut ::core::ffi::c_void);
    fn av_opt_free(obj: *mut ::core::ffi::c_void);
    fn av_opt_next(
        obj: *const ::core::ffi::c_void,
        prev: *const AVOption,
    ) -> *const AVOption;
    fn avcodec_get_name(id: AVCodecID) -> *const ::core::ffi::c_char;
    fn av_packet_alloc() -> *mut AVPacket;
    fn av_packet_free(pkt: *mut *mut AVPacket);
    fn av_opt_set_from_string(
        ctx: *mut ::core::ffi::c_void,
        opts: *const ::core::ffi::c_char,
        shorthand: *const *const ::core::ffi::c_char,
        key_val_sep: *const ::core::ffi::c_char,
        pairs_sep: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn av_opt_set_dict2(
        obj: *mut ::core::ffi::c_void,
        options: *mut *mut AVDictionary,
        search_flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_packet_unref(pkt: *mut AVPacket);
    fn av_packet_move_ref(dst: *mut AVPacket, src: *mut AVPacket);
    fn av_packet_make_refcounted(pkt: *mut AVPacket) -> ::core::ffi::c_int;
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
pub type uint8_t = ::core::ffi::c_uchar;
pub type uint32_t = ::core::ffi::c_uint;
pub type uint64_t = ::core::ffi::c_ulong;
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
pub type AVMediaType = ::core::ffi::c_int;
pub const AVMEDIA_TYPE_NB: AVMediaType = 5;
pub const AVMEDIA_TYPE_ATTACHMENT: AVMediaType = 4;
pub const AVMEDIA_TYPE_SUBTITLE: AVMediaType = 3;
pub const AVMEDIA_TYPE_DATA: AVMediaType = 2;
pub const AVMEDIA_TYPE_AUDIO: AVMediaType = 1;
pub const AVMEDIA_TYPE_VIDEO: AVMediaType = 0;
pub const AVMEDIA_TYPE_UNKNOWN: AVMediaType = -1;
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
pub type AVFieldOrder = ::core::ffi::c_uint;
pub const AV_FIELD_BT: AVFieldOrder = 5;
pub const AV_FIELD_TB: AVFieldOrder = 4;
pub const AV_FIELD_BB: AVFieldOrder = 3;
pub const AV_FIELD_TT: AVFieldOrder = 2;
pub const AV_FIELD_PROGRESSIVE: AVFieldOrder = 1;
pub const AV_FIELD_UNKNOWN: AVFieldOrder = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
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
pub struct AVBSFContext {
    pub av_class: *const AVClass,
    pub filter: *const AVBitStreamFilter,
    pub priv_data: *mut ::core::ffi::c_void,
    pub par_in: *mut AVCodecParameters,
    pub par_out: *mut AVCodecParameters,
    pub time_base_in: AVRational,
    pub time_base_out: AVRational,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBitStreamFilter {
    pub name: *const ::core::ffi::c_char,
    pub codec_ids: *const AVCodecID,
    pub priv_class: *const AVClass,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFBSFContext {
    pub pub_0: AVBSFContext,
    pub buffer_pkt: *mut AVPacket,
    pub eof: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FFBitStreamFilter {
    pub p: AVBitStreamFilter,
    pub priv_data_size: ::core::ffi::c_int,
    pub init: Option<unsafe extern "C" fn(*mut AVBSFContext) -> ::core::ffi::c_int>,
    pub filter: Option<
        unsafe extern "C" fn(*mut AVBSFContext, *mut AVPacket) -> ::core::ffi::c_int,
    >,
    pub close: Option<unsafe extern "C" fn(*mut AVBSFContext) -> ()>,
    pub flush: Option<unsafe extern "C" fn(*mut AVBSFContext) -> ()>,
}
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
pub struct AVBSFList {
    pub bsfs: *mut *mut AVBSFContext,
    pub nb_bsfs: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct BSFListContext {
    pub class: *const AVClass,
    pub bsfs: *mut *mut AVBSFContext,
    pub nb_bsfs: ::core::ffi::c_int,
    pub idx: ::core::ffi::c_uint,
    pub item_name: *mut ::core::ffi::c_char,
}
#[inline(always)]
unsafe extern "C" fn ff_bsf(
    mut bsf: *const AVBitStreamFilter,
) -> *const FFBitStreamFilter {
    return bsf as *const FFBitStreamFilter;
}
#[inline(always)]
unsafe extern "C" fn ffbsfcontext(mut ctx: *mut AVBSFContext) -> *mut FFBSFContext {
    return ctx as *mut FFBSFContext;
}
#[no_mangle]
pub unsafe extern "C" fn av_bsf_free(mut pctx: *mut *mut AVBSFContext) {
    let mut ctx: *mut AVBSFContext = ::core::ptr::null_mut::<AVBSFContext>();
    let mut bsfi: *mut FFBSFContext = ::core::ptr::null_mut::<FFBSFContext>();
    if pctx.is_null() || (*pctx).is_null() {
        return;
    }
    ctx = *pctx;
    bsfi = ffbsfcontext(ctx);
    if !(*ctx).priv_data.is_null() {
        if (*ff_bsf((*ctx).filter as *const AVBitStreamFilter)).close.is_some() {
            (*ff_bsf((*ctx).filter as *const AVBitStreamFilter))
                .close
                .expect("non-null function pointer")(ctx);
        }
        if !(*(*ctx).filter).priv_class.is_null() {
            av_opt_free((*ctx).priv_data);
        }
        av_freep(&raw mut (*ctx).priv_data as *mut ::core::ffi::c_void);
    }
    av_packet_free(&raw mut (*bsfi).buffer_pkt);
    avcodec_parameters_free(&raw mut (*ctx).par_in);
    avcodec_parameters_free(&raw mut (*ctx).par_out);
    av_freep(pctx as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn bsf_child_next(
    mut obj: *mut ::core::ffi::c_void,
    mut prev: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut ctx: *mut AVBSFContext = obj as *mut AVBSFContext;
    if prev.is_null() && !(*(*ctx).filter).priv_class.is_null() {
        return (*ctx).priv_data;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_void>();
}
unsafe extern "C" fn bsf_to_name(
    mut bsf: *mut ::core::ffi::c_void,
) -> *const ::core::ffi::c_char {
    return (*(*(bsf as *mut AVBSFContext)).filter).name;
}
static mut bsf_class: AVClass = unsafe {
    AVClass {
        class_name: b"AVBSFContext\0" as *const u8 as *const ::core::ffi::c_char,
        item_name: Some(
            bsf_to_name
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
        category: AV_CLASS_CATEGORY_BITSTREAM_FILTER,
        get_category: None,
        query_ranges: None,
        child_next: Some(
            bsf_child_next
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_void,
        ),
        child_class_iterate: Some(
            ff_bsf_child_class_iterate
                as unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> *const AVClass,
        ),
        state_flags_offset: 0,
    }
};
#[no_mangle]
pub unsafe extern "C" fn av_bsf_get_class() -> *const AVClass {
    return &raw const bsf_class;
}
#[no_mangle]
pub unsafe extern "C" fn av_bsf_alloc(
    mut filter: *const AVBitStreamFilter,
    mut pctx: *mut *mut AVBSFContext,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ctx: *mut AVBSFContext = ::core::ptr::null_mut::<AVBSFContext>();
    let mut bsfi: *mut FFBSFContext = ::core::ptr::null_mut::<FFBSFContext>();
    let mut ret: ::core::ffi::c_int = 0;
    bsfi = av_mallocz(::core::mem::size_of::<FFBSFContext>() as size_t)
        as *mut FFBSFContext;
    if bsfi.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    ctx = &raw mut (*bsfi).pub_0;
    (*ctx).av_class = &raw const bsf_class;
    (*ctx).filter = filter as *const AVBitStreamFilter;
    (*ctx).par_in = avcodec_parameters_alloc();
    (*ctx).par_out = avcodec_parameters_alloc();
    if (*ctx).par_in.is_null() || (*ctx).par_out.is_null() {
        ret = -(12 as ::core::ffi::c_int);
    } else {
        if (*ff_bsf(filter)).priv_data_size != 0 {
            (*ctx).priv_data = av_mallocz((*ff_bsf(filter)).priv_data_size as size_t);
            if (*ctx).priv_data.is_null() {
                ret = -(12 as ::core::ffi::c_int);
                current_block = 3643875838826757672;
            } else {
                if !(*filter).priv_class.is_null() {
                    let ref mut fresh0 = *((*ctx).priv_data as *mut *const AVClass);
                    *fresh0 = (*filter).priv_class;
                    av_opt_set_defaults((*ctx).priv_data);
                }
                current_block = 17860125682698302841;
            }
        } else {
            current_block = 17860125682698302841;
        }
        match current_block {
            3643875838826757672 => {}
            _ => {
                (*bsfi).buffer_pkt = av_packet_alloc();
                if (*bsfi).buffer_pkt.is_null() {
                    ret = -(12 as ::core::ffi::c_int);
                } else {
                    *pctx = ctx;
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    av_bsf_free(&raw mut ctx);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_bsf_init(mut ctx: *mut AVBSFContext) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    if !(*(*ctx).filter).codec_ids.is_null() {
        i = 0 as ::core::ffi::c_int;
        while *(*(*ctx).filter).codec_ids.offset(i as isize) as ::core::ffi::c_uint
            != AV_CODEC_ID_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (*(*ctx).par_in).codec_id as ::core::ffi::c_uint
                == *(*(*ctx).filter).codec_ids.offset(i as isize) as ::core::ffi::c_uint
            {
                break;
            }
            i += 1;
        }
        if *(*(*ctx).filter).codec_ids.offset(i as isize) as ::core::ffi::c_uint
            == AV_CODEC_ID_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut desc: *const AVCodecDescriptor = avcodec_descriptor_get(
                (*(*ctx).par_in).codec_id,
            );
            av_log(
                ctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"Codec '%s' (%d) is not supported by the bitstream filter '%s'. Supported codecs are: \0"
                    as *const u8 as *const ::core::ffi::c_char,
                if !desc.is_null() {
                    (*desc).name
                } else {
                    b"unknown\0" as *const u8 as *const ::core::ffi::c_char
                },
                (*(*ctx).par_in).codec_id as ::core::ffi::c_uint,
                (*(*ctx).filter).name,
            );
            i = 0 as ::core::ffi::c_int;
            while *(*(*ctx).filter).codec_ids.offset(i as isize) as ::core::ffi::c_uint
                != AV_CODEC_ID_NONE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                let mut codec_id: AVCodecID = *(*(*ctx).filter)
                    .codec_ids
                    .offset(i as isize);
                av_log(
                    ctx as *mut ::core::ffi::c_void,
                    16 as ::core::ffi::c_int,
                    b"%s (%d) \0" as *const u8 as *const ::core::ffi::c_char,
                    avcodec_get_name(codec_id),
                    codec_id as ::core::ffi::c_uint,
                );
                i += 1;
            }
            av_log(
                ctx as *mut ::core::ffi::c_void,
                16 as ::core::ffi::c_int,
                b"\n\0" as *const u8 as *const ::core::ffi::c_char,
            );
            return -(22 as ::core::ffi::c_int);
        }
    }
    ret = avcodec_parameters_copy((*ctx).par_out, (*ctx).par_in);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    (*ctx).time_base_out = (*ctx).time_base_in;
    if (*ff_bsf((*ctx).filter as *const AVBitStreamFilter)).init.is_some() {
        ret = (*ff_bsf((*ctx).filter as *const AVBitStreamFilter))
            .init
            .expect("non-null function pointer")(ctx);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_bsf_flush(mut ctx: *mut AVBSFContext) {
    let bsfi: *mut FFBSFContext = ffbsfcontext(ctx) as *mut FFBSFContext;
    (*bsfi).eof = 0 as ::core::ffi::c_int;
    av_packet_unref((*bsfi).buffer_pkt);
    if (*ff_bsf((*ctx).filter as *const AVBitStreamFilter)).flush.is_some() {
        (*ff_bsf((*ctx).filter as *const AVBitStreamFilter))
            .flush
            .expect("non-null function pointer")(ctx);
    }
}
#[no_mangle]
pub unsafe extern "C" fn av_bsf_send_packet(
    mut ctx: *mut AVBSFContext,
    mut pkt: *mut AVPacket,
) -> ::core::ffi::c_int {
    let bsfi: *mut FFBSFContext = ffbsfcontext(ctx) as *mut FFBSFContext;
    let mut ret: ::core::ffi::c_int = 0;
    if pkt.is_null() || (*pkt).data.is_null() && (*pkt).side_data_elems == 0 {
        if !pkt.is_null() {
            av_packet_unref(pkt);
        }
        (*bsfi).eof = 1 as ::core::ffi::c_int;
        return 0 as ::core::ffi::c_int;
    }
    if (*bsfi).eof != 0 {
        av_log(
            ctx as *mut ::core::ffi::c_void,
            16 as ::core::ffi::c_int,
            b"A non-NULL packet sent after an EOF.\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        return -(22 as ::core::ffi::c_int);
    }
    if !((*(*bsfi).buffer_pkt).data.is_null()
        && (*(*bsfi).buffer_pkt).side_data_elems == 0)
    {
        return -(11 as ::core::ffi::c_int);
    }
    ret = av_packet_make_refcounted(pkt);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    av_packet_move_ref((*bsfi).buffer_pkt, pkt);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_bsf_receive_packet(
    mut ctx: *mut AVBSFContext,
    mut pkt: *mut AVPacket,
) -> ::core::ffi::c_int {
    return (*ff_bsf((*ctx).filter as *const AVBitStreamFilter))
        .filter
        .expect("non-null function pointer")(ctx, pkt);
}
#[no_mangle]
pub unsafe extern "C" fn ff_bsf_get_packet(
    mut ctx: *mut AVBSFContext,
    mut pkt: *mut *mut AVPacket,
) -> ::core::ffi::c_int {
    let bsfi: *mut FFBSFContext = ffbsfcontext(ctx) as *mut FFBSFContext;
    let mut tmp_pkt: *mut AVPacket = ::core::ptr::null_mut::<AVPacket>();
    if (*bsfi).eof != 0 {
        return -((('E' as i32 | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*(*bsfi).buffer_pkt).data.is_null() && (*(*bsfi).buffer_pkt).side_data_elems == 0
    {
        return -(11 as ::core::ffi::c_int);
    }
    tmp_pkt = av_packet_alloc();
    if tmp_pkt.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    *pkt = (*bsfi).buffer_pkt;
    (*bsfi).buffer_pkt = tmp_pkt;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_bsf_get_packet_ref(
    mut ctx: *mut AVBSFContext,
    mut pkt: *mut AVPacket,
) -> ::core::ffi::c_int {
    let bsfi: *mut FFBSFContext = ffbsfcontext(ctx) as *mut FFBSFContext;
    if (*bsfi).eof != 0 {
        return -((('E' as i32 | ('O' as i32) << 8 as ::core::ffi::c_int
            | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    if (*(*bsfi).buffer_pkt).data.is_null() && (*(*bsfi).buffer_pkt).side_data_elems == 0
    {
        return -(11 as ::core::ffi::c_int);
    }
    av_packet_move_ref(pkt, (*bsfi).buffer_pkt);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn bsf_list_init(mut bsf: *mut AVBSFContext) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut lst: *mut BSFListContext = (*bsf).priv_data as *mut BSFListContext;
    let mut ret: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut cod_par: *const AVCodecParameters = (*bsf).par_in;
    let mut tb: AVRational = (*bsf).time_base_in;
    i = 0 as ::core::ffi::c_int;
    loop {
        if !(i < (*lst).nb_bsfs) {
            current_block = 5720623009719927633;
            break;
        }
        ret = avcodec_parameters_copy(
            (**(*lst).bsfs.offset(i as isize)).par_in,
            cod_par,
        );
        if ret < 0 as ::core::ffi::c_int {
            current_block = 10881884107188479999;
            break;
        }
        (**(*lst).bsfs.offset(i as isize)).time_base_in = tb;
        ret = av_bsf_init(*(*lst).bsfs.offset(i as isize));
        if ret < 0 as ::core::ffi::c_int {
            current_block = 10881884107188479999;
            break;
        }
        cod_par = (**(*lst).bsfs.offset(i as isize)).par_out;
        tb = (**(*lst).bsfs.offset(i as isize)).time_base_out;
        i += 1;
    }
    match current_block {
        5720623009719927633 => {
            (*bsf).time_base_out = tb;
            ret = avcodec_parameters_copy((*bsf).par_out, cod_par);
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn bsf_list_filter(
    mut bsf: *mut AVBSFContext,
    mut out: *mut AVPacket,
) -> ::core::ffi::c_int {
    let mut lst: *mut BSFListContext = (*bsf).priv_data as *mut BSFListContext;
    let mut ret: ::core::ffi::c_int = 0;
    let mut eof: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*lst).nb_bsfs == 0 {
        return ff_bsf_get_packet_ref(bsf, out);
    }
    loop {
        if (*lst).idx != 0 {
            ret = av_bsf_receive_packet(
                *(*lst)
                    .bsfs
                    .offset((*lst).idx.wrapping_sub(1 as ::core::ffi::c_uint) as isize),
                out,
            );
        } else {
            ret = ff_bsf_get_packet_ref(bsf, out);
        }
        if ret == -(11 as ::core::ffi::c_int) {
            if (*lst).idx == 0 {
                return ret;
            }
            (*lst).idx = (*lst).idx.wrapping_sub(1);
        } else {
            if ret
                == -((('E' as i32 | ('O' as i32) << 8 as ::core::ffi::c_int
                    | ('F' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | (' ' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int)
            {
                eof = 1 as ::core::ffi::c_int;
            } else if ret < 0 as ::core::ffi::c_int {
                return ret
            }
            if (*lst).idx < (*lst).nb_bsfs as ::core::ffi::c_uint {
                ret = av_bsf_send_packet(
                    *(*lst).bsfs.offset((*lst).idx as isize),
                    if eof != 0 { ::core::ptr::null_mut::<AVPacket>() } else { out },
                );
                if ret < 0 as ::core::ffi::c_int {
                    av_packet_unref(out);
                    return ret;
                }
                (*lst).idx = (*lst).idx.wrapping_add(1);
                eof = 0 as ::core::ffi::c_int;
            } else if eof != 0 {
                return ret
            } else {
                return 0 as ::core::ffi::c_int
            }
        }
    };
}
unsafe extern "C" fn bsf_list_flush(mut bsf: *mut AVBSFContext) {
    let mut lst: *mut BSFListContext = (*bsf).priv_data as *mut BSFListContext;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*lst).nb_bsfs {
        av_bsf_flush(*(*lst).bsfs.offset(i as isize));
        i += 1;
    }
    (*lst).idx = 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn bsf_list_close(mut bsf: *mut AVBSFContext) {
    let mut lst: *mut BSFListContext = (*bsf).priv_data as *mut BSFListContext;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < (*lst).nb_bsfs {
        av_bsf_free((*lst).bsfs.offset(i as isize) as *mut *mut AVBSFContext);
        i += 1;
    }
    av_freep(&raw mut (*lst).bsfs as *mut ::core::ffi::c_void);
    av_freep(&raw mut (*lst).item_name as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn bsf_list_item_name(
    mut ctx: *mut ::core::ffi::c_void,
) -> *const ::core::ffi::c_char {
    static mut null_filter_name: *const ::core::ffi::c_char = b"null\0" as *const u8
        as *const ::core::ffi::c_char;
    let mut bsf_ctx: *mut AVBSFContext = ctx as *mut AVBSFContext;
    let mut lst: *mut BSFListContext = (*bsf_ctx).priv_data as *mut BSFListContext;
    if (*lst).nb_bsfs == 0 {
        return null_filter_name;
    }
    if (*lst).item_name.is_null() {
        let mut i: ::core::ffi::c_int = 0;
        let mut bp: AVBPrint = AVBPrint {
            str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            len: 0,
            size: 0,
            size_max: 0,
            reserved_internal_buffer: [0; 1],
            reserved_padding: [0; 1000],
        };
        av_bprint_init(
            &raw mut bp,
            16 as ::core::ffi::c_uint,
            128 as ::core::ffi::c_uint,
        );
        av_bprintf(
            &raw mut bp,
            b"bsf_list(\0" as *const u8 as *const ::core::ffi::c_char,
        );
        i = 0 as ::core::ffi::c_int;
        while i < (*lst).nb_bsfs {
            av_bprintf(
                &raw mut bp,
                if i != 0 {
                    b",%s\0" as *const u8 as *const ::core::ffi::c_char
                } else {
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char
                },
                (*(**(*lst).bsfs.offset(i as isize)).filter).name,
            );
            i += 1;
        }
        av_bprintf(&raw mut bp, b")\0" as *const u8 as *const ::core::ffi::c_char);
        av_bprint_finalize(&raw mut bp, &raw mut (*lst).item_name);
    }
    return (*lst).item_name;
}
static mut bsf_list_class: AVClass = unsafe {
    AVClass {
        class_name: b"bsf_list\0" as *const u8 as *const ::core::ffi::c_char,
        item_name: Some(
            bsf_list_item_name
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
static mut list_bsf: FFBitStreamFilter = unsafe {
    FFBitStreamFilter {
        p: AVBitStreamFilter {
            name: b"bsf_list\0" as *const u8 as *const ::core::ffi::c_char,
            codec_ids: ::core::ptr::null::<AVCodecID>(),
            priv_class: &raw const bsf_list_class,
        },
        priv_data_size: ::core::mem::size_of::<BSFListContext>() as ::core::ffi::c_int,
        init: Some(
            bsf_list_init
                as unsafe extern "C" fn(*mut AVBSFContext) -> ::core::ffi::c_int,
        ),
        filter: Some(
            bsf_list_filter
                as unsafe extern "C" fn(
                    *mut AVBSFContext,
                    *mut AVPacket,
                ) -> ::core::ffi::c_int,
        ),
        close: Some(bsf_list_close as unsafe extern "C" fn(*mut AVBSFContext) -> ()),
        flush: Some(bsf_list_flush as unsafe extern "C" fn(*mut AVBSFContext) -> ()),
    }
};
#[no_mangle]
pub unsafe extern "C" fn av_bsf_list_alloc() -> *mut AVBSFList {
    return av_mallocz(::core::mem::size_of::<AVBSFList>() as size_t) as *mut AVBSFList;
}
#[no_mangle]
pub unsafe extern "C" fn av_bsf_list_free(mut lst: *mut *mut AVBSFList) {
    let mut i: ::core::ffi::c_int = 0;
    if (*lst).is_null() {
        return;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (**lst).nb_bsfs {
        av_bsf_free((**lst).bsfs.offset(i as isize) as *mut *mut AVBSFContext);
        i += 1;
    }
    av_free((**lst).bsfs as *mut ::core::ffi::c_void);
    av_freep(lst as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_bsf_list_append(
    mut lst: *mut AVBSFList,
    mut bsf: *mut AVBSFContext,
) -> ::core::ffi::c_int {
    return av_dynarray_add_nofree(
        &raw mut (*lst).bsfs as *mut ::core::ffi::c_void,
        &raw mut (*lst).nb_bsfs,
        bsf as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn bsf_list_append_internal(
    mut lst: *mut AVBSFList,
    mut bsf_name: *const ::core::ffi::c_char,
    mut options: *const ::core::ffi::c_char,
    mut options_dict: *mut *mut AVDictionary,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ret: ::core::ffi::c_int = 0;
    let mut filter: *const AVBitStreamFilter = ::core::ptr::null::<AVBitStreamFilter>();
    let mut bsf: *mut AVBSFContext = ::core::ptr::null_mut::<AVBSFContext>();
    filter = av_bsf_get_by_name(bsf_name);
    if filter.is_null() {
        return -(((0xf8 as ::core::ffi::c_int | ('B' as i32) << 8 as ::core::ffi::c_int
            | ('S' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('F' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    ret = av_bsf_alloc(filter, &raw mut bsf);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    if !options.is_null() && !(*filter).priv_class.is_null() {
        let mut opt: *const AVOption = av_opt_next(
            (*bsf).priv_data,
            ::core::ptr::null::<AVOption>(),
        );
        let mut shorthand: [*const ::core::ffi::c_char; 2] = [
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        ];
        if !opt.is_null() {
            shorthand[0 as ::core::ffi::c_int as usize] = (*opt).name;
        }
        ret = av_opt_set_from_string(
            (*bsf).priv_data,
            options,
            &raw mut shorthand as *mut *const ::core::ffi::c_char,
            b"=\0" as *const u8 as *const ::core::ffi::c_char,
            b":\0" as *const u8 as *const ::core::ffi::c_char,
        );
        if ret < 0 as ::core::ffi::c_int {
            current_block = 418820107395966299;
        } else {
            current_block = 13109137661213826276;
        }
    } else {
        current_block = 13109137661213826276;
    }
    match current_block {
        13109137661213826276 => {
            if !options_dict.is_null() {
                ret = av_opt_set_dict2(
                    bsf as *mut ::core::ffi::c_void,
                    options_dict as *mut *mut AVDictionary,
                    (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int,
                );
                if ret < 0 as ::core::ffi::c_int {
                    current_block = 418820107395966299;
                } else {
                    current_block = 12800627514080957624;
                }
            } else {
                current_block = 12800627514080957624;
            }
            match current_block {
                418820107395966299 => {}
                _ => {
                    ret = av_bsf_list_append(lst, bsf);
                }
            }
        }
        _ => {}
    }
    if ret < 0 as ::core::ffi::c_int {
        av_bsf_free(&raw mut bsf);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_bsf_list_append2(
    mut lst: *mut AVBSFList,
    mut bsf_name: *const ::core::ffi::c_char,
    mut options: *mut *mut AVDictionary,
) -> ::core::ffi::c_int {
    return bsf_list_append_internal(
        lst,
        bsf_name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        options,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_bsf_list_finalize(
    mut lst: *mut *mut AVBSFList,
    mut bsf: *mut *mut AVBSFContext,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ctx: *mut BSFListContext = ::core::ptr::null_mut::<BSFListContext>();
    if (**lst).nb_bsfs == 1 as ::core::ffi::c_int {
        *bsf = *(**lst).bsfs.offset(0 as ::core::ffi::c_int as isize);
        av_freep(&raw mut (**lst).bsfs as *mut ::core::ffi::c_void);
        (**lst).nb_bsfs = 0 as ::core::ffi::c_int;
    } else {
        ret = av_bsf_alloc(&raw const list_bsf.p, bsf);
        if ret < 0 as ::core::ffi::c_int {
            return ret;
        }
        ctx = (**bsf).priv_data as *mut BSFListContext;
        (*ctx).bsfs = (**lst).bsfs;
        (*ctx).nb_bsfs = (**lst).nb_bsfs;
    }
    av_freep(lst as *mut ::core::ffi::c_void);
    return ret;
}
unsafe extern "C" fn bsf_parse_single(
    mut str: *mut ::core::ffi::c_char,
    mut bsf_lst: *mut AVBSFList,
) -> ::core::ffi::c_int {
    let mut bsf_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut bsf_options_str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    bsf_name = av_strtok(
        str,
        b"=\0" as *const u8 as *const ::core::ffi::c_char,
        &raw mut bsf_options_str,
    );
    if bsf_name.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    return bsf_list_append_internal(
        bsf_lst,
        bsf_name,
        bsf_options_str,
        ::core::ptr::null_mut::<*mut AVDictionary>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_bsf_list_parse_str(
    mut str: *const ::core::ffi::c_char,
    mut bsf_lst: *mut *mut AVBSFContext,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut lst: *mut AVBSFList = ::core::ptr::null_mut::<AVBSFList>();
    let mut ret: ::core::ffi::c_int = 0;
    if str.is_null() {
        return av_bsf_get_null_filter(bsf_lst);
    }
    lst = av_bsf_list_alloc();
    if lst.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    loop {
        let mut bsf_str: *mut ::core::ffi::c_char = av_get_token(
            &raw mut str,
            b",\0" as *const u8 as *const ::core::ffi::c_char,
        );
        ret = bsf_parse_single(bsf_str, lst);
        av_free(bsf_str as *mut ::core::ffi::c_void);
        if ret < 0 as ::core::ffi::c_int {
            current_block = 5674972574772054786;
            break;
        }
        if !(*str as ::core::ffi::c_int != 0
            && {
                str = str.offset(1);
                *str as ::core::ffi::c_int != 0
            })
        {
            current_block = 14523784380283086299;
            break;
        }
    }
    match current_block {
        14523784380283086299 => {
            ret = av_bsf_list_finalize(&raw mut lst, bsf_lst);
        }
        _ => {}
    }
    if ret < 0 as ::core::ffi::c_int {
        av_bsf_list_free(&raw mut lst);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_bsf_get_null_filter(
    mut bsf: *mut *mut AVBSFContext,
) -> ::core::ffi::c_int {
    return av_bsf_alloc(&raw const list_bsf.p, bsf);
}
#[repr(C)]
pub struct AVDictionary {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVBuffer {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVProfile {
    _opaque: [u8; 0],
}
