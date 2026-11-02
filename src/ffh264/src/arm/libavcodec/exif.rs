// Translated from FFmpeg 9.0.1 libavcodec/exif.c
// (preprocessed for armv7-unknown-linux-musleabihf) by c2rust 0.22.1, then tools/ffh264/postprocess.py.
// Original notice:
//
// EXIF metadata parser
// Copyright (c) 2013 Thilo Borgmann <thilo.borgmann _at_ mail.de>
// Copyright (c) 2024-2025 Leo Izen <leo.izen@gmail.com>
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
    fn strcmp(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn abort() -> !;
    fn ff_tis_ifd(tag: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    fn __fpclassifyl(_: crate::rt::F128) -> ::core::ffi::c_int;
    fn ff_tget_short(
        gb: *mut GetByteContext,
        le: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uint;
    fn ff_tget_long(
        gb: *mut GetByteContext,
        le: ::core::ffi::c_int,
    ) -> ::core::ffi::c_uint;
    fn ff_tget_double(
        gb: *mut GetByteContext,
        le: ::core::ffi::c_int,
    ) -> ::core::ffi::c_double;
    fn av_display_rotation_get(matrix: *const int32_t) -> ::core::ffi::c_double;
    fn ff_tdecode_header(
        gb: *mut GetByteContext,
        le: *mut ::core::ffi::c_int,
        ifd_offset: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_display_rotation_set(matrix: *mut int32_t, angle: ::core::ffi::c_double);
    fn av_strerror(
        errnum: ::core::ffi::c_int,
        errbuf: *mut ::core::ffi::c_char,
        errbuf_size: size_t,
    ) -> ::core::ffi::c_int;
    fn av_display_matrix_flip(
        matrix: *mut int32_t,
        hflip: ::core::ffi::c_int,
        vflip: ::core::ffi::c_int,
    );
    fn av_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_mallocz(size: size_t) -> *mut ::core::ffi::c_void;
    fn av_bprint_init(
        buf: *mut AVBPrint,
        size_init: ::core::ffi::c_uint,
        size_max: ::core::ffi::c_uint,
    );
    fn av_buffer_ref(buf: *const AVBufferRef) -> *mut AVBufferRef;
    fn av_buffer_unref(buf: *mut *mut AVBufferRef);
    fn av_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    fn av_bprintf(buf: *mut AVBPrint, fmt: *const ::core::ffi::c_char, ...);
    fn av_dict_set(
        pm: *mut *mut AVDictionary,
        key: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn av_buffer_realloc(buf: *mut *mut AVBufferRef, size: size_t) -> ::core::ffi::c_int;
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
    fn av_fast_realloc(
        ptr: *mut ::core::ffi::c_void,
        size: *mut ::core::ffi::c_uint,
        min_size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_fast_mallocz(
        ptr: *mut ::core::ffi::c_void,
        size: *mut ::core::ffi::c_uint,
        min_size: size_t,
    );
    fn av_free(ptr: *mut ::core::ffi::c_void);
    fn av_freep(ptr: *mut ::core::ffi::c_void);
    fn av_memdup(
        p: *const ::core::ffi::c_void,
        size: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn av_size_mult(a: size_t, b: size_t, r: *mut size_t) -> ::core::ffi::c_int;
    fn av_frame_get_side_data(
        frame: *const AVFrame,
        type_0: AVFrameSideDataType,
    ) -> *mut AVFrameSideData;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBPrint {
    pub str_0: *mut ::core::ffi::c_char,
    pub len: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
    pub size_max: ::core::ffi::c_uint,
    pub reserved_internal_buffer: [::core::ffi::c_char; 1],
    pub reserved_padding: [::core::ffi::c_char; 1004],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __f: ::core::ffi::c_float,
    pub __i: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub __f: ::core::ffi::c_double,
    pub __i: ::core::ffi::c_ulonglong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVRational {
    pub num: ::core::ffi::c_int,
    pub den: ::core::ffi::c_int,
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
pub struct PutByteContext {
    pub buffer: *mut uint8_t,
    pub buffer_end: *mut uint8_t,
    pub buffer_start: *mut uint8_t,
    pub eof: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVBufferRef {
    pub buffer: *mut AVBuffer,
    pub data: *mut uint8_t,
    pub size: size_t,
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
    pub u: C2RustUnnamed_1,
    pub opaque: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub type AVTiffDataType = ::core::ffi::c_uint;
pub const AV_TIFF_IFD: AVTiffDataType = 13;
pub const AV_TIFF_DOUBLE: AVTiffDataType = 12;
pub const AV_TIFF_FLOAT: AVTiffDataType = 11;
pub const AV_TIFF_SRATIONAL: AVTiffDataType = 10;
pub const AV_TIFF_SLONG: AVTiffDataType = 9;
pub const AV_TIFF_SSHORT: AVTiffDataType = 8;
pub const AV_TIFF_UNDEFINED: AVTiffDataType = 7;
pub const AV_TIFF_SBYTE: AVTiffDataType = 6;
pub const AV_TIFF_RATIONAL: AVTiffDataType = 5;
pub const AV_TIFF_LONG: AVTiffDataType = 4;
pub const AV_TIFF_SHORT: AVTiffDataType = 3;
pub const AV_TIFF_STRING: AVTiffDataType = 2;
pub const AV_TIFF_BYTE: AVTiffDataType = 1;
pub type AVExifHeaderMode = ::core::ffi::c_uint;
pub const AV_EXIF_EXIF00: AVExifHeaderMode = 4;
pub const AV_EXIF_T_OFF: AVExifHeaderMode = 3;
pub const AV_EXIF_ASSUME_BE: AVExifHeaderMode = 2;
pub const AV_EXIF_ASSUME_LE: AVExifHeaderMode = 1;
pub const AV_EXIF_TIFF_HEADER: AVExifHeaderMode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVExifEntry {
    pub id: uint16_t,
    pub type_0: AVTiffDataType,
    pub count: uint32_t,
    pub ifd_offset: uint32_t,
    pub ifd_lead: *mut uint8_t,
    pub value: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub ptr: *mut ::core::ffi::c_void,
    pub sint: *mut int64_t,
    pub uint: *mut uint64_t,
    pub dbl: *mut ::core::ffi::c_double,
    pub str_0: *mut ::core::ffi::c_char,
    pub ubytes: *mut uint8_t,
    pub sbytes: *mut int8_t,
    pub rat: *mut AVRational,
    pub ifd: AVExifMetadata,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AVExifMetadata {
    pub entries: *mut AVExifEntry,
    pub count: ::core::ffi::c_uint,
    pub size: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exif_tag {
    pub name: [::core::ffi::c_char; 32],
    pub id: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exif_makernote_data {
    pub header: *const uint8_t,
    pub header_size: size_t,
    pub result: ::core::ffi::c_int,
}
#[inline(always)]
unsafe extern "C" fn av_bswap16(mut x: uint16_t) -> uint16_t {
    x = (x as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
        | (x as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint16_t;
    return x;
}
#[inline]
unsafe extern "C" fn __FLOAT_BITS(mut __f: ::core::ffi::c_float) -> ::core::ffi::c_uint {
    let mut __u: C2RustUnnamed = C2RustUnnamed { __f: 0. };
    __u.__f = __f;
    return __u.__i;
}
#[inline]
unsafe extern "C" fn __DOUBLE_BITS(
    mut __f: ::core::ffi::c_double,
) -> ::core::ffi::c_ulonglong {
    let mut __u: C2RustUnnamed_0 = C2RustUnnamed_0 { __f: 0. };
    __u.__f = __f;
    return __u.__i;
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
static mut tag_list: [exif_tag; 142] = unsafe {
    [
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSVersionID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSLatitudeRef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x1 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSLatitude\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x2 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSLongitudeRef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x3 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSLongitude\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x4 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSAltitudeRef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x5 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSAltitude\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x6 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSTimeStamp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x7 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSSatellites\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x8 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSStatus\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSMeasureMode\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSDOP\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xb as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSSpeedRef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xc as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSSpeed\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xd as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSTrackRef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xe as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSTrack\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xf as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSImgDirectionRef\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x10 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSImgDirection\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x11 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSMapDatum\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x12 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSDestLatitudeRef\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x13 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSDestLatitude\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x14 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSDestLongitudeRef\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x15 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSDestLongitude\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x16 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSDestBearingRef\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x17 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSDestBearing\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x18 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSDestDistanceRef\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x19 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSDestDistance\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x1a as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSProcessingMethod\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x1b as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSAreaInformation\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x1c as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSDateStamp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x1d as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSDifferential\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x1e as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ImageWidth\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x100 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ImageLength\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x101 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"BitsPerSample\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x102 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"Compression\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x103 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"PhotometricInterpretation\0\0\0\0\0\0\0"),
            id: 0x106 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"Orientation\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x112 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"SamplesPerPixel\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x115 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"PlanarConfiguration\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x11c as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"YCbCrSubSampling\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x212 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"YCbCrPositioning\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x213 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"XResolution\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x11a as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"YResolution\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x11b as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ResolutionUnit\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x128 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"StripOffsets\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x111 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"RowsPerStrip\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x116 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"StripByteCounts\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x117 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"JPEGInterchangeFormat\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x201 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"JPEGInterchangeFormatLength\0\0\0\0\0"),
            id: 0x202 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"TransferFunction\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x12d as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"WhitePoint\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x13e as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"PrimaryChromaticities\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x13f as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"YCbCrCoefficients\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x211 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ReferenceBlackWhite\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x214 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"DateTime\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x132 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ImageDescription\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x10e as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"Make\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x10f as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"Model\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x110 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"Software\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x131 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"Artist\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x13b as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"Copyright\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x8298 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ExifVersion\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9000 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"FlashpixVersion\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa000 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ColorSpace\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa001 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ComponentsConfiguration\0\0\0\0\0\0\0\0\0"),
            id: 0x9101 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"CompressedBitsPerPixel\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9102 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"PixelXDimension\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa002 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"PixelYDimension\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa003 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"MakerNote\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x927c as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"UserComment\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9286 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"RelatedSoundFile\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa004 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"DateTimeOriginal\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9003 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"DateTimeDigitized\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9004 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"SubSecTime\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9290 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"SubSecTimeOriginal\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9291 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"SubSecTimeDigitized\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9292 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ImageUniqueID\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa420 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ExposureTime\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x829a as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"FNumber\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x829d as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ExposureProgram\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x8822 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"SpectralSensitivity\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x8824 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ISOSpeedRatings\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x8827 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"OECF\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x8828 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ShutterSpeedValue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9201 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ApertureValue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9202 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"BrightnessValue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9203 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ExposureBiasValue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9204 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"MaxApertureValue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9205 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"SubjectDistance\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9206 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"MeteringMode\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9207 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"LightSource\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9208 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"Flash\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9209 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"FocalLength\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x920a as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"SubjectArea\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x9214 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"FlashEnergy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa20b as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"SpatialFrequencyResponse\0\0\0\0\0\0\0\0"),
            id: 0xa20c as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"FocalPlaneXResolution\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa20e as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"FocalPlaneYResolution\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa20f as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"FocalPlaneResolutionUnit\0\0\0\0\0\0\0\0"),
            id: 0xa210 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"SubjectLocation\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa214 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ExposureIndex\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa215 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"SensingMethod\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa217 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"FileSource\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa300 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"SceneType\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa301 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"CFAPattern\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa302 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"CustomRendered\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa401 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ExposureMode\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa402 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"WhiteBalance\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa403 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"DigitalZoomRatio\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa404 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"FocalLengthIn35mmFilm\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa405 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"SceneCaptureType\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa406 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GainControl\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa407 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"Contrast\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa408 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"Saturation\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa409 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"Sharpness\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa40a as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"DeviceSettingDescription\0\0\0\0\0\0\0\0"),
            id: 0xa40b as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"SubjectDistanceRange\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa40c as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"RelatedImageFileFormat\0\0\0\0\0\0\0\0\0\0"),
            id: 0x1000 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"RelatedImageWidth\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x1001 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"RelatedImageLength\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x1002 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"PrintImageMatching\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xc4a5 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ExifIFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x8769 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GPSInfo\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x8825 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"InteropIFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xa005 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"GlobalParametersIFD\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0x190 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"ProfileIFD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xc6f5 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD1\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xfffc as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD2\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xfffb as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD3\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xfffa as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD4\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xfff9 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD5\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xfff8 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD6\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xfff7 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD7\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xfff6 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD8\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xfff5 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD9\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xfff4 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD10\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xfff3 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD11\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xfff2 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD12\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xfff1 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD13\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xfff0 as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD14\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xffef as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD15\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xffee as uint16_t,
        },
        exif_tag {
            name: ::core::mem::transmute::<
                [u8; 32],
                [::core::ffi::c_char; 32],
            >(*b"IFD16\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"),
            id: 0xffed as uint16_t,
        },
    ]
};
#[inline]
unsafe extern "C" fn av_bswap64(mut x: uint64_t) -> uint64_t {
    return (av_bswap32(x as uint32_t) as uint64_t) << 32 as ::core::ffi::c_int
        | av_bswap32((x >> 32 as ::core::ffi::c_int) as uint32_t) as uint64_t;
}
#[inline]
unsafe extern "C" fn av_make_q(
    mut num: ::core::ffi::c_int,
    mut den: ::core::ffi::c_int,
) -> AVRational {
    let mut r: AVRational = AVRational { num: num, den: den };
    return r;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_put_le64(mut p: *mut PutByteContext, value: uint64_t) {
    if (*p).eof == 0
        && (*p).buffer_end.offset_from((*p).buffer) as ::core::ffi::c_int
            >= 8 as ::core::ffi::c_int
    {
        (*((*p).buffer as *mut unaligned_64)).l = value;
        (*p).buffer = (*p).buffer.offset(8 as ::core::ffi::c_int as isize);
    } else {
        (*p).eof = 1 as ::core::ffi::c_int;
    };
}
#[inline(always)]
unsafe extern "C" fn bytestream2_put_le32(
    mut p: *mut PutByteContext,
    value: ::core::ffi::c_uint,
) {
    if (*p).eof == 0
        && (*p).buffer_end.offset_from((*p).buffer) as ::core::ffi::c_int
            >= 4 as ::core::ffi::c_int
    {
        (*((*p).buffer as *mut unaligned_32)).l = value as uint32_t;
        (*p).buffer = (*p).buffer.offset(4 as ::core::ffi::c_int as isize);
    } else {
        (*p).eof = 1 as ::core::ffi::c_int;
    };
}
#[inline(always)]
unsafe extern "C" fn bytestream2_put_le16(
    mut p: *mut PutByteContext,
    value: ::core::ffi::c_uint,
) {
    if (*p).eof == 0
        && (*p).buffer_end.offset_from((*p).buffer) as ::core::ffi::c_int
            >= 2 as ::core::ffi::c_int
    {
        (*((*p).buffer as *mut unaligned_16)).l = value as uint16_t;
        (*p).buffer = (*p).buffer.offset(2 as ::core::ffi::c_int as isize);
    } else {
        (*p).eof = 1 as ::core::ffi::c_int;
    };
}
#[inline(always)]
unsafe extern "C" fn bytestream2_put_be64(mut p: *mut PutByteContext, value: uint64_t) {
    if (*p).eof == 0
        && (*p).buffer_end.offset_from((*p).buffer) as ::core::ffi::c_int
            >= 8 as ::core::ffi::c_int
    {
        (*((*p).buffer as *mut unaligned_64)).l = av_bswap64(value);
        (*p).buffer = (*p).buffer.offset(8 as ::core::ffi::c_int as isize);
    } else {
        (*p).eof = 1 as ::core::ffi::c_int;
    };
}
#[inline(always)]
unsafe extern "C" fn bytestream2_put_be32(
    mut p: *mut PutByteContext,
    value: ::core::ffi::c_uint,
) {
    if (*p).eof == 0
        && (*p).buffer_end.offset_from((*p).buffer) as ::core::ffi::c_int
            >= 4 as ::core::ffi::c_int
    {
        (*((*p).buffer as *mut unaligned_32)).l = av_bswap32(value as uint32_t);
        (*p).buffer = (*p).buffer.offset(4 as ::core::ffi::c_int as isize);
    } else {
        (*p).eof = 1 as ::core::ffi::c_int;
    };
}
#[inline(always)]
unsafe extern "C" fn bytestream2_put_be16(
    mut p: *mut PutByteContext,
    value: ::core::ffi::c_uint,
) {
    if (*p).eof == 0
        && (*p).buffer_end.offset_from((*p).buffer) as ::core::ffi::c_int
            >= 2 as ::core::ffi::c_int
    {
        (*((*p).buffer as *mut unaligned_16)).l = av_bswap16(value as uint16_t);
        (*p).buffer = (*p).buffer.offset(2 as ::core::ffi::c_int as isize);
    } else {
        (*p).eof = 1 as ::core::ffi::c_int;
    };
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
#[inline(always)]
unsafe extern "C" fn bytestream2_init(
    mut g: *mut GetByteContext,
    mut buf: *const uint8_t,
    mut buf_size: ::core::ffi::c_int,
) {
    if !(!buf.is_null() && buf_size >= 0 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"buf && buf_size >= 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/bytestream.h\0" as *const u8 as *const ::core::ffi::c_char,
            141 as ::core::ffi::c_int,
        );
        abort();
    }
    (*g).buffer = buf;
    (*g).buffer_start = buf;
    (*g).buffer_end = buf.offset(buf_size as isize);
}
#[inline(always)]
unsafe extern "C" fn bytestream2_init_writer(
    mut p: *mut PutByteContext,
    mut buf: *mut uint8_t,
    mut buf_size: ::core::ffi::c_int,
) {
    if !(!buf.is_null() && buf_size >= 0 as ::core::ffi::c_int) {
        av_log(
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
            0 as ::core::ffi::c_int,
            b"Assertion %s failed at %s:%d\n\0" as *const u8
                as *const ::core::ffi::c_char,
            b"buf && buf_size >= 0\0" as *const u8 as *const ::core::ffi::c_char,
            b"src/libavcodec/bytestream.h\0" as *const u8 as *const ::core::ffi::c_char,
            151 as ::core::ffi::c_int,
        );
        abort();
    }
    (*p).buffer = buf;
    (*p).buffer_start = buf;
    (*p).buffer_end = buf.offset(buf_size as isize);
    (*p).eof = 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_bytes_left(
    mut g: *const GetByteContext,
) -> ::core::ffi::c_int {
    return (*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn av_clip_c(
    mut a: ::core::ffi::c_int,
    mut amin: ::core::ffi::c_int,
    mut amax: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if a < amin { return amin } else if a > amax { return amax } else { return a };
}
#[inline(always)]
unsafe extern "C" fn bytestream2_tell(
    mut g: *const GetByteContext,
) -> ::core::ffi::c_int {
    return (*g).buffer.offset_from((*g).buffer_start) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_tell_p(
    mut p: *const PutByteContext,
) -> ::core::ffi::c_int {
    return (*p).buffer.offset_from((*p).buffer_start) as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_seek(
    mut g: *mut GetByteContext,
    mut offset: ::core::ffi::c_int,
    mut whence: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match whence {
        1 => {
            offset = av_clip_c(
                offset,
                -((*g).buffer.offset_from((*g).buffer_start) as ::core::ffi::c_int),
                (*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_int,
            );
            (*g).buffer = (*g).buffer.offset(offset as isize);
        }
        2 => {
            offset = av_clip_c(
                offset,
                -((*g).buffer_end.offset_from((*g).buffer_start) as ::core::ffi::c_int),
                0 as ::core::ffi::c_int,
            );
            (*g).buffer = (*g).buffer_end.offset(offset as isize);
        }
        0 => {
            offset = av_clip_c(
                offset,
                0 as ::core::ffi::c_int,
                (*g).buffer_end.offset_from((*g).buffer_start) as ::core::ffi::c_int,
            );
            (*g).buffer = (*g).buffer_start.offset(offset as isize);
        }
        _ => return -(22 as ::core::ffi::c_int),
    }
    return bytestream2_tell(g);
}
static mut exif_sizes: [size_t; 14] = [
    0 as ::core::ffi::c_int as size_t,
    1 as ::core::ffi::c_int as size_t,
    1 as ::core::ffi::c_int as size_t,
    2 as ::core::ffi::c_int as size_t,
    4 as ::core::ffi::c_int as size_t,
    8 as ::core::ffi::c_int as size_t,
    1 as ::core::ffi::c_int as size_t,
    1 as ::core::ffi::c_int as size_t,
    2 as ::core::ffi::c_int as size_t,
    4 as ::core::ffi::c_int as size_t,
    8 as ::core::ffi::c_int as size_t,
    4 as ::core::ffi::c_int as size_t,
    8 as ::core::ffi::c_int as size_t,
    4 as ::core::ffi::c_int as size_t,
];
#[inline]
unsafe extern "C" fn av_bprint_is_complete(
    mut buf: *const AVBPrint,
) -> ::core::ffi::c_int {
    return ((*buf).len < (*buf).size) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_exif_get_tag_name(
    mut id: uint16_t,
) -> *const ::core::ffi::c_char {
    let mut i: size_t = 0 as size_t;
    while (i as usize)
        < (::core::mem::size_of::<[exif_tag; 142]>() as usize)
            .wrapping_div(::core::mem::size_of::<exif_tag>() as usize)
    {
        if tag_list[i as usize].id as ::core::ffi::c_int == id as ::core::ffi::c_int {
            return &raw const (*(&raw const tag_list as *const exif_tag)
                .offset(i as isize))
                .name as *const ::core::ffi::c_char;
        }
        i = i.wrapping_add(1);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[inline(always)]
unsafe extern "C" fn bytestream2_seek_p(
    mut p: *mut PutByteContext,
    mut offset: ::core::ffi::c_int,
    mut whence: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*p).eof = 0 as ::core::ffi::c_int;
    match whence {
        1 => {
            if ((*p).buffer_end.offset_from((*p).buffer) as ::core::ffi::c_int) < offset
            {
                (*p).eof = 1 as ::core::ffi::c_int;
            }
            offset = av_clip_c(
                offset,
                -((*p).buffer.offset_from((*p).buffer_start) as ::core::ffi::c_int),
                (*p).buffer_end.offset_from((*p).buffer) as ::core::ffi::c_int,
            );
            (*p).buffer = (*p).buffer.offset(offset as isize);
        }
        2 => {
            if offset > 0 as ::core::ffi::c_int {
                (*p).eof = 1 as ::core::ffi::c_int;
            }
            offset = av_clip_c(
                offset,
                -((*p).buffer_end.offset_from((*p).buffer_start) as ::core::ffi::c_int),
                0 as ::core::ffi::c_int,
            );
            (*p).buffer = (*p).buffer_end.offset(offset as isize);
        }
        0 => {
            if ((*p).buffer_end.offset_from((*p).buffer_start) as ::core::ffi::c_int)
                < offset
            {
                (*p).eof = 1 as ::core::ffi::c_int;
            }
            offset = av_clip_c(
                offset,
                0 as ::core::ffi::c_int,
                (*p).buffer_end.offset_from((*p).buffer_start) as ::core::ffi::c_int,
            );
            (*p).buffer = (*p).buffer_start.offset(offset as isize);
        }
        _ => return -(22 as ::core::ffi::c_int),
    }
    return bytestream2_tell_p(p);
}
#[no_mangle]
pub unsafe extern "C" fn av_exif_get_tag_id(
    mut name: *const ::core::ffi::c_char,
) -> int32_t {
    if name.is_null() {
        return -(1 as int32_t);
    }
    let mut i: size_t = 0 as size_t;
    while (i as usize)
        < (::core::mem::size_of::<[exif_tag; 142]>() as usize)
            .wrapping_div(::core::mem::size_of::<exif_tag>() as usize)
    {
        if strcmp(
            &raw const (*(&raw const tag_list as *const exif_tag).offset(i as isize))
                .name as *const ::core::ffi::c_char,
            name,
        ) == 0
        {
            return tag_list[i as usize].id as int32_t;
        }
        i = i.wrapping_add(1);
    }
    return -(1 as int32_t);
}
#[inline]
unsafe extern "C" fn tput16(
    mut pb: *mut PutByteContext,
    le: ::core::ffi::c_int,
    value: uint16_t,
) {
    if le != 0 {
        bytestream2_put_le16(pb, value as ::core::ffi::c_uint);
    } else {
        bytestream2_put_be16(pb, value as ::core::ffi::c_uint);
    };
}
#[inline]
unsafe extern "C" fn tput32(
    mut pb: *mut PutByteContext,
    le: ::core::ffi::c_int,
    value: uint32_t,
) {
    if le != 0 {
        bytestream2_put_le32(pb, value as ::core::ffi::c_uint);
    } else {
        bytestream2_put_be32(pb, value as ::core::ffi::c_uint);
    };
}
#[inline]
unsafe extern "C" fn tput64(
    mut pb: *mut PutByteContext,
    le: ::core::ffi::c_int,
    value: uint64_t,
) {
    if le != 0 {
        bytestream2_put_le64(pb, value);
    } else {
        bytestream2_put_be64(pb, value);
    };
}
#[inline(always)]
unsafe extern "C" fn bytestream2_get_buffer(
    mut g: *mut GetByteContext,
    mut dst: *mut uint8_t,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut size2: ::core::ffi::c_uint = if (*g).buffer_end.offset_from((*g).buffer)
        as ::core::ffi::c_int as ::core::ffi::c_uint > size
    {
        size
    } else {
        (*g).buffer_end.offset_from((*g).buffer) as ::core::ffi::c_int
            as ::core::ffi::c_uint
    };
    memcpy(
        dst as *mut ::core::ffi::c_void,
        (*g).buffer as *const ::core::ffi::c_void,
        size2 as size_t,
    );
    (*g).buffer = (*g).buffer.offset(size2 as isize);
    return size2;
}
unsafe extern "C" fn exif_read_values(
    mut logctx: *mut ::core::ffi::c_void,
    mut gb: *mut GetByteContext,
    mut le: ::core::ffi::c_int,
    mut entry: *mut AVExifEntry,
) -> ::core::ffi::c_int {
    if exif_sizes[(*entry).type_0 as usize].wrapping_mul((*entry).count)
        > bytestream2_get_bytes_left(gb) as ::core::ffi::c_uint
    {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    match (*entry).type_0 as ::core::ffi::c_uint {
        3 | 4 => {
            (*entry).value.uint = av_calloc(
                (*entry).count as size_t,
                ::core::mem::size_of::<uint64_t>() as size_t,
            ) as *mut uint64_t;
        }
        8 | 9 => {
            (*entry).value.sint = av_calloc(
                (*entry).count as size_t,
                ::core::mem::size_of::<int64_t>() as size_t,
            ) as *mut int64_t;
        }
        12 | 11 => {
            (*entry).value.dbl = av_calloc(
                (*entry).count as size_t,
                ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
            ) as *mut ::core::ffi::c_double;
        }
        5 | 10 => {
            (*entry).value.rat = av_calloc(
                (*entry).count as size_t,
                ::core::mem::size_of::<AVRational>() as size_t,
            ) as *mut AVRational;
        }
        7 | 1 => {
            (*entry).value.ubytes = av_mallocz((*entry).count as size_t) as *mut uint8_t;
        }
        6 => {
            (*entry).value.sbytes = av_mallocz((*entry).count as size_t) as *mut int8_t;
        }
        2 => {
            (*entry).value.str_0 = av_mallocz(
                ((*entry).count as size_t).wrapping_add(1 as size_t),
            ) as *mut ::core::ffi::c_char;
        }
        13 => {
            av_log(
                logctx,
                24 as ::core::ffi::c_int,
                b"Bad IFD type for non-IFD tag\n\0" as *const u8
                    as *const ::core::ffi::c_char,
            );
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        _ => {}
    }
    if (*entry).value.ptr.is_null() {
        return -(12 as ::core::ffi::c_int);
    }
    match (*entry).type_0 as ::core::ffi::c_uint {
        3 => {
            let mut i: size_t = 0 as size_t;
            while i < (*entry).count {
                *(*entry).value.uint.offset(i as isize) = ff_tget_short(gb, le)
                    as uint64_t;
                i = i.wrapping_add(1);
            }
        }
        4 => {
            let mut i_0: size_t = 0 as size_t;
            while i_0 < (*entry).count {
                *(*entry).value.uint.offset(i_0 as isize) = ff_tget_long(gb, le)
                    as uint64_t;
                i_0 = i_0.wrapping_add(1);
            }
        }
        8 => {
            let mut i_1: size_t = 0 as size_t;
            while i_1 < (*entry).count {
                *(*entry).value.sint.offset(i_1 as isize) = ff_tget_short(gb, le)
                    as int16_t as int64_t;
                i_1 = i_1.wrapping_add(1);
            }
        }
        9 => {
            let mut i_2: size_t = 0 as size_t;
            while i_2 < (*entry).count {
                *(*entry).value.sint.offset(i_2 as isize) = ff_tget_long(gb, le)
                    as int32_t as int64_t;
                i_2 = i_2.wrapping_add(1);
            }
        }
        12 => {
            let mut i_3: size_t = 0 as size_t;
            while i_3 < (*entry).count {
                *(*entry).value.dbl.offset(i_3 as isize) = ff_tget_double(gb, le);
                i_3 = i_3.wrapping_add(1);
            }
        }
        11 => {
            let mut i_4: size_t = 0 as size_t;
            while i_4 < (*entry).count {
                let mut alias: av_alias32 = av_alias32 {
                    u32_0: ff_tget_long(gb, le),
                };
                *(*entry).value.dbl.offset(i_4 as isize) = alias.f32_0
                    as ::core::ffi::c_double;
                i_4 = i_4.wrapping_add(1);
            }
        }
        5 | 10 => {
            let mut i_5: size_t = 0 as size_t;
            while i_5 < (*entry).count {
                let mut num: int32_t = ff_tget_long(gb, le) as int32_t;
                let mut den: int32_t = ff_tget_long(gb, le) as int32_t;
                *(*entry).value.rat.offset(i_5 as isize) = av_make_q(
                    num as ::core::ffi::c_int,
                    den as ::core::ffi::c_int,
                );
                i_5 = i_5.wrapping_add(1);
            }
        }
        7 | 1 => {
            if (*entry).value.ubytes.is_null() {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    0 as ::core::ffi::c_int,
                    b"Assertion %s failed at %s:%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"entry->value.ubytes\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/libavcodec/exif.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    349 as ::core::ffi::c_int,
                );
                abort();
            }
            bytestream2_get_buffer(
                gb,
                (*entry).value.ubytes,
                (*entry).count as ::core::ffi::c_uint,
            );
        }
        6 => {
            if (*entry).value.sbytes.is_null() {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    0 as ::core::ffi::c_int,
                    b"Assertion %s failed at %s:%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"entry->value.sbytes\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/libavcodec/exif.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    353 as ::core::ffi::c_int,
                );
                abort();
            }
            bytestream2_get_buffer(
                gb,
                (*entry).value.sbytes as *mut uint8_t,
                (*entry).count as ::core::ffi::c_uint,
            );
        }
        2 => {
            if (*entry).value.str_0.is_null() {
                av_log(
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    0 as ::core::ffi::c_int,
                    b"Assertion %s failed at %s:%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    b"entry->value.str\0" as *const u8 as *const ::core::ffi::c_char,
                    b"src/libavcodec/exif.c\0" as *const u8
                        as *const ::core::ffi::c_char,
                    357 as ::core::ffi::c_int,
                );
                abort();
            }
            bytestream2_get_buffer(
                gb,
                (*entry).value.str_0 as *mut uint8_t,
                (*entry).count as ::core::ffi::c_uint,
            );
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[inline(always)]
unsafe extern "C" fn bytestream2_put_buffer(
    mut p: *mut PutByteContext,
    mut src: *const uint8_t,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_uint {
    let mut size2: ::core::ffi::c_uint = 0;
    if (*p).eof != 0 {
        return 0 as ::core::ffi::c_uint;
    }
    size2 = if (*p).buffer_end.offset_from((*p).buffer) as ::core::ffi::c_int
        as ::core::ffi::c_uint > size
    {
        size
    } else {
        (*p).buffer_end.offset_from((*p).buffer) as ::core::ffi::c_int
            as ::core::ffi::c_uint
    };
    if size2 != size {
        (*p).eof = 1 as ::core::ffi::c_int;
    }
    memcpy(
        (*p).buffer as *mut ::core::ffi::c_void,
        src as *const ::core::ffi::c_void,
        size2 as size_t,
    );
    (*p).buffer = (*p).buffer.offset(size2 as isize);
    return size2;
}
unsafe extern "C" fn exif_write_values(
    mut pb: *mut PutByteContext,
    mut le: ::core::ffi::c_int,
    mut entry: *const AVExifEntry,
) {
    match (*entry).type_0 as ::core::ffi::c_uint {
        3 => {
            let mut i: size_t = 0 as size_t;
            while i < (*entry).count {
                tput16(pb, le, *(*entry).value.uint.offset(i as isize) as uint16_t);
                i = i.wrapping_add(1);
            }
        }
        4 => {
            let mut i_0: size_t = 0 as size_t;
            while i_0 < (*entry).count {
                tput32(pb, le, *(*entry).value.uint.offset(i_0 as isize) as uint32_t);
                i_0 = i_0.wrapping_add(1);
            }
        }
        8 => {
            let mut i_1: size_t = 0 as size_t;
            while i_1 < (*entry).count {
                tput16(pb, le, *(*entry).value.sint.offset(i_1 as isize) as uint16_t);
                i_1 = i_1.wrapping_add(1);
            }
        }
        9 => {
            let mut i_2: size_t = 0 as size_t;
            while i_2 < (*entry).count {
                tput32(pb, le, *(*entry).value.sint.offset(i_2 as isize) as uint32_t);
                i_2 = i_2.wrapping_add(1);
            }
        }
        12 => {
            let mut i_3: size_t = 0 as size_t;
            while i_3 < (*entry).count {
                let a: av_alias64 = av_alias64 {
                    f64_0: *(*entry).value.dbl.offset(i_3 as isize),
                };
                tput64(pb, le, a.u64_0);
                i_3 = i_3.wrapping_add(1);
            }
        }
        11 => {
            let mut i_4: size_t = 0 as size_t;
            while i_4 < (*entry).count {
                let a_0: av_alias32 = av_alias32 {
                    f32_0: *(*entry).value.dbl.offset(i_4 as isize)
                        as ::core::ffi::c_float,
                };
                tput32(pb, le, a_0.u32_0);
                i_4 = i_4.wrapping_add(1);
            }
        }
        5 | 10 => {
            let mut i_5: size_t = 0 as size_t;
            while i_5 < (*entry).count {
                tput32(
                    pb,
                    le,
                    (*(*entry).value.rat.offset(i_5 as isize)).num as uint32_t,
                );
                tput32(
                    pb,
                    le,
                    (*(*entry).value.rat.offset(i_5 as isize)).den as uint32_t,
                );
                i_5 = i_5.wrapping_add(1);
            }
        }
        7 | 1 => {
            bytestream2_put_buffer(
                pb,
                (*entry).value.ubytes,
                (*entry).count as ::core::ffi::c_uint,
            );
        }
        6 => {
            bytestream2_put_buffer(
                pb,
                (*entry).value.sbytes as *const uint8_t,
                (*entry).count as ::core::ffi::c_uint,
            );
        }
        2 => {
            bytestream2_put_buffer(
                pb,
                (*entry).value.str_0 as *const uint8_t,
                (*entry).count as ::core::ffi::c_uint,
            );
        }
        _ => {}
    };
}
static mut aoc_header: [uint8_t; 4] = [
    'A' as i32 as uint8_t,
    'O' as i32 as uint8_t,
    'C' as i32 as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut casio_header: [uint8_t; 6] = [
    'Q' as i32 as uint8_t,
    'V' as i32 as uint8_t,
    'C' as i32 as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut foveon_header: [uint8_t; 8] = [
    'F' as i32 as uint8_t,
    'O' as i32 as uint8_t,
    'V' as i32 as uint8_t,
    'E' as i32 as uint8_t,
    'O' as i32 as uint8_t,
    'N' as i32 as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut fuji_header: [uint8_t; 4] = [
    'F' as i32 as uint8_t,
    'U' as i32 as uint8_t,
    'J' as i32 as uint8_t,
    'I' as i32 as uint8_t,
];
static mut nikon_header: [uint8_t; 6] = [
    'N' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    'k' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'n' as i32 as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut olympus1_header: [uint8_t; 6] = [
    'O' as i32 as uint8_t,
    'L' as i32 as uint8_t,
    'Y' as i32 as uint8_t,
    'M' as i32 as uint8_t,
    'P' as i32 as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut olympus2_header: [uint8_t; 10] = [
    'O' as i32 as uint8_t,
    'L' as i32 as uint8_t,
    'Y' as i32 as uint8_t,
    'M' as i32 as uint8_t,
    'P' as i32 as uint8_t,
    'U' as i32 as uint8_t,
    'S' as i32 as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    'I' as i32 as uint8_t,
    'I' as i32 as uint8_t,
];
static mut panasonic_header: [uint8_t; 12] = [
    'P' as i32 as uint8_t,
    'a' as i32 as uint8_t,
    'n' as i32 as uint8_t,
    'a' as i32 as uint8_t,
    's' as i32 as uint8_t,
    'o' as i32 as uint8_t,
    'n' as i32 as uint8_t,
    'i' as i32 as uint8_t,
    'c' as i32 as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut sigma_header: [uint8_t; 8] = [
    'S' as i32 as uint8_t,
    'I' as i32 as uint8_t,
    'G' as i32 as uint8_t,
    'M' as i32 as uint8_t,
    'A' as i32 as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut sony_header: [uint8_t; 12] = [
    'S' as i32 as uint8_t,
    'O' as i32 as uint8_t,
    'N' as i32 as uint8_t,
    'Y' as i32 as uint8_t,
    ' ' as i32 as uint8_t,
    'D' as i32 as uint8_t,
    'S' as i32 as uint8_t,
    'C' as i32 as uint8_t,
    ' ' as i32 as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut makernote_data: [exif_makernote_data; 9] = unsafe {
    [
        exif_makernote_data {
            header: &raw const aoc_header as *const uint8_t,
            header_size: ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
            result: 6 as ::core::ffi::c_int,
        },
        exif_makernote_data {
            header: &raw const casio_header as *const uint8_t,
            header_size: ::core::mem::size_of::<[uint8_t; 6]>() as size_t,
            result: -(1 as ::core::ffi::c_int),
        },
        exif_makernote_data {
            header: &raw const foveon_header as *const uint8_t,
            header_size: ::core::mem::size_of::<[uint8_t; 8]>() as size_t,
            result: 10 as ::core::ffi::c_int,
        },
        exif_makernote_data {
            header: &raw const fuji_header as *const uint8_t,
            header_size: ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
            result: -(1 as ::core::ffi::c_int),
        },
        exif_makernote_data {
            header: &raw const olympus1_header as *const uint8_t,
            header_size: ::core::mem::size_of::<[uint8_t; 6]>() as size_t,
            result: 8 as ::core::ffi::c_int,
        },
        exif_makernote_data {
            header: &raw const olympus2_header as *const uint8_t,
            header_size: ::core::mem::size_of::<[uint8_t; 10]>() as size_t,
            result: -(1 as ::core::ffi::c_int),
        },
        exif_makernote_data {
            header: &raw const panasonic_header as *const uint8_t,
            header_size: ::core::mem::size_of::<[uint8_t; 12]>() as size_t,
            result: 12 as ::core::ffi::c_int,
        },
        exif_makernote_data {
            header: &raw const sigma_header as *const uint8_t,
            header_size: ::core::mem::size_of::<[uint8_t; 8]>() as size_t,
            result: 10 as ::core::ffi::c_int,
        },
        exif_makernote_data {
            header: &raw const sony_header as *const uint8_t,
            header_size: ::core::mem::size_of::<[uint8_t; 12]>() as size_t,
            result: 12 as ::core::ffi::c_int,
        },
    ]
};
unsafe extern "C" fn exif_get_makernote_offset(
    mut gb: *mut GetByteContext,
) -> ::core::ffi::c_int {
    if bytestream2_get_bytes_left(gb) < 12 as ::core::ffi::c_int {
        return -(1 as ::core::ffi::c_int);
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i as usize)
        < (::core::mem::size_of::<[exif_makernote_data; 9]>() as usize)
            .wrapping_div(::core::mem::size_of::<exif_makernote_data>() as usize)
    {
        if memcmp(
            (*gb).buffer as *const ::core::ffi::c_void,
            makernote_data[i as usize].header as *const ::core::ffi::c_void,
            makernote_data[i as usize].header_size,
        ) == 0
        {
            return makernote_data[i as usize].result;
        }
        i += 1;
    }
    if memcmp(
        (*gb).buffer as *const ::core::ffi::c_void,
        &raw const nikon_header as *const uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 6]>() as size_t,
    ) == 0
    {
        if bytestream2_get_bytes_left(gb) < 14 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int)
        } else if av_bswap32(
            (*((*gb).buffer.offset(10 as ::core::ffi::c_int as isize)
                as *const unaligned_32))
                .l,
        ) == 0x4d4d002a as ::core::ffi::c_int as uint32_t
            || av_bswap32(
                (*((*gb).buffer.offset(10 as ::core::ffi::c_int as isize)
                    as *const unaligned_32))
                    .l,
            ) == 0x49492a00 as ::core::ffi::c_int as uint32_t
        {
            return -(1 as ::core::ffi::c_int)
        }
        return 8 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn exif_decode_tag(
    mut logctx: *mut ::core::ffi::c_void,
    mut gb: *mut GetByteContext,
    mut le: ::core::ffi::c_int,
    mut depth: ::core::ffi::c_int,
    mut entry: *mut AVExifEntry,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut makernote_offset: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut tell: ::core::ffi::c_int = 0;
    let mut is_ifd: ::core::ffi::c_int = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut type_0: AVTiffDataType = 0 as AVTiffDataType;
    let mut payload: uint32_t = 0;
    if depth > 3 as ::core::ffi::c_int {
        return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    }
    tell = bytestream2_tell(gb);
    (*entry).id = ff_tget_short(gb, le) as uint16_t;
    type_0 = ff_tget_short(gb, le) as AVTiffDataType;
    count = ff_tget_long(gb, le) as ::core::ffi::c_int;
    payload = ff_tget_long(gb, le) as uint32_t;
    av_log(
        logctx,
        48 as ::core::ffi::c_int,
        b"TIFF Tag: id: 0x%04x, type: %d, count: %u, offset: %d, payload: %u\n\0"
            as *const u8 as *const ::core::ffi::c_char,
        (*entry).id as ::core::ffi::c_int,
        type_0 as ::core::ffi::c_uint,
        count,
        tell,
        payload,
    );
    if type_0 as u64 == 0 {
        av_log(
            logctx,
            48 as ::core::ffi::c_int,
            b"Skipping invalid TIFF tag 0\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
    } else {
        if type_0 as ::core::ffi::c_uint
            > AV_TIFF_IFD as ::core::ffi::c_int as ::core::ffi::c_uint
            || count as ::core::ffi::c_uint
                >= (2147483647 as ::core::ffi::c_int as ::core::ffi::c_uint)
                    .wrapping_div(8 as ::core::ffi::c_uint)
        {
            return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        }
        is_ifd = (type_0 as ::core::ffi::c_uint
            == AV_TIFF_IFD as ::core::ffi::c_int as ::core::ffi::c_uint
            || ff_tis_ifd((*entry).id as ::core::ffi::c_uint) != 0
            || (*entry).id as ::core::ffi::c_int == 0x927c as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        if is_ifd != 0 {
            if payload == 0 {
                current_block = 7786694203366505283;
            } else {
                bytestream2_seek(
                    gb,
                    payload as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                );
                current_block = 11050875288958768710;
            }
        } else {
            current_block = 11050875288958768710;
        }
        match current_block {
            7786694203366505283 => {}
            _ => {
                if (*entry).id as ::core::ffi::c_int == 0x927c as ::core::ffi::c_int {
                    makernote_offset = exif_get_makernote_offset(gb);
                    if makernote_offset < 0 as ::core::ffi::c_int {
                        is_ifd = 0 as ::core::ffi::c_int;
                    }
                }
                if is_ifd != 0 {
                    (*entry).type_0 = AV_TIFF_IFD;
                    (*entry).count = 1 as uint32_t;
                    (*entry).ifd_offset = (if makernote_offset > 0 as ::core::ffi::c_int
                    {
                        makernote_offset
                    } else {
                        0 as ::core::ffi::c_int
                    }) as uint32_t;
                    if (*entry).ifd_offset != 0 {
                        (*entry).ifd_lead = av_malloc((*entry).ifd_offset as size_t)
                            as *mut uint8_t;
                        if (*entry).ifd_lead.is_null() {
                            return -(12 as ::core::ffi::c_int);
                        }
                        bytestream2_get_buffer(
                            gb,
                            (*entry).ifd_lead,
                            (*entry).ifd_offset as ::core::ffi::c_uint,
                        );
                    }
                    ret = exif_parse_ifd_list(
                        logctx,
                        gb,
                        le,
                        depth + 1 as ::core::ffi::c_int,
                        &raw mut (*entry).value.ifd,
                        ((*entry).id as ::core::ffi::c_int
                            == 0x927c as ::core::ffi::c_int) as ::core::ffi::c_int,
                    );
                    if ret < 0 as ::core::ffi::c_int
                        && (*entry).id as ::core::ffi::c_int
                            == 0x927c as ::core::ffi::c_int
                    {
                        av_log(
                            logctx,
                            48 as ::core::ffi::c_int,
                            b"unrecognized MakerNote IFD, retrying as blob\n\0"
                                as *const u8 as *const ::core::ffi::c_char,
                        );
                        is_ifd = 0 as ::core::ffi::c_int;
                    }
                }
                if is_ifd == 0 {
                    (*entry).type_0 = (if type_0 as ::core::ffi::c_uint
                        == AV_TIFF_IFD as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        AV_TIFF_UNDEFINED as ::core::ffi::c_int as ::core::ffi::c_uint
                    } else {
                        type_0 as ::core::ffi::c_uint
                    }) as AVTiffDataType;
                    (*entry).count = count as uint32_t;
                    bytestream2_seek(
                        gb,
                        (if (count as size_t).wrapping_mul(exif_sizes[type_0 as usize])
                            > 4 as size_t
                        {
                            payload
                        } else {
                            (tell + 8 as ::core::ffi::c_int) as uint32_t
                        }) as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                    ret = exif_read_values(logctx, gb, le, entry);
                }
            }
        }
    }
    bytestream2_seek(gb, tell + 12 as ::core::ffi::c_int, 0 as ::core::ffi::c_int);
    return ret;
}
unsafe extern "C" fn exif_parse_ifd_list(
    mut logctx: *mut ::core::ffi::c_void,
    mut gb: *mut GetByteContext,
    mut le: ::core::ffi::c_int,
    mut depth: ::core::ffi::c_int,
    mut ifd: *mut AVExifMetadata,
    mut guess: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut entries: uint32_t = 0;
    let mut required_size: size_t = 0;
    let mut temp: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    av_log(
        logctx,
        48 as ::core::ffi::c_int,
        b"parsing IFD list at offset: %d\n\0" as *const u8 as *const ::core::ffi::c_char,
        bytestream2_tell(gb),
    );
    if bytestream2_get_bytes_left(gb) < 2 as ::core::ffi::c_int {
        av_log(
            logctx,
            if guess != 0 { 48 as ::core::ffi::c_int } else { 16 as ::core::ffi::c_int },
            b"not enough bytes remaining in EXIF buffer: 2 required\n\0" as *const u8
                as *const ::core::ffi::c_char,
        );
        ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
    } else {
        entries = ff_tget_short(gb, le) as uint32_t;
        if (bytestream2_get_bytes_left(gb) as uint32_t)
            < entries.wrapping_mul(12 as uint32_t)
        {
            av_log(
                logctx,
                if guess != 0 {
                    48 as ::core::ffi::c_int
                } else {
                    16 as ::core::ffi::c_int
                },
                b"not enough bytes remaining in EXIF buffer. entries: %u\n\0"
                    as *const u8 as *const ::core::ffi::c_char,
                entries,
            );
            ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        } else if entries > 4096 as uint32_t {
            av_log(
                logctx,
                if guess != 0 {
                    48 as ::core::ffi::c_int
                } else {
                    16 as ::core::ffi::c_int
                },
                b"too many entries: %u\n\0" as *const u8 as *const ::core::ffi::c_char,
                entries,
            );
            ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                as ::core::ffi::c_int);
        } else {
            (*ifd).count = entries as ::core::ffi::c_uint;
            av_log(
                logctx,
                48 as ::core::ffi::c_int,
                b"entry count for IFD: %u\n\0" as *const u8
                    as *const ::core::ffi::c_char,
                (*ifd).count,
            );
            if (*ifd).count == 0 {
                ret = 0 as ::core::ffi::c_int;
            } else if av_size_mult(
                (*ifd).count as size_t,
                ::core::mem::size_of::<AVExifEntry>() as size_t,
                &raw mut required_size,
            ) < 0 as ::core::ffi::c_int
            {
                ret = -(12 as ::core::ffi::c_int);
            } else {
                temp = av_fast_realloc(
                    (*ifd).entries as *mut ::core::ffi::c_void,
                    &raw mut (*ifd).size,
                    required_size as size_t,
                );
                if temp.is_null() {
                    av_freep(&raw mut (*ifd).entries as *mut ::core::ffi::c_void);
                    ret = -(12 as ::core::ffi::c_int);
                } else {
                    (*ifd).entries = temp as *mut AVExifEntry;
                    memset(
                        (*ifd).entries as *mut ::core::ffi::c_void,
                        0 as ::core::ffi::c_int,
                        required_size,
                    );
                    let mut i: uint32_t = 0 as uint32_t;
                    while i < entries {
                        ret = exif_decode_tag(
                            logctx,
                            gb,
                            le,
                            depth,
                            (*ifd).entries.offset(i as isize) as *mut AVExifEntry,
                        );
                        if ret < 0 as ::core::ffi::c_int {
                            break;
                        }
                        i = i.wrapping_add(1);
                    }
                }
            }
        }
    }
    if ret < 0 as ::core::ffi::c_int {
        av_exif_free(ifd);
        return ret;
    }
    ret = ff_tget_long(gb, le) as ::core::ffi::c_int;
    if ret < 0 as ::core::ffi::c_int {
        ret = -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
            | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
            as ::core::ffi::c_int);
        av_exif_free(ifd);
    }
    return ret;
}
unsafe extern "C" fn exif_free_entry(mut entry: *mut AVExifEntry) {
    if entry.is_null() {
        return;
    }
    if (*entry).type_0 as ::core::ffi::c_uint
        == AV_TIFF_IFD as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        av_exif_free(&raw mut (*entry).value.ifd);
    } else {
        av_freep(&raw mut (*entry).value.ptr as *mut ::core::ffi::c_void);
    }
    av_freep(&raw mut (*entry).ifd_lead as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn av_exif_free(mut ifd: *mut AVExifMetadata) {
    if ifd.is_null() {
        return;
    }
    if (*ifd).entries.is_null() {
        (*ifd).count = 0 as ::core::ffi::c_uint;
        (*ifd).size = 0 as ::core::ffi::c_uint;
        return;
    }
    let mut i: size_t = 0 as size_t;
    while i < (*ifd).count {
        let mut entry: *mut AVExifEntry = (*ifd).entries.offset(i as isize)
            as *mut AVExifEntry;
        exif_free_entry(entry);
        i = i.wrapping_add(1);
    }
    av_freep(&raw mut (*ifd).entries as *mut ::core::ffi::c_void);
    (*ifd).count = 0 as ::core::ffi::c_uint;
    (*ifd).size = 0 as ::core::ffi::c_uint;
}
unsafe extern "C" fn exif_get_ifd_size(mut ifd: *const AVExifMetadata) -> size_t {
    let mut total_size: size_t = 6 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < (*ifd).count {
        let mut entry: *const AVExifEntry = (*ifd).entries.offset(i as isize)
            as *mut AVExifEntry;
        if (*entry).type_0 as ::core::ffi::c_uint
            == AV_TIFF_IFD as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            total_size = (total_size as ::core::ffi::c_uint)
                .wrapping_add(
                    (12 as uint32_t)
                        .wrapping_add(
                            exif_get_ifd_size(&raw const (*entry).value.ifd) as uint32_t,
                        )
                        .wrapping_add((*entry).ifd_offset) as ::core::ffi::c_uint,
                ) as size_t as size_t;
        } else {
            let mut payload_size: size_t = ((*entry).count as size_t)
                .wrapping_mul(exif_sizes[(*entry).type_0 as usize]);
            total_size = total_size
                .wrapping_add(
                    (12 as size_t)
                        .wrapping_add(
                            (if payload_size > 4 as size_t {
                                payload_size
                            } else {
                                0 as size_t
                            }),
                        ),
                );
        }
        i = i.wrapping_add(1);
    }
    return total_size;
}
unsafe extern "C" fn exif_write_ifd(
    mut logctx: *mut ::core::ffi::c_void,
    mut pb: *mut PutByteContext,
    mut le: ::core::ffi::c_int,
    mut depth: ::core::ffi::c_int,
    mut ifd: *const AVExifMetadata,
) -> ::core::ffi::c_int {
    let mut offset: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut tell: ::core::ffi::c_int = 0;
    let mut tell2: ::core::ffi::c_int = 0;
    tell = bytestream2_tell_p(pb);
    tput16(pb, le, (*ifd).count as uint16_t);
    offset = ((tell + 6 as ::core::ffi::c_int) as uint32_t)
        .wrapping_add((12 as uint32_t).wrapping_mul((*ifd).count)) as ::core::ffi::c_int;
    av_log(
        logctx,
        48 as ::core::ffi::c_int,
        b"writing IFD with %u entries and initial offset %d\n\0" as *const u8
            as *const ::core::ffi::c_char,
        (*ifd).count,
        offset,
    );
    let mut i: size_t = 0 as size_t;
    while i < (*ifd).count {
        let mut entry: *const AVExifEntry = (*ifd).entries.offset(i as isize)
            as *mut AVExifEntry;
        av_log(
            logctx,
            48 as ::core::ffi::c_int,
            b"writing TIFF entry: id: 0x%04x, type: %d, count: %u, offset: %d, offset value: %d\n\0"
                as *const u8 as *const ::core::ffi::c_char,
            (*entry).id as ::core::ffi::c_int,
            (*entry).type_0 as ::core::ffi::c_uint,
            (*entry).count,
            bytestream2_tell_p(pb),
            offset,
        );
        tput16(pb, le, (*entry).id);
        if (*entry).id as ::core::ffi::c_int == 0x927c as ::core::ffi::c_int
            && (*entry).type_0 as ::core::ffi::c_uint
                == AV_TIFF_IFD as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut ifd_size: size_t = exif_get_ifd_size(&raw const (*entry).value.ifd);
            tput16(pb, le, AV_TIFF_UNDEFINED as ::core::ffi::c_int as uint16_t);
            tput32(pb, le, ifd_size as uint32_t);
        } else {
            tput16(pb, le, (*entry).type_0 as uint16_t);
            tput32(pb, le, (*entry).count);
        }
        if (*entry).type_0 as ::core::ffi::c_uint
            == AV_TIFF_IFD as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            tput32(pb, le, offset as uint32_t);
            tell2 = bytestream2_tell_p(pb);
            bytestream2_seek_p(pb, offset, 0 as ::core::ffi::c_int);
            if (*entry).ifd_offset != 0 {
                bytestream2_put_buffer(
                    pb,
                    (*entry).ifd_lead,
                    (*entry).ifd_offset as ::core::ffi::c_uint,
                );
            }
            ret = exif_write_ifd(
                logctx,
                pb,
                le,
                depth + 1 as ::core::ffi::c_int,
                &raw const (*entry).value.ifd,
            );
            if ret < 0 as ::core::ffi::c_int {
                return ret;
            }
            offset = (offset as uint32_t)
                .wrapping_add((ret as uint32_t).wrapping_add((*entry).ifd_offset))
                as ::core::ffi::c_int as ::core::ffi::c_int;
            bytestream2_seek_p(pb, tell2, 0 as ::core::ffi::c_int);
        } else {
            let mut payload_size: size_t = ((*entry).count as size_t)
                .wrapping_mul(exif_sizes[(*entry).type_0 as usize]);
            if payload_size > 4 as size_t {
                tput32(pb, le, offset as uint32_t);
                tell2 = bytestream2_tell_p(pb);
                bytestream2_seek_p(pb, offset, 0 as ::core::ffi::c_int);
                exif_write_values(pb, le, entry);
                offset = (offset as size_t).wrapping_add(payload_size)
                    as ::core::ffi::c_int as ::core::ffi::c_int;
                bytestream2_seek_p(pb, tell2, 0 as ::core::ffi::c_int);
            } else {
                (*((*pb).buffer as *mut unaligned_32)).l = 0 as uint32_t;
                exif_write_values(pb, le, entry);
                bytestream2_seek_p(
                    pb,
                    (4 as size_t).wrapping_sub(payload_size) as ::core::ffi::c_int,
                    1 as ::core::ffi::c_int,
                );
            }
        }
        i = i.wrapping_add(1);
    }
    tput32(
        pb,
        le,
        (if depth != 0 { offset } else { 0 as ::core::ffi::c_int }) as uint32_t,
    );
    return offset - tell;
}
#[no_mangle]
pub unsafe extern "C" fn av_exif_write(
    mut logctx: *mut ::core::ffi::c_void,
    mut ifd: *const AVExifMetadata,
    mut buffer: *mut *mut AVBufferRef,
    mut header_mode: AVExifHeaderMode,
) -> ::core::ffi::c_int {
    let mut extras: ::core::ffi::c_int = 0;
    let mut current_block: u64;
    let mut buf: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut size: size_t = 0;
    let mut headsize: size_t = 8 as size_t;
    let mut pb: PutByteContext = PutByteContext {
        buffer: ::core::ptr::null_mut::<uint8_t>(),
        buffer_end: ::core::ptr::null_mut::<uint8_t>(),
        buffer_start: ::core::ptr::null_mut::<uint8_t>(),
        eof: 0,
    };
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut off: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut next: ::core::ffi::c_int = 0;
    let mut ifd_new: *mut AVExifMetadata = ::core::ptr::null_mut::<AVExifMetadata>();
    let mut extra_ifds: [AVExifMetadata; 16] = [
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
        AVExifMetadata {
            entries: ::core::ptr::null_mut::<AVExifEntry>(),
            count: 0,
            size: 0,
        },
    ];
    let mut le: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if !(*buffer).is_null() {
        ret = -(22 as ::core::ffi::c_int);
    } else {
        size = exif_get_ifd_size(ifd);
        match header_mode as ::core::ffi::c_uint {
            4 => {
                off = 6 as ::core::ffi::c_int;
            }
            3 => {
                off = 4 as ::core::ffi::c_int;
            }
            2 => {
                le = 0 as ::core::ffi::c_int;
                headsize = 0 as size_t;
            }
            1 => {
                le = 1 as ::core::ffi::c_int;
                headsize = 0 as size_t;
            }
            _ => {}
        }
        ret = av_buffer_realloc(
            &raw mut buf,
            size.wrapping_add(off as size_t).wrapping_add(headsize),
        );
        if !(ret < 0 as ::core::ffi::c_int) {
            if header_mode as ::core::ffi::c_uint
                == AV_EXIF_EXIF00 as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*((*buf).data as *mut unaligned_32)).l = (('E' as i32
                    | ('x' as i32) << 8 as ::core::ffi::c_int
                    | ('i' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('f' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as uint32_t;
                (*((*buf).data.offset(4 as ::core::ffi::c_int as isize)
                    as *mut unaligned_16))
                    .l = 0 as uint16_t;
            } else if header_mode as ::core::ffi::c_uint
                == AV_EXIF_T_OFF as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*((*buf).data as *mut unaligned_32)).l = 0 as uint32_t;
            }
            bytestream2_init_writer(
                &raw mut pb,
                (*buf).data.offset(off as isize),
                (*buf).size.wrapping_sub(off as size_t) as ::core::ffi::c_int,
            );
            if header_mode as ::core::ffi::c_uint
                != AV_EXIF_ASSUME_BE as ::core::ffi::c_int as ::core::ffi::c_uint
                && header_mode as ::core::ffi::c_uint
                    != AV_EXIF_ASSUME_LE as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                bytestream2_put_be32(
                    &raw mut pb,
                    0x49492a00 as ::core::ffi::c_int as ::core::ffi::c_uint,
                );
                tput32(&raw mut pb, le, 8 as uint32_t);
            }
            extras = 0 as ::core::ffi::c_int;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while (i as usize)
                < (::core::mem::size_of::<[AVExifMetadata; 16]>() as usize)
                    .wrapping_div(::core::mem::size_of::<AVExifMetadata>() as usize)
            {
                let mut extra_entry: *mut AVExifEntry = ::core::ptr::null_mut::<
                    AVExifEntry,
                >();
                let mut extra_tag: uint16_t = (0xfffc as ::core::ffi::c_uint)
                    .wrapping_sub(i as ::core::ffi::c_uint) as uint16_t;
                ret = av_exif_get_entry(
                    logctx,
                    ifd as *mut AVExifMetadata,
                    extra_tag,
                    0 as ::core::ffi::c_int,
                    &raw mut extra_entry,
                );
                if ret < 0 as ::core::ffi::c_int {
                    break;
                }
                if !(ret == 0) {
                    av_log(
                        logctx,
                        48 as ::core::ffi::c_int,
                        b"found extra IFD tag: %04x\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        extra_tag as ::core::ffi::c_int,
                    );
                    if ifd_new.is_null() {
                        ifd_new = av_exif_clone_ifd(ifd);
                        if ifd_new.is_null() {
                            break;
                        }
                        ifd = ifd_new;
                    }
                    let mut cloned: *mut AVExifMetadata = av_exif_clone_ifd(
                        &raw mut (*extra_entry).value.ifd,
                    );
                    if cloned.is_null() {
                        break;
                    }
                    let fresh3 = extras;
                    extras = extras + 1;
                    extra_ifds[fresh3 as usize] = *cloned;
                    av_free(cloned as *mut ::core::ffi::c_void);
                    ret = av_exif_remove_entry(
                        logctx,
                        ifd_new,
                        extra_tag,
                        0 as ::core::ffi::c_int,
                    );
                    if ret < 0 as ::core::ffi::c_int {
                        break;
                    }
                }
                i += 1;
            }
            if ret < 0 as ::core::ffi::c_int {
                let mut fresh4: [::core::ffi::c_char; 64] = [
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
                    logctx,
                    16 as ::core::ffi::c_int,
                    b"error popping additional IFD: %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    av_make_error_string(
                        &raw mut fresh4 as *mut ::core::ffi::c_char,
                        64 as size_t,
                        ret,
                    ),
                );
            } else {
                next = bytestream2_tell_p(&raw mut pb);
                ret = exif_write_ifd(
                    logctx,
                    &raw mut pb,
                    le,
                    0 as ::core::ffi::c_int,
                    ifd,
                );
                if ret < 0 as ::core::ffi::c_int {
                    let mut fresh5: [::core::ffi::c_char; 64] = [
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
                        logctx,
                        16 as ::core::ffi::c_int,
                        b"error writing EXIF data: %s\n\0" as *const u8
                            as *const ::core::ffi::c_char,
                        av_make_error_string(
                            &raw mut fresh5 as *mut ::core::ffi::c_char,
                            64 as size_t,
                            ret,
                        ),
                    );
                } else {
                    next += ret;
                    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    loop {
                        if !(i_0 < extras) {
                            current_block = 5892776923941496671;
                            break;
                        }
                        av_log(
                            logctx,
                            48 as ::core::ffi::c_int,
                            b"writing additional ifd at: %d\n\0" as *const u8
                                as *const ::core::ffi::c_char,
                            next,
                        );
                        bytestream2_seek_p(
                            &raw mut pb,
                            -(4 as ::core::ffi::c_int),
                            1 as ::core::ffi::c_int,
                        );
                        tput32(&raw mut pb, le, next as uint32_t);
                        bytestream2_seek_p(&raw mut pb, next, 0 as ::core::ffi::c_int);
                        ret = exif_write_ifd(
                            logctx,
                            &raw mut pb,
                            le,
                            0 as ::core::ffi::c_int,
                            (&raw mut extra_ifds as *mut AVExifMetadata)
                                .offset(i_0 as isize) as *mut AVExifMetadata,
                        );
                        if ret < 0 as ::core::ffi::c_int {
                            let mut fresh6: [::core::ffi::c_char; 64] = [
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
                                logctx,
                                16 as ::core::ffi::c_int,
                                b"error writing additional IFD: %s\n\0" as *const u8
                                    as *const ::core::ffi::c_char,
                                av_make_error_string(
                                    &raw mut fresh6 as *mut ::core::ffi::c_char,
                                    64 as size_t,
                                    ret,
                                ),
                            );
                            current_block = 14220213708459872294;
                            break;
                        } else {
                            next += ret;
                            i_0 += 1;
                        }
                    }
                    match current_block {
                        14220213708459872294 => {}
                        _ => {
                            ret = av_buffer_realloc(
                                &raw mut buf,
                                (*buf)
                                    .size
                                    .wrapping_sub((12 as ::core::ffi::c_int * extras) as size_t),
                            );
                            if !(ret < 0 as ::core::ffi::c_int) {
                                *buffer = buf;
                                ret = 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    av_exif_free(ifd_new);
    av_freep(&raw mut ifd_new as *mut ::core::ffi::c_void);
    let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i_1 as usize)
        < (::core::mem::size_of::<[AVExifMetadata; 16]>() as usize)
            .wrapping_div(::core::mem::size_of::<AVExifMetadata>() as usize)
    {
        av_exif_free(
            (&raw mut extra_ifds as *mut AVExifMetadata).offset(i_1 as isize)
                as *mut AVExifMetadata,
        );
        i_1 += 1;
    }
    if ret < 0 as ::core::ffi::c_int {
        av_buffer_unref(&raw mut buf);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_exif_parse_buffer(
    mut logctx: *mut ::core::ffi::c_void,
    mut buf: *const uint8_t,
    mut size: size_t,
    mut ifd: *mut AVExifMetadata,
    mut header_mode: AVExifHeaderMode,
) -> ::core::ffi::c_int {
    let mut next: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut le: ::core::ffi::c_int = 0;
    let mut gbytes: GetByteContext = GetByteContext {
        buffer: ::core::ptr::null::<uint8_t>(),
        buffer_end: ::core::ptr::null::<uint8_t>(),
        buffer_start: ::core::ptr::null::<uint8_t>(),
    };
    if size > 2147483647 as ::core::ffi::c_int as size_t {
        return -(22 as ::core::ffi::c_int);
    }
    let mut off: size_t = 0 as size_t;
    let mut current_block_24: u64;
    match header_mode as ::core::ffi::c_uint {
        4 => {
            if size < 6 as size_t {
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            off = 6 as size_t;
            current_block_24 = 6782506343435084693;
        }
        3 => {
            current_block_24 = 6782506343435084693;
        }
        0 => {
            current_block_24 = 1917311967535052937;
        }
        1 => {
            le = 1 as ::core::ffi::c_int;
            bytestream2_init(&raw mut gbytes, buf, size as ::core::ffi::c_int);
            current_block_24 = 11042950489265723346;
        }
        2 => {
            le = 0 as ::core::ffi::c_int;
            bytestream2_init(&raw mut gbytes, buf, size as ::core::ffi::c_int);
            current_block_24 = 11042950489265723346;
        }
        _ => return -(22 as ::core::ffi::c_int),
    }
    match current_block_24 {
        6782506343435084693 => {
            if size < 4 as size_t {
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            if off == 0 {
                off = av_bswap32((*(buf as *const unaligned_32)).l)
                    .wrapping_add(4 as uint32_t) as size_t;
            }
            current_block_24 = 1917311967535052937;
        }
        _ => {}
    }
    match current_block_24 {
        1917311967535052937 => {
            let mut ifd_offset: ::core::ffi::c_int = 0;
            if size <= off {
                return -((('I' as i32 | ('N' as i32) << 8 as ::core::ffi::c_int
                    | ('D' as i32) << 16 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    | ('A' as i32 as ::core::ffi::c_uint) << 24 as ::core::ffi::c_int)
                    as ::core::ffi::c_int);
            }
            bytestream2_init(
                &raw mut gbytes,
                buf.offset(off as isize),
                size.wrapping_sub(off) as ::core::ffi::c_int,
            );
            ret = ff_tdecode_header(&raw mut gbytes, &raw mut le, &raw mut ifd_offset);
            if ret < 0 as ::core::ffi::c_int {
                let mut fresh1: [::core::ffi::c_char; 64] = [
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
                    logctx,
                    16 as ::core::ffi::c_int,
                    b"invalid TIFF header in EXIF data: %s\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    av_make_error_string(
                        &raw mut fresh1 as *mut ::core::ffi::c_char,
                        64 as size_t,
                        ret,
                    ),
                );
                return ret;
            }
            bytestream2_seek(&raw mut gbytes, ifd_offset, 0 as ::core::ffi::c_int);
        }
        _ => {}
    }
    ret = exif_parse_ifd_list(
        logctx,
        &raw mut gbytes,
        le,
        0 as ::core::ffi::c_int,
        ifd,
        0 as ::core::ffi::c_int,
    );
    if ret < 0 as ::core::ffi::c_int {
        let mut fresh2: [::core::ffi::c_char; 64] = [
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
            logctx,
            16 as ::core::ffi::c_int,
            b"error decoding EXIF data: %s\n\0" as *const u8
                as *const ::core::ffi::c_char,
            av_make_error_string(
                &raw mut fresh2 as *mut ::core::ffi::c_char,
                64 as size_t,
                ret,
            ),
        );
        return ret;
    }
    if !(ret == 0) {
        next = ret;
        bytestream2_seek(&raw mut gbytes, next, 0 as ::core::ffi::c_int);
        let mut extra_tag: ::core::ffi::c_int = 0xfffc as ::core::ffi::c_uint
            as ::core::ffi::c_int;
        while extra_tag as ::core::ffi::c_uint > 0xffec as ::core::ffi::c_uint {
            let mut extra_ifd: AVExifMetadata = AVExifMetadata {
                entries: ::core::ptr::null_mut::<AVExifEntry>(),
                count: 0,
                size: 0,
            };
            ret = exif_parse_ifd_list(
                logctx,
                &raw mut gbytes,
                le,
                0 as ::core::ffi::c_int,
                &raw mut extra_ifd,
                1 as ::core::ffi::c_int,
            );
            if ret < 0 as ::core::ffi::c_int {
                av_exif_free(&raw mut extra_ifd);
                break;
            } else {
                next = ret;
                av_log(
                    logctx,
                    48 as ::core::ffi::c_int,
                    b"found extra IFD: %04x with next=%d\n\0" as *const u8
                        as *const ::core::ffi::c_char,
                    extra_tag,
                    ret,
                );
                bytestream2_seek(&raw mut gbytes, next, 0 as ::core::ffi::c_int);
                ret = av_exif_set_entry(
                    logctx,
                    ifd,
                    extra_tag as uint16_t,
                    AV_TIFF_IFD,
                    1 as uint32_t,
                    ::core::ptr::null::<uint8_t>(),
                    0 as uint32_t,
                    &raw mut extra_ifd as *const ::core::ffi::c_void,
                );
                av_exif_free(&raw mut extra_ifd);
                if ret < 0 as ::core::ffi::c_int || next == 0
                    || bytestream2_get_bytes_left(&raw mut gbytes)
                        <= 0 as ::core::ffi::c_int
                {
                    break;
                }
                extra_tag -= 1;
            }
        }
    }
    return (bytestream2_tell(&raw mut gbytes) as size_t).wrapping_add(off)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn exif_ifd_to_dict(
    mut logctx: *mut ::core::ffi::c_void,
    mut prefix: *const ::core::ffi::c_char,
    mut ifd: *const AVExifMetadata,
    mut metadata: *mut *mut AVDictionary,
) -> ::core::ffi::c_int {
    let mut bp: AVBPrint = AVBPrint {
        str_0: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        len: 0,
        size: 0,
        size_max: 0,
        reserved_internal_buffer: [0; 1],
        reserved_padding: [0; 1004],
    };
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut key: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    let mut value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<
        ::core::ffi::c_char,
    >();
    if prefix.is_null() {
        prefix = b"\0" as *const u8 as *const ::core::ffi::c_char;
    }
    let mut i: uint16_t = 0 as uint16_t;
    while (i as ::core::ffi::c_uint) < (*ifd).count {
        let mut entry: *const AVExifEntry = (*ifd).entries.offset(i as isize)
            as *mut AVExifEntry;
        let mut name: *const ::core::ffi::c_char = av_exif_get_tag_name((*entry).id);
        av_bprint_init(
            &raw mut bp,
            ((*entry).count as ::core::ffi::c_uint)
                .wrapping_mul(10 as ::core::ffi::c_uint),
            -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
        if *prefix != 0 {
            av_bprintf(
                &raw mut bp,
                b"%s/\0" as *const u8 as *const ::core::ffi::c_char,
                prefix,
            );
        }
        if !name.is_null() {
            av_bprintf(
                &raw mut bp,
                b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                name,
            );
        } else {
            av_bprintf(
                &raw mut bp,
                b"0x%04X\0" as *const u8 as *const ::core::ffi::c_char,
                (*entry).id as ::core::ffi::c_int,
            );
        }
        ret = av_bprint_finalize(&raw mut bp, &raw mut key);
        if ret < 0 as ::core::ffi::c_int {
            break;
        }
        av_bprint_init(
            &raw mut bp,
            ((*entry).count as ::core::ffi::c_uint)
                .wrapping_mul(10 as ::core::ffi::c_uint),
            -(1 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
        match (*entry).type_0 as ::core::ffi::c_uint {
            13 => {
                ret = exif_ifd_to_dict(
                    logctx,
                    key,
                    &raw const (*entry).value.ifd,
                    metadata,
                );
                if ret < 0 as ::core::ffi::c_int {
                    break;
                }
            }
            3 | 4 => {
                let mut j: uint32_t = 0 as uint32_t;
                while j < (*entry).count {
                    av_bprintf(
                        &raw mut bp,
                        b"%s%7u\0" as *const u8 as *const ::core::ffi::c_char,
                        if j != 0 {
                            if j.wrapping_rem(8 as uint32_t) != 0 {
                                b", \0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"\n\0" as *const u8 as *const ::core::ffi::c_char
                            }
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        *(*entry).value.uint.offset(j as isize) as uint32_t,
                    );
                    j = j.wrapping_add(1);
                }
            }
            8 | 9 => {
                let mut j_0: uint32_t = 0 as uint32_t;
                while j_0 < (*entry).count {
                    av_bprintf(
                        &raw mut bp,
                        b"%s%7d\0" as *const u8 as *const ::core::ffi::c_char,
                        if j_0 != 0 {
                            if j_0.wrapping_rem(8 as uint32_t) != 0 {
                                b", \0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"\n\0" as *const u8 as *const ::core::ffi::c_char
                            }
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        *(*entry).value.sint.offset(j_0 as isize) as int32_t,
                    );
                    j_0 = j_0.wrapping_add(1);
                }
            }
            5 | 10 => {
                let mut j_1: uint32_t = 0 as uint32_t;
                while j_1 < (*entry).count {
                    av_bprintf(
                        &raw mut bp,
                        b"%s%7i:%-7i\0" as *const u8 as *const ::core::ffi::c_char,
                        if j_1 != 0 {
                            if j_1.wrapping_rem(4 as uint32_t) != 0 {
                                b", \0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"\n\0" as *const u8 as *const ::core::ffi::c_char
                            }
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        (*(*entry).value.rat.offset(j_1 as isize)).num,
                        (*(*entry).value.rat.offset(j_1 as isize)).den,
                    );
                    j_1 = j_1.wrapping_add(1);
                }
            }
            12 | 11 => {
                let mut j_2: uint32_t = 0 as uint32_t;
                while j_2 < (*entry).count {
                    av_bprintf(
                        &raw mut bp,
                        b"%s%.15g\0" as *const u8 as *const ::core::ffi::c_char,
                        if j_2 != 0 {
                            if j_2.wrapping_rem(4 as uint32_t) != 0 {
                                b", \0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"\n\0" as *const u8 as *const ::core::ffi::c_char
                            }
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        *(*entry).value.dbl.offset(j_2 as isize),
                    );
                    j_2 = j_2.wrapping_add(1);
                }
            }
            2 => {
                av_bprintf(
                    &raw mut bp,
                    b"%s\0" as *const u8 as *const ::core::ffi::c_char,
                    (*entry).value.str_0,
                );
            }
            7 | 1 => {
                let mut j_3: uint32_t = 0 as uint32_t;
                while j_3 < (*entry).count {
                    av_bprintf(
                        &raw mut bp,
                        b"%s%3i\0" as *const u8 as *const ::core::ffi::c_char,
                        if j_3 != 0 {
                            if j_3.wrapping_rem(16 as uint32_t) != 0 {
                                b", \0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"\n\0" as *const u8 as *const ::core::ffi::c_char
                            }
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        *(*entry).value.ubytes.offset(j_3 as isize) as ::core::ffi::c_int,
                    );
                    j_3 = j_3.wrapping_add(1);
                }
            }
            6 => {
                let mut j_4: uint32_t = 0 as uint32_t;
                while j_4 < (*entry).count {
                    av_bprintf(
                        &raw mut bp,
                        b"%s%3i\0" as *const u8 as *const ::core::ffi::c_char,
                        if j_4 != 0 {
                            if j_4.wrapping_rem(16 as uint32_t) != 0 {
                                b", \0" as *const u8 as *const ::core::ffi::c_char
                            } else {
                                b"\n\0" as *const u8 as *const ::core::ffi::c_char
                            }
                        } else {
                            b"\0" as *const u8 as *const ::core::ffi::c_char
                        },
                        *(*entry).value.sbytes.offset(j_4 as isize) as ::core::ffi::c_int,
                    );
                    j_4 = j_4.wrapping_add(1);
                }
            }
            _ => {}
        }
        if (*entry).type_0 as ::core::ffi::c_uint
            != AV_TIFF_IFD as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if av_bprint_is_complete(&raw mut bp) == 0 {
                av_bprint_finalize(
                    &raw mut bp,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                ret = -(12 as ::core::ffi::c_int);
                break;
            } else {
                ret = av_bprint_finalize(&raw mut bp, &raw mut value);
                if ret < 0 as ::core::ffi::c_int {
                    break;
                }
                ret = av_dict_set(
                    metadata,
                    key,
                    value,
                    4 as ::core::ffi::c_int | 8 as ::core::ffi::c_int,
                );
                key = ::core::ptr::null_mut::<::core::ffi::c_char>();
                value = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if ret < 0 as ::core::ffi::c_int {
                    break;
                }
            }
        } else {
            av_freep(&raw mut key as *mut ::core::ffi::c_void);
        }
        i = i.wrapping_add(1);
    }
    av_freep(&raw mut key as *mut ::core::ffi::c_void);
    av_freep(&raw mut value as *mut ::core::ffi::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn av_exif_ifd_to_dict(
    mut logctx: *mut ::core::ffi::c_void,
    mut ifd: *const AVExifMetadata,
    mut metadata: *mut *mut AVDictionary,
) -> ::core::ffi::c_int {
    return exif_ifd_to_dict(
        logctx,
        b"\0" as *const u8 as *const ::core::ffi::c_char,
        ifd,
        metadata,
    );
}
unsafe extern "C" fn exif_clone_entry(
    mut dst: *mut AVExifEntry,
    mut src: *const AVExifEntry,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    memset(
        dst as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AVExifEntry>() as size_t,
    );
    (*dst).count = (*src).count;
    (*dst).id = (*src).id;
    (*dst).type_0 = (*src).type_0;
    (*dst).ifd_offset = (*src).ifd_offset;
    if !(*src).ifd_lead.is_null() {
        (*dst).ifd_lead = av_memdup(
            (*src).ifd_lead as *const ::core::ffi::c_void,
            (*src).ifd_offset as size_t,
        ) as *mut uint8_t;
        if (*dst).ifd_lead.is_null() {
            ret = -(12 as ::core::ffi::c_int);
            current_block = 9346614910828999223;
        } else {
            current_block = 13513818773234778473;
        }
    } else {
        (*dst).ifd_lead = ::core::ptr::null_mut::<uint8_t>();
        current_block = 13513818773234778473;
    }
    match current_block {
        13513818773234778473 => {
            match (*src).type_0 as ::core::ffi::c_uint {
                13 => {
                    let mut cloned: *mut AVExifMetadata = av_exif_clone_ifd(
                        &raw const (*src).value.ifd,
                    );
                    if cloned.is_null() {
                        ret = -(12 as ::core::ffi::c_int);
                        current_block = 9346614910828999223;
                    } else {
                        (*dst).value.ifd = *cloned;
                        av_freep(&raw mut cloned as *mut ::core::ffi::c_void);
                        current_block = 13125627826496529465;
                    }
                }
                3 | 4 => {
                    let mut sz: size_t = 0;
                    if av_size_mult(
                        (*src).count as size_t,
                        ::core::mem::size_of::<uint64_t>() as size_t,
                        &raw mut sz,
                    ) < 0 as ::core::ffi::c_int
                    {
                        ret = -(12 as ::core::ffi::c_int);
                        current_block = 9346614910828999223;
                    } else {
                        (*dst).value.uint = av_memdup(
                            (*src).value.uint as *const ::core::ffi::c_void,
                            sz as size_t,
                        ) as *mut uint64_t;
                        if (*dst).value.uint.is_null() {
                            ret = -(12 as ::core::ffi::c_int);
                            current_block = 9346614910828999223;
                        } else {
                            current_block = 13125627826496529465;
                        }
                    }
                }
                9 | 8 => {
                    let mut sz_0: size_t = 0;
                    if av_size_mult(
                        (*src).count as size_t,
                        ::core::mem::size_of::<int64_t>() as size_t,
                        &raw mut sz_0,
                    ) < 0 as ::core::ffi::c_int
                    {
                        ret = -(12 as ::core::ffi::c_int);
                        current_block = 9346614910828999223;
                    } else {
                        (*dst).value.sint = av_memdup(
                            (*src).value.sint as *const ::core::ffi::c_void,
                            sz_0 as size_t,
                        ) as *mut int64_t;
                        if (*dst).value.sint.is_null() {
                            ret = -(12 as ::core::ffi::c_int);
                            current_block = 9346614910828999223;
                        } else {
                            current_block = 13125627826496529465;
                        }
                    }
                }
                5 | 10 => {
                    let mut sz_1: size_t = 0;
                    if av_size_mult(
                        (*src).count as size_t,
                        ::core::mem::size_of::<AVRational>() as size_t,
                        &raw mut sz_1,
                    ) < 0 as ::core::ffi::c_int
                    {
                        ret = -(12 as ::core::ffi::c_int);
                        current_block = 9346614910828999223;
                    } else {
                        (*dst).value.rat = av_memdup(
                            (*src).value.rat as *const ::core::ffi::c_void,
                            sz_1 as size_t,
                        ) as *mut AVRational;
                        if (*dst).value.rat.is_null() {
                            ret = -(12 as ::core::ffi::c_int);
                            current_block = 9346614910828999223;
                        } else {
                            current_block = 13125627826496529465;
                        }
                    }
                }
                12 | 11 => {
                    let mut sz_2: size_t = 0;
                    if av_size_mult(
                        (*src).count as size_t,
                        ::core::mem::size_of::<::core::ffi::c_double>() as size_t,
                        &raw mut sz_2,
                    ) < 0 as ::core::ffi::c_int
                    {
                        ret = -(12 as ::core::ffi::c_int);
                        current_block = 9346614910828999223;
                    } else {
                        (*dst).value.dbl = av_memdup(
                            (*src).value.dbl as *const ::core::ffi::c_void,
                            sz_2 as size_t,
                        ) as *mut ::core::ffi::c_double;
                        if (*dst).value.dbl.is_null() {
                            ret = -(12 as ::core::ffi::c_int);
                            current_block = 9346614910828999223;
                        } else {
                            current_block = 13125627826496529465;
                        }
                    }
                }
                1 | 7 => {
                    let mut sz_3: size_t = 0;
                    if av_size_mult(
                        (*src).count as size_t,
                        ::core::mem::size_of::<uint8_t>() as size_t,
                        &raw mut sz_3,
                    ) < 0 as ::core::ffi::c_int
                    {
                        ret = -(12 as ::core::ffi::c_int);
                        current_block = 9346614910828999223;
                    } else {
                        (*dst).value.ubytes = av_memdup(
                            (*src).value.ubytes as *const ::core::ffi::c_void,
                            sz_3 as size_t,
                        ) as *mut uint8_t;
                        if (*dst).value.ubytes.is_null() {
                            ret = -(12 as ::core::ffi::c_int);
                            current_block = 9346614910828999223;
                        } else {
                            current_block = 13125627826496529465;
                        }
                    }
                }
                6 => {
                    let mut sz_4: size_t = 0;
                    if av_size_mult(
                        (*src).count as size_t,
                        ::core::mem::size_of::<int8_t>() as size_t,
                        &raw mut sz_4,
                    ) < 0 as ::core::ffi::c_int
                    {
                        ret = -(12 as ::core::ffi::c_int);
                        current_block = 9346614910828999223;
                    } else {
                        (*dst).value.sbytes = av_memdup(
                            (*src).value.sbytes as *const ::core::ffi::c_void,
                            sz_4 as size_t,
                        ) as *mut int8_t;
                        if (*dst).value.sbytes.is_null() {
                            ret = -(12 as ::core::ffi::c_int);
                            current_block = 9346614910828999223;
                        } else {
                            current_block = 13125627826496529465;
                        }
                    }
                }
                2 => {
                    (*dst).value.str_0 = av_memdup(
                        (*src).value.str_0 as *const ::core::ffi::c_void,
                        ((*src).count as size_t).wrapping_add(1 as size_t),
                    ) as *mut ::core::ffi::c_char;
                    if (*dst).value.str_0.is_null() {
                        ret = -(12 as ::core::ffi::c_int);
                        current_block = 9346614910828999223;
                    } else {
                        current_block = 13125627826496529465;
                    }
                }
                _ => {
                    current_block = 13125627826496529465;
                }
            }
            match current_block {
                9346614910828999223 => {}
                _ => return 0 as ::core::ffi::c_int,
            }
        }
        _ => {}
    }
    av_freep(&raw mut (*dst).ifd_lead as *mut ::core::ffi::c_void);
    if (*src).type_0 as ::core::ffi::c_uint
        == AV_TIFF_IFD as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        av_exif_free(&raw mut (*dst).value.ifd);
    } else {
        av_freep(&raw mut (*dst).value.ptr as *mut ::core::ffi::c_void);
    }
    memset(
        dst as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<AVExifEntry>() as size_t,
    );
    return ret;
}
unsafe extern "C" fn exif_get_entry(
    mut logctx: *mut ::core::ffi::c_void,
    mut ifd: *mut AVExifMetadata,
    mut id: uint16_t,
    mut depth: ::core::ffi::c_int,
    mut value: *mut *mut AVExifEntry,
) -> ::core::ffi::c_int {
    let mut offset: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if ifd.is_null() || (*ifd).count != 0 && (*ifd).entries.is_null() || value.is_null()
    {
        return -(22 as ::core::ffi::c_int);
    }
    let mut i: size_t = 0 as size_t;
    while i < (*ifd).count {
        if (*(*ifd).entries.offset(i as isize)).id as ::core::ffi::c_int
            == id as ::core::ffi::c_int
        {
            *value = (*ifd).entries.offset(i as isize) as *mut AVExifEntry;
            return i.wrapping_add(offset as size_t) as ::core::ffi::c_int;
        }
        if (*(*ifd).entries.offset(i as isize)).type_0 as ::core::ffi::c_uint
            == AV_TIFF_IFD as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if depth < 3 as ::core::ffi::c_int {
                let mut ret: ::core::ffi::c_int = exif_get_entry(
                    logctx,
                    &raw mut (*(*ifd).entries.offset(i as isize)).value.ifd,
                    id,
                    depth + 1 as ::core::ffi::c_int,
                    value,
                );
                if ret != 0 {
                    return if ret < 0 as ::core::ffi::c_int {
                        ret
                    } else {
                        ret + offset
                    };
                }
            }
            offset = (offset as ::core::ffi::c_uint)
                .wrapping_add((*(*ifd).entries.offset(i as isize)).value.ifd.count)
                as ::core::ffi::c_int as ::core::ffi::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_exif_get_entry(
    mut logctx: *mut ::core::ffi::c_void,
    mut ifd: *mut AVExifMetadata,
    mut id: uint16_t,
    mut flags: ::core::ffi::c_int,
    mut value: *mut *mut AVExifEntry,
) -> ::core::ffi::c_int {
    return exif_get_entry(
        logctx,
        ifd,
        id,
        if flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            2147483647 as ::core::ffi::c_int
        },
        value,
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_exif_set_entry(
    mut logctx: *mut ::core::ffi::c_void,
    mut ifd: *mut AVExifMetadata,
    mut id: uint16_t,
    mut type_0: AVTiffDataType,
    mut count: uint32_t,
    mut ifd_lead: *const uint8_t,
    mut ifd_offset: uint32_t,
    mut value: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut temp: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<
        ::core::ffi::c_void,
    >();
    let mut ret: ::core::ffi::c_int = 0;
    let mut offset: ::core::ffi::c_int = 0;
    let mut entry: *mut AVExifEntry = ::core::ptr::null_mut::<AVExifEntry>();
    let mut src: AVExifEntry = AVExifEntry {
        id: 0 as uint16_t,
        type_0: 0 as AVTiffDataType,
        count: 0,
        ifd_offset: 0,
        ifd_lead: ::core::ptr::null_mut::<uint8_t>(),
        value: C2RustUnnamed_2 {
            ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
    };
    if ifd.is_null() || (*ifd).count != 0 && (*ifd).entries.is_null()
        || !ifd_lead.is_null() && ifd_offset == 0
        || ifd_lead.is_null() && ifd_offset != 0 || value.is_null()
        || (*ifd).count == 0xffff as ::core::ffi::c_uint
    {
        return -(22 as ::core::ffi::c_int);
    }
    ret = av_exif_get_entry(logctx, ifd, id, 0 as ::core::ffi::c_int, &raw mut entry);
    if ret < 0 as ::core::ffi::c_int {
        return ret;
    }
    offset = ret;
    if !entry.is_null() {
        exif_free_entry(entry);
    } else {
        let mut required_size: size_t = 0;
        ret = av_size_mult(
            ((*ifd).count as size_t).wrapping_add(1 as size_t),
            ::core::mem::size_of::<AVExifEntry>() as size_t,
            &raw mut required_size,
        );
        if ret < 0 as ::core::ffi::c_int {
            return -(12 as ::core::ffi::c_int);
        }
        temp = av_fast_realloc(
            (*ifd).entries as *mut ::core::ffi::c_void,
            &raw mut (*ifd).size,
            required_size as size_t,
        );
        if temp.is_null() {
            return -(12 as ::core::ffi::c_int);
        }
        (*ifd).entries = temp as *mut AVExifEntry;
        let fresh0 = (*ifd).count;
        (*ifd).count = (*ifd).count.wrapping_add(1);
        entry = (*ifd).entries.offset(fresh0 as isize) as *mut AVExifEntry;
    }
    src.count = count;
    src.id = id;
    src.type_0 = type_0;
    src.ifd_lead = ifd_lead as *mut uint8_t;
    src.ifd_offset = ifd_offset;
    if type_0 as ::core::ffi::c_uint
        == AV_TIFF_IFD as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        src.value.ifd = *(value as *const AVExifMetadata);
    } else {
        src.value.ptr = value as *mut ::core::ffi::c_void;
    }
    ret = exif_clone_entry(entry, &raw mut src);
    if ret < 0 as ::core::ffi::c_int {
        if offset != 0 {
            let mut remaining: size_t = ((*ifd).count as size_t)
                .wrapping_sub(offset as size_t);
            memmove(
                (*ifd).entries.offset((offset - 1 as ::core::ffi::c_int) as isize)
                    as *mut AVExifEntry as *mut ::core::ffi::c_void,
                (*ifd).entries.offset(offset as isize) as *mut AVExifEntry
                    as *const ::core::ffi::c_void,
                (::core::mem::size_of::<AVExifEntry>() as size_t).wrapping_mul(remaining),
            );
        }
        (*ifd).count = (*ifd).count.wrapping_sub(1);
    }
    return ret;
}
unsafe extern "C" fn exif_remove_entry(
    mut logctx: *mut ::core::ffi::c_void,
    mut ifd: *mut AVExifMetadata,
    mut id: uint16_t,
    mut depth: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut index: int32_t = -(1 as int32_t);
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ifd.is_null() || (*ifd).count != 0 && (*ifd).entries.is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    let mut i: size_t = 0 as size_t;
    while i < (*ifd).count {
        if (*(*ifd).entries.offset(i as isize)).id as ::core::ffi::c_int
            == id as ::core::ffi::c_int
        {
            index = i as int32_t;
            break;
        } else {
            if (*(*ifd).entries.offset(i as isize)).type_0 as ::core::ffi::c_uint
                == AV_TIFF_IFD as ::core::ffi::c_int as ::core::ffi::c_uint
                && depth < 3 as ::core::ffi::c_int
            {
                ret = exif_remove_entry(
                    logctx,
                    &raw mut (*(*ifd).entries.offset(i as isize)).value.ifd,
                    id,
                    depth + 1 as ::core::ffi::c_int,
                );
                if ret != 0 {
                    return ret;
                }
            }
            i = i.wrapping_add(1);
        }
    }
    if index < 0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_int;
    }
    exif_free_entry((*ifd).entries.offset(index as isize) as *mut AVExifEntry);
    (*ifd).count = (*ifd).count.wrapping_sub(1);
    if index as ::core::ffi::c_uint == (*ifd).count {
        if index == 0 {
            av_freep(&raw mut (*ifd).entries as *mut ::core::ffi::c_void);
            (*ifd).size = 0 as ::core::ffi::c_uint;
        }
        return 1 as ::core::ffi::c_int;
    }
    memmove(
        (*ifd).entries.offset(index as isize) as *mut AVExifEntry
            as *mut ::core::ffi::c_void,
        (*ifd)
            .entries
            .offset((index as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize)
            as *mut AVExifEntry as *const ::core::ffi::c_void,
        ((*ifd).count as size_t)
            .wrapping_sub(index as size_t)
            .wrapping_mul(::core::mem::size_of::<AVExifEntry>() as size_t),
    );
    return (1 as ::core::ffi::c_uint)
        .wrapping_add((*ifd).count.wrapping_sub(index as ::core::ffi::c_uint))
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn av_exif_remove_entry(
    mut logctx: *mut ::core::ffi::c_void,
    mut ifd: *mut AVExifMetadata,
    mut id: uint16_t,
    mut flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return exif_remove_entry(
        logctx,
        ifd,
        id,
        if flags & (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            2147483647 as ::core::ffi::c_int
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn av_exif_clone_ifd(
    mut ifd: *const AVExifMetadata,
) -> *mut AVExifMetadata {
    let mut current_block: u64;
    let mut ret: *mut AVExifMetadata = av_mallocz(
        ::core::mem::size_of::<AVExifMetadata>() as size_t,
    ) as *mut AVExifMetadata;
    if ret.is_null() {
        return ::core::ptr::null_mut::<AVExifMetadata>();
    }
    (*ret).count = (*ifd).count;
    if (*ret).count != 0 {
        let mut required_size: size_t = 0;
        if av_size_mult(
            (*ret).count as size_t,
            ::core::mem::size_of::<AVExifEntry>() as size_t,
            &raw mut required_size,
        ) < 0 as ::core::ffi::c_int
        {
            current_block = 9131948177147154112;
        } else {
            av_fast_mallocz(
                &raw mut (*ret).entries as *mut ::core::ffi::c_void,
                &raw mut (*ret).size,
                required_size as size_t,
            );
            if (*ret).entries.is_null() {
                current_block = 9131948177147154112;
            } else {
                current_block = 4906268039856690917;
            }
        }
    } else {
        current_block = 4906268039856690917;
    }
    match current_block {
        4906268039856690917 => {
            let mut i: size_t = 0 as size_t;
            loop {
                if !(i < (*ret).count) {
                    current_block = 17965632435239708295;
                    break;
                }
                let mut entry: *const AVExifEntry = (*ifd).entries.offset(i as isize)
                    as *mut AVExifEntry;
                let mut ret_entry: *mut AVExifEntry = (*ret).entries.offset(i as isize)
                    as *mut AVExifEntry;
                let mut status: ::core::ffi::c_int = exif_clone_entry(ret_entry, entry);
                if status < 0 as ::core::ffi::c_int {
                    current_block = 9131948177147154112;
                    break;
                }
                i = i.wrapping_add(1);
            }
            match current_block {
                9131948177147154112 => {}
                _ => return ret,
            }
        }
        _ => {}
    }
    av_exif_free(ret);
    av_free(ret as *mut ::core::ffi::c_void);
    return ::core::ptr::null_mut::<AVExifMetadata>();
}
static mut rotation_lut: [[::core::ffi::c_int; 4]; 2] = [
    [
        1 as ::core::ffi::c_int,
        8 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        6 as ::core::ffi::c_int,
    ],
    [
        4 as ::core::ffi::c_int,
        7 as ::core::ffi::c_int,
        2 as ::core::ffi::c_int,
        5 as ::core::ffi::c_int,
    ],
];
#[no_mangle]
pub unsafe extern "C" fn av_exif_matrix_to_orientation(
    mut matrix: *const int32_t,
) -> ::core::ffi::c_int {
    let mut rotation: ::core::ffi::c_double = av_display_rotation_get(
        matrix as *const int32_t,
    );
    let mut vflip: ::core::ffi::c_int = ((*matrix
        .offset(0 as ::core::ffi::c_int as isize) as int64_t
        * *matrix.offset(4 as ::core::ffi::c_int as isize) as int64_t
        - *matrix.offset(1 as ::core::ffi::c_int as isize) as int64_t
            * *matrix.offset(3 as ::core::ffi::c_int as isize) as int64_t)
        < 0 as int64_t) as ::core::ffi::c_int;
    if if ::core::mem::size_of::<::core::ffi::c_double>() as usize
        == ::core::mem::size_of::<::core::ffi::c_float>() as usize
    {
        ((__FLOAT_BITS(rotation as ::core::ffi::c_float)
            & 0x7fffffff as ::core::ffi::c_int as ::core::ffi::c_uint)
            < 0x7f800000 as ::core::ffi::c_int as ::core::ffi::c_uint)
            as ::core::ffi::c_int
    } else if ::core::mem::size_of::<::core::ffi::c_double>() as usize
        == ::core::mem::size_of::<::core::ffi::c_double>() as usize
    {
        ((__DOUBLE_BITS(rotation)
            & (1 as ::core::ffi::c_ulonglong).wrapping_neg() >> 1 as ::core::ffi::c_int)
            < (0x7ff as ::core::ffi::c_ulonglong) << 52 as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } else {
        (__fpclassifyl(crate::rt::F128::new(rotation)) > 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    let mut rot: ::core::ffi::c_int = (rotation + 0.5f64) as ::core::ffi::c_int;
    rot = (rot % 360 as ::core::ffi::c_int + 360 as ::core::ffi::c_int)
        % 360 as ::core::ffi::c_int / 90 as ::core::ffi::c_int;
    return rotation_lut[vflip as usize][rot as usize];
}
#[no_mangle]
pub unsafe extern "C" fn av_exif_orientation_to_matrix(
    mut matrix: *mut int32_t,
    mut orientation: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    match orientation {
        1 => {
            av_display_rotation_set(matrix as *mut int32_t, 0.0f64);
        }
        2 => {
            av_display_rotation_set(matrix as *mut int32_t, 0.0f64);
            av_display_matrix_flip(
                matrix as *mut int32_t,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
        3 => {
            av_display_rotation_set(matrix as *mut int32_t, 180.0f64);
        }
        4 => {
            av_display_rotation_set(matrix as *mut int32_t, 180.0f64);
            av_display_matrix_flip(
                matrix as *mut int32_t,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
        5 => {
            av_display_rotation_set(matrix as *mut int32_t, 90.0f64);
            av_display_matrix_flip(
                matrix as *mut int32_t,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
        6 => {
            av_display_rotation_set(matrix as *mut int32_t, 90.0f64);
        }
        7 => {
            av_display_rotation_set(matrix as *mut int32_t, -90.0f64);
            av_display_matrix_flip(
                matrix as *mut int32_t,
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
            );
        }
        8 => {
            av_display_rotation_set(matrix as *mut int32_t, -90.0f64);
        }
        _ => return -(22 as ::core::ffi::c_int),
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ff_exif_sanitize_ifd(
    mut logctx: *mut ::core::ffi::c_void,
    mut frame: *const AVFrame,
    mut ifd: *mut AVExifMetadata,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sd_orient: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    let mut or: *mut AVExifEntry = ::core::ptr::null_mut::<AVExifEntry>();
    let mut iw: *mut AVExifEntry = ::core::ptr::null_mut::<AVExifEntry>();
    let mut ih: *mut AVExifEntry = ::core::ptr::null_mut::<AVExifEntry>();
    let mut pw: *mut AVExifEntry = ::core::ptr::null_mut::<AVExifEntry>();
    let mut ph: *mut AVExifEntry = ::core::ptr::null_mut::<AVExifEntry>();
    let mut orientation: uint64_t = 1 as uint64_t;
    let mut w: uint64_t = (*frame).width as uint64_t;
    let mut h: uint64_t = (*frame).height as uint64_t;
    let mut rewrite: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    sd_orient = av_frame_get_side_data(frame, AV_FRAME_DATA_DISPLAYMATRIX);
    if !sd_orient.is_null() {
        orientation = av_exif_matrix_to_orientation((*sd_orient).data as *mut int32_t)
            as uint64_t;
    }
    if orientation == 0 {
        av_log(
            logctx,
            24 as ::core::ffi::c_int,
            b"display matrix is singular\n\0" as *const u8 as *const ::core::ffi::c_char,
        );
        orientation = 1 as uint64_t;
    }
    if orientation != 1 as uint64_t {
        av_log(
            logctx,
            48 as ::core::ffi::c_int,
            b"matrix contains nontrivial EXIF orientation: %llu\n\0" as *const u8
                as *const ::core::ffi::c_char,
            orientation,
        );
    }
    let mut i: size_t = 0 as size_t;
    while i < (*ifd).count {
        let mut entry: *mut AVExifEntry = (*ifd).entries.offset(i as isize)
            as *mut AVExifEntry;
        if (*entry).id as ::core::ffi::c_int == 0x112 as ::core::ffi::c_int
            && (*entry).count > 0 as uint32_t
            && (*entry).type_0 as ::core::ffi::c_uint
                == AV_TIFF_SHORT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            or = entry;
        } else if (*entry).id as ::core::ffi::c_int == 0x100 as ::core::ffi::c_int
            && (*entry).count > 0 as uint32_t
            && (*entry).type_0 as ::core::ffi::c_uint
                == AV_TIFF_LONG as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            iw = entry;
        } else if (*entry).id as ::core::ffi::c_int == 0x101 as ::core::ffi::c_int
            && (*entry).count > 0 as uint32_t
            && (*entry).type_0 as ::core::ffi::c_uint
                == AV_TIFF_LONG as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            ih = entry;
        } else if (*entry).id as ::core::ffi::c_int == 0x8769 as ::core::ffi::c_int
            && (*entry).type_0 as ::core::ffi::c_uint
                == AV_TIFF_IFD as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut exif: *mut AVExifMetadata = &raw mut (*entry).value.ifd;
            let mut j: size_t = 0 as size_t;
            while j < (*exif).count {
                let mut exifentry: *mut AVExifEntry = (*exif).entries.offset(j as isize)
                    as *mut AVExifEntry;
                if (*exifentry).id as ::core::ffi::c_int == 0xa002 as ::core::ffi::c_int
                    && (*exifentry).count > 0 as uint32_t
                    && (*exifentry).type_0 as ::core::ffi::c_uint
                        == AV_TIFF_SHORT as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    pw = exifentry;
                } else if (*exifentry).id as ::core::ffi::c_int
                    == 0xa003 as ::core::ffi::c_int && (*exifentry).count > 0 as uint32_t
                    && (*exifentry).type_0 as ::core::ffi::c_uint
                        == AV_TIFF_SHORT as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    ph = exifentry;
                }
                j = j.wrapping_add(1);
            }
        }
        i = i.wrapping_add(1);
    }
    if !or.is_null()
        && *(*or).value.uint.offset(0 as ::core::ffi::c_int as isize) != orientation
    {
        rewrite = 1 as ::core::ffi::c_int;
        *(*or).value.uint.offset(0 as ::core::ffi::c_int as isize) = orientation;
    }
    if !iw.is_null() && *(*iw).value.uint.offset(0 as ::core::ffi::c_int as isize) != w {
        rewrite = 1 as ::core::ffi::c_int;
        *(*iw).value.uint.offset(0 as ::core::ffi::c_int as isize) = w;
    }
    if !ih.is_null() && *(*ih).value.uint.offset(0 as ::core::ffi::c_int as isize) != h {
        rewrite = 1 as ::core::ffi::c_int;
        *(*ih).value.uint.offset(0 as ::core::ffi::c_int as isize) = h;
    }
    if !pw.is_null() && *(*pw).value.uint.offset(0 as ::core::ffi::c_int as isize) != w {
        rewrite = 1 as ::core::ffi::c_int;
        *(*pw).value.uint.offset(0 as ::core::ffi::c_int as isize) = w;
    }
    if !ph.is_null() && *(*ph).value.uint.offset(0 as ::core::ffi::c_int as isize) != h {
        rewrite = 1 as ::core::ffi::c_int;
        *(*ph).value.uint.offset(0 as ::core::ffi::c_int as isize) = h;
    }
    if or.is_null() && orientation != 1 as uint64_t {
        rewrite = 1 as ::core::ffi::c_int;
        ret = av_exif_set_entry(
            logctx,
            ifd,
            0x112 as uint16_t,
            AV_TIFF_SHORT,
            1 as uint32_t,
            ::core::ptr::null::<uint8_t>(),
            0 as uint32_t,
            &raw mut orientation as *const ::core::ffi::c_void,
        );
        if ret < 0 as ::core::ffi::c_int {
            current_block = 7912353539484221458;
        } else {
            current_block = 313581471991351815;
        }
    } else {
        current_block = 313581471991351815;
    }
    match current_block {
        313581471991351815 => {
            if iw.is_null() && w != 0 {
                rewrite = 1 as ::core::ffi::c_int;
                ret = av_exif_set_entry(
                    logctx,
                    ifd,
                    0x100 as uint16_t,
                    AV_TIFF_LONG,
                    1 as uint32_t,
                    ::core::ptr::null::<uint8_t>(),
                    0 as uint32_t,
                    &raw mut w as *const ::core::ffi::c_void,
                );
                if ret < 0 as ::core::ffi::c_int {
                    current_block = 7912353539484221458;
                } else {
                    current_block = 7990025728955927862;
                }
            } else {
                current_block = 7990025728955927862;
            }
            match current_block {
                7912353539484221458 => {}
                _ => {
                    if ih.is_null() && h != 0 {
                        rewrite = 1 as ::core::ffi::c_int;
                        ret = av_exif_set_entry(
                            logctx,
                            ifd,
                            0x101 as uint16_t,
                            AV_TIFF_LONG,
                            1 as uint32_t,
                            ::core::ptr::null::<uint8_t>(),
                            0 as uint32_t,
                            &raw mut h as *const ::core::ffi::c_void,
                        );
                        if ret < 0 as ::core::ffi::c_int {
                            current_block = 7912353539484221458;
                        } else {
                            current_block = 9353995356876505083;
                        }
                    } else {
                        current_block = 9353995356876505083;
                    }
                    match current_block {
                        7912353539484221458 => {}
                        _ => {
                            if pw.is_null() && w != 0 && w <= 0xffff as uint64_t
                                || ph.is_null() && h != 0 && h <= 0xffff as uint64_t
                            {
                                let mut exif_0: *mut AVExifMetadata = ::core::ptr::null_mut::<
                                    AVExifMetadata,
                                >();
                                let mut exif_entry: *mut AVExifEntry = ::core::ptr::null_mut::<
                                    AVExifEntry,
                                >();
                                let mut exif_found: ::core::ffi::c_int = av_exif_get_entry(
                                    logctx,
                                    ifd,
                                    0x8769 as uint16_t,
                                    0 as ::core::ffi::c_int,
                                    &raw mut exif_entry,
                                );
                                rewrite = 1 as ::core::ffi::c_int;
                                if exif_found < 0 as ::core::ffi::c_int {
                                    current_block = 7912353539484221458;
                                } else {
                                    if exif_found > 0 as ::core::ffi::c_int {
                                        exif_0 = &raw mut (*exif_entry).value.ifd;
                                        current_block = 13707613154239713890;
                                    } else {
                                        let mut exif_new: AVExifMetadata = AVExifMetadata {
                                            entries: ::core::ptr::null_mut::<AVExifEntry>(),
                                            count: 0,
                                            size: 0,
                                        };
                                        ret = av_exif_set_entry(
                                            logctx,
                                            ifd,
                                            0x8769 as uint16_t,
                                            AV_TIFF_IFD,
                                            1 as uint32_t,
                                            ::core::ptr::null::<uint8_t>(),
                                            0 as uint32_t,
                                            &raw mut exif_new as *const ::core::ffi::c_void,
                                        );
                                        if ret < 0 as ::core::ffi::c_int {
                                            av_exif_free(&raw mut exif_new);
                                            current_block = 7912353539484221458;
                                        } else {
                                            exif_0 = &raw mut (*(*ifd)
                                                .entries
                                                .offset(
                                                    (*ifd).count.wrapping_sub(1 as ::core::ffi::c_uint) as isize,
                                                ))
                                                .value
                                                .ifd;
                                            current_block = 13707613154239713890;
                                        }
                                    }
                                    match current_block {
                                        7912353539484221458 => {}
                                        _ => {
                                            if pw.is_null() && w != 0 && w <= 0xffff as uint64_t {
                                                ret = av_exif_set_entry(
                                                    logctx,
                                                    exif_0,
                                                    0xa002 as uint16_t,
                                                    AV_TIFF_SHORT,
                                                    1 as uint32_t,
                                                    ::core::ptr::null::<uint8_t>(),
                                                    0 as uint32_t,
                                                    &raw mut w as *const ::core::ffi::c_void,
                                                );
                                                if ret < 0 as ::core::ffi::c_int {
                                                    current_block = 7912353539484221458;
                                                } else {
                                                    current_block = 16779030619667747692;
                                                }
                                            } else {
                                                current_block = 16779030619667747692;
                                            }
                                            match current_block {
                                                7912353539484221458 => {}
                                                _ => {
                                                    if ph.is_null() && h != 0 && h <= 0xffff as uint64_t {
                                                        ret = av_exif_set_entry(
                                                            logctx,
                                                            exif_0,
                                                            0xa003 as uint16_t,
                                                            AV_TIFF_SHORT,
                                                            1 as uint32_t,
                                                            ::core::ptr::null::<uint8_t>(),
                                                            0 as uint32_t,
                                                            &raw mut h as *const ::core::ffi::c_void,
                                                        );
                                                        if ret < 0 as ::core::ffi::c_int {
                                                            current_block = 7912353539484221458;
                                                        } else {
                                                            current_block = 7419121793134201633;
                                                        }
                                                    } else {
                                                        current_block = 7419121793134201633;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            } else {
                                current_block = 7419121793134201633;
                            }
                            match current_block {
                                7912353539484221458 => {}
                                _ => return rewrite,
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ff_exif_get_buffer(
    mut logctx: *mut ::core::ffi::c_void,
    mut frame: *const AVFrame,
    mut buffer_ptr: *mut *mut AVBufferRef,
    mut header_mode: AVExifHeaderMode,
) -> ::core::ffi::c_int {
    let mut current_block: u64;
    let mut sd_exif: *mut AVFrameSideData = ::core::ptr::null_mut::<AVFrameSideData>();
    let mut buffer: *mut AVBufferRef = ::core::ptr::null_mut::<AVBufferRef>();
    let mut ifd: AVExifMetadata = AVExifMetadata {
        entries: ::core::ptr::null_mut::<AVExifEntry>(),
        count: 0,
        size: 0,
    };
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut rewrite: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if buffer_ptr.is_null() || !(*buffer_ptr).is_null() {
        return -(22 as ::core::ffi::c_int);
    }
    sd_exif = av_frame_get_side_data(frame, AV_FRAME_DATA_EXIF);
    if sd_exif.is_null()
        && av_frame_get_side_data(frame, AV_FRAME_DATA_DISPLAYMATRIX).is_null()
    {
        return 0 as ::core::ffi::c_int;
    }
    if !sd_exif.is_null() {
        ret = av_exif_parse_buffer(
            logctx,
            (*sd_exif).data,
            (*sd_exif).size,
            &raw mut ifd,
            AV_EXIF_TIFF_HEADER,
        );
        if ret < 0 as ::core::ffi::c_int {
            current_block = 7971676384592167646;
        } else {
            current_block = 14523784380283086299;
        }
    } else {
        current_block = 14523784380283086299;
    }
    match current_block {
        14523784380283086299 => {
            rewrite = ff_exif_sanitize_ifd(logctx, frame, &raw mut ifd);
            if rewrite < 0 as ::core::ffi::c_int {
                ret = rewrite;
            } else {
                rewrite = ((rewrite != 0
                    || header_mode as ::core::ffi::c_uint
                        != AV_EXIF_TIFF_HEADER as ::core::ffi::c_int
                            as ::core::ffi::c_uint) && ifd.count != 0)
                    as ::core::ffi::c_int;
                if rewrite != 0 {
                    ret = av_exif_write(
                        logctx,
                        &raw mut ifd,
                        &raw mut buffer,
                        header_mode,
                    );
                    if ret < 0 as ::core::ffi::c_int {
                        current_block = 7971676384592167646;
                    } else {
                        *buffer_ptr = buffer;
                        current_block = 13242334135786603907;
                    }
                } else if !sd_exif.is_null() {
                    *buffer_ptr = av_buffer_ref((*sd_exif).buf);
                    if (*buffer_ptr).is_null() {
                        ret = -(12 as ::core::ffi::c_int);
                        current_block = 7971676384592167646;
                    } else {
                        current_block = 13242334135786603907;
                    }
                } else {
                    current_block = 13242334135786603907;
                }
                match current_block {
                    7971676384592167646 => {}
                    _ => {
                        av_exif_free(&raw mut ifd);
                        return (rewrite != 0 || !sd_exif.is_null())
                            as ::core::ffi::c_int;
                    }
                }
            }
        }
        _ => {}
    }
    av_exif_free(&raw mut ifd);
    return ret;
}
#[repr(C)]
pub struct AVBuffer {
    _opaque: [u8; 0],
}
#[repr(C)]
pub struct AVDictionary {
    _opaque: [u8; 0],
}
